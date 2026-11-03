# Arquitetura do UTSim

Este documento é a **especificação de comportamento** do UTSim, escrita a partir da
literatura de simulação a eventos discretos. Nenhum código ou binário proprietário foi
analisado. As decisões de projeto que merecem destaque aparecem como **"Nosso jeito"**.

## 1. Camadas

| Camada | Crate | Regra principal |
|---|---|---|
| Motor DES | `utsim-core` | determinístico, sem interface gráfica |
| Blocos | `utsim-objects` | só falam com o motor via `Ctx`, nunca entre si |
| Modelo | `utsim-model` | o modelo é dado (texto `.ron`), não código |
| Estatística | `utsim-stats` | funções puras sobre números (só depende de `utsim-core`); análise de saída e funções especiais |
| Execução | `utsim-cli` | replicações em paralelo, sem tela |
| PERT/CPM | `utsim-pert` | rede de atividades e caminho crítico de um trecho, sem tela (seção 4s) |
| 3D / editor | *(fase 2+)* | **apenas lê** o estado; nunca altera a simulação |

O 3D não comanda o tempo. Um item "andando" é uma interpolação entre dois eventos
(`entrou em t0`, `chega em t1`). Assim a mesma simulação roda a 1× com animação ou
1000× mais rápido sem tela, com resultados idênticos.

## 2. O motor de eventos

- **Relógio** `f64`, em unidades escolhidas pelo modelador.
- **Lista de eventos futuros**: heap mínimo ordenado por `(tempo, sequência)`. Eventos
  simultâneos rodam na ordem em que foram agendados, o que dá determinismo total.
- **Laço**: retira o próximo evento → avança o relógio → chama o bloco dono →
  **resolve o fluxo** (seção 3) → verifica erros. Para em `stop_time`.
- **Aquecimento (warm-up)**: um evento interno em `warmup` zera todas as estatísticas.
- **Números aleatórios**: cada bloco tem um *stream* PCG-64 próprio, derivado de
  `(seed, replicação, índice do bloco)`. Mexer na aleatoriedade de um bloco não altera
  os números sorteados pelos outros (variáveis aleatórias comuns entre cenários).

## 3. Protocolo de fluxo de itens

É o coração do comportamento. Todo recurso fixo segue o ciclo
**receber → processar → liberar → transferir**.

1. **Receber.** Um bloco aceita um item quando sua **entrada está aberta**
   (`Behavior::accepts`, normalmente "conteúdo < capacidade").
2. **Processar.** Cada tipo de bloco decide quanto tempo o item fica: a Queue libera
   na hora, o Processor segura durante setup + processamento.
3. **Liberar.** O bloco chama `ctx.release(item)`. Nesse instante a **regra de saída**
   escolhe a porta, uma única vez por item:
   - `FirstAvailable`: qualquer porta; vence a primeira aberta, em ordem de porta.
   - `RoundRobin` / `Random` / `Weighted([pesos])` / `ByLabel`: uma porta específica.
     O item espera por *aquela* porta, mesmo que outra esteja livre.
4. **Transferir.** A conexão anda quando **os dois lados estão abertos**: há item
   liberado na saída e o destino aceita. A transferência é instantânea (transporte
   por operadores e empilhadeiras vem na fase 4).

**Resolução.** Depois de cada evento, o motor processa uma fila de blocos "sujos",
isto é, blocos cuja saída pode ter destravado. Quando um item sai de B, B ganhou espaço
e seus blocos de entrada são marcados como sujos **em ordem de porta de entrada**. Essa
ordem é a prioridade de *pull* padrão: com duas filas ligadas a uma máquina, a conectada
primeiro tem preferência. Tudo acontece no mesmo instante simulado.

**Portas de entrada.** `accepts(item, conteúdo, porta)` e `on_entry(item, porta)`
recebem o número da porta de entrada (0, 1, ..., na ordem das conexões). É assim que o
Combiner distingue o contêiner (porta 0) dos componentes (portas 1..k).

**Ordem dos ganchos numa transferência A → B:**
`A.conteúdo -= 1` → `A.on_exit` → `B.conteúdo += 1` → `B.on_entry(porta)` → reavaliar
quem alimenta A.

> **Nosso jeito:** o conteúdo já foi atualizado quando `on_exit` roda, o que simplifica a lógica
> dos blocos ("fiquei vazio?").

> **Nosso jeito:** não existe um retorno mágico `-1` para "segurar o item".
> Um bloco simplesmente não chama `release` até querer.

## 4. Blocos da fase 0

### Source
- Cria um item, aplica rótulos (`labels`) e o libera.
- **O próximo intervalo começa quando o item sai**. Se a Source fica
  bloqueada, ela para de gerar (estado `blocked`). Para ter chegadas de Poisson "puras",
  coloque uma fila depois dela.
- `first_at_zero`: o primeiro item nasce em t=0, e não depois do primeiro intervalo.
- `on_blocked` (o que fazer com um item que não consegue sair no instante em que nasce):
  - `Wait` (padrão): o comportamento acima, a Source espera e para de gerar.
  - `Drop` (**nosso jeito**): o item é destruído e o próximo intervalo
    começa na hora, então as chegadas formam um processo de renovação independente do
    bloqueio. Cada perda registra `lost` na Source (a tally; a média vale sempre 1) e entra
    nas métricas `lost.count`, `entered.count` e `loss_fraction`, sempre presentes numa
    Source com `Drop` e zeradas na virada do aquecimento; itens perdidos não contam em
    `output`. Serve para M/M/1/K e Erlang de perda. `RunReport::metrics` soma as fontes em
    `model.lost.count`, `model.entered.count` e `model.loss_fraction`.
- Estados: `generating`, `blocked`, `idle` (quando atinge `max_arrivals`), `off_shift`
  (fora do horário das chegadas por período).
- **Chegadas por período** (`schedule: (cycle, periods: [(start, end, arrivals, labels)])`).
  A Source guarda as marcas do ciclo (inícios e fins dos períodos, com o fim igual ao ciclo
  virando 0) e agenda um evento próprio em cada marca. Na marca, o período em vigor muda:
  a chegada pendente é cancelada e uma nova é sorteada a partir da marca com o processo do
  novo período (`Default` usa `interarrival`, `Interarrival(Dist)`, `Rate(r)` sorteia
  `Exp(1/r)`, `Closed` e as lacunas não agendam nada e mostram `off_shift`). Dois períodos
  seguidos com o mesmo processo mantêm a chegada pendente. Como cada período recomeça do
  zero, um período `Rate` é um processo de Poisson não homogêneo exato, com taxa constante
  por trechos (falta de memória da exponencial), sem distorção nas fronteiras. A marca é
  agendada no início do período anterior, então vem antes de qualquer chegada do mesmo
  instante. Rótulos do período substituem os rótulos padrão de mesmo nome. Sem `schedule`
  nada muda (mesmos sorteios e eventos).

### Queue
- Capacidade opcional, FIFO ou LIFO. Libera cada item ao entrar.
- Estados: `empty`, `releasing`.
- Lote (batching), com tamanho-alvo, tempo máximo de espera e *flush*, fica para a fase 5.

### Processor
- Capacidade `n` (itens em paralelo), `setup` opcional, depois `process`.
- Estados: `idle`, `setup`, `processing`, `blocked` (terminou e o destino está fechado).
- Com `n > 1`, o estado reportado é o mais produtivo entre os itens presentes
  (processing > setup > blocked > idle).
- Publica `servers.busy` (média de itens em `setup` ou `processing`, zero com o bloco
  parado), `servers.blocked` (itens prontos que não saem) e `servers.capacity`. Com várias
  máquinas o estado `processing` (há pelo menos um) não serve de ocupação; estas médias
  servem. Os acumuladores zeram em `on_warmup`.

### Sink
- Aceita tudo, registra `time_in_system` (agora − criação) e destrói o item.

## 4b. Serviços do kernel (fase 1)

### Eventos canceláveis
`ctx.schedule(...)` devolve um `EventId`, e `ctx.cancel(id)` o descarta. O evento
continua no heap, mas é ignorado ao sair. Cancelar um evento que já disparou não faz nada.

### Quebras (MTBF/MTTR)
Qualquer objeto pode ter `downtime: (mtbf: Dist, mttr: Dist)`, contado no relógio.
- Na falha, o **kernel** fecha a entrada e a saída do bloco, mostra o estado `down` e
  chama `on_stop`. Blocos com timers (Processor, Source) cancelam os eventos e guardam
  o tempo restante.
- No reparo, o kernel restaura o último estado pedido pelo bloco e chama `on_resume`,
  que reagenda os timers com o tempo restante. Depois disso, os vizinhos são reavaliados.
- Durante a quebra, `ctx.set_state(...)` não sobrescreve `down`: o pedido fica guardado
  para o reparo.
- As quebras usam um *stream* aleatório próprio por bloco. Ligar ou desligar quebras
  não altera os tempos de chegada e de processo sorteados.

> **Nosso jeito:** o "estar quebrado" é responsabilidade única do kernel, e o bloco só pausa os
> próprios relógios.

### Turnos programados e motivos de parada
Um objeto pode ter `shifts: (period: P, off: [(start, duration), ...])`. As janelas se
repetem a cada `P`, a partir de t=0, e precisam estar em ordem, dentro do período e sem
sobreposição.

O kernel guarda **motivos de parada** (quebra, fora de turno). O bloco só volta a rodar
quando nenhum motivo está ativo. Na exibição, a quebra vence: o estado é `down` se há
quebra, senão `off_shift`. Os relógios do bloco pausam uma vez só, na primeira parada,
e retomam quando a última termina.

Janelas que se encostam (uma termina no instante em que a próxima começa, inclusive na
virada do período) são tratadas por **contagem de janelas abertas**, e não por um
sinalizador. Assim a ordem dos dois eventos simultâneos não importa.

> **Nosso jeito:** o MTBF das quebras corre no relógio também fora do turno. Uma opção
> "só conta tempo trabalhado" fica para depois.

**Fim de turno com `Finish`** (`shifts.on_end`, padrão `Pause`). Quando a janela começa e
o bloco está rodando, o kernel não para o bloco: marca-o como **fechando** (`closing`),
fecha as entradas (`find_destination` e `Inject` recusam o bloco) e o põe numa lista.
Depois de cada evento, o kernel pergunta a cada bloco da lista
`Behavior::work_in_progress`; quando não há mais trabalho, aplica o motivo de parada
`OFF_SHIFT` normalmente. O padrão de `work_in_progress` é "há item não liberado que não
espera transporte" (`Ctx::unreleased_count`); Queue e Rack respondem `false` (guardam,
não trabalham), o Combiner só durante preparação e processamento e o Transporter enquanto
tem uma viagem ou um posto em andamento. O `ShiftOn` continua no horário da tabela: se o
bloco ainda está fechando, ele só reabre as entradas. Um bloco fechando também não pega
trabalho novo por conta própria (`Ctx::is_closing`): o Transporter não começa a próxima
viagem e não se anuncia livre ao despachante, e o Dispatcher pula membros fechando ou fora
de turno com `Finish` (`Ctx::away_on_finish`). Com `Pause`, nada disso roda: a lista fica
vazia e os resultados são os mesmos de antes, byte a byte.

**Calendários.** `Model.calendars` é um mapa de nome para `ShiftTable`. O `shifts` de um
objeto é um `ShiftSpec`: a tabela no próprio objeto ou `Calendar("nome")` (opcionalmente
com o `on_end` do objeto). O modelo resolve a referência ao montar a simulação
(`Model::shift_table`), então o kernel só vê tabelas e os resultados são idênticos aos das
janelas escritas no objeto. O `ShiftSpec` tem leitura própria (o RON lido sem tipo perde o
nome antes dos parênteses), que aceita a tabela, `Calendar("x")`, `Calendar("x", Finish)`
e `(calendar: "x", on_end: ...)`.

### Medições de tempo entre dois pontos (`spans`)
O campo `spans: [(name, from, to)]` do modelo mede, por item, o tempo entre sair de `from`
e entrar em `to`. O kernel guarda o instante de saída no próprio item (`Item::marks`, por
índice de medição, em `Simulation::add_meter`) e, na chegada a `to`, registra a diferença
na tally `value` de um bloco `span` criado só para isso (depois dos objetos, fluxos e
fluidos). Fica no kernel, e não numa mensagem de assinatura, porque a `Sink` destrói o
item ao receber e a mensagem chegaria tarde demais. O relatório trata o bloco como
qualquer outro: média, IC, histograma e as métricas `<nome>.value.mean` e `.count`.

### Itens contêiner
`ctx.pack(contêiner, item)` tira o item do conteúdo do bloco e o coloca dentro do
contêiner (`item.container`, `contêiner.contents`). `ctx.unpack(contêiner)` devolve os
itens ao bloco atual. Um contêiner conta como **um** item no conteúdo. Destruir um
contêiner destrói o que está dentro. `ctx.clone_item` copia rótulos e o instante de criação.

## 4c. Rastro e replay (fase 2)
Com `enable_trace`, o kernel registra um `TraceEvent` para cada criação, movimento,
empacotamento, desempacotamento, destruição e mudança de estado, com o instante e na
ordem em que ocorrem. Um `Observer` recebe os mesmos eventos ao vivo.

- **Só leitura.** Observadores não alteram nada: rodar com ou sem rastro dá exatamente
  o mesmo resultado (há teste para isso).
- **Replay.** `utsim replay modelo.ron -o replay.json` grava uma replicação com o layout
  (posições e conexões) e o relatório. É a entrada do visualizador 3D.
- **Movimentos instantâneos.** As transferências não levam tempo na simulação. A animação
  de "andar" entre objetos é do visualizador, e transporte de verdade (operadores,
  esteiras) vem nas fases 4 e 5.

## 4d. Modelagem de dados de entrada (fase 1b)
`utsim fit dados.csv` faz, em ordem:
1. **Leitura**: CSV/TXT, com `;` e vírgula decimal, escolha de coluna e conversão de horários em intervalos.
2. **Diagnóstico**: estatística descritiva, teste de sequências, Ljung-Box, Mann-Kendall,
   valores atípicos (1,5·IQR) e empates, com avisos práticos.
3. **Ajuste**: 20 famílias por máxima verossimilhança. São 16 contínuas (4 delas com
   deslocamento) e 4 discretas. Família que não se aplica fica listada com o motivo.
4. **Aderência**: KS e Anderson-Darling com p-valor por **bootstrap paramétrico**
   (reajustando em cada réplica, porque os parâmetros vêm dos próprios dados), e
   qui-quadrado com classes equiprováveis.
5. **Escolha**: regra publicada em `rank.rs` e impressa em todo relatório.
   1. Rejeita quem tiver p < α em qualquer teste.
   2. Entre as aceitas, vence o menor AIC.
   3. Entre as equivalentes (ΔAIC < 2), vence a de menos parâmetros.
   4. Se todas forem rejeitadas, recomenda a distribuição empírica e gera a linha `.ron` dela.
6. **Relatório**: texto, HTML com gráficos (histograma com densidades, Q-Q, P-P, ECDF,
   dispersão x[i]×x[i+1]) e JSON. Cada candidata recebe uma frase que explica a sua
   posição no ranking.

> **Nosso jeito:** a parcimônia entre equivalentes é explícita. Nos dados de exemplo
> (Gama verdadeira), a Weibull deslocada, com 3 parâmetros, tinha AIC 0,3 menor. A regra
> escolhe a Gama, com 2, e diz por quê.

## 4e. Editor (fases 3 e 9)
O editor não altera o modelo diretamente: toda ação vira um `Command` (adicionar,
remover, renomear, mover, alterar campo, alterar aparência, ligar, desligar, lote)
aplicado por um `Document` (`utsim-edit`, sem dependência de interface gráfica).

- **Desfazer/refazer** por instantâneo do modelo, que é pequeno. Gestos contínuos, como
  arrastar um objeto, usam uma chave de agrupamento e viram um passo só.
- **Comando inválido não deixa rastro.** O documento continua igual, inclusive o
  histórico.
- **Validação ao vivo.** Os problemas aparecem enquanto o modelo é montado, cada um
  atribuído ao objeto correspondente. O modelo só roda quando não sobra nenhum.
- **Alterar campo** usa as mesmas chaves dos cenários (`objeto.campo`, `run.campo`).
  Editor, cenários e scripts falam a mesma língua.
- **Salvamento estável.** RON formatado de forma determinística e com os comentários do
  cabeçalho preservados. Comentários no meio do modelo se perdem ao salvar.

### Montar modelos sem editar arquivos (fase 9)

- **Biblioteca de objetos** (`utsim_edit::PALETTE`): cada tipo agrupado por categoria
  (fluxo de itens, montagem, manuseio de materiais, recursos móveis), com ícone e rótulo em
  português. Além dos tipos puros há predefinições: operador, empilhadeira e AGV são
  `Transporter` com o `visual` e as velocidades do papel. Coloca-se clicando no item e depois
  no chão (o fantasma mostra onde, Shift mantém a ferramenta) ou arrastando o item até o chão.
- **Grade.** Colocar e arrastar encaixam na grade quando "Encaixar na grade" está ligado; o
  passo é configurável e a grade desenhada segue o passo. As setas movem a seleção um passo.
- **Seleção múltipla.** Shift + clique põe ou tira um bloco; Shift + arrastar no chão desenha
  uma caixa e soma à seleção os blocos cujo centro projetado cai nela. O primeiro selecionado é
  o principal (contorno branco, os demais em azul). Arrastar um bloco da seleção move todos.
- **Tudo em um passo de desfazer.** O módulo puro `utsim_edit::arrange` devolve um único
  `Command::Batch` para cada operação em vários objetos: mover, excluir, alinhar (esquerda,
  centro, direita, topo, centro, base, pelos centros dos objetos), distribuir (os extremos
  ficam, os do meio ganham passos iguais) e editar uma propriedade comum (`fan_out` repete em
  cada selecionado do mesmo tipo os `Set` e `SetVisual` feitos no principal; renomear não se
  repete). O arrasto de uma seleção recalcula as posições a partir das iniciais e usa uma
  chave de agrupamento, como o arrasto de um bloco.
- **Área de transferência** (`arrange::Clipboard`, Ctrl+C, Ctrl+X, Ctrl+V, Ctrl+D):
  copia os objetos e só as ligações entre eles. Ao colar, cada cópia recebe um nome livre
  (`maquina` vira `maquina_2`, `maquina_2` vira `maquina_3`; depois de recortar, o nome
  original volta), é deslocada 2 m por colagem, e as referências entre objetos copiados
  (`transport`, `operators.team`, equipe de um despachante) passam a apontar para as cópias.
  Referências a objetos não copiados ficam, exceto na equipe de um despachante copiado, que
  perde os membros não copiados (um transportador só pertence a uma equipe). A área de
  transferência é interna ao editor, não a do sistema.
- **Aparência** (`Command::SetVisual`): a seção "Aparência" das propriedades escolhe o ativo
  da biblioteca embutida ou do catálogo de terceiros (`assets/terceiros/catalogo.ron`, com
  autor e licença na dica), cor, escala e giro, e "Restaurar padrão" apaga o `visual`. Os
  ajustes de um mesmo gesto (arrastar no seletor de cor) viram um passo de desfazer, que
  termina quando o botão do mouse é solto.
- **Atalhos.** A janela "Atalhos" (F1) lista a tabela `editing::SHORTCUTS`, a mesma testada.
- **Camadas** (`editing::Layer`): Objetos, Rede e Fluidos, com ferramentas, seleção e painel
  próprios (`layers.rs` no binário, `properties::network` na biblioteca). Tudo o que antes só se
  escrevia no `.ron` vira comando: `AddNode`, `MoveNode`, `RenameNode`, `RemoveNode` (leva as
  arestas), `AddEdge`, `SetEdge` (sentido e `capacity`), `RemoveEdge`, e para fluidos
  `AddFluidNode`, `MoveFluidNode`, `RenameFluidNode`, `SetFluidKind`, `RemoveFluidNode`,
  `AddFluidLink`, `RemoveFluidLink` (`utsim_edit::network` e `utsim_edit::fluids`). A rede (ou a
  rede de fluidos) nasce no primeiro nó e some quando fica vazia, então o arquivo salvo não muda à
  toa. Os comandos recusam só referências quebradas (nó inexistente, aresta repetida, laço); o
  resto (rede conexa, grafo acíclico, um `ToItems` por objeto, objeto do conversor) é validado ao
  vivo pelo `Model::validate`. Renomear um objeto atualiza os conversores que apontam para ele.
- **Rede no editor.** A ferramenta Aresta desenha caminhos clicando de nó em nó; um clique no chão
  cria um nó já ligado. O conector tracejado de cada objeto atendido vai até o nó mais próximo
  (`network::attachments`, mesma regra do roteador, empate para o nó listado primeiro). A base de
  um transportador é escolhida entre os nós: o editor move o transportador para o nó e liga
  `home` (sem campo novo no esquema).
- **Fluidos no editor.** Os nós aparecem em `Editar` com os níveis iniciais, desenhados pelo mesmo
  `fluid_scene.rs` do `Simular` (`editing::fluid_edit_view`). Se todos os nós estão na origem, o
  primeiro arrasto ou nó novo grava antes o layout automático no modelo, no mesmo passo de desfazer.
- **Avisos.** `Document::warnings` junta `Model::warnings` (só quando não há problemas) e os
  recursos móveis que não atendem ninguém; o painel os mostra em amarelo, separados dos problemas.

Lacunas: a área de transferência não passa entre janelas nem para outros programas; a seleção
por caixa usa o centro do bloco, não a área inteira; alinhar não considera o tamanho dos
blocos.

> **Nosso jeito:** o modelo continua sendo um texto legível, e o editor é só uma forma confortável de
> produzir esse texto.

## 4f. Recursos móveis (fase 4)
Um objeto com `transport: "empilhadeira"` não entrega o item direto ao próximo. A
sequência é:

1. Quando o destino aceita o item, o kernel **reserva o lugar** no destino. Itens a
   caminho contam como conteúdo, então um processador de capacidade 1 com uma peça
   chegando não aceita outra.
2. O item fica **no objeto de origem**, marcado como aguardando transporte (o
   Processor mostra `waiting_transport`).
3. O kernel envia uma tarefa `Transport` ao transportador ou ao despachante.
4. O transportador viaja até a origem (`travel_empty`), carrega (`loading`), apanha o
   item com `pick_up`, viaja (`travel_loaded`), descarrega (`unloading`) e entrega com
   `drop_off` na porta reservada.
5. As viagens seguem `Ctx::route` (linha reta, ou a rede de caminhos quando existir),
   com perfil de velocidade trapezoidal (aceleração e desaceleração opcionais). Cada
   trecho gera um `TraceEvent::Travel` para o 3D.

**Mensagens entre blocos.** Um hook só altera o próprio bloco. Tarefas (`on_task`),
mensagens (`on_message`), apanhar e entregar viram operações que o kernel executa logo
depois do hook, na ordem em que foram pedidas. Isso mantém o determinismo.

**Despachante.** Recebe tarefas e repassa a um membro livre da equipe. A escolha do
membro segue a regra: `FirstAvailable` (primeiro livre na ordem da equipe), `RoundRobin`
(rodízio) ou `Nearest` (o mais próximo de onde a tarefa começa). Se todos estão
ocupados, guarda na fila, por prioridade e, entre iguais, por ordem de chegada. Cada
membro informa a posição atual e avisa com `MSG_IDLE` quando fica livre.

**Transportador.**
- **Capacidade > 1:** leva junto as tarefas da fila que saem da mesma origem, até a
  capacidade. Carrega uma a uma e entrega na ordem, passando pelos destinos.
- **Tarefas do mesmo instante:** a partida acontece num evento de atraso zero, para que
  pedidos simultâneos (um lote liberado, por exemplo) possam dividir a viagem.
- **Quebras e turnos:** pausam viagem, carga e descarga. A retomada continua do ponto
  onde parou. O rastro registra o trecho restante, com posição linear no tempo, o que é
  aproximado quando há aceleração. Com `shifts.on_end: Finish`, o fim do turno espera a
  viagem (ou o trabalho no posto) em andamento terminar; a próxima tarefa da fila fica
  para o turno seguinte.
- **`home: true`:** ocioso, o transportador volta à sua posição de origem. Uma tarefa
  nova interrompe a volta e parte do ponto alcançado.

**Operadores.** Um processador com `operators: (team, count, setup, process)` pede
`count` tarefas `Operate { station }` à equipe quando a fase que precisa deles vai
começar. A fase só inicia quando todos chegam (`MSG_ARRIVED`). Liberação:

- Se setup e processo usam operadores, a mesma equipe é mantida do setup até o fim do
  processo.
- Se só o setup usa, eles são liberados ao fim do setup.
- A liberação acontece ao fim do processo, antes de a peça sair. Uma saída bloqueada
  não prende o operador.
- Cada peça tem seus próprios pedidos, então capacidade > 1 funciona.
- Numa quebra, os operadores ficam no posto e os tempos pausam.

Prioridade de estado: processando > setup > aguardando operador > aguardando
transporte/bloqueado > ocioso.

**Rede de caminhos.** Com `network: (nodes, edges)`, as viagens seguem o menor
trajeto (Dijkstra pré-calculado para todos os pares; arestas de mão única ou dupla).
- Um ponto fora da rede entra e sai pelo **nó mais próximo**, em linha reta. Empates vão
  para o nó listado primeiro.
- Entre caminhos de mesmo comprimento vence o que passa pelos nós listados primeiro, o
  que mantém o resultado determinístico.
- A validação recusa redes em que algum nó não alcança todos os outros (beco sem saída
  de mão única, rede partida). Laços de mão única são permitidos.
- O transportador só parte quando a tarefa existe. Ele não espera de antemão na origem.

> **Nosso jeito:** o transportador recebe só duas tarefas de alto
> nível, `Transport` e `Operate`, e decide os passos sozinho. É mais simples de
> modelar e de testar. Sequências genéricas podem vir com o fluxo de tokens (fase 6).

### Distâncias: tempo de viagem nas ligações diretas e avisos
Uma ligação direta entre dois objetos fixos é instantânea. Para que o
layout pese no resultado há três caminhos: `transport` (recurso móvel), `Conveyor` e
`transfer`, descrito aqui.

**`transfer: (speed, delay, path)`** em um objeto (ou `run.transfer` para o modelo todo;
o do objeto vence). Os itens que saem desse objeto por ligações diretas levam
`delay + distância / speed` para chegar. A distância vem da linha reta entre as
`position` (`path: Straight`, o padrão) ou do menor trajeto da `network`
(`path: Network`, o mesmo roteador dos transportadores). Sem `transfer` nada muda: os
modelos antigos e os resultados de referência ficam idênticos byte a byte.

- **Saída e chegada.** O kernel reaproveita as duas metades de um movimento (`leave` e `arrive`): no instante em
  que o destino aceita, o item sai da origem (`leave`: estatísticas, `on_exit`, nova
  tentativa dos blocos a montante) e um evento `Arrive` agendado o coloca no destino
  (`arrive`: contagem de entrada, `on_entry`, `Moved`). Se o tempo total for zero, o
  movimento é imediato.
- **Capacidade.** Como no transporte, o lugar no destino é reservado na saída
  (`incoming`), então a capacidade nunca é ultrapassada. Um processador de capacidade 1
  fica ocupado desde que o item parte e não aceita outro durante a viagem.
- **Contagens.** Durante a viagem o item não pertence a nenhum objeto (`location` vazio):
  o conteúdo da origem já o descontou e o do destino ainda não o inclui. A entrada do
  destino conta no instante da chegada. Para a lei de Little, o número em trânsito é
  `vazão × tempo de viagem`, somado ao conteúdo dos objetos.
- **Rastro.** `TraceEvent::Transit { item, from, to, delay, duration, path }` na saída e
  o `Moved` de sempre na chegada. A linha do tempo de `utsim-scene` mostra o item em
  `ItemPos::Flight` (parado na origem durante `delay`, depois em velocidade constante ao
  longo do caminho), fora do conteúdo dos dois objetos.
- **Isenções do padrão global.** `run.transfer` não vale para objetos com `transport`,
  recursos móveis, esteiras nem para quem alimenta uma esteira (a posição de uma esteira
  é o seu meio). `transfer` explícito em objeto com `transport` é erro de validação.
- **Experimentos.** `obj.transfer.speed`, `obj.transfer.delay`, `run.transfer.speed` e
  `run.transfer.delay` (e `transfer` inteiro) aceitam `set`.

**Avisos.** `Model::warnings()` devolve alertas que não impedem a execução; `utsim check`
e `utsim run` os mostram como `aviso: ...` (em stderr). Hoje cobrem: recursos móveis com
todos os objetos na origem; transporte ou equipe atendendo objetos na mesma posição; objeto
atendido a mais de 50 m de qualquer nó da rede; e ligação direta de mais de 10 m sem
transporte, esteira nem `transfer`.

## 4g. Manuseio de materiais (fase 5)

### Esteira (`Conveyor`)
`length` L, `speed` v, `item_length` s (espaço que cada item ocupa) e `accumulating`.
A esteira é cinemática: nada de física, só eventos calculados.

- **Capacidade:** C = ⌊L / s⌋. A validação exige L ≥ s.
- **Entrada:** aceita um item se tem menos de C itens **e** o último que entrou já
  andou pelo menos s, ou seja, passou s/v desde a entrada dele. Quando a entrada fecha só
  pelo espaçamento, a esteira agenda a reabertura e avisa os vizinhos de entrada
  (`Ctx::wake_inputs`).
- **Esteira com acúmulo** (padrão): o item i chega ao fim em
  `e_i = max(t_entrada_i + L/v, s_{i-1} + s/v)`, onde `s_{i-1}` é o instante em que o
  item da frente saiu. Se a saída está bloqueada, o da frente espera no fim e os de trás
  param encostados, um a cada s. Quando ele sai, o próximo percorre só o espaço s.
- **Esteira sem acúmulo:** enquanto o item da frente espera no fim, a correia inteira
  para. Todos os itens atrasam exatamente o tempo parado.
  A correia parada ainda recebe um item na entrada se houver espaço livre ali, como uma
  peça colocada sobre uma esteira desligada. O item fica parado na posição 0 até a
  correia voltar a andar.
- **Estados:** `empty`, `releasing` (movendo) e `blocked` (frente esperando).
- **Quebras e turnos:** a correia para e retoma do ponto onde estava.
- **Junções e desvios:** usam os blocos que já existem (filas, regras de saída). Fotocélulas
  e transferências dedicadas ficam para depois.

### Porta-paletes (`Rack`)
`bays` (colunas) × `levels` (níveis). A capacidade é bays·levels.

- **Posição:** cada item recebe uma posição (`slot`) pela regra `placement`.
  `FirstFree` preenche nível por nível, coluna por coluna. `Random` sorteia entre as
  livres, com o stream do bloco.
- **Saída:** o item fica pelo menos `dwell` (opcional) e depois é liberado, por ordem de
  prontidão (FIFO).
- **Rastro:** a posição vai para `TraceEvent::Slot { item, block, slot }`, e o 3D
  desenha o item na posição certa da estrutura (`bay_width`, `level_height`).
- **Estados:** `empty` e `releasing`.

## 4h. Lógica do usuário (fase 6)

### Fluxo de tokens (`flows`)
Um fluxo é um grafo de atividades por onde passam **tokens**. Tokens não são itens:
são "linhas de raciocínio" leves, que esperam, decidem, seguram recursos e podem
carregar a referência de um item. Cada fluxo roda como um bloco da simulação. Ele não
aparece no 3D, mas aparece nos relatórios e no rastro.

| Atividade | O que faz |
|---|---|
| `Source` / `Schedule` | cria tokens por intervalo aleatório ou em horários fixos |
| `OnEvent` | cria um token, carregando o item, a cada entrada ou saída num objeto |
| `Delay` | segura o token por um tempo sorteado |
| `Decide` | escolhe o próximo ramo: por probabilidade, pelo rótulo (1, 2, ...) ou pela primeira condição verdadeira |
| `Assign` | atribui rótulos ao token (`nome`) ou ao item carregado (`item.nome`) a partir de expressões |
| `Acquire` / `Release` | recursos de contagem compartilhados; quem espera forma fila FIFO |
| `WaitFor` | espera o próximo evento de entrada ou saída num objeto; o token passa a carregar esse item |
| `CreateItem` | cria um item dentro de um objeto (que precisa aceitá-lo) |
| `Batch` | junta *n* tokens e segue com um |
| `Sink` | encerra o token |

**Como o fluxo enxerga os objetos.**
- **Assinatura:** o fluxo pede para ser avisado (`Ctx::subscribe`) quando itens entram ou
  saem de um objeto, e recebe as mensagens `MSG_ITEM_ENTERED` e `MSG_ITEM_EXITED` com a
  chave do item.
- **Criação de itens:** `Ctx::inject_item` cria um item e o entrega ao objeto logo depois
  do hook. Se o objeto não aceitar, é erro de modelagem.
- **Rastro:** o número de tokens em cada atividade vai para `TraceEvent::Tokens`, que a
  visualização do fluxo usa.

### Scripts (`on_entry`, `OutputRule::Script`, expressões)
O kernel não conhece nenhuma linguagem. Ele só chama `ItemScript` (na entrada de um
item, **antes** do `on_entry` do bloco) e `PortScript` (para escolher a porta de
saída). O modelo compila os scripts, em Rhai, pelo crate `utsim-script`. Os scripts
usam o stream aleatório do próprio bloco, então a simulação continua reprodutível.
Erros de compilação aparecem na validação do modelo; erros durante a execução param a
rodada com uma mensagem.

> **Nosso jeito:** a lógica fica em dois lugares claros: o fluxo de tokens e
> poucos ganchos de script, numa linguagem embutível e livre.

## 4i. Otimização de parâmetros (fase 7)
Um problema de otimização (`utsim-opt`) reúne:
- **Modelo** e número de **replicações por ponto**.
- **Variáveis de decisão,** com as mesmas chaves dos cenários (`objeto.campo`, `run.campo`).
  Podem ser inteiras (com passo), contínuas (com um modelo de texto, ex.:
  `"Exp(mean: {x})"`) ou uma escolha entre valores.
- **Objetivo:** Σ peso·métrica + Σ custo·valor da variável, a minimizar ou maximizar.
- **Restrições** sobre a média de métricas.

**Números aleatórios comuns.** Todos os pontos usam as mesmas sementes. A diferença
entre dois pontos vem da decisão, não da sorte, e por isso a comparação é **pareada**:
o intervalo de 95% da diferença por replicação diz se o melhor ponto é de fato melhor
que cada um dos outros. Pontos que não se distinguem do melhor aparecem como
equivalentes, com a recomendação de mais replicações para separá-los.

**Algoritmos.** Todos usam um cache, então nenhum ponto é simulado duas vezes, e o
`budget` conta pontos distintos.
- `Grid`: todas as combinações.
- `Annealing { iterations }`: recozimento simulado. Começa no meio de cada domínio; o
  vizinho muda uma variável em um passo; aceitação de Metropolis. A temperatura inicial
  vem da variação entre os primeiros vizinhos e cai geometricamente até 1/1000 dela.
- `Genetic { population, generations }`: os genes são índices nos valores de cada
  variável; seleção por torneio de 2, cruzamento uniforme, mutação de 1/n por gene, um
  elite por geração.

Pontos que violam restrições recebem uma penalidade que os deixa piores que qualquer
ponto viável já visto. Entre dois pontos inviáveis, o mais próximo da viabilidade vence.

**Refinamento (`refine: (top, replications)`).** Depois da busca, os `top` melhores pontos
viáveis são simulados de novo com mais replicações e com as mesmas sementes entre si. A
comparação final (`best`, `equivalent`) usa essas avaliações refinadas. O resultado
também guarda o histórico de convergência (`history`).

> **Nosso jeito:** o otimizador é aberto, e o relatório mostra a significância estatística das
> diferenças em vez de entregar um "melhor" sem contexto.

## 4j. UTSim no navegador (fase 7, WebAssembly)

- As crates de biblioteca (`core`, `objects`, `script`, `flow`, `model`, `fit`) compilam para
  `wasm32-unknown-unknown`. Nenhuma usa relógio, sistema de arquivos nem entropia do SO nas
  rotas de simulação: só fluxos PCG com semente (`rand` sem `os_rng`/`thread_rng`).
- `rayon` é opcional (`utsim-model`, feature `parallel`, ligada por padrão). Sem ela,
  `run_replications` roda em sequência, com os mesmos resultados e a mesma ordem.
- `crates/utsim-web` (cdylib + rlib) expõe `run_model`, `validate_model`, `fit_data`, `replay`
  e `examples`, todas como texto em/texto out (JSON ou HTML). A lógica fica em `api.rs`
  (Rust puro, testado nativamente); `lib.rs` só embrulha com `wasm-bindgen`. A crate compila
  nativamente também e faz parte do `default-members`.
- `web/` é a página (HTML+CSS+JS puros). `bash scripts/web.sh` (ou `web.ps1`) gera
  `web/dist/` (ignorado pelo controle de versão) com `utsim.js`, `utsim_bg.wasm` e o
  `utsim.html` autocontido (wasm em base64, ~5,5 MB). A simulação roda na thread principal.
  Para abrir sem servidor, use `web/dist/utsim.html`; para `web/index.html`, sirva a pasta
  `web/` (`python3 -m http.server -d web`).

- **Visualizador 3D no navegador (RF-109).** `utsim-viewer` também compila para
  `wasm32-unknown-unknown` (binário `utsim-view`, ponto de entrada `src/web.rs`) com o Bevy em
  **WebGL2** (feature `webgl2`; escolhido em vez de WebGPU por rodar em todos os navegadores
  atuais). É somente leitura (modo Simular): a página grava um replay com `replay(ron, stop)`
  do `utsim-web` e o entrega ao visualizador. Fica em `cfg(not(target_arch = "wasm32"))`: a
  linha de comando (`clap`), o modo roteirizado (`script`/`roteiro`), arquivos e diálogos, a
  saída do processo e a thread da simulação rápida (no navegador o editor, se um dia for
  aberto, simula na própria thread). `scripts/web.sh` (ou `web.ps1`; `--sem-3d`/`-Sem3d` pula)
  gera `web/dist/viewer/utsim_viewer.js` + `utsim_viewer_bg.wasm`, que **não** entram no
  `utsim.html` autocontido (a aba 3D mostra uma nota). `web/viewer.html` roda num `<iframe>`
  (uma instância nova por execução, pois um app Bevy não reinicia na mesma página) e recebe o
  replay por `postMessage`. Só funciona servido por HTTP: `python3 -m http.server -d web`.

## 4k. Execução passo a passo, comandos externos e decisões (fase 7)
`run()` é apenas um laço sobre a API passo a passo. Os resultados são idênticos, e os
snapshots golden comprovam isso.
- `run_until(t)` processa eventos até `t` e devolve `Paused(t)`, `Decision(d)` ou
  `Finished`. `finish()` fecha a rodada e devolve o relatório.
- **Comandos externos** (emulação): `stop_block` e `resume_block` usam o mesmo mecanismo
  de motivos de parada das quebras e dos turnos. O estado mostrado é `stopped`.
  `block_status()` dá a visão ao vivo de cada bloco.
- **Decisões:** um bloco chama `Ctx::request_decision(opções, observação)`. A rodada pausa
  depois do evento corrente. O agente responde com `decide(escolha)`, e a resposta chega
  ao bloco como `MSG_DECISION`. No fluxo de tokens, isso é o `Decide` com regra `Agent`.
  Sem agente, um modelo que pede decisões dá erro em `run()`. Com `run.default_decision`
  (ex.: `Some(0)`), `Model::drive` responde sempre essa opção (limitada ao número de
  opções), e o modelo roda em `utsim run`, no replay e no 3D.
- **Métricas próprias:** um bloco pode expor métricas (`custom_metrics`) e reiniciá-las no
  fim do aquecimento (`on_warmup`).

## 4n. Aprendizado por reforço (fase 7)
- **Agente no fluxo.** `Decide` com regra `Agent { observation }` avalia as expressões de
  `observation` (números), chama `Ctx::request_decision(ramos, observação)` e **estaciona**
  o token numa fila FIFO do bloco de fluxo. A resposta (`MSG_DECISION`, payload = ramo)
  libera o token mais antigo para o ramo escolhido. O kernel responde às decisões na
  ordem em que foram pedidas, então várias decisões no mesmo instante também casam.
  A decisão leva tempo zero de simulação. Para que o agente mande um item para um lado,
  o fluxo costuma **criar** o item depois da escolha (`CreateItem` em cada ramo).
- **Ambiente** (`utsim-env`, `Env`): `reset(semente)` roda até a primeira decisão e
  `step(ação)` responde e roda até a próxima ou até o fim do episódio
  (`(observação, recompensa, done, info)`). O aquecimento é zerado, e `episode_length`
  substitui o tempo de parada.
- **Observação:** o vetor da decisão, seguido do conteúdo de blocos (`observe_blocks`;
  `"a+b"` soma os conteúdos) e de métricas ao vivo (`observe_metrics`). As expressões do
  fluxo só veem rótulos e `now`, por isso o estado físico vem do ambiente.
- **Recompensa:** Σ peso·Δmétrica entre decisões consecutivas, menos `decision_cost`. As
  métricas vêm de `Simulation::report()` (chaves de `RunReport::metrics`, ~20 µs por passo
  num modelo de 7 blocos). Um termo `time_weighted` multiplica a média no tempo pelo tempo
  decorrido, ou seja, usa a integral. Com peso −1 nos conteúdos dos blocos, o retorno do
  episódio é −Σ(tempos no sistema) (lei de Little): minimizar o atraso total.
- **Linha de base:** Q-learning tabular (`utsim_env::qlearning`) com faixas configuráveis,
  ε-guloso e taxa de aprendizado que decai com as visitas. `examples/roteamento_rl.ron`
  (linha rápida de 1 máquina × linha lenta de 2) mostra o ganho sobre a política aleatória.

## 4l. Fluidos (fase 7)
Uma rede de fluidos (`fluids`) é um bloco só: tanques, tubos, fontes, sumidouros e
conversores de/para itens discretos, ligados num grafo **acíclico**.

- **Taxas constantes entre eventos.** Os níveis variam linearmente, então o próximo
  instante relevante é calculado exatamente: tanque esvaziando ou enchendo, conversor
  completando um volume. O kernel agenda esse instante, e nele todas as taxas são
  recalculadas.
- **Cálculo das taxas.**
  1. Cada fonte oferece até a sua taxa.
  2. Um tanque com nível > 0 oferece até `max_out`. Um tanque cheio só aceita o que
     consegue escoar.
  3. Tubos e sumidouros limitam a vazão.
  4. Várias saídas de um nó dividem igualmente o que ele oferece; o que uma saída
     recusar volta para as outras.
  5. Conservação: o que sai de um nó é igual ao que entra, menos a variação do nível.
- **Itens → fluido** (`FromItems`): cada item que entra no objeto discreto vira um
  volume, entregue de uma vez ao tanque seguinte.
- **Fluido → itens** (`ToItems`): o nó acumula o que recebe e, a cada volume completo,
  cria um item no objeto discreto.
- **Rastro:** `TraceEvent::Fluid { node, level, rate }`, para o 3D.
- **Métricas:** `custom_metrics` traz o nível médio e máximo de cada tanque e o volume
  total de cada nó.
- **No visualizador 3D** (`utsim-view`; em `Editar` a camada Fluidos edita os nós, ver 4e).
  O `Replay` carrega o `FluidDef`; o bloco "Fluid" não vira caixa, e a rede é desenhada
  por `fluids.rs` (layout, formas) e `fluid_scene.rs` (cena).
  - **Layout:** as posições dos nós vêm do modelo. Se todas forem zero, os nós são
    dispostos em camadas ao longo das ligações, numa faixa abaixo dos blocos discretos.
  - **Tanque:** cilindro translúcido com o líquido (azul) dentro. A altura do líquido é
    `nível / capacidade`. O nível no tempo `t` vem de `Timeline::fluid_at`, que
    interpola `nível + taxa · (t − t_evento)`, limitado a `[0, capacidade]`, a partir do
    último `TraceEvent::Fluid` do nó.
  - **Outros nós:** fonte (cone verde), sumidouro (cilindro vermelho), tubulação
    (cilindro laranja deitado) e conversor (cubo roxo).
  - **Ligações:** setas em ciano quando há vazão no instante (a dos nós que não são
    tanque, ou a variação dos tanques), cinza-azulado quando parada. Cada conversor tem
    uma linha tracejada roxa até o seu objeto discreto.
  - **Seleção:** clicar num tanque ou escolhê-lo na lista "Fluidos" mostra nível,
    capacidade, ocupação e taxa no tempo atual. Roteiro de conferência:
    `crates/utsim-viewer/roteiros/fluidos_tanques.ron`.

## 4m. Tráfego de AGVs (fase 7)
Uma aresta da rede pode ter `capacity` (veículos ao mesmo tempo). Um transportador com
`agv: true` segue a rede aresta por aresta:
1. Reserva a próxima aresta antes de entrar nela.
2. Se a aresta está cheia, espera parado, em fila FIFO por aresta, com estado `blocked`.
3. Libera a aresta ao sair dela.

Se todos os veículos estão esperando uns pelos outros (impasse), a rodada para com uma
mensagem que lista o ciclo de espera.

Regras adotadas:
- **Arestas controladas:** só as que têm `capacity`. Uma aresta de mão dupla é um recurso
  só, com a capacidade dividida entre os dois sentidos.
- **Posse da aresta:** o veículo segura a aresta atual até conseguir a próxima, e é isso
  que gera bloqueios realistas. As ligações entre um bloco e a rede não são controladas.
- **Fila por aresta:** a espera é FIFO, em ordem de chegada. Ao liberar, o primeiro da
  fila já vira o dono da aresta, de modo que ninguém fura a fila.
- **Velocidade:** o perfil de velocidade é o da rota inteira, dividido entre as arestas.
  A espera congela o perfil (aproximação: não há custo extra de parar e arrancar).
- **Quebra:** pausa o trecho atual e mantém a aresta.
- **Impasse:** é detectado quando um veículo é barrado. O sistema procura o maior grupo
  de veículos parados em que os donos das arestas desejadas também estão parados
  (funciona com capacidade > 1). É reportado uma única vez.

No visualizador 3D, os AGVs usam o mesmo caminho dos demais transportadores: andam pelos
eventos `Travel` e, quando esperam numa aresta cheia, ficam parados na última posição com a
cor do estado `blocked` (laranja). As arestas controladas (`capacity`) aparecem em vermelho,
como linha dupla, com o rótulo "capacidade N" no meio, tanto em `Editar` quanto em `Simular`.
Roteiro de conferência: `crates/utsim-viewer/roteiros/agvs.ron`.

## 4o. Emulação com OPC UA (fase 7)
Comissionamento virtual: o modelo roda contra o relógio de parede e conversa com um CLP
(real ou simulado) por OPC UA (`utsim-emu`, `utsim emular`).
- **Ritmo:** a cada tick (50 ms) o `Pacer` avança a simulação com `run_until` até o tempo
  que corresponde ao relógio × velocidade. Uma unidade de tempo do modelo vale 1 s na
  velocidade 1.0.
- **Um só thread:** tudo o que vem de fora é um `Input` (parar/retomar bloco, mudar a
  velocidade), aplicado entre dois `run_until`, no tempo simulado corrente. Por isso a
  rodada é determinística para a mesma sequência de comandos.
- **Log de comandos:** gravar `(tempo, variável, valor)` basta para reproduzir a rodada
  sem rede e sem esperar o relógio (`--reproduzir`): o relatório sai igual.
- **Servidor:** `async-opcua`, namespace `urn:utsim:emulacao`, nós por bloco (`Conteudo`,
  `Estado`, `Entradas`, `Saidas`, `Parar`) atualizados a cada tick. Parar usa
  `stop_block` (motivo `EXTERNAL`, estado `stopped`). Endpoint sem segurança: só em rede
  confiável.
- **Cena ao vivo junto:** `--cena-porta` liga o servidor de cena (seção 4p) no mesmo
  `run_paced`. O OPC UA e a cena compartilham o relógio e o tempo simulado, então um CLP e
  um cliente de realidade virtual veem o mesmo instante. A trait `Frontend` ganhou
  `wants_trace`/`trace`: o `SceneFrontend` embrulha o frontend OPC UA, recebe os registros
  do rastro (`Emulator::new_traced`, `drain_trace`) e devolve ao `poll` os comandos que os
  clientes de cena mandam.

## 4p. Cena 3D independente de motor e preparo para realidade virtual
A simulação gera um rastro. A crate `utsim-scene` transforma esse rastro no que um motor
gráfico precisa desenhar em cada instante. Todos os clientes consomem a mesma cena, então
nunca discordam sobre onde as coisas estão:
- o visualizador Bevy;
- a página web;
- um modo de realidade virtual;
- o cliente Godot (3D, realidade virtual e aumentada).

- **Cena estática** (`StaticScene`), enviada uma vez: blocos, recursos móveis, nós de
  fluido, trechos da rede e o `visual` de cada objeto.
- **Quadros** (`Frame`): para cada instante, a pose (`Pose`) de cada entidade, com
  posição, direção, altura da carga (`lift`, por exemplo os garfos da empilhadeira),
  objeto onde está apoiada (`parent`), estado e nível dos tanques.
- **Movimento semântico** (`Motion`): parado, andando (com velocidade), carregando,
  descarregando, trabalhando, bloqueado ou parado por quebra. Cada motor associa o
  movimento aos seus próprios clipes de animação. Uma pessoa andando a 1,2 m/s toca o
  ciclo de caminhada nessa velocidade.
- **Papéis dos móveis** (`MobileRole`): pessoa, empilhadeira, AGV ou veículo genérico.
  O papel escolhe a família de animações.
- **Convenções:**
  - metros, eixo Z para cima, as coordenadas de planta do próprio modelo;
  - escala real, para uma câmera de realidade virtual na altura dos olhos (~1,6 m) ver a
    planta no tamanho certo;
  - Bevy converte `(x, y, z)` em `(x, z, -y)`;
  - Godot usa a mesma conversão do Bevy: `(x, z, -y)`, em metros.
- **Fluxo de cena** (protocolo `utsim-scene/1`):
  - formato JSON em linhas: um cabeçalho (`Header`) e depois um quadro por linha;
  - `SceneSink` abstrai o destino: arquivo, socket, runtime de realidade virtual ou teste;
  - `utsim cena` grava o fluxo de uma rodada, e o servidor ao vivo (abaixo) serve o mesmo
    fluxo durante uma emulação.
- **Aparência** (`visual` no objeto): um item da biblioteca embutida ou um `.glb`, com
  escala, giro e cor. É só apresentação: a simulação nunca lê esse campo.

### Câmeras e níveis de detalhe do visualizador

O visualizador Bevy é um cliente da cena: ele só lê. Duas peças dele importam para um
modo de realidade virtual e para modelos grandes.

**Câmeras** (`camera_modes.rs`; regras puras em `utsim_viewer::cameras`, com testes):

- Cada modo é um `CameraController`: recebe a entrada do quadro (`CamInput`: mouse, roda, teclas)
  e o que a cena sabe (`CamWorld`: o veículo seguido como está desenhado, as pegadas dos blocos
  para a colisão) e devolve a pose da câmera. O sistema `drive` escolhe o controlador ativo,
  desliza da pose antiga para a nova em 0,7 s quando o modo muda e grava o `Transform` da câmera.
  Nada mais no visualizador lê a câmera senão por esse `Transform`.
- Modos: **Órbita** (o de sempre), **Seguir** (por trás e por cima do recurso, com suavização
  da posição e da direção), **Primeira pessoa** (a 1,6 m do chão, `EYE_HEIGHT`, escala real,
  com colisão simples contra as pegadas dos blocos) e **Do recurso** (no assento do motorista,
  `driver_seat` por papel). Cada controlador guarda o próprio estado (distância, velocidade, giro),
  então voltar a um modo o encontra como foi deixado.
- **Ponto de extensão para realidade virtual:** uma câmera de headset é mais um controlador, com
  a pose vinda do XR em vez do mouse, mais uma variante de `CameraKind` e um campo em
  `CameraModes`. O restante (escala real, altura dos olhos, a cena em metros) já vale.
- O recurso seguido é lido do `Transform` da entidade desenhada (a pose do modelo realista,
  que já inclui a aproximação do porta-paletes), e o `drive` roda depois da animação dos
  modelos, para a câmera não ficar um quadro atrás.

**Níveis de detalhe** (`lod.rs`; regras e limiares em `utsim_viewer::lod_rules`):

- O sistema `apply` mede a distância de cada entidade até a câmera e troca as distantes por
  algo mais barato, achando-as pelos componentes que a cena já põe (`Realistic` nos blocos,
  `ItemVisual` nos itens). Cria um filho de substituição por entidade (caixa do tamanho do bloco,
  cubo do item), uma vez, e só liga e desliga a visibilidade depois.
- Degraus: bloco realista, caixa; item realista (palete com caixas), cubo, e além de
  `item_hide_m` nada. Rótulos: só os `label_max` mais próximos (150) e nenhum além de
  `label_hide_m`. Os limiares ficam em `LodSettings` (padrão: 45 m, 30 m, 140 m, 90 m, 150) e
  têm histerese de 10 %, para um objeto na fronteira não piscar.
- O LOD só age num modelo pesado (`min_blocks` 100 blocos realistas ou `min_items` 400 itens
  desenhados): uma planta pequena é vista inteira, de qualquer distância.
- Os itens já compartilham a malha e o material (cubo simples: um material por origem; modelo
  realista: as mesmas cenas glTF e uma cópia de material por cor), e o Bevy agrupa os
  desenhos iguais, então não houve necessidade de instâncias próprias.
- `F3` mostra tempo de quadro, entidades e a contagem por nível; `F4` liga e desliga o LOD.
  Medições em `docs/desempenho.md`, seção "Visualizador 3D".

### Cena ao vivo (socket)

`utsim emular modelo.ron --cena-porta 7878` serve a cena enquanto emula (OPC UA e cena no
mesmo relógio). `utsim cena modelo.ron --servir 7878 --velocidade 5` faz o mesmo sem OPC UA,
para desenvolver clientes. Opções: `--cena-endereco` (padrão `127.0.0.1`) e `--cena-fps`
(quadros por segundo de relógio, padrão 20).

- **Uma porta, dois transportes:** quem abre com `GET` (navegador, biblioteca WebSocket) é
  promovido a WebSocket e recebe um JSON por mensagem de texto. Os demais são TCP puro e
  recebem um JSON por linha, os mesmos bytes do arquivo de `utsim cena`. A primeira
  mensagem é o `Header`, depois os quadros. Vários clientes ao mesmo tempo.
- **Cena incremental** (`LiveScene`, `utsim-scene/src/live.rs`): guarda o replay estático e
  acrescenta os registros que chegam (`push`); `frame(t)` reindexa de forma preguiçosa, só
  quando chegaram registros, com o mesmo código do `Scene::build`. Logo, depois que todos os
  registros entraram, o quadro é idêntico ao do lote (testado em três exemplos, em muitos
  instantes). O que o ao vivo não pode saber é o futuro: a duração do trânsito visual entre
  blocos fixos é fixa (`LIVE_TRANSIT`, 0,4) em vez de automática, e o progresso de uma janela
  de carga/descarga só fecha quando o próximo evento chega. Um quadro até 0,8 unidade antes
  do registro mais novo já é idêntico ao do lote em modelos sem transportadores. O custo de
  cada reindexação é linear no número de registros, fora do fio da simulação.
- **Fios:** o fio da simulação só acrescenta registros a uma fila e grava o tempo (sem
  esperar ninguém). Um fio de quadros reindexa, renderiza e serializa o quadro uma vez por
  período; um fio aceita conexões; cada cliente tem um fio que escreve e lê comandos.
- **Contrapressão:** cada cliente tem uma fila limitada (4 mensagens). Se estiver cheia, a
  mensagem é descartada só para aquele cliente (contador `dropped`); um cliente lento ou
  travado vê menos quadros, e a simulação e os outros clientes não percebem. Um socket que
  não aceita dados por 1 s é desconectado. Sem clientes, a cena nem é reindexada.
- **Mensagens de extensão** (campo `"tipo"`; cabeçalho e quadros nunca o têm, então arquivos
  não mudam, e leitores devem ignorar tipos desconhecidos):
  - cliente para servidor: `{"tipo":"comando","variavel":"M1.Parar","valor":true}`, com as
    variáveis do OPC UA (`<bloco>.Parar` booleano, `Velocidade` número). Entra no mesmo
    `Input` da emulação e no log de comandos; o efeito aparece nos quadros seguintes
    (`state` do bloco vira `stopped`);
  - servidor para cliente: `{"tipo":"aviso","mensagem":"..."}` (JSON inválido, comando
    recusado) e `{"tipo":"fim","tempo":T}` ao fim da rodada.
- **Segurança:** não há autenticação nem criptografia, e a origem do WebSocket não é
  verificada. Quem alcança a porta vê a planta e pode parar blocos. Por isso o padrão é
  escutar só em `127.0.0.1`; `--cena-endereco 0.0.0.0` abre para a rede e imprime um aviso.
  Use só em rede local confiável.
- **Código:** `utsim-scene` com a feature `server` (só nativo; sem ela a crate compila para
  wasm32). Clientes de teste: `crates/utsim-scene/examples/cliente_cena.rs` (TCP) e
  `web/cena_ao_vivo.html` (WebSocket, vista de topo).

### Como a cena é derivada do rastro

`Scene::build(replay, opções)` indexa o replay uma vez. As opções trazem a duração visual das
transferências e, quando existe, o modelo que gerou o replay (tamanhos de esteiras e
porta-paletes, rede de caminhos, `visual` e parâmetros dos transportadores, casados pelo nome).
Os módulos puros que vieram do visualizador (`timeline`, `layout`, `geometry`, `handling`,
`fluids`, `style`, `network`, `tokens`) trabalham no espaço de layout do Bevy (Y para cima).
A saída (`static_scene`, `frame`) converte tudo para as coordenadas do modelo com
`layout::to_model`. O visualizador lê as mesmas funções (`base_at`, `items_at`), então só existe
uma implementação.

- **Papel do transportador**, vale a primeira regra que casar:
  1. `visual.asset` contém "empilhadeira" ou "forklift": empilhadeira; "agv": AGV;
     "operador", "pessoa" ou "person": pessoa;
  2. `agv: true`: AGV;
  3. serve um porta-paletes (um objeto cujo `transport` é ele, ou um despachante cuja equipe o
     inclui, é um `Rack` ou está ligado a um): empilhadeira;
  4. é operador (ele, ou um despachante com ele na equipe, é o `operators.team` de um
     processo): pessoa;
  5. senão, veículo genérico. Um replay `.json` sem o modelo só tem veículos genéricos.
- **Posição**: o fim do último trecho `Travel`, interpolado linearmente no tempo ao longo do
  caminho pelo comprimento de arco. Antes do primeiro trecho, a posição do layout. As quinas
  do caminho são arredondadas (módulo `turning`, função pura): nos últimos e primeiros 0,5 m
  em volta de cada vértice (menos em pedaços curtos, nunca mais que metade de um pedaço) a
  posição segue um arco tangente aos dois pedaços. O parâmetro continua sendo o comprimento
  do caminho original, então o trecho começa e termina onde e quando o rastro diz, e os
  pedaços retos não mudam. A quina entre dois trechos percorridos sem parar (os pedaços de
  um AGV) também é arredondada, metade em cada trecho. Quase meia-volta (acima de 170 graus)
  vira giro no lugar. Uma empilhadeira carregando ou descarregando num porta-paletes vai para
  o corredor na frente da célula, de frente para ela (`compose::rack_approach`, nos primeiros
  e últimos 15 % da janela), com o centro dos garfos (`FORK_REACH`, 1,555 m à frente) sob a
  célula.
- **Direção (`yaw`)**: a do pedaço de caminho percorrido, girando a taxa constante nas quinas
  arredondadas (contínua em posição e direção). Parado, entre trechos ou quebrado, mantém a
  direção do fim do último trecho com comprimento.
- **Distância percorrida** (`Scene::distance_at`): o comprimento de caminho andado até `t`,
  só dos trechos. Rodas e ciclo de passos dependem só de `t`, então saltar no tempo dá a mesma
  pose que chegar lá reproduzindo.
- **Movimento**, vale a primeira regra:
  1. `Stopped` nos estados `down`, `off_shift` e `stopped`;
  2. `Moving` enquanto percorre um trecho com comprimento;
  3. `Loading` e `Unloading` nesses estados;
  4. `Working` no estado `utilize` (operador num posto);
  5. `Blocked` no estado `blocked`;
  6. senão, `Idle`.

  Blocos fixos: `Working` em `processing` e `setup`, `Blocked` e `Stopped` como acima.
- **Velocidade** em `Moving`: comprimento do trecho dividido pela duração. Se o modelo dá
  aceleração ou desaceleração e o perfil trapezoidal do comprimento do trecho dura o mesmo que
  o trecho (tolerância de 0,5 %), vale a velocidade instantânea desse perfil. Os pedaços de um
  AGV são fatias de um perfil da rota inteira e ficam com a velocidade média do pedaço.
- **Janelas de carga e descarga**: o estado `loading` cobre a carga de todos os itens da
  viagem. Cada coleta (`Moved` para dentro do transportador) fecha uma janela e abre a próxima,
  e `progress` vai de 0 a 1 na janela que contém `t`. A descarga é igual, com as entregas
  (`Moved` para fora do transportador).
- **Altura da carga (`lift`)**: a empilhadeira descansa os garfos a 0,1 m vazia e a 0,3 m
  carregada. Ao carregar ou descarregar num porta-paletes, os garfos sobem até a base da
  célula do item, chegam lá no meio da janela e voltam à altura de viagem. Em outro bloco,
  passam de uma altura de repouso à outra. Uma pessoa carregando segura o item a 1,0 m. Os
  demais veículos informam 0.
- **Troca de mãos no meio da janela**: o item de uma janela de carga ou descarga muda de
  lugar em `HANDOFF_AT` (a metade) da janela, quando os garfos estão na célula, e não no
  `Moved` do fim dela. Dali em diante ele aparece onde vai ficar: a célula, o ponto da esteira
  ou a pilha da fila que terá, ou o transportador. Descarregando num porta-paletes, a carga
  sobe nos garfos, entra na célula lá em cima e os garfos descem vazios; a coleta é o
  inverso.
- **Objeto de apoio (`parent`)**: o bloco onde o item está (fila, célula, esteira,
  transportador); o contêiner de um item embalado; nenhum durante uma transferência. A carga
  de uma empilhadeira fica nos garfos: `FORK_REACH` à frente, a base na altura `lift`, no
  tamanho da carga do porta-paletes (`Scene::fork_scale`).
- **Nível (`fill`)**: nível do tanque sobre a capacidade, linear entre as amostras do rastro.
- **Itens**: ausentes do quadro antes de criados e depois de destruídos. Itens embalados e os
  que passam da pilha visível de uma fila estão no quadro, mas o visualizador não os desenha.

Lacunas conhecidas:
- o rastro não traz o perfil de velocidade de cada trecho, então a posição no meio do trecho é
  linear no tempo mesmo com aceleração (só a velocidade informada segue o perfil);
- a duração de cada carga sai do intervalo entre o início do estado `loading` (ou a coleta
  anterior) e a coleta seguinte. Isso é exato para um transportador, mas não existe um evento
  próprio de "início da carga";
- no modo ao vivo, a quina entre dois trechos só é arredondada quando o segundo trecho chega
  (ver `live.rs`), e o instante da troca de mãos depende do fim da janela, como o `progress`;
- a aproximação ao porta-paletes supõe o corredor no lado `-y` dele.

As esperas para a realidade virtual:
1. Todo o cálculo de posição sai do código Bevy e fica puro e testável em `utsim-scene`.
2. A escala é real.
3. O protocolo é versionado.
4. Há um ponto de saída único (`SceneSink`).

Com isso, o cliente Godot (com o OpenXR nativo para realidade virtual e aumentada) só
precisa ler a cena e desenhar.

### Biblioteca de modelos 3D (`utsim-assets`)

A crate `utsim-assets` gera por código, sem programa externo, os modelos articulados da
biblioteca de aparências (empilhadeira, paleteira, AGV, palete, caixa, porta-paletes, esteira,
tanque, máquina, bancada, operador e área de piso) e grava `.glb` com medidas reais. Cada
peça móvel é um nó com nome e pivô na articulação (`carro` e `garfos` da empilhadeira, rodas,
`luz`, braços e pernas do operador), então qualquer motor anima só mudando a transformação do
nó. Os padrões ficam versionados em `assets/modelos/` e um teste garante que o gerador
reproduz os mesmos bytes. O visualizador gera os mesmos bytes em memória e os carrega como
cenas glTF (desktop e navegador), escolhe o modelo pelo `MobileRole` e pelo tipo do objeto ou
pelo `visual.asset`, e move os nós a partir da `Pose`. O botão "Visual: simples / realista"
mantém as formas simples. A tabela de nós e as lacunas estão em `docs/plano_3d_realista.md`.

`visual.asset` também aceita os nomes do catálogo de terceiros (`assets/terceiros/catalogo.ron`,
compilado no visualizador pelo módulo `terceiros`). O arquivo é lido do disco, da pasta dada por
`UTSIM_TERCEIROS` ou da primeira `assets/terceiros` acima do executável ou da pasta de trabalho,
e posto no tamanho real virado para +x: escala `escala_real` (vezes `visual.scale`), giro
`giro_y_graus` (mais `visual.yaw`) e um deslocamento que leva o centro da planta
(`centro_m`) à posição do objeto e o ponto mais baixo (`minimo_m`) ao chão. Texturas que um
`.glb` cita por caminho relativo são servidas da pasta dele; se faltarem, uma textura cinza as
substitui para a malha continuar visível. Lacunas: no navegador os arquivos de terceiros não são
carregados (fica a forma padrão); a cor (`visual.color`) só pinta o material `pintura` da
biblioteca embutida.

### Planta CAD (`utsim-cad`)

Crate pura, sem interface e sem dependências além de `serde` e `thiserror`. Leitor mínimo
de DXF 2D em texto (a crate `dxf` não foi usada: o leitor próprio cobre `LINE`, `LWPOLYLINE`,
`POLYLINE`, `CIRCLE`, `ARC`, `TEXT`, `MTEXT` e `INSERT` aninhado, e evita uma dependência).
A planta achatada (`Plan`) tem camadas em ordem de nome, com polilinhas e rótulos em metros
(conforme `$INSUNITS`; ausente: milímetros, marcado em `units_assumed`). Calibração por dois
pontos (`Calibration`, escala e, com dois pares, rotação e translação), simplificação
Douglas-Peucker, arrays planos para a interface (`FlatLines`) e SVG determinístico (`to_svg`).
Camadas têm papel (`Role`: fundo, parede, coluna, porta, ignorar) sugerido pelo nome e
sobrescrito pela pessoa (`RoleMap`); `build_obstacles` devolve polígonos anti-horários
(paredes abertas ganham espessura, padrão 0,15 m) e as aberturas das portas, a entrada da
futura `utsim-nav`. A planta não é lida pela simulação.

**Proposta `floor_plan` do modelo (ainda não ligada ao `utsim-model`).** `FloorPlanDef`
(em `utsim-cad`, serializável em RON) traz arquivo relativo ao `.ron`, hash `fnv1a64`,
transformação, calibração, camadas com papel e visibilidade e espessura de parede. Ligá-lo
seria um campo opcional `floor_plan: Option<FloorPlanDef>` com `#[serde(default)]` no
`Model`, aditivo, sem passo de `MIGRATIONS` (modelos antigos seguem válidos). Fica como
proposta porque `utsim-model` está em mudança por outro trabalho; os obstáculos gerados
irão para um campo `obstacles` junto com o recurso de desvio.

### Pessoas animadas por esqueleto (A3D-3)

No visual realista, as pessoas (`MobileRole::Person`, inclusive os operadores) usam o boneco
CC0 da Universal Animation Library (`pessoa_operador` no catálogo), com os clipes dos dois
arquivos (`pessoa_operador_carga` traz o de andar carregando). Os dois têm o mesmo esqueleto e
os mesmos nomes de ossos, e o Bevy identifica o alvo de um clipe pelo caminho de nomes desde a
raiz da animação, então os clipes do segundo arquivo movem o boneco do primeiro sem conversão
(o visualizador confere, ao montar o grafo, que todo alvo de todo clipe usado existe no
esqueleto). O módulo `people` do binário monta um `AnimationGraph` só, com um nó por clipe:

| Situação (`Pose`) | Clipe |
|---|---|
| `Moving`, sem carga | `Walk_Loop` (`Jog_Fwd_Loop` acima de 2 m/s) |
| `Moving`, carregando (`lift` > 0 ou conteúdo) | `Walk_Carry_Loop` |
| `Loading`, `Unloading` | `PickUp_Table`, no ritmo do `progress` |
| `Working` | `Interact` (no painel da máquina) |
| parado carregando | pernas de `Idle_Loop` com os braços de `Walk_Carry_Loop` (dois nós com máscara) |
| `Idle`, `Blocked`, `Stopped` | `Idle_Loop` |

Todos os nós ficam pausados: a cada quadro o visualizador define o peso (transição de 0,25 s
de tempo real) e o tempo do clipe, que vem da cena (`visual::clip_time`): os clipes de andar
seguem a distância percorrida (`Scene::distance_at`), uma volta do clipe por passada medida
no próprio clipe (velocidade do pé de apoio: 1,31 m por volta no andar e no andar carregando,
5,44 m na corrida, na escala do arquivo), então a taxa do clipe acompanha a velocidade e o pé
não desliza; a coleta segue o `progress`; os demais repetem no relógio simulado. Saltar para
`t` dá a pose de `t`. O item carregado fica entre as mãos (ponto médio de `hand_l` e `hand_r`,
lido depois da propagação das transformações), nunca dentro do peito. O boneco não tem
textura: os materiais `M_Main` e `M_Joints` são tingidos de roupa de trabalho, e um capacete
amarelo e um colete laranja com faixas são presos aos ossos `Head` e `spine_02`. Sem os
arquivos (navegador) ou se eles não carregarem, as pessoas voltam ao `operador_simples`.

### Luz e sombras

No visual realista há um piso cinza sob a cena (com 15 m de margem) que recebe sombras, e o sol
(luz direcional) projeta sombras em cascata dimensionadas pela cena: do mínimo de 0,1 m até a
distância da câmera ao centro mais meia diagonal da cena, com a primeira cascata em um sexto
disso. Com sombras a luz ambiente sobe (520 em vez de 350) para a face na sombra não ficar
escura. A caixa "Sombras", ao lado de "Visual", liga e desliga; o padrão é ligado no desktop e
desligado no navegador (WebGL2, duas cascatas quando ligadas). Medido no
`centro_de_distribuicao.ron` (visão geral reproduzindo, build de release, RTX 4070 Ti, sem
sincronismo vertical, com os níveis de detalhe do A3D-4): 548 quadros por segundo sem sombras e
448 com elas.

## 4q. Resultados no editor (fase 10)

A janela **Resultados** do `utsim-view` roda replicações, cenários e otimizações do modelo aberto e mostra as estatísticas sem a linha de comando. A divisão segue a do resto do editor: números num lugar só, lógica pura testável e um painel fino.

- `utsim-model::analysis`: uma linha por objeto (`ObjectStats`, cada coluna um `Summary` entre replicações, os mesmos números de `summarize`), a ocupação (`BlockReport::utilization` em `utsim-core`: fração do tempo em `processing` e `setup` nas estações, e média de servidores ocupados sobre a capacidade quando há vários; o conteúdo **não** entra, porque uma montagem guarda componentes à espera; fração do tempo em viagem, carga, descarga ou trabalho nos transportadores) com as variantes incluindo bloqueio, dentro do turno e sem bloqueio, o gargalo (maior ocupação entre os recursos), a série temporal média entre replicações, os histogramas das replicações reunidos numa grade comum e a diferença pareada (`paired_difference`, `delta_cell` com `≈`).
- `Model::run_replications_with(progress)` e `experiment::run_scenarios(.., progress)` avisam cada replicação concluída; quando o aviso responde `false`, nenhuma replicação nova começa e o resultado é `ModelError::Cancelled`. `utsim_opt::optimize_with` faz o mesmo por ponto avaliado. Os resultados são idênticos aos das versões sem progresso.
- `ScenarioResult::per_replication` guarda o valor de cada métrica por replicação (fora do JSON), para a diferença pareada com o primeiro cenário, que o `utsim experiment` também passou a imprimir.
- `utsim-model::csv` (vindo do `utsim-cli`) e `utsim-model::html` geram os arquivos exportados: os CSV do `utsim run --csv`, mais `objetos.csv`, e um relatório HTML autocontido com SVG em linha.
- `utsim-viewer::results` (biblioteca, sem egui): configurações da rodada, ordenação e filtro da tabela, dados dos gráficos (cores de estado de `style::state_color`), cenários montados das linhas da grade, problema de otimização montado do formulário e o comando de "Aplicar ao modelo" (um `Command::Batch` de `Set`, um passo de desfazer).
- `results_panel.rs` (binário): a janela, com os gráficos em `egui_plot` 0.37 (a versão feita para o egui 0.36 do `bevy_egui` 0.43). Os trabalhos rodam numa thread no aplicativo de mesa e no próprio quadro no navegador; o progresso volta por um canal e "Cancelar" liga um `AtomicBool`. Arquivos (CSV, HTML, experimento `.ron`) só no aplicativo de mesa.

## 4r. Interface Godot (fases G1 e G2: ponte, reprodução e edição)

A interface 3D da versão 0.2 (e depois a realidade virtual e aumentada) é feita no Godot 4.7.
O editor e visualizador Bevy (`utsim-viewer`) continua sendo o da versão 0.1. O Godot não
reimplementa nada da simulação nem da cena: uma biblioteca em Rust, carregada como
GDExtension, roda o modelo e entrega ao GDScript os mesmos quadros que o `utsim cena` grava.
O plano das fases G1 a G5 está em `docs/plano_godot.md`.

**Ponte** (`crates/utsim-godot`, biblioteca `cdylib` com o godot-rust gdext, crate `godot`
0.5.5 com a feature `api-4-7`). Fica fora de `default-members` e dos passos do
`verificar.sh` sem `UTSIM_GODOT=1`. Só as classes do núcleo do Godot são geradas (sem
`__codegen-full`), porque a ponte devolve dados simples e todo nó é criado no GDScript; o
build da ponte leva cerca de 1 min. Três classes, todas `RefCounted`:

| Classe | Métodos |
|---|---|
| `UtsimModel` | `load(caminho)` lê e valida o `.ron` e devolve `{ok, errors, warnings, name, objects, replications, stop_time}`; `run(n)` roda `n` replicações em paralelo e devolve `{ok, replications, metrics}`, com `metrics["bloco.métrica"] = {mean, half_width, std_dev, n}` (os números do `utsim run`); `playback(rep, pessoas_animadas)` grava o replay de uma replicação e devolve um `UtsimPlayback` (ou `null`, com `last_error()`) |
| `UtsimPlayback` | `static_scene()` (uma vez), `frame(t)` (a cada quadro), `end()`, `next_event(t)`, `prev_event(t)`, `glb(chave)`, `person_clips()`, `set_clip_durations(...)` |
| `UtsimDocument` (G2) | o modelo em edição sobre o `utsim_edit::Document`: `new_model`, `open`, `save`, `save_as`, `undo`, `redo`, `is_dirty`, `revision`; `add(id, ponto, passo)` com encaixe na grade, `remove`, `connect_objects`, `disconnect_objects`, `move_connection`, `begin_drag`/`drag`/`end_drag` (um passo de desfazer), `nudge`, `copy`/`cut`/`paste`/`duplicate`, `align`, `distribute`; `objects()`, `connections()` (com o número da porta), `issues()`, `warnings()`; `properties(objeto)` (o esquema do painel), `set_property(objeto, caminho, valor, outros)` e `field_text`; `edit_scene()` (a cena do documento, sem rodar), `palette_scene(id)` (o fantasma ao colocar), `look_scene(objeto, ativo)` (a miniatura); `as_model()` (um `UtsimModel` para Simular); `file_differences(a, b)` (os modelos de dois arquivos, para os roteiros) |

O núcleo da ponte não tem tipos do Godot e é testado sem o motor (`cargo test -p utsim-godot`):

- `coords`: conversão de coordenadas, com testes;
- `playback`: a `Scene` e o empacotamento dos quadros;
- `document`: o `Editor` (documento, área de transferência, arrasto), as linhas de objetos e
  ligações, `edit_replay` (o modelo em edição desenhado pelo mesmo código da reprodução),
  `look_model` (a miniatura) e `model_differences` (o que difere entre dois modelos);
- `schema`: as propriedades de um objeto como uma lista de campos (caminho, rótulo, tipo,
  valor, escolhas, unidade, ajuda, seção, profundidade, ligado ou não) e a edição de um campo
  como um `Command` do `utsim-edit`; as distribuições vêm como família, parâmetros e texto
  RON; os turnos como tabela própria ou calendário do modelo.

Duas crates sem motor são usadas pelo Bevy e pela ponte, para os dois desenharem o mesmo
modelo do mesmo jeito:

- `utsim-look` (G2): as regras de aparência (`visual`: o ativo de cada entidade por papel,
  tipo e `visual.asset`, os clipes das pessoas, as cargas nos garfos, no deque e nas mãos, a
  luz do AGV, as rodas) e o catálogo de terceiros (`catalogue`). O `utsim-viewer` reexporta os
  dois módulos com os nomes antigos (`visual`, `terceiros`) e a ponte com `looks` e
  `catalogue`;
- `utsim-edit::ron_text`: o texto RON de cada valor que um `Command::Set` recebe.

**Fluxo de dados:**

1. O GDScript chama `UtsimModel.load` e `run`; a simulação e as replicações rodam no Rust.
2. `playback(0, ...)` grava o rastro da replicação 0 e monta a `Scene` com o modelo
   (`SceneOptions.model`), como o `utsim cena` faz.
3. `static_scene()` devolve um dicionário: blocos com nome, tipo, papel, posição da base,
   tamanho, forma simples e, quando há modelo realista, o ativo (`{source, name, key, path}`),
   a escala, o giro, o deslocamento, a cor, o modo de carregar (`forks`, `hands`, `deck`) e se
   é o boneco animado. Também a rede de caminhos, as ligações, os nós de fluido e os limites.
4. `frame(t)` devolve arrays empacotados (`PackedVector3Array`, `PackedFloat32Array`,
   `PackedInt32Array`, `PackedColorArray`), um elemento por bloco e um por item desenhado:
   - por bloco: `position`, `yaw`, `lift`, `motion` (código), `param` (velocidade ou
     progresso), `color` (cor do estado, ou a luz do AGV pelo movimento), `rolled`
     (distância andada sobre a escala, para as rodas), `clip` e `clip_time` (pessoas),
     `label`;
   - por item: `item_id`, `item_position`, `item_yaw`, `item_scale`, `item_color`,
     `item_carrier` e `item_stack`.

   Não há um dicionário por entidade, então o custo cresce pouco com centenas de itens.
5. O GDScript (`godot/scripts/cena.gd`) só move as raízes dos recursos móveis e os nós
   articulados, e troca cores e rótulos quando mudam.

**Coordenadas:** toda posição sai da ponte já no espaço do Godot: o `(x, y, z)` do modelo (Z
para cima) vira `(x, z, -y)` (Y para cima), em metros, e o `yaw` vira rotação em torno de `+Y`
(um rumo `(cos, sin, 0)` vira `(cos, 0, -sin)`, que é `Basis(UP, yaw) * RIGHT`). É a mesma
conversão do Bevy, porque os dois motores usam Y para cima com a mão direita; por isso o espaço
de layout interno da `utsim-scene` (`layout::to_bevy`) já é o espaço do Godot, e os testes de
`coords` conferem as duas coisas.

**O que a ponte calcula e o que o GDScript faz:**

- **Ponte:** a pose desenhada de cada móvel, como o `models::drawn_pose` do Bevy (veículos
  fora das máquinas, operador na frente do posto); onde fica cada carga (nos garfos, no
  deque do AGV, nas mãos, no chão da área da fila); o clipe e o tempo dentro dele (andar,
  correr, andar carregando, pegar, trabalhar, parado segurando a caixa), com a passada
  medida de `people.rs`, de modo que o pé não desliza; a cor da luz.
- **GDScript:** aplica os números aos nós com nome da biblioteca:
  - `carro` na altura `lift` (garfos);
  - rodas `roda*` com o ângulo `-(rolled / raio)`;
  - material próprio no nó `luz`, com emissão.

  O boneco (`godot/scripts/pessoa.gd`) usa uma `AnimationTree` em modo manual: dois espaços
  (o clipe que entra e o que sai, com transição de 0,25 s de tempo real), cada um com um
  `TimeSeek` e uma camada filtrada de tronco e braços para "parado segurando a caixa". Os
  clipes do segundo arquivo (`pessoa_operador_carga`) entram como a biblioteca de animações
  `ual2`; os caminhos das trilhas são iguais nos dois arquivos. O item carregado vai para o
  ponto médio dos ossos `hand_l` e `hand_r`. O importador do Godot tira o sufixo `_Loop` dos
  clipes cíclicos (`Idle_Loop` vira `Idle`), e `pessoa.gd` resolve os dois nomes.

**Ativos:**

- **Biblioteca com parâmetros padrão:** os `.glb` de `assets/modelos/` são importados pelo
  Godot.
- **Biblioteca com tamanho próprio:** um porta-paletes com seus vãos e níveis, ou uma esteira
  do seu comprimento, é gerado pela ponte com o `utsim-assets` (`UtsimPlayback.glb(chave)`, os
  mesmos bytes que o Bevy gera em memória) e carregado com `GLTFDocument` em tempo de
  execução.
- **Boneco animado:** os arquivos UAL de `assets/terceiros/quaternius_ual*` são importados.
- **Outros itens do catálogo e arquivos do usuário** (`visual.asset` com `.glb`): lidos do
  disco com `GLTFDocument`.

**Compartilhamento dos `.glb`:** o `scripts/godot.sh` copia os arquivos para
`godot/importados/` (fora do git) e roda `godot --headless --import`. Os `.import` e o cache
ficam no projeto Godot, e `assets/` continua só com os originais. Foi escolhida a cópia em vez
de link simbólico ou junção porque funciona igual no Windows sem permissão de administrador e
no Linux, e o Godot não suja `assets/` com arquivos `.import`. Os originais continuam a única
fonte: rodar o script de novo atualiza as cópias.

**Projeto** (`godot/`):

- `project.godot`: Forward+, OpenXR configurado e desligado, pronto para a fase G4;
- `utsim.gdextension`: Windows `bin/utsim_godot.dll`, Linux `bin/libutsim_godot.so`;
- `cenas/principal.tscn` com `scripts/principal.gd`, que monta tudo em código:
  - luz direcional com sombras em quatro cascatas, dimensionadas pela cena;
  - ambiente com a cor de fundo do Bevy, piso cinza e a grade do Bevy (cerca de 40 células,
    passo de 1, 2 ou 5 × 10^k m);
  - câmera de órbita (`camera_orbita.gd`), com as convenções do `OrbitCam` do Bevy;
  - painel: abrir modelo (FileDialog), replicações, "Simular", lista de blocos (clique foca a
    câmera), rótulos, sombras e resumo dos resultados;
  - barra de reprodução: reproduzir ou pausar, velocidade, linha do tempo, evento anterior e
    seguinte.

**Como compilar e rodar:**

```bash
bash scripts/godot.sh            # ponte (debug), cópias dos .glb e importação
bash scripts/godot.sh --release  # ponte otimizada
bash scripts/godot.sh --abrir    # e abre o projeto no Godot (F5 roda)
bash scripts/godot_teste.sh      # sem janela: extensão, 3 exemplos e os roteiros g2_*.json
```

O executável do Godot vem de `GODOT`, de `godot` no `PATH` ou de `~/Documents/Godot_v4.7*`.

**Exportação (G5):** `bash scripts/godot_exportar.sh [--instalar-modelos]` compila a ponte em
release, importa sem janela e exporta com os presets de `godot/export_presets.cfg`
(`--headless --export-release`) para `target/godot-export/<plataforma>/`: `utsim-3d`,
`utsim-3d.pck` (cena, scripts e `.glb`), a biblioteca da ponte ao lado do executável e
`AVISOS-TERCEIROS.txt`, com o copyright e as licenças do motor lidos do executável
(`scripts/godot_avisos.gd`; não é a licença do UTSim). `scripts/preparar-pacote.sh` leva essa
pasta como `3d/` ao instalador do Windows (componente "UTSim 3D (prévia)", com atalho próprio),
ao `.zip`, ao `.tar.gz` e ao `.deb`, ao lado do `utsim-view`. Os modelos de exportação (cerca de
1,2 GB) são baixados com o SHA512 conferido. Detalhes e pendências em `docs/plano_godot.md`.

**Teste sem janela** (`godot/testes/teste.gd`): confere que a extensão registrou as
classes e que cada modelo carrega, roda duas replicações, monta a cena (com quantos blocos
têm modelo e quantas pessoas animadas) e é reproduzido do início ao fim em 400 passos. Falha
se um recurso móvel nunca sai do lugar, se nenhum item aparece, se uma pessoa ficou sem
animação ou se o Godot imprimir um erro. Depois, o `scripts/godot_teste.sh` roda os roteiros
da interface (`godot/roteiros/g2_*.json`) também com `--headless`: a cena principal inteira,
com cliques, arrastos e teclas de verdade (`Input.parse_input_event`), com a vista do tamanho
do projeto; só as capturas de tela são puladas.

**Edição (G2):** o modo Editar desenha o documento com o mesmo `CenaUtsim` da reprodução
(`UtsimDocument.edit_scene`) e o `godot/scripts/editor.gd` trata a entrada, sem física:

- **escolher:** o raio da câmera cruza a caixa de cada bloco (pegada e altura); clicar de
  novo no mesmo ponto passa ao bloco de baixo; uma ligação é pega a poucos pixels da linha;
  arrastar no chão faz a seleção por caixa;
- **mover:** o ponto do chão sob o mouse (projeção no plano `y = 0`) dá o deslocamento, com
  encaixe na grade (`G` liga e desliga, passo de 0,25 a 5 m); o arrasto inteiro é um passo de
  desfazer;
- **colocar:** da Biblioteca (`biblioteca.gd`, ícones em `godot/icones/`), clicando e
  clicando no chão ou arrastando até o chão, com o fantasma do objeto sob o mouse;
- **ligar:** cada bloco tem a porta de saída (`+x`, verde) e a de entrada (`-x`, azul),
  mostradas no selecionado e no que está sob o mouse; arrastar da saída até outro bloco (ou a
  porta de entrada dele) liga; a ferramenta Ligar (`L`) liga arrastando de bloco a bloco ou
  clicando na origem e no destino. Recursos móveis não têm portas (usam `transport` e
  `operators`);
- **painel de propriedades** (`painel_propriedades.gd`): gerado do esquema, campo por campo
  (texto, número, escolha, distribuição com família e parâmetros ou texto RON, listas em RON,
  conjuntos, cor, ativo com miniatura). O painel não muda o modelo: emite a edição, a ponte a
  valida e aplica como comando, e o painel é refeito no quadro seguinte. Com vários objetos do
  mesmo tipo selecionados, a edição vale para todos (um passo de desfazer);
- **atalhos:** os do Bevy (Ctrl+Z, Ctrl+Y, Ctrl+S, Delete, Esc, setas, Ctrl+C/X/V/D, Ctrl+A),
  mais F1 (a janela de atalhos) e Tab (Editar e Simular);
- **Simular:** o documento vira um `UtsimModel` (`as_model`), só quando não tem problemas; a
  reprodução é a da G1.

**Capturas roteirizadas** (o autoload `godot/scripts/roteiro.gd`), para o controle visual:

```bash
godot --path godot -- --utsim-roteiro=roteiros/capturas.json --utsim-capturas=<pasta>
```

O roteiro JSON lista modelos e, para cada um:

- capturas: instante, nome e, opcional, `focar` com os números do passo `Focar` dos roteiros
  do Bevy (objeto, distância, giro, inclinação), o que permite comparar as mesmas vistas;
- uma medida opcional: reproduz por alguns segundos e grava os tempos em `log.txt`.

**Custo por quadro** (médias de duas rodadas do roteiro de capturas, reproduzindo 4 s; ponte em
debug, RTX 4070 Ti, sincronismo vertical em 144 Hz):

| Modelo | `frame(t)` na ponte | aplicar no GDScript | quadros por segundo |
|---|---|---|---|
| `centro_de_distribuicao.ron` | 0,17 a 0,57 ms | 0,19 a 0,62 ms | 125 a 145 |
| `agvs.ron` | 0,20 a 0,46 ms | 0,08 a 0,16 ms | 139 a 145 |
| `pessoas_carregando.ron` | 0,18 a 0,21 ms | 0,25 a 0,28 ms | 145 |

No teste sem janela (sem desenhar, 401 saltos pelo replay inteiro), a média é de 0,07 a 0,44 ms
na ponte e de 0,04 a 1,3 ms no GDScript; o maior valor é o do centro de distribuição, em que cada
salto cria e apaga muitos nós de itens (palete com oito caixas), coisa que a reprodução contínua
quase não faz.

**Lacunas depois da G2:**

- o `operador_simples` (sem o boneco) não mexe braços e pernas no Godot;
- os nós de fluido são cilindros simples, sem nível;
- os rótulos não têm o limite por distância do Bevy (A3D-4);
- só há a câmera de órbita;
- o replay é sempre o da replicação 0;
- `run` bloqueia o quadro enquanto as replicações rodam (a thread com progresso e cancelar
  fica para a G3, com os resultados);
- o Editar não edita a rede de caminhos, os fluidos, os fluxos de tokens nem os calendários
  (um objeto escolhe um calendário que já existe; a G3 traz a rede e os fluidos);
- a aparência do Editar (portas, contornos, miniatura) só foi conferida pelos roteiros sem
  janela: a conferência visual está anotada em `docs/plano_godot.md`.

## 4s. PERT/CPM de uma parte do processo

Crate `crates/utsim-pert`, pura (sem motor gráfico), dependendo de `utsim-core` (`Dist`,
`rng`, rastro, `Summary`), `utsim-model` (`Model`, `effective_transfer`, CSV e `esc` do
HTML), `utsim-objects` (`network::router` para as distâncias) e `utsim-scene`
(`layout::layers` para as colunas do diagrama). Comando: `utsim pert`
(`crates/utsim-cli/src/pert.rs`); manual em `docs/manual/src/pert-cpm.md`.

- `network.rs`: `derive(model, selection, options)` monta a `ActivityNetwork` (atividade no
  nó, marcos `início` e `fim`, avisos e hipóteses). A seleção é `Selection::Objects` ou
  `Selection::Between { from, to }` (interseção dos alcançáveis a partir de `from` com os que
  alcançam `to`, a semântica de `SpanDef`). Ordem dos passos: recursos móveis saem da seleção;
  ligações de volta são achadas por DFS a partir das raízes e cortadas, e as atividades do
  laço (alcançáveis de `v` que alcançam `u`) ganham o fator `1 / (1 - p)`, com `p` do
  `Weighted` de `u`; cada objeto com várias saídas que não é divisão E (`Separator`,
  `Combiner` em `Batch`) fica com um ramo (`WeightedBranch::MostLikely` ou o primeiro) ou gera
  cenários (`derive_scenarios`, até 64, mais provável primeiro); o que as raízes não alcançam
  mais sai da rede; ligações com `transport` ou `transfer` viram atividades próprias
  (`transport_name`, `transfer_name`). Uma atividade é `factor × soma das partes`, cada parte
  uma `Dist` sorteada à parte (setup + processo, ou viagem vazia + carga + viagem + descarga).
- `cpm.rs`: `Cpm` guarda a ordem topológica (Kahn, menor índice primeiro) e as adjacências
  para passagens repetidas. Folga total `LS - ES`, folga livre `min(ES dos sucessores) - EF`,
  crítica com tolerância `1e-9 × max(1, T)`, todos os caminhos críticos por DFS sobre as
  precedências justas (`EF = ES`), até 1000.
- `pert.rs`: `(a, m, b)` por distribuição (quantis de 1% e 99% e moda quando não há forma
  direta; quantis sem fórmula fechada vêm de 20 001 sorteios de um stream fixo), variância
  exata de todas as famílias de `Dist`, `Moments::Exact` (padrão) ou `Classic`, variância do
  caminho crítico de maior variância e `Φ` local (`math.rs`), sem depender de `utsim-fit`.
- `monte_carlo.rs`: blocos de 500 sorteios; o bloco `k` sorteia a atividade `i` de
  `rng::stream(semente, k, i)`; os blocos rodam no rayon e são juntados na ordem, então o
  resultado não depende do número de threads. Criticidade por atividade e por caminho,
  percentis, `P(T ≤ prazo)` com intervalo de Wilson, correlação de Spearman e crucialidade.
- `observed.rs`: roda as replicações do modelo com um `Observer` que guarda só as visitas
  aos objetos da seleção e aos recursos móveis (entrada, saída, origem, ligação com o item de
  origem em `Unpacked` e nas cópias do `Split`) e os estados dos objetos que trabalham um item
  por vez. Para cada unidade que sai de um objeto final, recua: em `Combiner`, pela entrada
  do conjunto que chegou por último (conjuntos não se sobrepõem no tempo, porque a montagem só
  aceita o próximo depois que o anterior saiu); em itens saídos de `Separator`, pelo item que
  entrou nele; para quando a parada anterior está fora do trecho. Os trechos são contíguos, então
  espera + trabalho + bloqueio somam o tempo total da unidade.
- `report.rs`: texto, CSV, HTML (rede, Gantt, histograma, tornado, tabela) e `diagram`, que
  devolve caixas (camada, raia, posição, datas, criticidade) e setas para a interface desenhar.

**Lacunas:** o campo `pert` do formato do modelo (análises salvas) ainda não existe; o modo
simulado guarda o rastro do trecho de uma replicação inteira na memória (não em fluxo); a
janela na interface Godot fica para depois.

## 5. Estatísticas

Coletadas automaticamente pelo motor para todo bloco:

| Métrica | Tipo |
|---|---|
| entradas, saídas | contadores |
| conteúdo médio / máximo | média ponderada pelo tempo |
| permanência (staytime) | média, desvio, mín, máx por item |
| fração de tempo em cada estado | `StateTracker` |
| métricas próprias (`ctx.record`) | ex.: `time_in_system` da Sink |

Entre replicações: média, desvio e **semiamplitude do IC de 95%** (t de Student).

### Análise de saída (`utsim-stats`)

A crate `utsim-stats` é pura: recebe números e devolve números, sem modelo nem kernel, e
só depende de `utsim-core` (pelo `Summary`). `utsim-model::analysis` transforma relatórios
e rastros nas séries que ela precisa, e `utsim analisar` imprime o resultado.

| Módulo | Conteúdo |
|---|---|
| `special` | `ln_gamma`, `beta_inc`, `gamma_p`, `norm_cdf`, `norm_ppf`, `brent`..., movidos de `utsim-fit` (que reexporta `utsim_fit::special`), e `t_cdf`/`t_ppf` |
| raiz (`lib.rs`) | `t_quantile(nível, gl)` e `summary(xs, nível)`; `parse_level`; autocorrelação lag 1 |
| `warmup` | média entre replicações, média móvel de Welch, MSER e MSER-5 |
| `precision` | replicações necessárias para uma semiamplitude alvo; médias em lotes |
| `little` | `L = λ·W` com a diferença pareada por replicação e tolerância relativa |

- **Nível de confiança.** Em 95% `t_quantile` usa a tabela `t_975` do núcleo, então os
  intervalos de 95% ficam idênticos bit a bit aos de antes. Outros níveis usam `t_ppf`
  (inversa da CDF da t por `beta_inc` e `brent`), que reproduz a tabela nas três casas.
- **Observações com instante.** `Simulation::keep_timed(bloco, medida)` liga, para uma
  medida só, o registro `(t, x)` de cada observação (`Tally::record_at`); o relatório traz
  a lista em `TallySummary::observations`, que não aparece no JSON quando desligada.
  Desligado, o custo é um teste de `Option` por observação, e nenhuma estatística muda.
  `Model::run_replications_timed` liga as medidas pedidas.
- **Lei de Little no sistema.** `L` soma o conteúdo de todos os objetos menos os
  sumidouros e, com `transfer`, o número médio em trânsito, calculado pelos eventos
  `Transit` do rastro (`analysis::transit_mean`). Montagens e desmontagens mudam o número
  de itens e ficam fora da verificação.
- **Gargalo móvel.** `analysis::active_periods` lê os eventos `State` do rastro: período
  ativo é o trecho contínuo em `Setup`, `Processing`, viagens, carga, descarga ou
  `Utilize`; trechos que se tocam no mesmo instante viram um só. Em cada instante o gargalo
  é o recurso com o período ativo mais longo; a sobreposição entre o período do gargalo
  atual e o do seguinte é gargalo compartilhado dos dois (`shifting_bottleneck_times`).

## 6. Pontos de extensão já previstos

- `position` em cada objeto: layout para o 3D.
- `State` já inclui `waiting_operator`, `waiting_transport`, `down` e `collecting`.
- `Behavior` é um trait comum: blocos novos não exigem mudar o motor.
- A busca de destino percorre todo o *outbox* a cada tentativa. Isso é simples e correto,
  mas tem custo O(n) em filas muito longas e bloqueadas. Otimizar quando houver um caso real.

## 7. Referências estudadas

- Law, A. M. *Simulation Modeling and Analysis*. McGraw-Hill.
- Banks, J. et al. *Discrete-Event System Simulation*. Pearson.

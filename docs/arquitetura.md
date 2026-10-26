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
| Execução | `utsim-cli` | replicações em paralelo, sem tela |
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
    bloqueio. Cada perda registra `lost` na Source (use o campo `count` da tally, aparece
    em `RunReport`); itens perdidos não contam em `output`. Serve para M/M/1/K e Erlang
    de perda.
- Estados: `generating`, `blocked`, `idle` (quando atinge `max_arrivals`).

### Queue
- Capacidade opcional, FIFO ou LIFO. Libera cada item ao entrar.
- Estados: `empty`, `releasing`.
- Lote (batching), com tamanho-alvo, tempo máximo de espera e *flush*, fica para a fase 5.

### Processor
- Capacidade `n` (itens em paralelo), `setup` opcional, depois `process`.
- Estados: `idle`, `setup`, `processing`, `blocked` (terminou e o destino está fechado).
- Com `n > 1`, o estado reportado é o mais produtivo entre os itens presentes
  (processing > setup > blocked > idle).

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
  aproximado quando há aceleração.
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
- motores externos (Unreal, Unity, Godot).

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
  - Unreal converte em `(100x, -100y, 100z)`, em cm.
- **Fluxo de cena** (protocolo `utsim-scene/1`):
  - formato JSON em linhas: um cabeçalho (`Header`) e depois um quadro por linha;
  - `SceneSink` abstrai o destino: arquivo, socket, runtime de realidade virtual ou teste;
  - `utsim cena` grava o fluxo de uma rodada, e o servidor ao vivo (abaixo) serve o mesmo
    fluxo durante uma emulação.
- **Aparência** (`visual` no objeto): um item da biblioteca embutida ou um `.glb`, com
  escala, giro e cor. É só apresentação: a simulação nunca lê esse campo.

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
  caminho pelo comprimento de arco. Antes do primeiro trecho, a posição do layout.
- **Direção (`yaw`)**: a do pedaço de caminho percorrido. Parado, entre trechos ou quebrado,
  mantém a direção do fim do último trecho com comprimento.
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
- **Objeto de apoio (`parent`)**: o bloco onde o item está (fila, célula, esteira,
  transportador); o contêiner de um item embalado; nenhum durante uma transferência.
- **Nível (`fill`)**: nível do tanque sobre a capacidade, linear entre as amostras do rastro.
- **Itens**: ausentes do quadro antes de criados e depois de destruídos. Itens embalados e os
  que passam da pilha visível de uma fila estão no quadro, mas o visualizador não os desenha.

Lacunas conhecidas:
- o rastro não traz o perfil de velocidade de cada trecho, então a posição no meio do trecho é
  linear no tempo mesmo com aceleração (só a velocidade informada segue o perfil);
- a duração de cada carga sai do intervalo entre o início do estado `loading` (ou a coleta
  anterior) e a coleta seguinte. Isso é exato para um transportador, mas não existe um evento
  próprio de "início da carga";
- os itens sobre uma empilhadeira continuam no quadro em cima do veículo, sem seguir a altura
  dos garfos. O visualizador realista corrige isso: com o `parent` e a `lift`, põe a carga nos
  garfos, e a cena expõe `Scene::rack_target` (a célula da janela de carga atual) para o
  motor virar a empilhadeira para ela.

As esperas para a realidade virtual:
1. Todo o cálculo de posição sai do código Bevy e fica puro e testável em `utsim-scene`.
2. A escala é real.
3. O protocolo é versionado.
4. Há um ponto de saída único (`SceneSink`).

Com isso, um cliente de realidade virtual (OpenXR no Bevy, ou o Unreal com o OpenXR
nativo) só precisa ler a cena e desenhar.

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
carregados (fica a forma padrão); as pessoas animadas aparecem na pose de repouso (a T do UAL),
porque os clipes ainda não são tocados; a cor (`visual.color`) só pinta o material `pintura` da
biblioteca embutida.

## 4q. Resultados no editor (fase 10)

A janela **Resultados** do `utsim-view` roda replicações, cenários e otimizações do modelo aberto e mostra as estatísticas sem a linha de comando. A divisão segue a do resto do editor: números num lugar só, lógica pura testável e um painel fino.

- `utsim-model::analysis`: uma linha por objeto (`ObjectStats`, cada coluna um `Summary` entre replicações, os mesmos números de `summarize`), a ocupação (conteúdo médio / capacidade nas estações; fração do tempo em viagem, carga, descarga ou trabalho nos transportadores), o gargalo (maior ocupação entre os recursos), a série temporal média entre replicações, os histogramas das replicações reunidos numa grade comum e a diferença pareada (`paired_difference`, `delta_cell` com `≈`).
- `Model::run_replications_with(progress)` e `experiment::run_scenarios(.., progress)` avisam cada replicação concluída; quando o aviso responde `false`, nenhuma replicação nova começa e o resultado é `ModelError::Cancelled`. `utsim_opt::optimize_with` faz o mesmo por ponto avaliado. Os resultados são idênticos aos das versões sem progresso.
- `ScenarioResult::per_replication` guarda o valor de cada métrica por replicação (fora do JSON), para a diferença pareada com o primeiro cenário, que o `utsim experiment` também passou a imprimir.
- `utsim-model::csv` (vindo do `utsim-cli`) e `utsim-model::html` geram os arquivos exportados: os CSV do `utsim run --csv`, mais `objetos.csv`, e um relatório HTML autocontido com SVG em linha.
- `utsim-viewer::results` (biblioteca, sem egui): configurações da rodada, ordenação e filtro da tabela, dados dos gráficos (cores de estado de `style::state_color`), cenários montados das linhas da grade, problema de otimização montado do formulário e o comando de "Aplicar ao modelo" (um `Command::Batch` de `Set`, um passo de desfazer).
- `results_panel.rs` (binário): a janela, com os gráficos em `egui_plot` 0.37 (a versão feita para o egui 0.36 do `bevy_egui` 0.43). Os trabalhos rodam numa thread no aplicativo de mesa e no próprio quadro no navegador; o progresso volta por um canal e "Cancelar" liga um `AtomicBool`. Arquivos (CSV, HTML, experimento `.ron`) só no aplicativo de mesa.

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

## 6. Pontos de extensão já previstos

- `position` em cada objeto: layout para o 3D.
- `State` já inclui `waiting_operator`, `waiting_transport`, `down` e `collecting`.
- `Behavior` é um trait comum: blocos novos não exigem mudar o motor.
- A busca de destino percorre todo o *outbox* a cada tentativa. Isso é simples e correto,
  mas tem custo O(n) em filas muito longas e bloqueadas. Otimizar quando houver um caso real.

## 7. Referências estudadas

- Law, A. M. *Simulation Modeling and Analysis*. McGraw-Hill.
- Banks, J. et al. *Discrete-Event System Simulation*. Pearson.

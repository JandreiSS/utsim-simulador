# Arquitetura do UTSim

Este documento é a **especificação de comportamento** do UTSim. Ele foi escrito a partir
do estudo da documentação pública de simuladores comerciais (principalmente o manual do
simuladores comerciais, hoje mantido pela fabricante) e da literatura de simulação. Nenhum código ou binário
proprietário foi analisado. Onde o UTSim decide diferente, isso aparece como **"Nosso jeito"**.

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
   escolhe a porta, uma única vez por item (o "send to port" do simuladores comerciais):
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
primeiro tem preferência, como no simuladores comerciais. Tudo acontece no mesmo instante simulado.

**Portas de entrada.** `accepts(item, conteúdo, porta)` e `on_entry(item, porta)`
recebem o número da porta de entrada (0, 1, ..., na ordem das conexões). É assim que o
Combiner distingue o contêiner (porta 0) dos componentes (portas 1..k).

**Ordem dos ganchos numa transferência A → B:**
`A.conteúdo -= 1` → `A.on_exit` → `B.conteúdo += 1` → `B.on_entry(porta)` → reavaliar
quem alimenta A.

> **Nosso jeito:** no simuladores comerciais o `OnExit` roda com o item ainda contado no conteúdo.
> Aqui o conteúdo já foi atualizado quando `on_exit` roda, o que simplifica a lógica
> dos blocos ("fiquei vazio?").

> **Nosso jeito:** não existe um retorno mágico `-1` para "segurar o item".
> Um bloco simplesmente não chama `release` até querer.

## 4. Blocos da fase 0

### Source
- Cria um item, aplica rótulos (`labels`) e o libera.
- **O próximo intervalo começa quando o item sai**, como no simuladores comerciais. Se a Source fica
  bloqueada, ela para de gerar (estado `blocked`). Para ter chegadas de Poisson "puras",
  coloque uma fila depois dela.
- `first_at_zero`: o primeiro item nasce em t=0, e não depois do primeiro intervalo.
- `on_blocked` (o que fazer com um item que não consegue sair no instante em que nasce):
  - `Wait` (padrão): o comportamento acima, a Source espera e para de gerar.
  - `Drop` (**nosso jeito**, o simuladores comerciais não tem): o item é destruído e o próximo intervalo
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

> **Nosso jeito:** o simuladores comerciais espalha isso entre MTBF/MTTR, Time Tables e `stop()`/`resume()`.
> Aqui o "estar quebrado" é responsabilidade única do kernel, e o bloco só pausa os
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

## 4e. Editor (fase 3)
O editor não altera o modelo diretamente: toda ação vira um `Command` (adicionar,
remover, renomear, mover, alterar campo, ligar, desligar, lote) aplicado por um
`Document` (`utsim-edit`, sem dependência de interface gráfica).

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

> **Nosso jeito:** no simuladores comerciais o modelo é uma árvore binária editada só pela interface.
> Aqui o modelo continua sendo um texto legível, e o editor é só uma forma confortável de
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

**Despachante.** Recebe tarefas e repassa a um membro livre da equipe. Se todos estão
ocupados, guarda na fila, por prioridade e, entre iguais, por ordem de chegada. Cada
membro avisa com `MSG_IDLE` quando fica livre.

**Operadores.** Um processador com `operators: (team, count, setup, process)` pede
`count` tarefas `Operate { station }` à equipe quando a fase que precisa deles vai
começar. A fase só inicia quando todos chegam (`MSG_ARRIVED`). Liberação:

- Se setup e processo usam operadores, a mesma equipe é mantida do setup até o fim do
  processo, como no "same as setup" do simuladores comerciais.
- Se só o setup usa, eles são liberados ao fim do setup.
- A liberação acontece ao fim do processo, antes de a peça sair. Uma saída bloqueada
  não prende o operador.
- Cada peça tem seus próprios pedidos, então capacidade > 1 funciona, coisa que o
  simuladores comerciais não suporta bem.
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

> **Nosso jeito:** no simuladores comerciais as sequências de tarefas são listas genéricas (travel,
> load, unload, utilize, break...). Aqui o transportador recebe só duas tarefas de alto
> nível, `Transport` e `Operate`, e decide os passos sozinho. É mais simples de
> modelar e de testar. Sequências genéricas podem vir com o fluxo de tokens (fase 6).

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

- Manual do simuladores comerciais (fabricante): *Fixed Resources Concepts*, *Source*, *Queue*,
  *Processor*, *Task Executer Concepts*, *Key Concepts About Events*,
  *Key Concepts About Experiments*, *Key Concepts About Flow Items*.
  Índice: <(link removido)>
- simuladores livres, simulador DES 3D de código aberto (Apache 2.0): <(link removido)>
- Law, A. M. *Simulation Modeling and Analysis*. McGraw-Hill.
- Banks, J. et al. *Discrete-Event System Simulation*. Pearson.

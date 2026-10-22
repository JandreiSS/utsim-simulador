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

### Itens contêiner
`ctx.pack(contêiner, item)` tira o item do conteúdo do bloco e o coloca dentro do
contêiner (`item.container`, `contêiner.contents`). `ctx.unpack(contêiner)` devolve os
itens ao bloco atual. Um contêiner conta como **um** item no conteúdo. Destruir um
contêiner destrói o que está dentro. `ctx.clone_item` copia rótulos e o instante de criação.

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

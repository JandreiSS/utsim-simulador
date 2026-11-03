# Recursos móveis

Recursos móveis são as pessoas e os veículos que andam pelo layout: operadores, empilhadeiras, AGVs. Eles não se ligam por conexões. Os outros objetos **apontam** para eles com `transport` (levar itens) ou `operators` (trabalhar numa máquina).

Há dois objetos:

- `Transporter`: um operador, uma empilhadeira ou um AGV;
- `Dispatcher`: o despachante, que reparte o serviço entre os membros de uma equipe de transportadores.

Por onde eles andam (caminho livre desviando dos objetos e das paredes, rede de caminhos ou o modo legado do formato 1) e onde param em cada objeto está em [Movimentação e obstáculos](movimentacao.md).

## Transporter (Transportador)

```ron
(name: "empilhadeira_1", kind: Transporter(
    speed: 2.0, acceleration: Some(0.5), deceleration: Some(0.5),
    load_time: Uniform(min: 8.0, max: 12.0), unload_time: Uniform(min: 8.0, max: 12.0),
    home: true,
), position: (20.0, 10.0, 0.0))
```

| Campo | Padrão | Significado |
|---|---|---|
| `speed` | obrigatório | velocidade máxima (unidades de comprimento por unidade de tempo). Precisa ser maior que 0 |
| `acceleration` | nenhuma | aceleração. Sem ela, o veículo atinge `speed` instantaneamente |
| `deceleration` | nenhuma | desaceleração, igual |
| `capacity` | `1` | quantos itens leva por viagem |
| `load_time` | `Const(0.0)` | tempo para carregar cada item |
| `unload_time` | `Const(0.0)` | tempo para descarregar cada item |
| `home` | `false` | se `true`, quando ocioso e sem serviço, volta para a posição do próprio objeto |
| `agv` | `false` | se `true`, segue a rede de caminhos aresta por aresta, com controle de tráfego |
| `movement` | o de `run` | modo de movimentação deste recurso: `Free`, `Network`, `Legacy` (veja [Movimentação e obstáculos](movimentacao.md)) |
| `clearance` | pelo papel | distância mínima (m) aos obstáculos nos modos `Free` e `Network`: pessoa 0,6, empilhadeira 0,9, outros 0,8 |

A **posição** do transportador (campo `position` do objeto) é onde ele começa e, com `home: true`, para onde volta.

### Tempo de viagem

O tempo de uma viagem vem da distância e do perfil de velocidade:

- sem aceleração: `tempo = distância / velocidade`;
- com aceleração `a` e desaceleração `d`: `tempo = distância / velocidade + (velocidade / 2) x (1/a + 1/d)`, se a distância for longa o bastante para atingir a velocidade máxima. Se for curta, o veículo faz uma rampa triangular e nem chega à velocidade máxima.

Exemplo: 40 m a 2 m/s com `a = d = 0,5 m/s²` leva 40 / 2 + 1 x (2 + 2) = 24 s.

### Estados do transportador

`idle` (ocioso), `travel_empty` (indo buscar), `loading`, `travel_loaded` (levando), `unloading`, `utilize` (trabalhando numa máquina como operador), `blocked` (AGV esperando uma aresta), `down`, `off_shift` e `resting` (pessoa em pausa pela fadiga, veja [Pessoas](pessoas.md)).

## Levar itens: `transport`

Um objeto com `transport: "frota"` não entrega o item direto ao próximo. A sequência é:

1. quando o destino aceita o item, o motor **reserva o lugar** no destino;
2. o item fica no objeto de origem, esperando transporte (um Processor mostra o estado `waiting_transport`);
3. o motor envia o serviço ao transportador (ou ao despachante);
4. o transportador viaja até a origem, carrega, viaja até o destino, descarrega e entrega.

```ron
(name: "recebimento", kind: Queue(), transport: "frota", position: (0.0, 0.0, 0.0)),
(name: "porta_paletes", kind: Rack(bays: 6, levels: 3), position: (40.0, 0.0, 0.0)),
```

Aqui a empilhadeira leva o palete de `recebimento` até `porta_paletes`. O `transport` pode apontar para um `Transporter` (um só) ou para um `Dispatcher` (uma equipe).

Se `capacity` for maior que 1, o transportador também leva, na mesma viagem, outros serviços da fila que saem do mesmo objeto, até a sua capacidade. Ele nunca espera por mais itens para completar a carga.

### Prioridade dos pedidos de transporte

Quando vários objetos disputam a mesma equipe, a prioridade decide quem é atendido primeiro. O pedido de prioridade maior passa na frente na fila do despachante (e na fila do próprio transportador); entre iguais vale a ordem de chegada.

```ron
(name: "urgente", kind: Queue(), transport: "frota", transport_priority: 5),
(name: "pedidos", kind: Queue(), transport: "frota", transport_priority_label: "prioridade"),
```

| Campo do objeto | Padrão | Significado |
|---|---|---|
| `transport_priority` | `0` | prioridade dos pedidos de transporte deste objeto (inteiro; maior vai primeiro) |
| `transport_priority_label` | nenhum | rótulo do item cujo valor, arredondado, substitui `transport_priority`. Um item sem o rótulo usa o valor fixo |

Os dois campos só valem num objeto com `transport`. Sem eles a prioridade é 0, como antes: os modelos antigos dão o mesmo resultado. A prioridade não interrompe um serviço já entregue a um membro; para desviar quem está indo vazio, veja a preempção no [Dispatcher](#dispatcher-despachante).

## Operadores numa máquina: `operators`

Um Processor pode exigir pessoas durante o setup, o processamento ou os dois.

```ron
(name: "maq1", kind: Processor(
    setup: Exp(mean: 20.0),
    process: Exp(mean: 60.0),
    operators: OperatorUse(team: "equipe", count: 1, setup: true, process: false),
), position: (15.0, 0.0, 0.0))
```

| Campo | Padrão | Significado |
|---|---|---|
| `team` | obrigatório | `Transporter` ou `Dispatcher` de onde vêm os operadores |
| `count` | `1` | quantos operadores a fase exige |
| `setup` | `true` | o setup exige operador |
| `process` | `true` | o processamento exige operador |
| `requires` | nenhum | habilidade que o operador precisa ter |
| `priority` | `0` | prioridade dos pedidos de operador desta máquina na fila do despachante (maior vai primeiro) |
| `priority_label` | nenhum | rótulo do item em processo cujo valor substitui `priority` |

Como funciona:

- Quando a fase que precisa de operador vai começar, a máquina pede `count` operadores à equipe. Enquanto eles não chegam, a máquina fica em `waiting_operator`.
- Os operadores **andam** até a posição da máquina. Só então a fase começa.
- Se setup e processamento exigem operador, a mesma equipe fica do início do setup até o fim do processamento.
- Se só o setup exige, o operador é liberado ao fim do setup e vai atender outra máquina enquanto a peça é processada.
- A liberação acontece quando termina a última fase que exige operador. Uma saída bloqueada não prende o operador.
- Numa quebra, os operadores ficam no posto e os tempos pausam.
- Cada item tem os seus próprios pedidos, então `capacity` maior que 1 funciona.
- Com `requires: "torno"`, só pessoas com essa habilidade no perfil (`person.skills`) operam; veja [Pessoas](pessoas.md).

O [tutorial 03](tutoriais/03-operadores.md) usa isso.

## Dispatcher (Despachante)

```ron
(name: "frota", kind: Dispatcher(team: ["empilhadeira_1", "empilhadeira_2"], rule: Nearest),
    position: (20.0, 10.0, 0.0))
```

| Campo | Padrão | Significado |
|---|---|---|
| `team` | obrigatório | nomes dos transportadores da equipe (não pode ser vazia) |
| `rule` | `FirstAvailable` | como escolher entre membros livres |
| `pick` | `Priority` | qual serviço da fila um membro que fica livre pega |

As regras:

- `FirstAvailable`: o primeiro membro livre, na ordem da equipe;
- `RoundRobin`: os membros livres em rodízio;
- `Nearest`: o membro livre mais próximo de onde o serviço começa (distância em linha reta, sem considerar a rede de caminhos). Empates vão para o primeiro da equipe;
- `LeastBusy`: o membro livre com o menor tempo ocupado acumulado (do serviço entregue até o aviso de livre), para equilibrar a carga. Empates vão para o primeiro da equipe;
- `LongestIdle`: o membro livre há mais tempo. No início todos estão livres desde 0, e o primeiro da equipe vence;
- `Random`: um membro livre ao acaso, com o fluxo de números aleatórios do próprio despachante (a mesma semente dá as mesmas escolhas).

Se todos estão ocupados, o serviço espera na fila do despachante, por prioridade e, entre iguais, por ordem de chegada. Quando um membro fica livre, `pick` diz qual serviço ele pega:

- `Priority` (padrão): o da frente da fila;
- `Nearest`: entre os serviços da maior prioridade que espera, o que começa mais perto do membro (empate: o mais antigo);
- `NearestAny`: o mais perto do membro, qualquer que seja a prioridade. Diminui as viagens vazias, mas um serviço longe ou de prioridade baixa pode esperar muito (inanição): confira a espera máxima nos resultados.

```ron
(name: "frota", kind: Dispatcher(team: ["emp1", "emp2", "emp3"], rule: LeastBusy, pick: Nearest))
``` Um transportador só pode estar na equipe de um despachante.

## Rede de caminhos

No modo `Legacy` (o dos modelos do formato 1), sem rede os veículos andam em **linha reta** entre os pontos e, com `network`, seguem os corredores como descrito abaixo. No modo `Network`, a rede é a mesma, mas a ligação até ela e o ponto de parada seguem o caminho livre (veja [Movimentação e obstáculos](movimentacao.md)). A rede pode ser desenhada ou gerada com `utsim rede gerar`:

```ron
network: Some((
    nodes: [
        (name: "garagem", position: (40.0, 20.0, 0.0)),
        (name: "meio",    position: (40.0, 0.0, 0.0)),
        (name: "rack",    position: (60.0, 0.0, 0.0)),
    ],
    edges: [
        (from: "garagem", to: "meio"),
        (from: "meio", to: "rack", two_way: true, capacity: Some(1)),
    ],
)),
```

| Campo | Padrão | Significado |
|---|---|---|
| `nodes` | obrigatório | nós com nome e posição |
| `edges` | obrigatório | arestas `from` e `to` |
| `two_way` | `true` | a aresta pode ser percorrida nos dois sentidos |
| `capacity` | ilimitada | quantos veículos cabem na aresta ao mesmo tempo (usado pelos AGVs) |

Regras:

- o comprimento de uma aresta é a distância em linha reta entre os seus nós;
- as viagens seguem o menor caminho. Em empate, vale o que passa antes pelos nós listados primeiro, o que mantém o resultado determinístico;
- um ponto fora da rede entra e sai pelo **nó mais próximo**, em linha reta;
- arestas de mão única são permitidas. A rede precisa permitir ir de qualquer nó a qualquer outro, senão o `utsim check` recusa.

Mão única pode alongar muito o caminho de volta. Veja `examples/rede_de_caminhos.ron`.

No visualizador, a camada **Rede** do modo Editar desenha a rede sem escrever o arquivo: a ferramenta **Nó** coloca nós no chão, a ferramenta **Aresta** liga um nó ao outro (clicar no chão cria o nó já ligado), e o painel mostra o comprimento, o sentido e a capacidade da aresta selecionada. A linha tracejada de cada objeto atendido até um nó mostra por onde ele entra na rede. Veja [Primeiros passos](primeiros-passos.md).

## AGVs

Com `agv: true`, o transportador segue a rede aresta por aresta. Se a próxima aresta tem `capacity` e está cheia, o AGV para (estado `blocked`) e entra numa fila por ordem de chegada. Quando o veículo da frente sai, o seguinte é liberado. Uma aresta de mão dupla com `capacity: Some(1)` vale para os dois sentidos juntos, então é uma pista única. Se os veículos se travarem mutuamente, a simulação para com a mensagem "impasse". Veja `examples/agvs.ron`.

O AGV da biblioteca do editor já vem com `agv: true`, velocidade de 1,5 m/s, aceleração e desaceleração de 0,5, carga e descarga de 0,5 e `home: true`. Nas propriedades de qualquer transportador, o campo **Base (nó da rede)** escolhe um nó: o transportador passa a ficar nele e volta para lá quando está ocioso.

## A distância importa?

Sim, mas só onde o modelo a usa. É a dúvida mais comum, então vale ler com calma.

**Onde a distância entra na conta:**

- Todo deslocamento de um recurso móvel (operador, empilhadeira, AGV) leva `distância / velocidade`, mais a aceleração e a desaceleração, se houver. A distância vem das **posições dos objetos em metros**, em linha reta, ou ao longo da rede de caminhos quando ela existe.
- Uma esteira leva um item de uma ponta à outra em `comprimento / velocidade`.
- Uma ligação direta entre objetos fixos, **se você ligar o `transfer`** (veja abaixo).

**Onde a distância não entra:**

- Uma **ligação direta** entre dois objetos fixos (uma fila ligada a uma máquina, por exemplo) é, por padrão, **instantânea**. Não importa se os objetos estão a 1 metro ou a 500 metros.
- As posições também não importam para nada além do desenho no 3D e das viagens dos recursos móveis.

Na prática, há três maneiras de a distância entre dois postos custar tempo:

1. um **transportador** (campo `transport` do objeto de origem);
2. uma **esteira** (`Conveyor`) entre eles;
3. o **`transfer`**, que dá à própria ligação direta um tempo de viagem. É o mais simples quando não há veículo nem esteira.

Afastar duas máquinas que se alimentam por ligação direta sem `transfer` não muda nada no resultado. Afastar duas máquinas que dependem do mesmo operador muda muito. O [tutorial 03](tutoriais/03-operadores.md) mostra as duas coisas com números.

A posição `z` entra no cálculo, que é a distância euclidiana em três dimensões.

### O `transfer`

```ron
// padrão do modelo todo
run: (stop_time: 1000.0, transfer: (speed: 1.0)),
// ou só para os itens que saem de um objeto (vale mais que o padrão do modelo)
(name: "fila", kind: Queue(), transfer: (speed: 1.5, delay: 2.0, path: Straight)),
```

| Campo | Padrão | Significado |
|---|---|---|
| `speed` | obrigatório | velocidade do item, em unidades de comprimento por unidade de tempo. Maior que 0 |
| `delay` | `0.0` | tempo fixo somado a cada transferência |
| `path` | `Straight` | `Straight`: linha reta entre as posições. `Network`: menor caminho pela rede de caminhos |

Os itens que saem por ligações diretas levam `delay + distância / velocidade`. Durante a viagem:

- o item já saiu do objeto de origem e ainda não chegou ao destino, então **não conta no conteúdo de nenhum dos dois**;
- o **lugar no destino fica reservado**. Uma máquina de capacidade 1 fica ocupada desde o instante em que o item sai da origem, e por isso uma distância maior pode reduzir a vazão.

Detalhes:

- o `transfer` do objeto vale mais que o `run.transfer`;
- recursos móveis e objetos que têm `transport` nunca usam `transfer` (usar os dois no mesmo objeto é erro);
- o padrão do modelo não vale para esteiras nem para os objetos que alimentam uma esteira, pois a esteira já leva o item e a posição dela é o meio dela;
- num cenário de experimento, use as chaves `"objeto.transfer.speed"` e `"objeto.transfer.delay"` (ou `"run.transfer.speed"` e `"run.transfer.delay"`), se o `transfer` já existe, ou `"objeto.transfer": "(speed: 1.0)"` para criá-lo.

O exemplo `examples/transferencia_por_distancia.ron` roda a mesma linha em dois layouts, com 1 m e com 5 m entre os objetos, e `transfer: (speed: 1.0)`. A vazão é a mesma, mas o tempo no sistema passa de 8,8 s (layout compacto) para 33,1 s (espalhado), pela soma das viagens e da fila que se forma.

### Avisos de distância

O `utsim check` e o `utsim run` imprimem avisos (`aviso: ...`) que **não impedem** a rodada, sobre coisas válidas mas provavelmente não pretendidas: ligações diretas com mais de 10 m e instantâneas, distâncias zero, objetos muito longe da rede de caminhos.
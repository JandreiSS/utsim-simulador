# Formato `.ron`

Os modelos são arquivos de texto em [RON](https://github.com/ron-rs/ron) (*Rusty Object Notation*): legíveis, aceitam comentários e são fáceis de versionar. Este capítulo lista **todos os campos**, com os valores padrão.

## Regras de escrita

- Comentários: `// até o fim da linha` e `/* bloco */`.
- Campos com nome: `Exp(mean: 4.0)`. Vírgula depois de cada campo; a última é opcional.
- Listas: `[a, b, c]`. Pares: `("a", "b")`. Posições: `(x, y, z)`.
- Textos entre aspas. Dentro de um texto, `\"` é uma aspa.
- Mapas: `{ "chave": valor }`.
- Números reais levam ponto: `4.0`. Inteiros não levam: `10`.
- **Campos opcionais** (`Option`): escreva o valor direto (`capacity: 20`) ou `Some(20)`. O mesmo vale para `setup`, `dwell`, `downtime`, `shifts`, `operators`, `visual` etc. Para "nenhum", omita o campo ou escreva `None`.
- Campos com valor padrão podem ser omitidos.
- O nome do tipo no início (`Model(`) é obrigatório.
- Os nomes das variantes (`Exp`, `Queue`, `Fifo`) diferenciam maiúsculas de minúsculas.

## `Model`

```ron
Model(
    format: 2,
    name: "Meu modelo",
    description: "Texto livre",
    run: (stop_time: 1000.0),
    calendars: { "S1": (period: 1440.0, off: [...]) },
    item_types: [ (name: "A", color: "#3b82f6", asset: "caixa_pequena", labels: {...}), ... ],
    areas: [ (name: "copa", position: (30.0, 15.0, 0.0), size: (4.0, 4.0)), ... ],
    objects: [ ... ],
    connections: [ ... ],
    network: Some((...)),
    flows: [ ... ],
    fluids: Some((...)),
    spans: [ ... ],
    data: [ ... ],
    obstacles: [ ... ],
    floor_plan: Some((...)),
    aisles: [ ... ],
    floor_marking: Some((...)),
    line: "lean_office",
)
```

| Campo | Padrão | Significado |
|---|---|---|
| `format` | `1` | versão do formato do arquivo (a atual é `2`). Ausente vale 1: o arquivo passa pela migração, que grava `movement: Legacy` quando há recursos móveis. Versão mais nova que a do programa é recusada |
| `name` | obrigatório | nome do modelo |
| `description` | `""` | descrição |
| `run` | obrigatório | configurações da rodada |
| `calendars` | `{}` | calendários de turno com nome (mapa de nome para tabela de turnos), usados com `shifts: Calendar("nome")` |
| `objects` | obrigatório | lista de objetos. Um modelo vazio (sem objetos, fluxos nem fluidos) é recusado |
| `connections` | `[]` | pares `(origem, destino)`. A ordem define portas e prioridades |
| `network` | nenhuma | rede de caminhos para recursos móveis |
| `flows` | `[]` | fluxos de tokens |
| `fluids` | nenhuma | rede de fluidos |
| `spans` | `[]` | tempo entre dois pontos do fluxo: `(name, from, to)`. Veja [Resultados](resultados.md) |
| `data` | `[]` | campos de distribuição alimentados por valores observados, ajustados ao abrir o modelo: `(field: "torno.process", source: "dados/t.csv", choose: Best, criterion: Aic)`. Veja [Dados reais e ajuste automático](dados-reais.md) |
| `obstacles` | `[]` | polígonos que os recursos não atravessam nos modos `Free` e `Network`: `(name, polygon: [(x, y), ...])`, ao menos 3 pontos, nomes únicos. Veja [Movimentação e obstáculos](movimentacao.md) |
| `floor_plan` | nenhuma | registro da planta CAD (arquivo, hash, transformação, calibração, camadas); a simulação não lê o arquivo. Gravado por `utsim planta --obstaculos` |
| `aisles` | `[]` | corredores de circulação: `(name, kind: Pedestrian \| Forklift \| Mixed, points: [(x, y), ...], width, two_way: true, secondary: false, profile: None)`; geram a rede do modo `Aisles`. Veja [Corredores e faixas de segurança](corredores.md) |
| `floor_marking` | perfil Brasil | marcação do piso: `(profile: Brasil \| Branco \| Custom((...)), line_width: 0.10, crossing_stop: 2.0, floor_color: Some("#d8d2bc"))`; `floor_color` é a cor do chão no 3D (ausente, o cinza de sempre) |
| `line` | nenhuma | linha especializada do modelo (`"lean_office"`): vocabulário, biblioteca e indicadores da linha. A simulação não lê o campo. Veja [Lean Office](linhas/lean-office/index.md) |

## `run`

| Campo | Padrão | Significado |
|---|---|---|
| `stop_time` | obrigatório | tempo de parada; finito e maior que 0 |
| `warmup` | `0.0` | aquecimento; em `[0, stop_time)` |
| `seed` | `1` | semente dos números aleatórios |
| `replications` | `1` | número de replicações; pelo menos 1 |
| `sample_interval` | nenhum | intervalo de amostragem das séries temporais; maior que 0 |
| `histogram_bins` | nenhum | número de classes dos histogramas; pelo menos 1 |
| `default_decision` | nenhuma | opção usada quando o modelo pede uma decisão externa e não há agente |
| `transfer` | nenhuma | tempo de viagem padrão das ligações diretas (veja `transfer`) |
| `movement` | `Free` | modo de movimentação dos recursos móveis: `Free`, `Network`, `Aisles`, `Legacy`. `Network` exige `network`; `Aisles` exige `aisles` |

## Objeto

```ron
(name: "maquina", kind: Processor(process: Exp(mean: 3.0)), position: (8.0, 0.0, 0.0))
```

| Campo | Padrão | Significado |
|---|---|---|
| `name` | obrigatório | nome único e não vazio |
| `kind` | obrigatório | o tipo (tabelas abaixo) |
| `output` | `FirstAvailable` | `FirstAvailable`, `RoundRobin`, `Random`, `ByLabel("r")`, `Weighted([..])` ou `Script("..")` |
| `position` | `(0.0, 0.0, 0.0)` | `(x, y, z)` |
| `downtime` | nenhuma | `(mtbf: Dist, mttr: Dist)`, mais os campos opcionais de manutenção `repair_crew`, `wear`, `basis`, `repair`, `preventive`, `cost` e `oee` ([Manutenção de máquinas e OEE](manutencao.md)) |
| `shifts` | nenhuma | `(period: n, off: [(start: n, duration: n), ...], on_end: Pause)` ou `Calendar("nome")` (veja abaixo) |
| `transport` | nenhum | nome de um `Transporter`, `Crane` ou `Dispatcher` que leva os itens que saem do objeto |
| `transport_priority` | `0` | só com `transport`: prioridade dos pedidos de transporte (maior vai primeiro) |
| `transport_priority_label` | nenhum | só com `transport`: rótulo do item que substitui `transport_priority` |
| `transfer` | nenhuma | tempo de viagem das ligações diretas que saem do objeto |
| `on_entry` | nenhum | script executado na entrada de cada item |
| `visual` | nenhuma | aparência no 3D |
| `access` | lado `-y` | ponto de acesso `(x, y)` em coordenadas do objeto (metros, gira com ele), onde os recursos param nos modos `Free` e `Network`. Não vale para recursos móveis |
| `size` | área padrão do tipo | dimensões `(comprimento, largura, altura)` em metros, no sentido do objeto: mudam o ponto de acesso (e as distâncias), a capacidade de uma espera com `area_per_item` e o desenho 3D. Não vale para recursos móveis, esteiras e porta-paletes |
| `waiting_area` | nenhuma | só numa `Queue`: espera real, `Some((kind: Table))` (`Table`, `Pallet`, `Shelf`, `Floor`; opcionais `layers` e `pitch`). Cada peça ocupa um lugar, só sai a peça sem nada em cima e quem retira vai até a frente dela (veja [Objetos](objetos.md)) |
| `area_per_item` | nenhuma | só numa `Queue`: área de cada item (m²); a capacidade passa a ser a área da espera dividida por ela, só itens inteiros, e no máximo `capacity` |
| `person` | nenhum | só num `Transporter`: perfil de pessoa `(fatigue, learning, skills, walk_speed_loaded, variation, weight_label, lift, sex, height_m, mass_kg, age, experience_years, capacity, study, times)` ([Pessoas](pessoas.md)) |
| `preset` | nenhuma | predefinição da biblioteca de uma linha que criou o objeto, `"linha/predefinição"` (por exemplo `"lean_office/aprovacao"`): nome e papel do objeto nos relatórios da linha. A simulação não lê o campo |
| `department` | nenhuma | departamento do objeto (`"Compras"`): raia do fluxograma multifuncional e relatórios por departamento. Não é uma das `areas` do modelo (áreas de piso). A simulação não lê o campo |
| `symbol` | inferido | símbolo do fluxograma: `inicio`, `fim`, `operacao`, `transporte`, `inspecao`, `espera` ou `armazenagem`. Ausente, o fluxograma escolhe pelo tipo do objeto; `--simbolo` do `utsim fluxograma` ainda tem precedência |

### `transfer`

Dá **tempo de viagem às ligações diretas** entre objetos fixos, em função da distância.

```ron
transfer: (speed: 1.0, delay: 0.5, path: Straight)
```

| Campo | Padrão | Significado |
|---|---|---|
| `speed` | obrigatório | velocidade da viagem; maior que 0 |
| `delay` | `0.0` | tempo fixo somado a cada transferência; maior ou igual a 0 |
| `path` | `Straight` | `Straight` (linha reta entre as posições) ou `Network` (menor caminho pela rede de caminhos; exige `network`) |

Veja a seção "A distância importa?" em [Recursos móveis](recursos-moveis.md).

### `shifts`

| Forma | Significado |
|---|---|
| `(period: n, off: [(start: n, duration: n), ...])` | janelas fora de turno no próprio objeto |
| `(period: n, off: [...], on_end: Finish)` | o mesmo, terminando o trabalho em andamento no fim do turno |
| `Calendar("nome")` | usa o calendário `nome` de `calendars`, com o `on_end` dele |
| `Calendar("nome", Finish)` ou `(calendar: "nome", on_end: Finish)` | usa o calendário com outro `on_end` |

`on_end`: `Pause` (padrão; o trabalho congela e continua quando o turno volta) ou `Finish` (fecha a entrada, termina o trabalho em andamento e só então sai de turno). Também valem `ao_fim_do_turno`, `Pausar`, `Terminar` e `Calendario`. Veja [Quebras e turnos](quebras-e-turnos.md).

### `visual`

| Campo | Padrão | Significado |
|---|---|---|
| `asset` | o padrão do tipo | modelo da biblioteca ou arquivo `.glb` |
| `scale` | `1.0` | escala |
| `yaw` | `0.0` | giro, em graus |
| `color` | a do modelo | `"#rrggbb"` |

## Tipos de objeto

### `Source`

| Campo | Padrão |
|---|---|
| `interarrival` | obrigatório |
| `first_at_zero` | `false` |
| `max_arrivals` | sem limite |
| `labels` | `{}` (mapa de nome para `Dist`) |
| `on_blocked` | `Wait` (ou `Drop`) |
| `schedule` | nenhuma: `(cycle: n, periods: [(start: n, end: n, arrivals: ..., labels: {...}), ...])` |
| `table` | nenhuma: `(mode: Times, repeat: Once, rows: [(at: n, count: 1, labels: {...}), ...])`; não junto com `schedule` |
| `batch` | nenhum (um item por chegada): `Dist` de itens por chegada, arredondada; não junto com `table` |
| `release` | `Together` (ou `OneByOne(Dist)`: itens da chegada um a um, com esse intervalo) |
| `item_type` | nenhum: nome de um tipo de `item_types` |
| `item_type_mix` | `[]`: `[("nome", peso), ...]`, pesos >= 0 com ao menos um > 0; não junto com `item_type` |
| `text_labels` | `{}`: mapa de nome para texto fixo (também nas linhas de `table` e em `item_types`) |
| `label_exprs` | `{}`: mapa de nome para expressão, calculada na criação do item (`now`, `count`, `item`) |

Períodos de `schedule`: `start` e `end` obrigatórios, `[start, end)` dentro de `[0, cycle]`, em ordem e sem sobreposição; `arrivals` é `Default` (padrão, igual ao `interarrival`), `Interarrival(Dist)`, `Rate(r)` (`r > 0` chegadas por unidade de tempo, Poisson) ou `Closed`; `labels` (padrão `{}`) substitui os rótulos de mesmo nome. Fora dos períodos não há chegadas. Veja [Objetos](objetos.md#chegadas-por-período-schedule).

Tabela `table`: `mode` é `Times` (padrão; cada linha tem `at`, em ordem não decrescente) ou `Sequence` (sem `at`; `interarrival` entre linhas); `repeat` é `Once` (padrão), `Cycle(d)` (só `Times`, `d > 0`, `at` em `[0, d)`) ou `Loop` (só `Sequence`); `count` (padrão 1, de 0 a 1.000.000), `labels` (mapa de nome para número) e `item_type` (nome de um tipo) por linha. Veja [Objetos](objetos.md#tabela-de-chegadas-table).

### `Queue`

| Campo | Padrão |
|---|---|
| `capacity` | ilimitada (mínimo 1) |
| `discipline` | `Fifo` (ou `Lifo`) |
| `batch` | nenhum: `Batch(size: n, max_wait: t, flush: b)` com `max_wait` padrão `0.0` e `flush` padrão `false` |
| `pick` | nenhuma: `ShortestProcess` (menor tempo pela `recipe` da máquina de saída) ou `LeastSetup` (menor troca pela `changeover`); não junto com `order` |
| `order` | nenhuma: ordem por um rótulo numérico, `Desc("prioridade")` (maior primeiro) ou `Asc("prazo")` (menor primeiro). Tem precedência sobre `discipline` (não use com `Lifo`); empates e itens sem o rótulo ficam na ordem de chegada, os sem rótulo depois dos outros |
| `patience` | nenhuma: ninguém desiste. `Some(Dist)`: paciência sorteada por item ao entrar; quem ainda estiver na fila quando ela acaba ganha o rótulo `abandonou = 1` e sai do modelo (`abandoned.count` e `abandon_wait` no relatório). Item já prometido a um transportador não desiste. Veja [Serviços e atendimento](linhas/servicos/index.md) |
| `gather` | nenhum: `Some((group: "pedido", count: "linhas"))` segura os itens de mesmo valor do rótulo `group` até chegarem tantos quanto diz o rótulo `count` do primeiro, e solta um só item com os outros dentro (consolidação do pedido). Não combina com `batch` |
| `waves` | nenhuma: `Some((cycle: 1440.0, times: [420.0, 600.0]))` segura os itens e solta todos os que esperam em cada horário de `times` (em `[0, cycle)`) de cada ciclo; `waves.count` no relatório. Não combina com `batch`. Veja [Logística e armazém](linhas/logistica/index.md) |

### `Processor`

| Campo | Padrão |
|---|---|
| `capacity` | `1` (mínimo 1) |
| `setup` | nenhum |
| `process` | obrigatório |
| `operators` | nenhum: `OperatorUse(team: "x", count: 1, setup: true, process: true, requires: None, priority: None, priority_label: None, team_speedup: None)`; `count` é o número de operadores por peça; `team_speedup: Some(g)` divide o tempo da etapa por `g > 0`; `requires: "habilidade"` só aceita pessoas com ela; `priority` e `priority_label` ordenam os pedidos na fila do despachante |
| `also` | `[]`: outras equipes pedidas junto com `operators`, cada uma um `OperatorUse` (por exemplo `[OperatorUse(team: "enfermagem")]` para o médico e o enfermeiro na mesma sala). A etapa só começa quando chegam as pessoas de todas as equipes que ela usa (`setup` e `process` de cada uma); se as equipes da preparação e do processo forem diferentes, quem preparou é liberado e as do processo são chamadas. Precisa de `operators`, e cada equipe aparece uma vez. Quem chega primeiro espera os outros na estação. Veja [Saúde e hospital](linhas/saude/index.md) |
| `changeover` | nenhum: preparação em toda peça. `Some((label: "produto", matrix: [(from: 1.0, to: 2.0, time: Const(15.0))]))`: o `setup` só acontece quando o rótulo `label` da peça muda em relação à peça anterior, com o tempo de cada troca (de `from` para `to`) na `matrix`; troca sem linha na matriz usa o `setup`, que é obrigatório. A máquina começa pronta para o primeiro produto; com `capacity` maior que 1 vale o último produto que começou. Conta `changeovers` no relatório do bloco. Veja [Manufatura discreta](linhas/manufatura/index.md) |
| `recipe` | nenhum: o mesmo `process` para todos. `Some((label: "produto", times: [(value: 1.0, time: Const(180.0))]))`: o item cujo rótulo `label` vale `value` processa pelo `time` daquela receita; os itens sem receita na tabela usam o `process`. Aceita o nome `receita`. Veja [Processos contínuos e batelada](linhas/processos/index.md) |

### `Combiner`

| Campo | Padrão |
|---|---|
| `components` | obrigatório (lista de inteiros a partir de 1) |
| `mode` | `Pack` (ou `Join`, `Batch`) |
| `setup` | nenhum |
| `process` | obrigatório |

### `Separator`

| Campo | Padrão |
|---|---|
| `mode` | `Unpack` (ou `Split(n)`, com `n` de 1 a 1.000.000, ou `SplitBy(count: "linhas", group: "pedido")`: o item vira tantos quanto diz o seu rótulo `count`, arredondado, de 1 a 10.000, todos com o número da divisão no rótulo `group` e o índice no rótulo `linha`) |
| `process` | obrigatório |

### `Sink`

Sem campos. Escreva `kind: Sink`.

### `Transporter`

| Campo | Padrão |
|---|---|
| `speed` | obrigatório (maior que 0) |
| `acceleration` | nenhuma (se dada, maior que 0) |
| `deceleration` | nenhuma (se dada, maior que 0) |
| `capacity` | `1` |
| `load_time` | `Const(0.0)` |
| `unload_time` | `Const(0.0)` |
| `home` | `false` |
| `agv` | `false` |
| `movement` | o de `run` |
| `clearance` | pelo papel: pessoa 0,6, empilhadeira 0,9, outros 0,8 (finita, maior ou igual a 0) |
| `home_at` / `home_object` | nenhum: casa num ponto, ou num objeto ou área (um ou outro; ligam a volta para casa) |
| `break_at` / `break_object` | nenhum: local de pausa num ponto, ou num objeto ou área (um ou outro) |
| `multitask` | nenhuma: um serviço por vez. `Some((max_open: n, slice: Dist, switch: Dist))`: até `n` serviços de operador abertos ao mesmo tempo (`max_open` maior ou igual a 1; com 1, igual a não ter o campo), trabalhados em turnos de `slice` (não pode ser `Const(0.0)`), perdendo `switch` (padrão `Const(0.0)`) a cada troca. Veja [Multitarefa](linhas/lean-office/multitarefa.md) |

### `Crane` (ponte rolante)

| Campo | Padrão |
|---|---|
| `bridge`, `trolley`, `hoist` | obrigatórios: `(speed, acceleration, deceleration)` de cada eixo |
| `hook_height` | obrigatória (maior que 0, até a altura do `size`) |
| `attach`, `detach` | `Const(0.0)` |
| `capacity_t` | obrigatória (toneladas, maior que 0) |
| `motion` | `Safe` (ou `Sequential`, `Parallel`) |
| `weight_label` | `"weight"` |
| `home` | `false` |
| `load_zone` | `2.0` (metade do lado da área sob a carga, m) |
| `operator` | `Cabin` (ou `Pendant(team: ...)`, `Radio(team: ...)`) |
| `bridge_gap` | `3.0` (m entre os ganchos de duas pontes no mesmo caminho) |
| `keep_clear` | `false` (com `true`, as pessoas não passam no caminho da carga) |

Precisa de `size` (curso da ponte, vão, altura útil). Veja [Ponte rolante](ponte-rolante.md).

### `Dispatcher`

| Campo | Padrão |
|---|---|
| `team` | obrigatório (lista de nomes de `Transporter` ou `Crane`; não vazia; um transportador só em uma equipe) |
| `rule` | `FirstAvailable` (ou `RoundRobin`, `Nearest`, `LeastBusy`, `LongestIdle`, `Random`) |
| `pick` | `Priority` (ou `Nearest`, `NearestAny`): qual serviço da fila o membro que fica livre pega |
| `preempt` | `Never` (ou `Empty`: um pedido de prioridade maior tira o serviço de um membro que vai vazio buscá-lo; `Full`: também tira um membro que está trabalhando num posto, e o posto guarda o tempo que faltava e espera outro operador, veja [Interrupções](linhas/lean-office/interrupcoes.md)) |
| `metrics` | `false` (com `true`: `request_wait`, `requests.count`, `member.<nome>.requests` e `.busy` no relatório) |

### `Conveyor`

| Campo | Padrão |
|---|---|
| `length` | obrigatório (maior que 0) |
| `speed` | obrigatório (maior que 0) |
| `item_length` | obrigatório (maior que 0 e no máximo `length`) |
| `accumulating` | `true` |

### `Rack`

| Campo | Padrão |
|---|---|
| `bays` | obrigatório (mínimo 1) |
| `levels` | obrigatório (mínimo 1) |
| `dwell` | nenhum |
| `placement` | `FirstFree` (ou `Random`, ou `Abc(a: 0.2, b: 0.3)`: as frações `a` e `b` das posições de acesso mais rápido são das classes A e B, pelo rótulo `classe` 1, 2 ou 3; sem rótulo, C) |
| `access` | nenhum: `Some((per_bay: 0.2, per_level: 0.5))` soma `coluna × per_bay + nível × per_level` à permanência e grava `access_time` (e `access_time.A`, `.B`, `.C`) |
| `bay_width` | `1.0` (maior que 0) |
| `level_height` | `1.0` (maior que 0) |

## Distribuições (`Dist`)

`Const(v)`, `Uniform(min, max)`, `Exp(mean)`, `Triangular(min, mode, max)`, `Normal(mean, sd)`, `LogNormal(mean, sd)`, `Gamma(shape, scale)`, `Erlang(k, mean)`, `Weibull(shape, scale)`, `Discrete([(valor, peso), ...])`, `Beta(alpha, beta, min, max)`, `LogLogistic(shape, scale)`, `PearsonV(shape, scale)`, `PearsonVI(shape1, shape2, scale)`, `Shifted(offset, dist)`, `Poisson(mean)`, `Geometric(p)`, `Binomial(n, p)`, `NegBinomial(r, p)` e `Empirical([...])`. Detalhes em [Distribuições](distribuicoes.md).

## `network`

```ron
network: Some((
    nodes: [(name: "a", position: (0.0, 0.0, 0.0))],
    edges: [(from: "a", to: "b", two_way: true, capacity: None)],
)),
```

| Campo | Padrão |
|---|---|
| `nodes[].name` | obrigatório, único |
| `nodes[].position` | obrigatório |
| `edges[].from`, `edges[].to` | obrigatórios, nós existentes |
| `edges[].two_way` | `true` |
| `edges[].capacity` | ilimitada |
| `generated` | nenhum | só na rede gerada por `utsim rede gerar`: `(fingerprint: "fnv1a64:...", clearance: 0.9)` |

A rede precisa permitir ir de qualquer nó a qualquer outro.

## `flows`

| Campo | Padrão |
|---|---|
| `name` | obrigatório, diferente dos nomes de objetos |
| `resources` | `[]`: `(name, capacity)` com `capacity` maior que 0 |
| `activities` | obrigatório: `(name, kind, next)` com `next` padrão `[]` |

Atividades (`kind`):

| Atividade | Campos (padrão) |
|---|---|
| `Source` | `interarrival`, `first_at_zero` (`false`), `limit` (nenhum) |
| `Schedule` | `times`: `[(instante, quantidade), ...]` |
| `OnEvent` | `block`, `event` (`Entry` ou `Exit`) |
| `Delay` | `time` |
| `Decide` | `rule`: `Probability([..])`, `ByLabel("r")`, `Condition([..])` ou `Agent(observation: [..])` |
| `Assign` | `labels`: mapa de nome para expressão |
| `Acquire` | `resource`, `count` (`1`) |
| `Release` | `resource`, `count` (`1`) |
| `WaitFor` | `block`, `event` |
| `CreateItem` | `into`, `port` (`0`), `labels` (`{}`) |
| `Batch` | `size` |
| `Sink` | sem campos |

## `fluids`

| Campo | Padrão |
|---|---|
| `name` | `"fluidos"` |
| `nodes` | obrigatório: `(name, kind, position)` com `position` padrão `(0,0,0)` |
| `links` | obrigatório: `(origem, destino)`, sem ciclos |

Tipos de nó: `Source(rate)`, `Tank(capacity, initial: 0, max_in: nenhum, max_out: nenhum)`, `Pipe(max_rate)`, `Sink(max_rate: nenhum)`, `FromItems(object, volume)` e `ToItems(object, volume, max_rate: nenhum)`.

## Arquivo de experimento (`Experiment`)

| Campo | Padrão |
|---|---|
| `name` | obrigatório |
| `model` | obrigatório (caminho relativo ao arquivo) |
| `replications` | as do modelo |
| `scenarios` | obrigatório: `(name, set)`, com `set` padrão `{}` |
| `measures` | `[]` (todas) |

Chaves de `set` além de `<objeto>.<campo>` e `run.<campo>`: `<fonte>.schedule.cycle`, `<fonte>.schedule[n]` e `<fonte>.schedule[n].start`, `.end`, `.arrivals`, `.rate`, `.interarrival`, `.labels` (períodos contados a partir de 1); `<fonte>.table.rows[n]` e `<fonte>.table.rows[n].at`, `.count`, `.labels`, `.labels.<nome>` (linhas contadas a partir de 1); `<objeto>.shifts.on_end`; `calendars.<nome>` (a tabela inteira, ou `None` para apagar) e `calendars.<nome>.on_end`.

## Arquivo de otimização (`OptProblem`)

| Campo | Padrão |
|---|---|
| `name` | obrigatório |
| `model` | obrigatório |
| `replications` | `5` |
| `variables` | obrigatório: `(key, domain)` |
| `objective` | obrigatório: `(sense: Minimize, terms: [(métrica, peso)], costs: [(variável, custo)])`, com `sense` padrão `Minimize` e `costs` padrão `[]` |
| `constraints` | `[]`: `(metric, min, max)`, `min` e `max` opcionais |
| `algorithm` | `Grid` (ou `Annealing(iterations: n)`, `Genetic(population: p, generations: g)`) |
| `budget` | `200` |
| `seed` | `1` |
| `refine` | nenhum: `(top: n, replications: n)` |

Domínios: `Integer(min, max, step: 1)`, `Continuous(min, max, template: "{x}", steps: 5)` e `Choice([..])`.

## Validação

`utsim check modelo.ron` aponta todos os problemas de uma vez, cada um com o nome do objeto. Entre os mais comuns:

- nomes repetidos ou vazios;
- objeto sem saída conectada;
- conexão para objeto que não existe;
- parâmetros fora do limite (`Exp exige mean > 0`);
- `Combiner` com número de entradas errado;
- períodos de `schedule` fora do ciclo, fora de ordem ou com taxa menor ou igual a 0;
- `shifts: Calendar("x")` com um calendário que não existe;
- `transport` apontando para algo que não é `Transporter` nem `Dispatcher`;
- usar `transport` e `transfer` no mesmo objeto;
- rede de caminhos em que algum nó não alcança os outros;
- script com erro de sintaxe.
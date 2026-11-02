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
    format: 1,
    name: "Meu modelo",
    description: "Texto livre",
    run: (stop_time: 1000.0),
    objects: [ ... ],
    connections: [ ... ],
    network: Some((...)),
    flows: [ ... ],
    fluids: Some((...)),
    spans: [ ... ],
)
```

| Campo | Padrão | Significado |
|---|---|---|
| `format` | `1` | versão do formato do arquivo. Versão mais nova que a do programa é recusada |
| `name` | obrigatório | nome do modelo |
| `description` | `""` | descrição |
| `run` | obrigatório | configurações da rodada |
| `objects` | obrigatório | lista de objetos. Um modelo vazio (sem objetos, fluxos nem fluidos) é recusado |
| `connections` | `[]` | pares `(origem, destino)`. A ordem define portas e prioridades |
| `network` | nenhuma | rede de caminhos para recursos móveis |
| `flows` | `[]` | fluxos de tokens |
| `fluids` | nenhuma | rede de fluidos |
| `spans` | `[]` | tempo entre dois pontos do fluxo: `(name, from, to)`. Veja [Resultados](resultados.md) |

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
| `downtime` | nenhuma | `(mtbf: Dist, mttr: Dist)` |
| `shifts` | nenhuma | `(period: n, off: [(start: n, duration: n), ...])` |
| `transport` | nenhum | nome de um `Transporter` ou `Dispatcher` que leva os itens que saem do objeto |
| `transfer` | nenhuma | tempo de viagem das ligações diretas que saem do objeto |
| `on_entry` | nenhum | script executado na entrada de cada item |
| `visual` | nenhuma | aparência no 3D |

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

### `Queue`

| Campo | Padrão |
|---|---|
| `capacity` | ilimitada (mínimo 1) |
| `discipline` | `Fifo` (ou `Lifo`) |
| `batch` | nenhum: `Batch(size: n, max_wait: t, flush: b)` com `max_wait` padrão `0.0` e `flush` padrão `false` |

### `Processor`

| Campo | Padrão |
|---|---|
| `capacity` | `1` (mínimo 1) |
| `setup` | nenhum |
| `process` | obrigatório |
| `operators` | nenhum: `OperatorUse(team: "x", count: 1, setup: true, process: true)` |

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
| `mode` | `Unpack` (ou `Split(n)`, com `n` de 1 a 1.000.000) |
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

### `Dispatcher`

| Campo | Padrão |
|---|---|
| `team` | obrigatório (lista de nomes de `Transporter`; não vazia; um transportador só em uma equipe) |
| `rule` | `FirstAvailable` (ou `RoundRobin`, `Nearest`) |

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
| `placement` | `FirstFree` (ou `Random`) |
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
- `transport` apontando para algo que não é `Transporter` nem `Dispatcher`;
- usar `transport` e `transfer` no mesmo objeto;
- rede de caminhos em que algum nó não alcança os outros;
- script com erro de sintaxe.
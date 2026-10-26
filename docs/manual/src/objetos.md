# Objetos

Um modelo é uma lista de **objetos** ligados por **conexões**. Este capítulo descreve cada tipo, seus parâmetros e quando usá-lo. Os nomes dos campos são os do arquivo `.ron`; a tabela completa com os valores padrão está no [formato `.ron`](referencia-ron.md).

Os tempos (`interarrival`, `process`, `setup` e outros) são [distribuições](distribuicoes.md). Um tempo sorteado negativo, como pode ocorrer com a `Normal`, vira zero.

## Campos comuns a todos os objetos

| Campo | Padrão | Para que serve |
|---|---|---|
| `name` | obrigatório | nome único do objeto. As conexões e as métricas usam esse nome |
| `kind` | obrigatório | o tipo e seus parâmetros (veja abaixo) |
| `output` | `FirstAvailable` | regra de escolha da porta de saída. Veja [Conexões e regras de saída](conexoes.md) |
| `position` | `(0.0, 0.0, 0.0)` | posição `(x, y, z)` no layout, em metros |
| `downtime` | nenhum | quebras aleatórias. Veja [Quebras e turnos](quebras-e-turnos.md) |
| `shifts` | nenhum | janelas fora de turno. Veja [Quebras e turnos](quebras-e-turnos.md) |
| `transport` | nenhum | transportador ou despachante que leva os itens que saem deste objeto |
| `on_entry` | nenhum | script executado quando um item entra. Veja [Fluxos de tokens e scripts](fluxos-e-scripts.md) |
| `visual` | nenhum | aparência no 3D. Veja [3D realista](visual-3d.md) |

## Source (Chegada)

Cria itens. É por aqui que o trabalho entra no modelo.

```ron
(name: "chegada", kind: Source(interarrival: Exp(mean: 5.0)))
```

| Campo | Padrão | Significado |
|---|---|---|
| `interarrival` | obrigatório | tempo entre chegadas |
| `first_at_zero` | `false` | se `true`, o primeiro item nasce em t = 0, e não depois do primeiro intervalo |
| `max_arrivals` | sem limite | número máximo de itens criados |
| `labels` | vazio | rótulos sorteados para cada item, por exemplo `{ "tipo": Discrete([(1.0, 0.7), (2.0, 0.3)]) }` |
| `on_blocked` | `Wait` | o que fazer com o item que não consegue sair no instante em que nasce |

Os dois valores de `on_blocked`:

- `Wait`: a fonte segura o item e para de gerar até ele sair. O próximo intervalo só começa quando o item sai. Para ter chegadas "puras", ponha uma fila logo depois da fonte.
- `Drop`: o item é descartado e contado como perdido, e o relógio de chegadas continua. Serve para modelar clientes que vão embora quando a sala de espera está cheia.

**Use** uma Source para cada fluxo de entrada independente (um tipo de peça, um cliente, um caminhão). Estados: `generating`, `blocked` e `idle` (quando chega a `max_arrivals`).

## Queue (Fila)

Guarda itens enquanto o próximo objeto não está pronto.

```ron
(name: "fila", kind: Queue(capacity: 20, discipline: Lifo))
```

| Campo | Padrão | Significado |
|---|---|---|
| `capacity` | ilimitada | número máximo de itens. Quando enche, bloqueia quem vem antes |
| `discipline` | `Fifo` | `Fifo` (primeiro que entra, primeiro que sai) ou `Lifo` (último que entra, primeiro que sai) |
| `batch` | nenhum | libera os itens em grupos (veja abaixo) |

Com `batch: Batch(size: 4, max_wait: 10.0, flush: true)`:

- `size`: quantos itens formam um lote;
- `max_wait`: tempo máximo de espera do primeiro item do lote. Depois dele, o lote parcial é liberado. Com 0, espera para sempre;
- `flush`: se `true`, a entrada fecha depois que um lote é liberado, até todos os seus itens saírem.

Se a fila tem `capacity` menor que `batch.size`, o lote nunca fecharia, e o `utsim check` recusa o modelo.

**Use** uma Queue entre duas etapas para absorver variações. Uma fila de capacidade pequena cria **bloqueio**: a etapa anterior para quando o buffer enche. Estados: `empty` e `releasing`. A fila é o lugar certo para medir o tempo de espera (`staytime`).

## Processor (Processo)

Uma ou mais máquinas ou postos de trabalho em paralelo.

```ron
(name: "furacao", kind: Processor(capacity: 2, setup: Const(0.5), process: Exp(mean: 3.6)))
```

| Campo | Padrão | Significado |
|---|---|---|
| `capacity` | `1` | quantos itens são processados ao mesmo tempo (máquinas idênticas em paralelo) |
| `setup` | nenhum | tempo de preparação antes de cada item |
| `process` | obrigatório | tempo de processamento |
| `operators` | nenhum | operadores necessários. Veja [Recursos móveis](recursos-moveis.md) |

Com `capacity` maior que 1, os itens são processados em paralelo e **o conteúdo médio do objeto é o número médio de máquinas ocupadas**. Divida por `capacity` para obter a utilização por máquina.

Estados: `idle`, `setup`, `processing`, `blocked` (terminou e o destino está fechado), `waiting_operator`, `waiting_transport`, `down` e `off_shift`. Com mais de uma máquina, o estado mostrado é o mais produtivo entre os itens presentes.

**Use** para qualquer etapa em que o item gasta tempo: usinagem, atendimento no caixa, inspeção.

## Combiner (Montagem)

Junta itens: um contêiner e seus componentes.

```ron
(name: "montagem", kind: Combiner(components: [4], mode: Pack,
    setup: Uniform(min: 0.2, max: 0.5), process: Triangular(min: 1.0, mode: 1.5, max: 2.5)))
```

| Campo | Padrão | Significado |
|---|---|---|
| `components` | obrigatório | quantos itens cada porta de componentes fornece. `[4]` quer dizer uma porta com 4 itens |
| `mode` | `Pack` | o que acontece com os componentes |
| `setup` | nenhum | tempo de preparação |
| `process` | obrigatório | tempo de montagem |

A **porta 0** recebe o contêiner (por exemplo, o palete). As portas 1, 2... recebem os componentes, na ordem das conexões. O número de conexões que chegam precisa ser `components.len() + 1`, senão o `utsim check` reclama.

Os modos:

- `Pack`: os componentes vão para dentro do contêiner, e só o contêiner sai;
- `Join`: os componentes são destruídos e o contêiner sai (uma peça que recebe um parafuso, por exemplo);
- `Batch`: o contêiner e os componentes saem, o contêiner primeiro, ainda separados.

**Use** para paletizar, embalar e montar. Exemplo completo: `examples/montagem_paletes.ron`.

## Separator (Desmontagem)

O inverso do Combiner. Processa um item por vez e o separa.

```ron
(name: "descarga", kind: Separator(mode: Split(6), process: Uniform(min: 180.0, max: 300.0)))
```

| Campo | Padrão | Significado |
|---|---|---|
| `mode` | `Unpack` | `Unpack` tira o que está embalado no item; `Split(n)` libera o item mais `n - 1` cópias (com os mesmos rótulos) |
| `process` | obrigatório | tempo de processamento |

No `Unpack`, o contêiner sai primeiro e depois o conteúdo, na ordem de embalagem. Para mandar contêiner e conteúdo para lugares diferentes, use `output: ByLabel("tipo")` ou `RoundRobin` no Separator.

**Use** `Unpack` para desembalar e `Split(n)` para transformar um caminhão em n paletes.

## Sink (Saída)

Recebe tudo e destrói o item. Registra o `time_in_system`: o tempo desde a criação do item até a chegada na saída.

```ron
(name: "saida", kind: Sink)
```

Não tem parâmetros e não tem saída. **Todo modelo precisa de pelo menos uma Sink**, pois um objeto sem saída conectada deixaria os itens presos, e o `utsim check` recusa o modelo (a Sink é a única exceção).

## Conveyor (Esteira) e Rack (Porta-paletes)

Veja o capítulo [Esteiras e porta-paletes](esteiras-e-porta-paletes.md).

## Transporter (Transportador) e Dispatcher (Despachante)

São os recursos móveis: operadores, empilhadeiras, AGVs. Não se ligam por conexões. Veja [Recursos móveis](recursos-moveis.md).

## Quando usar cada objeto

| Eu quero modelar... | Use |
|---|---|
| a chegada de clientes, peças ou caminhões | `Source` |
| uma sala de espera, um buffer entre máquinas | `Queue` |
| uma máquina, um caixa, um posto | `Processor` |
| N máquinas iguais atendendo a mesma fila | `Processor` com `capacity: N` |
| paletizar, embalar, montar | `Combiner` |
| desembalar, dividir um caminhão em paletes | `Separator` |
| o fim do fluxo (expedição, cliente atendido) | `Sink` |
| transporte contínuo de peças | `Conveyor` |
| armazenagem endereçada | `Rack` |
| pessoas ou veículos que andam | `Transporter` e `Dispatcher` |
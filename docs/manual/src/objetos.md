# Objetos

Um modelo é uma lista de **objetos** ligados por **conexões**. Este capítulo descreve cada tipo, seus parâmetros e quando usá-lo. Os nomes dos campos são os do arquivo `.ron`; a tabela completa com os valores padrão está no [formato `.ron`](referencia-ron.md).

Os tempos (`interarrival`, `process`, `setup` e outros) são [distribuições](distribuicoes.md). Uma duração sorteada negativa, como pode ocorrer com a `Normal`, vira zero.

## Campos comuns a todos os objetos

| Campo | Padrão | Para que serve |
|---|---|---|
| `name` | obrigatório | nome único do objeto. As conexões e as métricas usam esse nome |
| `kind` | obrigatório | o tipo e seus parâmetros (veja abaixo) |
| `output` | `FirstAvailable` | regra de escolha da porta de saída. Veja [Conexões e regras de saída](conexoes.md) |
| `position` | `(0.0, 0.0, 0.0)` | posição `(x, y, z)` no layout, em metros |
| `downtime` | nenhum | quebras aleatórias. Veja [Quebras e turnos](quebras-e-turnos.md) |
| `shifts` | nenhum | janelas fora de turno, ou um calendário do modelo (`Calendar("nome")`). Veja [Quebras e turnos](quebras-e-turnos.md) |
| `transport` | nenhum | transportador ou despachante que leva os itens que saem deste objeto |
| `transfer` | nenhum | tempo de viagem (`speed`, `delay`, `path`) das ligações diretas que saem deste objeto. Veja [Recursos móveis](recursos-moveis.md) |
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
| `schedule` | nenhuma | chegadas por período: a taxa ou o tempo entre chegadas muda com a hora (veja abaixo) |
| `table` | nenhuma | tabela de chegadas: instantes fixos ou sequência ordenada de linhas com quantidade e rótulos (veja abaixo) |
| `batch` | um item | itens criados em cada chegada (`Dist`, arredondada, mínimo 0), por exemplo `Const(6.0)` para caixas de seis. Sem `table` |
| `release` | `Together` | como os itens de uma chegada (lote ou linha da tabela) saem: `Together` (todos no instante da chegada) ou `OneByOne(Dist)` (um a um, com esse intervalo entre eles) |
| `item_type` | nenhum | tipo de todos os itens (um nome de `item_types`, veja abaixo) |
| `item_type_mix` | `[]` | tipos sorteados para cada item, com pesos: `[("A", 0.7), ("B", 0.3)]`. Não junto com `item_type` |

Os dois valores de `on_blocked`:

- `Wait`: a fonte segura o item e para de gerar até ele sair. O próximo intervalo só começa quando o item sai. Para ter chegadas "puras", ponha uma fila logo depois da fonte.
- `Drop`: o item é descartado e contado como perdido, e o relógio de chegadas continua. Serve para modelar clientes que vão embora quando a sala de espera está cheia.

Na interface 3D, uma fonte mostra sempre as suas portas no Editar (as saídas também mostram a de entrada), tem sobre ela uma caixinha na cor que os seus itens recebem e, no rótulo, como cria os itens (`chegada · tabela`, `· sequência`, `· horário`, `· lote`) e, no Simular, quantos criou até o instante mostrado (`· 37 criados`). Uma fonte com tipos de item tem em volta da caixinha um anel com uma fatia por tipo, na cor do tipo e do tamanho da sua parte. Os itens que a fonte segura (com `on_blocked: Wait` e a saída cheia, ou um lote esperando para sair) ficam lado a lado na boca da porta de saída, e com `Drop` a caixinha pisca em vermelho a cada item perdido.

**Use** uma Source para cada fluxo de entrada independente (um tipo de peça, um cliente, um caminhão). Estados: `generating`, `blocked`, `idle` (quando chega a `max_arrivals`) e `off_shift` (fora do horário das chegadas por período, ou fora de turno).

### Chegadas por período (`schedule`)

Quando o ritmo de chegada muda ao longo do dia (a manhã é cheia, o almoço é calmo), uma fonte só basta: dê a ela uma tabela de períodos que se repete a cada ciclo.

```ron
(name: "chegada", kind: Source(
    interarrival: Exp(mean: 4.0),
    labels: { "classe": Discrete([(1.0, 0.1), (2.0, 0.9)]) },
    schedule: (cycle: 1440.0, periods: [
        (start: 0.0, end: 60.0, arrivals: Rate(0.1667)),                   // 10 por hora
        (start: 60.0, end: 240.0, arrivals: Rate(0.4333)),                 // 26 por hora
        (start: 240.0, end: 300.0),                                        // igual ao padrão
        (start: 300.0, end: 420.0, arrivals: Interarrival(Uniform(min: 3.0, max: 7.0))),
        (start: 420.0, end: 600.0, arrivals: Closed),                      // sem chegadas
        (start: 600.0, end: 720.0, labels: { "classe": Discrete([(1.0, 0.3), (2.0, 0.7)]) }),
    ]),
))
```

| Campo | Padrão | Significado |
|---|---|---|
| `cycle` | obrigatório | tamanho do ciclo que se repete (1440 para um dia em minutos, 10080 para uma semana) |
| `periods` | obrigatório | períodos `[start, end)` dentro do ciclo, em ordem e sem sobreposição |
| `periods[].arrivals` | `Default` | `Default` (igual ao padrão: usa o `interarrival` da fonte), `Interarrival(Dist)` (tempo entre chegadas próprio), `Rate(r)` (chegadas de Poisson com `r` chegadas por unidade de tempo) ou `Closed` (sem chegadas) |
| `periods[].labels` | `{}` | rótulos do período: substituem os rótulos padrão de mesmo nome; os outros rótulos padrão continuam |

Regras:

- **Fora de qualquer período não há chegadas**, assim como num período `Closed`. A fonte mostra o estado `off_shift` ("fora do horário"). No exemplo acima não chega ninguém de 12h00 às 07h00.
- **Taxa (`Rate`).** As chegadas formam um processo de Poisson com a taxa do período. A taxa é por unidade de tempo do modelo: 26 por hora num modelo em minutos é `Rate(0.4333)` (26 / 60). A troca de taxa na fronteira é exata (o processo de Poisson não tem memória), então a contagem esperada em cada período é `taxa x duração`, também nos primeiros e nos últimos minutos.
- **Troca de período.** Quando um período começa, a chegada que estava marcada é descartada e uma nova é sorteada a partir do início do período, com o processo do novo período. Nada passa de um período para o outro. A exceção são dois períodos seguidos com o mesmo processo (por exemplo, dois `Default`, ou a mesma taxa): a chegada marcada continua valendo.
- **Rótulos** são sorteados com os do período em que o item nasce.
- `first_at_zero` só cria o item em t = 0 se um período com chegadas começar em 0.
- Um item que espera para sair da fonte (`Wait`) na troca de período não é afetado: o próximo intervalo é sorteado quando ele sai, com o processo do período em vigor.
- Uma quebra ou um turno da própria fonte (`downtime`, `shifts`) continua pausando o relógio de chegadas, como sem tabela.
- Os erros apontam o período, contado a partir de 1: `chegada: schedule[2]: começa em 40 antes do fim do período anterior (50); os períodos precisam estar em ordem e sem sobreposição`. A taxa precisa ser maior que 0 (para não ter chegadas, use `Closed`).

No editor, a seção **Chegadas por período** das propriedades da fonte mostra a tabela: ciclo, uma linha por período com início, fim, modo (igual ao padrão, distribuição, taxa ou sem chegadas) e valor, o botão "copiar do padrão" e o número esperado de chegadas por período e por ciclo, para conferir com os dados. Com ciclo de 1440 (um dia em minutos) os horários aparecem e são digitados como hh:mm e as taxas são por hora; com 10080 (uma semana) como "dia N hh:mm". O modelo não tem unidade de tempo própria: com outros ciclos os horários e as taxas ficam na unidade do modelo.

Na interface Godot, o painel de propriedades mostra o horário como uma faixa do ciclo com os períodos em blocos coloridos pelo modo (padrão, distribuição, taxa, fechado; o tempo fora dos períodos aparece hachurado), uma linha por período (início, fim, modo e a taxa em itens por hora ou a distribuição), os botões "+ Período" e × e um gráfico de barras das chegadas esperadas em cada período, com o total por ciclo. Os erros aparecem na linha do período. Os blocos da faixa se arrastam com o mouse: perto da borda muda o início ou o fim, no meio move o período inteiro, encaixando de 15 em 15 minutos; ao soltar, a mudança é um passo de desfazer. O botão `{ }` troca o editor pela linha de texto RON.

Nos experimentos, as chaves `chegada.schedule`, `chegada.schedule.cycle`, `chegada.schedule[n]` e `chegada.schedule[n].<campo>` (`start`, `end`, `arrivals`, `rate`, `interarrival`, `labels`) mudam a tabela; por exemplo `"chegada.schedule[2].rate": "0.5"`.

### Tabela de chegadas (`table`)

Quando as chegadas vêm de uma lista (caminhões com hora marcada, ordens de produção numa ordem definida, o mix de pedidos do dia), dê à fonte uma tabela. Há dois modos:

- **`Times` (horário)**: cada linha cria `count` itens no instante `at`. Com `repeat: Cycle(d)` a tabela se repete a cada `d` unidades de tempo e os `at` ficam dentro de `[0, d)`; com `Once` (o padrão) a fonte cria as linhas uma vez e fica `idle`.
- **`Sequence` (sequência)**: as linhas não têm instante. A fonte percorre a lista em ordem, uma linha por chegada, e sorteia `interarrival` entre uma linha e a próxima. Com `repeat: Loop` volta à primeira linha; com `Once` para no fim.

```ron
(name: "pedidos", kind: Source(
    interarrival: Exp(mean: 5.0),            // só no modo Sequence, entre linhas
    table: (
        mode: Times,
        repeat: Cycle(1440.0),
        rows: [
            (at: 480.0, count: 12, labels: { "prioridade": 2.0 }),
            (at: 600.0, count: 5),
            (at: 840.0, count: 8, labels: { "prazo": 1080.0 }),
        ],
    ),
))
```

| Campo | Padrão | Significado |
|---|---|---|
| `mode` | `Times` | `Times` (horário) ou `Sequence` (sequência) |
| `repeat` | `Once` | `Once` (uma vez), `Cycle(d)` (só `Times`: repete a cada `d`) ou `Loop` (só `Sequence`: volta ao início) |
| `rows[].at` | obrigatório em `Times` | instante da linha, em ordem não decrescente; não vale em `Sequence` |
| `rows[].count` | `1` | itens criados de uma vez pela linha (0 é aceito: a linha não cria nada) |
| `rows[].labels` | `{}` | rótulos constantes dos itens da linha; substituem os rótulos sorteados de mesmo nome |
| `rows[].item_type` | o da fonte | tipo dos itens da linha (veja "Tipos de item") |

Regras:

- Itens criados no mesmo instante saem na ordem da tabela.
- No modo `Times` os instantes são fixos: com `on_blocked: Wait`, os itens que não conseguem sair esperam na fonte, e a linha seguinte chega no seu horário mesmo assim. Com `Drop`, os que não saem são perdidos.
- No modo `Sequence` com `Wait`, o intervalo até a próxima linha começa quando o último item da linha saiu, como no modo normal. `first_at_zero: true` cria a primeira linha em t = 0.
- `max_arrivals` limita o número de chegadas (linhas usadas), não de itens.
- `table` e `schedule` juntos é erro: use um ou outro. Uma sequência em laço precisa de `interarrival` com média maior que 0.
- Os erros apontam a linha, contada a partir de 1: `pedidos: table.rows[2]: at (300) antes da linha anterior (480); as linhas precisam estar em ordem de at`.
- A fonte com tabela ganha as métricas `arrivals.count` (chegadas, isto é, linhas usadas) e `entered.count` (itens que saíram).
- O `utsim check` resume a tabela: `chegadas: pedidos: horário com 3 linha(s), 25 item(ns) a cada ciclo de 1440`.

Na interface Godot, a **Tabela de chegadas** das propriedades da fonte é uma grade: modo (horário ou sequência), repetição e ciclo, e uma linha por linha da tabela com instante (só no horário; em hh:mm quando o ciclo é de um dia ou uma semana), quantidade, tipo (os tipos do modelo) e rótulos escritos como `prioridade=2; prazo=1080`. Os botões ↑, ↓ e × movem e removem a linha, "+ Linha" acrescenta uma no fim e "Ordenar por instante" põe as linhas em ordem. "Colar da planilha" recebe linhas copiadas de uma planilha, separadas por tabulação ou ponto e vírgula, com vírgula decimal; a primeira linha pode ser um cabeçalho (`hora`, `qtd`, `tipo`, e qualquer outro nome vira um rótulo), e as linhas substituem a tabela ou são acrescentadas. As mensagens de erro aparecem na linha a que se referem, e embaixo ficam o total de itens por ciclo e por tipo. "Ver as primeiras chegadas" roda a fonte sozinha por um ciclo (ligada só a uma saída, com o lote, a liberação, os turnos e os sorteios) e mostra o histograma das chegadas por hora e por tipo; enquanto roda, ou se a fonte não rodar sozinha, mostra a estimativa lida da tabela e diz qual das duas é. Cada mudança é um passo de desfazer.

### Tipos de item (`item_types`)

O modelo pode declarar tipos de item no nível de cima, ao lado de `objects`. O tipo dá nome, cor e aparência no 3D aos itens e rótulos iniciais:

```ron
item_types: [
    (name: "A", color: "#3b82f6", asset: "caixa_pequena", labels: { "peso": 8.0 }),
    (name: "B", color: "#f59e0b", asset: "palete", labels: { "peso": 400.0 }),
],
```

| Campo | Padrão | Significado |
|---|---|---|
| `name` | obrigatório | nome único do tipo |
| `color` | nenhuma | cor dos itens no 3D, `"#rrggbb"` |
| `asset` | nenhum | aparência dos itens no 3D: um modelo da biblioteca ou um arquivo `.glb` |
| `labels` | `{}` | rótulos constantes que todo item do tipo recebe ao nascer |

A fonte escolhe o tipo com `item_type` (fixo), `item_type_mix` (sorteado com pesos) ou, numa tabela de chegadas, com `item_type` em cada linha (a linha vence a fonte).

- **Rótulo reservado `type`.** O item de tipo `k` (contado a partir de 1, na ordem de `item_types`) leva o rótulo numérico `type = k`. Assim `output: ByLabel("type")` manda o tipo 1 para a saída 1, o tipo 2 para a saída 2, e os scripts leem `type` como qualquer rótulo. Por isso `type` não pode aparecer nos `labels` de uma fonte que dá tipo aos itens, nem nos rótulos de um tipo ou de uma linha.
- **Ordem dos rótulos.** Primeiro os do tipo, depois os sorteados da fonte (e do período), por fim os da linha da tabela: o mais específico vence.
- **Relatório por tipo.** Num modelo com tipos, cada `Sink` registra também `time_in_system.<tipo>` (por exemplo `saida.time_in_system.A.mean`); a contagem dessa medida é a vazão do tipo, e as contagens somam o `time_in_system` total.
- **3D.** A cena (`utsim cena` e a cena ao vivo) lista os tipos no cabeçalho, com cor e aparência, e cada item no quadro diz o seu tipo; os visualizadores desenham o item com a cor e o modelo do tipo.
- O `utsim check` lista os tipos: `chegadas: tipos de item (rótulo 'type'): A = 1 (#3b82f6, caixa_pequena); B = 2 (#f59e0b, palete)`.
- Um nome de tipo que não existe é erro, com a lista dos tipos do modelo.

### Lote por chegada (`batch` e `release`)

Com `batch`, cada chegada cria vários itens de uma vez: `batch: Const(6.0)` cria seis, `batch: Poisson(mean: 4.0)` um número sorteado (arredondado; 0 não cria nada). Com `release: OneByOne(Const(2.0))` os itens do lote saem um a um, com 2 unidades de tempo entre eles; com `Together` (o padrão) saem juntos e o próximo objeto decide quantos cabem. O espaçamento vale também para os itens de uma linha da tabela.

- Com `on_blocked: Wait`, o intervalo até a próxima chegada começa quando o **último** item do lote saiu. Com `Drop`, cada item que não cabe é perdido.
- `max_arrivals` conta chegadas (lotes), não itens.
- A fonte com `batch` ganha as métricas `arrivals.count` (chegadas) e `entered.count` (itens que saíram); com `Drop` continuam `lost.count` e `loss_fraction`.
- `batch` não vale com `table`: na tabela a quantidade de cada linha vem de `count`.

O exemplo `examples/chegadas_por_tabela.ron` junta os três jeitos: um horário diário de caminhões com tipos por linha, uma sequência de ordens em laço e lotes avulsos com tipo sorteado, roteados por `ByLabel("type")`.

Nos experimentos, as chaves `pedidos.table`, `pedidos.table.rows[n]` e `pedidos.table.rows[n].<campo>` (`at`, `count`, `labels`, `labels.<nome>`) mudam a tabela; por exemplo `"pedidos.table.rows[1].count": "15"`.

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

Na interface 3D a Queue se chama **Espera**: há um só item na Biblioteca (os novos objetos se chamam `espera`, `espera_2`...), e o painel de propriedades tem o seletor **Política de acesso**:

| Política | No arquivo |
|---|---|
| FIFO (padrão) | `discipline: Fifo` |
| LIFO | `discipline: Lifo` |
| Prioridade: maior rótulo primeiro | `order: Some(Desc("prioridade"))` |
| Prioridade: menor rótulo primeiro (prazo) | `order: Some(Asc("prazo"))` |

Com prioridade aparece o campo **Rótulo da prioridade**, o nome do rótulo lido em cada item; empates e itens sem o rótulo saem na ordem de chegada. LIFO e `order` nunca vão juntos: escolher uma política limpa a outra. O arquivo `.ron` não muda de formato, e um modelo antigo com uma pilha abre como Espera LIFO (na cena o rótulo diz `· LIFO`).

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
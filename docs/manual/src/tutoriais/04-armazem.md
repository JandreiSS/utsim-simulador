# Tutorial 04: empilhadeiras e porta-paletes num armazém

**Objetivo:** dimensionar a frota de empilhadeiras e o porta-paletes de um armazém pequeno.

**Arquivos:** `examples/tutorial_04_armazem.ron` e `examples/experimento_tutorial_04.ron`

Tempo em **segundos**, distâncias em **metros**.

## O problema

Um palete chega ao recebimento a cada 90 s, em média. Uma empilhadeira o leva até um porta-paletes de 6 colunas por 3 níveis (**18 posições**). O palete fica guardado de 10 a 30 minutos (`Uniform(600, 1800)`, média de 1.200 s) e, depois, outra viagem de empilhadeira o leva até a expedição.

```text
recebimento (0, 0) ---- 40 m ---- porta_paletes (40, 0)
     |
expedicao (0, 30)                      garagem das empilhadeiras (20, 10)
```

O modelo:

```ron
(name: "chegada", kind: Source(interarrival: Exp(mean: 90.0)), position: (-6.0, 0.0, 0.0)),
(name: "recebimento", kind: Queue(), transport: "frota", position: (0.0, 0.0, 0.0)),
(name: "porta_paletes", kind: Rack(bays: 6, levels: 3,
    dwell: Some(Uniform(min: 600.0, max: 1800.0)), placement: FirstFree,
    bay_width: 1.2, level_height: 1.5),
  transport: "frota", position: (40.0, 0.0, 0.0)),
(name: "expedicao", kind: Sink, position: (0.0, 30.0, 0.0)),
(name: "empilhadeira_1", kind: Transporter(speed: 2.0, acceleration: Some(0.5), deceleration: Some(0.5),
    load_time: Uniform(min: 8.0, max: 12.0), unload_time: Uniform(min: 8.0, max: 12.0), home: true),
  position: (20.0, 10.0, 0.0)),
(name: "frota", kind: Dispatcher(team: ["empilhadeira_1", "empilhadeira_2"], rule: Nearest), ...),
```

- O campo `transport: "frota"` em `recebimento` diz que os paletes que saem dele são levados pela frota até o próximo objeto (o porta-paletes). O mesmo em `porta_paletes`, que leva até a expedição.
- As empilhadeiras andam a 2 m/s, aceleram e freiam a 0,5 m/s², e levam de 8 a 12 s para carregar e descarregar.
- Com `home: true`, voltam para a garagem quando ficam sem serviço.
- A frota padrão tem 2 empilhadeiras. A terceira existe no arquivo, mas só trabalha se estiver na equipe.

## A conta de cabeça

Cada palete pede **duas viagens**: do recebimento ao rack (40 m) e do rack à expedição (50 m, pois vai de (40, 0) a (0, 30)).

- Viagem carregada de 40 m: 40 / 2 + 1 x (1/0,5 + 1/0,5) = 20 + 4 = 24 s, mais 10 s de carga e 10 s de descarga.
- Mais a viagem a vazio até o palete.

Dá uns 60 a 70 s por viagem, ou **cerca de 140 s de empilhadeira por palete**, a cada 90 s. Uma empilhadeira sozinha não dá conta. Duas dão, com folga pequena.

O porta-paletes guarda em média 1.200 / 90 ≈ **13 paletes** (lei de Little), de 18 posições.

## Rodar

```bash
utsim check examples/tutorial_04_armazem.ron
utsim run examples/tutorial_04_armazem.ron
```

Trecho do resultado (2 empilhadeiras):

```text
objeto         tipo         entradas   saídas   conteúdo médio   permanência média
recebimento    queue          525.6     523.5       2.4120 ± 1.62    207.3 ± 130.0
porta_paletes  rack           523.6     521.7      14.1622 ± 0.53   1257.7 ±   7.8
expedicao      sink           521.7       0.0

empilhadeira_1 idle 18.72% | loading 11.70% | travel_empty 26.72% | travel_loaded 31.15% | unloading 11.72%
empilhadeira_2 idle 25.76% | loading 10.82% | travel_empty 24.05% | travel_loaded 28.54% | unloading 10.83%
empilhadeira_3 idle 100.00%
frota          blocked 35.49% | idle 64.51%
```

- O porta-paletes guarda em média **14,2 paletes** (previsto: 13).
- As empilhadeiras ficam ocupadas 81% e 74% do tempo (100% menos o `idle`). A de número 3 não trabalha.
- Uma empilhadeira gasta 27% do tempo indo buscar paletes **sem carga** (`travel_empty`) e 31% levando-os. Quase metade do trabalho é viagem.
- O palete fica no porta-paletes 1.258 s em média, um pouco mais que os 1.200 s do `dwell`, pois depois de pronto ainda espera a empilhadeira.

## Quantas empilhadeiras?

```bash
utsim experiment examples/experimento_tutorial_04.ron
```

```text
medida                       1 empilhadeira   2 empilhadeiras   3 empilhadeiras   2 empilhadeiras, 24 posições
empilhadeira_1.state.idle        0.00             0.19              0.31              0.17
empilhadeira_2.state.idle        1.00             0.26              0.43              0.26
recebimento.content.mean        83.8               2.4               1.2               0.8
recebimento.staytime.mean     7355               207               103                70
porta_paletes.content.mean      13.6              14.2              13.9              14.3
porta_paletes.content.max       17.9              18.0              18.0              23.9
expedicao.input                368               522               524               524
```

(Médias entre 10 replicações. O programa mostra também os intervalos de confiança.)

### Leitura

- **Uma empilhadeira** (`empilhadeira_1.idle` = 0): trabalha o tempo todo e não dá conta. A fila no recebimento tem 84 paletes em média, cada um espera 7.355 s (mais de duas horas) e só 368 paletes saem. O armazém está **instável**.
- **Duas empilhadeiras**: a fila cai para 2,4 paletes e a espera para 207 s. Saem 522 paletes, a demanda toda.
- **Três empilhadeiras**: a espera cai pela metade (103 s), mas a vazão é a mesma. Cada empilhadeira fica ociosa 31% a 43% do tempo, ou seja, a terceira compra pouco.
- Mesmo com três, ainda há espera. O porta-paletes chega ao máximo de **18 paletes** (`content.max`) em todos os cenários: ele **enche** nos picos, e então o recebimento acumula paletes que não têm onde ir. Mais empilhadeiras não resolvem isso.
- **Mais posições**: com 8 colunas (24 posições) e duas empilhadeiras, a espera cai de 207 s para 70 s. O porta-paletes continua chegando perto de 24 nos picos (máximo médio de 23,9), mas o recebimento só espera pela empilhadeira.

A resposta do dimensionamento: **duas empilhadeiras** resolvem a demanda; para reduzir ainda mais a espera, o porta-paletes é o gargalo, e uma terceira empilhadeira ajuda menos que mais posições.

## Faça no visualizador

Abra `utsim-view examples/tutorial_04_armazem.ron`. Em **Visual: realista**, as empilhadeiras sobem os garfos até o nível do porta-paletes e os paletes aparecem em cada posição. Selecione o `porta_paletes` para ver as estatísticas dele.

## Exercícios

1. **Rede de corredores.** Acrescente uma rede de caminhos (veja `examples/centro_de_distribuicao.ron`) com um corredor em forma de T. Como o caminho fica mais longo que a reta, o tempo de viagem sobe. Quanto?
2. **Despachante.** Troque `rule: Nearest` por `RoundRobin` ou `FirstAvailable`. A diferença é grande?
3. **Empilhadeira mais lenta.** Reduza `speed` para 1,2 (uma pessoa com paleteira). Com 2 empilhadeiras, o armazém continua estável?
4. **Dwell menor.** Com `dwell` de `Uniform(300, 900)`, o porta-paletes ainda enche?

## O que você aprendeu

- O campo `transport` liga o objeto a uma frota. O despachante reparte as viagens.
- O tempo de viagem vem da distância, da velocidade e da aceleração.
- Quando o porta-paletes enche, a fila acumula, e mais empilhadeiras não ajudam: é preciso mais posições.
- A lei de Little dá uma estimativa rápida de ocupação do porta-paletes (λ x permanência).

Próximo: [Tutorial 05, ajuste de distribuição](05-ajuste.md).
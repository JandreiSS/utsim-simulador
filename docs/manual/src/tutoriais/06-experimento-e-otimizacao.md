# Tutorial 06: experimento e otimização

**Objetivo:** escolher o número de atendentes de uma central de atendimento, primeiro comparando alternativas com `utsim experiment` e depois deixando o `utsim otimizar` procurar a melhor combinação.

**Arquivos:** `examples/tutorial_06_atendimento.ron`, `examples/experimento_tutorial_06.ron` e `examples/otimizacao_tutorial_06.ron`

Tempo em **minutos**.

## O problema

Uma central recebe um cliente por minuto, em média. O atendimento dura 3 minutos, em média (`Gamma(shape: 2.0, scale: 1.5)`). A carga é de **3 atendentes inteiros** (3 minutos de trabalho por minuto), então com menos de 4 atendentes a fila não para de crescer.

A sala de espera tem **10 lugares**. Quem chega e a encontra cheia vai embora (`on_blocked: Drop`).

```ron
(name: "chegada",    kind: Source(interarrival: Exp(mean: 1.0), on_blocked: Drop)),
(name: "fila",       kind: Queue(capacity: Some(10))),
(name: "atendentes", kind: Processor(capacity: 4, process: Gamma(shape: 2.0, scale: 1.5))),
(name: "saida",      kind: Sink),
```

O modelo base tem 4 atendentes. Na janela de estatísticas (19.000 minutos) chegam uns 19.000 clientes.

```bash
utsim check examples/tutorial_06_atendimento.ron
utsim run examples/tutorial_06_atendimento.ron
```

Com 4 atendentes: utilização de 3,0 / 4 = 75%, **1,05 minuto de espera** média, e 18.930 clientes atendidos (os outros cerca de 70 foram embora).

## Parte 1: experimento

O arquivo `experimento_tutorial_06.ron` troca só `atendentes.capacity`, de 3 a 8:

```ron
scenarios: [
    (name: "3", set: { "atendentes.capacity": "3" }),
    (name: "4", set: { "atendentes.capacity": "4" }),
    ...
],
measures: ["atendentes.content.mean", "fila.staytime.mean", "fila.content.mean", "saida.input"],
```

```bash
utsim experiment examples/experimento_tutorial_06.ron
```

```text
medida                       3        4        5        6        7        8
atendentes.content.mean   2.8156   2.9931   3.0021   3.0026   3.0026   3.0026
fila.staytime.mean        4.6243   1.0533   0.2801   0.0793   0.0229   0.0064
fila.content.mean         4.3342   1.0493   0.2799   0.0793   0.0229   0.0064
saida.input              17808    18930    18988    18992    18992    18991
```

(Médias entre 10 replicações. O programa mostra os intervalos também.)

### Leitura

- **3 atendentes**: a utilização por atendente é 2,82 / 3 = 94%, a espera é de 4,6 minutos e a sala de espera vive cheia. Só 17.808 clientes são atendidos: **6% se perdem**. É instável.
- **4 atendentes**: espera de 1,05 minuto, 18.930 atendidos. Estável, mas com perda de cerca de 0,4%.
- **5 atendentes**: espera de 0,28 minuto, 18.988 atendidos.
- **6 ou mais**: a espera continua caindo, mas a vazão não muda (18.992). O trabalho médio é o mesmo, **3,0 atendentes ocupados**; o que muda é só a folga.

Retornos decrescentes: de 4 para 5 atendentes a espera cai 0,77 minuto; de 5 para 6, 0,20; de 6 para 7, 0,06. Qual é o ponto certo depende do custo.

## Parte 2: otimização

Agora diga ao programa o que você quer e deixe-o procurar. O arquivo `otimizacao_tutorial_06.ron`:

```ron
OptProblem(
    name: "Tutorial 06: atendentes e sala de espera",
    model: "tutorial_06_atendimento.ron",
    replications: 5,
    variables: [
        (key: "atendentes.capacity", domain: Integer(min: 3, max: 8)),
        (key: "fila.capacity",       domain: Integer(min: 2, max: 14, step: 4)),
    ],
    objective: (
        sense: Minimize,
        terms: [],
        costs: [("atendentes.capacity", 18.0), ("fila.capacity", 1.0)],
    ),
    constraints: [
        (metric: "fila.staytime.mean", max: 1.0),
        (metric: "saida.input", min: 18950.0),
    ],
    algorithm: Grid,
    budget: 30,
    refine: (top: 3, replications: 15),
)
```

Em palavras:

- **Variáveis:** 3 a 8 atendentes e 2, 6, 10 ou 14 lugares na sala de espera. São 24 combinações.
- **Objetivo:** minimizar o custo por hora, 18 por atendente e 1 por lugar de espera.
- **Restrições:** a espera média não passa de 1 minuto, e pelo menos 18.950 dos cerca de 19.000 clientes são atendidos (no máximo 0,3% de perda).
- **Algoritmo:** `Grid`, que testa as 24 combinações. O `budget` de 30 cobre todas.
- **Refinamento:** os 3 melhores são simulados de novo com 15 replicações.

```bash
utsim otimizar examples/otimizacao_tutorial_06.ron --html relatorio.html
```

Resultado esperado (trecho):

```text
Melhor ponto
  atendentes.capacity = 5
  fila.capacity = 10
  objetivo: 100.0000

Restrições do melhor ponto
  ✓ fila.staytime.mean ≤ 1 (média observada: 0.2745 ± 0.0114)
  ✓ saida.input ≥ 18950 (média observada: 18980.3333 ± 66.6226)

Cenário para colar em um arquivo de experimento
  (name: "otimo", set: { "atendentes.capacity": "5", "fila.capacity": "10" }),
```

O melhor ponto é **5 atendentes e 10 lugares de espera**, a 18 x 5 + 1 x 10 = **100 por hora**.

### Como o programa chegou lá

- Com **3 ou 4 atendentes** nenhuma combinação cumpre as restrições (a espera passa de 1 minuto, ou a perda passa de 0,3%): todos os pontos com ✗.
- Com **5 atendentes**, a sala de **2 lugares** perde clientes demais (✗). Com **6** lugares, ela passou com 5 replicações (custo 96), mas **foi reprovada no refinamento**, com 15 replicações. É o que o refinamento serve para mostrar: um ponto que parecia bom por sorte. A sala de **10 lugares** (custo 100) passa em ambos.
- **14 lugares** (custo 104) também passa, mas custa 4 a mais e não melhora a vazão de forma útil: o relatório mostra a diferença para o melhor, +4,0, sem incerteza, pois o custo é uma conta e não uma medição.
- Com **6 ou mais atendentes** quase todos são viáveis (a sala de 2 lugares falha até com 7), e todos custam mais (a partir de 114).

Em português claro: **5 atendentes com sala de 10 lugares é o menor custo que cumpre o nível de serviço**.

## Exercícios

1. Mude a restrição de espera para `max: 0.2`. O ótimo muda? (Dica: a espera com 5 atendentes é de 0,28 minuto.)
2. Acrescente o custo de espera ao objetivo, em vez de usar restrição: `terms: [("fila.staytime.mean", 30.0)]` (R$ 0,50 por cliente-minuto, com 60 clientes por hora). Tire a restrição de espera e compare o ótimo.
3. Troque o algoritmo: `algorithm: Annealing(iterations: 15)`, `budget: 15`. Ele acha o mesmo ponto com menos simulações?
4. Faça um **experimento** com o ótimo: copie o cenário que o relatório imprimiu para um arquivo de experimento e compare com o cenário do modelo base.

## O que você aprendeu

- O `utsim experiment` compara alternativas que **você escolhe**, lado a lado.
- O `utsim otimizar` procura entre muitas combinações, com **custos**, **restrições** e comparação estatística.
- As mesmas sementes em todos os pontos tornam a comparação justa.
- O refinamento com mais replicações elimina pontos que só pareciam bons por sorte.
- Na prática, a melhor solução é a mais barata entre as que cumprem o nível de serviço.
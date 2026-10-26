# Experimentos e otimização

Depois de ter um modelo, você quer responder "e se?": e se eu comprar uma máquina, e se a demanda crescer 20%, e se eu afastar dois postos. Há duas ferramentas:

- **experimentos** (`utsim experiment`): você lista cenários e o programa os compara lado a lado;
- **otimização** (`utsim otimizar`): você descreve o que pode variar e o que quer minimizar, e o programa procura a melhor combinação.

Ambos usam as **mesmas sementes** para todos os cenários. Assim a diferença entre dois cenários vem da decisão, e não da sorte do sorteio. Na teoria da simulação isso se chama *números aleatórios comuns*, e reduz muito o ruído da comparação.

## Experimentos

O arquivo de experimento (`.ron`) parte de um modelo e lista cenários, cada um com alterações:

```ron
Experiment(
    name: "Capacidade da máquina",
    model: "mm1.ron",            // caminho relativo a este arquivo
    replications: 10,            // opcional: sobrescreve run.replications
    scenarios: [
        (name: "base"),
        (name: "máquina rápida", set: { "maquina.process": "Exp(mean: 0.7)" }),
        (name: "duas máquinas",  set: { "maquina.capacity": "2" }),
    ],
    measures: ["fila.content.mean", "maquina.state.processing", "saida.time_in_system.mean"],
)
```

```bash
utsim experiment examples/experimento_mm1.ron
```

| Campo | Padrão | Significado |
|---|---|---|
| `name` | obrigatório | nome do experimento |
| `model` | obrigatório | modelo base, em caminho relativo ao arquivo do experimento |
| `replications` | as do modelo | replicações por cenário |
| `scenarios` | obrigatório | lista de cenários (`name` e `set`) |
| `measures` | todas | métricas a mostrar, pelo nome (veja [Resultados](resultados.md)). Vazio mostra todas |

### O campo `set`

`set` é um mapa de **chave** para **valor**. O valor é um texto com o código RON do novo valor, por isso vai entre aspas.

| Chave | O que altera |
|---|---|
| `"objeto.campo"` | um campo do objeto, por exemplo `"maquina.capacity"`, `"fila.capacity"`, `"chegada.interarrival"` |
| `"run.campo"` | `stop_time`, `warmup`, `seed`, `replications` ou `default_decision` |
| `"flow.nome"` | troca (ou acrescenta) o fluxo inteiro; o valor `None` remove |

Os campos que se pode alterar em cada objeto são os do [formato `.ron`](referencia-ron.md): os de cada tipo, mais `output`, `position`, `downtime`, `shifts`, `transport` e `on_entry`. Exemplos de valores:

```ron
"maquina.capacity": "2",
"maquina.process": "Gamma(shape: 2.0, scale: 1.5)",
"fila.capacity": "Some(10)",
"maquina.position": "(15.0, -15.0, 0.0)",
"torno.downtime": "None",
"frota.team": "[\"empilhadeira_1\", \"empilhadeira_2\"]",
```

Observe que, dentro de um valor de texto, as aspas do RON precisam de barra invertida (`\"`). Se o valor for inválido ou a chave não existir, o programa diz qual cenário e qual campo, e lista os campos válidos.

### O resultado

```text
Capacidade da máquina: média ± meia-amplitude do IC de 95% entre replicações

medida                                base   maquina rapida    duas maquinas
fila.content.mean          3.2147 ± 0.1019  1.6344 ± 0.0365  0.1519 ± 0.0042
...
```

Cada coluna é um cenário e cada linha, uma medida. Com `--json arquivo.json` o resultado completo vai para um arquivo.

Use os experimentos para **comparar poucas alternativas** que você mesmo escolheu. Para explorar muitas combinações, use a otimização.

### Dicas

- Dê nomes curtos aos cenários; eles viram títulos de colunas.
- Mude uma coisa por vez, para saber o que causou a diferença. Mude várias de uma vez só quando quiser testar um pacote, como "demanda alta com duas furadeiras".
- Cuidado ao comparar um cenário estável com um instável (veja [Resultados](resultados.md)). Os números do cenário instável dependem do tempo simulado.

## Otimização

O problema de otimização também é um arquivo `.ron`:

```ron
OptProblem(
    name: "Dimensionamento de servidores",
    model: "servidores.ron",
    replications: 5,
    variables: [
        (key: "maquina.capacity", domain: Integer(min: 2, max: 6)),
        (key: "fila.capacity",    domain: Integer(min: 2, max: 10, step: 4)),
    ],
    objective: (
        sense: Minimize,
        terms: [("saida.time_in_system.mean", 40.0)],
        costs: [("maquina.capacity", 10.0), ("fila.capacity", 0.5)],
    ),
    constraints: [
        (metric: "maquina.state.processing", max: 0.95),
    ],
    algorithm: Grid,
    budget: 50,
    refine: (top: 4, replications: 15),
)
```

```bash
utsim otimizar examples/otimizacao_servidores.ron --html relatorio.html
```

### Campos

| Campo | Padrão | Significado |
|---|---|---|
| `name` | obrigatório | nome do problema |
| `model` | obrigatório | modelo base, em caminho relativo |
| `replications` | `5` | replicações por ponto avaliado |
| `variables` | obrigatório | o que pode variar: `key` (a mesma chave do `set` dos experimentos) e `domain` |
| `objective` | obrigatório | o que minimizar ou maximizar |
| `constraints` | nenhuma | limites sobre métricas |
| `algorithm` | `Grid` | como procurar |
| `budget` | `200` | máximo de pontos distintos avaliados |
| `seed` | `1` | semente da própria busca (não das simulações) |
| `refine` | nenhum | refinamento final (abaixo) |

### Domínios das variáveis

| Domínio | Significado |
|---|---|
| `Integer(min, max, step)` | inteiros de `min` a `max`, de `step` em `step` (`step` padrão: 1) |
| `Continuous(min, max, template, steps)` | números reais entre `min` e `max`. `template` é o texto RON com `{x}` no lugar do valor (padrão `"{x}"`). Por exemplo, `template: "Exp(mean: {x})"`. A busca em grade usa `steps` valores igualmente espaçados (padrão 5) |
| `Choice([...])` | uma entre as opções de texto, por exemplo `Choice(["Fifo", "Lifo"])` |

### Objetivo

```text
objetivo = soma de (peso x métrica) + soma de (custo x valor da variável)
```

- `terms`: pares `(métrica, peso)`. A métrica é a **média entre replicações**.
- `costs`: pares `(variável, custo por unidade)`, só para variáveis numéricas. É o custo de cada máquina, por exemplo.
- `sense`: `Minimize` (padrão) ou `Maximize`.

Para maximizar uma vazão, use `Maximize` com a métrica de entradas na Sink, ou `Minimize` com peso negativo.

### Restrições

`(metric: "...", min: ..., max: ...)`, ambos opcionais. Valem para a **média** da métrica entre replicações. Um ponto que viola uma restrição é inviável e fica pior que qualquer ponto viável.

### Algoritmos

| Algoritmo | Quando usar |
|---|---|
| `Grid` | testa todas as combinações. Ideal para poucas variáveis e poucos valores |
| `Annealing(iterations: n)` | recozimento simulado: começa no meio de cada domínio e anda por vizinhos. Bom para muitos valores |
| `Genetic(population: p, generations: g)` | algoritmo genético: seleção por torneio, cruzamento e mutação. Bom para muitas variáveis |

Todos guardam os pontos já simulados, então nenhum ponto é avaliado duas vezes, e o `budget` conta pontos distintos.

### Refinamento

Com `refine: (top: 4, replications: 15)`, depois da busca os 4 melhores pontos viáveis são simulados de novo com 15 replicações (e as mesmas sementes). A comparação final usa essas simulações mais precisas. Pontos que pareciam bons com poucas replicações podem cair, e é para isso que o refinamento existe.

### Ler o relatório

```text
Melhor ponto
  fila.capacity = 2
  maquina.capacity = 4
  objetivo: 151.0812 ± 1.0630

Restrições do melhor ponto
  ✓ maquina.state.processing ≤ 0.95 (média observada: 0.9207 ± 0.0025)

Cenário para colar em um arquivo de experimento
  (name: "otimo", set: { "fila.capacity": "2", "maquina.capacity": "4" }),
```

- O **melhor ponto**, o valor do objetivo e a verificação das restrições.
- O **cenário pronto** para colar num arquivo de experimento.
- As tabelas de pontos, com a diferença de cada um para o melhor (Δ ± intervalo de 95%). O sinal `≈` marca os pontos que **não se distinguem estatisticamente** do melhor. Se vários pontos são equivalentes, escolha o mais barato ou o mais simples, ou aumente as replicações para separá-los.
- ✓ marca ponto viável, ✗ marca ponto inviável.

Com `--html`, o relatório inclui gráficos do objetivo por ordem de avaliação e dos 10 melhores pontos. Com `--json`, o resultado completo vai para um arquivo. Com `--reps N` e `--orcamento N` você troca as replicações e o orçamento sem editar o arquivo.

### Limitações

- As métricas `<bloco>.lost.*` ainda não servem como objetivo (só `.mean` é exposto e vale sempre 1). Para contar clientes perdidos, use a vazão (por exemplo, `saida.input`) como restrição ou como termo do objetivo.
- A otimização só mexe em valores numéricos ou de lista que o `set` consegue escrever. Não cria objetos novos.

O [tutorial 06](tutoriais/06-experimento-e-otimizacao.md) usa as duas ferramentas no mesmo modelo.
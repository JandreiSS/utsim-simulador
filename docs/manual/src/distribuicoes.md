# Distribuições

Toda duração do modelo (tempo entre chegadas, de setup, de processamento, de reparo) e todo rótulo sorteado é uma **distribuição de probabilidade**. Você escreve no `.ron` o nome da distribuição e seus parâmetros:

```ron
Exp(mean: 4.0)
Uniform(min: 2.0, max: 5.0)
Const(3.0)
Discrete([(1.0, 0.7), (2.0, 0.3)])
```

Cada objeto tem o seu próprio gerador de números aleatórios, derivado da semente, da replicação e da posição do objeto. Por isso mexer na aleatoriedade de um objeto não muda os sorteios dos outros, e dois cenários com a mesma semente são comparáveis.

## Como escolher

Pergunte primeiro: **eu tenho dados?**

- **Sim.** Use o [`utsim fit`](ajuste-de-distribuicoes.md). Ele testa 20 famílias e entrega a linha pronta.
- **Não, só a opinião de quem conhece o processo.** Peça três números: o menor tempo, o mais comum e o maior. Use `Triangular`. Se só souber o menor e o maior, use `Uniform`.
- **O tempo é fixo** (ciclo automático, tempo de cura). Use `Const`.

Duas dicas que evitam erros comuns:

1. **Não use só a média.** Dois processos com a mesma média e variabilidade diferente dão filas diferentes. O [tutorial 05](tutoriais/05-ajuste.md) mostra isso com números.
2. **Para o tempo entre chegadas de clientes independentes, `Exp` é o ponto de partida.** Para tempos de atendimento, `Exp` costuma exagerar a variabilidade; `Gamma`, `Erlang` e `LogNormal` costumam servir melhor.

## Distribuições contínuas

| Nome | Parâmetros | Média | Quando usar |
|---|---|---|---|
| `Const(v)` | valor `v` | `v` | tempo fixo |
| `Uniform(min, max)` | `min <= max` | (min + max) / 2 | só se conhece o intervalo |
| `Triangular(min, mode, max)` | `min <= mode <= max` | (min + mode + max) / 3 | estimativa de especialista: menor, mais comum, maior |
| `Exp(mean)` | `mean > 0` | `mean` | tempo entre eventos independentes; o coeficiente de variação é 1 |
| `Normal(mean, sd)` | `sd >= 0` | `mean` | tempo em torno de um valor, simétrico. Cuidado: quando usada como duração, valores negativos viram zero. Prefira quando `sd` é bem menor que `mean` |
| `LogNormal(mean, sd)` | `mean > 0`, `sd >= 0` | `mean` | tempo positivo, assimétrico à direita (cauda longa). `mean` e `sd` são os da própria variável |
| `Gamma(shape, scale)` | ambos `> 0` | shape x scale | tempos de serviço com variabilidade moderada; `shape` grande dá menos variação |
| `Erlang(k, mean)` | `k` inteiro `> 0`, `mean > 0` | `mean` | soma de `k` etapas exponenciais; coeficiente de variação `1/raiz(k)`. É a Gama com `shape` inteiro |
| `Weibull(shape, scale)` | ambos `> 0` | scale x Γ(1 + 1/shape) | vida útil e falhas: `shape < 1` falha cedo, `shape > 1` desgaste |
| `Beta(alpha, beta, min, max)` | `alpha, beta > 0`, `min <= max` | min + (max - min) x alpha / (alpha + beta) | valor limitado entre dois extremos, com forma flexível |
| `LogLogistic(shape, scale)` | ambos `> 0` | existe se `shape > 1` | cauda longa, parecida com a LogNormal |
| `PearsonV(shape, scale)` | ambos `> 0` | existe se `shape > 1` | cauda muito longa |
| `PearsonVI(shape1, shape2, scale)` | todos `> 0` | existe se `shape2 > 1` | razão de duas Gamas; cauda longa |
| `Shifted(offset, dist)` | `offset` finito, `dist` válida | offset + média de `dist` | tempo mínimo fixo mais uma parte variável |

`Shifted` combina com qualquer outra: `Shifted(offset: 2.0, dist: Exp(mean: 3.0))` é um tempo de no mínimo 2, com média 5.

## Distribuições discretas (contagens)

Retornam números inteiros. São úteis para rótulos (quantidade de itens, tipo) e para dados contados.

| Nome | Parâmetros | Média | Quando usar |
|---|---|---|---|
| `Discrete([(valor, peso), ...])` | pares com peso `>= 0` e algum peso `> 0`. Os pesos não precisam somar 1 | média ponderada | tabela de valores com probabilidades. Exemplo: `Discrete([(1.0, 0.6), (2.0, 0.4)])` para 60% do tipo 1 e 40% do tipo 2 |
| `Poisson(mean)` | `mean > 0` | `mean` | número de eventos num intervalo |
| `Geometric(p)` | `0 < p <= 1` | (1 - p) / p | falhas antes do primeiro sucesso |
| `Binomial(n, p)` | `n > 0`, `0 <= p <= 1` | n x p | sucessos em `n` tentativas |
| `NegBinomial(r, p)` | `r > 0`, `0 < p <= 1` | r x (1 - p) / p | falhas antes do `r`-ésimo sucesso |

## Distribuição empírica

`Empirical([x1, x2, ...])` usa as próprias observações, em ordem crescente, ligadas por uma função de distribuição linear por partes. Cada intervalo entre vizinhos é igualmente provável. É a saída do `utsim fit` quando nenhuma família teórica passa nos testes. Exige pelo menos uma observação finita e a lista em ordem crescente.

## Erros de validação

O `utsim check` confere os parâmetros e explica o que está errado. Exemplos de mensagens: `Exp exige mean > 0`, `Triangular exige min <= mode <= max`, `Discrete exige ao menos um par (valor, peso) com peso > 0`.
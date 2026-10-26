# Tutorial 05: ajuste de distribuição a partir de dados reais

**Objetivo:** transformar duas colunas de dados medidos (tempos de atendimento e horários de chegada) em distribuições para o modelo, entendendo **por que** uma distribuição vence.

**Arquivos:** `examples/dados/tutorial_05_atendimento.csv`, `examples/dados/tutorial_05_chegadas.csv`, `examples/tutorial_05_ajuste.ron` e `examples/experimento_tutorial_05.ron`

Tempo em **minutos**.

## Os dados

Imagine que você cronometrou um balcão de atendimento.

`tutorial_05_atendimento.csv` tem **80 tempos de atendimento** (um por cliente), com `;` como separador e vírgula decimal, como numa planilha brasileira:

```text
cliente;tempo_min
C1;1,96
C2;4,9
C3;1,82
...
```

`tutorial_05_chegadas.csv` tem os **horários de 100 chegadas**, em minutos desde a abertura:

```text
chegada;instante_min
1;484,82
2;486,0
...
```

## Passo 1: tempos de atendimento

```bash
utsim fit examples/dados/tutorial_05_atendimento.csv
```

O relatório começa pelos dados e pelo diagnóstico:

```text
n: 80    média: 5,115    mediana: 4,635    desvio padrão: 2,809    CV: 0,549
nenhum aviso: não há sinal de violação das hipóteses i.i.d.
```

- A média é 5,1 minutos e o CV é 0,55: bem abaixo de 1, então **não é exponencial**. A distribuição é concentrada em torno da média e com cauda à direita.
- Os testes de sequências, tendência e autocorrelação não acusam nada (p entre 0,37 e 0,76). Os dados parecem independentes, o que permite ajustar.

Depois vem a tabela de candidatas (as primeiras linhas):

```text
pos.  família            parâmetros                        KS p   AD p   χ² p   AIC    ΔAIC    situação
1     Gama               forma = 3,506, escala = 1,459     0,654  0,771  0,272  375,5  0,000   escolhida
2     Erlang             k = 4,000, média = 5,115          0,505  0,475  0,103  376,3  0,791   aceita
3     Weibull deslocada  desloc. = 0,772, forma = 1,612... 0,482  0,512  0,256  376,7  1,188   aceita
4     Gama deslocada     ...                               0,561  0,611  0,176  376,7  1,225   aceita
5     Weibull            forma = 1,950, escala = 5,791     0,113  0,183  0,316  379,7  4,267   aceita
...
7     Lognormal          mu = 1,483, sigma = 0,565         0,189  0,276  0,020  376,9  1,461   rejeitada
...
15    Exponencial        média = 5,115                     0,003  0,003  0,003  423,1  47,661  rejeitada
```

### Por que a Gama venceu

1. **Não foi rejeitada**: os três testes deram p bem acima de 0,05.
2. **Tem o menor AIC** (375,5) entre as aceitas.
3. As vizinhas (Erlang, Weibull deslocada, Gama deslocada) estão a menos de 2 de AIC, ou seja, são **equivalentes**. O relatório avisa: "os dados não permitem distinguir entre elas". Usar a Gama, ou a Erlang com `k = 4`, daria o mesmo resultado prático.

### Por que a Lognormal ficou de fora

Ela tem AIC próximo (ΔAIC = 1,46), mas foi **rejeitada pelo qui-quadrado** (p = 0,020, menor que 0,05). O formato dela difere do dos dados nas faixas onde há mais observações. É o caso em que "perto no AIC" não basta.

### Por que a Exponencial é péssima

A exponencial seria a escolha "automática" para tempos de serviço, mas tem AIC 47 pontos acima e p de 0,003 nos três testes. Os dados têm muito menos variabilidade do que ela supõe.

No fim, o relatório dá a linha pronta:

```text
PARA O MODELO (.ron)
  Gamma(shape:3.5062574525060994,scale:1.4587491276044857)
```

## Passo 2: horários de chegada

Aqui os dados são **instantes**, não durações. Use `--timestamps` para ajustar os **intervalos** entre chegadas, e `--coluna` para dizer qual coluna é o instante:

```bash
utsim fit examples/dados/tutorial_05_chegadas.csv --timestamps --coluna instante_min
```

> **Cuidado.** Sem `--coluna`, o programa usa a primeira coluna numérica, que aqui é a numeração (1, 2, 3...). Os intervalos todos dariam 1 e o ajuste seria absurdo (ele acusa "99% dos valores são repetidos"). Confira sempre o nome da coluna que aparece no título do relatório.

Resultado (99 intervalos, média 2,896 min, CV 1,06):

```text
pos.  família      parâmetros                    KS p   AD p   χ² p   AIC    ΔAIC   situação
1     Exponencial  média = 2,896                 0,153  0,086  0,754  410,6  1,229  escolhida
2     Gama         forma = 0,807, escala = 3,588 0,880  0,894  0,983  409,3  0,000  aceita
3     Weibull      forma = 0,876, escala = 2,716 0,877  0,841  0,960  409,6  0,277  aceita
```

Repare: a **Gama tem o menor AIC** (409,3), e mesmo assim a **Exponencial foi a escolhida**. A regra de parcimônia explica: a diferença é de 1,2 (menor que 2), as duas descrevem os dados igualmente bem, e a Exponencial tem **um parâmetro contra dois**. Modelos mais simples generalizam melhor. Além disso, o CV de 1,06 já sugeria exponencial, que é o esperado para chegadas independentes.

```text
PARA O MODELO (.ron)
  Exp(mean:2.896262626262626)
```

## Passo 3: montar o modelo

O arquivo `tutorial_05_ajuste.ron` usa as duas linhas, arredondadas, com **três atendentes** em paralelo:

```ron
(name: "chegada",    kind: Source(interarrival: Exp(mean: 2.896))),
(name: "fila",       kind: Queue()),
(name: "atendentes", kind: Processor(capacity: 3, process: Gamma(shape: 3.506, scale: 1.459))),
(name: "saida",      kind: Sink),
```

Por que três? A carga é 5,115 / 2,896 = 1,77 atendentes. Com um só, a fila explodiria; com dois, a utilização seria de 88%; com três, de 59%.

```bash
utsim check examples/tutorial_05_ajuste.ron
utsim run examples/tutorial_05_ajuste.ron
```

Resultado esperado:

```text
objeto      tipo       conteúdo médio      permanência média
fila        queue        0.3214 ± 0.0178      0.9304 ± 0.0474
atendentes  processor    1.7640 ± 0.0140      5.1093 ± 0.0260
saida       time_in_system 6.0396 ± 0.0675
```

- Em média 1,76 atendentes ocupados de 3, ou **59% de utilização** por atendente.
- Cada cliente espera 0,93 minuto na fila, e fica 6,04 minutos no sistema.
- O tempo de atendimento medido no modelo, 5,11 minutos, bate com a média dos dados (5,115).

## Passo 4: a forma da distribuição importa?

Vale ajustar a distribuição, ou bastaria usar a média? Compare três versões **com a mesma média de atendimento** (5,115 min):

```bash
utsim experiment examples/experimento_tutorial_05.ron
```

```text
medida                      Gama ajustada   só a média (Const)   Exponencial
fila.content.mean                 0.3214            0.2586           0.4972
fila.staytime.mean                0.9304            0.7488           1.4396
saida.time_in_system.mean         6.0396            5.8636           6.5627
```

- Usar **só a média** subestima a espera em **20%** (0,75 contra 0,93 minuto), porque ignora a variabilidade.
- Usar a **Exponencial** superestima em **55%** (1,44 contra 0,93).
- A distribuição ajustada fica entre as duas, que é onde a realidade está.

Num sistema com folga, como este (59% de utilização), a diferença é de menos de um minuto. Em sistemas mais carregados, ela é muito maior.

## Exercícios

1. Rode `utsim fit` com `--html relatorio.html` e abra o arquivo. Veja o gráfico Q-Q da Gama e o da Exponencial: os pontos da Gama ficam sobre a reta, os da Exponencial se afastam.
2. Rode com `--alpha 0.2` (testes mais severos). Alguma família aceita passa a ser rejeitada?
3. Rode com `--bootstrap 1000 --seed 5`. Os p-valores mudam? (Um pouco; o ranking não deve mudar.)
4. Troque a linha do `process` para `Exp(mean: 5.115)` e rode `utsim run`. Compare a espera com a da Gama.
5. Pegue os seus próprios dados (um CSV com uma coluna de tempos) e rode `utsim fit`.

## O que você aprendeu

- `utsim fit` testa 20 famílias, mostra todas e explica a escolha.
- Entre duas distribuições equivalentes, ganha a de menos parâmetros.
- AIC baixo não basta: a família ainda precisa passar nos testes de aderência.
- Para horários de chegada, use `--timestamps` e confira a coluna.
- Trocar a distribuição pela média muda a espera, para mais ou para menos.

Próximo: [Tutorial 06, experimento e otimização](06-experimento-e-otimizacao.md).
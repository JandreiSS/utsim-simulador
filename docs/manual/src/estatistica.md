# Estatística: descritiva e testes de hipótese

O comando `utsim estatistica` resume uma coluna de dados e faz os testes de hipótese das
fases Medir e Analisar de um projeto Seis Sigma sobre as colunas de um arquivo CSV. As contas
moram na crate `utsim-stats` (módulos `descriptive`, `hypothesis` e `nonparametric`), a mesma
que o futuro aplicativo UTSim LSS vai usar. Cartas de controle, capacidade, nível sigma
e MSA ficam no capítulo [Qualidade](qualidade.md).

| Comando | O que faz |
|---|---|
| `utsim estatistica descritiva` | n, média, desvio, quartis, assimetria, curtose, moda, intervalos da média, do desvio e da mediana; dados do histograma, do boxplot e do gráfico de sequência |
| `utsim estatistica teste-t` | t de uma amostra, de duas amostras (Welch ou variância combinada) e pareado |
| `utsim estatistica anova` | ANOVA de um fator com as comparações de Tukey |
| `utsim estatistica qui-quadrado` | independência numa tabela de contagens ou aderência a proporções |
| `utsim estatistica proporcao` | uma proporção (exato ou normal) e duas proporções, com Fisher exato |
| `utsim estatistica mann-whitney` | duas amostras independentes, por postos |
| `utsim estatistica wilcoxon` | uma amostra ou amostras pareadas, por postos sinalizados |

O arquivo segue o formato do capítulo [Qualidade](qualidade.md#formato-dos-arquivos)
(separador `;`, tabulação ou vírgula; cabeçalho opcional; `#` comenta). `--coluna` e
`--coluna2` aceitam o nome do cabeçalho ou o número da coluna a partir de 1; sem `--coluna`,
vale a primeira coluna numérica. Células vazias são puladas, então as colunas podem ter
tamanhos diferentes; nos testes pareados só entram as linhas com as duas células preenchidas.
Todos os comandos aceitam `--confianca` (padrão 95%) e `--json arquivo.json`; os testes
aceitam `--alternativa diferente|menor|maior`. Os exemplos estão em `examples/estatistica/`
(dados sintéticos).

```bash
utsim estatistica descritiva examples/estatistica/setup.csv --coluna antes
utsim estatistica teste-t examples/estatistica/setup.csv --coluna antes --coluna2 depois --pareado
utsim estatistica teste-t examples/estatistica/setup.csv --coluna depois --mu 40 --alternativa menor
utsim estatistica anova examples/estatistica/setup.csv
utsim estatistica qui-quadrado examples/estatistica/turnos.csv
utsim estatistica proporcao --x 4 --n 200 --p0 0.05 --metodo normal --alternativa menor
utsim estatistica proporcao --x 27 --n 100 --x2 19 --n2 100 --fisher
utsim estatistica mann-whitney examples/estatistica/setup.csv --coluna antes --coluna2 depois
utsim estatistica wilcoxon examples/estatistica/setup.csv --coluna antes --coluna2 depois
```

## Como ler um teste

Cada teste mostra a estatística, os graus de liberdade quando há, e **três valores-p**:
bilateral (H₁: diferente), unilateral à esquerda (H₁: menor) e à direita (H₁: maior). O `p =`
da primeira linha é o da alternativa escolhida. O valor-p é a probabilidade, se H₀ fosse
verdadeira, de ver um resultado tão ou mais afastado de H₀ quanto o observado; abaixo do nível
de significância (em geral 5%) rejeita-se H₀. O intervalo de confiança fica do mesmo lado da
alternativa: bilateral para "diferente", com um limite infinito (ou 0 e 1 nas proporções) para
as unilaterais. O intervalo exclui o valor de H₀ exatamente quando o teste rejeita no nível
correspondente. Um valor-p grande não prova que H₀ é verdadeira; diz que os dados não bastam
para descartá-la (veja o tamanho de efeito e o intervalo).

## Descritiva

| Medida | Fórmula |
|---|---|
| média | x̄ = Σxᵢ/n (com um passo de correção pelo resíduo médio) |
| desvio padrão | s = √(Σ(xᵢ − x̄)²/(n − 1)), em duas passadas |
| erro padrão e CV | s/√n e s/\|x̄\| |
| quartis | por padrão, posição (n + 1)p com interpolação linear (`--quartis weibull`, tipo 6 de Hyndman e Fan); `--quartis linear` usa (n − 1)p + 1 (tipo 7). A mediana é sempre a usual |
| assimetria | G1 = √(n(n − 1))/(n − 2) · m3/m2^{3/2} |
| curtose (excesso) | G2 = (n − 1)/((n − 2)(n − 3)) · ((n + 1)(m4/m2² − 3) + 6), zero na normal |
| moda | os valores mais frequentes; "nenhuma" quando todos são distintos |
| IC da média | x̄ ± t(1 − α/2; n − 1) · s/√n |
| IC do desvio | s√((n − 1)/χ²(1 − α/2; n − 1)) a s√((n − 1)/χ²(α/2; n − 1)); vale para dado normal |
| IC da mediana | [x₍ₖ₎, x₍ₙ₋ₖ₊₁₎] pelas estatísticas de ordem, sem supor distribuição; o nível alcançado (binomial com p = 1/2) é mostrado e é pelo menos o pedido |

**Gráficos (dados).** O histograma usa a regra de Freedman-Diaconis (largura
2 · IQR · n^{−1/3}; volta para Sturges quando o IQR é zero, e o resultado diz qual regra
valeu), a de Sturges (⌈log₂ n⌉ + 1 classes) ou um número dado (`--classes`). O boxplot tem as
cercas em Q1 − 1,5 IQR e Q3 + 1,5 IQR, bigodes até o valor mais extremo dentro delas e os
atípicos fora. O gráfico de probabilidade normal põe cada valor ordenado contra o escore
Φ⁻¹((i − 0,3)/(n + 0,4)) (posição de Benard; Blom como opção na biblioteca), com a reta
x̄ + s·z e a correlação entre valores e escores (perto de 1 em dado normal).

**Gráfico de sequência.** A série no tempo com a mediana e dois testes de aleatoriedade:
corridas em torno da mediana (Wald e Wolfowitz; poucas corridas indicam **agrupamento**,
muitas indicam **mistura**) e corridas para cima e para baixo (poucas indicam **tendência**,
muitas indicam **oscilação**). Os valores-p vêm da aproximação normal com as médias e
variâncias usuais (`1 + 2n₁n₂/N` e `(2m − 1)/3`).

## Testes t

| Teste | Estatística | Graus de liberdade |
|---|---|---|
| uma amostra | t = (x̄ − μ₀)/(s/√n) | n − 1 |
| duas amostras, Welch (padrão) | t = (x̄₁ − x̄₂ − Δ₀)/√(s₁²/n₁ + s₂²/n₂) | ν de Satterthwaite: (v₁ + v₂)²/(v₁²/(n₁ − 1) + v₂²/(n₂ − 1)), vᵢ = sᵢ²/nᵢ |
| duas amostras, combinada (`--combinada`) | t = (x̄₁ − x̄₂ − Δ₀)/(s_p√(1/n₁ + 1/n₂)), s_p² = ((n₁ − 1)s₁² + (n₂ − 1)s₂²)/(n₁ + n₂ − 2) | n₁ + n₂ − 2 |
| pareado (`--pareado`) | o teste de uma amostra sobre as diferenças dᵢ = xᵢ − yᵢ | n − 1 |

`--mu` é μ₀ no teste de uma amostra e Δ₀ nos outros. Welch é o padrão porque não supõe
variâncias iguais e perde muito pouco quando elas são iguais. O pareado é o certo quando cada
linha é a mesma peça, máquina ou dia medido duas vezes (antes e depois): a variação entre as
unidades sai da conta. O **tamanho de efeito** é o d de Cohen: (estimativa − H₀)/s, com o
desvio da amostra, o das diferenças (pareado) ou o combinado (duas amostras). Como referência
grosseira, 0,2 é pequeno, 0,5 médio e 0,8 grande; com amostras grandes, um efeito pequeno
pode ser "significativo" sem importar na prática.

## ANOVA de um fator

Cada coluna numérica do arquivo é um grupo. A tabela tem as somas de quadrados entre e dentro
dos grupos, F = QM_entre/QM_dentro com (k − 1, N − k) graus de liberdade, o valor-p e
R² = SQ_entre/SQ_total. As comparações de Tukey (Tukey-Kramer com grupos de tamanhos
diferentes) dão, para cada par, a diferença das médias com o intervalo simultâneo
± q(nível; k, N − k)·√(QM_dentro/2 · (1/nᵢ + 1/nⱼ)) e o valor-p ajustado; um `*` marca os
pares diferentes. A ANOVA supõe variâncias parecidas entre os grupos e resíduos próximos da
normal.

## Qui-quadrado

**Independência.** O arquivo é a tabela de contagens (linhas e colunas numéricas; uma primeira
coluna de texto é rótulo). Esperado Eᵢⱼ = rᵢcⱼ/N; X² = Σ(O − E)²/E com (r − 1)(c − 1) graus de
liberdade; também a razão de verossimilhanças G² = 2ΣO ln(O/E). Os **resíduos ajustados**
(O − E)/√(E(1 − rᵢ/N)(1 − cⱼ/N)) são normais padrão sob independência: acima de 2 em módulo
apontam as células que mais contribuem. Quando mais de 20% das células têm esperado abaixo de
5, ou alguma abaixo de 1, o programa avisa que a aproximação é duvidosa (regra de Cochran):
junte categorias ou, em 2 × 2, use Fisher.

**Aderência** (`--aderencia`). As contagens da primeira coluna numérica contra as proporções de
`--proporcoes` (ex.: `9,3,3,1`; iguais quando omitidas), com k − 1 − `--estimados` graus de
liberdade.

## Proporções

| Caso | Teste | Intervalo |
|---|---|---|
| uma proporção, `--metodo exato` (padrão) | binomial exato; o bilateral soma as probabilidades que não passam da do valor observado | Clopper-Pearson |
| uma proporção, `--metodo normal` | z = (p̂ − p₀)/√(p₀(1 − p₀)/n) | Wald, p̂ ± z√(p̂(1 − p̂)/n) |
| duas proporções | z = (p̂₁ − p̂₂)/√(p̂(1 − p̂)(1/n₁ + 1/n₂)), p̂ combinada | Wald da diferença |
| 2 × 2 com contagens pequenas (`--fisher`) | Fisher exato, pela hipergeométrica com as margens fixas | — |

O intervalo de Wilson também é mostrado na proporção única: é melhor que o de Wald com n
pequeno ou p perto de 0 ou 1. A aproximação normal pede n·p e n·(1 − p) de pelo menos 5 a 10.

## Mann-Whitney e Wilcoxon

Testes por postos, para quando a normalidade não vale (dado assimétrico, atípicos, escalas
ordinais). Não comparam médias, e sim a **posição** das distribuições.

**Mann-Whitney** (soma de postos de Wilcoxon), duas amostras independentes: ordena-se tudo
junto (empates com o posto médio), W é a soma dos postos da primeira amostra e
U₁ = W − n₁(n₁ + 1)/2. Sem empates e com n₁ + n₂ ≤ 60, o valor-p é **exato** (distribuição
de U contada por completo); senão, aproximação normal com média n₁n₂/2, variância
n₁n₂/12 · (N + 1 − Σ(t³ − t)/(N(N − 1))) (correção de empates) e correção de continuidade de
0,5. O deslocamento estimado é o de Hodges-Lehmann, a mediana de todas as diferenças xᵢ − yⱼ.

**Wilcoxon dos postos sinalizados**, uma amostra contra `--mu` ou pareado (`--coluna2`): as
diferenças iguais a zero saem, os |dᵢ| são ordenados e W⁺ é a soma dos postos das diferenças
positivas. Exato sem empates e com n ≤ 60; senão normal com média n(n + 1)/4, variância
n(n + 1)(2n + 1)/24 − Σ(t³ − t)/48 e correção de continuidade. A estimativa é a pseudomediana
de Hodges-Lehmann (mediana das médias de Walsh). `--normal` força a aproximação nos dois.

## Validação

Os testes da crate conferem os resultados com valores de referência:

- **NIST StRD**, estatísticas univariadas (Lew, Lottery, Mavro, Michelso, NumAcc1 a NumAcc4):
  média e desvio com pelo menos 13 dígitos certos nos conjuntos de dificuldade baixa, 9 nos de
  dificuldade média e 8 no de dificuldade alta; autocorrelação de ordem 1 também.
- **NIST StRD**, ANOVA (SiRstv, SmLs01, SmLs04, AtmWtAg, SmLs07): somas e quadrados médios, F,
  R² e desvio residual com 9 ou mais dígitos nos de dificuldade baixa e média. No SmLs07 (13
  dígitos constantes, 1000000000000,4) o próprio dado não cabe num `f64`: a leitura já
  desloca o desvio de 0,1 em até 6·10⁻⁵, e cerca de 3 dígitos é o máximo possível.
- **Exemplos de livro**: Montgomery e Runger, *Applied Statistics and Probability for
  Engineers* (t de uma amostra, combinada, Welch, proporções, Mann-Whitney, Wilcoxon), Student
  (1908) para o pareado, Agresti, *An Introduction to Categorical Data Analysis*, para o
  qui-quadrado e Fisher, e a tabela de Mendel para a aderência.
- **Distribuições exatas** de Mann-Whitney e Wilcoxon contra a enumeração completa e contra
  os valores críticos tabelados; **erro tipo I** por simulação perto de 5% em cada teste.

## Nível sigma e o deslocamento de 1,5σ

O nível sigma continua no comando `utsim qualidade capacidade` e é sempre mostrado com o
deslocamento de 1,5σ explícito (convenção do Seis Sigma, decisão G2 do plano): nível sigma =
Z + 1,5, com `--deslocamento 0` para o valor sem deslocamento.

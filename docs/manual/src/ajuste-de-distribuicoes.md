# Ajuste de distribuições a partir de dados

Os tempos do modelo (atendimento, chegadas, reparo) devem vir de **dados reais** sempre que possível. O comando `utsim fit` pega uma coluna de números, testa vinte famílias de distribuições, mostra todas numa tabela, escolhe uma e **explica por quê**. No fim entrega a linha pronta para colar no `.ron`.

```bash
utsim fit examples/dados/tempos_atendimento.csv --html relatorio.html
```

O [tutorial 05](tutoriais/05-ajuste.md) percorre um caso completo.

## Que dados servem

- Arquivo **CSV** ou **TXT**, com separador `,`, `;` ou tabulação. O cabeçalho e a vírgula decimal são detectados sozinhos.
- Por padrão o programa usa a **primeira coluna numérica**. Para escolher outra, use `--coluna nome` ou `--coluna 3` (posição a partir de 1).
- Se a coluna traz **instantes crescentes** (a hora de cada chegada, por exemplo), use `--timestamps`: o ajuste é feito sobre os intervalos entre eles. Cuidado: se o arquivo tem uma coluna de numeração antes do instante, indique a coluna certa com `--coluna`, senão o programa usa a numeração (1, 2, 3...) e acha que todos os intervalos valem 1.

Quanto mais dados, melhor. Com menos de 30 observações os testes têm pouca força; com 100 ou mais, o ajuste fica confiável.

## Opções

| Opção | Significado |
|---|---|
| `--coluna <nome ou n>` | coluna a usar |
| `--timestamps` | a coluna são instantes; ajusta os intervalos |
| `--alpha 0.05` | nível de significância dos testes (padrão 0,05) |
| `--bootstrap 300` | réplicas do bootstrap paramétrico para os p-valores (padrão 300) |
| `--seed 1` | semente do bootstrap, para o resultado ser repetível |
| `--html relatorio.html` | relatório completo com gráficos |
| `--json saida.json` | relatório em JSON |

## O que o relatório mostra

### 1. Dados

Número de observações, média, mediana, desvio padrão, coeficiente de variação (CV), assimetria, curtose, mínimo, máximo e quartis. O CV é um bom primeiro palpite: perto de 1 sugere exponencial, bem abaixo de 1 sugere Gama ou Erlang, e acima de 1 sugere caudas longas.

### 2. Diagnóstico

Os testes estatísticos supõem que as observações são **independentes e identicamente distribuídas** (i.i.d.). O diagnóstico verifica isso:

- teste de sequências (Wald-Wolfowitz): há dependência na ordem?
- tendência (Mann-Kendall): os tempos aumentam ou diminuem ao longo da coleta?
- autocorrelação (Ljung-Box): um valor alto puxa o seguinte?
- valores atípicos (regra de 1,5 x IQR) e valores repetidos.

Se aparecer um aviso, pense antes de confiar no ajuste. Uma tendência, por exemplo, indica que o processo mudou durante a coleta.

### 3. Regra de escolha

A regra é a mesma em todo relatório:

1. famílias que não se aplicam aos dados ficam por último;
2. uma família é **rejeitada** se algum teste de aderência (KS, Anderson-Darling, qui-quadrado) der p menor que α;
3. entre as não rejeitadas, vence o **menor AIC** (em caso de empate, a menor estatística de Anderson-Darling);
4. se todas forem rejeitadas, recomenda-se a **distribuição empírica**;
5. as aceitas com ΔAIC menor que 2 em relação ao menor AIC são **equivalentes**; entre elas vence a de **menos parâmetros** (parcimônia).

### 4. Candidatas

Uma tabela com todas as famílias:

| Coluna | Significado |
|---|---|
| `pos.` | posição no ranking |
| `família` e `parâmetros` | a distribuição e os parâmetros estimados por máxima verossimilhança |
| `KS p`, `AD p`, `χ² p` | p-valores dos testes de aderência. Quanto menor, mais os dados "discordam" da família |
| `AIC` | critério de Akaike: mede o ajuste com um desconto pelo número de parâmetros. Menor é melhor |
| `ΔAIC` | diferença para o menor AIC. Abaixo de 2, as famílias são praticamente equivalentes |
| `peso Akaike` | probabilidade relativa de a família ser a melhor entre as candidatas |
| `situação` | `escolhida`, `aceita`, `rejeitada` ou `não se aplica` |

As famílias são: Exponencial, Gama, Erlang, Weibull, Lognormal, Normal, Uniforme, Triangular, Beta, Log-logística, Pearson V, Pearson VI, as versões deslocadas (que acrescentam um valor mínimo) da Exponencial, Gama, Weibull e Lognormal, e as discretas Binomial, Binomial negativa, Geométrica e Poisson. As discretas só se aplicam a números inteiros não negativos.

Os testes são:

- **KS (Kolmogorov-Smirnov)**: a maior distância entre a curva acumulada dos dados e a da família;
- **AD (Anderson-Darling)**: parecido, mas dá mais peso às caudas;
- **qui-quadrado**: compara a contagem de dados em faixas de igual probabilidade com a esperada.

Como os parâmetros foram estimados dos próprios dados, os p-valores de KS e AD vêm de um **bootstrap paramétrico**: o programa simula amostras da família ajustada, reajusta cada uma e conta com que frequência o teste dá pior. É por isso que os p-valores não passam de 1/(`bootstrap`+1) em precisão, e com 300 réplicas o menor p possível é cerca de 0,003.

### 5. Por que

Uma frase por candidata, em ordem. Para a escolhida, diz quais testes passou e por que venceu. Para as demais, diz se foi rejeitada (e por qual teste), ou se foi aceita mas perdeu (por AIC maior ou por ter mais parâmetros).

### 6. Para o modelo

A linha `Dist` pronta, por exemplo:

```ron
Gamma(shape:3.5062574525060994,scale:1.4587491276044857)
```

Copie e cole no `process`, `interarrival` ou onde o tempo for usado.

## Como ler o ranking

Duas ideias evitam as confusões mais comuns.

**Menor AIC não é sempre o vencedor.** Se a Gama tem AIC 409,3 e a Exponencial tem 410,6 (ΔAIC = 1,2), as duas descrevem os dados igualmente bem, e a regra escolhe a **Exponencial**, porque tem um parâmetro só contra dois. Modelos mais simples generalizam melhor e são mais fáceis de explicar. O relatório avisa: "praticamente equivalente: os dados não permitem distinguir entre elas".

**Uma família com AIC baixo pode ser rejeitada.** O AIC mede o ajuste médio, mas um teste pode reprovar uma família mesmo assim, por exemplo se a forma da cauda for diferente. No tutorial 05, a Lognormal fica a 1,5 de ΔAIC da melhor e mesmo assim é rejeitada pelo qui-quadrado (p = 0,020).

Quando **nenhuma** família passa, o relatório recomenda a **distribuição empírica** (`Empirical([...])`), que usa os próprios dados. Ela é segura, mas não extrapola: nunca gera um valor maior que o maior dado observado.

## Os gráficos (`--html`)

O relatório HTML traz, além do texto:

- o **histograma** dos dados com as densidades das candidatas;
- o gráfico **Q-Q** (quantis dos dados contra os da família): pontos sobre a reta indicam bom ajuste;
- o gráfico **P-P** (probabilidades acumuladas);
- a curva **ECDF** (acumulada empírica);
- a dispersão `x[i]` contra `x[i+1]`, que mostra dependência entre observações consecutivas.

Olhe os gráficos: os testes dizem "sim ou não", os gráficos dizem **onde** a distribuição falha.

## Cuidados

- Dados **arredondados** (inteiros, ou com poucas casas) enfraquecem os testes de KS e AD. O programa avisa.
- Não ajuste uma distribuição a dados que misturam situações diferentes (turno da manhã e da noite, dois tipos de peça). Separe e ajuste cada um.
- Dados de **tempo de atendimento** devem incluir só o tempo de atendimento, sem a espera na fila.
- O ajuste não substitui o bom senso. Se uma família física faz sentido para o processo (Erlang para etapas em série, Exponencial para chegadas independentes), prefira-a quando estiver entre as equivalentes.
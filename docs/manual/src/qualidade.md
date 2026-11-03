# Qualidade: cartas de controle, capacidade e MSA

O comando `utsim qualidade` faz o controle estatístico de processo (CEP), a análise de
capacidade e a análise do sistema de medição (MSA) sobre um arquivo CSV. As contas moram na
crate `utsim-quality`, a mesma que o painel do gêmeo digital e o futuro aplicativo Lean Seis
Sigma vão usar, então o número que sai aqui é o mesmo que vai aparecer lá.

| Comando | O que faz |
|---|---|
| `utsim qualidade carta` | cartas I-MR, X̄-R, X̄-S, p, np, c, u, EWMA e CUSUM, com regras de Western Electric e de Nelson |
| `utsim qualidade capacidade` | Cp, Cpk, Pp, Ppk, Cpm com intervalos, PPM, nível sigma; Box-Cox e método dos percentis |
| `utsim qualidade msa` | Gage R&R por ANOVA e por médias e amplitudes, atributos (kappa), viés e linearidade |
| `utsim qualidade arl` | comprimento médio de corrida de um esquema de carta, por simulação |

Todos aceitam `--html arquivo.html` (relatório com gráficos, sem dependências externas) e,
exceto `arl`, `--json arquivo.json`. Os arquivos de exemplo estão em `examples/qualidade/`
(dados sintéticos).

## Formato dos arquivos

O separador é `;` quando a primeira linha tem um (e então a vírgula é a marca decimal, como
nas planilhas em português), senão tabulação, senão vírgula. Uma primeira linha com texto é
cabeçalho. Linhas vazias e linhas começando com `#` são ignoradas. O arquivo pode estar em
UTF-8 ou em Latin-1 (o que muitas planilhas gravam no Windows); no segundo caso o programa
avisa.

| Análise | Layout |
|---|---|
| Medidas (cartas de variáveis, capacidade, EWMA, CUSUM) | uma linha por subgrupo, cada célula numérica é um valor; uma primeira coluna chamada `subgrupo`, `amostra`, `lote`, `data`, `hora` ou `id` é rótulo. Com `--subgrupo N`, uma coluna única é cortada em subgrupos consecutivos de `N` |
| Atributos (p, np, c, u) | `contagem,tamanho` por linha (nomes aceitos: `defeituosos`, `defeitos`; `tamanho`, `n`, `unidades`); sem tamanho, vale 1 (carta c) |
| Gage R&R | `peça,operador,valor`, uma linha por medição; as repetições são a ordem das linhas de cada peça e operador |
| Atributos (MSA) | `item,avaliador,resposta[,padrão]` |
| Viés e linearidade | `referência,valor` |

## Cartas de controle

```bash
utsim qualidade carta examples/qualidade/diametros.csv --tipo xbarr --regras we --html carta.html
utsim qualidade carta examples/qualidade/defeituosos.csv --tipo p
utsim qualidade carta tempos.csv --tipo imr --regras nelson
```

**Tipos.** `imr` (valores individuais e amplitude móvel de 2), `xbarr` (médias e amplitudes),
`xbars` (médias e desvios), `p` (proporção de defeituosos, tamanho de amostra variável),
`np` (número de defeituosos, tamanho fixo), `c` (número de defeitos por unidade de inspeção
fixa), `u` (defeitos por unidade, tamanho variável), `ewma` e `cusum`.

**Limites.**

| Carta | Linha central | Limites |
|---|---|---|
| I | x̄ | x̄ ± 3·MR̄/d2 (com d2 de n = 2) |
| MR | MR̄ | 0 e D4·MR̄ |
| X̄ (com R) | x̿ | x̿ ± A2·R̄ |
| R | R̄ | D3·R̄ e D4·R̄ |
| X̄ (com S) | x̿ | x̿ ± A3·S̄ |
| S | S̄ | B3·S̄ e B4·S̄ |
| p | p̄ = Σd/Σn | p̄ ± 3√(p̄(1 − p̄)/nᵢ), por subgrupo |
| np | np̄ | np̄ ± 3√(np̄(1 − p̄)) |
| c | c̄ | c̄ ± 3√c̄ |
| u | ū = Σc/Σn | ū ± 3√(ū/nᵢ), por subgrupo |

Limites inferiores negativos viram 0. Na carta p e na carta u, com tamanhos diferentes, os
limites mudam de subgrupo para subgrupo (aparecem em degraus no gráfico).

As constantes `d2`, `d3` e `c4` são **calculadas**, não copiadas de tabela: `d2` é a média da
amplitude de `n` normais padrão, obtida por integração numérica; `d3`, o desvio da amplitude,
por uma integral dupla; `c4` tem fórmula fechada com a função gama. `A2`, `A3`, `B3`, `B4`,
`D3` e `D4` saem delas. Os testes conferem os valores com a tabela publicada (de n = 2 a 25).

**Fase I e fase II.**

- Fase I (padrão): os limites são estimados dos próprios dados. Quando um ponto tem **causa
  atribuível** identificada (troca de lote, máquina regulada errada), tire-o da estimativa
  com `--excluir 7,12` (números a partir de 1): ele continua no gráfico, vazado, mas não
  entra nos limites nem nas regras. `--revisar` faz o ciclo clássico sozinho (tira os pontos
  fora dos limites e reestima até não sobrar nenhum) e lista os retirados: use só se cada um
  tiver causa conhecida.
- `--salvar-limites limites.json` grava os limites estimados.
- Fase II: `--limites limites.json` aplica limites congelados a dados novos. Com `--media` e
  `--sigma` (cartas de variáveis), os limites vêm de um padrão dado.

**Regras.** `--regras` escolhe quais padrões sinalizam:

| Código | Regra |
|---|---|
| `1` (N1, WE1) | 1 ponto além de 3σ |
| `2` (N2) | 9 pontos seguidos do mesmo lado da linha central |
| `we4` | 8 pontos seguidos do mesmo lado (versão de Western Electric) |
| `3` (N3) | 6 pontos seguidos subindo ou descendo |
| `4` (N4) | 14 pontos seguidos alternando para cima e para baixo |
| `5` (N5, WE2) | 2 de 3 pontos além de 2σ do mesmo lado |
| `6` (N6, WE3) | 4 de 5 pontos além de 1σ do mesmo lado |
| `7` (N7) | 15 pontos seguidos dentro de 1σ (estratificação) |
| `8` (N8) | 8 pontos seguidos fora de 1σ, dos dois lados (mistura) |

`--regras we` liga as quatro de Western Electric, `--regras nelson` as oito de Nelson, e uma
lista como `1,2,5` escolhe uma a uma. O padrão é só a regra 1. As regras de zona e de
sequência valem na carta de posição (I, X̄, p, np, c, u); na carta de dispersão (MR, R, S) só
vale "fora dos limites", porque as amplitudes móveis são correlacionadas e as amplitudes são
assimétricas. Cada regra a mais aumenta os alarmes falsos: com as quatro de Western Electric
o comprimento médio de corrida sob controle cai de cerca de 370 para cerca de 92 (veja `arl`
abaixo).

**EWMA e CUSUM.** Detectam deslocamentos pequenos e persistentes que a carta de Shewhart
demora a ver. O EWMA acumula `zᵢ = λxᵢ + (1 − λ)zᵢ₋₁` (`--lambda`, padrão 0,2) com limites
`μ0 ± Lσ√(λ/(2 − λ)·[1 − (1 − λ)²ⁱ])` (`--l`, padrão 3). O CUSUM tabular acumula os desvios
acima de `μ0 + kσ` e abaixo de `μ0 − kσ` e sinaliza quando passa de `hσ` (`--k` 0,5 e `--h`
5, o par usual para detectar um deslocamento de 1σ); no primeiro sinal, informa a nova média
estimada. Sem `--media` e `--sigma`, o alvo e o σ são estimados dos dados (fase I). Com
subgrupos, a carta usa as médias e σ/√n.

```bash
utsim qualidade carta examples/qualidade/diametros.csv --tipo cusum --media 50 --sigma 0.01
```

## Capacidade do processo

```bash
utsim qualidade capacidade examples/qualidade/diametros.csv --lie 49.97 --lse 50.03
utsim qualidade capacidade examples/qualidade/lead_time.csv --lse 18 --metodo todos --html cap.html
```

Basta um dos limites (`--lie` ou `--lse`); `--alvo` é usado no Cpm (padrão: o meio da
especificação).

**Dois desvios.** O **σ dentro** mede a variação de curto prazo: `R̄/d2` para subgrupos (padrão),
`S̄/c4` ou `MR̄/d2` para valores individuais (`--sigma-dentro rbar|sbar|mr`). O **σ global** é o
desvio amostral de todos os valores juntos e inclui as mudanças entre subgrupos.

| Índice | Fórmula | Desvio |
|---|---|---|
| Cp | (LSE − LIE)/6σ | dentro |
| Cpu, Cpl | (LSE − μ)/3σ, (μ − LIE)/3σ | dentro |
| Cpk | min(Cpu, Cpl) | dentro |
| Pp, Ppu, Ppl, Ppk | as mesmas fórmulas | global |
| Cpm | (LSE − LIE)/(6√(s² + (μ − alvo)²)) | global |

Os intervalos (nível em `--confianca`, padrão 95%) são o intervalo χ² para Cp e Pp e a
aproximação de Bissell para Cpk e Ppk, `Cpk ± z·√(1/(9n) + Cpk²/(2(n − 1)))`. Com menos de 30
valores o programa avisa que os intervalos são largos.

**Fora da especificação.** O relatório traz o PPM **observado** (contado nos dados) e o
**esperado** pela normal com σ dentro e com σ global, acima, abaixo e no total, e o
`Z bench = Φ⁻¹(1 − p_total)`.

**Nível sigma.** É o Z bench global **mais 1,5**, o deslocamento de longo prazo da convenção do
Seis Sigma. Ele aparece explícito no relatório e pode ser mudado ou desligado com
`--deslocamento 0`. Pela convenção, 3,4 DPMO correspondem a 6σ, 233 a 5σ, 6.210 a 4σ e 66.807 a
3σ. A crate também traz `dpmo(defeitos, unidades, oportunidades)` e o rendimento encadeado
(RTY).

**Normalidade.** Todo estudo roda o teste de Anderson-Darling com média e desvio estimados
(estatística modificada `A*² = A²(1 + 0,75/n + 2,25/n²)` e p-valor pelas fórmulas de
D'Agostino e Stephens). Quando ele rejeita a 5%, o relatório avisa e sugere:

- `--metodo boxcox`: transforma dados, limites e alvo por `y = (x^λ − 1)/λ` (`ln x` em λ = 0),
  com λ por máxima verossimilhança e seu intervalo; `--arredondar` troca λ pelo valor usual
  mais próximo (−2, −1, −0,5, 0, 0,5, 1, 2) quando ele está no intervalo. Os dados precisam ser
  positivos. O relatório mostra a normalidade antes e depois;
- `--metodo percentis`: ajusta distribuições com o `utsim fit` e usa a melhor:
  `Ppu = (LSE − P50)/(P99,865 − P50)`, `Ppl = (P50 − LIE)/(P50 − P0,135)`. Se nenhuma família
  for aceita e houver ao menos 100 valores, usa os percentis empíricos;
- `--metodo todos`: as três análises.

## Análise do sistema de medição (MSA)

### Gage R&R

```bash
utsim qualidade msa examples/qualidade/grr.csv --tolerancia 6 --html grr.html
```

O estudo cruzado precisa ser **balanceado**: todas as peças medidas por todos os operadores o
mesmo número de vezes (ao menos 2 peças, 2 operadores e 2 repetições). O programa faz os dois
métodos:

- **ANOVA** de dois fatores aleatórios com interação. Se a interação peça × operador tiver
  p > 0,25, ela é incorporada à repetibilidade (modelo reduzido) e o relatório avisa. Os
  componentes de variância saem dos quadrados médios; um componente negativo vira 0, com aviso;
- **médias e amplitudes**: repetibilidade por `R̿/d2*`, reprodutibilidade pela amplitude das
  médias dos operadores (descontada a parte da repetibilidade) e variação peça a peça pela
  amplitude das médias das peças. As constantes `d2*` são calculadas (`√(d2² + d3²/g)`) e
  aparecem no relatório.

Para cada componente (GRR total, repetibilidade, reprodutibilidade, operador, interação,
peça a peça e total): variância, **%contribuição** (sobre as variâncias), **%VE** (sobre os
desvios, a variação do estudo) e **%tolerância** (`k·σ/(LSE − LIE)`, com `--tolerancia` ou
`--lie` e `--lse`). `--k` muda o multiplicador da variação do estudo (6, padrão, ou 5,15). O
**número de categorias distintas** é `ndc = ⌊1,41·σ_peça/σ_GRR⌋`.

O critério impresso é a regra prática usual: %VE do GRR até 10% aceitável, de 10% a 30%
condicional, acima de 30% inaceitável, e ndc de 5 ou mais. Cada empresa pode ter o seu.

### Atributos

```bash
utsim qualidade msa examples/qualidade/atributos.csv --tipo atributos
```

Cada avaliador classifica cada item o mesmo número de vezes. O relatório traz, com intervalo
exato de Clopper-Pearson: a concordância **dentro** de cada avaliador (itens em que todas as
suas avaliações coincidem), de cada avaliador **contra o padrão** (quando a coluna existe),
**entre** avaliadores e de todos contra o padrão. Os kappas são o de **Cohen** (cada avaliador
contra o padrão) e o de **Fleiss** (avaliações repetidas de um avaliador e todas as avaliações
juntas, com o kappa de cada categoria e o teste de κ = 0).

### Viés e linearidade

```bash
utsim qualidade msa examples/qualidade/linearidade.csv --tipo linearidade --variacao-processo 6
```

Com uma só referência, o programa faz o estudo de **viés**: média das medições menos a
referência, teste t e intervalo. Com várias, faz também a **linearidade**: regressão do viés
contra a referência, com inclinação, intercepto, seus p-valores, R², `%linearidade =
100·|inclinação|` e, com `--variacao-processo`, a linearidade absoluta.

## Comprimento médio de corrida (ARL)

```bash
utsim qualidade arl --tipo we
utsim qualidade arl --tipo cusum --k 0.5 --h 4 --deslocamento 1
```

Simula o esquema (`shewhart`, `we`, `nelson`, `ewma`, `cusum`) sobre valores normais com a
média deslocada de `--deslocamento` σ e informa quantos pontos, em média, ele leva para dar o
primeiro sinal. Com deslocamento 0, é o tempo médio entre alarmes falsos (370 para a regra 1
a 3σ). Para a carta de Shewhart com a regra 1 o programa mostra também a fórmula fechada, e
para o CUSUM a aproximação de Siegmund. É com esse número que se escolhem `k` e `h` do CUSUM
e `λ` e `L` do EWMA para uma taxa de alarmes falsos aceitável.

## Uso pela biblioteca

A crate `utsim-quality` não lê arquivos: recebe números. Além do que o comando mostra, ela
oferece:

- `charts::limits_from_model(amostras)`: limites por quantis (0,135%, 50% e 99,865%) da
  distribuição do KPI entre as replicações do modelo, para ter limites antes de haver
  histórico real e para KPIs assimétricos (item E4 do plano do gêmeo digital);
- `memory::standardized_residuals(real, média_simulada, desvio_simulado)`: os resíduos
  real − simulado padronizados, entrada do EWMA e do CUSUM do monitor de desvio do gêmeo;
- `capability::percentile_capability(quantil, especificação)`: o método dos percentis com
  qualquer função quantil.

## Limitações

- As cartas de variáveis X̄-R e X̄-S exigem subgrupos do mesmo tamanho.
- Os limites clássicos supõem observações independentes. Com autocorrelação alta (comum em
  séries de simulação e de sensores), use médias em lotes (`utsim analisar --lotes`) ou CEP
  sobre resíduos.
- O Gage R&R exige estudo balanceado e cruzado; estudos aninhados (ensaios destrutivos) ainda
  não estão disponíveis.
- A transformação de Johnson ainda não está disponível; use Box-Cox ou os percentis.

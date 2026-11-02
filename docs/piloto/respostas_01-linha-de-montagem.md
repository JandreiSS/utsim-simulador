# Caso 1: Vale Verde Eletrônica, linha de montagem com inspeção e retrabalho

Consultor: engenheiro de produção (piloto de usabilidade do UTSim).
Modelo final: `trabalho/01-linha-de-montagem/modelo_caso01_n3.ron` (3 inspetores na equipe) e
`modelo_caso01.ron` (mesmo modelo com 4 inspetores na equipe). Ambos foram montados e salvos pelo
editor, por roteiros (`roteiros/r04` a `r13`).

Todas as rodadas: 11 dias simulados (950.400 s), aquecimento de 1 dia (86.400 s), janela de 10 dias,
20 replicações, semente 1, mesmas sementes em todos os cenários. Todo valor é média ± meia-amplitude
do IC de 95% entre replicações. "Por dia" é a contagem da janela dividida por 10.

## 1. O modelo e as distribuições

| Elemento | Objeto UTSim | Parâmetros | Por quê |
|---|---|---|---|
| Alimentador de bases | Chegada (Source), espera se bloqueada | Uniform(45, 65) s | enunciado: uniforme |
| Alimentador de placas | Chegada | Uniform(12, 24) s | enunciado: uniforme |
| Fila de bases / placas | Fila, capacidade 4 / 12 | FIFO | enunciado |
| Estação 1 | Montagem (Combiner), 1 contêiner + 3 componentes, modo Embalar | setup Uniform(3, 7); processo Triangular(38, 45, 60) | a base é a porta 0 (ligada primeiro); a placa é a porta 1 com 3 itens |
| Buffers A, B, C | Fila, capacidade 2 | | enunciado |
| Solda | Processo | Triangular(30, 38, 48); quebras MTBF Exp(7220,67), MTTR Exp(1612,02) | ajuste abaixo |
| Calibração | Processo | LogNormal(média 44,443; dp 6,148) | ajuste abaixo |
| Inspeção | Processo, capacidade 3 (3 bancadas), precisa de 1 operador da equipe "inspetores" no processamento | Gamma(forma 4,1172; escala 24,7874), média 102,05 s | ajuste abaixo |
| Saída da inspeção | regra "Pesos" 88 / 12 | porta 1 = expedição, porta 2 = fila do retrabalho | 12% de reprovação a cada passagem |
| Retrabalho | Processo | Triangular(150, 240, 420) | enunciado |
| Fila de retorno | Fila sem limite, ligada à calibração ANTES do buffer B | | a ordem das ligações dá a prioridade de puxar |
| Inspetores | 4 Operadores (Transporter), 1,3 m/s, voltam à base, base no nó "no" (40, 11) | equipe "inspetores" (Despachante, primeiro livre) | 8 m até a bancada (nó "no_2" em (40, 3)) |
| Calendário | turnos em 11 objetos (2 chegadas, 5 processos, 4 inspetores) | período 86.400 s; fora de turno [14.400, +1.800], [43.200, +1.800], [57.600, +28.800] | t = 0 é 06h00; 37,5% do dia fora de turno |
| Expedição | Saída (Sink) | | |

Ajustes com `utsim fit <csv> --coluna 2` (relatórios HTML em `fit_*.html`):

| Dado | n | média | CV | Escolhida | Observação |
|---|---|---|---|---|---|
| Calibração | 250 | 44,44 | 0,139 | Lognormal(mu 3,785; sigma 0,138), KS p 0,375, AD p 0,233, χ² p 0,528 | 7 famílias aceitas e equivalentes (ΔAIC < 2,3); 40% de valores repetidos (1 casa decimal) |
| Inspeção | 300 | 102,1 | 0,512 | Gama(4,117; 24,787), KS p 0,083, AD p 0,169 | Lognormal deslocada tem AIC 0,64 menor, mas a regra de parcimônia escolhe a Gama; Ljung-Box no limite (p = 0,047) |
| Entre falhas da solda | 60 | 7220,7 | 1,073 | Exponencial, KS p 0,950 | coerente com falhas aleatórias |
| Reparo da solda | 60 | 1612,0 | 1,087 | Exponencial, KS p 0,488 | |

Atenção: sem `--coluna 2`, o `utsim fit` ajusta a coluna de numeração (1, 2, 3...) e recomenda
Uniform(1, 250) para a calibração. Ver o relatório do piloto.

## 2. Tabela de validação (modelo base, 3 ou 4 inspetores, buffers de 2)

| Conferência | Esperado | Simulado | OK? |
|---|---|---|---|
| Fração reprovada na inspeção | 12% | 1.135,55 / 9.538,85 = 11,90% | sim |
| Inspeções por peça aprovada | 1 / 0,88 = 1,136 | 9.538,85 / 8.403,3 = 1,135 | sim |
| Solda em falha (relógio corrido) | MTTR / (MTBF + MTTR) = 18,25% | 17,64% ± 0,84% | sim (IC cobre) |
| Fração fora de turno (todos os objetos com turno) | 9 h / 24 h = 37,5% | 37,50% ± 0,00 | sim |
| Lei de Little na fila do retrabalho | λW = (1.135,55 / 864.000) x 436,06 = 0,573 | L = 0,568 ± 0,091 | sim |
| Bases geradas | 54.000 s úteis/dia x fração "gerando" / 55 s | 0,5346 x 864.000 / 55 = 8.398 contra 8.402,7 | sim |
| Inspetor 4 (equipe de 4, 3 bancadas) | nunca é chamado | 0% de uso | sim (só há 3 bancadas) |

O tempo de atravessamento médio (848 s) parece baixo para uma linha que para 8 h por noite, mas
fecha: a maioria das peças entra e sai no mesmo turno; as poucas peças que estão na linha às 22h
carregam 28.800 s cada, e isso explica boa parte da média e do seu aumento quando o buffer cresce.

## 3. Pergunta 1: quantos inspetores por turno?

Experimento montado na aba Cenários (`cap/r15/modelo_caso01_experimento.ron`), chave `inspetores.team`.

| Cenário | Aprovadas por dia | Δ pareado vs 4 inspetores (por dia) | Atravessamento (s) | Utilização média dos inspetores no turno |
|---|---|---|---|---|
| 4 inspetores | 840,3 ± 11,0 | referência | 848,4 ± 20,2 | 49,2% (o 4º fica parado) |
| 3 inspetores | 840,3 ± 11,0 | +0,0 ± 0,0 (≈) | 848,4 ± 20,2 | 65,6% (± ~0,9) |
| 2 inspetores | 794,1 ± 10,1 | −46,2 ± 1,8 (pior) | 1.415,8 ± 27,5 | 85,6% (± ~1,1) |

Utilização no turno = (utilize + travel_empty) / (1 − off_shift), com off_shift = 0,375. Por inspetor
(3 inspetores): 75,7%, 67,3% e 53,6%. A regra "primeiro livre" do despachante concentra o trabalho no
inspetor 1; com "rodízio" a carga se distribui, mas a média é a mesma. O IC da média entre inspetores
foi estimado de forma conservadora pela soma das meias-amplitudes (o editor não dá a métrica pronta).

**Resposta: 3 inspetores por turno.** Com 3, a vazão é idêntica à de 4 (a diferença pareada é
exatamente zero: com 3 bancadas, no máximo 3 inspeções acontecem ao mesmo tempo, e o 4º inspetor nunca
é chamado) e a utilização média no turno é de 65,6%, abaixo de 85%. Com 2, a vazão cai 46 peças por
dia (significativo) e a utilização média passa de 85% (85,6%).

## 4. Pergunta 2: onde colocar mais buffer?

Com 3 inspetores. Seis posições extras em cada lugar (`cap/r16`):

| Cenário | Aprovadas por dia | Δ pareado (por dia) | Atravessamento (s) |
|---|---|---|---|
| Atual (2, 2, 2) | 840,3 ± 11,0 | referência | 848,4 ± 20,2 |
| A + 6 (8, 2, 2) | 862,4 ± 10,5 | +22,0 ± 0,96 | 1.088,1 ± 25,2 |
| B + 6 (2, 8, 2) | 840,5 ± 11,0 | +0,18 ± 0,13 | 845,1 ± 18,0 |
| C + 6 (2, 2, 8) | 840,3 ± 11,0 | −0,05 ± 0,13 (≈) | 841,1 ± 18,0 |

**O buffer A (antes da solda) é o único que rende.** Varredura no buffer A (`cap/r17`):

| Extras em A | Capacidade A | Aprovadas por dia | Δ pareado (por dia) | Atravessamento (s) | Cumpre 850/dia e 1.200 s? |
|---|---|---|---|---|---|
| 0 | 2 | 840,3 ± 11,0 | referência | 848,4 ± 20,2 | não (vazão) |
| 2 | 4 | 849,4 ± 10,8 | +9,1 ± 0,55 | 910,6 ± 22,2 | não (849,4 < 850) |
| **4** | **6** | **856,7 ± 10,9** | +16,4 ± 0,57 | **995,6 ± 22,8** | **sim** |
| 6 | 8 | 862,4 ± 10,5 | +22,0 ± 0,96 | 1.088,1 ± 25,2 | sim |
| 8 | 10 | 867,2 ± 10,5 | +26,9 ± 1,08 | 1.188,8 ± 26,9 | sim na média, mas o IC passa de 1.200 |
| 10 | 12 | 871,5 ± 10,5 | +31,1 ± 1,11 | 1.295,5 ± 32,5 | não (atravessamento) |

**Resposta: no buffer A, o menor número que cumpre os dois critérios é 4 posições extras (buffer A com 6).**

Por que: a solda quebra com reparos longos (média de 27 min) e, com só 2 lugares no buffer A, a
estação 1 bloqueia em poucos minutos; a fila de bases enche e o alimentador de bases para (ele espera
o espaço vagar, então a chegada perdida não volta). Depois do reparo, a solda (38,7 s por peça) é mais
rápida que a estação 1 (cerca de 52,7 s com setup) e que a chegada de bases (55 s), então ela não
consegue recuperar o que perdeu: fica ociosa 13,5% do tempo. Um buffer A maior deixa a estação 1 e os
alimentadores trabalharem durante o reparo, e a solda consome o estoque depois. Depois da solda (B e
C) há folga: calibração e inspeção têm capacidade sobrando e a solda quase nunca fica bloqueada
(0,6%), então espaço extra ali não muda a vazão. O preço do buffer A é o estoque em processo: cada 2
posições somam cerca de 90 a 100 s ao atravessamento, e isso esbarra no limite de 1.200 s a partir de
8 a 10 extras.

## 5. Pergunta 3: plano de menor custo

Otimização em grade na aba Otimização (63 pontos, 20 replicações por ponto; buffer A de 2 a 14, B e
C de 2 a 6, em passos de 2; custos 40, 60 e 50 por posição; restrições sobre a média: aprovadas ≥
8.500 na janela de 10 dias e atravessamento ≤ 1.200 s). Conferi com `utsim otimizar` (mesmo
resultado).

| Plano | Extras | Custo das extras | Aprovadas por dia | Atravessamento (s) |
|---|---|---|---|---|
| **3 inspetores, A = 6, B = 2, C = 2** | 4 em A | **R$ 160** | **856,7 ± 10,9** (IC 845,9 a 867,6) | 995,6 ± 22,8 |
| 3 inspetores, A = 4, B = 4, C = 2 | 2 em A e 2 em B | R$ 200 | viável | viável |
| 3 inspetores, A = 8, B = 2, C = 2 | 6 em A | R$ 240 | 862,4 ± 10,5 | 1.088,1 ± 25,2 |

O objetivo do otimizador soma o custo de todas as posições (460 no melhor ponto, contando as 2 já
existentes em cada buffer: 40 x 6 + 60 x 2 + 50 x 2); o custo das extras é 460 − 300 = R$ 160.

**Resposta: 3 inspetores por turno e 4 carrinhos extras no buffer A (A = 6, B = 2, C = 2), custo de
R$ 160.** Vazão de 856,7 ± 10,9 aprovadas por dia e atravessamento de 995,6 ± 22,8 s.

Ressalva para a gerência: a meta vale para a média, e a média cumpre (856,7), mas o limite inferior
do IC (845,9) fica abaixo de 850. Se a gerência quiser folga com 95% de confiança, o plano A = 8
(R$ 240, 862,4 ± 10,5, IC de 851,9 a 872,9) cumpre a meta com o IC inteiro acima de 850.

## 6. Dificuldades encontradas

Fácil: ajuste de distribuições (com `--coluna 2`), quebras e turnos pelo painel, a aba Cenários com
diferença pareada e a otimização em grade.

O que faltou ou tive de aproximar:

- Não há campo de posição no painel do objeto. Os 4 inspetores na mesma mesa de apoio ficaram
  empilhados no mesmo ponto e só dava para selecionar o de cima. Resolvi com um truque: criei uma rede
  de caminhos com um nó na mesa e usei "Base (nó da rede)", que move o operador para o nó.
- O calendário (3 janelas) teve de ser digitado em 11 objetos, um por um (88 campos). Não há turno
  global nem copiar e colar de propriedades.
- A ordem das ligações define a prioridade da fila de retorno; o editor não mostra nem deixa trocar
  essa ordem. Tive de ligar na ordem certa desde o início.
- A janela Resultados aponta a estação 1 como gargalo com ocupação de 347%: o Combiner conta as placas
  que esperam dentro dele. Ignorei.
- A utilização do inspetor dentro do turno não existe pronta: calculei (utilize + travel_empty) /
  (1 − off_shift) à mão.
- No editor, o ✓ e o ✗ da otimização aparecem como um quadrado vazio, e não dá para ver quais pontos
  são viáveis. Conferi pelo `utsim otimizar`.
- As 3 bancadas são um Processo de capacidade 3 num único ponto (40, 3): todas a 8 m da mesa.

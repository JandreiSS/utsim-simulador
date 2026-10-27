# Estudo de caso 1: resolução (gabarito)

Este arquivo é só para quem corrige. Quem resolve o caso recebe apenas o `enunciado.md`, os arquivos de `dados/` e o manual.

Arquivos de referência desta pasta:

| Arquivo | Para que serve |
|---|---|
| `modelo.ron` | modelo de referência (20 objetos, 15 conexões) |
| `experimento.ron` | perguntas 1 e 2 (`utsim experiment`) |
| `otimizacao.ron` | pergunta 3 (`utsim otimizar`) |
| `gerar_dados.py` | gera os CSVs de `dados/` com semente fixa |

```bash
utsim check docs/estudos_de_caso/01-linha-de-montagem/gabarito/modelo.ron
utsim run docs/estudos_de_caso/01-linha-de-montagem/gabarito/modelo.ron
utsim experiment docs/estudos_de_caso/01-linha-de-montagem/gabarito/experimento.ron
utsim otimizar docs/estudos_de_caso/01-linha-de-montagem/gabarito/otimizacao.ron
```

## De onde vieram os dados

Os CSVs foram gerados com `gerar_dados.py` (semente fixa). As distribuições verdadeiras só aparecem aqui:

| Arquivo | n | Distribuição verdadeira | O que o `utsim fit` recomenda |
|---|---|---|---|
| `tempos_inspecao.csv` | 300 | Gama(forma 4, escala 25), média 100 s | `Gamma(shape: 4.117, scale: 24.787)` |
| `tempos_calibracao.csv` | 250 | LogNormal(média 44 s, dp 6 s) | `LogNormal(mean: 44.443, sd: 6.148)` |
| `tempos_reparo_solda.csv` | 60 | Exponencial(média 1500 s) | `Exp(mean: 1612.0)` |
| `entre_falhas_solda.csv` | 60 | Exponencial(média 7200 s) | `Exp(mean: 7220.7)` |

O modelo de referência usa os valores **ajustados**, que é o que um bom analista faria. Com 60 observações o ajuste do reparo erra em 7% (1612 contra 1500); isso é normal e faz parte do caso: o resultado de quem usar `Exp(mean: 1500)` será um pouco melhor. Os arquivos têm uma coluna de numeração antes do tempo, então é preciso usar `--coluna 2` no `utsim fit`.

## Como o modelo de referência traduz o enunciado

| Enunciado | Modelo |
|---|---|
| bases e placas chegando | duas `Source` com `Uniform`; filas `fila_bases` (capacidade 4) e `fila_placas` (capacidade 12) |
| estação 1, kit de 1 base + 3 placas | `Combiner(components: [3], mode: Pack)`, com `setup: Uniform(3, 7)` e `process: Triangular(38, 45, 60)`. A fila de bases é ligada primeiro (porta 0, o contêiner) e a de placas depois (porta 1) |
| buffers A, B, C com 2 posições | `fila_solda`, `fila_cal` e `fila_insp`, `Queue(capacity: 2)` |
| solda com quebras | `Processor` com `downtime: (mtbf: Exp(7220.7), mttr: Exp(1612.0))` |
| calibração com prioridade para o retrabalho | `fila_retorno` é ligada **antes** de `fila_cal` na calibração: a ordem das conexões de entrada é a prioridade de puxar |
| inspeção com 3 bancadas e inspetores | `Processor(capacity: 3, operators: OperatorUse(team: "inspetores", count: 1, setup: false, process: true))` |
| inspetores a 8 m da bancada | quatro `Transporter(speed: 1.3, home: true)` na posição da mesa de apoio, dentro de um `Dispatcher(rule: Nearest)`. A equipe tem 3 membros no modelo base |
| 12% reprovadas | `output: Weighted([0.88, 0.12])` na inspeção (porta 1: expedição, porta 2: retrabalho) |
| retrabalho volta à calibração | conexão `retrabalho -> fila_retorno` (forma um laço; o `utsim check` aceita) |
| dois turnos, refeição, noite parada | `shifts: (period: 86400, off: [(14400, 1800), (43200, 1800), (57600, 28800)])`, igual em todos os objetos da linha (inclusive fontes e inspetores) |

Notas de modelagem:

- **A quebra conta no relógio.** O `mtbf` do UTSim conta em tempo corrido, e o enunciado diz de propósito que as falhas foram registradas assim. Por isso o estado `down` da solda aparece com 18,5% da janela (18,3% esperado pela fórmula MTTR / (MTBF + MTTR)). Quem entendeu que o MTBF é "em horas de operação" e dividiu os valores por 0,625 comete um erro de modelagem. Veja a lacuna 5 do [README](../../README.md).
- **O quarto inspetor nunca trabalha**, porque só há 3 bancadas e cada inspeção usa uma pessoa. Por isso "4 inspetores" e "3 inspetores" dão resultados idênticos (mesmas sementes). A pergunta 1 testa se o analista percebe isso.
- **A semente é a mesma em todos os cenários**: a diferença pareada entre cenários é muito mais estreita que o intervalo de cada um.
- Janela: `stop_time: 950400` (11 dias), `warmup: 86400` (1 dia), 20 replicações. Peças por dia = `expedicao.input / 10`.

## Validação do modelo base (3 inspetores, buffers de 2)

Resultados do `utsim run` (20 replicações, média ± IC de 95%):

| Medida | Valor |
|---|---|
| Peças aprovadas na janela de 10 dias | 8.328 ± 147 (**832,8 ± 14,7 por dia**) |
| Peças inspecionadas | 9.442 ± 167 |
| Peças enviadas ao retrabalho | 1.114 ± 24 (**11,8%**; esperado 12%) |
| Peças inspecionadas por peça aprovada | 1,134 (esperado 1 / 0,88 = 1,136) |
| Solda: fração do tempo `down` | 18,5% ± 1,2 (esperado: 1612 / (7221 + 1612) = 18,3%) |
| Tempo de atravessamento médio | 840,5 ± 23,8 s |
| Bancadas de inspeção ocupadas em média | 1,84 ± 0,06 |
| Fonte de bases bloqueada | 9,5% do tempo (perda de produção por falta de espaço antes da solda) |

Conferência pela lei de Little na inspeção: λ = 9.442 / 864.000 s (janela de 10 dias corridos) = 0,01093 peça/s; W = 168,5 s (inclui o tempo de espera pelo inspetor e a caminhada); L = 0,01093 × 168,5 = 1,84, igual ao conteúdo médio.

## Pergunta 1: quantos inspetores?

Experimento (`experimento.ron`), 20 replicações:

| Cenário | Peças aprovadas na janela | Por dia | Atravessamento (s) |
|---|---|---|---|
| 2 inspetores | 7.879 ± 138 | 787,9 | 1.424 ± 31 |
| 3 inspetores | 8.328 ± 147 | 832,8 | 841 ± 24 |
| 4 inspetores | 8.328 ± 147 | 832,8 | 841 ± 24 |

Diferença pareada de 3 inspetores contra 2: **+449 ± 23 peças** na janela (+45 por dia, ± 2,3); o intervalo está longe de zero. Contra 4 inspetores a diferença é exatamente zero.

Utilização dos inspetores dentro do turno (`(utilize + travel_empty) / (1 - off_shift)`, com `off_shift` = 37,5%):

| Cenário | Inspetor 1 | Inspetor 2 | Inspetor 3 | Média |
|---|---|---|---|---|
| 2 inspetores | 85,8% | 84,2% | | 85,0% |
| 3 inspetores | 73,7% | 66,3% | 54,5% | 64,8% |

**Resposta: 3 inspetores.** Com 2, a vazão cai 45 peças por dia, a fila diante da inspeção sobe de 8 s para 98 s de espera e a utilização fica no limite de 85%. A espera por inspetor (`waiting_operator`) é de 0,3% do tempo com 3 pessoas. Uma quarta pessoa não ajuda: as 3 bancadas já limitam a inspeção a 3 pessoas ao mesmo tempo.

## Pergunta 2: onde colocar o buffer?

Seis posições extras (buffer de 8) em cada lugar, com 3 inspetores (20 replicações):

| Onde | Peças aprovadas na janela | Por dia | Atravessamento (s) |
|---|---|---|---|
| nenhum (base) | 8.328 ± 147 | 832,8 | 841 ± 24 |
| antes da solda (A) | **8.560 ± 140** | **856,0** | 1.093 ± 37 |
| depois da solda (B) | 8.329 ± 147 | 832,9 | 840 ± 26 |
| antes da inspeção (C) | 8.328 ± 147 | 832,8 | 838 ± 27 |

Diferenças pareadas em relação à base: A **+232 ± 14 peças** (+23 por dia); B e C: +0,8 ± 0,9 e −0,6 ± 0,9, sem diferença significativa.

Curva do buffer A (40 replicações, mesma semente; cada linha soma posições extras ao buffer de 2):

| Posições extras | Por dia | Atravessamento (s) | Meta de 850 por dia | Limite de 1.200 s |
|---|---|---|---|---|
| 0 | 833,6 ± 9,2 | 846 | não | sim |
| 2 | 843,1 ± 8,9 | 913 | não | sim |
| 4 | 850,6 ± 8,7 | 1.001 | no limite (+0,6) | sim |
| **6** | **856,4 ± 8,6** | **1.099** | **sim** | **sim** |
| 8 | 861,0 ± 8,6 | 1.202 | sim | não (1.202) |
| 10 | 865,1 ± 8,5 | 1.324 | sim | não |

**Resposta: antes da solda (buffer A), com 6 posições extras (buffer de 8).** Com 4 extras a média cai em cima da meta (850,6 contra 850) e qualquer repetição com outra semente pode ficar abaixo; com 8 extras o atravessamento passa de 1.200 s. Aceite como boa a resposta "4 ou 6 extras, com 6 mais segura", desde que o aluno mostre os números.

### Por que antes da solda?

- A **entrada da linha é o que limita a vazão** (um item a cada 55 s em média, e a base só entra se a fila tiver espaço). Quando a solda quebra, em média por 1.612 s, o buffer A de 2 posições enche em poucos minutos, a montagem bloqueia (10,5% do tempo), a fila de bases enche e o alimentador de bases também bloqueia (9,5% do tempo). **Tempo de alimentador bloqueado é produção perdida para sempre**, porque ninguém compensa.
- As estações depois da solda têm folga (calibração ocupada 77% do tempo de turno, inspeção com 3 inspetores 65%). Quando a solda volta, elas "esvaziam" o que vier depois com facilidade: o buffer B tem 0,4 peça em média e quase nunca enche (conteúdo médio 0,40 de 2). Um buffer ali nunca é usado.
- O buffer C pela mesma razão: conteúdo médio de 0,10 peça.
- Por isso o buffer só ajuda **do lado de quem fica bloqueado** (antes do equipamento que quebra), e não do lado de quem fica faminto, desde que o equipamento seguinte seja mais rápido. O contrário seria verdade se a calibração fosse o gargalo.
- O preço do buffer A é a espera: pela lei de Little, cada peça a mais parada ali soma ao atravessamento.

## Pergunta 3: o plano de menor custo

`otimizacao.ron` (grade com 189 pontos, 10 replicações por ponto, refinamento dos 4 melhores com 30 replicações), metas de 850 por dia (8.500 na janela) e 1.200 s:

| | |
|---|---|
| Melhor ponto | **buffer A = 8, buffer B = 2, buffer C = 2, 3 inspetores** |
| Custo das posições | 8 × 40 + 2 × 60 + 2 × 50 = **R$ 540** (R$ 240 de posições extras sobre o que já existe) |
| Peças aprovadas na janela | 8.544 ± 103 (**854 por dia**) |
| Atravessamento | 1.103 ± 29 s |
| Equivalente (teste pareado) | o mesmo plano com 4 inspetores (sem custo no arquivo; escolher 3 pela simplicidade) |

Observação importante: o ponto A = 6 (4 extras, custo R$ 460) parece viável na busca com 10 replicações e **vira inviável no refinamento com 30 replicações**: a vazão média fica abaixo de 8.500. É o refinamento fazendo o seu trabalho. A resposta do aluno (A = 8) deve coincidir com a da otimização, ou ser A = 6 acompanhada de uma discussão honesta da margem.

A equipe de inspetores entra como variável de escolha no arquivo e **não tem custo** (variável de escolha não aceita custo; veja a lacuna 6 do README). Por isso o resultado diz "3 e 4 inspetores são equivalentes" em vez de preferir 3 por custo.

## Funcionalidades do UTSim que o caso exercita

| Funcionalidade | Onde |
|---|---|
| `Combiner` com componentes (kit de 3 placas) e prioridade de portas | estação 1 |
| quebras (`downtime`) com MTBF e MTTR vindos de dados | solda |
| turnos (`shifts`) com refeição e noite, em todos os objetos | linha toda |
| saída ponderada (`Weighted`) e retorno em laço (retrabalho) | inspeção |
| prioridade por ordem de conexão | `fila_retorno` na calibração |
| operadores que andam (`OperatorUse`, `Dispatcher`, `Nearest`) | inspeção |
| ajuste de distribuições a partir de dados (`utsim fit`) | 4 arquivos CSV |
| experimentos com cenários e diferença pareada | `experimento.ron` |
| otimização com restrições e refinamento | `otimizacao.ron` |
| bloqueio e falta de peça (`blocked`) | fontes, montagem e buffers |

## Erros comuns esperados

- Ligar a fila de placas antes da de bases no Combiner (a base seria tratada como componente e o modelo travaria).
- Esquecer que as peças retrabalhadas voltam para a calibração, e não para a solda.
- Pôr a quebra em tempo de operação (dividir o MTBF por 0,625).
- Esquecer de aplicar os turnos aos alimentadores, o que enche a linha à noite.
- Medir a vazão pela saída das estações em vez da `expedicao.input`.
- Concluir "o buffer depois da solda ajuda porque protege a calibração": o experimento mostra que não.

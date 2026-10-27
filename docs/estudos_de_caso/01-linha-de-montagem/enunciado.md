# Estudo de caso 1: linha de montagem com inspeção e retrabalho

## A empresa e o problema

A Vale Verde Eletrônica fabrica controladores de irrigação. Os pedidos cresceram e a gerência de produção quer saber se a linha de montagem atual consegue entregar **850 controladores aprovados por dia** sem comprar equipamento novo. Hoje a linha entrega menos que isso e ninguém sabe dizer se o culpado é a inspeção, a solda (que quebra bastante) ou a falta de espaço entre as estações.

Você foi chamado para construir um modelo de simulação da linha e responder às três perguntas da seção "Perguntas de decisão".

## Como a linha funciona

A linha tem quatro estações em sequência, mais um posto de retrabalho. Entre uma estação e a seguinte há um pequeno **buffer** (um carrinho ou prateleira onde as peças esperam).

```text
 bases ---> [fila de bases] --\
                               >--> ESTAÇÃO 1 --> buffer A --> ESTAÇÃO 2 --> buffer B --> ESTAÇÃO 3 --> buffer C --> ESTAÇÃO 4 --> aprovadas --> expedição
 placas --> [fila de placas] -/     montagem                      solda                    calibração                  inspeção
                                    do kit                      (quebra)                                      (3 bancadas,
                                                                                                               inspetores)
                                                   ^                                            |  12% reprovadas
                                                   |                                            v
                                         [fila de retorno] <---------------------------- RETRABALHO
```

1. **Estação 1, montagem do kit.** Cada peça é uma **base** (a placa-mãe vazia) que recebe **3 placas pequenas** (sensores). As bases e as placas chegam de dois alimentadores separados e esperam em filas: a fila de bases guarda até 4 e a fila de placas guarda até 12. A estação só começa quando tem 1 base e 3 placas. O que sai é um conjunto único (a base já com as 3 placas).
2. **Estação 2, solda.** Solda os componentes. É a única estação que **quebra**.
3. **Estação 3, calibração.** Ajusta o controlador.
4. **Estação 4, inspeção.** Há **3 bancadas** de inspeção em paralelo. Cada inspeção ocupa uma bancada **e um inspetor** durante todo o tempo da inspeção. Em cada passagem pela inspeção, **12% das peças são reprovadas**; as outras 88% são aprovadas e vão para a expedição.
5. **Retrabalho.** As peças reprovadas esperam numa fila (sem limite de espaço) diante de um único posto de retrabalho. Depois de retrabalhadas, voltam para a **fila da estação 3** (calibração) e passam de novo pela calibração e pela inspeção. A taxa de 12% de reprovação vale a cada passagem pela inspeção, inclusive para peças que já foram retrabalhadas. Ao voltar, elas esperam numa fila própria, também sem limite de espaço, e **têm prioridade** sobre as peças novas quando a calibração fica livre.

Quando um buffer está cheio, a estação anterior não consegue entregar a peça e fica parada (bloqueada) com ela. Quando o buffer está vazio, a estação seguinte espera (falta de peça).

## Calendário de trabalho

A fábrica opera **dois turnos de 8 horas por dia**: o 1º das 06h às 14h e o 2º das 14h às 22h. Há uma **parada de refeição de 30 minutos** no meio de cada turno (das 10h00 às 10h30 e das 18h00 às 18h30). Das 22h às 06h a linha fica parada. Toda a linha para junto: os alimentadores, as estações e os inspetores. Uma peça que estava sendo trabalhada na hora da parada continua de onde parou quando a linha volta. Ignore sábados, domingos e feriados (considere dias corridos de produção).

Considere que a linha **começa o primeiro dia às 06h00, vazia**.

## Dados

Todos os tempos estão em **segundos** e as distâncias em **metros**.

### Alimentadores e filas

| Item | Dado |
|---|---|
| Chegada de bases | uma base a cada 45 a 65 s, qualquer valor do intervalo com a mesma chance |
| Chegada de placas pequenas | uma placa a cada 12 a 24 s, qualquer valor do intervalo com a mesma chance |
| Fila de bases | até 4 bases |
| Fila de placas | até 12 placas |

Se uma fila de entrada está cheia, o alimentador espera o espaço vagar antes de produzir a próxima peça.

### Estações

| Estação | Capacidade | Tempo de preparação | Tempo de processo |
|---|---|---|---|
| 1. Montagem do kit | 1 | 3 a 7 s (uniforme) | mínimo 38 s, mais comum 45 s, máximo 60 s |
| 2. Solda | 1 | nenhum | mínimo 30 s, mais comum 38 s, máximo 48 s |
| 3. Calibração | 1 | nenhum | medido (arquivo `dados/tempos_calibracao.csv`) |
| 4. Inspeção | 3 bancadas | nenhum | medido (arquivo `dados/tempos_inspecao.csv`) |
| Retrabalho | 1 posto | nenhum | mínimo 150 s, mais comum 240 s, máximo 420 s |

Os arquivos `dados/*.csv` têm duas colunas separadas por ponto e vírgula: o número da medição e o tempo, em segundos, com vírgula decimal. Use os dados para escolher a distribuição de cada tempo.

### Buffers entre as estações

Hoje cada um dos três buffers (A entre as estações 1 e 2, B entre 2 e 3, C entre 3 e 4) guarda **no máximo 2 peças**. A fila de retorno do retrabalho não tem limite.

### Quebras da solda

A solda fica ligada o tempo todo, mesmo fora de turno, e o sistema de manutenção registra as quebras em **tempo corrido de relógio**, 24 horas por dia. Isso significa que a máquina também pode quebrar à noite (e a parada acaba contando como parada de equipamento). Os registros de 60 quebras estão em:

- `dados/entre_falhas_solda.csv`: o tempo entre o fim de um reparo e a falha seguinte, em segundos;
- `dados/tempos_reparo_solda.csv`: a duração de cada reparo, em segundos.

Durante o reparo a máquina não processa. A peça que estava na máquina na hora da quebra continua de onde parou depois do reparo.

### Inspetores

Cada inspeção em andamento precisa de **um inspetor do início ao fim**. Os inspetores ficam numa **mesa de apoio**, a **8 metros** das bancadas. Quando uma bancada está pronta para inspecionar uma peça, um inspetor livre vai caminhar até ela (velocidade de 1,3 m/s), faz a inspeção e, se não houver outra peça esperando, volta para a mesa de apoio. Os inspetores trabalham nos mesmos turnos e nas mesmas paradas da linha. A empresa pode ter até 4 inspetores por turno.

### Posições na planta (metros)

As peças passam de uma estação para a seguinte pelo buffer, por transferência manual rápida: **ignore o tempo de deslocamento das peças entre estações**. A única distância que importa é a dos inspetores. Mesmo assim, aqui está a planta, para você organizar o layout:

| Objeto | x | y |
|---|---|---|
| Alimentador de bases | 0 | 0 |
| Alimentador de placas | 0 | 6 |
| Fila de bases | 4 | 0 |
| Fila de placas | 4 | 6 |
| Estação 1 | 10 | 3 |
| Buffer A | 14 | 3 |
| Estação 2 (solda) | 20 | 3 |
| Buffer B | 24 | 3 |
| Fila de retorno | 24 | 9 |
| Estação 3 (calibração) | 30 | 3 |
| Buffer C | 34 | 3 |
| Posto de retrabalho | 34 | 9 |
| Estação 4 (inspeção, 3 bancadas) | 40 | 3 |
| Fila do retrabalho | 40 | 9 |
| Mesa de apoio dos inspetores | 40 | 11 |
| Expedição | 48 | 0 |

## Medidas que a gerência usa

- **Peças aprovadas por dia**: total de controladores que chegam à expedição, dividido pelo número de dias de operação analisados.
- **Tempo de atravessamento**: tempo médio entre a entrada da base na linha e a chegada na expedição.
- **Utilização de um inspetor** (dentro do turno): tempo em que ele está caminhando ou inspecionando, dividido pelo tempo em que ele está em turno (sem contar paradas).

## Como analisar

- Analise **10 dias de operação**, depois de descartar o primeiro dia como aquecimento.
- Use pelo menos **20 replicações** e a **mesma semente** em todos os cenários que você comparar.
- Diga sempre a média e o intervalo de confiança de 95%.
- Considere que a meta de peças por dia vale para a média das replicações.

## Perguntas de decisão

**Pergunta 1. Quantos inspetores por turno?** Compare 2, 3 e 4 inspetores (com os buffers atuais de 2 posições). A inspeção **não pode ser o gargalo** da linha. Critérios de aceitação:

- a vazão de peças aprovadas com N inspetores não pode ser estatisticamente pior que a vazão com 4 inspetores (diferença pareada com intervalo de 95% que inclui zero ou favorece N);
- a utilização média dos inspetores (dentro do turno) deve ficar **abaixo de 85%**.

Escolha o menor N que cumpre as duas.

**Pergunta 2. Onde colocar mais buffer?** Com o número de inspetores que você escolheu, a gerência pode comprar **posições extras de buffer** (carrinhos) e colocá-las em **um só** dos três lugares: antes da solda (buffer A), depois da solda (buffer B) ou antes da inspeção (buffer C). Critérios de aceitação:

- **850 peças aprovadas por dia** (média das replicações);
- tempo de atravessamento médio de **até 1.200 s**.

Teste 6 posições extras em cada um dos três lugares e diga qual rende mais. Depois, no melhor lugar, diga o **menor número de posições extras** (em passos de 2: 0, 2, 4, 6, 8, 10) que cumpre os dois critérios. Explique por que o resultado é o que é.

**Pergunta 3. Plano de menor custo.** Cada posição extra tem um custo de aquisição: R$ 40 por posição no buffer A, R$ 60 no buffer B e R$ 50 no buffer C. Junte as respostas anteriores e proponha o plano (número de inspetores e tamanho de cada buffer, em passos de 2: de 2 a 14 posições no buffer A e de 2 a 6 nos buffers B e C) de **menor custo** que cumpre as duas metas da pergunta 2. Diga o custo total das posições extras e o intervalo de confiança da vazão do plano escolhido.

## O que entregar

1. O modelo, com a descrição das distribuições que você escolheu e por quê.
2. Uma tabela de validação: confira, por exemplo, que cerca de 12% das peças inspecionadas vão para o retrabalho e que a quebra da solda ocupa a fração do tempo que você espera.
3. Os resultados de cada pergunta, com intervalos de confiança.
4. Uma lista curta de **dificuldades que você encontrou** para montar o modelo (o que foi fácil, o que faltou, o que você teve de aproximar).

# Estudo de caso 3: pronto atendimento com turnos e prioridade

## A clínica e o problema

O Pronto Atendimento Bem-Estar atende pacientes sem hora marcada, todos os dias, das 07h às 19h. É um serviço, não uma fábrica: os "itens" são pessoas, e o que importa é **quanto elas esperam**. Os pacientes chegam ao longo do dia em ritmos muito diferentes (a manhã é cheia, o almoço é calmo, o fim da tarde esvazia). A clínica tem uma escala fixa de pessoal por turno e a diretoria acha que está gastando demais com ela, mas tem medo de aumentar as filas.

Você deve construir um modelo de simulação da clínica e dizer **qual é a escala de pessoal de menor custo que ainda cumpre as metas de espera**.

## Como o atendimento funciona

```text
 chegadas --> SALA DE ESPERA --> TRIAGEM -----> urgentes (12%) ---> médicos  (atendidos primeiro)
 (por hora)   (30 lugares)      (enfermeiros)   normais   (88%) ---> médicos  (atendidos depois dos urgentes)
                                                                       |
                                                                       v
                                                                    saída
```

1. **Chegada.** Os pacientes chegam ao acaso, de forma independente uns dos outros, num ritmo que muda com a hora (tabela abaixo). Chegam das 07h às 19h. Depois das 19h ninguém mais entra.
2. **Sala de espera.** Tem **30 lugares**. Se um paciente chega e a sala está cheia, ele **vai embora e é contado como perdido**. A sala é onde se espera pela triagem.
3. **Triagem.** Um enfermeiro mede os sinais vitais e classifica o paciente. **12% dos pacientes são urgentes** e 88% são normais, em qualquer hora do dia. O tempo de triagem é igual para os dois tipos.
4. **Consulta.** Depois da triagem o paciente espera um médico. **Os urgentes passam na frente dos normais**: sempre que um médico fica livre, ele atende primeiro o urgente que estiver esperando há mais tempo e, só se não houver urgente, o normal que estiver esperando há mais tempo. Uma consulta em andamento **nunca é interrompida** por causa de um urgente que chega. O tempo de consulta é medido para todos os pacientes juntos (arquivo de dados).
5. **Saída.** Depois da consulta o paciente vai embora.

## Turnos de pessoal

A clínica trabalha com três turnos, tanto para enfermeiros (triagem) quanto para médicos:

| Turno | Horário | Duração |
|---|---|---|
| S1 | 07h às 12h | 5 h |
| S2 | 12h às 17h | 5 h |
| S3 | 17h às 21h | 4 h |

O S3 vai até as 21h, duas horas depois do fim das chegadas, para **esvaziar a fila**. Cada profissional trabalha em **um** turno só. Uma consulta ou triagem em andamento na hora da troca de turno **é terminada pelo mesmo profissional** (hora extra) e a pessoa do turno seguinte já assume os novos pacientes; se o seu simulador não consegue representar isso, aproxime e registre a diferença. Os pacientes que ainda estiverem na fila às 21h esperam pelo turno S1 do dia seguinte.

### Escala atual

| Turno | Médicos | Enfermeiros de triagem |
|---|---|---|
| S1 (07h às 12h) | 6 | 3 |
| S2 (12h às 17h) | 5 | 2 |
| S3 (17h às 21h) | 4 | 2 |

### Limites da escala nos estudos

Nas análises de escala, a diretoria só aceita estas faixas:

| Turno | Médicos | Enfermeiros |
|---|---|---|
| S1 | de 4 a 7 | de 2 a 4 |
| S2 | de 4 a 7 | de 2 a 3 |
| S3 | de 2 a 4 | de 1 a 2 |

### Custos por turno

| Profissional | Custo por hora | S1 e S2 (5 h) | S3 (4 h) |
|---|---|---|---|
| Médico | R$ 220 | R$ 1.100 | R$ 880 |
| Enfermeiro de triagem | R$ 70 | R$ 350 | R$ 280 |

O custo da escala atual, por dia, é de R$ 17.930.

## Dados

Tempos em **minutos**.

### Chegadas por hora

Média de pacientes que chegam por hora, observada ao longo de vários meses:

| Hora | Pacientes por hora |
|---|---|
| 07h às 08h | 10 |
| 08h às 09h | 26 |
| 09h às 10h | 26 |
| 10h às 11h | 26 |
| 11h às 12h | 18 |
| 12h às 13h | 12 |
| 13h às 14h | 12 |
| 14h às 15h | 22 |
| 15h às 16h | 22 |
| 16h às 17h | 22 |
| 17h às 18h | 12 |
| 18h às 19h | 12 |
| **Total por dia** | **220** |

Dentro de cada hora, os pacientes chegam ao acaso e de forma independente. As chegadas variam com a hora do dia: represente isso da forma mais fiel que o simulador permitir e registre o que você teve de aproximar.

### Triagem e consulta

| Item | Dado |
|---|---|
| Tempo de triagem | 250 medições em `dados/triagem.csv` (minutos) |
| Tempo de consulta | 400 medições em `dados/consultas.csv` (minutos). As consultas de urgentes tendem a ser mais longas, mas o arquivo não diz quais são urgentes: use a distribuição do conjunto |

Os arquivos têm duas colunas separadas por ponto e vírgula: o número da medição e a duração, com vírgula decimal.

## Metas de espera

As metas valem para as **médias do dia**, sobre pelo menos 30 dias de operação:

| Meta | Limite |
|---|---|
| Espera na sala de espera, até começar a triagem | até **8 min** |
| Espera do paciente **urgente** pelo médico (depois da triagem) | até **3 min** |
| Espera do paciente **normal** pelo médico (depois da triagem) | até **15 min** |
| Pacientes perdidos (sala de espera cheia) | até **2%** dos que chegam |

## Como analisar

- Considere que a clínica **abre às 07h00 vazia** e que cada dia é igual ao anterior (sem sábado, domingo ou feriado).
- Simule **31 dias** e descarte o primeiro como aquecimento; a janela de análise tem então 30 dias.
- Use pelo menos **10 replicações** e a **mesma semente** em todos os cenários que comparar.
- Dê sempre a média e o intervalo de confiança de 95%.

## Perguntas de decisão

**Pergunta 1. A escala atual cumpre as metas?** Rode o modelo com a escala atual e diga, para cada meta, o valor encontrado e a folga. Antes de responder, valide o modelo: confira que chegam cerca de 220 pacientes por dia, 12% de urgentes e que as chegadas por hora seguem a tabela.

**Pergunta 2. Quanto a prioridade ajuda os urgentes?** Compare a escala ótima da pergunta 3 com e sem a prioridade para urgentes (sem ela, todo mundo entra numa fila única por ordem de chegada). Diga o que acontece com a espera dos urgentes e a dos normais.

**Pergunta 3. Qual a escala de menor custo?** Dentro das faixas da tabela de limites, encontre a combinação de médicos e enfermeiros em cada turno (seis números) de **menor custo diário** que cumpre as quatro metas. Diga o custo, a economia sobre a escala atual e o valor de cada meta na escala escolhida. Indique também se há escalas diferentes com o mesmo custo e o mesmo desempenho.

**Pergunta 4. E se a demanda crescer 15%?** A escala da pergunta 3 ainda cumpre as metas se chegarem 15% mais pacientes em todas as horas? Se não, quais metas falham e qual é a escala de menor custo que cumpre as metas com a demanda maior? Diga quanto custa a mais por dia.

## O que entregar

1. O modelo, com a descrição das distribuições escolhidas (mostre a saída do ajuste) e uma justificativa para cada.
2. Uma tabela de validação com os números da pergunta 1.
3. Os resultados de cada pergunta, com intervalos de confiança.
4. Uma lista curta de **dificuldades que você encontrou** para montar o modelo (o que foi fácil, o que faltou, o que você teve de aproximar, especialmente as chegadas que variam com a hora, as trocas de turno e a prioridade).

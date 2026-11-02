# Caso 3: Pronto Atendimento Bem-Estar, turnos e prioridade

Modelo: `trabalho/03-clinica-com-turnos/modelo_caso03.ron` (demanda atual) e
`modelo_caso03_d115.ron` (demanda +15%), montados e salvos pelo editor (roteiros `r01` a `r06`).
Unidade de tempo: minuto; t = 0 é 07h00 do primeiro dia. Rodadas: 31 dias (44.640 min), aquecimento
de 1 dia, janela de 30 dias, 10 replicações, semente 1, mesmas sementes em todos os cenários.
Valores: média ± meia-amplitude do IC de 95%.

Experimentos e otimizações pela linha de comando (`exp_caso3.ron`, `exp_q4.ron`, `otim_q3.ron`,
`otim_q4.ron`): com 6 variáveis e 576 pontos, montar o problema no editor seria possível, mas a grade
de cenários não mostra mais de 4 colunas; preferi a CLI.

## 1. Modelo e aproximações

| Elemento | Como modelei | Observação |
|---|---|---|
| Chegadas por hora | **12 Chegadas**, uma por hora (chegada_07 ... chegada_18), cada uma Exp(60 / taxa) e com turnos que a deixam ligada só na sua hora (período 1.440 min) | o UTSim não tem chegada com taxa variável no tempo; a exponencial sem memória torna a pausa fora da hora inofensiva |
| Paciente perdido | Chegada com "Se a saída estiver bloqueada: Descartar (perda)" | sala de espera = Fila com capacidade 30 |
| Triagem | 3 Processos (triagem_s1, s2, s3), capacidade = número de enfermeiros, cada um com o turno do seu grupo | Triangular(3,278; 4,9; 8,822), escolhida pelo `utsim fit triagem.csv --coluna 2` (KS p 0,169, AD p 0,329, χ² p 0,086; a Beta tem AIC 0,9 menor, mas mais parâmetros) |
| Urgente / normal | saída da triagem com "Pesos" 88 / 12 (porta 1 = fila normal, porta 2 = fila urgente) | 12% sorteados na triagem, independente da hora |
| Prioridade | fila_urgente ligada às 3 consultas **antes** da fila_normal (a ordem das ligações é a prioridade de puxar) | o médico livre puxa primeiro da fila urgente |
| Consulta | 3 Processos (consulta_s1, s2, s3), capacidade = número de médicos, com turno | Pearson V(forma 4,8206; escala 50,404), média 13,1 min, escolhida pelo `utsim fit consultas.csv --coluna 2` (KS p 0,083; AD p 0,053; χ² p 0,887) |
| Turnos | S1: fora de 300 a 1.440; S2: fora de 0 a 300 e de 600 a 1.440; S3: fora de 0 a 600 e de 840 a 1.440 | |

**Troca de turno (aproximação importante):** no UTSim, um Processo que sai de turno **pausa** o
atendimento em andamento, que só continua quando o mesmo objeto volta ao turno (no dia seguinte). Não
há como representar "o profissional termina a consulta em hora extra". Efeito: cerca de 3 a 4
pacientes por dia ficam "presos" numa consulta pausada até o dia seguinte (a permanência média em
consulta_s1 é de 61 a 68 min, contra 13 min de consulta). Isso não muda as esperas medidas nas filas,
mas deixa o tempo no sistema exagerado e tira alguns minutos de capacidade no começo de cada turno
no dia seguinte.

A meta de perdidos não tem métrica direta usável: a métrica `lost` aparece só na linha de estados da
chegada que perdeu alguém e o manual diz que `.lost.*` não serve como objetivo. Usei como restrição
`sala.input ≥ 98%` das chegadas esperadas e conferi os `lost` no relatório.

## 2. Validação (pergunta 1, escala atual)

| Conferência | Esperado | Simulado | OK? |
|---|---|---|---|
| Pacientes por dia | 220 | 6.601,2 / 30 = 220,0 ± 1,4 | sim |
| Urgentes | 12% | 785,6 / 6.601,2 = 11,9% | sim |
| Chegadas 07h-08h | 10/h | 307,5 / 30 = 10,3 | sim |
| Chegadas 08h-09h | 26/h | 773,9 / 30 = 25,8 | sim |
| Chegadas 12h-13h | 12/h | 359,1 / 30 = 12,0 | sim |
| Chegadas 15h-16h | 22/h | 664,1 / 30 = 22,1 | sim |
| Chegadas 18h-19h | 12/h | 365,4 / 30 = 12,2 | sim |
| Tempo de triagem no S3 (sem pausa) | 5,67 min | 5,67 | sim |

Nota: na primeira versão, a chegada das 18h ficou com uma janela fora de turno faltando e gerava
pacientes até 07h do dia seguinte (1.974 em vez de 365). Só percebi pela tabela de validação; o
painel mostra "fora de turno: 45,8% do período", mas nada avisa que a fonte está ligada 13 h.

## 3. Pergunta 1: a escala atual cumpre as metas?

| Meta | Limite | Escala atual (6/5/4 médicos, 3/2/2 enfermeiros) | Folga |
|---|---|---|---|
| Espera na sala até a triagem | 8 min | 6,29 ± 0,48 | 1,7 min |
| Espera do urgente pelo médico | 3 min | 1,02 ± 0,06 | 2,0 min |
| Espera do normal pelo médico | 15 min | 3,92 ± 0,39 | 11,1 min |
| Perdidos | 2% | 0% (sala chega a 22,7 lugares no pico) | 2 pp |

**Sim, cumpre as quatro**, com folga grande nos médicos e folga pequena na triagem (a sala de espera
é a meta mais apertada).

## 4. Pergunta 3: escala de menor custo

Busca em grade com todas as 576 combinações das faixas (10 replicações por ponto).

| Escala | Médicos S1/S2/S3 | Enfermeiros S1/S2/S3 | Custo por dia |
|---|---|---|---|
| Atual | 6 / 5 / 4 | 3 / 2 / 2 | R$ 17.930 |
| **Ótima** | **5 / 5 / 3** | **3 / 2 / 2** | **R$ 15.950** |
| Ótima alternativa, mesmo custo | 6 / 4 / 3 | 3 / 2 / 2 | R$ 15.950 |

Economia: **R$ 1.980 por dia** (11%).

| Meta | Ótima 5/5/3 | Alternativa 6/4/3 |
|---|---|---|
| Sala | 6,29 ± 0,48 | 6,29 ± 0,48 |
| Urgente | 1,69 ± 0,07 | 1,74 ± 0,06 |
| Normal | 13,49 ± 1,36 | 13,45 ± 1,23 |
| Perdidos | 0% | 0% |

As duas escalas têm o mesmo custo e desempenho estatisticamente igual (o otimizador as marca como
equivalentes no teste pareado). A espera do normal fica perto do limite (IC até 14,85 min): a escala
cumpre a meta, mas sem margem.

## 5. Pergunta 2: quanto a prioridade ajuda os urgentes? (escala ótima)

Sem prioridade (todos na fila única por ordem de chegada; fiz isso no cenário mandando 100% da
triagem para a fila normal):

| | Com prioridade | Sem prioridade (fila única) |
|---|---|---|
| Espera do urgente | 1,69 ± 0,07 | cerca de 12,1 (igual à média da fila única) |
| Espera do normal | 13,49 ± 1,36 | 12,09 ± 1,21 (fila única) |

Sem prioridade, o urgente espera como qualquer um: cerca de 12 min, quatro vezes a meta de 3 min. O
normal ganha só 1,4 min. A prioridade custa pouco aos normais e é o que faz a meta dos urgentes ser
cumprida. (Aproximação: sem separar os tipos na fila única, não há medida separada do urgente; como
os 12% são sorteados ao acaso, a espera do urgente numa fila FIFO é a espera média da fila.)

## 6. Pergunta 4: demanda +15%

| Meta | Escala da P3 (5/5/3; 3/2/2) com +15% | Nova ótima (6/5/4; 3/3/1) com +15% |
|---|---|---|
| Sala | **12,53 ± 0,56 (falha)** | 7,03 ± 0,57 |
| Urgente | 2,19 ± 0,06 | 1,46 ± 0,06 |
| Normal | **36,16 ± 2,87 (falha)** | 12,72 ± 0,86 |
| Perdidos | cerca de 0% (sala chega a 29,6 de 30 no pico) | 0% |
| Custo por dia | R$ 15.950 | **R$ 18.000** |

A escala da P3 deixa de cumprir a espera na sala e a espera do normal. A escala de menor custo com
+15% é **6/5/4 médicos e 3/3/1 enfermeiros, R$ 18.000 por dia**: R$ 2.050 a mais que a ótima da P3 e
só R$ 70 a mais que a escala atual. As próximas viáveis custam R$ 18.220 (6/6/3 ou 7/5/3 médicos com
3/3/1 enfermeiros).

## 7. Dificuldades

- Chegadas por hora: não há fonte com taxa variável. Precisei de 12 fontes com 2 janelas de turno
  cada (cerca de 70 campos digitados). Um erro numa janela passou despercebido até a validação.
- Troca de turno: não há "terminar o atendimento em hora extra"; a pausa até o dia seguinte é uma
  distorção.
- Número de profissionais por turno: só deu para variar com um Processo por turno. Com operadores
  (Transportadores) com turno, a prioridade dos urgentes se perderia, porque o Processo puxa o
  paciente da fila antes de ter médico livre (e o manual não diz o que acontece com um operador que
  sai de turno no meio de um atendimento).
- Perdidos: não há métrica de fração perdida utilizável em restrição.
- Sem prioridade: não há disciplina por prioridade na fila; a prioridade só existe pela ordem das
  ligações, que o editor não mostra.

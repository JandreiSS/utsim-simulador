# Estudo de caso 3: resolução (gabarito)

Este arquivo é só para quem corrige. Quem resolve o caso recebe apenas o `enunciado.md`, os arquivos de `dados/` e o manual.

| Arquivo | Para que serve |
|---|---|
| `modelo.ron` | modelo de referência com a escala atual (16 objetos, 24 conexões) |
| `experimento.ron` | perguntas 1, 2 e 4 (`utsim experiment`) |
| `otimizacao.ron` | pergunta 3: a escala de menor custo (`utsim otimizar`, grade com 576 pontos) |
| `modelo_demanda_15.ron` e `otimizacao_demanda_15.ron` | pergunta 4: o mesmo modelo com 15% mais pacientes e a escala de menor custo para ele |
| `gerar_dados.py` | gera os CSVs de `dados/` com semente fixa |

```bash
utsim check docs/estudos_de_caso/03-clinica-com-turnos/gabarito/modelo.ron
utsim run docs/estudos_de_caso/03-clinica-com-turnos/gabarito/modelo.ron
utsim experiment docs/estudos_de_caso/03-clinica-com-turnos/gabarito/experimento.ron
utsim otimizar docs/estudos_de_caso/03-clinica-com-turnos/gabarito/otimizacao.ron
utsim otimizar docs/estudos_de_caso/03-clinica-com-turnos/gabarito/otimizacao_demanda_15.ron
```

## De onde vieram os dados

| Arquivo | n | Distribuição verdadeira | O que o `utsim fit` recomenda |
|---|---|---|---|
| `triagem.csv` | 250 | Triangular(3, 5, 9), média 5,67 min | `Triangular(min: 3.278, mode: 4.9, max: 8.822)` |
| `consultas.csv` | 400 | **mistura**: 88% LogNormal(média 12, dp 5) e 12% LogNormal(média 25, dp 10), média 13,6 min | `PearsonV(shape: 4.8206, scale: 50.4039)`, média 13,2 min (Pearson VI, LogNormal deslocada, LogNormal e Log-logística também passam nos testes) |

Os urgentes têm consultas mais longas na verdade (média de 25 min), mas o enunciado entrega o conjunto sem rótulo, e o UTSim não deixa o tempo de uma consulta depender do tipo do paciente (lacuna 4). A aproximação é usar a mistura como uma distribuição só. A espera total não muda muito, mas a espera dos urgentes fica subestimada, porque na vida real o médico que pega um urgente fica ocupado por mais tempo.

## Como o modelo de referência traduz o enunciado

O relógio do modelo começa às 07h00 (t = 0) e cada dia tem 1.440 min.

| Enunciado | Modelo |
|---|---|
| chegadas que variam por hora | **seis** `Source` com `Exp`, cada uma ligada só dentro da sua faixa de horário (o resto do dia ela está "fora de turno", com o relógio de chegadas pausado). Faixas de taxa constante: 07-08 (10/h), 08-11 (26/h), 11-12 (18/h), 12-14 (12/h), 14-17 (22/h), 17-19 (12/h). Como o `Exp` não tem memória, pausar e retomar o relógio é exato |
| tipo do paciente | rótulo `classe` na `Source`: `Discrete([(1.0, 0.12), (2.0, 0.88)])`. 1 é urgente, 2 é normal |
| sala de espera de 30 lugares | `sala_espera`, `Queue(capacity: 30)`; as seis fontes têm `on_blocked: Drop` e o paciente que chega com a sala cheia é perdido (métrica `lost`) |
| triagem em turnos | três `Processor` (`triagem_S1`, `_S2`, `_S3`), cada um com `capacity` igual ao número de enfermeiros do turno e `shifts` com a janela fora do turno. `output: ByLabel("classe")` manda o paciente para a porta 1 (urgentes) ou 2 (normais) |
| prioridade dos urgentes | duas filas, `fila_urgente` e `fila_normal`. Cada grupo de médicos liga a `fila_urgente` **antes** da `fila_normal`; a ordem das conexões de entrada é a prioridade de puxar |
| consulta em turnos | três `Processor` (`medicos_S1`, `_S2`, `_S3`), cada um com `capacity` igual ao número de médicos do turno e `shifts` |
| turnos | S1: 0 a 300 min; S2: 300 a 600; S3: 600 a 840. Janelas fora de turno: o complemento de cada uma dentro do período de 1.440 min |
| o paciente que não termina no turno | fica **congelado** no profissional até o turno voltar no dia seguinte (lacuna 2) |

Detalhes que valem uma conferência:

- A ordem das conexões importa. As conexões `fila_urgente -> medicos_S*` vêm antes das `fila_normal -> medicos_S*`. Se vierem na ordem inversa, os normais passam na frente.
- Sem as seis fontes (por exemplo, com uma só fonte de taxa média de 18,3/h) a espera da manhã sai subestimada e a da tarde superestimada, e a escala de mínimo custo sai errada.
- Janela: `stop_time: 44640` (31 dias), `warmup: 1440` (1 dia), 10 replicações no modelo, 20 no experimento.

### O viés dos turnos (lacuna 2)

No UTSim uma troca de turno **pausa** o atendimento em andamento, e não o termina. Os pacientes em consulta às 12h ficam congelados até as 07h do dia seguinte. Consequências no modelo de referência:

1. O `time_in_system` da saída (93,8 min) e a permanência nos médicos (62 min no S1) saem inflados. **Não use essas medidas.** O esperado, sem congelamento, é uns 30 min (6 de sala de espera, 5,7 de triagem, 1 a 4 de fila de consulta, 13 de consulta). Use só as esperas nas filas.
2. No início do S2, os 4 pacientes congelados do dia anterior ocupam 4 dos 5 médicos pelo tempo que faltava (uns 9 min), justamente quando a fila do S1 chega. Isso tira umas 2,4% da capacidade do S2 (36 de 1.500 médico-minutos) e deixa a espera do S2 um pouco pessimista. O mesmo acontece em escala menor no S3. No início do S1 não faz diferença, porque a clínica abre vazia.

O enunciado pede ao aluno que registre a diferença. Aceite uma resposta que mencione o congelamento e use as esperas em fila.

## Pergunta 1: a escala atual cumpre as metas?

Escala atual: médicos 6/5/4 e enfermeiros 3/2/2, custo de R$ 17.930 por dia. Resultados do experimento (20 replicações):

| Meta | Limite | Valor | Cumpre |
|---|---|---|---|
| Espera na sala de espera | 8 min | 6,17 ± 0,28 | sim |
| Espera dos urgentes | 3 min | 0,98 ± 0,03 | sim |
| Espera dos normais | 15 min | 4,00 ± 0,23 | sim |
| Perdas | 2% | ~0,2% (saída: 6.589 ± 37 pacientes em 30 dias; chegam 6.600) | sim |

Validação: chegadas por dia por faixa 9,9 (esperado 10), 77,7 (78), 18,1 (18), 23,4 (24), 65,7 (66) e 24,0 (24), total **218,9 por dia (esperado 220)**; urgentes: 779,8 de 6.568 = **11,9%** (esperado 12%).

A escala atual cumpre com folga e a espera dos normais é só um quarto do limite: há espaço para economizar.

## Pergunta 3: a escala de menor custo

(Respondida antes da 2 porque a 2 usa o resultado.)

`otimizacao.ron`: grade com 576 pontos (médicos de S1 e S2 de 4 a 7, de S3 de 2 a 4, enfermeiros de S1 de 2 a 4, de S2 de 2 a 3, de S3 de 1 a 2), 5 replicações por ponto e refinamento dos 6 melhores com 30. O custo é a soma de custo por turno vezes o número de profissionais.

| | Médicos S1/S2/S3 | Enfermeiros S1/S2/S3 | Custo por dia |
|---|---|---|---|
| Atual | 6 / 5 / 4 | 3 / 2 / 2 | R$ 17.930 |
| **Ótima** | **5 / 5 / 3** | **3 / 2 / 2** | **R$ 15.950** |
| Equivalente | 6 / 4 / 3 | 3 / 2 / 2 | R$ 15.950 (mesmo custo e desempenho, "≈" no teste pareado) |

Economia: **R$ 1.980 por dia (11%)**. Desempenho da escala ótima (20 replicações): sala de espera 6,17 ± 0,28 min; urgentes 1,65 ± 0,04 min; normais **13,72 ± 0,68 min** (perto do limite de 15); perdas ~0,2%.

Pontos seguintes do refinamento: 5/5/3 com enfermeiros 3/3/1 (R$ 16.020, +70); 5/6/2 com 3/3/1 (R$ 16.240).

Uma busca exaustiva em domínio mais largo (14.112 pontos: médicos de 1 a 8, enfermeiros de 1 a 4) deu o mesmo ótimo, de modo que os limites do enunciado não estão atrapalhando. Uma busca genética (353 pontos avaliados), em contraste, parou em R$ 16.240: com muitas variáveis inteiras, o algoritmo genético com orçamento pequeno pode perder o ótimo, e vale conferir com a grade num domínio menor.

**Por que o S1 não tem mais médicos que o S2, se a manhã é mais cheia?** A manhã tem 26 pacientes por hora por três horas, mas o S1 só começa com a clínica vazia. O S2 já pega a **fila que sobrou do S1** (que acaba às 12h) e depois o pico da tarde (22 por hora). O S3 recebe o resto do pico da tarde e esvazia a fila até as 21h. Parte do efeito vem também do congelamento descrito acima. Por isso há dois planos equivalentes: 5/5/3 e 6/4/3 (um médico a mais cedo, outro a menos depois).

A triagem pede 3 enfermeiros de manhã (26 por hora × 5,67 min = 2,5 enfermeiros em pico), 2 à tarde e 2 no S3 para que a espera na sala de espera, que enche com as chegadas, fique abaixo de 8 min. A sala de espera tem em média 1 lugar ocupado.

## Pergunta 2: quanto a prioridade ajuda?

Escala ótima, 20 replicações:

| | Urgentes | Normais | Todos juntos (média ponderada 12% e 88%) |
|---|---|---|---|
| Com prioridade | **1,65 ± 0,04** min | 13,72 ± 0,68 min | 12,27 min |
| Sem prioridade (fila única FIFO) | 12,28 ± 0,59 min | 12,28 ± 0,59 min | 12,28 min |

A prioridade **reduz a espera dos urgentes de 12,3 para 1,65 min (−87%)** e custa apenas **1,4 min a mais** para os normais (de 12,3 para 13,7). A espera média de todos os pacientes não muda (12,27 contra 12,28): quando o tempo de consulta não depende do tipo e a prioridade não interrompe a consulta, ela só **redistribui** a mesma espera entre os dois grupos (lei de conservação das filas). Sem prioridade, os urgentes passam do limite de 3 min em mais de 9 minutos.

## Pergunta 4: demanda 15% maior

Escala ótima (5/5/3 e 3/2/2) com 15% mais pacientes (20 replicações):

| Meta | Limite | Valor | Cumpre |
|---|---|---|---|
| Sala de espera | 8 min | 12,67 ± 0,44 | **não** |
| Urgentes | 3 min | 2,13 ± 0,05 | sim |
| Normais | 15 min | **36,5 ± 2,2** | **não** |
| Perdas | 2% | saída 7.564 ± 36 de ~7.590 (0,3%); sala de espera chega a 29,9 de 30 | sim |

A escala de menor custo para a demanda maior (`otimizacao_demanda_15.ron`, 384 pontos):

| | Médicos S1/S2/S3 | Enfermeiros S1/S2/S3 | Custo por dia |
|---|---|---|---|
| Ótima da demanda atual | 5 / 5 / 3 | 3 / 2 / 2 | R$ 15.950 |
| **Ótima com +15%** | **6 / 5 / 4** | **3 / 3 / 2** | **R$ 18.280** |

Diferença: **+R$ 2.330 por dia (+14,6%)** sobre a escala ótima atual, e +R$ 350 sobre o que a clínica paga hoje. Com a demanda maior: sala de espera 3,9 min; urgentes 1,6 min; normais 14,0 min (limite 15); saída 7.574 pacientes. O enfermeiro a mais no S2 é o que protege a sala de espera, e o médico a mais no S1 e no S3 absorve a manhã e o fim do dia.

A conclusão para a diretoria: a escala ótima atual não tem margem; um crescimento de 15% a quebra, e o custo de reforçar é pequeno em relação ao que já se paga.

## Funcionalidades do UTSim que o caso exercita

| Funcionalidade | Onde |
|---|---|
| fontes com `shifts` para representar chegadas por faixa de horário | `chegada_B1..B6` |
| rótulos sorteados na fonte (`labels`, `Discrete`) | `classe` |
| roteamento por rótulo (`ByLabel`) | triagem para as duas filas |
| prioridade pela ordem das conexões de entrada | médicos |
| perda por sala cheia (`on_blocked: Drop`) | seis fontes |
| equipes por turno como `Processor` com `capacity` variável e `shifts` | triagem e consulta |
| ajuste de distribuição a partir de dados, incluindo uma mistura | `utsim fit` |
| otimização com custo por variável e restrições sobre médias | `otimizacao.ron` |
| experimento com `set` em várias chaves e troca de regra de saída | `experimento.ron` |
| conferência por lei de Little e por conservação das filas | validação |

## Erros comuns esperados

- Usar uma única fonte com a taxa média do dia (18,3 por hora): a manhã e a tarde ficam sem fila e a escala ótima sai errada.
- Ligar as filas dos médicos na ordem inversa, o que dá prioridade aos normais.
- Esquecer de `on_blocked: Drop` nas fontes, e então ter fonte bloqueada em vez de paciente perdido.
- Usar `time_in_system` como medida de espera (inflado pelo congelamento).
- Aplicar a meta de espera dos normais ao conjunto das filas.
- Esquecer de ligar a fonte da faixa 17h às 19h, o que esvazia o fim da tarde.

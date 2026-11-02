# Caso 2: Distribuidora Rio Claro, centro de distribuição com corredor estreito

Modelo: `trabalho/02-centro-de-distribuicao/modelo_caso02.ron`, montado e salvo pelo editor (roteiros
`roteiros/r02` a `r07`). Rodadas: 44 h (158.400 s), aquecimento de 4 h (14.400 s), janela de 40 h,
30 replicações, semente 1, mesmas sementes em todos os cenários. Valores: média ± meia-amplitude do
IC de 95%.

Experimentos: Q1 pela aba Cenários do editor (`cap/r08/modelo_caso02_experimento.ron`); Q2 e Q3 por
arquivos de experimento escritos à mão e rodados com `utsim experiment` (`exp_q2.ron`, `exp_q3.ron`),
porque com 5 a 6 cenários a tabela do editor não cabe na janela (ver relatório).

## 1. Modelo e distribuições

| Elemento | Objeto | Parâmetros | Justificativa |
|---|---|---|---|
| Caminhões | Chegada | Exp(média 1.732,4 s) | `utsim fit chegadas_caminhoes.csv --timestamps --coluna 2`: Exponencial escolhida (KS p 0,272; AD p 0,100; χ² p 0,179); Weibull e Gama equivalentes (ΔAIC < 2), parcimônia. Chegadas independentes: Exp faz sentido físico |
| Pátio | Fila sem limite | | |
| Docas | Processo, capacidade 2 | Uniform(901, 1499) | `utsim fit tempos_doca.csv --coluna 2`: Uniforme (KS p 0,987), peso de Akaike 98% |
| Saída da doca | Desmontagem "Dividir em cópias", 10 itens | Uniform(5, 15) | um caminhão vira 10 paletes |
| Esteira de rolos | Esteira, 30 m, 0,4 m/s, 1,2 m por item, com acúmulo | | trânsito 75 s |
| Recepção | Fila sem limite, transporte de saída "frota" | | |
| Porta-paletes | Porta-paletes 24 x 4, permanência mínima Uniform(3.600, 10.800), primeira livre, transporte de saída "frota" | | |
| Fila de picking, quebra, fila de pedidos | Fila; Desmontagem "Dividir em cópias" 6 com Uniform(15, 25); Fila | | um palete vira 6 pedidos |
| Faces de picking | Processo, capacidade 6, operador da "equipe_pickers" no processamento, transporte de saída "equipe_pickers" | Gama(forma 33,961; escala 1,596), média 54,2 s | `utsim fit tempos_picking.csv`: escolhida a Weibull deslocada; usei a Gama (aceita, ΔAIC 4,9) porque não achei a Weibull deslocada no menu (só depois, no caso 3, vi que a lista rola e tem "Deslocada" no fim) e o roteiro não digita maiúsculas no modo texto |
| Fila da embalagem | Fila sem limite | | |
| Embalagem | Processo, capacidade 5, Triangular(50, 70, 110), transferência pela distância a 1,0 m/s (reta) até a expedição | | 16 m hoje, 56 m na proposta |
| Empilhadeiras 1 a 4 | Transportador AGV (para respeitar a pista única), 2,0 m/s, acel./desacel. 0,6, carga U(8, 14), descarga U(10, 20), voltam à base, base no nó da garagem | | |
| Frota | Despachante, regra "Mais próximo", equipe de 3 (ou 2, ou 4) | | |
| Pickers 1 a 10 | Transportador, 1,3 m/s, capacidade 3, carga e descarga U(3, 6), voltam à base, base no nó do picking | | |
| Equipe de pickers | Despachante, "Mais próximo", equipe de 5 a 10 | | |
| Rede | nós (36,0), (60,0), (100,0), (60,10), (60,−22), (60,−82); aresta (60,0) a (100,0) limitada a 1 veículo | | |

Aproximações: a fila de picking, a quebra, a fila de pedidos e as faces ficaram em y = −10, −14, −18 e
−22 (e não −18, −20, −21 e −22), porque com 1 m de distância os blocos se sobrepõem no editor e não dá
para clicar em cada um. A empilhadeira entra na rede pelo nó mais próximo, então anda cerca de 10 m a
mais na entrega ao picking (uns 5 s). O picker que separa não é necessariamente o que leva o carrinho
(o motor pede um transporte à equipe depois da separação; em geral vai o mesmo, que é o mais próximo).

## 2. Validação

| Conferência | Esperado | Simulado | OK? |
|---|---|---|---|
| Caminhões em 40 h | 144.000 / 1.732,4 = 83,1 | 87,1 ± 3,2 (30 rep.); 83,17 ± 1,03 com 300 rep. | sim (com 30 rep. a diferença é acaso) |
| Paletes por caminhão | 10 | 870,3 / 87,03 = 10,0 | sim |
| Pedidos por palete | 6 | 5.229,4 / 871,7 = 6,0 | sim |
| Tempo na doca | 1.200 s | 1.204,8 ± 7,0 | sim |
| Trânsito na esteira | 30 / 0,4 = 75 s | 75,0 | sim |
| Ocupação média do porta-paletes (Little) | 871,6 / 144.000 x 7.298,8 = 44,2 | 44,2 ± 1,7 | sim |
| Permanência no porta-paletes | 7.200 s + espera da retirada | 7.298,8 ± 27,3 | sim |

## 3. Pergunta 1: quantas empilhadeiras? (5 pickers)

| Empilhadeiras | Espera na recepção (s) | Δ pareado vs 3 (s) | Recepção, máximo | Porta-paletes, máximo | Esperando o corredor (por empilhadeira) | Utilização (sem ocioso e sem espera) |
|---|---|---|---|---|---|---|
| 2 | 375,2 ± 20,7 | +134,0 ± 15,3 | 21,3 | 91,4 | 4,1% e 4,8% | 60,4% e 48,0% |
| **3** | **241,2 ± 6,0** | referência | 18,2 | 91,2 | 8,4%, 9,3%, 9,4% | 49,1%, 34,4%, 26,6% (média 36,7%) |
| 4 | 207,2 ± 4,7 | −33,9 ± 1,9 | 17,3 | 91,0 | 12,5%, 13,4%, 13,1%, 12,8% | de 17,7% a 43,2% |

**Resposta: 3 empilhadeiras.** Com 2, a espera passa de 300 s. A quarta reduz a espera em só 34 s
(menos que os 60 s exigidos). O corredor estreito pesa: com 3 empilhadeiras, cada uma passa de 8% a
9% do tempo (cerca de 3,5 h em 40 h) parada na entrada esperando a pista; com 4, de 12% a 13%. A
quarta empilhadeira passa boa parte do ganho esperando o corredor.

Alerta: o porta-paletes chega a 91 de 96 posições no pico (média de 42 a 44). Está perto de encher.

## 4. Pergunta 2: quantos pickers, embalagem a 60 m? (3 empilhadeiras)

| Pickers | Espera na fila de pedidos (s) | Espera na fila da embalagem (s) | Utilização média dos pickers |
|---|---|---|---|
| 5 | 5.282 ± 1.131 (instável) | 0,2 | 92,5% |
| 6 | 2.696 ± 651 | 0,8 | 85,6% |
| 7 | 1.601 ± 397 | 1,1 | 78,8% |
| 8 | 891 ± 212 | 1,9 | 74,0% |
| 9 | 507 ± 109 | 3,5 | 64,9% |
| **10** | **288,8 ± 58,3** | **8,2 ± 0,1** | **58,6%** |

**Resposta: 10 pickers**, o máximo testado, e por pouco: a média (289 s) cumpre os 300 s, mas o IC vai
até 347 s. Utilização média dos pickers: 58,6% (1 − ocioso). Os pickers passam cerca de metade do tempo
andando os 60 m com o carrinho (com 5 pickers: 24,6% indo vazio e 24,5% indo carregado), e por isso
são o gargalo.

## 5. Pergunta 3: mover a embalagem para 20 m?

Cenários com `fila_embalagem.position = (60, −42)` e `embalagem.position = (60, −44)`:

| Pickers | Espera na fila de pedidos (s) | Espera na fila da embalagem (s) | Utilização média dos pickers |
|---|---|---|---|
| 4 | 1.189 ± 287 | 1,2 | 75,7% |
| 5 | 449 ± 91 | 3,4 | 65,1% |
| **6** | **191,5 ± 35,3** | **16,9 ± 1,3** | **56,1%** |
| 7 | 87,2 ± 11,1 | 110,3 ± 23,5 | |
| 8 | 52,5 ± 5,2 | 148,7 ± 29,0 | |

Com a mudança bastam **6 pickers** (contra 10 hoje): economia de **4 pickers x R$ 5.200 = R$ 20.800
por mês**. A obra (R$ 360.000) se paga em 360.000 / 20.800 = **17,3 meses**, dentro do horizonte de 24
meses (ganho líquido de cerca de R$ 139.000 em 24 meses).

Tempo de ciclo do pedido: o UTSim não mede o tempo entre dois pontos do meio do fluxo (a métrica da
saída conta desde a criação do caminhão, que inclui as horas no porta-paletes). Estimei somando as
partes: espera na fila de pedidos + permanência nas faces + caminhada carregado + descarga + espera na
fila da embalagem + embalagem + esteira de saída.

| Situação | Estimativa do ciclo |
|---|---|
| Hoje, 10 pickers | 289 + 87 + 46 + 5 + 8 + 77 + 16 ≈ 527 s |
| Proposta, 6 pickers | 192 + 82 + 15 + 5 + 17 + 77 + 56 ≈ 443 s |

O ciclo **melhora** cerca de 85 s, apesar da esteira de saída 40 m mais longa.

Gargalo novo: a **fila da embalagem**. Com a caminhada curta, os pickers entregam em rajadas e a
fila da embalagem passa de segundos para 110 a 150 s com 7 ou 8 pickers (pico de 55 a 72 caixas),
embora as mesas fiquem ocupadas só 56% do tempo em média. Com 6 pickers ainda está em 17 s, mas é o
ponto a vigiar se a demanda crescer.

**Resposta: vale a pena mover a embalagem** (paga-se em cerca de 17 meses), com 3 empilhadeiras e 6
pickers.

## 6. Dificuldades

- O `utsim fit` sem `--coluna 2` ajusta a coluna "caminhao" (1, 2, 3...) e devolve
  `Discrete([(1.0,119.0)])`, sem aviso claro.
- Não há métrica de tempo entre dois pontos do fluxo (ciclo do pedido); tive de somar partes.
- Blocos de 1 m de distância se sobrepõem; afastei-os.
- A lista de equipes e de transporte no editor rola e esconde o despachante no fim; o clique cai no
  rótulo da cena e desmarca o objeto.
- A utilização da empilhadeira "sem contar a espera do corredor" tem de ser calculada à mão a partir
  dos estados (1 − idle − blocked).
- Cenários com mais de 4 colunas não cabem na tabela do editor; usei a linha de comando.

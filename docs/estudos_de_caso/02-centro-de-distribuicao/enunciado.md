# Estudo de caso 2: centro de distribuição com corredor estreito

## A empresa e o problema

A Distribuidora Rio Claro tem um centro de distribuição (CD) de produtos de mercearia. Os caminhões chegam com paletes, que são guardados num porta-paletes. Quando um palete é chamado, ele é levado à área de picking, aberto em caixas, e as caixas viram pedidos que são embalados e despachados.

Dois problemas:

1. Hoje existem **3 empilhadeiras** e **5 pickers**. Os pedidos esperam mais de **uma hora** na fila de separação nos horários cheios. A diretoria quer saber quantas pessoas e máquinas são realmente necessárias.
2. A área de embalagem está longe do picking: os pickers andam 60 metros com cada carrinho. Uma proposta é **mudar a embalagem para perto do picking**, a 20 metros. Custa caro, e a diretoria quer saber se se paga.

Você deve construir um modelo de simulação do CD e responder às três perguntas da seção "Perguntas de decisão".

## Como o CD funciona

```text
 PÁTIO       DOCAS (2)        ESTEIRA 30 m          RECEPÇÃO             CRUZAMENTO          CORREDOR ESTREITO (40 m)       PORTA-PALETES
 caminhões -> descarga ------> de rolos ------------> (fila de ----------> de circulação ====================================> seletivo 24x4
              10 paletes      0,4 m/s                  paletes)                 |                                                  (96 posições)
                                                                                |  retirada: o palete volta pelo mesmo corredor
                                                                                v
                                                                         ÁREA DE PICKING (22 m do cruzamento)
                                                                         quebra do palete em 6 caixas -> fila de pedidos -> 6 faces de picking
                                                                                |
                                                                   pickers andam com carrinho (60 m hoje, 20 m na proposta)
                                                                                v
                                                                         EMBALAGEM (5 mesas) --> 16 m --> EXPEDIÇÃO
```

1. **Chegada dos caminhões.** Os caminhões esperam num pátio (sem limite de espaço) e vão para uma das **2 docas** assim que uma fica livre. Cada caminhão traz **10 paletes**. A doca fica ocupada durante toda a descarga. No fim da descarga, os 10 paletes saem juntos da doca e são colocados na esteira de rolos (5 a 15 s para colocar os 10).
2. **Esteira de rolos.** Tem 30 m, anda a 0,4 m/s e cada palete ocupa 1,2 m. Se o fim da esteira estiver ocupado, os paletes se acumulam na esteira. No fim dela está a **recepção**, uma fila de paletes (sem limite de espaço) onde esperam uma empilhadeira.
3. **Empilhadeiras.** Levam um palete por viagem e servem dois tipos de serviço: guardar (recepção até o porta-paletes) e retirar (porta-paletes até a área de picking). Os serviços são atendidos na ordem em que são pedidos; a empilhadeira livre mais próxima do local do serviço pega o serviço. Quando ficam ociosas, voltam à garagem. Elas só andam pelos corredores de circulação descritos abaixo, e o **corredor estreito do porta-paletes só comporta uma empilhadeira de cada vez, nos dois sentidos**. Quem chegar e encontrar o corredor ocupado espera na entrada, na ordem de chegada.
4. **Porta-paletes seletivo.** Tem 24 colunas por 4 níveis (96 posições). O palete é guardado na posição livre mais baixa (do nível mais baixo para o mais alto, da coluna mais baixa para a mais alta) e fica lá de **1 a 3 horas** (qualquer valor do intervalo). Depois disso, é chamado para o picking e uma empilhadeira o retira. A posição só é liberada quando o palete sai.
5. **Área de picking.** Quando o palete chega, uma pessoa o abre e o divide em **6 caixas** (cada caixa é um pedido). Isso leva de 15 a 25 s por palete (uniforme). As caixas esperam numa fila de pedidos (sem limite) até que haja uma **face de picking** livre (são **6 faces**, que podem trabalhar ao mesmo tempo) **e um picker disponível**.
6. **Pickers.** Cada picker pega um pedido, vai até a face, separa os itens (o picker precisa estar presente durante toda a separação) e depois leva a caixa até a embalagem num carrinho. O carrinho leva até **3 caixas por viagem**; se houver mais de uma caixa pronta esperando quando o picker chegar, ele leva até 3 de uma vez. Cada caixa leva de 3 a 6 s para ser carregada no carrinho e de 3 a 6 s para ser descarregada na embalagem. O picker anda a 1,3 m/s e, depois de entregar, volta à área de picking se não houver outro serviço esperando. Quando mais de um picker está livre, vale o mais próximo do serviço. A empresa pode ter de 4 a 10 pickers.
7. **Embalagem.** São **5 mesas** de embalagem em paralelo, com operadores próprios (não são os pickers). Cada caixa leva de 50 s a 110 s para embalar, sendo 70 s o valor mais comum. Depois de embalada, a caixa segue por esteira de rolos até a expedição, a **1,0 m/s** em linha reta (16 m hoje).

## Dados

Tempos em **segundos**, distâncias em **metros**.

### Chegada de caminhões e docas

| Item | Dado |
|---|---|
| Chegada de caminhões | registro de 120 chegadas em `dados/chegadas_caminhoes.csv` (instante de cada chegada, em segundos desde o início da observação) |
| Tempo de descarga na doca | 200 medições em `dados/tempos_doca.csv` |
| Paletes por caminhão | 10 |
| Docas | 2 |

### Empilhadeiras

| Item | Dado |
|---|---|
| Velocidade | 2,0 m/s |
| Aceleração e desaceleração | 0,6 m/s² cada |
| Carga do palete | 8 a 14 s (uniforme) |
| Descarga do palete | 10 a 20 s (uniforme; inclui a colocação em altura no porta-paletes) |
| Capacidade | 1 palete por viagem |
| Garagem | posição (60, 10) |
| Quantidade hoje | 3 (pode chegar a 4) |

### Rede de circulação das empilhadeiras

| Nó | x | y | Observação |
|---|---|---|---|
| Recepção | 36 | 0 | onde os paletes chegam |
| Cruzamento | 60 | 0 | encontro dos corredores |
| Garagem | 60 | 10 | estacionamento |
| Porta-paletes | 100 | 0 | fim do corredor estreito |
| Picking | 60 | -22 | área de picking |
| Embalagem | 60 | -82 | corredor a pé dos pickers (só tem pessoas a pé) |

| Trecho | Entre | Comprimento (m) | Regra |
|---|---|---|---|
| 1 | Recepção e cruzamento | 24 | duas mãos, sem limite |
| 2 | Cruzamento e garagem | 10 | duas mãos, sem limite |
| 3 | Cruzamento e porta-paletes | 40 | **corredor estreito: um veículo por vez, nos dois sentidos** |
| 4 | Cruzamento e picking | 22 | duas mãos, sem limite |
| 5 | Picking e embalagem | 60 | duas mãos, só pickers a pé |

Os pickers também seguem esses corredores quando andam (só usam o trecho 5 na prática). Cada ponto do CD que não é um nó (docas, filas, estações) entra na rede pelo nó mais próximo, andando em linha reta até ele.

### Picking e embalagem

| Item | Dado |
|---|---|
| Quebra do palete em 6 caixas | 15 a 25 s por palete (uniforme) |
| Tempo de separação de uma caixa | 300 medições em `dados/tempos_picking.csv`, em segundos |
| Faces de picking | 6 |
| Picker | 1,3 m/s, até 3 caixas por carrinho, 3 a 6 s para carregar e 3 a 6 s para descarregar cada caixa |
| Embalagem | 5 mesas, 50 a 110 s por caixa (mais comum 70 s) |
| Saída da embalagem até a expedição | 1,0 m/s em linha reta |

### Posições na planta (metros)

| Objeto | x | y |
|---|---|---|
| Caminhões (entrada) | -5 | 0 |
| Pátio | -3 | 0 |
| Docas | 0 | 0 |
| Saída da doca (divisão dos paletes) | 3 | 0 |
| Esteira de rolos (centro) | 18 | 0 |
| Recepção | 36 | 0 |
| Cruzamento (sem equipamento) | 60 | 0 |
| Garagem das empilhadeiras | 60 | 10 |
| Base dos pickers (onde esperam) | 60 | -22 |
| Porta-paletes | 100 | 0 |
| Fila de picking (chegada do palete) | 60 | -18 |
| Quebra do palete | 60 | -20 |
| Fila de pedidos | 60 | -21 |
| Faces de picking | 60 | -22 |
| Fila da embalagem (hoje) | 60 | -82 |
| Mesas de embalagem (hoje) | 60 | -84 |
| Expedição | 60 | -100 |

Na proposta de mudança, a fila de embalagem passa para (60, -42) e as mesas de embalagem para (60, -44). A expedição não muda de lugar, de modo que a esteira de saída da embalagem passa de 16 m para 56 m.

## Medidas que a diretoria usa

- **Espera do palete na recepção**: tempo médio que o palete fica na fila da recepção até a empilhadeira pegá-lo.
- **Espera do pedido na fila de pedidos**: tempo médio entre o pedido (caixa) nascer da quebra do palete e uma face de picking e um picker o começarem a atender.
- **Espera na fila da embalagem**: tempo médio na fila diante das mesas de embalagem.
- **Tempo de ciclo do pedido**: tempo entre o pedido entrar na fila de pedidos e chegar à expedição.
- **Utilização de uma empilhadeira**: fração do tempo em que ela está carregando, descarregando ou viajando (vazia ou carregada), sem contar o tempo parada ou esperando o corredor estreito.
- O tempo total do palete no CD (da chegada do caminhão à expedição) **não** é uma boa medida aqui, porque o palete fica de 1 a 3 horas guardado de propósito.

## Como analisar

- O CD opera continuamente. Simule **44 horas** e descarte as **4 primeiras** (o porta-paletes leva umas 2 horas para chegar ao regime). A janela de análise tem então 40 horas.
- Use pelo menos **30 replicações** e a **mesma semente** em todos os cenários que comparar.
- Dê sempre a média e o intervalo de confiança de 95%.
- Registre também a ocupação máxima do porta-paletes (para saber se ele chega perto de encher) e a ocupação máxima da recepção.

## Custos

| Item | Custo |
|---|---|
| Empilhadeira (aluguel mais operador) | R$ 14.500 por mês |
| Picker | R$ 5.200 por mês |
| Mudança da embalagem para perto do picking (obra única) | R$ 360.000 |
| Horizonte de decisão | 24 meses |

## Perguntas de decisão

**Pergunta 1. Quantas empilhadeiras?** Compare 2, 3 e 4 (com 5 pickers e a embalagem onde está). Critério de aceitação: espera média do palete na recepção de **até 5 min (300 s)**. A quarta empilhadeira só se justifica se reduzir essa espera em mais de 60 s em relação a 3. Descreva também o que acontece com o corredor estreito: quanto tempo cada empilhadeira passa esperando para entrar nele.

**Pergunta 2. Quantos pickers, com a embalagem onde está (60 m do picking)?** Teste de 5 a 10 pickers (3 empilhadeiras). Critérios de aceitação (os dois):

- espera média na fila de pedidos de **até 5 min (300 s)**;
- espera média na fila da embalagem de **até 5 min (300 s)**.

Diga o menor número de pickers que cumpre os critérios, e a utilização média dos pickers nesse caso (fração do tempo fora do estado ocioso).

**Pergunta 3. Vale a pena mover a embalagem?** Com a embalagem a 20 m do picking (e a 56 m da expedição), refaça a pergunta 2 testando de 4 a 8 pickers. Compare com a situação atual: quantos pickers a mudança economiza, e o tempo de ciclo do pedido melhora ou piora? Calcule em quantos meses a obra se paga pela economia de pickers e diga se ela se paga no horizonte de 24 meses. Se aparecer um gargalo novo depois da mudança, diga qual é.

## O que entregar

1. O modelo, com a descrição das distribuições escolhidas (mostre a saída do ajuste) e uma justificativa para cada.
2. Uma tabela de validação: confira, por exemplo, o número de caminhões e paletes por hora, a ocupação média do porta-paletes (pela lei de Little: taxa de paletes por hora vezes tempo guardado em horas) e o número de pedidos (6 por palete).
3. Os resultados de cada pergunta, com intervalos de confiança.
4. Uma lista curta de **dificuldades que você encontrou** para montar o modelo (o que foi fácil, o que faltou, o que você teve de aproximar).

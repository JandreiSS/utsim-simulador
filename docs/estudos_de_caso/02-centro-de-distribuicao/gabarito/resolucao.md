# Estudo de caso 2: resolução (gabarito)

Este arquivo é só para quem corrige. Quem resolve o caso recebe apenas o `enunciado.md`, os arquivos de `dados/` e o manual.

| Arquivo | Para que serve |
|---|---|
| `modelo.ron` | modelo de referência (30 objetos, 13 conexões, rede com 6 nós) |
| `experimento.ron` | as três perguntas (`utsim experiment`) |
| `gerar_dados.py` | gera os CSVs de `dados/` com semente fixa |

Este caso não tem arquivo de otimização, de propósito: a quantidade de empilhadeiras e de pickers é a **lista de nomes** de uma equipe, e variável de escolha (`Choice`) não aceita custo (lacuna 6 do [README](../../README.md)). A decisão de custo é feita à mão, com a tabela do experimento.

```bash
utsim check docs/estudos_de_caso/02-centro-de-distribuicao/gabarito/modelo.ron
utsim run docs/estudos_de_caso/02-centro-de-distribuicao/gabarito/modelo.ron
utsim experiment docs/estudos_de_caso/02-centro-de-distribuicao/gabarito/experimento.ron
```

## De onde vieram os dados

| Arquivo | n | Distribuição verdadeira | O que o `utsim fit` recomenda |
|---|---|---|---|
| `chegadas_caminhoes.csv` | 120 instantes (119 intervalos) | tempo entre chegadas Exponencial(média 1.800 s) | `Exp(mean: 1732.4)` (use `--coluna 2 --timestamps`) |
| `tempos_doca.csv` | 200 | Uniforme(900, 1.500) | `Uniform(min: 901.0, max: 1499.0)` |
| `tempos_picking.csv` | 300 | Triangular(35, 50, 80), média 55 s | `Shifted(offset: 33.062, dist: Weibull(shape: 2.437, scale: 23.854))`, média 54,2 s (a Triangular verdadeira também passa nos testes, com ΔAIC = 5,2) |

O modelo de referência usa os valores **ajustados**. Como a média de intervalos ajustada (1.732 s) é 4% menor que a verdadeira (1.800 s), chegam um pouco mais caminhões que o "esperado" (80,2 em 40 h, contra 80 pela média verdadeira de 1.800 s: 144.000 / 1.800 = 80). Um aluno que use `Exp(1800)` terá resultados parecidos.

## Como o modelo de referência traduz o enunciado

| Enunciado | Modelo |
|---|---|
| caminhões, pátio e 2 docas | `Source` + `Queue` + `Processor(capacity: 2)` com a descarga ajustada |
| 10 paletes saem juntos no fim da descarga | `Separator(mode: Split(10))` com 5 a 15 s |
| esteira de 30 m a 0,4 m/s, palete de 1,2 m | `Conveyor(length: 30, speed: 0.4, item_length: 1.2, accumulating: true)`. Trânsito de 75 s, capacidade de 25 paletes, vazão máxima de 0,33 palete/s |
| recepção | `Queue` com `transport: "frota"` (a empilhadeira leva dali até o porta-paletes) |
| porta-paletes seletivo | `Rack(bays: 24, levels: 4, dwell: Uniform(3600, 10800), placement: FirstFree)` com `transport: "frota"` (a empilhadeira leva dali até a fila de picking) |
| frota de empilhadeiras no corredor estreito | `Transporter(agv: true)` (segue a rede aresta por aresta) + `Dispatcher(rule: Nearest)`. A aresta cruzamento-rack tem `two_way: true, capacity: Some(1)`: uma pista única |
| quebra do palete em 6 caixas | `Separator(mode: Split(6))` com 15 a 25 s |
| 6 faces de picking com picker presente | `Processor(capacity: 6)` com `operators: OperatorUse(team: "pickers", count: 1, setup: false, process: true)` |
| picker leva até 3 caixas num carrinho | o **mesmo** `Dispatcher` `pickers` é a equipe de operadores e o `transport` da separação. Os `Transporter` dos pickers têm `capacity: 3`, 3 a 6 s para carregar e descarregar, `home: true` |
| mesas de embalagem | `Processor(capacity: 5)` |
| esteira de saída da embalagem | `transfer: (speed: 1.0)` na embalagem: o tempo de viagem depende da distância até a expedição (16 s hoje, 56 s com a embalagem perto) |
| mudar a embalagem para perto | experimento muda `fila_embalagem.position` e `embalagem.position` |

Notas de modelagem:

- Os pickers andam pela mesma rede. A base deles está exatamente em cima do nó `picking`, e há um nó `embalagem` a 60 m. Por isso a viagem à embalagem tem exatamente 60 m (46 s a 1,3 m/s). Na proposta, a embalagem vai para (60, -44), cujo nó mais próximo é `picking`: o picker anda 22 m (e a esteira de saída fica em 56 m).
- A **rede não pode ser alterada num experimento** (lacuna 7 do README). O nó `embalagem` continua na rede depois da mudança de posição, sem atrapalhar.
- Os pontos `caminhoes` e `patio` ficam a 5 m e 3 m da doca, para a ligação direta instantânea não gerar aviso de distância.
- Janela: `stop_time: 158400` (44 h), `warmup: 14400` (4 h), 30 replicações. O primeiro período enche o porta-paletes.
- O aluno pode decidir representar a doca como o conjunto "fila + processo" de outras formas (por exemplo, descarregar paletes um a um); o resultado muda pouco, porque a espera relevante está depois.

## Validação do modelo base (3 empilhadeiras, 5 pickers, embalagem longe)

| Medida | Valor (média ± IC de 95%, 30 replicações) |
|---|---|
| Caminhões na janela de 40 h | 80,2 ± 2,9 (2,0 por hora) |
| Paletes por hora | 20,1; cada palete gera 6 pedidos: 4.877 ± 202 pedidos |
| Porta-paletes: conteúdo médio | 41,0 ± 1,7 paletes (ocupação média de 43% das 96 posições) |
| Lei de Little no porta-paletes | λ = 20,1 paletes/h; W = 7.298 s = 2,03 h; L = 20,1 × 2,03 = 40,7 (confere com 41,0) |
| Porta-paletes: ocupação máxima | 90,0 ± 2,6 de 96 posições (chega perto de encher em picos) |
| Pedidos entrando na embalagem | 4.713 ± 169 (os 164 restantes ainda estavam na fila de pedidos no fim da janela) |
| Embalagem: mesas ocupadas em média | 2,51 ± 0,09 de 5 (50%) |

## Pergunta 1: quantas empilhadeiras?

O experimento usa as 30 replicações do modelo, com a mesma semente em todos os cenários.

| Cenário | Espera na recepção (s) | Máximo na recepção | Empilhadeira 1 esperando o corredor |
|---|---|---|---|
| 2 empilhadeiras | **401 ± 33** | 22,5 ± 2,2 | 3,5% |
| **3 empilhadeiras** | **243,6 ± 9,4** | 18,7 ± 1,1 | 7,0% |
| 4 empilhadeiras | 209,5 ± 8,8 | 17,7 ± 1,2 | 11,0% |

Diferença pareada de 4 contra 3: **−34,1 ± 1,3 s** (menor que os 60 s exigidos). A de 3 contra 2 é de aproximadamente −158 s.

Utilização de cada empilhadeira com 3 (carregando, descarregando e viajando): 48,9%, 35,3% e 27,4% (média de 37%); somando as três, o trabalho vale 1,12 empilhadeira.

**Resposta: 3 empilhadeiras.** Com 2, a espera passa de 5 minutos (401 s); com 3, cumpre (244 s); a quarta tira só 34 s e não passa do critério de 60 s.

Interpretação: o gargalo **não é o número de empilhadeiras**, porque a utilização é baixa (37% em média, mesmo com 2 é de uns 55%). O que faz o palete esperar é a **chegada em rajada** (10 paletes saem juntos da doca) e o **corredor estreito de pista única**: cada passagem de 40 m ocupa a pista por uns 23 s (20 s em velocidade de cruzeiro mais a rampa de aceleração e frenagem), e com 4 empilhadeiras cada uma passa 11% do tempo esperando para entrar. Um palete da rajada precisa esperar que os anteriores sejam levados. Mais empilhadeiras só aumentam a fila na entrada do corredor.

## Pergunta 2: quantos pickers com a embalagem onde está?

| Pickers | Espera na fila de pedidos (s) | Espera na embalagem (s) | Cumpre? |
|---|---|---|---|
| 5 (hoje) | 4.070 ± 1.011 (68 min) | 0,2 | não |
| 6 | 2.087 ± 521 | 1,0 | não |
| 7 | 1.274 ± 338 | 1,4 | não |
| 8 | 756 ± 201 | 2,2 | não |
| 9 | 456 ± 112 | 3,7 | não |
| **10** | **258 ± 56** | **8,2** | **sim** (no limite: o IC chega a 314 s) |

Com 10 pickers a utilização média de um picker (tempo fora do ocioso) é de uns **64%**. O ciclo do pedido (da fila de pedidos à expedição) é, em média, 258 + 85 (separação, incluindo a espera pelo carrinho) + 8 (fila da embalagem) + 77 (embalagem) + 16 (esteira de saída) = **444 s**.

**Resposta: 10 pickers** (o máximo disponível). Aceite 9 ou 10 se o aluno mostrar o IC e discutir a margem. Aceite também a resposta "mesmo com 10 mal cumpre", que é o ponto principal: com a embalagem longe, a frota de pickers precisa dobrar.

Por que tantos? Cada pedido ocupa o picker em dois momentos: 54 s de separação mais uma viagem de 60 m de ida e 60 m de volta (92 s de caminhada mais 6 a 12 s de carga e descarga), ou seja, **uns 155 s de picker por pedido, dos quais 60% são caminhada**. Com 118 pedidos/h, isso é 5,1 pickers em tempo puro, e a chegada em rajada (6 caixas por palete) pede muita folga.

## Pergunta 3: mover a embalagem

Embalagem a 20 m do picking (e a 56 m da expedição), 3 empilhadeiras:

| Pickers | Espera na fila de pedidos (s) | Espera na embalagem (s) | Cumpre? |
|---|---|---|---|
| 4 | 987 ± 262 | 1,3 | não |
| 5 | 409 ± 94 | 3,4 | não |
| **6** | **169 ± 31** | **17,4** | **sim** |
| 7 | 78 ± 11 | 100 ± 21 | sim |
| 8 | 47 ± 5 | 134 ± 26 | sim |

Com 6 pickers a utilização de um picker é de uns **53%**. O ciclo do pedido é 169 + 80 + 17 + 77 + 56 = **399 s**, contra 444 s da situação "longe com 10 pickers": a mudança **melhora também o tempo de ciclo em 10%**, embora a esteira de saída fique 40 s mais longa.

**Gargalo novo:** com 7 ou 8 pickers, a fila da embalagem sobe para 100 a 134 s. As 5 mesas passam a ser o limite, porque os pickers entregam as caixas em rajadas, não mais espaçadas pela caminhada longa. A embalagem sozinha ainda tem folga (utilização de 50%), mas rajadas de 6 caixas enchem a fila. Aceite como correta qualquer menção a "a embalagem vira o próximo gargalo se houver mais de 6 pickers".

### O retorno do investimento

| | Embalagem longe | Embalagem perto |
|---|---|---|
| Pickers que cumprem a meta | 10 | 6 |
| Custo mensal dos pickers | R$ 52.000 | R$ 31.200 |
| Economia mensal | | **R$ 20.800** |
| Obra | | R$ 360.000 |
| Retorno (payback) | | 360.000 / 20.800 = **17,3 meses** |
| Saldo em 24 meses | | 20.800 × 24 − 360.000 = **R$ 139.200** a favor |

**Resposta: vale a pena**, com retorno em cerca de 17 meses, dentro do horizonte de 24. A conclusão é sensível: se o aluno achar que 9 pickers bastam na situação atual (a diferença entre 9 e 10 está dentro do ruído), a economia cai para 3 pickers (R$ 15.600 por mês) e o retorno sobe para 23 meses, no limite do horizonte. O bom analista diz isso.

## Funcionalidades do UTSim que o caso exercita

| Funcionalidade | Onde |
|---|---|
| `Separator` com `Split(n)` | caminhão em 10 paletes, palete em 6 caixas |
| esteira com acúmulo | recebimento |
| porta-paletes com `dwell` e `FirstFree` | armazenagem; lei de Little |
| empilhadeiras como AGV, rede de caminhos e aresta de pista única (`capacity: Some(1)`) | corredor estreito |
| `Dispatcher` com `Nearest` e `transport` em dois objetos | frota |
| o mesmo `Dispatcher` como equipe de operadores e como transporte | pickers |
| carrinho com `capacity: 3` | viagens em lote |
| `transfer` por distância | esteira de saída da embalagem |
| ajuste com `--timestamps` e coluna extra (`--coluna 2`) | chegadas, docas, picking |
| experimentos que trocam listas de equipe e posições | `experimento.ron` |

## Erros comuns esperados

- Usar `Exp(mean: 1800)` para a descarga de caminhões ou esquecer a doca como recurso (o pátio nunca forma fila).
- Esquecer de limitar o corredor estreito (`capacity: 1`) ou de marcar as empilhadeiras como `agv`. Sem isso a pista deixa de ser única e os veículos passam um por cima do outro no corredor, o que subestima a espera.
- Não ligar o `transport` do porta-paletes para a retirada: o palete nunca sai e o porta-paletes enche.
- Medir o "tempo no sistema" do palete pela saída (`expedicao.time_in_system`), que mistura o tempo de armazenagem. O enunciado avisa contra isso.
- Mover a embalagem sem lembrar que a esteira de saída fica mais longa.
- Esquecer o `warmup` (o porta-paletes vazio no começo distorce a recepção).

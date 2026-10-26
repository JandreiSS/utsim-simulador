# Glossário

Termos do UTSim em português, com os equivalentes em inglês usados na literatura de simulação.

## Objetos

| UTSim | Nome no arquivo | Termo em inglês | Observação |
|---|---|---|---|
| Chegada | `Source` | Source | cria itens |
| Fila | `Queue` | Queue | buffer; pode formar lotes |
| Processo | `Processor` | Processor | máquina ou posto; `capacity` > 1 faz máquinas em paralelo |
| Montagem | `Combiner` | Combiner | junta contêiner e componentes |
| Desmontagem | `Separator` | Separator | desembala ou divide |
| Saída | `Sink` | Sink | destrói o item |
| Esteira | `Conveyor` | Conveyor | cinemática, com ou sem acúmulo |
| Porta-paletes | `Rack` | Rack | armazenagem endereçada |
| Transportador | `Transporter` | Transporter / operator / AGV | pessoa ou veículo |
| Despachante | `Dispatcher` | Dispatcher | reparte tarefas a uma equipe |
| Rede de caminhos | `network` | path network (A*) | corredores dos veículos |
| Fluxo de tokens | `flows` | token flow | lógica do usuário em grafo |
| Fluidos | `fluids` | fluids | tanques, tubos, conversores |

## Conceitos

| Português | Inglês | Significado |
|---|---|---|
| item | item, flow item | o que circula: peça, cliente, palete |
| rótulo | label | valor guardado no item ou no token |
| contêiner | container / tote | item que carrega outros |
| regra de saída | output port rule | como escolher a porta de saída |
| prioridade de puxar | pull / input priority | ordem em que as entradas são atendidas |
| bloqueio | blocking | objeto parado porque o destino está cheio |
| setup | setup time | preparação antes de processar |
| tempo de processo | process time | duração do trabalho |
| quebra | breakdown, MTBF / MTTR | falha aleatória |
| MTBF | mean time between failures | tempo médio entre falhas |
| MTTR | mean time to repair | tempo médio de reparo |
| turno | time table, shift | janelas de trabalho e parada |
| operador | operator | pessoa que opera uma máquina |
| tarefa de transporte | transport task, task sequence | pedido de uma viagem |
| ocioso | idle | sem serviço |
| gargalo | bottleneck | etapa que limita a vazão |
| vazão | throughput | itens por unidade de tempo |
| utilização | utilization | fração do tempo ocupado |
| conteúdo | content | itens dentro do objeto |
| permanência | staytime / residence time | tempo que o item fica no objeto |
| tempo no sistema | time in system / cycle time | da criação à saída |
| fila (tamanho) | queue length | itens esperando |
| replicação | replication | rodada independente |
| aquecimento | warm-up | período descartado no início |
| semente | seed | fixa os sorteios |
| números aleatórios comuns | common random numbers | mesmas sementes entre cenários |
| intervalo de confiança | confidence interval | faixa provável da média verdadeira |
| meia-amplitude | half-width | metade do intervalo de confiança |
| lei de Little | Little's law | L = λ x W |
| cenário | scenario | variação do modelo base |
| experimento | experiment | comparação de cenários |
| otimização | optimization | busca da melhor combinação de parâmetros |
| restrição | constraint | limite sobre uma métrica |
| ajuste de distribuição | input modelling | escolher a distribuição a partir de dados |
| aderência | goodness of fit | quão bem a distribuição descreve os dados |
| parcimônia | parsimony | preferir o modelo mais simples entre equivalentes |
| emulação | emulation | modelo ligado a um CLP em tempo real |
| aprendizado por reforço | reinforcement learning | agente que aprende por tentativa e erro |
| ativo 3D | 3D shape / asset | modelo `.glb` do objeto |
| replay | replay | rodada gravada para assistir |

## Estatística

| Termo | Significado |
|---|---|
| AIC | critério de Akaike: ajuste com desconto pelo número de parâmetros. Menor é melhor |
| ΔAIC | diferença para o menor AIC. Menos de 2 significa equivalente |
| peso de Akaike | probabilidade relativa de uma família ser a melhor |
| KS | teste de Kolmogorov-Smirnov |
| AD | teste de Anderson-Darling |
| χ² | teste qui-quadrado |
| p-valor | probabilidade de ver um desvio tão grande por acaso, se a distribuição fosse a certa. Pequeno (menos de α) leva a rejeitar |
| bootstrap paramétrico | simular amostras da distribuição ajustada para calcular o p-valor |
| i.i.d. | independentes e identicamente distribuídos |
| CV | coeficiente de variação: desvio padrão dividido pela média |
| M/M/1 | fila com chegadas e atendimentos exponenciais e um servidor |
| ρ | utilização, ρ = λ / μ |
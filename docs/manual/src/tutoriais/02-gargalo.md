# Tutorial 02: linha de produção com gargalo

**Objetivo:** achar o gargalo de uma linha pela **utilização**, e medir o ganho de reforçá-lo.

**Arquivos:** `examples/tutorial_02_linha.ron` e `examples/experimento_tutorial_02.ron`

## O problema

Uma peça chega a cada 4 minutos, em média, e passa por três máquinas em série, cada uma com a sua fila:

| Máquina | Tempo médio | Utilização esperada |
|---|---|---|
| `corte` | 3,0 min | 3,0 / 4 = 0,75 |
| `furacao` | 3,6 min | 3,6 / 4 = **0,90** |
| `pintura` | 2,5 min | 2,5 / 4 = 0,625 |

A utilização esperada é o tempo de atendimento dividido pelo intervalo entre chegadas. A mais alta indica o **gargalo**: a etapa que limita a linha.

```ron
(name: "furacao", kind: Processor(capacity: 1, process: Exp(mean: 3.6)))
```

## Rodar e achar o gargalo

```bash
utsim check examples/tutorial_02_linha.ron
utsim run examples/tutorial_02_linha.ron
```

Trecho do resultado (conteúdo médio e permanência média):

```text
objeto         tipo       conteúdo médio      permanência média
fila_corte     queue        2.2125 ± 0.1281        8.8330 ± 0.4356
corte          processor    0.7457 ± 0.0097        2.9786 ± 0.0247
fila_furacao   queue        9.0402 ± 1.7625       36.0358 ± 6.8532
furacao        processor    0.9040 ± 0.0141        3.6114 ± 0.0331
fila_pintura   queue        1.0694 ± 0.0532        4.2701 ± 0.1845
pintura        processor    0.6272 ± 0.0093        2.5058 ± 0.0191

saida          time_in_system 58.2360 ± 7.1710
```

Como achar o gargalo:

1. **Utilização.** O conteúdo médio de cada `Processor` é a sua utilização: corte 0,75, **furação 0,90**, pintura 0,63. A furação é a mais carregada.
2. **Fila.** A fila antes da furação tem em média 9 peças e as peças esperam 36 minutos. As outras filas têm 1 a 2 peças. O gargalo é onde a fila se acumula.
3. **Intervalo.** O intervalo de confiança da fila da furação é enorme (±1,76), sinal de que o sistema está perto do limite.

A peça fica em média 58 minutos na linha, mas só 9,1 minutos são de trabalho útil (3,0 + 3,6 + 2,5). Os outros 49 minutos são espera, e a maior parte está na fila da furação.

## Reforçar o gargalo

Edite `furacao` e troque `capacity: 1` por `capacity: 2`, ou use o experimento pronto:

```bash
utsim experiment examples/experimento_tutorial_02.ron
```

```text
medida                         base       2 furadeiras         demanda alta   demanda alta, 2 furadeiras
corte.content.mean          0.7457             0.7457               0.8771                       0.8771
furacao.content.mean        0.9040             0.9043               1.0000                       1.0613
furacao.state.processing    0.9040             0.6240               1.0000                       0.6945
pintura.content.mean        0.6272             0.6275               0.6955                       0.7381
fila_furacao.content.mean   9.0402             0.2280             473.7506                       0.4150
saida.time_in_system.mean   58.236             22.980             1646.991                       39.389
saida.input              12013.9            12016.3              13298.1                      14122.6
```

(O programa mostra também os intervalos de confiança; aqui estão só as médias.)

### Primeira comparação: base contra 2 furadeiras

- A fila da furação cai de **9,04** para **0,23** peças.
- O tempo no sistema cai de **58,2** para **23,0** minutos (queda de 60%).
- A vazão (`saida.input`) **não muda**: 12.014 contra 12.016 peças. Faz sentido: a linha já dava conta da demanda, só que com muita espera. Reforçar o gargalo reduz a espera, não aumenta a produção.

Duas leituras importantes:

- `furacao.content.mean` continua em 0,90, porque é o número médio de furadeiras ocupadas, e o trabalho a fazer é o mesmo. A utilização **por furadeira** é 0,90 / 2 = 0,45.
- `furacao.state.processing` cai de 0,90 para 0,62. Esse estado é a fração do tempo com **pelo menos uma** furadeira ocupada, por isso não é a utilização. É a armadilha que o capítulo [Resultados](../resultados.md) descreve.

### Segunda comparação: demanda 18% maior

Com uma peça a cada **3,4 minutos**, a furação precisaria de 3,6 / 3,4 = 1,06 de capacidade. Passa de 100%:

- **Uma furadeira não dá conta.** A utilização trava em 1,00 e a fila cresce sem parar: média de **474 peças** na fila e **1.647 minutos** no sistema. Isso depende do tempo simulado, pois o sistema é instável.
- A vazão fica em 13.298 peças, **limitada pela furação**.
- **Com duas furadeiras** a fila volta para 0,4 peça, o tempo no sistema para 39 minutos e a vazão sobe para **14.123 peças**, 6,2% a mais.

Aqui o reforço **aumenta a produção**. Esta é a diferença entre um gargalo "que atrasa" e um gargalo "que limita".

Repare também que, com a demanda alta, o `corte` chega a 0,88 de utilização e passa a ser o próximo candidato a gargalo. Resolver um gargalo revela o seguinte.

## Exercícios

1. Em vez de `capacity: 2`, mude o tempo da furação para `Exp(mean: 2.7)` (máquina 25% mais rápida). O tempo no sistema cai para uns 26,9 minutos, contra 23,0 com a segunda furadeira. Por que a segunda furadeira ganha?
2. Ache a **demanda máxima** que a linha com duas furadeiras suporta, aumentando a taxa de chegada (`Exp(mean: 3.0)`, `Exp(mean: 2.8)`...). O corte, com 3,0 min por peça, trava em 1 peça a cada 3 minutos.
3. Limite o buffer antes da furação a 5 peças (`Queue(capacity: 5)`). O `corte` passa a ficar em `blocked` cerca de 18% do tempo, esperando espaço. A espera não some: ela se desloca para a fila antes do corte, que passa a ter uns 20 itens, e o tempo no sistema sobe para uns 105 minutos.

## O que você aprendeu

- O gargalo é a etapa de maior utilização e de maior fila.
- Reforçar o gargalo reduz a espera. Só aumenta a vazão se a demanda é maior que a capacidade dele.
- Para um processo com várias máquinas, a utilização por máquina é o conteúdo médio dividido pela capacidade.
- Um sistema com utilização igual a 1 é instável, e os números dele não convergem.

Próximo: [Tutorial 03, operadores que andam](03-operadores.md).
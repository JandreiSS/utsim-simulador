# Resultados e estatística

Este capítulo explica como ler o relatório do `utsim run` e o que está por trás dos números: replicações, aquecimento, intervalos de confiança e a lei de Little.

## Rodadas, replicações e sementes

Uma **rodada** é uma simulação completa, do tempo 0 até `run.stop_time`. Como os tempos são sorteados, duas rodadas dão números diferentes. Uma **replicação** é uma rodada independente, com os seus próprios números aleatórios. O UTSim roda `run.replications` replicações (em paralelo) e resume os resultados.

- A **semente** (`run.seed`) fixa os sorteios. Com a mesma semente, o resultado é idêntico a cada execução.
- Cada objeto tem o seu próprio gerador, derivado da semente, da replicação e da posição do objeto. Isso permite comparar cenários: se você muda só o tempo de uma máquina, os sorteios dos outros objetos continuam os mesmos.
- Para trocar a semente ou o número de replicações sem editar o arquivo: `utsim run modelo.ron --seed 7 -n 20`.

## Aquecimento (warm-up)

No início da rodada o sistema está vazio, o que não representa a rotina da fábrica. As primeiras horas da simulação puxam as médias para baixo. Para evitar isso, `run.warmup` descarta o começo: as estatísticas só passam a valer a partir de `t = warmup`. O sistema continua com os itens que já estavam dentro, apenas as medições são zeradas.

Como escolher o aquecimento:

- sistemas **que começam vazios e voltam sempre ao vazio** (um turno de trabalho, uma agenda do dia) não precisam de aquecimento: o que importa é justamente o começo. Use `warmup: 0.0`;
- sistemas **contínuos**, que operam sem parar, precisam de aquecimento suficiente para sair do estado vazio. Uma regra prática: rode com `--sample` e olhe a série do conteúdo das filas; o aquecimento é o tempo até a série deixar de subir;
- teste: rode duas vezes, com aquecimento diferente, e veja se os resultados mudam. Se não mudam, o aquecimento é suficiente.

O `warmup` precisa ser menor que o `stop_time`.

## Como ler a tabela do `utsim run`

```text
objeto         tipo           entradas       saídas   conteúdo médio   permanência média
chegada        source       0.0000 ± ...   19623.3 ± ...   0.0000 ± ...      0.0000 ± ...
fila           queue        19623.3 ± ...  19622.5 ± ...   3.3128 ± 0.2128  16.5400 ± 1.0244
caixa          processor    19622.5 ± ...  19622.7 ± ...   0.8022 ± 0.0046   4.0067 ± 0.0194
saida          sink         19622.7 ± ...      0.0000 ...
```

| Coluna | Significado |
|---|---|
| `entradas` | itens que entraram no objeto durante a janela de estatísticas |
| `saídas` | itens que saíram dele |
| `conteúdo médio` | número médio de itens dentro do objeto, ponderado pelo tempo |
| `permanência média` | tempo médio que cada item ficou no objeto |

Detalhes:

- A **fonte** mostra `0` em entradas (ela não recebe) e o número de itens criados em saídas.
- A **saída (Sink)** mostra o total em entradas e `0` em saídas.
- Numa **fila**, o `conteúdo médio` é o tamanho médio da fila e a `permanência média` é o tempo médio de espera.
- Num **processo com uma máquina**, o `conteúdo médio` é a **utilização**: a fração do tempo em que a máquina está ocupada. Com `capacity: n`, é o número médio de máquinas ocupadas; divida por `n` para obter a utilização por máquina.
- Para esteiras, porta-paletes e recursos móveis, o conteúdo é o número de itens dentro (ou, para transportadores, carregados).

Depois da tabela vêm as linhas de **estados**, uma por objeto:

```text
caixa    idle 19.78% ± 0.46 | processing 80.22% ± 0.46
```

É a fração do tempo da janela em que o objeto esteve em cada estado. As frações somam 100%. Estados comuns: `idle`, `empty`, `generating`, `releasing`, `setup`, `processing`, `blocked`, `waiting_operator`, `waiting_transport`, `down`, `off_shift`, `stopped`, e, para recursos móveis, `travel_empty`, `travel_loaded`, `loading`, `unloading`, `utilize`.

A mesma linha traz as métricas próprias de cada objeto, como o `time_in_system` da `Sink` (o tempo total do item no sistema), as métricas de fluxos e as de fluidos.

> **Cuidado com `processing`.** É a fração do tempo em que há pelo menos um item sendo processado. Num processo com várias máquinas, ele é bem maior que a utilização por máquina. Para a utilização, use o conteúdo médio dividido pela capacidade.

Na última linha o programa informa quantos eventos foram processados e quanto tempo levou.

## Intervalo de confiança

Todo número vem como `média ± meia-amplitude`. A média é a das replicações. A meia-amplitude é a do **intervalo de confiança de 95%**:

```text
meia-amplitude = t x desvio-padrão / raiz(n)
```

onde `n` é o número de replicações e `t` é o valor da distribuição t de Student para `n - 1` graus de liberdade (por exemplo, 2,262 para 10 replicações). Com uma única replicação não há como calcular o intervalo, e o programa mostra `NaN`.

Como interpretar: "se eu repetisse o experimento muitas vezes, em 95% delas o intervalo cobriria o valor verdadeiro". Na prática, o valor verdadeiro provavelmente está dentro do intervalo.

- O intervalo **estreita com a raiz do número de replicações**. Para reduzir a meia-amplitude à metade, quadruplique as replicações.
- Uma boa meta é meia-amplitude de uns 5% da média ou menos.
- Se duas médias têm intervalos que não se encostam, elas são diferentes. Se os intervalos se sobrepõem, **não conclua nada** olhando só os intervalos. Use a comparação pareada de [Experimentos e otimização](experimentos-e-otimizacao.md), que usa as mesmas sementes e é muito mais sensível.

## A lei de Little

Para qualquer parte do sistema em regime estável:

```text
L = λ x W
```

onde `L` é o número médio de itens, `λ` é a taxa de chegada (itens por unidade de tempo) e `W` é o tempo médio que cada item fica. É uma conferência valiosa de qualquer relatório.

Exemplo, com o [tutorial 01](tutoriais/01-um-caixa.md) (`λ` = 1 cliente a cada 5 minutos = 0,2 por minuto):

- na fila: `λ x Wq` = 0,2 x 16,54 = 3,31, e o relatório mostra um conteúdo médio de 3,31;
- no caixa: 0,2 x 4,01 = 0,80, e o conteúdo médio é 0,80;
- no sistema todo: 0,2 x 20,55 = 4,11, que é a soma 3,31 + 0,80.

Se a conta não fecha, algo está errado: aquecimento curto demais, sistema instável ou erro de modelagem.

Para ver `λ` direto do relatório, divida as saídas pelo comprimento da janela (`stop_time - warmup`).

## Sistema estável ou não

Se a taxa de chegada média é maior que a capacidade de atendimento (utilização maior ou igual a 1), a fila cresce sem parar. O conteúdo médio e a espera dependem de quanto tempo você simulou, e não convergem. O [tutorial 02](tutoriais/02-gargalo.md) mostra um caso: com a furação saturada, a fila média chega a 474 peças e o tempo no sistema, a 1.647 minutos.

Sinais de instabilidade:

- utilização de um objeto igual a 1,00 ou muito perto disso;
- fila que só cresce na série temporal;
- intervalos de confiança enormes.

## Séries temporais e histogramas

```bash
utsim run modelo.ron --sample 100 --bins 20 --csv saida/
```

- `--sample <intervalo>` (ou `run.sample_interval`) registra o conteúdo e o estado de cada objeto a cada intervalo, a partir do aquecimento;
- `--bins <n>` (ou `run.histogram_bins`) gera histogramas da permanência e das métricas próprias (por exemplo, `time_in_system`).

## Arquivos de saída

| Opção | Resultado |
|---|---|
| `--json arquivo.json` | os relatórios completos de todas as replicações |
| `--csv pasta/` | quatro arquivos: `resumo.csv` (média, desvio e meia-amplitude de cada métrica), `replicacoes.csv` (valor por replicação), e, quando existem, `series.csv` e `histogramas.csv` |

As colunas do `resumo.csv` são `metrica`, `n`, `media`, `desvio_padrao` e `meia_largura_95`.

## Nomes das métricas

Experimentos, otimização e testes referem-se às métricas por nome, no formato `objeto.métrica`:

| Nome | Significado |
|---|---|
| `objeto.input` | itens que entraram |
| `objeto.output` | itens que saíram |
| `objeto.content.mean` | conteúdo médio |
| `objeto.content.max` | conteúdo máximo |
| `objeto.staytime.mean` | permanência média |
| `objeto.state.<estado>` | fração do tempo no estado (por exemplo, `maquina.state.processing`) |
| `objeto.<medida>.mean` | média de uma medida própria (por exemplo, `saida.time_in_system.mean`) |
| `objeto.<nome>` | métrica calculada pelo objeto (nos fluidos, `fluidos.tanque.nivel_medio`) |

## Quanto confiar em um modelo

Um bom hábito é a **validação**: compare a simulação com algo que você conhece. Pode ser uma fórmula (como a de uma fila M/M/1 no tutorial 01), pode ser um dado real da fábrica (vazão por turno, tempo de atravessamento). Se o modelo não reproduz o que já se mede, não use-o para prever o que ainda não se mede.
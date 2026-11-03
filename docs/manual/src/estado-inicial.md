# Simular a partir de um estado

Toda rodada começa, por padrão, com o sistema vazio: filas sem itens, máquinas ociosas, relógio dos turnos em zero. Para estudar "o que acontece a partir de agora" (a linha de segunda às 6 h, o turno da noite que começa com as filas cheias, a próxima hora a partir do que a fábrica tem neste instante), dê à rodada um **estado inicial**, num arquivo à parte do modelo:

```bash
utsim run examples/turnos.ron --estado examples/estados/turnos_21h30.ron
```

## O arquivo

```ron
InitialState(
    clock: 930.0,          // instante do calendário em t = 0 (aqui, 21h30 em minutos desde 06h00)
    // warmup: 60.0,       // aquecimento; sem ele, nenhum
    objects: {
        "fila": (count: 20),                                    // 20 itens, sem história
        "fila_montagem": (items: [(elapsed: 12.0, labels: {"lote": 7.0}), (elapsed: 8.0)]),
        "corte": (items: [(phase: Processing, elapsed: 1.5)]),  // 1,5 min já processados
        "M2": (down: 30.0),                                      // quebrada há 30 min
        "M3": (up: 400.0),                                       // funcionando há 400 min desde o reparo
        "esteira": (items: [(at: 9.0), (at: 6.5)]),             // posições, da frente para trás
        "estoque": (items: [(slot: 3, elapsed: 20.0), (phase: Done)]),
        "empilhadeira": (position: (12.0, 4.0, 0.0)),
        "chegada": (elapsed: 2.0),                               // 2 min desde a última chegada
        "fluidos": (levels: {"tanque": 40.0}),
    },
)
```

Os tempos são sempre **decorridos** (quanto já passou), na unidade do modelo. Objetos que não aparecem começam como de costume. Os campos de cada item são opcionais:

| campo | significado |
|---|---|
| `elapsed` | tempo já gasto na atividade atual: a fase do processador, a permanência do porta-paletes, a espera na fila, a viagem na esteira |
| `stay` | tempo no objeto até agora, para a permanência (padrão: `elapsed`) |
| `age` | tempo desde a criação, para o tempo no sistema (padrão: `stay`) |
| `labels` | rótulos do item |
| `phase` | no processador: `Setup`, `Processing` ou `Done` (pronto, esperando sair); no porta-paletes, `Done` diz que a permanência mínima já foi cumprida |
| `at` | na esteira: distância do início |
| `slot` | no porta-paletes: posição `nível * baias + baia` |

| objeto | aceita |
|---|---|
| `Queue` | `count` ou `items` (até a capacidade); lotes são montados de novo, e o `max_wait` conta do primeiro item |
| `Processor` | `count` ou `items` com `phase` (até a capacidade); com operadores, cada fase espera de novo a equipe |
| `Conveyor` | `items` com `at` (sem `at`, encostados no fim, como numa esteira bloqueada) |
| `Rack` | `items` com `slot` e `phase` |
| `Source` | `elapsed` (o relógio de chegadas já correu) ou um item que ainda não saiu (`on_blocked: Wait`) |
| `Transporter` | `position` (parado ali; com `home`, volta para casa) |
| rede de fluidos (pelo nome em `fluids.name`) | `levels` por tanque |
| qualquer objeto com `downtime` | `down` (quebrado há tanto tempo) ou `up` (funcionando há tanto tempo desde o último reparo) |
| objeto com [manutenção](manutencao.md) | também `age` (idade da máquina) e `preventive` (quanto falta para cada plano de preventiva) |
| qualquer objeto com `shifts` | `off_shift`, só como conferência: quem decide é o calendário no `clock` |

`Combiner`, `Separator`, `Sink`, `Dispatcher` e os fluxos de tokens ainda não aceitam estado: aparecer com itens no arquivo é um erro. Um estado incompatível com o modelo é recusado antes de rodar, com o objeto e o motivo (objeto inexistente, mais itens que a capacidade, fase `Setup` num processador sem setup, `down` num objeto sem quebras, esteira com itens sobrepostos, campo que o tipo não aceita, nível de tanque fora da capacidade).

## O relógio do calendário

`clock` diz em que ponto do calendário está o `t = 0` da rodada. Os turnos (`shifts`, `calendars`) e as chegadas por período (`schedule`) começam nesse ponto do ciclo: com período de 1440 e `clock: 930`, a janela que começa no minuto 960 abre em `t = 30`; uma janela já aberta em `clock` começa aberta, com o tempo que falta. O resto do modelo não muda: o relatório continua contando de `t = 0`.

## O que falta de cada atividade

Uma atividade que já durou `a` não recomeça do zero: o tempo que falta é sorteado da distribuição condicionada a ter durado mais que `a` (`X - a`, dado `X > a`), do *stream* do próprio objeto. Mesma semente e mesmo estado dão o mesmo resultado.

| distribuição | como o restante é sorteado |
|---|---|
| `Exp` | sem memória: um sorteio comum |
| `Const(c)` | `c - a` (sem sorteio) |
| `Uniform` | uniforme entre `a` e o máximo |
| `Weibull`, `LogLogistic` | fórmula fechada pela função de sobrevivência |
| `Triangular`, `Empirical` | inversa da distribuição acumulada no trecho acima de `a` |
| `Normal`, `LogNormal`, `Gamma`, `Erlang`, `Beta`, `PearsonV`, `PearsonVI` | inversão numérica da sobrevivência exata (bisseção) |
| `Discrete` | os valores acima de `a`, com os pesos renormalizados |
| `Geometric` | sem memória nos inteiros |
| `Poisson`, `Binomial`, `NegBinomial` | a massa de probabilidade acima de `a`, renormalizada |
| `Shifted` | a distribuição interna com `a - offset` |

Vale para o processamento e o setup, a permanência no porta-paletes, o reparo (`down`, sobre o `mttr`), a próxima quebra (`up`, sobre o `mtbf`) e a próxima chegada (`elapsed` da fonte). Se `a` passa do máximo possível da distribuição (por exemplo 7 num `Triangular` que vai até 6), o restante é zero e o `utsim run` avisa: o dado provavelmente está errado. Itens dados só por `count` também geram aviso: entram com permanência zero, e a permanência e o tempo no sistema dos primeiros ficam subestimados.

## Prever a partir do estado

`utsim prever` roda várias replicações a partir do estado até um horizonte e resume o que
deve acontecer: a produção esperada com a precisão da média e a faixa do que pode
acontecer, a chance de cumprir uma meta, quando uma fila passa de um limite e o efeito de
variações "e se" comparadas com as mesmas sementes:

```bash
utsim prever examples/turnos.ron --estado examples/estados/turnos_21h30.ron --horizonte 2h --unidade min --meta "producao>=10"
```

Veja a seção [Previsão](gemeo-digital.md#previsão) do gêmeo digital, que faz o mesmo a
partir do estado observado na fábrica.

## Aquecimento

Com estado inicial, o padrão é **sem aquecimento**: o estado já é o sistema em operação, e o que interessa é justamente o que acontece a partir dele. Para descartar o começo mesmo assim, ponha `warmup` no arquivo ou use `--warmup` na linha de comando (que, com `--estado`, vale para o estado).

## Gravar o estado num instante e encadear rodadas

```bash
utsim run modelo.ron --salvar-estado 480 segunda_14h.ron       # estado da replicação 0 em t = 480
utsim run modelo.ron --estado segunda_14h.ron --stop 240         # as 4 h seguintes, a partir dele
utsim run modelo.ron --estado segunda_14h.ron --salvar-estado 240 segunda_18h.ron
```

`--salvar-estado T ARQUIVO` roda a replicação 0 (a partir do `--estado`, se houver) até `T`, grava o estado e depois faz a rodada pedida normalmente. O `clock` gravado soma o do estado de partida com `T`, então as pernas encadeadas seguem o mesmo calendário. O que não cabe no arquivo é avisado e se perde: itens em viagem numa ligação com tempo, itens dentro de contêineres, itens carregados por transportadores, o volume parcial dos conversores de fluidos. As tarefas de transporte e de operador em andamento não são gravadas: os objetos que as pediram pedem de novo ao retomar.

Retomar não dá exatamente os mesmos números da rodada contínua quando há aleatoriedade, porque os restantes são sorteios novos; dá resultados **estatisticamente compatíveis**. Num modelo só com tempos constantes, a retomada repete a rodada contínua.

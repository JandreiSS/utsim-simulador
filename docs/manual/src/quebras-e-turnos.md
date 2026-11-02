# Quebras e turnos

Qualquer objeto pode quebrar ao acaso (`downtime`) e parar em horários fixos (`shifts`). Os dois usam o mesmo mecanismo: o objeto fica parado, o item em processamento é **pausado** (não perdido) e o trabalho continua do ponto onde parou quando o objeto volta. No fim de um turno também dá para **terminar** o trabalho em andamento antes de sair (`on_end: Finish`, veja abaixo).

## Quebras: `downtime`

```ron
(name: "torno", kind: Processor(process: Exp(mean: 3.0)),
    downtime: (mtbf: Exp(mean: 300.0), mttr: Exp(mean: 20.0)))
```

| Campo | Significado |
|---|---|
| `mtbf` | tempo entre falhas (distribuição) |
| `mttr` | tempo para reparar (distribuição) |

- O `mtbf` conta no **relógio**, e não no tempo de uso. A falha vem depois de `mtbf` desde o último reparo, mesmo que o objeto esteja parado por falta de peças.
- Na falha, a entrada e a saída fecham e o estado vira `down`.
- Os tempos do objeto pausam e retomam com o tempo que faltava.
- Os sorteios das quebras usam um gerador próprio. Ligar ou desligar quebras não muda os tempos de chegada e de processamento sorteados.
- Uma fonte quebrada para de gerar chegadas, e o relógio de chegadas fica pausado.
- Um Processor quebrado **não libera** os operadores: eles ficam no posto e os tempos pausam.

### Disponibilidade

Uma conta rápida: `disponibilidade = MTBF / (MTBF + MTTR)`. Com MTBF 300 e MTTR 20, a máquina fica disponível 300 / 320 = 93,75% do tempo. O relatório mostra a fração real no estado `down`.

## Turnos: `shifts`

```ron
(name: "corte", kind: Processor(process: Triangular(min: 2.0, mode: 3.0, max: 5.0)),
    shifts: (period: 1440.0, off: [
        (start: 360.0, duration: 60.0),
        (start: 960.0, duration: 480.0),
    ]))
```

`period` é o tamanho do ciclo (um dia, uma semana) e `off` lista as janelas **fora de turno**. Cada janela tem `start` (deslocamento dentro do ciclo, a partir de t = 0) e `duration`.

As janelas se repetem a cada `period`. Elas precisam:

- ter duração maior que 0;
- ficar dentro do período;
- estar em ordem;
- não se sobrepor.

Fora de turno, o estado mostrado é `off_shift`. Se há quebra e turno ao mesmo tempo, o estado mostrado é `down`. O objeto só volta a trabalhar quando nenhum motivo de parada está ativo.

### No fim do turno: pausar ou terminar (`on_end`)

```ron
shifts: (period: 1440.0, off: [(start: 300.0, duration: 1140.0)], on_end: Finish)
```

| `on_end` | O que acontece quando uma janela fora de turno começa |
|---|---|
| `Pause` (padrão) | o objeto para na hora. O item em processamento congela e continua quando o turno volta, do ponto onde parou |
| `Finish` | o objeto **não aceita itens novos** a partir do fim do turno, termina o que está em andamento (hora extra) e só então sai de turno. Se estiver ocioso, sai na hora |

Também valem os nomes em português: `ao_fim_do_turno: Pausar` ou `ao_fim_do_turno: Terminar`.

Com `Finish`:

- a janela fora de turno não muda: o turno seguinte começa na hora de sempre, e a hora extra **não reduz** a capacidade dele. Se a hora extra passar do fim da janela, o objeto simplesmente continua trabalhando;
- durante a hora extra o estado continua sendo o do trabalho (`processing`, por exemplo); o `off_shift` começa quando o trabalho acaba;
- o que conta como "trabalho em andamento": num Processor, Separator ou esteira, os itens ainda não liberados; numa montagem (`Combiner`), só a preparação ou o processamento já começados (um conjunto incompleto espera o próximo turno); filas e porta-paletes guardam itens, mas não trabalham, e saem de turno na hora;
- um item pronto que não consegue sair (saída bloqueada) espera o próximo turno, como com `Pause`;
- se o objeto estiver quebrado na hora do fim do turno, ele sai de turno como com `Pause`.

Exemplo, com processo de 4 e janela fora de turno de 6 a 9 (ciclo de 10): com `Pause`, o item que começou em 4 trabalha até 6, fica parado de 6 a 9 e termina em 11; com `Finish`, ele termina em 8 (hora extra de 6 a 8), o objeto fica fora de turno de 8 a 9 e o próximo item entra em 9. Numa clínica, é a diferença entre a consulta das 11h59 terminar ao meio-dia ou ficar parada até a manhã seguinte.

### Recursos móveis e equipes

Um `Transporter` (empilhadeira, operador) também pode ter `shifts`:

- com `Pause`, ele para onde estiver (no meio de uma viagem, inclusive) e continua quando o turno volta. Um operador que está **num posto** (estado `utilize`) fica nele: o posto continua o trabalho e libera o operador no fim, normalmente. Um membro de equipe que está livre ao sair de turno ainda pode receber uma tarefa do despachante, que espera o turno dele voltar;
- com `Finish`, ele termina a viagem ou o trabalho no posto em andamento e então sai de turno, sem pegar a próxima tarefa da fila. Numa equipe (`Dispatcher`), o despachante não dá tarefas novas a um membro que está terminando o turno ou fora dele; os outros membros atendem, e o membro volta à lista de livres quando o turno recomeça.

Para dar a mesma escala a todos os membros de uma equipe, use um calendário (abaixo).

## Calendários: `calendars`

Quando vários objetos seguem a mesma escala (todos os médicos do turno da manhã, toda a linha 1), escreva as janelas uma vez, num calendário do modelo, e aponte os objetos para ele:

```ron
Model(
    name: "Clínica",
    run: (stop_time: 44640.0),
    calendars: {
        "S1": (period: 1440.0, off: [(start: 300.0, duration: 1140.0)], on_end: Finish),
        "adm": (period: 1440.0, off: [(start: 600.0, duration: 840.0)]),
    },
    objects: [
        (name: "medicos_S1", kind: Processor(capacity: 6, process: Exp(mean: 13.0)), shifts: Calendar("S1")),
        (name: "triagem_S1", kind: Processor(capacity: 3, process: Exp(mean: 5.0)), shifts: Calendar("S1")),
        (name: "recepcao", kind: Processor(process: Exp(mean: 2.0)), shifts: Calendar("adm", Pause)),
        // ...
    ],
)
```

- Um calendário é uma tabela de turnos igual à de `shifts` (`period`, `off`, `on_end`).
- `shifts: Calendar("nome")` usa o calendário como ele é; `Calendar("nome", Finish)` (ou a forma longa `(calendar: "nome", on_end: Finish)`) troca só o `on_end` neste objeto. `Calendario("nome")` também vale.
- A forma antiga, com as janelas no próprio objeto, continua valendo, e os resultados são idênticos aos de um calendário com as mesmas janelas.
- Um calendário que não existe é um erro: `medicos_S1: o calendário 'S4' não existe (calendários: S1, adm)`.

No editor, os calendários ficam na seção **Calendários** do painel Simulação (clique num ponto vazio da cena): criar, renomear (os objetos que o usam acompanham), editar as janelas e o fim do turno, e ver quem usa cada um. Na seção Turnos de um objeto, escolha "usar calendário" e o nome na lista, ou "personalizado" para janelas só dele; "salvar como calendário" transforma as janelas personalizadas num calendário novo.

Nos experimentos: `"calendars.S1": "(period: 1440.0, off: [...])"` troca o calendário inteiro, `"calendars.S1.on_end": "Pause"` só o fim do turno, e `"medicos_S1.shifts.on_end": "Finish"` muda um objeto.

Exemplo: o modelo `examples/turnos.ron` usa minutos, um dia de 1440 min, almoço de 60 min e noite de 480 min.

> **Dica:** o relógio do modelo começa em t = 0. Escolha esse instante para ser, por exemplo, o início do primeiro turno, e calcule as janelas a partir dele.

## Como aparece no relatório

Os estados `down` e `off_shift` aparecem na linha de estados do objeto, como fração do tempo da janela de estatísticas. Os recursos móveis também têm esses estados.

Para saber o efeito das quebras, compare dois cenários: um com `downtime` e outro sem. O capítulo [Experimentos e otimização](experimentos-e-otimizacao.md) mostra como (o campo `downtime` pode ser alterado em um cenário com `"torno.downtime": "None"`).
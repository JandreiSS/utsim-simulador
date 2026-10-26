# Quebras e turnos

Qualquer objeto pode quebrar ao acaso (`downtime`) e parar em horários fixos (`shifts`). Os dois usam o mesmo mecanismo: o objeto fica parado, o item em processamento é **pausado** (não perdido) e o trabalho continua do ponto onde parou quando o objeto volta.

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

Exemplo: o modelo `examples/turnos.ron` usa minutos, um dia de 1440 min, almoço de 60 min e noite de 480 min.

> **Dica:** o relógio do modelo começa em t = 0. Escolha esse instante para ser, por exemplo, o início do primeiro turno, e calcule as janelas a partir dele.

## Como aparece no relatório

Os estados `down` e `off_shift` aparecem na linha de estados do objeto, como fração do tempo da janela de estatísticas. Os recursos móveis também têm esses estados.

Para saber o efeito das quebras, compare dois cenários: um com `downtime` e outro sem. O capítulo [Experimentos e otimização](experimentos-e-otimizacao.md) mostra como (o campo `downtime` pode ser alterado em um cenário com `"torno.downtime": "None"`).
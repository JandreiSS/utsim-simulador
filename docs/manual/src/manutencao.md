# Manutenção de máquinas e OEE

As [quebras](quebras-e-turnos.md) simples (`downtime: (mtbf, mttr)`) param a máquina ao
acaso e a consertam sozinhas. Este capítulo mostra o que mais cabe no mesmo `downtime`:

- **equipe de manutenção**: o reparo só começa quando um técnico chega à máquina;
- **desgaste**: a chance de quebrar cresce com a idade da máquina (Weibull);
- **manutenção preventiva** por tempo de uso, por ciclos, pelo relógio ou por calendário;
- **custos** de manutenção;
- **OEE** (disponibilidade × desempenho × qualidade), **MTBF e MTTR observados** e o
  tempo esperando técnico, no relatório e no painel.

Tudo é opcional. Um `downtime` só com `mtbf` e `mttr` continua exatamente como antes,
com os mesmos resultados. O exemplo completo está em `examples/manutencao.ron`.

## Exemplo

```ron
(name: "torno", kind: Processor(process: Triangular(min: 0.9, mode: 1.0, max: 1.4)),
 downtime: (
    mtbf: Exp(mean: 600.0),
    mttr: Triangular(min: 10.0, mode: 15.0, max: 30.0),
    repair_crew: "manutencao",
    wear: Weibull(shape: 2.0, scale: 600.0),
    basis: Busy,
    preventive: [(every: Hours(300.0), duration: Const(12.0))],
    cost: (per_time_down: 3.0, per_repair: 150.0, per_preventive: 40.0),
 )),
(name: "tecnico", kind: Transporter(speed: 60.0, home: true), position: (9.0, -6.0, 0.0)),
(name: "manutencao", kind: Dispatcher(team: ["tecnico"])),
```

## Campos do `downtime`

| Campo | Padrão | Significado |
|---|---|---|
| `mtbf` | (obrigatório) | tempo entre falhas, contado na base `basis`; não é usado quando há `wear` |
| `mttr` | (obrigatório) | tempo de reparo; com equipe, conta a partir da chegada do técnico |
| `repair_crew` | nenhuma | equipe (`Transporter` ou `Dispatcher`) que manda o técnico |
| `wear` | nenhum | lei de falha pela idade: `Weibull(shape: β, scale: η)` |
| `basis` | `Clock` | o que a idade conta: `Clock` (relógio), `Busy` (tempo de uso) ou `Cycles` (peças) |
| `repair` | `AsBadAsOld` | quanto a corretiva rejuvenesce: `AsBadAsOld`, `AsGoodAsNew` ou `Partial(q)` |
| `preventive` | `[]` | planos de preventiva (abaixo) |
| `cost` | sem custos | `(per_time_down, per_repair, per_preventive)` |
| `oee` | padrões | `(ideal_cycle, reject_to, reject_label)` |

### Idade da máquina (`basis`)

- `Clock`: o tempo do relógio em que a máquina não está quebrada nem em preventiva
  (também fora do turno, como nas quebras simples).
- `Busy`: o tempo de uso, isto é, os estados de trabalho (`processing`, `setup` e, para
  recursos móveis, viagem, carga, descarga e trabalho no posto). Ociosa, bloqueada ou
  parada, a máquina não envelhece.
- `Cycles`: as peças que saíram da máquina. A falha acontece logo depois da saída da
  peça que leva a idade ao valor sorteado.

Sem `wear`, a próxima falha vem `mtbf` (um sorteio, na unidade da base) depois do fim do
último reparo ou da última preventiva.

### Desgaste (`wear`)

Com `Weibull(shape: β, scale: η)`, a taxa de falha numa idade `a` é
`h(a) = (β/η)·(a/η)^(β−1)`:

- `β < 1`: falhas cada vez mais raras (mortalidade infantil);
- `β = 1`: taxa constante, sem memória, igual à exponencial de média `η`;
- `β > 1`: desgaste, a máquina velha quebra mais. É o caso em que a preventiva compensa.

A idade na próxima falha é sorteada condicionada à idade atual `a`:
`A = η·((a/η)^β − ln(1 − U))^(1/β)`, com `U` uniforme do gerador de quebras da máquina.
Com `β = 1` isso é `a − η·ln(1 − U)`, o mesmo sorteio do `Exp(mean: η)`: trocar
`mtbf: Exp(mean: 100.0)` por `wear: Weibull(shape: 1.0, scale: 100.0)` com
`repair: AsGoodAsNew` dá exatamente os mesmos resultados.

### Efeito da corretiva (`repair`)

| Valor | Idade depois do reparo |
|---|---|
| `AsBadAsOld` (padrão) | a mesma: reparo mínimo, "tão ruim quanto velha" |
| `AsGoodAsNew` | zero: "tão boa quanto nova" |
| `Partial(q)` | `q × idade`, com `q` entre 0 e 1 (idade virtual de Kijima, tipo II) |

A preventiva sempre deixa a máquina como nova (idade zero).

## Equipe de manutenção (`repair_crew`)

Na quebra, a máquina pede um técnico à equipe e fica no estado **`waiting_repair`**
(aguardando técnico). O técnico é um `Transporter` (uma pessoa, em geral com
`home: true` para voltar à oficina) ou um `Dispatcher` com vários técnicos. Ele anda até a
máquina pelo mesmo caminho que os operadores usam (linha reta, rede de caminhos ou
caminho livre, conforme o [modo de movimentação](movimentacao.md)). Quando ele chega, a
máquina passa a `down` e o `mttr` começa a contar; no fim, o técnico é liberado.

**Prioridades.** Os pedidos de corretiva (prioridade 20) passam na frente dos de
preventiva (10), que passam na frente dos pedidos de operadores (0). Um técnico que já
começou um serviço termina antes de pegar o próximo. Com um técnico só e duas máquinas
quebradas, a segunda espera na fila do técnico.

Se a equipe de manutenção também opera ou transporta, `utsim check` avisa: um técnico
ocupado atrasa o reparo e pode haver espera circular.

## Manutenção preventiva (`preventive`)

```ron
preventive: [
    (every: Hours(300.0), duration: Const(12.0)),               // a cada 300 de uso
    (every: Cycles(400), duration: Const(8.0)),                 // a cada 400 peças
    (every: Clock(1440.0), duration: Const(30.0)),              // 1440 depois da anterior
    (every: Calendar("almoco"), duration: Const(30.0), crew: "eletrica"),
]
```

| `every` | A preventiva fica devida |
|---|---|
| `Hours(t)` | depois de `t` de tempo de uso desde a anterior (na unidade de tempo do modelo; o nome vem do horímetro) |
| `Cycles(n)` | depois de `n` peças desde a anterior |
| `Clock(t)` | `t` de relógio depois do fim da anterior |
| `Calendar("nome")` | no início de cada janela do calendário `nome` do modelo (`calendars`) |

- **Sem interromper a peça.** Quando a preventiva fica devida, a máquina fecha a
  entrada, termina a peça em processo e só então para (como o fim de turno com
  `Finish`). Itens a caminho dela também chegam antes.
- Com equipe (`crew` do plano ou, sem ele, a `repair_crew`), a máquina espera o técnico
  em `waiting_repair` e depois fica em **`maintenance`** pela `duration`. Sem equipe, ela
  para sozinha pela `duration`.
- Uma preventiva devida enquanto a máquina está quebrada espera o reparo. Fora do turno,
  ela pode acontecer se a máquina não tem trabalho pausado (é o uso típico de
  `Calendar`: manutenção no horário em que a linha para).
- Duas preventivas devidas ao mesmo tempo são feitas uma depois da outra.
- As durações das preventivas usam um gerador próprio: ligar uma preventiva não muda os
  sorteios das quebras nem dos tempos de processo.

### Quando a preventiva compensa

Com desgaste `Weibull(β, η)`, reparo mínimo e preventiva que renova a cada `T` de uso, o
número esperado de quebras entre duas preventivas é `H(T) = (T/η)^β`, e o custo por
unidade de tempo é (substituição periódica com reparo mínimo, Barlow e Hunter, 1960)

```text
C(T) = (c_p + c_f · (T/η)^β) / T
```

com `c_p` o custo da preventiva e `c_f` o da corretiva. Para `β > 1` o mínimo fica em
`T* = η · (c_p / (c_f · (β − 1)))^(1/β)`. O teste
`age_based_preventive_matches_the_textbook_cost_rate` confere a simulação com essa
fórmula. Para `β ≤ 1` a preventiva não reduz as quebras: só custa.

## Custos (`cost`)

| Campo | Custo |
|---|---|
| `per_time_down` | por unidade de tempo parada por manutenção (esperando técnico, em reparo ou em preventiva) |
| `per_repair` | por corretiva concluída |
| `per_preventive` | por preventiva concluída |

O resultado é `maintenance.cost`, na unidade de dinheiro que você escolher.

## OEE

Para cada objeto com manutenção, o relatório traz o OEE da janela de estatística (depois
do aquecimento):

- **tempo planejado** = janela − tempo fora do turno − tempo em preventiva;
- **disponibilidade** = (planejado − tempo em corretiva) / planejado, com o tempo em
  corretiva somando a espera pelo técnico e o reparo;
- **desempenho** = ciclo ideal × peças / tempo funcionando (no máximo 1);
- **qualidade** = peças boas / peças;
- **OEE** = disponibilidade × desempenho × qualidade.

O **ciclo ideal** é `oee.ideal_cycle` quando informado; senão o mínimo da distribuição do
`process` (a média quando o mínimo é zero ou ilimitado, como na exponencial), dividido
pela `capacity`. Para objetos sem tempo de processo, o desempenho é 1.

Peças **refugadas** são as que saem para um dos objetos de `oee.reject_to` (em geral uma
`Sink` de refugo ligada à saída da máquina) ou as que saem com o rótulo
`oee.reject_label` diferente de 0 ou vazio:

```ron
oee: (ideal_cycle: 1.0, reject_to: ["refugo"], reject_label: "defeito"),
```

## Resultados

Cada objeto com manutenção ganha estas medidas (em `utsim run`, no JSON, no CSV, nos
experimentos e no [painel](painel.md), entre as métricas do objeto). No painel, o OEE
medido também vira o KPI `oee:<objeto>` com os três fatores, a menos que o arquivo do
painel já configure o OEE desse objeto:

| Medida | Significado |
|---|---|
| `oee`, `oee.availability`, `oee.performance`, `oee.quality` | OEE e seus três fatores |
| `maintenance.failures` | quebras na janela |
| `maintenance.repairs`, `maintenance.preventives` | corretivas e preventivas concluídas |
| `maintenance.mtbf` | tempo funcionando (sem corretiva nem preventiva) / quebras; sem quebras, o tempo funcionando todo (um limite inferior) |
| `maintenance.mttr` | tempo médio da quebra até voltar a funcionar (espera + reparo) |
| `maintenance.repair_time` | duração média do reparo em si |
| `maintenance.wait_technician` | espera média pelo técnico (corretivas e preventivas concluídas) |
| `maintenance.wait_technician_total` | espera total pelo técnico, incluindo a que ainda está em curso |
| `maintenance.down_time`, `maintenance.preventive_time` | tempo parado em corretiva e em preventiva |
| `maintenance.cost` | custo de manutenção |
| `maintenance.age` | idade da máquina no fim da rodada |
| `maintenance.rejects` | peças refugadas |

Os estados novos `waiting_repair` (aguardando técnico) e `maintenance` (em preventiva)
aparecem na tabela de estados, nos gráficos de estados e no Pareto de perdas do painel
(`waiting_repair` entra como perda; a preventiva é parada planejada).

`utsim check` lista, para cada objeto com manutenção, a lei de falha, a equipe, o efeito
da corretiva, os planos de preventiva e o ciclo ideal do OEE.

## Estado inicial

Ao [gravar o estado](estado-inicial.md) de uma máquina com manutenção, entram:

- `age`: a idade da máquina (na unidade da sua base);
- `preventive`: quanto falta para cada plano ficar devido (uso, peças ou relógio; vazio
  para planos de calendário, que seguem o relógio);
- `down` (quebrada) ou `up` (funcionando), como nas quebras simples.

```ron
"torno": (age: 412.5, preventive: [87.5], up: 30.0),
```

Uma preventiva em curso recomeça do início; um técnico a caminho é chamado de novo, e um
reparo em curso com equipe continua (pelo tempo que faltava) depois que o técnico chega
outra vez. As notas da gravação avisam quando isso acontece.

## Limites atuais

- A corretiva não interrompe o técnico que já está numa preventiva (sem preempção).
- Um técnico que também é operador pode gerar espera circular; o aviso do `utsim check`
  ajuda a evitar.
- Custos de peças de reposição e de hora do técnico entram pelo `per_repair` e pelo
  `per_time_down`; não há tabela de peças.

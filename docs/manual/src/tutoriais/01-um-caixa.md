# Tutorial 01: uma fila e um caixa (M/M/1)

**Objetivo:** montar o modelo mais simples que existe em teoria das filas, rodar e conferir se o resultado bate com a fórmula. Isso mostra que o simulador está calculando o que deveria e ensina a ler o relatório.

**Arquivo:** `examples/tutorial_01_caixa.ron`

## O problema

Um caixa de loja atende um cliente por vez. Os clientes chegam ao acaso, em média **um a cada 5 minutos**. O atendimento leva, em média, **4 minutos**. Quem chega e encontra o caixa ocupado espera numa fila única.

Chamamos de **M/M/1** uma fila em que as chegadas são aleatórias sem memória (M), o atendimento também (M) e há 1 servidor.

## O modelo

```ron
objects: [
    (name: "chegada", kind: Source(interarrival: Exp(mean: 5.0))),
    (name: "fila",    kind: Queue()),
    (name: "caixa",   kind: Processor(process: Exp(mean: 4.0))),
    (name: "saida",   kind: Sink),
],
connections: [
    ("chegada", "fila"),
    ("fila", "caixa"),
    ("caixa", "saida"),
],
```

- `chegada` cria um cliente a cada `Exp(mean: 5.0)` minutos.
- `fila` guarda quem espera. Sem `capacity`, cabe qualquer número.
- `caixa` atende um de cada vez, em `Exp(mean: 4.0)` minutos.
- `saida` recebe o cliente atendido.

A rodada simula 100.000 minutos, descarta os 2.000 primeiros (aquecimento) e repete 10 vezes.

## O que a teoria diz

Com taxa de chegada λ = 1/5 = 0,2 por minuto e taxa de atendimento μ = 1/4 = 0,25 por minuto:

| Grandeza | Fórmula | Valor |
|---|---|---|
| Utilização do caixa | ρ = λ / μ | 0,8 |
| Clientes na fila | Lq = ρ² / (1 - ρ) | 3,2 |
| Espera na fila | Wq = Lq / λ | 16 min |
| Tempo no sistema | W = Wq + 1/μ | 20 min |
| Clientes no sistema | L = ρ / (1 - ρ) | 4 |

## Rodar

```bash
utsim check examples/tutorial_01_caixa.ron
utsim run examples/tutorial_01_caixa.ron
```

Resultado esperado:

```text
Tutorial 01: um caixa: 10 replicação(ões), estatísticas de t=2000 até t=100000

objeto         tipo                     entradas                 saídas         conteúdo médio      permanência média
chegada        source            0.0000 ± 0.0000  19623.3000 ± 122.1243        0.0000 ± 0.0000        0.0000 ± 0.0000
fila           queue       19623.3000 ± 122.1243  19622.5000 ± 124.3982        3.3128 ± 0.2128       16.5400 ± 1.0244
caixa          processor   19622.5000 ± 124.3982  19622.7000 ± 124.5215        0.8022 ± 0.0046        4.0067 ± 0.0194
saida          sink        19622.7000 ± 124.5215        0.0000 ± 0.0000        0.0000 ± 0.0000        0.0000 ± 0.0000

fila           empty 35.31% ± 0.73 | releasing 64.69% ± 0.73
caixa          idle 19.78% ± 0.46 | processing 80.22% ± 0.46
saida          time_in_system 20.5463 ± 1.0275
```

## Comparar com a fórmula

| Grandeza | Teoria | Simulação (média ± IC 95%) | Onde ler |
|---|---|---|---|
| Utilização ρ | 0,8 | 0,8022 ± 0,0046 | `caixa`, conteúdo médio (ou `processing`) |
| Fila Lq | 3,2 | 3,3128 ± 0,2128 | `fila`, conteúdo médio |
| Espera Wq | 16 | 16,54 ± 1,02 | `fila`, permanência média |
| Tempo no sistema W | 20 | 20,55 ± 1,03 | `saida`, `time_in_system` |

**Todos os valores teóricos estão dentro dos intervalos de confiança.** O simulador reproduz a fórmula.

## Lei de Little

A lei de Little diz que `L = λ x W`. Confira com o relatório:

- λ = 0,2 clientes por minuto (19.623 chegadas em 98.000 minutos);
- na fila: 0,2 x 16,54 = 3,31 clientes, e o relatório diz 3,31;
- no caixa: 0,2 x 4,01 = 0,80, e o relatório diz 0,80.

A conta fecha. Se não fechasse, seria sinal de erro no modelo.

## Exercícios

1. **Mais carga.** Mude o atendimento para `Exp(mean: 4.5)` (utilização de 90%). A teoria dá Lq = 8,1, Wq = 40,5 e W = 45. A simulação dá cerca de 8,54 ± 0,75, 42,65 ± 3,64 e 47,16 ± 3,65. Veja como a fila **dispara** perto de 100% de utilização.
2. **Menos carga.** Com `Exp(mean: 3.5)` (utilização de 70%), a teoria dá Lq = 1,63, Wq = 8,17 e W = 11,67. A simulação dá cerca de 1,67, 8,35 e 11,85.
3. **Número de replicações.** Rode `utsim run examples/tutorial_01_caixa.ron -n 40` e veja o intervalo da fila cair de ±0,21 para cerca de ±0,08. Quadruplicar as replicações corta o intervalo pela metade.
4. **Rodada curta.** Rode com `--stop 3000 --warmup 0 -n 10`. Os intervalos ficam bem mais largos (fila 2,85 ± 0,76), e os valores se afastam da teoria. Rodadas curtas são ruidosas.
5. **Ver em 3D.** Abra `utsim-view examples/tutorial_01_caixa.ron`, clique em **▶ Simular** e assista aos clientes se acumulando na fila.

## O que você aprendeu

- Um modelo é uma lista de objetos e de conexões.
- O relatório mostra média ± intervalo de confiança entre replicações.
- A utilização é o conteúdo médio de um processo com uma máquina.
- A simulação concorda com a teoria, e a lei de Little serve para conferir qualquer relatório.

Próximo: [Tutorial 02, linha com gargalo](02-gargalo.md).
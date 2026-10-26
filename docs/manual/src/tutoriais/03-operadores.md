# Tutorial 03: operadores que andam entre máquinas

**Objetivo:** ver, com números, que a **distância entre as máquinas** muda o resultado quando há operadores que precisam andar até elas.

**Arquivos:** `examples/tutorial_03_operadores.ron` e `examples/experimento_tutorial_03.ron`

Tempo em **segundos**, distâncias em **metros**.

## O problema

Uma célula tem três máquinas alimentadas por uma fila única. Cada máquina precisa de **um operador durante o setup** (média de 20 s). Depois do setup, o operador sai para atender outra máquina e a peça é processada sozinha (média de 60 s). Há dois operadores, que andam a **1,2 m/s**. Chega uma peça a cada 35 s, em média.

O operador mais próximo da máquina que pediu ajuda é quem vai (`rule: Nearest`).

```ron
(name: "maq1", kind: Processor(
    setup: Some(Exp(mean: 20.0)),
    process: Exp(mean: 60.0),
    operators: Some(OperatorUse(team: "equipe", count: 1, setup: true, process: false)),
), position: (15.0, 0.0, 0.0)),
...
(name: "ana",   kind: Transporter(speed: 1.2), position: (10.0, 2.0, 0.0)),
(name: "bruno", kind: Transporter(speed: 1.2), position: (10.0, 8.0, 0.0)),
(name: "equipe", kind: Dispatcher(team: ["ana", "bruno"], rule: Nearest), position: (10.0, 5.0, 0.0)),
```

As máquinas estão em `y = 0`, `y = 5` e `y = 10`, ou seja, a **5 m** uma da outra. Uma caminhada entre máquinas leva 5 / 1,2 ≈ 4 s.

## Rodar o modelo original

```bash
utsim check examples/tutorial_03_operadores.ron
utsim run examples/tutorial_03_operadores.ron
```

Trecho do resultado:

```text
maq1    idle 15.10% | processing 61.75% | setup 20.69% | waiting_operator 2.47%
maq2    idle 21.32% | processing 56.93% | setup 19.01% | waiting_operator 2.74%
maq3    idle 28.22% | processing 51.38% | setup 17.56% | waiting_operator 2.84%
saida   time_in_system 149.6152 ± 11.1576
ana     idle 64.90% | travel_empty 3.76% | utilize 31.34%
bruno   idle 71.43% | travel_empty 2.65% | utilize 25.92%
```

- Cada máquina passa de 2,5% a 2,8% do tempo em `waiting_operator`: espera um operador chegar.
- Os operadores passam 3,8% e 2,7% do tempo andando (`travel_empty`) e 31% e 26% trabalhando nas máquinas (`utilize`).
- Cada peça fica em média 150 s no sistema.

## Mudar a distância

O arquivo `experimento_tutorial_03.ron` roda o mesmo modelo com as máquinas mais afastadas. **Só mudam as posições**; tempos, número de máquinas e de operadores são os mesmos.

| Cenário | Posição das máquinas (`y`) | Distância entre máquinas |
|---|---|---|
| `5 m` | 0, 5, 10 | 5 m |
| `20 m` | -15, 5, 25 | 20 m |
| `50 m` | -45, 5, 55 | 50 m |

Os operadores são reposicionados entre as máquinas. Rode:

```bash
utsim experiment examples/experimento_tutorial_03.ron
```

Resultado (médias entre 10 replicações):

```text
medida                         5 m   fila a 200 m       20 m        50 m
maq1.state.waiting_operator   0.0247     0.0247       0.1019      0.3021
maq2.state.waiting_operator   0.0274     0.0274       0.1037      0.2826
maq3.state.waiting_operator   0.0284     0.0284       0.1090      0.3043
ana.state.travel_empty        0.0376     0.0376       0.1520      0.3907
bruno.state.travel_empty      0.0265     0.0265       0.1271      0.3724
fila.content.mean             1.92       1.92         3.84        112.00
saida.time_in_system.mean     149.6      149.6        224.9       4014.3
saida.input                   2796.7     2796.7       2794.6      2593.5
```

### O que os números dizem

**Quanto maior a distância, mais as máquinas esperam o operador.** A espera por operador sobe de cerca de 2,7% do tempo (5 m) para 10% (20 m) e para 29% (50 m).

**Os operadores passam a maior parte do tempo andando.** A caminhada vai de 3% para 15% e para 39% do tempo. Em 50 m, cada viagem leva uns 42 s, mais que o próprio setup de 20 s.

**O efeito não é linear.** De 5 m para 20 m, o tempo no sistema passa de 150 s para 225 s (+50%). Mas de 20 m para 50 m ele passa de 225 s para **4.014 s**. A 50 m, os operadores já não dão conta: a fila cresce (112 peças em média) e a produção cai de 2.797 para 2.594 peças (-7%). O sistema ficou **instável**: o tempo de caminhada consumiu a capacidade dos operadores.

Uma conta rápida confirma. A 50 m, cada setup custa ao operador cerca de 20 s de trabalho mais uns 42 s de caminhada até lá, 62 s por peça. Duas pessoas oferecem 2 / 62 = 0,032 peça por segundo, e a demanda é 1 / 35 = 0,029. O operador fica perto de 90% ocupado só com setups e caminhadas, e as variações aleatórias levam a fila ao crescimento.

### A contraprova: a fila que está longe

O cenário `fila a 200 m` afasta a fila e a chegada **200 metros** das máquinas. O resultado é **idêntico** ao de `5 m`, até a última casa decimal.

Isso acontece porque a ligação da fila para a máquina é uma **transferência direta entre objetos fixos**, e ela é instantânea. Ninguém anda por ali. Só os operadores se deslocam, então só as distâncias que **eles** percorrem importam. Veja a seção "A distância importa?" em [Recursos móveis](../recursos-moveis.md).

> **Em breve:** uma opção para dar tempo de transferência proporcional à distância também nas ligações diretas. Hoje, para que uma distância entre dois objetos fixos custe tempo, ponha entre eles um transportador (como no [tutorial 04](04-armazem.md)) ou uma esteira.

## Faça no visualizador

1. Abra `utsim-view examples/tutorial_03_operadores.ron`.
2. Clique em **▶ Simular** e assista aos operadores caminhando entre as máquinas.
3. Volte para **Editar**, arraste `maq1` e `maq3` para mais longe, clique de novo em **▶ Simular** e compare o tempo que os operadores passam andando.

## Exercícios

1. **Mais um operador.** Acrescente um terceiro `Transporter` (por exemplo, `carla`) e ponha-o na equipe (`team: ["ana", "bruno", "carla"]`). Rode com as máquinas a 50 m. A instabilidade desaparece?
2. **Setup no processamento.** Mude `process: false` para `process: true` em `OperatorUse`. O operador agora fica na máquina até o fim do processamento. Veja o que acontece com a espera por operador.
3. **Operador volta para casa.** Em `Transporter`, acrescente `home: true`. Veja o efeito sobre a caminhada: o operador ocioso volta para a posição inicial.

## O que você aprendeu

- A posição dos objetos importa para os recursos que **andam**: operadores, empilhadeiras, AGVs.
- Ligações diretas entre objetos fixos são instantâneas, e a posição delas não muda o resultado.
- Um pequeno aumento de distância pode empurrar um sistema folgado para a instabilidade.

Próximo: [Tutorial 04, empilhadeiras e porta-paletes](04-armazem.md).
# Ergonomia sobre o estudo de tempos

O comando `utsim ergonomia` dá uma nota de ergonomia a cada elemento do estudo de tempos (`utsim tempos`) e a cada posto. A repetição e a duração saem dos ciclos que o UTSim mede. A postura, a carga, a pega e a vibração são marcadas pelo usuário em escalas de poucos níveis.

> A nota de ergonomia do UTSim é uma pontuação própria para estudo, ensino e comparação de métodos. Ela não é a análise ergonômica do trabalho exigida pela NR-17 quando aplicável, que continua sendo responsabilidade do profissional habilitado.

## Marcações

Um CSV com uma linha por elemento de um ator, com os nomes de elemento que o `utsim tempos` mostra:

```text
ator;elemento;tronco;bracos;pescoco;pernas;carga_kg;pega;vibracao
ana;carga/descarga maq1;2;2;1;0;12;1;0
ana;andar até maq1;0;0;0;1;0;0;0
```

| Campo | Escala |
|---|---|
| `tronco`, `bracos`, `pescoco`, `pernas` | 0 neutra, 1 leve, 2 acentuada, 3 extrema |
| `carga_kg` | peso ou força manipulada, em kg |
| `pega` | 0 boa, 1 regular, 2 ruim |
| `vibracao` | 0 nenhuma, 1 alguma, 2 forte |

Elemento sem marcação conta como neutro. Esperas, ociosidade, bloqueio e máquina trabalhando sozinha ficam fora da nota.

## A nota

A nota vai de 0 a 10. Ela junta três partes, cada uma de 0 a 1, com pesos: a postura (tronco e braços pesam o dobro de pescoço e pernas), a carga com a pega (a carga em degraus de 3, 10 e 20 kg) e a vibração. Essa base é multiplicada por um fator de exposição, de 0,5 a 1, que cresce com a maior entre a repetição (degraus de 60, 300 e 900 vezes por hora) e a fração do ciclo gasta no elemento. As faixas são baixa (até 3), moderada (até 5), alta (até 7) e muito alta.

A regra é monótona: piorar a postura, aumentar a carga, a repetição ou a duração nunca baixa a nota. Um teste confere isso em todas as combinações das escalas. A nota do posto é a maior nota dos seus elementos, com a média ponderada pelo tempo ao lado.

A empresa que usa outro método cadastra os pesos, os degraus e as faixas num CSV (`--metodo`), no formato que `parametro;valor` descreve. Um método com peso negativo ou degraus fora de ordem é recusado.

## Comparar métodos

```bash
utsim ergonomia examples/homem_maquina.ron --marcacoes examples/ergonomia/marcacoes_homem_maquina.csv --comparar examples/homem_maquina_melhorado.ron --html ergonomia.html
```

A comparação usa o mesmo par de estudos do `utsim tempos --comparar` e mostra, por posto, a nota e o tempo padrão antes e depois da mudança. Com `--marcacoes-depois` o estudo novo usa outras marcações (uma bancada mais alta, um dispositivo que tira a carga das mãos).

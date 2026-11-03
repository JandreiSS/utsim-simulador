# Pessoas: fadiga, aprendizado e habilidades

Um `Transporter` pode representar uma pessoa que anda, carrega itens e opera postos
(veja [Recursos móveis](recursos-moveis.md)). Com o campo opcional `person`, essa pessoa
ganha características físicas que mudam o desempenho ao longo da rodada:

- **fadiga e recuperação**: uma tolerância fixa nos tempos (engenharia de métodos) ou um
  nível de fadiga que sobe trabalhando e andando e cai parado, deixando os tempos mais
  longos e a caminhada mais lenta, com **pausas** automáticas quando passa de um limite;
- **curva de aprendizado**: cada repetição da mesma tarefa fica mais rápida, até o tempo
  padrão;
- **caminhada com carga**: mais lenta conforme o peso levado;
- **habilidades**: quem pode operar qual posto;
- **ritmo individual**: uma pessoa mais rápida ou mais lenta que a outra, sorteado em cada
  replicação;
- **levantamento manual**: aviso quando o peso passa do limite de referência.

Tudo é opcional e desligado. Um modelo sem `person` roda exatamente como antes, com os
mesmos resultados. O exemplo completo está em `examples/pessoas_fadiga.ron`.

> **Os parâmetros são estimativas.** Os padrões abaixo são pontos de partida defensáveis,
> não medições. Para conclusões sobre um posto real, meça os tempos e as pausas ou use os
> valores de um estudo de ergonomia do local. As referências legais e técnicas estão
> marcadas **[conferir no texto vigente]**: confira a versão atual de cada uma antes de
> usar o resultado para decidir.

## Exemplo

```ron
(name: "ana", kind: Transporter(speed: 1.3, load_time: Const(4.0), unload_time: Const(4.0)),
    shifts: Calendar("pausas"),
    person: Some((
        fatigue: Some(Dynamic(work: 2.0e-4, walk: 1.0e-4, carry: 1.0e-5,
                              limit: Some(0.5), target: 0.2)),
        learning: Some((rate: 0.85, first: 1.6)),
        skills: ["torno"],
        walk_speed_loaded: Some(()),
        variation: Some(()),
        lift: Some((horizontal: 40.0, vertical: 30.0, distance: 50.0)), // limite ≈ 11 kg
    ))),
(name: "torno", kind: Processor(process: Triangular(min: 40.0, mode: 50.0, max: 70.0),
    operators: Some(OperatorUse(team: "equipe", requires: "torno")))),
```

`Some(())` liga uma parte com todos os padrões. O peso de cada item vem do rótulo
`weight`, em kg (por exemplo `labels: {"weight": Uniform(min: 8.0, max: 18.0)}` na
`Source`); `weight_label: "peso"` usa outro rótulo.

## Campos de `person`

| Campo | Padrão | Significado |
|---|---|---|
| `fatigue` | nenhuma | `Allowance(p)` ou `Dynamic(...)` (abaixo) |
| `learning` | nenhuma | curva de aprendizado `(rate, first)` |
| `skills` | `[]` | habilidades da pessoa, por exemplo `["torno", "empilhadeira"]` |
| `walk_speed_loaded` | nenhuma | velocidade com carga `(per_kg, above, min)` |
| `variation` | nenhuma | ritmo individual `(dist, min, max)` |
| `weight_label` | `"weight"` | rótulo com o peso do item, em kg |
| `lift` | nenhum | geometria do levantamento, para o aviso do NIOSH |

O perfil só vale para um `Transporter`. Ele vale para tudo o que a pessoa faz: andar,
carregar e descarregar itens, operar um posto (`OperatorUse`) e atender a manutenção
(neste último caso só a caminhada; o tempo de reparo é o `mttr` da máquina).

## Fadiga

### Tolerância fixa (`Allowance`)

`fatigue: Allowance(0.15)` multiplica os tempos de operação da pessoa (processo e setup
dos postos que ela opera, carga e descarga) por 1,15. É a **tolerância de descanso** da
engenharia de métodos: um percentual somado ao tempo normal para cobrir fadiga e
necessidades pessoais. Não tem estado: a primeira peça e a última levam o mesmo. A
caminhada não muda.

As tabelas de tolerância por fadiga mais usadas estão em *Introduction to Work Study*
(OIT, Kanawaty (ed.), 4.ª ed., 1992) e em Niebel e Freivalds, *Methods, Standards, and
Work Design*. Valores típicos de tolerância constante ficam entre 9 % e 11 %, somados aos
variáveis pela postura, pelo peso e pelo ambiente [conferir no texto vigente].

### Modelo dinâmico (`Dynamic`)

A pessoa tem um nível de fadiga `F` entre 0 (descansada) e 1. Ele muda só nos eventos
(início e fim de tarefa, de pausa e de turno), por fórmulas fechadas, sem passo de tempo:

- trabalhando por `t`: `F(t) = 1 − (1 − F0)·e^(−λ·t)`;
- parada por `t`: `F(t) = F0·e^(−μ·t)`.

É a forma usada por Jaber, Givi e Neumann, "Incorporating human fatigue and recovery into
the learning–forgetting process", *Applied Mathematical Modelling* 37, 2013, com base
fisiológica em Rohmert (1960) e em Konz, "Work/rest", *International Journal of
Industrial Ergonomics*, 1998 [conferir no texto vigente].

| Campo | Padrão | Significado |
|---|---|---|
| `work` | `1.0e-4` | `λ` trabalhando no posto, carregando ou descarregando (por unidade de tempo) |
| `walk` | `5.0e-5` | `λ` andando vazia |
| `carry` | `5.0e-6` | `λ` a mais por kg levado andando |
| `recovery` | `1.0e-3` | `μ` parada (ociosa, em pausa, fora do turno, esperando) |
| `time_factor` | `0.2` | `β`: os tempos de operação começados com fadiga `F` ficam × `(1 + β·F)` |
| `speed_factor` | `0.2` | `α`: a caminhada começada com fadiga `F` fica × `(1 − α·F)` |
| `limit` | nenhum | com ele, a pessoa pausa quando `F ≥ limit` |
| `target` | `0.1` | nível ao fim da pausa |

As taxas estão **por segundo** nos padrões (com `work = 1e-4`, uma hora seguida de
trabalho leva a fadiga a 0,30; com `recovery = 1e-3`, 15 minutos parada tiram 59 % dela).
Num modelo em minutos, multiplique as taxas por 60.

O que conta como trabalho: `utilize` (no posto), `loading`, `unloading`, `travel_empty`
(com `walk`) e `travel_loaded` (com `walk + carry × kg`). Todo o resto recupera, inclusive
`off_shift`: os **intervalos de turno** e as paradas programadas já são pausas.

### Pausas pelo limite

Com `limit`, ao terminar uma tarefa com `F ≥ limit` a pessoa entra no estado `resting`
pelo tempo que leva para a fadiga cair até `target`, calculado em forma fechada:
`ln(F / target) / μ`. Só depois ela pega a próxima tarefa (o despachante espera por ela e
dá o trabalho a quem está livre). A pausa nunca interrompe uma tarefa no meio.

## Curva de aprendizado

`learning: (rate: 0.85, first: 2.0)` faz a `n`-ésima repetição da mesma tarefa levar

`max(first · n^(−b), 1)` × o tempo padrão, com `b = −log₂(rate)`

(Wright, "Factors affecting the cost of airplanes", *Journal of the Aeronautical
Sciences*, 1936 [conferir no texto vigente]). Com taxa de 85 %, cada vez que as
repetições dobram o tempo fica 85 % do anterior: `T₈ = T₁ × 0,85³`. O tempo do modelo
(`process`, `setup`, `load_time`, `unload_time`) é o tempo padrão, o piso da curva; `first`
diz quantas vezes mais a primeira repetição leva.

Cada tarefa conta à parte, por pessoa: operar um posto (o nome do posto), o setup do
posto (`posto:setup`), carregar (`carga`) e descarregar (`descarga`). O esquecimento
entre turnos não está implementado.

## Caminhada com carga

`walk_speed_loaded: (per_kg: 0.01, above: 5.0, min: 0.8)` reduz a velocidade em 1 % por
kg acima de 5 kg, sem passar abaixo de 0,8 m/s (nem acima da velocidade vazia). O peso é a
soma dos rótulos `weight` dos itens levados naquele trecho. A velocidade de caminhada
usual de um adulto fica entre 1,2 e 1,5 m/s; 1,3 m/s é um bom padrão para `speed`.

Com fadiga dinâmica a velocidade ainda é multiplicada por `1 − α·F`, e com ritmo
individual por `v`. A velocidade é calculada no início de cada trecho.

## Habilidades

Cada pessoa lista o que sabe fazer em `skills`; o posto diz o que exige em
`OperatorUse(requires: "torno")`. O despachante só entrega a tarefa a quem tem a
habilidade, mantendo a regra (`FirstAvailable`, `RoundRobin`, `Nearest`) entre os
qualificados; uma pessoa que fica livre pega a primeira tarefa da fila para a qual está
habilitada. Sem `requires`, qualquer membro da equipe serve, como antes.

`utsim check` avisa quando ninguém da equipe tem a habilidade exigida (a tarefa ficaria
esperando para sempre) e quando há menos qualificados que o `count` pedido. Com a equipe
sendo um único `Transporter` (sem despachante), não há escolha: o aviso diz que a tarefa
vai para ele assim mesmo.

## Ritmo individual

`variation: (dist: Normal(mean: 1.0, sd: 0.1), min: 0.7, max: 1.3)` (os padrões) sorteia,
no início de cada replicação, um ritmo `v` para a pessoa: os tempos de operação ficam
divididos por `v` e a velocidade multiplicada por `v`. O sorteio é truncado em
`[min, max]` e vem de um *stream* próprio da pessoa, então ligar a variação não muda as
chegadas nem os tempos sorteados pelos outros objetos.

## Levantamento manual (aviso)

O UTSim não impede um levantamento pesado; ele avisa no `utsim check` e no `utsim run`.
Para cada pessoa com perfil que carrega itens, o maior peso que as fontes dão ao rótulo
`weight` é comparado com:

- o **limite de peso recomendado** (RWL) da equação revisada de levantamento do NIOSH
  (Waters, Putz-Anderson, Garg e Fine, "Revised NIOSH equation for the design and
  evaluation of manual lifting tasks", *Ergonomics* 36(7), 1993 [conferir no texto
  vigente]): `RWL = 23 kg × HM × VM × DM × AM × FM × CM`, com `HM = 25/H`,
  `VM = 1 − 0,003·|V − 75|`, `DM = 0,82 + 4,5/D`, `AM = 1 − 0,0032·A` (distâncias em cm,
  ângulo em graus) e `FM`, `CM` das tabelas da publicação. O índice de levantamento é
  `LI = peso / RWL`: aviso com `LI > 1` e **alerta** com `LI > 3`. Sem `lift`, o RWL é a
  constante de carga de 23 kg (condições ideais);
- o limite de referência da **CLT, art. 198** (60 kg por empregado na remoção individual)
  [conferir no texto vigente]. A CLT, art. 390, traz limites menores para o trabalho da
  mulher (20 kg contínuo, 25 kg ocasional) [conferir no texto vigente].

A **NR-17 (Ergonomia)** pede que o transporte manual de cargas não comprometa a saúde
ou a segurança do trabalhador e que ele seja treinado; ela não fixa um peso único e
remete à análise ergonômica do posto [conferir no texto vigente].

| Campo de `lift` | Padrão | Significado |
|---|---|---|
| `horizontal` | `25` | `H`: distância horizontal das mãos ao ponto médio dos tornozelos (cm) |
| `vertical` | `75` | `V`: altura das mãos na origem (cm) |
| `distance` | `25` | `D`: deslocamento vertical da carga (cm) |
| `asymmetry` | `0` | `A`: ângulo de torção do tronco (graus) |
| `frequency` | `1` | `FM`: multiplicador de frequência (tabela da publicação) |
| `coupling` | `1` | `CM`: multiplicador de pega (tabela da publicação) |

Referências consultadas em 2026-11-03; a conferência do texto vigente fica com quem usa o
resultado.

## Métricas

Cada pessoa com perfil ganha, no relatório (`<pessoa>.<medida>`), no CSV, no JSON e nos
experimentos:

| Métrica | Significado |
|---|---|
| `person.pace` | ritmo individual da replicação (1 sem `variation`) |
| `person.fatigue_mean` | fadiga média no tempo (dinâmico) |
| `person.fatigue_max` | fadiga máxima (dinâmico) |
| `person.fatigue_end` | fadiga no fim da rodada (dinâmico) |
| `person.time_above_limit` | tempo com `F ≥ limit` (dinâmico com `limit`) |
| `person.rests` | pausas pelo limite (dinâmico com `limit`) |
| `person.rest_time` | tempo em pausa (`resting`) |
| `person.break_time` | tempo fora do turno (intervalos) |
| `person.repetitions` | repetições contadas pela curva de aprendizado |
| `person.learning_factor` | multiplicador médio do tempo padrão pela curva |
| `person.learning_gain` | `1 − learning_factor / first`: fração do tempo da 1.ª repetição que a prática economizou |

O estado `resting` aparece junto dos outros estados da pessoa.

## Estado inicial

Num [estado inicial](estado-inicial.md), uma pessoa com perfil aceita também
`fatigue` (o nível em `[0, 1)`, com fadiga dinâmica) e `experience` (repetições já feitas
de cada tarefa, por exemplo `experience: {"torno": 40}`). `--salvar-estado` grava os dois;
o ritmo individual é sorteado de novo na rodada seguinte.

```ron
"ana": (position: (6.0, 3.0, 0.0), fatigue: 0.31, experience: {"torno": 42, "carga": 80}),
```

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
| `sex` | nenhum | `Some(Female)` ou `Some(Male)` (também `Feminino`, `Masculino`); sem valor, não informado |
| `height_m` | nenhuma | altura em metros |
| `mass_kg` | nenhuma | massa corporal em kg |
| `age` | nenhuma | idade em anos (registro) |
| `experience_years` | nenhuma | anos de experiência na função (registro) |
| `capacity` | nenhuma | capacidade medida `(load_kg, force_n)`: maior carga que a pessoa levanta na função (kg) e maior força que ela faz ao empurrar ou puxar (N) |
| `study` | nenhum | estudo de tempos da pessoa: as cronometragens brutas (abaixo) |
| `times` | `{}` | tempos próprios da pessoa por tarefa, `{"torno": (dist: ..., method: "A")}` (abaixo) |

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

### Curva de fadiga a partir de cronometragens

A fadiga de uma pessoa pode sair das medições dela. Cronometre a mesma tarefa, pelo mesmo
método, em momentos diferentes do turno (no início, no meio e no fim, pelo menos) e guarde
as leituras brutas no `person.study`, para que o ajuste possa ser refeito:

```ron
person: Some((
    study: Some((
        readings: [
            (task: "torno", method: "A", hour: 0.5, times: [41.2, 39.8, 40.5]),  // início
            (task: "torno", method: "A", hour: 4.0, times: [44.0, 45.1, 43.7]),  // meio
            (task: "torno", method: "A", hour: 7.5, times: [47.9, 48.6, 49.2]),  // fim
        ],
        pauses: [(4.25, 4.5)],   // pausas do turno, (início, fim) em horas
    )),
)),
```

`hour` são as horas desde o começo do turno; os tempos estão na unidade do modelo. Os
nomes em português também valem (`estudo`, `leituras`, `tarefa`, `metodo`, `hora`,
`tempos`, `pausas`). A rodada não lê o estudo: ela usa o resultado do ajuste, gravado em
`times` e em `fatigue` (veja "Tempos próprios" abaixo). Não precisa de pausa: sem
`pauses`, o turno é corrido.

O ajuste (`utsim_fit::fatigue`) usa o mesmo modelo dinâmico do núcleo:

1. **A curva.** O tempo médio na hora `h` é `t0·(1 + β·F(h))`, com
   `F(h) = 1 − e^(−λ·h)` contando só as horas de trabalho e a recuperação nas pausas.
   Saem `t0` (o tempo da pessoa descansada), `β` e `λ`. Com 3 horários a curva passa
   exatamente pelas médias, e o resultado avisa que não há folga para conferi-la. Com 4 ou
   mais, ela é ajustada por mínimos quadrados e vem com os intervalos de confiança de `t0`
   e do acréscimo `t0·β`.
2. **A distribuição.** Cada tempo dividido por `1 + β·F(h)` é o tempo que a pessoa levaria
   descansada. Esses tempos passam pelo ajuste de distribuições de sempre (candidatas,
   critério e aderência), e sai a distribuição de `t0` daquela pessoa naquela tarefa e
   método.
3. **A recuperação (`μ`).** Ela só é estimada quando há leituras antes e depois de uma
   pausa e pelo menos 4 horários. Sem isso, fica o padrão do UTSim, e o resultado avisa.

O resultado também avisa quando os tempos caem ao longo do turno (é aprendizado ou
aquecimento, não fadiga: `β` fica 0 no modelo) e quando a curva é quase reta (`λ` e `β`
não se separam: meça mais perto do fim do turno ou em mais horários). As taxas saem por
hora e são convertidas para a unidade do modelo (por segundo, as do padrão).

`PersonFit::apply` grava o resultado no perfil: a distribuição do tempo descansado vai
para `times` (com o método medido), e a curva dá `work`, `time_factor` e, quando foi
estimada, `recovery` da fadiga `Dynamic`. Os outros campos da fadiga (`walk`, `carry`,
`speed_factor`, `limit`, `target`), e a recuperação que não foi estimada, ficam como
estavam no perfil.

Pela linha de comando:

```sh
utsim pessoa ajustar examples/pessoas_tempos_proprios.ron --pessoa ana
utsim pessoa ajustar modelo.ron --pessoa ana --gravar --saida ajustado.ron
```

Sem `--gravar`, o comando só mostra o ajuste de cada tarefa e método do estudo: `t0`, o
acréscimo da fadiga, `λ`, `μ` (estimada ou padrão), os intervalos, a média de cada
horário ao lado da curva, a distribuição escolhida e os avisos. Com `--gravar`, ele grava
o resultado no perfil, no próprio modelo ou em `--saida`. O estudo fica no perfil, para o
ajuste poder ser refeito. Para gravar, cada tarefa precisa de um método só: com leituras
de dois métodos, o comando mostra os dois para comparar e pede `--metodo`. Com várias
tarefas, todas ganham tempo próprio, e a curva da que tem mais leituras vira a fadiga do
perfil (ou a de `--fadiga-de`). `--unidade` diz a unidade de tempo do modelo (`s`, `min`
ou `h`; as taxas saem por hora), e `--json` grava tudo, com a comparação de todas as
distribuições candidatas. Gravar regrava o modelo inteiro pelo editor, que mantém o
comentário do começo do arquivo, mas não os comentários do meio. Por isso, use
`--saida` se o modelo tiver comentários que você quer manter.

No UTSim 3D, ao selecionar uma pessoa com estudo, o painel de propriedades ganha a seção
**Curva de fadiga (estudo de tempos)**. Para cada tarefa e método, ela traz um gráfico
com a média medida em cada horário (pontos), a curva ajustada (linha, pelo mesmo ajuste do
comando) e as pausas sombreadas. Embaixo do gráfico vêm `t0`, o acréscimo da fadiga, `λ` e
`μ` (estimada ou padrão) e os avisos. A seção é só de leitura: as leituras e a gravação do
ajuste ficam no UTSim Estatística ou no comando. O botão **Abrir no UTSim Estatística**
abre as leituras numa planilha do Estatística, uma linha por tempo medido (`tarefa`,
`metodo`, `hora` e `tempo`), para analisar. O modelo não muda nem fica com o outro app.

### Tempos próprios

Com `times`, a pessoa tem o seu próprio tempo numa tarefa:

```ron
person: Some((
    times: {"torno": (dist: Normal(mean: 40.0, sd: 3.0), method: "A")},
    fatigue: Some(Dynamic(work: 1.6667e-4, time_factor: 0.3)),
)),
```

Quando essa pessoa faz a tarefa, o tempo é sorteado da distribuição dela, no lugar do
tempo padrão do posto, e a fadiga (ou a tolerância) e a curva de aprendizado continuam
valendo por cima. O ritmo individual (`variation`) não entra, porque o tempo medido da
pessoa já tem o ritmo dela. A tarefa é o nome do posto (`Processor`), `<posto>:setup` para
a preparação, `carga` e `descarga` para pegar e deixar itens. Uma tarefa sem entrada
continua com o tempo padrão, e uma pessoa sem `times` continua exatamente como antes.

- O sorteio sai do gerador da própria pessoa. O posto ainda sorteia o seu tempo padrão,
  e por isso as outras sequências de números aleatórios do modelo não mudam.
- Com mais de um operador por peça, quem demora mais manda, como no tempo padrão. Com
  `team_speedup`, o tempo próprio também é dividido pelo ganho da equipe.
- Uma peça que já estava no posto num estado inicial termina com o tempo padrão que
  restava.
- O `utsim check` mostra os tempos próprios de cada pessoa e avisa quando a tarefa não é
  posto do modelo nem `carga` ou `descarga` (o tempo nunca seria usado).
- Os nomes em português também valem (`tempos`, `distribuicao`, `metodo`).

O exemplo `examples/pessoas_tempos_proprios.ron` põe duas pessoas no mesmo torno, com
curvas diferentes: ana é mais rápida descansada, mas cansa depressa; bruno é mais lento,
mas quase não cansa.

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

## Regra de operação do posto

O campo `operation` (ou `operacao`) de um `Processor` diz quem pode operar o posto e quanto
tempo o operador leva para entrar e sair dele (porta, proteção, célula):

```ron
(name: "torno", kind: Processor(process: Const(60.0),
    operators: Some(OperatorUse(team: "equipe"))),
    ppe: ["capacete"],
    operation: Some((
        skills: ["torno"],      // habilidades, além do `requires` de cada equipe
        require_ppe: true,      // exigir o EPI do posto e das zonas em que ele está
        enter: Some(Const(5.0)),
        leave: Some(Const(4.0)),
    ))),
(name: "ana", kind: Transporter(speed: 1.2), ppe: ["capacete", "oculos"],
    person: Some((skills: ["torno"]))),
```

| Campo | Padrão | O que faz |
|---|---|---|
| `skills` (`habilidades`) | `[]` | habilidades que todo operador precisa ter |
| `require_ppe` (`exigir_epi`) | `false` | só opera quem veste o EPI do posto (`ppe` do nó) e o das zonas de risco que contêm o posto |
| `enter` (`entrar`) | nenhum | tempo para a equipe entrar antes do trabalho; o posto fica em `entering` |
| `leave` (`sair`) | nenhum | tempo para a equipe sair depois do trabalho; o posto fica em `leaving` |

O EPI que uma pessoa veste é o `ppe` do próprio nó dela. O despachante só entrega a tarefa
a quem atende à regra inteira, e `utsim check` avisa quando ninguém da equipe atende.

A equipe entra quando chega completa para o primeiro estágio que precisa dela e sai depois
do último. Com setup e processo feitos pela mesma equipe, ela entra uma vez e sai uma vez.
Se as equipes forem diferentes, a do setup sai antes de a do processo entrar. O operador
fica no posto (`utilize`) da entrada até o fim da saída. Quebra ou turno pausam a entrada e
a saída como pausam o trabalho. Um operador tirado para um trabalho mais urgente durante a
entrada faz a entrada recomeçar do ponto em que parou quando a equipe volta a estar
completa. Durante a saída, ele simplesmente vai embora. Sem `operation`, o posto funciona
como antes.

## Ritmo individual

`variation: (dist: Normal(mean: 1.0, sd: 0.1), min: 0.7, max: 1.3)` (os padrões) sorteia,
no início de cada replicação, um ritmo `v` para a pessoa: os tempos de operação ficam
divididos por `v` e a velocidade multiplicada por `v`. O sorteio é truncado em
`[min, max]` e vem de um *stream* próprio da pessoa, então ligar a variação não muda as
chegadas nem os tempos sorteados pelos outros objetos.

## Quem é a pessoa

```ron
person: Some((
    sex: Some(Female), height_m: Some(1.62), mass_kg: Some(58.0),
    age: Some(34), experience_years: Some(6.0),
    capacity: Some((load_kg: Some(18.0))),
)),
```

Os campos também aceitam os nomes em português (`sexo`, `altura_m`, `massa_kg`, `idade`,
`experiencia_anos`, `capacidade` com `carga_kg` e `forca_n`). O que eles fazem hoje:

- **Nada muda na rodada.** O núcleo não lê nenhum desses campos: com ou sem eles, os
  resultados são os mesmos.
- **3D.** O sexo escolhe o corpo (sem valor, o corpo continua escolhido pelo nome). A
  altura dá a altura do boneco, e a massa, junto com a altura, dá a largura pelo índice de
  massa corporal: a largura e a profundidade crescem com √(IMC ÷ IMC do corpo base), entre
  0,85 e 1,25. Sem altura, ela é sorteada pelo nome; sem massa, a largura fica a do corpo.
  No UTSim 3D, esses campos ficam no grupo "Pessoa" do painel de propriedades.
- **Avisos.** Uma carga acima da capacidade medida (`capacity.load_kg`) dá aviso. Com
  `sex: Some(Female)`, uma carga acima de 20 kg dá o aviso da CLT, art. 390 (abaixo). A
  altura fora de 1,40 a 2,10 m e a massa fora de 35 a 200 kg também dão aviso, porque quase
  sempre são outra unidade (centímetros, libras). É só aviso: há pessoas reais fora
  dessas faixas.
- **Fadiga.** Não há padrão de fadiga por sexo, e nenhum campo daqui muda a fadiga. A
  variação entre pessoas é maior que a diferença média entre os sexos; quando houver
  medições, a fadiga vem delas.
- **Massa corporal.** As normas usadas aqui (NIOSH e CLT) não usam a massa da pessoa. Por
  isso ela é só registro e serve para o 3D.
- **Idade e experiência** são só registro.

**Dados pessoais (LGPD).** Sexo, idade, medidas e tempos de pessoas reais são dados
pessoais. Use um apelido ou um código como nome do objeto, e cuide do arquivo do modelo
como de qualquer cadastro de pessoal antes de mandá-lo a alguém.

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
  mulher (20 kg contínuo, 25 kg ocasional) [conferir no texto vigente]: com
  `sex: Some(Female)`, uma carga acima de 20 kg dá aviso;
- a **capacidade medida** da pessoa (`capacity.load_kg`), quando ela existe.

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
| `person.own_times` | operações cujo tempo saiu dos tempos próprios da pessoa (só com `times`) |

O estado `resting` aparece junto dos outros estados da pessoa.

## Estado inicial

Num [estado inicial](estado-inicial.md), uma pessoa com perfil aceita também
`fatigue` (o nível em `[0, 1)`, com fadiga dinâmica) e `experience` (repetições já feitas
de cada tarefa, por exemplo `experience: {"torno": 40}`). `--salvar-estado` grava os dois;
o ritmo individual é sorteado de novo na rodada seguinte.

```ron
"ana": (position: (6.0, 3.0, 0.0), fatigue: 0.31, experience: {"torno": 42, "carga": 80}),
```

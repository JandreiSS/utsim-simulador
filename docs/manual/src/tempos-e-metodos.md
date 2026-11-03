# Estudo de tempos e métodos (homem-máquina)

O comando `utsim tempos` faz a micro-análise do trabalho de um operador e das máquinas que ele
atende, como no estudo de tempos e métodos da engenharia de produção: acha o ciclo de trabalho
que se repete no rastro da simulação, desenha um ciclo representativo no gráfico homem-máquina
(gráfico de atividades múltiplas), mede cada elemento ao longo dos ciclos, calcula quantas
máquinas o operador consegue atender e o tempo padrão, e compara dois métodos (antes e depois).

Ele lê o mesmo rastro do [gráfico homem-máquina do fluxograma](fluxograma.md#gráfico-homem-máquina-homem-maquina)
(`utsim fluxograma --tipo homem-maquina`), que mostra a ocupação de todos os recursos num
intervalo. O `utsim tempos` vai além: separa o ciclo, os elementos e as contas do estudo de
tempos.

```bash
utsim tempos examples/homem_maquina.ron
utsim tempos examples/homem_maquina.ron --html estudo.html --csv elementos.csv
utsim tempos examples/homem_maquina.ron --ritmo 110 --tolerancia 15
utsim tempos examples/homem_maquina.ron --custo-operador 30 --custo-maquina 50
utsim tempos examples/homem_maquina.ron --comparar examples/homem_maquina_melhorado.ron --html antes_depois.html
utsim tempos examples/homem_maquina.ron --ator ana --de 1000 --ate 3000 --ciclo 5 --svg ciclo.svg
```

O exemplo `examples/homem_maquina.ron` tem uma operadora (`ana`) que atende três máquinas
automáticas: em cada uma ela descarrega a peça pronta e carrega a próxima (o setup da máquina,
de 18 a 24 s, feito com a operadora), a máquina trabalha sozinha por 70 s e a operadora anda
uns 6 m até a próxima. O `examples/homem_maquina_melhorado.ron` é o mesmo posto com troca
rápida (carga e descarga de 12 a 16 s) e máquinas mais próximas.

## Como o modelo vira um estudo de tempos

Nada de novo precisa ir no modelo. O operador é um `Transporter` (uma pessoa) que a máquina
chama com `operators: Some(OperatorUse(team: "ana", setup: true, process: false))`:

| No modelo | No estudo de tempos |
|---|---|
| máquina em `setup` com o operador | **carga/descarga**: operador e máquina juntos, a máquina parada (tempo concorrente, `a`) |
| máquina em `processing` com o operador (`process: true`) | **operação assistida**: a máquina roda com o operador junto |
| máquina em `processing` sem operador | **automático**: a máquina sozinha (`t`) |
| operador andando vazio | **andar até** a máquina seguinte (parte do tempo independente, `b`) |
| operador carregando, pegando ou entregando peça | **transportar**, **pegar peça**, **entregar peça** (também independentes) |
| máquina esperando o operador | **interferência**: máquina parada esperando |
| operador ocioso | **ocioso**; enquanto uma máquina dele roda sozinha, é o operador esperando a máquina |
| quebra, manutenção, pausa de fadiga, fora do turno | **parado**: o ciclo vira irregular |

Uma máquina cujo nome tem uma das palavras de inspeção do fluxograma (`inspec`, `confer`,
`verific`, `qualidade`, `teste`, `audit`, `triagem`, sem acento e sem diferença de maiúsculas)
faz **inspeção**. Em qual máquina o operador está sai do rastro:
ele começa a trabalhar no instante em que a máquina começa a fase que precisava dele.

## O ciclo

Como na cronometragem no chão de fábrica, um ciclo vai do início de um elemento até o próximo
início do mesmo elemento. O elemento-âncora é o trabalho mais frequente do operador na máquina
que ele mais visita (no exemplo, `carga/descarga maq1`); sem operador, a primeira fase de
trabalho da máquina.

- **Aquecimento e ciclos parciais.** Só entram os ciclos que começam e terminam dentro do
  intervalo (`--de` e `--ate`; o padrão vai do fim do aquecimento ao fim da rodada).
- **Ciclos irregulares.** A sequência dos elementos de trabalho de cada ciclo é a sua
  assinatura (por exemplo `carga/descarga maq1 > andar até maq2 > carga/descarga maq2 > ...`).
  A assinatura mais frequente é o método; um ciclo com outra assinatura (uma máquina atendida
  fora da vez) ou com uma quebra ou pausa de qualquer ator é listado como irregular e fica fora
  das estatísticas, como o elemento estranho da cronometragem. Esperas e ociosidade não entram na
  assinatura, então um pouco mais ou menos de espera não quebra o padrão.
- **Ciclo representativo.** O de duração mediana entre os regulares, ou o pedido com
  `--ciclo N` (numerados a partir de 1, na ordem do HTML).

## Como ler o relatório

**Gráfico homem-máquina do ciclo.** Uma coluna por ator (o operador e cada máquina), com o tempo
descendo num eixo comum a partir do início do ciclo. Cada faixa é um elemento, com o nome e a
duração; a cor é o tipo (legenda no topo da página). Lendo na horizontal, vê-se o que cada ator
faz no mesmo instante: no exemplo, enquanto `ana` carrega a `maq1` a `maq1` está em
carga/descarga e as outras duas rodam sozinhas.

**Elementos.** Para cada ator e elemento, o tempo por ciclo (somado se o elemento aparece mais de
uma vez no ciclo) com média, desvio-padrão, mínimo, máximo e `n`, o número de ciclos regulares
em que ele aparece. Cada ocorrência conta inteira no ciclo em que começa (uma máquina que roda
atravessando o início do ciclo conta os 70 s, não dois pedaços); uma ocorrência cortada pelo fim
da rodada fica de fora.

**Atores.** Utilização (tempo de trabalho sobre o tempo de ciclo), tempo ocioso por ciclo e
**interferência** por ciclo: na máquina, o tempo parada esperando o operador; no operador, o
tempo ocioso enquanto uma das suas máquinas roda sozinha (esperando a máquina).

**Quantas máquinas por operador.** Medido nos ciclos regulares, por máquina:

- `a`: tempo do operador junto com a máquina (carga, descarga, operação assistida);
- `b`: tempo do operador independente da máquina (caminhada incluída);
- `t`: tempo automático da máquina.

O número de máquinas que um operador atende sem deixar máquina parada é
**n' = (a + t)/(a + b)** (atendimento sincronizado). Com `n ≤ n'` máquinas, o ciclo é `a + t`
e o operador sobra; com `n > n'`, o ciclo é `n·(a + b)` e as máquinas esperam. A tabela mostra o
ciclo e as utilizações para `⌊n'⌋`, `⌊n'⌋ + 1` e o número atual. Com `--custo-operador` e
`--custo-maquina` (por hora, na moeda que quiser; `--unidade` diz se o modelo está em `s`, `min`
ou `h`), cada linha ganha o custo por unidade **(K1 + n·K2)·ciclo/n** e a mais barata entre
`⌊n'⌋` e `⌊n'⌋ + 1` fica destacada.

O ciclo teórico `max(a + t, n·(a + b))` aparece ao lado do observado. No exemplo, o observado
é maior (cerca de 97 s contra 91 s) porque no modelo a operadora só sai para a próxima máquina
quando ela chama: a máquina espera a caminhada (a interferência, uns 6 s por máquina e ciclo).
No modelo de livro o operador anda enquanto as máquinas rodam. Essa diferença é uma pista de
método: antecipar a caminhada encurta o ciclo.

**Tempo padrão.**

- tempo normal = tempo observado × ritmo (`--ritmo`, em %, padrão 100);
- tempo padrão = tempo normal × (1 + tolerância) (`--tolerancia`, em % do tempo normal:
  necessidades pessoais, fadiga e esperas).

Só os elementos do operador (e os da máquina com o operador junto) levam ritmo e tolerância; o
tempo automático da máquina não, e esperas e ociosidade não fazem parte do conteúdo de trabalho.
O ciclo normal e o padrão são `max(a + t + espera, n·(a + b))` com `a` e `b` normais ou
padrão e a espera da máquina pelo operador como observada; o tempo por unidade divide pelas
unidades do ciclo (as peças que as máquinas do grupo começam a processar nele). Sem ritmo nem
tolerância, o ciclo normal fica igual ao observado, o que confere a conta.

Se o operador tem no perfil a tolerância fixa de fadiga (`person: Some((fatigue:
Some(Allowance(0.1))))`, veja [Pessoas](pessoas.md)), os tempos de operação simulados **já**
estão multiplicados por `1 + p`: o estudo divide esses elementos por `1 + p` antes do ritmo e
usa `p` como tolerância padrão (a caminhada não é alongada pela `Allowance`, então não é
dividida). Sem `--tolerancia` nem `Allowance`, a tolerância é 0 e o relatório avisa.

## Comparação de métodos

`--comparar depois.ron` roda o mesmo estudo no segundo modelo (o método novo) e pareia os
elementos pelo nome do ator e do elemento: tempo médio antes, depois, diferença e variação. Os
indicadores do fim da tabela comparam o tempo de ciclo, as unidades por ciclo, o tempo padrão do
ciclo e por unidade, `n'` e, por ator, a utilização (diferença em pontos percentuais), o tempo
ocioso e a interferência. Para comparar dois cenários de um mesmo posto, grave cada um como um
modelo (o mesmo arquivo com os campos trocados). Um elemento que só existe num dos métodos
aparece com "—" do outro lado.

No exemplo, a troca rápida tira cerca de um terço da carga e descarga, a caminhada cai de 6 m
para 4 m, o ciclo cai uns 9 % e `n'` sobe de 3,4 para 4,7: com o método novo a mesma operadora
poderia atender quatro máquinas.

## Saídas

| Opção | O que grava |
|---|---|
| (nenhuma) | resumo no terminal: ciclos, elementos, atores, homem-máquina, tempo padrão e comparação |
| `--html arquivo.html` | relatório completo: gráfico do ciclo, tabelas, lista dos ciclos (regulares e irregulares) e comparação |
| `--svg arquivo.svg` | só o gráfico do ciclo representativo (do primeiro grupo) |
| `--csv arquivo.csv` | tabela de elementos (`grupo,ator,elemento,tipo,media,dp,min,max,n,normal,padrao`); com `--comparar`, também `arquivo-comparacao.csv` |
| `--json arquivo.json` | o estudo, o do segundo modelo e a comparação, completos |

Sem `--ator`, cada operador que trabalhou em máquinas no intervalo vira um grupo (ele e as
máquinas que atendeu). `--ator` escolhe um operador ou uma máquina (que traz o operador que a
atende); num modelo sem operadores, cada máquina é estudada sozinha. `--replica` escolhe a
replicação gravada (a primeira é a 0).

## Limitações

- O estudo é de **uma replicação**; para a variação entre replicações, repita com `--replica`.
- Com uma equipe de vários operadores (um `Dispatcher`), cada operador é um grupo com as máquinas
  que atendeu; como a escala muda a cada chamado, sobram poucos ciclos regulares.
- Processadores com capacidade acima de 1 aparecem pelo estado do bloco (o mais produtivo entre
  as peças dentro), não peça a peça.
- O cálculo de `n'` supõe máquinas iguais em rodízio fixo; com máquinas diferentes, os valores
  são médias por máquina.

## Referências

- A. Freivalds e B. W. Niebel, *Niebel's Methods, Standards, and Work Design*, cap. 2
  (gráficos homem-máquina, atendimento sincronizado e custo por unidade) e caps. 9 e 11
  (avaliação de ritmo e tolerâncias).
- M. P. Groover, *Work Systems and the Methods, Measurement, and Management of Work*, cap. 4
  (sistemas homem-máquina).
- R. M. Barnes, *Motion and Time Study: Design and Measurement of Work* (gráfico de atividades
  múltiplas, avaliação de ritmo, tolerâncias).
- Organização Internacional do Trabalho, *Introduction to Work Study* (G. Kanawaty, org.),
  parte sobre medida do trabalho.

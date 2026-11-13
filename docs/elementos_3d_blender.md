# Elementos 3D gerados no Blender

Os elementos da fábrica (máquinas, posto, robô, célula, esteira, mesa de espera, veículos,
estantes e cargas) são gerados por scripts Python no Blender, sem janela, e versionados em
`assets/modelos3d/`. Os modelos em Rust de `assets/modelos/` (`utsim-assets`) continuam
valendo como reserva e para os tamanhos por parâmetro.

## Como gerar

```bash
bash scripts/blender.sh            # todos
bash scripts/blender.sh --so cnc   # só alguns
```

O script acha o Blender (`BLENDER`, `PATH` ou a pasta padrão do Windows) e roda
`blender -b --factory-startup -P tools/blender/gerar.py -- --saida assets/modelos3d`.
As pessoas precisam também do MPFB e do pacote de assets do MakeHuman, fora do repositório
(veja `assets/modelos3d/LEIA-ME.md`).
Para conferir o visual, `tools/blender/vitrine.py` monta uma pequena linha com pessoas
e renderiza uma imagem.

Cada execução grava exatamente os mesmos bytes (mesma versão do Blender). Os testes de
`crates/utsim-assets/tests/modelos3d.rs` conferem os arquivos versionados sem o Blender.
Com `UTSIM_BLENDER=1`, o `scripts/verificar.sh` gera tudo de novo e falha se algo mudar.

## Arquivos

- `<nome>.glb`: detalhe baixo, o padrão (até 64 KB e 1500 triângulos cada).
- `<nome>_medio.glb`: arestas chanfradas e cilindros mais redondos.
- `catalogo.json`: para cada arquivo, o tamanho, os nós, os pontos (posição e giro), os
  clipes (em laço ou não, e os nós que movem) e as medidas que a ponte usa (`dados`).

Sem compressão Draco ou meshopt: o `GLTFDocument` do Godot não as lê. A leveza vem do
nível de detalhe baixo e de malhas sem textura (cerca de 1,4 MB para os 28 arquivos).

## Convenções

As convenções são as mesmas do `utsim-assets`:

- metros, Z para cima, `+x` é a frente e `+y` a esquerda;
- a origem fica no piso, no centro da base;
- o glTF fica com Y para cima;
- a raiz tem o nome do elemento;
- cada parte móvel é um nó com o pivô na junta;
- o material `pintura` recebe a cor do nó;
- o material `luz` é a luz de estado que a interface recolore.

Nas máquinas e no posto, o lado de trabalho fica em `-y`.

### Pontos (nós vazios)

A posição do ponto é o lugar. O `+x` local do ponto é para onde se olha ou para onde a
peça anda.

| Ponto | Para que serve |
|---|---|
| `ponto_operador` | onde a pessoa fica para operar (olha para a máquina) |
| `ponto_porta` | por onde a pessoa chega: nas máquinas, 0,6 m além de `ponto_operador`; na célula, fora do portão |
| `ponto_peca` | onde a peça fica durante o processo |
| `ponto_entrada`, `ponto_saida` | onde a peça chega e de onde sai (ligam um elemento ao próximo) |
| (mesa de espera) | `espera` tem só `ponto_entrada` (`-x`) e `ponto_saida` (`+x`), no tampo a 0,76 m, a altura da esteira. Os nós são `estrutura` e `faixa` (a faixa amarela no chão, que o 3D pode esconder). `dados.tampo` é a altura do tampo e `dados.area` o retângulo livre do tampo (1,10 × 0,66 m) para distribuir as peças da fila |
| `ponto_garra` | ponta dos dedos do robô (a base da peça presa) |
| `ponto_carga` | garfo da empilhadeira, mesa do AGV |
| `ponto_vao_<b>_<n>` | vão `b`, nível `n` do porta-paletes e da estante (`+x` aponta para o corredor) |

### Clipes

Cada clipe dura 1 s (24 quadros). A interface posiciona o clipe pela **fase**, de 0 a 1,
do estado que a simulação manda. Não há evento solto: a fase sai do intervalo do estado
no trace (`(t - início) / (fim - início)`). Os clipes em laço repetem a cada unidade de
fase. Assim o 3D mostra o que a simulação está fazendo em qualquer instante, mesmo ao
pular no tempo.

| Elemento | Clipe | Laço | Gatilho na simulação |
|---|---|---|---|
| cnc, prensa, injetora | `ciclo` | sim | processando |
| | `setup` | sim | em setup |
| | `porta_abrir` | não | posto em `entering` (fase 0 → 1) e em `leaving` (1 → 0), da regra de operação do nó |
| | `quebra` | não | quebrado (abre a tampa de manutenção) |
| | `ciclo_auto` | não | processando sem operador: a porta abre para a peça entrar, fecha durante o `ciclo` e abre para ela sair |
| posto | `ciclo` | sim | processando |
| | `setup` | sim | em setup (o grampo abre todo e volta) |
| robo | `pegar`, `levar`, `soltar` | não | carregar, transportar e descarregar a peça |
| | `ciclo` | sim | os três seguidos, para um robô sempre ocupado |
| | `setup` | sim | em setup (a base gira e os dedos abrem e fecham) |
| | `quebra` | não | quebrado (o braço cai e fica parado) |
| celula | `porta_abrir` | não | posto da célula em `entering` (0 → 1) e em `leaving` (1 → 0) |
| esteira | `rolar` | sim | fase = distância / (2π · `raio_rolo`) |
| empilhadeira | `garfo_subir` | não | altura do garfo / `garfo_curso` (6 m); os estágios `mastro_m` e `mastro_i` do mastro sobem junto |
| agv | `mesa_subir` | não | carregando ou descarregando |

Parado, livre e bloqueado aparecem pela cor da `luz`, como hoje.

### Tabela `estados`

Cada item do `catalogo.json` tem a tabela `estados`, que liga o estado do posto na
simulação ao clipe. A ponte lê a tabela e não precisa saber de cada elemento:

| Estado | Clipe | `de` → `ate` | Laço | `operador` |
|---|---|---|---|---|
| `entering` | `porta_abrir` | 0 → 1 | não | `ponto_porta` → `ponto_operador` |
| `leaving` | `porta_abrir` | 1 → 0 | não | `ponto_operador` → `ponto_porta` |
| `processing` | `ciclo` | 0 → 1 | sim | |
| `processing_auto` | `ciclo_auto` | 0 → 1 | não | |
| `setup` | `setup` | 0 → 1 | sim | |
| `down` | `quebra` | 0 → 1 | não | |
| `waiting_repair`, `maintenance` | `quebra` | 1 → 1 | não | |
| `loading` | `garfo_subir` ou `mesa_subir` | 0 → `levante` | não | |
| `travel_loaded` | idem | `levante` → `levante` | não | |
| `unloading` | idem | `levante` → 0 | não | |

A fase do clipe é `de + (ate - de) * fase`, com a fase do estado de 0 a 1. Num clipe em
laço, uma volta cabe no estado: uma peça, um ciclo. Só entram os estados cujo clipe o
elemento tem; a célula, por exemplo, só tem `entering` e `leaving`. O caminho
`operador` só aparece quando o elemento tem os dois pontos. Sem ele, a ponte põe a pessoa
de frente para a estação (`station_front`). Os pontos giram com a estação.

`processing_auto` não é um estado da simulação. É o `processing` de uma estação que
ninguém opera, como uma máquina carregada por robô ou por esteira. A ponte usa essa
linha no lugar de `processing` quando o nó não pede operador (o `Processor` não tem
`operators` nem `also`) e o elemento tem a linha. O critério é do modelo, não do trace,
então vale igual em qualquer instante e para qualquer peça.
A porta fica toda aberta até 2/24 da fase, fecha até 4/24, fica fechada enquanto o
`ciclo` roda até 20/24 e abre de novo até 22/24.

`levante` é a fase que só tira a carga do chão: na empilhadeira, 1/60 do curso do garfo
(0,10 m, em `dados.levante`); no AGV, a mesa sobe tudo (1). Para pôr o palete num vão
alto do porta-paletes, a ponte usa a altura do vão (fase = altura / `garfo_curso`).

### Tabela `peca`

A tabela `peca` diz onde a peça (ou a carga) fica em cada estado do elemento. Cada estado
tem trechos `[fase_de, fase_ate, ponto]`, que cobrem a fase de 0 a 1. A peça fica no ponto
do trecho em que a fase está. O ponto é um nó do `.glb` e anda com o pai, então a peça
presa na garra, no garfo ou na mesa vai junto com o clipe.

| Elemento | Estado | Onde a peça fica |
|---|---|---|
| máquinas e posto | `entering` | `ponto_entrada` (o operador vai buscá-la) |
| | `setup`, `processing`, `down`, `waiting_repair`, `maintenance` | `ponto_peca` |
| | `leaving`, `blocked` | `ponto_saida` |
| cnc, prensa, injetora | `processing_auto` | `ponto_entrada` até 2/24, `ponto_peca` até 22/24 e `ponto_saida` até o fim |
| robo | `processing` | `ponto_entrada` até 1/3, `ponto_garra` até 3/4 e `ponto_saida` até o fim |
| | `blocked` | `ponto_saida` |
| empilhadeira, agv | `loading`, `travel_loaded`, `unloading` | `ponto_carga` |

A passagem entre elementos fica com a ponte. A peça sai do `ponto_saida` de um posto e vai
para o `ponto_carga` do veículo que a carrega, ou para o `ponto_entrada` do próximo posto.
O teste confere que a garra do robô está mesmo sobre a peça em 1/3 e em 3/4 do `ciclo`.
Outro teste confere que os dedos fechados encostam na peça (0,12 m) sem atravessá-la.
Também confere que, no `ciclo_auto`, a porta está toda aberta quando a peça entra e
quando sai.

### Interações

A peça passa de um elemento a outro pelos pontos:

- da esteira, sai em `ponto_saida`;
- no AGV, vai na mesa (`ponto_carga`). A mesa sobe 6 cm e mostra os pistões embaixo;
- na garra, vai presa em `ponto_garra` entre `pegar` e `soltar`;
- na máquina, entra em `ponto_entrada` e fica em `ponto_peca`;
- na máquina sem operador, a porta abre sozinha para a peça entrar e sair
  (`processing_auto`). Com o `ponto_saida` do robô sobre o `ponto_entrada` da máquina,
  o robô carrega a máquina.

O palete vai no `ponto_carga` do garfo e fica no `ponto_vao_*` do porta-paletes. O
operador chega por `ponto_porta` e anda até `ponto_operador` enquanto a porta abre
(`entering`), opera e volta pelo mesmo caminho enquanto ela fecha (`leaving`). Fazer a peça, o palete e a pessoa seguirem esses pontos no 3D é da ponte
e da interface.

## Pessoas

`tools/blender/pessoas.py` (chamado pelo `gerar.py`) gera `pessoa_homem.glb` e
`pessoa_mulher.glb` a partir de humanos do MakeHuman, feitos pelo complemento MPFB no
Blender. O MPFB e o pacote de assets (CC0) ficam fora do repositório; a fonte, a versão, a
licença e o jeito de preparar a pasta estão em `assets/modelos3d/LEIA-ME.md`. Os arquivos têm:

- o esqueleto do boneco livre UAL1 (`assets/terceiros/quaternius_ual`), com os mesmos ossos
  e a mesma pose de repouso, e sem animações próprias. O 3D toca nelas as animações do UAL1
  e do UAL2 (andar, operar, carregar, pegar, interagir), como faz hoje no boneco;
- `corpo`: o humano do MakeHuman, com rosto, mãos, cabelo e sobrancelhas, vestindo camiseta,
  macacão de trabalho e sapato fechado. O homem e a mulher vêm de macros diferentes do
  MakeHuman (sexo, idade, músculo, peso, proporções), então mudam no corpo (ombros, cintura,
  quadril, busto) e no rosto. O cabelo é curto nele e um rabo de cavalo nela. A cor vem só dos
  materiais, sem textura: `pele`, `labio`, `olho`, `olho_branco`, `cabelo` (cabelo e
  sobrancelhas), `camisa`, `calca` (o macacão), `fivela`, `calcado` e `sola`;
- `epi_<nome>`: uma malha por EPI do modelo (`capacete`, `oculos`, `protetor_auricular`,
  `luvas`, `botina`, `colete`, `mascara`, `avental`). O 3D mostra só as do EPI que a pessoa
  veste (o `ppe` do nó dela). Cada EPI é feito sobre o corpo vestido: raios de fora para
  dentro acham a pele ou a roupa, e a peça fica com folga sobre ela. O capacete fica acima da
  testa, as lentes dos óculos são transparentes e a máscara cobre só o nariz e a boca, então
  os olhos aparecem mesmo com tudo vestido. Os dois elásticos da máscara saem da borda da
  concha inclinados: o de cima sobe por cima das orelhas, o de baixo desce para a nuca, e os
  dois passam por fora do cabelo. As luvas são a mão do corpo com folga. A botina é
  o próprio sapato em couro, um pouco maior, para cobri-lo em qualquer pose. A saia do avental
  segue a bacia e as duas coxas, com a parte de cada coxa variando de um lado ao outro, para
  não rasgar no meio ao andar.

### Do MPFB ao esqueleto do UAL1

O rig `game_engine` do MPFB tem os mesmos nomes de ossos do UAL1 (só `head` vira `Head`), mas
as juntas ficam em outros lugares: o UAL1 tem o tronco um pouco inclinado para trás, o ombro
mais alto e a coluna dividida de outro jeito. Como as animações do UAL trazem a posição de
todos os ossos, o corpo é que se encaixa nas juntas do UAL1:

- o tronco (bacia, coluna e clavículas) vai inteiro, girado e escalado, para que o meio do
  quadril e o meio dos ombros caiam nos do UAL1;
- o pescoço e a cabeça vão com o tronco, com as proporções do corpo. No UAL1, a junta da
  cabeça fica uns 10 cm mais baixa e 8 cm mais à frente em relação aos ombros, porque o boneco
  tem o pescoço curto. Levar a cabeça até essa junta esmagava o pescoço (de 22 cm entre o ombro
  e a cabeça para 13 cm) e afundava a cabeça nos ombros. As juntas do UAL1 ficam então mais
  baixas dentro do pescoço, e só o centro do giro muda;
- braços e pernas giram até a direção dos ossos do UAL1 (de braços abertos) e os ossos longos
  esticam até o comprimento deles; mãos e pés só giram, e o pé desce até a sola tocar o chão;
- cada falange dos dedos vai para a direção e o comprimento da falange do UAL1. Sem isso, os
  clipes fecham a mão em garra;
- a perna da calça é estreitada do joelho até a barra (a do pacote abre em boca de sino).

Cada vértice vai com a média das matrizes dos seus ossos, com os pesos do próprio MPFB, que
continuam no arquivo. Depois, cada parte é reduzida (Decimate simétrico) para caber no
orçamento do 3D em tempo real: cerca de 19 mil triângulos e 1,2 MB por pessoa, com os oito
EPIs.

### Postura: a compensação dos clipes

Os clipes do UAL1 foram feitos para um boneco de jogo. Em relação ao repouso, o `Idle_Loop` e o
`Walk_Loop` dobram o tórax uns 12° para a frente e baixam a cabeça uns 13°, com a lombar uns 5°
para trás. Num corpo humano isso aparece como corcunda. A correção fica na malha, e o esqueleto
e os clipes não mudam:

- em repouso, a malha é dobrada ao contrário em cada junta da coluna (`POSTURA` em
  `pessoas.py`): `spine_01` 3° para a frente, `spine_02` 5° e `spine_03` 5° para trás,
  `neck_01` 2° e `Head` 3° para a frente;
- a dobra usa os pesos da pele, então fica suave, e os braços e a cabeça vão com o tórax;
- tocado o clipe, as duas dobras se anulam e a pessoa fica ereta.

Por isso a pessoa em repouso (pose T, que o 3D nunca mostra) aparece inclinada para trás. As
medidas do catálogo (`tamanho`, `altura_base`) são as do corpo ereto, antes da compensação.
Com o pescoço inteiro, o homem do arquivo tem 1,86 m e a mulher, 1,84 m (`altura_base`); a
escala por pessoa leva cada um à sua altura.

Sentado (`Driving_Loop`, `Sitting_Idle_Loop`), o clipe já reclina o tronco uns 18°, e a
compensação soma uns 7°. Isso não se corrige na malha sem piorar a pessoa em pé. O catálogo traz
`dados.sentado_inclinar` (15°): a ponte pode inclinar a pessoa sentada para a frente por esse
ângulo, em torno do quadril.
O teste `a_postura_de_repouso_compensa_a_curva_dos_clipes` confere a compensação.

Para conferir, as prévias ficam em `.claude/previa-3d/operador-postura/` (corpo inteiro) e
`.claude/previa-3d/operador-pescoco/` (pescoço de perto, de perfil e de 3/4).

Variação por pessoa, em `dados` no catálogo, sorteada pelo 3D:

| Campo | O que é |
|---|---|
| `recolorir` | materiais que o 3D recolore |
| `peles`, `cabelos` | tons sugeridos (sRGB) |
| `alturas` | faixa de altura do sexo, em metros |
| `altura_base` | altura do arquivo, do chão ao topo da cabeça; a escala é altura / `altura_base` |
| `imc_base` | IMC (kg/m²) estimado do corpo do arquivo |
| `largura` | faixa do fator de largura, `[mín, máx]` |
| `sentado_inclinar` | graus para a frente que a ponte pode dar à pessoa sentada (ver "Postura") |

Escala por pessoa. Quando o modelo traz a altura e o peso da pessoa, a ponte usa os dois.
Sem eles, a altura é sorteada pelo nome dentro de `alturas` e a largura fica 1.

- Vertical (eixo da altura): altura / `altura_base`.
- Largura e profundidade: a escala vertical vezes sqrt(IMC / `imc_base`), com o IMC = peso /
  altura² e a raiz limitada à faixa `largura`.

A escala vai no nó raiz da pessoa, então o esqueleto, os clipes do UAL1 e os EPIs acompanham.

O teste `as_pessoas_tem_corpo_epis_e_o_esqueleto_do_ual1` confere quatro coisas:

- os ossos e a pose de repouso batem com os do UAL1;
- há um nó por EPI;
- todo vértice tem peso;
- o corpo tem os materiais recoloríveis, olhos e boca;
- cada dedo tem malha presa ao próprio osso.

O teste `o_homem_e_a_mulher_tem_corpos_diferentes` confere que o quadril dela é mais largo e
que a cintura e o peito dela são mais estreitos que os dele.

## Clipes de pessoa corrigidos (`pessoa_clipes.glb`)

Os clipes das pessoas vêm da biblioteca de animações CC0 em `assets/terceiros`. Alguns foram
feitos para um boneco de jogo e ficam errados num corpo humano. `tools/blender/clipes.py` lê o
clipe original, corrige a pose quadro a quadro e grava `pessoa_clipes.glb`: só o esqueleto e
os clipes corrigidos, com os mesmos nomes de nó e de clipe.

- `Walk_Carry_Loop` (andar com caixa, e os braços de quem está parado com caixa): no original o
  tronco deita uns 25° para trás e a cabeça olha para o teto. O corrigido dobra a coluna, o
  pescoço e a cabeça para a frente; a caixa fica na frente da barriga.
- No catálogo, `pessoa_clipes.clipes` lista os clipes do arquivo e `dados.substitui` diz quais
  clipes originais ele troca. Quem toca um clipe de pessoa usa o deste arquivo quando o nome
  está na lista, e o original nos outros casos.

## Motorista e operador

- **Empilhadeira:** `ponto_operador` é o quadril do motorista sentado (0,09 m acima do
  assento), em (-0,34; 0; 1,07): o mesmo assento que a ponte do 3D usa para a empilhadeira
  (`FORKLIFT_HIP`). O piso da cabine fica 0,61 m abaixo dele, entre as soleiras, e o volante
  fica onde as mãos do clipe `Driving_Loop` seguram (0,54 m à frente e 0,28 m acima do
  quadril, com a inclinação `sentado_inclinar`).
- **Torno e injetora:** `ponto_operador` fica na frente do painel. A mão esquerda do clipe
  `Interact` (0,17 m à esquerda e 0,58 m à frente do ponto, a 1,3 m do chão) cai na tela.
  `ponto_porta` continua 0,6 m além. Na prensa o operador fica no centro, de frente para a
  mesa, porque o painel fica junto da saída da peça.

## Ponte rolante (`ponte_rolante.glb`)

Ponte rolante birrail sobre caminho de rolamento em colunas (`tools/blender/elementos/ponte.py`).
Origem no chão, no centro. `+x` é o caminho de rolamento, `y` é o vão e `z` é a altura.
Tamanho base: caminho de 12 m, vão de 8 m entre trilhos, trilho a 6 m do chão.

Em repouso a ponte e o carro ficam no começo do curso (`-curso_x/2` e `-curso_y/2`) e o gancho
fica recolhido.

| Nó | Pai | O que é | Com outro tamanho |
|---|---|---|---|
| `trilho_e`, `trilho_d` | raiz | viga de rolamento com trilho, em `y = ±vao/2`; origem no centro, no topo do trilho | escala em x = comprimento / `comprimento`; posição y e z |
| `coluna_e0..2`, `coluna_d0..2` | raiz | colunas sob a viga (pontas e meio), com o pé amarelo e preto; origem no chão | escala em z = altura do trilho / `altura_trilho`; posição x e y |
| `ponte` | raiz | a ponte: anda em x; origem no eixo dela, na altura do trilho | posição z |
| `viga_a`, `viga_b` | `ponte` | as duas vigas-caixão amarelas, com o trilho do carro em cima | escala em y = vão / `vao` |
| `cabeceira_e`, `cabeceira_d` | `ponte` | cabeceiras com rodas, para-choques e faixa amarela e preta | posição y = ±vão/2 |
| `luz` | `ponte` | luz de aviso (emissiva) | posição y |
| `cabine` | `ponte` | cabine do operador pendurada sob a ponte; opcional, pode ser escondida | posição y |
| `cabo_botoeira` | `ponte` | cabo da botoeira; origem em cima | escala em z |
| `botoeira` | `ponte` | botoeira pendente, a `botoeira_altura` do chão | posição z |
| `carro` | `ponte` | o trole: anda em y sobre as vigas; tambor, motor e redutor | — |
| `cabo` | `carro` | cabo do tambor ao moitão; origem em cima, comprimento `cabo_base` em repouso | escala em z = comprimento do cabo / `cabo_base` |
| `gancho` | `carro` | moitão com gancho, amarelo e preto: desce em z; origem no eixo das polias | — |

Pontos:

- `ponto_gancho` (pai `gancho`): o fundo da sela do gancho. Serve para pegar e para soltar; a
  carga fica pendurada abaixo dele.
- `ponto_operador` (pai `ponte`, no chão): onde fica quem opera pela botoeira; anda com a ponte.

Clipes, fase de 0 a 1, no tamanho base:

- `ponte_andar`: `ponte` anda `curso_x` em x.
- `carro_andar`: `carro` anda `curso_y` em y.
- `gancho_descer`: `gancho` desce `curso_gancho`, e `cabo` estica por escala em z.

`dados`: `comprimento`, `vao`, `altura_trilho`, `curso_x`, `curso_y`, `margem_x`, `margem_y`
(metade do que o curso é menor que o tamanho), `curso_gancho`, `gancho_repouso` (altura de
`ponto_gancho` recolhido), `cabo_base`, `cabo_botoeira_base`, `botoeira_altura` e `estica` (os
nós que esticam e o eixo de cada um: `x`, `y` ou `z`).

Para outro vão, curso ou altura, quem desenha não escala a raiz (as vigas e as rodas
deformariam): escala só os nós da tabela e põe os outros no lugar, a partir de `dados`. Com a
ponte em `x`, o carro em `y` e o gancho na altura `h`, `ponte` vai para `x`, `carro` para `y`
(dentro de `ponte`), `gancho` desce `gancho_repouso - h`, e o `cabo` ganha esse mesmo comprimento.

O teste `a_ponte_rolante_anda_nos_tres_eixos` confere os três movimentos e os cursos.

## Peças da esteira por trajeto

A esteira desenhada por trajeto é feita na cena, ao longo da linha
(`godot/scripts/trajeto_malha.gd`). Três peças pequenas completam o desenho, para repetir ao
longo dela (`tools/blender/elementos/trajeto.py`). Em todas, a origem é o ponto de apoio, `+x`
vai ao longo da esteira e `z` para cima. As medidas seguem as da malha: a linha é o topo da
superfície, a longarina vai de 0,14 m abaixo a 0,02 m acima da linha e tem 0,045 m de
espessura, por fora da largura.

| Arquivo | Origem | Nós | Como usar |
|---|---|---|---|
| `esteira_pe` | no chão, no centro da sapata | `sapata` (disco de borracha, haste roscada e porca; fixa, `dados.sapata_altura` = 0,06) e `perna` (tubo de `dados.lado` = 0,05; origem na base dela; comprimento `dados.perna_base` = 1,0) | uma perna por arquivo, sob o meio da longarina. Escala de `perna` em z = (altura da base da longarina − `sapata_altura`) / `perna_base` |
| `esteira_ponteira` | na linha, no fim da esteira | `tambor` (ao longo de y, raio `dados.raio` = 0,07, comprimento `dados.largura_base` = 1,0; o topo fica na linha) e `mancal_e`, `mancal_d` (origem no eixo do tambor, em `y = ±0,5`) | escala de `tambor` em y = largura / `largura_base`; os mancais vão para `y = ±largura/2` e cobrem a ponta da longarina. Clipe `rolar` (laço): o tambor dá uma volta |
| `esteira_canaleta` | no topo da longarina, na face de fora, do lado direito do sentido (`-y`) | `suporte` (sapata aparafusada na longarina, haste por fora da guia e gancho sobre o topo dela) | tamanho fixo; do lado esquerdo, gira 180°. `dados.altura` = 0,14 (o topo da guia acima do topo da longarina), `guia_fora` = 0,015 e `guia_espessura` = 0,035 |

`dados.estica` diz o nó que estica e o eixo (`perna` em z, `tambor` em y). A ponteira e o
suporte não ficam no piso: o `minimo` deles é negativo em z.

O teste `as_pecas_da_esteira_por_trajeto_tem_a_origem_no_apoio` confere as origens e as medidas.

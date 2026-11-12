# Fichas dos modelos 3D

Uma ficha por item, com as medidas, a pose, as peças móveis, o limite de triângulos e o nome do
arquivo. As fichas valem como especificação de cada elemento.

**Situação em 2026-11-11:** o dono decidiu não usar o gerador 3D externo, porque o plano gratuito
é público e não comercial. Os modelos continuam feitos por script no Blender
(`docs/elementos_3d_blender.md`). Os prompts e o "Caminho de volta" ficam guardados para o caso de
um gerador com licença adequada ser usado no futuro.

## Por que o operador ficou corcunda

A pose de repouso do corpo era ereta. A corcunda aparecia ao tocar os clipes do UAL1: medidos
contra o repouso, o `Idle` e o `Walk` dobram o tórax uns 12° para a frente e baixam a cabeça uns
13°. Isso já foi corrigido no corpo atual, com uma dobra contrária na malha de repouso (veja
"Postura" em `docs/elementos_3d_blender.md`). Qualquer corpo novo precisa da mesma compensação e
da conferência de perfil em `Idle`, `Walk` e `Interact`.

## Regras comuns a todos os itens

- **Estilo** (já vai no fim de cada prompt): industrial realista, malha low-poly limpa,
  materiais PBR, luz neutra e uniforme, sem luz nem sombra pintadas na textura, sem texto e sem
  logotipo, um objeto só, fundo liso.
- **Unidades e eixos na entrega:** metros, Y para cima, frente para +Z (o padrão dos
  geradores). Se sair de outro jeito, tudo bem: no encaixe eu giro para a convenção do UTSim
  (frente em +X, Y para cima), ponho a origem no chão, no centro da base, e acerto a escala
  pelas medidas da ficha.
- **Formato:** GLB com as texturas embutidas (cor, normal e metal/rugosidade), em 1024 px.
  Animação de exemplo, se o gerador puser, eu descarto.
- **Opções do gerador:**
  - Textura: sim. PBR: sim.
  - Redução de malha ou retopologia: sim, com o limite de triângulos da ficha.
  - Quad: não precisa, porque o GLB vira triângulos de qualquer jeito. Ligue só se a malha
    sair mais limpa.
  - Auto-rig: só nas pessoas (humanoide). Nas máquinas, não.
- **Cores:** lisas, sem estampa. Assim cada região vira um material que o 3D pode recolorir
  (camisa, calça, pele e cabelo nas pessoas; `pintura` nas máquinas).
- **Peças móveis:** o gerador costuma entregar uma malha só. Por isso, cada ficha diz o que
  gerar à parte (um GLB por peça) e o que eu recorto ou faço por script no Blender.
- **Imagem de referência (opcional):** se o texto não der certo, gere antes uma imagem de
  frente com o mesmo prompt e use a função de imagem para 3D.

---

## 1. Pessoa: homem

- **Arquivo:** `pessoa_homem.glb`
- **Medidas:** 1,76 m de altura, do chão ao topo da cabeça. Na A-pose, cerca de 1,4 m de mão a
  mão e 0,30 m de profundidade.
- **Pose:** A-pose, com os braços retos a cerca de 45° do corpo e as palmas viradas para as
  coxas. Mãos abertas, com os dedos levemente separados. Pés afastados na largura dos ombros,
  apontando para a frente. Costas retas, ombros soltos e para trás, cabeça nivelada olhando em
  frente, rosto neutro e boca fechada.
- **EPI:** sem. Os EPIs (capacete, óculos, protetor auricular, máscara, luvas, colete, avental
  e botina) são vestidos pelo UTSim, um nó por EPI, que a simulação liga e desliga. Por isso o
  cabelo deve ser curto e rente à cabeça, para o capacete assentar.
- **Roupa:** camisa de trabalho azul-marinho de manga comprida, para dentro da calça; calça de
  trabalho azul-marinho com cinto; sapato de segurança preto, de couro.
- **Peças móveis:** nenhuma. O corpo anda pelo esqueleto do UAL1.
- **Limite:** 20 000 triângulos; uma textura de 1024 px (2048 px só se o rosto ficar ruim).
- **Gerador:** auto-rig sim (humanoide, sem animação), retopologia sim, textura sim.

Prompt:

```text
Full-body 3D character of an adult male factory worker, 1.76 m tall, average build, realistic
human proportions (7.5 heads tall). Standing upright in an A-pose: arms straight at 45 degrees
from the body, palms facing the thighs, hands open with fingers slightly apart, feet
shoulder-width apart pointing forward, straight back, shoulders relaxed and pulled back, head
level looking straight ahead, neutral face with closed mouth. Wearing a navy blue long-sleeve
work shirt tucked into navy blue work trousers with a black belt, and black leather safety work
shoes. Short hair close to the head. No helmet, no glasses, no gloves, no vest, no mask, nothing
in the hands. Realistic industrial style, clean low-poly topology, PBR materials, solid uniform
colors without patterns, even neutral lighting, no baked lighting or shadows, no text or logos,
single character, plain background.
```

## 2. Pessoa: mulher

- **Arquivo:** `pessoa_mulher.glb`
- **Medidas:** 1,65 m de altura. Na A-pose, cerca de 1,3 m de mão a mão.
- **Pose, EPI e opções:** iguais às do homem.
- **Roupa:** a mesma do homem. O cabelo fica preso num coque baixo, rente à cabeça, para o
  capacete assentar.
- **Limite:** 20 000 triângulos, textura de 1024 px.

Prompt:

```text
Full-body 3D character of an adult female factory worker, 1.65 m tall, average build, realistic
human proportions (7.5 heads tall). Standing upright in an A-pose: arms straight at 45 degrees
from the body, palms facing the thighs, hands open with fingers slightly apart, feet
shoulder-width apart pointing forward, straight back, shoulders relaxed and pulled back, head
level looking straight ahead, neutral face with closed mouth. Wearing a navy blue long-sleeve
work shirt tucked into navy blue work trousers with a black belt, and black leather safety work
shoes. Hair tied in a low bun close to the head. No helmet, no glasses, no gloves, no vest, no
mask, nothing in the hands. Realistic industrial style, clean low-poly topology, PBR materials,
solid uniform colors without patterns, even neutral lighting, no baked lighting or shadows, no
text or logos, single character, plain background.
```

## 3. Empilhadeira

- **Arquivos:** `empilhadeira.glb` (corpo) e `empilhadeira_garfo.glb` (carro com os garfos).
- **Medidas do corpo, sem mastro:**
  - 2,30 m de comprimento, do contrapeso até a frente das rodas;
  - 1,16 m de largura;
  - 2,25 m de altura até o teto de proteção;
  - rodas da frente com 0,66 m de diâmetro, de trás com 0,50 m, e 1,40 m entre os eixos.
- **Peças móveis:**
  - as quatro rodas: eu recorto do corpo pela posição. Se o recorte ficar ruim, uso as rodas do
    script;
  - o mastro de três estágios, que sobe 6 m: fica por script (perfis de aço), porque precisa
    deslizar em estágios;
  - o carro com os garfos: o arquivo à parte, com 1,0 m de largura, encosto de carga de 1,2 m
    de altura e garfos de 1,15 m de comprimento.
- **Limite:** corpo até 12 000 triângulos, garfo até 2 000; textura de 1024 px.
- **Gerador:** sem auto-rig.

Prompt do corpo:

```text
3D model of a counterbalance forklift truck WITHOUT mast and WITHOUT forks, seen as the base
vehicle only: yellow body, dark grey rounded counterweight at the rear, operator seat, steering
wheel, black overhead safety guard made of steel tubes, four black solid rubber tires (larger at
the front), small LPG gas cylinder lying behind the seat, headlights. 2.3 m long, 1.16 m wide,
2.25 m tall to the top of the overhead guard. The front face is flat where the mast would mount.
Realistic industrial style, clean low-poly topology, PBR materials, solid uniform colors without
patterns, even neutral lighting, no baked lighting or shadows, no text or logos, single object,
plain background.
```

Prompt do garfo:

```text
3D model of a forklift fork carriage: a flat steel carriage plate 1.0 m wide with a tall load
backrest grille 1.2 m high, and two L-shaped steel forks 1.15 m long, 0.10 m wide, pointing
forward, matte black paint. Realistic industrial style, clean low-poly topology, PBR materials,
solid uniform colors, even neutral lighting, no baked lighting or shadows, no text or logos,
single object, plain background.
```

## 4. AGV

- **Arquivo:** `agv.glb` (corpo).
- **Medidas:** 1,14 m × 0,80 m, com 0,35 m de altura até o topo da mesa.
- **Peças móveis:** a mesa elevatória (sobe 6 cm) e as duas rodas de tração (quase escondidas)
  ficam por script. O corpo pode vir com a mesa no lugar: eu recorto a mesa por altura.
- **Limite:** 4 000 triângulos; textura de 1024 px.
- **Gerador:** sem auto-rig.

Prompt:

```text
3D model of a low-profile automated guided vehicle (AGV) for factories: rectangular body with
rounded corners, light grey and white housing, black rubber bumper all around the bottom, flat
dark grey lifting table on top with an anti-slip rubber surface, red emergency stop buttons on
the top corners, safety laser scanners on two diagonal corners, thin blue LED light strip along
the sides, small status display at the rear. 1.14 m long, 0.80 m wide, 0.35 m tall. Realistic
industrial style, clean low-poly topology, PBR materials, solid uniform colors without patterns,
even neutral lighting, no baked lighting or shadows, no text or logos, single object, plain
background.
```

## 5. Robô

- **Arquivo:** `robo.glb`.
- **Pose:** a de repouso do UTSim. O braço de cima fica vertical, o antebraço na horizontal
  para a frente e a garra apontando para baixo. Assim eu recorto as juntas por planos.
- **Medidas:**
  - pedestal de 0,50 m, com a base de 0,70 m × 0,70 m;
  - ombro a 0,78 m do chão, braço de 0,70 m, antebraço de 0,65 m e garra de 0,27 m;
  - 1,60 m de altura total em repouso e alcance de 1,0 m.
- **Peças móveis:**
  - recorto por planos: `base` (gira no eixo vertical), `braco` (ombro), `antebraco`
    (cotovelo) e `garra` (punho);
  - os dois dedos ficam por script. Eles precisam abrir 20 cm e fechar em 13 cm, na peça de
    0,12 m.
- **Limite:** 10 000 triângulos; textura de 1024 px.
- **Gerador:** sem auto-rig.

Prompt:

```text
3D model of a 6-axis industrial robot arm on a square steel pedestal, orange paint with dark
grey motors and joints. Pose: upper arm pointing straight up, forearm horizontal pointing
forward, wrist and two-finger parallel gripper pointing straight down. Pedestal 0.5 m tall,
shoulder joint at 0.78 m, upper arm 0.70 m, forearm 0.65 m, gripper 0.27 m, total height about
1.6 m. Visible servo motors at the base, shoulder and elbow, round joint housings. Realistic
industrial style, clean low-poly topology, PBR materials, solid uniform colors without patterns,
even neutral lighting, no baked lighting or shadows, no text or logos, single object, plain
background.
```

## 6. Esteira

- **Arquivo:** `esteira.glb` (estrutura sem os rolos).
- **Medidas:** 3,0 m de comprimento, 0,80 m de largura e topo dos rolos a 0,76 m. Pernas a
  cada 1,5 m, guias laterais e motor embaixo, perto da saída.
- **Peças móveis:** os rolos, um a cada 0,15 m, com 0,06 m de diâmetro. Ficam por script,
  porque cada um gira e o comprimento da esteira muda por parâmetro. Por isso a estrutura vem
  sem rolos.
- **Limite:** 5 000 triângulos; textura de 1024 px.
- **Gerador:** sem auto-rig.

Prompt:

```text
3D model of the steel frame of a straight roller conveyor WITHOUT the rollers: two parallel
blue painted side channels 3.0 m long and 0.80 m apart at a height of 0.76 m, grey steel legs
with floor feet every 1.5 m, cross braces, low side guide rails on small brackets, and an
electric drive motor with gearbox mounted under the frame near one end. The space between the
side channels is empty. Realistic industrial style, clean low-poly topology, PBR materials,
solid uniform colors without patterns, even neutral lighting, no baked lighting or shadows, no
text or logos, single object, plain background.
```

## 7. Máquina (centro de usinagem CNC)

- **Arquivos:** `cnc.glb` (máquina com a porta fechada) e `cnc_porta.glb` (a porta).
- **Medidas:** 2,0 m de largura, 1,84 m de profundidade e 2,30 m de altura até o topo da torre
  de luz. A porta fica no lado de trabalho (a frente): corre para o lado, com 0,8 m de
  curso, e tem 0,9 m × 1,0 m, com visor.
- **Peças móveis:**
  - a porta: o arquivo à parte, que eu ponho no trilho;
  - a torre de luz: eu recorto e recoloro como a `luz` de estado;
  - a tampa de manutenção e o fuso interno: ficam por script, porque quase não aparecem.
- **Limite:** 12 000 triângulos; porta até 1 500; textura de 1024 px.
- **Gerador:** sem auto-rig.

A prensa e a injetora seguem a mesma ficha depois, trocando o prompt.

Prompt da máquina:

```text
3D model of an enclosed CNC vertical machining center: boxy light grey sheet metal enclosure
with dark grey trim, a large sliding front door with a rectangular safety glass window (door
closed), an operator control panel with screen on a swing arm at the front right, a coolant
tank at the base, a three-color signal tower light (red, yellow, green) on the top corner.
2.0 m wide, 1.84 m deep, 2.3 m tall to the top of the signal tower. Realistic industrial style,
clean low-poly topology, PBR materials, solid uniform colors without patterns, even neutral
lighting, no baked lighting or shadows, no text or logos, single object, plain background.
```

Prompt da porta:

```text
3D model of a sliding machine door panel: 0.9 m wide, 1.0 m tall, light grey sheet metal frame
with a large rectangular safety glass window and a vertical black handle. Realistic industrial
style, clean low-poly topology, PBR materials, solid uniform colors, even neutral lighting, no
baked lighting or shadows, no text or logos, single object, plain background.
```

## 8. Rack (porta-paletes)

- **Arquivo:** `porta_paletes_vao.glb` (um vão; eu repito para formar o rack).
- **Medidas:** um vão de 1,40 m de largura entre colunas e 1,14 m de profundidade. Níveis das
  longarinas a 1,58 m, 3,08 m, 4,58 m e 6,08 m; o primeiro nível é o chão.
- **Peças móveis:** nenhuma. Os pontos de cada vão e nível (`ponto_vao_<b>_<n>`) eu ponho no
  encaixe.
- **Limite:** 3 000 triângulos; textura de 1024 px.
- **Gerador:** sem auto-rig.

Prompt:

```text
3D model of ONE bay of an industrial selective pallet rack: two blue upright frames with
diagonal bracing, 1.14 m deep and 6.2 m tall, 1.40 m apart, connected by orange box beams at
heights 1.58 m, 3.08 m, 4.58 m and 6.08 m, with steel base plates and yellow column protectors
at the floor. Empty, no pallets. Realistic industrial style, clean low-poly topology, PBR
materials, solid uniform colors without patterns, even neutral lighting, no baked lighting or
shadows, no text or logos, single object, plain background.
```

## 9. Espera (mesa de espera, fila ou buffer)

- **Arquivo:** `espera.glb`.
- **Medidas:** 1,20 m × 0,80 m, com o tampo a 0,76 m (a altura da esteira, para a peça passar
  de uma à outra).
- **Peças móveis:** nenhuma. A faixa amarela no chão em volta e os pontos de entrada e saída eu
  faço no encaixe.
- **Limite:** 3 000 triângulos; textura de 1024 px.
- **Gerador:** sem auto-rig.

Prompt:

```text
3D model of an industrial buffer table for parts waiting between machines: sturdy grey steel
frame table 1.2 m long, 0.8 m wide, top surface at 0.76 m, with a light grey steel top, low side
rails along the long edges, a lower shelf, and four leveling feet. Empty. Realistic industrial
style, clean low-poly topology, PBR materials, solid uniform colors without patterns, even
neutral lighting, no baked lighting or shadows, no text or logos, single object, plain
background.
```

---

## Caminho de volta

1. **Onde salvar.** Grave cada GLB com o nome da ficha em
   `~/Documents/Codes/UTSim-origem-3d/gerados/`, fora do repositório, como as outras fontes 3D.
   Anote ao lado qual plano do gerador foi usado: a licença do modelo depende do plano, e ela
   vai para o `assets/modelos3d/LEIA-ME.md` antes de qualquer arquivo entrar no git.
2. **Pessoas.** Escala pela altura da ficha e origem no chão. Depois, o retarget para o
   esqueleto UAL1:
   - os ossos do auto-rig são renomeados para os do UAL1, ou o UAL1 é posto dentro do corpo e
     os pesos são transferidos;
   - o tronco dobra por `spine_01` a `spine_03` com pesos suaves, para não repetir a corcunda;
   - os clipes atuais (`Idle`, `Walk`, `Interact` e os outros) tocam sem mudança no 3D;
   - as cores viram os materiais `camisa`, `calca`, `pele` e `cabelo`, que o 3D recolore;
   - os oito EPIs são feitos de novo sobre o corpo novo (`epi_*`).
3. **Máquinas e veículos.**
   - Giro, escala e origem pelas medidas da ficha.
   - Recorte das peças móveis e junção com as que ficam por script.
   - Pivôs nas juntas, com os nomes de nós do catálogo (`carro`, `garfos`, `mastro_m`,
     `mastro_i`, `mesa`, `roda_*`, `base`, `braco`, `antebraco`, `garra`, `dedo_*`, `porta`,
     `luz`, `rolo_*`).
   - Os pontos (`ponto_operador`, `ponto_porta`, `ponto_peca`, `ponto_entrada`,
     `ponto_saida`, `ponto_garra`, `ponto_carga`, `ponto_vao_*`) e os clipes atuais.
4. **Testes.** `crates/utsim-assets/tests/modelos3d.rs` confere:
   - os nós, os pontos e os clipes do contrato;
   - a garra sobre a peça, o garfo até o vão mais alto e a porta aberta quando a peça passa;
   - o esqueleto e os pesos das pessoas.
   Os limites de tamanho de arquivo sobem, porque as texturas pesam mais que as cores lisas.
   O limite de triângulos continua o da ficha.
5. **Publicação.**
   - O `catalogo.json` e o `docs/elementos_3d_blender.md` são atualizados.
   - Faço a verificação completa e o push.
   - A Interface é avisada. Para ela não muda nada, porque o contrato (nós, pontos, clipes e
     estados) é o mesmo.

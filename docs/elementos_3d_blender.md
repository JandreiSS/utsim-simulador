# Elementos 3D gerados no Blender

Os elementos da fábrica (máquinas, posto, robô, célula, esteira, veículos, estantes e
cargas) são gerados por scripts Python no Blender, sem janela, e versionados em
`assets/modelos3d/`. Os modelos em Rust de `assets/modelos/` (`utsim-assets`) continuam
valendo como reserva e para os tamanhos por parâmetro.

## Como gerar

```bash
bash scripts/blender.sh            # todos
bash scripts/blender.sh --so cnc   # só alguns
```

O script acha o Blender (`BLENDER`, `PATH` ou a pasta padrão do Windows) e roda
`blender -b --factory-startup -P tools/blender/gerar.py -- --saida assets/modelos3d`.
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
| `ponto_porta` | por onde a pessoa chega (na célula, fora do portão) |
| `ponto_peca` | onde a peça fica durante o processo |
| `ponto_entrada`, `ponto_saida` | onde a peça chega e de onde sai (ligam um elemento ao próximo) |
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
| posto | `ciclo` | sim | processando |
| | `setup` | sim | em setup (o grampo abre todo e volta) |
| robo | `pegar`, `levar`, `soltar` | não | carregar, transportar e descarregar a peça |
| | `ciclo` | sim | os três seguidos, para um robô sempre ocupado |
| | `setup` | sim | em setup (a base gira e os dedos abrem e fecham) |
| | `quebra` | não | quebrado (o braço cai e fica parado) |
| celula | `porta_abrir` | não | posto da célula em `entering` (0 → 1) e em `leaving` (1 → 0) |
| esteira | `rolar` | sim | fase = distância / (2π · `raio_rolo`) |
| empilhadeira | `garfo_subir` | não | altura do garfo / `garfo_curso` |
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
| `setup` | `setup` | 0 → 1 | sim | |
| `down` | `quebra` | 0 → 1 | não | |
| `waiting_repair`, `maintenance` | `quebra` | 1 → 1 | não | |

A fase do clipe é `de + (ate - de) * fase`, com a fase do estado de 0 a 1. Num clipe em
laço, uma volta cabe no estado: uma peça, um ciclo. Só entram os estados cujo clipe o
elemento tem; a célula, por exemplo, só tem `entering` e `leaving`. O caminho
`operador` só aparece quando o elemento tem os dois pontos. Sem ele, a ponte põe a pessoa
de frente para a estação (`station_front`). Os pontos giram com a estação.

### Interações

A peça passa de um elemento a outro pelos pontos:

- da esteira, sai em `ponto_saida`;
- na garra, vai presa em `ponto_garra` entre `pegar` e `soltar`;
- na máquina, entra em `ponto_entrada` e fica em `ponto_peca`.

O palete vai no `ponto_carga` do garfo e fica no `ponto_vao_*` do porta-paletes. O
operador anda até `ponto_porta`, abre a porta, vai a `ponto_operador`, opera e sai pelo
mesmo caminho. Fazer a peça, o palete e a pessoa seguirem esses pontos no 3D é da ponte
e da interface (próximas etapas).

## Pessoas

`tools/blender/pessoas.py` (chamado pelo `gerar.py`) gera `pessoa_homem.glb` e
`pessoa_mulher.glb`. Elas são feitas sem nada baixado: o corpo é de metaball, por famílias
(tronco, cada braço, cada perna), convertido em malha leve. Os arquivos têm:

- o esqueleto do boneco livre UAL1 (`assets/terceiros/quaternius_ual`), com os mesmos ossos
  e a mesma pose de repouso, e sem animações próprias. O 3D toca nelas as animações do UAL1
  e do UAL2 (andar, operar, carregar, pegar, interagir), como faz hoje no boneco;
- `corpo`: corpo liso com rosto simples (olhos, sobrancelhas, nariz, orelhas e boca),
  cabelo, camisa de manga curta, calça e calçado. A cor vem dos materiais `pele`, `cabelo`,
  `camisa`, `calca`, `calcado`, `olho` e `labio`, sem textura;
- `epi_<nome>`: uma malha por EPI do modelo (`capacete`, `oculos`, `protetor_auricular`,
  `luvas`, `botina`, `colete`, `mascara`, `avental`). O 3D mostra só as do EPI que a pessoa
  veste (o `ppe` do nó dela).

Os pesos de pele vêm do boneco UAL1, pelo ponto mais próximo. Há dois cuidados:

- cada família só pode seguir os próprios ossos (o tronco não segue o braço);
- perto do ombro, o peso passa aos poucos da clavícula para o braço.

As peças da cabeça, o capacete, os óculos, o protetor e a máscara ficam presos ao osso
`Head`, e as luvas à mão.

Variação por pessoa, em `dados` no catálogo, sorteada pelo 3D:

| Campo | O que é |
|---|---|
| `recolorir` | materiais que o 3D recolore |
| `peles`, `cabelos` | tons sugeridos (sRGB) |
| `alturas` | faixa de altura do sexo, em metros |
| `altura_base` | altura do esqueleto (1,82 m); a escala é altura / `altura_base` |

O teste `as_pessoas_tem_corpo_epis_e_o_esqueleto_do_ual1` confere quatro coisas:

- os ossos e a pose de repouso batem com os do UAL1;
- há um nó por EPI;
- todo vértice tem peso;
- o corpo tem os materiais recoloríveis.


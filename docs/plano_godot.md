# Plano da interface Godot (versão 0.2)

A interface 3D do UTSim passa a ser feita no Godot 4.7, que também leva a realidade virtual e a
realidade aumentada (OpenXR). O editor e visualizador Bevy (`utsim-viewer`) continua sendo o da
versão 0.1 até a interface Godot cobrir o que ele faz. A arquitetura está em
`docs/arquitetura.md`, seção 4r.

Princípios:

- O Godot não reimplementa a simulação, a cena nem a edição. Ele chama o Rust pela ponte
  GDExtension (`crates/utsim-godot`), que usa as mesmas crates do resto do programa:
  - `utsim-model` para rodar;
  - `utsim-scene` para a cena;
  - `utsim-edit` para editar com desfazer;
  - `utsim-assets` para os modelos.
- A ponte devolve dados simples (dicionários e arrays empacotados), em coordenadas do Godot
  (`(x, z, -y)`, metros), e o GDScript só desenha e trata a entrada.
- Toda regra que puder ser testada sem o motor fica no Rust, com testes.
- O controle visual é roteirizado: capturas de tela comparadas com as mesmas vistas do Bevy.

## Fases (estimativa em horas Opus 5.5)

| Fase | Conteúdo | Estimativa |
|---|---|---|
| G1 | Ponte e reprodução (**feita**) | 6–8 h |
| G2 | Edição do fluxo | 10–14 h |
| G3 | Rede, fluidos e resultados | 8–12 h |
| G4 | Realidade virtual e aumentada (OpenXR) | 10–16 h |
| G5 | Empacotamento e exportação | 4–6 h |

Total previsto: 38 a 56 h.

## G1: ponte e reprodução (feita)

- **Crate `utsim-godot`:** gdext `godot` 0.5.5 com `api-4-7`, só as classes do núcleo.
  - `UtsimModel`: carrega, valida, roda as replicações, resume e grava o replay.
  - `UtsimPlayback`: cena estática e `frame(t)` em arrays empacotados.
  - Testes de conversão de coordenadas, escolha dos modelos e empacotamento.
- **Projeto `godot/`:**
  - Forward+ e OpenXR desligado;
  - câmera de órbita, grade, sol com sombras;
  - painel de abrir modelo, simular e resultados;
  - reprodução com velocidade, linha do tempo e passo por evento;
  - lista de blocos e rótulos.
- **Modelos da biblioteca por papel e tipo:**
  - empilhadeira com garfos na altura do `lift`;
  - AGV com a luz pelo movimento;
  - rodas girando pela distância;
  - porta-paletes e esteira do tamanho do modelo;
  - máquina, bancada e área da fila.
- **Pessoas animadas** pelo `Pose.motion`, com os clipes e as passadas do Bevy:
  - andar, correr e andar carregando;
  - pegar, trabalhar e parado segurando;
  - a caixa entre as mãos.
- **Itens** como caixas sobre paletes, presos aos garfos, ao deque e às mãos.
- **`scripts/godot.sh`:** compila, copia os `.glb` e importa.
- **`scripts/godot_teste.sh`:** teste sem janela.
- **Capturas roteirizadas** (`godot/roteiros/capturas.json`) dos três exemplos, conferidas
  contra o Bevy.

## G2: edição do fluxo

Objetivo: montar e alterar um modelo no Godot, como no editor Bevy, com desfazer e salvar.

- **Ponte:** classe `UtsimDocument` sobre o `utsim-edit::Document`:
  - abrir, novo e salvar;
  - os comandos (`Command`: adicionar, mover, ligar, remover, `Set` de propriedades, `Batch`)
    com desfazer e refazer;
  - os problemas do modelo a cada mudança;
  - o esquema das propriedades por tipo, para gerar o painel.
- **Interação:**
  - arrastar blocos de uma paleta para o piso;
  - mover com o mouse, com encaixe na grade;
  - ligar por arrasto entre as portas;
  - selecionar e apagar;
  - painel de propriedades gerado do esquema, com validação no Rust;
  - atalhos iguais aos do Bevy (Ctrl+Z, Ctrl+Y, Ctrl+S, Delete, Esc).
- **Modos:** "Editar" (o documento) e "Simular" (o replay), como no Bevy.
- **Aparência:** escolher o `visual.asset` da biblioteca e do catálogo, com miniatura.
- **Antes:** mover as regras puras de aparência (`visual.rs` do Bevy, `looks.rs` da ponte)
  para uma crate sem motor usada pelos dois.
- **Testes:** comandos e desfazer já são testados no `utsim-edit`; aqui, um roteiro que monta
  o tutorial 1 pela interface e compara o `.ron` salvo.
- **Riscos:**
  - seleção e arrasto em 3D precisam de colisores ou de projeção no piso (preferir a
    projeção, sem física);
  - o painel de propriedades gerado precisa cobrir as distribuições e as listas
    (esteiras, porta-paletes, equipes).

## G3: rede, fluidos e resultados

- **Rede de caminhos:**
  - nós e trechos desenhados e editáveis (mão única, dupla, capacidade);
  - pistas controladas em vermelho, como no Bevy.
- **Fluidos:**
  - tanques do `utsim-assets` com o nível do `fill`;
  - tubos entre os nós;
  - cores da taxa.
- **Fluxos de tokens:** painel 2D do grafo, com contagens no tempo.
- **Resultados:**
  - tabela por objeto com intervalos de 95 %;
  - gargalo;
  - gráficos de ocupação, estados, série temporal e histograma;
  - cenários e otimização, usando `utsim-model::analysis` e o `utsim-opt` pela ponte.
- **Replicações:** em uma thread da ponte, com progresso e cancelar (`run_replications_with`).
- **Cena ao vivo:** cliente do socket `utsim-scene/1` para a emulação OPC UA, com os mesmos
  arrays de quadro.
- **Níveis de detalhe e limite de rótulos** para modelos grandes, como no A3D-4.
- **Riscos:** os gráficos no Godot não têm um equivalente pronto do `egui_plot`. Opções:
  - desenhar no `Control` com `draw_*`;
  - gerar SVG ou PNG no Rust.

## G4: realidade virtual e aumentada (OpenXR)

- **SteamVR no PC:**
  - liga `xr/openxr/enabled` e usa um `XROrigin3D` com `XRCamera3D` e controles;
  - escala real (a cena já está em metros, olhos a ~1,6 m);
  - teleporte e andar suave;
  - apontar para um bloco mostra o rótulo e os números;
  - painel de reprodução preso ao controle.
- **Meta Quest (exportação Android):**
  - plugin de fornecedores OpenXR do Godot;
  - Vulkan Mobile;
  - o mesmo projeto com um preset de exportação.
  - A biblioteca da ponte precisa ser compilada para `aarch64-linux-android`
    (`cargo-ndk`), com `android.arm64` no `.gdextension`.
- **Realidade aumentada:**
  - passthrough do Quest (`environment_blend_mode` alfa e o fundo transparente);
  - a planta sobre o piso real, ancorada por um toque;
  - escala ajustável (1:1 ou maquete na mesa).
- **Desempenho:** 72–90 quadros por segundo no Quest.
  - níveis de detalhe obrigatórios;
  - menos sombras;
  - itens instanciados com `MultiMesh` quando passarem de algumas centenas;
  - medir a ponte compilada em release no ARM.
- **Riscos:**
  - os dispositivos e o SteamVR precisam estar à mão para testar, e os roteiros de captura
    não cobrem o headset;
  - o build Android da ponte (NDK, `rayon` em ARM) ainda não foi experimentado;
  - o boneco animado tem 7,6 MB por arquivo: avaliar uma versão reduzida para o Quest.

## G5: empacotamento e exportação

- **Modelos de exportação do Godot 4.7.2:**
  - Windows, Linux e Android;
  - instalados pelo editor ou pelo script, com o hash conferido.
- **Presets de exportação** (`export_presets.cfg`):
  - a biblioteca da ponte em release ao lado do executável;
  - os `.glb` importados dentro do `.pck`;
  - o catálogo de terceiros com os créditos (`assets/terceiros/CREDITOS.md`).
- **Instaladores:**
  - integrar ao instalador do Windows (NSIS) e ao `.deb` e `.tar.gz` da fase 12, com o
    `utsim` e o `utsim-view`;
  - `.apk` do Quest.
- **CI:**
  - job que baixa o Godot do console, roda `scripts/godot_teste.sh` sem janela e exporta os
    pacotes;
  - capturas roteirizadas guardadas como artefato.
- **Riscos:**
  - tamanho do pacote (os arquivos UAL);
  - assinatura do `.apk`;
  - o executável do console do Godot em runners Linux sem GPU, que roda com o driver
    `headless` (já é o caso do teste).

## Lacunas conhecidas depois da G1

- Regras de aparência duplicadas entre o Bevy e a ponte (resolver no início da G2).
- `operador_simples` sem braços e pernas animados no Godot (só o boneco é animado).
- Nós de fluido como cilindros simples; rede só desenhada, sem edição.
- Só a câmera de órbita (o Bevy tem seguir, primeira pessoa e do recurso).
- `run` bloqueia a interface durante as replicações.
- A interface não lê um replay `.json` gravado; só modelos `.ron`.

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
| G5 | Empacotamento e exportação (parcialmente feita) | 4–6 h |

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

## G5: empacotamento e exportação (parcialmente feita)

Feito:

- **Abertura e avisos:** a tela de abertura é a do UTSim (`godot/abertura.png`, configurada em
  `application/boot_splash`), sem o logo do motor. A licença MIT do Godot pede só o aviso de
  copyright acessível ao usuário: a tela "Sobre" mostra `Engine.get_license_text()` e
  `Engine.get_copyright_info()`, ao lado dos créditos dos ativos.
- **Modelos de exportação do Godot 4.7.2** (Windows, Linux e os demais):
  - `scripts/godot_exportar.sh --instalar-modelos` baixa o `.tpz` (cerca de 1,2 GB), confere o
    SHA512 publicado no mesmo release (`SHA512-SUMS.txt`) e instala em
    `%APPDATA%\Godot\export_templates\4.7.2.stable\` (Linux: `~/.local/share/godot/...`);
  - também dá para instalar pelo editor (Editor > Gerenciar modelos de exportação).
- **Presets** (`godot/export_presets.cfg`): "Windows Desktop" e "Linux".
  - Executável `utsim-3d` (`.exe` no Windows, `.x86_64` no Linux) e `utsim-3d.pck` separado;
  - os `.glb` importados e o catálogo entram no `.pck`; `testes/` e `capturas/` ficam de fora;
  - a biblioteca da ponte em release (`utsim_godot.dll` / `libutsim_godot.so`) é copiada pelo
    Godot ao lado do executável;
  - o ícone do UTSim (`assets/icone/utsim.ico`) e os metadados estão no preset.
- **`scripts/godot_exportar.sh`**:
  - compila a ponte em release (`scripts/godot.sh --release`), importa sem janela e exporta com
    `--headless --export-release`;
  - deixa a saída em `target/godot-export/<plataforma>/`;
  - gera `AVISOS-TERCEIROS.txt` (`scripts/godot_avisos.gd`: licença, licenças e copyright dos
    componentes do motor, lidos do próprio executável). Não é a licença do UTSim, que
    segue indefinida.
- **Pacotes** (`scripts/preparar-pacote.sh` aceita a pasta da exportação como 3º argumento):
  - a pasta `3d/` entra no instalador do Windows (NSIS) como o componente opcional "UTSim 3D
    (prévia)", com os atalhos "UTSim 3D (prévia)" e "Avisos de terceiros do UTSim 3D" no
    Menu Iniciar, sem mexer no `utsim-view`;
  - no `.zip` do Windows, no `.tar.gz` (com `instalar.sh`) e no `.deb` do Linux (comando
    `utsim-3d` e atalho no menu);
  - sem a exportação, tudo sai como antes (sem a pasta `3d/`).
- **CI**:
  - `scripts/godot_instalar_ci.sh` baixa o Godot do console e confere o SHA512;
  - job `godot-exportar` no `ci.yml`, só ao disparar à mão: teste sem janela, exportação,
    execução do programa exportado com `--headless --quit` e artefato;
  - `release.yml` aceita `incluir_3d` (ou a variável do repositório `UTSIM_INCLUIR_3D=true`)
    para levar o UTSim 3D aos pacotes.

Medidas no Windows (exportação real, ponte em release):

| Arquivo | Tamanho |
|---|---|
| `utsim-3d.exe` (modelo de exportação completo) | 109 MB |
| `utsim-3d.pck` (cena, scripts e todos os `.glb`, incluindo os UAL) | 2,3 MB |
| `utsim_godot.dll` | 6,4 MB |
| `AVISOS-TERCEIROS.txt` | 94 KB |

O programa exportado abre e carrega a ponte (`utsim-3d.exe --headless --quit`). O executável do
Linux tem 73 MB.

Pendente:

- **Ícone embutido no `.exe`:** o Godot só troca o ícone do executável com o `rcedit`
  (Configurações do editor > Exportar > Windows). Sem ele, o `utsim-3d.exe` mostra o ícone do
  Godot; o atalho do Menu Iniciar já usa o `utsim.ico`.
- **Instalador não compilado aqui:** o NSIS não estava instalado na máquina do desenvolvimento;
  o trecho novo do `utsim.nsi` está escrito e a montagem da pasta `3d/` foi testada, mas a
  compilação do `makensis` fica para o CI ou para quem tiver o NSIS.
- **Exportação para o Linux:** o preset foi conferido até a falta da `.so` (a ponte é compilada
  para o sistema atual); a exportação completa roda no job do CI.
- **`.apk` do Quest:** sem preset; precisa do NDK, do JDK, de uma ponte para `aarch64` e da
  fase G4. O preset será acrescentado junto com ela.
- **Tamanho:** o executável do modelo de exportação pesa 109 MB no Windows (o instalador comprime com LZMA;
  o tamanho final não foi medido sem o NSIS). Reduzir exige compilar um modelo
  personalizado do Godot sem os módulos que a interface não usa.
- **Assinatura** do instalador e do `.apk`.
- Capturas roteirizadas como artefato do CI (precisam de GPU).

## Lacunas conhecidas depois da G1

- Regras de aparência duplicadas entre o Bevy e a ponte (resolver no início da G2).
- `operador_simples` sem braços e pernas animados no Godot (só o boneco é animado).
- Nós de fluido como cilindros simples; rede só desenhada, sem edição.
- Só a câmera de órbita (o Bevy tem seguir, primeira pessoa e do recurso).
- `run` bloqueia a interface durante as replicações.
- A interface não lê um replay `.json` gravado; só modelos `.ron`.

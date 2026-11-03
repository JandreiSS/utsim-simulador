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
| G2 | Edição do fluxo (**feita**) | 10–14 h |
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

## G2: edição do fluxo (feita)

Objetivo: montar e alterar um modelo no Godot, como no editor Bevy, com desfazer e salvar.

O que ficou feito (detalhes em `docs/arquitetura.md`, seção 4r):

- **Antes:** a crate `utsim-look` (sem motor) tem as regras de aparência e o catálogo de
  terceiros, com os testes; o `utsim-viewer` e a ponte usam as mesmas. O texto RON do
  `Command::Set` foi para `utsim-edit::ron_text`.
- **Ponte:** `UtsimDocument` com abrir, novo, salvar e salvar como, desfazer e refazer, os
  problemas e avisos a cada mudança, o esquema das propriedades por tipo (distribuições por
  família e parâmetros, listas e mapas em RON, equipes como conjunto, turnos por tabela ou
  calendário), nome e descrição do modelo (`Command::SetInfo`), área de transferência,
  alinhar, distribuir, a cena do documento, o fantasma da Biblioteca e a miniatura do ativo.
- **Interação** (`editor.gd`, `biblioteca.gd`, `painel_propriedades.gd`, `principal.gd`):
  Biblioteca com ícones (clicar e clicar no chão, ou arrastar até o chão); mover com encaixe na
  grade, por projeção no piso, sem física; seleção por clique, Shift e caixa; ligar por arrasto
  da porta de saída até o destino, ou com a ferramenta Ligar; apagar; painel gerado do esquema
  e validado no Rust; atalhos Ctrl+Z, Ctrl+Y, Ctrl+S, Delete, Esc e os outros do Bevy; modos
  Editar e Simular.
- **Aparência:** o `visual.asset` escolhido da biblioteca do UTSim e do catálogo, com os
  créditos na dica e uma miniatura do objeto com o ativo (o que está sob o mouse na lista).
- **Testes:**
  - Rust (`cargo test -p utsim-godot`): o esquema de cada campo de cada exemplo lido e escrito
    de volta sem mudar o modelo, os turnos, a montagem do tutorial 1 pelas mesmas chamadas da
    interface comparada com `examples/tutorial_01_caixa.ron`, a miniatura e as diferenças
    entre modelos; `utsim-look` e `utsim-edit::ron_text` com os próprios testes;
  - roteiros da interface sem janela, no `scripts/godot_teste.sh`:
    `godot/roteiros/g2_tutorial_01.json` monta o tutorial 1 só pela interface e compara o
    `.ron` salvo com o exemplo (`igual_a`); `g2_montar_linha.json` monta uma linha, edita
    propriedades, escolhe um ativo, simula, desfaz, salva e reabre.

**Conferência visual feita** em 2026-11-02 (os dois roteiros passaram com janela). Ela achou
rótulos cortados no painel de propriedades, o nome do modelo cortado na barra superior e o
resumo do Simular apertado, corrigidos na G3 (ver lá). Para repetir:

```bash
bash scripts/godot.sh
"$HOME/Documents/Godot_v4.7.2-stable_win64.exe/Godot_v4.7.2-stable_win64_console.exe" --path godot -- \
    --utsim-roteiro=roteiros/g2_montar_linha.json --utsim-capturas="$(pwd -W)/godot/capturas/g2"
"$HOME/Documents/Godot_v4.7.2-stable_win64.exe/Godot_v4.7.2-stable_win64_console.exe" --path godot -- \
    --utsim-roteiro=roteiros/g2_tutorial_01.json --utsim-capturas="$(pwd -W)/godot/capturas/g2"
```

e confira nas capturas (`godot/capturas/g2/*.png`, fora do git):

- as portas (losangos verde e azul) no bloco selecionado e a seta de ligação seguindo o mouse
  (`g2_linha_04_ligando`);
- o fantasma do objeto ao colocar (`g2_linha_02_fantasma`);
- o painel de propriedades sem cortar à direita (as listas de escolha não alargam mais o
  painel) e a miniatura do ativo (`g2_linha_05b_ativo`);
- o modelo do tutorial 1 montado (`g2_tutorial_01`).

Abrir o projeto (`bash scripts/godot.sh --abrir`, F5) e montar um modelo à mão também vale a
pena: a dica de um botão da Biblioteca, o arrasto até o chão e o arrasto entre as portas são
coisas que só a mão no mouse confirma.

O plano original da G2:

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

## G3: rede, fluidos e resultados (feita)

O que ficou feito (detalhes em `docs/arquitetura.md`, seção 4r):

- **Rede de caminhos** (`godot/scripts/rede.gd`, ferramenta Rede ou tecla N): clicar no chão
  põe um nó; com um nó armado (verde) o clique seguinte o liga ao novo, então um caminho sai
  clique a clique; clicar em outro nó liga o armado a ele (laços); arrastar move o nó, com
  encaixe na grade; clicar num trecho mostra no painel mão dupla ou única, capacidade (com
  capacidade a pista é controlada e fica vermelha, como no Bevy), inverter e excluir. Esc
  encerra o caminho, Del exclui. Tudo pelos comandos do `utsim-edit` (`AddNode`, `AddEdge`,
  `SetEdge`, `MoveNode`...), com desfazer. A rede é desenhada por cima da cena em todas as
  ferramentas. Na ponte: `crates/utsim-godot/src/network_edit.rs`.
- **Fluidos** (`cena.gd`): tanque como casca transparente com o líquido na altura do nível e a
  cor pela taxa (enchendo, esvaziando, parado); fonte, sumidouro, tubulação e conversores como
  marcadores coloridos; tubos entre as portas, acesos quando passa fluido; linha roxa do
  conversor ao objeto discreto; rótulo com a porcentagem. Na ponte: `extras.rs`
  (`UtsimPlayback.fluid_frame(t)`).
- **Fluxos de tokens** (`fluxos.gd`, botão Fluxos no Simular): o grafo de cada fluxo em 2D
  (camadas ao longo de `next`), os tokens de cada atividade no instante da reprodução e as
  contagens ao longo da rodada, com a linha do instante.
- **Resultados** (`resultados.gd`, botão Resultados): resumo com o gargalo geral e dentro do
  turno, tabela por objeto com IC de 95 %, exportação CSV; gráficos de ocupação (com o traço
  do IC), estados (cores da cena), série temporal e histograma, desenhados com `draw_*`
  (`grafico.gd`); cenários escritos como texto (`[nome]` e `objeto.campo = valor`), com números
  aleatórios comuns; otimização (métrica, variáveis, método) pelo `utsim-opt`.
- **Análise** (dentro de Resultados; `docs/plano_analises_e_dinamica.md`, seção 2):
  - aquecimento: curva de Welch e corte do MSER-5, com "Aplicar", que grava `run.warmup`;
  - replicações necessárias para uma precisão, com "Rodar mais" (acrescenta às já feitas,
    com as sementes seguintes);
  - lei de Little por objeto e no sistema, com semáforo (verde, amarelo, vermelho);
  - gargalo móvel (único e compartilhado).

  Cada uma diz em uma linha o que significa.
- **Replicações em thread** (`jobs.rs`, classe `UtsimJob`): simular, rodar mais, cenários,
  otimização, análises e PERT rodam numa thread da ponte; a barra inferior mostra o progresso
  e o Cancelar, que para na próxima replicação (`run_replications_with`).
- **PERT/CPM da seleção** (`pert.gd`, botão PERT/CPM ou menu Analisar): com dois objetos
  selecionados é "de… até…" (do primeiro clicado até o segundo); com outros, a parte do
  processo.
  - Abre no modo **simulado** (decisão do dono); Clássico e Monte Carlo a um clique, com prazo
    e número de sorteios.
  - Abas: Rede (atividade no nó com ES/EF/LS/LF, folga, caminho crítico em vermelho, setas
    pela criticidade), Gantt, Distribuição (histograma e `P(T ≤ prazo)`), Tabela e Texto (o
    relatório do `utsim pert`); exporta HTML e CSV.
  - Os objetos críticos ganham contorno vermelho na cena do Editar. Na ponte: `pert_view.rs`.
- **Câmeras** (Simular): Órbita, Seguir, Primeira pessoa e Do recurso, acompanhando o bloco
  escolhido na lista (o primeiro recurso móvel, se nenhum).
- **Detalhe:** no máximo 60 rótulos, os mais próximos da câmera até 160 m; itens além de
  140 m não são desenhados.
- **Replay `.json`:** "Gravar replay" grava a replicação 0 no formato do `utsim run --replay`;
  Abrir aceita `.json` e reproduz, com as estatísticas daquela replicação em Resultados.
- **Ajustes da conferência visual da G2:** os rótulos do painel de propriedades quebram a linha
  (a dica traz o texto inteiro e a ajuda) e as listas encolhem em vez de cortar o rótulo; a
  barra superior mostra só o nome do modelo, com reticências, e o arquivo vai para o título
  da janela; o resumo do Simular ficou curto (o resto está em Resultados, que usa a largura
  toda da vista).
- **Testes:**
  - Rust (`cargo test -p utsim-godot`, 53 testes): rede desenhada clique a clique com desfazer,
    trechos (sentido, capacidade, inverter), nós (mover, renomear, achar, excluir); tanques
    mudando de nível com as cores; fluxos com tokens no tempo; tabela igual à do `utsim run`,
    com o gargalo; estados somando 1 com as cores da cena; "rodar mais" igual a uma rodada
    maior; cenários em texto; formulário de otimização; semáforo de Little; tarefas em thread
    (mesmo resultado, cancelar, análises, cenários, otimização); PERT nos três modos;
  - `godot/testes/teste.gd` roda as replicações também por `UtsimJob`;
  - roteiros sem janela no `scripts/godot_teste.sh`:
    - `g3_rede.json`: desenha, fecha um laço, mão única, pista controlada, inverte, desfaz,
      arrasta, exclui, salva e reabre;
    - `g3_resultados.json`: simula em thread, tabela, gráficos, aquecimento com aplicar,
      precisão com rodar mais, Little, gargalo móvel, cenários, otimização, progresso e
      cancelar numa rodada de 3000 replicações;
    - `g3_pert.json`: de chegada até saída nos três modos, prazo, abas, HTML e seleção de
      objetos;
    - `g3_cena.json`: tanques, fluxos, câmeras, limite de rótulos, gravar e abrir o replay.

**Conferência visual feita** em 2026-11-03: os roteiros `g3_*` e `g2_*` rodaram com janela e as
capturas (`godot/capturas/g3/*.png`, fora do git) foram olhadas. Ela achou e corrigiu:

- **Fluidos fora do quadro:** a caixa da cena (`bounds_min`/`bounds_max` da ponte, ver
  `extras::scene_bounds`) não incluía a rede de fluidos, que fica numa faixa abaixo dos blocos;
  o enquadramento inicial deixava os tanques junto da câmera (um cilindro gigante no canto) e
  os tubos saindo da tela. Agora a caixa, o piso e o enquadramento do Editar incluem os
  fluidos. As posições, o raio e a altura já eram os do Bevy (`utsim-scene::fluids`).
- **Câmera Do recurso dentro de uma "cúpula":** era a base do cone da chegada, no mesmo ponto
  de partida do AGV, vista de trás. A câmera agora fica na posição de condução (acima do
  veículo, olhando à frente e um pouco para baixo) e esconde, enquanto está ali, as peças do
  próprio veículo na linha de visão e os blocos que contêm o olho (`CenaUtsim.clear_view`);
  a Primeira pessoa também.
- **Fluxos de tokens com caixas sobrepostas e texto cortado:** a ponte manda a camada e a
  pista de cada atividade; o painel dimensiona as caixas pelo texto, rola e tem zoom (botões
  e Ctrl + roda), começando ajustado à largura mas nunca abaixo de 75 %.
- **Série e histograma vazios:** o gráfico oferece "Ligar e rodar de novo", que liga
  `run.sample_interval` (cerca de 200 pontos na janela de estatística) e
  `run.histogram_bins = 20` num passo de desfazer e simula de novo. Para isso o `patch` do
  `utsim-model` passou a aceitar esses dois campos de `run`.
- **PERT/CPM no Editar:** a janela vai até a linha de status e o painel de propriedades fica
  oculto enquanto ela (ou Resultados) está aberta.
- **Roda do mouse:** rolar um painel ou lista também aproximava a câmera (a soltura da roda
  passava sempre). Agora a roda e os arrastos que começam sobre a interface não chegam à vista.
- **Visual dos painéis:** tema único `godot/tema/utsim.tres` (projeto inteiro) com títulos de
  painel e de seção, ajuda em cinza, caixas de marcar visíveis no escuro, campos e listas
  contrastados; seções das propriedades recolhíveis; a barra superior cabe com o nome do
  modelo. Os rótulos do painel de propriedades quebram a linha sem cortar (conferido com janela
  no `g2_tutorial_01`).
- **Pilha (LIFO)** na Biblioteca (Queue com `discipline: Lifo`), com "pilha" no rótulo e a seta
  invertida na cena.

Os roteiros ganharam conferências que pegariam esses defeitos: `fluidos` com `enquadrados` e
`altura_no_maximo`, `camera_livre` (nada a menos de X m do olho nem no começo da linha de
visão), `fluxos.legivel` (sem sobreposição, sem corte, zoom legível), `modelo_contem`,
`camera_distancia` com o passo `rolar`, e as propriedades ocultas sob o PERT. Para repetir:

```bash
bash scripts/godot.sh
G="$HOME/Documents/Godot_v4.7.2-stable_win64.exe/Godot_v4.7.2-stable_win64_console.exe"
for r in g3_rede g3_resultados g3_pert g3_cena g2_tutorial_01 g2_montar_linha; do
    "$G" --path godot -- --utsim-roteiro=roteiros/$r.json --utsim-capturas="$(pwd -W)/godot/capturas/g3"
done
```

Restou conferir à mão (abrindo o projeto, F5): desenhar uma rede no mouse real (escolha de
trecho perto de um nó, arrasto do nó) e a roda num mouse de verdade (o roteiro usa eventos
simulados).

O plano original da G3:

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
  arrays de quadro (**não feita**, ver as lacunas).
- **Níveis de detalhe e limite de rótulos** para modelos grandes, como no A3D-4.
- **Riscos:** os gráficos no Godot não têm um equivalente pronto do `egui_plot`. Opções:
  - desenhar no `Control` com `draw_*` (a escolhida);
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

### Dados reais nos campos de distribuição (feito)

Todo campo de distribuição do painel de propriedades tem a linha **Fonte** (Distribuição ou Dados
reais); o ajuste automático de `utsim_fit::auto` roda numa thread da ponte e o resultado (família,
parâmetros, n, histograma com a curva, avisos e ranking das candidatas) aparece no próprio painel.
Na ponte (`crates/utsim-godot/src/dados.rs`): `UtsimDocument.data_fields`, `data_entry`,
`data_families`, `data_columns`, `data_parse_values`, `data_relative`, `data_fit` (job `ajuste`),
`data_apply` e `data_to_dist` (ambos por `Command::SetFieldSource`, um passo de desfazer) e
`data_report` (job `relatorio_dados`, o `index.html` do modelo e um HTML e um CSV por campo); um
cache compartilhado com a thread evita ajustar de novo ao aplicar, desfazer e refazer. Na interface:
`godot/scripts/dados_reais.gd` (formulário e resultado), `mini_histograma.gd` e os tipos `Selo`,
`AvisoDados` e `ErroDados` do tema. Roteiro `godot/roteiros/g3_dados_reais.json` (sem janela no
`scripts/godot_teste.sh`; com janela, capturas em `godot/capturas/dados/`).

## Lacunas conhecidas depois da G3

- O ajuste de dados reais não cobre ainda os campos fora dos objetos (fluxos de tokens, fluidos,
  calendários): eles aparecem no painel só quando a interface passar a editá-los.


- A Pilha (LIFO) aparece com o asset da fila virado; não há um modelo 3D próprio de pilha.
- A câmera Do recurso esconde o que bloqueia a vista, mas não desvia de paredes de um layout
  CAD (não há colisão).
- `operador_simples` sem braços e pernas animados no Godot (só o boneco é animado).
- O Editar desenha os fluidos com o nível inicial, mas não cria nem edita os nós de fluido; os
  tanques são cilindros simples (os modelos do `utsim-assets` ainda não entram).
- O Editar não cria nem edita calendários, fluxos de tokens ou chegadas por período: um objeto
  escolhe um calendário que já existe, e as chegadas por período (`schedule`) são editadas
  como texto RON, sem a tabela de períodos do Bevy.
- Cena ao vivo (cliente do socket `utsim-scene/1`) não foi feita.
- O replay mostrado é sempre o da replicação 0.
- A aba Análise ainda não tem o Gantt de estados, a série de WIP com banda, ANOVA/Tukey nem o
  mapa de calor (P2 do plano de análises). "Aplicar" o aquecimento muda o modelo; os números
  com ele vêm na próxima simulação.
- A janela PERT/CPM não tem a aba do tornado (sensibilidade do Monte Carlo); o relatório HTML
  exportado tem.
- O nível de detalhe é só o limite de rótulos e a distância dos itens: os modelos dos blocos
  não têm versões simplificadas (fica para a G4, no Quest).
- O roteiro de interface escolhe um item de lista (`escolher`) chamando a lista direto: sem
  janela, a lista aberta é uma janela à parte que os eventos simulados não alcançam.

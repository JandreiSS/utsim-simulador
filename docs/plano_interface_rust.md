# Plano da interface em Rust (egui dentro do Godot)

Decisão do dono (2026-11-03): na interface Godot (`godot/`) só a vista 3D central continua
no Godot. Toda a interface em volta dela (barra superior, biblioteca, lista de objetos,
propriedades, resultados, PERT/CPM, dados reais, rede, corredores, janelas e barra de status)
sai do GDScript e passa a ser Rust, desenhada com egui dentro da janela do Godot pela ponte
GDExtension (`crates/utsim-godot`). Este documento é a primeira frente dessa migração.

Por quê:

- a lógica dos painéis já está no Rust (o esquema das propriedades em `schema.rs`, a edição
  com desfazer em `utsim-edit`, os resultados em `results.rs`): o GDScript só traduzia
  dicionários em nós; com egui o painel chama o Rust direto, sem a volta pelo GDScript;
- tudo o que é Rust é testável sem o motor (`cargo test -p utsim-godot`);
- o mesmo visual (crate `utsim-ui-tema`) serve à interface Godot, ao launcher e ao app
  UTSim Seis Sigma, que já são egui.

## Arquitetura

```
Godot (janela, vista 3D, câmera, cena)            Rust (ponte utsim-godot)
┌──────────────────────────────────────┐         ┌──────────────────────────────────┐
│ principal.gd                         │ set_bar │ UtsimUi (Node2D, ui_class.rs)    │
│  ├─ CanvasLayer "Interface"          │────────▶│  ├─ Host (egui_host.rs)          │
│  │   ├─ painéis GDScript (a migrar)  │         │  │   ├─ egui::Context            │
│  │   └─ InterfaceRust (UtsimUi) ─────┼─────────┼──┤   ├─ ui_bar.rs, ui_left.rs,   │
│  │                                   │         │  │   │  ui_props.rs, ui_bottom.rs│
│  ├─ editor.gd, camera_orbita.gd      │ action  │  │   └─ Registry (nomes)          │
│  └─ cena.gd (vista 3D)               │◀────────│  ├─ egui_input.rs (entrada)      │
└──────────────────────────────────────┘         │  └─ egui_mesh.rs (desenho)       │
                                                  └──────────────────────────────────┘
```

- **Nó `UtsimUi`** (`ui_class.rs`): um `Node2D` no `CanvasLayer` da interface, com
  `z_index` 100 (os menus do egui abrem sobre os painéis Godot que ainda existem). Só as
  classes do núcleo do Godot são geradas para a ponte (sem `__codegen-full`, para o build
  continuar curto); por isso os eventos de entrada, a `Image` e o `DisplayServer` são lidos
  e chamados dinamicamente (`get`, `call`), poucas chamadas por evento.
- **Entrada** (`egui_input.rs`, puro): `_input` lê cada evento (mouse, roda, teclas, texto,
  modificadores) para um `Input` simples e o converte para eventos do egui, em pontos
  (pixels sobre a escala). Ctrl+C, Ctrl+X e Ctrl+V viram copiar, recortar e colar, com a
  área de transferência do `DisplayServer`. A escala vem do `content_scale_factor` da
  janela vezes `ui_scale` do nó.
- **Quem fica com o evento** (`egui_host.rs`, puro): o evento é do egui (marcado como
  tratado, e a vista 3D não o recebe) quando o ponteiro está sobre um painel ou um menu do
  egui, quando um clique começou sobre o egui e ainda está pressionado (um arrasto), ou,
  para teclas, quando um campo de texto do egui tem o foco. **A roda sobre um painel é
  sempre do painel: nunca aproxima a câmera.** O resto segue para os controles Godot, o
  `editor.gd` e a câmera, como antes. Tab nunca vai para o egui fora de um campo de texto
  (é a troca Editar/Simular), e um botão do egui não guarda o foco do teclado, então os
  atalhos (Ctrl+O/N/S, Tab, F, F1, F3, Del, L, N, C, G...) continuam do `principal.gd`.
- **Desenho** (`egui_mesh.rs`, puro, e `ui_class.rs`): o egui tessela a saída em malhas;
  cada malha vira um canvas item filho do nó (reaproveitado entre quadros), com os triângulos cortados na CPU ao recorte do egui
  (`egui_mesh::clip_triangles`; o recorte do canvas item não bastou) e um `canvas_item_add_triangle_array`
  com a textura certa. As texturas do egui (o atlas das fontes) têm uma cópia na CPU, onde
  as atualizações parciais são aplicadas, e são enviadas ao Godot como `Image` RGBA8. As
  cores do egui são pré-multiplicadas; a ponte as converte para alfa direto, que é a mistura
  do canvas 2D do Godot.
- **Redesenho só quando precisa**: um quadro do egui roda quando chega entrada, quando o
  estado mostrado muda (`set_bar` com outro estado), quando a janela muda de tamanho ou
  quando o próprio egui pede (animações, dicas). Fora disso os canvas items ficam como
  estão e o custo por quadro é zero.
- **Área livre** (`central_rect()`): o que os painéis do egui deixam para a vista 3D. O
  `principal.gd` combina com os painéis Godot que ainda existem em `free_span()` (o
  enquadramento da câmera centraliza o modelo entre os painéis) e `over_ui()` pergunta
  `is_over(p)` ao nó.
- **Ações**: o egui não muda o modelo nem o modo. A barra devolve ações em texto
  (`novo`, `abrir`, `modo:simular`, `ferramenta:ligar`, `organizar:alinhar:esquerda`,
  `grade:ligar`, `passo:0.5`, `analisar:resultados`...), emitidas no sinal `action(what)`
  depois do quadro (diferido, fora do empréstimo do nó), e o `principal.gd`
  (`bar_action`) chama as mesmas funções dos botões antigos. O estado mostrado vem de
  `_sync_bar()` (modo, ferramenta, grade, desfazer/refazer, título).
- **IME**: o texto digitado chega pelo `unicode` das teclas, o que cobre acentos e teclas
  mortas; a composição de IME (idiomas asiáticos) fica para depois.

### Tema comum (`crates/utsim-ui-tema`)

Crate só de egui, sem Godot: as cores com papel (painel, botão, selecionado, destaque,
campo, menu, texto, ajuda, erro, aviso, selo), os tamanhos (corpo 15, títulos 17, barra de
46 px), o estilo de cada estado dos widgets, as fontes (a fonte de interface do sistema,
Segoe UI no Windows e Noto Sans ou DejaVu Sans no Linux, lida de onde o sistema a guarda,
sem ir no pacote, com as fontes do egui como reserva) e dois widgets: o botão primário
("Aplicar") e o controle segmentado (Editar | Simular). As cores são as do tema Godot
(`godot/tema/utsim.tres`, removido no passo 5), então os painéis egui e os painéis Godot
ficaram iguais durante a migração. **O launcher (`crates/utsim-launcher`) e o app UTSim Seis Sigma
(`crates/utsim-lss-app`) passam a usar este tema** (`utsim_ui_tema::install(&ctx)`) quando
forem mexidos; não foram tocados nesta frente.

### Um só egui

O workspace tem um egui só (0.36.2): o `eframe =0.36.2` do launcher e o `bevy_egui 0.43` do
visualizador usam o mesmo. A ponte e o tema fixam `egui = "=0.36.2"` sem features além de
`default_fonts`. O egui 0.36 pede Rust 1.95, que passou a ser o `rust-version` da ponte.

## Os roteiros continuam com os mesmos nomes

Os roteiros de interface (`godot/roteiros/*.json`, rodados pelo `roteiro.gd`, com janela ou
sem) acham os controles pelo nome do nó (`Modo_Editar`, `Botao_Novo`, `Prop_name`...). Os
widgets do egui ficam com esses nomes:

- a cada quadro a interface registra o retângulo, o texto, o estado e as escolhas de cada
  widget nomeado (`egui_widgets.rs`, `Registry`);
- a ponte responde `widget_rect(nome)`, `has_widget(nome)`, `widget_info(nome)`,
  `widget_names()` e `choose(nome, item)`;
- o `roteiro.gd` procura primeiro um nó Godot com o nome e, se não houver, o widget do egui
  (`_widget`): `clicar`, `mover`, `arrastar` e `rolar` mandam os mesmos eventos de entrada
  ao centro do widget; `escolher` usa `choose` (como escolher na lista aberta); `texto`,
  `visivel` e `oculto` leem o registro.

Assim um passo `{"clicar": {"controle": "Modo_Simular"}}` funciona igual antes e depois da
migração, e um roteiro novo não precisa saber o que é egui. Nomes novos da barra: `Menu_Mais`
(os grupos que não cabem), `Menu_Analisar`, `Organizar_<ação>`, `Analisar_<ação>`,
`Grade_Passo_<passo>`.

## Ordem da migração e estimativas (horas Opus 5.5)

| Passo | Conteúdo | Estimativa | Situação |
|---|---|---|---|
| 1 | Fundação: nó `UtsimUi`, entrada, desenho, área livre, registro de nomes, tema comum, testes | 6–8 h | **feito** |
| 2a | Barra superior em egui, com grupos e menu "Mais" | 2–3 h | **feito** |
| 2b | Painel de propriedades em egui, chamando `schema.rs`/`utsim-edit` direto (grade de duas colunas, seções recolhíveis, texto que quebra, "Aplicar" primário); miniatura do ativo como textura de um `SubViewport` | 6–9 h | **feito** |
| 3 | Biblioteca (arrastar para o chão), lista de objetos com busca, barra de status com progresso e Cancelar, painel do Simular (replicações, blocos, câmera, resumo) e barra de reprodução | 5–7 h | **feito** |
| 4 | Resultados: tabela, gráficos (egui_plot da mesma versão do egui), análises, cenários, otimização | 8–12 h | **feito** |
| 5 | PERT/CPM, dados reais (formulário e mini-histograma), rede, corredores, ligação, janelas de arquivo (seletor próprio em egui ou o `FileDialog` nativo pela ponte), atalhos, confirmação de descarte; remoção do tema Godot | 10–14 h | **feito** (5a) |
| 5b | Editores de tabela de chegadas e de horário (C6 de `docs/plano_chegadas_e_equipes_3d.md`), fonte no 3D (C7), ligações de equipe e pedido no editor 3D (E1b) e equipe ao vivo (E8b) | 30–45 h | a fazer |

Total: 37 a 53 h. Cada passo remove o GDScript que substitui e mantém os roteiros
`g2_*`, `g3_*` e `g4_*` passando sem janela (`UTSIM_GODOT=1 bash scripts/verificar.sh`).

### Passo 2b (propriedades), como foi feito

- `ui_props.rs` (puro, testado com um `Editor` de verdade) desenha o esquema de
  `Editor::properties` direto, sem dicionário no meio. O nó recebe o documento uma vez
  (`set_document(doc)`) e o objeto a mostrar (`show_properties(objeto, outros, linha, motorista)`);
  a vista só é refeita quando a revisão do documento ou o objeto mudam.
- Cada mudança sai como `PropAction::Set` com o mesmo caminho e o mesmo valor que o painel
  Godot mandava e a ponte a aplica com `Editor::set_property` (a validação do `schema.rs`, um
  passo de desfazer; a cor fecha o passo quando o botão do mouse sobe). O `principal.gd`
  recebe `prop_ok`, `prop_erro` e o resto pelo sinal `action` (`_panel_action`): atualiza a
  cena, a seleção (ao renomear) e a linha de status.
- Rótulo à esquerda (150 pt, quebra em várias linhas, dica com a ajuda) e controle à
  direita, em colunas alinhadas; seções recolhíveis por título (ficam fechadas para todos
  os objetos); problemas em vermelho e avisos em amarelo, clicáveis; distribuição com
  família, parâmetros, média e "editar como texto"; "Aplicar" primário na descrição e nos
  scripts; saídas em ordem de porta com ▲ ▼ ✕; botões de alinhar e distribuir com vários
  selecionados; "Mostrar motorista" na aparência da empilhadeira.
- Campos de texto: selecionam tudo ao ganhar o foco, gravam com Enter ou ao perder o foco,
  Esc descarta; enquanto há um campo com o foco as teclas são do egui (Del, Ctrl+Z e as
  letras não chegam aos atalhos).
- Miniatura: um `SubViewport` do `principal.gd` (`_build_miniature`, `show_miniature`) com a
  mesma cena; o egui mostra a textura dele (`TextureId::User(1)`) e pede, pela ação
  `miniatura`, o ativo escolhido ou o que está sob o mouse na lista aberta.
- O painel Godot (`painel_propriedades.gd`) continua para o que ainda não migrou: ferramentas
  Rede e Corredor, ligação selecionada e o formulário de dados reais (campo alimentado por
  dados, ou "Fonte: Dados reais" escolhida no painel egui). O `principal.gd` escolhe em
  `_show_properties` e mostra um ou outro.
- Roteiros: `reveal(nome)` rola o painel até um widget fora da vista antes do clique;
  um nó Godot oculto com o mesmo nome (o painel Godot) cede ao widget do egui.

### Passo 3 (painel da esquerda, status e reprodução), como foi feito

- `ui_left.rs` (puro, testado): no Editar, a **Biblioteca** (as entradas de
  `document::palette()` por categoria, com os ícones `res://icones/<id>.svg` que o
  `principal.gd` passa uma vez por `set_library_icons` e a ponte mostra como texturas
  `TextureId::User`), a lista **Objetos** com a busca e **Mostrar obstáculos e folgas**; no
  Simular, replicações (campo com − e +), **Simular de novo**, a linha de situação, os blocos,
  a câmera, **Fluxos**, **Gravar replay**, **Rótulos**, **Sombras** e o resumo com o gargalo.
  `ui_bottom.rs` (puro, testado): a barra de status (texto cortado com "…" e o texto inteiro
  na dica, progresso com porcentagem ou indeterminado, **Cancelar**) e a barra de reprodução
  (◀ evento, Reproduzir/Pausar, evento ▶, velocidade, linha do tempo e `t = … / …`).
- O que mostram vem do `principal.gd` em dicionários pequenos (`set_edit_state`,
  `set_sim_state`, `set_status_bar`, `set_playback_bar`), e um quadro só roda quando algo muda
  (durante a reprodução, o tempo muda a cada quadro). Os objetos da lista a ponte lê do
  documento (`Editor::objects`), de novo a cada `set_edit_state`, que o `after_edit` chama.
- Ações novas no sinal `action`: `biblioteca:<id>`, `biblioteca_arrastar:<id>`,
  `objeto:<nome>`, `objeto_focar:<nome>`, `obstaculos:ligar|desligar`, `replicacoes:<n>`,
  `rodar_de_novo`, `bloco:<k>`, `camera:<modo>`, `gravar_replay`, `rotulos:…`, `sombras:…`,
  `cancelar`, `evento:±1`, `reproduzir`, `velocidade:<x>`, `tempo:<s>`. "Fluxos" reusa
  `analisar:fluxos`.
- **Arrastar da biblioteca**: o arrasto começa no egui (o ponteiro andou 8 pontos desde o
  aperto numa entrada) e sai como `biblioteca_arrastar:<id>`; o `editor.gd` passa a seguir o
  mouse (`start_palette_drag`) e resolve a soltura na vista 3D: o `_input` dele roda antes do
  `_input` do `UtsimUi`, então vê o movimento e a soltura mesmo com o egui dono do arrasto, e
  coloca o objeto quando o botão sobe sobre a área livre central (`over_ui` falso). Soltar
  sobre um painel cancela, como antes.
- Nomes mantidos: `Biblioteca`, `Biblioteca_<id>`, `Objetos_Busca`, `Objetos_Lista`,
  `Ver_Obstaculos`, `Status`, `Progresso`, `Botao_Cancelar`, `Simular_Replicacoes`,
  `Botao_RodarDeNovo`, `Simular_Blocos`, `Simular_Camera`, `Botao_Fluxos`,
  `Botao_GravarReplay`, `Reproducao_Anterior`, `Reproducao_Play`, `Reproducao_Proximo`,
  `Reproducao_Velocidade`. Novos: `PainelEsquerdo`, `Objeto_<nome>`, `Objetos_Limpar`,
  `Bloco_<k>`, `Simular_Status`, `Simular_Menos`, `Simular_Mais`, `Simular_Rotulos`,
  `Simular_Sombras`, `Simular_Resumo`, `BarraInferior`, `BarraReproducao`,
  `Reproducao_Tempo`, `Reproducao_T`. O `roteiro.gd` lê a linha de status em
  `principal.status_text` e revela (rola até) a origem de um `arrastar` antes do arrasto.
- Removidos: `biblioteca.gd` e, do `principal.gd`, o painel esquerdo, a barra de status e a
  barra de reprodução em Godot (e `set_reserved_bottom`: a barra de status agora é do egui, e o
  painel de propriedades para acima dela). Sem a ponte, uma linha Godot diz que ela não
  carregou.
- Painel de propriedades: a coluna dos rótulos passou a ser 40 % da linha, entre 140 e 200
  pontos (`ui_props::label_width`), e a coluna dos valores fica com o resto; um rótulo maior
  quebra em mais linhas e nunca é cortado (a dica tem o texto inteiro com a ajuda). Na
  conferência a 1280×720 nenhum rótulo saiu cortado.
- Roteiros a 1280×720: o passo `camera` agora põe o alvo no meio da área livre (entre os
  painéis) e, numa janela em que essa área mostra menos chão que a 1600×900, afasta a câmera
  na mesma proporção (as conferências `camera_distancia` usam a mesma escala). Assim os pontos
  do chão de `g2_tutorial_01` e `g2_montar_linha` caem na área livre e os dois roteiros
  passam com janela a 1600×900 e a 1280×720; sem janela (1600×900) nada muda. O `main`
  tinha a mesma falha a 1280×720: com o `main` do início deste passo, o passo 19 de
  `g2_tutorial_01` falha (a `saida` solta em x = 11,7 m cai sob o painel de propriedades), e
  antes da frente egui o painel de propriedades em Godot tinha a mesma largura (420) e o mesmo
  lugar, então o ponto caía sob ele do mesmo jeito.
- Conferência com janela a 1600×900 e 1280×720 (`g2_tutorial_01`, `g2_montar_linha`,
  `g3_cena` e um roteiro do editor com `centro_de_distribuicao.ron`: busca, dica da
  biblioteca, entrada armada, obstáculos, Simular, seguir um bloco, linha do tempo,
  velocidade, sombras, 40 replicações). Achados e correções: a lista de objetos ficava vazia ao
  abrir um modelo (a revisão do documento novo podia ser igual à do anterior; a lista é lida de
  novo a cada `set_edit_state`); "Desmontagem" saía cortada na biblioteca (ícone e margens
  menores); as listas cortadas não mostravam que rolam (alça fina visível em repouso); a
  biblioteca ocupa até 60 % do painel.

### Passo 4 (Resultados), como foi feito

- `ui_results.rs` (puro, testado com corridas de verdade de `servidores.ron`): a janela cobre a
  área livre central (o retângulo vem do host, `Host::results_rect`, e não mais de
  `LEFT_WIDTH`/`BOTTOM_HEIGHT`), com as cinco abas. A ponte recebe a corrida uma vez
  (`UtsimUi.set_results(res, pode_rodar, pasta_csv)`) e guarda os formulários e as análises
  feitas em `ResultsMemory`; uma corrida nova mantém os formulários e o objeto do gráfico.
- **Resumo**: cartões (replicações e semente, tempo de parada e aquecimento, gargalo geral e no
  turno com a ocupação, saídas por replicação), a tabela por objeto desenhada à mão (cabeçalho,
  números à direita, linhas alternadas, o gargalo com fundo e marca amarelos e o selo
  "gargalo") e a exportação CSV, feita na ponte (`results_export`).
- **Gráficos** com `egui_plot =0.37.0` (o da mesma versão do egui): barras de ocupação com o IC,
  estados empilhados com as cores da cena, série e histograma; a legenda e o nome do eixo y
  ficam numa linha acima do gráfico (dentro, cobriam as curvas; um nome longo no eixo passava
  por cima do texto); a faixa do y é calculada (`y_range`: uma curva plana do egui ia a
  ±2000). Os gráficos não aproximam nem arrastam: a roda rola a janela.
- **Análise, Cenários, Otimização**: cada botão vira `resultados:trabalho:<tipo>`; o
  `principal.gd` pede a corrida à ponte (`results_job`, montada de `ui_results::task` com o
  que os formulários têm, ou o motivo de não começar), roda com `start_job` (progresso e
  Cancelar) e devolve o trabalho pronto (`results_done`), que guarda a tabela e dá a linha de
  status. "Rodar mais" passa a corrida nova por `on_results` antes.
- Outras ações: `resultados:fechar`, `resultados:exportar`, `resultados:ligar_amostras`,
  `resultados:aplicar_aquecimento:<t>`.
- Nomes mantidos para o `g3_resultados.json` (`PainelResultados`, `Res_Fechar`, `Res_Resumo`,
  `Res_Tabela`, `Res_TipoGrafico`, `Res_Grafico`, `Res_AquecAnalisar`, `Res_Precisao`...). As
  abas são widgets `Res_Abas_<título>` e o passo `{"clicar": {"aba": ["Res_Abas", "Gráficos"]}}`
  do `roteiro.gd` acha o widget quando não há `TabContainer`; cada gráfico registra quantas
  coisas desenha (`count` no `widget_info`) para o `grafico`. O registro guarda também o
  texto dos widgets rolados para fora da vista (`Registry::read`), e o resumo de uma aba não
  mostrada continua legível, como numa aba Godot oculta.
- Removidos: `resultados.gd`. `grafico.gd` e `mini_histograma.gd` ficam para o PERT/CPM, os
  fluxos e os dados reais (passo 5).
- Fila com o visual de esteira (relato do dono, com vídeo): a fila usava os lugares da forma
  simples (caixa baixa de 0,3 m, três faixas, camadas a cada 0,32 m) e a ponte só baixava os
  itens ao chão, então as caixas ficavam no chão dentro da armação da esteira e as que
  sobravam subiam em escada ao lado. Agora `utsim_look::visual::queue_belt` dá a esteira de
  verdade da aparência (comprimento, largura e altura do topo dos rolos da `ConveyorSpec`) e
  os itens em repouso ficam numa linha em cima dos rolos, com uma segunda camada por cima e o
  resto contado no rótulo ("+N"); um item voando para a fila ou saindo dela não afunda nos
  rolos. Testes: todos os lugares sobre o topo e dentro da área da esteira
  (`utsim-look`) e os itens de um replay de verdade em cima da esteira (`playback.rs`).
- Conferência com janela a 1600×900 e 1280×720 (`g3_resultados.json` inteiro e uma fila com
  esteira de 27 e 231 itens). Achados e correções: os cartões saíam altos e com o texto em linha
  (o conteúdo herdava o layout horizontal); a legenda cobria as barras dos estados e as curvas;
  o eixo da ocupação ia a 120 %; o nome longo do eixo do aquecimento passava sobre o texto; a
  série plana tinha o eixo y de -2000 a 2000.
- Achado no caminho: um `
` dentro de uma cadeia do `roteiro.gd` (o `digitar` com quebras de
  linha) quebra se o arquivo for gravado com CRLF; os arquivos ficam com LF (`.gitattributes`).

### Passo 5a (PERT/CPM, fluxos, painéis das ferramentas, dados reais, janelas), como foi feito

- **PERT/CPM** (`ui_pert.rs`): a mesma janela sobre a área livre (uma janela por vez com
  Resultados), com os modos Simulado, Clássico e Monte Carlo, prazo (Enter recalcula),
  sorteios, exportação HTML e CSV e as abas Rede (caixas e setas na disposição da ponte, o
  caminho crítico em vermelho, setas mais grossas com a criticidade), Gantt, Distribuição
  (histograma com o prazo), Tabela e Texto. O `principal.gd` só abre (`open_pert`), pede o
  cálculo (`pert_calculate`, com `pert_form`) e devolve o resultado (`set_pert`); as
  conferências leem `pert_data()`.
- **Fluxos de tokens** (`ui_flows.rs`): o grafo de cada fluxo (caixas da largura do texto,
  colunas da ponte, setas para a frente e de volta por baixo), zoom com −, +, Ajustar e
  Ctrl + roda, e as contagens ao longo da rodada com a linha do instante. A ponte calcula as
  séries uma vez (`set_flows_playback`) e as contagens no instante (`flows_time`).
- **Painéis das ferramentas** (`ui_form.rs`): Rede, Corredor e a ligação selecionada são
  linhas que o `principal.gd` descreve em JSON (`show_form`: título, texto, ajuda, aviso, erro,
  caixa, número, linha, escolha, botão) e que voltam como `form:<ação>:<valor>`. Os nomes dos
  controles continuam (`Rede_TrechoCapacidade`, `Corredor_NovoTipo`, `Ligacao_Remover`...).
- **Dados reais** (`ui_props.rs`): "Fonte: Dados reais" abre o formulário no próprio painel
  (origem, arquivo com Escolher..., coluna lida do arquivo, valores colados, instantes,
  escolha, critério, Aplicar) e, aplicado, o selo "dados", o resultado, os parâmetros, o
  mini-histograma com a curva, os avisos, as candidatas e o relatório. As ações voltam como
  `dados	<ação>	<campo>	<rascunho JSON>` e o `principal.gd` usa as mesmas funções de antes.
- **Janelas**: a confirmação de descarte e os atalhos (F1) são modais do egui
  (`show_dialog`); a linha de desempenho (F3) é desenhada pelo egui sobre a vista. As janelas
  de arquivo são as do sistema (`FileDialog.use_native_dialog`); sem tela, ou com um roteiro
  rodando, ficam as do Godot, que o passo `dialogo` responde.
- Removidos: `painel_propriedades.gd`, `dados_reais.gd`, `mini_histograma.gd`, `pert.gd`,
  `fluxos.gd`, `grafico.gd` e o tema `godot/tema/` (o projeto não tem mais tema Godot). No
  GDScript ficam só a vista e as ferramentas 3D (`principal.gd`, `editor.gd`, `cena.gd`,
  `rede.gd`, `corredores.gd`, `camera_orbita.gd`, `pessoa.gd`) e o `roteiro.gd`.
- Conferência com janela a 1600×900 e 1280×720 (`g3_pert`, `g3_dados_reais`, `g3_rede`,
  `g4_corredores`, `g3_cena` e um roteiro das janelas). Achados e correções: o texto das caixas
  do PERT passava da borda de baixo; a nota dos fluxos falava de uma linha amarela que é azul;
  as janelas de arquivo do sistema não deixavam os roteiros com janela responder.

### Passo 5b, ligações de equipe e equipe ao vivo (E1b, E8b), como foi feito

- A ponte manda as linhas de `utsim_scene::team_links` com os blocos desenhados
  (`UtsimPlayback.team_links()`), o destino da tarefa de cada recurso móvel no quadro
  (`task_target`, de `travel_target`) e, no rótulo do despachante, quantos membros estão
  livres. O `editor.gd` desenha as linhas numa malha própria, visível no Editar e no Simular
  (decisão D5: apagadas no Simular, acesas as do bloco seguido), o anel de estado dos membros e
  a linha da tarefa do seguido.
- Ferramenta **Equipe** (decisão D4: ferramenta própria, tecla E, e `Alt` com Ligar):
  `UtsimDocument.team_link(de, para, operadores)` decide pelo tipo dos dois objetos (equipe,
  transporte ou, com Shift, operadores), com as operações de `utsim_edit::teams`, um passo de
  desfazer, e diz o que mudou.
- O menu ▼ de Analisar ganhou os interruptores das linhas por tipo (`Ligacoes_<tipo>`); para a
  barra caber a 1600 px com a ferramenta nova, o botão Atalhos virou "?" (a dica diz F1).
- Roteiro `g4_equipes.json` (tira e põe um membro, transporte, operadores, desfazer, Simular
  seguindo um membro) e o teste `team_lines_and_the_free_members_come_with_the_frame`.

### Passo 5b, a fonte no 3D (C7), como foi feito

- As fontes e as saídas mostram sempre as portas no Editar (`editor._port_blocks`).
- Sobre cada fonte, uma caixinha na cor que os seus itens recebem (`preview_color` da cena
  estática, `style::origin_color`).
- No rótulo da fonte, o selo do modo (`tabela`, `sequência`, `horário`, `lote`) e, numa
  rodada, os itens criados até o instante (dos eventos `Created` do replay). Teste
  `a_source_label_says_how_it_creates_and_how_many`.
- Ficaram para depois: o anel com as proporções dos tipos, o item preso na boca da porta com
  `on_blocked: Wait` e os perdidos piscando, e os itens de um lote lado a lado.

### Plano original do passo 2b

- O painel lê o esquema com `schema::properties` sobre o `utsim_edit::Document` do
  `UtsimDocument` (o nó recebe o documento uma vez, `set_document(doc)`) e aplica cada mudança
  como `Command::Set` pela mesma validação de hoje; depois avisa o `principal.gd`
  (`after_edit`) pelo sinal `action("editado:<caminho>")`.
- Campos: rótulo à esquerda e controle à direita numa grade de duas colunas alinhadas;
  seções recolhíveis (o estado fica por título, como hoje); problemas em vermelho e avisos
  em amarelo, clicáveis; texto longo quebra em várias linhas; "Aplicar" como botão
  primário; distribuição com família, parâmetros e "editar como texto".
- Nomes `Prop_<caminho>` com os sufixos de hoje (`_ligado`, `_familia`, `_texto`, `_ron`,
  `_aplicar`, `Secao_<título>`, `Problemas_<k>`, `Avisos_<k>`), para os roteiros.
- A miniatura do ativo (um mundo 3D pequeno) continua desenhada pelo Godot num
  `SubViewport`; o egui mostra a textura dele (`TextureId::User`) no lugar.
- Os subpainéis de rede, corredores e ligação e o formulário de dados reais vêm no passo 5;
  até lá, quando a ferramenta Rede ou Corredor estiver ativa, o painel Godot de hoje é
  mostrado no lugar do painel egui.

## O que esta frente entregou

- Crate `utsim-ui-tema` (tema, fontes, botão primário, controle segmentado) com testes de
  contraste e de layout.
- Na ponte: `egui_input.rs`, `egui_mesh.rs`, `egui_widgets.rs`, `egui_host.rs` (puros, com
  testes) e `ui_class.rs` (o nó `UtsimUi`); `ui_bar.rs` (a barra superior) e `ui_props.rs`
  (o painel de propriedades).
- Testes dos comandos de desenho sem o motor (`egui_host::tests::the_canvas_commands_*`): a
  barra e o painel a 1600×900 e 1280×720 geram malhas com índices válidos, coordenadas de
  textura em 0..1, cores em alfa direto em 0..1, recortes dentro da janela e só texturas já
  criadas. É o que dá para conferir sem renderizador; a imagem em si só a janela mostra.
- Um clique no egui tira o foco de um campo Godot (`gui_release_focus`).
- `principal.gd` sem a barra superior em GDScript: `_sync_bar()` e `bar_action()`; o
  `roteiro.gd` acha widgets do egui pelo nome.
- Conferência visual com janela (roteiros g2_tutorial_01, g2_montar_linha, g3_cena,
  g3_resultados e um roteiro do editor com um modelo aberto, a 1600×900 e 1280×720). Achados
  e correções:
  - o recorte dos canvas items (`canvas_item_set_clip` com o retângulo próprio) não cortava
    o que o egui desenha num `Node2D`: o texto do painel rolado passava por cima da barra de
    status. Os triângulos agora são cortados na CPU (`egui_mesh::clip_triangles`, com cor e
    coordenada de textura interpoladas), e os testes conferem que nenhum vértice sai do
    recorte;
  - as setas pequenas (▾ ▸) e o ✕ não existem na fonte de interface e saíam como caixas: os
    menus usam um ▼ pequeno (`widgets::down_arrow`), as seções um triângulo desenhado
    (`widgets::paint_chevron`) e a remoção de ligação um ×; as seções ficaram alinhadas à
    esquerda;
  - a unidade dos campos numéricos (m, s, graus) só aparecia depois de uma edição;
  - os rótulos ficaram centrados na altura dos campos e a borda das caixas de marcar mais
    clara (as desmarcadas quase sumiam no painel);
  - dentro do menu "Mais", "Organizar" ficava com duas setas.
  Fontes (acentos ã ç é), cores e transparência, dicas ao parar o mouse, o menu "Mais" a
  1280 e a rolagem do painel sem mexer na câmera ficaram certos. A 1280×720, os roteiros
  g2_tutorial_01 e g2_montar_linha falham ao soltar objetos em pontos do chão escritos para
  1600×900 (o ponto fica sob o painel da direita, de mesma largura que o painel Godot de
  antes); são roteiros feitos para 1600×900.

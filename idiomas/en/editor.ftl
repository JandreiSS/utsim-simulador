# UTSim: the simulator editor (shell: bar, layers, panels and general dialogs).

ui-sem_observacoes = no observations
ui-resumo = n={ $n }  mean={ $media }  sd={ $dp }  min={ $min }  max={ $max }
ui-recebidos = received: { $n }
ui-simulacao = Simulation
ui-modelo_replicacao = { $modelo } - replication { $n }
ui-contagem = { $blocos ->
    [one] { $blocos } block
   *[other] { $blocos } blocks
}, { $itens ->
    [one] { $itens } item
   *[other] { $itens } items
}, { $eventos ->
    [one] { $eventos } trace event
   *[other] { $eventos } trace events
}
ui-pausar = Pause
ui-reproduzir = Play
ui-evento_anterior = < Event
ui-evento_seguinte = Event >
ui-inicio = Start
ui-velocidade = speed x{ $v }
ui-unidades_por_segundo = simulated time units per second
ui-tempo = time
ui-tempo_simulado = simulated time: { $t } of { $fim }
ui-ajuda_teclas = Space: play/pause | +/-: speed | arrows: previous/next event | F: frame
ui-ajuda_mouse = Mouse: left drags (moves) and clicks (selects), right rotates, wheel zooms
ui-ajuda_camera = Camera: 1 orbit, 2 follow, 3 first person, 4 from the resource | F3: frame times
ui-blocos = Blocks
ui-clique_em_um_bloco = Click a block to see its statistics.
ui-capacidade_rotulo = capacity { $n }
ui-nivel_agora = level now
ui-capacidade = capacity
ui-ocupacao = occupancy
ui-taxa_agora = rate now
ui-nivel_inicial = initial level
ui-entrada_maxima = maximum inflow
ui-saida_maxima = maximum outflow
ui-vazao_da_fonte = source flow rate
ui-volume_por_item = volume per item
ui-acumulado = accumulated for the next item
ui-vazao_agora = flow rate now
ui-vazao_maxima = maximum flow rate
ui-objeto = object
ui-por_unidade_de_tempo = { $taxa } per time unit ({ $sentido })
ui-nivel_interpolado = Level and rate interpolated from the trace of the fluid network.
ui-seguir_recurso = Follow this resource
ui-seguir_recurso_dica = The camera follows the resource from behind and above (2)
ui-ver_do_recurso = View from the resource
ui-ver_do_recurso_dica = The camera goes to the driver's position (4)
ui-recebidos_ate_agora = received so far
ui-estado_agora = state now
ui-conteudo_agora = content now
ui-entradas = inputs
ui-saidas = outputs
ui-conteudo_medio = mean content
ui-conteudo_maximo = maximum content
ui-tempo_de_permanencia = time in block
ui-fracao_em = fraction in { $estado }
ui-estatisticas_finais = Final statistics of the replication (window after the warm-up).
camadas-no_excluido = Node { $nome } and its edges deleted (Ctrl+Z undoes)
camadas-aresta_excluida = Edge { $de } - { $para } deleted (Ctrl+Z undoes)
camadas-no_fluido_excluido = Node { $nome } and its links deleted (Ctrl+Z undoes)
camadas-objetos_dica = Model objects: library, connections and properties
camadas-rede_dica = Path network of the mobile resources: nodes and edges
camadas-fluidos_dica = Fluid network: sources, tanks, pipes and converters
camada-objetos = Objects
camada-rede = Network
camada-fluidos = Fluids
camadas-rede_de_caminhos = Path network
camadas-selecionar = Select
camadas-selecionar_rede_dica = Click selects a node or an edge; dragging moves the node
camadas-no = Node
camadas-no_dica = Click on the floor places a node
camadas-aresta = Edge
camadas-aresta_dica = Click a node and then another; clicking on the floor creates a node already linked
camadas-mao_dupla = New edges are two-way
camadas-ajuda_aresta = Edge: click from node to node to draw the path; Esc ends.
camadas-ajuda_del_rede = Del deletes the selected node (with its edges) or edge.
camadas-ajuda_atendido = Each served object joins the network at the nearest node (dashed line).
camadas-rede_de_fluidos = Fluid network
camadas-selecionar_fluidos_dica = Click selects a node; dragging moves the node
camadas-ligar = Link
camadas-ligar_dica = Click the source node and then the destination (the flow follows the arrow)
camadas-colocar = Place
camadas-colocar_dica = Click on the floor places the node (Shift keeps the tool)
camadas-ajuda_aciclica = The network must be acyclic. The converters link the network to an object.
camadas-ajuda_del_fluidos = Del deletes the selected node and its links.
camadas-aresta_daqui = Edge from here
camadas-ligar_daqui = Link from here
camadas-sem_rede = The model has no path network.
camadas-sem_rede_ajuda = Without a network, the mobile resources move in a straight line. Use "Node" to place the nodes and "Edge" to link them.
camadas-resumo_rede = { $nos ->
    [one] { $nos } node
   *[other] { $nos } nodes
} and { $arestas ->
    [one] { $arestas } edge
   *[other] { $arestas } edges
}. Select one to edit.
camadas-nos = Nodes ({ $n })
camadas-arestas = Edges ({ $n })
camadas-sem_fluidos = The model has no fluid network.
camadas-sem_fluidos_ajuda = Choose a type in "Place" and click on the floor; then use "Link".
camadas-resumo_fluidos = Network "{ $rede }": { $nos ->
    [one] { $nos } node
   *[other] { $nos } nodes
} and { $ligacoes ->
    [one] { $ligacoes } link
   *[other] { $ligacoes } links
}. Select a node to edit.
editor-layout_aplicado = Automatic layout applied: save to keep the positions
editor-layout_com_rede = Model with a path network: place the objects on the network nodes (the automatic layout was not applied)
editor-sem_titulo = untitled.ron
editor-excluidos = { $n } objects deleted (Ctrl+Z undoes)
editor-nada_para_copiar = Nothing selected to copy
editor-copiados = { $objetos ->
    [one] { $objetos } object
   *[other] { $objetos } objects
} and { $ligacoes ->
    [one] { $ligacoes } connection copied
   *[other] { $ligacoes } connections copied
}
editor-recortados = { $n ->
    [one] { $n } object cut (Ctrl+V pastes)
   *[other] { $n } objects cut (Ctrl+V pastes)
}
editor-area_vazia = The clipboard is empty (Ctrl+C copies)
editor-colados = { $n ->
    [one] { $n } object pasted: { $nomes }
   *[other] { $n } objects pasted: { $nomes }
}
editor-nada_para_duplicar = Nothing selected to duplicate
editor-salvo_em = Saved to { $arquivo }
editor-erro_ao_salvar = Error saving: { $erro }
editor-tempo_invalido = Invalid maximum time: use a number greater than zero
editor-simulando = Simulating...
editor-sem_resultado = the simulation ended without a result
editor-erro_na_simulacao = Simulation error: { $erro }
editor-na_caixa = { $n ->
    [one] { $n } object in the box
   *[other] { $n } objects in the box
}, { $sel ->
    [one] { $sel } selected
   *[other] { $sel } selected
}
editor-outro_destino = Choose another block as the destination
editor-ligacao_removida = Connection { $de } -> { $para } removed (Ctrl+Z undoes)
editor-editar = Edit
editor-simular = ▶ Simulate
editor-ate_t = until t =
editor-novo = New
editor-salvar = Save
editor-salvar_como_botao = Save as...
editor-desfazer = Undo
editor-refazer = Redo
editor-enquadrar = Frame (F)
editor-fluxos = Flows
editor-fluxos_dica = User logic: token flows
editor-resultados = Results
editor-resultados_dica = Replications, summary with confidence intervals, charts, scenarios and optimization
editor-atalhos = Shortcuts
editor-atalhos_dica = Keyboard and mouse shortcuts (F1)
editor-salvar_como = Save as
editor-caminho_do_arquivo = File path (.ron), relative to the program folder or full:
editor-sobrescrito = This file already exists and will be overwritten.
editor-cancelar = Cancel
editor-novo_modelo = New model
editor-descartar_pergunta = There are unsaved changes. Discard them and start a new model?
editor-descartar = Discard
editor-atalhos_do_editor = Editor shortcuts
editor-f1_fecha = F1 or Esc closes this window.
editor-nao_simula = Cannot simulate
editor-corrija = Fix the problems of the model before simulating:
editor-fechar = Close
editor-valores_gravados = Values written to the model (Ctrl+Z undoes)
editor-nenhum_problema = No problems: the model can be simulated.
editor-modelo = Model
editor-biblioteca = Library
editor-propriedades = Properties
editor-selecione_um_bloco = Select a block to edit its properties.
editor-ligar = Connect (C)
editor-excluir = Delete (Del)
editor-clique_no_destino = Click the destination block (Esc cancels).
editor-problemas = Problems ({ $n })
editor-avisos = Warnings ({ $n })
editor-colocar_dica = { $tipo }: click and then click on the floor, or drag to the floor
editor-encaixar = Snap to grid
editor-passo = Step (m)
editor-esc_cancela = Esc cancels. Shift + click on the floor places several.
editor-atalhos_f1 = Shortcuts: F1.
editor-selecionados = { $n } objects selected
editor-copiar = Copy
editor-duplicar = Duplicate
editor-excluir_todos = Delete all
editor-alinhar = Align
editor-esquerda = Left
editor-centro_x = Center X
editor-direita = Right
editor-topo = Top
editor-centro_y = Center Y
editor-base = Bottom
editor-distribuir_x = Distribute in x
editor-distribuir_y = Distribute in y
editor-distribuir_dica = Spaces evenly between the two ends (3 objects or more)
editor-propriedades_de_todos = Properties of all
editor-campos_de_todos = The fields show the values of { $primeiro }; each change applies to the { $n } objects.
editor-buscar_por_nome = Search by name
editor-todos_os_tipos = All kinds
editor-de_objetos = { $n } of { $total } objects
editor-objeto_dica = { $tipo }. Click selects, Shift adds to the selection, double click frames
editor-subir = Up
editor-descer = Down
editor-subir_dica = Swaps places with the previous one (Ctrl+Z undoes)
editor-descer_dica = Swaps places with the next one (Ctrl+Z undoes)
editor-remover_ligacao = Remove connection
editor-ligacoes = Connections
editor-nenhuma_ligacao = None. Use "Connect" and click the destination.
editor-ordem_importa = The order matters. Outputs: the order is the port number (1, 2, ...) that the output rules use. Inputs: the first has priority when the object pulls an item.
editor-saidas_por_porta = Outputs, by port:
editor-porta = port { $n }: { $de } -> { $para }
editor-entradas_por_prioridade = Inputs, by priority:
editor-entrada = #{ $n }: { $de } -> { $para }
atalho-01-teclas = Click
atalho-01 = selects the block (or removes the clicked connection)
atalho-02-teclas = Repeated click at the same spot
atalho-02 = cycles through the overlapping objects (a hint shows which one is selected)
atalho-03-teclas = Shift + click
atalho-03 = adds the block to or removes it from the selection
atalho-04-teclas = Shift + drag on the floor
atalho-04 = box selection (adds to the selection)
atalho-05-teclas = Drag a block
atalho-05 = moves the block or the whole selection
atalho-06-teclas = Drag on the floor
atalho-06 = pans the view
atalho-07-teclas = Right button + drag
atalho-07 = rotates the view
atalho-08-teclas = Mouse wheel
atalho-08 = zooms in and out, towards the cursor
atalho-09-teclas = Objects tab: click, Shift + click
atalho-09 = selects, adds to the selection; a double click frames the object
atalho-10-teclas = Library: click and click on the floor
atalho-10 = places an object (Shift keeps the tool)
atalho-11-teclas = Library: drag to the floor
atalho-11 = places an object where you drop it
atalho-12-teclas = Arrows
atalho-12 = moves the selection one grid step
atalho-13-teclas = Del
atalho-13 = deletes the selection
atalho-14-teclas = C
atalho-14 = connects the selected block to the next one clicked
atalho-15-teclas = Esc
atalho-15 = cancels the tool, closes the dialogs and leaves the follow and first-person cameras
atalho-16-teclas = F
atalho-16 = frames the model (back to the orbit camera)
atalho-17-teclas = 1, 2, 3, 4
atalho-17 = camera: orbit, follow a resource, first person, from the resource
atalho-18-teclas = W A S D
atalho-18 = walks in first person (Shift runs, the wheel changes the speed)
atalho-19-teclas = L
atalho-19 = locks the mouse in first person
atalho-20-teclas = F1
atalho-20 = shows or hides this window
atalho-21-teclas = F3
atalho-21 = shows or hides the frame times
atalho-22-teclas = F4
atalho-22 = turns the levels of detail on or off
atalho-23-teclas = Ctrl + A
atalho-23 = selects all objects
atalho-24-teclas = Ctrl + C
atalho-24 = copies the selection
atalho-25-teclas = Ctrl + X
atalho-25 = cuts the selection
atalho-26-teclas = Ctrl + V
atalho-26 = pastes (unique names, internal connections kept)
atalho-27-teclas = Ctrl + D
atalho-27 = duplicates the selection
atalho-28-teclas = Ctrl + Z
atalho-28 = undoes
atalho-29-teclas = Ctrl + Y or Ctrl + Shift + Z
atalho-29 = redoes
atalho-30-teclas = Ctrl + S
atalho-30 = saves
atalho-31-teclas = Ctrl + Shift + S
atalho-31 = saves under another name (Save as)
atalho-32-teclas = Ctrl + N
atalho-32 = new model

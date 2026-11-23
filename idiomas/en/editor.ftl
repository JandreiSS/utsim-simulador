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

# Part 2: the properties panel (properties/).

## Properties: distribution families, rules and kinds

dist-constante = Constant
dist-uniforme = Uniform
dist-exponencial = Exponential
dist-triangular = Triangular
dist-normal = Normal
dist-lognormal = Lognormal
dist-gama = Gamma
dist-erlang = Erlang
dist-weibull = Weibull
dist-beta = Beta
dist-log-logistica = Log-logistic
dist-pearson-v = Pearson V
dist-pearson-vi = Pearson VI
dist-discreta = Discrete
dist-empirica = Empirical
dist-poisson = Poisson
dist-geometrica = Geometric
dist-binomial = Binomial
dist-binomial-negativa = Negative binomial
dist-deslocada = Shifted
prop-vaga-primeira-livre = First free
prop-vaga-aleatoria = Random
prop-despacho-primeiro-livre = First available
prop-despacho-rodizio = Round robin
prop-despacho-mais-proximo = Nearest
tipo-chegada = Arrival
tipo-fila = Queue
tipo-processo = Process
tipo-montagem = Assembly
tipo-desmontagem = Disassembly
tipo-saida = Exit
tipo-transportador = Transporter
tipo-ponte-rolante = Overhead crane
tipo-despachante = Dispatcher
tipo-esteira = Conveyor
tipo-porta-paletes = Pallet rack
prop-modo-igual-ao-padrao = same as default
prop-modo-distribuicao = distribution
prop-modo-taxa = rate
prop-modo-sem-chegadas = no arrivals
prop-turno-pausar = Pause
prop-turno-terminar = Finish
prop-turno-pausar-dica = Work in progress freezes at the end of the shift and resumes when the shift comes back
prop-turno-terminar-dica = At the end of the shift the object takes no new items, finishes the item in progress (overtime) and only then goes off shift; the next shift starts at the usual time

## properties/network.rs

prop-network-o_nome_nao_pode_ficar = the name cannot be empty
prop-network-ja_existe_um_no_chamado = there is already a node called "{ $t }"
prop-network-nome = Name
prop-network-no_da_rede_de_caminhos = Path network node
prop-network-arestas = Edges
prop-network-nenhuma_use_a_ferramenta_aresta = None: use the "Edge" tool and click another node.
prop-network-remover_aresta = Remove edge
prop-network-objetos_ligados_a_este_no = Objects linked to this node
prop-network-nenhum_cada_objeto_atendido_entra = None (each served object enters through the nearest node).
prop-network-aresta = Edge { $edge }
prop-network-comprimento_m_calculado_pelas_posicoes = Length: { $len } m (computed from the node positions)
prop-network-mao_dupla = Two-way ({ $from } <-> { $to })
prop-network-mao_unica = One-way ({ $from } -> { $to })
prop-network-mao_unica-2 = One-way ({ $to } -> { $from })
prop-network-sentido = Direction
prop-network-limitar_veiculos_agv = Limit vehicles (AGV)
prop-network-quantos_agvs_podem_estar_na = How many AGVs can be on the edge at the same time
prop-network-sem_limite = no limit
prop-network-so_agvs_agv_true_respeitam = Only AGVs (agv: true) respect the capacity; the others pass freely.
prop-network-objeto_discreto = Discrete object
prop-network-escolha = (choose)
prop-network-nao_e_um_objeto_discreto = "{ $current }" is not a discrete object of the model
prop-network-vazao = Flow rate
prop-network-capacidade = Capacity
prop-network-nivel_inicial = Initial level
prop-network-entrada_maxima = Maximum inflow
prop-network-saida_maxima = Maximum outflow
prop-network-vazao_maxima = Maximum flow rate
prop-network-cada_item_que_entra_no = Each item entering the object becomes this volume, delivered at once.
prop-network-volume_por_item = Volume per item
prop-network-a_cada_volume_completo_cria = For each full volume, creates an item inside the object.
prop-network-problema = Problem: { $message }
prop-network-tipo = Kind
prop-network-ligacoes = Links
prop-network-nenhuma_use_a_ferramenta_ligar = None: use the "Link" tool and click the next node.
prop-network-remover_ligacao = Remove link

## properties/panel.rs

prop-panel-buscar = Search
prop-panel-texto_invalido_nao_aplicado = Invalid text, not applied: { $error }
prop-panel-descartar_texto = discard text
prop-panel-volta_ao_valor_que_esta = Goes back to the value in the model
prop-panel-nao_e_um_numero = "{ $s }" is not a number
prop-panel-infinita = infinite
prop-panel-valor = value
prop-panel-peso = weight
prop-panel-remover = remove
prop-panel-adicionar_valor = add value
prop-panel-observacoes_em_ordem_crescente_separadas = Observations in ascending order, separated by spaces:
prop-panel-deslocamento = shift
prop-panel-min = min.
prop-panel-max = max.
prop-panel-media = mean
prop-panel-moda = mode
prop-panel-desvio = std. dev.
prop-panel-forma = shape
prop-panel-escala = scale
prop-panel-alfa = alpha
prop-panel-beta = beta
prop-panel-forma_1 = shape 1
prop-panel-forma_2 = shape 2
prop-panel-media-2 = mean = { $mean }
prop-panel-sem_limite = no limit
prop-panel-sem_limite_velocidade_constante = no limit (constant speed)
prop-panel-equipe_transportadores_do_modelo = Team (transporters of the model):
prop-panel-nenhum_transportador_no_modelo_crie = No transporter in the model. Create one in the palette.
prop-panel-nao_e_um_transportador = "{ $missing }" is not a transporter
prop-panel-a_equipe_nao_pode_ficar = The team cannot be empty.
prop-panel-precisa_de_operadores = Needs operators
prop-panel-crie_um_transportador_ou_despachante = Create a Transporter or Dispatcher to use as the team.
prop-panel-equipe = team
prop-panel-nenhuma = (none)
prop-panel-nao_e_um_transportador_nem = "{ $team }" is neither a transporter nor a dispatcher
prop-panel-quantidade = count
prop-panel-durante_a_preparacao = during setup
prop-panel-durante_o_processamento = during processing
prop-panel-transporte_de_saida = Outbound transport
prop-panel-nenhum = none
prop-panel-os_itens_passam_direto_ao = Items go straight to the next block.
prop-panel-nao_e_um_transportador_nem-2 = "{ $t }" is neither a transporter nor a dispatcher
prop-panel-sem_rede_de_caminhos_a = No path network: the base is the transporter's position.
prop-panel-base_no_da_rede = Base (network node)
prop-panel-fora_da_rede = off the network
prop-panel-escolher_um_no_poe_o = Choosing a node puts the transporter on it and turns on "Return to base".
prop-panel-editar_como_texto = edit as text
prop-panel-quantidade_de_itens_exigida_em = Number of items required at each component input:
prop-panel-entrada = input { $i }
prop-panel-adicionar_entrada = add input
prop-panel-o_objeto_precisa_de_ligacoes = The object needs { $list } input links (1 container + { $list2 }).
prop-panel-valor_do_rotulo = Label value
prop-panel-remover_rotulo = remove label
prop-panel-nome_do_rotulo = label name
prop-panel-adicionar_rotulo = add label
prop-panel-lote_agrupar_itens = Batch (group items)
prop-panel-tamanho_do_lote = batch size
prop-panel-espera_maxima_0_esperar_sempre = maximum wait (0 = always wait)
prop-panel-fechar_a_entrada_ate_o = close the input until the batch leaves
prop-panel-primeira_livre = First free
prop-panel-rodizio = Round robin
prop-panel-aleatoria = Random
prop-panel-pesos = Weights
prop-panel-por_rotulo = By label
prop-panel-script = Script
prop-panel-regra_de_saida = Output rule
prop-panel-peso_de_cada_saida_na = Weight of each output (in link order):
prop-panel-saida = output { $i } ({ $to })
prop-panel-saida-2 = output { $i }
prop-panel-ha_pesos_mas_o_objeto = There are { $w } weights, but the object has { $n_out } outputs.
prop-panel-ajustar_ao_numero_de_saidas = match the number of outputs
prop-panel-rotulo_com_o_numero_da = label holding the port number
prop-panel-o_nome_do_rotulo_nao = the label name cannot be empty
prop-panel-script_que_devolve_o_numero = Script returning the port number (1, 2, ...)
prop-panel-script_ao_entrar_um_item = Script when an item enters (on_entry)
prop-panel-roda_para_cada_item_que = Runs for each entering item, before any other rule.
prop-panel-quebras_mtbf_mttr = Breakdowns (MTBF/MTTR)
prop-panel-tempo_entre_falhas_mtbf = Time between failures (MTBF)
prop-panel-tempo_de_reparo_mttr = Repair time (MTTR)
prop-panel-transferencia_pela_distancia = Transfer by distance
prop-panel-os_itens_levam_atraso_distancia = Items take delay + distance / speed to reach the next block through direct links (no transporter)
prop-panel-velocidade = Speed
prop-panel-atraso = Delay
prop-panel-caminho = Path
prop-panel-reta = straight
prop-panel-pela_rede = through the network
prop-panel-tempo_entre_chegadas = Time between arrivals
prop-panel-primeira_chegada_em_t_0 = First arrival at t = 0
prop-panel-maximo_de_chegadas = Maximum arrivals
prop-panel-se_a_saida_estiver_bloqueada = If the output is blocked
prop-panel-esperar = Wait
prop-panel-descartar_perda = Discard (loss)
prop-panel-rotulos = Labels
prop-panel-capacidade_limitada = Limited capacity
prop-panel-disciplina = Discipline
prop-panel-fifo_primeiro_a_entrar_primeiro = FIFO (first in, first out)
prop-panel-lifo_ultimo_a_entrar_primeiro = LIFO (last in, first out)
prop-panel-capacidade = Capacity
prop-panel-preparacao_setup = Setup
prop-panel-tempo_de_processamento = Processing time
prop-panel-operadores = Operators
prop-panel-modo = Mode
prop-panel-embalar_so_o_conteiner_sai = Pack (only the container leaves)
prop-panel-juntar_componentes_destruidos = Join (components destroyed)
prop-panel-lote_todos_saem = Batch (all leave)
prop-panel-desembalar = Unpack
prop-panel-dividir_em_copias = Split into copies
prop-panel-numero_de_itens_gerados = number of items created
prop-panel-a_saida_nao_tem_parametros = The exit has no parameters.
prop-panel-velocidade_maxima = Maximum speed
prop-panel-aceleracao = Acceleration
prop-panel-desaceleracao = Deceleration
prop-panel-capacidade_itens = Capacity (items)
prop-panel-tempo_de_carga = Loading time
prop-panel-tempo_de_descarga = Unloading time
prop-panel-voltar_a_base_quando_ocioso = Return to base when idle
prop-panel-agv_controle_de_trafego_na = AGV (traffic control on the network)
prop-panel-comprimento = Length
prop-panel-comprimento_por_item = Length per item
prop-panel-acumula_itens_quando_a_saida = Accumulates items when the output blocks
prop-panel-capacidade_itens-2 = Capacity: { $item_length } items
prop-panel-colunas = Bays
prop-panel-niveis = Levels
prop-panel-permanencia_minima = Minimum dwell
prop-panel-regra_de_ocupacao = Placement rule
prop-panel-largura_da_coluna = Bay width
prop-panel-altura_do_nivel = Level height
prop-panel-capacidade_posicoes = Capacity: { $levels } positions
prop-panel-regra_de_escolha = Choice rule
prop-panel-o_nome_nao_pode_ficar = the name cannot be empty
prop-panel-ja_existe_um_objeto_chamado = there is already an object called "{ $t }"
prop-panel-nome = Name
prop-panel-problema = Problem: { $message }
prop-panel-padrao_do_tipo = (kind default)
prop-panel-aparencia = Appearance
prop-panel-ativo = Asset:
prop-panel-clipes_de_animacao_ainda_nao = Animation clips are not played yet: rest pose.
prop-panel-biblioteca_do_utsim = UTSim library
prop-panel-utsim_gerado_por_utsim_assets = UTSim (made by utsim-assets)
prop-panel-terceiros_assets_terceiros = Third party (assets/terceiros)
prop-panel-cor = Colour
prop-panel-escala-2 = Scale
prop-panel-giro_graus = Rotation (degrees)
prop-panel-restaurar_padrao = Restore default
prop-panel-volta_a_aparencia_padrao_do = Goes back to the default look of the object kind
prop-panel-simulacao = Simulation
prop-panel-tempo_de_simulacao = Simulation time
prop-panel-aquecimento = Warm-up
prop-panel-semente = Seed
prop-panel-replicacoes = Replications

## properties/position.rs

prop-position-posicao_m = Position (m)
prop-position-giro_graus = Rotation (degrees)

## properties/schedule.rs

prop-schedule-segundos = seconds
prop-schedule-minutos = minutes
prop-schedule-horas = hours
prop-schedule-use_dia_n_hh_mm = use "dia N hh:mm" (got "{ $t }")
prop-schedule-dia_invalido_em_a_partir = invalid day in "{ $t }" (from 1)
prop-schedule-use_um_numero = use a number
prop-schedule-hh_mm_so_vale_com = hh:mm only works with a day or week cycle (1440 min, 24 h, 86400 s); use a number
prop-schedule-hora_invalida_em = invalid hour in "{ $t }"
prop-schedule-minuto_invalido_em = invalid minute in "{ $t }"
prop-schedule-minuto_invalido_em_0_a = invalid minute in "{ $t }" (0 to 59)
prop-schedule-nao_e_um_horario_hh = "{ $t }" is neither a time (hh:mm) nor a number

## properties/schedule_ui.rs

prop-schedule-ui-media = { $family }, mean { $mean }
prop-schedule-ui-chegadas_por_periodo = Arrivals by period
prop-schedule-ui-a_taxa_de_chegada_muda = the arrival rate changes with the time of day
prop-schedule-ui-uma_tabela_de_periodos_que = A table of periods repeated every cycle (a day, a week). Outside the periods there are no arrivals.
prop-schedule-ui-desligado_o_tempo_entre_chegadas = Off: the time between arrivals above holds all the time.
prop-schedule-ui-ciclo = cycle
prop-schedule-ui-dia = day
prop-schedule-ui-semana = week
prop-schedule-ui-ciclo_de_tempo_do_modelo = cycle of { $v } (model time in minutes)
prop-schedule-ui-ciclo_de_tempo_do_modelo-2 = Cycle of { $cycle }: model time in { $name }. Times in hh:mm, rates per hour.
prop-schedule-ui-horarios_e_taxas_em_unidades = Times and rates in model units (with cycle 1440, a day in minutes, the table shows hh:mm).
prop-schedule-ui-taxa_por_hora = rate per hour
prop-schedule-ui-taxa = rate
prop-schedule-ui-inicio = start
prop-schedule-ui-fim = end
prop-schedule-ui-modo = mode
prop-schedule-ui-valor = value
prop-schedule-ui-esperado = expected
prop-schedule-ui-copiar_do_padrao = copy from default
prop-schedule-ui-usa_no_periodo_uma_copia = Uses a copy of the default time between arrivals in the period, to tune it only here
prop-schedule-ui-remover_o_periodo = remove the period
prop-schedule-ui-adicionar_periodo = add period
prop-schedule-ui-por_dia = {" "}(per day)
prop-schedule-ui-por_semana = {" "}(per week)
prop-schedule-ui-esperado_por_ciclo_chegadas = Expected per cycle{ $hint }: { $per_cycle } arrivals
prop-schedule-ui-na_troca_de_periodo_a = When the period changes, the pending arrival is drawn again with the process of the new period (consecutive periods with the same process keep the arrival).
prop-schedule-ui-distribuicoes_dos_periodos = Distributions of the periods
prop-schedule-ui-periodo_tempo_entre_chegadas = Period { $i }: time between arrivals
prop-schedule-ui-rotulos_por_periodo = Labels by period
prop-schedule-ui-os_rotulos_de_um_periodo = The labels of a period replace the default labels of the same name (for example, more urgent at night); the others still apply.
prop-schedule-ui-a_fonte_nao_tem_rotulos = The source has no default labels.
prop-schedule-ui-a = { $a } to { $b }
prop-schedule-ui-a-2 = { $start } to { $end }
prop-schedule-ui-periodo = Period { $i } ({ $span })

## properties/shifts_ui.rs

prop-shifts-ui-como_no_calendario = as in the calendar ({ $to_lowercase })
prop-shifts-ui-ao_fim_do_turno = At the end of the shift
prop-shifts-ui-periodo = period
prop-shifts-ui-inicio = start
prop-shifts-ui-duracao = duration
prop-shifts-ui-remover = remove
prop-shifts-ui-adicionar_janela = add window
prop-shifts-ui-fora_de_turno_do_periodo = off shift: { $off_fraction }% of the period
prop-shifts-ui-turnos_janelas_fora_de_turno = Shifts (off-shift windows)
prop-shifts-ui-usar_calendario = use calendar
prop-shifts-ui-crie_calendarios_na_secao_calendarios = Create calendars in the Calendars section of the Simulation panel (click an empty spot of the scene)
prop-shifts-ui-personalizado = custom
prop-shifts-ui-periodo_janela_s_fora_de = period { $period }, { $off } off-shift window(s)
prop-shifts-ui-o_calendario_nao_existe = the calendar '{ $name }' does not exist
prop-shifts-ui-nome_do_calendario = calendar name
prop-shifts-ui-salvar_como_calendario = save as calendar
prop-shifts-ui-outros_objetos_podem_usar_o = Other objects can use the same calendar
prop-shifts-ui-calendarios = Calendars ({ $calendars })
prop-shifts-ui-tabelas_de_turno_com_nome = Named shift tables. An object uses one of them in Shifts > use calendar; changing the calendar changes every object that uses it.
prop-shifts-ui-nenhum_objeto_usa_este_calendario = no object uses this calendar
prop-shifts-ui-usado_por = used by: { $users }
prop-shifts-ui-remover_calendario = remove calendar
prop-shifts-ui-troque_os_objetos_para_outro = Switch the objects to another calendar or to custom first
prop-shifts-ui-novo_calendario = new calendar
prop-shifts-ui-o_nome_nao_pode_ficar = the name cannot be empty
prop-shifts-ui-ja_existe_um_calendario_chamado = there is already a calendar called "{ $t }"
prop-shifts-ui-nome = Name

## properties/spec.rs

prop-spec-continuas = Continuous
prop-spec-discretas_e_empirica = Discrete and empirical
prop-spec-combinacao = Combination

# Parte 3: os resultados (results.rs, results_panel.rs)
tipo-fluxo = Flow
tipo-fluidos = Fluids
tipo-medicao-de-tempo = Time measurement
tipo-outro = Other
res-status-replicacoes = { $n ->
    [one] { $n } replication finished
   *[other] { $n } replications finished
}
res-status-replicacoes-gargalo = { $n ->
    [one] { $n } replication finished
   *[other] { $n } replications finished
}; bottleneck: { $gargalo }
res-status-pontos = { $n ->
    [one] { $n } point evaluated
   *[other] { $n } points evaluated
}
res-status-pontos-melhor = { $n ->
    [one] { $n } point evaluated
   *[other] { $n } points evaluated
}; best: { $melhor }

## results.rs

res-replicacao_oes_de_t_0 = { $replications ->
    [one] { $replications } replication
   *[other] { $replications } replications
}, from t = 0 to t = { $stop_time } with warm-up until t = { $warmup }, seed { $seed }
res-ocupacao = Utilization
res-incluindo_bloqueado = Including blocked
res-dentro_do_turno = Within the shift
res-sem_o_tempo_bloqueado = Without blocked time
res-fracao_do_tempo_trabalhando_em = Fraction of time working (processing or setting up). With capacity above 1, it is the mean number of busy servers divided by the capacity.
res-ocupacao_mais_o_tempo_em = Utilization plus the time the resource holds a finished item that cannot leave.
res-ocupacao_sobre_o_tempo_em = Utilization over the time on shift: time off shift leaves the denominator.
res-ocupacao_sobre_o_tempo_nao = Utilization over the unblocked time: shows the resource’s own load.
res-objeto = Object
res-tipo = Kind
res-entradas = Inputs
res-saidas = Outputs
res-conteudo_medio = Mean content
res-permanencia_media = Mean stay time
res-c_bloqueio = w/ blocking
res-no_turno = on shift
res-s_bloqueio = w/o blocking
res-velocidade_da_transferencia = { $name }: transfer speed
res-simulacao = Simulation: { $label }
res-simulacao_velocidade_padrao_das_transferencias = Simulation: default transfer speed
res-cenario = scenario { $n }
res-adicione_pelo_menos_um_cenario = add at least one scenario
res-cenario_de_um_nome = scenario { $i }: give it a name
res-cenario_nome_repetido = scenario '{ $name }': repeated name
res-cenario-2 = scenario '{ $name }': { $e }
res-restricao_de_nao_e_um = constraint on '{ $metric }': { $what } '{ $text }' is not a number
res-escolha_a_metrica_do_objetivo = choose the objective metric
res-minimo = minimum
res-maximo = maximum
res-restricao_de_informe_minimo_ou = constraint on '{ $metric }': give a minimum or a maximum

## results_panel.rs

res-painel-resumo = Summary
res-painel-graficos = Charts
res-painel-cenarios = Scenarios
res-painel-otimizacao = Optimization
res-painel-ocupacao = Utilization
res-painel-estados = States
res-painel-serie_temporal = Time series
res-painel-histograma = Histogram
res-painel-a_execucao_terminou_sem_resultado = the run ended without a result
res-painel-cenario_s_comparados = { $scenarios ->
    [one] { $scenarios } scenario compared
   *[other] { $scenarios } scenarios compared
}
res-painel-cancelado = { $what }: cancelled
res-painel-erro = { $what }: error
res-painel-rodando_replicacoes = Running replications
res-painel-comparando_cenarios = Comparing scenarios
res-painel-otimizando = Optimizing
res-painel-resultados = Results
res-painel-o_modelo_tem_problema_s = The model has { $blocked ->
    [one] { $blocked } problem
   *[other] { $blocked } problems
}: fix them in Edit before running.
res-painel-de = { $what }: { $done } of { $total }
res-painel-cancelar = Cancel
res-painel-rodada = Run
res-painel-replicacoes = Replications
res-painel-semente = Seed
res-painel-tempo_de_parada = Stop time
res-painel-aquecimento = Warm-up
res-painel-series_temporais_a_cada = Time series every
res-painel-histogramas_com_classes = Histograms with bins
res-painel-rodar_replicacoes = Run replications
res-painel-gravar_no_modelo = Save to the model
res-painel-grava_replicacoes_tempos_e_semente = Saves replications, times and seed in the model (Ctrl+Z undoes)
res-painel-restaurar_do_modelo = Restore from the model
res-painel-rode_as_replicacoes_para_ver = Run the replications to see the summary per object, the bottleneck and the charts.
res-painel-cada_valor_media_entre_replicacoes = { $describe }. Each value: mean across replications ± half-width of the 95% CI.
res-painel-gargalo_geral = Bottleneck (overall):
res-painel-ocupacao_a_maior_entre_processos = (utilization { $content }; the highest among processes and resources)
res-painel-gargalo_dentro_do_turno = Bottleneck (within the shift):
res-painel-ocupacao_no_tempo_em_turno = (utilization { $content } over the time on shift)
res-painel-tipo = Kind:
res-painel-todos = all
res-painel-clique_no_titulo_de_uma = Click a column title to sort.
res-painel-decr = {" "}(desc.)
res-painel-cresc = {" "}(asc.)
res-painel-metricas_proprias_tempo_no_sistema = Custom metrics (time in system, flows, tank levels...)
res-painel-pasta_de_exportacao = Export folder:
res-painel-exportar_csv = Export CSV
res-painel-resumo_csv_objetos_csv_replicacoes = resumo.csv, objetos.csv, replicacoes.csv and, if any, series.csv and histogramas.csv
res-painel-exportar_html = Export HTML
res-painel-relatorio_com_tabelas_e_graficos = Report with tables and charts (includes the scenario comparison, if any)
res-painel-a_exportacao_de_arquivos_existe = Exporting files is only available in the desktop app.
res-painel-csv_gravado_em = CSV saved in { $dir }: { $files }
res-painel-nao_foi_possivel_gravar_em = could not write to { $dir }: { $e }
res-painel-relatorio_gravado_em = Report saved to { $file }
res-painel-nao_foi_possivel_gravar = could not write { $file }: { $e }
res-painel-rode_as_replicacoes_na_aba = Run the replications in the Summary tab to see the charts.
res-painel-o_modelo_nao_tem_processos = The model has no processes or mobile resources.
res-painel-de_cada_recurso_com_o = { $label } of each resource (%), with the 95% interval, never above 100% per server. In orange, the bottleneck (highest utilization).
res-painel-fracao_do_tempo_em_cada = Fraction of time in each state (%), with the 3D colors.
res-painel-objeto = Object:
res-painel-este_objeto_nao_tem_observacoes = This object has no stay-time observations.
res-painel-a_rodada_foi_feita_sem = The run was made without histograms.
res-painel-medida = Measure:
res-painel-ativar_series_temporais_e_rodar = Turn on time series and run again
res-painel-ativar_histogramas_e_rodar_de = Turn on histograms and run again
res-painel-ic_95 = 95% CI
res-painel-sem_estados_registrados = No states recorded.
res-painel-este_objeto_nao_tem_pontos = This object has no sampled points.
res-painel-a_rodada_foi_feita_sem-2 = The run was made without time series (run.sample_interval off).
res-painel-conteudo_de_ao_longo_do = Content of { $object } over time: mean of the replications (bold line) and each replication.
res-painel-observacoes_de_todas_as_replicacoes = { $total } observations from all replications, from { $h } to { $h2 }.
res-painel-parametro = Parameter:
res-painel-atual = { $key } (current: { $current })
res-painel-ou_chave = or key:
res-painel-objeto_campo = object.field
res-painel-adicionar_parametro = Add parameter
res-painel-cada_linha_e_um_cenario = Each row is a scenario; each column, a parameter. An empty cell keeps the model’s value. Values in RON text, as in the set field of experiments.
res-painel-experimento = Experiment:
res-painel-cenario = Scenario
res-painel-remover_parametro = Remove parameter
res-painel-remover_cenario = Remove scenario
res-painel-adicionar_cenario = Add scenario
res-painel-metricas_separadas_por_virgula_vazio = Metrics (comma-separated; empty: all):
res-painel-rodar_cenarios = Run scenarios
res-painel-replicacao_oes_por_cenario_aba = { $reps ->
    [one] { $reps } replication
   *[other] { $reps } replications
} per scenario (Summary tab), same seeds in all
res-painel-media_ic_de_95_e = { $name }: mean ± 95% CI. Δ is the paired difference (same seeds) to '{ $name2 }'; ≈ marks a non-significant difference, significant ones in orange.
res-painel-filtrar_metricas = Filter metrics:
res-painel-metrica = Metric
res-painel-vs = Δ vs { $name }
res-painel-mostrar_no_grafico = Show in the chart
res-painel-grafico_da_metrica = Metric chart:
res-painel-arquivo_de_experimento = Experiment file:
res-painel-abrir_experimento = Open experiment
res-painel-experimento_aberto = Experiment '{ $name }' opened
res-painel-experimento_aberto_os_cenarios_valem = Experiment '{ $name }' opened; the scenarios apply to the open model (the file points to { $model_path })
res-painel-nao_foi_possivel_abrir = could not open: { $e }
res-painel-salvar_experimento = Save experiment
res-painel-salve_o_modelo_antes_o = save the model first: the experiment points to the model file
res-painel-experimento_salvo_em = Experiment saved to { $target }
res-painel-variaveis_de_decisao_mesmas_chaves = Decision variables (same keys as the scenarios), objective and constraints. All points use the same seeds; the best is compared with the others by a paired test.
res-painel-variavel = Variable
res-painel-dominio = Domain
res-painel-minimo = Minimum
res-painel-maximo = Maximum
res-painel-passo_valores = Step / values
res-painel-custo_por_unidade = Cost per unit
res-painel-inteiro = integer
res-painel-continuo = continuous
res-painel-lista = list
res-painel-objetivo = Objective:
res-painel-minimizar = minimize
res-painel-maximizar = maximize
res-painel-peso_da_metrica = metric weight
res-painel-custos_das_variaveis = + variable costs
res-painel-restricoes_sobre_a_media_da = Constraints (on the metric’s mean)
res-painel-minimo-2 = minimum
res-painel-maximo-2 = maximum
res-painel-adicionar_restricao = Add constraint
res-painel-metodo = Method:
res-painel-grade = grid
res-painel-recozimento_simulado = simulated annealing
res-painel-genetico = genetic
res-painel-iteracoes = iterations
res-painel-populacao = population
res-painel-geracoes = generations
res-painel-replicacoes_por_ponto = Replications per point
res-painel-orcamento_pontos = Budget (points)
res-painel-semente_da_busca = Search seed
res-painel-refinar_os = Refine the
res-painel-melhores_com = best with
res-painel-replicacoes-2 = replications
res-painel-otimizar = Optimize
res-painel-melhor_ponto = Best point:
res-painel-objetivo-2 = objective { $objective }
res-painel-aplicar_ao_modelo = Apply to the model
res-painel-grava_estes_valores_no_modelo = Saves these values in the model (Ctrl+Z undoes)
res-painel-nao_atende = not met
res-painel-media_observada = { $bound } (observed mean: { $cell })
res-painel-nenhum_ponto_avaliado_atende_as = No evaluated point meets the constraints.
res-painel-convergencia_melhor_objetivo_viavel_por = Convergence: best feasible objective by number of evaluated points
res-painel-pontos_avaliados = evaluated points
res-painel-melhor_ate_entao = best so far
res-painel-comparacao_pareada_com_o_melhor = Paired comparison with the best (≈: no significant difference at 95%)
res-painel-ponto = Point
res-painel-objetivo-3 = Objective
res-painel-viavel = Feasible
res-painel-vs_melhor = Δ vs best
res-painel-equivalente = { $delta } equivalent
res-painel-objeto_metrica = object.metric

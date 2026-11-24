## UTSim 3D (cliente Godot): configurações e campos de distribuição

barra-configuracoes = Settings
barra-configuracoes-dica = UTSim 3D preferences on this computer
configuracoes-titulo = Settings
configuracoes-distribuicoes = Distribution fields open as
configuracoes-formulario = Form
configuracoes-codigo = Code
configuracoes-distribuicoes-ajuda = Applies to every distribution field (process time, arrivals, loading, unloading...). The {"{"} {"}"} button of each field still switches only that field, until another object is chosen.
configuracoes-fechar = Close
props-editar_como_texto = Edit as code (RON text)
props-editar_como_formulario = Back to the form

## Painéis ao vivo em janela própria

vivo-soltar = Window
vivo-soltar-dica = Opens this panel in a window of its own, which can go to another monitor; it stays live
vivo-prender = Back
vivo-prender-dica = Closes the window and puts the panel back over the 3D view
vivo-janela-titulo = UTSim 3D: { $objeto }

## Idioma

configuracoes-idioma = Language
configuracoes-idioma-ajuda = For now only the Settings window, the live panels and the Quality, Safety and Maintenance tabs of the nodes change language; the other screens stay in Portuguese.

## Curva de fadiga da pessoa

fadiga-secao = Fatigue curve (time study)
fadiga-nota = Read only. The readings and the fit are edited in UTSim Estatística or with the command utsim pessoa ajustar.
fadiga-tarefa = Task { $tarefa }: { $leituras } readings at { $horarios } times
fadiga-tarefa-metodo = Task { $tarefa }, method { $metodo }: { $leituras } readings at { $horarios } times
fadiga-resumo = Rested: { $t0 }. At maximum fatigue, { $acrescimo } % more. Fatigue λ = { $lambda } per hour; recovery μ = { $mu } per hour ({ $origem }).
fadiga-estimada = estimated
fadiga-padrao = default
fadiga-hora = hour of the shift
fadiga-pausa = Break
fadiga-curva = Fitted curve
fadiga-media = Measured mean
fadiga-erro = Not fitted: { $erro }
fadiga-abrir = Open in UTSim Estatística
fadiga-abrir-ajuda = Opens the readings of this time study in a UTSim Estatística spreadsheet (one row per measured time: task, method, hour of the shift and time) for analysis. The model does not change.

## Abas dos nós (Qualidade, Segurança, Manutenção) e painéis ao vivo

aba-so-leitura = { $quem }. Read-only until that person closes the model.
aba-abrir-qualidade = Open in UTSim Quality
aba-abrir-qualidade-dica = Opens this node in UTSim Quality (defects, inspection, costs, SPC). The model is saved first and stays with the other app for editing; UTSim 3D reloads what it saves.
aba-abrir-seguranca = Open in UTSim Safety
aba-abrir-seguranca-dica = Opens this node in UTSim Occupational Safety (hazards, risk matrix, PPE). The model is saved first and stays with the other app for editing; UTSim 3D reloads what it saves.
aba-calculando = Computing the simulation figures...
aba-nenhuma = none
aba-nenhum-escolha = (none)
aba-nenhum = none
aba-opcoes = Separate with commas. Options: { $opcoes }.
aba-vazio-nao-usa = empty: not used
aba-adicionar = + Add
aba-adicionar-dica = Adds an item to { $lista }
aba-abrir-item = Opens the item
aba-fechar-item = Closes the item
aba-sem-nome = (no name)
aba-remover = Remove
aba-remover-dica = Takes this item off the list
vivo-fechar = Close
vivo-fechar-dica = Close the panel (Esc closes the most recent)
vivo-fixar = Pin
vivo-fixado = Pinned
vivo-fixar-dica = Pin keeps this panel; the next click on an object opens another one beside it
vivo-sem-numeros = No figures for this object.
vivo-eixo-tempo = time
vivo-eixo-itens = items
vivo-sem-dados = No data yet.
vivo-estados = States in the window
vivo-sem-estados = no states

## Painel ao vivo: linhas (rótulo e ajuda)

vivo-em-processamento-agora = Processing now
vivo-itens-no-objeto-neste-instante = Items in the object at this moment (processing or finished but unable to leave).
vivo-itens-em-espera-fila = Items waiting (queue)
vivo-itens-nas-filas-que-alimentam = Items in the queues feeding the object, at this moment.
vivo-entradas = Inputs
vivo-itens-que-entraram-na-janela = Items that came in during the window.
vivo-ja-processados = Already processed
vivo-itens-que-sairam-do-objeto = Items that left the object during the window.
vivo-rejeitados-para-descarte = Rejected as scrap
vivo-itens-que-chegaram-a-uma = Items from here that reached a scrap exit (sink_role: Scrap), also through queues or transport.
vivo-nenhuma-saida-tem-sink-role = No exit has sink_role: Scrap; mark the scrap exit in the model to count the scrap.
vivo-rejeitados-para-retrabalho = Rejected for rework
vivo-itens-que-chegaram-a-um = Items from here that reached a rework station, also through queues or transport.
vivo-ciclo-medio-permanencia = Mean cycle (stay time)
vivo-tempo-medio-no-objeto-dos = Mean time in the object of the items that left during the window.
vivo-espera-media-na-fila = Mean wait in queue
vivo-tempo-medio-nas-filas-que = Mean time in the queues feeding the object.
vivo-ocupado-processando = Busy (processing)
vivo-fracao-da-janela-no-estado = Fraction of the window in the processing state.
vivo-ocioso = Idle
vivo-fracao-da-janela-sem-trabalho = Fraction of the window with no work.
vivo-bloqueado = Blocked
vivo-pronto-sem-poder-sair-bloqueado = Finished but unable to leave (blocked or waiting for transport).
vivo-parado-quebra = Stopped (breakdown)
vivo-quebrado-aguardando-reparo-ou-em = Broken down, waiting for repair or in maintenance.
vivo-setup = Setup
vivo-fracao-em-setup = Fraction in setup.
vivo-conteudo-agora = Content now
vivo-itens-no-objeto-neste-instante-2 = Items in the object at this moment.
vivo-conteudo-medio = Mean content
vivo-media-no-tempo-do-conteudo = Time average of the content, in the window.
vivo-conteudo-maximo = Maximum content
vivo-maior-conteudo-da-janela = Largest content in the window.
vivo-espera-media = Mean wait
vivo-saidas = Outputs
vivo-itens-que-sairam-na-janela = Items that left during the window.
vivo-fracao-da-janela-sem-poder = Fraction of the window unable to release the created item.
vivo-recebidos-total = Received (total)
vivo-itens-que-chegaram-a-saida = Items that reached the exit during the window.
vivo-vazao-por-unidade-de-tempo = Throughput (per time unit)
vivo-recebidos-divididos-pela-duracao-da = Received divided by the length of the window.
vivo-tempo-medio-no-sistema = Mean time in system
vivo-da-criacao-do-item-a = From the creation of the item to its arrival here.
vivo-papel-na-qualidade = Role in quality
vivo-o-papel-da-saida-nos = The role of the exit in the quality reports.
vivo-utilizacao = Utilization
vivo-andando-carregando-descarregando-ou-trabalhando = Moving, loading, unloading or working at a station.
vivo-viagens = Trips
vivo-deslocamentos-comecados-na-janela = Trips started during the window.
vivo-itens-transportados = Items transported
vivo-itens-entregues-na-janela = Items delivered during the window.
vivo-carregando-agora = Carrying now
vivo-itens-consigo-neste-instante = Items carried at this moment.
vivo-viajando-carregado = Travelling loaded
vivo-fracao-da-janela-andando-com = Fraction of the window moving with a load.
vivo-viajando-vazio = Travelling empty
vivo-fracao-da-janela-andando-sem = Fraction of the window moving without a load.
vivo-trabalhando-num-posto = Working at a station
vivo-fracao-da-janela-operando-um = Fraction of the window operating a process (or an overhead crane by pendant or radio).
vivo-descanso-fadiga = Rest (fatigue)
vivo-fracao-da-janela-em-pausa = Fraction of the window on a break to recover.
vivo-pausas = Breaks
vivo-vezes-que-entrou-em-descanso = Times it went on a break during the window.
vivo-fadiga-media-maxima-fim-da = Mean / maximum fatigue (end of the replication)
vivo-a-fadiga-nao-esta-no = Fatigue is not in the playback record: values from the final report.
vivo-esperando-o-operador = Waiting for the operator
vivo-fracao-da-janela-parada-na = Fraction of the window stopped at the load until the pendant or radio operator gets next to it.

## Painel ao vivo: cabeçalho e seções

godot-ao-vivo-pessoa = person
godot-ao-vivo-ponte_rolante = overhead crane
godot-ao-vivo-empilhadeira = forklift
godot-ao-vivo-veiculo = vehicle
godot-ao-vivo-aquecimento_ate_numeros_desde_0 = warm-up until { $inicio_estatisticas }: figures since 0
godot-ao-vivo-desde_o_fim_do_aquecimento = since the end of the warm-up ({ $inicio_estatisticas })
godot-ao-vivo-desde_o_inicio = since the start
godot-ao-vivo-t_de_replicacao = t = { $t } of { $fim } · replication { $replicacao } · { $janela }
godot-ao-vivo-agora = Now
godot-ao-vivo-marque_a_saida_de_refugo = mark the scrap exit
godot-ao-vivo-acumulado = Cumulative
godot-ao-vivo-tempos = Times
godot-ao-vivo-em_processo_wip = In process (WIP)
godot-ao-vivo-agora_e_na_janela = Now and in the window
godot-ao-vivo-conteudo = Content
godot-ao-vivo-saida = Exit
godot-ao-vivo-recebidos_acumulado = Received (cumulative)
godot-ao-vivo-pessoa-2 = Person
godot-ao-vivo-ponte_rolante-2 = Overhead crane
godot-ao-vivo-veiculo-2 = Vehicle
godot-ao-vivo-itens_consigo = Items carried
godot-ao-vivo-fila = Queue

## Fluxos de tokens

godot-ui-flows-fluxos_de_tokens = Token flows
godot-ui-flows-fechar = Close
godot-ui-flows-fechar_os_fluxos = Close the flows
godot-ui-flows-o_modelo_nao_tem_fluxos = The model has no token flows.
godot-ui-flows-tokens_em_cada_atividade_no = Tokens in each activity at the playback moment; below, the counts over the run (the vertical line t is the current moment).
godot-ui-flows-zoom_ctrl_roda_no_grafo = Zoom (Ctrl + wheel on the graph)
godot-ui-flows-diminui = Zoom out
godot-ui-flows-aumenta = Zoom in
godot-ui-flows-ajustar = Fit
godot-ui-flows-cabe_na_largura = Fits the width
godot-ui-flows-sem_contagens = no counts

## Resultados

godot-ui-results-resumo = Summary
godot-ui-results-graficos = Charts
godot-ui-results-analise = Analysis
godot-ui-results-cenarios = Scenarios
godot-ui-results-otimizacao = Optimization
godot-ui-results-ocupacao = Utilization
godot-ui-results-estados = States
godot-ui-results-serie_temporal = Time series
godot-ui-results-histograma = Histogram
godot-ui-results-nenhum_recurso_ocupado = no busy resource
godot-ui-results-invalido_use_um_numero_inteiro = invalid { $what } "{ $text }": use a whole number
godot-ui-results-serie_invalida_use_objeto_medida = invalid series "{ $other }": use object.measure
godot-ui-results-calcule_antes_quantas_replicacoes_faltam = first compute how many replications are missing
godot-ui-results-simule_o_modelo_antes_de = Simulate the model before comparing scenarios.
godot-ui-results-simule_o_modelo_antes_de-2 = Simulate the model before optimizing.
godot-ui-results-trabalho_desconhecido = unknown job: { $other }
godot-ui-results-serie_curta_demais_para_o = Series too short for MSER-5 ({ $points } points): increase the stop time or the sampling.
godot-ui-results-corte_sugerido_mas_o_minimo = Suggested cut: { $cut }, but the minimum fell in the second half of the series: the run is short or does not settle.
godot-ui-results-corte_sugerido_mser_5_antes = Suggested cut (MSER-5): { $cut }. Before it the mean still rises or falls.
godot-ui-results-gargalo_movel_na_maior_parte = Shifting bottleneck: { $n } most of the time.
godot-ui-results-nenhum_recurso_ativo = No active resource.
godot-ui-results-replicacoes_faltam_para_o_alvo = { $reports } replications; { $more } more needed for the target.
godot-ui-results-aquecimento_analisado = Warm-up analysed: { $d }
godot-ui-results-lei_de_little_verde_s = Little’s law: { $verde } green, { $amarelo } yellow, { $vermelho } red.
godot-ui-results-cenario_s_comparados = { $n ->
    [one] { $n } scenario compared.
   *[other] { $n } scenarios compared.
}
godot-ui-results-otimizacao_melhor_ponto = Optimization: best point { $ponto }.
godot-ui-results-conteudo_de = content of { $object }
godot-ui-results-mser_5 = MSER-5: { $cut }
godot-ui-results-media_entre_replicacoes = mean across replications
godot-ui-results-unico_compartilhado = { $name } (sole { $sole }%, shared { $shifting }%)
godot-ui-results-melhor_objetivo = best objective
godot-ui-results-pontos_avaliados = evaluated points
godot-ui-results-resultados = Results
godot-ui-results-fechar = Close
godot-ui-results-fechar_os_resultados_a_cena = Close the results (the scene stays)
godot-ui-results-gargalo_no_turno = bottleneck on shift
godot-ui-results-objeto = Object
godot-ui-results-tipo = Kind
godot-ui-results-entradas = Inputs
godot-ui-results-saidas = Outputs
godot-ui-results-conteudo_medio = Mean content
godot-ui-results-permanencia = Stay time
godot-ui-results-ocupacao-2 = utilization { $valor }
godot-ui-results-replicacoes = Replications
godot-ui-results-semente = seed { $seed }
godot-ui-results-tempo_de_parada = Stop time
godot-ui-results-aquecimento = warm-up { $warmup }
godot-ui-results-gargalo = Bottleneck
godot-ui-results-gargalo_no_turno-2 = Bottleneck on shift
godot-ui-results-saidas_por_replicacao = Outputs per replication
godot-ui-results-soma_das_saidas_do_modelo = sum of the model’s outputs
godot-ui-results-o_gargalo_e_o_recurso = The bottleneck is the busiest resource; within the shift counts only the time it works (a station stopped half the day looks idle overall, but may be saturated on shift).
godot-ui-results-por_objeto = By object
godot-ui-results-media_semiamplitude_do_ic_de = Mean ± half-width of the 95 % CI across replications.
godot-ui-results-exportar_csv_para = Export CSV to
godot-ui-results-exportar = Export
godot-ui-results-grava_resumo_replicacoes_series_e = Saves summary, replications, series and histograms as CSV
godot-ui-results-grafico = Chart
godot-ui-results-fracao_do_tempo_trabalhando_turno = Fraction of time working (shift: only the time within the shift; blocking: adding the blocked time). The yellow bar is the bottleneck; the whisker is the 95 % CI.
godot-ui-results-fracao_do_tempo_em_cada = Fraction of time in each state, with the colours of the 3D scene.
godot-ui-results-conteudo_de_no_tempo_cada = Content of { $object } over time: each replication in grey and the mean in yellow.
godot-ui-results-histograma_de_juntando_todas_as = Histogram of { $object }.{ $tally }, pooling all replications.
godot-ui-results-o_modelo_nao_grava_ligar = {" "}The model does not record { $falta }: "Turn on and run again" turns it on and simulates again.
godot-ui-results-falta-serie = the content series
godot-ui-results-falta-histogramas = the histograms
godot-ui-results-ligar_e_rodar_de_novo = Turn on and run again
godot-ui-results-liga_no_modelo_a_serie = Turns on the content series in the model (run.sample_interval, by simulation time) and the histograms (run.histogram_bins = 20), in one step that Ctrl+Z undoes, and runs the replications again
godot-ui-results-sem_serie_ligue_run_sample = no series: turn on run.sample_interval in the model
godot-ui-results-sem_histograma_ligue_run_histogram = no histogram: turn on run.histogram_bins in the model
godot-ui-results-sem_dados = no data
godot-ui-results-ic_95 = 95 % CI
godot-ui-results-a = { $lo } to { $hi }: { $c }
godot-ui-results-aquecimento-2 = Warm-up
godot-ui-results-roda_as_replicacoes_desde_o = Runs the replications from time 0, without warm-up, and shows their mean (Welch curve). The MSER-5 cut is where the series stops having the empty start; "Apply" writes that time to run.warmup.
godot-ui-results-serie = Series
godot-ui-results-wip_ou_objeto_medida = wip or object.measure
godot-ui-results-janela = Window
godot-ui-results-campo-janela = window
godot-ui-results-campo-replicacoes = replications
godot-ui-results-campo-orcamento = budget
godot-ui-results-analisar = Analyse
godot-ui-results-roda_e_calcula_welch_e = Runs and computes Welch and MSER-5
godot-ui-results-aplicar = Apply
godot-ui-results-grava_o_corte_sugerido_em = Writes the suggested cut to run.warmup of the model
godot-ui-results-sem_serie = no series
godot-ui-results-replicacoes_necessarias = Replications needed
godot-ui-results-quantas_replicacoes_faltam_para_a = How many replications are missing for the half-width of the 95 % CI to fall below the target (5% is relative to the mean; 0.5 is absolute). "Run more" adds the missing ones to those already done, with new seeds.
godot-ui-results-precisao = Precision
godot-ui-results-calcular = Compute
godot-ui-results-calcula_as_replicacoes_necessarias = Computes the replications needed
godot-ui-results-rodar_mais = Run { $more } more
godot-ui-results-rodar_mais-2 = Run more
godot-ui-results-roda_as_replicacoes_que_faltam = Runs the missing replications and adds them to the current ones
godot-ui-results-para_todas_as_medidas_chegarem = For all measures to reach the target, { $more ->
    [one] { $more } more replication is needed.
   *[other] { $more } more replications are needed.
}
godot-ui-results-todas_as_medidas_ja_estao = All measures are already within the target.
godot-ui-results-consistencia_lei_de_little = Consistency: Little’s law
godot-ui-results-confere_l_w_em_cada = Checks L = λ·W in each object and in the system. Green: it holds; yellow: the difference is statistically different from zero, but small (up to 2 %); red: it does not hold, which usually means a short warm-up or items stuck at the end.
godot-ui-results-conferir = Check
godot-ui-results-roda_as_replicacoes_e_confere = Runs the replications and checks Little’s law
godot-ui-results-gargalo_movel = Shifting bottleneck
godot-ui-results-em_cada_instante_o_gargalo = At each moment, the bottleneck is the resource in the longest active period; when two periods overlap, the bottleneck is shared. Shows the fraction of time each resource is the sole and the shared bottleneck.
godot-ui-results-roda_as_replicacoes_com_o = Runs the replications with the trace and computes the shifting bottleneck
godot-ui-results-medida = Measure
godot-ui-results-atual = Current
godot-ui-results-alvo = Target (±)
godot-ui-results-faltam = Missing
godot-ui-results-diferenca = Difference
godot-ui-results-semaforo = Light
godot-ui-results-um_cenario_por_bloco_nome = One scenario per block: [name] and, below, the changes (object.field = value in RON, or run.field). The model as it is comes first, as the base. All use the same seeds (common random numbers), so the difference to the base is paired; significant differences in yellow.
godot-ui-results-medidas_virgulas_vazio_todas = Measures (commas; empty: all)
godot-ui-results-comparar = Compare
godot-ui-results-roda_todos_os_cenarios_e = Runs all scenarios and compares
godot-ui-results-texto = Δ { $valor }
godot-ui-results-procura_os_valores_das_variaveis = Searches for the values of the variables that minimize (or maximize) the mean of a metric. One variable per line: key minimum maximum [step] (integers) or with a decimal point (continuous, five values in the grid). Points not significantly different from the best (95 % paired test) are marked.
godot-ui-results-objetivo = Objective
godot-ui-results-maximizar = maximize
godot-ui-results-metodo = Method
godot-ui-results-replicacoes_por_ponto = Replications per point
godot-ui-results-orcamento_pontos = Budget (points)
godot-ui-results-otimizar = Optimize
godot-ui-results-roda_a_busca = Runs the search
godot-ui-results-melhor_ponto_ponto_s_simulados = Best point: { $ponto } ({ $simulated ->
    [one] { $simulated } point simulated
   *[other] { $simulated } points simulated
})
godot-ui-results-sem_historico = no history
godot-ui-results-ponto = Point
godot-ui-results-nenhum = none
godot-ui-results-nenhum-viavel = none feasible
godot-ui-results-util-trabalho = work
godot-ui-results-util-turno = shift
godot-ui-results-util-bloqueio = blocking
godot-ui-results-metodo-grade = grid
godot-ui-results-metodo-recozimento = annealing
godot-ui-results-metodo-genetico = genetic
godot-ui-results-replicacao_oes_tempo_de_parada = 
    { $model }: { $replications ->
        [one] { $replications } replication
       *[other] { $replications } replications
    }, stop time { $stop_time }, warm-up { $warmup }, seed { $seed }.
    Bottleneck (overall): { $b }
    Bottleneck (within the shift): { $bs }
godot-ui-results-replicacao_oes = { $model } · { $reports ->
    [one] { $reports } replication
   *[other] { $reports } replications
}

## Tipos de bloco nos relatórios (results.rs)

godot-tipo-source = arrival
godot-tipo-queue = queue
godot-tipo-processor = process
godot-tipo-combiner = assembly
godot-tipo-separator = disassembly
godot-tipo-sink = exit
godot-tipo-transporter = transporter
godot-tipo-dispatcher = dispatcher
godot-tipo-conveyor = conveyor
godot-tipo-rack = pallet rack
godot-tipo-span = time measurement
godot-tipo-flow = flow
godot-tipo-fluid = fluids

## Propriedades dos objetos (schema.rs)

godot-campo-movimentacao = Movement
godot-campo-folga-dos-obstaculos = Obstacle clearance
godot-campo-ponto-de-acesso-proprio = Own access point
godot-campo-x-no-objeto = X on the object
godot-campo-y-no-objeto = Y on the object
godot-campo-dimensoes = Dimensions
godot-campo-dimensoes-proprias = Own dimensions
godot-campo-capacidade-pela-area = Capacity by area
godot-campo-area-por-item = Area per item
godot-campo-cabem = Fits
godot-campo-tempo-entre-chegadas = Time between arrivals
godot-campo-primeira-chegada-em-t-0 = First arrival at t = 0
godot-campo-maximo-de-chegadas = Maximum arrivals
godot-campo-se-a-saida-estiver-bloqueada = If the output is blocked
godot-opcao-esperar = Wait
godot-opcao-descartar-perda = Discard (loss)
godot-campo-rotulos = Labels
godot-campo-chegadas-por-periodo = Arrivals per period
godot-campo-tabela-de-chegadas = Arrival table
godot-campo-capacidade-limitada = Limited capacity
godot-campo-politica-de-acesso = Access policy
godot-opcao-fifo-primeiro-a-entrar-primeiro = FIFO (first in, first out)
godot-opcao-lifo-ultimo-a-entrar-primeiro = LIFO (last in, first out)
godot-opcao-prioridade-maior-rotulo-primeiro = Priority: highest label first
godot-opcao-prioridade-menor-rotulo-primeiro-prazo = Priority: lowest label first (deadline)
godot-campo-rotulo-da-prioridade = Priority label
godot-campo-lote = Batch
godot-campo-itens-por-lote = Items per batch
godot-campo-espera-maxima = Maximum wait
godot-campo-fecha-a-entrada-ate-o = Closes the entrance until the batch leaves
godot-campo-capacidade = Capacity
godot-campo-preparacao-setup = Setup
godot-campo-troca-de-produto = Product changeover
godot-campo-tempo-de-processamento = Processing time
godot-campo-tempo-por-receita = Time per recipe
godot-campo-operadores = Operators
godot-campo-quem-opera = Who operates
godot-campo-quantos-por-peca = How many per part
godot-campo-ganho-da-equipe = Team speedup
godot-campo-durante-a-preparacao = During setup
godot-campo-durante-o-processamento = During processing
godot-campo-outras-equipes-ao-mesmo-tempo = Other teams at the same time
godot-campo-componentes-por-entrada = Components per input
godot-campo-modo = Mode
godot-opcao-embalar-so-o-conteiner-sai = Pack (only the container leaves)
godot-opcao-juntar-componentes-destruidos = Join (components destroyed)
godot-opcao-lote-todos-saem = Batch (all leave)
godot-opcao-desembalar = Unpack
godot-opcao-dividir-em-copias = Split into copies
godot-opcao-dividir-pelo-rotulo = Split by label
godot-campo-copias = Copies
godot-campo-sem-parametros = No parameters
godot-campo-velocidade-maxima = Maximum speed
godot-campo-tempo-de-carga = Loading time
godot-campo-tempo-de-descarga = Unloading time
godot-campo-voltar-a-base-quando-ocioso = Return to base when idle
godot-campo-agv-controle-de-trafego-na = AGV (network traffic control)
godot-campo-trajeto-desenhado = Drawn path
godot-campo-editar-nos = Edit nodes
godot-campo-comprimento = Length
godot-campo-velocidade = Speed
godot-campo-comprimento-por-item = Length per item
godot-campo-acumula-itens-quando-a-saida = Accumulates items when the output blocks
godot-campo-colunas = Columns
godot-campo-niveis = Levels
godot-campo-permanencia-minima = Minimum dwell time
godot-campo-regra-de-ocupacao = Occupancy rule
godot-opcao-primeira-livre = First free
godot-opcao-aleatoria = Random
godot-opcao-curva-abc-rotulo-classe = ABC curve (class label)
godot-campo-fracao-das-posicoes-da-classe = Fraction of positions for class A
godot-campo-fracao-das-posicoes-da-classe-2 = Fraction of positions for class B
godot-campo-largura-da-coluna = Column width
godot-campo-altura-do-nivel = Level height
godot-campo-equipe = Team
godot-campo-regra-de-escolha = Selection rule
godot-opcao-primeiro-livre = First free
godot-opcao-rodizio = Round robin
godot-opcao-mais-proximo = Nearest
godot-campo-ordem-dos-movimentos = Movement order
godot-opcao-segura-sobe-anda-desce = Safe (lift, travel, lower)
godot-opcao-um-eixo-de-cada-vez = One axis at a time
godot-opcao-os-tres-juntos = All three together
godot-campo-altura-de-passagem-do-gancho = Hook travel height
godot-campo-rotulo-do-peso-do-item = Item weight label
godot-campo-tempo-de-engate = Hook-on time
godot-campo-tempo-de-desengate = Unhook time
godot-campo-voltar-a-posicao-propria-quando = Return to own position when idle
godot-campo-controle = Control
godot-opcao-cabine-operador-nao-modelado = Cab (operator not modeled)
godot-opcao-botoeira-a-pessoa-anda-com = Pendant (the person walks with the load)
godot-opcao-radio-a-pessoa-encontra-a = Radio (the person meets the crane at the load)
godot-campo-equipe-do-operador = Operator team
godot-campo-meia-largura-da-area-sob = Half width of the area under the load
godot-campo-afastar-as-pessoas-do-caminho = Keep people out of the load path
godot-campo-folga-entre-pontes = Clearance between cranes
godot-campo-caminho = Path
godot-campo-regra-de-saida = Output rule
godot-campo-pesos = Weights
godot-campo-rotulo-com-o-numero-da = Label with the port number
godot-campo-script = Script
godot-campo-nenhum-passagem-direta = None (direct pass-through)
godot-campo-transporte-de-saida = Output transport
godot-campo-transferencia-pela-distancia = Transfer by distance
godot-campo-atraso = Delay
godot-opcao-reta = straight
godot-opcao-pela-rede = via network
godot-campo-script-ao-entrar-um-item = Script when an item enters
godot-campo-quebras = Breakdowns
godot-campo-tempo-entre-falhas-mtbf = Time between failures (MTBF)
godot-campo-tempo-de-reparo-mttr = Repair time (MTTR)
godot-campo-turnos = Shifts
godot-campo-origem = Source
godot-opcao-personalizado = Custom
godot-opcao-calendario-do-modelo = Model calendar
godot-campo-periodo = Period
godot-campo-pausas = Breaks
godot-campo-no-fim-do-turno = At the end of the shift
godot-campo-calendario = Calendar
godot-campo-ativo = Active
godot-campo-cor = Color
godot-campo-escala = Scale
godot-campo-modelo-da-peca = Part model
godot-campo-giro-do-desenho = Drawing rotation
godot-campo-restaurar-padrao = Restore default
godot-campo-nome = Name
godot-campo-tipo = Type
godot-campo-rotacao = Rotation
godot-campo-sexo = Sex
godot-opcao-nao-informado = Not specified
godot-opcao-feminino = Female
godot-opcao-masculino = Male
godot-campo-altura = Height
godot-ajuda-a-altura-do-boneco-no = The figure's height in 3D. When off, it is drawn at random from the name.
godot-campo-massa = Mass
godot-ajuda-com-a-altura-da-a = Together with the height, it gives the figure's width in 3D through the body mass index.
godot-campo-idade = Age
godot-campo-experiencia-na-funcao = Experience in the role
godot-ajuda-so-registro-a-simulacao-nao = Record only: the simulation does not read it.
godot-campo-carga-maxima-medida = Maximum measured load
godot-ajuda-uma-carga-acima-dela-da = A load above it gives a warning in the model.
godot-campo-forca-maxima-medida = Maximum measured force
godot-ajuda-empurrar-ou-puxar-so-registro = Push or pull. Record only.
godot-campo-tipo-de-espera = Queue type
godot-opcao-fila-simples-sem-lugares = Simple queue (no slots)
godot-opcao-mesa = Table
godot-opcao-palete = Pallet
godot-opcao-estante = Shelf
godot-opcao-chao-area-demarcada = Floor (marked area)
godot-campo-camadas = Layers
godot-campo-passo-entre-pecas = Spacing between parts
godot-campo-lugares = Slots
godot-campo-opera-as-maquinas = Operates the machines
godot-campo-leva-os-itens-de = Takes the items from
godot-campo-nome-do-modelo = Model name
godot-campo-descricao = Description
godot-campo-tempo-de-simulacao = Simulation time
godot-campo-aquecimento = Warm-up
godot-campo-semente = Seed
godot-campo-replicacoes = Replications
godot-campo-movimentacao-padrao = Default movement
godot-campo-perfil-da-marcacao-do-piso = Floor marking profile
godot-opcao-brasil-faixa-amarela = Brazil (yellow stripe)
godot-opcao-branco-redacao-original-da-nr = White (original NR-26 wording)
godot-campo-largura-da-faixa = Stripe width
godot-campo-parada-no-cruzamento = Stop at the crossing
godot-campo-cor-do-chao = Floor color
godot-ajuda-como-o-recurso-anda-o = How the resource moves: the model default (Simulation), in a straight line as in the old format, free across the floor avoiding obstacles, along the path network, or only along the aisles of its class.
godot-ajuda-onde-os-recursos-moveis-param = Where mobile resources stop to serve this object, in meters from its center and rotating with it. When off, it is the -y side. It can also be dragged in the scene, with "Show obstacles and clearances" on.
godot-ajuda-comprimento-largura-e-altura-do = Length, width and height of the object in meters, in its own orientation. The floor area changes the access point and, with it, the distances and times of operators and vehicles; for a queue with area per item, the capacity as well. The 3D view stretches the model to these measures. When off, the default area for the type applies.
godot-ajuda-a-capacidade-da-espera-sai = The queue capacity comes from its floor area divided by the area each item takes up (whole items only). If a capacity is also given, the smaller one applies.
godot-ajuda-intervalo-entre-duas-chegadas-seguidas = Interval between two consecutive arrivals.
godot-ajuda-valor-sorteado-para-cada-item = Value drawn for each item, by name: {"{"}"tipo": Discrete([(1.0, 3.0), (2.0, 1.0)]){"}"}.
godot-ajuda-ciclo-e-periodos-com-o = Cycle and periods with their own interval, rate or closed: (cycle: 1440.0, periods: [(start: 0.0, end: 480.0, arrivals: Rate(0.5))]).
godot-ajuda-horario-instante-quantidade-tipo-e = Schedule (time, quantity, type and labels of each row) or sequence of rows: (mode: Times, repeat: Cycle(1440.0), rows: [(at: 480.0, count: 12)]).
godot-ajuda-desligada-a-espera-nao-tem = When off, the queue has no limit.
godot-ajuda-qual-item-sai-primeiro-pela = Which item leaves first. By priority, the item with the highest (or lowest) label value leaves first; ties and items without the label leave in arrival order.
godot-ajuda-segura-os-itens-e-libera = Holds the items and releases them in groups.
godot-ajuda-libera-um-lote-incompleto-depois = Releases an incomplete batch after this time (0 = always waits).
godot-ajuda-preparacao-so-quando-o-produto = Setup only when the product changes: the product label and, if you want, the time of each changeover (from, to); with no row in the matrix, the setup above applies. Example: (label: "produto", matrix: [(from: 1, to: 2, time: Const(30.0))]). None: setup on every part.
godot-ajuda-tempo-de-processamento-que-depende = Processing time that depends on the item: the recipe label and the time for each value; items without a recipe in the table use the time above. Example: (label: "produto", times: [(value: 1, time: Const(90.0)), (value: 2, time: Const(120.0))]). None: the same time for all.
godot-ajuda-quem-opera-esta-maquina-marque = Who operates this machine: check and choose below, or drag from the operator's (or team's) orange node to the machine. The process waits for one of them to be free.
godot-ajuda-operadores-que-a-peca-precisa = Operators the part needs at the same time: it only starts when all of them arrive, and they stay with it until the end. With more operators on the team than capacity × this number, the extras stay idle.
godot-ajuda-quantas-vezes-mais-rapido-a = How many times faster the team does the step: the drawn time is divided by this number. 1: no speedup (the time is the same with one or several).
godot-ajuda-equipes-que-precisam-estar-juntas = Teams that must be together with the operators (a doctor and a nurse in the same room). Example: [OperatorUse(team: "enfermagem")]. The step only starts when all of them arrive.
godot-ajuda-a-entrada-1-traz-o = Input 1 brings the container; input k + 1 brings the k-th number of items. Example: [2, 1].
godot-ajuda-desligada-a-velocidade-muda-na = When off, the speed changes immediately.
godot-ajuda-abre-a-ferramenta-de-desenho = Opens the drawing tool with this conveyor: drag the nodes, change the height and the radius.
godot-ajuda-transportadores-que-o-despachante-comanda = Transporters the dispatcher controls.
godot-ajuda-segura-o-gancho-sobe-a = Safe: the hook rises to the travel height, the bridge and trolley move together and the hook lowers.
godot-ajuda-rotulo-com-o-peso-de = Label with the weight of each item, in kg: items above the capacity raise a warning.
godot-ajuda-na-botoeira-a-ponte-nao = With the pendant, the crane does not move faster than the person; with the radio, it keeps its own speed and waits for the person to arrive.
godot-ajuda-meio-lado-do-quadrado-sob = Half side of the square under the hook where nobody should stand with the load suspended (exposure analysis).
godot-ajuda-antes-de-andar-carregada-a = Before moving loaded, the crane waits for anyone crossing the load strip; and anyone about to enter it stops at the edge until the load has passed.
godot-ajuda-distancia-mantida-entre-os-ganchos = Distance kept between the hooks of two cranes on the same path: one does not pass the other.
godot-ajuda-para-qual-ligacao-vai-cada = Which link each item goes to, in the order of the output links.
godot-ajuda-peso-de-cada-saida-na = Weight of each output, in the order of the links. Example: [3.0, 1.0].
godot-ajuda-devolve-o-numero-da-porta = Returns the port number (1, 2, ...).
godot-ajuda-transportador-ou-despachante-que-leva = Transporter or dispatcher that takes the items to the next block.
godot-ajuda-os-itens-levam-atraso-distancia = Items take delay + distance / speed to reach the next block over direct links (without a transporter).
godot-ajuda-roda-para-cada-item-que = Runs for each item that enters, before any other rule.
godot-ajuda-falhas-aleatorias-com-o-tempo = Random failures, with time counted on the clock.
godot-ajuda-janelas-fora-do-turno-repetidas = Windows outside the shift, repeated every period.
godot-ajuda-inicio-e-duracao-de-cada = Start and duration of each break within the period, in order.
godot-ajuda-corrige-para-onde-o-modelo = Corrects where the 3D model faces; the resource turns toward where it moves.
godot-ajuda-volta-a-aparencia-padrao-do = Returns to the default appearance of the object type.
godot-ajuda-giro-em-graus-anti-horario = Rotation in degrees, counterclockwise seen from above (R rotates 90°, Ctrl+R 15°).
godot-ajuda-escolhe-o-corpo-no-3d = Chooses the body in 3D (not specified, the body comes from the name). No fatigue pattern depends on it; with Female, a load above 20 kg gives the CLT warning, art. 390.
godot-ajuda-numa-espera-real-cada-peca = In a real queue each part takes a slot (the type's layout, recalculated from the Dimensions): only a part with nothing on top can leave, the oldest of them first, and whoever picks it up goes to the front of the part. It changes the result. Simple queue: as before.
godot-ajuda-quantas-pecas-empilhadas-em-cada = How many parts are stacked in each slot (on the shelf, on each rack level).
godot-ajuda-distancia-entre-os-centros-de = Distance between the centers of two neighboring parts.
godot-ajuda-arraste-do-no-laranja-ate = Drag from the orange node to the machine to connect; click the dashed line and the × to remove.
godot-ajuda-arraste-do-no-laranja-ate-2 = Drag from the orange node to the object (a queue, an arrival, or with Shift a machine) to connect.
godot-ajuda-texto-livre-sobre-o-modelo = Free text about the model, saved in the file.
godot-ajuda-as-estatisticas-so-contam-depois = Statistics only count after this time.
godot-ajuda-como-andam-os-recursos-moveis = How mobile resources that did not choose their own mode move. When empty, Free applies (shortest path on the floor, avoiding obstacles).
godot-ajuda-cores-das-faixas-e-setas = Colors of the aisle stripes and arrows. The program does not certify compliance: check the current text of the standards.
godot-ajuda-a-pratica-usual-vai-de = Usual practice ranges from 0.05 to 0.15 m.
godot-ajuda-o-veiculo-para-esse-tempo = The vehicle stops for this time before a crossing with pedestrians.
godot-ajuda-a-cor-do-piso-na = The floor color in the 3D scene, saved in the model; the grid changes tone to stay visible. When off, the usual gray.
godot-opcao-por-rotulo = By label
godot-opcao-padrao-do-modelo = Model default
godot-opcao-legado-linha-reta-ou-rede = Legacy (straight line or network)
godot-opcao-livre-menor-caminho-no-piso = Free (shortest path on the floor)
godot-opcao-pela-rede-de-caminhos = Via path network
godot-opcao-pelos-corredores = Via aisles


## Propriedades dos objetos: seções, tipos, famílias, parâmetros, turnos e visual (schema.rs)

godot-secao-objeto = Object
godot-secao-posicao = Position
godot-secao-parametros = Parameters
godot-secao-saida = Output
godot-secao-quebras-e-turnos = Breakdowns and shifts
godot-secao-aparencia = Appearance
godot-secao-simulacao = Simulation
godot-secao-movimentacao = Movement
godot-secao-piso-e-corredores = Floor and aisles
godot-secao-modelo = Model
godot-secao-pessoa = Person
godot-objeto-chegada = Arrival
godot-objeto-espera = Queue
godot-objeto-processo = Process
godot-objeto-montagem = Assembly
godot-objeto-desmontagem = Disassembly
godot-objeto-saida = Exit
godot-objeto-transportador = Transporter
godot-objeto-ponte-rolante = Overhead crane
godot-objeto-despachante = Dispatcher
godot-objeto-esteira = Conveyor
godot-objeto-porta-paletes = Pallet rack
godot-dist-constante = Constant
godot-dist-uniforme = Uniform
godot-dist-exponencial = Exponential
godot-dist-triangular = Triangular
godot-dist-normal = Normal
godot-dist-lognormal = Lognormal
godot-dist-gama = Gamma
godot-dist-erlang = Erlang
godot-dist-weibull = Weibull
godot-dist-beta = Beta
godot-dist-log-logistica = Log-logistic
godot-dist-pearson-v = Pearson V
godot-dist-pearson-vi = Pearson VI
godot-dist-discreta = Discrete
godot-dist-empirica = Empirical
godot-dist-poisson = Poisson
godot-dist-geometrica = Geometric
godot-dist-binomial = Binomial
godot-dist-binomial-negativa = Negative binomial
godot-dist-deslocada = Shifted
godot-param-valor = value
godot-param-minimo = minimum
godot-param-maximo = maximum
godot-param-media = mean
godot-param-moda = mode
godot-param-desvio-padrao = standard deviation
godot-param-forma = shape
godot-param-escala = scale
godot-param-fases = phases (k)
godot-param-forma-1 = shape 1
godot-param-forma-2 = shape 2
godot-param-probabilidade = probability (p)
godot-param-tentativas = trials (n)
godot-param-sucessos = successes (r)
godot-param-alfa = alpha
godot-param-beta = beta
godot-param-deslocamento = shift
godot-dim-comprimento-do-caminho = Path length
godot-dim-vao = Span
godot-dim-altura-do-caminho = Path height
godot-dim-comprimento = Length
godot-dim-largura = Width
godot-dim-altura = Height
godot-turno-pausar = Pause
godot-turno-terminar = Finish the work
godot-turno-do-calendario = (the calendar's)
godot-ajuda-folga-papel = Minimum distance from any obstacle. When off, the resource role's value applies ({ $m } m: half width plus 0.3 m).
godot-dim-esteira = { $l } × { $w } m: the length is the conveyor's (Parameters)
godot-dim-porta-paletes = { $l } × { $w } × { $h } m: from the spans and levels (Parameters)
godot-dim-padrao = { $l } × { $w } m (default)
godot-campo-sem-limite = no limit
godot-campo-n-itens = { $n ->
    [one] { $n } item
   *[other] { $n } items
}
godot-campo-nao-existe = { $nome } (does not exist)
godot-campo-n-posicoes = { $n ->
    [one] { $n } position
   *[other] { $n } positions
}
godot-campo-movimento = Movement { $o_que }.
godot-campo-n-lugares = { $n ->
    [one] { $n } slot
   *[other] { $n } slots
}
godot-campo-divide-caminho = Shares the path with { $com } ({ $m } m clearance between hooks)
godot-campo-pesos-e-saidas = There { $pesos ->
    [one] is { $pesos } weight
   *[other] are { $pesos } weights
}, but the object has { $n ->
    [one] { $n } output
   *[other] { $n } outputs
}. { $ajuda }
godot-turno-sem-calendarios = The model has no calendars: create one in "calendars" to use it here.
godot-turno-calendario-ajuda = A calendar is the same shift table shared by several objects.
godot-visual-padrao-do-tipo = (type default)
godot-visual-gerado = UTSim (generated by utsim-assets)
godot-visual-grupo-biblioteca = UTSim Library
godot-visual-grupo-terceiros = Third-party (assets/terceiros)
godot-visual-grupo-elementos = 3D elements (assets/modelos3d)
godot-visual-caixa-padrao = box (default)
godot-visual-modelo-da-peca-ajuda = 3D element used to draw the parts created here; the box is the default.
godot-visual-no-tamanho-da-peca = { $arquivo } ({ $x } x { $y } x { $z } m), drawn at the part's size
godot-eixo-z = Z (height)


## ui_props.rs

godot-ui-props-modelo_nada_selecionado = Model (nothing selected)
godot-ui-props-propriedades = Properties: { $name }
godot-ui-props-problemas = Problems
godot-ui-props-avisos = Warnings
godot-ui-props-mostrar_motorista = Show driver
godot-ui-props-desenha_o_operador_sentado_e = Draws the operator seated and animated on the forklift. Appearance only: it does not change the simulation and is not saved in the model.
godot-ui-props-clique_para_destacar_na_cena = Click to highlight in the scene what this warning mentions
godot-ui-props-abre-a-secao = Opens the { $title } section
godot-ui-props-fecha-a-secao = Closes the { $title } section
godot-ui-props-liga_ou_desliga = Turns { $campo } on or off
godot-ui-props-aplicar = Apply
godot-ui-props-aplicar_script = Apply script
godot-ui-props-nenhum_transportador_no_modelo = No transporter in the model.
godot-ui-props-este_campo_vem_de_dados = This field comes from real data: { $label }
godot-ui-props-distribuicao = Distribution
godot-ui-props-dados_reais = Real data
godot-ui-props-fonte = Source
godot-ui-props-de_onde_vem_a_distribuicao = Where this field's distribution comes from: given parameters or real values, fitted automatically
godot-ui-props-media = mean { $mean }
godot-ui-props-arquivo_csv = CSV file
godot-ui-props-valores_colados = Pasted values
godot-ui-props-valores_reais = Real values…
godot-ui-props-abre_a_janela_grande_para = Opens the large window to type, paste or import the values and compare all distributions at once
godot-ui-props-origem = Source
godot-ui-props-arquivo = File
godot-ui-props-arquivo_csv_ou_txt = .csv or .txt file
godot-ui-props-escolher = Choose...
godot-ui-props-escolhe_o_arquivo_de_dados = Chooses the data file (CSV or TXT)
godot-ui-props-automatica = (automatic)
godot-ui-props-coluna = Column
godot-ui-props-coluna_do_arquivo_com_os = File column with the values
godot-ui-props-um_valor_por_linha_ou = one value per line (or separated by ; or space)
godot-ui-props-sao_instantes_usar_os_intervalos = These are instants (use the intervals between them)
godot-ui-props-marque_quando_os_valores_sao = Check when the values are increasing instants (such as arrival times): the fit uses the differences between them. Unchecked, the values are durations.
godot-ui-props-melhor_automatica = Best automatic
godot-ui-props-empirica_os_proprios_valores = Empirical (the values themselves)
godot-ui-props-so = Only { $f }
godot-ui-props-escolha = Choose
godot-ui-props-criterio = Criterion
godot-ui-props-o_que_ordena_as_familias = What ranks the families accepted by the tests
godot-ui-props-reajustar_com_estas_opcoes = Refit with these options
godot-ui-props-ajusta_a_distribuicao_aos_dados = Fits the distribution to the data (in the background) and starts using it in this field
godot-ui-props-nao_foi_possivel_ler_os = Could not read the data: { $err }
godot-ui-props-os_dados_mudaram_desde_o = The data changed since the last fit.
godot-ui-props-reajustar_aos_dados_atuais = Refit to the current data
godot-ui-props-o_campo_foi_reajustado_salve = { $c }. The field was refitted; save to record it.
godot-ui-props-n = { $family } · n = { $n } · { $label }
godot-ui-props-analisando_os_dados = Analyzing the data...
godot-ui-props-candidatas = Candidates ({ $criterion })
godot-ui-props-aic_bic_usa_esta_familia =
    { $params }
    AIC { $aic }   BIC { $bic }
    Use this family in this field
godot-ui-props-usar_a_empirica_os_proprios = Use the empirical (the values themselves)
godot-ui-props-relatorio = Report
godot-ui-props-grava_o_relatorio_completo_do = Saves the full fit report (HTML) to a folder and opens it in the browser
godot-ui-props-histograma_dos_dados_e_a = Histogram of the data and the curve of the chosen distribution
godot-ui-props-buscar_ativo = Search asset
godot-ui-props-miniatura_do_objeto_com_o = Thumbnail of the object with the chosen asset (or the one under the mouse in the list)
godot-ui-props-ligacoes_de_saida_ordem_das = Output links (port order)
godot-ui-props-nenhuma_use_a_ferramenta_ligar = None. Use the Link tool (L).
godot-ui-props-subir = Move up
godot-ui-props-sobe_esta_saida_na_ordem = Moves this output up in the port order
godot-ui-props-descer = Move down
godot-ui-props-desce_esta_saida_na_ordem = Moves this output down in the port order
godot-ui-props-remover = Remove
godot-ui-props-remove_esta_ligacao = Removes this link
godot-ui-props-entradas = Inputs: { $inputs }

## Round 13: UTSim 3D windows

## dados.rs

godot-dados-criterio_desconhecido = unknown criterion: { $other }
godot-dados-use_um_arquivo_ou_valores = use a file or pasted values, not both
godot-dados-escolha_um_arquivo_de_dados = choose a data file or paste the values
godot-dados-registro_reaproveitado = record reused
godot-dados-dados_mudaram = data changed
godot-dados-opcoes_mudaram = options changed
godot-dados-dados_ilegiveis = unreadable data
godot-dados-os_dados_mudaram_desde_o = the data changed since the last fit (was { $old_family }, hash { $old_hash })
godot-dados-nao_foi_possivel_ler = could not read { $path }: { $e }
godot-dados-nenhum_campo_do_modelo_usa = no model field uses real data
godot-dados-nao_foi_possivel_gravar_em = could not write to { $dir }: { $e }

## document.rs

godot-document-transportadores_e_despachantes_nao_usam = Transporters and dispatchers do not use flow links: drag from their orange node to the machine they serve (or to the object whose items they carry).
godot-document-objeto_nao_existe = object '{ $n }' does not exist
godot-document-arraste_de_um_objeto_ate = drag from one object to another
godot-document-saiu_da_equipe_de = { $to } left the team of { $from }.
godot-document-entrou_na_equipe_de = { $to } joined the team of { $from }.
godot-document-nao_pede_mais_operadores_de = { $from } no longer requests operators from { $to }.
godot-document-pede_operadores_de = { $from } requests operators from { $to }.
godot-document-nao_pede_mais_transporte_a = { $from } no longer requests transport from { $to }.
godot-document-pede_transporte_a = { $from } requests transport from { $to }.
godot-document-arraste_do_despachante_a_um = drag from the dispatcher to a transporter, or from an object to the transporter or dispatcher that serves it
godot-document-a_serie_e_os_histogramas = the series and the histograms are already wired into the model
godot-document-desenhe_clicando_os_nos_no = draw by clicking the nodes on the floor (Enter finishes)
godot-document-tipo_desconhecido = unknown type: { $id }
godot-document-nada_selecionado = nothing selected
godot-document-nao_ha_para_onde_mover = there is nowhere to move this link
godot-document-nenhum_arrasto_comecou = no drag has started
godot-document-objeto_nao_existe-2 = object '{ $name }' does not exist
godot-document-e_um_recurso_movel_ele = { $name } is a mobile resource: it turns toward where it moves
godot-document-recursos_moveis_nao_giram_eles = mobile resources do not rotate: they turn toward where they move
godot-document-a_area_de_transferencia_esta = the clipboard is empty (Ctrl+C copies)
godot-document-alinhamento_desconhecido = unknown alignment: { $how }
godot-document-selecione_dois_objetos_ou_mais = select two or more objects to align
godot-document-eixo_desconhecido = unknown axis: { $axis }
godot-document-selecione_tres_objetos_ou_mais = select three or more objects to distribute
godot-document-nao_tem_distribuicao = '{ $field }' has no distribution
godot-document-modelo = Model
godot-document-nome_em_vez_de = name: { $name } instead of { $name2 }
godot-document-descricao_em_vez_de = description: { $description } instead of { $description2 }
godot-document-run_em_vez_de = run: { $run } instead of { $run2 }
godot-document-falta_o_objeto = object { $name } is missing
godot-document-objeto_em_vez_de = object { $name }: { $g } instead of { $w }
godot-document-objeto_a_mais = extra object: { $name }
godot-document-ligacoes_em_vez_de = links: { $connections } instead of { $connections2 }
godot-document-outras_partes_do_modelo_calendarios = other parts of the model (calendars, network, flows, fluids) differ
godot-document-clique = Click
godot-document-seleciona_o_bloco_ou_a = selects the block or the link
godot-document-clique_de_novo_no_mesmo = Click again on the same spot
godot-document-passa_para_o_proximo_bloco = moves to the next overlapping block
godot-document-poe_ou_tira_o_bloco = adds or removes the block from the selection
godot-document-shift_arrastar_no_chao = Shift + drag on the floor
godot-document-selecao_por_caixa_soma_a = box selection (adds to the selection)
godot-document-arrastar_no_chao = Drag on the floor
godot-document-selecao_por_caixa_troca_a = box selection (replaces the selection)
godot-document-arrastar_um_bloco = Drag a block
godot-document-move_o_bloco_ou_a = moves the block or the whole selection
godot-document-arrastar_da_porta_de_saida = Drag from the output port (green)
godot-document-liga_ao_bloco_ou_a = links to the block (or the input port) where you drop it
godot-document-botao_direito_arrastar = Right button + drag
godot-document-gira_a_vista = rotates the view
godot-document-shift_botao_direito_arrastar = Shift + right button + drag
godot-document-desloca_a_vista = pans the view
godot-document-roda_do_mouse = Mouse wheel
godot-document-aproxima_e_afasta = zooms in and out
godot-document-biblioteca_clique_e_clique_no = Library: click, then click on the floor
godot-document-coloca_um_objeto_shift_mantem = places an object (Shift keeps the tool)
godot-document-biblioteca_arrastar_ate_o_chao = Library: drag to the floor
godot-document-coloca_um_objeto_onde_soltar = places an object where you drop it
godot-document-setas = Arrow keys
godot-document-move_a_selecao_um_passo = moves the selection one grid step
godot-document-girar_90_anti_horario_visto = rotate 90° (counterclockwise seen from above)
godot-document-girar_90_horario = rotate −90° (clockwise)
godot-document-girar_15_15 = rotate 15° / −15°
godot-document-arrastar_a_alca_de_giro = Drag the rotation handle (purple)
godot-document-gira_o_objeto_selecionado_em = rotates the selected object in 15° steps (with Shift, 90°)
godot-document-exclui_a_selecao_ou_a = deletes the selection (or the selected link)
godot-document-ferramenta_ligar_arraste_da_origem = Link tool: drag from the source to the destination, or click and click
godot-document-ferramenta_corredor_clique_pontos_no = Aisle tool: click points on the floor, Enter or double-click finishes
godot-document-ferramenta_zona_zonas_de_risco = Zone tool: risk zones on the floor; click the corners, Enter or the first point closes
godot-document-e_ou_alt_com_ligar = E (or Alt with Link)
godot-document-ferramenta_equipe_arraste_do_despachante = Team tool: drag from the dispatcher to the transporter, or from the object to whoever carries its items (Shift: process operators)
godot-document-cancela_a_ferramenta_a_ligacao = cancels the tool, the link or aisle in progress, or the selection
godot-document-simular_clique_num_objeto = Simulate: click on an object
godot-document-abre_o_painel_de_resultados = opens the object's live results panel in the corner of the view
godot-document-simular_esc = Simulate: Esc
godot-document-fecha_o_painel_ao_vivo = closes the most recent live panel (the unpinned one first)
godot-document-f_editar_ou_home = F (Edit) or Home
godot-document-enquadra_o_modelo = frames the model
godot-document-simular_f = Simulate: F
godot-document-anda_pela_fabrica_em_primeira = walks through the factory in first person: WASD walks, the mouse looks, Shift runs, E/Q goes up and down, V flies, the wheel changes the speed, clicking the crosshair opens the panel, Esc exits
godot-document-liga_ou_desliga_o_encaixe = turns grid snapping on or off
godot-document-alterna_entre_editar_e_simular = switches between Edit and Simulate
godot-document-mostra_ou_esconde_esta_janela = shows or hides this window
godot-document-seleciona_todos_os_objetos = selects all objects
godot-document-copia_a_selecao = copies the selection
godot-document-recorta_a_selecao = cuts the selection
godot-document-cola_nomes_unicos_ligacoes_internas = pastes (unique names, internal links preserved)
godot-document-duplica_a_selecao = duplicates the selection
godot-document-salva_como = saves as
godot-document-abre_um_modelo = opens a model
godot-document-novo_modelo = new model

## editor3d.rs

godot-editor3d-opera = { $to } operates { $from }
godot-editor3d-leva_os_itens_de = { $to } carries the items of { $from }
godot-editor3d-esta_na_equipe_de = { $to } is in the team of { $from }
godot-editor3d-ligacao_de = Link of { $k }: { $from } → { $to }
godot-editor3d-arraste_do_no_laranja_dele = { $name }: drag from its orange node to a machine for it to operate (or to an object, to carry its items).
godot-editor3d-de_clique_no_no_de = {" "}({ $next } of { $links }: click the node again for the next one)
godot-editor3d-ligacao_selecionada_clique_no_ou = Link { $a } → { $b } selected{ $extra }. Click the × or press Del to remove.
godot-editor3d-esta_ligacao_se_muda_no = This link ({ $kind }) is changed in the properties panel of { $from }.
godot-editor3d-ligacao_removida_ctrl_z_desfaz = Link { $a } → { $b } removed (Ctrl+Z undoes).
godot-editor3d-nenhuma_zona_selecionada_para_excluir = No zone selected to delete.
godot-editor3d-nada_selecionado_para_excluir = Nothing selected to delete.
godot-editor3d-zona_cancelada = Zone cancelled.
godot-editor3d-ligacao_cancelada = Link cancelled.
godot-editor3d-girar_arraste_em_volta_do = Rotate: drag around the object (15° steps; with Shift, 90°).
godot-editor3d-solte_sobre_a_maquina_que = Drop on the machine that { $res } will operate (or on an object, to carry its items). Esc cancels.
godot-editor3d-ligar_solte_sobre_o_destino = Link: drop on the destination (or click it). Esc cancels.
godot-editor3d-ligar_solte_sobre_a_origem = Link: drop on the source (or its output node). Esc cancels.
godot-editor3d-ligar_clique_em_um_bloco = Link: click a block.
godot-editor3d-ligado_clique_em_outra_origem = Linked: { $from } → { $to }. Click another source or press Esc.
godot-editor3d-ligacao_selecionada_clique_no_ou-2 = Link { $a } → { $b } selected. Click the × or press Del to remove.
godot-editor3d-clique_no_ou_del_remove = { $said }. Click the × or press Del to remove.
godot-editor3d-de_blocos_neste_ponto_clique = { $name } ({ $index } of { $hits } blocks at this point: click again for the next one).
godot-editor3d-arraste_deste_no_verde_ate = { $from }: drag from this green node to the destination to link.
godot-editor3d-ligar_clique_no_destino_esc = Link: { $from } → click the destination (Esc cancels).
godot-editor3d-ligado = Linked: { $from } → { $to }.
godot-editor3d-ligacao_cancelada_solte_sobre_o = Link cancelled: drop on the destination block.
godot-editor3d-nao_recebe_de_ninguem_arraste = { $to } receives from no one: drag from the source's output node to here.
godot-editor3d-ligacao_cancelada_solte_sobre_o-2 = Link cancelled: drop on the source block.
godot-editor3d-girado_para = { $name } rotated to { $degrees }°.
godot-editor3d-arraste_a_alca_em_volta = { $name }: drag the handle around the object to rotate it (R rotates 90°, Ctrl+R 15°).
godot-editor3d-objeto_s_movidos =
    { $selected } { $selected ->
        [one] object moved
       *[other] objects moved
    }.
godot-editor3d-objeto_s_na_caixa =
    { $inside } { $inside ->
        [one] object in the box
       *[other] objects in the box
    }.
godot-editor3d-nada_mudou_solte_sobre_a = Nothing changed: drop on the machine (or object) that { $res } will serve.
godot-editor3d-solte_sobre_uma_maquina_ou = Drop on a machine or an object (a transporter can only join a dispatcher's team).
godot-editor3d-ja_esta_ligado_para_desfazer = Already linked: { $to } → { $res }. To undo, click the line and then the ×.
godot-editor3d-ligado_ctrl_z_desfaz = Linked: { $said }. Ctrl+Z undoes.
godot-editor3d-aponte_para_o_chao_para = Point at the floor to place the object.
godot-editor3d-equipe_clique_no_transportador_ou = Team: click the target transporter or dispatcher.
godot-editor3d-equipe_comece_num_despachante_ou = Team: start at a dispatcher or an object.
godot-editor3d-equipe_clique_no_destino_esc = Team: { $from } → click the destination (Esc cancels).
godot-editor3d-equipe_solte_sobre_o_transportador = Team: drop on the transporter or dispatcher.
godot-editor3d-equipe_solte_sobre_outro_objeto = Team: drop on another object.
godot-editor3d-aponte_para_o_chao_para-2 = Point at the floor to place an aisle point.
godot-editor3d-corredor_clique_os_proximos_pontos = Aisle: click the next points; Enter or double-click finishes, Backspace goes back one point, Esc cancels.
godot-editor3d-ponto_s_enter_ou_duplo =
    { $aisle_points } { $aisle_points ->
        [one] point
       *[other] points
    }. Enter or double-click finishes.
godot-editor3d-vertice_de_selecionado_arraste_para = Vertex { $k } of { $name } selected (drag to move).
godot-editor3d-corredor_selecionado_del_exclui_shift = Aisle { $name } selected (Del deletes; Shift + click on the line adds a vertex).
godot-editor3d-um_corredor_precisa_de_ao = An aisle needs at least 2 points.
godot-editor3d-ponto_removido = Point removed ({ $aisle_points }).
godot-editor3d-trecho_selecionado_del_remove = Segment { $a } → { $b } selected (Del removes).
godot-editor3d-aponte_para_o_chao_para-3 = Point at the floor to place a node.
godot-editor3d-zona_ponto_s_clique_no =
    Zone: { $zone_points } { $zone_points ->
        [one] point
       *[other] points
    }. Click the first point, Enter or double-click closes; Backspace goes back; Esc cancels.
godot-editor3d-zona_arraste_para_mover_arraste = Zone { $name }: drag to move, drag a vertex to change the shape; Del deletes.
godot-editor3d-zona_clique_para_por_os = Zone: click to place the other points; Enter, double-click or the first point closes.
godot-editor3d-zona_alterada_ctrl_z_desfaz = Zone changed (Ctrl+Z undoes).
godot-editor3d-uma_zona_precisa_de_pelo = A zone needs at least 3 points.

## indicadores.rs

godot-indicadores-itens_refugados = Scrapped items
godot-indicadores-itens_que_terminaram_neste_refugo = Items that ended up in this scrap, per replication.
godot-indicadores-defeitos_gerados = Defects generated
godot-indicadores-defeitos_que_este_posto_gerou = Defects this station generated, per replication.
godot-indicadores-defeitos_que_escaparam = Defects that escaped
godot-indicadores-defeitos_deste_posto_que_chegaram = Defects from this station that reached the customer (external failure), per replication.
godot-indicadores-rendimento_de_primeira = First-pass yield
godot-indicadores-itens_inspecionados_uma_so_vez = Items inspected only once and delivered, over those inspected.
godot-indicadores-custo_da_qualidade = Cost of quality
godot-indicadores-avaliacao_falha_interna_e_falha = Appraisal, internal failure and external failure attributed to this station, per replication.
godot-indicadores-tempo_em_zonas = Time in zones
godot-indicadores-tempo_desta_pessoa_dentro_das = Time this person spent inside the risk zones, per replication.
godot-indicadores-exposicao_nas_zonas = Exposure in zones
godot-indicadores-tempo_de_pessoas_dentro_das = Time people spent inside the zones where this station is, summed, per replication.
godot-indicadores-zonas = Zones
godot-indicadores-zonas_de_risco_onde_o = Risk zones where the station is.
godot-indicadores-entradas_sem_epi = Entries without PPE
godot-indicadores-entradas_na_zona_sem_o = Entries into the zone without the protective equipment it requires, per replication.
godot-indicadores-quase_conflitos = Near-conflicts
godot-indicadores-pessoa_e_veiculo_mais_perto = Person and vehicle closer than the threshold, involving this resource, per replication. Simulated: useful for comparing layouts, does not estimate real accidents.
godot-indicadores-pessoa_e_veiculo_mais_perto-2 = Person and vehicle closer than the threshold within 3 m of this station, per replication. Simulated: useful for comparing layouts, does not estimate real accidents.
godot-indicadores-quebras = Breakdowns
godot-indicadores-falhas_do_posto_por_replicacao = Station failures, per replication.
godot-indicadores-tempo_medio_entre_falhas_tempo = Mean time between failures (uptime over the number of failures).
godot-indicadores-tempo_medio_de_reparo_da = Mean time to repair, from the breakdown until production resumes.
godot-indicadores-tempo_parado = Downtime
godot-indicadores-tempo_em_manutencao_corretiva_por = Time in corrective maintenance, per replication.
godot-indicadores-disponibilidade = Availability
godot-indicadores-tempo_produzindo_sobre_o_tempo = Time producing over planned time (excluding shifts and preventive maintenance).
godot-indicadores-custo_de_manutencao = Maintenance cost
godot-indicadores-tempo_parado_reparos_e_preventivas = Downtime, repairs and preventive maintenance times the block's costs, per replication.
godot-indicadores-parcela_do_tempo_em_quebra = Share of time in breakdown, per replication.
godot-indicadores-tempo_fora_de_quebra_sobre = Time outside breakdown over planned time (excluding shifts).

## jobs.rs

godot-jobs-rodando_replicacoes = Running replications
godot-jobs-rodando_mais_replicacoes = Running more replications
godot-jobs-comparando_cenarios = Comparing scenarios
godot-jobs-otimizando = Optimizing
godot-jobs-analisando_o_aquecimento = Analyzing the warm-up
godot-jobs-conferindo_a_lei_de_little = Checking Little's law
godot-jobs-procurando_o_gargalo_movel = Looking for the moving bottleneck
godot-jobs-calculando_o_pert_cpm = Calculating the PERT/CPM
godot-jobs-ajustando_a_distribuicao_aos_dados = Fitting the distribution to the data
godot-jobs-gravando_o_relatorio_dos_dados = Writing the data report
godot-jobs-erro_interno_durante_a_execucao = internal error during the run
godot-jobs-a_execucao_terminou_sem_resultado = the run finished without a result
godot-jobs-objeto_nao_existe_no_modelo = object "{ $object }" does not exist in the model

## linha.rs


## path_edit.rs

godot-path-edit-nao_existe = '{ $name }' does not exist
godot-path-edit-e_uma_esteira_reta_sem = '{ $name }' is a straight conveyor, with no nodes
godot-path-edit-nao_e_uma_esteira = '{ $name }' is not a conveyor
godot-path-edit-a_esteira_precisa_de_ao = the conveyor needs at least 2 different nodes
godot-path-edit-a_biblioteca_nao_tem_esteira = the library has no conveyor
godot-path-edit-o_caminho_precisa_de_ao = the path needs at least 2 different points
godot-path-edit-nao_existe_o_caminho = path '{ $name }' does not exist
godot-path-edit-nao_tem_o_no = '{ $name }' has no node { $index }
godot-path-edit-o_raio_deve_ser_um = the radius must be a number >= 0 (meters)
godot-path-edit-a_distancia_do_paralelo_deve = the parallel distance must be nonzero
godot-path-edit-o_caminho_novo_sumiu = the new path vanished
godot-path-edit-o_caminho_precisa_de_ao-2 = the path needs at least 2 points
godot-path-edit-a_esteira_precisa_de_ao-2 = the conveyor needs at least 2 nodes
godot-path-edit-nao_e_uma_esteira_desenhada = '{ $name }' is not a drawn conveyor
godot-path-edit-as_duas_pontas_ja_estao = both ends are already on the floor
godot-path-edit-tipo_de_esteira_desconhecido = unknown conveyor type: { $style }
godot-path-edit-acionamento_desconhecido = unknown drive: { $drive }
godot-path-edit-a_largura_deve_ser_0 = the width must be > 0 (meters)
godot-path-edit-a_inclinacao_maxima_fica_entre = the maximum slope is between 0 and 90 degrees
godot-path-edit-divida_num_no_do_meio = split at a node in the middle of the conveyor
godot-path-edit-nao_ha_trecho_depois_do = there is no segment after the last node
godot-path-edit-o_caminho_so_tem_esse = the path has only this segment: delete the whole path
godot-path-edit-a_esteira_so_tem_esse = the conveyor has only this segment: delete the whole conveyor
godot-path-edit-nenhum_trajeto_comeca_onde_este = no route starts where this one ends
godot-path-edit-nao_existe_o_caminho-2 = path '{ $other }' does not exist

## tool_panels.rs

godot-tool-panels-ligacao = Link
godot-tool-panels-remover_ligacao_del = Remove link (Del)
godot-tool-panels-rede_de_caminhos = Path network
godot-tool-panels-no_s_trecho_s_pista =
    { $nodes } { $nodes ->
        [one] node
       *[other] nodes
    }, { $edges } { $edges ->
        [one] segment
       *[other] segments
    }, { $controlled } { $controlled ->
        [one] controlled lane
       *[other] controlled lanes
    }.
godot-tool-panels-clique_no_chao_para_por = Click on the floor to place a node; with a node armed (green), the next click links it to the new one by a segment. Click a node to arm it, drag to move it. Click a segment to change its direction and capacity (with a capacity, the lane is controlled and shown in red). Esc ends the path; Del deletes the selection.
godot-tool-panels-novos_trechos_em_mao_dupla = New segments two-way
godot-tool-panels-no = Node { $name }
godot-tool-panels-nome = Name
godot-tool-panels-posicao_m = Position ({ $position }, { $position2 }) m
godot-tool-panels-excluir_no_del = Delete node (Del)
godot-tool-panels-trecho = Segment { $from } → { $to }
godot-tool-panels-comprimento_m = Length { $length } m
godot-tool-panels-mao_dupla = Two-way
godot-tool-panels-capacidade_veiculos_0_sem_controle = Capacity (vehicles; 0 = no control)
godot-tool-panels-pista_controlada_no_maximo_veiculo =
    Controlled lane: at most { $cap } { $cap ->
        [one] vehicle
       *[other] vehicles
    } at the same time.
godot-tool-panels-inverter_sentido = Reverse direction
godot-tool-panels-excluir_trecho_del = Delete segment (Del)
godot-tool-panels-pedestres = Pedestrians
godot-tool-panels-empilhadeiras = Forklifts
godot-tool-panels-misto = Mixed
godot-tool-panels-corredores_de_circulacao = Circulation aisles
godot-tool-panels-corredor_es_de_pedestres_de =
    { $list } { $list ->
        [one] aisle
       *[other] aisles
    }: { $pedestrian } pedestrian, { $forklift } forklift, { $forklift2 } mixed; { $crossings } { $crossings ->
        [one] crossing
       *[other] crossings
    }.
godot-tool-panels-clique_no_chao_para_por-2 = Click on the floor to place the centerline points; Enter or double-click finishes, Backspace goes back one point, Esc cancels. Points snap to other aisles. Click an aisle to edit it, drag a vertex to move it, Shift + click on the line adds a vertex. The yellow stripes, the crossing zebra and the arrows appear on the floor.
godot-tool-panels-novos_corredores = New aisles
godot-tool-panels-tipo = Type
godot-tool-panels-largura_m = Width (m)
godot-tool-panels-corredor = Aisle { $name }
godot-tool-panels-ponto_s_m_de_comprimento =
    { $points } { $points ->
        [one] point
       *[other] points
    }, { $length } m long
godot-tool-panels-passagem_secundaria_de_pedestres_minimo = Secondary pedestrian passage (smaller minimum)
godot-tool-panels-perfil = Profile
godot-tool-panels-padrao_do_modelo = Model default
godot-tool-panels-brasil_amarelo = Brazil (yellow)
godot-tool-panels-brasil = Brazil
godot-tool-panels-branco = White
godot-tool-panels-vertice_selecionado = Vertex { $v } selected
godot-tool-panels-remover_vertice = Remove vertex
godot-tool-panels-excluir_corredor_del = Delete aisle (Del)
godot-tool-panels-zonas_de_risco = Risk zones
godot-tool-panels-zona_s_de_risco_no =
    { $zones } risk { $zones ->
        [one] zone
       *[other] zones
    } on the floor.
godot-tool-panels-clique_no_chao_para_por-3 = Click on the floor to place the corners; the first point, Enter or double-click closes the zone, Backspace goes back one point, Esc cancels. Click a zone to edit it: drag the middle to move, drag a vertex to change the shape; Del deletes. Points snap to the grid.
godot-tool-panels-zona = Zone { $name }
godot-tool-panels-canto_s =
    { $polygon } { $polygon ->
        [one] corner
       *[other] corners
    }
godot-tool-panels-nivel_de_risco_1_a = Risk level (1 to 5; 0 = no level)
godot-tool-panels-epi_exigido_para_entrar = PPE required to enter
godot-tool-panels-excluir_zona_del = Delete zone (Del)

## ui_arrivals.rs

godot-ui-arrivals-nao_e_um_numero = "{ $t }" is not a number
godot-ui-arrivals-use_um_numero = use a number
godot-ui-arrivals-hh_mm_so_vale_com = hh:mm is only valid with a day or week cycle (1440 min, 24 h, 86400 s); use a number
godot-ui-arrivals-hora_invalida_em = invalid hour in "{ $t }"
godot-ui-arrivals-minuto_invalido_em = invalid minute in "{ $t }"
godot-ui-arrivals-minuto_invalido_em_0_a = invalid minute in "{ $t }" (0 to 59)
godot-ui-arrivals-nao_e_um_horario_hh = "{ $t }" is not a time (hh:mm) nor a number
godot-ui-arrivals-quantidade_um_inteiro_0 = quantity "{ $t }": an integer >= 0
godot-ui-arrivals-rotulo_use_nome_valor = label "{ $part }": use name=value
godot-ui-arrivals-rotulo_sem_nome = label "{ $part }" has no name
godot-ui-arrivals-rotulo = label { $name }: { $e }
godot-ui-arrivals-uma_vez = Once
godot-ui-arrivals-a_cada_ciclo = Every cycle
godot-ui-arrivals-volta_ao_inicio = Back to the start
godot-ui-arrivals-por_ciclo_de = per cycle of { $d }
godot-ui-arrivals-por_volta = per lap
godot-ui-arrivals-no_total = in total
godot-ui-arrivals-em_linha_s =
    { $items } { $items ->
        [one] item
       *[other] items
    } { $per } in { $rows } { $rows ->
        [one] row
       *[other] rows
    }
godot-ui-arrivals-tipo_da_fonte = source type
godot-ui-arrivals-um_dia = { $d } (one day)
godot-ui-arrivals-uma_semana = { $d } (one week)
godot-ui-arrivals-tipo_de_item = item type
godot-ui-arrivals-nada_para_colar_copie_as = nothing to paste: copy the rows from the spreadsheet
godot-ui-arrivals-linha = row { $n }: { $e }
godot-ui-arrivals-falta_o_instante_modo_horario = the instant is missing (schedule mode)
godot-ui-arrivals-so_o_cabecalho_nenhuma_linha = only the header: no rows to paste
godot-ui-arrivals-editar_como_texto_ron = Edit as text (RON)
godot-ui-arrivals-modo = Mode
godot-ui-arrivals-horario = Schedule
godot-ui-arrivals-sequencia = Sequence
godot-ui-arrivals-horario_cada_linha_no_seu = Schedule: each row at its own instant. Sequence: one row per arrival, with the source's interarrival time between them.
godot-ui-arrivals-repetir = Repeat
godot-ui-arrivals-ciclo = Cycle
godot-ui-arrivals-da_fonte = (from the source)
godot-ui-arrivals-o_tipo_da_fonte = The source's type
godot-ui-arrivals-tipo_de_item_da_linha = Item type of the row
godot-ui-arrivals-subir_a_linha = Move the row up
godot-ui-arrivals-descer_a_linha = Move the row down
godot-ui-arrivals-remover_a_linha = Remove the row
godot-ui-arrivals-linha-2 = + Row
godot-ui-arrivals-acrescenta_uma_linha_no_fim = Adds a row at the end
godot-ui-arrivals-ordenar_por_instante = Sort by instant
godot-ui-arrivals-poe_as_linhas_em_ordem = Puts the rows in instant order
godot-ui-arrivals-as_linhas_ja_estao_em = The rows are already in order (or it is a sequence)
godot-ui-arrivals-colar_da_planilha = Paste from spreadsheet
godot-ui-arrivals-linhas_copiadas_de_uma_planilha = Rows copied from a spreadsheet (separated by tab or ;)
godot-ui-arrivals-hora_qtd_tipo_rotulo_08 =
    time; qty; type; label...
    08:00; 12; box; priority=2
godot-ui-arrivals-qtd_tipo_rotulo_2_caixa =
    qty; type; label...
    2; box; priority=2
godot-ui-arrivals-substituir_as_linhas = Replace the rows
godot-ui-arrivals-acrescentar = Append
godot-ui-arrivals-ver_as_primeiras_chegadas = View the first arrivals
godot-ui-arrivals-histograma_das_chegadas_de_um = Histogram of the arrivals in one cycle (or one lap), by hour and by type
godot-ui-arrivals-simulado_so_a_fonte_ligada = Simulated: only the source, linked to an exit, for one cycle.
godot-ui-arrivals-estimado_pela_tabela_a_fonte = Estimated from the table (the source alone did not run: { $e }).
godot-ui-arrivals-estimado_pela_tabela_rodando_a = Estimated from the table; running the source...
godot-ui-arrivals-maximo_por = maximum { $num_text } per { $bin_label }
godot-ui-arrivals-por_instantes_da_tabela_sem = per { $bin_label } ({ $bin }); table instants, without running the model
godot-ui-arrivals-horarios_em_hh_mm_taxas = { $name } (times in hh:mm, rates per hour)
godot-ui-arrivals-unidade_do_modelo = model unit
godot-ui-arrivals-fora_dos_periodos_a_fonte = Outside the periods the source creates no items
godot-ui-arrivals-periodo_s =
    { $periods } { $periods ->
        [one] period
       *[other] periods
    }
godot-ui-arrivals-itens_h_ou_distribuicao = items/h or distribution
godot-ui-arrivals-taxa_ou_distribuicao = rate or distribution
godot-ui-arrivals-padrao_o_tempo_entre_chegadas = Default: the source's interarrival time. Distribution: its own interarrival time. Rate: Poisson arrivals. Closed: no arrivals.
godot-ui-arrivals-distribuicao = distribution: { $e }
godot-ui-arrivals-media = mean { $mean }
godot-ui-arrivals-sem_chegadas = no arrivals
godot-ui-arrivals-remover_o_periodo = Remove the period
godot-ui-arrivals-periodo = + Period
godot-ui-arrivals-acrescenta_um_periodo_depois_do = Adds a period after the last one
godot-ui-arrivals-o_ultimo_periodo_ja_vai = The last period already runs to the end of the cycle
godot-ui-arrivals-esperado_por_ciclo_chegadas = Expected per cycle: { $total } arrivals
godot-ui-arrivals-padrao_media = default (mean { $mean })
godot-ui-arrivals-distribuicao-2 = distribution { $d }
godot-ui-arrivals-taxa_itens_h = rate { $cycle } items/h
godot-ui-arrivals-taxa = rate { $num_text }
godot-ui-arrivals-periodo_a_esperadas = Period { $k }: { $cycle } to { $cycle2 }, { $how }; expected { $default }
godot-ui-arrivals-chegadas_esperadas_em_cada_periodo = Expected arrivals in each period (rate × duration, or duration ÷ mean)
godot-ui-arrivals-barra_s =
    { $exp } { $exp ->
        [one] bar
       *[other] bars
    }
godot-ui-arrivals-nao_existe = '{ $source }' does not exist
godot-ui-arrivals-nao_e_uma_chegada = '{ $source }' is not an arrival

## ui_bar.rs

godot-ui-bar-equipe_despachante_membro = Team (dispatcher → member)
godot-ui-bar-transporte_pedido = Transport requested
godot-ui-bar-operadores_pedidos = Operators requested
godot-ui-bar-equipe_de_manutencao = Maintenance team
godot-ui-bar-local_de_pausa = Break location
godot-ui-bar-mais = More
godot-ui-bar-os_grupos_que_nao_cabem = The groups that do not fit in the bar
godot-ui-bar-arquivo = File
godot-ui-bar-editar = Edit
godot-ui-bar-organizar = Arrange
godot-ui-bar-grade = Grid
godot-ui-bar-ajuda = Help
godot-ui-bar-analise = Analysis
godot-ui-bar-novo = New
godot-ui-bar-novo_modelo_ctrl_n = New model (Ctrl+N)
godot-ui-bar-abrir = Open…
godot-ui-bar-abrir_um_modelo_ou_um = Open a model or a replay (Ctrl+O)
godot-ui-bar-salvar = Save
godot-ui-bar-salvar_ctrl_s = Save (Ctrl+S)
godot-ui-bar-salvar_como = Save as…
godot-ui-bar-salvar_com_outro_nome_ctrl = Save with another name (Ctrl+Shift+S)
godot-ui-bar-desfazer = Undo
godot-ui-bar-desfazer_ctrl_z = Undo (Ctrl+Z)
godot-ui-bar-refazer = Redo
godot-ui-bar-refazer_ctrl_y = Redo (Ctrl+Y)
godot-ui-bar-alinhar_a_esquerda = Align left
godot-ui-bar-alinhar_ao_centro_x = Align center (X)
godot-ui-bar-alinhar_a_direita = Align right
godot-ui-bar-alinhar_ao_topo = Align top
godot-ui-bar-alinhar_ao_centro_y = Align center (Y)
godot-ui-bar-alinhar_a_base = Align bottom
godot-ui-bar-distribuir_em_x = Distribute in X
godot-ui-bar-distribuir_em_y = Distribute in Y
godot-ui-bar-copiar_ctrl_c = Copy (Ctrl+C)
godot-ui-bar-recortar_ctrl_x = Cut (Ctrl+X)
godot-ui-bar-colar_ctrl_v = Paste (Ctrl+V)
godot-ui-bar-duplicar_ctrl_d = Duplicate (Ctrl+D)
godot-ui-bar-excluir_del = Delete (Del)
godot-ui-bar-selecionar_tudo_ctrl_a = Select all (Ctrl+A)
godot-ui-bar-alinhar_distribuir_copiar_colar_e = Align, distribute, copy, paste and delete the selection
godot-ui-bar-encaixar_na_grade_ao_por = Snap to the grid when placing and dragging (G; step beside it)
godot-ui-bar-passo_da_grade = Grid step
godot-ui-bar-atalhos_f1_a_lista_dos = Shortcuts (F1): the list of keyboard and mouse shortcuts
godot-ui-bar-resultados = Results
godot-ui-bar-tabela_graficos_analises_cenarios_e = Table, charts, analyses, scenarios and optimization of the last run
godot-ui-bar-pert_cpm = PERT/CPM
godot-ui-bar-pert_cpm_da_selecao_com = PERT/CPM of the selection (with two objects, from one to the other)
godot-ui-bar-pert_cpm_da_selecao = PERT/CPM of the selection
godot-ui-bar-pert_cpm_de_ate_dois = PERT/CPM from… to… (two selected)
godot-ui-bar-fluxos_de_tokens = Token flows
godot-ui-bar-modo_missao_utsim_ensino = Mission mode (UTSim Ensino)
godot-ui-bar-abre_as_missoes_do_utsim = Opens the UTSim Ensino missions in this same program
godot-ui-bar-outras_analises_pert_cpm_de = Other analyses: PERT/CPM from… to…, token flows; the team and request links in the 3D view
godot-ui-bar-simular = Simulate
godot-ui-bar-montar_o_modelo_tab_alterna = Build the model (Tab toggles)
godot-ui-bar-rodar_as_replicacoes_e_reproduzir = Run the replications and play back (Tab toggles)
godot-ui-bar-selecionar = Select
godot-ui-bar-selecionar_e_mover_esc = Select and move (Esc)
godot-ui-bar-ligar = Link
godot-ui-bar-ligar_arraste_da_origem_ao = Link: drag from the source to the destination, or click the source and then the destination (L)
godot-ui-bar-rede = Network
godot-ui-bar-rede_de_caminhos_nos_e = Path network: nodes and segments (N)
godot-ui-bar-corredor = Aisle
godot-ui-bar-corredores_de_circulacao_com_faixas = Circulation aisles with safety stripes: click points on the floor, Enter finishes (C)
godot-ui-bar-zona = Zone
godot-ui-bar-zonas_de_risco_no_piso = Risk zones on the floor: click the corners, the first point or Enter closes; click a zone to edit (Z)
godot-ui-bar-equipe = Team
godot-ui-bar-equipe_arraste_do_despachante_a = Team: drag from the dispatcher to a transporter (joins or leaves the team), or from an object to the transporter or dispatcher that carries its items; with Shift, from a process to whoever supplies the operators (E; Alt with Link)
godot-ui-bar-ligacoes_na_vista_3d = Links in the 3D view
godot-ui-bar-tambem_durante_a_simulacao = Also during the simulation
godot-ui-bar-as_linhas_tracejadas_entre_operadores = The dashed lines between operators, vehicles and machines in Simulate mode

## ui_fitwin.rs

godot-ui-fitwin-linha_nao_e_um_numero = row { $k }: "{ $t }" is not a number
godot-ui-fitwin-os_instantes_precisam_ser_crescentes = the instants must be increasing
godot-ui-fitwin-digite_ou_cole_pelo_menos = type or paste at least two values
godot-ui-fitwin-compare_as_distribuicoes_antes = compare the distributions first
godot-ui-fitwin-utsim_3d_valores_reais = UTSim 3D, Real values
godot-ui-fitwin-desvio_padrao = standard deviation
godot-ui-fitwin-valores_reais = Real values: { $field }
godot-ui-fitwin-o_que_e = What it is: { $title }
godot-ui-fitwin-valores_observados = Observed values
godot-ui-fitwin-digite_cole_da_planilha_varias = Type, paste from a spreadsheet (multiple rows, decimal comma) or import a CSV.
godot-ui-fitwin-cole_aqui_e_clique_adicionar = paste here and click Add
godot-ui-fitwin-adicionar = Add
godot-ui-fitwin-valor_es_adicionados =
    { $n } { $n ->
        [one] value added
       *[other] values added
    }.
godot-ui-fitwin-importar_csv = Import CSV…
godot-ui-fitwin-le_os_valores_de_um = Reads the values from a CSV or TXT file
godot-ui-fitwin-limpar = Clear
godot-ui-fitwin-apaga_todos_os_valores_da = Deletes all the values in the table
godot-ui-fitwin-sao_instantes_crescentes_usar_os = They are increasing instants (use the intervals)
godot-ui-fitwin-marque_quando_os_valores_sao = Check when the values are arrival times: the fit uses the differences between them.
godot-ui-fitwin-valor_es =
    { $n } { $n ->
        [one] value
       *[other] values
    }
godot-ui-fitwin-apaga_este_valor = Deletes this value
godot-ui-fitwin-intervalos = Intervals
godot-ui-fitwin-estatisticas = Statistics
godot-ui-fitwin-comparacao_das_distribuicoes = Comparison of the distributions
godot-ui-fitwin-comparar_de_novo = Compare again
godot-ui-fitwin-comparar_todas = Compare all
godot-ui-fitwin-ajusta_todas_as_distribuicoes_candidatas = Fits all the candidate distributions and computes all the measures at once
godot-ui-fitwin-ajustando_todas_as_candidatas = fitting all the candidates...
godot-ui-fitwin-depois_de_comparar_a_tabela = After comparing, the table shows all the measures for each distribution; click a row to see the charts and choose.
godot-ui-fitwin-os_valores_mudaram_compare_de = The values changed: compare again.
godot-ui-fitwin-ordenada_por_sugestao_inicial_aic = Sorted by { $label }. Initial suggestion (AIC with the UTSim rule): { $family }.
godot-ui-fitwin-usar_esta_distribuicao = Use this distribution ({ $fam })
godot-ui-fitwin-passa_a_usar_esta_distribuicao = Starts using this distribution in this field, fitted to these values (Ctrl+Z undoes)
godot-ui-fitwin-usar_a_empirica = Use the empirical one
godot-ui-fitwin-usa_os_proprios_valores_observados = Uses the observed values themselves, with no family
godot-ui-fitwin-gerar_relatorio = Generate report
godot-ui-fitwin-grava_o_relatorio_html_com = Writes the HTML report with the data, the full comparison, the charts and the choice
godot-ui-fitwin-fechar = Close
godot-ui-fitwin-fecha_a_explicacao = Closes the explanation
godot-ui-fitwin-como_ler_quando_usar_cuidados =
    { $what }
    How to read: { $read }
    When to use: { $when }
    Cautions: { $care }
godot-ui-fitwin-aceita_pelos_testes_clique_para = Accepted by the tests. Click to see the charts and choose.
godot-ui-fitwin-nao_se_aplica_a_estes = Does not apply to these data.
godot-ui-fitwin-rejeitada_por_algum_teste_valor = Rejected by some test (p-value below 0.05). It can still be chosen.
godot-ui-fitwin-ordena_por = Sort by { $label } ({ $melhor })
godot-ui-fitwin-escolhida_por = chosen by
godot-ui-fitwin-histograma_e_curvas_em_destaque = Histogram and curves ({ $selected_family } highlighted)
godot-ui-fitwin-poisson = Poisson
godot-ui-fitwin-geometrica = Geometric
godot-ui-fitwin-binomial = Binomial
godot-ui-fitwin-binomial_negativa = Negative binomial
godot-ui-fitwin-a_empirica_reproduz_os_proprios = The empirical one reproduces the data itself: there is no curve to compare.

## ui_left.rs

godot-ui-left-orbita = Orbit
godot-ui-left-planta_2d = Plan (2D)
godot-ui-left-seguir = Follow
godot-ui-left-primeira_pessoa = First person
godot-ui-left-do_recurso = From the resource
godot-ui-left-recentes = Recent
godot-ui-left-usados_por_ultimo = Last used
godot-ui-left-espera = Wait
godot-ui-left-fluxo = Flow
godot-ui-left-fluxo_de_itens_e_montagem = Item flow and assembly
godot-ui-left-mover = Move
godot-ui-left-movimentacao_recursos_e_logistica = Handling, resources and logistics
godot-ui-left-fabrica = Factory
godot-ui-left-manufatura = Manufacturing
godot-ui-left-servicos = Services
godot-ui-left-servicos_e_saude = Services and health
godot-ui-left-escritorio = Office
godot-ui-left-escritorio_e_processos = Office and processes
godot-ui-left-q_e_s = Q and S
godot-ui-left-qualidade_e_seguranca = Quality and safety
godot-ui-left-biblioteca = Library
godot-ui-left-objetos = Objects
godot-ui-left-mostrar_obstaculos_e_folgas = Show obstacles and clearances
godot-ui-left-mostra_a_area_ocupada_dos = Shows the footprint of the objects, the walls, the clearance of the resources and each object's access point (drag it with the Select tool)
godot-ui-left-nada_com_esse_nome = Nothing with that name.
godot-ui-left-clique_de_novo_para_recolher = { $title }. Click again to collapse.
godot-ui-left-os_elementos_que_voce_usar = The elements you use appear here.
godot-ui-left-buscar_elemento = Search element
godot-ui-left-limpa_a_busca = Clears the search
godot-ui-left-clique_e_depois_clique_no = { $label }: click, then click on the floor, or drag to the floor
godot-ui-left-buscar_por_nome_ou_tipo = Search by name or type
godot-ui-left-problema_s =
    { $problems } { $problems ->
        [one] problem
       *[other] problems
    }
godot-ui-left-aviso_s =
    { $warnings } { $warnings ->
        [one] warning
       *[other] warnings
    }
godot-ui-left-simular_de_novo = Simulate again
godot-ui-left-roda_as_replicacoes_outra_vez = Runs the replications again
godot-ui-left-blocos_clique_para_focar = Blocks (click to focus)
godot-ui-left-fluxos = Flows
godot-ui-left-fluxos_de_tokens_em_2d = Token flows in 2D, with counts over time
godot-ui-left-gravar_replay = Record replay
godot-ui-left-grava_o_replay_desta_replicacao = Writes the replay of this replication to .json (opens again with Open)
godot-ui-left-rotulos = Labels
godot-ui-left-os_nomes_dos_blocos_sobre = The block names over the scene
godot-ui-left-sombras = Shadows
godot-ui-left-as_sombras_do_sol_desligadas = The sun's shadows (off, the scene is lighter)
godot-ui-left-seguranca = Safety
godot-ui-left-zonas_de_risco_epi_das = Risk zones, people's PPE, simulated near-conflicts, defective items and inspections
godot-ui-left-calor = Heat
godot-ui-left-mapa_de_calor_da_presenca = Heat map of the presence of people and near-conflicts on the floor
godot-ui-left-quase_conflitos_simulados = Simulated near-conflicts ({ $quase })
godot-ui-left-replicacoes = Replications
godot-ui-left-uma_replicacao_a_menos = One replication fewer
godot-ui-left-uma_replicacao_a_mais = One replication more
godot-ui-left-camera = Camera
godot-ui-left-seguir_primeira_pessoa_e_do = Follow, First person and From the resource track the block chosen in the list
godot-ui-left-os_resultados_aparecem_aqui_depois = The results appear here after the simulation.
godot-ui-left-gargalo = Bottleneck: { $bottleneck }
godot-ui-left-a_tabela_completa_os_graficos = The full table, the charts and the analyses are in Results.
godot-ui-left-gargalo-2 =
    {""}
    Bottleneck: { $bottleneck }

## ui_pert.rs

godot-ui-pert-rede = Network
godot-ui-pert-gantt = Gantt
godot-ui-pert-distribuicao = Distribution
godot-ui-pert-tabela = Table
godot-ui-pert-texto = Text
godot-ui-pert-simulado = Simulated
godot-ui-pert-caminho_critico_observado_na_simulacao = Critical path observed in the simulation: queues, transport and resources included
godot-ui-pert-classico = Classic
godot-ui-pert-pert_classico_estimativas_a_m = Classic PERT: estimates (a, m, b), CPM on the means and P(deadline) by the normal
godot-ui-pert-monte_carlo = Monte Carlo
godot-ui-pert-sorteios_das_duracoes_sobre_a = Draws of the durations over the network: criticality and distribution of the total time
godot-ui-pert-desvio = , deviation { $sd }
godot-ui-pert-unidades_sorteios = {" "}({ $units } units/draws)
godot-ui-pert-tempo_total_cpm_nas_medias = Total time: { $t }{ $spread }{ $units }. CPM on the means: { $project }.
godot-ui-pert-ic_a = {" "}(CI { $lo }% to { $hi }%)
godot-ui-pert-p_t = P(T ≤ { $deadline }) = { $p }
godot-ui-pert-caminho_critico = Critical path: { $names } ({ $index }%)
godot-ui-pert-aviso = warning: { $w }
godot-ui-pert-aproximacao_normal_do_pert_classico = normal approximation of the classic PERT
godot-ui-pert-unidades_observadas = observed units
godot-ui-pert-a_linha_amarela_e_o = ; the yellow line is the deadline
godot-ui-pert-distribuicao_do_tempo_total = Distribution of the total time ({ $source }){ $marker }.
godot-ui-pert-pert_cpm_tempo_total = PERT/CPM ({ $mode }): total time { $total }.
godot-ui-pert-atividade = Activity
godot-ui-pert-duracao = Duration
godot-ui-pert-folga_total = Total float
godot-ui-pert-folga_livre = Free float
godot-ui-pert-criticidade = Criticality
godot-ui-pert-prazo = deadline { $num }
godot-ui-pert-sem_distribuicao_para_este_modo = no distribution for this mode
godot-ui-pert-hipotese = assumption: { $h }
godot-ui-pert-fechar = Close
godot-ui-pert-fechar_a_janela_pert_cpm = Close the PERT/CPM window
godot-ui-pert-prazo-2 = Deadline
godot-ui-pert-sem_prazo = no deadline
godot-ui-pert-sorteios_monte_carlo = Draws (Monte Carlo)
godot-ui-pert-recalcular = Recalculate
godot-ui-pert-calcula_de_novo_no_modo = Calculates again in the current mode
godot-ui-pert-relatorio_html = .html report
godot-ui-pert-exportar_html = Export HTML
godot-ui-pert-grava_o_relatorio_html_do = Writes the PERT/CPM HTML report
godot-ui-pert-csv = CSV
godot-ui-pert-grava_as_tabelas_em_csv = Writes the tables as CSV in the file's folder

## zone_edit.rs

godot-zone-edit-trafego_de_veiculos = Vehicle traffic
godot-zone-edit-alcance_de_maquina = Machine reach
godot-zone-edit-carga_suspensa = Suspended load
godot-zone-edit-ruido = Noise
godot-zone-edit-calor = Heat
godot-zone-edit-outro = Other
godot-zone-edit-a_zona_nao_existe = zone '{ $name }' does not exist
godot-zone-edit-uma_zona_precisa_de_pelo = a zone needs at least 3 points
godot-zone-edit-a_zona_precisa_de_um = the zone needs a name
godot-zone-edit-tipo_de_zona_desconhecido = unknown zone type: { $value }
godot-zone-edit-nivel_um_numero_de_1 = level: a number from 1 to 5
godot-zone-edit-campo_de_zona_desconhecido = unknown zone field: { $field }

## Round 13: dialogs and error messages

godot-dialogo-descartar-titulo = Discard changes?
godot-dialogo-descartar-texto = The model has unsaved changes. Discard?
godot-dialogo-descartar = Discard
godot-dialogo-voltar = Back
godot-dialogo-atalhos = Shortcuts
godot-dialogo-fechar = Close
godot-dialogo-desempenho = Shows or hides the performance measurements over the 3D view

godot-schema-distribuicao_invalida = invalid distribution: { $e }
godot-schema-familia_invalida = invalid family
godot-schema-texto_ron_esperado = RON text expected
godot-schema-so_a_distribuicao_deslocada_tem = only the shifted distribution has an inner distribution
godot-schema-a_distribuicao_nao_tem_o = the distribution has no parameter "{ $key }"
godot-schema-modo_de_movimentacao_desconhecido = unknown movement mode: { $name }
godot-schema-modo_de_movimentacao_esperado = movement mode expected
godot-schema-a_velocidade_deve_ser_maior = the speed must be greater than zero
godot-schema-campo_desconhecido = unknown field: { $w }
godot-schema-rotulo_esperado = label expected
godot-schema-o_rotulo_do_peso_nao = the weight label cannot be empty
godot-schema-ordem_safe_sequential_ou_parallel = order: Safe, Sequential or Parallel
godot-schema-crie_antes_uma_pessoa_ou = first create a person (or a dispatcher) to operate the bridge crane
godot-schema-controle_cabin_pendant_ou_radio = control: Cabin, Pendant or Radio
godot-schema-escolha_a_equipe_do_operador = choose the operator's team
godot-schema-escolha_antes_botoeira_ou_radio = first choose pendant control or radio
godot-schema-tipo_de_espera_table_pallet = wait type: Table, Pallet, Shelf or Floor, not { $k }
godot-schema-camadas_demais = too many layers
godot-schema-espera_nao_tem_o_campo = wait has no field "{ $rest }"
godot-schema-espera = wait: { $e }
godot-schema-sexo_female_male_ou_vazio = sex: Female, Male or empty, not { $other }
godot-schema-idade_grande_demais = age too large
godot-schema-pessoa_nao_tem_o_campo = person has no field "{ $field }"
godot-schema-pessoa = person: { $e }
godot-schema-crie_antes_um_transportador_ou = first create a transporter or a dispatcher to operate the process
godot-schema-ligue_a_transferencia_antes = turn on the transfer first
godot-schema-caminho_straight_ou_network = path: Straight or Network
godot-schema-ligue_as_quebras_antes = turn on the breaks first
godot-schema-ligue_os_turnos_antes = turn on the shifts first
godot-schema-fim_do_turno_pause_ou = end of shift: Pause or Finish, not { $t }
godot-schema-pausas_invalidas = invalid breaks: { $e }
godot-schema-escolha_um_calendario = choose a calendar
godot-schema-ligue_o_lote_antes = turn on the batch first
godot-schema-ligue_antes = turn on { $head } first
godot-schema-ligue_os_operadores_antes = turn on the operators first
godot-schema-o_ganho_da_equipe_deve = the team's gain must be greater than zero
godot-schema-objeto_nao_existe = object "{ $object }" does not exist
godot-schema-posicao_x_y_ou_z = position: x, y or z
godot-schema-politica_esperada = policy expected
godot-schema-politica_de_acesso_desconhecida = unknown access policy: { $v }
godot-schema-o_rotulo_da_prioridade_nao = the priority label cannot be empty
godot-schema-script_esperado = script expected
godot-schema-regra_firstfree_random_ou_abc = rule: FirstFree, Random or Abc
godot-schema-o_rotulo_nao_pode_ficar = the label cannot be empty
godot-schema-modo_unpack_ou_split = mode: Unpack or Split
godot-schema-o_modelo_nao_tem_calendarios = the model has no calendars (create one in "calendars")
godot-schema-origem_dos_turnos_inline_ou = shift source: inline or calendar
godot-schema-texto_esperado = text expected
godot-schema-modo_esperado = mode expected
godot-schema-perfil_brasil_ou_branco = profile: Brasil or Branco
godot-schema-largura_da_faixa = lane width
godot-schema-parada_no_cruzamento = stop at the crossing
godot-schema-cor_invalida_use_rrggbb = invalid color: { $t } (use #rrggbb)
godot-schema-cor_rrggbb = color: #rrggbb
godot-schema-run_nao_tem_o_campo = run has no field "{ $path }"
godot-schema-regra_esperada = rule expected
godot-schema-lista_de_pesos_esperada = list of weights expected
godot-schema-pesos_invalidos = invalid weights: { $e }
godot-schema-campo_desconhecido_output = unknown field: output.{ $rest }
godot-schema-a_escala_precisa_ser_maior = the scale must be greater than zero
godot-schema-giro_um_numero_de_graus = rotation: a number of degrees
godot-schema-campo_desconhecido_visual = unknown field: visual.{ $rest }

godot-ui-bar-casa = Home

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

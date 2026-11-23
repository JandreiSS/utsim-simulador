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

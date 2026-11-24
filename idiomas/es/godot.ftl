## UTSim 3D (cliente Godot): configurações e campos de distribuição

barra-configuracoes = Configuración
barra-configuracoes-dica = Preferencias de UTSim 3D en este equipo
configuracoes-titulo = Configuración
configuracoes-distribuicoes = Los campos de distribución se abren como
configuracoes-formulario = Formulario
configuracoes-codigo = Código
configuracoes-distribuicoes-ajuda = Vale para todos los campos de distribución (tiempo de proceso, llegadas, carga, descarga...). El botón {"{"} {"}"} de cada campo sigue cambiando solo ese campo, hasta que se elija otro objeto.
configuracoes-fechar = Cerrar
props-editar_como_texto = Editar como código (texto RON)
props-editar_como_formulario = Volver al formulario

## Painéis ao vivo em janela própria

vivo-soltar = Ventana
vivo-soltar-dica = Abre este panel en una ventana propia, que puede ir a otro monitor; sigue en vivo
vivo-prender = Volver
vivo-prender-dica = Cierra la ventana y devuelve el panel sobre la vista 3D
vivo-janela-titulo = UTSim 3D: { $objeto }

## Idioma

configuracoes-idioma = Idioma
configuracoes-idioma-ajuda = Por ahora solo la ventana Configuración, los paneles en vivo y las pestañas Calidad, Seguridad y Mantenimiento de los nodos cambian de idioma; el resto de las pantallas sigue en portugués.

## Curva de fadiga da pessoa

fadiga-secao = Curva de fatiga (estudio de tiempos)
fadiga-nota = Solo lectura. Las lecturas y el ajuste se editan en UTSim Estatística o con el comando utsim pessoa ajustar.
fadiga-tarefa = Tarea { $tarefa }: { $leituras } lecturas en { $horarios } horarios
fadiga-tarefa-metodo = Tarea { $tarefa }, método { $metodo }: { $leituras } lecturas en { $horarios } horarios
fadiga-resumo = Descansada: { $t0 }. Con la fatiga máxima, { $acrescimo } % más. Cansancio λ = { $lambda } por hora; recuperación μ = { $mu } por hora ({ $origem }).
fadiga-estimada = estimada
fadiga-padrao = por defecto
fadiga-hora = hora del turno
fadiga-pausa = Pausa
fadiga-curva = Curva ajustada
fadiga-media = Media medida
fadiga-erro = No ajustada: { $erro }
fadiga-abrir = Abrir en UTSim Estatística
fadiga-abrir-ajuda = Abre las lecturas de este estudio de tiempos en una hoja de UTSim Estatística (una fila por tiempo medido: tarea, método, hora del turno y tiempo) para analizarlas. El modelo no cambia.

## Abas dos nós (Qualidade, Segurança, Manutenção) e painéis ao vivo

aba-so-leitura = { $quem }. Solo lectura hasta que esa persona cierre el modelo.
aba-abrir-qualidade = Abrir en UTSim Calidad
aba-abrir-qualidade-dica = Abre este nodo en UTSim Calidad (defectos, inspección, costos, CEP). El modelo se guarda antes y queda con la otra aplicación para editar; UTSim 3D vuelve a leer lo que ella guarde.
aba-abrir-seguranca = Abrir en UTSim Seguridad
aba-abrir-seguranca-dica = Abre este nodo en UTSim Seguridad en el Trabajo (peligros, matriz de riesgo, EPP). El modelo se guarda antes y queda con la otra aplicación para editar; UTSim 3D vuelve a leer lo que ella guarde.
aba-calculando = Calculando los números de la simulación...
aba-nenhuma = ninguna
aba-nenhum-escolha = (ninguno)
aba-nenhum = ninguno
aba-opcoes = Separe con comas. Opciones: { $opcoes }.
aba-vazio-nao-usa = vacío: no se usa
aba-adicionar = + Agregar
aba-adicionar-dica = Agrega un ítem a { $lista }
aba-abrir-item = Abre el ítem
aba-fechar-item = Cierra el ítem
aba-sem-nome = (sin nombre)
aba-remover = Quitar
aba-remover-dica = Quita este ítem de la lista
vivo-fechar = Cerrar
vivo-fechar-dica = Cerrar el panel (Esc cierra el más reciente)
vivo-fixar = Fijar
vivo-fixado = Fijado
vivo-fixar-dica = Fijar mantiene este panel; el próximo clic en un objeto abre otro al lado
vivo-sem-numeros = Sin números para este objeto.
vivo-eixo-tempo = tiempo
vivo-eixo-itens = ítems
vivo-sem-dados = Sin datos todavía.
vivo-estados = Estados en la ventana
vivo-sem-estados = sin estados

## Painel ao vivo: linhas (rótulo e ajuda)

vivo-em-processamento-agora = En proceso ahora
vivo-itens-no-objeto-neste-instante = Ítems en el objeto en este instante (procesando o listos sin poder salir).
vivo-itens-em-espera-fila = Ítems en espera (cola)
vivo-itens-nas-filas-que-alimentam = Ítems en las colas que alimentan el objeto, en este instante.
vivo-entradas = Entradas
vivo-itens-que-entraram-na-janela = Ítems que entraron en la ventana.
vivo-ja-processados = Ya procesados
vivo-itens-que-sairam-do-objeto = Ítems que salieron del objeto en la ventana.
vivo-rejeitados-para-descarte = Rechazados para descarte
vivo-itens-que-chegaram-a-uma = Ítems que llegaron a una salida de descarte (sink_role: Scrap) desde aquí, también por colas o transporte.
vivo-nenhuma-saida-tem-sink-role = Ninguna salida tiene sink_role: Scrap; marque la salida de descarte en el modelo para contar los descartes.
vivo-rejeitados-para-retrabalho = Rechazados para retrabajo
vivo-itens-que-chegaram-a-um = Ítems que llegaron a un puesto de retrabajo desde aquí, también por colas o transporte.
vivo-ciclo-medio-permanencia = Ciclo medio (permanencia)
vivo-tempo-medio-no-objeto-dos = Tiempo medio en el objeto de los ítems que salieron en la ventana.
vivo-espera-media-na-fila = Espera media en la cola
vivo-tempo-medio-nas-filas-que = Tiempo medio en las colas que alimentan el objeto.
vivo-ocupado-processando = Ocupado (procesando)
vivo-fracao-da-janela-no-estado = Fracción de la ventana en el estado procesando.
vivo-ocioso = Ocioso
vivo-fracao-da-janela-sem-trabalho = Fracción de la ventana sin trabajo.
vivo-bloqueado = Bloqueado
vivo-pronto-sem-poder-sair-bloqueado = Listo sin poder salir (bloqueado o esperando transporte).
vivo-parado-quebra = Parado (avería)
vivo-quebrado-aguardando-reparo-ou-em = Averiado, esperando reparación o en mantenimiento.
vivo-setup = Preparación
vivo-fracao-em-setup = Fracción en preparación.
vivo-conteudo-agora = Contenido ahora
vivo-itens-no-objeto-neste-instante-2 = Ítems en el objeto en este instante.
vivo-conteudo-medio = Contenido medio
vivo-media-no-tempo-do-conteudo = Media en el tiempo del contenido, en la ventana.
vivo-conteudo-maximo = Contenido máximo
vivo-maior-conteudo-da-janela = Mayor contenido de la ventana.
vivo-espera-media = Espera media
vivo-saidas = Salidas
vivo-itens-que-sairam-na-janela = Ítems que salieron en la ventana.
vivo-fracao-da-janela-sem-poder = Fracción de la ventana sin poder liberar el ítem creado.
vivo-recebidos-total = Recibidos (total)
vivo-itens-que-chegaram-a-saida = Ítems que llegaron a la salida en la ventana.
vivo-vazao-por-unidade-de-tempo = Rendimiento (por unidad de tiempo)
vivo-recebidos-divididos-pela-duracao-da = Recibidos divididos por la duración de la ventana.
vivo-tempo-medio-no-sistema = Tiempo medio en el sistema
vivo-da-criacao-do-item-a = Desde la creación del ítem hasta su llegada aquí.
vivo-papel-na-qualidade = Papel en la calidad
vivo-o-papel-da-saida-nos = El papel de la salida en los informes de calidad.
vivo-utilizacao = Utilización
vivo-andando-carregando-descarregando-ou-trabalhando = Andando, cargando, descargando o trabajando en un puesto.
vivo-viagens = Viajes
vivo-deslocamentos-comecados-na-janela = Desplazamientos iniciados en la ventana.
vivo-itens-transportados = Ítems transportados
vivo-itens-entregues-na-janela = Ítems entregados en la ventana.
vivo-carregando-agora = Llevando ahora
vivo-itens-consigo-neste-instante = Ítems que lleva en este instante.
vivo-viajando-carregado = Viajando cargado
vivo-fracao-da-janela-andando-com = Fracción de la ventana andando con carga.
vivo-viajando-vazio = Viajando vacío
vivo-fracao-da-janela-andando-sem = Fracción de la ventana andando sin carga.
vivo-trabalhando-num-posto = Trabajando en un puesto
vivo-fracao-da-janela-operando-um = Fracción de la ventana operando un proceso (o un puente grúa por botonera o por radio).
vivo-descanso-fadiga = Descanso (fatiga)
vivo-fracao-da-janela-em-pausa = Fracción de la ventana en pausa para recuperarse.
vivo-pausas = Pausas
vivo-vezes-que-entrou-em-descanso = Veces que entró en descanso en la ventana.
vivo-fadiga-media-maxima-fim-da = Fatiga media / máxima (fin de la replicación)
vivo-a-fadiga-nao-esta-no = La fatiga no está en el registro de la reproducción: valores del informe final.
vivo-esperando-o-operador = Esperando al operador
vivo-fracao-da-janela-parada-na = Fracción de la ventana detenida en la carga hasta que la persona de la botonera o la radio llegue a su lado.

## Painel ao vivo: cabeçalho e seções

godot-ao-vivo-pessoa = persona
godot-ao-vivo-ponte_rolante = puente grúa
godot-ao-vivo-empilhadeira = montacargas
godot-ao-vivo-veiculo = vehículo
godot-ao-vivo-aquecimento_ate_numeros_desde_0 = calentamiento hasta { $inicio_estatisticas }: números desde 0
godot-ao-vivo-desde_o_fim_do_aquecimento = desde el fin del calentamiento ({ $inicio_estatisticas })
godot-ao-vivo-desde_o_inicio = desde el inicio
godot-ao-vivo-t_de_replicacao = t = { $t } de { $fim } · replicación { $replicacao } · { $janela }
godot-ao-vivo-agora = Ahora
godot-ao-vivo-marque_a_saida_de_refugo = marque la salida de descarte
godot-ao-vivo-acumulado = Acumulado
godot-ao-vivo-tempos = Tiempos
godot-ao-vivo-em_processo_wip = En proceso (WIP)
godot-ao-vivo-agora_e_na_janela = Ahora y en la ventana
godot-ao-vivo-conteudo = Contenido
godot-ao-vivo-saida = Salida
godot-ao-vivo-recebidos_acumulado = Recibidos (acumulado)
godot-ao-vivo-pessoa-2 = Persona
godot-ao-vivo-ponte_rolante-2 = Puente grúa
godot-ao-vivo-veiculo-2 = Vehículo
godot-ao-vivo-itens_consigo = Ítems que lleva
godot-ao-vivo-fila = Cola

## Fluxos de tokens

godot-ui-flows-fluxos_de_tokens = Flujos de tokens
godot-ui-flows-fechar = Cerrar
godot-ui-flows-fechar_os_fluxos = Cerrar los flujos
godot-ui-flows-o_modelo_nao_tem_fluxos = El modelo no tiene flujos de tokens.
godot-ui-flows-tokens_em_cada_atividade_no = Tokens en cada actividad en el instante de la reproducción; abajo, los conteos a lo largo de la ejecución (la línea vertical t es el instante actual).
godot-ui-flows-zoom_ctrl_roda_no_grafo = Zoom (Ctrl + rueda en el grafo)
godot-ui-flows-diminui = Reduce
godot-ui-flows-aumenta = Aumenta
godot-ui-flows-ajustar = Ajustar
godot-ui-flows-cabe_na_largura = Cabe en el ancho
godot-ui-flows-sem_contagens = sin conteos

## Resultados

godot-ui-results-resumo = Resumen
godot-ui-results-graficos = Gráficos
godot-ui-results-analise = Análisis
godot-ui-results-cenarios = Escenarios
godot-ui-results-otimizacao = Optimización
godot-ui-results-ocupacao = Ocupación
godot-ui-results-estados = Estados
godot-ui-results-serie_temporal = Serie temporal
godot-ui-results-histograma = Histograma
godot-ui-results-nenhum_recurso_ocupado = ningún recurso ocupado
godot-ui-results-invalido_use_um_numero_inteiro = { $what } "{ $text }" inválido: use un número entero
godot-ui-results-serie_invalida_use_objeto_medida = serie "{ $other }" inválida: use objeto.medida
godot-ui-results-calcule_antes_quantas_replicacoes_faltam = calcule antes cuántas replicaciones faltan
godot-ui-results-simule_o_modelo_antes_de = Simule el modelo antes de comparar escenarios.
godot-ui-results-simule_o_modelo_antes_de-2 = Simule el modelo antes de optimizar.
godot-ui-results-trabalho_desconhecido = trabajo desconocido: { $other }
godot-ui-results-serie_curta_demais_para_o = Serie demasiado corta para el MSER-5 ({ $points } puntos): aumente el tiempo de parada o el muestreo.
godot-ui-results-corte_sugerido_mas_o_minimo = Corte sugerido: { $cut }, pero el mínimo quedó en la segunda mitad de la serie: la ejecución es corta o no se estabiliza.
godot-ui-results-corte_sugerido_mser_5_antes = Corte sugerido (MSER-5): { $cut }. Antes de él la media aún sube o baja.
godot-ui-results-gargalo_movel_na_maior_parte = Cuello de botella móvil: { $n } la mayor parte del tiempo.
godot-ui-results-nenhum_recurso_ativo = Ningún recurso activo.
godot-ui-results-replicacoes_faltam_para_o_alvo = { $reports } replicaciones; faltan { $more } para el objetivo.
godot-ui-results-aquecimento_analisado = Calentamiento analizado: { $d }
godot-ui-results-lei_de_little_verde_s = Ley de Little: { $verde } verde(s), { $amarelo } amarillo(s), { $vermelho } rojo(s).
godot-ui-results-cenario_s_comparados = { $n ->
    [one] { $n } escenario comparado.
   *[other] { $n } escenarios comparados.
}
godot-ui-results-otimizacao_melhor_ponto = Optimización: mejor punto { $ponto }.
godot-ui-results-conteudo_de = contenido de { $object }
godot-ui-results-mser_5 = MSER-5: { $cut }
godot-ui-results-media_entre_replicacoes = media entre replicaciones
godot-ui-results-unico_compartilhado = { $name } (único { $sole }%, compartido { $shifting }%)
godot-ui-results-melhor_objetivo = mejor objetivo
godot-ui-results-pontos_avaliados = puntos evaluados
godot-ui-results-resultados = Resultados
godot-ui-results-fechar = Cerrar
godot-ui-results-fechar_os_resultados_a_cena = Cerrar los resultados (la escena sigue)
godot-ui-results-gargalo_no_turno = cuello de botella en el turno
godot-ui-results-objeto = Objeto
godot-ui-results-tipo = Tipo
godot-ui-results-entradas = Entradas
godot-ui-results-saidas = Salidas
godot-ui-results-conteudo_medio = Contenido medio
godot-ui-results-permanencia = Permanencia
godot-ui-results-ocupacao-2 = ocupación { $valor }
godot-ui-results-replicacoes = Replicaciones
godot-ui-results-semente = semilla { $seed }
godot-ui-results-tempo_de_parada = Tiempo de parada
godot-ui-results-aquecimento = calentamiento { $warmup }
godot-ui-results-gargalo = Cuello de botella
godot-ui-results-gargalo_no_turno-2 = Cuello de botella en el turno
godot-ui-results-saidas_por_replicacao = Salidas por replicación
godot-ui-results-soma_das_saidas_do_modelo = suma de las salidas del modelo
godot-ui-results-o_gargalo_e_o_recurso = El cuello de botella es el recurso más ocupado; dentro del turno cuenta solo el tiempo en que trabaja (un puesto parado medio día parece ocioso en general, pero puede estar saturado en el turno).
godot-ui-results-por_objeto = Por objeto
godot-ui-results-media_semiamplitude_do_ic_de = Media ± semiamplitud del IC del 95 % entre replicaciones.
godot-ui-results-exportar_csv_para = Exportar CSV a
godot-ui-results-exportar = Exportar
godot-ui-results-grava_resumo_replicacoes_series_e = Guarda resumen, replicaciones, series e histogramas en CSV
godot-ui-results-grafico = Gráfico
godot-ui-results-fracao_do_tempo_trabalhando_turno = Fracción del tiempo trabajando (turno: solo el tiempo dentro del turno; bloqueo: sumando el tiempo bloqueado). La barra amarilla es el cuello de botella; el trazo es el IC del 95 %.
godot-ui-results-fracao_do_tempo_em_cada = Fracción del tiempo en cada estado, con los colores de la escena 3D.
godot-ui-results-conteudo_de_no_tempo_cada = Contenido de { $object } en el tiempo: cada replicación en gris y la media en amarillo.
godot-ui-results-histograma_de_juntando_todas_as = Histograma de { $object }.{ $tally }, juntando todas las replicaciones.
godot-ui-results-o_modelo_nao_grava_ligar = {" "}El modelo no guarda { $falta }: "Activar y ejecutar de nuevo" lo activa y simula otra vez.
godot-ui-results-falta-serie = la serie del contenido
godot-ui-results-falta-histogramas = los histogramas
godot-ui-results-ligar_e_rodar_de_novo = Activar y ejecutar de nuevo
godot-ui-results-liga_no_modelo_a_serie = Activa en el modelo la serie del contenido (run.sample_interval, según el tiempo de simulación) y los histogramas (run.histogram_bins = 20), en un paso que se deshace con Ctrl+Z, y ejecuta las replicaciones de nuevo
godot-ui-results-sem_serie_ligue_run_sample = sin serie: active run.sample_interval en el modelo
godot-ui-results-sem_histograma_ligue_run_histogram = sin histograma: active run.histogram_bins en el modelo
godot-ui-results-sem_dados = sin datos
godot-ui-results-ic_95 = IC 95 %
godot-ui-results-a = { $lo } a { $hi }: { $c }
godot-ui-results-aquecimento-2 = Calentamiento
godot-ui-results-roda_as_replicacoes_desde_o = Ejecuta las replicaciones desde el tiempo 0, sin calentamiento, y muestra la media entre ellas (curva de Welch). El corte del MSER-5 es donde la serie deja de tener el comienzo vacío; "Aplicar" guarda ese tiempo en run.warmup.
godot-ui-results-serie = Serie
godot-ui-results-wip_ou_objeto_medida = wip u objeto.medida
godot-ui-results-janela = Ventana
godot-ui-results-campo-janela = ventana
godot-ui-results-campo-replicacoes = replicaciones
godot-ui-results-campo-orcamento = presupuesto
godot-ui-results-analisar = Analizar
godot-ui-results-roda_e_calcula_welch_e = Ejecuta y calcula Welch y MSER-5
godot-ui-results-aplicar = Aplicar
godot-ui-results-grava_o_corte_sugerido_em = Guarda el corte sugerido en run.warmup del modelo
godot-ui-results-sem_serie = sin serie
godot-ui-results-replicacoes_necessarias = Replicaciones necesarias
godot-ui-results-quantas_replicacoes_faltam_para_a = Cuántas replicaciones faltan para que la semiamplitud del IC del 95 % quede por debajo del objetivo (5% es relativo a la media; 0.5 es absoluto). "Ejecutar más" agrega las que faltan a las ya hechas, con semillas nuevas.
godot-ui-results-precisao = Precisión
godot-ui-results-calcular = Calcular
godot-ui-results-calcula_as_replicacoes_necessarias = Calcula las replicaciones necesarias
godot-ui-results-rodar_mais = Ejecutar { $more } más
godot-ui-results-rodar_mais-2 = Ejecutar más
godot-ui-results-roda_as_replicacoes_que_faltam = Ejecuta las replicaciones que faltan y las junta a las actuales
godot-ui-results-para_todas_as_medidas_chegarem = Para que todas las medidas lleguen al objetivo faltan { $more ->
    [one] { $more } replicación.
   *[other] { $more } replicaciones.
}
godot-ui-results-todas_as_medidas_ja_estao = Todas las medidas ya están dentro del objetivo.
godot-ui-results-consistencia_lei_de_little = Consistencia: ley de Little
godot-ui-results-confere_l_w_em_cada = Verifica L = λ·W en cada objeto y en el sistema. Verde: cierra; amarillo: la diferencia es estadísticamente distinta de cero, pero pequeña (hasta 2 %); rojo: no cierra, lo que suele indicar calentamiento corto o ítems atascados al final.
godot-ui-results-conferir = Verificar
godot-ui-results-roda_as_replicacoes_e_confere = Ejecuta las replicaciones y verifica la ley de Little
godot-ui-results-gargalo_movel = Cuello de botella móvil
godot-ui-results-em_cada_instante_o_gargalo = En cada instante, el cuello de botella es el recurso en el período activo más largo; cuando dos períodos se superponen, el cuello de botella es compartido. Muestra la fracción del tiempo de cada recurso como cuello de botella único y compartido.
godot-ui-results-roda_as_replicacoes_com_o = Ejecuta las replicaciones con el rastro y calcula el cuello de botella móvil
godot-ui-results-medida = Medida
godot-ui-results-atual = Actual
godot-ui-results-alvo = Objetivo (±)
godot-ui-results-faltam = Faltan
godot-ui-results-diferenca = Diferencia
godot-ui-results-semaforo = Semáforo
godot-ui-results-um_cenario_por_bloco_nome = Un escenario por bloque: [nombre] y, abajo, los cambios (objeto.campo = valor en RON, o run.campo). El modelo tal como está entra primero, como base. Todos usan las mismas semillas (números aleatorios comunes), así que la diferencia con la base es pareada; en amarillo, las diferencias significativas.
godot-ui-results-medidas_virgulas_vazio_todas = Medidas (comas; vacío: todas)
godot-ui-results-comparar = Comparar
godot-ui-results-roda_todos_os_cenarios_e = Ejecuta todos los escenarios y compara
godot-ui-results-texto = Δ { $valor }
godot-ui-results-procura_os_valores_das_variaveis = Busca los valores de las variables que minimizan (o maximizan) la media de una métrica. Una variable por línea: clave mínimo máximo [paso] (enteros) o con punto decimal (continua, cinco valores en la rejilla). Los puntos no significativamente distintos del mejor (prueba pareada del 95 %) quedan marcados.
godot-ui-results-objetivo = Objetivo
godot-ui-results-maximizar = maximizar
godot-ui-results-metodo = Método
godot-ui-results-replicacoes_por_ponto = Replicaciones por punto
godot-ui-results-orcamento_pontos = Presupuesto (puntos)
godot-ui-results-otimizar = Optimizar
godot-ui-results-roda_a_busca = Ejecuta la búsqueda
godot-ui-results-melhor_ponto_ponto_s_simulados = Mejor punto: { $ponto } ({ $simulated ->
    [one] { $simulated } punto simulado
   *[other] { $simulated } puntos simulados
})
godot-ui-results-sem_historico = sin historial
godot-ui-results-ponto = Punto
godot-ui-results-nenhum = ninguno
godot-ui-results-nenhum-viavel = ninguno factible
godot-ui-results-util-trabalho = trabajo
godot-ui-results-util-turno = turno
godot-ui-results-util-bloqueio = bloqueo
godot-ui-results-metodo-grade = rejilla
godot-ui-results-metodo-recozimento = recocido
godot-ui-results-metodo-genetico = genético
godot-ui-results-replicacao_oes_tempo_de_parada = 
    { $model }: { $replications ->
        [one] { $replications } replicación
       *[other] { $replications } replicaciones
    }, tiempo de parada { $stop_time }, calentamiento { $warmup }, semilla { $seed }.
    Cuello de botella (general): { $b }
    Cuello de botella (dentro del turno): { $bs }
godot-ui-results-replicacao_oes = { $model } · { $reports ->
    [one] { $reports } replicación
   *[other] { $reports } replicaciones
}

## Tipos de bloco nos relatórios (results.rs)

godot-tipo-source = llegada
godot-tipo-queue = cola
godot-tipo-processor = proceso
godot-tipo-combiner = montaje
godot-tipo-separator = desmontaje
godot-tipo-sink = salida
godot-tipo-transporter = transportador
godot-tipo-dispatcher = despachador
godot-tipo-conveyor = cinta transportadora
godot-tipo-rack = estantería de palés
godot-tipo-span = medición de tiempo
godot-tipo-flow = flujo
godot-tipo-fluid = fluidos

## Propriedades dos objetos (schema.rs)

godot-campo-movimentacao = Movimiento
godot-campo-folga-dos-obstaculos = Holgura de los obstáculos
godot-campo-ponto-de-acesso-proprio = Punto de acceso propio
godot-campo-x-no-objeto = X en el objeto
godot-campo-y-no-objeto = Y en el objeto
godot-campo-dimensoes = Dimensiones
godot-campo-dimensoes-proprias = Dimensiones propias
godot-campo-capacidade-pela-area = Capacidad por el área
godot-campo-area-por-item = Área por ítem
godot-campo-cabem = Caben
godot-campo-tempo-entre-chegadas = Tiempo entre llegadas
godot-campo-primeira-chegada-em-t-0 = Primera llegada en t = 0
godot-campo-maximo-de-chegadas = Máximo de llegadas
godot-campo-se-a-saida-estiver-bloqueada = Si la salida está bloqueada
godot-opcao-esperar = Esperar
godot-opcao-descartar-perda = Descartar (pérdida)
godot-campo-rotulos = Etiquetas
godot-campo-chegadas-por-periodo = Llegadas por período
godot-campo-tabela-de-chegadas = Tabla de llegadas
godot-campo-capacidade-limitada = Capacidad limitada
godot-campo-politica-de-acesso = Política de acceso
godot-opcao-fifo-primeiro-a-entrar-primeiro = FIFO (primero en entrar, primero en salir)
godot-opcao-lifo-ultimo-a-entrar-primeiro = LIFO (último en entrar, primero en salir)
godot-opcao-prioridade-maior-rotulo-primeiro = Prioridad: mayor etiqueta primero
godot-opcao-prioridade-menor-rotulo-primeiro-prazo = Prioridad: menor etiqueta primero (plazo)
godot-campo-rotulo-da-prioridade = Etiqueta de la prioridad
godot-campo-lote = Lote
godot-campo-itens-por-lote = Ítems por lote
godot-campo-espera-maxima = Espera máxima
godot-campo-fecha-a-entrada-ate-o = Cierra la entrada hasta que el lote salga
godot-campo-capacidade = Capacidad
godot-campo-preparacao-setup = Preparación (setup)
godot-campo-troca-de-produto = Cambio de producto
godot-campo-tempo-de-processamento = Tiempo de procesamiento
godot-campo-tempo-por-receita = Tiempo por receta
godot-campo-operadores = Operadores
godot-campo-quem-opera = Quién opera
godot-campo-quantos-por-peca = Cuántos por pieza
godot-campo-ganho-da-equipe = Ganancia del equipo
godot-campo-durante-a-preparacao = Durante la preparación
godot-campo-durante-o-processamento = Durante el procesamiento
godot-campo-outras-equipes-ao-mesmo-tempo = Otros equipos al mismo tiempo
godot-campo-componentes-por-entrada = Componentes por entrada
godot-campo-modo = Modo
godot-opcao-embalar-so-o-conteiner-sai = Embalar (solo sale el contenedor)
godot-opcao-juntar-componentes-destruidos = Unir (componentes destruidos)
godot-opcao-lote-todos-saem = Lote (salen todos)
godot-opcao-desembalar = Desembalar
godot-opcao-dividir-em-copias = Dividir en copias
godot-opcao-dividir-pelo-rotulo = Dividir por la etiqueta
godot-campo-copias = Copias
godot-campo-sem-parametros = Sin parámetros
godot-campo-velocidade-maxima = Velocidad máxima
godot-campo-tempo-de-carga = Tiempo de carga
godot-campo-tempo-de-descarga = Tiempo de descarga
godot-campo-voltar-a-base-quando-ocioso = Volver a la base cuando está ocioso
godot-campo-agv-controle-de-trafego-na = AGV (control de tráfico en la red)
godot-campo-trajeto-desenhado = Trayecto dibujado
godot-campo-editar-nos = Editar nodos
godot-campo-comprimento = Longitud
godot-campo-velocidade = Velocidad
godot-campo-comprimento-por-item = Longitud por ítem
godot-campo-acumula-itens-quando-a-saida = Acumula ítems cuando la salida se bloquea
godot-campo-colunas = Columnas
godot-campo-niveis = Niveles
godot-campo-permanencia-minima = Permanencia mínima
godot-campo-regra-de-ocupacao = Regla de ocupación
godot-opcao-primeira-livre = Primera libre
godot-opcao-aleatoria = Aleatoria
godot-opcao-curva-abc-rotulo-classe = Curva ABC (etiqueta clase)
godot-campo-fracao-das-posicoes-da-classe = Fracción de las posiciones de la clase A
godot-campo-fracao-das-posicoes-da-classe-2 = Fracción de las posiciones de la clase B
godot-campo-largura-da-coluna = Ancho de la columna
godot-campo-altura-do-nivel = Altura del nivel
godot-campo-equipe = Equipo
godot-campo-regra-de-escolha = Regla de elección
godot-opcao-primeiro-livre = Primero libre
godot-opcao-rodizio = Rotación
godot-opcao-mais-proximo = Más cercano
godot-campo-ordem-dos-movimentos = Orden de los movimientos
godot-opcao-segura-sobe-anda-desce = Segura (sube, avanza, baja)
godot-opcao-um-eixo-de-cada-vez = Un eje a la vez
godot-opcao-os-tres-juntos = Los tres juntos
godot-campo-altura-de-passagem-do-gancho = Altura de paso del gancho
godot-campo-rotulo-do-peso-do-item = Etiqueta del peso del ítem
godot-campo-tempo-de-engate = Tiempo de enganche
godot-campo-tempo-de-desengate = Tiempo de desenganche
godot-campo-voltar-a-posicao-propria-quando = Volver a la posición propia cuando está ociosa
godot-campo-controle = Control
godot-opcao-cabine-operador-nao-modelado = Cabina (operador no modelado)
godot-opcao-botoeira-a-pessoa-anda-com = Botonera (la persona camina con la carga)
godot-opcao-radio-a-pessoa-encontra-a = Radio (la persona encuentra el puente en la carga)
godot-campo-equipe-do-operador = Equipo del operador
godot-campo-meia-largura-da-area-sob = Medio ancho del área bajo la carga
godot-campo-afastar-as-pessoas-do-caminho = Alejar a las personas del camino de la carga
godot-campo-folga-entre-pontes = Holgura entre puentes
godot-campo-caminho = Camino
godot-campo-regra-de-saida = Regla de salida
godot-campo-pesos = Pesos
godot-campo-rotulo-com-o-numero-da = Etiqueta con el número de la puerta
godot-campo-script = Script
godot-campo-nenhum-passagem-direta = Ninguno (paso directo)
godot-campo-transporte-de-saida = Transporte de salida
godot-campo-transferencia-pela-distancia = Transferencia por la distancia
godot-campo-atraso = Retraso
godot-opcao-reta = recta
godot-opcao-pela-rede = por la red
godot-campo-script-ao-entrar-um-item = Script al entrar un ítem
godot-campo-quebras = Averías
godot-campo-tempo-entre-falhas-mtbf = Tiempo entre fallas (MTBF)
godot-campo-tempo-de-reparo-mttr = Tiempo de reparación (MTTR)
godot-campo-turnos = Turnos
godot-campo-origem = Origen
godot-opcao-personalizado = Personalizado
godot-opcao-calendario-do-modelo = Calendario del modelo
godot-campo-periodo = Período
godot-campo-pausas = Pausas
godot-campo-no-fim-do-turno = Al final del turno
godot-campo-calendario = Calendario
godot-campo-ativo = Activo
godot-campo-cor = Color
godot-campo-escala = Escala
godot-campo-modelo-da-peca = Modelo de la pieza
godot-campo-giro-do-desenho = Giro del dibujo
godot-campo-restaurar-padrao = Restaurar predeterminado
godot-campo-nome = Nombre
godot-campo-tipo = Tipo
godot-campo-rotacao = Rotación
godot-campo-sexo = Sexo
godot-opcao-nao-informado = No informado
godot-opcao-feminino = Femenino
godot-opcao-masculino = Masculino
godot-campo-altura = Altura
godot-ajuda-a-altura-do-boneco-no = La altura del muñeco en el 3D. Desactivada, se sortea según el nombre.
godot-campo-massa = Masa
godot-ajuda-com-a-altura-da-a = Junto con la altura, da el ancho del muñeco en el 3D mediante el índice de masa corporal.
godot-campo-idade = Edad
godot-campo-experiencia-na-funcao = Experiencia en el puesto
godot-ajuda-so-registro-a-simulacao-nao = Solo registro: la simulación no lo lee.
godot-campo-carga-maxima-medida = Carga máxima medida
godot-ajuda-uma-carga-acima-dela-da = Una carga por encima de ella genera un aviso en el modelo.
godot-campo-forca-maxima-medida = Fuerza máxima medida
godot-ajuda-empurrar-ou-puxar-so-registro = Empujar o jalar. Solo registro.
godot-campo-tipo-de-espera = Tipo de cola
godot-opcao-fila-simples-sem-lugares = Cola simple (sin lugares)
godot-opcao-mesa = Mesa
godot-opcao-palete = Palé
godot-opcao-estante = Estantería
godot-opcao-chao-area-demarcada = Piso (área demarcada)
godot-campo-camadas = Capas
godot-campo-passo-entre-pecas = Paso entre piezas
godot-campo-lugares = Lugares
godot-campo-opera-as-maquinas = Opera las máquinas
godot-campo-leva-os-itens-de = Lleva los ítems de
godot-campo-nome-do-modelo = Nombre del modelo
godot-campo-descricao = Descripción
godot-campo-tempo-de-simulacao = Tiempo de simulación
godot-campo-aquecimento = Calentamiento
godot-campo-semente = Semilla
godot-campo-replicacoes = Replicaciones
godot-campo-movimentacao-padrao = Movimiento predeterminado
godot-campo-perfil-da-marcacao-do-piso = Perfil de la marcación del piso
godot-opcao-brasil-faixa-amarela = Brasil (franja amarilla)
godot-opcao-branco-redacao-original-da-nr = Blanco (redacción original de la NR-26)
godot-campo-largura-da-faixa = Ancho de la franja
godot-campo-parada-no-cruzamento = Parada en el cruce
godot-campo-cor-do-chao = Color del piso
godot-ajuda-como-o-recurso-anda-o = Cómo se mueve el recurso: el predeterminado del modelo (Simulación), en línea recta como en el formato antiguo, libre por el piso esquivando los obstáculos, por la red de caminos o solo por los pasillos de su clase.
godot-ajuda-onde-os-recursos-moveis-param = Dónde se detienen los recursos móviles para atender este objeto, en metros desde su centro y girando con él. Desactivado, es el lado -y. También se arrastra en la escena, con "Mostrar obstáculos y holguras" activado.
godot-ajuda-comprimento-largura-e-altura-do = Longitud, ancho y altura del objeto en metros, en su sentido. El área en el piso cambia el punto de acceso y, con él, las distancias y los tiempos de los operadores y vehículos; en una cola con área por ítem, también la capacidad. El 3D estira el modelo a esas medidas. Desactivado, vale el área predeterminada del tipo.
godot-ajuda-a-capacidade-da-espera-sai = La capacidad de la cola sale de su área en el piso dividida por el área que ocupa cada ítem (solo ítems enteros). Si también se da una capacidad, vale la menor.
godot-ajuda-intervalo-entre-duas-chegadas-seguidas = Intervalo entre dos llegadas seguidas.
godot-ajuda-valor-sorteado-para-cada-item = Valor sorteado para cada ítem, por nombre: {"{"}"tipo": Discrete([(1.0, 3.0), (2.0, 1.0)]){"}"}.
godot-ajuda-ciclo-e-periodos-com-o = Ciclo y períodos con su propio intervalo, tasa o cerrados: (cycle: 1440.0, periods: [(start: 0.0, end: 480.0, arrivals: Rate(0.5))]).
godot-ajuda-horario-instante-quantidade-tipo-e = Horario (instante, cantidad, tipo y etiquetas de cada fila) o secuencia de filas: (mode: Times, repeat: Cycle(1440.0), rows: [(at: 480.0, count: 12)]).
godot-ajuda-desligada-a-espera-nao-tem = Desactivada, la cola no tiene límite.
godot-ajuda-qual-item-sai-primeiro-pela = Qué ítem sale primero. Por prioridad, sale antes el ítem con el mayor (o el menor) valor de la etiqueta; los empates y los ítems sin la etiqueta salen en orden de llegada.
godot-ajuda-segura-os-itens-e-libera = Retiene los ítems y los libera en grupos.
godot-ajuda-libera-um-lote-incompleto-depois = Libera un lote incompleto después de este tiempo (0 = espera siempre).
godot-ajuda-preparacao-so-quando-o-produto = Preparación solo cuando el producto cambia: la etiqueta del producto y, si se desea, el tiempo de cada cambio (de, a); sin fila en la matriz vale la preparación de arriba. Ejemplo: (label: "produto", matrix: [(from: 1, to: 2, time: Const(30.0))]). None: preparación en cada pieza.
godot-ajuda-tempo-de-processamento-que-depende = Tiempo de procesamiento que depende del ítem: la etiqueta de la receta y el tiempo de cada valor; los ítems sin receta en la tabla usan el tiempo de arriba. Ejemplo: (label: "produto", times: [(value: 1, time: Const(90.0)), (value: 2, time: Const(120.0))]). None: el mismo tiempo para todos.
godot-ajuda-quem-opera-esta-maquina-marque = Quién opera esta máquina: marque y elija abajo, o arrastre desde el nodo naranja del operador (o del equipo) hasta la máquina. El proceso espera a que uno de ellos esté libre.
godot-ajuda-operadores-que-a-peca-precisa = Operadores que la pieza necesita al mismo tiempo: solo comienza cuando llegan todos, y se quedan con ella hasta el final. Con más operadores en el equipo que capacidad × este número, los sobrantes quedan detenidos.
godot-ajuda-quantas-vezes-mais-rapido-a = Cuántas veces más rápido hace el equipo la etapa: el tiempo sorteado se divide por este número. 1: sin ganancia (el tiempo es el mismo con uno o con varios).
godot-ajuda-equipes-que-precisam-estar-juntas = Equipos que deben estar juntos con los operadores (un médico y un enfermero en la misma sala). Ejemplo: [OperatorUse(team: "enfermagem")]. La etapa solo comienza cuando llegan todos.
godot-ajuda-a-entrada-1-traz-o = La entrada 1 trae el contenedor; la entrada k + 1 trae el k-ésimo número de ítems. Ejemplo: [2, 1].
godot-ajuda-desligada-a-velocidade-muda-na = Desactivada, la velocidad cambia al instante.
godot-ajuda-abre-a-ferramenta-de-desenho = Abre la herramienta de dibujo con esta cinta transportadora: arrastre los nodos, cambie la altura y el radio.
godot-ajuda-transportadores-que-o-despachante-comanda = Transportadores que el despachador comanda.
godot-ajuda-segura-o-gancho-sobe-a = Segura: el gancho sube a la altura de paso, el puente y el carro avanzan juntos y el gancho baja.
godot-ajuda-rotulo-com-o-peso-de = Etiqueta con el peso de cada ítem, en kg: los ítems por encima de la capacidad generan un aviso.
godot-ajuda-na-botoeira-a-ponte-nao = Con botonera, el puente no avanza más rápido que la persona; con radio, mantiene su propia velocidad y espera a que la persona llegue.
godot-ajuda-meio-lado-do-quadrado-sob = Medio lado del cuadrado bajo el gancho donde nadie debe estar con la carga suspendida (análisis de exposición).
godot-ajuda-antes-de-andar-carregada-a = Antes de avanzar cargado, el puente espera a quien pasa por la franja de la carga; y quien iba a entrar en ella se detiene en el borde hasta que la carga pase.
godot-ajuda-distancia-mantida-entre-os-ganchos = Distancia mantenida entre los ganchos de dos puentes en el mismo camino: uno no pasa al otro.
godot-ajuda-para-qual-ligacao-vai-cada = A qué conexión va cada ítem, en el orden de las conexiones de salida.
godot-ajuda-peso-de-cada-saida-na = Peso de cada salida, en el orden de las conexiones. Ejemplo: [3.0, 1.0].
godot-ajuda-devolve-o-numero-da-porta = Devuelve el número de la puerta (1, 2, ...).
godot-ajuda-transportador-ou-despachante-que-leva = Transportador o despachador que lleva los ítems al siguiente bloque.
godot-ajuda-os-itens-levam-atraso-distancia = Los ítems tardan retraso + distancia / velocidad en llegar al siguiente bloque por las conexiones directas (sin transportador).
godot-ajuda-roda-para-cada-item-que = Se ejecuta por cada ítem que entra, antes de cualquier otra regla.
godot-ajuda-falhas-aleatorias-com-o-tempo = Fallas aleatorias, con el tiempo contado en el reloj.
godot-ajuda-janelas-fora-do-turno-repetidas = Ventanas fuera del turno, repetidas en cada período.
godot-ajuda-inicio-e-duracao-de-cada = Inicio y duración de cada pausa dentro del período, en orden.
godot-ajuda-corrige-para-onde-o-modelo = Corrige hacia dónde mira el modelo 3D; el recurso gira hacia donde avanza.
godot-ajuda-volta-a-aparencia-padrao-do = Vuelve a la apariencia predeterminada del tipo de objeto.
godot-ajuda-giro-em-graus-anti-horario = Giro en grados, antihorario visto desde arriba (R gira 90°, Ctrl+R 15°).
godot-ajuda-escolhe-o-corpo-no-3d = Elige el cuerpo en el 3D (no informado, el cuerpo sale del nombre). Ningún patrón de fatiga depende de él; con Femenino, una carga por encima de 20 kg genera el aviso de la CLT, art. 390.
godot-ajuda-numa-espera-real-cada-peca = En una cola real cada pieza ocupa un lugar (la disposición del tipo, recalculada por las Dimensiones): solo sale la pieza sin nada encima, la más antigua de ellas primero, y quien la retira va hasta el frente de la pieza. Cambia el resultado. Cola simple: como antes.
godot-ajuda-quantas-pecas-empilhadas-em-cada = Cuántas piezas apiladas en cada lugar (en la estantería, en cada repisa).
godot-ajuda-distancia-entre-os-centros-de = Distancia entre los centros de dos piezas vecinas.
godot-ajuda-arraste-do-no-laranja-ate = Arrastre desde el nodo naranja hasta la máquina para conectar; haga clic en la línea punteada y en la × para quitar.
godot-ajuda-arraste-do-no-laranja-ate-2 = Arrastre desde el nodo naranja hasta el objeto (una cola, una llegada, o con Shift una máquina) para conectar.
godot-ajuda-texto-livre-sobre-o-modelo = Texto libre sobre el modelo, guardado en el archivo.
godot-ajuda-as-estatisticas-so-contam-depois = Las estadísticas solo cuentan después de este tiempo.
godot-ajuda-como-andam-os-recursos-moveis = Cómo se mueven los recursos móviles que no eligieron un modo propio. Vacío, vale Libre (camino más corto en el piso, esquivando los obstáculos).
godot-ajuda-cores-das-faixas-e-setas = Colores de las franjas y flechas de los pasillos. El programa no certifica conformidad: verifique el texto vigente de las normas.
godot-ajuda-a-pratica-usual-vai-de = La práctica habitual va de 0,05 a 0,15 m.
godot-ajuda-o-veiculo-para-esse-tempo = El vehículo se detiene este tiempo antes de un cruce con peatones.
godot-ajuda-a-cor-do-piso-na = El color del piso en la escena 3D, guardado en el modelo; la cuadrícula cambia de tono para seguir visible. Desactivada, el gris de siempre.
godot-opcao-por-rotulo = Por etiqueta
godot-opcao-padrao-do-modelo = Predeterminado del modelo
godot-opcao-legado-linha-reta-ou-rede = Heredado (línea recta o red)
godot-opcao-livre-menor-caminho-no-piso = Libre (camino más corto en el piso)
godot-opcao-pela-rede-de-caminhos = Por la red de caminos
godot-opcao-pelos-corredores = Por los pasillos


## Propriedades dos objetos: seções, tipos, famílias, parâmetros, turnos e visual (schema.rs)

godot-secao-objeto = Objeto
godot-secao-posicao = Posición
godot-secao-parametros = Parámetros
godot-secao-saida = Salida
godot-secao-quebras-e-turnos = Averías y turnos
godot-secao-aparencia = Apariencia
godot-secao-simulacao = Simulación
godot-secao-movimentacao = Movimiento
godot-secao-piso-e-corredores = Piso y pasillos
godot-secao-modelo = Modelo
godot-secao-pessoa = Persona
godot-objeto-chegada = Llegada
godot-objeto-espera = Cola
godot-objeto-processo = Proceso
godot-objeto-montagem = Montaje
godot-objeto-desmontagem = Desmontaje
godot-objeto-saida = Salida
godot-objeto-transportador = Transportador
godot-objeto-ponte-rolante = Puente grúa
godot-objeto-despachante = Despachador
godot-objeto-esteira = Cinta transportadora
godot-objeto-porta-paletes = Estantería de palés
godot-dist-constante = Constante
godot-dist-uniforme = Uniforme
godot-dist-exponencial = Exponencial
godot-dist-triangular = Triangular
godot-dist-normal = Normal
godot-dist-lognormal = Lognormal
godot-dist-gama = Gamma
godot-dist-erlang = Erlang
godot-dist-weibull = Weibull
godot-dist-beta = Beta
godot-dist-log-logistica = Log-logística
godot-dist-pearson-v = Pearson V
godot-dist-pearson-vi = Pearson VI
godot-dist-discreta = Discreta
godot-dist-empirica = Empírica
godot-dist-poisson = Poisson
godot-dist-geometrica = Geométrica
godot-dist-binomial = Binomial
godot-dist-binomial-negativa = Binomial negativa
godot-dist-deslocada = Desplazada
godot-param-valor = valor
godot-param-minimo = mínimo
godot-param-maximo = máximo
godot-param-media = media
godot-param-moda = moda
godot-param-desvio-padrao = desviación estándar
godot-param-forma = forma
godot-param-escala = escala
godot-param-fases = fases (k)
godot-param-forma-1 = forma 1
godot-param-forma-2 = forma 2
godot-param-probabilidade = probabilidad (p)
godot-param-tentativas = intentos (n)
godot-param-sucessos = éxitos (r)
godot-param-alfa = alfa
godot-param-beta = beta
godot-param-deslocamento = desplazamiento
godot-dim-comprimento-do-caminho = Longitud del camino
godot-dim-vao = Vano
godot-dim-altura-do-caminho = Altura del camino
godot-dim-comprimento = Longitud
godot-dim-largura = Ancho
godot-dim-altura = Altura
godot-turno-pausar = Pausar
godot-turno-terminar = Terminar el trabajo
godot-turno-do-calendario = (el del calendario)
godot-ajuda-folga-papel = Distancia mínima a cualquier obstáculo. Desactivada, vale la del rol del recurso ({ $m } m: medio ancho más 0,3 m).
godot-dim-esteira = { $l } × { $w } m: la longitud es la de la cinta transportadora (Parámetros)
godot-dim-porta-paletes = { $l } × { $w } × { $h } m: por los vanos y niveles (Parámetros)
godot-dim-padrao = { $l } × { $w } m (predeterminado)
godot-campo-sem-limite = sin límite
godot-campo-n-itens = { $n ->
    [one] { $n } ítem
   *[other] { $n } ítems
}
godot-campo-nao-existe = { $nome } (no existe)
godot-campo-n-posicoes = { $n ->
    [one] { $n } posición
   *[other] { $n } posiciones
}
godot-campo-movimento = Movimiento { $o_que }.
godot-campo-n-lugares = { $n ->
    [one] { $n } lugar
   *[other] { $n } lugares
}
godot-campo-divide-caminho = Comparte el camino con { $com } (holgura de { $m } m entre los ganchos)
godot-campo-pesos-e-saidas = Hay { $pesos ->
    [one] { $pesos } peso
   *[other] { $pesos } pesos
}, pero el objeto tiene { $n ->
    [one] { $n } salida
   *[other] { $n } salidas
}. { $ajuda }
godot-turno-sem-calendarios = El modelo no tiene calendarios: cree uno en "calendars" para usarlo aquí.
godot-turno-calendario-ajuda = Un calendario es la misma tabla de turnos usada por varios objetos.
godot-visual-padrao-do-tipo = (predeterminado del tipo)
godot-visual-gerado = UTSim (generado por utsim-assets)
godot-visual-grupo-biblioteca = Biblioteca de UTSim
godot-visual-grupo-terceiros = Terceros (assets/terceiros)
godot-visual-grupo-elementos = Elementos 3D (assets/modelos3d)
godot-visual-caixa-padrao = caja (predeterminado)
godot-visual-modelo-da-peca-ajuda = Elemento 3D con el que se dibujan las piezas creadas aquí; la caja es el predeterminado.
godot-visual-no-tamanho-da-peca = { $arquivo } ({ $x } x { $y } x { $z } m), dibujado al tamaño de la pieza
godot-eixo-z = Z (altura)


## ui_props.rs

godot-ui-props-modelo_nada_selecionado = Modelo (nada seleccionado)
godot-ui-props-propriedades = Propiedades: { $name }
godot-ui-props-problemas = Problemas
godot-ui-props-avisos = Avisos
godot-ui-props-mostrar_motorista = Mostrar conductor
godot-ui-props-desenha_o_operador_sentado_e = Dibuja al operador sentado y animado en la montacargas. Solo la apariencia: no cambia la simulación ni se guarda en el modelo.
godot-ui-props-clique_para_destacar_na_cena = Haga clic para resaltar en la escena lo que cita este aviso
godot-ui-props-abre-a-secao = Abre la sección { $title }
godot-ui-props-fecha-a-secao = Cierra la sección { $title }
godot-ui-props-liga_ou_desliga = Activa o desactiva { $campo }
godot-ui-props-aplicar = Aplicar
godot-ui-props-aplicar_script = Aplicar script
godot-ui-props-nenhum_transportador_no_modelo = Ningún transportador en el modelo.
godot-ui-props-este_campo_vem_de_dados = Este campo viene de datos reales: { $label }
godot-ui-props-distribuicao = Distribución
godot-ui-props-dados_reais = Datos reales
godot-ui-props-fonte = Fuente
godot-ui-props-de_onde_vem_a_distribuicao = De dónde viene la distribución de este campo: parámetros informados o valores reales, ajustados automáticamente
godot-ui-props-media = media { $mean }
godot-ui-props-arquivo_csv = Archivo CSV
godot-ui-props-valores_colados = Valores pegados
godot-ui-props-valores_reais = Valores reales…
godot-ui-props-abre_a_janela_grande_para = Abre la ventana grande para escribir, pegar o importar los valores y comparar todas las distribuciones a la vez
godot-ui-props-origem = Origen
godot-ui-props-arquivo = Archivo
godot-ui-props-arquivo_csv_ou_txt = archivo .csv o .txt
godot-ui-props-escolher = Elegir...
godot-ui-props-escolhe_o_arquivo_de_dados = Elige el archivo de datos (CSV o TXT)
godot-ui-props-automatica = (automática)
godot-ui-props-coluna = Columna
godot-ui-props-coluna_do_arquivo_com_os = Columna del archivo con los valores
godot-ui-props-um_valor_por_linha_ou = un valor por línea (o separados por ; o espacio)
godot-ui-props-sao_instantes_usar_os_intervalos = Son instantes (usar los intervalos entre ellos)
godot-ui-props-marque_quando_os_valores_sao = Marque cuando los valores son instantes crecientes (como horas de llegada): el ajuste usa las diferencias entre ellos. Desmarcado, los valores son duraciones.
godot-ui-props-melhor_automatica = Mejor automática
godot-ui-props-empirica_os_proprios_valores = Empírica (los propios valores)
godot-ui-props-so = Solo { $f }
godot-ui-props-escolha = Elección
godot-ui-props-criterio = Criterio
godot-ui-props-o_que_ordena_as_familias = Lo que ordena las familias aceptadas por las pruebas
godot-ui-props-reajustar_com_estas_opcoes = Reajustar con estas opciones
godot-ui-props-ajusta_a_distribuicao_aos_dados = Ajusta la distribución a los datos (en segundo plano) y pasa a usarla en este campo
godot-ui-props-nao_foi_possivel_ler_os = No fue posible leer los datos: { $err }
godot-ui-props-os_dados_mudaram_desde_o = Los datos cambiaron desde el último ajuste.
godot-ui-props-reajustar_aos_dados_atuais = Reajustar a los datos actuales
godot-ui-props-o_campo_foi_reajustado_salve = { $c }. El campo fue reajustado; guarde para registrar.
godot-ui-props-n = { $family } · n = { $n } · { $label }
godot-ui-props-analisando_os_dados = Analizando los datos...
godot-ui-props-candidatas = Candidatas ({ $criterion })
godot-ui-props-aic_bic_usa_esta_familia =
    { $params }
    AIC { $aic }   BIC { $bic }
    Usa esta familia en este campo
godot-ui-props-usar_a_empirica_os_proprios = Usar la empírica (los propios valores)
godot-ui-props-relatorio = Informe
godot-ui-props-grava_o_relatorio_completo_do = Guarda el informe completo del ajuste (HTML) en una carpeta y lo abre en el navegador
godot-ui-props-histograma_dos_dados_e_a = Histograma de los datos y la curva de la distribución elegida
godot-ui-props-buscar_ativo = Buscar activo
godot-ui-props-miniatura_do_objeto_com_o = Miniatura del objeto con el activo elegido (o el que está bajo el mouse en la lista)
godot-ui-props-ligacoes_de_saida_ordem_das = Conexiones de salida (orden de las puertas)
godot-ui-props-nenhuma_use_a_ferramenta_ligar = Ninguna. Use la herramienta Conectar (L).
godot-ui-props-subir = Subir
godot-ui-props-sobe_esta_saida_na_ordem = Sube esta salida en el orden de las puertas
godot-ui-props-descer = Bajar
godot-ui-props-desce_esta_saida_na_ordem = Baja esta salida en el orden de las puertas
godot-ui-props-remover = Quitar
godot-ui-props-remove_esta_ligacao = Quita esta conexión
godot-ui-props-entradas = Entradas: { $inputs }

## Ronda 13: ventanas de UTSim 3D

## dados.rs

godot-dados-criterio_desconhecido = criterio desconocido: { $other }
godot-dados-use_um_arquivo_ou_valores = use un archivo o valores pegados, no ambos
godot-dados-escolha_um_arquivo_de_dados = elija un archivo de datos o pegue los valores
godot-dados-registro_reaproveitado = registro reaprovechado
godot-dados-dados_mudaram = los datos cambiaron
godot-dados-opcoes_mudaram = las opciones cambiaron
godot-dados-dados_ilegiveis = datos ilegibles
godot-dados-os_dados_mudaram_desde_o = los datos cambiaron desde el último ajuste (era { $old_family }, hash { $old_hash })
godot-dados-nao_foi_possivel_ler = no fue posible leer { $path }: { $e }
godot-dados-nenhum_campo_do_modelo_usa = ningún campo del modelo usa datos reales
godot-dados-nao_foi_possivel_gravar_em = no fue posible grabar en { $dir }: { $e }

## document.rs

godot-document-transportadores_e_despachantes_nao_usam = Los transportadores y despachadores no usan conexiones de flujo: arrastre desde su nodo naranja hasta la máquina que operan (o hasta el objeto cuyos ítems llevan).
godot-document-objeto_nao_existe = el objeto '{ $n }' no existe
godot-document-arraste_de_um_objeto_ate = arrastre de un objeto a otro
godot-document-saiu_da_equipe_de = { $to } salió del equipo de { $from }.
godot-document-entrou_na_equipe_de = { $to } entró en el equipo de { $from }.
godot-document-nao_pede_mais_operadores_de = { $from } ya no pide operadores de { $to }.
godot-document-pede_operadores_de = { $from } pide operadores de { $to }.
godot-document-nao_pede_mais_transporte_a = { $from } ya no pide transporte a { $to }.
godot-document-pede_transporte_a = { $from } pide transporte a { $to }.
godot-document-arraste_do_despachante_a_um = arrastre del despachador a un transportador, o de un objeto al transportador o despachador que lo atiende
godot-document-a_serie_e_os_histogramas = la serie y los histogramas ya están conectados en el modelo
godot-document-desenhe_clicando_os_nos_no = dibuje haciendo clic en los nodos sobre el piso (Enter termina)
godot-document-tipo_desconhecido = tipo desconocido: { $id }
godot-document-nada_selecionado = nada seleccionado
godot-document-nao_ha_para_onde_mover = no hay hacia dónde mover esta conexión
godot-document-nenhum_arrasto_comecou = no comenzó ningún arrastre
godot-document-objeto_nao_existe-2 = el objeto '{ $name }' no existe
godot-document-e_um_recurso_movel_ele = { $name } es un recurso móvil: gira hacia donde se desplaza
godot-document-recursos_moveis_nao_giram_eles = los recursos móviles no giran: se orientan hacia donde se desplazan
godot-document-a_area_de_transferencia_esta = el portapapeles está vacío (Ctrl+C copia)
godot-document-alinhamento_desconhecido = alineación desconocida: { $how }
godot-document-selecione_dois_objetos_ou_mais = seleccione dos objetos o más para alinear
godot-document-eixo_desconhecido = eje desconocido: { $axis }
godot-document-selecione_tres_objetos_ou_mais = seleccione tres objetos o más para distribuir
godot-document-nao_tem_distribuicao = '{ $field }' no tiene distribución
godot-document-modelo = Modelo
godot-document-nome_em_vez_de = nombre: { $name } en lugar de { $name2 }
godot-document-descricao_em_vez_de = descripción: { $description } en lugar de { $description2 }
godot-document-run_em_vez_de = run: { $run } en lugar de { $run2 }
godot-document-falta_o_objeto = falta el objeto { $name }
godot-document-objeto_em_vez_de = objeto { $name }: { $g } en lugar de { $w }
godot-document-objeto_a_mais = objeto de más: { $name }
godot-document-ligacoes_em_vez_de = conexiones: { $connections } en lugar de { $connections2 }
godot-document-outras_partes_do_modelo_calendarios = otras partes del modelo (calendarios, red, flujos, fluidos) difieren
godot-document-clique = Clic
godot-document-seleciona_o_bloco_ou_a = selecciona el bloque o la conexión
godot-document-clique_de_novo_no_mesmo = Clic de nuevo en el mismo punto
godot-document-passa_para_o_proximo_bloco = pasa al siguiente bloque superpuesto
godot-document-poe_ou_tira_o_bloco = agrega o quita el bloque de la selección
godot-document-shift_arrastar_no_chao = Shift + arrastrar sobre el piso
godot-document-selecao_por_caixa_soma_a = selección por caja (se suma a la selección)
godot-document-arrastar_no_chao = Arrastrar sobre el piso
godot-document-selecao_por_caixa_troca_a = selección por caja (reemplaza la selección)
godot-document-arrastar_um_bloco = Arrastrar un bloque
godot-document-move_o_bloco_ou_a = mueve el bloque o toda la selección
godot-document-arrastar_da_porta_de_saida = Arrastrar desde el puerto de salida (verde)
godot-document-liga_ao_bloco_ou_a = conecta con el bloque (o con el puerto de entrada) donde se suelte
godot-document-botao_direito_arrastar = Botón derecho + arrastrar
godot-document-gira_a_vista = gira la vista
godot-document-shift_botao_direito_arrastar = Shift + botón derecho + arrastrar
godot-document-desloca_a_vista = desplaza la vista
godot-document-roda_do_mouse = Rueda del mouse
godot-document-aproxima_e_afasta = acerca y aleja
godot-document-biblioteca_clique_e_clique_no = Biblioteca: clic y clic en el piso
godot-document-coloca_um_objeto_shift_mantem = coloca un objeto (Shift mantiene la herramienta)
godot-document-biblioteca_arrastar_ate_o_chao = Biblioteca: arrastrar hasta el piso
godot-document-coloca_um_objeto_onde_soltar = coloca un objeto donde se suelte
godot-document-setas = Flechas
godot-document-move_a_selecao_um_passo = mueve la selección un paso de la cuadrícula
godot-document-girar_90_anti_horario_visto = girar 90° (antihorario visto desde arriba)
godot-document-girar_90_horario = girar −90° (horario)
godot-document-girar_15_15 = girar 15° / −15°
godot-document-arrastar_a_alca_de_giro = Arrastrar el asa de giro (morada)
godot-document-gira_o_objeto_selecionado_em = gira el objeto seleccionado en pasos de 15° (con Shift, 90°)
godot-document-exclui_a_selecao_ou_a = elimina la selección (o la conexión seleccionada)
godot-document-ferramenta_ligar_arraste_da_origem = herramienta Conectar: arrastre del origen al destino, o clic y clic
godot-document-ferramenta_corredor_clique_pontos_no = herramienta Pasillo: haga clic en puntos del piso, Enter o doble clic termina
godot-document-ferramenta_zona_zonas_de_risco = herramienta Zona: zonas de riesgo en el piso; haga clic en las esquinas, Enter o el primer punto cierra
godot-document-e_ou_alt_com_ligar = E (o Alt con Conectar)
godot-document-ferramenta_equipe_arraste_do_despachante = herramienta Equipo: arrastre del despachador al transportador, o del objeto a quien lleva los ítems (Shift: operadores del proceso)
godot-document-cancela_a_ferramenta_a_ligacao = cancela la herramienta, la conexión o el pasillo en curso, o la selección
godot-document-simular_clique_num_objeto = Simular: clic en un objeto
godot-document-abre_o_painel_de_resultados = abre el panel de resultados en vivo del objeto en la esquina de la vista
godot-document-simular_esc = Simular: Esc
godot-document-fecha_o_painel_ao_vivo = cierra el panel en vivo más reciente (primero el no fijado)
godot-document-f_editar_ou_home = F (Editar) o Home
godot-document-enquadra_o_modelo = encuadra el modelo
godot-document-simular_f = Simular: F
godot-document-anda_pela_fabrica_em_primeira = recorre la fábrica en primera persona: WASD camina, el mouse mira, Shift corre, E/Q sube y baja, V vuela, la rueda cambia la velocidad, el clic en la mira abre el panel, Esc sale
godot-document-liga_ou_desliga_o_encaixe = activa o desactiva el ajuste a la cuadrícula
godot-document-alterna_entre_editar_e_simular = alterna entre Editar y Simular
godot-document-mostra_ou_esconde_esta_janela = muestra u oculta esta ventana
godot-document-seleciona_todos_os_objetos = selecciona todos los objetos
godot-document-copia_a_selecao = copia la selección
godot-document-recorta_a_selecao = corta la selección
godot-document-cola_nomes_unicos_ligacoes_internas = pega (nombres únicos, conexiones internas preservadas)
godot-document-duplica_a_selecao = duplica la selección
godot-document-salva_como = guarda como
godot-document-abre_um_modelo = abre un modelo
godot-document-novo_modelo = nuevo modelo

## editor3d.rs

godot-editor3d-opera = { $to } opera { $from }
godot-editor3d-leva_os_itens_de = { $to } lleva los ítems de { $from }
godot-editor3d-esta_na_equipe_de = { $to } está en el equipo de { $from }
godot-editor3d-ligacao_de = Conexión de { $k }: { $from } → { $to }
godot-editor3d-arraste_do_no_laranja_dele = { $name }: arrastre desde su nodo naranja hasta una máquina para que la opere (o hasta un objeto, para llevar sus ítems).
godot-editor3d-de_clique_no_no_de = {" "}({ $next } de { $links }: haga clic en el nodo de nuevo para ir al siguiente)
godot-editor3d-ligacao_selecionada_clique_no_ou = Conexión { $a } → { $b } seleccionada{ $extra }. Clic en la × o Del la elimina.
godot-editor3d-esta_ligacao_se_muda_no = Esta conexión ({ $kind }) se cambia en el panel de propiedades de { $from }.
godot-editor3d-ligacao_removida_ctrl_z_desfaz = Conexión { $a } → { $b } eliminada (Ctrl+Z deshace).
godot-editor3d-nenhuma_zona_selecionada_para_excluir = Ninguna zona seleccionada para eliminar.
godot-editor3d-nada_selecionado_para_excluir = Nada seleccionado para eliminar.
godot-editor3d-zona_cancelada = Zona cancelada.
godot-editor3d-ligacao_cancelada = Conexión cancelada.
godot-editor3d-girar_arraste_em_volta_do = Girar: arrastre alrededor del objeto (pasos de 15°; con Shift, 90°).
godot-editor3d-solte_sobre_a_maquina_que = Suelte sobre la máquina que { $res } va a operar (o sobre un objeto, para llevar sus ítems). Esc cancela.
godot-editor3d-ligar_solte_sobre_o_destino = Conectar: suelte sobre el destino (o haga clic en él). Esc cancela.
godot-editor3d-ligar_solte_sobre_a_origem = Conectar: suelte sobre el origen (o sobre su nodo de salida). Esc cancela.
godot-editor3d-ligar_clique_em_um_bloco = Conectar: haga clic en un bloque.
godot-editor3d-ligado_clique_em_outra_origem = Conectado: { $from } → { $to }. Haga clic en otro origen o Esc.
godot-editor3d-ligacao_selecionada_clique_no_ou-2 = Conexión { $a } → { $b } seleccionada. Clic en la × o Del la elimina.
godot-editor3d-clique_no_ou_del_remove = { $said }. Clic en la × o Del la elimina.
godot-editor3d-de_blocos_neste_ponto_clique = { $name } ({ $index } de { $hits } bloques en este punto: haga clic de nuevo para el siguiente).
godot-editor3d-arraste_deste_no_verde_ate = { $from }: arrastre desde este nodo verde hasta el destino para conectar.
godot-editor3d-ligar_clique_no_destino_esc = Conectar: { $from } → haga clic en el destino (Esc cancela).
godot-editor3d-ligado = Conectado: { $from } → { $to }.
godot-editor3d-ligacao_cancelada_solte_sobre_o = Conexión cancelada: suelte sobre el bloque de destino.
godot-editor3d-nao_recebe_de_ninguem_arraste = { $to } no recibe de nadie: arrastre desde el nodo de salida del origen hasta aquí.
godot-editor3d-ligacao_cancelada_solte_sobre_o-2 = Conexión cancelada: suelte sobre el bloque de origen.
godot-editor3d-girado_para = { $name } girado a { $degrees }°.
godot-editor3d-arraste_a_alca_em_volta = { $name }: arrastre el asa alrededor del objeto para girarlo (R gira 90°, Ctrl+R 15°).
godot-editor3d-objeto_s_movidos =
    { $selected } { $selected ->
        [one] objeto movido
       *[other] objetos movidos
    }.
godot-editor3d-objeto_s_na_caixa =
    { $inside } { $inside ->
        [one] objeto en la caja
       *[other] objetos en la caja
    }.
godot-editor3d-nada_mudou_solte_sobre_a = Nada cambió: suelte sobre la máquina (o el objeto) que { $res } va a atender.
godot-editor3d-solte_sobre_uma_maquina_ou = Suelte sobre una máquina o un objeto (un transportador solo entra en el equipo de un despachador).
godot-editor3d-ja_esta_ligado_para_desfazer = Ya está conectado: { $to } → { $res }. Para deshacer, haga clic en la línea y en la ×.
godot-editor3d-ligado_ctrl_z_desfaz = Conectado: { $said }. Ctrl+Z deshace.
godot-editor3d-aponte_para_o_chao_para = Apunte al piso para colocar el objeto.
godot-editor3d-equipe_clique_no_transportador_ou = Equipo: haga clic en el transportador o despachador de destino.
godot-editor3d-equipe_comece_num_despachante_ou = Equipo: comience en un despachador o en un objeto.
godot-editor3d-equipe_clique_no_destino_esc = Equipo: { $from } → haga clic en el destino (Esc cancela).
godot-editor3d-equipe_solte_sobre_o_transportador = Equipo: suelte sobre el transportador o despachador.
godot-editor3d-equipe_solte_sobre_outro_objeto = Equipo: suelte sobre otro objeto.
godot-editor3d-aponte_para_o_chao_para-2 = Apunte al piso para poner un punto del pasillo.
godot-editor3d-corredor_clique_os_proximos_pontos = Pasillo: haga clic en los siguientes puntos; Enter o doble clic termina, Backspace retrocede un punto, Esc cancela.
godot-editor3d-ponto_s_enter_ou_duplo =
    { $aisle_points } { $aisle_points ->
        [one] punto
       *[other] puntos
    }. Enter o doble clic termina.
godot-editor3d-vertice_de_selecionado_arraste_para = Vértice { $k } de { $name } seleccionado (arrastre para mover).
godot-editor3d-corredor_selecionado_del_exclui_shift = Pasillo { $name } seleccionado (Del elimina; Shift + clic en la línea agrega un vértice).
godot-editor3d-um_corredor_precisa_de_ao = Un pasillo necesita al menos 2 puntos.
godot-editor3d-ponto_removido = Punto eliminado ({ $aisle_points }).
godot-editor3d-trecho_selecionado_del_remove = Tramo { $a } → { $b } seleccionado (Del elimina).
godot-editor3d-aponte_para_o_chao_para-3 = Apunte al piso para colocar un nodo.
godot-editor3d-zona_ponto_s_clique_no =
    Zona: { $zone_points } { $zone_points ->
        [one] punto
       *[other] puntos
    }. Haga clic en el primer punto, Enter o doble clic cierra; Backspace retrocede; Esc cancela.
godot-editor3d-zona_arraste_para_mover_arraste = Zona { $name }: arrastre para mover, arrastre un vértice para cambiar la forma; Del elimina.
godot-editor3d-zona_clique_para_por_os = Zona: haga clic para poner los demás puntos; Enter, doble clic o el primer punto cierra.
godot-editor3d-zona_alterada_ctrl_z_desfaz = Zona modificada (Ctrl+Z deshace).
godot-editor3d-uma_zona_precisa_de_pelo = Una zona necesita al menos 3 puntos.

## indicadores.rs

godot-indicadores-itens_refugados = Ítems desechados
godot-indicadores-itens_que_terminaram_neste_refugo = Ítems que terminaron en este desecho, por replicación.
godot-indicadores-defeitos_gerados = Defectos generados
godot-indicadores-defeitos_que_este_posto_gerou = Defectos que este puesto generó, por replicación.
godot-indicadores-defeitos_que_escaparam = Defectos que escaparon
godot-indicadores-defeitos_deste_posto_que_chegaram = Defectos de este puesto que llegaron al cliente (falla externa), por replicación.
godot-indicadores-rendimento_de_primeira = Rendimiento de primera
godot-indicadores-itens_inspecionados_uma_so_vez = Ítems inspeccionados una sola vez y entregados, sobre los inspeccionados.
godot-indicadores-custo_da_qualidade = Costo de la calidad
godot-indicadores-avaliacao_falha_interna_e_falha = Evaluación, falla interna y falla externa atribuidas a este puesto, por replicación.
godot-indicadores-tempo_em_zonas = Tiempo en zonas
godot-indicadores-tempo_desta_pessoa_dentro_das = Tiempo de esta persona dentro de las zonas de riesgo, por replicación.
godot-indicadores-exposicao_nas_zonas = Exposición en las zonas
godot-indicadores-tempo_de_pessoas_dentro_das = Tiempo de personas dentro de las zonas donde está este puesto, sumado, por replicación.
godot-indicadores-zonas = Zonas
godot-indicadores-zonas_de_risco_onde_o = Zonas de riesgo donde está el puesto.
godot-indicadores-entradas_sem_epi = Entradas sin EPP
godot-indicadores-entradas_na_zona_sem_o = Entradas a la zona sin el equipo de protección que exige, por replicación.
godot-indicadores-quase_conflitos = Casi conflictos
godot-indicadores-pessoa_e_veiculo_mais_perto = Persona y vehículo más cerca que el umbral, involucrando este recurso, por replicación. Simulado: sirve para comparar layouts, no estima accidentes reales.
godot-indicadores-pessoa_e_veiculo_mais_perto-2 = Persona y vehículo más cerca que el umbral a hasta 3 m de este puesto, por replicación. Simulado: sirve para comparar layouts, no estima accidentes reales.
godot-indicadores-quebras = Averías
godot-indicadores-falhas_do_posto_por_replicacao = Fallas del puesto, por replicación.
godot-indicadores-tempo_medio_entre_falhas_tempo = Tiempo medio entre fallas (tiempo en pie sobre el número de fallas).
godot-indicadores-tempo_medio_de_reparo_da = Tiempo medio de reparación, desde la avería hasta volver a producir.
godot-indicadores-tempo_parado = Tiempo detenido
godot-indicadores-tempo_em_manutencao_corretiva_por = Tiempo en mantenimiento correctivo, por replicación.
godot-indicadores-disponibilidade = Disponibilidad
godot-indicadores-tempo_produzindo_sobre_o_tempo = Tiempo produciendo sobre el tiempo planificado (sin turno ni preventivos).
godot-indicadores-custo_de_manutencao = Costo de mantenimiento
godot-indicadores-tempo_parado_reparos_e_preventivas = Tiempo detenido, reparaciones y preventivos por los costos del bloque, por replicación.
godot-indicadores-parcela_do_tempo_em_quebra = Fracción del tiempo en avería, por replicación.
godot-indicadores-tempo_fora_de_quebra_sobre = Tiempo fuera de avería sobre el tiempo planificado (sin turno).

## jobs.rs

godot-jobs-rodando_replicacoes = Ejecutando replicaciones
godot-jobs-rodando_mais_replicacoes = Ejecutando más replicaciones
godot-jobs-comparando_cenarios = Comparando escenarios
godot-jobs-otimizando = Optimizando
godot-jobs-analisando_o_aquecimento = Analizando el calentamiento
godot-jobs-conferindo_a_lei_de_little = Verificando la ley de Little
godot-jobs-procurando_o_gargalo_movel = Buscando el cuello de botella móvil
godot-jobs-calculando_o_pert_cpm = Calculando el PERT/CPM
godot-jobs-ajustando_a_distribuicao_aos_dados = Ajustando la distribución a los datos
godot-jobs-gravando_o_relatorio_dos_dados = Grabando el informe de los datos
godot-jobs-carregando_a_reproducao = Cargando la reproducción
godot-jobs-erro_interno_durante_a_execucao = error interno durante la ejecución
godot-jobs-a_execucao_terminou_sem_resultado = la ejecución terminó sin resultado
godot-jobs-objeto_nao_existe_no_modelo = el objeto "{ $object }" no existe en el modelo

## linha.rs


## path_edit.rs

godot-path-edit-nao_existe = '{ $name }' no existe
godot-path-edit-e_uma_esteira_reta_sem = '{ $name }' es una cinta transportadora recta, sin nodos
godot-path-edit-nao_e_uma_esteira = '{ $name }' no es una cinta transportadora
godot-path-edit-a_esteira_precisa_de_ao = la cinta transportadora necesita al menos 2 nodos distintos
godot-path-edit-a_biblioteca_nao_tem_esteira = la biblioteca no tiene cinta transportadora
godot-path-edit-o_caminho_precisa_de_ao = el camino necesita al menos 2 puntos distintos
godot-path-edit-nao_existe_o_caminho = no existe el camino '{ $name }'
godot-path-edit-nao_tem_o_no = '{ $name }' no tiene el nodo { $index }
godot-path-edit-o_raio_deve_ser_um = el radio debe ser un número >= 0 (metros)
godot-path-edit-a_distancia_do_paralelo_deve = la distancia del paralelo debe ser distinta de cero
godot-path-edit-o_caminho_novo_sumiu = el camino nuevo desapareció
godot-path-edit-o_caminho_precisa_de_ao-2 = el camino necesita al menos 2 puntos
godot-path-edit-a_esteira_precisa_de_ao-2 = la cinta transportadora necesita al menos 2 nodos
godot-path-edit-nao_e_uma_esteira_desenhada = '{ $name }' no es una cinta transportadora dibujada
godot-path-edit-as_duas_pontas_ja_estao = los dos extremos ya están en el piso
godot-path-edit-tipo_de_esteira_desconhecido = tipo de cinta transportadora desconocido: { $style }
godot-path-edit-acionamento_desconhecido = accionamiento desconocido: { $drive }
godot-path-edit-a_largura_deve_ser_0 = el ancho debe ser > 0 (metros)
godot-path-edit-a_inclinacao_maxima_fica_entre = la inclinación máxima está entre 0 y 90 grados
godot-path-edit-divida_num_no_do_meio = divida en un nodo del medio de la cinta transportadora
godot-path-edit-nao_ha_trecho_depois_do = no hay tramo después del último nodo
godot-path-edit-o_caminho_so_tem_esse = el camino solo tiene este tramo: borre el camino completo
godot-path-edit-a_esteira_so_tem_esse = la cinta transportadora solo tiene este tramo: borre la cinta completa
godot-path-edit-nenhum_trajeto_comeca_onde_este = ningún trayecto comienza donde termina este
godot-path-edit-nao_existe_o_caminho-2 = no existe el camino '{ $other }'

## tool_panels.rs

godot-tool-panels-ligacao = Conexión
godot-tool-panels-remover_ligacao_del = Quitar conexión (Del)
godot-tool-panels-rede_de_caminhos = Red de caminos
godot-tool-panels-no_s_trecho_s_pista =
    { $nodes } { $nodes ->
        [one] nodo
       *[other] nodos
    }, { $edges } { $edges ->
        [one] tramo
       *[other] tramos
    }, { $controlled } { $controlled ->
        [one] carril controlado
       *[other] carriles controlados
    }.
godot-tool-panels-clique_no_chao_para_por = Haga clic en el piso para poner un nodo; con un nodo armado (verde), el siguiente clic lo conecta con el nuevo mediante un tramo. Haga clic en un nodo para armarlo, arrastre para moverlo. Haga clic en un tramo para cambiar el sentido y la capacidad (con capacidad, el carril es controlado y se muestra en rojo). Esc termina el camino; Del elimina el seleccionado.
godot-tool-panels-novos_trechos_em_mao_dupla = Nuevos tramos de doble sentido
godot-tool-panels-no = Nodo { $name }
godot-tool-panels-nome = Nombre
godot-tool-panels-posicao_m = Posición ({ $position }, { $position2 }) m
godot-tool-panels-excluir_no_del = Eliminar nodo (Del)
godot-tool-panels-trecho = Tramo { $from } → { $to }
godot-tool-panels-comprimento_m = Longitud { $length } m
godot-tool-panels-mao_dupla = Doble sentido
godot-tool-panels-capacidade_veiculos_0_sem_controle = Capacidad (vehículos; 0 = sin control)
godot-tool-panels-pista_controlada_no_maximo_veiculo =
    Carril controlado: como máximo { $cap } { $cap ->
        [one] vehículo
       *[other] vehículos
    } al mismo tiempo.
godot-tool-panels-inverter_sentido = Invertir sentido
godot-tool-panels-excluir_trecho_del = Eliminar tramo (Del)
godot-tool-panels-pedestres = Peatones
godot-tool-panels-empilhadeiras = Montacargas
godot-tool-panels-misto = Mixto
godot-tool-panels-corredores_de_circulacao = Pasillos de circulación
godot-tool-panels-corredor_es_de_pedestres_de =
    { $list } { $list ->
        [one] pasillo
       *[other] pasillos
    }: { $pedestrian } de peatones, { $forklift } de montacargas, { $forklift2 } mixtos; { $crossings } { $crossings ->
        [one] cruce
       *[other] cruces
    }.
godot-tool-panels-clique_no_chao_para_por-2 = Haga clic en el piso para poner los puntos del eje; Enter o doble clic termina, Backspace retrocede un punto, Esc cancela. Los puntos se ajustan a los otros pasillos. Haga clic en un pasillo para editarlo, arrastre un vértice para moverlo, Shift + clic en la línea agrega un vértice. Las franjas amarillas, la cebra de los cruces y las flechas aparecen en el piso.
godot-tool-panels-novos_corredores = Nuevos pasillos
godot-tool-panels-tipo = Tipo
godot-tool-panels-largura_m = Ancho (m)
godot-tool-panels-corredor = Pasillo { $name }
godot-tool-panels-ponto_s_m_de_comprimento =
    { $points } { $points ->
        [one] punto
       *[other] puntos
    }, { $length } m de longitud
godot-tool-panels-passagem_secundaria_de_pedestres_minimo = Paso secundario de peatones (mínimo menor)
godot-tool-panels-perfil = Perfil
godot-tool-panels-padrao_do_modelo = Predeterminado del modelo
godot-tool-panels-brasil_amarelo = Brasil (amarillo)
godot-tool-panels-brasil = Brasil
godot-tool-panels-branco = Blanco
godot-tool-panels-vertice_selecionado = Vértice { $v } seleccionado
godot-tool-panels-remover_vertice = Quitar vértice
godot-tool-panels-excluir_corredor_del = Eliminar pasillo (Del)
godot-tool-panels-zonas_de_risco = Zonas de riesgo
godot-tool-panels-zona_s_de_risco_no =
    { $zones } { $zones ->
        [one] zona
       *[other] zonas
    } de riesgo en el piso.
godot-tool-panels-clique_no_chao_para_por-3 = Haga clic en el piso para poner las esquinas; el primer punto, Enter o doble clic cierra la zona, Backspace retrocede un punto, Esc cancela. Haga clic en una zona para editarla: arrastre el centro para mover, arrastre un vértice para cambiar la forma; Del elimina. Los puntos se ajustan a la cuadrícula.
godot-tool-panels-zona = Zona { $name }
godot-tool-panels-canto_s =
    { $polygon } { $polygon ->
        [one] esquina
       *[other] esquinas
    }
godot-tool-panels-nivel_de_risco_1_a = Nivel de riesgo (1 a 5; 0 = sin nivel)
godot-tool-panels-epi_exigido_para_entrar = EPP exigido para entrar
godot-tool-panels-excluir_zona_del = Eliminar zona (Del)

## ui_arrivals.rs

godot-ui-arrivals-nao_e_um_numero = "{ $t }" no es un número
godot-ui-arrivals-use_um_numero = use un número
godot-ui-arrivals-hh_mm_so_vale_com = hh:mm solo vale con ciclo de día o semana (1440 min, 24 h, 86400 s); use un número
godot-ui-arrivals-hora_invalida_em = hora inválida en "{ $t }"
godot-ui-arrivals-minuto_invalido_em = minuto inválido en "{ $t }"
godot-ui-arrivals-minuto_invalido_em_0_a = minuto inválido en "{ $t }" (0 a 59)
godot-ui-arrivals-nao_e_um_horario_hh = "{ $t }" no es un horario (hh:mm) ni un número
godot-ui-arrivals-quantidade_um_inteiro_0 = cantidad "{ $t }": un entero >= 0
godot-ui-arrivals-rotulo_use_nome_valor = etiqueta "{ $part }": use nombre=valor
godot-ui-arrivals-rotulo_sem_nome = etiqueta "{ $part }" sin nombre
godot-ui-arrivals-rotulo = etiqueta { $name }: { $e }
godot-ui-arrivals-uma_vez = Una vez
godot-ui-arrivals-a_cada_ciclo = En cada ciclo
godot-ui-arrivals-volta_ao_inicio = Vuelve al inicio
godot-ui-arrivals-por_ciclo_de = por ciclo de { $d }
godot-ui-arrivals-por_volta = por vuelta
godot-ui-arrivals-no_total = en total
godot-ui-arrivals-em_linha_s =
    { $items } { $items ->
        [one] ítem
       *[other] ítems
    } { $per } en { $rows } { $rows ->
        [one] línea
       *[other] líneas
    }
godot-ui-arrivals-tipo_da_fonte = tipo de la fuente
godot-ui-arrivals-um_dia = { $d } (un día)
godot-ui-arrivals-uma_semana = { $d } (una semana)
godot-ui-arrivals-tipo_de_item = tipo de ítem
godot-ui-arrivals-nada_para_colar_copie_as = nada que pegar: copie las líneas de la hoja de cálculo
godot-ui-arrivals-linha = línea { $n }: { $e }
godot-ui-arrivals-falta_o_instante_modo_horario = falta el instante (modo horario)
godot-ui-arrivals-so_o_cabecalho_nenhuma_linha = solo el encabezado: ninguna línea que pegar
godot-ui-arrivals-editar_como_texto_ron = Editar como texto (RON)
godot-ui-arrivals-modo = Modo
godot-ui-arrivals-horario = Horario
godot-ui-arrivals-sequencia = Secuencia
godot-ui-arrivals-horario_cada_linha_no_seu = Horario: cada línea en su instante. Secuencia: una línea por llegada, con el tiempo entre llegadas de la fuente entre ellas.
godot-ui-arrivals-repetir = Repetir
godot-ui-arrivals-ciclo = Ciclo
godot-ui-arrivals-da_fonte = (de la fuente)
godot-ui-arrivals-o_tipo_da_fonte = El tipo de la fuente
godot-ui-arrivals-tipo_de_item_da_linha = Tipo de ítem de la línea
godot-ui-arrivals-subir_a_linha = Subir la línea
godot-ui-arrivals-descer_a_linha = Bajar la línea
godot-ui-arrivals-remover_a_linha = Quitar la línea
godot-ui-arrivals-linha-2 = + Línea
godot-ui-arrivals-acrescenta_uma_linha_no_fim = Agrega una línea al final
godot-ui-arrivals-ordenar_por_instante = Ordenar por instante
godot-ui-arrivals-poe_as_linhas_em_ordem = Pone las líneas en orden de instante
godot-ui-arrivals-as_linhas_ja_estao_em = Las líneas ya están en orden (o es una secuencia)
godot-ui-arrivals-colar_da_planilha = Pegar de la hoja de cálculo
godot-ui-arrivals-linhas_copiadas_de_uma_planilha = Líneas copiadas de una hoja de cálculo (separadas por tabulación o ;)
godot-ui-arrivals-hora_qtd_tipo_rotulo_08 =
    hora; cant; tipo; etiqueta...
    08:00; 12; caja; prioridad=2
godot-ui-arrivals-qtd_tipo_rotulo_2_caixa =
    cant; tipo; etiqueta...
    2; caja; prioridad=2
godot-ui-arrivals-substituir_as_linhas = Reemplazar las líneas
godot-ui-arrivals-acrescentar = Agregar
godot-ui-arrivals-ver_as_primeiras_chegadas = Ver las primeras llegadas
godot-ui-arrivals-histograma_das_chegadas_de_um = Histograma de las llegadas de un ciclo (o una vuelta), por hora y por tipo
godot-ui-arrivals-simulado_so_a_fonte_ligada = Simulado: solo la fuente, conectada a una salida, por un ciclo.
godot-ui-arrivals-estimado_pela_tabela_a_fonte = Estimado por la tabla (la fuente sola no se ejecutó: { $e }).
godot-ui-arrivals-estimado_pela_tabela_rodando_a = Estimado por la tabla; ejecutando la fuente...
godot-ui-arrivals-maximo_por = máximo { $num_text } por { $bin_label }
godot-ui-arrivals-por_instantes_da_tabela_sem = por { $bin_label } ({ $bin }); instantes de la tabla, sin ejecutar el modelo
godot-ui-arrivals-horarios_em_hh_mm_taxas = { $name } (horarios en hh:mm, tasas por hora)
godot-ui-arrivals-unidade_do_modelo = unidad del modelo
godot-ui-arrivals-fora_dos_periodos_a_fonte = Fuera de los períodos la fuente no crea ítems
godot-ui-arrivals-periodo_s =
    { $periods } { $periods ->
        [one] período
       *[other] períodos
    }
godot-ui-arrivals-itens_h_ou_distribuicao = ítems/h o distribución
godot-ui-arrivals-taxa_ou_distribuicao = tasa o distribución
godot-ui-arrivals-padrao_o_tempo_entre_chegadas = Predeterminado: el tiempo entre llegadas de la fuente. Distribución: un tiempo entre llegadas propio. Tasa: llegadas de Poisson. Cerrado: sin llegadas.
godot-ui-arrivals-distribuicao = distribución: { $e }
godot-ui-arrivals-media = media { $mean }
godot-ui-arrivals-sem_chegadas = sin llegadas
godot-ui-arrivals-remover_o_periodo = Quitar el período
godot-ui-arrivals-periodo = + Período
godot-ui-arrivals-acrescenta_um_periodo_depois_do = Agrega un período después del último
godot-ui-arrivals-o_ultimo_periodo_ja_vai = El último período ya llega hasta el final del ciclo
godot-ui-arrivals-esperado_por_ciclo_chegadas = Esperado por ciclo: { $total } llegadas
godot-ui-arrivals-padrao_media = predeterminado (media { $mean })
godot-ui-arrivals-distribuicao-2 = distribución { $d }
godot-ui-arrivals-taxa_itens_h = tasa { $cycle } ítems/h
godot-ui-arrivals-taxa = tasa { $num_text }
godot-ui-arrivals-periodo_a_esperadas = Período { $k }: { $cycle } a { $cycle2 }, { $how }; esperadas { $default }
godot-ui-arrivals-chegadas_esperadas_em_cada_periodo = Llegadas esperadas en cada período (tasa × duración, o duración ÷ media)
godot-ui-arrivals-barra_s =
    { $exp } { $exp ->
        [one] barra
       *[other] barras
    }
godot-ui-arrivals-nao_existe = '{ $source }' no existe
godot-ui-arrivals-nao_e_uma_chegada = '{ $source }' no es una llegada

## ui_bar.rs

godot-ui-bar-equipe_despachante_membro = Equipo (despachador → miembro)
godot-ui-bar-transporte_pedido = Transporte pedido
godot-ui-bar-operadores_pedidos = Operadores pedidos
godot-ui-bar-equipe_de_manutencao = Equipo de mantenimiento
godot-ui-bar-local_de_pausa = Lugar de pausa
godot-ui-bar-mais = Más
godot-ui-bar-os_grupos_que_nao_cabem = Los grupos que no caben en la barra
godot-ui-bar-arquivo = Archivo
godot-ui-bar-editar = Editar
godot-ui-bar-organizar = Organizar
godot-ui-bar-grade = Cuadrícula
godot-ui-bar-ajuda = Ayuda
godot-ui-bar-analise = Análisis
godot-ui-bar-novo = Nuevo
godot-ui-bar-novo_modelo_ctrl_n = Nuevo modelo (Ctrl+N)
godot-ui-bar-abrir = Abrir…
godot-ui-bar-abrir_um_modelo_ou_um = Abrir un modelo o una repetición (Ctrl+O)
godot-ui-bar-salvar = Guardar
godot-ui-bar-salvar_ctrl_s = Guardar (Ctrl+S)
godot-ui-bar-salvar_como = Guardar como…
godot-ui-bar-salvar_com_outro_nome_ctrl = Guardar con otro nombre (Ctrl+Shift+S)
godot-ui-bar-desfazer = Deshacer
godot-ui-bar-desfazer_ctrl_z = Deshacer (Ctrl+Z)
godot-ui-bar-refazer = Rehacer
godot-ui-bar-refazer_ctrl_y = Rehacer (Ctrl+Y)
godot-ui-bar-alinhar_a_esquerda = Alinear a la izquierda
godot-ui-bar-alinhar_ao_centro_x = Alinear al centro (X)
godot-ui-bar-alinhar_a_direita = Alinear a la derecha
godot-ui-bar-alinhar_ao_topo = Alinear arriba
godot-ui-bar-alinhar_ao_centro_y = Alinear al centro (Y)
godot-ui-bar-alinhar_a_base = Alinear abajo
godot-ui-bar-distribuir_em_x = Distribuir en X
godot-ui-bar-distribuir_em_y = Distribuir en Y
godot-ui-bar-copiar_ctrl_c = Copiar (Ctrl+C)
godot-ui-bar-recortar_ctrl_x = Cortar (Ctrl+X)
godot-ui-bar-colar_ctrl_v = Pegar (Ctrl+V)
godot-ui-bar-duplicar_ctrl_d = Duplicar (Ctrl+D)
godot-ui-bar-excluir_del = Eliminar (Del)
godot-ui-bar-selecionar_tudo_ctrl_a = Seleccionar todo (Ctrl+A)
godot-ui-bar-alinhar_distribuir_copiar_colar_e = Alinear, distribuir, copiar, pegar y eliminar la selección
godot-ui-bar-encaixar_na_grade_ao_por = Ajustar a la cuadrícula al colocar y arrastrar (G; paso al lado)
godot-ui-bar-passo_da_grade = Paso de la cuadrícula
godot-ui-bar-atalhos_f1_a_lista_dos = Atajos (F1): la lista de atajos de teclado y mouse
godot-ui-bar-resultados = Resultados
godot-ui-bar-tabela_graficos_analises_cenarios_e = Tabla, gráficos, análisis, escenarios y optimización de la última ronda
godot-ui-bar-pert_cpm = PERT/CPM
godot-ui-bar-pert_cpm_da_selecao_com = PERT/CPM de la selección (con dos objetos, de uno hasta el otro)
godot-ui-bar-pert_cpm_da_selecao = PERT/CPM de la selección
godot-ui-bar-pert_cpm_de_ate_dois = PERT/CPM de… hasta… (dos seleccionados)
godot-ui-bar-fluxos_de_tokens = Flujos de tokens
godot-ui-bar-modo_missao_utsim_ensino = Modo misión (UTSim Enseñanza)
godot-ui-bar-abre_as_missoes_do_utsim = Abre las misiones de UTSim Enseñanza en este mismo programa
godot-ui-bar-outras_analises_pert_cpm_de = Otros análisis: PERT/CPM de… hasta…, flujos de tokens; las conexiones de equipo y de pedido en la vista 3D
godot-ui-bar-simular = Simular
godot-ui-bar-montar_o_modelo_tab_alterna = Armar el modelo (Tab alterna)
godot-ui-bar-rodar_as_replicacoes_e_reproduzir = Ejecutar las replicaciones y reproducir (Tab alterna)
godot-ui-bar-selecionar = Seleccionar
godot-ui-bar-selecionar_e_mover_esc = Seleccionar y mover (Esc)
godot-ui-bar-ligar = Conectar
godot-ui-bar-ligar_arraste_da_origem_ao = Conectar: arrastre del origen al destino, o haga clic en el origen y luego en el destino (L)
godot-ui-bar-rede = Red
godot-ui-bar-rede_de_caminhos_nos_e = Red de caminos: nodos y tramos (N)
godot-ui-bar-corredor = Pasillo
godot-ui-bar-corredores_de_circulacao_com_faixas = Pasillos de circulación con franjas de seguridad: haga clic en puntos del piso, Enter termina (C)
godot-ui-bar-zona = Zona
godot-ui-bar-zonas_de_risco_no_piso = Zonas de riesgo en el piso: haga clic en las esquinas, el primer punto o Enter cierra; haga clic en una zona para editar (Z)
godot-ui-bar-equipe = Equipo
godot-ui-bar-equipe_arraste_do_despachante_a = Equipo: arrastre del despachador a un transportador (entra o sale del equipo), o de un objeto al transportador o despachador que lleva los ítems; con Shift, de un proceso a quien provee los operadores (E; Alt con Conectar)
godot-ui-bar-ligacoes_na_vista_3d = Conexiones en la vista 3D
godot-ui-bar-tambem_durante_a_simulacao = También durante la simulación
godot-ui-bar-as_linhas_tracejadas_entre_operadores = Las líneas punteadas entre operadores, vehículos y máquinas en el modo Simular

## ui_fitwin.rs

godot-ui-fitwin-linha_nao_e_um_numero = línea { $k }: "{ $t }" no es un número
godot-ui-fitwin-os_instantes_precisam_ser_crescentes = los instantes deben ser crecientes
godot-ui-fitwin-digite_ou_cole_pelo_menos = escriba o pegue al menos dos valores
godot-ui-fitwin-compare_as_distribuicoes_antes = compare las distribuciones antes
godot-ui-fitwin-utsim_3d_valores_reais = UTSim 3D, Valores reales
godot-ui-fitwin-desvio_padrao = desviación estándar
godot-ui-fitwin-valores_reais = Valores reales: { $field }
godot-ui-fitwin-o_que_e = Qué es: { $title }
godot-ui-fitwin-valores_observados = Valores observados
godot-ui-fitwin-digite_cole_da_planilha_varias = Escriba, pegue de la hoja de cálculo (varias líneas, coma decimal) o importe un CSV.
godot-ui-fitwin-cole_aqui_e_clique_adicionar = pegue aquí y haga clic en Agregar
godot-ui-fitwin-adicionar = Agregar
godot-ui-fitwin-valor_es_adicionados =
    { $n } { $n ->
        [one] valor agregado
       *[other] valores agregados
    }.
godot-ui-fitwin-importar_csv = Importar CSV…
godot-ui-fitwin-le_os_valores_de_um = Lee los valores de un archivo CSV o TXT
godot-ui-fitwin-limpar = Limpiar
godot-ui-fitwin-apaga_todos_os_valores_da = Borra todos los valores de la tabla
godot-ui-fitwin-sao_instantes_crescentes_usar_os = Son instantes crecientes (usar los intervalos)
godot-ui-fitwin-marque_quando_os_valores_sao = Márquelo cuando los valores sean horas de llegada: el ajuste usa las diferencias entre ellos.
godot-ui-fitwin-valor_es =
    { $n } { $n ->
        [one] valor
       *[other] valores
    }
godot-ui-fitwin-apaga_este_valor = Borra este valor
godot-ui-fitwin-intervalos = Intervalos
godot-ui-fitwin-estatisticas = Estadísticas
godot-ui-fitwin-comparacao_das_distribuicoes = Comparación de las distribuciones
godot-ui-fitwin-comparar_de_novo = Comparar de nuevo
godot-ui-fitwin-comparar_todas = Comparar todas
godot-ui-fitwin-ajusta_todas_as_distribuicoes_candidatas = Ajusta todas las distribuciones candidatas y calcula todas las medidas de una vez
godot-ui-fitwin-ajustando_todas_as_candidatas = ajustando todas las candidatas...
godot-ui-fitwin-depois_de_comparar_a_tabela = Después de comparar, la tabla muestra todas las medidas de cada distribución; haga clic en una línea para ver los gráficos y elegir.
godot-ui-fitwin-os_valores_mudaram_compare_de = Los valores cambiaron: compare de nuevo.
godot-ui-fitwin-ordenada_por_sugestao_inicial_aic = Ordenada por { $label }. Sugerencia inicial (AIC con la regla de UTSim): { $family }.
godot-ui-fitwin-usar_esta_distribuicao = Usar esta distribución ({ $fam })
godot-ui-fitwin-passa_a_usar_esta_distribuicao = Pasa a usar esta distribución en este campo, ajustada a estos valores (Ctrl+Z deshace)
godot-ui-fitwin-usar_a_empirica = Usar la empírica
godot-ui-fitwin-usa_os_proprios_valores_observados = Usa los propios valores observados, sin familia
godot-ui-fitwin-gerar_relatorio = Generar informe
godot-ui-fitwin-grava_o_relatorio_html_com = Graba el informe HTML con los datos, la comparación completa, los gráficos y la elección
godot-ui-fitwin-fechar = Cerrar
godot-ui-fitwin-fecha_a_explicacao = Cierra la explicación
godot-ui-fitwin-como_ler_quando_usar_cuidados =
    { $what }
    Cómo leer: { $read }
    Cuándo usar: { $when }
    Cuidados: { $care }
godot-ui-fitwin-aceita_pelos_testes_clique_para = Aceptada por las pruebas. Haga clic para ver los gráficos y elegir.
godot-ui-fitwin-nao_se_aplica_a_estes = No se aplica a estos datos.
godot-ui-fitwin-rejeitada_por_algum_teste_valor = Rechazada por alguna prueba (valor p por debajo de 0,05). Aun así puede elegirse.
godot-ui-fitwin-ordena_por = Ordena por { $label } ({ $melhor })
godot-ui-fitwin-escolhida_por = elegida por
godot-ui-fitwin-histograma_e_curvas_em_destaque = Histograma y curvas ({ $selected_family } destacada)
godot-ui-fitwin-poisson = Poisson
godot-ui-fitwin-geometrica = Geométrica
godot-ui-fitwin-binomial = Binomial
godot-ui-fitwin-binomial_negativa = Binomial negativa
godot-ui-fitwin-a_empirica_reproduz_os_proprios = La empírica reproduce los propios datos: no hay curva para comparar.

## ui_left.rs

godot-ui-left-orbita = Órbita
godot-ui-left-planta_2d = Planta (2D)
godot-ui-left-seguir = Seguir
godot-ui-left-primeira_pessoa = Primera persona
godot-ui-left-do_recurso = Del recurso
godot-ui-left-recentes = Recientes
godot-ui-left-usados_por_ultimo = Usados por última vez
godot-ui-left-espera = Espera
godot-ui-left-fluxo = Flujo
godot-ui-left-fluxo_de_itens_e_montagem = Flujo de ítems y montaje
godot-ui-left-mover = Mover
godot-ui-left-movimentacao_recursos_e_logistica = Movimiento, recursos y logística
godot-ui-left-fabrica = Fábrica
godot-ui-left-manufatura = Manufactura
godot-ui-left-servicos = Servicios
godot-ui-left-servicos_e_saude = Servicios y salud
godot-ui-left-escritorio = Oficina
godot-ui-left-escritorio_e_processos = Oficina y procesos
godot-ui-left-q_e_s = C y S
godot-ui-left-qualidade_e_seguranca = Calidad y seguridad
godot-ui-left-biblioteca = Biblioteca
godot-ui-left-objetos = Objetos
godot-ui-left-mostrar_obstaculos_e_folgas = Mostrar obstáculos y holguras
godot-ui-left-mostra_a_area_ocupada_dos = Muestra el área ocupada de los objetos, las paredes, la holgura de los recursos y el punto de acceso de cada objeto (arrástrelo con la herramienta Seleccionar)
godot-ui-left-nada_com_esse_nome = Nada con ese nombre.
godot-ui-left-clique_de_novo_para_recolher = { $title }. Haga clic de nuevo para contraer.
godot-ui-left-os_elementos_que_voce_usar = Los elementos que use aparecen aquí.
godot-ui-left-buscar_elemento = Buscar elemento
godot-ui-left-limpa_a_busca = Limpia la búsqueda
godot-ui-left-clique_e_depois_clique_no = { $label }: haga clic y luego clic en el piso, o arrastre hasta el piso
godot-ui-left-buscar_por_nome_ou_tipo = Buscar por nombre o tipo
godot-ui-left-problema_s =
    { $problems } { $problems ->
        [one] problema
       *[other] problemas
    }
godot-ui-left-aviso_s =
    { $warnings } { $warnings ->
        [one] aviso
       *[other] avisos
    }
godot-ui-left-simular_de_novo = Simular de nuevo
godot-ui-left-roda_as_replicacoes_outra_vez = Ejecuta las replicaciones otra vez
godot-ui-left-blocos_clique_para_focar = Bloques (clic para enfocar)
godot-ui-left-fluxos = Flujos
godot-ui-left-fluxos_de_tokens_em_2d = Flujos de tokens en 2D, con los conteos en el tiempo
godot-ui-left-gravar_replay = Grabar repetición
godot-ui-left-grava_o_replay_desta_replicacao = Graba la repetición de esta replicación en .json (se abre de nuevo con Abrir)
godot-ui-left-rotulos = Etiquetas
godot-ui-left-os_nomes_dos_blocos_sobre = Los nombres de los bloques sobre la escena
godot-ui-left-sombras = Sombras
godot-ui-left-as_sombras_do_sol_desligadas = Las sombras del sol (desactivadas, la escena queda más ligera)
godot-ui-left-seguranca = Seguridad
godot-ui-left-zonas_de_risco_epi_das = Zonas de riesgo, EPP de las personas, casi conflictos simulados, ítems con defecto e inspecciones
godot-ui-left-calor = Calor
godot-ui-left-mapa_de_calor_da_presenca = Mapa de calor de la presencia de las personas y de los casi conflictos en el piso
godot-ui-left-quase_conflitos_simulados = Casi conflictos simulados ({ $quase })
godot-ui-left-replicacoes = Replicaciones
godot-ui-left-uma_replicacao_a_menos = Una replicación menos
godot-ui-left-uma_replicacao_a_mais = Una replicación más
godot-ui-left-camera = Cámara
godot-ui-left-seguir_primeira_pessoa_e_do = Seguir, Primera persona y Del recurso acompañan el bloque elegido en la lista
godot-ui-left-os_resultados_aparecem_aqui_depois = Los resultados aparecen aquí después de la simulación.
godot-ui-left-gargalo = Cuello de botella: { $bottleneck }
godot-ui-left-a_tabela_completa_os_graficos = La tabla completa, los gráficos y los análisis están en Resultados.
godot-ui-left-gargalo-2 =
    {""}
    Cuello de botella: { $bottleneck }

## ui_pert.rs

godot-ui-pert-rede = Red
godot-ui-pert-gantt = Gantt
godot-ui-pert-distribuicao = Distribución
godot-ui-pert-tabela = Tabla
godot-ui-pert-texto = Texto
godot-ui-pert-simulado = Simulado
godot-ui-pert-caminho_critico_observado_na_simulacao = Camino crítico observado en la simulación: colas, transportes y recursos incluidos
godot-ui-pert-classico = Clásico
godot-ui-pert-pert_classico_estimativas_a_m = PERT clásico: estimaciones (a, m, b), CPM en las medias y P(plazo) por la normal
godot-ui-pert-monte_carlo = Monte Carlo
godot-ui-pert-sorteios_das_duracoes_sobre_a = Sorteos de las duraciones sobre la red: criticidad y distribución del tiempo total
godot-ui-pert-desvio = , desviación { $sd }
godot-ui-pert-unidades_sorteios = {" "}({ $units } unidades/sorteos)
godot-ui-pert-tempo_total_cpm_nas_medias = Tiempo total: { $t }{ $spread }{ $units }. CPM en las medias: { $project }.
godot-ui-pert-ic_a = {" "}(IC { $lo }% a { $hi }%)
godot-ui-pert-p_t = P(T ≤ { $deadline }) = { $p }
godot-ui-pert-caminho_critico = Camino crítico: { $names } ({ $index }%)
godot-ui-pert-aviso = aviso: { $w }
godot-ui-pert-aproximacao_normal_do_pert_classico = aproximación normal del PERT clásico
godot-ui-pert-unidades_observadas = unidades observadas
godot-ui-pert-a_linha_amarela_e_o = ; la línea amarilla es el plazo
godot-ui-pert-distribuicao_do_tempo_total = Distribución del tiempo total ({ $source }){ $marker }.
godot-ui-pert-pert_cpm_tempo_total = PERT/CPM ({ $mode }): tiempo total { $total }.
godot-ui-pert-atividade = Actividad
godot-ui-pert-duracao = Duración
godot-ui-pert-folga_total = Holgura total
godot-ui-pert-folga_livre = Holgura libre
godot-ui-pert-criticidade = Criticidad
godot-ui-pert-prazo = plazo { $num }
godot-ui-pert-sem_distribuicao_para_este_modo = sin distribución para este modo
godot-ui-pert-hipotese = hipótesis: { $h }
godot-ui-pert-fechar = Cerrar
godot-ui-pert-fechar_a_janela_pert_cpm = Cerrar la ventana PERT/CPM
godot-ui-pert-prazo-2 = Plazo
godot-ui-pert-sem_prazo = sin plazo
godot-ui-pert-sorteios_monte_carlo = Sorteos (Monte Carlo)
godot-ui-pert-recalcular = Recalcular
godot-ui-pert-calcula_de_novo_no_modo = Calcula de nuevo en el modo actual
godot-ui-pert-relatorio_html = informe .html
godot-ui-pert-exportar_html = Exportar HTML
godot-ui-pert-grava_o_relatorio_html_do = Graba el informe HTML del PERT/CPM
godot-ui-pert-csv = CSV
godot-ui-pert-grava_as_tabelas_em_csv = Graba las tablas en CSV en la carpeta del archivo

## zone_edit.rs

godot-zone-edit-trafego_de_veiculos = Tráfico de vehículos
godot-zone-edit-alcance_de_maquina = Alcance de máquina
godot-zone-edit-carga_suspensa = Carga suspendida
godot-zone-edit-ruido = Ruido
godot-zone-edit-calor = Calor
godot-zone-edit-outro = Otro
godot-zone-edit-a_zona_nao_existe = la zona '{ $name }' no existe
godot-zone-edit-uma_zona_precisa_de_pelo = una zona necesita al menos 3 puntos
godot-zone-edit-a_zona_precisa_de_um = la zona necesita un nombre
godot-zone-edit-tipo_de_zona_desconhecido = tipo de zona desconocido: { $value }
godot-zone-edit-nivel_um_numero_de_1 = nivel: un número de 1 a 5
godot-zone-edit-campo_de_zona_desconhecido = campo de zona desconocido: { $field }

## Ronda 13: diálogos y mensajes de error

godot-dialogo-descartar-titulo = ¿Descartar cambios?
godot-dialogo-descartar-texto = El modelo tiene cambios sin guardar. ¿Descartar?
godot-dialogo-descartar = Descartar
godot-dialogo-voltar = Volver
godot-dialogo-atalhos = Atajos
godot-dialogo-fechar = Cerrar
godot-dialogo-desempenho = Muestra u oculta las medidas de rendimiento sobre la vista 3D

godot-schema-distribuicao_invalida = distribución no válida: { $e }
godot-schema-familia_invalida = familia no válida
godot-schema-texto_ron_esperado = se esperaba texto RON
godot-schema-so_a_distribuicao_deslocada_tem = solo la distribución desplazada tiene una distribución interna
godot-schema-a_distribuicao_nao_tem_o = la distribución no tiene el parámetro "{ $key }"
godot-schema-modo_de_movimentacao_desconhecido = modo de movimiento desconocido: { $name }
godot-schema-modo_de_movimentacao_esperado = se esperaba un modo de movimiento
godot-schema-a_velocidade_deve_ser_maior = la velocidad debe ser mayor que cero
godot-schema-campo_desconhecido = campo desconocido: { $w }
godot-schema-rotulo_esperado = se esperaba una etiqueta
godot-schema-o_rotulo_do_peso_nao = la etiqueta del peso no puede quedar vacía
godot-schema-ordem_safe_sequential_ou_parallel = orden: Safe, Sequential o Parallel
godot-schema-crie_antes_uma_pessoa_ou = cree antes una persona (o un despachador) para operar el puente grúa
godot-schema-controle_cabin_pendant_ou_radio = control: Cabin, Pendant o Radio
godot-schema-escolha_a_equipe_do_operador = elija el equipo del operador
godot-schema-escolha_antes_botoeira_ou_radio = elija antes botonera o radio
godot-schema-tipo_de_espera_table_pallet = tipo de espera: Table, Pallet, Shelf o Floor, no { $k }
godot-schema-camadas_demais = demasiadas capas
godot-schema-espera_nao_tem_o_campo = espera no tiene el campo "{ $rest }"
godot-schema-espera = espera: { $e }
godot-schema-sexo_female_male_ou_vazio = sexo: Female, Male o vacío, no { $other }
godot-schema-idade_grande_demais = edad demasiado grande
godot-schema-pessoa_nao_tem_o_campo = persona no tiene el campo "{ $field }"
godot-schema-pessoa = persona: { $e }
godot-schema-crie_antes_um_transportador_ou = cree antes un transportador o un despachador para operar el proceso
godot-schema-ligue_a_transferencia_antes = active antes la transferencia
godot-schema-caminho_straight_ou_network = camino: Straight o Network
godot-schema-ligue_as_quebras_antes = active antes los descansos
godot-schema-ligue_os_turnos_antes = active antes los turnos
godot-schema-fim_do_turno_pause_ou = fin del turno: Pause o Finish, no { $t }
godot-schema-pausas_invalidas = descansos no válidos: { $e }
godot-schema-escolha_um_calendario = elija un calendario
godot-schema-ligue_o_lote_antes = active antes el lote
godot-schema-ligue_antes = active antes { $head }
godot-schema-ligue_os_operadores_antes = active antes los operadores
godot-schema-o_ganho_da_equipe_deve = la ganancia del equipo debe ser mayor que cero
godot-schema-objeto_nao_existe = el objeto "{ $object }" no existe
godot-schema-posicao_x_y_ou_z = posición: x, y o z
godot-schema-politica_esperada = se esperaba una política
godot-schema-politica_de_acesso_desconhecida = política de acceso desconocida: { $v }
godot-schema-o_rotulo_da_prioridade_nao = la etiqueta de la prioridad no puede quedar vacía
godot-schema-script_esperado = se esperaba un script
godot-schema-regra_firstfree_random_ou_abc = regla: FirstFree, Random o Abc
godot-schema-o_rotulo_nao_pode_ficar = la etiqueta no puede quedar vacía
godot-schema-modo_unpack_ou_split = modo: Unpack o Split
godot-schema-o_modelo_nao_tem_calendarios = el modelo no tiene calendarios (cree uno en "calendars")
godot-schema-origem_dos_turnos_inline_ou = origen de los turnos: inline o calendar
godot-schema-texto_esperado = se esperaba texto
godot-schema-modo_esperado = se esperaba un modo
godot-schema-perfil_brasil_ou_branco = perfil: Brasil o Branco
godot-schema-largura_da_faixa = ancho del carril
godot-schema-parada_no_cruzamento = parada en el cruce
godot-schema-cor_invalida_use_rrggbb = color no válido: { $t } (use #rrggbb)
godot-schema-cor_rrggbb = color: #rrggbb
godot-schema-run_nao_tem_o_campo = run no tiene el campo "{ $path }"
godot-schema-regra_esperada = se esperaba una regla
godot-schema-lista_de_pesos_esperada = se esperaba una lista de pesos
godot-schema-pesos_invalidos = pesos no válidos: { $e }
godot-schema-campo_desconhecido_output = campo desconocido: output.{ $rest }
godot-schema-a_escala_precisa_ser_maior = la escala debe ser mayor que cero
godot-schema-giro_um_numero_de_graus = giro: un número de grados
godot-schema-campo_desconhecido_visual = campo desconocido: visual.{ $rest }

godot-ui-bar-casa = Casa

## Ronda 14: mensajes y paneles del GDScript (`UtsimDocument.texto`)

godot-gd-obstaculos-visiveis = Obstáculos y holguras visibles.
godot-gd-obstaculos-ocultos = Obstáculos y holguras ocultos.
godot-gd-desempenho-visiveis = Medidas de desempeño visibles (F3).
godot-gd-desempenho-ocultas = Medidas de desempeño ocultas (F3).
godot-gd-mao-dupla = doble sentido
godot-gd-mao-unica = sentido único
godot-gd-mao-unica-setas = sentido único (flechas en el piso)
godot-gd-do-modelo = el del modelo
godot-gd-foi-cancelada = fue cancelada
godot-gd-falhou = falló
godot-gd-ligada = conectada
godot-gd-desligada = desconectada
godot-gd-ligado = activado
godot-gd-desligado = desactivado
godot-gd-abrindo-leituras = Abriendo las lecturas de { $p1 } en UTSim Estadística.
godot-gd-valores-lidos-de = Valores leídos de { $p1 }.
godot-gd-nao-leu = No leyó { $p1 }: { $p2 }
godot-gd-objetos-copiados =
    { $p1 } { $p1 ->
        [one] objeto copiado
       *[other] objetos copiados
    }.
godot-gd-nada-para-copiar = Nada seleccionado para copiar.
godot-gd-removida-desfaz = Quitada: { $p1 } (Ctrl+Z deshace)
godot-gd-nao-removeu-msg = No quitó: { $p1 }
godot-gd-nao-ligou-msg = No conectó: { $p1 }
godot-gd-passagem-cabem = , { $p1 } s de paso, caben { $p2 }
godot-gd-esteira-desenhada = Cinta dibujada
godot-gd-esteira-com-canaletas = Cinta con canaletas
godot-gd-caminho-no-piso = Camino en el piso
godot-gd-pedestres = Peatones
godot-gd-empilhadeiras = Montacargas
godot-gd-misto = Mixto
godot-gd-trajeto-inclinacao-pct = Inclinación (%; negativa baja)
godot-gd-trajeto-acima-maximo = • Por encima del máximo: la pieza se desliza o vuelca.
godot-gd-trajeto-rampa-ingreme = • Rampa más empinada que 1:12 (8,33 %).
godot-gd-esteira-simples = Cinta simple
godot-gd-motorizada = Motorizada
godot-gd-roletes-gravidade = Rodillos por gravedad
godot-gd-calha = Canal (desliza)
godot-gd-trajeto-caminho-n = Camino { $p1 }
godot-gd-trajeto-esteira-n = Cinta { $p1 }
godot-gd-trajeto-no-n = Nodo { $p1 }

godot-gd-corredores-corredor_cancelado_clique_no_chao = Pasillo cancelado. Haga clic en el piso para empezar otro, o Esc para salir de la herramienta.
godot-gd-corredores-nao_criou_o_corredor = No creó el pasillo: { $p1 }
godot-gd-corredores-corredor_criado_m = Pasillo { $p1 } creado ({ $p2 }, { $p3 } m).
godot-gd-corredores-vertice_de_movido = Vértice { $p1 } de { $p2 } movido.
godot-gd-corredores-nao_moveu = No movió: { $p1 }
godot-gd-corredores-vertice_acrescentado_a = Vértice agregado a { $p1 }.
godot-gd-corredores-nao_acrescentou = No agregó: { $p1 }
godot-gd-corredores-corredor_excluido = Pasillo { $p1 } eliminado.
godot-gd-corredores-nao_excluiu = No eliminó: { $p1 }
godot-gd-corredores-ponto_de_acesso_de_movido = Punto de acceso de { $p1 } movido (desactive "Punto de acceso propio" en el panel para volver al estándar).
godot-gd-corredores-nao_moveu_o_ponto_de = No movió el punto de acceso: { $p1 }
godot-gd-corredores-corredor_agora_e_de = El pasillo { $p1 } ahora es de { $p2 }.
godot-gd-corredores-corredor_com_m_de_largura = Pasillo { $p1 } con { $p2 } m de ancho.
godot-gd-corredores-nao_mudou_a_largura = No cambió el ancho: { $p1 }
godot-gd-corredores-corredor_em = Pasillo { $p1 } en { $p2 }.
godot-gd-corredores-perfil_do_corredor = Perfil del pasillo { $p1 }: { $p2 }.
godot-gd-corredores-sentido_do_corredor_invertido = Sentido del pasillo { $p1 } invertido.
godot-gd-corredores-corredor_agora_se_chama = El pasillo { $p1 } ahora se llama { $p2 }.
godot-gd-corredores-nao_renomeou = No renombró: { $p1 }
godot-gd-corredores-vertice_removido = Vértice quitado.
godot-gd-corredores-nao_removeu = No quitó: { $p1 }
godot-gd-corredores-esse_aviso_nao_aponta_para = Este aviso no apunta a ningún pasillo ni objeto de la escena.
godot-gd-corredores-destacado_na_cena = Resaltado en la escena: { $p1 }.
godot-gd-editor-equipe_arraste_do_despachante_a = Equipo: arrastre del despachador a un transportador, o de un objeto a quien lleva los ítems (Shift: de un proceso a quien provee los operadores); de nuevo deshace. Más fácil: arrastre del nodo naranja del operador hasta la máquina. Esc cancela.
godot-gd-editor-ligar_arraste_do_bloco_de = Conectar: arrastre del bloque de origen al destino, o haga clic en uno y luego en el otro (Esc cancela).
godot-gd-editor-clique_no_chao_para_colocar = Haga clic en el piso para colocar (Shift mantiene la herramienta, Esc cancela).
godot-gd-editor-corredor_clique_no_chao_para = Pasillo: haga clic en el piso para poner los puntos del eje (Enter o doble clic termina), haga clic en un pasillo para editarlo, arrastre un vértice para moverlo; Esc cancela.
godot-gd-editor-zona_clique_no_chao_para = Zona: haga clic en el piso para poner las esquinas de la zona de riesgo; el primer punto, Enter o doble clic la cierra; haga clic en una zona para editarla. Esc cancela.
godot-gd-editor-rede_clique_no_chao_para = Red: haga clic en el piso para poner nodos (cada clic continúa el camino), haga clic en un tramo para editarlo, arrastre un nodo para moverlo; Esc termina el camino.
godot-gd-editor-nao_mudou_a_zona = No cambió la zona: { $p1 }
godot-gd-editor-nao_ligou = No conectó: { $p1 }
godot-gd-editor-nao_removeu = No quitó: { $p1 }
godot-gd-editor-nao_girou = No giró: { $p1 }
godot-gd-editor-nao_criou_a_zona = No creó la zona: { $p1 }
godot-gd-editor-zona_criada_escolha_o_tipo = Zona { $p1 } creada: elija el tipo, el nivel y el EPP en el panel (Ctrl+Z deshace).
godot-gd-editor-zona_excluida_ctrl_z_desfaz = Zona { $p1 } eliminada (Ctrl+Z deshace).
godot-gd-editor-nao_excluiu_a_zona = No eliminó la zona: { $p1 }
godot-gd-editor-objeto_s_excluidos =
    { $p1 } { $p1 ->
        [one] objeto eliminado
       *[other] objetos eliminados
    }.
godot-gd-editor-colocacao_cancelada_solte_o_objeto = Colocación cancelada: suelte el objeto sobre el piso.
godot-gd-editor-aponte_para_o_chao_para = Apunte al piso para colocar el objeto.
godot-gd-editor-nao_foi_possivel_colocar = No fue posible colocar: { $p1 }
godot-gd-editor-colocado = { $p1 } colocado.
godot-gd-editor-equipe_cancelado = Equipo: cancelado.
godot-gd-editor-nada_selecionado_para_girar = Nada seleccionado para girar.
godot-gd-editor-girado_ctrl_z_desfaz = { $p1 } girado (Ctrl+Z deshace).
godot-gd-editor-objetos_girados_cada_um_em = { $p1 } objetos girados { $p2 }, cada uno alrededor de su propio centro (Ctrl+Z deshace).
godot-gd-editor-nenhum_corredor_selecionado_para_excluir = Ningún pasillo seleccionado para eliminar.
godot-gd-editor-nenhum_no_selecionado_para_remover = Ningún nodo seleccionado para quitar.
godot-gd-principal-arraste_um_objeto_da_biblioteca = Arrastre un objeto de la Biblioteca al piso, o abra un modelo (Ctrl+O).
godot-gd-principal-encaixe_na_grade_passo_m = Ajuste a la grilla { $p1 } (paso { $p2 } m).
godot-gd-principal-o_modo_missao_do_utsim = El modo misión de UTSim Enseñanza no está en esta versión.
godot-gd-principal-para_andar_pela_fabrica_simule = Para recorrer la fábrica, simule primero (F en Simular, después de la ronda).
godot-gd-principal-alterado = { $p1 }: { $p2 } modificado.
godot-gd-principal-nao_alterou = No modificó { $p1 }: { $p2 }
godot-gd-principal-novo_modelo = Nuevo modelo.
godot-gd-principal-nao_abriu = No abrió { $p1 }: { $p2 }
godot-gd-principal-aberto_mas_salvar_fica_para = { $p1 } abierto, pero { $p2 }: Guardar queda para cuando la persona cierre (o use Guardar como).
godot-gd-principal-aberto_objetos_simular_roda_as = { $p1 } abierto: { $p2 } objetos. Simular ejecuta las replicaciones.
godot-gd-principal-aberto_com_problema_s_veja =
    { $p1 } abierto con { $p2 } { $p2 ->
        [one] problema
       *[other] problemas
    }: vea el panel de propiedades.
godot-gd-principal-o_outro_app_fechou_o = La otra app cerró el modelo: las pestañas Calidad, Seguridad y Mantenimiento volvieron a editar.
godot-gd-principal-outro_app_gravou_e_ha = Otra app guardó { $p1 } y hay cambios suyos sin guardar: use Guardar como para no borrar el trabajo de ella.
godot-gd-principal-nao_releu = No releyó { $p1 }: { $p2 }
godot-gd-principal-foi_gravado_por_outro_app = { $p1 } fue guardado por otra app: recargado desde el disco.
godot-gd-principal-salve_o_modelo_antes_de = Guarde el modelo antes de abrir en { $p1 }.
godot-gd-principal-abrindo_no_ele_edita_o = Abriendo { $p1 } en { $p2 }: este edita el modelo; UTSim 3D relee lo que guarde.
godot-gd-principal-salvo_em = Guardado en { $p1 }.
godot-gd-principal-nao_salvou = No guardó: { $p1 }
godot-gd-principal-desfeito = Deshecho.
godot-gd-principal-nada_para_desfazer = Nada para deshacer.
godot-gd-principal-refeito = Rehecho.
godot-gd-principal-nada_para_refazer = Nada para rehacer.
godot-gd-principal-nao_foi_possivel_desenhar_o = No fue posible dibujar el modelo: { $p1 }
godot-gd-principal-escolha_o_arquivo_de_dados = Elija el archivo de datos.
godot-gd-principal-valores_colados = Valores pegados: { $p1 }
godot-gd-principal-aguarde = Espere: { $p1 }.
godot-gd-principal-nao_ajustou = No ajustó: { $p1 }
godot-gd-principal-agora_usa_dados_reais_ctrl = { $p1 } ahora usa datos reales (Ctrl+Z deshace).
godot-gd-principal-nao_aplicou_os_dados = No aplicó los datos: { $p1 }
godot-gd-principal-volta_a_usar_uma_distribuicao = { $p1 } vuelve a usar una distribución (Ctrl+Z deshace).
godot-gd-principal-nao_voltou_para_a_distribuicao = No volvió a la distribución: { $p1 }
godot-gd-principal-sem_relatorio = Sin informe: { $p1 }
godot-gd-principal-relatorio_gravado_em = Informe guardado en { $p1 }.
godot-gd-principal-relatorio_gravado_em_arquivos = Informe guardado en { $p1 } ({ $p2 } archivos).
godot-gd-principal-objetos_alinhados = { $p1 } objetos alineados.
godot-gd-principal-objetos_distribuidos = { $p1 } objetos distribuidos.
godot-gd-principal-objeto_s_recortados_ctrl_v =
    { $p1 } { $p1 ->
        [one] objeto recortado
       *[other] objetos recortados
    } (Ctrl+V pega).
godot-gd-principal-nada_selecionado_para_recortar = Nada seleccionado para recortar.
godot-gd-principal-objeto_s_colados =
    { $p1 } { $p1 ->
        [one] objeto pegado
       *[other] objetos pegados
    }: { $p2 }.
godot-gd-principal-o_modelo_tem_problema_s =
    El modelo tiene { $p1 } { $p1 ->
        [one] problema y
       *[other] problemas y
    } no corre: { $p2 }
godot-gd-principal-a_simulacao_nao_comecou = La simulación no comenzó: { $p1 }
godot-gd-principal-a_simulacao = La simulación { $p1 }: { $p2 }
godot-gd-principal-nao_foi_possivel_gravar_a = No fue posible guardar la replicación 0: { $p1 }
godot-gd-principal-replicacoes_concluidas_reproduzindo_a_replicacao = { $p1 } replicaciones concluidas. Reproduciendo la replicación 0.
godot-gd-principal-texto = { $p1 }...
godot-gd-principal-cancelando = Cancelando: { $p1 }...
godot-gd-principal-simule_o_modelo_para_ver = Simule el modelo para ver los resultados (Tab o el botón Simular).
godot-gd-principal-os_fluxos_de_tokens_aparecem = Los flujos de tokens aparecen en Simular.
godot-gd-principal-selecione_os_objetos_da_parte = Seleccione los objetos de la parte del proceso (o dos: el inicio y el fin) y pida el PERT/CPM.
godot-gd-principal-pert_cpm_de_ate_selecione = PERT/CPM de... hasta...: seleccione exactamente dos objetos.
godot-gd-principal-o_modelo_tem_problemas_e = El modelo tiene problemas y no corre: el PERT/CPM necesita un modelo válido.
godot-gd-principal-nao_comecou = No comenzó: { $p1 }
godot-gd-principal-um_replay_gravado_nao_roda = Un replay guardado no se ejecuta de nuevo: abra el modelo.
godot-gd-principal-nao_ligou_a_serie_e = No conectó la serie y los histogramas: { $p1 }
godot-gd-principal-ligado_no_modelo_ctrl_z = Activado en el modelo ({ $p1 }; Ctrl+Z deshace). Simulando de nuevo...
godot-gd-principal-aquecimento_aplicado_ao_modelo_run = Calentamiento { $p1 } aplicado al modelo (run.warmup). Simule de nuevo para los números con él.
godot-gd-principal-nao_aplicou_o_aquecimento = No aplicó el calentamiento: { $p1 }
godot-gd-principal-nao_abriu_pedido_na_linha = No abrió { $p1 } (pedido en la línea de comandos).
godot-gd-principal-nao_foi_possivel_reproduzir = No fue posible reproducir { $p1 }: { $p2 }
godot-gd-principal-replay_aberto_blocos_so_reproducao = Replay { $p1 } abierto ({ $p2 } bloques): solo reproducción; Editar vuelve al modelo en edición.
godot-gd-principal-simule_um_modelo_antes_de = Simule un modelo antes de guardar el replay.
godot-gd-principal-replay_da_replicacao_0_gravado = Replay de la replicación 0 guardado en { $p1 }.
godot-gd-principal-nao_gravou_o_replay = No guardó el replay: { $p1 }
godot-gd-principal-camera_de_volta = Cámara de vuelta.
godot-gd-rede-caminho_encerrado_clique_no_chao = Camino terminado. Haga clic en el piso para empezar otro, o Esc para salir de la herramienta.
godot-gd-rede-nao_colocou_o_no = No colocó el nodo: { $p1 }
godot-gd-rede-no_ligado_a_clique_de = Nodo { $p1 } conectado a { $p2 }. Haga clic de nuevo para continuar el camino (Esc para).
godot-gd-rede-no_colocado_clique_em_outro = Nodo { $p1 } colocado. Haga clic en otro punto para trazar un tramo (Esc para).
godot-gd-rede-no_movido = Nodo { $p1 } movido.
godot-gd-rede-nao_moveu = No movió: { $p1 }
godot-gd-rede-trecho_criado = Tramo { $p1 } → { $p2 } creado ({ $p3 }).
godot-gd-rede-nao_ligou = No conectó: { $p1 }
godot-gd-rede-trecho_removido = Tramo { $p1 } → { $p2 } quitado.
godot-gd-rede-no_removido_com_os_trechos = Nodo { $p1 } quitado con sus tramos.
godot-gd-rede-trecho_em = Tramo { $p1 } → { $p2 } en { $p3 }.
godot-gd-rede-pista_controlada_ate_veiculo_s =
    Carril controlado: hasta { $p1 } { $p1 ->
        [one] vehículo
       *[other] vehículos
    } a la vez en el tramo.
godot-gd-rede-trecho_sem_controle_de_trafego = Tramo sin control de tráfico.
godot-gd-rede-trecho_invertido = Tramo invertido: { $p1 } → { $p2 }.
godot-gd-rede-no_agora_se_chama = El nodo { $p1 } ahora se llama { $p2 }.
godot-gd-rede-nao_renomeou = No renombró: { $p1 }
godot-gd-trajeto-clique_no_chao_para_por = { $p1 }: haga clic en el piso para poner los nodos (Enter o doble clic termina). PageUp/PageDown sube y baja el próximo nodo, [ y ] redondean la curva. Haga clic en un nodo para editarlo; Esc cancela.
godot-gd-trajeto-arraste_em_volta_do_meio = Arrastre alrededor del centro para girar { $p1 } (de 15 en 15°; Shift: de 1 en 1°).
godot-gd-trajeto-arraste_para_mover_inteiro_r = Arrastre para mover { $p1 } entero; R y Shift+R giran 15°.
godot-gd-trajeto-arraste_para_curvar_o_trecho = Arrastre para curvar el tramo { $p1 } de { $p2 }.
godot-gd-trajeto-no_acrescentado_em_o_trecho = Nodo agregado en { $p1 } (el tramo fue dividido).
godot-gd-trajeto-nao_acrescentou = No agregó: { $p1 }
godot-gd-trajeto-aponte_para_o_chao_para = Apunte al piso para poner el nodo.
godot-gd-trajeto-no_em_m_altura_m = { $p1 }: nodo { $p2 } en ({ $p3 }; { $p4 }) m, altura { $p5 } m ({ $p6 }).
godot-gd-trajeto-comeca_em_m_altura_m = { $p1 }: comienza en ({ $p2 }; { $p3 }) m, altura { $p4 } m. Haga clic en los próximos nodos; Enter termina.
godot-gd-trajeto-trecho_de_selecionado_apagar_o = Tramo { $p1 } de { $p2 } seleccionado: "Borrar el tramo entre los nodos" en el panel; arrastre el rombo para curvarlo.
godot-gd-trajeto-movido_m_em_x_e = { $p1 } movido { $p2 } m en X y { $p3 } m en Y (Ctrl+Z deshace).
godot-gd-trajeto-nao_moveu = No movió: { $p1 }
godot-gd-trajeto-trecho_curvado_no_com_raio = Tramo curvado: nodo { $p1 } con radio de { $p2 } m (Ctrl+Z deshace).
godot-gd-trajeto-nao_curvou = No curvó: { $p1 }
godot-gd-trajeto-comprimento_do_trecho_m_enter = Longitud del tramo: { $p1 } m (Enter pone el nodo en la dirección del cursor, Backspace borra).
godot-gd-trajeto-proximo_no_a_m_de = Próximo nodo a { $p1 } m de altura.
godot-gd-trajeto-raio_das_proximas_curvas_m = Radio de las próximas curvas: { $p1 } m.
godot-gd-trajeto-descendo_por_m_o_no = Bajando { $p1 } % por { $p2 } m, el nodo quedaría debajo del piso.
godot-gd-trajeto-o_trecho_de_m_nao = El tramo de { $p1 } m no alcanza la altura de { $p2 } m.
godot-gd-trajeto-no_a_m_do_anterior = { $p1 }: nodo { $p2 } a { $p3 } m del anterior.
godot-gd-trajeto-girado_r_gira_de_novo = { $p1 } girado { $p2 }° (R gira de nuevo; Shift+R para el otro lado).
godot-gd-trajeto-nao_girou = No giró: { $p1 }
godot-gd-trajeto-ultimo_no_desfeito = Último nodo deshecho.
godot-gd-trajeto-nao_criou = No creó: { $p1 }
godot-gd-trajeto-criado_m = { $p1 } { $p2 } creado: { $p3 } m{ $p4 }.
godot-gd-trajeto-desenho_cancelado_clique_no_chao = Dibujo cancelado. Haga clic en el piso para empezar otro, o Esc para salir de la herramienta.
godot-gd-trajeto-no_s_removido_s =
    { $p1 } { $p1 ->
        [one] nodo eliminado
       *[other] nodos eliminados
    }.
godot-gd-trajeto-nao_removeu = No quitó: { $p1 }
godot-gd-trajeto-no_x_y_altura_m = { $p1 }, nodo { $p2 }: X { $p3 }, Y { $p4 }, altura { $p5 } m. Flechas y PageUp/PageDown ajustan (Shift: 1 cm).
godot-gd-trajeto-nao_e_uma_esteira_desenhada = { $p1 } no es una cinta dibujada.
godot-gd-trajeto-esteira_clique_num_no_para = Cinta { $p1 }: haga clic en un nodo para editarlo; flechas y PageUp/PageDown mueven los nodos seleccionados.
godot-gd-trajeto-inclinacao_fixa = Inclinación fija { $p1 } ({ $p2 } %).
godot-gd-trajeto-esteira_dividida = Cinta dividida: { $p1 } → { $p2 }.
godot-gd-trajeto-trecho_apagado_de = Tramo borrado de { $p1 }.
godot-gd-trajeto-e_viraram_um_trajeto_so = { $p1 } y { $p2 } se volvieron un solo trayecto (Ctrl+Z deshace).
godot-gd-trajeto-agora_corre_no_sentido_contrario = { $p1 } ahora corre en sentido contrario (las conexiones también).
godot-gd-trajeto-desenhado_a_m_de = { $p1 } dibujado a { $p2 } m de { $p3 }.
godot-gd-trajeto-rampa_s_de_acesso_de =
    { $p1 } { $p1 ->
        [one] rampa de acceso de 1:12 añadida
       *[other] rampas de acceso de 1:12 añadidas
    } a { $p2 }.
godot-gd-trajeto-nao_mudou = No cambió: { $p1 }
godot-gd-trajeto-no_a_m_de_altura = { $p1 }, nodo { $p2 } a { $p3 } m de altura (perfil; Ctrl+Z deshace).
godot-gd-trajeto-nao_mudou_a_altura = No cambió la altura: { $p1 }

godot-gd-trajeto-clique_no_chao_para_por-2 = Haga clic en el piso para poner los nodos; el tramo muestra la longitud y la inclinación. Los nodos encajan en las puertas (puntas de las cintas y objetos), en los nodos de otros trayectos, de 15 en 15° y en la grilla. PageUp/PageDown: altura del próximo nodo; [ y ]: radio de la curva; Enter termina; Backspace vuelve un nodo; Esc cancela.
godot-gd-trajeto-altura_do_proximo_no_m = Altura del próximo nodo (m)
godot-gd-trajeto-inclinacao_fixa_no_proximo_trecho = Inclinación fija en el próximo tramo
godot-gd-trajeto-raio_das_curvas_m_0 = Radio de las curvas (m; 0 = esquina)
godot-gd-trajeto-largura_do_caminho_m = Ancho del camino (m)
godot-gd-trajeto-esteira_s_desenhada_s_caminho =
    { $p1 } { $p1 ->
        [one] cinta dibujada
       *[other] cintas dibujadas
    }, { $p2 } { $p2 ->
        [one] camino
       *[other] caminos
    }.
godot-gd-trajeto-maior_inclinacao_maximo = Mayor inclinación { $p1 }° (máximo { $p2 }°).
godot-gd-trajeto-girar_graus_anti_horario = Girar (grados; + antihorario)
godot-gd-trajeto-girar_o_trajeto = Girar el trayecto
godot-gd-trajeto-girar_15_anti_horario_r = Girar 15° (antihorario, R)
godot-gd-trajeto-girar_15_horario_shift_r = Girar 15° (horario, Shift+R)
godot-gd-trajeto-juntar_com_num_trajeto_so = Unir con { $p1 } en un solo trayecto
godot-gd-trajeto-paralelo_a_m_a_esquerda = Paralelo a (m; + a la izquierda)
godot-gd-trajeto-desenhar_paralelo = Dibujar paralelo
godot-gd-trajeto-escada_degraus_so_pedestres = Escalera (peldaños; solo peatones)
godot-gd-trajeto-rampas_de_acesso_nas_pontas = Rampas de acceso en las puntas (1:12)
godot-gd-trajeto-tipo = Tipo
godot-gd-trajeto-acionamento = Accionamiento
godot-gd-trajeto-inverter_o_sentido = Invertir el sentido
godot-gd-trajeto-largura_m = Ancho (m)
godot-gd-trajeto-altura_m = Altura (m)
godot-gd-trajeto-raio_da_curva_m_0 = Radio de la curva (m; 0 = esquina)
godot-gd-trajeto-dividir_a_esteira_neste_no = Dividir la cinta en este nodo
godot-gd-trajeto-nos_selecionados = { $p1 } nodos seleccionados
godot-gd-trajeto-nivelar_na_altura_m = Nivelar a la altura (m)
godot-gd-trajeto-inclinacao_constante_entre_as_pontas = Inclinación constante entre las puntas
godot-gd-trajeto-remover_no_s_del = Quitar nodo(s) (Del)
godot-gd-trajeto-apagar_o_trecho_entre_os = Borrar el tramo entre los nodos
godot-gd-trajeto-arraste_o_losango_do_no = Arrastre el rombo del nodo en el piso para moverlo y la cima del poste para cambiar la altura. Las flechas mueven (Shift: 1 cm), PageUp/PageDown suben y bajan, [ y ] cambian el radio. Shift + clic en la línea agrega un nodo; un clic en el rombo naranja (o dos nodos vecinos con Shift + clic) selecciona el tramo, que puede borrarse. Arrastre el rombo naranja en medio de un tramo para curvarlo, el cuadrado verde en medio del trayecto para moverlo entero y el anillo naranja a su lado para girarlo; sin nodo seleccionado, las flechas mueven y R gira el trayecto.
godot-gd-principal-o_modelo_tem_mudancas_que = El modelo tiene cambios sin guardar. ¿Descartar?

## Ronda 14: ventanas y errores restantes

godot-ui-class-sem_replicacao = sin replicación
godot-ui-class-sem_rastro_da_replicacao = sin rastro de la replicación
godot-ui-class-a_janela_de_valores_nao = la ventana de valores no está abierta
godot-ui-class-valor_es_lidos_de_coluna =
    { $n } { $n ->
        [one] valor leído
       *[other] valores leídos
    } de { $file } (columna { $column }).
godot-ui-class-nao_leu_o_arquivo = No leyó el archivo: { $e }
godot-ui-class-relatorio_gravado_em = Informe guardado en { $f }.
godot-ui-class-escreva_o_arquivo_do_relatorio = Escriba el archivo del informe.
godot-ui-class-relatorio_pert_cpm_gravado_em = Informe PERT/CPM guardado en { $path }.
godot-ui-class-nao_gravou = No guardó { $path }: { $e }
godot-ui-class-tabela_s_csv_em =
    { $files } { $files ->
        [one] tabla
       *[other] tablas
    } CSV en { $dir }.
godot-ui-class-nao_gravou_os_csv_em = No guardó los CSV en { $dir }: { $e }
godot-ui-class-escreva_a_pasta_da_exportacao = Escriba la carpeta de la exportación.
godot-ui-class-arquivo_s_csv_em =
    { $files } { $files ->
        [one] archivo
       *[other] archivos
    } CSV en { $dir }.
godot-ui-class-nao_exportou = No exportó: { $e }
godot-ui-class-so_leitura = { $quem }: solo lectura
godot-ui-class-na_ultima_simulacao_media_semiamplitude =
    En la última simulación ({ $replicacoes } { $replicacoes ->
        [one] replicación
       *[other] replicaciones
    }), media ± semiamplitud del IC de 95 %:
godot-schema-inteiro_maior_ou_igual_a_zero = { $key } debe ser un entero >= 0
godot-schema-numero_maior_ou_igual_a_zero = { $w }: número mayor o igual a cero
godot-schema-nao_tem_o_campo = { $name } no tiene el campo "{ $field }"
godot-schema-campo_maior_que_zero = { $field }: debe ser mayor que cero
godot-schema-inteiro_minimo = { $what } debe ser un entero >= { $min }
godot-schema-numero_esperado_de = { $what }: se esperaba un número
godot-schema-verdadeiro_ou_falso_esperado = { $what }: se esperaba verdadero o falso

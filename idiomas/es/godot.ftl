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

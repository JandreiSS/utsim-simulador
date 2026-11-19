# UTSim: el editor del simulador (base: barra, capas, paneles y diálogos generales).

ui-sem_observacoes = sin observaciones
ui-resumo = n={ $n }  media={ $media }  de={ $dp }  mín={ $min }  máx={ $max }
ui-recebidos = recibidos: { $n }
ui-simulacao = Simulación
ui-modelo_replicacao = { $modelo } - réplica { $n }
ui-contagem = { $blocos ->
    [one] { $blocos } bloque
   *[other] { $blocos } bloques
}, { $itens ->
    [one] { $itens } ítem
   *[other] { $itens } ítems
}, { $eventos ->
    [one] { $eventos } evento de rastro
   *[other] { $eventos } eventos de rastro
}
ui-pausar = Pausar
ui-reproduzir = Reproducir
ui-evento_anterior = < Evento
ui-evento_seguinte = Evento >
ui-inicio = Inicio
ui-velocidade = velocidad x{ $v }
ui-unidades_por_segundo = unidades de tiempo simulado por segundo
ui-tempo = tiempo
ui-tempo_simulado = tiempo simulado: { $t } de { $fim }
ui-ajuda_teclas = Espacio: reproducir/pausar | +/-: velocidad | flechas: evento anterior/siguiente | F: encuadrar
ui-ajuda_mouse = Ratón: el izquierdo arrastra (mueve) y hace clic (selecciona), el derecho gira, la rueda acerca
ui-ajuda_camera = Cámara: 1 órbita, 2 seguir, 3 primera persona, 4 desde el recurso | F3: tiempos de cuadro
ui-blocos = Bloques
ui-clique_em_um_bloco = Haga clic en un bloque para ver sus estadísticas.
ui-capacidade_rotulo = capacidad { $n }
ui-nivel_agora = nivel ahora
ui-capacidade = capacidad
ui-ocupacao = ocupación
ui-taxa_agora = tasa ahora
ui-nivel_inicial = nivel inicial
ui-entrada_maxima = entrada máxima
ui-saida_maxima = salida máxima
ui-vazao_da_fonte = caudal de la fuente
ui-volume_por_item = volumen por ítem
ui-acumulado = acumulado para el próximo ítem
ui-vazao_agora = caudal ahora
ui-vazao_maxima = caudal máximo
ui-objeto = objeto
ui-por_unidade_de_tempo = { $taxa } por unidad de tiempo ({ $sentido })
ui-nivel_interpolado = Nivel y tasa interpolados a partir del rastro de la red de fluidos.
ui-seguir_recurso = Seguir este recurso
ui-seguir_recurso_dica = La cámara sigue al recurso por detrás y por encima (2)
ui-ver_do_recurso = Ver desde el recurso
ui-ver_do_recurso_dica = La cámara va a la posición del conductor (4)
ui-recebidos_ate_agora = recibidos hasta ahora
ui-estado_agora = estado ahora
ui-conteudo_agora = contenido ahora
ui-entradas = entradas
ui-saidas = salidas
ui-conteudo_medio = contenido medio
ui-conteudo_maximo = contenido máximo
ui-tempo_de_permanencia = tiempo de permanencia
ui-fracao_em = fracción en { $estado }
ui-estatisticas_finais = Estadísticas finales de la réplica (ventana después del calentamiento).
camadas-no_excluido = Nodo { $nome } y sus aristas eliminados (Ctrl+Z deshace)
camadas-aresta_excluida = Arista { $de } - { $para } eliminada (Ctrl+Z deshace)
camadas-no_fluido_excluido = Nodo { $nome } y sus conexiones eliminados (Ctrl+Z deshace)
camadas-objetos_dica = Objetos del modelo: biblioteca, conexiones y propiedades
camadas-rede_dica = Red de caminos de los recursos móviles: nodos y aristas
camadas-fluidos_dica = Red de fluidos: fuentes, tanques, tubos y convertidores
camada-objetos = Objetos
camada-rede = Red
camada-fluidos = Fluidos
camadas-rede_de_caminhos = Red de caminos
camadas-selecionar = Seleccionar
camadas-selecionar_rede_dica = El clic selecciona un nodo o una arista; arrastrar mueve el nodo
camadas-no = Nodo
camadas-no_dica = Un clic en el suelo coloca un nodo
camadas-aresta = Arista
camadas-aresta_dica = Haga clic en un nodo y luego en otro; un clic en el suelo crea un nodo ya conectado
camadas-mao_dupla = Aristas nuevas de doble sentido
camadas-ajuda_aresta = Arista: haga clic de nodo en nodo para dibujar el camino; Esc termina.
camadas-ajuda_del_rede = Supr elimina el nodo (con sus aristas) o la arista seleccionada.
camadas-ajuda_atendido = Cada objeto atendido entra en la red por el nodo más cercano (línea discontinua).
camadas-rede_de_fluidos = Red de fluidos
camadas-selecionar_fluidos_dica = El clic selecciona un nodo; arrastrar mueve el nodo
camadas-ligar = Conectar
camadas-ligar_dica = Haga clic en el nodo de origen y luego en el de destino (el caudal sigue la flecha)
camadas-colocar = Colocar
camadas-colocar_dica = Un clic en el suelo coloca el nodo (Shift mantiene la herramienta)
camadas-ajuda_aciclica = La red debe ser acíclica. Los convertidores conectan la red a un objeto.
camadas-ajuda_del_fluidos = Supr elimina el nodo seleccionado y sus conexiones.
camadas-aresta_daqui = Arista desde aquí
camadas-ligar_daqui = Conectar desde aquí
camadas-sem_rede = El modelo no tiene red de caminos.
camadas-sem_rede_ajuda = Sin red, los recursos móviles se mueven en línea recta. Use "Nodo" para colocar los nodos y "Arista" para conectarlos.
camadas-resumo_rede = { $nos ->
    [one] { $nos } nodo
   *[other] { $nos } nodos
} y { $arestas ->
    [one] { $arestas } arista
   *[other] { $arestas } aristas
}. Seleccione uno para editar.
camadas-nos = Nodos ({ $n })
camadas-arestas = Aristas ({ $n })
camadas-sem_fluidos = El modelo no tiene red de fluidos.
camadas-sem_fluidos_ajuda = Elija un tipo en "Colocar" y haga clic en el suelo; después use "Conectar".
camadas-resumo_fluidos = Red "{ $rede }": { $nos ->
    [one] { $nos } nodo
   *[other] { $nos } nodos
} y { $ligacoes ->
    [one] { $ligacoes } conexión
   *[other] { $ligacoes } conexiones
}. Seleccione un nodo para editar.
editor-layout_aplicado = Diseño automático aplicado: guarde para mantener las posiciones
editor-layout_com_rede = Modelo con red de caminos: coloque los objetos en los nodos de la red (no se aplicó el diseño automático)
editor-sem_titulo = sin título.ron
editor-excluidos = { $n } objetos eliminados (Ctrl+Z deshace)
editor-nada_para_copiar = Nada seleccionado para copiar
editor-copiados = { $objetos ->
    [one] { $objetos } objeto
   *[other] { $objetos } objetos
} y { $ligacoes ->
    [one] { $ligacoes } conexión copiados
   *[other] { $ligacoes } conexiones copiados
}
editor-recortados = { $n ->
    [one] { $n } objeto cortado (Ctrl+V pega)
   *[other] { $n } objetos cortados (Ctrl+V pega)
}
editor-area_vazia = El portapapeles está vacío (Ctrl+C copia)
editor-colados = { $n ->
    [one] { $n } objeto pegado: { $nomes }
   *[other] { $n } objetos pegados: { $nomes }
}
editor-nada_para_duplicar = Nada seleccionado para duplicar
editor-salvo_em = Guardado en { $arquivo }
editor-erro_ao_salvar = Error al guardar: { $erro }
editor-tempo_invalido = Tiempo máximo no válido: use un número mayor que cero
editor-simulando = Simulando...
editor-sem_resultado = la simulación terminó sin resultado
editor-erro_na_simulacao = Error en la simulación: { $erro }
editor-na_caixa = { $n ->
    [one] { $n } objeto en la caja
   *[other] { $n } objetos en la caja
}, { $sel ->
    [one] { $sel } seleccionado
   *[other] { $sel } seleccionados
}
editor-outro_destino = Elija otro bloque como destino
editor-ligacao_removida = Conexión { $de } -> { $para } eliminada (Ctrl+Z deshace)
editor-editar = Editar
editor-simular = ▶ Simular
editor-ate_t = hasta t =
editor-novo = Nuevo
editor-salvar = Guardar
editor-salvar_como_botao = Guardar como...
editor-desfazer = Deshacer
editor-refazer = Rehacer
editor-enquadrar = Encuadrar (F)
editor-fluxos = Flujos
editor-fluxos_dica = Lógica del usuario: flujos de tokens
editor-resultados = Resultados
editor-resultados_dica = Réplicas, resumen con intervalos de confianza, gráficos, escenarios y optimización
editor-atalhos = Atajos
editor-atalhos_dica = Atajos de teclado y ratón (F1)
editor-salvar_como = Guardar como
editor-caminho_do_arquivo = Ruta del archivo (.ron), relativa a la carpeta del programa o completa:
editor-sobrescrito = Ese archivo ya existe y se sobrescribirá.
editor-cancelar = Cancelar
editor-novo_modelo = Modelo nuevo
editor-descartar_pergunta = Hay cambios sin guardar. ¿Descartarlos y empezar un modelo nuevo?
editor-descartar = Descartar
editor-atalhos_do_editor = Atajos del editor
editor-f1_fecha = F1 o Esc cierra esta ventana.
editor-nao_simula = No es posible simular
editor-corrija = Corrija los problemas del modelo antes de simular:
editor-fechar = Cerrar
editor-valores_gravados = Valores grabados en el modelo (Ctrl+Z deshace)
editor-nenhum_problema = Ningún problema: el modelo se puede simular.
editor-modelo = Modelo
editor-biblioteca = Biblioteca
editor-propriedades = Propiedades
editor-selecione_um_bloco = Seleccione un bloque para editar sus propiedades.
editor-ligar = Conectar (C)
editor-excluir = Eliminar (Supr)
editor-clique_no_destino = Haga clic en el bloque de destino (Esc cancela).
editor-problemas = Problemas ({ $n })
editor-avisos = Avisos ({ $n })
editor-colocar_dica = { $tipo }: haga clic y luego clic en el suelo, o arrastre hasta el suelo
editor-encaixar = Ajustar a la cuadrícula
editor-passo = Paso (m)
editor-esc_cancela = Esc cancela. Shift + clic en el suelo coloca varios.
editor-atalhos_f1 = Atajos: F1.
editor-selecionados = { $n } objetos seleccionados
editor-copiar = Copiar
editor-duplicar = Duplicar
editor-excluir_todos = Eliminar todos
editor-alinhar = Alinear
editor-esquerda = Izquierda
editor-centro_x = Centro X
editor-direita = Derecha
editor-topo = Arriba
editor-centro_y = Centro Y
editor-base = Abajo
editor-distribuir_x = Distribuir en x
editor-distribuir_y = Distribuir en y
editor-distribuir_dica = Espacia por igual entre los dos extremos (3 objetos o más)
editor-propriedades_de_todos = Propiedades de todos
editor-campos_de_todos = Los campos muestran los valores de { $primeiro }; cada cambio vale para los { $n } objetos.
editor-buscar_por_nome = Buscar por nombre
editor-todos_os_tipos = Todos los tipos
editor-de_objetos = { $n } de { $total } objetos
editor-objeto_dica = { $tipo }. El clic selecciona, Shift suma a la selección, el doble clic encuadra
editor-subir = Subir
editor-descer = Bajar
editor-subir_dica = Cambia de lugar con la anterior (Ctrl+Z deshace)
editor-descer_dica = Cambia de lugar con la siguiente (Ctrl+Z deshace)
editor-remover_ligacao = Quitar conexión
editor-ligacoes = Conexiones
editor-nenhuma_ligacao = Ninguna. Use "Conectar" y haga clic en el destino.
editor-ordem_importa = El orden importa. Salidas: el orden es el número del puerto (1, 2, ...) que usan las reglas de salida. Entradas: la primera tiene prioridad cuando el objeto toma un ítem.
editor-saidas_por_porta = Salidas, por puerto:
editor-porta = puerto { $n }: { $de } -> { $para }
editor-entradas_por_prioridade = Entradas, por prioridad:
editor-entrada = { $n }.ª: { $de } -> { $para }
atalho-01-teclas = Clic
atalho-01 = selecciona el bloque (o quita la conexión pulsada)
atalho-02-teclas = Clic repetido en el mismo punto
atalho-02 = alterna entre los objetos superpuestos (una pista muestra cuál está seleccionado)
atalho-03-teclas = Shift + clic
atalho-03 = pone o quita el bloque de la selección
atalho-04-teclas = Shift + arrastrar en el suelo
atalho-04 = selección por caja (suma a la selección)
atalho-05-teclas = Arrastrar un bloque
atalho-05 = mueve el bloque o toda la selección
atalho-06-teclas = Arrastrar en el suelo
atalho-06 = desplaza la vista
atalho-07-teclas = Botón derecho + arrastrar
atalho-07 = gira la vista
atalho-08-teclas = Rueda del ratón
atalho-08 = acerca y aleja, hacia el cursor
atalho-09-teclas = Pestaña Objetos: clic, Shift + clic
atalho-09 = selecciona, suma a la selección; el doble clic encuadra el objeto
atalho-10-teclas = Biblioteca: clic y clic en el suelo
atalho-10 = coloca un objeto (Shift mantiene la herramienta)
atalho-11-teclas = Biblioteca: arrastrar hasta el suelo
atalho-11 = coloca un objeto donde se suelta
atalho-12-teclas = Flechas
atalho-12 = mueve la selección un paso de la cuadrícula
atalho-13-teclas = Supr
atalho-13 = elimina la selección
atalho-14-teclas = C
atalho-14 = conecta el bloque seleccionado con el próximo pulsado
atalho-15-teclas = Esc
atalho-15 = cancela la herramienta, cierra los diálogos y sale de las cámaras seguir y primera persona
atalho-16-teclas = F
atalho-16 = encuadra el modelo (vuelve a la cámara en órbita)
atalho-17-teclas = 1, 2, 3, 4
atalho-17 = cámara: órbita, seguir un recurso, primera persona, desde el recurso
atalho-18-teclas = W A S D
atalho-18 = camina en primera persona (Shift corre, la rueda cambia la velocidad)
atalho-19-teclas = L
atalho-19 = bloquea el ratón en primera persona
atalho-20-teclas = F1
atalho-20 = muestra u oculta esta ventana
atalho-21-teclas = F3
atalho-21 = muestra u oculta los tiempos de cuadro
atalho-22-teclas = F4
atalho-22 = activa o desactiva los niveles de detalle
atalho-23-teclas = Ctrl + A
atalho-23 = selecciona todos los objetos
atalho-24-teclas = Ctrl + C
atalho-24 = copia la selección
atalho-25-teclas = Ctrl + X
atalho-25 = corta la selección
atalho-26-teclas = Ctrl + V
atalho-26 = pega (nombres únicos, conexiones internas conservadas)
atalho-27-teclas = Ctrl + D
atalho-27 = duplica la selección
atalho-28-teclas = Ctrl + Z
atalho-28 = deshace
atalho-29-teclas = Ctrl + Y o Ctrl + Shift + Z
atalho-29 = rehace
atalho-30-teclas = Ctrl + S
atalho-30 = guarda
atalho-31-teclas = Ctrl + Shift + S
atalho-31 = guarda con otro nombre (Guardar como)
atalho-32-teclas = Ctrl + N
atalho-32 = modelo nuevo

# Parte 2: el panel de propiedades (properties/).

## Propiedades: familias de distribución, reglas y tipos

dist-constante = Constante
dist-uniforme = Uniforme
dist-exponencial = Exponencial
dist-triangular = Triangular
dist-normal = Normal
dist-lognormal = Lognormal
dist-gama = Gamma
dist-erlang = Erlang
dist-weibull = Weibull
dist-beta = Beta
dist-log-logistica = Log-logística
dist-pearson-v = Pearson V
dist-pearson-vi = Pearson VI
dist-discreta = Discreta
dist-empirica = Empírica
dist-poisson = Poisson
dist-geometrica = Geométrica
dist-binomial = Binomial
dist-binomial-negativa = Binomial negativa
dist-deslocada = Desplazada
prop-vaga-primeira-livre = Primera libre
prop-vaga-aleatoria = Aleatoria
prop-despacho-primeiro-livre = Primero libre
prop-despacho-rodizio = Rotación
prop-despacho-mais-proximo = Más cercano
tipo-chegada = Llegada
tipo-fila = Cola
tipo-processo = Proceso
tipo-montagem = Montaje
tipo-desmontagem = Desmontaje
tipo-saida = Salida
tipo-transportador = Transportador
tipo-ponte-rolante = Puente grúa
tipo-despachante = Despachador
tipo-esteira = Cinta transportadora
tipo-porta-paletes = Estantería de palés
prop-modo-igual-ao-padrao = igual al predeterminado
prop-modo-distribuicao = distribución
prop-modo-taxa = tasa
prop-modo-sem-chegadas = sin llegadas
prop-turno-pausar = Pausar
prop-turno-terminar = Terminar
prop-turno-pausar-dica = El trabajo en curso se congela al final del turno y continúa cuando vuelve el turno
prop-turno-terminar-dica = Al final del turno el objeto no acepta ítems nuevos, termina el ítem en curso (hora extra) y solo entonces sale de turno; el turno siguiente empieza a la hora de siempre

## properties/network.rs

prop-network-o_nome_nao_pode_ficar = el nombre no puede quedar vacío
prop-network-ja_existe_um_no_chamado = ya existe un nodo llamado "{ $t }"
prop-network-nome = Nombre
prop-network-no_da_rede_de_caminhos = Nodo de la red de caminos
prop-network-arestas = Aristas
prop-network-nenhuma_use_a_ferramenta_aresta = Ninguna: use la herramienta "Arista" y haga clic en otro nodo.
prop-network-remover_aresta = Quitar arista
prop-network-objetos_ligados_a_este_no = Objetos unidos a este nodo
prop-network-nenhum_cada_objeto_atendido_entra = Ninguno (cada objeto atendido entra por el nodo más cercano).
prop-network-aresta = Arista { $edge }
prop-network-comprimento_m_calculado_pelas_posicoes = Longitud: { $len } m (calculada por las posiciones de los nodos)
prop-network-mao_dupla = Doble sentido ({ $from } <-> { $to })
prop-network-mao_unica = Sentido único ({ $from } -> { $to })
prop-network-mao_unica-2 = Sentido único ({ $to } -> { $from })
prop-network-sentido = Sentido
prop-network-limitar_veiculos_agv = Limitar vehículos (AGV)
prop-network-quantos_agvs_podem_estar_na = Cuántos AGV pueden estar en la arista al mismo tiempo
prop-network-sem_limite = sin límite
prop-network-so_agvs_agv_true_respeitam = Solo los AGV (agv: true) respetan la capacidad; los demás pasan libremente.
prop-network-objeto_discreto = Objeto discreto
prop-network-escolha = (elija)
prop-network-nao_e_um_objeto_discreto = "{ $current }" no es un objeto discreto del modelo
prop-network-vazao = Caudal
prop-network-capacidade = Capacidad
prop-network-nivel_inicial = Nivel inicial
prop-network-entrada_maxima = Entrada máxima
prop-network-saida_maxima = Salida máxima
prop-network-vazao_maxima = Caudal máximo
prop-network-cada_item_que_entra_no = Cada ítem que entra en el objeto se convierte en este volumen, entregado de una vez.
prop-network-volume_por_item = Volumen por ítem
prop-network-a_cada_volume_completo_cria = Con cada volumen completo, crea un ítem dentro del objeto.
prop-network-problema = Problema: { $message }
prop-network-tipo = Tipo
prop-network-ligacoes = Conexiones
prop-network-nenhuma_use_a_ferramenta_ligar = Ninguna: use la herramienta "Conectar" y haga clic en el nodo siguiente.
prop-network-remover_ligacao = Quitar conexión

## properties/panel.rs

prop-panel-buscar = Buscar
prop-panel-texto_invalido_nao_aplicado = Texto inválido, no aplicado: { $error }
prop-panel-descartar_texto = descartar texto
prop-panel-volta_ao_valor_que_esta = Vuelve al valor que está en el modelo
prop-panel-nao_e_um_numero = "{ $s }" no es un número
prop-panel-infinita = infinita
prop-panel-valor = valor
prop-panel-peso = peso
prop-panel-remover = quitar
prop-panel-adicionar_valor = añadir valor
prop-panel-observacoes_em_ordem_crescente_separadas = Observaciones en orden creciente, separadas por espacios:
prop-panel-deslocamento = desplazamiento
prop-panel-min = mín.
prop-panel-max = máx.
prop-panel-media = media
prop-panel-moda = moda
prop-panel-desvio = desviación
prop-panel-forma = forma
prop-panel-escala = escala
prop-panel-alfa = alfa
prop-panel-beta = beta
prop-panel-forma_1 = forma 1
prop-panel-forma_2 = forma 2
prop-panel-media-2 = media = { $mean }
prop-panel-sem_limite = sin límite
prop-panel-sem_limite_velocidade_constante = sin límite (velocidad constante)
prop-panel-equipe_transportadores_do_modelo = Equipo (transportadores del modelo):
prop-panel-nenhum_transportador_no_modelo_crie = Ningún transportador en el modelo. Cree uno en la paleta.
prop-panel-nao_e_um_transportador = "{ $missing }" no es un transportador
prop-panel-a_equipe_nao_pode_ficar = El equipo no puede quedar vacío.
prop-panel-precisa_de_operadores = Necesita operadores
prop-panel-crie_um_transportador_ou_despachante = Cree un Transportador o Despachador para usar como equipo.
prop-panel-equipe = equipo
prop-panel-nenhuma = (ninguno)
prop-panel-nao_e_um_transportador_nem = "{ $team }" no es un transportador ni un despachador
prop-panel-quantidade = cantidad
prop-panel-durante_a_preparacao = durante la preparación
prop-panel-durante_o_processamento = durante el procesamiento
prop-panel-transporte_de_saida = Transporte de salida
prop-panel-nenhum = ninguno
prop-panel-os_itens_passam_direto_ao = Los ítems pasan directo al bloque siguiente.
prop-panel-nao_e_um_transportador_nem-2 = "{ $t }" no es un transportador ni un despachador
prop-panel-sem_rede_de_caminhos_a = Sin red de caminos: la base es la posición del transportador.
prop-panel-base_no_da_rede = Base (nodo de la red)
prop-panel-fora_da_rede = fuera de la red
prop-panel-escolher_um_no_poe_o = Elegir un nodo pone el transportador en él y activa "Volver a la base".
prop-panel-editar_como_texto = editar como texto
prop-panel-quantidade_de_itens_exigida_em = Cantidad de ítems exigida en cada entrada de componentes:
prop-panel-entrada = entrada { $i }
prop-panel-adicionar_entrada = añadir entrada
prop-panel-o_objeto_precisa_de_ligacoes = El objeto necesita { $list } conexiones de entrada (1 contenedor + { $list2 }).
prop-panel-valor_do_rotulo = Valor de la etiqueta
prop-panel-remover_rotulo = quitar etiqueta
prop-panel-nome_do_rotulo = nombre de la etiqueta
prop-panel-adicionar_rotulo = añadir etiqueta
prop-panel-lote_agrupar_itens = Lote (agrupar ítems)
prop-panel-tamanho_do_lote = tamaño del lote
prop-panel-espera_maxima_0_esperar_sempre = espera máxima (0 = esperar siempre)
prop-panel-fechar_a_entrada_ate_o = cerrar la entrada hasta que salga el lote
prop-panel-primeira_livre = Primera libre
prop-panel-rodizio = Rotación
prop-panel-aleatoria = Aleatoria
prop-panel-pesos = Pesos
prop-panel-por_rotulo = Por etiqueta
prop-panel-script = Script
prop-panel-regra_de_saida = Regla de salida
prop-panel-peso_de_cada_saida_na = Peso de cada salida (en el orden de las conexiones):
prop-panel-saida = salida { $i } ({ $to })
prop-panel-saida-2 = salida { $i }
prop-panel-ha_pesos_mas_o_objeto = Hay { $w } pesos, pero el objeto tiene { $n_out } salidas.
prop-panel-ajustar_ao_numero_de_saidas = ajustar al número de salidas
prop-panel-rotulo_com_o_numero_da = etiqueta con el número del puerto
prop-panel-o_nome_do_rotulo_nao = el nombre de la etiqueta no puede quedar vacío
prop-panel-script_que_devolve_o_numero = Script que devuelve el número del puerto (1, 2, ...)
prop-panel-script_ao_entrar_um_item = Script al entrar un ítem (on_entry)
prop-panel-roda_para_cada_item_que = Se ejecuta para cada ítem que entra, antes de cualquier otra regla.
prop-panel-quebras_mtbf_mttr = Averías (MTBF/MTTR)
prop-panel-tempo_entre_falhas_mtbf = Tiempo entre fallas (MTBF)
prop-panel-tempo_de_reparo_mttr = Tiempo de reparación (MTTR)
prop-panel-transferencia_pela_distancia = Transferencia por la distancia
prop-panel-os_itens_levam_atraso_distancia = Los ítems tardan retraso + distancia / velocidad en llegar al bloque siguiente por las conexiones directas (sin transportador)
prop-panel-velocidade = Velocidad
prop-panel-atraso = Retraso
prop-panel-caminho = Camino
prop-panel-reta = recto
prop-panel-pela_rede = por la red
prop-panel-tempo_entre_chegadas = Tiempo entre llegadas
prop-panel-primeira_chegada_em_t_0 = Primera llegada en t = 0
prop-panel-maximo_de_chegadas = Máximo de llegadas
prop-panel-se_a_saida_estiver_bloqueada = Si la salida está bloqueada
prop-panel-esperar = Esperar
prop-panel-descartar_perda = Descartar (pérdida)
prop-panel-rotulos = Etiquetas
prop-panel-capacidade_limitada = Capacidad limitada
prop-panel-disciplina = Disciplina
prop-panel-fifo_primeiro_a_entrar_primeiro = FIFO (primero en entrar, primero en salir)
prop-panel-lifo_ultimo_a_entrar_primeiro = LIFO (último en entrar, primero en salir)
prop-panel-capacidade = Capacidad
prop-panel-preparacao_setup = Preparación (setup)
prop-panel-tempo_de_processamento = Tiempo de procesamiento
prop-panel-operadores = Operadores
prop-panel-modo = Modo
prop-panel-embalar_so_o_conteiner_sai = Embalar (solo sale el contenedor)
prop-panel-juntar_componentes_destruidos = Unir (componentes destruidos)
prop-panel-lote_todos_saem = Lote (salen todos)
prop-panel-desembalar = Desembalar
prop-panel-dividir_em_copias = Dividir en copias
prop-panel-numero_de_itens_gerados = número de ítems generados
prop-panel-a_saida_nao_tem_parametros = La salida no tiene parámetros.
prop-panel-velocidade_maxima = Velocidad máxima
prop-panel-aceleracao = Aceleración
prop-panel-desaceleracao = Desaceleración
prop-panel-capacidade_itens = Capacidad (ítems)
prop-panel-tempo_de_carga = Tiempo de carga
prop-panel-tempo_de_descarga = Tiempo de descarga
prop-panel-voltar_a_base_quando_ocioso = Volver a la base cuando está ocioso
prop-panel-agv_controle_de_trafego_na = AGV (control de tráfico en la red)
prop-panel-comprimento = Longitud
prop-panel-comprimento_por_item = Longitud por ítem
prop-panel-acumula_itens_quando_a_saida = Acumula ítems cuando la salida se bloquea
prop-panel-capacidade_itens-2 = Capacidad: { $item_length } ítems
prop-panel-colunas = Columnas
prop-panel-niveis = Niveles
prop-panel-permanencia_minima = Permanencia mínima
prop-panel-regra_de_ocupacao = Regla de ocupación
prop-panel-largura_da_coluna = Ancho de la columna
prop-panel-altura_do_nivel = Altura del nivel
prop-panel-capacidade_posicoes = Capacidad: { $levels } posiciones
prop-panel-regra_de_escolha = Regla de elección
prop-panel-o_nome_nao_pode_ficar = el nombre no puede quedar vacío
prop-panel-ja_existe_um_objeto_chamado = ya existe un objeto llamado "{ $t }"
prop-panel-nome = Nombre
prop-panel-problema = Problema: { $message }
prop-panel-padrao_do_tipo = (predeterminado del tipo)
prop-panel-aparencia = Apariencia
prop-panel-ativo = Recurso:
prop-panel-clipes_de_animacao_ainda_nao = Los clips de animación todavía no se reproducen: pose de reposo.
prop-panel-biblioteca_do_utsim = Biblioteca de UTSim
prop-panel-utsim_gerado_por_utsim_assets = UTSim (generado por utsim-assets)
prop-panel-terceiros_assets_terceiros = Terceros (assets/terceiros)
prop-panel-cor = Color
prop-panel-escala-2 = Escala
prop-panel-giro_graus = Giro (grados)
prop-panel-restaurar_padrao = Restaurar predeterminado
prop-panel-volta_a_aparencia_padrao_do = Vuelve a la apariencia predeterminada del tipo de objeto
prop-panel-simulacao = Simulación
prop-panel-tempo_de_simulacao = Tiempo de simulación
prop-panel-aquecimento = Calentamiento
prop-panel-semente = Semilla
prop-panel-replicacoes = Réplicas

## properties/position.rs

prop-position-posicao_m = Posición (m)
prop-position-giro_graus = Giro (grados)

## properties/schedule.rs

prop-schedule-segundos = segundos
prop-schedule-minutos = minutos
prop-schedule-horas = horas
prop-schedule-use_dia_n_hh_mm = use "dia N hh:mm" (recibido "{ $t }")
prop-schedule-dia_invalido_em_a_partir = día inválido en "{ $t }" (a partir de 1)
prop-schedule-use_um_numero = use un número
prop-schedule-hh_mm_so_vale_com = hh:mm solo vale con ciclo de día o semana (1440 min, 24 h, 86400 s); use un número
prop-schedule-hora_invalida_em = hora inválida en "{ $t }"
prop-schedule-minuto_invalido_em = minuto inválido en "{ $t }"
prop-schedule-minuto_invalido_em_0_a = minuto inválido en "{ $t }" (0 a 59)
prop-schedule-nao_e_um_horario_hh = "{ $t }" no es un horario (hh:mm) ni un número

## properties/schedule_ui.rs

prop-schedule-ui-media = { $family }, media { $mean }
prop-schedule-ui-chegadas_por_periodo = Llegadas por período
prop-schedule-ui-a_taxa_de_chegada_muda = la tasa de llegada cambia con la hora
prop-schedule-ui-uma_tabela_de_periodos_que = Una tabla de períodos que se repite en cada ciclo (un día, una semana). Fuera de los períodos no hay llegadas.
prop-schedule-ui-desligado_o_tempo_entre_chegadas = Apagado: el tiempo entre llegadas de arriba vale todo el tiempo.
prop-schedule-ui-ciclo = ciclo
prop-schedule-ui-dia = día
prop-schedule-ui-semana = semana
prop-schedule-ui-ciclo_de_tempo_do_modelo = ciclo de { $v } (tiempo del modelo en minutos)
prop-schedule-ui-ciclo_de_tempo_do_modelo-2 = Ciclo de { $cycle }: tiempo del modelo en { $name }. Horarios en hh:mm, tasas por hora.
prop-schedule-ui-horarios_e_taxas_em_unidades = Horarios y tasas en unidades del modelo (con ciclo 1440, un día en minutos, la tabla muestra hh:mm).
prop-schedule-ui-taxa_por_hora = tasa por hora
prop-schedule-ui-taxa = tasa
prop-schedule-ui-inicio = inicio
prop-schedule-ui-fim = fin
prop-schedule-ui-modo = modo
prop-schedule-ui-valor = valor
prop-schedule-ui-esperado = esperado
prop-schedule-ui-copiar_do_padrao = copiar del predeterminado
prop-schedule-ui-usa_no_periodo_uma_copia = Usa en el período una copia del tiempo entre llegadas predeterminado, para ajustar solo aquí
prop-schedule-ui-remover_o_periodo = quitar el período
prop-schedule-ui-adicionar_periodo = añadir período
prop-schedule-ui-por_dia = {" "}(por día)
prop-schedule-ui-por_semana = {" "}(por semana)
prop-schedule-ui-esperado_por_ciclo_chegadas = Esperado por ciclo{ $hint }: { $per_cycle } llegadas
prop-schedule-ui-na_troca_de_periodo_a = En el cambio de período la llegada pendiente se sortea de nuevo con el proceso del nuevo período (los períodos seguidos con el mismo proceso mantienen la llegada).
prop-schedule-ui-distribuicoes_dos_periodos = Distribuciones de los períodos
prop-schedule-ui-periodo_tempo_entre_chegadas = Período { $i }: tiempo entre llegadas
prop-schedule-ui-rotulos_por_periodo = Etiquetas por período
prop-schedule-ui-os_rotulos_de_um_periodo = Las etiquetas de un período reemplazan las etiquetas predeterminadas del mismo nombre (por ejemplo, más urgentes de noche); las demás siguen valiendo.
prop-schedule-ui-a_fonte_nao_tem_rotulos = La fuente no tiene etiquetas predeterminadas.
prop-schedule-ui-a = { $a } a { $b }
prop-schedule-ui-a-2 = { $start } a { $end }
prop-schedule-ui-periodo = Período { $i } ({ $span })

## properties/shifts_ui.rs

prop-shifts-ui-como_no_calendario = como en el calendario ({ $to_lowercase })
prop-shifts-ui-ao_fim_do_turno = Al final del turno
prop-shifts-ui-periodo = período
prop-shifts-ui-inicio = inicio
prop-shifts-ui-duracao = duración
prop-shifts-ui-remover = quitar
prop-shifts-ui-adicionar_janela = añadir ventana
prop-shifts-ui-fora_de_turno_do_periodo = fuera de turno: { $off_fraction }% del período
prop-shifts-ui-turnos_janelas_fora_de_turno = Turnos (ventanas fuera de turno)
prop-shifts-ui-usar_calendario = usar calendario
prop-shifts-ui-crie_calendarios_na_secao_calendarios = Cree calendarios en la sección Calendarios del panel Simulación (haga clic en un punto vacío de la escena)
prop-shifts-ui-personalizado = personalizado
prop-shifts-ui-periodo_janela_s_fora_de = período { $period }, { $off } ventana(s) fuera de turno
prop-shifts-ui-o_calendario_nao_existe = el calendario '{ $name }' no existe
prop-shifts-ui-nome_do_calendario = nombre del calendario
prop-shifts-ui-salvar_como_calendario = guardar como calendario
prop-shifts-ui-outros_objetos_podem_usar_o = Otros objetos pueden usar el mismo calendario
prop-shifts-ui-calendarios = Calendarios ({ $calendars })
prop-shifts-ui-tabelas_de_turno_com_nome = Tablas de turno con nombre. Un objeto usa una de ellas en Turnos > usar calendario; cambiar el calendario cambia todos los objetos que lo usan.
prop-shifts-ui-nenhum_objeto_usa_este_calendario = ningún objeto usa este calendario
prop-shifts-ui-usado_por = usado por: { $users }
prop-shifts-ui-remover_calendario = quitar calendario
prop-shifts-ui-troque_os_objetos_para_outro = Cambie los objetos a otro calendario o a personalizado antes
prop-shifts-ui-novo_calendario = nuevo calendario
prop-shifts-ui-o_nome_nao_pode_ficar = el nombre no puede quedar vacío
prop-shifts-ui-ja_existe_um_calendario_chamado = ya existe un calendario llamado "{ $t }"
prop-shifts-ui-nome = Nombre

## properties/spec.rs

prop-spec-continuas = Continuas
prop-spec-discretas_e_empirica = Discretas y empírica
prop-spec-combinacao = Combinación

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

# Parte 3: os resultados (results.rs, results_panel.rs)
tipo-fluxo = Flujo
tipo-fluidos = Fluidos
tipo-medicao-de-tempo = Medición de tiempo
tipo-outro = Otro
res-status-replicacoes = { $n ->
    [one] { $n } replicación terminada
   *[other] { $n } replicaciones terminadas
}
res-status-replicacoes-gargalo = { $n ->
    [one] { $n } replicación terminada
   *[other] { $n } replicaciones terminadas
}; cuello de botella: { $gargalo }
res-status-pontos = { $n ->
    [one] { $n } punto evaluado
   *[other] { $n } puntos evaluados
}
res-status-pontos-melhor = { $n ->
    [one] { $n } punto evaluado
   *[other] { $n } puntos evaluados
}; mejor: { $melhor }

## results.rs

res-replicacao_oes_de_t_0 = { $replications ->
    [one] { $replications } replicación
   *[other] { $replications } replicaciones
}, de t = 0 a t = { $stop_time } con calentamiento hasta t = { $warmup }, semilla { $seed }
res-ocupacao = Ocupación
res-incluindo_bloqueado = Incluyendo bloqueado
res-dentro_do_turno = Dentro del turno
res-sem_o_tempo_bloqueado = Sin el tiempo bloqueado
res-fracao_do_tempo_trabalhando_em = Fracción del tiempo trabajando (en proceso o preparación). Con capacidad mayor que 1, es la media de servidores ocupados dividida por la capacidad.
res-ocupacao_mais_o_tempo_em = Ocupación más el tiempo en que el recurso retiene un ítem terminado que no puede salir.
res-ocupacao_sobre_o_tempo_em = Ocupación sobre el tiempo en turno: el tiempo fuera de turno sale del denominador.
res-ocupacao_sobre_o_tempo_nao = Ocupación sobre el tiempo no bloqueado: muestra la carga propia del recurso.
res-objeto = Objeto
res-tipo = Tipo
res-entradas = Entradas
res-saidas = Salidas
res-conteudo_medio = Contenido medio
res-permanencia_media = Permanencia media
res-c_bloqueio = c/ bloqueo
res-no_turno = en turno
res-s_bloqueio = s/ bloqueo
res-velocidade_da_transferencia = { $name }: velocidad de la transferencia
res-simulacao = Simulación: { $label }
res-simulacao_velocidade_padrao_das_transferencias = Simulación: velocidad predeterminada de las transferencias
res-cenario = escenario { $n }
res-adicione_pelo_menos_um_cenario = agregue al menos un escenario
res-cenario_de_um_nome = escenario { $i }: póngale un nombre
res-cenario_nome_repetido = escenario '{ $name }': nombre repetido
res-cenario-2 = escenario '{ $name }': { $e }
res-restricao_de_nao_e_um = restricción de '{ $metric }': { $what } '{ $text }' no es un número
res-escolha_a_metrica_do_objetivo = elija la métrica del objetivo
res-minimo = mínimo
res-maximo = máximo
res-restricao_de_informe_minimo_ou = restricción de '{ $metric }': indique mínimo o máximo

## results_panel.rs

res-painel-resumo = Resumen
res-painel-graficos = Gráficos
res-painel-cenarios = Escenarios
res-painel-otimizacao = Optimización
res-painel-ocupacao = Ocupación
res-painel-estados = Estados
res-painel-serie_temporal = Serie temporal
res-painel-histograma = Histograma
res-painel-a_execucao_terminou_sem_resultado = la ejecución terminó sin resultado
res-painel-cenario_s_comparados = { $scenarios ->
    [one] { $scenarios } escenario comparado
   *[other] { $scenarios } escenarios comparados
}
res-painel-cancelado = { $what }: cancelado
res-painel-erro = { $what }: error
res-painel-rodando_replicacoes = Ejecutando replicaciones
res-painel-comparando_cenarios = Comparando escenarios
res-painel-otimizando = Optimizando
res-painel-resultados = Resultados
res-painel-o_modelo_tem_problema_s = El modelo tiene { $blocked ->
    [one] { $blocked } problema
   *[other] { $blocked } problemas
}: corríjalos en Editar antes de ejecutar.
res-painel-de = { $what }: { $done } de { $total }
res-painel-cancelar = Cancelar
res-painel-rodada = Ejecución
res-painel-replicacoes = Replicaciones
res-painel-semente = Semilla
res-painel-tempo_de_parada = Tiempo de parada
res-painel-aquecimento = Calentamiento
res-painel-series_temporais_a_cada = Series temporales cada
res-painel-histogramas_com_classes = Histogramas con clases
res-painel-rodar_replicacoes = Ejecutar replicaciones
res-painel-gravar_no_modelo = Guardar en el modelo
res-painel-grava_replicacoes_tempos_e_semente = Guarda replicaciones, tiempos y semilla en el modelo (Ctrl+Z deshace)
res-painel-restaurar_do_modelo = Restaurar del modelo
res-painel-rode_as_replicacoes_para_ver = Ejecute las replicaciones para ver el resumen por objeto, el cuello de botella y los gráficos.
res-painel-cada_valor_media_entre_replicacoes = { $describe }. Cada valor: media entre replicaciones ± semiamplitud del IC del 95%.
res-painel-gargalo_geral = Cuello de botella (general):
res-painel-ocupacao_a_maior_entre_processos = (ocupación { $content }; la mayor entre procesos y recursos)
res-painel-gargalo_dentro_do_turno = Cuello de botella (dentro del turno):
res-painel-ocupacao_no_tempo_em_turno = (ocupación { $content } en el tiempo en turno)
res-painel-tipo = Tipo:
res-painel-todos = todos
res-painel-clique_no_titulo_de_uma = Haga clic en el título de una columna para ordenar.
res-painel-decr = {" "}(desc.)
res-painel-cresc = {" "}(asc.)
res-painel-metricas_proprias_tempo_no_sistema = Métricas propias (tiempo en el sistema, flujos, niveles de tanques...)
res-painel-pasta_de_exportacao = Carpeta de exportación:
res-painel-exportar_csv = Exportar CSV
res-painel-resumo_csv_objetos_csv_replicacoes = resumo.csv, objetos.csv, replicacoes.csv y, si los hay, series.csv e histogramas.csv
res-painel-exportar_html = Exportar HTML
res-painel-relatorio_com_tabelas_e_graficos = Informe con tablas y gráficos (incluye la comparación de escenarios, si la hay)
res-painel-a_exportacao_de_arquivos_existe = La exportación de archivos solo existe en la aplicación de escritorio.
res-painel-csv_gravado_em = CSV guardado en { $dir }: { $files }
res-painel-nao_foi_possivel_gravar_em = no se pudo guardar en { $dir }: { $e }
res-painel-relatorio_gravado_em = Informe guardado en { $file }
res-painel-nao_foi_possivel_gravar = no se pudo guardar { $file }: { $e }
res-painel-rode_as_replicacoes_na_aba = Ejecute las replicaciones en la pestaña Resumen para ver los gráficos.
res-painel-o_modelo_nao_tem_processos = El modelo no tiene procesos ni recursos móviles.
res-painel-de_cada_recurso_com_o = { $label } de cada recurso (%), con el intervalo del 95%, nunca por encima del 100% por servidor. En naranja, el cuello de botella (mayor ocupación).
res-painel-fracao_do_tempo_em_cada = Fracción del tiempo en cada estado (%), con los colores del 3D.
res-painel-objeto = Objeto:
res-painel-este_objeto_nao_tem_observacoes = Este objeto no tiene observaciones de permanencia.
res-painel-a_rodada_foi_feita_sem = La ejecución se hizo sin histogramas.
res-painel-medida = Medida:
res-painel-ativar_series_temporais_e_rodar = Activar series temporales y ejecutar de nuevo
res-painel-ativar_histogramas_e_rodar_de = Activar histogramas y ejecutar de nuevo
res-painel-ic_95 = IC 95%
res-painel-sem_estados_registrados = Sin estados registrados.
res-painel-este_objeto_nao_tem_pontos = Este objeto no tiene puntos muestreados.
res-painel-a_rodada_foi_feita_sem-2 = La ejecución se hizo sin series temporales (run.sample_interval desactivado).
res-painel-conteudo_de_ao_longo_do = Contenido de { $object } a lo largo del tiempo: media de las replicaciones (línea gruesa) y cada replicación.
res-painel-observacoes_de_todas_as_replicacoes = { $total } observaciones de todas las replicaciones, de { $h } a { $h2 }.
res-painel-parametro = Parámetro:
res-painel-atual = { $key } (actual: { $current })
res-painel-ou_chave = o clave:
res-painel-objeto_campo = objeto.campo
res-painel-adicionar_parametro = Agregar parámetro
res-painel-cada_linha_e_um_cenario = Cada fila es un escenario; cada columna, un parámetro. Una celda vacía mantiene el valor del modelo. Valores en texto RON, como en el campo set de los experimentos.
res-painel-experimento = Experimento:
res-painel-cenario = Escenario
res-painel-remover_parametro = Quitar parámetro
res-painel-remover_cenario = Quitar escenario
res-painel-adicionar_cenario = Agregar escenario
res-painel-metricas_separadas_por_virgula_vazio = Métricas (separadas por coma; vacío: todas):
res-painel-rodar_cenarios = Ejecutar escenarios
res-painel-replicacao_oes_por_cenario_aba = { $reps ->
    [one] { $reps } replicación
   *[other] { $reps } replicaciones
} por escenario (pestaña Resumen), mismas semillas en todos
res-painel-media_ic_de_95_e = { $name }: media ± IC del 95%. Δ es la diferencia pareada (mismas semillas) respecto de '{ $name2 }'; ≈ indica diferencia no significativa, en naranja las significativas.
res-painel-filtrar_metricas = Filtrar métricas:
res-painel-metrica = Métrica
res-painel-vs = Δ vs { $name }
res-painel-mostrar_no_grafico = Mostrar en el gráfico
res-painel-grafico_da_metrica = Gráfico de la métrica:
res-painel-arquivo_de_experimento = Archivo de experimento:
res-painel-abrir_experimento = Abrir experimento
res-painel-experimento_aberto = Experimento '{ $name }' abierto
res-painel-experimento_aberto_os_cenarios_valem = Experimento '{ $name }' abierto; los escenarios valen para el modelo abierto (el archivo apunta a { $model_path })
res-painel-nao_foi_possivel_abrir = no se pudo abrir: { $e }
res-painel-salvar_experimento = Guardar experimento
res-painel-salve_o_modelo_antes_o = guarde el modelo antes: el experimento apunta al archivo del modelo
res-painel-experimento_salvo_em = Experimento guardado en { $target }
res-painel-variaveis_de_decisao_mesmas_chaves = Variables de decisión (mismas claves de los escenarios), objetivo y restricciones. Todos los puntos usan las mismas semillas; el mejor se compara con los demás por prueba pareada.
res-painel-variavel = Variable
res-painel-dominio = Dominio
res-painel-minimo = Mínimo
res-painel-maximo = Máximo
res-painel-passo_valores = Paso / valores
res-painel-custo_por_unidade = Costo por unidad
res-painel-inteiro = entero
res-painel-continuo = continuo
res-painel-lista = lista
res-painel-objetivo = Objetivo:
res-painel-minimizar = minimizar
res-painel-maximizar = maximizar
res-painel-peso_da_metrica = peso de la métrica
res-painel-custos_das_variaveis = + costos de las variables
res-painel-restricoes_sobre_a_media_da = Restricciones (sobre la media de la métrica)
res-painel-minimo-2 = mínimo
res-painel-maximo-2 = máximo
res-painel-adicionar_restricao = Agregar restricción
res-painel-metodo = Método:
res-painel-grade = rejilla
res-painel-recozimento_simulado = recocido simulado
res-painel-genetico = genético
res-painel-iteracoes = iteraciones
res-painel-populacao = población
res-painel-geracoes = generaciones
res-painel-replicacoes_por_ponto = Replicaciones por punto
res-painel-orcamento_pontos = Presupuesto (puntos)
res-painel-semente_da_busca = Semilla de la búsqueda
res-painel-refinar_os = Refinar los
res-painel-melhores_com = mejores con
res-painel-replicacoes-2 = replicaciones
res-painel-otimizar = Optimizar
res-painel-melhor_ponto = Mejor punto:
res-painel-objetivo-2 = objetivo { $objective }
res-painel-aplicar_ao_modelo = Aplicar al modelo
res-painel-grava_estes_valores_no_modelo = Guarda estos valores en el modelo (Ctrl+Z deshace)
res-painel-nao_atende = no cumple
res-painel-media_observada = { $bound } (media observada: { $cell })
res-painel-nenhum_ponto_avaliado_atende_as = Ningún punto evaluado cumple las restricciones.
res-painel-convergencia_melhor_objetivo_viavel_por = Convergencia: mejor objetivo factible por número de puntos evaluados
res-painel-pontos_avaliados = puntos evaluados
res-painel-melhor_ate_entao = mejor hasta ahora
res-painel-comparacao_pareada_com_o_melhor = Comparación pareada con el mejor (≈: sin diferencia significativa al 95%)
res-painel-ponto = Punto
res-painel-objetivo-3 = Objetivo
res-painel-viavel = Factible
res-painel-vs_melhor = Δ vs mejor
res-painel-equivalente = { $delta } equivalente
res-painel-objeto_metrica = objeto.métrica

# Parte 4: fluxos, câmeras e modelos 3D (flows*.rs, cameras.rs, camera_modes.rs, models.rs)

## Tipos de atividade e modos de câmera

fluxo-tipo-gerar-tokens = Generar tokens
fluxo-tipo-agenda = Agenda
fluxo-tipo-ao-evento = Al evento
fluxo-tipo-esperar = Esperar
fluxo-tipo-decidir = Decidir
fluxo-tipo-atribuir = Asignar
fluxo-tipo-adquirir = Adquirir
fluxo-tipo-liberar = Liberar
fluxo-tipo-aguardar-evento = Esperar evento
fluxo-tipo-criar-item = Crear ítem
fluxo-tipo-lote = Lote
fluxo-tipo-fim = Fin
camera-modo-orbita = Órbita
camera-modo-seguir = Seguir
camera-modo-primeira-pessoa = Primera persona
camera-modo-do-recurso = Desde el recurso

## camera_modes.rs

camera-modes-seguir_e_do_recurso_so = Seguir y Desde el recurso solo funcionan en Simular
camera-modes-o_modelo_nao_tem_recursos = El modelo no tiene recursos móviles para seguir
camera-modes-camera = Cámara: { $modo }
camera-modes-orbita_seguir_um_recurso_andar = Órbita, seguir un recurso, caminar por la planta o ver desde el recurso (teclas 1 a 4)
camera-modes-recurso = recurso: { $name }
camera-modes-distancia_m = distancia (m)
camera-modes-velocidade_m_s = velocidad (m/s)
camera-modes-travar_o_mouse_l = Bloquear el ratón (L)
camera-modes-sair_esc = Salir (Esc)

## cameras.rs

cameras-esquerdo_arrasta_desloca_direito_gira = izquierdo arrastra (desplaza), derecho gira, rueda acerca
cameras-direito_gira_em_volta_do = derecho gira alrededor del recurso, rueda cambia la distancia, Esc sale
cameras-wasd_anda_shift_corre_direito = WASD camina, Shift corre, derecho (o L) mira, rueda cambia la velocidad, Esc sale
cameras-direito_olha_em_volta_esc = derecho mira alrededor, Esc sale

## flows.rs

flows-o_nome_nao_pode_ficar = el nombre no puede quedar vacío
flows-ja_existe_uma_atividade_chamada = ya existe una actividad llamada "{ $to }"

## flows_ui.rs

flows-ui-fluxos = Flujos
flows-ui-nenhum_fluxo_neste_modelo = Ningún flujo en este modelo.
flows-ui-novo_fluxo = Nuevo flujo
flows-ui-cria_um_fluxo_gerar_tokens = Crea un flujo Generar tokens, Esperar, Fin
flows-ui-remover = Quitar
flows-ui-remove_o_fluxo_selecionado_ctrl = Quita el flujo seleccionado (Ctrl+Z deshace)
flows-ui-nome = Nombre
flows-ui-o_nome_nao_pode_ficar = el nombre no puede quedar vacío
flows-ui-ja_existe_um_objeto_ou = ya existe un objeto o flujo llamado "{ $t }"
flows-ui-recursos = Recursos
flows-ui-contadores_compartilhados_ex_docas = Contadores compartidos (ej.: muelles).
flows-ui-cap = cap.
flows-ui-remover_recurso = Quitar recurso
flows-ui-recurso = + recurso
flows-ui-crie_um_fluxo_com_novo = Cree un flujo con "Nuevo flujo" para dibujar la lógica del modelo.
flows-ui-adicionar = Agregar:
flows-ui-adicionar_atividade = Agregar actividad
flows-ui-excluir_atividade = Eliminar actividad
flows-ui-remove_a_atividade_e_as = Quita la actividad y las conexiones que llegan a ella
flows-ui-e_mais_problema_s = ... y { $problems ->
    [one] { $problems } problema más
   *[other] { $problems } problemas más
}
flows-ui-simulacao_em_t_o_numero = Simulación en t = { $t }: el número en cada caja es la cantidad de tokens allí.
flows-ui-fluxo_vazio_adicione_uma_atividade = Flujo vacío: agregue una actividad.
flows-ui-atividade = Actividad
flows-ui-clique_numa_atividade_do_diagrama = Haga clic en una actividad del diagrama para ver y editar sus campos.
flows-ui-tokens_agora = Tokens ahora: { $t }
flows-ui-volte_para_editar_para_alterar = Vuelva a Editar para cambiar el flujo.
flows-ui-escolha = (elija)
flows-ui-objeto_nao_existe = el objeto "{ $value }" no existe
flows-ui-remover_rotulo = Quitar etiqueta
flows-ui-rotulo = + etiqueta
flows-ui-proxima_atividade = Próxima actividad
flows-ui-fim_do_fluxo_o_token = Fin del flujo: el token termina aquí.
flows-ui-ramos_para_onde_o_token = Ramas (adónde va el token)
flows-ui-remover_ligacao = Quitar conexión
flows-ui-peso = {"     "}peso
flows-ui-se = {"     "}si
flows-ui-ligar_a_outro_ramo = + conectar a otra rama...
flows-ui-ligar_a = + conectar a...
flows-ui-ja_existe_uma_atividade_chamada = ya existe una actividad llamada "{ $t }"
flows-ui-intervalo_entre_tokens = Intervalo entre tokens
flows-ui-criar_um_token_em_t = crear un token en t = 0
flows-ui-limitar_a = limitar a
flows-ui-tokens = tokens
flows-ui-horarios = Horarios
flows-ui-t = t =
flows-ui-remover_horario = Quitar horario
flows-ui-horario = + horario
flows-ui-objeto = Objeto
flows-ui-evento = Evento
flows-ui-o_token_passa_a_carregar = El token pasa a llevar el ítem de ese evento.
flows-ui-tempo_de_espera = Tiempo de espera
flows-ui-probabilidade = Probabilidad
flows-ui-pelo_rotulo = Por etiqueta
flows-ui-condicoes = Condiciones
flows-ui-agente_externo = Agente externo
flows-ui-regra = Regla
flows-ui-rotulo_do_token_1_2 = etiqueta del token (1, 2, ...)
flows-ui-a_primeira_condicao_verdadeira_vence = Gana la primera condición verdadera; ninguna = última rama.
flows-ui-pesos_relativos_um_por_ramo = Pesos relativos, uno por rama.
flows-ui-um_agente_externo_escolhe_o = Un agente externo elige la rama; la simulación se pausa y muestra estas observaciones:
flows-ui-observacao = + observación
flows-ui-rotulos = Etiquetas
flows-ui-nome_atribui_ao_token_item = nombre asigna al token; item.nombre al ítem llevado.
flows-ui-recurso-2 = Recurso
flows-ui-recurso_nao_existe = el recurso "{ $resource }" no existe
flows-ui-crie_um_recurso_na_coluna = Cree un recurso en la columna de la izquierda.
flows-ui-quantidade = cantidad
flows-ui-dentro_do_objeto = Dentro del objeto
flows-ui-porta_de_entrada = puerto de entrada
flows-ui-rotulos_do_item = Etiquetas del ítem
flows-ui-juntar = Juntar

## models.rs

models-visual = Visual:
models-simples = simple
models-realista = realista
models-modelos_3d_articulados_da_biblioteca = Modelos 3D articulados de la biblioteca (montacargas, AGV, operador...)

## Estados dos blocos (painel do bloco e gráficos de estados)

estado-sem-estado = sin estado
estado-ocioso = ocioso
estado-vazio = vacío
estado-gerando = generando
estado-liberando = liberando
estado-coletando = recolectando
estado-preparacao = preparación
estado-processando = procesando
estado-bloqueado = bloqueado
estado-aguardando-operador = esperando operador
estado-aguardando-transporte = esperando transporte
estado-em-falha = en falla
estado-fora-do-turno = fuera de turno
estado-viajando-vazio = viajando vacío
estado-viajando-carregado = viajando cargado
estado-carregando = cargando
estado-descarregando = descargando
estado-trabalhando-no-posto = trabajando en el puesto
estado-parado = detenido (comando externo)
estado-aguardando-tecnico = esperando técnico
estado-em-manutencao = en mantenimiento preventivo
estado-em-pausa = en pausa (fatiga)
estado-operador-entrando = operador entrando
estado-operador-saindo = operador saliendo

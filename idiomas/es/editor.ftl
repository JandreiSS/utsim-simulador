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

# UTSim: o editor do simulador (casca: barra, camadas, painéis e diálogos gerais).

ui-sem_observacoes = sem observações
ui-resumo = n={ $n }  média={ $media }  dp={ $dp }  mín={ $min }  máx={ $max }
ui-recebidos = recebidos: { $n }
ui-simulacao = Simulação
ui-modelo_replicacao = { $modelo } - replicação { $n }
ui-contagem = { $blocos ->
    [one] { $blocos } bloco
   *[other] { $blocos } blocos
}, { $itens ->
    [one] { $itens } item
   *[other] { $itens } itens
}, { $eventos ->
    [one] { $eventos } evento de rastro
   *[other] { $eventos } eventos de rastro
}
ui-pausar = Pausar
ui-reproduzir = Reproduzir
ui-evento_anterior = < Evento
ui-evento_seguinte = Evento >
ui-inicio = Início
ui-velocidade = velocidade x{ $v }
ui-unidades_por_segundo = unidades de tempo simulado por segundo
ui-tempo = tempo
ui-tempo_simulado = tempo simulado: { $t } de { $fim }
ui-ajuda_teclas = Espaço: reproduzir/pausar | +/-: velocidade | setas: evento anterior/próximo | F: enquadrar
ui-ajuda_mouse = Mouse: esquerdo arrasta (move) e clica (seleciona), direito gira, roda aproxima
ui-ajuda_camera = Câmera: 1 órbita, 2 seguir, 3 primeira pessoa, 4 do recurso | F3: tempos de quadro
ui-blocos = Blocos
ui-clique_em_um_bloco = Clique em um bloco para ver suas estatísticas.
ui-capacidade_rotulo = capacidade { $n }
ui-nivel_agora = nível agora
ui-capacidade = capacidade
ui-ocupacao = ocupação
ui-taxa_agora = taxa agora
ui-nivel_inicial = nível inicial
ui-entrada_maxima = entrada máxima
ui-saida_maxima = saída máxima
ui-vazao_da_fonte = vazão da fonte
ui-volume_por_item = volume por item
ui-acumulado = acumulado para o próximo item
ui-vazao_agora = vazão agora
ui-vazao_maxima = vazão máxima
ui-objeto = objeto
ui-por_unidade_de_tempo = { $taxa } por unidade de tempo ({ $sentido })
ui-nivel_interpolado = Nível e taxa interpolados a partir do rastro da rede de fluidos.
ui-seguir_recurso = Seguir este recurso
ui-seguir_recurso_dica = A câmera acompanha o recurso por trás e por cima (2)
ui-ver_do_recurso = Ver do recurso
ui-ver_do_recurso_dica = A câmera vai na posição do motorista (4)
ui-recebidos_ate_agora = recebidos até agora
ui-estado_agora = estado agora
ui-conteudo_agora = conteúdo agora
ui-entradas = entradas
ui-saidas = saídas
ui-conteudo_medio = conteúdo médio
ui-conteudo_maximo = conteúdo máximo
ui-tempo_de_permanencia = tempo de permanência
ui-fracao_em = fração em { $estado }
ui-estatisticas_finais = Estatísticas finais da replicação (janela após o aquecimento).
camadas-no_excluido = Nó { $nome } e suas arestas excluídos (Ctrl+Z desfaz)
camadas-aresta_excluida = Aresta { $de } - { $para } excluída (Ctrl+Z desfaz)
camadas-no_fluido_excluido = Nó { $nome } e suas ligações excluídos (Ctrl+Z desfaz)
camadas-objetos_dica = Objetos do modelo: biblioteca, ligações e propriedades
camadas-rede_dica = Rede de caminhos dos recursos móveis: nós e arestas
camadas-fluidos_dica = Rede de fluidos: fontes, tanques, tubos e conversores
camada-objetos = Objetos
camada-rede = Rede
camada-fluidos = Fluidos
camadas-rede_de_caminhos = Rede de caminhos
camadas-selecionar = Selecionar
camadas-selecionar_rede_dica = Clique seleciona um nó ou uma aresta; arrastar move o nó
camadas-no = Nó
camadas-no_dica = Clique no chão coloca um nó
camadas-aresta = Aresta
camadas-aresta_dica = Clique num nó e depois em outro; clicar no chão cria um nó já ligado
camadas-mao_dupla = Arestas novas de mão dupla
camadas-ajuda_aresta = Aresta: clique de nó em nó para desenhar o caminho; Esc termina.
camadas-ajuda_del_rede = Del exclui o nó (com as arestas) ou a aresta selecionada.
camadas-ajuda_atendido = Cada objeto atendido entra na rede pelo nó mais próximo (linha tracejada).
camadas-rede_de_fluidos = Rede de fluidos
camadas-selecionar_fluidos_dica = Clique seleciona um nó; arrastar move o nó
camadas-ligar = Ligar
camadas-ligar_dica = Clique no nó de origem e depois no de destino (a vazão segue a seta)
camadas-colocar = Colocar
camadas-colocar_dica = Clique no chão coloca o nó (Shift mantém a ferramenta)
camadas-ajuda_aciclica = A rede precisa ser acíclica. Os conversores ligam a rede a um objeto.
camadas-ajuda_del_fluidos = Del exclui o nó selecionado e suas ligações.
camadas-aresta_daqui = Aresta daqui
camadas-ligar_daqui = Ligar daqui
camadas-sem_rede = O modelo não tem rede de caminhos.
camadas-sem_rede_ajuda = Sem rede, os recursos móveis andam em linha reta. Use "Nó" para colocar os nós e "Aresta" para ligá-los.
camadas-resumo_rede = { $nos ->
    [one] { $nos } nó
   *[other] { $nos } nós
} e { $arestas ->
    [one] { $arestas } aresta
   *[other] { $arestas } arestas
}. Selecione um para editar.
camadas-nos = Nós ({ $n })
camadas-arestas = Arestas ({ $n })
camadas-sem_fluidos = O modelo não tem rede de fluidos.
camadas-sem_fluidos_ajuda = Escolha um tipo em "Colocar" e clique no chão; depois use "Ligar".
camadas-resumo_fluidos = Rede "{ $rede }": { $nos ->
    [one] { $nos } nó
   *[other] { $nos } nós
} e { $ligacoes ->
    [one] { $ligacoes } ligação
   *[other] { $ligacoes } ligações
}. Selecione um nó para editar.
editor-layout_aplicado = Layout automático aplicado: salve para manter as posições
editor-layout_com_rede = Modelo com rede de caminhos: posicione os objetos nos nós da rede (o layout automático não foi aplicado)
editor-sem_titulo = sem título.ron
editor-excluidos = { $n } objetos excluídos (Ctrl+Z desfaz)
editor-nada_para_copiar = Nada selecionado para copiar
editor-copiados = { $objetos ->
    [one] { $objetos } objeto
   *[other] { $objetos } objetos
} e { $ligacoes ->
    [one] { $ligacoes } ligação copiados
   *[other] { $ligacoes } ligações copiados
}
editor-recortados = { $n ->
    [one] { $n } objeto recortado (Ctrl+V cola)
   *[other] { $n } objetos recortados (Ctrl+V cola)
}
editor-area_vazia = A área de transferência está vazia (Ctrl+C copia)
editor-colados = { $n ->
    [one] { $n } objeto colado: { $nomes }
   *[other] { $n } objetos colados: { $nomes }
}
editor-nada_para_duplicar = Nada selecionado para duplicar
editor-salvo_em = Salvo em { $arquivo }
editor-erro_ao_salvar = Erro ao salvar: { $erro }
editor-tempo_invalido = Tempo máximo inválido: use um número maior que zero
editor-simulando = Simulando...
editor-sem_resultado = a simulação terminou sem resultado
editor-erro_na_simulacao = Erro na simulação: { $erro }
editor-na_caixa = { $n ->
    [one] { $n } objeto na caixa
   *[other] { $n } objetos na caixa
}, { $sel ->
    [one] { $sel } selecionado
   *[other] { $sel } selecionados
}
editor-outro_destino = Escolha outro bloco como destino
editor-ligacao_removida = Ligação { $de } -> { $para } removida (Ctrl+Z desfaz)
editor-editar = Editar
editor-simular = ▶ Simular
editor-ate_t = até t =
editor-novo = Novo
editor-salvar = Salvar
editor-salvar_como_botao = Salvar como...
editor-desfazer = Desfazer
editor-refazer = Refazer
editor-enquadrar = Enquadrar (F)
editor-fluxos = Fluxos
editor-fluxos_dica = Lógica do usuário: fluxos de tokens
editor-resultados = Resultados
editor-resultados_dica = Replicações, resumo com intervalos de confiança, gráficos, cenários e otimização
editor-atalhos = Atalhos
editor-atalhos_dica = Atalhos de teclado e mouse (F1)
editor-salvar_como = Salvar como
editor-caminho_do_arquivo = Caminho do arquivo (.ron), relativo à pasta do programa ou completo:
editor-sobrescrito = Esse arquivo já existe e será sobrescrito.
editor-cancelar = Cancelar
editor-novo_modelo = Novo modelo
editor-descartar_pergunta = Há alterações não salvas. Descartá-las e começar um modelo novo?
editor-descartar = Descartar
editor-atalhos_do_editor = Atalhos do editor
editor-f1_fecha = F1 ou Esc fecha esta janela.
editor-nao_simula = Não é possível simular
editor-corrija = Corrija os problemas do modelo antes de simular:
editor-fechar = Fechar
editor-valores_gravados = Valores gravados no modelo (Ctrl+Z desfaz)
editor-nenhum_problema = Nenhum problema: o modelo pode ser simulado.
editor-modelo = Modelo
editor-biblioteca = Biblioteca
editor-propriedades = Propriedades
editor-selecione_um_bloco = Selecione um bloco para editar suas propriedades.
editor-ligar = Ligar (C)
editor-excluir = Excluir (Del)
editor-clique_no_destino = Clique no bloco de destino (Esc cancela).
editor-problemas = Problemas ({ $n })
editor-avisos = Avisos ({ $n })
editor-colocar_dica = { $tipo }: clique e depois clique no chão, ou arraste até o chão
editor-encaixar = Encaixar na grade
editor-passo = Passo (m)
editor-esc_cancela = Esc cancela. Shift + clique no chão coloca vários.
editor-atalhos_f1 = Atalhos: F1.
editor-selecionados = { $n } objetos selecionados
editor-copiar = Copiar
editor-duplicar = Duplicar
editor-excluir_todos = Excluir todos
editor-alinhar = Alinhar
editor-esquerda = Esquerda
editor-centro_x = Centro X
editor-direita = Direita
editor-topo = Topo
editor-centro_y = Centro Y
editor-base = Base
editor-distribuir_x = Distribuir em x
editor-distribuir_y = Distribuir em y
editor-distribuir_dica = Espaça igualmente entre os dois extremos (3 objetos ou mais)
editor-propriedades_de_todos = Propriedades de todos
editor-campos_de_todos = Os campos mostram os valores de { $primeiro }; cada alteração vale para os { $n } objetos.
editor-buscar_por_nome = Buscar por nome
editor-todos_os_tipos = Todos os tipos
editor-de_objetos = { $n } de { $total } objetos
editor-objeto_dica = { $tipo }. Clique seleciona, Shift soma à seleção, duplo clique enquadra
editor-subir = Subir
editor-descer = Descer
editor-subir_dica = Troca de lugar com a anterior (Ctrl+Z desfaz)
editor-descer_dica = Troca de lugar com a seguinte (Ctrl+Z desfaz)
editor-remover_ligacao = Remover ligação
editor-ligacoes = Ligações
editor-nenhuma_ligacao = Nenhuma. Use "Ligar" e clique no destino.
editor-ordem_importa = A ordem importa. Saídas: a ordem é o número da porta (1, 2, ...) que as regras de saída usam. Entradas: a primeira tem prioridade quando o objeto puxa um item.
editor-saidas_por_porta = Saídas, por porta:
editor-porta = porta { $n }: { $de } -> { $para }
editor-entradas_por_prioridade = Entradas, por prioridade:
editor-entrada = { $n }a: { $de } -> { $para }
atalho-01-teclas = Clique
atalho-01 = seleciona o bloco (ou remove a ligação clicada)
atalho-02-teclas = Clique repetido no mesmo ponto
atalho-02 = alterna entre os objetos sobrepostos (uma dica mostra qual está selecionado)
atalho-03-teclas = Shift + clique
atalho-03 = põe ou tira o bloco da seleção
atalho-04-teclas = Shift + arrastar no chão
atalho-04 = seleção por caixa (soma à seleção)
atalho-05-teclas = Arrastar um bloco
atalho-05 = move o bloco ou a seleção inteira
atalho-06-teclas = Arrastar no chão
atalho-06 = desloca a vista
atalho-07-teclas = Botão direito + arrastar
atalho-07 = gira a vista
atalho-08-teclas = Roda do mouse
atalho-08 = aproxima e afasta, em direção ao cursor
atalho-09-teclas = Aba Objetos: clique, Shift + clique
atalho-09 = seleciona, soma à seleção; o duplo clique enquadra o objeto
atalho-10-teclas = Biblioteca: clique e clique no chão
atalho-10 = coloca um objeto (Shift mantém a ferramenta)
atalho-11-teclas = Biblioteca: arrastar até o chão
atalho-11 = coloca um objeto onde soltar
atalho-12-teclas = Setas
atalho-12 = move a seleção um passo da grade
atalho-13-teclas = Del
atalho-13 = exclui a seleção
atalho-14-teclas = C
atalho-14 = liga o bloco selecionado ao próximo clicado
atalho-15-teclas = Esc
atalho-15 = cancela a ferramenta, fecha os diálogos e sai das câmeras seguir e primeira pessoa
atalho-16-teclas = F
atalho-16 = enquadra o modelo (volta à câmera em órbita)
atalho-17-teclas = 1, 2, 3, 4
atalho-17 = câmera: órbita, seguir um recurso, primeira pessoa, do recurso
atalho-18-teclas = W A S D
atalho-18 = anda na primeira pessoa (Shift corre, a roda muda a velocidade)
atalho-19-teclas = L
atalho-19 = trava o mouse na primeira pessoa
atalho-20-teclas = F1
atalho-20 = mostra ou esconde esta janela
atalho-21-teclas = F3
atalho-21 = mostra ou esconde os tempos de quadro
atalho-22-teclas = F4
atalho-22 = liga ou desliga os níveis de detalhe
atalho-23-teclas = Ctrl + A
atalho-23 = seleciona todos os objetos
atalho-24-teclas = Ctrl + C
atalho-24 = copia a seleção
atalho-25-teclas = Ctrl + X
atalho-25 = recorta a seleção
atalho-26-teclas = Ctrl + V
atalho-26 = cola (nomes únicos, ligações internas preservadas)
atalho-27-teclas = Ctrl + D
atalho-27 = duplica a seleção
atalho-28-teclas = Ctrl + Z
atalho-28 = desfaz
atalho-29-teclas = Ctrl + Y ou Ctrl + Shift + Z
atalho-29 = refaz
atalho-30-teclas = Ctrl + S
atalho-30 = salva
atalho-31-teclas = Ctrl + Shift + S
atalho-31 = salva com outro nome (Salvar como)
atalho-32-teclas = Ctrl + N
atalho-32 = novo modelo

# Parte 2: o painel de propriedades (properties/).

## Propriedades: famílias de distribuição, regras e tipos

dist-constante = Constante
dist-uniforme = Uniforme
dist-exponencial = Exponencial
dist-triangular = Triangular
dist-normal = Normal
dist-lognormal = Lognormal
dist-gama = Gama
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
dist-deslocada = Deslocada
prop-vaga-primeira-livre = Primeira livre
prop-vaga-aleatoria = Aleatória
prop-despacho-primeiro-livre = Primeiro livre
prop-despacho-rodizio = Rodízio
prop-despacho-mais-proximo = Mais próximo
tipo-chegada = Chegada
tipo-fila = Fila
tipo-processo = Processo
tipo-montagem = Montagem
tipo-desmontagem = Desmontagem
tipo-saida = Saída
tipo-transportador = Transportador
tipo-ponte-rolante = Ponte rolante
tipo-despachante = Despachante
tipo-esteira = Esteira
tipo-porta-paletes = Porta-paletes
prop-modo-igual-ao-padrao = igual ao padrão
prop-modo-distribuicao = distribuição
prop-modo-taxa = taxa
prop-modo-sem-chegadas = sem chegadas
prop-turno-pausar = Pausar
prop-turno-terminar = Terminar
prop-turno-pausar-dica = O trabalho em andamento congela no fim do turno e continua quando o turno volta
prop-turno-terminar-dica = No fim do turno o objeto não aceita itens novos, termina o item em andamento (hora extra) e só então sai de turno; o turno seguinte começa na hora de sempre

## properties/network.rs

prop-network-o_nome_nao_pode_ficar = o nome não pode ficar vazio
prop-network-ja_existe_um_no_chamado = já existe um nó chamado "{ $t }"
prop-network-nome = Nome
prop-network-no_da_rede_de_caminhos = Nó da rede de caminhos
prop-network-arestas = Arestas
prop-network-nenhuma_use_a_ferramenta_aresta = Nenhuma: use a ferramenta "Aresta" e clique em outro nó.
prop-network-remover_aresta = Remover aresta
prop-network-objetos_ligados_a_este_no = Objetos ligados a este nó
prop-network-nenhum_cada_objeto_atendido_entra = Nenhum (cada objeto atendido entra pelo nó mais próximo).
prop-network-aresta = Aresta { $edge }
prop-network-comprimento_m_calculado_pelas_posicoes = Comprimento: { $len } m (calculado pelas posições dos nós)
prop-network-mao_dupla = Mão dupla ({ $from } <-> { $to })
prop-network-mao_unica = Mão única ({ $from } -> { $to })
prop-network-mao_unica-2 = Mão única ({ $to } -> { $from })
prop-network-sentido = Sentido
prop-network-limitar_veiculos_agv = Limitar veículos (AGV)
prop-network-quantos_agvs_podem_estar_na = Quantos AGVs podem estar na aresta ao mesmo tempo
prop-network-sem_limite = sem limite
prop-network-so_agvs_agv_true_respeitam = Só AGVs (agv: true) respeitam a capacidade; os demais passam livremente.
prop-network-objeto_discreto = Objeto discreto
prop-network-escolha = (escolha)
prop-network-nao_e_um_objeto_discreto = "{ $current }" não é um objeto discreto do modelo
prop-network-vazao = Vazão
prop-network-capacidade = Capacidade
prop-network-nivel_inicial = Nível inicial
prop-network-entrada_maxima = Entrada máxima
prop-network-saida_maxima = Saída máxima
prop-network-vazao_maxima = Vazão máxima
prop-network-cada_item_que_entra_no = Cada item que entra no objeto vira este volume, entregue de uma vez.
prop-network-volume_por_item = Volume por item
prop-network-a_cada_volume_completo_cria = A cada volume completo, cria um item dentro do objeto.
prop-network-problema = Problema: { $message }
prop-network-tipo = Tipo
prop-network-ligacoes = Ligações
prop-network-nenhuma_use_a_ferramenta_ligar = Nenhuma: use a ferramenta "Ligar" e clique no próximo nó.
prop-network-remover_ligacao = Remover ligação

## properties/panel.rs

prop-panel-buscar = Buscar
prop-panel-texto_invalido_nao_aplicado = Texto inválido, não aplicado: { $error }
prop-panel-descartar_texto = descartar texto
prop-panel-volta_ao_valor_que_esta = Volta ao valor que está no modelo
prop-panel-nao_e_um_numero = "{ $s }" não é um número
prop-panel-infinita = infinita
prop-panel-valor = valor
prop-panel-peso = peso
prop-panel-remover = remover
prop-panel-adicionar_valor = adicionar valor
prop-panel-observacoes_em_ordem_crescente_separadas = Observações em ordem crescente, separadas por espaço:
prop-panel-deslocamento = deslocamento
prop-panel-min = mín.
prop-panel-max = máx.
prop-panel-media = média
prop-panel-moda = moda
prop-panel-desvio = desvio
prop-panel-forma = forma
prop-panel-escala = escala
prop-panel-alfa = alfa
prop-panel-beta = beta
prop-panel-forma_1 = forma 1
prop-panel-forma_2 = forma 2
prop-panel-media-2 = média = { $mean }
prop-panel-sem_limite = sem limite
prop-panel-sem_limite_velocidade_constante = sem limite (velocidade constante)
prop-panel-equipe_transportadores_do_modelo = Equipe (transportadores do modelo):
prop-panel-nenhum_transportador_no_modelo_crie = Nenhum transportador no modelo. Crie um na paleta.
prop-panel-nao_e_um_transportador = "{ $missing }" não é um transportador
prop-panel-a_equipe_nao_pode_ficar = A equipe não pode ficar vazia.
prop-panel-precisa_de_operadores = Precisa de operadores
prop-panel-crie_um_transportador_ou_despachante = Crie um Transportador ou Despachante para usar como equipe.
prop-panel-equipe = equipe
prop-panel-nenhuma = (nenhuma)
prop-panel-nao_e_um_transportador_nem = "{ $team }" não é um transportador nem despachante
prop-panel-quantidade = quantidade
prop-panel-durante_a_preparacao = durante a preparação
prop-panel-durante_o_processamento = durante o processamento
prop-panel-transporte_de_saida = Transporte de saída
prop-panel-nenhum = nenhum
prop-panel-os_itens_passam_direto_ao = Os itens passam direto ao próximo bloco.
prop-panel-nao_e_um_transportador_nem-2 = "{ $t }" não é um transportador nem despachante
prop-panel-sem_rede_de_caminhos_a = Sem rede de caminhos: a base é a posição do transportador.
prop-panel-base_no_da_rede = Base (nó da rede)
prop-panel-fora_da_rede = fora da rede
prop-panel-escolher_um_no_poe_o = Escolher um nó põe o transportador nele e liga "Voltar à base".
prop-panel-editar_como_texto = editar como texto
prop-panel-quantidade_de_itens_exigida_em = Quantidade de itens exigida em cada entrada de componentes:
prop-panel-entrada = entrada { $i }
prop-panel-adicionar_entrada = adicionar entrada
prop-panel-o_objeto_precisa_de_ligacoes = O objeto precisa de { $list } ligações de entrada (1 contêiner + { $list2 }).
prop-panel-valor_do_rotulo = Valor do rótulo
prop-panel-remover_rotulo = remover rótulo
prop-panel-nome_do_rotulo = nome do rótulo
prop-panel-adicionar_rotulo = adicionar rótulo
prop-panel-lote_agrupar_itens = Lote (agrupar itens)
prop-panel-tamanho_do_lote = tamanho do lote
prop-panel-espera_maxima_0_esperar_sempre = espera máxima (0 = esperar sempre)
prop-panel-fechar_a_entrada_ate_o = fechar a entrada até o lote sair
prop-panel-primeira_livre = Primeira livre
prop-panel-rodizio = Rodízio
prop-panel-aleatoria = Aleatória
prop-panel-pesos = Pesos
prop-panel-por_rotulo = Por rótulo
prop-panel-script = Script
prop-panel-regra_de_saida = Regra de saída
prop-panel-peso_de_cada_saida_na = Peso de cada saída (na ordem das ligações):
prop-panel-saida = saída { $i } ({ $to })
prop-panel-saida-2 = saída { $i }
prop-panel-ha_pesos_mas_o_objeto = Há { $w } pesos, mas o objeto tem { $n_out } saídas.
prop-panel-ajustar_ao_numero_de_saidas = ajustar ao número de saídas
prop-panel-rotulo_com_o_numero_da = rótulo com o número da porta
prop-panel-o_nome_do_rotulo_nao = o nome do rótulo não pode ficar vazio
prop-panel-script_que_devolve_o_numero = Script que devolve o número da porta (1, 2, ...)
prop-panel-script_ao_entrar_um_item = Script ao entrar um item (on_entry)
prop-panel-roda_para_cada_item_que = Roda para cada item que entra, antes de qualquer outra regra.
prop-panel-quebras_mtbf_mttr = Quebras (MTBF/MTTR)
prop-panel-tempo_entre_falhas_mtbf = Tempo entre falhas (MTBF)
prop-panel-tempo_de_reparo_mttr = Tempo de reparo (MTTR)
prop-panel-transferencia_pela_distancia = Transferência pela distância
prop-panel-os_itens_levam_atraso_distancia = Os itens levam atraso + distância / velocidade para chegar ao próximo bloco pelas ligações diretas (sem transportador)
prop-panel-velocidade = Velocidade
prop-panel-atraso = Atraso
prop-panel-caminho = Caminho
prop-panel-reta = reta
prop-panel-pela_rede = pela rede
prop-panel-tempo_entre_chegadas = Tempo entre chegadas
prop-panel-primeira_chegada_em_t_0 = Primeira chegada em t = 0
prop-panel-maximo_de_chegadas = Máximo de chegadas
prop-panel-se_a_saida_estiver_bloqueada = Se a saída estiver bloqueada
prop-panel-esperar = Esperar
prop-panel-descartar_perda = Descartar (perda)
prop-panel-rotulos = Rótulos
prop-panel-capacidade_limitada = Capacidade limitada
prop-panel-disciplina = Disciplina
prop-panel-fifo_primeiro_a_entrar_primeiro = FIFO (primeiro a entrar, primeiro a sair)
prop-panel-lifo_ultimo_a_entrar_primeiro = LIFO (último a entrar, primeiro a sair)
prop-panel-capacidade = Capacidade
prop-panel-preparacao_setup = Preparação (setup)
prop-panel-tempo_de_processamento = Tempo de processamento
prop-panel-operadores = Operadores
prop-panel-modo = Modo
prop-panel-embalar_so_o_conteiner_sai = Embalar (só o contêiner sai)
prop-panel-juntar_componentes_destruidos = Juntar (componentes destruídos)
prop-panel-lote_todos_saem = Lote (todos saem)
prop-panel-desembalar = Desembalar
prop-panel-dividir_em_copias = Dividir em cópias
prop-panel-numero_de_itens_gerados = número de itens gerados
prop-panel-a_saida_nao_tem_parametros = A saída não tem parâmetros.
prop-panel-velocidade_maxima = Velocidade máxima
prop-panel-aceleracao = Aceleração
prop-panel-desaceleracao = Desaceleração
prop-panel-capacidade_itens = Capacidade (itens)
prop-panel-tempo_de_carga = Tempo de carga
prop-panel-tempo_de_descarga = Tempo de descarga
prop-panel-voltar_a_base_quando_ocioso = Voltar à base quando ocioso
prop-panel-agv_controle_de_trafego_na = AGV (controle de tráfego na rede)
prop-panel-comprimento = Comprimento
prop-panel-comprimento_por_item = Comprimento por item
prop-panel-acumula_itens_quando_a_saida = Acumula itens quando a saída bloqueia
prop-panel-capacidade_itens-2 = Capacidade: { $item_length } itens
prop-panel-colunas = Colunas
prop-panel-niveis = Níveis
prop-panel-permanencia_minima = Permanência mínima
prop-panel-regra_de_ocupacao = Regra de ocupação
prop-panel-largura_da_coluna = Largura da coluna
prop-panel-altura_do_nivel = Altura do nível
prop-panel-capacidade_posicoes = Capacidade: { $levels } posições
prop-panel-regra_de_escolha = Regra de escolha
prop-panel-o_nome_nao_pode_ficar = o nome não pode ficar vazio
prop-panel-ja_existe_um_objeto_chamado = já existe um objeto chamado "{ $t }"
prop-panel-nome = Nome
prop-panel-problema = Problema: { $message }
prop-panel-padrao_do_tipo = (padrão do tipo)
prop-panel-aparencia = Aparência
prop-panel-ativo = Ativo:
prop-panel-clipes_de_animacao_ainda_nao = Clipes de animação ainda não são tocados: pose de repouso.
prop-panel-biblioteca_do_utsim = Biblioteca do UTSim
prop-panel-utsim_gerado_por_utsim_assets = UTSim (gerado por utsim-assets)
prop-panel-terceiros_assets_terceiros = Terceiros (assets/terceiros)
prop-panel-cor = Cor
prop-panel-escala-2 = Escala
prop-panel-giro_graus = Giro (graus)
prop-panel-restaurar_padrao = Restaurar padrão
prop-panel-volta_a_aparencia_padrao_do = Volta à aparência padrão do tipo de objeto
prop-panel-simulacao = Simulação
prop-panel-tempo_de_simulacao = Tempo de simulação
prop-panel-aquecimento = Aquecimento
prop-panel-semente = Semente
prop-panel-replicacoes = Replicações

## properties/position.rs

prop-position-posicao_m = Posição (m)
prop-position-giro_graus = Giro (graus)

## properties/schedule.rs

prop-schedule-segundos = segundos
prop-schedule-minutos = minutos
prop-schedule-horas = horas
prop-schedule-use_dia_n_hh_mm = use "dia N hh:mm" (recebido "{ $t }")
prop-schedule-dia_invalido_em_a_partir = dia inválido em "{ $t }" (a partir de 1)
prop-schedule-use_um_numero = use um número
prop-schedule-hh_mm_so_vale_com = hh:mm só vale com ciclo de dia ou semana (1440 min, 24 h, 86400 s); use um número
prop-schedule-hora_invalida_em = hora inválida em "{ $t }"
prop-schedule-minuto_invalido_em = minuto inválido em "{ $t }"
prop-schedule-minuto_invalido_em_0_a = minuto inválido em "{ $t }" (0 a 59)
prop-schedule-nao_e_um_horario_hh = "{ $t }" não é um horário (hh:mm) nem um número

## properties/schedule_ui.rs

prop-schedule-ui-media = { $family }, média { $mean }
prop-schedule-ui-chegadas_por_periodo = Chegadas por período
prop-schedule-ui-a_taxa_de_chegada_muda = a taxa de chegada muda com a hora
prop-schedule-ui-uma_tabela_de_periodos_que = Uma tabela de períodos que se repete a cada ciclo (um dia, uma semana). Fora dos períodos não há chegadas.
prop-schedule-ui-desligado_o_tempo_entre_chegadas = Desligado: o tempo entre chegadas acima vale o tempo todo.
prop-schedule-ui-ciclo = ciclo
prop-schedule-ui-dia = dia
prop-schedule-ui-semana = semana
prop-schedule-ui-ciclo_de_tempo_do_modelo = ciclo de { $v } (tempo do modelo em minutos)
prop-schedule-ui-ciclo_de_tempo_do_modelo-2 = Ciclo de { $cycle }: tempo do modelo em { $name }. Horários em hh:mm, taxas por hora.
prop-schedule-ui-horarios_e_taxas_em_unidades = Horários e taxas em unidades do modelo (com ciclo 1440, um dia em minutos, a tabela mostra hh:mm).
prop-schedule-ui-taxa_por_hora = taxa por hora
prop-schedule-ui-taxa = taxa
prop-schedule-ui-inicio = início
prop-schedule-ui-fim = fim
prop-schedule-ui-modo = modo
prop-schedule-ui-valor = valor
prop-schedule-ui-esperado = esperado
prop-schedule-ui-copiar_do_padrao = copiar do padrão
prop-schedule-ui-usa_no_periodo_uma_copia = Usa no período uma cópia do tempo entre chegadas padrão, para ajustar só aqui
prop-schedule-ui-remover_o_periodo = remover o período
prop-schedule-ui-adicionar_periodo = adicionar período
prop-schedule-ui-por_dia = {" "}(por dia)
prop-schedule-ui-por_semana = {" "}(por semana)
prop-schedule-ui-esperado_por_ciclo_chegadas = Esperado por ciclo{ $hint }: { $per_cycle } chegadas
prop-schedule-ui-na_troca_de_periodo_a = Na troca de período a chegada pendente é sorteada de novo com o processo do novo período (períodos seguidos com o mesmo processo mantêm a chegada).
prop-schedule-ui-distribuicoes_dos_periodos = Distribuições dos períodos
prop-schedule-ui-periodo_tempo_entre_chegadas = Período { $i }: tempo entre chegadas
prop-schedule-ui-rotulos_por_periodo = Rótulos por período
prop-schedule-ui-os_rotulos_de_um_periodo = Os rótulos de um período substituem os rótulos padrão de mesmo nome (por exemplo, mais urgentes à noite); os outros continuam valendo.
prop-schedule-ui-a_fonte_nao_tem_rotulos = A fonte não tem rótulos padrão.
prop-schedule-ui-a = { $a } a { $b }
prop-schedule-ui-a-2 = { $start } a { $end }
prop-schedule-ui-periodo = Período { $i } ({ $span })

## properties/shifts_ui.rs

prop-shifts-ui-como_no_calendario = como no calendário ({ $to_lowercase })
prop-shifts-ui-ao_fim_do_turno = Ao fim do turno
prop-shifts-ui-periodo = período
prop-shifts-ui-inicio = início
prop-shifts-ui-duracao = duração
prop-shifts-ui-remover = remover
prop-shifts-ui-adicionar_janela = adicionar janela
prop-shifts-ui-fora_de_turno_do_periodo = fora de turno: { $off_fraction }% do período
prop-shifts-ui-turnos_janelas_fora_de_turno = Turnos (janelas fora de turno)
prop-shifts-ui-usar_calendario = usar calendário
prop-shifts-ui-crie_calendarios_na_secao_calendarios = Crie calendários na seção Calendários do painel Simulação (clique num ponto vazio da cena)
prop-shifts-ui-personalizado = personalizado
prop-shifts-ui-periodo_janela_s_fora_de = período { $period }, { $off } janela(s) fora de turno
prop-shifts-ui-o_calendario_nao_existe = o calendário '{ $name }' não existe
prop-shifts-ui-nome_do_calendario = nome do calendário
prop-shifts-ui-salvar_como_calendario = salvar como calendário
prop-shifts-ui-outros_objetos_podem_usar_o = Outros objetos podem usar o mesmo calendário
prop-shifts-ui-calendarios = Calendários ({ $calendars })
prop-shifts-ui-tabelas_de_turno_com_nome = Tabelas de turno com nome. Um objeto usa uma delas em Turnos > usar calendário; mudar o calendário muda todos os objetos que o usam.
prop-shifts-ui-nenhum_objeto_usa_este_calendario = nenhum objeto usa este calendário
prop-shifts-ui-usado_por = usado por: { $users }
prop-shifts-ui-remover_calendario = remover calendário
prop-shifts-ui-troque_os_objetos_para_outro = Troque os objetos para outro calendário ou para personalizado antes
prop-shifts-ui-novo_calendario = novo calendário
prop-shifts-ui-o_nome_nao_pode_ficar = o nome não pode ficar vazio
prop-shifts-ui-ja_existe_um_calendario_chamado = já existe um calendário chamado "{ $t }"
prop-shifts-ui-nome = Nome

## properties/spec.rs

prop-spec-continuas = Contínuas
prop-spec-discretas_e_empirica = Discretas e empírica
prop-spec-combinacao = Combinação

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

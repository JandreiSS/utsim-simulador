## UTSim 3D (cliente Godot): configurações e campos de distribuição

barra-configuracoes = Configurações
barra-configuracoes-dica = Preferências do UTSim 3D neste computador
configuracoes-titulo = Configurações
configuracoes-distribuicoes = Campos de distribuição abrem em
configuracoes-formulario = Formulário
configuracoes-codigo = Código
configuracoes-distribuicoes-ajuda = Vale para todos os campos de distribuição (tempo de processo, chegadas, carga, descarga...). O botão {"{"} {"}"} de cada campo continua trocando só aquele campo, até outro objeto ser escolhido.
configuracoes-fechar = Fechar
props-editar_como_texto = Editar como código (texto RON)
props-editar_como_formulario = Voltar ao formulário

## Painéis ao vivo em janela própria

vivo-soltar = Janela
vivo-soltar-dica = Abre este painel numa janela própria, que pode ir para outro monitor; ele continua ao vivo
vivo-prender = Voltar
vivo-prender-dica = Fecha a janela e devolve o painel para cima da vista 3D
vivo-janela-titulo = UTSim 3D: { $objeto }

## Idioma

configuracoes-idioma = Idioma
configuracoes-idioma-ajuda = Por ora, só a janela Configurações, os painéis ao vivo e as abas Qualidade, Segurança e Manutenção dos nós mudam de idioma; o resto das telas segue em português.

## Curva de fadiga da pessoa

fadiga-secao = Curva de fadiga (estudo de tempos)
fadiga-nota = Só leitura. As leituras e o ajuste se editam no UTSim Estatística ou com o comando utsim pessoa ajustar.
fadiga-tarefa = Tarefa { $tarefa }: { $leituras } leituras em { $horarios } horários
fadiga-tarefa-metodo = Tarefa { $tarefa }, método { $metodo }: { $leituras } leituras em { $horarios } horários
fadiga-resumo = Descansada: { $t0 }. Com a fadiga máxima, mais { $acrescimo } %. Cansaço λ = { $lambda } por hora; recuperação μ = { $mu } por hora ({ $origem }).
fadiga-estimada = estimada
fadiga-padrao = padrão
fadiga-hora = hora do turno
fadiga-pausa = Pausa
fadiga-curva = Curva ajustada
fadiga-media = Média medida
fadiga-erro = Não ajustada: { $erro }
fadiga-abrir = Abrir no UTSim Estatística
fadiga-abrir-ajuda = Abre as leituras deste estudo de tempos numa planilha do UTSim Estatística (uma linha por tempo medido: tarefa, método, hora do turno e tempo), para analisar. O modelo não muda.

## Abas dos nós (Qualidade, Segurança, Manutenção) e painéis ao vivo

aba-so-leitura = { $quem }. Só leitura até a pessoa fechar o modelo.
aba-abrir-qualidade = Abrir no UTSim Qualidade
aba-abrir-qualidade-dica = Abre este nó no UTSim Qualidade (defeitos, inspeção, custos, CEP). O modelo é salvo antes e fica com o outro app para editar; o UTSim 3D relê o que ele gravar.
aba-abrir-seguranca = Abrir no UTSim Segurança
aba-abrir-seguranca-dica = Abre este nó no UTSim Segurança do Trabalho (perigos, matriz de risco, EPI). O modelo é salvo antes e fica com o outro app para editar; o UTSim 3D relê o que ele gravar.
aba-calculando = Calculando os números da simulação...
aba-nenhuma = nenhuma
aba-nenhum-escolha = (nenhum)
aba-nenhum = nenhum
aba-opcoes = Separe por vírgulas. Opções: { $opcoes }.
aba-vazio-nao-usa = vazio: não usa
aba-adicionar = + Adicionar
aba-adicionar-dica = Acrescenta um item em { $lista }
aba-abrir-item = Abre o item
aba-fechar-item = Fecha o item
aba-sem-nome = (sem nome)
aba-remover = Remover
aba-remover-dica = Tira este item da lista
vivo-fechar = Fechar
vivo-fechar-dica = Fechar o painel (Esc fecha o mais recente)
vivo-fixar = Fixar
vivo-fixado = Fixado
vivo-fixar-dica = Fixar mantém este painel; o próximo clique num objeto abre outro ao lado
vivo-sem-numeros = Sem números para este objeto.
vivo-eixo-tempo = tempo
vivo-eixo-itens = itens
vivo-sem-dados = Sem dados ainda.
vivo-estados = Estados na janela
vivo-sem-estados = sem estados

## Painel ao vivo: linhas (rótulo e ajuda)

vivo-em-processamento-agora = Em processamento agora
vivo-itens-no-objeto-neste-instante = Itens no objeto neste instante (processando ou prontos sem poder sair).
vivo-itens-em-espera-fila = Itens em espera (fila)
vivo-itens-nas-filas-que-alimentam = Itens nas filas que alimentam o objeto, neste instante.
vivo-entradas = Entradas
vivo-itens-que-entraram-na-janela = Itens que entraram na janela.
vivo-ja-processados = Já processados
vivo-itens-que-sairam-do-objeto = Itens que saíram do objeto na janela.
vivo-rejeitados-para-descarte = Rejeitados para descarte
vivo-itens-que-chegaram-a-uma = Itens que chegaram a uma saída de refugo (sink_role: Scrap) vindos daqui, também por filas ou transporte.
vivo-nenhuma-saida-tem-sink-role = Nenhuma saída tem sink_role: Scrap; marque a saída de refugo no modelo para contar os descartes.
vivo-rejeitados-para-retrabalho = Rejeitados para retrabalho
vivo-itens-que-chegaram-a-um = Itens que chegaram a um posto com retrabalho vindos daqui, também por filas ou transporte.
vivo-ciclo-medio-permanencia = Ciclo médio (permanência)
vivo-tempo-medio-no-objeto-dos = Tempo médio no objeto dos itens que saíram na janela.
vivo-espera-media-na-fila = Espera média na fila
vivo-tempo-medio-nas-filas-que = Tempo médio nas filas que alimentam o objeto.
vivo-ocupado-processando = Ocupado (processando)
vivo-fracao-da-janela-no-estado = Fração da janela no estado processando.
vivo-ocioso = Ocioso
vivo-fracao-da-janela-sem-trabalho = Fração da janela sem trabalho.
vivo-bloqueado = Bloqueado
vivo-pronto-sem-poder-sair-bloqueado = Pronto sem poder sair (bloqueado ou aguardando transporte).
vivo-parado-quebra = Parado (quebra)
vivo-quebrado-aguardando-reparo-ou-em = Quebrado, aguardando reparo ou em manutenção.
vivo-setup = Setup
vivo-fracao-em-setup = Fração em setup.
vivo-conteudo-agora = Conteúdo agora
vivo-itens-no-objeto-neste-instante-2 = Itens no objeto neste instante.
vivo-conteudo-medio = Conteúdo médio
vivo-media-no-tempo-do-conteudo = Média no tempo do conteúdo, na janela.
vivo-conteudo-maximo = Conteúdo máximo
vivo-maior-conteudo-da-janela = Maior conteúdo da janela.
vivo-espera-media = Espera média
vivo-saidas = Saídas
vivo-itens-que-sairam-na-janela = Itens que saíram na janela.
vivo-fracao-da-janela-sem-poder = Fração da janela sem poder liberar o item criado.
vivo-recebidos-total = Recebidos (total)
vivo-itens-que-chegaram-a-saida = Itens que chegaram à saída na janela.
vivo-vazao-por-unidade-de-tempo = Vazão (por unidade de tempo)
vivo-recebidos-divididos-pela-duracao-da = Recebidos divididos pela duração da janela.
vivo-tempo-medio-no-sistema = Tempo médio no sistema
vivo-da-criacao-do-item-a = Da criação do item à chegada aqui.
vivo-papel-na-qualidade = Papel na qualidade
vivo-o-papel-da-saida-nos = O papel da saída nos relatórios de qualidade.
vivo-utilizacao = Utilização
vivo-andando-carregando-descarregando-ou-trabalhando = Andando, carregando, descarregando ou trabalhando num posto.
vivo-viagens = Viagens
vivo-deslocamentos-comecados-na-janela = Deslocamentos começados na janela.
vivo-itens-transportados = Itens transportados
vivo-itens-entregues-na-janela = Itens entregues na janela.
vivo-carregando-agora = Carregando agora
vivo-itens-consigo-neste-instante = Itens consigo neste instante.
vivo-viajando-carregado = Viajando carregado
vivo-fracao-da-janela-andando-com = Fração da janela andando com carga.
vivo-viajando-vazio = Viajando vazio
vivo-fracao-da-janela-andando-sem = Fração da janela andando sem carga.
vivo-trabalhando-num-posto = Trabalhando num posto
vivo-fracao-da-janela-operando-um = Fração da janela operando um processo (ou uma ponte rolante pela botoeira ou pelo rádio).
vivo-descanso-fadiga = Descanso (fadiga)
vivo-fracao-da-janela-em-pausa = Fração da janela em pausa para recuperar.
vivo-pausas = Pausas
vivo-vezes-que-entrou-em-descanso = Vezes que entrou em descanso na janela.
vivo-fadiga-media-maxima-fim-da = Fadiga média / máxima (fim da replicação)
vivo-a-fadiga-nao-esta-no = A fadiga não está no registro da reprodução: valores do relatório final.
vivo-esperando-o-operador = Esperando o operador
vivo-fracao-da-janela-parada-na = Fração da janela parada na carga até a pessoa da botoeira ou do rádio chegar ao lado dela.

## Painel ao vivo: cabeçalho e seções

godot-ao-vivo-pessoa = pessoa
godot-ao-vivo-ponte_rolante = ponte rolante
godot-ao-vivo-empilhadeira = empilhadeira
godot-ao-vivo-veiculo = veículo
godot-ao-vivo-aquecimento_ate_numeros_desde_0 = aquecimento até { $inicio_estatisticas }: números desde 0
godot-ao-vivo-desde_o_fim_do_aquecimento = desde o fim do aquecimento ({ $inicio_estatisticas })
godot-ao-vivo-desde_o_inicio = desde o início
godot-ao-vivo-t_de_replicacao = t = { $t } de { $fim } · replicação { $replicacao } · { $janela }
godot-ao-vivo-agora = Agora
godot-ao-vivo-marque_a_saida_de_refugo = marque a saída de refugo
godot-ao-vivo-acumulado = Acumulado
godot-ao-vivo-tempos = Tempos
godot-ao-vivo-em_processo_wip = Em processo (WIP)
godot-ao-vivo-agora_e_na_janela = Agora e na janela
godot-ao-vivo-conteudo = Conteúdo
godot-ao-vivo-saida = Saída
godot-ao-vivo-recebidos_acumulado = Recebidos (acumulado)
godot-ao-vivo-pessoa-2 = Pessoa
godot-ao-vivo-ponte_rolante-2 = Ponte rolante
godot-ao-vivo-veiculo-2 = Veículo
godot-ao-vivo-itens_consigo = Itens consigo
godot-ao-vivo-fila = Fila

## Fluxos de tokens

godot-ui-flows-fluxos_de_tokens = Fluxos de tokens
godot-ui-flows-fechar = Fechar
godot-ui-flows-fechar_os_fluxos = Fechar os fluxos
godot-ui-flows-o_modelo_nao_tem_fluxos = O modelo não tem fluxos de tokens.
godot-ui-flows-tokens_em_cada_atividade_no = Tokens em cada atividade no instante da reprodução; embaixo, as contagens ao longo da rodada (a linha vertical t é o instante atual).
godot-ui-flows-zoom_ctrl_roda_no_grafo = Zoom (Ctrl + roda no grafo)
godot-ui-flows-diminui = Diminui
godot-ui-flows-aumenta = Aumenta
godot-ui-flows-ajustar = Ajustar
godot-ui-flows-cabe_na_largura = Cabe na largura
godot-ui-flows-sem_contagens = sem contagens

## Resultados

godot-ui-results-resumo = Resumo
godot-ui-results-graficos = Gráficos
godot-ui-results-analise = Análise
godot-ui-results-cenarios = Cenários
godot-ui-results-otimizacao = Otimização
godot-ui-results-ocupacao = Ocupação
godot-ui-results-estados = Estados
godot-ui-results-serie_temporal = Série temporal
godot-ui-results-histograma = Histograma
godot-ui-results-nenhum_recurso_ocupado = nenhum recurso ocupado
godot-ui-results-invalido_use_um_numero_inteiro = { $what } "{ $text }" inválido: use um número inteiro
godot-ui-results-serie_invalida_use_objeto_medida = série "{ $other }" inválida: use objeto.medida
godot-ui-results-calcule_antes_quantas_replicacoes_faltam = calcule antes quantas replicações faltam
godot-ui-results-simule_o_modelo_antes_de = Simule o modelo antes de comparar cenários.
godot-ui-results-simule_o_modelo_antes_de-2 = Simule o modelo antes de otimizar.
godot-ui-results-trabalho_desconhecido = trabalho desconhecido: { $other }
godot-ui-results-serie_curta_demais_para_o = Série curta demais para o MSER-5 ({ $points } pontos): aumente o tempo de parada ou a amostragem.
godot-ui-results-corte_sugerido_mas_o_minimo = Corte sugerido: { $cut }, mas o mínimo ficou na segunda metade da série: a rodada é curta ou não se estabiliza.
godot-ui-results-corte_sugerido_mser_5_antes = Corte sugerido (MSER-5): { $cut }. Antes dele a média ainda sobe ou desce.
godot-ui-results-gargalo_movel_na_maior_parte = Gargalo móvel: { $n } na maior parte do tempo.
godot-ui-results-nenhum_recurso_ativo = Nenhum recurso ativo.
godot-ui-results-replicacoes_faltam_para_o_alvo = { $reports } replicações; faltam { $more } para o alvo.
godot-ui-results-aquecimento_analisado = Aquecimento analisado: { $d }
godot-ui-results-lei_de_little_verde_s = Lei de Little: { $verde } verde(s), { $amarelo } amarelo(s), { $vermelho } vermelho(s).
godot-ui-results-cenario_s_comparados = { $n ->
    [one] { $n } cenário comparado.
   *[other] { $n } cenários comparados.
}
godot-ui-results-otimizacao_melhor_ponto = Otimização: melhor ponto { $ponto }.
godot-ui-results-conteudo_de = conteúdo de { $object }
godot-ui-results-mser_5 = MSER-5: { $cut }
godot-ui-results-media_entre_replicacoes = média entre replicações
godot-ui-results-unico_compartilhado = { $name } (único { $sole }%, compartilhado { $shifting }%)
godot-ui-results-melhor_objetivo = melhor objetivo
godot-ui-results-pontos_avaliados = pontos avaliados
godot-ui-results-resultados = Resultados
godot-ui-results-fechar = Fechar
godot-ui-results-fechar_os_resultados_a_cena = Fechar os resultados (a cena continua)
godot-ui-results-gargalo_no_turno = gargalo no turno
godot-ui-results-objeto = Objeto
godot-ui-results-tipo = Tipo
godot-ui-results-entradas = Entradas
godot-ui-results-saidas = Saídas
godot-ui-results-conteudo_medio = Conteúdo médio
godot-ui-results-permanencia = Permanência
godot-ui-results-ocupacao-2 = ocupação { $valor }
godot-ui-results-replicacoes = Replicações
godot-ui-results-semente = semente { $seed }
godot-ui-results-tempo_de_parada = Tempo de parada
godot-ui-results-aquecimento = aquecimento { $warmup }
godot-ui-results-gargalo = Gargalo
godot-ui-results-gargalo_no_turno-2 = Gargalo no turno
godot-ui-results-saidas_por_replicacao = Saídas por replicação
godot-ui-results-soma_das_saidas_do_modelo = soma das saídas do modelo
godot-ui-results-o_gargalo_e_o_recurso = O gargalo é o recurso mais ocupado; dentro do turno conta só o tempo em que ele trabalha (um posto parado meio dia parece ocioso no geral, mas pode estar saturado no turno).
godot-ui-results-por_objeto = Por objeto
godot-ui-results-media_semiamplitude_do_ic_de = Média ± semiamplitude do IC de 95 % entre replicações.
godot-ui-results-exportar_csv_para = Exportar CSV para
godot-ui-results-exportar = Exportar
godot-ui-results-grava_resumo_replicacoes_series_e = Grava resumo, replicações, séries e histogramas em CSV
godot-ui-results-grafico = Gráfico
godot-ui-results-fracao_do_tempo_trabalhando_turno = Fração do tempo trabalhando (turno: só o tempo dentro do turno; bloqueio: somando o tempo bloqueado). A barra amarela é o gargalo; o traço é o IC de 95 %.
godot-ui-results-fracao_do_tempo_em_cada = Fração do tempo em cada estado, com as cores da cena 3D.
godot-ui-results-conteudo_de_no_tempo_cada = Conteúdo de { $object } no tempo: cada replicação em cinza e a média em amarelo.
godot-ui-results-histograma_de_juntando_todas_as = Histograma de { $object }.{ $tally }, juntando todas as replicações.
godot-ui-results-o_modelo_nao_grava_ligar = {" "}O modelo não grava { $falta }: "Ligar e rodar de novo" liga e simula outra vez.
godot-ui-results-falta-serie = a série do conteúdo
godot-ui-results-falta-histogramas = os histogramas
godot-ui-results-ligar_e_rodar_de_novo = Ligar e rodar de novo
godot-ui-results-liga_no_modelo_a_serie = Liga no modelo a série do conteúdo (run.sample_interval, pelo tempo de simulação) e os histogramas (run.histogram_bins = 20), num passo que se desfaz com Ctrl+Z, e roda as replicações de novo
godot-ui-results-sem_serie_ligue_run_sample = sem série: ligue run.sample_interval no modelo
godot-ui-results-sem_histograma_ligue_run_histogram = sem histograma: ligue run.histogram_bins no modelo
godot-ui-results-sem_dados = sem dados
godot-ui-results-ic_95 = IC 95 %
godot-ui-results-a = { $lo } a { $hi }: { $c }
godot-ui-results-aquecimento-2 = Aquecimento
godot-ui-results-roda_as_replicacoes_desde_o = Roda as replicações desde o tempo 0, sem aquecimento, e mostra a média entre elas (curva de Welch). O corte do MSER-5 é onde a série deixa de ter o começo vazio; "Aplicar" grava esse tempo em run.warmup.
godot-ui-results-serie = Série
godot-ui-results-wip_ou_objeto_medida = wip ou objeto.medida
godot-ui-results-janela = Janela
godot-ui-results-campo-janela = janela
godot-ui-results-campo-replicacoes = replicações
godot-ui-results-campo-orcamento = orçamento
godot-ui-results-analisar = Analisar
godot-ui-results-roda_e_calcula_welch_e = Roda e calcula Welch e MSER-5
godot-ui-results-aplicar = Aplicar
godot-ui-results-grava_o_corte_sugerido_em = Grava o corte sugerido em run.warmup do modelo
godot-ui-results-sem_serie = sem série
godot-ui-results-replicacoes_necessarias = Replicações necessárias
godot-ui-results-quantas_replicacoes_faltam_para_a = Quantas replicações faltam para a semiamplitude do IC de 95 % ficar abaixo do alvo (5% é relativo à média; 0.5 é absoluto). "Rodar mais" acrescenta as que faltam às já feitas, com sementes novas.
godot-ui-results-precisao = Precisão
godot-ui-results-calcular = Calcular
godot-ui-results-calcula_as_replicacoes_necessarias = Calcula as replicações necessárias
godot-ui-results-rodar_mais = Rodar mais { $more }
godot-ui-results-rodar_mais-2 = Rodar mais
godot-ui-results-roda_as_replicacoes_que_faltam = Roda as replicações que faltam e junta às atuais
godot-ui-results-para_todas_as_medidas_chegarem = Para todas as medidas chegarem ao alvo faltam { $more ->
    [one] { $more } replicação.
   *[other] { $more } replicações.
}
godot-ui-results-todas_as_medidas_ja_estao = Todas as medidas já estão dentro do alvo.
godot-ui-results-consistencia_lei_de_little = Consistência: lei de Little
godot-ui-results-confere_l_w_em_cada = Confere L = λ·W em cada objeto e no sistema. Verde: fecha; amarelo: a diferença é estatisticamente diferente de zero, mas pequena (até 2 %); vermelho: não fecha, o que costuma indicar aquecimento curto ou itens presos no fim.
godot-ui-results-conferir = Conferir
godot-ui-results-roda_as_replicacoes_e_confere = Roda as replicações e confere a lei de Little
godot-ui-results-gargalo_movel = Gargalo móvel
godot-ui-results-em_cada_instante_o_gargalo = Em cada instante, o gargalo é o recurso no período ativo mais longo; quando dois períodos se sobrepõem, o gargalo é compartilhado. Mostra a fração do tempo de cada recurso como gargalo único e compartilhado.
godot-ui-results-roda_as_replicacoes_com_o = Roda as replicações com o rastro e calcula o gargalo móvel
godot-ui-results-medida = Medida
godot-ui-results-atual = Atual
godot-ui-results-alvo = Alvo (±)
godot-ui-results-faltam = Faltam
godot-ui-results-diferenca = Diferença
godot-ui-results-semaforo = Semáforo
godot-ui-results-um_cenario_por_bloco_nome = Um cenário por bloco: [nome] e, embaixo, as mudanças (objeto.campo = valor em RON, ou run.campo). O modelo como está entra primeiro, como base. Todos usam as mesmas sementes (números aleatórios comuns), então a diferença para a base é pareada; em amarelo, as diferenças significativas.
godot-ui-results-medidas_virgulas_vazio_todas = Medidas (vírgulas; vazio: todas)
godot-ui-results-comparar = Comparar
godot-ui-results-roda_todos_os_cenarios_e = Roda todos os cenários e compara
godot-ui-results-texto = Δ { $valor }
godot-ui-results-procura_os_valores_das_variaveis = Procura os valores das variáveis que minimizam (ou maximizam) a média de uma métrica. Uma variável por linha: chave mínimo máximo [passo] (inteiros) ou com ponto decimal (contínua, cinco valores na grade). Os pontos não significativamente diferentes do melhor (teste pareado de 95 %) ficam marcados.
godot-ui-results-objetivo = Objetivo
godot-ui-results-maximizar = maximizar
godot-ui-results-metodo = Método
godot-ui-results-replicacoes_por_ponto = Replicações por ponto
godot-ui-results-orcamento_pontos = Orçamento (pontos)
godot-ui-results-otimizar = Otimizar
godot-ui-results-roda_a_busca = Roda a busca
godot-ui-results-melhor_ponto_ponto_s_simulados = Melhor ponto: { $ponto } ({ $simulated ->
    [one] { $simulated } ponto simulado
   *[other] { $simulated } pontos simulados
})
godot-ui-results-sem_historico = sem histórico
godot-ui-results-ponto = Ponto
godot-ui-results-nenhum = nenhum
godot-ui-results-nenhum-viavel = nenhum viável
godot-ui-results-util-trabalho = trabalho
godot-ui-results-util-turno = turno
godot-ui-results-util-bloqueio = bloqueio
godot-ui-results-metodo-grade = grade
godot-ui-results-metodo-recozimento = recozimento
godot-ui-results-metodo-genetico = genético
godot-ui-results-replicacao_oes_tempo_de_parada = 
    { $model }: { $replications ->
        [one] { $replications } replicação
       *[other] { $replications } replicações
    }, tempo de parada { $stop_time }, aquecimento { $warmup }, semente { $seed }.
    Gargalo (geral): { $b }
    Gargalo (dentro do turno): { $bs }
godot-ui-results-replicacao_oes = { $model } · { $reports ->
    [one] { $reports } replicação
   *[other] { $reports } replicações
}

## Tipos de bloco nos relatórios (results.rs)

godot-tipo-source = chegada
godot-tipo-queue = fila
godot-tipo-processor = processo
godot-tipo-combiner = montagem
godot-tipo-separator = desmontagem
godot-tipo-sink = saída
godot-tipo-transporter = transportador
godot-tipo-dispatcher = despachante
godot-tipo-conveyor = esteira
godot-tipo-rack = porta-paletes
godot-tipo-span = medida de tempo
godot-tipo-flow = fluxo
godot-tipo-fluid = fluidos

## Propriedades dos objetos (schema.rs)

godot-campo-movimentacao = Movimentação
godot-campo-folga-dos-obstaculos = Folga dos obstáculos
godot-campo-ponto-de-acesso-proprio = Ponto de acesso próprio
godot-campo-x-no-objeto = X no objeto
godot-campo-y-no-objeto = Y no objeto
godot-campo-dimensoes = Dimensões
godot-campo-dimensoes-proprias = Dimensões próprias
godot-campo-capacidade-pela-area = Capacidade pela área
godot-campo-area-por-item = Área por item
godot-campo-cabem = Cabem
godot-campo-tempo-entre-chegadas = Tempo entre chegadas
godot-campo-primeira-chegada-em-t-0 = Primeira chegada em t = 0
godot-campo-maximo-de-chegadas = Máximo de chegadas
godot-campo-se-a-saida-estiver-bloqueada = Se a saída estiver bloqueada
godot-opcao-esperar = Esperar
godot-opcao-descartar-perda = Descartar (perda)
godot-campo-rotulos = Rótulos
godot-campo-chegadas-por-periodo = Chegadas por período
godot-campo-tabela-de-chegadas = Tabela de chegadas
godot-campo-capacidade-limitada = Capacidade limitada
godot-campo-politica-de-acesso = Política de acesso
godot-opcao-fifo-primeiro-a-entrar-primeiro = FIFO (primeiro a entrar, primeiro a sair)
godot-opcao-lifo-ultimo-a-entrar-primeiro = LIFO (último a entrar, primeiro a sair)
godot-opcao-prioridade-maior-rotulo-primeiro = Prioridade: maior rótulo primeiro
godot-opcao-prioridade-menor-rotulo-primeiro-prazo = Prioridade: menor rótulo primeiro (prazo)
godot-campo-rotulo-da-prioridade = Rótulo da prioridade
godot-campo-lote = Lote
godot-campo-itens-por-lote = Itens por lote
godot-campo-espera-maxima = Espera máxima
godot-campo-fecha-a-entrada-ate-o = Fecha a entrada até o lote sair
godot-campo-capacidade = Capacidade
godot-campo-preparacao-setup = Preparação (setup)
godot-campo-troca-de-produto = Troca de produto
godot-campo-tempo-de-processamento = Tempo de processamento
godot-campo-tempo-por-receita = Tempo por receita
godot-campo-operadores = Operadores
godot-campo-quem-opera = Quem opera
godot-campo-quantos-por-peca = Quantos por peça
godot-campo-ganho-da-equipe = Ganho da equipe
godot-campo-durante-a-preparacao = Durante a preparação
godot-campo-durante-o-processamento = Durante o processamento
godot-campo-outras-equipes-ao-mesmo-tempo = Outras equipes ao mesmo tempo
godot-campo-componentes-por-entrada = Componentes por entrada
godot-campo-modo = Modo
godot-opcao-embalar-so-o-conteiner-sai = Embalar (só o contêiner sai)
godot-opcao-juntar-componentes-destruidos = Juntar (componentes destruídos)
godot-opcao-lote-todos-saem = Lote (todos saem)
godot-opcao-desembalar = Desembalar
godot-opcao-dividir-em-copias = Dividir em cópias
godot-opcao-dividir-pelo-rotulo = Dividir pelo rótulo
godot-campo-copias = Cópias
godot-campo-sem-parametros = Sem parâmetros
godot-campo-velocidade-maxima = Velocidade máxima
godot-campo-tempo-de-carga = Tempo de carga
godot-campo-tempo-de-descarga = Tempo de descarga
godot-campo-voltar-a-base-quando-ocioso = Voltar à base quando ocioso
godot-campo-agv-controle-de-trafego-na = AGV (controle de tráfego na rede)
godot-campo-trajeto-desenhado = Trajeto desenhado
godot-campo-editar-nos = Editar nós
godot-campo-comprimento = Comprimento
godot-campo-velocidade = Velocidade
godot-campo-comprimento-por-item = Comprimento por item
godot-campo-acumula-itens-quando-a-saida = Acumula itens quando a saída bloqueia
godot-campo-colunas = Colunas
godot-campo-niveis = Níveis
godot-campo-permanencia-minima = Permanência mínima
godot-campo-regra-de-ocupacao = Regra de ocupação
godot-opcao-primeira-livre = Primeira livre
godot-opcao-aleatoria = Aleatória
godot-opcao-curva-abc-rotulo-classe = Curva ABC (rótulo classe)
godot-campo-fracao-das-posicoes-da-classe = Fração das posições da classe A
godot-campo-fracao-das-posicoes-da-classe-2 = Fração das posições da classe B
godot-campo-largura-da-coluna = Largura da coluna
godot-campo-altura-do-nivel = Altura do nível
godot-campo-equipe = Equipe
godot-campo-regra-de-escolha = Regra de escolha
godot-opcao-primeiro-livre = Primeiro livre
godot-opcao-rodizio = Rodízio
godot-opcao-mais-proximo = Mais próximo
godot-campo-ordem-dos-movimentos = Ordem dos movimentos
godot-opcao-segura-sobe-anda-desce = Segura (sobe, anda, desce)
godot-opcao-um-eixo-de-cada-vez = Um eixo de cada vez
godot-opcao-os-tres-juntos = Os três juntos
godot-campo-altura-de-passagem-do-gancho = Altura de passagem do gancho
godot-campo-rotulo-do-peso-do-item = Rótulo do peso do item
godot-campo-tempo-de-engate = Tempo de engate
godot-campo-tempo-de-desengate = Tempo de desengate
godot-campo-voltar-a-posicao-propria-quando = Voltar à posição própria quando ociosa
godot-campo-controle = Controle
godot-opcao-cabine-operador-nao-modelado = Cabine (operador não modelado)
godot-opcao-botoeira-a-pessoa-anda-com = Botoeira (a pessoa anda com a carga)
godot-opcao-radio-a-pessoa-encontra-a = Rádio (a pessoa encontra a ponte na carga)
godot-campo-equipe-do-operador = Equipe do operador
godot-campo-meia-largura-da-area-sob = Meia largura da área sob a carga
godot-campo-afastar-as-pessoas-do-caminho = Afastar as pessoas do caminho da carga
godot-campo-folga-entre-pontes = Folga entre pontes
godot-campo-caminho = Caminho
godot-campo-regra-de-saida = Regra de saída
godot-campo-pesos = Pesos
godot-campo-rotulo-com-o-numero-da = Rótulo com o número da porta
godot-campo-script = Script
godot-campo-nenhum-passagem-direta = Nenhum (passagem direta)
godot-campo-transporte-de-saida = Transporte de saída
godot-campo-transferencia-pela-distancia = Transferência pela distância
godot-campo-atraso = Atraso
godot-opcao-reta = reta
godot-opcao-pela-rede = pela rede
godot-campo-script-ao-entrar-um-item = Script ao entrar um item
godot-campo-quebras = Quebras
godot-campo-tempo-entre-falhas-mtbf = Tempo entre falhas (MTBF)
godot-campo-tempo-de-reparo-mttr = Tempo de reparo (MTTR)
godot-campo-turnos = Turnos
godot-campo-origem = Origem
godot-opcao-personalizado = Personalizado
godot-opcao-calendario-do-modelo = Calendário do modelo
godot-campo-periodo = Período
godot-campo-pausas = Pausas
godot-campo-no-fim-do-turno = No fim do turno
godot-campo-calendario = Calendário
godot-campo-ativo = Ativo
godot-campo-cor = Cor
godot-campo-escala = Escala
godot-campo-modelo-da-peca = Modelo da peça
godot-campo-giro-do-desenho = Giro do desenho
godot-campo-restaurar-padrao = Restaurar padrão
godot-campo-nome = Nome
godot-campo-tipo = Tipo
godot-campo-rotacao = Rotação
godot-campo-sexo = Sexo
godot-opcao-nao-informado = Não informado
godot-opcao-feminino = Feminino
godot-opcao-masculino = Masculino
godot-campo-altura = Altura
godot-ajuda-a-altura-do-boneco-no = A altura do boneco no 3D. Desligada, ela é sorteada pelo nome.
godot-campo-massa = Massa
godot-ajuda-com-a-altura-da-a = Com a altura, dá a largura do boneco no 3D pelo índice de massa corporal.
godot-campo-idade = Idade
godot-campo-experiencia-na-funcao = Experiência na função
godot-ajuda-so-registro-a-simulacao-nao = Só registro: a simulação não lê.
godot-campo-carga-maxima-medida = Carga máxima medida
godot-ajuda-uma-carga-acima-dela-da = Uma carga acima dela dá aviso no modelo.
godot-campo-forca-maxima-medida = Força máxima medida
godot-ajuda-empurrar-ou-puxar-so-registro = Empurrar ou puxar. Só registro.
godot-campo-tipo-de-espera = Tipo de espera
godot-opcao-fila-simples-sem-lugares = Fila simples (sem lugares)
godot-opcao-mesa = Mesa
godot-opcao-palete = Palete
godot-opcao-estante = Estante
godot-opcao-chao-area-demarcada = Chão (área demarcada)
godot-campo-camadas = Camadas
godot-campo-passo-entre-pecas = Passo entre peças
godot-campo-lugares = Lugares
godot-campo-opera-as-maquinas = Opera as máquinas
godot-campo-leva-os-itens-de = Leva os itens de
godot-campo-nome-do-modelo = Nome do modelo
godot-campo-descricao = Descrição
godot-campo-tempo-de-simulacao = Tempo de simulação
godot-campo-aquecimento = Aquecimento
godot-campo-semente = Semente
godot-campo-replicacoes = Replicações
godot-campo-movimentacao-padrao = Movimentação padrão
godot-campo-perfil-da-marcacao-do-piso = Perfil da marcação do piso
godot-opcao-brasil-faixa-amarela = Brasil (faixa amarela)
godot-opcao-branco-redacao-original-da-nr = Branco (redação original da NR-26)
godot-campo-largura-da-faixa = Largura da faixa
godot-campo-parada-no-cruzamento = Parada no cruzamento
godot-campo-cor-do-chao = Cor do chão
godot-ajuda-como-o-recurso-anda-o = Como o recurso anda: o padrão do modelo (Simulação), em linha reta como no formato antigo, livre pelo piso desviando dos obstáculos, pela rede de caminhos ou só pelos corredores da sua classe.
godot-ajuda-onde-os-recursos-moveis-param = Onde os recursos móveis param para atender este objeto, em metros a partir do seu centro e girando com ele. Desligado, é o lado -y. Também se arrasta na cena, com "Mostrar obstáculos e folgas" ligado.
godot-ajuda-comprimento-largura-e-altura-do = Comprimento, largura e altura do objeto em metros, no sentido dele. A área no piso muda o ponto de acesso e, com ele, as distâncias e os tempos dos operadores e veículos; numa espera com área por item, também a capacidade. O 3D estica o modelo para essas medidas. Desligado, vale a área padrão do tipo.
godot-ajuda-a-capacidade-da-espera-sai = A capacidade da espera sai da sua área no piso dividida pela área que cada item ocupa (só itens inteiros). Com uma capacidade dada também, vale a menor.
godot-ajuda-intervalo-entre-duas-chegadas-seguidas = Intervalo entre duas chegadas seguidas.
godot-ajuda-valor-sorteado-para-cada-item = Valor sorteado para cada item, por nome: {"{"}"tipo": Discrete([(1.0, 3.0), (2.0, 1.0)]){"}"}.
godot-ajuda-ciclo-e-periodos-com-o = Ciclo e períodos com o próprio intervalo, taxa ou fechados: (cycle: 1440.0, periods: [(start: 0.0, end: 480.0, arrivals: Rate(0.5))]).
godot-ajuda-horario-instante-quantidade-tipo-e = Horário (instante, quantidade, tipo e rótulos de cada linha) ou sequência de linhas: (mode: Times, repeat: Cycle(1440.0), rows: [(at: 480.0, count: 12)]).
godot-ajuda-desligada-a-espera-nao-tem = Desligada, a espera não tem limite.
godot-ajuda-qual-item-sai-primeiro-pela = Qual item sai primeiro. Pela prioridade, sai antes o item com o maior (ou o menor) valor do rótulo; empates e itens sem o rótulo saem na ordem de chegada.
godot-ajuda-segura-os-itens-e-libera = Segura os itens e libera em grupos.
godot-ajuda-libera-um-lote-incompleto-depois = Libera um lote incompleto depois deste tempo (0 = espera sempre).
godot-ajuda-preparacao-so-quando-o-produto = Preparação só quando o produto muda: o rótulo do produto e, se quiser, o tempo de cada troca (de, para); sem linha na matriz vale a preparação acima. Exemplo: (label: "produto", matrix: [(from: 1, to: 2, time: Const(30.0))]). None: preparação em toda peça.
godot-ajuda-tempo-de-processamento-que-depende = Tempo de processamento que depende do item: o rótulo da receita e o tempo de cada valor; os itens sem receita na tabela usam o tempo acima. Exemplo: (label: "produto", times: [(value: 1, time: Const(90.0)), (value: 2, time: Const(120.0))]). None: o mesmo tempo para todos.
godot-ajuda-quem-opera-esta-maquina-marque = Quem opera esta máquina: marque e escolha abaixo, ou arraste do nó laranja do operador (ou da equipe) até a máquina. O processo espera um deles livre.
godot-ajuda-operadores-que-a-peca-precisa = Operadores que a peça precisa ao mesmo tempo: ela só começa quando todos chegam, e eles ficam com ela até o fim. Com mais operadores na equipe do que capacidade × este número, os de sobra ficam parados.
godot-ajuda-quantas-vezes-mais-rapido-a = Quantas vezes mais rápido a equipe faz a etapa: o tempo sorteado é dividido por este número. 1: sem ganho (o tempo é o mesmo com um ou com vários).
godot-ajuda-equipes-que-precisam-estar-juntas = Equipes que precisam estar juntas com os operadores (um médico e um enfermeiro na mesma sala). Exemplo: [OperatorUse(team: "enfermagem")]. A etapa só começa quando todos chegam.
godot-ajuda-a-entrada-1-traz-o = A entrada 1 traz o contêiner; a entrada k + 1 traz o k-ésimo número de itens. Exemplo: [2, 1].
godot-ajuda-desligada-a-velocidade-muda-na = Desligada, a velocidade muda na hora.
godot-ajuda-abre-a-ferramenta-de-desenho = Abre a ferramenta de desenho com esta esteira: arraste os nós, mude a altura e o raio.
godot-ajuda-transportadores-que-o-despachante-comanda = Transportadores que o despachante comanda.
godot-ajuda-segura-o-gancho-sobe-a = Segura: o gancho sobe à altura de passagem, a ponte e o carro andam juntos e o gancho desce.
godot-ajuda-rotulo-com-o-peso-de = Rótulo com o peso de cada item, em kg: itens acima da capacidade geram um aviso.
godot-ajuda-na-botoeira-a-ponte-nao = Na botoeira, a ponte não anda mais rápido que a pessoa; no rádio, ela mantém a própria velocidade e espera a pessoa chegar.
godot-ajuda-meio-lado-do-quadrado-sob = Meio lado do quadrado sob o gancho onde ninguém deve ficar com a carga suspensa (análise de exposição).
godot-ajuda-antes-de-andar-carregada-a = Antes de andar carregada, a ponte espera quem passa pela faixa da carga; e quem ia entrar nela para na borda até a carga passar.
godot-ajuda-distancia-mantida-entre-os-ganchos = Distância mantida entre os ganchos de duas pontes no mesmo caminho: uma não passa a outra.
godot-ajuda-para-qual-ligacao-vai-cada = Para qual ligação vai cada item, na ordem das ligações de saída.
godot-ajuda-peso-de-cada-saida-na = Peso de cada saída, na ordem das ligações. Exemplo: [3.0, 1.0].
godot-ajuda-devolve-o-numero-da-porta = Devolve o número da porta (1, 2, ...).
godot-ajuda-transportador-ou-despachante-que-leva = Transportador ou despachante que leva os itens para o próximo bloco.
godot-ajuda-os-itens-levam-atraso-distancia = Os itens levam atraso + distância / velocidade para chegar ao próximo bloco pelas ligações diretas (sem transportador).
godot-ajuda-roda-para-cada-item-que = Roda para cada item que entra, antes de qualquer outra regra.
godot-ajuda-falhas-aleatorias-com-o-tempo = Falhas aleatórias, com o tempo contado no relógio.
godot-ajuda-janelas-fora-do-turno-repetidas = Janelas fora do turno, repetidas a cada período.
godot-ajuda-inicio-e-duracao-de-cada = Início e duração de cada pausa dentro do período, em ordem.
godot-ajuda-corrige-para-onde-o-modelo = Corrige para onde o modelo 3D olha; o recurso vira para onde anda.
godot-ajuda-volta-a-aparencia-padrao-do = Volta à aparência padrão do tipo de objeto.
godot-ajuda-giro-em-graus-anti-horario = Giro em graus, anti-horário visto de cima (R gira 90°, Ctrl+R 15°).
godot-ajuda-escolhe-o-corpo-no-3d = Escolhe o corpo no 3D (não informado, o corpo sai do nome). Nenhum padrão de fadiga depende dele; com Feminino, carga acima de 20 kg dá o aviso da CLT, art. 390.
godot-ajuda-numa-espera-real-cada-peca = Numa espera real cada peça ocupa um lugar (o arranjo do tipo, recalculado pelas Dimensões): só sai a peça sem nada em cima, a mais antiga delas primeiro, e quem retira vai até a frente da peça. Muda o resultado. Fila simples: como antes.
godot-ajuda-quantas-pecas-empilhadas-em-cada = Quantas peças empilhadas em cada lugar (na estante, em cada prateleira).
godot-ajuda-distancia-entre-os-centros-de = Distância entre os centros de duas peças vizinhas.
godot-ajuda-arraste-do-no-laranja-ate = Arraste do nó laranja até a máquina para ligar; clique na linha tracejada e no × para tirar.
godot-ajuda-arraste-do-no-laranja-ate-2 = Arraste do nó laranja até o objeto (uma espera, uma chegada, ou com Shift uma máquina) para ligar.
godot-ajuda-texto-livre-sobre-o-modelo = Texto livre sobre o modelo, gravado no arquivo.
godot-ajuda-as-estatisticas-so-contam-depois = As estatísticas só contam depois deste tempo.
godot-ajuda-como-andam-os-recursos-moveis = Como andam os recursos móveis que não escolheram um modo próprio. Vazio, vale Livre (menor caminho no piso, desviando dos obstáculos).
godot-ajuda-cores-das-faixas-e-setas = Cores das faixas e setas dos corredores. O programa não certifica conformidade: confira o texto vigente das normas.
godot-ajuda-a-pratica-usual-vai-de = A prática usual vai de 0,05 a 0,15 m.
godot-ajuda-o-veiculo-para-esse-tempo = O veículo para esse tempo antes de um cruzamento com pedestres.
godot-ajuda-a-cor-do-piso-na = A cor do piso na cena 3D, gravada no modelo; a grade muda de tom para continuar visível. Desligada, o cinza de sempre.
godot-opcao-por-rotulo = Por rótulo
godot-opcao-padrao-do-modelo = Padrão do modelo
godot-opcao-legado-linha-reta-ou-rede = Legado (linha reta ou rede)
godot-opcao-livre-menor-caminho-no-piso = Livre (menor caminho no piso)
godot-opcao-pela-rede-de-caminhos = Pela rede de caminhos
godot-opcao-pelos-corredores = Pelos corredores


## Propriedades dos objetos: seções, tipos, famílias, parâmetros, turnos e visual (schema.rs)

godot-secao-objeto = Objeto
godot-secao-posicao = Posição
godot-secao-parametros = Parâmetros
godot-secao-saida = Saída
godot-secao-quebras-e-turnos = Quebras e turnos
godot-secao-aparencia = Aparência
godot-secao-simulacao = Simulação
godot-secao-movimentacao = Movimentação
godot-secao-piso-e-corredores = Piso e corredores
godot-secao-modelo = Modelo
godot-secao-pessoa = Pessoa
godot-objeto-chegada = Chegada
godot-objeto-espera = Espera
godot-objeto-processo = Processo
godot-objeto-montagem = Montagem
godot-objeto-desmontagem = Desmontagem
godot-objeto-saida = Saída
godot-objeto-transportador = Transportador
godot-objeto-ponte-rolante = Ponte rolante
godot-objeto-despachante = Despachante
godot-objeto-esteira = Esteira
godot-objeto-porta-paletes = Porta-paletes
godot-dist-constante = Constante
godot-dist-uniforme = Uniforme
godot-dist-exponencial = Exponencial
godot-dist-triangular = Triangular
godot-dist-normal = Normal
godot-dist-lognormal = Lognormal
godot-dist-gama = Gama
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
godot-dist-deslocada = Deslocada
godot-param-valor = valor
godot-param-minimo = mínimo
godot-param-maximo = máximo
godot-param-media = média
godot-param-moda = moda
godot-param-desvio-padrao = desvio padrão
godot-param-forma = forma
godot-param-escala = escala
godot-param-fases = fases (k)
godot-param-forma-1 = forma 1
godot-param-forma-2 = forma 2
godot-param-probabilidade = probabilidade (p)
godot-param-tentativas = tentativas (n)
godot-param-sucessos = sucessos (r)
godot-param-alfa = alfa
godot-param-beta = beta
godot-param-deslocamento = deslocamento
godot-dim-comprimento-do-caminho = Comprimento do caminho
godot-dim-vao = Vão
godot-dim-altura-do-caminho = Altura do caminho
godot-dim-comprimento = Comprimento
godot-dim-largura = Largura
godot-dim-altura = Altura
godot-turno-pausar = Pausar
godot-turno-terminar = Terminar o trabalho
godot-turno-do-calendario = (o do calendário)
godot-ajuda-folga-papel = Distância mínima de qualquer obstáculo. Desligada, vale a do papel do recurso ({ $m } m: meia largura mais 0,3 m).
godot-dim-esteira = { $l } × { $w } m: o comprimento é o da esteira (Parâmetros)
godot-dim-porta-paletes = { $l } × { $w } × { $h } m: pelos vãos e níveis (Parâmetros)
godot-dim-padrao = { $l } × { $w } m (padrão)
godot-campo-sem-limite = sem limite
godot-campo-n-itens = { $n ->
    [one] { $n } item
   *[other] { $n } itens
}
godot-campo-nao-existe = { $nome } (não existe)
godot-campo-n-posicoes = { $n ->
    [one] { $n } posição
   *[other] { $n } posições
}
godot-campo-movimento = Movimento { $o_que }.
godot-campo-n-lugares = { $n ->
    [one] { $n } lugar
   *[other] { $n } lugares
}
godot-campo-divide-caminho = Divide o caminho com { $com } (folga de { $m } m entre os ganchos)
godot-campo-pesos-e-saidas = Há { $pesos ->
    [one] { $pesos } peso
   *[other] { $pesos } pesos
}, mas o objeto tem { $n ->
    [one] { $n } saída
   *[other] { $n } saídas
}. { $ajuda }
godot-turno-sem-calendarios = O modelo não tem calendários: crie um em "calendars" para usá-lo aqui.
godot-turno-calendario-ajuda = Um calendário é a mesma tabela de turnos usada por vários objetos.
godot-visual-padrao-do-tipo = (padrão do tipo)
godot-visual-gerado = UTSim (gerado por utsim-assets)
godot-visual-grupo-biblioteca = Biblioteca do UTSim
godot-visual-grupo-terceiros = Terceiros (assets/terceiros)
godot-visual-grupo-elementos = Elementos 3D (assets/modelos3d)
godot-visual-caixa-padrao = caixa (padrão)
godot-visual-modelo-da-peca-ajuda = Elemento 3D com que as peças criadas aqui são desenhadas; a caixa é o padrão.
godot-visual-no-tamanho-da-peca = { $arquivo } ({ $x } x { $y } x { $z } m), desenhado no tamanho da peça
godot-eixo-z = Z (altura)


## ui_props.rs

godot-ui-props-modelo_nada_selecionado = Modelo (nada selecionado)
godot-ui-props-propriedades = Propriedades: { $name }
godot-ui-props-problemas = Problemas
godot-ui-props-avisos = Avisos
godot-ui-props-mostrar_motorista = Mostrar motorista
godot-ui-props-desenha_o_operador_sentado_e = Desenha o operador sentado e animado na empilhadeira. Só a aparência: não muda a simulação nem é gravado no modelo.
godot-ui-props-clique_para_destacar_na_cena = Clique para destacar na cena o que este aviso cita
godot-ui-props-abre-a-secao = Abre a seção { $title }
godot-ui-props-fecha-a-secao = Fecha a seção { $title }
godot-ui-props-liga_ou_desliga = Liga ou desliga { $campo }
godot-ui-props-aplicar = Aplicar
godot-ui-props-aplicar_script = Aplicar script
godot-ui-props-nenhum_transportador_no_modelo = Nenhum transportador no modelo.
godot-ui-props-este_campo_vem_de_dados = Este campo vem de dados reais: { $label }
godot-ui-props-distribuicao = Distribuição
godot-ui-props-dados_reais = Dados reais
godot-ui-props-fonte = Fonte
godot-ui-props-de_onde_vem_a_distribuicao = De onde vem a distribuição deste campo: parâmetros informados ou valores reais, ajustados automaticamente
godot-ui-props-media = média { $mean }
godot-ui-props-arquivo_csv = Arquivo CSV
godot-ui-props-valores_colados = Valores colados
godot-ui-props-valores_reais = Valores reais…
godot-ui-props-abre_a_janela_grande_para = Abre a janela grande para digitar, colar ou importar os valores e comparar todas as distribuições de uma vez
godot-ui-props-origem = Origem
godot-ui-props-arquivo = Arquivo
godot-ui-props-arquivo_csv_ou_txt = arquivo .csv ou .txt
godot-ui-props-escolher = Escolher...
godot-ui-props-escolhe_o_arquivo_de_dados = Escolhe o arquivo de dados (CSV ou TXT)
godot-ui-props-automatica = (automática)
godot-ui-props-coluna = Coluna
godot-ui-props-coluna_do_arquivo_com_os = Coluna do arquivo com os valores
godot-ui-props-um_valor_por_linha_ou = um valor por linha (ou separados por ; ou espaço)
godot-ui-props-sao_instantes_usar_os_intervalos = São instantes (usar os intervalos entre eles)
godot-ui-props-marque_quando_os_valores_sao = Marque quando os valores são instantes crescentes (como horas de chegada): o ajuste usa as diferenças entre eles. Desmarcado, os valores são durações.
godot-ui-props-melhor_automatica = Melhor automática
godot-ui-props-empirica_os_proprios_valores = Empírica (os próprios valores)
godot-ui-props-so = Só { $f }
godot-ui-props-escolha = Escolha
godot-ui-props-criterio = Critério
godot-ui-props-o_que_ordena_as_familias = O que ordena as famílias aceitas pelos testes
godot-ui-props-reajustar_com_estas_opcoes = Reajustar com estas opções
godot-ui-props-ajusta_a_distribuicao_aos_dados = Ajusta a distribuição aos dados (em segundo plano) e passa a usá-la neste campo
godot-ui-props-nao_foi_possivel_ler_os = Não foi possível ler os dados: { $err }
godot-ui-props-os_dados_mudaram_desde_o = Os dados mudaram desde o último ajuste.
godot-ui-props-reajustar_aos_dados_atuais = Reajustar aos dados atuais
godot-ui-props-o_campo_foi_reajustado_salve = { $c }. O campo foi reajustado; salve para registrar.
godot-ui-props-n = { $family } · n = { $n } · { $label }
godot-ui-props-analisando_os_dados = Analisando os dados...
godot-ui-props-candidatas = Candidatas ({ $criterion })
godot-ui-props-aic_bic_usa_esta_familia =
    { $params }
    AIC { $aic }   BIC { $bic }
    Usa esta família neste campo
godot-ui-props-usar_a_empirica_os_proprios = Usar a empírica (os próprios valores)
godot-ui-props-relatorio = Relatório
godot-ui-props-grava_o_relatorio_completo_do = Grava o relatório completo do ajuste (HTML) numa pasta e abre no navegador
godot-ui-props-histograma_dos_dados_e_a = Histograma dos dados e a curva da distribuição escolhida
godot-ui-props-buscar_ativo = Buscar ativo
godot-ui-props-miniatura_do_objeto_com_o = Miniatura do objeto com o ativo escolhido (ou o que está sob o mouse na lista)
godot-ui-props-ligacoes_de_saida_ordem_das = Ligações de saída (ordem das portas)
godot-ui-props-nenhuma_use_a_ferramenta_ligar = Nenhuma. Use a ferramenta Ligar (L).
godot-ui-props-subir = Subir
godot-ui-props-sobe_esta_saida_na_ordem = Sobe esta saída na ordem das portas
godot-ui-props-descer = Descer
godot-ui-props-desce_esta_saida_na_ordem = Desce esta saída na ordem das portas
godot-ui-props-remover = Remover
godot-ui-props-remove_esta_ligacao = Remove esta ligação
godot-ui-props-entradas = Entradas: { $inputs }

## Rodada 13: janelas do UTSim 3D

## dados.rs

godot-dados-criterio_desconhecido = critério desconhecido: { $other }
godot-dados-use_um_arquivo_ou_valores = use um arquivo ou valores colados, não os dois
godot-dados-escolha_um_arquivo_de_dados = escolha um arquivo de dados ou cole os valores
godot-dados-registro_reaproveitado = registro reaproveitado
godot-dados-dados_mudaram = dados mudaram
godot-dados-opcoes_mudaram = opções mudaram
godot-dados-dados_ilegiveis = dados ilegíveis
godot-dados-os_dados_mudaram_desde_o = os dados mudaram desde o último ajuste (era { $old_family }, hash { $old_hash })
godot-dados-nao_foi_possivel_ler = não foi possível ler { $path }: { $e }
godot-dados-nenhum_campo_do_modelo_usa = nenhum campo do modelo usa dados reais
godot-dados-nao_foi_possivel_gravar_em = não foi possível gravar em { $dir }: { $e }

## document.rs

godot-document-transportadores_e_despachantes_nao_usam = Transportadores e despachantes não usam ligações de fluxo: arraste do nó laranja deles até a máquina que operam (ou até o objeto cujos itens levam).
godot-document-objeto_nao_existe = objeto '{ $n }' não existe
godot-document-arraste_de_um_objeto_ate = arraste de um objeto até outro
godot-document-saiu_da_equipe_de = { $to } saiu da equipe de { $from }.
godot-document-entrou_na_equipe_de = { $to } entrou na equipe de { $from }.
godot-document-nao_pede_mais_operadores_de = { $from } não pede mais operadores de { $to }.
godot-document-pede_operadores_de = { $from } pede operadores de { $to }.
godot-document-nao_pede_mais_transporte_a = { $from } não pede mais transporte a { $to }.
godot-document-pede_transporte_a = { $from } pede transporte a { $to }.
godot-document-arraste_do_despachante_a_um = arraste do despachante a um transportador, ou de um objeto ao transportador ou despachante que o atende
godot-document-a_serie_e_os_histogramas = a série e os histogramas já estão ligados no modelo
godot-document-desenhe_clicando_os_nos_no = desenhe clicando os nós no chão (Enter termina)
godot-document-tipo_desconhecido = tipo desconhecido: { $id }
godot-document-nada_selecionado = nada selecionado
godot-document-nao_ha_para_onde_mover = não há para onde mover esta ligação
godot-document-nenhum_arrasto_comecou = nenhum arrasto começou
godot-document-objeto_nao_existe-2 = objeto '{ $name }' não existe
godot-document-e_um_recurso_movel_ele = { $name } é um recurso móvel: ele vira para onde anda
godot-document-recursos_moveis_nao_giram_eles = recursos móveis não giram: eles viram para onde andam
godot-document-a_area_de_transferencia_esta = a área de transferência está vazia (Ctrl+C copia)
godot-document-alinhamento_desconhecido = alinhamento desconhecido: { $how }
godot-document-selecione_dois_objetos_ou_mais = selecione dois objetos ou mais para alinhar
godot-document-eixo_desconhecido = eixo desconhecido: { $axis }
godot-document-selecione_tres_objetos_ou_mais = selecione três objetos ou mais para distribuir
godot-document-nao_tem_distribuicao = '{ $field }' não tem distribuição
godot-document-modelo = Modelo
godot-document-nome_em_vez_de = nome: { $name } em vez de { $name2 }
godot-document-descricao_em_vez_de = descrição: { $description } em vez de { $description2 }
godot-document-run_em_vez_de = run: { $run } em vez de { $run2 }
godot-document-falta_o_objeto = falta o objeto { $name }
godot-document-objeto_em_vez_de = objeto { $name }: { $g } em vez de { $w }
godot-document-objeto_a_mais = objeto a mais: { $name }
godot-document-ligacoes_em_vez_de = ligações: { $connections } em vez de { $connections2 }
godot-document-outras_partes_do_modelo_calendarios = outras partes do modelo (calendários, rede, fluxos, fluidos) diferem
godot-document-clique = Clique
godot-document-seleciona_o_bloco_ou_a = seleciona o bloco ou a ligação
godot-document-clique_de_novo_no_mesmo = Clique de novo no mesmo ponto
godot-document-passa_para_o_proximo_bloco = passa para o próximo bloco sobreposto
godot-document-poe_ou_tira_o_bloco = põe ou tira o bloco da seleção
godot-document-shift_arrastar_no_chao = Shift + arrastar no chão
godot-document-selecao_por_caixa_soma_a = seleção por caixa (soma à seleção)
godot-document-arrastar_no_chao = Arrastar no chão
godot-document-selecao_por_caixa_troca_a = seleção por caixa (troca a seleção)
godot-document-arrastar_um_bloco = Arrastar um bloco
godot-document-move_o_bloco_ou_a = move o bloco ou a seleção inteira
godot-document-arrastar_da_porta_de_saida = Arrastar da porta de saída (verde)
godot-document-liga_ao_bloco_ou_a = liga ao bloco (ou à porta de entrada) onde soltar
godot-document-botao_direito_arrastar = Botão direito + arrastar
godot-document-gira_a_vista = gira a vista
godot-document-shift_botao_direito_arrastar = Shift + botão direito + arrastar
godot-document-desloca_a_vista = desloca a vista
godot-document-roda_do_mouse = Roda do mouse
godot-document-aproxima_e_afasta = aproxima e afasta
godot-document-biblioteca_clique_e_clique_no = Biblioteca: clique e clique no chão
godot-document-coloca_um_objeto_shift_mantem = coloca um objeto (Shift mantém a ferramenta)
godot-document-biblioteca_arrastar_ate_o_chao = Biblioteca: arrastar até o chão
godot-document-coloca_um_objeto_onde_soltar = coloca um objeto onde soltar
godot-document-setas = Setas
godot-document-move_a_selecao_um_passo = move a seleção um passo da grade
godot-document-girar_90_anti_horario_visto = girar 90° (anti-horário visto de cima)
godot-document-girar_90_horario = girar −90° (horário)
godot-document-girar_15_15 = girar 15° / −15°
godot-document-arrastar_a_alca_de_giro = Arrastar a alça de giro (roxa)
godot-document-gira_o_objeto_selecionado_em = gira o objeto selecionado em passos de 15° (com Shift, 90°)
godot-document-exclui_a_selecao_ou_a = exclui a seleção (ou a ligação selecionada)
godot-document-ferramenta_ligar_arraste_da_origem = ferramenta Ligar: arraste da origem ao destino, ou clique e clique
godot-document-ferramenta_corredor_clique_pontos_no = ferramenta Corredor: clique pontos no chão, Enter ou duplo clique termina
godot-document-ferramenta_zona_zonas_de_risco = ferramenta Zona: zonas de risco no piso; clique os cantos, Enter ou o primeiro ponto fecha
godot-document-e_ou_alt_com_ligar = E (ou Alt com Ligar)
godot-document-ferramenta_equipe_arraste_do_despachante = ferramenta Equipe: arraste do despachante ao transportador, ou do objeto a quem leva os itens (Shift: operadores do processo)
godot-document-cancela_a_ferramenta_a_ligacao = cancela a ferramenta, a ligação ou o corredor começado, ou a seleção
godot-document-simular_clique_num_objeto = Simular: clique num objeto
godot-document-abre_o_painel_de_resultados = abre o painel de resultados ao vivo do objeto no canto da vista
godot-document-simular_esc = Simular: Esc
godot-document-fecha_o_painel_ao_vivo = fecha o painel ao vivo mais recente (o não fixado primeiro)
godot-document-f_editar_ou_home = F (Editar) ou Home
godot-document-enquadra_o_modelo = enquadra o modelo
godot-document-simular_f = Simular: F
godot-document-anda_pela_fabrica_em_primeira = anda pela fábrica em primeira pessoa: WASD anda, o mouse olha, Shift corre, E/Q sobe e desce, V voa, a roda muda a velocidade, o clique na mira abre o painel, Esc sai
godot-document-liga_ou_desliga_o_encaixe = liga ou desliga o encaixe na grade
godot-document-alterna_entre_editar_e_simular = alterna entre Editar e Simular
godot-document-mostra_ou_esconde_esta_janela = mostra ou esconde esta janela
godot-document-seleciona_todos_os_objetos = seleciona todos os objetos
godot-document-copia_a_selecao = copia a seleção
godot-document-recorta_a_selecao = recorta a seleção
godot-document-cola_nomes_unicos_ligacoes_internas = cola (nomes únicos, ligações internas preservadas)
godot-document-duplica_a_selecao = duplica a seleção
godot-document-salva_como = salva como
godot-document-abre_um_modelo = abre um modelo
godot-document-novo_modelo = novo modelo

## editor3d.rs

godot-editor3d-opera = { $to } opera { $from }
godot-editor3d-leva_os_itens_de = { $to } leva os itens de { $from }
godot-editor3d-esta_na_equipe_de = { $to } está na equipe de { $from }
godot-editor3d-ligacao_de = Ligação de { $k }: { $from } → { $to }
godot-editor3d-arraste_do_no_laranja_dele = { $name }: arraste do nó laranja dele até uma máquina para que ele a opere (ou até um objeto, para levar os itens dele).
godot-editor3d-de_clique_no_no_de = {" "}({ $next } de { $links }: clique no nó de novo para a próxima)
godot-editor3d-ligacao_selecionada_clique_no_ou = Ligação { $a } → { $b } selecionada{ $extra }. Clique no × ou Del remove.
godot-editor3d-esta_ligacao_se_muda_no = Esta ligação ({ $kind }) se muda no painel de propriedades de { $from }.
godot-editor3d-ligacao_removida_ctrl_z_desfaz = Ligação { $a } → { $b } removida (Ctrl+Z desfaz).
godot-editor3d-nenhuma_zona_selecionada_para_excluir = Nenhuma zona selecionada para excluir.
godot-editor3d-nada_selecionado_para_excluir = Nada selecionado para excluir.
godot-editor3d-zona_cancelada = Zona cancelada.
godot-editor3d-ligacao_cancelada = Ligação cancelada.
godot-editor3d-girar_arraste_em_volta_do = Girar: arraste em volta do objeto (passos de 15°; com Shift, 90°).
godot-editor3d-solte_sobre_a_maquina_que = Solte sobre a máquina que { $res } vai operar (ou sobre um objeto, para levar os itens dele). Esc cancela.
godot-editor3d-ligar_solte_sobre_o_destino = Ligar: solte sobre o destino (ou clique nele). Esc cancela.
godot-editor3d-ligar_solte_sobre_a_origem = Ligar: solte sobre a origem (ou o nó de saída dela). Esc cancela.
godot-editor3d-ligar_clique_em_um_bloco = Ligar: clique em um bloco.
godot-editor3d-ligado_clique_em_outra_origem = Ligado: { $from } → { $to }. Clique em outra origem ou Esc.
godot-editor3d-ligacao_selecionada_clique_no_ou-2 = Ligação { $a } → { $b } selecionada. Clique no × ou Del remove.
godot-editor3d-clique_no_ou_del_remove = { $said }. Clique no × ou Del remove.
godot-editor3d-de_blocos_neste_ponto_clique = { $name } ({ $index } de { $hits } blocos neste ponto: clique de novo para o próximo).
godot-editor3d-arraste_deste_no_verde_ate = { $from }: arraste deste nó verde até o destino para ligar.
godot-editor3d-ligar_clique_no_destino_esc = Ligar: { $from } → clique no destino (Esc cancela).
godot-editor3d-ligado = Ligado: { $from } → { $to }.
godot-editor3d-ligacao_cancelada_solte_sobre_o = Ligação cancelada: solte sobre o bloco de destino.
godot-editor3d-nao_recebe_de_ninguem_arraste = { $to } não recebe de ninguém: arraste do nó de saída da origem até aqui.
godot-editor3d-ligacao_cancelada_solte_sobre_o-2 = Ligação cancelada: solte sobre o bloco de origem.
godot-editor3d-girado_para = { $name } girado para { $degrees }°.
godot-editor3d-arraste_a_alca_em_volta = { $name }: arraste a alça em volta do objeto para girá-lo (R gira 90°, Ctrl+R 15°).
godot-editor3d-objeto_s_movidos =
    { $selected } { $selected ->
        [one] objeto movido
       *[other] objetos movidos
    }.
godot-editor3d-objeto_s_na_caixa =
    { $inside } { $inside ->
        [one] objeto na caixa
       *[other] objetos na caixa
    }.
godot-editor3d-nada_mudou_solte_sobre_a = Nada mudou: solte sobre a máquina (ou o objeto) que { $res } vai atender.
godot-editor3d-solte_sobre_uma_maquina_ou = Solte sobre uma máquina ou um objeto (um transportador só entra na equipe de um despachante).
godot-editor3d-ja_esta_ligado_para_desfazer = Já está ligado: { $to } → { $res }. Para desfazer, clique na linha e no ×.
godot-editor3d-ligado_ctrl_z_desfaz = Ligado: { $said }. Ctrl+Z desfaz.
godot-editor3d-aponte_para_o_chao_para = Aponte para o chão para colocar o objeto.
godot-editor3d-equipe_clique_no_transportador_ou = Equipe: clique no transportador ou despachante de destino.
godot-editor3d-equipe_comece_num_despachante_ou = Equipe: comece num despachante ou num objeto.
godot-editor3d-equipe_clique_no_destino_esc = Equipe: { $from } → clique no destino (Esc cancela).
godot-editor3d-equipe_solte_sobre_o_transportador = Equipe: solte sobre o transportador ou despachante.
godot-editor3d-equipe_solte_sobre_outro_objeto = Equipe: solte sobre outro objeto.
godot-editor3d-aponte_para_o_chao_para-2 = Aponte para o chão para pôr um ponto do corredor.
godot-editor3d-corredor_clique_os_proximos_pontos = Corredor: clique os próximos pontos; Enter ou duplo clique termina, Backspace volta um ponto, Esc cancela.
godot-editor3d-ponto_s_enter_ou_duplo =
    { $aisle_points } { $aisle_points ->
        [one] ponto
       *[other] pontos
    }. Enter ou duplo clique termina.
godot-editor3d-vertice_de_selecionado_arraste_para = Vértice { $k } de { $name } selecionado (arraste para mover).
godot-editor3d-corredor_selecionado_del_exclui_shift = Corredor { $name } selecionado (Del exclui; Shift + clique na linha acrescenta um vértice).
godot-editor3d-um_corredor_precisa_de_ao = Um corredor precisa de ao menos 2 pontos.
godot-editor3d-ponto_removido = Ponto removido ({ $aisle_points }).
godot-editor3d-trecho_selecionado_del_remove = Trecho { $a } → { $b } selecionado (Del remove).
godot-editor3d-aponte_para_o_chao_para-3 = Aponte para o chão para colocar um nó.
godot-editor3d-zona_ponto_s_clique_no =
    Zona: { $zone_points } { $zone_points ->
        [one] ponto
       *[other] pontos
    }. Clique no primeiro ponto, Enter ou duplo clique fecha; Backspace volta; Esc cancela.
godot-editor3d-zona_arraste_para_mover_arraste = Zona { $name }: arraste para mover, arraste um vértice para mudar a forma; Del exclui.
godot-editor3d-zona_clique_para_por_os = Zona: clique para pôr os outros pontos; Enter, duplo clique ou o primeiro ponto fecha.
godot-editor3d-zona_alterada_ctrl_z_desfaz = Zona alterada (Ctrl+Z desfaz).
godot-editor3d-uma_zona_precisa_de_pelo = Uma zona precisa de pelo menos 3 pontos.

## indicadores.rs

godot-indicadores-itens_refugados = Itens refugados
godot-indicadores-itens_que_terminaram_neste_refugo = Itens que terminaram neste refugo, por replicação.
godot-indicadores-defeitos_gerados = Defeitos gerados
godot-indicadores-defeitos_que_este_posto_gerou = Defeitos que este posto gerou, por replicação.
godot-indicadores-defeitos_que_escaparam = Defeitos que escaparam
godot-indicadores-defeitos_deste_posto_que_chegaram = Defeitos deste posto que chegaram ao cliente (falha externa), por replicação.
godot-indicadores-rendimento_de_primeira = Rendimento de primeira
godot-indicadores-itens_inspecionados_uma_so_vez = Itens inspecionados uma só vez e entregues, sobre os inspecionados.
godot-indicadores-custo_da_qualidade = Custo da qualidade
godot-indicadores-avaliacao_falha_interna_e_falha = Avaliação, falha interna e falha externa atribuídas a este posto, por replicação.
godot-indicadores-tempo_em_zonas = Tempo em zonas
godot-indicadores-tempo_desta_pessoa_dentro_das = Tempo desta pessoa dentro das zonas de risco, por replicação.
godot-indicadores-exposicao_nas_zonas = Exposição nas zonas
godot-indicadores-tempo_de_pessoas_dentro_das = Tempo de pessoas dentro das zonas onde este posto está, somado, por replicação.
godot-indicadores-zonas = Zonas
godot-indicadores-zonas_de_risco_onde_o = Zonas de risco onde o posto está.
godot-indicadores-entradas_sem_epi = Entradas sem EPI
godot-indicadores-entradas_na_zona_sem_o = Entradas na zona sem o equipamento de proteção que ela pede, por replicação.
godot-indicadores-quase_conflitos = Quase-conflitos
godot-indicadores-pessoa_e_veiculo_mais_perto = Pessoa e veículo mais perto que o limiar, envolvendo este recurso, por replicação. Simulado: serve para comparar layouts, não estima acidentes reais.
godot-indicadores-pessoa_e_veiculo_mais_perto-2 = Pessoa e veículo mais perto que o limiar a até 3 m deste posto, por replicação. Simulado: serve para comparar layouts, não estima acidentes reais.
godot-indicadores-quebras = Quebras
godot-indicadores-falhas_do_posto_por_replicacao = Falhas do posto, por replicação.
godot-indicadores-tempo_medio_entre_falhas_tempo = Tempo médio entre falhas (tempo em pé sobre o número de falhas).
godot-indicadores-tempo_medio_de_reparo_da = Tempo médio de reparo, da quebra até voltar a produzir.
godot-indicadores-tempo_parado = Tempo parado
godot-indicadores-tempo_em_manutencao_corretiva_por = Tempo em manutenção corretiva, por replicação.
godot-indicadores-disponibilidade = Disponibilidade
godot-indicadores-tempo_produzindo_sobre_o_tempo = Tempo produzindo sobre o tempo planejado (sem turno e preventivas).
godot-indicadores-custo_de_manutencao = Custo de manutenção
godot-indicadores-tempo_parado_reparos_e_preventivas = Tempo parado, reparos e preventivas pelos custos do bloco, por replicação.
godot-indicadores-parcela_do_tempo_em_quebra = Parcela do tempo em quebra, por replicação.
godot-indicadores-tempo_fora_de_quebra_sobre = Tempo fora de quebra sobre o tempo planejado (sem turno).

## jobs.rs

godot-jobs-rodando_replicacoes = Rodando replicações
godot-jobs-rodando_mais_replicacoes = Rodando mais replicações
godot-jobs-comparando_cenarios = Comparando cenários
godot-jobs-otimizando = Otimizando
godot-jobs-analisando_o_aquecimento = Analisando o aquecimento
godot-jobs-conferindo_a_lei_de_little = Conferindo a lei de Little
godot-jobs-procurando_o_gargalo_movel = Procurando o gargalo móvel
godot-jobs-calculando_o_pert_cpm = Calculando o PERT/CPM
godot-jobs-ajustando_a_distribuicao_aos_dados = Ajustando a distribuição aos dados
godot-jobs-gravando_o_relatorio_dos_dados = Gravando o relatório dos dados
godot-jobs-carregando_a_reproducao = Carregando a reprodução
godot-jobs-erro_interno_durante_a_execucao = erro interno durante a execução
godot-jobs-a_execucao_terminou_sem_resultado = a execução terminou sem resultado
godot-jobs-objeto_nao_existe_no_modelo = objeto "{ $object }" não existe no modelo

## linha.rs


## path_edit.rs

godot-path-edit-nao_existe = '{ $name }' não existe
godot-path-edit-e_uma_esteira_reta_sem = '{ $name }' é uma esteira reta, sem nós
godot-path-edit-nao_e_uma_esteira = '{ $name }' não é uma esteira
godot-path-edit-a_esteira_precisa_de_ao = a esteira precisa de ao menos 2 nós diferentes
godot-path-edit-a_biblioteca_nao_tem_esteira = a biblioteca não tem esteira
godot-path-edit-o_caminho_precisa_de_ao = o caminho precisa de ao menos 2 pontos diferentes
godot-path-edit-nao_existe_o_caminho = não existe o caminho '{ $name }'
godot-path-edit-nao_tem_o_no = '{ $name }' não tem o nó { $index }
godot-path-edit-o_raio_deve_ser_um = o raio deve ser um número >= 0 (metros)
godot-path-edit-a_distancia_do_paralelo_deve = a distância do paralelo deve ser diferente de zero
godot-path-edit-o_caminho_novo_sumiu = o caminho novo sumiu
godot-path-edit-o_caminho_precisa_de_ao-2 = o caminho precisa de ao menos 2 pontos
godot-path-edit-a_esteira_precisa_de_ao-2 = a esteira precisa de ao menos 2 nós
godot-path-edit-nao_e_uma_esteira_desenhada = '{ $name }' não é uma esteira desenhada
godot-path-edit-as_duas_pontas_ja_estao = as duas pontas já estão no piso
godot-path-edit-tipo_de_esteira_desconhecido = tipo de esteira desconhecido: { $style }
godot-path-edit-acionamento_desconhecido = acionamento desconhecido: { $drive }
godot-path-edit-a_largura_deve_ser_0 = a largura deve ser > 0 (metros)
godot-path-edit-a_inclinacao_maxima_fica_entre = a inclinação máxima fica entre 0 e 90 graus
godot-path-edit-divida_num_no_do_meio = divida num nó do meio da esteira
godot-path-edit-nao_ha_trecho_depois_do = não há trecho depois do último nó
godot-path-edit-o_caminho_so_tem_esse = o caminho só tem esse trecho: apague o caminho inteiro
godot-path-edit-a_esteira_so_tem_esse = a esteira só tem esse trecho: apague a esteira inteira
godot-path-edit-nenhum_trajeto_comeca_onde_este = nenhum trajeto começa onde este acaba
godot-path-edit-nao_existe_o_caminho-2 = não existe o caminho '{ $other }'

## tool_panels.rs

godot-tool-panels-ligacao = Ligação
godot-tool-panels-remover_ligacao_del = Remover ligação (Del)
godot-tool-panels-rede_de_caminhos = Rede de caminhos
godot-tool-panels-no_s_trecho_s_pista =
    { $nodes } { $nodes ->
        [one] nó
       *[other] nós
    }, { $edges } { $edges ->
        [one] trecho
       *[other] trechos
    }, { $controlled } { $controlled ->
        [one] pista controlada
       *[other] pistas controladas
    }.
godot-tool-panels-clique_no_chao_para_por = Clique no chão para pôr um nó; com um nó armado (verde), o próximo clique o liga ao novo por um trecho. Clique num nó para armá-lo, arraste para movê-lo. Clique num trecho para mudar o sentido e a capacidade (com capacidade, a pista é controlada e fica em vermelho). Esc encerra o caminho; Del exclui o selecionado.
godot-tool-panels-novos_trechos_em_mao_dupla = Novos trechos em mão dupla
godot-tool-panels-no = Nó { $name }
godot-tool-panels-nome = Nome
godot-tool-panels-posicao_m = Posição ({ $position }, { $position2 }) m
godot-tool-panels-excluir_no_del = Excluir nó (Del)
godot-tool-panels-trecho = Trecho { $from } → { $to }
godot-tool-panels-comprimento_m = Comprimento { $length } m
godot-tool-panels-mao_dupla = Mão dupla
godot-tool-panels-capacidade_veiculos_0_sem_controle = Capacidade (veículos; 0 = sem controle)
godot-tool-panels-pista_controlada_no_maximo_veiculo =
    Pista controlada: no máximo { $cap } { $cap ->
        [one] veículo
       *[other] veículos
    } ao mesmo tempo.
godot-tool-panels-inverter_sentido = Inverter sentido
godot-tool-panels-excluir_trecho_del = Excluir trecho (Del)
godot-tool-panels-pedestres = Pedestres
godot-tool-panels-empilhadeiras = Empilhadeiras
godot-tool-panels-misto = Misto
godot-tool-panels-corredores_de_circulacao = Corredores de circulação
godot-tool-panels-corredor_es_de_pedestres_de =
    { $list } { $list ->
        [one] corredor
       *[other] corredores
    }: { $pedestrian } de pedestres, { $forklift } de empilhadeiras, { $forklift2 } mistos; { $crossings } { $crossings ->
        [one] cruzamento
       *[other] cruzamentos
    }.
godot-tool-panels-clique_no_chao_para_por-2 = Clique no chão para pôr os pontos do eixo; Enter ou duplo clique termina, Backspace volta um ponto, Esc cancela. Os pontos encaixam nos outros corredores. Clique num corredor para editá-lo, arraste um vértice para movê-lo, Shift + clique na linha acrescenta um vértice. As faixas amarelas, a zebra dos cruzamentos e as setas aparecem no piso.
godot-tool-panels-novos_corredores = Novos corredores
godot-tool-panels-tipo = Tipo
godot-tool-panels-largura_m = Largura (m)
godot-tool-panels-corredor = Corredor { $name }
godot-tool-panels-ponto_s_m_de_comprimento =
    { $points } { $points ->
        [one] ponto
       *[other] pontos
    }, { $length } m de comprimento
godot-tool-panels-passagem_secundaria_de_pedestres_minimo = Passagem secundária de pedestres (mínimo menor)
godot-tool-panels-perfil = Perfil
godot-tool-panels-padrao_do_modelo = Padrão do modelo
godot-tool-panels-brasil_amarelo = Brasil (amarelo)
godot-tool-panels-brasil = Brasil
godot-tool-panels-branco = Branco
godot-tool-panels-vertice_selecionado = Vértice { $v } selecionado
godot-tool-panels-remover_vertice = Remover vértice
godot-tool-panels-excluir_corredor_del = Excluir corredor (Del)
godot-tool-panels-zonas_de_risco = Zonas de risco
godot-tool-panels-zona_s_de_risco_no =
    { $zones } { $zones ->
        [one] zona
       *[other] zonas
    } de risco no piso.
godot-tool-panels-clique_no_chao_para_por-3 = Clique no chão para pôr os cantos; o primeiro ponto, Enter ou duplo clique fecha a zona, Backspace volta um ponto, Esc cancela. Clique numa zona para editá-la: arraste o meio para mover, arraste um vértice para mudar a forma; Del exclui. Os pontos encaixam na grade.
godot-tool-panels-zona = Zona { $name }
godot-tool-panels-canto_s =
    { $polygon } { $polygon ->
        [one] canto
       *[other] cantos
    }
godot-tool-panels-nivel_de_risco_1_a = Nível de risco (1 a 5; 0 = sem nível)
godot-tool-panels-epi_exigido_para_entrar = EPI exigido para entrar
godot-tool-panels-excluir_zona_del = Excluir zona (Del)

## ui_arrivals.rs

godot-ui-arrivals-nao_e_um_numero = "{ $t }" não é um número
godot-ui-arrivals-use_um_numero = use um número
godot-ui-arrivals-hh_mm_so_vale_com = hh:mm só vale com ciclo de dia ou semana (1440 min, 24 h, 86400 s); use um número
godot-ui-arrivals-hora_invalida_em = hora inválida em "{ $t }"
godot-ui-arrivals-minuto_invalido_em = minuto inválido em "{ $t }"
godot-ui-arrivals-minuto_invalido_em_0_a = minuto inválido em "{ $t }" (0 a 59)
godot-ui-arrivals-nao_e_um_horario_hh = "{ $t }" não é um horário (hh:mm) nem um número
godot-ui-arrivals-quantidade_um_inteiro_0 = quantidade "{ $t }": um inteiro >= 0
godot-ui-arrivals-rotulo_use_nome_valor = rótulo "{ $part }": use nome=valor
godot-ui-arrivals-rotulo_sem_nome = rótulo "{ $part }" sem nome
godot-ui-arrivals-rotulo = rótulo { $name }: { $e }
godot-ui-arrivals-uma_vez = Uma vez
godot-ui-arrivals-a_cada_ciclo = A cada ciclo
godot-ui-arrivals-volta_ao_inicio = Volta ao início
godot-ui-arrivals-por_ciclo_de = por ciclo de { $d }
godot-ui-arrivals-por_volta = por volta
godot-ui-arrivals-no_total = no total
godot-ui-arrivals-em_linha_s =
    { $items } { $items ->
        [one] item
       *[other] itens
    } { $per } em { $rows } { $rows ->
        [one] linha
       *[other] linhas
    }
godot-ui-arrivals-tipo_da_fonte = tipo da fonte
godot-ui-arrivals-um_dia = { $d } (um dia)
godot-ui-arrivals-uma_semana = { $d } (uma semana)
godot-ui-arrivals-tipo_de_item = tipo de item
godot-ui-arrivals-nada_para_colar_copie_as = nada para colar: copie as linhas da planilha
godot-ui-arrivals-linha = linha { $n }: { $e }
godot-ui-arrivals-falta_o_instante_modo_horario = falta o instante (modo horário)
godot-ui-arrivals-so_o_cabecalho_nenhuma_linha = só o cabeçalho: nenhuma linha para colar
godot-ui-arrivals-editar_como_texto_ron = Editar como texto (RON)
godot-ui-arrivals-modo = Modo
godot-ui-arrivals-horario = Horário
godot-ui-arrivals-sequencia = Sequência
godot-ui-arrivals-horario_cada_linha_no_seu = Horário: cada linha no seu instante. Sequência: uma linha por chegada, com o tempo entre chegadas da fonte entre elas.
godot-ui-arrivals-repetir = Repetir
godot-ui-arrivals-ciclo = Ciclo
godot-ui-arrivals-da_fonte = (da fonte)
godot-ui-arrivals-o_tipo_da_fonte = O tipo da fonte
godot-ui-arrivals-tipo_de_item_da_linha = Tipo de item da linha
godot-ui-arrivals-subir_a_linha = Subir a linha
godot-ui-arrivals-descer_a_linha = Descer a linha
godot-ui-arrivals-remover_a_linha = Remover a linha
godot-ui-arrivals-linha-2 = + Linha
godot-ui-arrivals-acrescenta_uma_linha_no_fim = Acrescenta uma linha no fim
godot-ui-arrivals-ordenar_por_instante = Ordenar por instante
godot-ui-arrivals-poe_as_linhas_em_ordem = Põe as linhas em ordem de instante
godot-ui-arrivals-as_linhas_ja_estao_em = As linhas já estão em ordem (ou é uma sequência)
godot-ui-arrivals-colar_da_planilha = Colar da planilha
godot-ui-arrivals-linhas_copiadas_de_uma_planilha = Linhas copiadas de uma planilha (separadas por tabulação ou ;)
godot-ui-arrivals-hora_qtd_tipo_rotulo_08 =
    hora; qtd; tipo; rótulo...
    08:00; 12; caixa; prioridade=2
godot-ui-arrivals-qtd_tipo_rotulo_2_caixa =
    qtd; tipo; rótulo...
    2; caixa; prioridade=2
godot-ui-arrivals-substituir_as_linhas = Substituir as linhas
godot-ui-arrivals-acrescentar = Acrescentar
godot-ui-arrivals-ver_as_primeiras_chegadas = Ver as primeiras chegadas
godot-ui-arrivals-histograma_das_chegadas_de_um = Histograma das chegadas de um ciclo (ou uma volta), por hora e por tipo
godot-ui-arrivals-simulado_so_a_fonte_ligada = Simulado: só a fonte, ligada a uma saída, por um ciclo.
godot-ui-arrivals-estimado_pela_tabela_a_fonte = Estimado pela tabela (a fonte sozinha não rodou: { $e }).
godot-ui-arrivals-estimado_pela_tabela_rodando_a = Estimado pela tabela; rodando a fonte...
godot-ui-arrivals-maximo_por = máximo { $num_text } por { $bin_label }
godot-ui-arrivals-por_instantes_da_tabela_sem = por { $bin_label } ({ $bin }); instantes da tabela, sem rodar o modelo
godot-ui-arrivals-horarios_em_hh_mm_taxas = { $name } (horários em hh:mm, taxas por hora)
godot-ui-arrivals-unidade_do_modelo = unidade do modelo
godot-ui-arrivals-fora_dos_periodos_a_fonte = Fora dos períodos a fonte não cria itens
godot-ui-arrivals-periodo_s =
    { $periods } { $periods ->
        [one] período
       *[other] períodos
    }
godot-ui-arrivals-itens_h_ou_distribuicao = itens/h ou distribuição
godot-ui-arrivals-taxa_ou_distribuicao = taxa ou distribuição
godot-ui-arrivals-padrao_o_tempo_entre_chegadas = Padrão: o tempo entre chegadas da fonte. Distribuição: um tempo entre chegadas próprio. Taxa: chegadas de Poisson. Fechado: sem chegadas.
godot-ui-arrivals-distribuicao = distribuição: { $e }
godot-ui-arrivals-media = média { $mean }
godot-ui-arrivals-sem_chegadas = sem chegadas
godot-ui-arrivals-remover_o_periodo = Remover o período
godot-ui-arrivals-periodo = + Período
godot-ui-arrivals-acrescenta_um_periodo_depois_do = Acrescenta um período depois do último
godot-ui-arrivals-o_ultimo_periodo_ja_vai = O último período já vai até o fim do ciclo
godot-ui-arrivals-esperado_por_ciclo_chegadas = Esperado por ciclo: { $total } chegadas
godot-ui-arrivals-padrao_media = padrão (média { $mean })
godot-ui-arrivals-distribuicao-2 = distribuição { $d }
godot-ui-arrivals-taxa_itens_h = taxa { $cycle } itens/h
godot-ui-arrivals-taxa = taxa { $num_text }
godot-ui-arrivals-periodo_a_esperadas = Período { $k }: { $cycle } a { $cycle2 }, { $how }; esperadas { $default }
godot-ui-arrivals-chegadas_esperadas_em_cada_periodo = Chegadas esperadas em cada período (taxa × duração, ou duração ÷ média)
godot-ui-arrivals-barra_s =
    { $exp } { $exp ->
        [one] barra
       *[other] barras
    }
godot-ui-arrivals-nao_existe = '{ $source }' não existe
godot-ui-arrivals-nao_e_uma_chegada = '{ $source }' não é uma chegada

## ui_bar.rs

godot-ui-bar-equipe_despachante_membro = Equipe (despachante → membro)
godot-ui-bar-transporte_pedido = Transporte pedido
godot-ui-bar-operadores_pedidos = Operadores pedidos
godot-ui-bar-equipe_de_manutencao = Equipe de manutenção
godot-ui-bar-local_de_pausa = Local de pausa
godot-ui-bar-mais = Mais
godot-ui-bar-os_grupos_que_nao_cabem = Os grupos que não cabem na barra
godot-ui-bar-arquivo = Arquivo
godot-ui-bar-editar = Editar
godot-ui-bar-organizar = Organizar
godot-ui-bar-grade = Grade
godot-ui-bar-ajuda = Ajuda
godot-ui-bar-analise = Análise
godot-ui-bar-novo = Novo
godot-ui-bar-novo_modelo_ctrl_n = Novo modelo (Ctrl+N)
godot-ui-bar-abrir = Abrir…
godot-ui-bar-abrir_um_modelo_ou_um = Abrir um modelo ou um replay (Ctrl+O)
godot-ui-bar-salvar = Salvar
godot-ui-bar-salvar_ctrl_s = Salvar (Ctrl+S)
godot-ui-bar-salvar_como = Salvar como…
godot-ui-bar-salvar_com_outro_nome_ctrl = Salvar com outro nome (Ctrl+Shift+S)
godot-ui-bar-desfazer = Desfazer
godot-ui-bar-desfazer_ctrl_z = Desfazer (Ctrl+Z)
godot-ui-bar-refazer = Refazer
godot-ui-bar-refazer_ctrl_y = Refazer (Ctrl+Y)
godot-ui-bar-alinhar_a_esquerda = Alinhar à esquerda
godot-ui-bar-alinhar_ao_centro_x = Alinhar ao centro (X)
godot-ui-bar-alinhar_a_direita = Alinhar à direita
godot-ui-bar-alinhar_ao_topo = Alinhar ao topo
godot-ui-bar-alinhar_ao_centro_y = Alinhar ao centro (Y)
godot-ui-bar-alinhar_a_base = Alinhar à base
godot-ui-bar-distribuir_em_x = Distribuir em X
godot-ui-bar-distribuir_em_y = Distribuir em Y
godot-ui-bar-copiar_ctrl_c = Copiar (Ctrl+C)
godot-ui-bar-recortar_ctrl_x = Recortar (Ctrl+X)
godot-ui-bar-colar_ctrl_v = Colar (Ctrl+V)
godot-ui-bar-duplicar_ctrl_d = Duplicar (Ctrl+D)
godot-ui-bar-excluir_del = Excluir (Del)
godot-ui-bar-selecionar_tudo_ctrl_a = Selecionar tudo (Ctrl+A)
godot-ui-bar-alinhar_distribuir_copiar_colar_e = Alinhar, distribuir, copiar, colar e excluir a seleção
godot-ui-bar-encaixar_na_grade_ao_por = Encaixar na grade ao pôr e arrastar (G; passo ao lado)
godot-ui-bar-passo_da_grade = Passo da grade
godot-ui-bar-atalhos_f1_a_lista_dos = Atalhos (F1): a lista dos atalhos de teclado e mouse
godot-ui-bar-resultados = Resultados
godot-ui-bar-tabela_graficos_analises_cenarios_e = Tabela, gráficos, análises, cenários e otimização da última rodada
godot-ui-bar-pert_cpm = PERT/CPM
godot-ui-bar-pert_cpm_da_selecao_com = PERT/CPM da seleção (com dois objetos, de um até o outro)
godot-ui-bar-pert_cpm_da_selecao = PERT/CPM da seleção
godot-ui-bar-pert_cpm_de_ate_dois = PERT/CPM de… até… (dois selecionados)
godot-ui-bar-fluxos_de_tokens = Fluxos de tokens
godot-ui-bar-modo_missao_utsim_ensino = Modo missão (UTSim Ensino)
godot-ui-bar-abre_as_missoes_do_utsim = Abre as missões do UTSim Ensino neste mesmo programa
godot-ui-bar-outras_analises_pert_cpm_de = Outras análises: PERT/CPM de… até…, fluxos de tokens; as ligações de equipe e de pedido na vista 3D
godot-ui-bar-simular = Simular
godot-ui-bar-montar_o_modelo_tab_alterna = Montar o modelo (Tab alterna)
godot-ui-bar-rodar_as_replicacoes_e_reproduzir = Rodar as replicações e reproduzir (Tab alterna)
godot-ui-bar-selecionar = Selecionar
godot-ui-bar-selecionar_e_mover_esc = Selecionar e mover (Esc)
godot-ui-bar-ligar = Ligar
godot-ui-bar-ligar_arraste_da_origem_ao = Ligar: arraste da origem ao destino, ou clique na origem e depois no destino (L)
godot-ui-bar-rede = Rede
godot-ui-bar-rede_de_caminhos_nos_e = Rede de caminhos: nós e trechos (N)
godot-ui-bar-corredor = Corredor
godot-ui-bar-corredores_de_circulacao_com_faixas = Corredores de circulação com faixas de segurança: clique pontos no chão, Enter termina (C)
godot-ui-bar-zona = Zona
godot-ui-bar-zonas_de_risco_no_piso = Zonas de risco no piso: clique os cantos, o primeiro ponto ou Enter fecha; clique numa zona para editar (Z)
godot-ui-bar-equipe = Equipe
godot-ui-bar-equipe_arraste_do_despachante_a = Equipe: arraste do despachante a um transportador (entra ou sai da equipe), ou de um objeto ao transportador ou despachante que leva os itens; com Shift, de um processo a quem fornece os operadores (E; Alt com Ligar)
godot-ui-bar-ligacoes_na_vista_3d = Ligações na vista 3D
godot-ui-bar-tambem_durante_a_simulacao = Também durante a simulação
godot-ui-bar-as_linhas_tracejadas_entre_operadores = As linhas tracejadas entre operadores, veículos e máquinas no modo Simular

## ui_fitwin.rs

godot-ui-fitwin-linha_nao_e_um_numero = linha { $k }: "{ $t }" não é um número
godot-ui-fitwin-os_instantes_precisam_ser_crescentes = os instantes precisam ser crescentes
godot-ui-fitwin-digite_ou_cole_pelo_menos = digite ou cole pelo menos dois valores
godot-ui-fitwin-compare_as_distribuicoes_antes = compare as distribuições antes
godot-ui-fitwin-utsim_3d_valores_reais = UTSim 3D, Valores reais
godot-ui-fitwin-desvio_padrao = desvio padrão
godot-ui-fitwin-valores_reais = Valores reais: { $field }
godot-ui-fitwin-o_que_e = O que é: { $title }
godot-ui-fitwin-valores_observados = Valores observados
godot-ui-fitwin-digite_cole_da_planilha_varias = Digite, cole da planilha (várias linhas, vírgula decimal) ou importe um CSV.
godot-ui-fitwin-cole_aqui_e_clique_adicionar = cole aqui e clique Adicionar
godot-ui-fitwin-adicionar = Adicionar
godot-ui-fitwin-valor_es_adicionados =
    { $n } { $n ->
        [one] valor adicionado
       *[other] valores adicionados
    }.
godot-ui-fitwin-importar_csv = Importar CSV…
godot-ui-fitwin-le_os_valores_de_um = Lê os valores de um arquivo CSV ou TXT
godot-ui-fitwin-limpar = Limpar
godot-ui-fitwin-apaga_todos_os_valores_da = Apaga todos os valores da tabela
godot-ui-fitwin-sao_instantes_crescentes_usar_os = São instantes crescentes (usar os intervalos)
godot-ui-fitwin-marque_quando_os_valores_sao = Marque quando os valores são horários de chegada: o ajuste usa as diferenças entre eles.
godot-ui-fitwin-valor_es =
    { $n } { $n ->
        [one] valor
       *[other] valores
    }
godot-ui-fitwin-apaga_este_valor = Apaga este valor
godot-ui-fitwin-intervalos = Intervalos
godot-ui-fitwin-estatisticas = Estatísticas
godot-ui-fitwin-comparacao_das_distribuicoes = Comparação das distribuições
godot-ui-fitwin-comparar_de_novo = Comparar de novo
godot-ui-fitwin-comparar_todas = Comparar todas
godot-ui-fitwin-ajusta_todas_as_distribuicoes_candidatas = Ajusta todas as distribuições candidatas e calcula todas as medidas de uma vez
godot-ui-fitwin-ajustando_todas_as_candidatas = ajustando todas as candidatas...
godot-ui-fitwin-depois_de_comparar_a_tabela = Depois de comparar, a tabela mostra todas as medidas de cada distribuição; clique numa linha para ver os gráficos e escolher.
godot-ui-fitwin-os_valores_mudaram_compare_de = Os valores mudaram: compare de novo.
godot-ui-fitwin-ordenada_por_sugestao_inicial_aic = Ordenada por { $label }. Sugestão inicial (AIC com a regra do UTSim): { $family }.
godot-ui-fitwin-usar_esta_distribuicao = Usar esta distribuição ({ $fam })
godot-ui-fitwin-passa_a_usar_esta_distribuicao = Passa a usar esta distribuição neste campo, ajustada a estes valores (Ctrl+Z desfaz)
godot-ui-fitwin-usar_a_empirica = Usar a empírica
godot-ui-fitwin-usa_os_proprios_valores_observados = Usa os próprios valores observados, sem família
godot-ui-fitwin-gerar_relatorio = Gerar relatório
godot-ui-fitwin-grava_o_relatorio_html_com = Grava o relatório HTML com os dados, a comparação completa, os gráficos e a escolha
godot-ui-fitwin-fechar = Fechar
godot-ui-fitwin-fecha_a_explicacao = Fecha a explicação
godot-ui-fitwin-como_ler_quando_usar_cuidados =
    { $what }
    Como ler: { $read }
    Quando usar: { $when }
    Cuidados: { $care }
godot-ui-fitwin-aceita_pelos_testes_clique_para = Aceita pelos testes. Clique para ver os gráficos e escolher.
godot-ui-fitwin-nao_se_aplica_a_estes = Não se aplica a estes dados.
godot-ui-fitwin-rejeitada_por_algum_teste_valor = Rejeitada por algum teste (valor-p abaixo de 0,05). Ainda pode ser escolhida.
godot-ui-fitwin-ordena_por = Ordena por { $label } ({ $melhor })
godot-ui-fitwin-escolhida_por = escolhida por
godot-ui-fitwin-histograma_e_curvas_em_destaque = Histograma e curvas ({ $selected_family } em destaque)
godot-ui-fitwin-poisson = Poisson
godot-ui-fitwin-geometrica = Geométrica
godot-ui-fitwin-binomial = Binomial
godot-ui-fitwin-binomial_negativa = Binomial negativa
godot-ui-fitwin-a_empirica_reproduz_os_proprios = A empírica reproduz os próprios dados: não há curva para comparar.

## ui_left.rs

godot-ui-left-orbita = Órbita
godot-ui-left-planta_2d = Planta (2D)
godot-ui-left-seguir = Seguir
godot-ui-left-primeira_pessoa = Primeira pessoa
godot-ui-left-do_recurso = Do recurso
godot-ui-left-recentes = Recentes
godot-ui-left-usados_por_ultimo = Usados por último
godot-ui-left-espera = Espera
godot-ui-left-fluxo = Fluxo
godot-ui-left-fluxo_de_itens_e_montagem = Fluxo de itens e montagem
godot-ui-left-mover = Mover
godot-ui-left-movimentacao_recursos_e_logistica = Movimentação, recursos e logística
godot-ui-left-fabrica = Fábrica
godot-ui-left-manufatura = Manufatura
godot-ui-left-servicos = Serviços
godot-ui-left-servicos_e_saude = Serviços e saúde
godot-ui-left-escritorio = Escritório
godot-ui-left-escritorio_e_processos = Escritório e processos
godot-ui-left-q_e_s = Q e S
godot-ui-left-qualidade_e_seguranca = Qualidade e segurança
godot-ui-left-biblioteca = Biblioteca
godot-ui-left-objetos = Objetos
godot-ui-left-mostrar_obstaculos_e_folgas = Mostrar obstáculos e folgas
godot-ui-left-mostra_a_area_ocupada_dos = Mostra a área ocupada dos objetos, as paredes, a folga dos recursos e o ponto de acesso de cada objeto (arraste-o com a ferramenta Selecionar)
godot-ui-left-nada_com_esse_nome = Nada com esse nome.
godot-ui-left-clique_de_novo_para_recolher = { $title }. Clique de novo para recolher.
godot-ui-left-os_elementos_que_voce_usar = Os elementos que você usar aparecem aqui.
godot-ui-left-buscar_elemento = Buscar elemento
godot-ui-left-limpa_a_busca = Limpa a busca
godot-ui-left-clique_e_depois_clique_no = { $label }: clique e depois clique no chão, ou arraste até o chão
godot-ui-left-buscar_por_nome_ou_tipo = Buscar por nome ou tipo
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
godot-ui-left-simular_de_novo = Simular de novo
godot-ui-left-roda_as_replicacoes_outra_vez = Roda as replicações outra vez
godot-ui-left-blocos_clique_para_focar = Blocos (clique para focar)
godot-ui-left-fluxos = Fluxos
godot-ui-left-fluxos_de_tokens_em_2d = Fluxos de tokens em 2D, com as contagens no tempo
godot-ui-left-gravar_replay = Gravar replay
godot-ui-left-grava_o_replay_desta_replicacao = Grava o replay desta replicação em .json (abre de novo com Abrir)
godot-ui-left-rotulos = Rótulos
godot-ui-left-os_nomes_dos_blocos_sobre = Os nomes dos blocos sobre a cena
godot-ui-left-sombras = Sombras
godot-ui-left-as_sombras_do_sol_desligadas = As sombras do sol (desligadas, a cena fica mais leve)
godot-ui-left-seguranca = Segurança
godot-ui-left-zonas_de_risco_epi_das = Zonas de risco, EPI das pessoas, quase-conflitos simulados, itens com defeito e inspeções
godot-ui-left-calor = Calor
godot-ui-left-mapa_de_calor_da_presenca = Mapa de calor da presença das pessoas e dos quase-conflitos no piso
godot-ui-left-quase_conflitos_simulados = Quase-conflitos simulados ({ $quase })
godot-ui-left-replicacoes = Replicações
godot-ui-left-uma_replicacao_a_menos = Uma replicação a menos
godot-ui-left-uma_replicacao_a_mais = Uma replicação a mais
godot-ui-left-camera = Câmera
godot-ui-left-seguir_primeira_pessoa_e_do = Seguir, Primeira pessoa e Do recurso acompanham o bloco escolhido na lista
godot-ui-left-os_resultados_aparecem_aqui_depois = Os resultados aparecem aqui depois da simulação.
godot-ui-left-gargalo = Gargalo: { $bottleneck }
godot-ui-left-a_tabela_completa_os_graficos = A tabela completa, os gráficos e as análises estão em Resultados.
godot-ui-left-gargalo-2 =
    {""}
    Gargalo: { $bottleneck }

## ui_pert.rs

godot-ui-pert-rede = Rede
godot-ui-pert-gantt = Gantt
godot-ui-pert-distribuicao = Distribuição
godot-ui-pert-tabela = Tabela
godot-ui-pert-texto = Texto
godot-ui-pert-simulado = Simulado
godot-ui-pert-caminho_critico_observado_na_simulacao = Caminho crítico observado na simulação: filas, transportes e recursos incluídos
godot-ui-pert-classico = Clássico
godot-ui-pert-pert_classico_estimativas_a_m = PERT clássico: estimativas (a, m, b), CPM nas médias e P(prazo) pela normal
godot-ui-pert-monte_carlo = Monte Carlo
godot-ui-pert-sorteios_das_duracoes_sobre_a = Sorteios das durações sobre a rede: criticidade e distribuição do tempo total
godot-ui-pert-desvio = , desvio { $sd }
godot-ui-pert-unidades_sorteios = {" "}({ $units } unidades/sorteios)
godot-ui-pert-tempo_total_cpm_nas_medias = Tempo total: { $t }{ $spread }{ $units }. CPM nas médias: { $project }.
godot-ui-pert-ic_a = {" "}(IC { $lo }% a { $hi }%)
godot-ui-pert-p_t = P(T ≤ { $deadline }) = { $p }
godot-ui-pert-caminho_critico = Caminho crítico: { $names } ({ $index }%)
godot-ui-pert-aviso = aviso: { $w }
godot-ui-pert-aproximacao_normal_do_pert_classico = aproximação normal do PERT clássico
godot-ui-pert-unidades_observadas = unidades observadas
godot-ui-pert-a_linha_amarela_e_o = ; a linha amarela é o prazo
godot-ui-pert-distribuicao_do_tempo_total = Distribuição do tempo total ({ $source }){ $marker }.
godot-ui-pert-pert_cpm_tempo_total = PERT/CPM ({ $mode }): tempo total { $total }.
godot-ui-pert-atividade = Atividade
godot-ui-pert-duracao = Duração
godot-ui-pert-folga_total = Folga total
godot-ui-pert-folga_livre = Folga livre
godot-ui-pert-criticidade = Criticidade
godot-ui-pert-prazo = prazo { $num }
godot-ui-pert-sem_distribuicao_para_este_modo = sem distribuição para este modo
godot-ui-pert-hipotese = hipótese: { $h }
godot-ui-pert-fechar = Fechar
godot-ui-pert-fechar_a_janela_pert_cpm = Fechar a janela PERT/CPM
godot-ui-pert-prazo-2 = Prazo
godot-ui-pert-sem_prazo = sem prazo
godot-ui-pert-sorteios_monte_carlo = Sorteios (Monte Carlo)
godot-ui-pert-recalcular = Recalcular
godot-ui-pert-calcula_de_novo_no_modo = Calcula de novo no modo atual
godot-ui-pert-relatorio_html = relatório .html
godot-ui-pert-exportar_html = Exportar HTML
godot-ui-pert-grava_o_relatorio_html_do = Grava o relatório HTML do PERT/CPM
godot-ui-pert-csv = CSV
godot-ui-pert-grava_as_tabelas_em_csv = Grava as tabelas em CSV na pasta do arquivo

## zone_edit.rs

godot-zone-edit-trafego_de_veiculos = Tráfego de veículos
godot-zone-edit-alcance_de_maquina = Alcance de máquina
godot-zone-edit-carga_suspensa = Carga suspensa
godot-zone-edit-ruido = Ruído
godot-zone-edit-calor = Calor
godot-zone-edit-outro = Outro
godot-zone-edit-a_zona_nao_existe = a zona '{ $name }' não existe
godot-zone-edit-uma_zona_precisa_de_pelo = uma zona precisa de pelo menos 3 pontos
godot-zone-edit-a_zona_precisa_de_um = a zona precisa de um nome
godot-zone-edit-tipo_de_zona_desconhecido = tipo de zona desconhecido: { $value }
godot-zone-edit-nivel_um_numero_de_1 = nível: um número de 1 a 5
godot-zone-edit-campo_de_zona_desconhecido = campo de zona desconhecido: { $field }

godot-dialogo-descartar-titulo = Descartar mudanças?
godot-dialogo-descartar-texto = O modelo tem mudanças que não foram salvas. Descartar?
godot-dialogo-descartar = Descartar
godot-dialogo-voltar = Voltar
godot-dialogo-atalhos = Atalhos
godot-dialogo-fechar = Fechar
godot-dialogo-desempenho = Mostra ou oculta as medidas de desempenho sobre a vista 3D

## Mensagens de erro dos campos (schema.rs)

godot-schema-distribuicao_invalida = distribuição inválida: { $e }
godot-schema-familia_invalida = família inválida
godot-schema-texto_ron_esperado = texto RON esperado
godot-schema-so_a_distribuicao_deslocada_tem = só a distribuição deslocada tem uma distribuição interna
godot-schema-a_distribuicao_nao_tem_o = a distribuição não tem o parâmetro "{ $key }"
godot-schema-modo_de_movimentacao_desconhecido = modo de movimentação desconhecido: { $name }
godot-schema-modo_de_movimentacao_esperado = modo de movimentação esperado
godot-schema-a_velocidade_deve_ser_maior = a velocidade deve ser maior que zero
godot-schema-campo_desconhecido = campo desconhecido: { $w }
godot-schema-rotulo_esperado = rótulo esperado
godot-schema-o_rotulo_do_peso_nao = o rótulo do peso não pode ficar vazio
godot-schema-ordem_safe_sequential_ou_parallel = ordem: Safe, Sequential ou Parallel
godot-schema-crie_antes_uma_pessoa_ou = crie antes uma pessoa (ou um despachante) para operar a ponte
godot-schema-controle_cabin_pendant_ou_radio = controle: Cabin, Pendant ou Radio
godot-schema-escolha_a_equipe_do_operador = escolha a equipe do operador
godot-schema-escolha_antes_botoeira_ou_radio = escolha antes botoeira ou rádio
godot-schema-tipo_de_espera_table_pallet = tipo de espera: Table, Pallet, Shelf ou Floor, não { $k }
godot-schema-camadas_demais = camadas demais
godot-schema-espera_nao_tem_o_campo = espera não tem o campo "{ $rest }"
godot-schema-espera = espera: { $e }
godot-schema-sexo_female_male_ou_vazio = sexo: Female, Male ou vazio, não { $other }
godot-schema-idade_grande_demais = idade grande demais
godot-schema-pessoa_nao_tem_o_campo = pessoa não tem o campo "{ $field }"
godot-schema-pessoa = pessoa: { $e }
godot-schema-crie_antes_um_transportador_ou = crie antes um transportador ou um despachante para operar o processo
godot-schema-ligue_a_transferencia_antes = ligue a transferência antes
godot-schema-caminho_straight_ou_network = caminho: Straight ou Network
godot-schema-ligue_as_quebras_antes = ligue as quebras antes
godot-schema-ligue_os_turnos_antes = ligue os turnos antes
godot-schema-fim_do_turno_pause_ou = fim do turno: Pause ou Finish, não { $t }
godot-schema-pausas_invalidas = pausas inválidas: { $e }
godot-schema-escolha_um_calendario = escolha um calendário
godot-schema-ligue_o_lote_antes = ligue o lote antes
godot-schema-ligue_antes = ligue { $head } antes
godot-schema-ligue_os_operadores_antes = ligue os operadores antes
godot-schema-o_ganho_da_equipe_deve = o ganho da equipe deve ser maior que zero
godot-schema-objeto_nao_existe = objeto "{ $object }" não existe
godot-schema-posicao_x_y_ou_z = posição: x, y ou z
godot-schema-politica_esperada = política esperada
godot-schema-politica_de_acesso_desconhecida = política de acesso desconhecida: { $v }
godot-schema-o_rotulo_da_prioridade_nao = o rótulo da prioridade não pode ficar vazio
godot-schema-script_esperado = script esperado
godot-schema-regra_firstfree_random_ou_abc = regra: FirstFree, Random ou Abc
godot-schema-o_rotulo_nao_pode_ficar = o rótulo não pode ficar vazio
godot-schema-modo_unpack_ou_split = modo: Unpack ou Split
godot-schema-o_modelo_nao_tem_calendarios = o modelo não tem calendários (crie um em "calendars")
godot-schema-origem_dos_turnos_inline_ou = origem dos turnos: inline ou calendar
godot-schema-texto_esperado = texto esperado
godot-schema-modo_esperado = modo esperado
godot-schema-perfil_brasil_ou_branco = perfil: Brasil ou Branco
godot-schema-largura_da_faixa = largura da faixa
godot-schema-parada_no_cruzamento = parada no cruzamento
godot-schema-cor_invalida_use_rrggbb = cor inválida: { $t } (use #rrggbb)
godot-schema-cor_rrggbb = cor: #rrggbb
godot-schema-run_nao_tem_o_campo = run não tem o campo "{ $path }"
godot-schema-regra_esperada = regra esperada
godot-schema-lista_de_pesos_esperada = lista de pesos esperada
godot-schema-pesos_invalidos = pesos inválidos: { $e }
godot-schema-campo_desconhecido_output = campo desconhecido: output.{ $rest }
godot-schema-a_escala_precisa_ser_maior = a escala precisa ser maior que zero
godot-schema-giro_um_numero_de_graus = giro: um número de graus
godot-schema-campo_desconhecido_visual = campo desconhecido: visual.{ $rest }

godot-ui-bar-casa = Casa

## Rodada 14: mensagens e painéis do GDScript (`UtsimDocument.texto`)

godot-gd-obstaculos-visiveis = Obstáculos e folgas visíveis.
godot-gd-obstaculos-ocultos = Obstáculos e folgas ocultos.
godot-gd-desempenho-visiveis = Medidas de desempenho visíveis (F3).
godot-gd-desempenho-ocultas = Medidas de desempenho ocultas (F3).
godot-gd-mao-dupla = mão dupla
godot-gd-mao-unica = mão única
godot-gd-mao-unica-setas = mão única (setas no piso)
godot-gd-do-modelo = o do modelo
godot-gd-foi-cancelada = foi cancelada
godot-gd-falhou = falhou
godot-gd-ligada = ligada
godot-gd-desligada = desligada
godot-gd-ligado = ligado
godot-gd-desligado = desligado
godot-gd-abrindo-leituras = Abrindo as leituras de { $p1 } no UTSim Estatística.
godot-gd-valores-lidos-de = Valores lidos de { $p1 }.
godot-gd-nao-leu = Não leu { $p1 }: { $p2 }
godot-gd-objetos-copiados =
    { $p1 } { $p1 ->
        [one] objeto copiado
       *[other] objetos copiados
    }.
godot-gd-nada-para-copiar = Nada selecionado para copiar.
godot-gd-removida-desfaz = Removida: { $p1 } (Ctrl+Z desfaz)
godot-gd-nao-removeu-msg = Não removeu: { $p1 }
godot-gd-nao-ligou-msg = Não ligou: { $p1 }
godot-gd-passagem-cabem = , { $p1 } s de passagem, cabem { $p2 }
godot-gd-esteira-desenhada = Esteira desenhada
godot-gd-esteira-com-canaletas = Esteira com canaletas
godot-gd-caminho-no-piso = Caminho no piso
godot-gd-pedestres = Pedestres
godot-gd-empilhadeiras = Empilhadeiras
godot-gd-misto = Misto
godot-gd-trajeto-inclinacao-pct = Inclinação (%; negativa desce)
godot-gd-trajeto-acima-maximo = • Acima do máximo: a peça escorrega ou tomba.
godot-gd-trajeto-rampa-ingreme = • Rampa mais íngreme que 1:12 (8,33 %).
godot-gd-esteira-simples = Esteira simples
godot-gd-motorizada = Motorizada
godot-gd-roletes-gravidade = Roletes por gravidade
godot-gd-calha = Calha (escorrega)
godot-gd-trajeto-caminho-n = Caminho { $p1 }
godot-gd-trajeto-esteira-n = Esteira { $p1 }
godot-gd-trajeto-no-n = Nó { $p1 }

godot-gd-corredores-corredor_cancelado_clique_no_chao = Corredor cancelado. Clique no chão para começar outro, ou Esc para sair da ferramenta.
godot-gd-corredores-nao_criou_o_corredor = Não criou o corredor: { $p1 }
godot-gd-corredores-corredor_criado_m = Corredor { $p1 } criado ({ $p2 }, { $p3 } m).
godot-gd-corredores-vertice_de_movido = Vértice { $p1 } de { $p2 } movido.
godot-gd-corredores-nao_moveu = Não moveu: { $p1 }
godot-gd-corredores-vertice_acrescentado_a = Vértice acrescentado a { $p1 }.
godot-gd-corredores-nao_acrescentou = Não acrescentou: { $p1 }
godot-gd-corredores-corredor_excluido = Corredor { $p1 } excluído.
godot-gd-corredores-nao_excluiu = Não excluiu: { $p1 }
godot-gd-corredores-ponto_de_acesso_de_movido = Ponto de acesso de { $p1 } movido (desligue "Ponto de acesso próprio" no painel para voltar ao padrão).
godot-gd-corredores-nao_moveu_o_ponto_de = Não moveu o ponto de acesso: { $p1 }
godot-gd-corredores-corredor_agora_e_de = Corredor { $p1 } agora é de { $p2 }.
godot-gd-corredores-corredor_com_m_de_largura = Corredor { $p1 } com { $p2 } m de largura.
godot-gd-corredores-nao_mudou_a_largura = Não mudou a largura: { $p1 }
godot-gd-corredores-corredor_em = Corredor { $p1 } em { $p2 }.
godot-gd-corredores-perfil_do_corredor = Perfil do corredor { $p1 }: { $p2 }.
godot-gd-corredores-sentido_do_corredor_invertido = Sentido do corredor { $p1 } invertido.
godot-gd-corredores-corredor_agora_se_chama = Corredor { $p1 } agora se chama { $p2 }.
godot-gd-corredores-nao_renomeou = Não renomeou: { $p1 }
godot-gd-corredores-vertice_removido = Vértice removido.
godot-gd-corredores-nao_removeu = Não removeu: { $p1 }
godot-gd-corredores-esse_aviso_nao_aponta_para = Esse aviso não aponta para nenhum corredor ou objeto da cena.
godot-gd-corredores-destacado_na_cena = Destacado na cena: { $p1 }.
godot-gd-editor-equipe_arraste_do_despachante_a = Equipe: arraste do despachante a um transportador, ou de um objeto a quem leva os itens (Shift: de um processo a quem fornece os operadores); de novo desfaz. Mais fácil: arraste do nó laranja do operador até a máquina. Esc cancela.
godot-gd-editor-ligar_arraste_do_bloco_de = Ligar: arraste do bloco de origem até o destino, ou clique em um e depois no outro (Esc cancela).
godot-gd-editor-clique_no_chao_para_colocar = Clique no chão para colocar (Shift mantém a ferramenta, Esc cancela).
godot-gd-editor-corredor_clique_no_chao_para = Corredor: clique no chão para pôr os pontos do eixo (Enter ou duplo clique termina), clique num corredor para editá-lo, arraste um vértice para movê-lo; Esc cancela.
godot-gd-editor-zona_clique_no_chao_para = Zona: clique no chão para pôr os cantos da zona de risco; o primeiro ponto, Enter ou duplo clique fecha; clique numa zona para editá-la. Esc cancela.
godot-gd-editor-rede_clique_no_chao_para = Rede: clique no chão para pôr nós (cada clique continua o caminho), clique num trecho para editá-lo, arraste um nó para movê-lo; Esc encerra o caminho.
godot-gd-editor-nao_mudou_a_zona = Não mudou a zona: { $p1 }
godot-gd-editor-nao_ligou = Não ligou: { $p1 }
godot-gd-editor-nao_removeu = Não removeu: { $p1 }
godot-gd-editor-nao_girou = Não girou: { $p1 }
godot-gd-editor-nao_criou_a_zona = Não criou a zona: { $p1 }
godot-gd-editor-zona_criada_escolha_o_tipo = Zona { $p1 } criada: escolha o tipo, o nível e o EPI no painel (Ctrl+Z desfaz).
godot-gd-editor-zona_excluida_ctrl_z_desfaz = Zona { $p1 } excluída (Ctrl+Z desfaz).
godot-gd-editor-nao_excluiu_a_zona = Não excluiu a zona: { $p1 }
godot-gd-editor-objeto_s_excluidos =
    { $p1 } { $p1 ->
        [one] objeto excluído
       *[other] objetos excluídos
    }.
godot-gd-editor-colocacao_cancelada_solte_o_objeto = Colocação cancelada: solte o objeto sobre o chão.
godot-gd-editor-aponte_para_o_chao_para = Aponte para o chão para colocar o objeto.
godot-gd-editor-nao_foi_possivel_colocar = Não foi possível colocar: { $p1 }
godot-gd-editor-colocado = { $p1 } colocado.
godot-gd-editor-equipe_cancelado = Equipe: cancelado.
godot-gd-editor-nada_selecionado_para_girar = Nada selecionado para girar.
godot-gd-editor-girado_ctrl_z_desfaz = Girado { $p1 } (Ctrl+Z desfaz).
godot-gd-editor-objetos_girados_cada_um_em = { $p1 } objetos girados { $p2 }, cada um em volta do próprio centro (Ctrl+Z desfaz).
godot-gd-editor-nenhum_corredor_selecionado_para_excluir = Nenhum corredor selecionado para excluir.
godot-gd-editor-nenhum_no_selecionado_para_remover = Nenhum nó selecionado para remover.
godot-gd-principal-arraste_um_objeto_da_biblioteca = Arraste um objeto da Biblioteca para o chão, ou abra um modelo (Ctrl+O).
godot-gd-principal-encaixe_na_grade_passo_m = Encaixe na grade { $p1 } (passo { $p2 } m).
godot-gd-principal-o_modo_missao_do_utsim = O modo missão do UTSim Ensino não está nesta versão.
godot-gd-principal-para_andar_pela_fabrica_simule = Para andar pela fábrica, simule primeiro (F no Simular, depois da rodada).
godot-gd-principal-alterado = { $p1 }: { $p2 } alterado.
godot-gd-principal-nao_alterou = Não alterou { $p1 }: { $p2 }
godot-gd-principal-novo_modelo = Novo modelo.
godot-gd-principal-nao_abriu = Não abriu { $p1 }: { $p2 }
godot-gd-principal-aberto_mas_salvar_fica_para = { $p1 } aberto, mas { $p2 }: Salvar fica para quando a pessoa fechar (ou use Salvar como).
godot-gd-principal-aberto_objetos_simular_roda_as = { $p1 } aberto: { $p2 } objetos. Simular roda as replicações.
godot-gd-principal-aberto_com_problema_s_veja =
    { $p1 } aberto com { $p2 } { $p2 ->
        [one] problema
       *[other] problemas
    }: veja o painel de propriedades.
godot-gd-principal-o_outro_app_fechou_o = O outro app fechou o modelo: as abas Qualidade, Segurança e Manutenção voltaram a editar.
godot-gd-principal-outro_app_gravou_e_ha = Outro app gravou { $p1 } e há mudanças suas não salvas: use Salvar como para não apagar o trabalho dele.
godot-gd-principal-nao_releu = Não releu { $p1 }: { $p2 }
godot-gd-principal-foi_gravado_por_outro_app = { $p1 } foi gravado por outro app: recarregado do disco.
godot-gd-principal-salve_o_modelo_antes_de = Salve o modelo antes de abrir no { $p1 }.
godot-gd-principal-abrindo_no_ele_edita_o = Abrindo { $p1 } no { $p2 }: ele edita o modelo; o UTSim 3D relê o que ele gravar.
godot-gd-principal-salvo_em = Salvo em { $p1 }.
godot-gd-principal-nao_salvou = Não salvou: { $p1 }
godot-gd-principal-desfeito = Desfeito.
godot-gd-principal-nada_para_desfazer = Nada para desfazer.
godot-gd-principal-refeito = Refeito.
godot-gd-principal-nada_para_refazer = Nada para refazer.
godot-gd-principal-nao_foi_possivel_desenhar_o = Não foi possível desenhar o modelo: { $p1 }
godot-gd-principal-escolha_o_arquivo_de_dados = Escolha o arquivo de dados.
godot-gd-principal-valores_colados = Valores colados: { $p1 }
godot-gd-principal-aguarde = Aguarde: { $p1 }.
godot-gd-principal-nao_ajustou = Não ajustou: { $p1 }
godot-gd-principal-agora_usa_dados_reais_ctrl = { $p1 } agora usa dados reais (Ctrl+Z desfaz).
godot-gd-principal-nao_aplicou_os_dados = Não aplicou os dados: { $p1 }
godot-gd-principal-volta_a_usar_uma_distribuicao = { $p1 } volta a usar uma distribuição (Ctrl+Z desfaz).
godot-gd-principal-nao_voltou_para_a_distribuicao = Não voltou para a distribuição: { $p1 }
godot-gd-principal-sem_relatorio = Sem relatório: { $p1 }
godot-gd-principal-relatorio_gravado_em = Relatório gravado em { $p1 }.
godot-gd-principal-relatorio_gravado_em_arquivos = Relatório gravado em { $p1 } ({ $p2 } arquivos).
godot-gd-principal-objetos_alinhados = { $p1 } objetos alinhados.
godot-gd-principal-objetos_distribuidos = { $p1 } objetos distribuídos.
godot-gd-principal-objeto_s_recortados_ctrl_v =
    { $p1 } { $p1 ->
        [one] objeto recortado
       *[other] objetos recortados
    } (Ctrl+V cola).
godot-gd-principal-nada_selecionado_para_recortar = Nada selecionado para recortar.
godot-gd-principal-objeto_s_colados =
    { $p1 } { $p1 ->
        [one] objeto colado
       *[other] objetos colados
    }: { $p2 }.
godot-gd-principal-o_modelo_tem_problema_s =
    O modelo tem { $p1 } { $p1 ->
        [one] problema e
       *[other] problemas e
    } não roda: { $p2 }
godot-gd-principal-a_simulacao_nao_comecou = A simulação não começou: { $p1 }
godot-gd-principal-a_simulacao = A simulação { $p1 }: { $p2 }
godot-gd-principal-nao_foi_possivel_gravar_a = Não foi possível gravar a replicação 0: { $p1 }
godot-gd-principal-replicacoes_concluidas_reproduzindo_a_replicacao = { $p1 } replicações concluídas. Reproduzindo a replicação 0.
godot-gd-principal-texto = { $p1 }...
godot-gd-principal-cancelando = Cancelando: { $p1 }...
godot-gd-principal-simule_o_modelo_para_ver = Simule o modelo para ver os resultados (Tab ou o botão Simular).
godot-gd-principal-os_fluxos_de_tokens_aparecem = Os fluxos de tokens aparecem no Simular.
godot-gd-principal-selecione_os_objetos_da_parte = Selecione os objetos da parte do processo (ou dois: o início e o fim) e peça o PERT/CPM.
godot-gd-principal-pert_cpm_de_ate_selecione = PERT/CPM de... até...: selecione exatamente dois objetos.
godot-gd-principal-o_modelo_tem_problemas_e = O modelo tem problemas e não roda: o PERT/CPM precisa de um modelo válido.
godot-gd-principal-nao_comecou = Não começou: { $p1 }
godot-gd-principal-um_replay_gravado_nao_roda = Um replay gravado não roda de novo: abra o modelo.
godot-gd-principal-nao_ligou_a_serie_e = Não ligou a série e os histogramas: { $p1 }
godot-gd-principal-ligado_no_modelo_ctrl_z = Ligado no modelo ({ $p1 }; Ctrl+Z desfaz). Simulando de novo...
godot-gd-principal-aquecimento_aplicado_ao_modelo_run = Aquecimento { $p1 } aplicado ao modelo (run.warmup). Simule de novo para os números com ele.
godot-gd-principal-nao_aplicou_o_aquecimento = Não aplicou o aquecimento: { $p1 }
godot-gd-principal-nao_abriu_pedido_na_linha = Não abriu { $p1 } (pedido na linha de comando).
godot-gd-principal-nao_foi_possivel_reproduzir = Não foi possível reproduzir { $p1 }: { $p2 }
godot-gd-principal-replay_aberto_blocos_so_reproducao = Replay { $p1 } aberto ({ $p2 } blocos): só reprodução; Editar volta ao modelo em edição.
godot-gd-principal-simule_um_modelo_antes_de = Simule um modelo antes de gravar o replay.
godot-gd-principal-replay_da_replicacao_0_gravado = Replay da replicação 0 gravado em { $p1 }.
godot-gd-principal-nao_gravou_o_replay = Não gravou o replay: { $p1 }
godot-gd-principal-camera_de_volta = Câmera de volta.
godot-gd-rede-caminho_encerrado_clique_no_chao = Caminho encerrado. Clique no chão para começar outro, ou Esc para sair da ferramenta.
godot-gd-rede-nao_colocou_o_no = Não colocou o nó: { $p1 }
godot-gd-rede-no_ligado_a_clique_de = Nó { $p1 } ligado a { $p2 }. Clique de novo para continuar o caminho (Esc para).
godot-gd-rede-no_colocado_clique_em_outro = Nó { $p1 } colocado. Clique em outro ponto para traçar um trecho (Esc para).
godot-gd-rede-no_movido = Nó { $p1 } movido.
godot-gd-rede-nao_moveu = Não moveu: { $p1 }
godot-gd-rede-trecho_criado = Trecho { $p1 } → { $p2 } criado ({ $p3 }).
godot-gd-rede-nao_ligou = Não ligou: { $p1 }
godot-gd-rede-trecho_removido = Trecho { $p1 } → { $p2 } removido.
godot-gd-rede-no_removido_com_os_trechos = Nó { $p1 } removido com os trechos dele.
godot-gd-rede-trecho_em = Trecho { $p1 } → { $p2 } em { $p3 }.
godot-gd-rede-pista_controlada_ate_veiculo_s =
    Pista controlada: até { $p1 } { $p1 ->
        [one] veículo
       *[other] veículos
    } ao mesmo tempo no trecho.
godot-gd-rede-trecho_sem_controle_de_trafego = Trecho sem controle de tráfego.
godot-gd-rede-trecho_invertido = Trecho invertido: { $p1 } → { $p2 }.
godot-gd-rede-no_agora_se_chama = Nó { $p1 } agora se chama { $p2 }.
godot-gd-rede-nao_renomeou = Não renomeou: { $p1 }
godot-gd-trajeto-clique_no_chao_para_por = { $p1 }: clique no chão para pôr os nós (Enter ou duplo clique termina). PageUp/PageDown sobe e desce o próximo nó, [ e ] arredondam a curva. Clique num nó para editá-lo; Esc cancela.
godot-gd-trajeto-arraste_em_volta_do_meio = Arraste em volta do meio para girar { $p1 } (de 15 em 15°; Shift: de 1 em 1°).
godot-gd-trajeto-arraste_para_mover_inteiro_r = Arraste para mover { $p1 } inteiro; R e Shift+R giram 15°.
godot-gd-trajeto-arraste_para_curvar_o_trecho = Arraste para curvar o trecho { $p1 } de { $p2 }.
godot-gd-trajeto-no_acrescentado_em_o_trecho = Nó acrescentado em { $p1 } (o trecho foi dividido).
godot-gd-trajeto-nao_acrescentou = Não acrescentou: { $p1 }
godot-gd-trajeto-aponte_para_o_chao_para = Aponte para o chão para pôr o nó.
godot-gd-trajeto-no_em_m_altura_m = { $p1 }: nó { $p2 } em ({ $p3 }; { $p4 }) m, altura { $p5 } m ({ $p6 }).
godot-gd-trajeto-comeca_em_m_altura_m = { $p1 }: começa em ({ $p2 }; { $p3 }) m, altura { $p4 } m. Clique os próximos nós; Enter termina.
godot-gd-trajeto-trecho_de_selecionado_apagar_o = Trecho { $p1 } de { $p2 } selecionado: "Apagar o trecho entre os nós" no painel; arraste o losango para curvá-lo.
godot-gd-trajeto-movido_m_em_x_e = { $p1 } movido { $p2 } m em X e { $p3 } m em Y (Ctrl+Z desfaz).
godot-gd-trajeto-nao_moveu = Não moveu: { $p1 }
godot-gd-trajeto-trecho_curvado_no_com_raio = Trecho curvado: nó { $p1 } com raio de { $p2 } m (Ctrl+Z desfaz).
godot-gd-trajeto-nao_curvou = Não curvou: { $p1 }
godot-gd-trajeto-comprimento_do_trecho_m_enter = Comprimento do trecho: { $p1 } m (Enter põe o nó na direção do cursor, Backspace apaga).
godot-gd-trajeto-proximo_no_a_m_de = Próximo nó a { $p1 } m de altura.
godot-gd-trajeto-raio_das_proximas_curvas_m = Raio das próximas curvas: { $p1 } m.
godot-gd-trajeto-descendo_por_m_o_no = Descendo { $p1 } % por { $p2 } m, o nó ficaria abaixo do piso.
godot-gd-trajeto-o_trecho_de_m_nao = O trecho de { $p1 } m não alcança a altura de { $p2 } m.
godot-gd-trajeto-no_a_m_do_anterior = { $p1 }: nó { $p2 } a { $p3 } m do anterior.
godot-gd-trajeto-girado_r_gira_de_novo = { $p1 } girado { $p2 }° (R gira de novo; Shift+R para o outro lado).
godot-gd-trajeto-nao_girou = Não girou: { $p1 }
godot-gd-trajeto-ultimo_no_desfeito = Último nó desfeito.
godot-gd-trajeto-nao_criou = Não criou: { $p1 }
godot-gd-trajeto-criado_m = { $p1 } { $p2 } criado: { $p3 } m{ $p4 }.
godot-gd-trajeto-desenho_cancelado_clique_no_chao = Desenho cancelado. Clique no chão para começar outro, ou Esc para sair da ferramenta.
godot-gd-trajeto-no_s_removido_s =
    { $p1 } { $p1 ->
        [one] nó removido
       *[other] nós removidos
    }.
godot-gd-trajeto-nao_removeu = Não removeu: { $p1 }
godot-gd-trajeto-no_x_y_altura_m = { $p1 }, nó { $p2 }: X { $p3 }, Y { $p4 }, altura { $p5 } m. Setas e PageUp/PageDown ajustam (Shift: 1 cm).
godot-gd-trajeto-nao_e_uma_esteira_desenhada = { $p1 } não é uma esteira desenhada.
godot-gd-trajeto-esteira_clique_num_no_para = Esteira { $p1 }: clique num nó para editá-lo; setas e PageUp/PageDown movem os nós selecionados.
godot-gd-trajeto-inclinacao_fixa = Inclinação fixa { $p1 } ({ $p2 } %).
godot-gd-trajeto-esteira_dividida = Esteira dividida: { $p1 } → { $p2 }.
godot-gd-trajeto-trecho_apagado_de = Trecho apagado de { $p1 }.
godot-gd-trajeto-e_viraram_um_trajeto_so = { $p1 } e { $p2 } viraram um trajeto só (Ctrl+Z desfaz).
godot-gd-trajeto-agora_corre_no_sentido_contrario = { $p1 } agora corre no sentido contrário (as ligações também).
godot-gd-trajeto-desenhado_a_m_de = { $p1 } desenhado a { $p2 } m de { $p3 }.
godot-gd-trajeto-rampa_s_de_acesso_de =
    { $p1 } { $p1 ->
        [one] rampa de acesso de 1:12 acrescentada
       *[other] rampas de acesso de 1:12 acrescentadas
    } a { $p2 }.
godot-gd-trajeto-nao_mudou = Não mudou: { $p1 }
godot-gd-trajeto-no_a_m_de_altura = { $p1 }, nó { $p2 } a { $p3 } m de altura (perfil; Ctrl+Z desfaz).
godot-gd-trajeto-nao_mudou_a_altura = Não mudou a altura: { $p1 }

godot-gd-trajeto-clique_no_chao_para_por-2 = Clique no chão para pôr os nós; o trecho mostra o comprimento e a inclinação. Os nós encaixam nas portas (pontas das esteiras e objetos), nos nós de outros trajetos, de 15 em 15° e na grade. PageUp/PageDown: altura do próximo nó; [ e ]: raio da curva; Enter termina; Backspace volta um nó; Esc cancela.
godot-gd-trajeto-altura_do_proximo_no_m = Altura do próximo nó (m)
godot-gd-trajeto-inclinacao_fixa_no_proximo_trecho = Inclinação fixa no próximo trecho
godot-gd-trajeto-raio_das_curvas_m_0 = Raio das curvas (m; 0 = quina)
godot-gd-trajeto-largura_do_caminho_m = Largura do caminho (m)
godot-gd-trajeto-esteira_s_desenhada_s_caminho =
    { $p1 } { $p1 ->
        [one] esteira desenhada
       *[other] esteiras desenhadas
    }, { $p2 } { $p2 ->
        [one] caminho
       *[other] caminhos
    }.
godot-gd-trajeto-maior_inclinacao_maximo = Maior inclinação { $p1 }° (máximo { $p2 }°).
godot-gd-trajeto-girar_graus_anti_horario = Girar (graus; + anti-horário)
godot-gd-trajeto-girar_o_trajeto = Girar o trajeto
godot-gd-trajeto-girar_15_anti_horario_r = Girar 15° (anti-horário, R)
godot-gd-trajeto-girar_15_horario_shift_r = Girar 15° (horário, Shift+R)
godot-gd-trajeto-juntar_com_num_trajeto_so = Juntar com { $p1 } num trajeto só
godot-gd-trajeto-paralelo_a_m_a_esquerda = Paralelo a (m; + à esquerda)
godot-gd-trajeto-desenhar_paralelo = Desenhar paralelo
godot-gd-trajeto-escada_degraus_so_pedestres = Escada (degraus; só pedestres)
godot-gd-trajeto-rampas_de_acesso_nas_pontas = Rampas de acesso nas pontas (1:12)
godot-gd-trajeto-tipo = Tipo
godot-gd-trajeto-acionamento = Acionamento
godot-gd-trajeto-inverter_o_sentido = Inverter o sentido
godot-gd-trajeto-largura_m = Largura (m)
godot-gd-trajeto-altura_m = Altura (m)
godot-gd-trajeto-raio_da_curva_m_0 = Raio da curva (m; 0 = quina)
godot-gd-trajeto-dividir_a_esteira_neste_no = Dividir a esteira neste nó
godot-gd-trajeto-nos_selecionados = { $p1 } nós selecionados
godot-gd-trajeto-nivelar_na_altura_m = Nivelar na altura (m)
godot-gd-trajeto-inclinacao_constante_entre_as_pontas = Inclinação constante entre as pontas
godot-gd-trajeto-remover_no_s_del = Remover nó(s) (Del)
godot-gd-trajeto-apagar_o_trecho_entre_os = Apagar o trecho entre os nós
godot-gd-trajeto-arraste_o_losango_do_no = Arraste o losango do nó no chão para movê-lo e o topo da haste para mudar a altura. Setas movem (Shift: 1 cm), PageUp/PageDown sobem e descem, [ e ] mudam o raio. Shift + clique na linha acrescenta um nó; um clique no losango laranja (ou dois nós vizinhos com Shift + clique) seleciona o trecho, que pode ser apagado. Arraste o losango laranja no meio de um trecho para curvá-lo e o quadrado verde no meio do trajeto para movê-lo inteiro e o anel laranja ao lado dele para girá-lo; sem nó selecionado, as setas movem e R gira o trajeto.
godot-gd-principal-o_modelo_tem_mudancas_que = O modelo tem mudanças que não foram salvas. Descartar?

## Rodada 14: janelas e erros restantes

godot-ui-class-sem_replicacao = sem replicação
godot-ui-class-sem_rastro_da_replicacao = sem rastro da replicação
godot-ui-class-a_janela_de_valores_nao = a janela de valores não está aberta
godot-ui-class-valor_es_lidos_de_coluna =
    { $n } { $n ->
        [one] valor lido
       *[other] valores lidos
    } de { $file } (coluna { $column }).
godot-ui-class-nao_leu_o_arquivo = Não leu o arquivo: { $e }
godot-ui-class-relatorio_gravado_em = Relatório gravado em { $f }.
godot-ui-class-escreva_o_arquivo_do_relatorio = Escreva o arquivo do relatório.
godot-ui-class-relatorio_pert_cpm_gravado_em = Relatório PERT/CPM gravado em { $path }.
godot-ui-class-nao_gravou = Não gravou { $path }: { $e }
godot-ui-class-tabela_s_csv_em =
    { $files } { $files ->
        [one] tabela
       *[other] tabelas
    } CSV em { $dir }.
godot-ui-class-nao_gravou_os_csv_em = Não gravou os CSV em { $dir }: { $e }
godot-ui-class-escreva_a_pasta_da_exportacao = Escreva a pasta da exportação.
godot-ui-class-arquivo_s_csv_em =
    { $files } { $files ->
        [one] arquivo
       *[other] arquivos
    } CSV em { $dir }.
godot-ui-class-nao_exportou = Não exportou: { $e }
godot-ui-class-so_leitura = { $quem }: só leitura
godot-ui-class-na_ultima_simulacao_media_semiamplitude =
    Na última simulação ({ $replicacoes } { $replicacoes ->
        [one] replicação
       *[other] replicações
    }), média ± semiamplitude do IC de 95 %:
godot-schema-inteiro_maior_ou_igual_a_zero = { $key } precisa ser um inteiro >= 0
godot-schema-numero_maior_ou_igual_a_zero = { $w }: número maior ou igual a zero
godot-schema-nao_tem_o_campo = { $name } não tem o campo "{ $field }"
godot-schema-campo_maior_que_zero = { $field }: precisa ser maior que zero
godot-schema-inteiro_minimo = { $what } precisa ser um inteiro >= { $min }
godot-schema-numero_esperado_de = { $what }: número esperado
godot-schema-verdadeiro_ou_falso_esperado = { $what }: verdadeiro ou falso esperado

## aba Ambiental

aba-abrir-ambiental = Abrir no UTSim Ambiental
aba-abrir-ambiental-dica = Abre este nó no UTSim Ambiental (mapa de calor, fluxos, pegada, cenários e metas). O modelo é salvo antes e fica com o outro app para editar; o UTSim 3D relê o que ele gravar.
godot-indicadores-energia = Energia
godot-indicadores-energia_ajuda = Potência de cada grupo de estado × horas no estado, na janela de estatística (média das replicações).
godot-indicadores-energia_sem_valor = Energia sem valor
godot-indicadores-energia_sem_valor_ajuda = Parte da energia gasta esperando, parada ou fora de turno.
godot-indicadores-kwh_por_peca = Energia por peça
godot-indicadores-kwh_por_peca_ajuda = Energia do nó dividida pelas peças que saíram dele.

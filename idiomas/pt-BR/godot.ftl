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

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

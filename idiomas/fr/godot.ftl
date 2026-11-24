# Traduction automatique, en attente de relecture humaine.
# Tradução automática, aguardando revisão humana.

## UTSim 3D (cliente Godot): configurações e campos de distribuição

barra-configuracoes = Paramètres
barra-configuracoes-dica = Préférences de UTSim 3D sur cet ordinateur
configuracoes-titulo = Paramètres
configuracoes-distribuicoes = Les champs de distribution s’ouvrent en
configuracoes-formulario = Formulaire
configuracoes-codigo = Code
configuracoes-distribuicoes-ajuda = S’applique à tous les champs de distribution (temps de traitement, arrivées, chargement, déchargement...). Le bouton {"{"} {"}"} de chaque champ ne change toujours que ce champ, jusqu’au choix d’un autre objet.
configuracoes-fechar = Fermer
props-editar_como_texto = Modifier comme code (texte RON)
props-editar_como_formulario = Revenir au formulaire

## Painéis ao vivo em janela própria

vivo-soltar = Fenêtre
vivo-soltar-dica = Ouvre ce panneau dans sa propre fenêtre, qui peut aller sur un autre écran ; il reste en direct
vivo-prender = Retour
vivo-prender-dica = Ferme la fenêtre et remet le panneau sur la vue 3D
vivo-janela-titulo = UTSim 3D : { $objeto }

## Idioma

configuracoes-idioma = Langue
configuracoes-idioma-ajuda = Pour l’instant, seuls la fenêtre Paramètres, les panneaux en direct et les onglets Qualité, Sécurité et Maintenance des nœuds changent de langue ; les autres écrans restent en portugais.

## Curva de fadiga da pessoa

fadiga-secao = Courbe de fatigue (étude des temps)
fadiga-nota = Lecture seule. Les relevés et l’ajustement se modifient dans UTSim Estatística ou avec la commande utsim pessoa ajustar.
fadiga-tarefa = Tâche { $tarefa } : { $leituras } relevés à { $horarios } horaires
fadiga-tarefa-metodo = Tâche { $tarefa }, méthode { $metodo } : { $leituras } relevés à { $horarios } horaires
fadiga-resumo = Reposée : { $t0 }. À la fatigue maximale, { $acrescimo } % de plus. Fatigue λ = { $lambda } par heure ; récupération μ = { $mu } par heure ({ $origem }).
fadiga-estimada = estimée
fadiga-padrao = par défaut
fadiga-hora = heure du poste
fadiga-pausa = Pause
fadiga-curva = Courbe ajustée
fadiga-media = Moyenne mesurée
fadiga-erro = Non ajustée : { $erro }
fadiga-abrir = Ouvrir dans UTSim Estatística
fadiga-abrir-ajuda = Ouvre les relevés de cette étude des temps dans une feuille de UTSim Estatística (une ligne par temps mesuré : tâche, méthode, heure du poste et temps) pour les analyser. Le modèle ne change pas.

## Abas dos nós (Qualidade, Segurança, Manutenção) e painéis ao vivo

aba-so-leitura = { $quem }. Lecture seule jusqu’à ce que cette personne ferme le modèle.
aba-abrir-qualidade = Ouvrir dans UTSim Qualité
aba-abrir-qualidade-dica = Ouvre ce nœud dans UTSim Qualité (défauts, contrôle, coûts, MSP). Le modèle est d’abord enregistré et reste à l’autre application pour l’édition ; UTSim 3D relit ce qu’elle enregistre.
aba-abrir-seguranca = Ouvrir dans UTSim Sécurité
aba-abrir-seguranca-dica = Ouvre ce nœud dans UTSim Sécurité au travail (dangers, matrice des risques, EPI). Le modèle est d’abord enregistré et reste à l’autre application pour l’édition ; UTSim 3D relit ce qu’elle enregistre.
aba-calculando = Calcul des chiffres de la simulation...
aba-nenhuma = aucune
aba-nenhum-escolha = (aucun)
aba-nenhum = aucun
aba-opcoes = Séparez par des virgules. Options : { $opcoes }.
aba-vazio-nao-usa = vide : non utilisé
aba-adicionar = + Ajouter
aba-adicionar-dica = Ajoute un élément à { $lista }
aba-abrir-item = Ouvre l’élément
aba-fechar-item = Ferme l’élément
aba-sem-nome = (sans nom)
aba-remover = Retirer
aba-remover-dica = Retire cet élément de la liste
vivo-fechar = Fermer
vivo-fechar-dica = Fermer le panneau (Échap ferme le plus récent)
vivo-fixar = Épingler
vivo-fixado = Épinglé
vivo-fixar-dica = Épingler garde ce panneau ; le prochain clic sur un objet en ouvre un autre à côté
vivo-sem-numeros = Pas de chiffres pour cet objet.
vivo-eixo-tempo = temps
vivo-eixo-itens = éléments
vivo-sem-dados = Pas encore de données.
vivo-estados = États dans la fenêtre
vivo-sem-estados = aucun état

## Painel ao vivo: linhas (rótulo e ajuda)

vivo-em-processamento-agora = En traitement maintenant
vivo-itens-no-objeto-neste-instante = Éléments dans l’objet à cet instant (en traitement ou prêts sans pouvoir sortir).
vivo-itens-em-espera-fila = Éléments en attente (file)
vivo-itens-nas-filas-que-alimentam = Éléments dans les files qui alimentent l’objet, à cet instant.
vivo-entradas = Entrées
vivo-itens-que-entraram-na-janela = Éléments entrés pendant la fenêtre.
vivo-ja-processados = Déjà traités
vivo-itens-que-sairam-do-objeto = Éléments sortis de l’objet pendant la fenêtre.
vivo-rejeitados-para-descarte = Rejetés au rebut
vivo-itens-que-chegaram-a-uma = Éléments partis d’ici arrivés à une sortie rebut (sink_role: Scrap), aussi par des files ou un transport.
vivo-nenhuma-saida-tem-sink-role = Aucune sortie n’a sink_role: Scrap ; marquez la sortie rebut dans le modèle pour compter les rebuts.
vivo-rejeitados-para-retrabalho = Rejetés pour retouche
vivo-itens-que-chegaram-a-um = Éléments partis d’ici arrivés à un poste de retouche, aussi par des files ou un transport.
vivo-ciclo-medio-permanencia = Cycle moyen (séjour)
vivo-tempo-medio-no-objeto-dos = Temps moyen dans l’objet des éléments sortis pendant la fenêtre.
vivo-espera-media-na-fila = Attente moyenne en file
vivo-tempo-medio-nas-filas-que = Temps moyen dans les files qui alimentent l’objet.
vivo-ocupado-processando = Occupé (en traitement)
vivo-fracao-da-janela-no-estado = Fraction de la fenêtre dans l’état en traitement.
vivo-ocioso = Inactif
vivo-fracao-da-janela-sem-trabalho = Fraction de la fenêtre sans travail.
vivo-bloqueado = Bloqué
vivo-pronto-sem-poder-sair-bloqueado = Prêt sans pouvoir sortir (bloqué ou en attente de transport).
vivo-parado-quebra = Arrêté (panne)
vivo-quebrado-aguardando-reparo-ou-em = En panne, en attente de réparation ou en maintenance.
vivo-setup = Réglage
vivo-fracao-em-setup = Fraction en réglage.
vivo-conteudo-agora = Contenu maintenant
vivo-itens-no-objeto-neste-instante-2 = Éléments dans l’objet à cet instant.
vivo-conteudo-medio = Contenu moyen
vivo-media-no-tempo-do-conteudo = Moyenne dans le temps du contenu, sur la fenêtre.
vivo-conteudo-maximo = Contenu maximal
vivo-maior-conteudo-da-janela = Plus grand contenu de la fenêtre.
vivo-espera-media = Attente moyenne
vivo-saidas = Sorties
vivo-itens-que-sairam-na-janela = Éléments sortis pendant la fenêtre.
vivo-fracao-da-janela-sem-poder = Fraction de la fenêtre sans pouvoir libérer l’élément créé.
vivo-recebidos-total = Reçus (total)
vivo-itens-que-chegaram-a-saida = Éléments arrivés à la sortie pendant la fenêtre.
vivo-vazao-por-unidade-de-tempo = Débit (par unité de temps)
vivo-recebidos-divididos-pela-duracao-da = Reçus divisés par la durée de la fenêtre.
vivo-tempo-medio-no-sistema = Temps moyen dans le système
vivo-da-criacao-do-item-a = De la création de l’élément à son arrivée ici.
vivo-papel-na-qualidade = Rôle dans la qualité
vivo-o-papel-da-saida-nos = Le rôle de la sortie dans les rapports qualité.
vivo-utilizacao = Utilisation
vivo-andando-carregando-descarregando-ou-trabalhando = En déplacement, chargement, déchargement ou au travail à un poste.
vivo-viagens = Trajets
vivo-deslocamentos-comecados-na-janela = Déplacements commencés pendant la fenêtre.
vivo-itens-transportados = Éléments transportés
vivo-itens-entregues-na-janela = Éléments livrés pendant la fenêtre.
vivo-carregando-agora = Transporte maintenant
vivo-itens-consigo-neste-instante = Éléments transportés à cet instant.
vivo-viajando-carregado = Roule chargé
vivo-fracao-da-janela-andando-com = Fraction de la fenêtre en déplacement avec charge.
vivo-viajando-vazio = Roule à vide
vivo-fracao-da-janela-andando-sem = Fraction de la fenêtre en déplacement sans charge.
vivo-trabalhando-num-posto = Au travail à un poste
vivo-fracao-da-janela-operando-um = Fraction de la fenêtre à conduire un processus (ou un pont roulant par boîte à boutons ou radio).
vivo-descanso-fadiga = Repos (fatigue)
vivo-fracao-da-janela-em-pausa = Fraction de la fenêtre en pause pour récupérer.
vivo-pausas = Pauses
vivo-vezes-que-entrou-em-descanso = Nombre de mises en repos pendant la fenêtre.
vivo-fadiga-media-maxima-fim-da = Fatigue moyenne / maximale (fin de la réplication)
vivo-a-fadiga-nao-esta-no = La fatigue n’est pas dans l’enregistrement de la lecture : valeurs du rapport final.
vivo-esperando-o-operador = Attend l’opérateur
vivo-fracao-da-janela-parada-na = Fraction de la fenêtre arrêtée à la charge jusqu’à ce que la personne de la boîte à boutons ou de la radio arrive à côté.

## Painel ao vivo: cabeçalho e seções

godot-ao-vivo-pessoa = personne
godot-ao-vivo-ponte_rolante = pont roulant
godot-ao-vivo-empilhadeira = chariot élévateur
godot-ao-vivo-veiculo = véhicule
godot-ao-vivo-aquecimento_ate_numeros_desde_0 = préchauffage jusqu’à { $inicio_estatisticas } : chiffres depuis 0
godot-ao-vivo-desde_o_fim_do_aquecimento = depuis la fin du préchauffage ({ $inicio_estatisticas })
godot-ao-vivo-desde_o_inicio = depuis le début
godot-ao-vivo-t_de_replicacao = t = { $t } sur { $fim } · réplication { $replicacao } · { $janela }
godot-ao-vivo-agora = Maintenant
godot-ao-vivo-marque_a_saida_de_refugo = marquez la sortie rebut
godot-ao-vivo-acumulado = Cumul
godot-ao-vivo-tempos = Temps
godot-ao-vivo-em_processo_wip = En cours (WIP)
godot-ao-vivo-agora_e_na_janela = Maintenant et sur la fenêtre
godot-ao-vivo-conteudo = Contenu
godot-ao-vivo-saida = Sortie
godot-ao-vivo-recebidos_acumulado = Reçus (cumul)
godot-ao-vivo-pessoa-2 = Personne
godot-ao-vivo-ponte_rolante-2 = Pont roulant
godot-ao-vivo-veiculo-2 = Véhicule
godot-ao-vivo-itens_consigo = Éléments transportés
godot-ao-vivo-fila = File

## Fluxos de tokens

godot-ui-flows-fluxos_de_tokens = Flux de jetons
godot-ui-flows-fechar = Fermer
godot-ui-flows-fechar_os_fluxos = Fermer les flux
godot-ui-flows-o_modelo_nao_tem_fluxos = Le modèle n’a pas de flux de jetons.
godot-ui-flows-tokens_em_cada_atividade_no = Jetons dans chaque activité à l’instant de la lecture ; en dessous, les comptes au fil de l’exécution (la ligne verticale t est l’instant actuel).
godot-ui-flows-zoom_ctrl_roda_no_grafo = Zoom (Ctrl + molette sur le graphe)
godot-ui-flows-diminui = Réduit
godot-ui-flows-aumenta = Agrandit
godot-ui-flows-ajustar = Ajuster
godot-ui-flows-cabe_na_largura = Tient dans la largeur
godot-ui-flows-sem_contagens = aucun compte

## Resultados

godot-ui-results-resumo = Synthèse
godot-ui-results-graficos = Graphiques
godot-ui-results-analise = Analyse
godot-ui-results-cenarios = Scénarios
godot-ui-results-otimizacao = Optimisation
godot-ui-results-ocupacao = Taux d’occupation
godot-ui-results-estados = États
godot-ui-results-serie_temporal = Série temporelle
godot-ui-results-histograma = Histogramme
godot-ui-results-nenhum_recurso_ocupado = aucune ressource occupée
godot-ui-results-invalido_use_um_numero_inteiro = { $what } "{ $text }" invalide : utilisez un nombre entier
godot-ui-results-serie_invalida_use_objeto_medida = série "{ $other }" invalide : utilisez objet.mesure
godot-ui-results-calcule_antes_quantas_replicacoes_faltam = calculez d’abord combien de réplications manquent
godot-ui-results-simule_o_modelo_antes_de = Simulez le modèle avant de comparer des scénarios.
godot-ui-results-simule_o_modelo_antes_de-2 = Simulez le modèle avant d’optimiser.
godot-ui-results-trabalho_desconhecido = tâche inconnue : { $other }
godot-ui-results-serie_curta_demais_para_o = Série trop courte pour le MSER-5 ({ $points } points) : augmentez le temps d’arrêt ou l’échantillonnage.
godot-ui-results-corte_sugerido_mas_o_minimo = Coupure suggérée : { $cut }, mais le minimum est tombé dans la seconde moitié de la série : l’exécution est courte ou ne se stabilise pas.
godot-ui-results-corte_sugerido_mser_5_antes = Coupure suggérée (MSER-5) : { $cut }. Avant elle, la moyenne monte ou descend encore.
godot-ui-results-gargalo_movel_na_maior_parte = Goulot mobile : { $n } la plupart du temps.
godot-ui-results-nenhum_recurso_ativo = Aucune ressource active.
godot-ui-results-replicacoes_faltam_para_o_alvo = { $reports } réplications ; il en manque { $more } pour la cible.
godot-ui-results-aquecimento_analisado = Préchauffage analysé : { $d }
godot-ui-results-lei_de_little_verde_s = Loi de Little : { $verde } vert(s), { $amarelo } jaune(s), { $vermelho } rouge(s).
godot-ui-results-cenario_s_comparados = { $n ->
    [one] { $n } scénario comparé.
   *[other] { $n } scénarios comparés.
}
godot-ui-results-otimizacao_melhor_ponto = Optimisation : meilleur point { $ponto }.
godot-ui-results-conteudo_de = contenu de { $object }
godot-ui-results-mser_5 = MSER-5 : { $cut }
godot-ui-results-media_entre_replicacoes = moyenne des réplications
godot-ui-results-unico_compartilhado = { $name } (seul { $sole } %, partagé { $shifting } %)
godot-ui-results-melhor_objetivo = meilleur objectif
godot-ui-results-pontos_avaliados = points évalués
godot-ui-results-resultados = Résultats
godot-ui-results-fechar = Fermer
godot-ui-results-fechar_os_resultados_a_cena = Fermer les résultats (la scène reste)
godot-ui-results-gargalo_no_turno = goulot en équipe
godot-ui-results-objeto = Objet
godot-ui-results-tipo = Type
godot-ui-results-entradas = Entrées
godot-ui-results-saidas = Sorties
godot-ui-results-conteudo_medio = Contenu moyen
godot-ui-results-permanencia = Séjour
godot-ui-results-ocupacao-2 = occupation { $valor }
godot-ui-results-replicacoes = Réplications
godot-ui-results-semente = graine { $seed }
godot-ui-results-tempo_de_parada = Temps d’arrêt
godot-ui-results-aquecimento = préchauffage { $warmup }
godot-ui-results-gargalo = Goulot
godot-ui-results-gargalo_no_turno-2 = Goulot en équipe
godot-ui-results-saidas_por_replicacao = Sorties par réplication
godot-ui-results-soma_das_saidas_do_modelo = somme des sorties du modèle
godot-ui-results-o_gargalo_e_o_recurso = Le goulot est la ressource la plus occupée ; pendant l’équipe ne compte que le temps où elle travaille (un poste arrêté une demi-journée semble inactif au global, mais peut être saturé pendant l’équipe).
godot-ui-results-por_objeto = Par objet
godot-ui-results-media_semiamplitude_do_ic_de = Moyenne ± demi-largeur de l’IC à 95 % entre réplications.
godot-ui-results-exportar_csv_para = Exporter en CSV vers
godot-ui-results-exportar = Exporter
godot-ui-results-grava_resumo_replicacoes_series_e = Enregistre synthèse, réplications, séries et histogrammes en CSV
godot-ui-results-grafico = Graphique
godot-ui-results-fracao_do_tempo_trabalhando_turno = Fraction du temps de travail (équipe : seul le temps pendant l’équipe ; blocage : en ajoutant le temps bloqué). La barre jaune est le goulot ; le trait est l’IC à 95 %.
godot-ui-results-fracao_do_tempo_em_cada = Fraction du temps dans chaque état, avec les couleurs de la scène 3D.
godot-ui-results-conteudo_de_no_tempo_cada = Contenu de { $object } dans le temps : chaque réplication en gris et la moyenne en jaune.
godot-ui-results-histograma_de_juntando_todas_as = Histogramme de { $object }.{ $tally }, en regroupant toutes les réplications.
godot-ui-results-o_modelo_nao_grava_ligar = {" "}Le modèle n’enregistre pas { $falta } : « Activer et relancer » l’active et simule à nouveau.
godot-ui-results-falta-serie = la série du contenu
godot-ui-results-falta-histogramas = les histogrammes
godot-ui-results-ligar_e_rodar_de_novo = Activer et relancer
godot-ui-results-liga_no_modelo_a_serie = Active dans le modèle la série du contenu (run.sample_interval, selon le temps de simulation) et les histogrammes (run.histogram_bins = 20), en une étape annulable avec Ctrl+Z, et relance les réplications
godot-ui-results-sem_serie_ligue_run_sample = pas de série : activez run.sample_interval dans le modèle
godot-ui-results-sem_histograma_ligue_run_histogram = pas d’histogramme : activez run.histogram_bins dans le modèle
godot-ui-results-sem_dados = aucune donnée
godot-ui-results-ic_95 = IC 95 %
godot-ui-results-a = { $lo } à { $hi } : { $c }
godot-ui-results-aquecimento-2 = Préchauffage
godot-ui-results-roda_as_replicacoes_desde_o = Lance les réplications depuis le temps 0, sans préchauffage, et montre leur moyenne (courbe de Welch). La coupure du MSER-5 est là où la série cesse d’avoir le début vide ; « Appliquer » écrit ce temps dans run.warmup.
godot-ui-results-serie = Série
godot-ui-results-wip_ou_objeto_medida = wip ou objet.mesure
godot-ui-results-janela = Fenêtre
godot-ui-results-campo-janela = fenêtre
godot-ui-results-campo-replicacoes = réplications
godot-ui-results-campo-orcamento = budget
godot-ui-results-analisar = Analyser
godot-ui-results-roda_e_calcula_welch_e = Lance et calcule Welch et MSER-5
godot-ui-results-aplicar = Appliquer
godot-ui-results-grava_o_corte_sugerido_em = Écrit la coupure suggérée dans run.warmup du modèle
godot-ui-results-sem_serie = pas de série
godot-ui-results-replicacoes_necessarias = Réplications nécessaires
godot-ui-results-quantas_replicacoes_faltam_para_a = Combien de réplications manquent pour que la demi-largeur de l’IC à 95 % passe sous la cible (5% est relatif à la moyenne ; 0.5 est absolu). « Lancer plus » ajoute celles qui manquent à celles déjà faites, avec de nouvelles graines.
godot-ui-results-precisao = Précision
godot-ui-results-calcular = Calculer
godot-ui-results-calcula_as_replicacoes_necessarias = Calcule les réplications nécessaires
godot-ui-results-rodar_mais = Lancer { $more } de plus
godot-ui-results-rodar_mais-2 = Lancer plus
godot-ui-results-roda_as_replicacoes_que_faltam = Lance les réplications manquantes et les ajoute aux actuelles
godot-ui-results-para_todas_as_medidas_chegarem = Pour que toutes les mesures atteignent la cible, il manque { $more ->
    [one] { $more } réplication.
   *[other] { $more } réplications.
}
godot-ui-results-todas_as_medidas_ja_estao = Toutes les mesures sont déjà dans la cible.
godot-ui-results-consistencia_lei_de_little = Cohérence : loi de Little
godot-ui-results-confere_l_w_em_cada = Vérifie L = λ·W dans chaque objet et dans le système. Vert : ça tient ; jaune : la différence est statistiquement non nulle, mais petite (jusqu’à 2 %) ; rouge : ça ne tient pas, ce qui indique souvent un préchauffage court ou des éléments bloqués à la fin.
godot-ui-results-conferir = Vérifier
godot-ui-results-roda_as_replicacoes_e_confere = Lance les réplications et vérifie la loi de Little
godot-ui-results-gargalo_movel = Goulot mobile
godot-ui-results-em_cada_instante_o_gargalo = À chaque instant, le goulot est la ressource dans la plus longue période active ; quand deux périodes se chevauchent, le goulot est partagé. Montre la fraction du temps où chaque ressource est goulot seul et partagé.
godot-ui-results-roda_as_replicacoes_com_o = Lance les réplications avec la trace et calcule le goulot mobile
godot-ui-results-medida = Mesure
godot-ui-results-atual = Actuel
godot-ui-results-alvo = Cible (±)
godot-ui-results-faltam = Manquent
godot-ui-results-diferenca = Différence
godot-ui-results-semaforo = Feu
godot-ui-results-um_cenario_por_bloco_nome = Un scénario par bloc : [nom] et, en dessous, les changements (objet.champ = valeur en RON, ou run.champ). Le modèle tel quel vient d’abord, comme base. Tous utilisent les mêmes graines (nombres aléatoires communs), donc la différence avec la base est appariée ; en jaune, les différences significatives.
godot-ui-results-medidas_virgulas_vazio_todas = Mesures (virgules ; vide : toutes)
godot-ui-results-comparar = Comparer
godot-ui-results-roda_todos_os_cenarios_e = Lance tous les scénarios et compare
godot-ui-results-texto = Δ { $valor }
godot-ui-results-procura_os_valores_das_variaveis = Cherche les valeurs des variables qui minimisent (ou maximisent) la moyenne d’une métrique. Une variable par ligne : clé minimum maximum [pas] (entiers) ou avec un point décimal (continue, cinq valeurs dans la grille). Les points non significativement différents du meilleur (test apparié à 95 %) sont marqués.
godot-ui-results-objetivo = Objectif
godot-ui-results-maximizar = maximiser
godot-ui-results-metodo = Méthode
godot-ui-results-replicacoes_por_ponto = Réplications par point
godot-ui-results-orcamento_pontos = Budget (points)
godot-ui-results-otimizar = Optimiser
godot-ui-results-roda_a_busca = Lance la recherche
godot-ui-results-melhor_ponto_ponto_s_simulados = Meilleur point : { $ponto } ({ $simulated ->
    [one] { $simulated } point simulé
   *[other] { $simulated } points simulés
})
godot-ui-results-sem_historico = pas d’historique
godot-ui-results-ponto = Point
godot-ui-results-nenhum = aucun
godot-ui-results-nenhum-viavel = aucun réalisable
godot-ui-results-util-trabalho = travail
godot-ui-results-util-turno = équipe
godot-ui-results-util-bloqueio = blocage
godot-ui-results-metodo-grade = grille
godot-ui-results-metodo-recozimento = recuit
godot-ui-results-metodo-genetico = génétique
godot-ui-results-replicacao_oes_tempo_de_parada = 
    { $model } : { $replications ->
        [one] { $replications } réplication
       *[other] { $replications } réplications
    }, temps d’arrêt { $stop_time }, préchauffage { $warmup }, graine { $seed }.
    Goulot (global) : { $b }
    Goulot (pendant l’équipe) : { $bs }
godot-ui-results-replicacao_oes = { $model } · { $reports ->
    [one] { $reports } réplication
   *[other] { $reports } réplications
}

## Tipos de bloco nos relatórios (results.rs)

godot-tipo-source = arrivée
godot-tipo-queue = file
godot-tipo-processor = processus
godot-tipo-combiner = assemblage
godot-tipo-separator = désassemblage
godot-tipo-sink = sortie
godot-tipo-transporter = transporteur
godot-tipo-dispatcher = répartiteur
godot-tipo-conveyor = convoyeur
godot-tipo-rack = rayonnage à palettes
godot-tipo-span = mesure de temps
godot-tipo-flow = flux
godot-tipo-fluid = fluides

## Propriedades dos objetos (schema.rs)

godot-campo-movimentacao = Déplacement
godot-campo-folga-dos-obstaculos = Marge des obstacles
godot-campo-ponto-de-acesso-proprio = Point d’accès propre
godot-campo-x-no-objeto = X sur l’objet
godot-campo-y-no-objeto = Y sur l’objet
godot-campo-dimensoes = Dimensions
godot-campo-dimensoes-proprias = Dimensions propres
godot-campo-capacidade-pela-area = Capacité selon la surface
godot-campo-area-por-item = Surface par élément
godot-campo-cabem = Capacité d’accueil
godot-campo-tempo-entre-chegadas = Temps entre arrivées
godot-campo-primeira-chegada-em-t-0 = Première arrivée à t = 0
godot-campo-maximo-de-chegadas = Maximum d’arrivées
godot-campo-se-a-saida-estiver-bloqueada = Si la sortie est bloquée
godot-opcao-esperar = Attendre
godot-opcao-descartar-perda = Écarter (perte)
godot-campo-rotulos = Étiquettes
godot-campo-chegadas-por-periodo = Arrivées par période
godot-campo-tabela-de-chegadas = Tableau d’arrivées
godot-campo-capacidade-limitada = Capacité limitée
godot-campo-politica-de-acesso = Politique d’accès
godot-opcao-fifo-primeiro-a-entrar-primeiro = FIFO (premier entré, premier sorti)
godot-opcao-lifo-ultimo-a-entrar-primeiro = LIFO (dernier entré, premier sorti)
godot-opcao-prioridade-maior-rotulo-primeiro = Priorité : plus grande étiquette d’abord
godot-opcao-prioridade-menor-rotulo-primeiro-prazo = Priorité : plus petite étiquette d’abord (échéance)
godot-campo-rotulo-da-prioridade = Étiquette de priorité
godot-campo-lote = Lot
godot-campo-itens-por-lote = Éléments par lot
godot-campo-espera-maxima = Attente maximale
godot-campo-fecha-a-entrada-ate-o = Ferme l’entrée jusqu’à la sortie du lot
godot-campo-capacidade = Capacité
godot-campo-preparacao-setup = Réglage (setup)
godot-campo-troca-de-produto = Changement de produit
godot-campo-tempo-de-processamento = Temps de traitement
godot-campo-tempo-por-receita = Temps par recette
godot-campo-operadores = Opérateurs
godot-campo-quem-opera = Qui opère
godot-campo-quantos-por-peca = Combien par pièce
godot-campo-ganho-da-equipe = Gain de l’équipe
godot-campo-durante-a-preparacao = Pendant le réglage
godot-campo-durante-o-processamento = Pendant le traitement
godot-campo-outras-equipes-ao-mesmo-tempo = Autres équipes en même temps
godot-campo-componentes-por-entrada = Composants par entrée
godot-campo-modo = Mode
godot-opcao-embalar-so-o-conteiner-sai = Emballer (seul le conteneur sort)
godot-opcao-juntar-componentes-destruidos = Joindre (composants détruits)
godot-opcao-lote-todos-saem = Lot (tous sortent)
godot-opcao-desembalar = Déballer
godot-opcao-dividir-em-copias = Diviser en copies
godot-opcao-dividir-pelo-rotulo = Diviser selon l’étiquette
godot-campo-copias = Copies
godot-campo-sem-parametros = Sans paramètres
godot-campo-velocidade-maxima = Vitesse maximale
godot-campo-tempo-de-carga = Temps de chargement
godot-campo-tempo-de-descarga = Temps de déchargement
godot-campo-voltar-a-base-quando-ocioso = Retourner à la base quand inactif
godot-campo-agv-controle-de-trafego-na = AGV (contrôle du trafic sur le réseau)
godot-campo-trajeto-desenhado = Trajet dessiné
godot-campo-editar-nos = Modifier les nœuds
godot-campo-comprimento = Longueur
godot-campo-velocidade = Vitesse
godot-campo-comprimento-por-item = Longueur par élément
godot-campo-acumula-itens-quando-a-saida = Accumule les éléments quand la sortie bloque
godot-campo-colunas = Colonnes
godot-campo-niveis = Niveaux
godot-campo-permanencia-minima = Séjour minimal
godot-campo-regra-de-ocupacao = Règle d’occupation
godot-opcao-primeira-livre = Première libre
godot-opcao-aleatoria = Aléatoire
godot-opcao-curva-abc-rotulo-classe = Courbe ABC (étiquette de classe)
godot-campo-fracao-das-posicoes-da-classe = Fraction des emplacements de la classe A
godot-campo-fracao-das-posicoes-da-classe-2 = Fraction des emplacements de la classe B
godot-campo-largura-da-coluna = Largeur de la colonne
godot-campo-altura-do-nivel = Hauteur du niveau
godot-campo-equipe = Équipe
godot-campo-regra-de-escolha = Règle de choix
godot-opcao-primeiro-livre = Premier libre
godot-opcao-rodizio = Rotation
godot-opcao-mais-proximo = Le plus proche
godot-campo-ordem-dos-movimentos = Ordre des mouvements
godot-opcao-segura-sobe-anda-desce = Sécurisé (monte, avance, descend)
godot-opcao-um-eixo-de-cada-vez = Un axe à la fois
godot-opcao-os-tres-juntos = Les trois ensemble
godot-campo-altura-de-passagem-do-gancho = Hauteur de passage du crochet
godot-campo-rotulo-do-peso-do-item = Étiquette du poids de l’élément
godot-campo-tempo-de-engate = Temps d’accrochage
godot-campo-tempo-de-desengate = Temps de décrochage
godot-campo-voltar-a-posicao-propria-quando = Retourner à sa position propre quand inactif
godot-campo-controle = Commande
godot-opcao-cabine-operador-nao-modelado = Cabine (opérateur non modélisé)
godot-opcao-botoeira-a-pessoa-anda-com = Télécommande filaire (la personne marche avec la charge)
godot-opcao-radio-a-pessoa-encontra-a = Radio (la personne rejoint le pont à la charge)
godot-campo-equipe-do-operador = Équipe de l’opérateur
godot-campo-meia-largura-da-area-sob = Demi-largeur de la zone sous la charge
godot-campo-afastar-as-pessoas-do-caminho = Écarter les personnes du chemin de la charge
godot-campo-folga-entre-pontes = Marge entre ponts
godot-campo-caminho = Chemin
godot-campo-regra-de-saida = Règle de sortie
godot-campo-pesos = Poids
godot-campo-rotulo-com-o-numero-da = Étiquette avec le numéro de la porte
godot-campo-script = Script
godot-campo-nenhum-passagem-direta = Aucun (passage direct)
godot-campo-transporte-de-saida = Transport de sortie
godot-campo-transferencia-pela-distancia = Transfert selon la distance
godot-campo-atraso = Retard
godot-opcao-reta = droite
godot-opcao-pela-rede = par le réseau
godot-campo-script-ao-entrar-um-item = Script à l’entrée d’un élément
godot-campo-quebras = Pannes
godot-campo-tempo-entre-falhas-mtbf = Temps entre défaillances (MTBF)
godot-campo-tempo-de-reparo-mttr = Temps de réparation (MTTR)
godot-campo-turnos = Équipes
godot-campo-origem = Origine
godot-opcao-personalizado = Personnalisé
godot-opcao-calendario-do-modelo = Calendrier du modèle
godot-campo-periodo = Période
godot-campo-pausas = Pauses
godot-campo-no-fim-do-turno = À la fin de l’équipe
godot-campo-calendario = Calendrier
godot-campo-ativo = Actif
godot-campo-cor = Couleur
godot-campo-escala = Échelle
godot-campo-modelo-da-peca = Modèle de la pièce
godot-campo-giro-do-desenho = Rotation du dessin
godot-campo-restaurar-padrao = Restaurer par défaut
godot-campo-nome = Nom
godot-campo-tipo = Type
godot-campo-rotacao = Rotation
godot-campo-sexo = Sexe
godot-opcao-nao-informado = Non renseigné
godot-opcao-feminino = Féminin
godot-opcao-masculino = Masculin
godot-campo-altura = Hauteur
godot-ajuda-a-altura-do-boneco-no = La hauteur du personnage en 3D. Désactivée, elle est tirée au sort selon le nom.
godot-campo-massa = Masse
godot-ajuda-com-a-altura-da-a = Avec la hauteur, donne la largeur du personnage en 3D selon l’indice de masse corporelle.
godot-campo-idade = Âge
godot-campo-experiencia-na-funcao = Expérience dans la fonction
godot-ajuda-so-registro-a-simulacao-nao = Simple enregistrement : la simulation ne le lit pas.
godot-campo-carga-maxima-medida = Charge maximale mesurée
godot-ajuda-uma-carga-acima-dela-da = Une charge supérieure déclenche un avertissement dans le modèle.
godot-campo-forca-maxima-medida = Force maximale mesurée
godot-ajuda-empurrar-ou-puxar-so-registro = Pousser ou tirer. Simple enregistrement.
godot-campo-tipo-de-espera = Type de file
godot-opcao-fila-simples-sem-lugares = File simple (sans places)
godot-opcao-mesa = Table
godot-opcao-palete = Palette
godot-opcao-estante = Étagère
godot-opcao-chao-area-demarcada = Sol (zone délimitée)
godot-campo-camadas = Couches
godot-campo-passo-entre-pecas = Pas entre pièces
godot-campo-lugares = Places
godot-campo-opera-as-maquinas = Opère les machines
godot-campo-leva-os-itens-de = Transporte les éléments de
godot-campo-nome-do-modelo = Nom du modèle
godot-campo-descricao = Description
godot-campo-tempo-de-simulacao = Temps de simulation
godot-campo-aquecimento = Préchauffage
godot-campo-semente = Graine
godot-campo-replicacoes = Réplications
godot-campo-movimentacao-padrao = Déplacement par défaut
godot-campo-perfil-da-marcacao-do-piso = Profil du marquage au sol
godot-opcao-brasil-faixa-amarela = Brésil (bande jaune)
godot-opcao-branco-redacao-original-da-nr = Blanc (rédaction originale de la NR-26)
godot-campo-largura-da-faixa = Largeur de la bande
godot-campo-parada-no-cruzamento = Arrêt au croisement
godot-campo-cor-do-chao = Couleur du sol
godot-ajuda-como-o-recurso-anda-o = Comment la ressource se déplace : le défaut du modèle (Simulation), en ligne droite comme dans l’ancien format, libre sur le sol en évitant les obstacles, par le réseau de chemins ou seulement par les couloirs de sa classe.
godot-ajuda-onde-os-recursos-moveis-param = Où les ressources mobiles s’arrêtent pour desservir cet objet, en mètres à partir de son centre et en tournant avec lui. Désactivé, c’est le côté -y. Se déplace aussi par glisser dans la scène, avec « Afficher obstacles et marges » activé.
godot-ajuda-comprimento-largura-e-altura-do = Longueur, largeur et hauteur de l’objet en mètres, dans son sens. La surface au sol modifie le point d’accès et, avec lui, les distances et les temps des opérateurs et véhicules ; dans une file avec surface par élément, aussi la capacité. La 3D étire le modèle à ces mesures. Désactivé, la surface par défaut du type s’applique.
godot-ajuda-a-capacidade-da-espera-sai = La capacité de la file découle de sa surface au sol divisée par la surface qu’occupe chaque élément (éléments entiers seulement). Avec une capacité donnée aussi, la plus petite prévaut.
godot-ajuda-intervalo-entre-duas-chegadas-seguidas = Intervalle entre deux arrivées consécutives.
godot-ajuda-valor-sorteado-para-cada-item = Valeur tirée au sort pour chaque élément, par nom : {"{"}"tipo": Discrete([(1.0, 3.0), (2.0, 1.0)]){"}"}.
godot-ajuda-ciclo-e-periodos-com-o = Cycle et périodes avec leur propre intervalle, taux ou fermées : (cycle: 1440.0, periods: [(start: 0.0, end: 480.0, arrivals: Rate(0.5))]).
godot-ajuda-horario-instante-quantidade-tipo-e = Horaire (instant, quantité, type et étiquettes de chaque ligne) ou séquence de lignes : (mode: Times, repeat: Cycle(1440.0), rows: [(at: 480.0, count: 12)]).
godot-ajuda-desligada-a-espera-nao-tem = Désactivée, la file n’a pas de limite.
godot-ajuda-qual-item-sai-primeiro-pela = Quel élément sort en premier. Avec la priorité, sort d’abord l’élément à la plus grande (ou plus petite) valeur d’étiquette ; les égalités et les éléments sans étiquette sortent par ordre d’arrivée.
godot-ajuda-segura-os-itens-e-libera = Retient les éléments et les libère par groupes.
godot-ajuda-libera-um-lote-incompleto-depois = Libère un lot incomplet après ce temps (0 = attend toujours).
godot-ajuda-preparacao-so-quando-o-produto = Réglage seulement quand le produit change : l’étiquette du produit et, si souhaité, le temps de chaque changement (de, vers) ; sans ligne dans la matrice, le réglage ci-dessus s’applique. Exemple : (label: "produto", matrix: [(from: 1, to: 2, time: Const(30.0))]). None : réglage à chaque pièce.
godot-ajuda-tempo-de-processamento-que-depende = Temps de traitement qui dépend de l’élément : l’étiquette de la recette et le temps de chaque valeur ; les éléments sans recette dans le tableau utilisent le temps ci-dessus. Exemple : (label: "produto", times: [(value: 1, time: Const(90.0)), (value: 2, time: Const(120.0))]). None : le même temps pour tous.
godot-ajuda-quem-opera-esta-maquina-marque = Qui opère cette machine : cochez et choisissez ci-dessous, ou faites glisser depuis le nœud orange de l’opérateur (ou de l’équipe) jusqu’à la machine. Le processus attend qu’un d’eux soit libre.
godot-ajuda-operadores-que-a-peca-precisa = Opérateurs dont la pièce a besoin en même temps : elle ne démarre que quand tous arrivent, et ils restent avec elle jusqu’à la fin. Avec plus d’opérateurs dans l’équipe que capacité × ce nombre, les surnuméraires restent à l’arrêt.
godot-ajuda-quantas-vezes-mais-rapido-a = Combien de fois plus vite l’équipe réalise l’étape : le temps tiré est divisé par ce nombre. 1 : aucun gain (le temps est le même avec un ou plusieurs).
godot-ajuda-equipes-que-precisam-estar-juntas = Équipes qui doivent être présentes avec les opérateurs (un médecin et un infirmier dans la même salle). Exemple : [OperatorUse(team: "enfermagem")]. L’étape ne démarre que quand tous arrivent.
godot-ajuda-a-entrada-1-traz-o = L’entrée 1 apporte le conteneur ; l’entrée k + 1 apporte le k-ième nombre d’éléments. Exemple : [2, 1].
godot-ajuda-desligada-a-velocidade-muda-na = Désactivée, la vitesse change immédiatement.
godot-ajuda-abre-a-ferramenta-de-desenho = Ouvre l’outil de dessin avec ce convoyeur : faites glisser les nœuds, modifiez la hauteur et le rayon.
godot-ajuda-transportadores-que-o-despachante-comanda = Transporteurs que le répartiteur commande.
godot-ajuda-segura-o-gancho-sobe-a = Sécurisé : le crochet monte à la hauteur de passage, le pont et le chariot avancent ensemble et le crochet descend.
godot-ajuda-rotulo-com-o-peso-de = Étiquette avec le poids de chaque élément, en kg : les éléments au-dessus de la capacité génèrent un avertissement.
godot-ajuda-na-botoeira-a-ponte-nao = Avec la télécommande filaire, le pont n’avance pas plus vite que la personne ; en radio, il garde sa propre vitesse et attend l’arrivée de la personne.
godot-ajuda-meio-lado-do-quadrado-sob = Demi-côté du carré sous le crochet où personne ne doit se tenir avec la charge suspendue (analyse d’exposition).
godot-ajuda-antes-de-andar-carregada-a = Avant d’avancer en charge, le pont attend que les personnes passent la bande de la charge ; et celles qui allaient y entrer s’arrêtent au bord jusqu’au passage de la charge.
godot-ajuda-distancia-mantida-entre-os-ganchos = Distance maintenue entre les crochets de deux ponts sur le même chemin : l’un ne dépasse pas l’autre.
godot-ajuda-para-qual-ligacao-vai-cada = Vers quelle liaison va chaque élément, dans l’ordre des liaisons de sortie.
godot-ajuda-peso-de-cada-saida-na = Poids de chaque sortie, dans l’ordre des liaisons. Exemple : [3.0, 1.0].
godot-ajuda-devolve-o-numero-da-porta = Renvoie le numéro de la porte (1, 2, ...).
godot-ajuda-transportador-ou-despachante-que-leva = Transporteur ou répartiteur qui amène les éléments au bloc suivant.
godot-ajuda-os-itens-levam-atraso-distancia = Les éléments mettent retard + distance / vitesse pour arriver au bloc suivant par les liaisons directes (sans transporteur).
godot-ajuda-roda-para-cada-item-que = S’exécute pour chaque élément qui entre, avant toute autre règle.
godot-ajuda-falhas-aleatorias-com-o-tempo = Défaillances aléatoires, avec le temps compté à l’horloge.
godot-ajuda-janelas-fora-do-turno-repetidas = Fenêtres hors équipe, répétées à chaque période.
godot-ajuda-inicio-e-duracao-de-cada = Début et durée de chaque pause dans la période, dans l’ordre.
godot-ajuda-corrige-para-onde-o-modelo = Corrige la direction du modèle 3D ; la ressource se tourne vers où elle avance.
godot-ajuda-volta-a-aparencia-padrao-do = Revient à l’apparence par défaut du type d’objet.
godot-ajuda-giro-em-graus-anti-horario = Rotation en degrés, sens antihoraire vu de dessus (R tourne de 90°, Ctrl+R de 15°).
godot-ajuda-escolhe-o-corpo-no-3d = Choisit le corps en 3D (non renseigné, le corps est déduit du nom). Aucun schéma de fatigue n’en dépend ; avec Féminin, une charge supérieure à 20 kg déclenche l’avertissement de la CLT, art. 390.
godot-ajuda-numa-espera-real-cada-peca = Dans une vraie file, chaque pièce occupe une place (la disposition du type, recalculée par les Dimensions) : seule sort la pièce sans rien dessus, la plus ancienne d’abord, et celui qui retire va jusqu’à l’avant de la pièce. Cela change le résultat. File simple : comme avant.
godot-ajuda-quantas-pecas-empilhadas-em-cada = Combien de pièces empilées à chaque place (sur l’étagère, à chaque niveau).
godot-ajuda-distancia-entre-os-centros-de = Distance entre les centres de deux pièces voisines.
godot-ajuda-arraste-do-no-laranja-ate = Faites glisser depuis le nœud orange jusqu’à la machine pour relier ; cliquez sur la ligne pointillée puis sur × pour retirer.
godot-ajuda-arraste-do-no-laranja-ate-2 = Faites glisser depuis le nœud orange jusqu’à l’objet (une file, une arrivée, ou avec Shift une machine) pour relier.
godot-ajuda-texto-livre-sobre-o-modelo = Texte libre sur le modèle, enregistré dans le fichier.
godot-ajuda-as-estatisticas-so-contam-depois = Les statistiques ne comptent qu’après ce temps.
godot-ajuda-como-andam-os-recursos-moveis = Comment se déplacent les ressources mobiles qui n’ont pas choisi de mode propre. Vide, Libre s’applique (plus court chemin au sol, en évitant les obstacles).
godot-ajuda-cores-das-faixas-e-setas = Couleurs des bandes et flèches des couloirs. Le programme ne certifie pas la conformité : vérifiez le texte en vigueur des normes.
godot-ajuda-a-pratica-usual-vai-de = La pratique courante va de 0,05 à 0,15 m.
godot-ajuda-o-veiculo-para-esse-tempo = Le véhicule s’arrête ce temps avant un croisement avec des piétons.
godot-ajuda-a-cor-do-piso-na = La couleur du sol dans la scène 3D, enregistrée dans le modèle ; la grille change de ton pour rester visible. Désactivée, le gris habituel.
godot-opcao-por-rotulo = Par étiquette
godot-opcao-padrao-do-modelo = Défaut du modèle
godot-opcao-legado-linha-reta-ou-rede = Héritage (ligne droite ou réseau)
godot-opcao-livre-menor-caminho-no-piso = Libre (plus court chemin au sol)
godot-opcao-pela-rede-de-caminhos = Par le réseau de chemins
godot-opcao-pelos-corredores = Par les couloirs


## Propriedades dos objetos: seções, tipos, famílias, parâmetros, turnos e visual (schema.rs)

godot-secao-objeto = Objet
godot-secao-posicao = Position
godot-secao-parametros = Paramètres
godot-secao-saida = Sortie
godot-secao-quebras-e-turnos = Pannes et équipes
godot-secao-aparencia = Apparence
godot-secao-simulacao = Simulation
godot-secao-movimentacao = Déplacement
godot-secao-piso-e-corredores = Sol et couloirs
godot-secao-modelo = Modèle
godot-secao-pessoa = Personne
godot-objeto-chegada = Arrivée
godot-objeto-espera = File
godot-objeto-processo = Processus
godot-objeto-montagem = Assemblage
godot-objeto-desmontagem = Désassemblage
godot-objeto-saida = Sortie
godot-objeto-transportador = Transporteur
godot-objeto-ponte-rolante = Pont roulant
godot-objeto-despachante = Répartiteur
godot-objeto-esteira = Convoyeur
godot-objeto-porta-paletes = Rayonnage à palettes
godot-dist-constante = Constante
godot-dist-uniforme = Uniforme
godot-dist-exponencial = Exponentielle
godot-dist-triangular = Triangulaire
godot-dist-normal = Normale
godot-dist-lognormal = Log-normale
godot-dist-gama = Gamma
godot-dist-erlang = Erlang
godot-dist-weibull = Weibull
godot-dist-beta = Bêta
godot-dist-log-logistica = Log-logistique
godot-dist-pearson-v = Pearson V
godot-dist-pearson-vi = Pearson VI
godot-dist-discreta = Discrète
godot-dist-empirica = Empirique
godot-dist-poisson = Poisson
godot-dist-geometrica = Géométrique
godot-dist-binomial = Binomiale
godot-dist-binomial-negativa = Binomiale négative
godot-dist-deslocada = Décalée
godot-param-valor = valeur
godot-param-minimo = minimum
godot-param-maximo = maximum
godot-param-media = moyenne
godot-param-moda = mode
godot-param-desvio-padrao = écart type
godot-param-forma = forme
godot-param-escala = échelle
godot-param-fases = phases (k)
godot-param-forma-1 = forme 1
godot-param-forma-2 = forme 2
godot-param-probabilidade = probabilité (p)
godot-param-tentativas = essais (n)
godot-param-sucessos = succès (r)
godot-param-alfa = alpha
godot-param-beta = bêta
godot-param-deslocamento = décalage
godot-dim-comprimento-do-caminho = Longueur du chemin
godot-dim-vao = Portée
godot-dim-altura-do-caminho = Hauteur du chemin
godot-dim-comprimento = Longueur
godot-dim-largura = Largeur
godot-dim-altura = Hauteur
godot-turno-pausar = Mettre en pause
godot-turno-terminar = Terminer le travail
godot-turno-do-calendario = (celui du calendrier)
godot-ajuda-folga-papel = Distance minimale de tout obstacle. Désactivée, celle du rôle de la ressource s’applique ({ $m } m : demi-largeur plus 0,3 m).
godot-dim-esteira = { $l } × { $w } m : la longueur est celle du convoyeur (Paramètres)
godot-dim-porta-paletes = { $l } × { $w } × { $h } m : selon les portées et niveaux (Paramètres)
godot-dim-padrao = { $l } × { $w } m (par défaut)
godot-campo-sem-limite = sans limite
godot-campo-n-itens = { $n ->
    [one] { $n } élément
   *[other] { $n } éléments
}
godot-campo-nao-existe = { $nome } (n’existe pas)
godot-campo-n-posicoes = { $n ->
    [one] { $n } emplacement
   *[other] { $n } emplacements
}
godot-campo-movimento = Mouvement { $o_que }.
godot-campo-n-lugares = { $n ->
    [one] { $n } place
   *[other] { $n } places
}
godot-campo-divide-caminho = Partage le chemin avec { $com } (marge de { $m } m entre les crochets)
godot-campo-pesos-e-saidas = Il y a { $pesos ->
    [one] { $pesos } poids
   *[other] { $pesos } poids
}, mais l’objet a { $n ->
    [one] { $n } sortie
   *[other] { $n } sorties
}. { $ajuda }
godot-turno-sem-calendarios = Le modèle n’a pas de calendriers : créez-en un dans « calendars » pour l’utiliser ici.
godot-turno-calendario-ajuda = Un calendrier est le même tableau d’équipes utilisé par plusieurs objets.
godot-visual-padrao-do-tipo = (défaut du type)
godot-visual-gerado = UTSim (généré par utsim-assets)
godot-visual-grupo-biblioteca = Bibliothèque UTSim
godot-visual-grupo-terceiros = Tiers (assets/terceiros)
godot-visual-grupo-elementos = Éléments 3D (assets/modelos3d)
godot-visual-caixa-padrao = boîte (par défaut)
godot-visual-modelo-da-peca-ajuda = Élément 3D avec lequel les pièces créées ici sont dessinées ; la boîte est le défaut.
godot-visual-no-tamanho-da-peca = { $arquivo } ({ $x } x { $y } x { $z } m), dessiné à la taille de la pièce
godot-eixo-z = Z (hauteur)


## ui_props.rs

godot-ui-props-modelo_nada_selecionado = Modèle (rien de sélectionné)
godot-ui-props-propriedades = Propriétés : { $name }
godot-ui-props-problemas = Problèmes
godot-ui-props-avisos = Avertissements
godot-ui-props-mostrar_motorista = Afficher le conducteur
godot-ui-props-desenha_o_operador_sentado_e = Dessine l’opérateur assis et animé sur le chariot élévateur. Apparence seulement : ne modifie pas la simulation et n’est pas enregistré dans le modèle.
godot-ui-props-clique_para_destacar_na_cena = Cliquez pour mettre en évidence dans la scène ce que cet avertissement cite
godot-ui-props-abre-a-secao = Ouvre la section { $title }
godot-ui-props-fecha-a-secao = Ferme la section { $title }
godot-ui-props-liga_ou_desliga = Active ou désactive { $campo }
godot-ui-props-aplicar = Appliquer
godot-ui-props-aplicar_script = Appliquer le script
godot-ui-props-nenhum_transportador_no_modelo = Aucun transporteur dans le modèle.
godot-ui-props-este_campo_vem_de_dados = Ce champ provient de données réelles : { $label }
godot-ui-props-distribuicao = Distribution
godot-ui-props-dados_reais = Données réelles
godot-ui-props-fonte = Source
godot-ui-props-de_onde_vem_a_distribuicao = D’où vient la distribution de ce champ : paramètres renseignés ou valeurs réelles, ajustées automatiquement
godot-ui-props-media = moyenne { $mean }
godot-ui-props-arquivo_csv = Fichier CSV
godot-ui-props-valores_colados = Valeurs collées
godot-ui-props-valores_reais = Valeurs réelles…
godot-ui-props-abre_a_janela_grande_para = Ouvre la grande fenêtre pour saisir, coller ou importer les valeurs et comparer toutes les distributions en une fois
godot-ui-props-origem = Origine
godot-ui-props-arquivo = Fichier
godot-ui-props-arquivo_csv_ou_txt = fichier .csv ou .txt
godot-ui-props-escolher = Choisir...
godot-ui-props-escolhe_o_arquivo_de_dados = Choisit le fichier de données (CSV ou TXT)
godot-ui-props-automatica = (automatique)
godot-ui-props-coluna = Colonne
godot-ui-props-coluna_do_arquivo_com_os = Colonne du fichier avec les valeurs
godot-ui-props-um_valor_por_linha_ou = une valeur par ligne (ou séparées par ; ou espace)
godot-ui-props-sao_instantes_usar_os_intervalos = Ce sont des instants (utiliser les intervalles entre eux)
godot-ui-props-marque_quando_os_valores_sao = Cochez quand les valeurs sont des instants croissants (comme des heures d’arrivée) : l’ajustement utilise les différences entre eux. Décoché, les valeurs sont des durées.
godot-ui-props-melhor_automatica = Meilleure automatique
godot-ui-props-empirica_os_proprios_valores = Empirique (les valeurs elles-mêmes)
godot-ui-props-so = Seulement { $f }
godot-ui-props-escolha = Choix
godot-ui-props-criterio = Critère
godot-ui-props-o_que_ordena_as_familias = Ce qui ordonne les familles acceptées par les tests
godot-ui-props-reajustar_com_estas_opcoes = Réajuster avec ces options
godot-ui-props-ajusta_a_distribuicao_aos_dados = Ajuste la distribution aux données (en arrière-plan) et l’utilise dans ce champ
godot-ui-props-nao_foi_possivel_ler_os = Impossible de lire les données : { $err }
godot-ui-props-os_dados_mudaram_desde_o = Les données ont changé depuis le dernier ajustement.
godot-ui-props-reajustar_aos_dados_atuais = Réajuster aux données actuelles
godot-ui-props-o_campo_foi_reajustado_salve = { $c }. Le champ a été réajusté ; enregistrez pour le conserver.
godot-ui-props-n = { $family } · n = { $n } · { $label }
godot-ui-props-analisando_os_dados = Analyse des données...
godot-ui-props-candidatas = Candidates ({ $criterion })
godot-ui-props-aic_bic_usa_esta_familia =
    { $params }
    AIC { $aic }   BIC { $bic }
    Utilise cette famille dans ce champ
godot-ui-props-usar_a_empirica_os_proprios = Utiliser l’empirique (les valeurs elles-mêmes)
godot-ui-props-relatorio = Rapport
godot-ui-props-grava_o_relatorio_completo_do = Enregistre le rapport complet de l’ajustement (HTML) dans un dossier et l’ouvre dans le navigateur
godot-ui-props-histograma_dos_dados_e_a = Histogramme des données et courbe de la distribution choisie
godot-ui-props-buscar_ativo = Chercher un actif
godot-ui-props-miniatura_do_objeto_com_o = Miniature de l’objet avec l’actif choisi (ou celui sous la souris dans la liste)
godot-ui-props-ligacoes_de_saida_ordem_das = Liaisons de sortie (ordre des portes)
godot-ui-props-nenhuma_use_a_ferramenta_ligar = Aucune. Utilisez l’outil Relier (L).
godot-ui-props-subir = Monter
godot-ui-props-sobe_esta_saida_na_ordem = Monte cette sortie dans l’ordre des portes
godot-ui-props-descer = Descendre
godot-ui-props-desce_esta_saida_na_ordem = Descend cette sortie dans l’ordre des portes
godot-ui-props-remover = Retirer
godot-ui-props-remove_esta_ligacao = Retire cette liaison
godot-ui-props-entradas = Entrées : { $inputs }

## Tour 13 : fenêtres d'UTSim 3D

## dados.rs

godot-dados-criterio_desconhecido = critère inconnu : { $other }
godot-dados-use_um_arquivo_ou_valores = utilisez un fichier ou des valeurs collées, pas les deux
godot-dados-escolha_um_arquivo_de_dados = choisissez un fichier de données ou collez les valeurs
godot-dados-registro_reaproveitado = enregistrement réutilisé
godot-dados-dados_mudaram = les données ont changé
godot-dados-opcoes_mudaram = les options ont changé
godot-dados-dados_ilegiveis = données illisibles
godot-dados-os_dados_mudaram_desde_o = les données ont changé depuis le dernier ajustement (c'était { $old_family }, hash { $old_hash })
godot-dados-nao_foi_possivel_ler = impossible de lire { $path } : { $e }
godot-dados-nenhum_campo_do_modelo_usa = aucun champ du modèle n'utilise de données réelles
godot-dados-nao_foi_possivel_gravar_em = impossible d'écrire dans { $dir } : { $e }

## document.rs

godot-document-transportadores_e_despachantes_nao_usam = Les transporteurs et les répartiteurs n'utilisent pas de liaisons de flux : faites glisser depuis leur nœud orange jusqu'à la machine qu'ils servent (ou jusqu'à l'objet dont ils transportent les articles).
godot-document-objeto_nao_existe = l'objet '{ $n }' n'existe pas
godot-document-arraste_de_um_objeto_ate = faites glisser d'un objet vers un autre
godot-document-saiu_da_equipe_de = { $to } a quitté l'équipe de { $from }.
godot-document-entrou_na_equipe_de = { $to } a rejoint l'équipe de { $from }.
godot-document-nao_pede_mais_operadores_de = { $from } ne demande plus d'opérateurs à { $to }.
godot-document-pede_operadores_de = { $from } demande des opérateurs à { $to }.
godot-document-nao_pede_mais_transporte_a = { $from } ne demande plus de transport à { $to }.
godot-document-pede_transporte_a = { $from } demande du transport à { $to }.
godot-document-arraste_do_despachante_a_um = faites glisser du répartiteur vers un transporteur, ou d'un objet vers le transporteur ou le répartiteur qui le sert
godot-document-a_serie_e_os_histogramas = la série et les histogrammes sont déjà liés dans le modèle
godot-document-desenhe_clicando_os_nos_no = dessinez en cliquant les nœuds au sol (Enter termine)
godot-document-tipo_desconhecido = type inconnu : { $id }
godot-document-nada_selecionado = rien de sélectionné
godot-document-nao_ha_para_onde_mover = il n'y a nulle part où déplacer cette liaison
godot-document-nenhum_arrasto_comecou = aucun glissement n'a commencé
godot-document-objeto_nao_existe-2 = l'objet '{ $name }' n'existe pas
godot-document-e_um_recurso_movel_ele = { $name } est une ressource mobile : elle s'oriente dans le sens de son déplacement
godot-document-recursos_moveis_nao_giram_eles = les ressources mobiles ne pivotent pas : elles s'orientent dans le sens de leur déplacement
godot-document-a_area_de_transferencia_esta = le presse-papiers est vide (Ctrl+C copie)
godot-document-alinhamento_desconhecido = alignement inconnu : { $how }
godot-document-selecione_dois_objetos_ou_mais = sélectionnez au moins deux objets pour aligner
godot-document-eixo_desconhecido = axe inconnu : { $axis }
godot-document-selecione_tres_objetos_ou_mais = sélectionnez au moins trois objets pour répartir
godot-document-nao_tem_distribuicao = '{ $field }' n'a pas de distribution
godot-document-modelo = Modèle
godot-document-nome_em_vez_de = nom : { $name } au lieu de { $name2 }
godot-document-descricao_em_vez_de = description : { $description } au lieu de { $description2 }
godot-document-run_em_vez_de = run : { $run } au lieu de { $run2 }
godot-document-falta_o_objeto = l'objet { $name } est absent
godot-document-objeto_em_vez_de = objet { $name } : { $g } au lieu de { $w }
godot-document-objeto_a_mais = objet en trop : { $name }
godot-document-ligacoes_em_vez_de = liaisons : { $connections } au lieu de { $connections2 }
godot-document-outras_partes_do_modelo_calendarios = d'autres parties du modèle (calendriers, réseau, flux, fluides) diffèrent
godot-document-clique = Clic
godot-document-seleciona_o_bloco_ou_a = sélectionne le bloc ou la liaison
godot-document-clique_de_novo_no_mesmo = Cliquer de nouveau au même endroit
godot-document-passa_para_o_proximo_bloco = passe au bloc suivant superposé
godot-document-poe_ou_tira_o_bloco = ajoute ou retire le bloc de la sélection
godot-document-shift_arrastar_no_chao = Shift + glisser au sol
godot-document-selecao_por_caixa_soma_a = sélection par boîte (s'ajoute à la sélection)
godot-document-arrastar_no_chao = Glisser au sol
godot-document-selecao_por_caixa_troca_a = sélection par boîte (remplace la sélection)
godot-document-arrastar_um_bloco = Glisser un bloc
godot-document-move_o_bloco_ou_a = déplace le bloc ou toute la sélection
godot-document-arrastar_da_porta_de_saida = Glisser depuis le port de sortie (vert)
godot-document-liga_ao_bloco_ou_a = relie au bloc (ou au port d'entrée) où l'on relâche
godot-document-botao_direito_arrastar = Bouton droit + glisser
godot-document-gira_a_vista = fait pivoter la vue
godot-document-shift_botao_direito_arrastar = Shift + bouton droit + glisser
godot-document-desloca_a_vista = déplace la vue
godot-document-roda_do_mouse = Molette de la souris
godot-document-aproxima_e_afasta = zoome et dézoome
godot-document-biblioteca_clique_e_clique_no = Bibliothèque : clic puis clic au sol
godot-document-coloca_um_objeto_shift_mantem = place un objet (Shift garde l'outil)
godot-document-biblioteca_arrastar_ate_o_chao = Bibliothèque : glisser jusqu'au sol
godot-document-coloca_um_objeto_onde_soltar = place un objet là où l'on relâche
godot-document-setas = Flèches
godot-document-move_a_selecao_um_passo = déplace la sélection d'un pas de grille
godot-document-girar_90_anti_horario_visto = pivoter de 90° (sens antihoraire vu de dessus)
godot-document-girar_90_horario = pivoter de −90° (sens horaire)
godot-document-girar_15_15 = pivoter de 15° / −15°
godot-document-arrastar_a_alca_de_giro = Glisser la poignée de rotation (violette)
godot-document-gira_o_objeto_selecionado_em = fait pivoter l'objet sélectionné par pas de 15° (avec Shift, 90°)
godot-document-exclui_a_selecao_ou_a = supprime la sélection (ou la liaison sélectionnée)
godot-document-ferramenta_ligar_arraste_da_origem = outil Relier : glissez de l'origine à la destination, ou cliquez puis cliquez
godot-document-ferramenta_corredor_clique_pontos_no = outil Allée : cliquez des points au sol, Enter ou double-clic termine
godot-document-ferramenta_zona_zonas_de_risco = outil Zone : zones de risque au sol ; cliquez les coins, Enter ou le premier point ferme
godot-document-e_ou_alt_com_ligar = E (ou Alt avec Relier)
godot-document-ferramenta_equipe_arraste_do_despachante = outil Équipe : glissez du répartiteur vers le transporteur, ou de l'objet vers celui qui transporte ses articles (Shift : opérateurs du processus)
godot-document-cancela_a_ferramenta_a_ligacao = annule l'outil, la liaison ou l'allée en cours, ou la sélection
godot-document-simular_clique_num_objeto = Simuler : clic sur un objet
godot-document-abre_o_painel_de_resultados = ouvre le panneau de résultats en direct de l'objet dans le coin de la vue
godot-document-simular_esc = Simuler : Esc
godot-document-fecha_o_painel_ao_vivo = ferme le panneau en direct le plus récent (le non épinglé d'abord)
godot-document-f_editar_ou_home = F (Éditer) ou Home
godot-document-enquadra_o_modelo = cadre le modèle
godot-document-simular_f = Simuler : F
godot-document-anda_pela_fabrica_em_primeira = parcourt l'usine à la première personne : WASD pour avancer, la souris pour regarder, Shift pour courir, E/Q pour monter et descendre, V pour voler, la molette change la vitesse, le clic sur le viseur ouvre le panneau, Esc quitte
godot-document-liga_ou_desliga_o_encaixe = active ou désactive l'accrochage à la grille
godot-document-alterna_entre_editar_e_simular = bascule entre Éditer et Simuler
godot-document-mostra_ou_esconde_esta_janela = affiche ou masque cette fenêtre
godot-document-seleciona_todos_os_objetos = sélectionne tous les objets
godot-document-copia_a_selecao = copie la sélection
godot-document-recorta_a_selecao = coupe la sélection
godot-document-cola_nomes_unicos_ligacoes_internas = colle (noms uniques, liaisons internes préservées)
godot-document-duplica_a_selecao = duplique la sélection
godot-document-salva_como = enregistre sous
godot-document-abre_um_modelo = ouvre un modèle
godot-document-novo_modelo = nouveau modèle

## editor3d.rs

godot-editor3d-opera = { $to } opère { $from }
godot-editor3d-leva_os_itens_de = { $to } transporte les articles de { $from }
godot-editor3d-esta_na_equipe_de = { $to } fait partie de l'équipe de { $from }
godot-editor3d-ligacao_de = Liaison de { $k } : { $from } → { $to }
godot-editor3d-arraste_do_no_laranja_dele = { $name } : faites glisser depuis son nœud orange jusqu'à une machine pour qu'il l'opère (ou jusqu'à un objet, pour transporter ses articles).
godot-editor3d-de_clique_no_no_de = {" "}({ $next } sur { $links } : cliquez de nouveau sur le nœud pour le suivant)
godot-editor3d-ligacao_selecionada_clique_no_ou = Liaison { $a } → { $b } sélectionnée{ $extra }. Cliquez sur × ou Del pour la supprimer.
godot-editor3d-esta_ligacao_se_muda_no = Cette liaison ({ $kind }) se modifie dans le panneau de propriétés de { $from }.
godot-editor3d-ligacao_removida_ctrl_z_desfaz = Liaison { $a } → { $b } supprimée (Ctrl+Z annule).
godot-editor3d-nenhuma_zona_selecionada_para_excluir = Aucune zone sélectionnée à supprimer.
godot-editor3d-nada_selecionado_para_excluir = Rien de sélectionné à supprimer.
godot-editor3d-zona_cancelada = Zone annulée.
godot-editor3d-ligacao_cancelada = Liaison annulée.
godot-editor3d-girar_arraste_em_volta_do = Pivoter : glissez autour de l'objet (pas de 15° ; avec Shift, 90°).
godot-editor3d-solte_sobre_a_maquina_que = Relâchez sur la machine que { $res } va opérer (ou sur un objet, pour transporter ses articles). Esc annule.
godot-editor3d-ligar_solte_sobre_o_destino = Relier : relâchez sur la destination (ou cliquez dessus). Esc annule.
godot-editor3d-ligar_solte_sobre_a_origem = Relier : relâchez sur l'origine (ou sur son nœud de sortie). Esc annule.
godot-editor3d-ligar_clique_em_um_bloco = Relier : cliquez sur un bloc.
godot-editor3d-ligado_clique_em_outra_origem = Reliés : { $from } → { $to }. Cliquez sur une autre origine ou Esc.
godot-editor3d-ligacao_selecionada_clique_no_ou-2 = Liaison { $a } → { $b } sélectionnée. Cliquez sur × ou Del pour la supprimer.
godot-editor3d-clique_no_ou_del_remove = { $said }. Cliquez sur × ou Del pour la supprimer.
godot-editor3d-de_blocos_neste_ponto_clique = { $name } ({ $index } sur { $hits } blocs à cet endroit : cliquez de nouveau pour le suivant).
godot-editor3d-arraste_deste_no_verde_ate = { $from } : faites glisser depuis ce nœud vert jusqu'à la destination pour relier.
godot-editor3d-ligar_clique_no_destino_esc = Relier : { $from } → cliquez sur la destination (Esc annule).
godot-editor3d-ligado = Reliés : { $from } → { $to }.
godot-editor3d-ligacao_cancelada_solte_sobre_o = Liaison annulée : relâchez sur le bloc de destination.
godot-editor3d-nao_recebe_de_ninguem_arraste = { $to } ne reçoit de personne : faites glisser depuis le nœud de sortie de l'origine jusqu'ici.
godot-editor3d-ligacao_cancelada_solte_sobre_o-2 = Liaison annulée : relâchez sur le bloc d'origine.
godot-editor3d-girado_para = { $name } pivoté à { $degrees }°.
godot-editor3d-arraste_a_alca_em_volta = { $name } : faites glisser la poignée autour de l'objet pour le faire pivoter (R pivote de 90°, Ctrl+R de 15°).
godot-editor3d-objeto_s_movidos =
    { $selected } { $selected ->
        [one] objet déplacé
       *[other] objets déplacés
    }.
godot-editor3d-objeto_s_na_caixa =
    { $inside } { $inside ->
        [one] objet dans la boîte
       *[other] objets dans la boîte
    }.
godot-editor3d-nada_mudou_solte_sobre_a = Rien n'a changé : relâchez sur la machine (ou l'objet) que { $res } va servir.
godot-editor3d-solte_sobre_uma_maquina_ou = Relâchez sur une machine ou un objet (un transporteur ne rejoint que l'équipe d'un répartiteur).
godot-editor3d-ja_esta_ligado_para_desfazer = Déjà relié : { $to } → { $res }. Pour annuler, cliquez sur la ligne puis sur ×.
godot-editor3d-ligado_ctrl_z_desfaz = Relié : { $said }. Ctrl+Z annule.
godot-editor3d-aponte_para_o_chao_para = Pointez vers le sol pour placer l'objet.
godot-editor3d-equipe_clique_no_transportador_ou = Équipe : cliquez sur le transporteur ou le répartiteur de destination.
godot-editor3d-equipe_comece_num_despachante_ou = Équipe : commencez par un répartiteur ou un objet.
godot-editor3d-equipe_clique_no_destino_esc = Équipe : { $from } → cliquez sur la destination (Esc annule).
godot-editor3d-equipe_solte_sobre_o_transportador = Équipe : relâchez sur le transporteur ou le répartiteur.
godot-editor3d-equipe_solte_sobre_outro_objeto = Équipe : relâchez sur un autre objet.
godot-editor3d-aponte_para_o_chao_para-2 = Pointez vers le sol pour placer un point de l'allée.
godot-editor3d-corredor_clique_os_proximos_pontos = Allée : cliquez les points suivants ; Enter ou double-clic termine, Backspace revient d'un point, Esc annule.
godot-editor3d-ponto_s_enter_ou_duplo =
    { $aisle_points } { $aisle_points ->
        [one] point
       *[other] points
    }. Enter ou double-clic termine.
godot-editor3d-vertice_de_selecionado_arraste_para = Sommet { $k } de { $name } sélectionné (glissez pour déplacer).
godot-editor3d-corredor_selecionado_del_exclui_shift = Allée { $name } sélectionnée (Del supprime ; Shift + clic sur la ligne ajoute un sommet).
godot-editor3d-um_corredor_precisa_de_ao = Une allée a besoin d'au moins 2 points.
godot-editor3d-ponto_removido = Point supprimé ({ $aisle_points }).
godot-editor3d-trecho_selecionado_del_remove = Tronçon { $a } → { $b } sélectionné (Del supprime).
godot-editor3d-aponte_para_o_chao_para-3 = Pointez vers le sol pour placer un nœud.
godot-editor3d-zona_ponto_s_clique_no =
    Zone : { $zone_points } { $zone_points ->
        [one] point
       *[other] points
    }. Cliquez sur le premier point, Enter ou double-clic ferme ; Backspace revient en arrière ; Esc annule.
godot-editor3d-zona_arraste_para_mover_arraste = Zone { $name } : glissez pour déplacer, glissez un sommet pour changer la forme ; Del supprime.
godot-editor3d-zona_clique_para_por_os = Zone : cliquez pour placer les autres points ; Enter, double-clic ou le premier point ferme.
godot-editor3d-zona_alterada_ctrl_z_desfaz = Zone modifiée (Ctrl+Z annule).
godot-editor3d-uma_zona_precisa_de_pelo = Une zone a besoin d'au moins 3 points.

## indicadores.rs

godot-indicadores-itens_refugados = Articles rebutés
godot-indicadores-itens_que_terminaram_neste_refugo = Articles qui ont fini dans ce rebut, par réplication.
godot-indicadores-defeitos_gerados = Défauts générés
godot-indicadores-defeitos_que_este_posto_gerou = Défauts générés par ce poste, par réplication.
godot-indicadores-defeitos_que_escaparam = Défauts échappés
godot-indicadores-defeitos_deste_posto_que_chegaram = Défauts de ce poste arrivés jusqu'au client (défaillance externe), par réplication.
godot-indicadores-rendimento_de_primeira = Rendement au premier passage
godot-indicadores-itens_inspecionados_uma_so_vez = Articles inspectés une seule fois et livrés, sur les articles inspectés.
godot-indicadores-custo_da_qualidade = Coût de la qualité
godot-indicadores-avaliacao_falha_interna_e_falha = Évaluation, défaillance interne et défaillance externe attribuées à ce poste, par réplication.
godot-indicadores-tempo_em_zonas = Temps en zones
godot-indicadores-tempo_desta_pessoa_dentro_das = Temps de cette personne dans les zones de risque, par réplication.
godot-indicadores-exposicao_nas_zonas = Exposition dans les zones
godot-indicadores-tempo_de_pessoas_dentro_das = Temps des personnes dans les zones où se trouve ce poste, cumulé, par réplication.
godot-indicadores-zonas = Zones
godot-indicadores-zonas_de_risco_onde_o = Zones de risque où se trouve le poste.
godot-indicadores-entradas_sem_epi = Entrées sans EPI
godot-indicadores-entradas_na_zona_sem_o = Entrées dans la zone sans l'équipement de protection qu'elle exige, par réplication.
godot-indicadores-quase_conflitos = Quasi-conflits
godot-indicadores-pessoa_e_veiculo_mais_perto = Personne et véhicule plus proches que le seuil, impliquant cette ressource, par réplication. Simulé : sert à comparer des agencements, n'estime pas des accidents réels.
godot-indicadores-pessoa_e_veiculo_mais_perto-2 = Personne et véhicule plus proches que le seuil, à moins de 3 m de ce poste, par réplication. Simulé : sert à comparer des agencements, n'estime pas des accidents réels.
godot-indicadores-quebras = Pannes
godot-indicadores-falhas_do_posto_por_replicacao = Défaillances du poste, par réplication.
godot-indicadores-tempo_medio_entre_falhas_tempo = Temps moyen entre défaillances (temps en marche sur le nombre de défaillances).
godot-indicadores-tempo_medio_de_reparo_da = Temps moyen de réparation, de la panne jusqu'à la reprise de la production.
godot-indicadores-tempo_parado = Temps à l'arrêt
godot-indicadores-tempo_em_manutencao_corretiva_por = Temps en maintenance corrective, par réplication.
godot-indicadores-disponibilidade = Disponibilité
godot-indicadores-tempo_produzindo_sobre_o_tempo = Temps de production sur le temps planifié (hors équipes et préventives).
godot-indicadores-custo_de_manutencao = Coût de maintenance
godot-indicadores-tempo_parado_reparos_e_preventivas = Temps à l'arrêt, réparations et préventives multipliés par les coûts du bloc, par réplication.
godot-indicadores-parcela_do_tempo_em_quebra = Part du temps en panne, par réplication.
godot-indicadores-tempo_fora_de_quebra_sobre = Temps hors panne sur le temps planifié (hors équipes).

## jobs.rs

godot-jobs-rodando_replicacoes = Exécution des réplications
godot-jobs-rodando_mais_replicacoes = Exécution de réplications supplémentaires
godot-jobs-comparando_cenarios = Comparaison des scénarios
godot-jobs-otimizando = Optimisation
godot-jobs-analisando_o_aquecimento = Analyse de la période de chauffe
godot-jobs-conferindo_a_lei_de_little = Vérification de la loi de Little
godot-jobs-procurando_o_gargalo_movel = Recherche du goulot d'étranglement mobile
godot-jobs-calculando_o_pert_cpm = Calcul du PERT/CPM
godot-jobs-ajustando_a_distribuicao_aos_dados = Ajustement de la distribution aux données
godot-jobs-gravando_o_relatorio_dos_dados = Écriture du rapport des données
godot-jobs-erro_interno_durante_a_execucao = erreur interne pendant l'exécution
godot-jobs-a_execucao_terminou_sem_resultado = l'exécution s'est terminée sans résultat
godot-jobs-objeto_nao_existe_no_modelo = l'objet "{ $object }" n'existe pas dans le modèle

## linha.rs


## path_edit.rs

godot-path-edit-nao_existe = '{ $name }' n'existe pas
godot-path-edit-e_uma_esteira_reta_sem = '{ $name }' est un convoyeur droit, sans nœuds
godot-path-edit-nao_e_uma_esteira = '{ $name }' n'est pas un convoyeur
godot-path-edit-a_esteira_precisa_de_ao = le convoyeur a besoin d'au moins 2 nœuds différents
godot-path-edit-a_biblioteca_nao_tem_esteira = la bibliothèque n'a pas de convoyeur
godot-path-edit-o_caminho_precisa_de_ao = le chemin a besoin d'au moins 2 points différents
godot-path-edit-nao_existe_o_caminho = le chemin '{ $name }' n'existe pas
godot-path-edit-nao_tem_o_no = '{ $name }' n'a pas le nœud { $index }
godot-path-edit-o_raio_deve_ser_um = le rayon doit être un nombre >= 0 (mètres)
godot-path-edit-a_distancia_do_paralelo_deve = la distance du parallèle doit être différente de zéro
godot-path-edit-o_caminho_novo_sumiu = le nouveau chemin a disparu
godot-path-edit-o_caminho_precisa_de_ao-2 = le chemin a besoin d'au moins 2 points
godot-path-edit-a_esteira_precisa_de_ao-2 = le convoyeur a besoin d'au moins 2 nœuds
godot-path-edit-nao_e_uma_esteira_desenhada = '{ $name }' n'est pas un convoyeur dessiné
godot-path-edit-as_duas_pontas_ja_estao = les deux extrémités sont déjà au sol
godot-path-edit-tipo_de_esteira_desconhecido = type de convoyeur inconnu : { $style }
godot-path-edit-acionamento_desconhecido = entraînement inconnu : { $drive }
godot-path-edit-a_largura_deve_ser_0 = la largeur doit être > 0 (mètres)
godot-path-edit-a_inclinacao_maxima_fica_entre = l'inclinaison maximale est comprise entre 0 et 90 degrés
godot-path-edit-divida_num_no_do_meio = divisez à un nœud du milieu du convoyeur
godot-path-edit-nao_ha_trecho_depois_do = il n'y a pas de tronçon après le dernier nœud
godot-path-edit-o_caminho_so_tem_esse = le chemin n'a que ce tronçon : supprimez le chemin entier
godot-path-edit-a_esteira_so_tem_esse = le convoyeur n'a que ce tronçon : supprimez le convoyeur entier
godot-path-edit-nenhum_trajeto_comeca_onde_este = aucun trajet ne commence là où celui-ci finit
godot-path-edit-nao_existe_o_caminho-2 = le chemin '{ $other }' n'existe pas

## tool_panels.rs

godot-tool-panels-ligacao = Liaison
godot-tool-panels-remover_ligacao_del = Supprimer la liaison (Del)
godot-tool-panels-rede_de_caminhos = Réseau de chemins
godot-tool-panels-no_s_trecho_s_pista =
    { $nodes } { $nodes ->
        [one] nœud
       *[other] nœuds
    }, { $edges } { $edges ->
        [one] tronçon
       *[other] tronçons
    }, { $controlled } { $controlled ->
        [one] voie contrôlée
       *[other] voies contrôlées
    }.
godot-tool-panels-clique_no_chao_para_por = Cliquez au sol pour placer un nœud ; avec un nœud armé (vert), le clic suivant le relie au nouveau par un tronçon. Cliquez sur un nœud pour l'armer, glissez pour le déplacer. Cliquez sur un tronçon pour changer le sens et la capacité (avec une capacité, la voie est contrôlée et devient rouge). Esc termine le chemin ; Del supprime l'élément sélectionné.
godot-tool-panels-novos_trechos_em_mao_dupla = Nouveaux tronçons à double sens
godot-tool-panels-no = Nœud { $name }
godot-tool-panels-nome = Nom
godot-tool-panels-posicao_m = Position ({ $position }, { $position2 }) m
godot-tool-panels-excluir_no_del = Supprimer le nœud (Del)
godot-tool-panels-trecho = Tronçon { $from } → { $to }
godot-tool-panels-comprimento_m = Longueur { $length } m
godot-tool-panels-mao_dupla = Double sens
godot-tool-panels-capacidade_veiculos_0_sem_controle = Capacité (véhicules ; 0 = sans contrôle)
godot-tool-panels-pista_controlada_no_maximo_veiculo =
    Voie contrôlée : au maximum { $cap } { $cap ->
        [one] véhicule
       *[other] véhicules
    } en même temps.
godot-tool-panels-inverter_sentido = Inverser le sens
godot-tool-panels-excluir_trecho_del = Supprimer le tronçon (Del)
godot-tool-panels-pedestres = Piétons
godot-tool-panels-empilhadeiras = Chariots élévateurs
godot-tool-panels-misto = Mixte
godot-tool-panels-corredores_de_circulacao = Allées de circulation
godot-tool-panels-corredor_es_de_pedestres_de =
    { $list } { $list ->
        [one] allée
       *[other] allées
    } : { $pedestrian } de piétons, { $forklift } de chariots élévateurs, { $forklift2 } mixtes ; { $crossings } { $crossings ->
        [one] croisement
       *[other] croisements
    }.
godot-tool-panels-clique_no_chao_para_por-2 = Cliquez au sol pour placer les points de l'axe ; Enter ou double-clic termine, Backspace revient d'un point, Esc annule. Les points s'accrochent aux autres allées. Cliquez sur une allée pour la modifier, glissez un sommet pour le déplacer, Shift + clic sur la ligne ajoute un sommet. Les bandes jaunes, le zébra des croisements et les flèches apparaissent au sol.
godot-tool-panels-novos_corredores = Nouvelles allées
godot-tool-panels-tipo = Type
godot-tool-panels-largura_m = Largeur (m)
godot-tool-panels-corredor = Allée { $name }
godot-tool-panels-ponto_s_m_de_comprimento =
    { $points } { $points ->
        [one] point
       *[other] points
    }, { $length } m de longueur
godot-tool-panels-passagem_secundaria_de_pedestres_minimo = Passage piéton secondaire (minimum réduit)
godot-tool-panels-perfil = Profil
godot-tool-panels-padrao_do_modelo = Défaut du modèle
godot-tool-panels-brasil_amarelo = Brésil (jaune)
godot-tool-panels-brasil = Brésil
godot-tool-panels-branco = Blanc
godot-tool-panels-vertice_selecionado = Sommet { $v } sélectionné
godot-tool-panels-remover_vertice = Supprimer le sommet
godot-tool-panels-excluir_corredor_del = Supprimer l'allée (Del)
godot-tool-panels-zonas_de_risco = Zones de risque
godot-tool-panels-zona_s_de_risco_no =
    { $zones } { $zones ->
        [one] zone
       *[other] zones
    } de risque au sol.
godot-tool-panels-clique_no_chao_para_por-3 = Cliquez au sol pour placer les coins ; le premier point, Enter ou double-clic ferme la zone, Backspace revient d'un point, Esc annule. Cliquez sur une zone pour la modifier : glissez le milieu pour déplacer, glissez un sommet pour changer la forme ; Del supprime. Les points s'accrochent à la grille.
godot-tool-panels-zona = Zone { $name }
godot-tool-panels-canto_s =
    { $polygon } { $polygon ->
        [one] coin
       *[other] coins
    }
godot-tool-panels-nivel_de_risco_1_a = Niveau de risque (1 à 5 ; 0 = sans niveau)
godot-tool-panels-epi_exigido_para_entrar = EPI exigé pour entrer
godot-tool-panels-excluir_zona_del = Supprimer la zone (Del)

## ui_arrivals.rs

godot-ui-arrivals-nao_e_um_numero = "{ $t }" n'est pas un nombre
godot-ui-arrivals-use_um_numero = utilisez un nombre
godot-ui-arrivals-hh_mm_so_vale_com = hh:mm n'est valable qu'avec un cycle de jour ou de semaine (1440 min, 24 h, 86400 s) ; utilisez un nombre
godot-ui-arrivals-hora_invalida_em = heure invalide dans "{ $t }"
godot-ui-arrivals-minuto_invalido_em = minute invalide dans "{ $t }"
godot-ui-arrivals-minuto_invalido_em_0_a = minute invalide dans "{ $t }" (0 à 59)
godot-ui-arrivals-nao_e_um_horario_hh = "{ $t }" n'est ni une heure (hh:mm) ni un nombre
godot-ui-arrivals-quantidade_um_inteiro_0 = quantité "{ $t }" : un entier >= 0
godot-ui-arrivals-rotulo_use_nome_valor = étiquette "{ $part }" : utilisez nom=valeur
godot-ui-arrivals-rotulo_sem_nome = étiquette "{ $part }" sans nom
godot-ui-arrivals-rotulo = étiquette { $name } : { $e }
godot-ui-arrivals-uma_vez = Une fois
godot-ui-arrivals-a_cada_ciclo = À chaque cycle
godot-ui-arrivals-volta_ao_inicio = Retour au début
godot-ui-arrivals-por_ciclo_de = par cycle de { $d }
godot-ui-arrivals-por_volta = par tour
godot-ui-arrivals-no_total = au total
godot-ui-arrivals-em_linha_s =
    { $items } { $items ->
        [one] article
       *[other] articles
    } { $per } en { $rows } { $rows ->
        [one] ligne
       *[other] lignes
    }
godot-ui-arrivals-tipo_da_fonte = type de la source
godot-ui-arrivals-um_dia = { $d } (un jour)
godot-ui-arrivals-uma_semana = { $d } (une semaine)
godot-ui-arrivals-tipo_de_item = type d'article
godot-ui-arrivals-nada_para_colar_copie_as = rien à coller : copiez les lignes du tableur
godot-ui-arrivals-linha = ligne { $n } : { $e }
godot-ui-arrivals-falta_o_instante_modo_horario = l'instant est absent (mode horaire)
godot-ui-arrivals-so_o_cabecalho_nenhuma_linha = en-tête seul : aucune ligne à coller
godot-ui-arrivals-editar_como_texto_ron = Éditer comme texte (RON)
godot-ui-arrivals-modo = Mode
godot-ui-arrivals-horario = Horaire
godot-ui-arrivals-sequencia = Séquence
godot-ui-arrivals-horario_cada_linha_no_seu = Horaire : chaque ligne à son instant. Séquence : une ligne par arrivée, avec le temps entre arrivées de la source entre elles.
godot-ui-arrivals-repetir = Répéter
godot-ui-arrivals-ciclo = Cycle
godot-ui-arrivals-da_fonte = (de la source)
godot-ui-arrivals-o_tipo_da_fonte = Le type de la source
godot-ui-arrivals-tipo_de_item_da_linha = Type d'article de la ligne
godot-ui-arrivals-subir_a_linha = Monter la ligne
godot-ui-arrivals-descer_a_linha = Descendre la ligne
godot-ui-arrivals-remover_a_linha = Supprimer la ligne
godot-ui-arrivals-linha-2 = + Ligne
godot-ui-arrivals-acrescenta_uma_linha_no_fim = Ajoute une ligne à la fin
godot-ui-arrivals-ordenar_por_instante = Trier par instant
godot-ui-arrivals-poe_as_linhas_em_ordem = Met les lignes dans l'ordre des instants
godot-ui-arrivals-as_linhas_ja_estao_em = Les lignes sont déjà en ordre (ou c'est une séquence)
godot-ui-arrivals-colar_da_planilha = Coller depuis le tableur
godot-ui-arrivals-linhas_copiadas_de_uma_planilha = Lignes copiées d'un tableur (séparées par tabulation ou ;)
godot-ui-arrivals-hora_qtd_tipo_rotulo_08 =
    heure; qté; type; étiquette...
    08:00; 12; caisse; priorite=2
godot-ui-arrivals-qtd_tipo_rotulo_2_caixa =
    qté; type; étiquette...
    2; caisse; priorite=2
godot-ui-arrivals-substituir_as_linhas = Remplacer les lignes
godot-ui-arrivals-acrescentar = Ajouter
godot-ui-arrivals-ver_as_primeiras_chegadas = Voir les premières arrivées
godot-ui-arrivals-histograma_das_chegadas_de_um = Histogramme des arrivées d'un cycle (ou d'un tour), par heure et par type
godot-ui-arrivals-simulado_so_a_fonte_ligada = Simulé : la source seule, reliée à une sortie, pendant un cycle.
godot-ui-arrivals-estimado_pela_tabela_a_fonte = Estimé d'après le tableau (la source seule n'a pas tourné : { $e }).
godot-ui-arrivals-estimado_pela_tabela_rodando_a = Estimé d'après le tableau ; exécution de la source...
godot-ui-arrivals-maximo_por = maximum { $num_text } par { $bin_label }
godot-ui-arrivals-por_instantes_da_tabela_sem = par { $bin_label } ({ $bin }) ; instants du tableau, sans exécuter le modèle
godot-ui-arrivals-horarios_em_hh_mm_taxas = { $name } (heures en hh:mm, taux par heure)
godot-ui-arrivals-unidade_do_modelo = unité du modèle
godot-ui-arrivals-fora_dos_periodos_a_fonte = En dehors des périodes, la source ne crée pas d'articles
godot-ui-arrivals-periodo_s =
    { $periods } { $periods ->
        [one] période
       *[other] périodes
    }
godot-ui-arrivals-itens_h_ou_distribuicao = articles/h ou distribution
godot-ui-arrivals-taxa_ou_distribuicao = taux ou distribution
godot-ui-arrivals-padrao_o_tempo_entre_chegadas = Défaut : le temps entre arrivées de la source. Distribution : un temps entre arrivées propre. Taux : arrivées de Poisson. Fermé : sans arrivées.
godot-ui-arrivals-distribuicao = distribution : { $e }
godot-ui-arrivals-media = moyenne { $mean }
godot-ui-arrivals-sem_chegadas = sans arrivées
godot-ui-arrivals-remover_o_periodo = Supprimer la période
godot-ui-arrivals-periodo = + Période
godot-ui-arrivals-acrescenta_um_periodo_depois_do = Ajoute une période après la dernière
godot-ui-arrivals-o_ultimo_periodo_ja_vai = La dernière période va déjà jusqu'à la fin du cycle
godot-ui-arrivals-esperado_por_ciclo_chegadas = Attendu par cycle : { $total } arrivées
godot-ui-arrivals-padrao_media = défaut (moyenne { $mean })
godot-ui-arrivals-distribuicao-2 = distribution { $d }
godot-ui-arrivals-taxa_itens_h = taux { $cycle } articles/h
godot-ui-arrivals-taxa = taux { $num_text }
godot-ui-arrivals-periodo_a_esperadas = Période { $k } : { $cycle } à { $cycle2 }, { $how } ; attendues { $default }
godot-ui-arrivals-chegadas_esperadas_em_cada_periodo = Arrivées attendues dans chaque période (taux × durée, ou durée ÷ moyenne)
godot-ui-arrivals-barra_s =
    { $exp } { $exp ->
        [one] barre
       *[other] barres
    }
godot-ui-arrivals-nao_existe = '{ $source }' n'existe pas
godot-ui-arrivals-nao_e_uma_chegada = '{ $source }' n'est pas une arrivée

## ui_bar.rs

godot-ui-bar-equipe_despachante_membro = Équipe (répartiteur → membre)
godot-ui-bar-transporte_pedido = Transport demandé
godot-ui-bar-operadores_pedidos = Opérateurs demandés
godot-ui-bar-equipe_de_manutencao = Équipe de maintenance
godot-ui-bar-local_de_pausa = Lieu de pause
godot-ui-bar-mais = Plus
godot-ui-bar-os_grupos_que_nao_cabem = Les groupes qui ne tiennent pas dans la barre
godot-ui-bar-arquivo = Fichier
godot-ui-bar-editar = Éditer
godot-ui-bar-organizar = Organiser
godot-ui-bar-grade = Grille
godot-ui-bar-ajuda = Aide
godot-ui-bar-analise = Analyse
godot-ui-bar-novo = Nouveau
godot-ui-bar-novo_modelo_ctrl_n = Nouveau modèle (Ctrl+N)
godot-ui-bar-abrir = Ouvrir…
godot-ui-bar-abrir_um_modelo_ou_um = Ouvrir un modèle ou un replay (Ctrl+O)
godot-ui-bar-salvar = Enregistrer
godot-ui-bar-salvar_ctrl_s = Enregistrer (Ctrl+S)
godot-ui-bar-salvar_como = Enregistrer sous…
godot-ui-bar-salvar_com_outro_nome_ctrl = Enregistrer sous un autre nom (Ctrl+Shift+S)
godot-ui-bar-desfazer = Annuler
godot-ui-bar-desfazer_ctrl_z = Annuler (Ctrl+Z)
godot-ui-bar-refazer = Rétablir
godot-ui-bar-refazer_ctrl_y = Rétablir (Ctrl+Y)
godot-ui-bar-alinhar_a_esquerda = Aligner à gauche
godot-ui-bar-alinhar_ao_centro_x = Aligner au centre (X)
godot-ui-bar-alinhar_a_direita = Aligner à droite
godot-ui-bar-alinhar_ao_topo = Aligner en haut
godot-ui-bar-alinhar_ao_centro_y = Aligner au centre (Y)
godot-ui-bar-alinhar_a_base = Aligner en bas
godot-ui-bar-distribuir_em_x = Répartir en X
godot-ui-bar-distribuir_em_y = Répartir en Y
godot-ui-bar-copiar_ctrl_c = Copier (Ctrl+C)
godot-ui-bar-recortar_ctrl_x = Couper (Ctrl+X)
godot-ui-bar-colar_ctrl_v = Coller (Ctrl+V)
godot-ui-bar-duplicar_ctrl_d = Dupliquer (Ctrl+D)
godot-ui-bar-excluir_del = Supprimer (Del)
godot-ui-bar-selecionar_tudo_ctrl_a = Tout sélectionner (Ctrl+A)
godot-ui-bar-alinhar_distribuir_copiar_colar_e = Aligner, répartir, copier, coller et supprimer la sélection
godot-ui-bar-encaixar_na_grade_ao_por = Accrocher à la grille au placement et au glissement (G ; pas à côté)
godot-ui-bar-passo_da_grade = Pas de la grille
godot-ui-bar-atalhos_f1_a_lista_dos = Raccourcis (F1) : la liste des raccourcis clavier et souris
godot-ui-bar-resultados = Résultats
godot-ui-bar-tabela_graficos_analises_cenarios_e = Tableau, graphiques, analyses, scénarios et optimisation de la dernière exécution
godot-ui-bar-pert_cpm = PERT/CPM
godot-ui-bar-pert_cpm_da_selecao_com = PERT/CPM de la sélection (avec deux objets, de l'un à l'autre)
godot-ui-bar-pert_cpm_da_selecao = PERT/CPM de la sélection
godot-ui-bar-pert_cpm_de_ate_dois = PERT/CPM de… à… (deux sélectionnés)
godot-ui-bar-fluxos_de_tokens = Flux de jetons
godot-ui-bar-modo_missao_utsim_ensino = Mode mission (UTSim Enseignement)
godot-ui-bar-abre_as_missoes_do_utsim = Ouvre les missions d'UTSim Enseignement dans ce même programme
godot-ui-bar-outras_analises_pert_cpm_de = Autres analyses : PERT/CPM de… à…, flux de jetons ; les liaisons d'équipe et de demande dans la vue 3D
godot-ui-bar-simular = Simuler
godot-ui-bar-montar_o_modelo_tab_alterna = Construire le modèle (Tab bascule)
godot-ui-bar-rodar_as_replicacoes_e_reproduzir = Exécuter les réplications et rejouer (Tab bascule)
godot-ui-bar-selecionar = Sélectionner
godot-ui-bar-selecionar_e_mover_esc = Sélectionner et déplacer (Esc)
godot-ui-bar-ligar = Relier
godot-ui-bar-ligar_arraste_da_origem_ao = Relier : glissez de l'origine à la destination, ou cliquez sur l'origine puis sur la destination (L)
godot-ui-bar-rede = Réseau
godot-ui-bar-rede_de_caminhos_nos_e = Réseau de chemins : nœuds et tronçons (N)
godot-ui-bar-corredor = Allée
godot-ui-bar-corredores_de_circulacao_com_faixas = Allées de circulation avec bandes de sécurité : cliquez des points au sol, Enter termine (C)
godot-ui-bar-zona = Zone
godot-ui-bar-zonas_de_risco_no_piso = Zones de risque au sol : cliquez les coins, le premier point ou Enter ferme ; cliquez sur une zone pour modifier (Z)
godot-ui-bar-equipe = Équipe
godot-ui-bar-equipe_arraste_do_despachante_a = Équipe : glissez du répartiteur vers un transporteur (il rejoint ou quitte l'équipe), ou d'un objet vers le transporteur ou le répartiteur qui transporte ses articles ; avec Shift, d'un processus vers celui qui fournit les opérateurs (E ; Alt avec Relier)
godot-ui-bar-ligacoes_na_vista_3d = Liaisons dans la vue 3D
godot-ui-bar-tambem_durante_a_simulacao = Aussi pendant la simulation
godot-ui-bar-as_linhas_tracejadas_entre_operadores = Les lignes en pointillés entre opérateurs, véhicules et machines en mode Simuler

## ui_fitwin.rs

godot-ui-fitwin-linha_nao_e_um_numero = ligne { $k } : "{ $t }" n'est pas un nombre
godot-ui-fitwin-os_instantes_precisam_ser_crescentes = les instants doivent être croissants
godot-ui-fitwin-digite_ou_cole_pelo_menos = saisissez ou collez au moins deux valeurs
godot-ui-fitwin-compare_as_distribuicoes_antes = comparez d'abord les distributions
godot-ui-fitwin-utsim_3d_valores_reais = UTSim 3D, Valeurs réelles
godot-ui-fitwin-desvio_padrao = écart type
godot-ui-fitwin-valores_reais = Valeurs réelles : { $field }
godot-ui-fitwin-o_que_e = Qu'est-ce que c'est : { $title }
godot-ui-fitwin-valores_observados = Valeurs observées
godot-ui-fitwin-digite_cole_da_planilha_varias = Saisissez, collez depuis le tableur (plusieurs lignes, virgule décimale) ou importez un CSV.
godot-ui-fitwin-cole_aqui_e_clique_adicionar = collez ici et cliquez sur Ajouter
godot-ui-fitwin-adicionar = Ajouter
godot-ui-fitwin-valor_es_adicionados =
    { $n } { $n ->
        [one] valeur ajoutée
       *[other] valeurs ajoutées
    }.
godot-ui-fitwin-importar_csv = Importer un CSV…
godot-ui-fitwin-le_os_valores_de_um = Lit les valeurs d'un fichier CSV ou TXT
godot-ui-fitwin-limpar = Effacer
godot-ui-fitwin-apaga_todos_os_valores_da = Efface toutes les valeurs du tableau
godot-ui-fitwin-sao_instantes_crescentes_usar_os = Ce sont des instants croissants (utiliser les intervalles)
godot-ui-fitwin-marque_quando_os_valores_sao = Cochez quand les valeurs sont des heures d'arrivée : l'ajustement utilise les différences entre elles.
godot-ui-fitwin-valor_es =
    { $n } { $n ->
        [one] valeur
       *[other] valeurs
    }
godot-ui-fitwin-apaga_este_valor = Efface cette valeur
godot-ui-fitwin-intervalos = Intervalles
godot-ui-fitwin-estatisticas = Statistiques
godot-ui-fitwin-comparacao_das_distribuicoes = Comparaison des distributions
godot-ui-fitwin-comparar_de_novo = Comparer de nouveau
godot-ui-fitwin-comparar_todas = Comparer toutes
godot-ui-fitwin-ajusta_todas_as_distribuicoes_candidatas = Ajuste toutes les distributions candidates et calcule toutes les mesures en une fois
godot-ui-fitwin-ajustando_todas_as_candidatas = ajustement de toutes les candidates...
godot-ui-fitwin-depois_de_comparar_a_tabela = Après la comparaison, le tableau montre toutes les mesures de chaque distribution ; cliquez sur une ligne pour voir les graphiques et choisir.
godot-ui-fitwin-os_valores_mudaram_compare_de = Les valeurs ont changé : comparez de nouveau.
godot-ui-fitwin-ordenada_por_sugestao_inicial_aic = Triée par { $label }. Suggestion initiale (AIC avec la règle d'UTSim) : { $family }.
godot-ui-fitwin-usar_esta_distribuicao = Utiliser cette distribution ({ $fam })
godot-ui-fitwin-passa_a_usar_esta_distribuicao = Utilise désormais cette distribution dans ce champ, ajustée à ces valeurs (Ctrl+Z annule)
godot-ui-fitwin-usar_a_empirica = Utiliser l'empirique
godot-ui-fitwin-usa_os_proprios_valores_observados = Utilise les valeurs observées elles-mêmes, sans famille
godot-ui-fitwin-gerar_relatorio = Générer le rapport
godot-ui-fitwin-grava_o_relatorio_html_com = Écrit le rapport HTML avec les données, la comparaison complète, les graphiques et le choix
godot-ui-fitwin-fechar = Fermer
godot-ui-fitwin-fecha_a_explicacao = Ferme l'explication
godot-ui-fitwin-como_ler_quando_usar_cuidados =
    { $what }
    Comment lire : { $read }
    Quand l'utiliser : { $when }
    Précautions : { $care }
godot-ui-fitwin-aceita_pelos_testes_clique_para = Acceptée par les tests. Cliquez pour voir les graphiques et choisir.
godot-ui-fitwin-nao_se_aplica_a_estes = Ne s'applique pas à ces données.
godot-ui-fitwin-rejeitada_por_algum_teste_valor = Rejetée par au moins un test (valeur p inférieure à 0,05). Peut encore être choisie.
godot-ui-fitwin-ordena_por = Trie par { $label } ({ $melhor })
godot-ui-fitwin-escolhida_por = choisie par
godot-ui-fitwin-histograma_e_curvas_em_destaque = Histogramme et courbes ({ $selected_family } en évidence)
godot-ui-fitwin-poisson = Poisson
godot-ui-fitwin-geometrica = Géométrique
godot-ui-fitwin-binomial = Binomiale
godot-ui-fitwin-binomial_negativa = Binomiale négative
godot-ui-fitwin-a_empirica_reproduz_os_proprios = L'empirique reproduit les données elles-mêmes : il n'y a pas de courbe à comparer.

## ui_left.rs

godot-ui-left-orbita = Orbite
godot-ui-left-planta_2d = Plan (2D)
godot-ui-left-seguir = Suivre
godot-ui-left-primeira_pessoa = Première personne
godot-ui-left-do_recurso = Depuis la ressource
godot-ui-left-recentes = Récents
godot-ui-left-usados_por_ultimo = Utilisés en dernier
godot-ui-left-espera = Attente
godot-ui-left-fluxo = Flux
godot-ui-left-fluxo_de_itens_e_montagem = Flux d'articles et assemblage
godot-ui-left-mover = Déplacer
godot-ui-left-movimentacao_recursos_e_logistica = Manutention, ressources et logistique
godot-ui-left-fabrica = Usine
godot-ui-left-manufatura = Fabrication
godot-ui-left-servicos = Services
godot-ui-left-servicos_e_saude = Services et santé
godot-ui-left-escritorio = Bureau
godot-ui-left-escritorio_e_processos = Bureau et processus
godot-ui-left-q_e_s = Q et S
godot-ui-left-qualidade_e_seguranca = Qualité et sécurité
godot-ui-left-biblioteca = Bibliothèque
godot-ui-left-objetos = Objets
godot-ui-left-mostrar_obstaculos_e_folgas = Afficher les obstacles et les dégagements
godot-ui-left-mostra_a_area_ocupada_dos = Affiche l'emprise des objets, les murs, le dégagement des ressources et le point d'accès de chaque objet (glissez-le avec l'outil Sélectionner)
godot-ui-left-nada_com_esse_nome = Rien avec ce nom.
godot-ui-left-clique_de_novo_para_recolher = { $title }. Cliquez de nouveau pour replier.
godot-ui-left-os_elementos_que_voce_usar = Les éléments que vous utiliserez apparaissent ici.
godot-ui-left-buscar_elemento = Chercher un élément
godot-ui-left-limpa_a_busca = Efface la recherche
godot-ui-left-clique_e_depois_clique_no = { $label } : cliquez puis cliquez au sol, ou glissez jusqu'au sol
godot-ui-left-buscar_por_nome_ou_tipo = Chercher par nom ou type
godot-ui-left-problema_s =
    { $problems } { $problems ->
        [one] problème
       *[other] problèmes
    }
godot-ui-left-aviso_s =
    { $warnings } { $warnings ->
        [one] avertissement
       *[other] avertissements
    }
godot-ui-left-simular_de_novo = Simuler de nouveau
godot-ui-left-roda_as_replicacoes_outra_vez = Exécute les réplications une nouvelle fois
godot-ui-left-blocos_clique_para_focar = Blocs (cliquez pour cadrer)
godot-ui-left-fluxos = Flux
godot-ui-left-fluxos_de_tokens_em_2d = Flux de jetons en 2D, avec les comptages dans le temps
godot-ui-left-gravar_replay = Enregistrer le replay
godot-ui-left-grava_o_replay_desta_replicacao = Enregistre le replay de cette réplication en .json (se rouvre avec Ouvrir)
godot-ui-left-rotulos = Étiquettes
godot-ui-left-os_nomes_dos_blocos_sobre = Les noms des blocs sur la scène
godot-ui-left-sombras = Ombres
godot-ui-left-as_sombras_do_sol_desligadas = Les ombres du soleil (désactivées, la scène est plus légère)
godot-ui-left-seguranca = Sécurité
godot-ui-left-zonas_de_risco_epi_das = Zones de risque, EPI des personnes, quasi-conflits simulés, articles défectueux et inspections
godot-ui-left-calor = Chaleur
godot-ui-left-mapa_de_calor_da_presenca = Carte de chaleur de la présence des personnes et des quasi-conflits au sol
godot-ui-left-quase_conflitos_simulados = Quasi-conflits simulés ({ $quase })
godot-ui-left-replicacoes = Réplications
godot-ui-left-uma_replicacao_a_menos = Une réplication de moins
godot-ui-left-uma_replicacao_a_mais = Une réplication de plus
godot-ui-left-camera = Caméra
godot-ui-left-seguir_primeira_pessoa_e_do = Suivre, Première personne et Depuis la ressource accompagnent le bloc choisi dans la liste
godot-ui-left-os_resultados_aparecem_aqui_depois = Les résultats apparaissent ici après la simulation.
godot-ui-left-gargalo = Goulot d'étranglement : { $bottleneck }
godot-ui-left-a_tabela_completa_os_graficos = Le tableau complet, les graphiques et les analyses se trouvent dans Résultats.
godot-ui-left-gargalo-2 =
    {""}
    Goulot d'étranglement : { $bottleneck }

## ui_pert.rs

godot-ui-pert-rede = Réseau
godot-ui-pert-gantt = Gantt
godot-ui-pert-distribuicao = Distribution
godot-ui-pert-tabela = Tableau
godot-ui-pert-texto = Texte
godot-ui-pert-simulado = Simulé
godot-ui-pert-caminho_critico_observado_na_simulacao = Chemin critique observé dans la simulation : files d'attente, transports et ressources inclus
godot-ui-pert-classico = Classique
godot-ui-pert-pert_classico_estimativas_a_m = PERT classique : estimations (a, m, b), CPM sur les moyennes et P(délai) par la loi normale
godot-ui-pert-monte_carlo = Monte Carlo
godot-ui-pert-sorteios_das_duracoes_sobre_a = Tirages des durées sur le réseau : criticité et distribution du temps total
godot-ui-pert-desvio = , écart type { $sd }
godot-ui-pert-unidades_sorteios = {" "}({ $units } unités/tirages)
godot-ui-pert-tempo_total_cpm_nas_medias = Temps total : { $t }{ $spread }{ $units }. CPM sur les moyennes : { $project }.
godot-ui-pert-ic_a = {" "}(IC { $lo } % à { $hi } %)
godot-ui-pert-p_t = P(T ≤ { $deadline }) = { $p }
godot-ui-pert-caminho_critico = Chemin critique : { $names } ({ $index } %)
godot-ui-pert-aviso = avertissement : { $w }
godot-ui-pert-aproximacao_normal_do_pert_classico = approximation normale du PERT classique
godot-ui-pert-unidades_observadas = unités observées
godot-ui-pert-a_linha_amarela_e_o = ; la ligne jaune est le délai
godot-ui-pert-distribuicao_do_tempo_total = Distribution du temps total ({ $source }){ $marker }.
godot-ui-pert-pert_cpm_tempo_total = PERT/CPM ({ $mode }) : temps total { $total }.
godot-ui-pert-atividade = Activité
godot-ui-pert-duracao = Durée
godot-ui-pert-folga_total = Marge totale
godot-ui-pert-folga_livre = Marge libre
godot-ui-pert-criticidade = Criticité
godot-ui-pert-prazo = délai { $num }
godot-ui-pert-sem_distribuicao_para_este_modo = pas de distribution pour ce mode
godot-ui-pert-hipotese = hypothèse : { $h }
godot-ui-pert-fechar = Fermer
godot-ui-pert-fechar_a_janela_pert_cpm = Fermer la fenêtre PERT/CPM
godot-ui-pert-prazo-2 = Délai
godot-ui-pert-sem_prazo = sans délai
godot-ui-pert-sorteios_monte_carlo = Tirages (Monte Carlo)
godot-ui-pert-recalcular = Recalculer
godot-ui-pert-calcula_de_novo_no_modo = Recalcule dans le mode actuel
godot-ui-pert-relatorio_html = rapport .html
godot-ui-pert-exportar_html = Exporter en HTML
godot-ui-pert-grava_o_relatorio_html_do = Écrit le rapport HTML du PERT/CPM
godot-ui-pert-csv = CSV
godot-ui-pert-grava_as_tabelas_em_csv = Écrit les tableaux en CSV dans le dossier du fichier

## zone_edit.rs

godot-zone-edit-trafego_de_veiculos = Trafic de véhicules
godot-zone-edit-alcance_de_maquina = Portée de machine
godot-zone-edit-carga_suspensa = Charge suspendue
godot-zone-edit-ruido = Bruit
godot-zone-edit-calor = Chaleur
godot-zone-edit-outro = Autre
godot-zone-edit-a_zona_nao_existe = la zone '{ $name }' n'existe pas
godot-zone-edit-uma_zona_precisa_de_pelo = une zone a besoin d'au moins 3 points
godot-zone-edit-a_zona_precisa_de_um = la zone a besoin d'un nom
godot-zone-edit-tipo_de_zona_desconhecido = type de zone inconnu : { $value }
godot-zone-edit-nivel_um_numero_de_1 = niveau : un nombre de 1 à 5
godot-zone-edit-campo_de_zona_desconhecido = champ de zone inconnu : { $field }

## Tour 13 : boîtes de dialogue et messages d'erreur

godot-dialogo-descartar-titulo = Abandonner les modifications ?
godot-dialogo-descartar-texto = Le modèle a des modifications non enregistrées. Abandonner ?
godot-dialogo-descartar = Abandonner
godot-dialogo-voltar = Retour
godot-dialogo-atalhos = Raccourcis
godot-dialogo-fechar = Fermer
godot-dialogo-desempenho = Affiche ou masque les mesures de performance sur la vue 3D

godot-schema-distribuicao_invalida = distribution invalide : { $e }
godot-schema-familia_invalida = famille invalide
godot-schema-texto_ron_esperado = texte RON attendu
godot-schema-so_a_distribuicao_deslocada_tem = seule la distribution décalée a une distribution interne
godot-schema-a_distribuicao_nao_tem_o = la distribution n'a pas le paramètre "{ $key }"
godot-schema-modo_de_movimentacao_desconhecido = mode de déplacement inconnu : { $name }
godot-schema-modo_de_movimentacao_esperado = mode de déplacement attendu
godot-schema-a_velocidade_deve_ser_maior = la vitesse doit être supérieure à zéro
godot-schema-campo_desconhecido = champ inconnu : { $w }
godot-schema-rotulo_esperado = libellé attendu
godot-schema-o_rotulo_do_peso_nao = le libellé du poids ne peut pas rester vide
godot-schema-ordem_safe_sequential_ou_parallel = ordre : Safe, Sequential ou Parallel
godot-schema-crie_antes_uma_pessoa_ou = créez d'abord une personne (ou un répartiteur) pour utiliser le pont roulant
godot-schema-controle_cabin_pendant_ou_radio = contrôle : Cabin, Pendant ou Radio
godot-schema-escolha_a_equipe_do_operador = choisissez l'équipe de l'opérateur
godot-schema-escolha_antes_botoeira_ou_radio = choisissez d'abord commande suspendue ou radio
godot-schema-tipo_de_espera_table_pallet = type d'attente : Table, Pallet, Shelf ou Floor, pas { $k }
godot-schema-camadas_demais = trop de couches
godot-schema-espera_nao_tem_o_campo = attente n'a pas le champ "{ $rest }"
godot-schema-espera = attente : { $e }
godot-schema-sexo_female_male_ou_vazio = sexe : Female, Male ou vide, pas { $other }
godot-schema-idade_grande_demais = âge trop grand
godot-schema-pessoa_nao_tem_o_campo = personne n'a pas le champ "{ $field }"
godot-schema-pessoa = personne : { $e }
godot-schema-crie_antes_um_transportador_ou = créez d'abord un transporteur ou un répartiteur pour exploiter le processus
godot-schema-ligue_a_transferencia_antes = activez d'abord le transfert
godot-schema-caminho_straight_ou_network = chemin : Straight ou Network
godot-schema-ligue_as_quebras_antes = activez d'abord les pauses
godot-schema-ligue_os_turnos_antes = activez d'abord les postes
godot-schema-fim_do_turno_pause_ou = fin du poste : Pause ou Finish, pas { $t }
godot-schema-pausas_invalidas = pauses invalides : { $e }
godot-schema-escolha_um_calendario = choisissez un calendrier
godot-schema-ligue_o_lote_antes = activez d'abord le lot
godot-schema-ligue_antes = activez d'abord { $head }
godot-schema-ligue_os_operadores_antes = activez d'abord les opérateurs
godot-schema-o_ganho_da_equipe_deve = le gain de l'équipe doit être supérieur à zéro
godot-schema-objeto_nao_existe = l'objet "{ $object }" n'existe pas
godot-schema-posicao_x_y_ou_z = position : x, y ou z
godot-schema-politica_esperada = politique attendue
godot-schema-politica_de_acesso_desconhecida = politique d'accès inconnue : { $v }
godot-schema-o_rotulo_da_prioridade_nao = le libellé de la priorité ne peut pas rester vide
godot-schema-script_esperado = script attendu
godot-schema-regra_firstfree_random_ou_abc = règle : FirstFree, Random ou Abc
godot-schema-o_rotulo_nao_pode_ficar = le libellé ne peut pas rester vide
godot-schema-modo_unpack_ou_split = mode : Unpack ou Split
godot-schema-o_modelo_nao_tem_calendarios = le modèle n'a pas de calendriers (créez-en un dans "calendars")
godot-schema-origem_dos_turnos_inline_ou = origine des postes : inline ou calendar
godot-schema-texto_esperado = texte attendu
godot-schema-modo_esperado = mode attendu
godot-schema-perfil_brasil_ou_branco = profil : Brasil ou Branco
godot-schema-largura_da_faixa = largeur de la voie
godot-schema-parada_no_cruzamento = arrêt au carrefour
godot-schema-cor_invalida_use_rrggbb = couleur invalide : { $t } (utilisez #rrggbb)
godot-schema-cor_rrggbb = couleur : #rrggbb
godot-schema-run_nao_tem_o_campo = run n'a pas le champ "{ $path }"
godot-schema-regra_esperada = règle attendue
godot-schema-lista_de_pesos_esperada = liste de poids attendue
godot-schema-pesos_invalidos = poids invalides : { $e }
godot-schema-campo_desconhecido_output = champ inconnu : output.{ $rest }
godot-schema-a_escala_precisa_ser_maior = l'échelle doit être supérieure à zéro
godot-schema-giro_um_numero_de_graus = rotation : un nombre de degrés
godot-schema-campo_desconhecido_visual = champ inconnu : visual.{ $rest }

godot-ui-bar-casa = Maison

## Lot 14: messages et panneaux du GDScript (`UtsimDocument.texto`)

godot-gd-obstaculos-visiveis = Obstacles et dégagements visibles.
godot-gd-obstaculos-ocultos = Obstacles et dégagements masqués.
godot-gd-desempenho-visiveis = Mesures de performance visibles (F3).
godot-gd-desempenho-ocultas = Mesures de performance masquées (F3).
godot-gd-mao-dupla = double sens
godot-gd-mao-unica = sens unique
godot-gd-mao-unica-setas = sens unique (flèches au sol)
godot-gd-do-modelo = celui du modèle
godot-gd-foi-cancelada = a été annulée
godot-gd-falhou = a échoué
godot-gd-ligada = activée
godot-gd-desligada = désactivée
godot-gd-ligado = activé
godot-gd-desligado = désactivé
godot-gd-abrindo-leituras = Ouverture des lectures de { $p1 } dans UTSim Statistiques.
godot-gd-valores-lidos-de = Valeurs lues depuis { $p1 }.
godot-gd-nao-leu = Lecture impossible de { $p1 } : { $p2 }
godot-gd-objetos-copiados =
    { $p1 } { $p1 ->
        [one] objet copié
       *[other] objets copiés
    }.
godot-gd-nada-para-copiar = Rien de sélectionné à copier.
godot-gd-removida-desfaz = Supprimée : { $p1 } (Ctrl+Z annule)
godot-gd-nao-removeu-msg = Suppression impossible : { $p1 }
godot-gd-nao-ligou-msg = Liaison impossible : { $p1 }
godot-gd-passagem-cabem = , { $p1 } s de passage, { $p2 } peuvent tenir
godot-gd-esteira-desenhada = Convoyeur dessiné
godot-gd-esteira-com-canaletas = Convoyeur avec goulottes
godot-gd-caminho-no-piso = Chemin au sol
godot-gd-pedestres = Piétons
godot-gd-empilhadeiras = Chariots élévateurs
godot-gd-misto = Mixte
godot-gd-trajeto-inclinacao-pct = Pente (% ; négative = descente)
godot-gd-trajeto-acima-maximo = • Au-dessus du maximum : la pièce glisse ou bascule.
godot-gd-trajeto-rampa-ingreme = • Rampe plus raide que 1:12 (8,33 %).
godot-gd-esteira-simples = Convoyeur simple
godot-gd-motorizada = Motorisé
godot-gd-roletes-gravidade = Rouleaux par gravité
godot-gd-calha = Goulotte (glissement)
godot-gd-trajeto-caminho-n = Chemin { $p1 }
godot-gd-trajeto-esteira-n = Convoyeur { $p1 }
godot-gd-trajeto-no-n = Nœud { $p1 }

godot-gd-corredores-corredor_cancelado_clique_no_chao = Allée annulée. Cliquez sur le sol pour en commencer une autre, ou Esc pour quitter l'outil.
godot-gd-corredores-nao_criou_o_corredor = Allée non créée : { $p1 }
godot-gd-corredores-corredor_criado_m = Allée { $p1 } créée ({ $p2 }, { $p3 } m).
godot-gd-corredores-vertice_de_movido = Sommet { $p1 } de { $p2 } déplacé.
godot-gd-corredores-nao_moveu = Déplacement impossible : { $p1 }
godot-gd-corredores-vertice_acrescentado_a = Sommet ajouté à { $p1 }.
godot-gd-corredores-nao_acrescentou = Ajout impossible : { $p1 }
godot-gd-corredores-corredor_excluido = Allée { $p1 } supprimée.
godot-gd-corredores-nao_excluiu = Suppression impossible : { $p1 }
godot-gd-corredores-ponto_de_acesso_de_movido = Point d'accès de { $p1 } déplacé (désactivez "Point d'accès propre" dans le panneau pour revenir au réglage par défaut).
godot-gd-corredores-nao_moveu_o_ponto_de = Déplacement du point d'accès impossible : { $p1 }
godot-gd-corredores-corredor_agora_e_de = L'allée { $p1 } est maintenant { $p2 }.
godot-gd-corredores-corredor_com_m_de_largura = Allée { $p1 } de { $p2 } m de largeur.
godot-gd-corredores-nao_mudou_a_largura = Modification de la largeur impossible : { $p1 }
godot-gd-corredores-corredor_em = Allée { $p1 } en { $p2 }.
godot-gd-corredores-perfil_do_corredor = Profil de l'allée { $p1 } : { $p2 }.
godot-gd-corredores-sentido_do_corredor_invertido = Sens de l'allée { $p1 } inversé.
godot-gd-corredores-corredor_agora_se_chama = L'allée { $p1 } s'appelle maintenant { $p2 }.
godot-gd-corredores-nao_renomeou = Renommage impossible : { $p1 }
godot-gd-corredores-vertice_removido = Sommet supprimé.
godot-gd-corredores-nao_removeu = Suppression impossible : { $p1 }
godot-gd-corredores-esse_aviso_nao_aponta_para = Cet avertissement ne pointe vers aucune allée ni aucun objet de la scène.
godot-gd-corredores-destacado_na_cena = Mis en évidence dans la scène : { $p1 }.
godot-gd-editor-equipe_arraste_do_despachante_a = Équipe : faites glisser du répartiteur vers un transporteur, ou d'un objet vers ce qui lui apporte les articles (Shift : d'un processus vers ce qui fournit les opérateurs) ; recommencer annule. Plus simple : faites glisser du nœud orange de l'opérateur jusqu'à la machine. Esc annule.
godot-gd-editor-ligar_arraste_do_bloco_de = Relier : faites glisser du bloc d'origine jusqu'à la destination, ou cliquez sur l'un puis sur l'autre (Esc annule).
godot-gd-editor-clique_no_chao_para_colocar = Cliquez sur le sol pour placer (Shift garde l'outil, Esc annule).
godot-gd-editor-corredor_clique_no_chao_para = Allée : cliquez sur le sol pour poser les points de l'axe (Enter ou double-clic termine), cliquez sur une allée pour la modifier, faites glisser un sommet pour le déplacer ; Esc annule.
godot-gd-editor-zona_clique_no_chao_para = Zone : cliquez sur le sol pour poser les coins de la zone de risque ; le premier point, Enter ou double-clic la ferme ; cliquez sur une zone pour la modifier. Esc annule.
godot-gd-editor-rede_clique_no_chao_para = Réseau : cliquez sur le sol pour poser des nœuds (chaque clic prolonge le chemin), cliquez sur un tronçon pour le modifier, faites glisser un nœud pour le déplacer ; Esc termine le chemin.
godot-gd-editor-nao_mudou_a_zona = Modification de la zone impossible : { $p1 }
godot-gd-editor-nao_ligou = Liaison impossible : { $p1 }
godot-gd-editor-nao_removeu = Suppression impossible : { $p1 }
godot-gd-editor-nao_girou = Rotation impossible : { $p1 }
godot-gd-editor-nao_criou_a_zona = Création de la zone impossible : { $p1 }
godot-gd-editor-zona_criada_escolha_o_tipo = Zone { $p1 } créée : choisissez le type, le niveau et l'EPI dans le panneau (Ctrl+Z annule).
godot-gd-editor-zona_excluida_ctrl_z_desfaz = Zone { $p1 } supprimée (Ctrl+Z annule).
godot-gd-editor-nao_excluiu_a_zona = Suppression de la zone impossible : { $p1 }
godot-gd-editor-objeto_s_excluidos =
    { $p1 } { $p1 ->
        [one] objet supprimé
       *[other] objets supprimés
    }.
godot-gd-editor-colocacao_cancelada_solte_o_objeto = Placement annulé : déposez l'objet sur le sol.
godot-gd-editor-aponte_para_o_chao_para = Pointez vers le sol pour placer l'objet.
godot-gd-editor-nao_foi_possivel_colocar = Placement impossible : { $p1 }
godot-gd-editor-colocado = { $p1 } placé.
godot-gd-editor-equipe_cancelado = Équipe : annulé.
godot-gd-editor-nada_selecionado_para_girar = Rien de sélectionné à faire pivoter.
godot-gd-editor-girado_ctrl_z_desfaz = { $p1 } pivoté (Ctrl+Z annule).
godot-gd-editor-objetos_girados_cada_um_em = { $p1 } objets pivotés de { $p2 }, chacun autour de son propre centre (Ctrl+Z annule).
godot-gd-editor-nenhum_corredor_selecionado_para_excluir = Aucune allée sélectionnée à supprimer.
godot-gd-editor-nenhum_no_selecionado_para_remover = Aucun nœud sélectionné à supprimer.
godot-gd-principal-arraste_um_objeto_da_biblioteca = Faites glisser un objet de la Bibliothèque vers le sol, ou ouvrez un modèle (Ctrl+O).
godot-gd-principal-encaixe_na_grade_passo_m = Accrochage à la grille { $p1 } (pas { $p2 } m).
godot-gd-principal-o_modo_missao_do_utsim = Le mode mission d'UTSim Ensino n'est pas dans cette version.
godot-gd-principal-para_andar_pela_fabrica_simule = Pour parcourir l'usine, simulez d'abord (F dans Simuler, après la série).
godot-gd-principal-alterado = { $p1 } : { $p2 } modifié.
godot-gd-principal-nao_alterou = Modification impossible de { $p1 } : { $p2 }
godot-gd-principal-novo_modelo = Nouveau modèle.
godot-gd-principal-nao_abriu = Ouverture impossible de { $p1 } : { $p2 }
godot-gd-principal-aberto_mas_salvar_fica_para = { $p1 } ouvert, mais { $p2 } : l'enregistrement est reporté à la fermeture (ou utilisez Enregistrer sous).
godot-gd-principal-aberto_objetos_simular_roda_as = { $p1 } ouvert : { $p2 } objets. Simuler lance les réplications.
godot-gd-principal-aberto_com_problema_s_veja =
    { $p1 } ouvert avec { $p2 } { $p2 ->
        [one] problème
       *[other] problèmes
    } : voir le panneau des propriétés.
godot-gd-principal-o_outro_app_fechou_o = L'autre application a fermé le modèle : les onglets Qualité, Sécurité et Maintenance peuvent de nouveau modifier.
godot-gd-principal-outro_app_gravou_e_ha = Une autre application a enregistré { $p1 } et vous avez des modifications non enregistrées : utilisez Enregistrer sous pour ne pas effacer son travail.
godot-gd-principal-nao_releu = Relecture impossible de { $p1 } : { $p2 }
godot-gd-principal-foi_gravado_por_outro_app = { $p1 } a été enregistré par une autre application : rechargé depuis le disque.
godot-gd-principal-salve_o_modelo_antes_de = Enregistrez le modèle avant d'ouvrir dans { $p1 }.
godot-gd-principal-abrindo_no_ele_edita_o = Ouverture de { $p1 } dans { $p2 } : il modifie le modèle ; UTSim 3D relit ce qu'il enregistre.
godot-gd-principal-salvo_em = Enregistré dans { $p1 }.
godot-gd-principal-nao_salvou = Enregistrement impossible : { $p1 }
godot-gd-principal-desfeito = Annulé.
godot-gd-principal-nada_para_desfazer = Rien à annuler.
godot-gd-principal-refeito = Rétabli.
godot-gd-principal-nada_para_refazer = Rien à rétablir.
godot-gd-principal-nao_foi_possivel_desenhar_o = Impossible de dessiner le modèle : { $p1 }
godot-gd-principal-escolha_o_arquivo_de_dados = Choisissez le fichier de données.
godot-gd-principal-valores_colados = Valeurs collées : { $p1 }
godot-gd-principal-aguarde = Patientez : { $p1 }.
godot-gd-principal-nao_ajustou = Ajustement impossible : { $p1 }
godot-gd-principal-agora_usa_dados_reais_ctrl = { $p1 } utilise maintenant des données réelles (Ctrl+Z annule).
godot-gd-principal-nao_aplicou_os_dados = Application des données impossible : { $p1 }
godot-gd-principal-volta_a_usar_uma_distribuicao = { $p1 } utilise de nouveau une distribution (Ctrl+Z annule).
godot-gd-principal-nao_voltou_para_a_distribuicao = Retour à la distribution impossible : { $p1 }
godot-gd-principal-sem_relatorio = Pas de rapport : { $p1 }
godot-gd-principal-relatorio_gravado_em = Rapport enregistré dans { $p1 }.
godot-gd-principal-relatorio_gravado_em_arquivos = Rapport enregistré dans { $p1 } ({ $p2 } fichiers).
godot-gd-principal-objetos_alinhados = { $p1 } objets alignés.
godot-gd-principal-objetos_distribuidos = { $p1 } objets répartis.
godot-gd-principal-objeto_s_recortados_ctrl_v =
    { $p1 } { $p1 ->
        [one] objet coupé
       *[other] objets coupés
    } (Ctrl+V colle).
godot-gd-principal-nada_selecionado_para_recortar = Rien de sélectionné à couper.
godot-gd-principal-objeto_s_colados =
    { $p1 } { $p1 ->
        [one] objet collé
       *[other] objets collés
    } : { $p2 }.
godot-gd-principal-o_modelo_tem_problema_s =
    Le modèle a { $p1 } { $p1 ->
        [one] problème et
       *[other] problèmes et
    } ne tourne pas : { $p2 }
godot-gd-principal-a_simulacao_nao_comecou = La simulation n'a pas démarré : { $p1 }
godot-gd-principal-a_simulacao = La simulation { $p1 } : { $p2 }
godot-gd-principal-nao_foi_possivel_gravar_a = Impossible d'enregistrer la réplication 0 : { $p1 }
godot-gd-principal-replicacoes_concluidas_reproduzindo_a_replicacao = { $p1 } réplications terminées. Lecture de la réplication 0.
godot-gd-principal-texto = { $p1 }...
godot-gd-principal-cancelando = Annulation : { $p1 }...
godot-gd-principal-simule_o_modelo_para_ver = Simulez le modèle pour voir les résultats (Tab ou le bouton Simuler).
godot-gd-principal-os_fluxos_de_tokens_aparecem = Les flux de jetons apparaissent dans Simuler.
godot-gd-principal-selecione_os_objetos_da_parte = Sélectionnez les objets de la partie du processus (ou deux : le début et la fin) et demandez le PERT/CPM.
godot-gd-principal-pert_cpm_de_ate_selecione = PERT/CPM de... à... : sélectionnez exactement deux objets.
godot-gd-principal-o_modelo_tem_problemas_e = Le modèle a des problèmes et ne tourne pas : le PERT/CPM exige un modèle valide.
godot-gd-principal-nao_comecou = Démarrage impossible : { $p1 }
godot-gd-principal-um_replay_gravado_nao_roda = Un replay enregistré ne se relance pas : ouvrez le modèle.
godot-gd-principal-nao_ligou_a_serie_e = Liaison impossible de la série et des histogrammes : { $p1 }
godot-gd-principal-ligado_no_modelo_ctrl_z = Lié au modèle ({ $p1 } ; Ctrl+Z annule). Nouvelle simulation...
godot-gd-principal-aquecimento_aplicado_ao_modelo_run = Échauffement { $p1 } appliqué au modèle (run.warmup). Simulez de nouveau pour obtenir les chiffres avec lui.
godot-gd-principal-nao_aplicou_o_aquecimento = Application de l'échauffement impossible : { $p1 }
godot-gd-principal-nao_abriu_pedido_na_linha = Ouverture impossible de { $p1 } (demandé en ligne de commande).
godot-gd-principal-nao_foi_possivel_reproduzir = Impossible de lire { $p1 } : { $p2 }
godot-gd-principal-replay_aberto_blocos_so_reproducao = Replay { $p1 } ouvert ({ $p2 } blocs) : lecture seule ; Éditer revient au modèle en édition.
godot-gd-principal-simule_um_modelo_antes_de = Simulez un modèle avant d'enregistrer le replay.
godot-gd-principal-replay_da_replicacao_0_gravado = Replay de la réplication 0 enregistré dans { $p1 }.
godot-gd-principal-nao_gravou_o_replay = Enregistrement du replay impossible : { $p1 }
godot-gd-principal-camera_de_volta = Caméra rétablie.
godot-gd-rede-caminho_encerrado_clique_no_chao = Chemin terminé. Cliquez sur le sol pour en commencer un autre, ou Esc pour quitter l'outil.
godot-gd-rede-nao_colocou_o_no = Placement du nœud impossible : { $p1 }
godot-gd-rede-no_ligado_a_clique_de = Nœud { $p1 } relié à { $p2 }. Cliquez de nouveau pour continuer le chemin (Esc pour arrêter).
godot-gd-rede-no_colocado_clique_em_outro = Nœud { $p1 } placé. Cliquez sur un autre point pour tracer un tronçon (Esc pour arrêter).
godot-gd-rede-no_movido = Nœud { $p1 } déplacé.
godot-gd-rede-nao_moveu = Déplacement impossible : { $p1 }
godot-gd-rede-trecho_criado = Tronçon { $p1 } → { $p2 } créé ({ $p3 }).
godot-gd-rede-nao_ligou = Liaison impossible : { $p1 }
godot-gd-rede-trecho_removido = Tronçon { $p1 } → { $p2 } supprimé.
godot-gd-rede-no_removido_com_os_trechos = Nœud { $p1 } supprimé avec ses tronçons.
godot-gd-rede-trecho_em = Tronçon { $p1 } → { $p2 } en { $p3 }.
godot-gd-rede-pista_controlada_ate_veiculo_s =
    Voie contrôlée : jusqu’à { $p1 } { $p1 ->
        [one] véhicule
       *[other] véhicules
    } en même temps sur le tronçon.
godot-gd-rede-trecho_sem_controle_de_trafego = Tronçon sans contrôle de trafic.
godot-gd-rede-trecho_invertido = Tronçon inversé : { $p1 } → { $p2 }.
godot-gd-rede-no_agora_se_chama = Le nœud { $p1 } s'appelle maintenant { $p2 }.
godot-gd-rede-nao_renomeou = Renommage impossible : { $p1 }
godot-gd-trajeto-clique_no_chao_para_por = { $p1 } : cliquez sur le sol pour poser les nœuds (Enter ou double-clic termine). PageUp/PageDown monte et descend le prochain nœud, [ et ] arrondissent la courbe. Cliquez sur un nœud pour le modifier ; Esc annule.
godot-gd-trajeto-arraste_em_volta_do_meio = Faites glisser autour du milieu pour faire pivoter { $p1 } (de 15 en 15° ; Shift : de 1 en 1°).
godot-gd-trajeto-arraste_para_mover_inteiro_r = Faites glisser pour déplacer { $p1 } entier ; R et Shift+R pivotent de 15°.
godot-gd-trajeto-arraste_para_curvar_o_trecho = Faites glisser pour courber le tronçon { $p1 } de { $p2 }.
godot-gd-trajeto-no_acrescentado_em_o_trecho = Nœud ajouté à { $p1 } (le tronçon a été divisé).
godot-gd-trajeto-nao_acrescentou = Ajout impossible : { $p1 }
godot-gd-trajeto-aponte_para_o_chao_para = Pointez vers le sol pour poser le nœud.
godot-gd-trajeto-no_em_m_altura_m = { $p1 } : nœud { $p2 } en ({ $p3 } ; { $p4 }) m, hauteur { $p5 } m ({ $p6 }).
godot-gd-trajeto-comeca_em_m_altura_m = { $p1 } : commence en ({ $p2 } ; { $p3 }) m, hauteur { $p4 } m. Cliquez les nœuds suivants ; Enter termine.
godot-gd-trajeto-trecho_de_selecionado_apagar_o = Tronçon { $p1 } de { $p2 } sélectionné : "Supprimer le tronçon entre les nœuds" dans le panneau ; faites glisser le losange pour le courber.
godot-gd-trajeto-movido_m_em_x_e = { $p1 } déplacé de { $p2 } m en X et de { $p3 } m en Y (Ctrl+Z annule).
godot-gd-trajeto-nao_moveu = Déplacement impossible : { $p1 }
godot-gd-trajeto-trecho_curvado_no_com_raio = Tronçon courbé : nœud { $p1 } avec un rayon de { $p2 } m (Ctrl+Z annule).
godot-gd-trajeto-nao_curvou = Courbure impossible : { $p1 }
godot-gd-trajeto-comprimento_do_trecho_m_enter = Longueur du tronçon : { $p1 } m (Enter pose le nœud dans la direction du curseur, Backspace efface).
godot-gd-trajeto-proximo_no_a_m_de = Prochain nœud à { $p1 } m de hauteur.
godot-gd-trajeto-raio_das_proximas_curvas_m = Rayon des prochaines courbes : { $p1 } m.
godot-gd-trajeto-descendo_por_m_o_no = En descendant de { $p1 } % sur { $p2 } m, le nœud serait sous le sol.
godot-gd-trajeto-o_trecho_de_m_nao = Le tronçon de { $p1 } m n'atteint pas la hauteur de { $p2 } m.
godot-gd-trajeto-no_a_m_do_anterior = { $p1 } : nœud { $p2 } à { $p3 } m du précédent.
godot-gd-trajeto-girado_r_gira_de_novo = { $p1 } pivoté de { $p2 }° (R pivote de nouveau ; Shift+R dans l'autre sens).
godot-gd-trajeto-nao_girou = Rotation impossible : { $p1 }
godot-gd-trajeto-ultimo_no_desfeito = Dernier nœud annulé.
godot-gd-trajeto-nao_criou = Création impossible : { $p1 }
godot-gd-trajeto-criado_m = { $p1 } { $p2 } créé : { $p3 } m{ $p4 }.
godot-gd-trajeto-desenho_cancelado_clique_no_chao = Dessin annulé. Cliquez sur le sol pour en commencer un autre, ou Esc pour quitter l'outil.
godot-gd-trajeto-no_s_removido_s =
    { $p1 } { $p1 ->
        [one] nœud supprimé
       *[other] nœuds supprimés
    }.
godot-gd-trajeto-nao_removeu = Suppression impossible : { $p1 }
godot-gd-trajeto-no_x_y_altura_m = { $p1 }, nœud { $p2 } : X { $p3 }, Y { $p4 }, hauteur { $p5 } m. Les flèches et PageUp/PageDown ajustent (Shift : 1 cm).
godot-gd-trajeto-nao_e_uma_esteira_desenhada = { $p1 } n'est pas un convoyeur dessiné.
godot-gd-trajeto-esteira_clique_num_no_para = Convoyeur { $p1 } : cliquez sur un nœud pour le modifier ; les flèches et PageUp/PageDown déplacent les nœuds sélectionnés.
godot-gd-trajeto-inclinacao_fixa = Pente fixe { $p1 } ({ $p2 } %).
godot-gd-trajeto-esteira_dividida = Convoyeur divisé : { $p1 } → { $p2 }.
godot-gd-trajeto-trecho_apagado_de = Tronçon supprimé de { $p1 }.
godot-gd-trajeto-e_viraram_um_trajeto_so = { $p1 } et { $p2 } ne forment plus qu'un seul trajet (Ctrl+Z annule).
godot-gd-trajeto-agora_corre_no_sentido_contrario = { $p1 } circule maintenant en sens inverse (les liaisons aussi).
godot-gd-trajeto-desenhado_a_m_de = { $p1 } dessiné à { $p2 } m de { $p3 }.
godot-gd-trajeto-rampa_s_de_acesso_de =
    { $p1 } { $p1 ->
        [one] rampe d’accès 1:12 ajoutée
       *[other] rampes d’accès 1:12 ajoutées
    } à { $p2 }.
godot-gd-trajeto-nao_mudou = Modification impossible : { $p1 }
godot-gd-trajeto-no_a_m_de_altura = { $p1 }, nœud { $p2 } à { $p3 } m de hauteur (profil ; Ctrl+Z annule).
godot-gd-trajeto-nao_mudou_a_altura = Modification de la hauteur impossible : { $p1 }

godot-gd-trajeto-clique_no_chao_para_por-2 = Cliquez sur le sol pour poser les nœuds ; le tronçon affiche la longueur et la pente. Les nœuds s'accrochent aux portes (extrémités des convoyeurs et objets), aux nœuds d'autres trajets, de 15 en 15° et à la grille. PageUp/PageDown : hauteur du prochain nœud ; [ et ] : rayon de la courbe ; Enter termine ; Backspace revient d'un nœud ; Esc annule.
godot-gd-trajeto-altura_do_proximo_no_m = Hauteur du prochain nœud (m)
godot-gd-trajeto-inclinacao_fixa_no_proximo_trecho = Pente fixe sur le prochain tronçon
godot-gd-trajeto-raio_das_curvas_m_0 = Rayon des courbes (m ; 0 = angle vif)
godot-gd-trajeto-largura_do_caminho_m = Largeur du chemin (m)
godot-gd-trajeto-esteira_s_desenhada_s_caminho =
    { $p1 } { $p1 ->
        [one] convoyeur dessiné
       *[other] convoyeurs dessinés
    }, { $p2 } { $p2 ->
        [one] chemin
       *[other] chemins
    }.
godot-gd-trajeto-maior_inclinacao_maximo = Pente maximale { $p1 }° (maximum { $p2 }°).
godot-gd-trajeto-girar_graus_anti_horario = Pivoter (degrés ; + sens antihoraire)
godot-gd-trajeto-girar_o_trajeto = Faire pivoter le trajet
godot-gd-trajeto-girar_15_anti_horario_r = Pivoter de 15° (antihoraire, R)
godot-gd-trajeto-girar_15_horario_shift_r = Pivoter de 15° (horaire, Shift+R)
godot-gd-trajeto-juntar_com_num_trajeto_so = Fusionner avec { $p1 } en un seul trajet
godot-gd-trajeto-paralelo_a_m_a_esquerda = Parallèle à (m ; + à gauche)
godot-gd-trajeto-desenhar_paralelo = Dessiner un parallèle
godot-gd-trajeto-escada_degraus_so_pedestres = Escalier (marches ; piétons seulement)
godot-gd-trajeto-rampas_de_acesso_nas_pontas = Rampes d'accès aux extrémités (1:12)
godot-gd-trajeto-tipo = Type
godot-gd-trajeto-acionamento = Actionnement
godot-gd-trajeto-inverter_o_sentido = Inverser le sens
godot-gd-trajeto-largura_m = Largeur (m)
godot-gd-trajeto-altura_m = Hauteur (m)
godot-gd-trajeto-raio_da_curva_m_0 = Rayon de la courbe (m ; 0 = angle vif)
godot-gd-trajeto-dividir_a_esteira_neste_no = Diviser le convoyeur à ce nœud
godot-gd-trajeto-nos_selecionados = { $p1 } nœuds sélectionnés
godot-gd-trajeto-nivelar_na_altura_m = Mettre à niveau à la hauteur (m)
godot-gd-trajeto-inclinacao_constante_entre_as_pontas = Pente constante entre les extrémités
godot-gd-trajeto-remover_no_s_del = Supprimer le(s) nœud(s) (Del)
godot-gd-trajeto-apagar_o_trecho_entre_os = Supprimer le tronçon entre les nœuds
godot-gd-trajeto-arraste_o_losango_do_no = Faites glisser le losange du nœud au sol pour le déplacer et le haut de la tige pour changer la hauteur. Les flèches déplacent (Shift : 1 cm), PageUp/PageDown montent et descendent, [ et ] changent le rayon. Shift + clic sur la ligne ajoute un nœud ; un clic sur le losange orange (ou sur deux nœuds voisins avec Shift + clic) sélectionne le tronçon, qui peut être supprimé. Faites glisser le losange orange au milieu d'un tronçon pour le courber, le carré vert au milieu du trajet pour le déplacer entier et l'anneau orange à côté pour le faire pivoter ; sans nœud sélectionné, les flèches déplacent et R fait pivoter le trajet.
godot-gd-principal-o_modelo_tem_mudancas_que = Le modèle a des modifications non enregistrées. Abandonner ?

## Lot 14: fenêtres et erreurs restantes

godot-ui-class-sem_replicacao = sans réplication
godot-ui-class-sem_rastro_da_replicacao = sans trace de la réplication
godot-ui-class-a_janela_de_valores_nao = la fenêtre des valeurs n'est pas ouverte
godot-ui-class-valor_es_lidos_de_coluna =
    { $n } { $n ->
        [one] valeur lue
       *[other] valeurs lues
    } depuis { $file } (colonne { $column }).
godot-ui-class-nao_leu_o_arquivo = Lecture du fichier impossible : { $e }
godot-ui-class-relatorio_gravado_em = Rapport enregistré dans { $f }.
godot-ui-class-escreva_o_arquivo_do_relatorio = Saisissez le fichier du rapport.
godot-ui-class-relatorio_pert_cpm_gravado_em = Rapport PERT/CPM enregistré dans { $path }.
godot-ui-class-nao_gravou = Enregistrement impossible de { $path } : { $e }
godot-ui-class-tabela_s_csv_em =
    { $files } { $files ->
        [one] table
       *[other] tables
    } CSV dans { $dir }.
godot-ui-class-nao_gravou_os_csv_em = Enregistrement impossible des CSV dans { $dir } : { $e }
godot-ui-class-escreva_a_pasta_da_exportacao = Saisissez le dossier d'exportation.
godot-ui-class-arquivo_s_csv_em =
    { $files } { $files ->
        [one] fichier
       *[other] fichiers
    } CSV dans { $dir }.
godot-ui-class-nao_exportou = Exportation impossible : { $e }
godot-ui-class-so_leitura = { $quem } : lecture seule
godot-ui-class-na_ultima_simulacao_media_semiamplitude =
    Dans la dernière simulation ({ $replicacoes } { $replicacoes ->
        [one] réplication
       *[other] réplications
    }), moyenne ± demi-amplitude de l'IC à 95 % :
godot-schema-inteiro_maior_ou_igual_a_zero = { $key } doit être un entier >= 0
godot-schema-numero_maior_ou_igual_a_zero = { $w } : nombre supérieur ou égal à zéro
godot-schema-nao_tem_o_campo = { $name } n'a pas le champ "{ $field }"
godot-schema-campo_maior_que_zero = { $field } : doit être supérieur à zéro
godot-schema-inteiro_minimo = { $what } doit être un entier >= { $min }
godot-schema-numero_esperado_de = { $what } : nombre attendu
godot-schema-verdadeiro_ou_falso_esperado = { $what } : vrai ou faux attendu

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

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

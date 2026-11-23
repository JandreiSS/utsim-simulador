# Traduction automatique, en attente de relecture humaine.
# Tradução automática, aguardando revisão humana.
# UTSim : l'éditeur du simulateur (base : barre, calques, panneaux et dialogues généraux).

ui-sem_observacoes = aucune observation
ui-resumo = n={ $n }  moyenne={ $media }  é-t={ $dp }  min={ $min }  max={ $max }
ui-recebidos = reçus : { $n }
ui-simulacao = Simulation
ui-modelo_replicacao = { $modelo } - réplication { $n }
ui-contagem = { $blocos ->
    [one] { $blocos } bloc
   *[other] { $blocos } blocs
}, { $itens ->
    [one] { $itens } article
   *[other] { $itens } articles
}, { $eventos ->
    [one] { $eventos } événement de trace
   *[other] { $eventos } événements de trace
}
ui-pausar = Pause
ui-reproduzir = Lecture
ui-evento_anterior = < Événement
ui-evento_seguinte = Événement >
ui-inicio = Début
ui-velocidade = vitesse x{ $v }
ui-unidades_por_segundo = unités de temps simulé par seconde
ui-tempo = temps
ui-tempo_simulado = temps simulé : { $t } sur { $fim }
ui-ajuda_teclas = Espace : lecture/pause | +/- : vitesse | flèches : événement précédent/suivant | F : cadrer
ui-ajuda_mouse = Souris : le gauche fait glisser (déplace) et clique (sélectionne), le droit fait pivoter, la molette zoome
ui-ajuda_camera = Caméra : 1 orbite, 2 suivre, 3 première personne, 4 depuis la ressource | F3 : temps d'image
ui-blocos = Blocs
ui-clique_em_um_bloco = Cliquez sur un bloc pour voir ses statistiques.
ui-capacidade_rotulo = capacité { $n }
ui-nivel_agora = niveau actuel
ui-capacidade = capacité
ui-ocupacao = occupation
ui-taxa_agora = taux actuel
ui-nivel_inicial = niveau initial
ui-entrada_maxima = entrée maximale
ui-saida_maxima = sortie maximale
ui-vazao_da_fonte = débit de la source
ui-volume_por_item = volume par article
ui-acumulado = accumulé pour l'article suivant
ui-vazao_agora = débit actuel
ui-vazao_maxima = débit maximal
ui-objeto = objet
ui-por_unidade_de_tempo = { $taxa } par unité de temps ({ $sentido })
ui-nivel_interpolado = Niveau et taux interpolés à partir de la trace du réseau de fluides.
ui-seguir_recurso = Suivre cette ressource
ui-seguir_recurso_dica = La caméra suit la ressource par derrière et par-dessus (2)
ui-ver_do_recurso = Voir depuis la ressource
ui-ver_do_recurso_dica = La caméra se place à la position du conducteur (4)
ui-recebidos_ate_agora = reçus jusqu'ici
ui-estado_agora = état actuel
ui-conteudo_agora = contenu actuel
ui-entradas = entrées
ui-saidas = sorties
ui-conteudo_medio = contenu moyen
ui-conteudo_maximo = contenu maximal
ui-tempo_de_permanencia = temps de séjour
ui-fracao_em = fraction en { $estado }
ui-estatisticas_finais = Statistiques finales de la réplication (fenêtre après la mise en régime).
camadas-no_excluido = Nœud { $nome } et ses arêtes supprimés (Ctrl+Z annule)
camadas-aresta_excluida = Arête { $de } - { $para } supprimée (Ctrl+Z annule)
camadas-no_fluido_excluido = Nœud { $nome } et ses liaisons supprimés (Ctrl+Z annule)
camadas-objetos_dica = Objets du modèle : bibliothèque, liaisons et propriétés
camadas-rede_dica = Réseau de chemins des ressources mobiles : nœuds et arêtes
camadas-fluidos_dica = Réseau de fluides : sources, réservoirs, tuyaux et convertisseurs
camada-objetos = Objets
camada-rede = Réseau
camada-fluidos = Fluides
camadas-rede_de_caminhos = Réseau de chemins
camadas-selecionar = Sélectionner
camadas-selecionar_rede_dica = Un clic sélectionne un nœud ou une arête ; glisser déplace le nœud
camadas-no = Nœud
camadas-no_dica = Un clic au sol place un nœud
camadas-aresta = Arête
camadas-aresta_dica = Cliquez sur un nœud puis sur un autre ; un clic au sol crée un nœud déjà relié
camadas-mao_dupla = Nouvelles arêtes à double sens
camadas-ajuda_aresta = Arête : cliquez de nœud en nœud pour tracer le chemin ; Échap termine.
camadas-ajuda_del_rede = Suppr supprime le nœud (avec ses arêtes) ou l'arête sélectionnée.
camadas-ajuda_atendido = Chaque objet desservi entre dans le réseau par le nœud le plus proche (ligne pointillée).
camadas-rede_de_fluidos = Réseau de fluides
camadas-selecionar_fluidos_dica = Un clic sélectionne un nœud ; glisser déplace le nœud
camadas-ligar = Relier
camadas-ligar_dica = Cliquez sur le nœud d'origine puis sur celui de destination (le débit suit la flèche)
camadas-colocar = Placer
camadas-colocar_dica = Un clic au sol place le nœud (Maj garde l'outil)
camadas-ajuda_aciclica = Le réseau doit être acyclique. Les convertisseurs relient le réseau à un objet.
camadas-ajuda_del_fluidos = Suppr supprime le nœud sélectionné et ses liaisons.
camadas-aresta_daqui = Arête d'ici
camadas-ligar_daqui = Relier d'ici
camadas-sem_rede = Le modèle n'a pas de réseau de chemins.
camadas-sem_rede_ajuda = Sans réseau, les ressources mobiles se déplacent en ligne droite. Utilisez « Nœud » pour placer les nœuds et « Arête » pour les relier.
camadas-resumo_rede = { $nos ->
    [one] { $nos } nœud
   *[other] { $nos } nœuds
} et { $arestas ->
    [one] { $arestas } arête
   *[other] { $arestas } arêtes
}. Sélectionnez-en un pour le modifier.
camadas-nos = Nœuds ({ $n })
camadas-arestas = Arêtes ({ $n })
camadas-sem_fluidos = Le modèle n'a pas de réseau de fluides.
camadas-sem_fluidos_ajuda = Choisissez un type dans « Placer » et cliquez au sol ; puis utilisez « Relier ».
camadas-resumo_fluidos = Réseau « { $rede } » : { $nos ->
    [one] { $nos } nœud
   *[other] { $nos } nœuds
} et { $ligacoes ->
    [one] { $ligacoes } liaison
   *[other] { $ligacoes } liaisons
}. Sélectionnez un nœud pour le modifier.
editor-layout_aplicado = Disposition automatique appliquée : enregistrez pour garder les positions
editor-layout_com_rede = Modèle avec réseau de chemins : placez les objets sur les nœuds du réseau (la disposition automatique n'a pas été appliquée)
editor-sem_titulo = sans titre.ron
editor-excluidos = { $n } objets supprimés (Ctrl+Z annule)
editor-nada_para_copiar = Rien de sélectionné à copier
editor-copiados = { $objetos ->
    [one] { $objetos } objet
   *[other] { $objetos } objets
} et { $ligacoes ->
    [one] { $ligacoes } liaison copiés
   *[other] { $ligacoes } liaisons copiés
}
editor-recortados = { $n ->
    [one] { $n } objet coupé (Ctrl+V colle)
   *[other] { $n } objets coupés (Ctrl+V colle)
}
editor-area_vazia = Le presse-papiers est vide (Ctrl+C copie)
editor-colados = { $n ->
    [one] { $n } objet collé : { $nomes }
   *[other] { $n } objets collés : { $nomes }
}
editor-nada_para_duplicar = Rien de sélectionné à dupliquer
editor-salvo_em = Enregistré dans { $arquivo }
editor-erro_ao_salvar = Erreur d'enregistrement : { $erro }
editor-tempo_invalido = Temps maximal invalide : utilisez un nombre supérieur à zéro
editor-simulando = Simulation en cours...
editor-sem_resultado = la simulation s'est terminée sans résultat
editor-erro_na_simulacao = Erreur de simulation : { $erro }
editor-na_caixa = { $n ->
    [one] { $n } objet dans la boîte
   *[other] { $n } objets dans la boîte
}, { $sel ->
    [one] { $sel } sélectionné
   *[other] { $sel } sélectionnés
}
editor-outro_destino = Choisissez un autre bloc comme destination
editor-ligacao_removida = Liaison { $de } -> { $para } supprimée (Ctrl+Z annule)
editor-editar = Modifier
editor-simular = ▶ Simuler
editor-ate_t = jusqu'à t =
editor-novo = Nouveau
editor-salvar = Enregistrer
editor-salvar_como_botao = Enregistrer sous...
editor-desfazer = Annuler
editor-refazer = Rétablir
editor-enquadrar = Cadrer (F)
editor-fluxos = Flux
editor-fluxos_dica = Logique de l'utilisateur : flux de jetons
editor-resultados = Résultats
editor-resultados_dica = Réplications, résumé avec intervalles de confiance, graphiques, scénarios et optimisation
editor-atalhos = Raccourcis
editor-atalhos_dica = Raccourcis clavier et souris (F1)
editor-salvar_como = Enregistrer sous
editor-caminho_do_arquivo = Chemin du fichier (.ron), relatif au dossier du programme ou complet :
editor-sobrescrito = Ce fichier existe déjà et sera écrasé.
editor-cancelar = Annuler
editor-novo_modelo = Nouveau modèle
editor-descartar_pergunta = Il y a des modifications non enregistrées. Les abandonner et commencer un nouveau modèle ?
editor-descartar = Abandonner
editor-atalhos_do_editor = Raccourcis de l'éditeur
editor-f1_fecha = F1 ou Échap ferme cette fenêtre.
editor-nao_simula = Impossible de simuler
editor-corrija = Corrigez les problèmes du modèle avant de simuler :
editor-fechar = Fermer
editor-valores_gravados = Valeurs écrites dans le modèle (Ctrl+Z annule)
editor-nenhum_problema = Aucun problème : le modèle peut être simulé.
editor-modelo = Modèle
editor-biblioteca = Bibliothèque
editor-propriedades = Propriétés
editor-selecione_um_bloco = Sélectionnez un bloc pour modifier ses propriétés.
editor-ligar = Relier (C)
editor-excluir = Supprimer (Suppr)
editor-clique_no_destino = Cliquez sur le bloc de destination (Échap annule).
editor-problemas = Problèmes ({ $n })
editor-avisos = Avertissements ({ $n })
editor-colocar_dica = { $tipo } : cliquez puis cliquez au sol, ou faites glisser jusqu'au sol
editor-encaixar = Aligner sur la grille
editor-passo = Pas (m)
editor-esc_cancela = Échap annule. Maj + clic au sol en place plusieurs.
editor-atalhos_f1 = Raccourcis : F1.
editor-selecionados = { $n } objets sélectionnés
editor-copiar = Copier
editor-duplicar = Dupliquer
editor-excluir_todos = Tout supprimer
editor-alinhar = Aligner
editor-esquerda = Gauche
editor-centro_x = Centre X
editor-direita = Droite
editor-topo = Haut
editor-centro_y = Centre Y
editor-base = Bas
editor-distribuir_x = Répartir en x
editor-distribuir_y = Répartir en y
editor-distribuir_dica = Espace régulièrement entre les deux extrémités (3 objets ou plus)
editor-propriedades_de_todos = Propriétés de tous
editor-campos_de_todos = Les champs montrent les valeurs de { $primeiro } ; chaque modification s'applique aux { $n } objets.
editor-buscar_por_nome = Rechercher par nom
editor-todos_os_tipos = Tous les types
editor-de_objetos = { $n } sur { $total } objets
editor-objeto_dica = { $tipo }. Un clic sélectionne, Maj ajoute à la sélection, un double clic cadre
editor-subir = Monter
editor-descer = Descendre
editor-subir_dica = Échange sa place avec la précédente (Ctrl+Z annule)
editor-descer_dica = Échange sa place avec la suivante (Ctrl+Z annule)
editor-remover_ligacao = Retirer la liaison
editor-ligacoes = Liaisons
editor-nenhuma_ligacao = Aucune. Utilisez « Relier » et cliquez sur la destination.
editor-ordem_importa = L'ordre compte. Sorties : l'ordre est le numéro du port (1, 2, ...) qu'utilisent les règles de sortie. Entrées : la première est prioritaire quand l'objet tire un article.
editor-saidas_por_porta = Sorties, par port :
editor-porta = port { $n } : { $de } -> { $para }
editor-entradas_por_prioridade = Entrées, par priorité :
editor-entrada = n° { $n } : { $de } -> { $para }
atalho-01-teclas = Clic
atalho-01 = sélectionne le bloc (ou retire la liaison cliquée)
atalho-02-teclas = Clic répété au même endroit
atalho-02 = alterne entre les objets superposés (une info-bulle montre lequel est sélectionné)
atalho-03-teclas = Maj + clic
atalho-03 = ajoute le bloc à la sélection ou l'en retire
atalho-04-teclas = Maj + glisser au sol
atalho-04 = sélection par boîte (ajoute à la sélection)
atalho-05-teclas = Glisser un bloc
atalho-05 = déplace le bloc ou toute la sélection
atalho-06-teclas = Glisser au sol
atalho-06 = déplace la vue
atalho-07-teclas = Bouton droit + glisser
atalho-07 = fait pivoter la vue
atalho-08-teclas = Molette de la souris
atalho-08 = rapproche et éloigne, vers le curseur
atalho-09-teclas = Onglet Objets : clic, Maj + clic
atalho-09 = sélectionne, ajoute à la sélection ; le double clic cadre l'objet
atalho-10-teclas = Bibliothèque : clic puis clic au sol
atalho-10 = place un objet (Maj garde l'outil)
atalho-11-teclas = Bibliothèque : glisser jusqu'au sol
atalho-11 = place un objet là où on le lâche
atalho-12-teclas = Flèches
atalho-12 = déplace la sélection d'un pas de grille
atalho-13-teclas = Suppr
atalho-13 = supprime la sélection
atalho-14-teclas = C
atalho-14 = relie le bloc sélectionné au prochain cliqué
atalho-15-teclas = Échap
atalho-15 = annule l'outil, ferme les dialogues et quitte les caméras suivre et première personne
atalho-16-teclas = F
atalho-16 = cadre le modèle (retour à la caméra en orbite)
atalho-17-teclas = 1, 2, 3, 4
atalho-17 = caméra : orbite, suivre une ressource, première personne, depuis la ressource
atalho-18-teclas = W A S D
atalho-18 = marche en première personne (Maj court, la molette change la vitesse)
atalho-19-teclas = L
atalho-19 = verrouille la souris en première personne
atalho-20-teclas = F1
atalho-20 = affiche ou masque cette fenêtre
atalho-21-teclas = F3
atalho-21 = affiche ou masque les temps d'image
atalho-22-teclas = F4
atalho-22 = active ou désactive les niveaux de détail
atalho-23-teclas = Ctrl + A
atalho-23 = sélectionne tous les objets
atalho-24-teclas = Ctrl + C
atalho-24 = copie la sélection
atalho-25-teclas = Ctrl + X
atalho-25 = coupe la sélection
atalho-26-teclas = Ctrl + V
atalho-26 = colle (noms uniques, liaisons internes conservées)
atalho-27-teclas = Ctrl + D
atalho-27 = duplique la sélection
atalho-28-teclas = Ctrl + Z
atalho-28 = annule
atalho-29-teclas = Ctrl + Y ou Ctrl + Maj + Z
atalho-29 = rétablit
atalho-30-teclas = Ctrl + S
atalho-30 = enregistre
atalho-31-teclas = Ctrl + Maj + S
atalho-31 = enregistre sous un autre nom (Enregistrer sous)
atalho-32-teclas = Ctrl + N
atalho-32 = nouveau modèle

# Partie 2 : le panneau des propriétés (properties/).

## Propriétés : familles de lois, règles et types

dist-constante = Constante
dist-uniforme = Uniforme
dist-exponencial = Exponentielle
dist-triangular = Triangulaire
dist-normal = Normale
dist-lognormal = Lognormale
dist-gama = Gamma
dist-erlang = Erlang
dist-weibull = Weibull
dist-beta = Bêta
dist-log-logistica = Log-logistique
dist-pearson-v = Pearson V
dist-pearson-vi = Pearson VI
dist-discreta = Discrète
dist-empirica = Empirique
dist-poisson = Poisson
dist-geometrica = Géométrique
dist-binomial = Binomiale
dist-binomial-negativa = Binomiale négative
dist-deslocada = Décalée
prop-vaga-primeira-livre = Première libre
prop-vaga-aleatoria = Aléatoire
prop-despacho-primeiro-livre = Premier libre
prop-despacho-rodizio = Tourniquet
prop-despacho-mais-proximo = Le plus proche
tipo-chegada = Arrivée
tipo-fila = File
tipo-processo = Processus
tipo-montagem = Assemblage
tipo-desmontagem = Désassemblage
tipo-saida = Sortie
tipo-transportador = Transporteur
tipo-ponte-rolante = Pont roulant
tipo-despachante = Répartiteur
tipo-esteira = Convoyeur
tipo-porta-paletes = Rayonnage à palettes
prop-modo-igual-ao-padrao = comme par défaut
prop-modo-distribuicao = loi
prop-modo-taxa = taux
prop-modo-sem-chegadas = sans arrivées
prop-turno-pausar = Suspendre
prop-turno-terminar = Terminer
prop-turno-pausar-dica = Le travail en cours se fige à la fin de l'équipe et reprend quand l'équipe revient
prop-turno-terminar-dica = À la fin de l'équipe l'objet n'accepte plus de nouveaux articles, termine l'article en cours (heures supplémentaires) et seulement alors quitte l'équipe ; l'équipe suivante commence à l'heure habituelle

## properties/network.rs

prop-network-o_nome_nao_pode_ficar = le nom ne peut pas être vide
prop-network-ja_existe_um_no_chamado = il existe déjà un nœud nommé « { $t } »
prop-network-nome = Nom
prop-network-no_da_rede_de_caminhos = Nœud du réseau de chemins
prop-network-arestas = Arêtes
prop-network-nenhuma_use_a_ferramenta_aresta = Aucune : utilisez l'outil « Arête » et cliquez sur un autre nœud.
prop-network-remover_aresta = Supprimer l'arête
prop-network-objetos_ligados_a_este_no = Objets reliés à ce nœud
prop-network-nenhum_cada_objeto_atendido_entra = Aucun (chaque objet desservi entre par le nœud le plus proche).
prop-network-aresta = Arête { $edge }
prop-network-comprimento_m_calculado_pelas_posicoes = Longueur : { $len } m (calculée à partir des positions des nœuds)
prop-network-mao_dupla = Double sens ({ $from } <-> { $to })
prop-network-mao_unica = Sens unique ({ $from } -> { $to })
prop-network-mao_unica-2 = Sens unique ({ $to } -> { $from })
prop-network-sentido = Sens
prop-network-limitar_veiculos_agv = Limiter les véhicules (AGV)
prop-network-quantos_agvs_podem_estar_na = Combien d'AGV peuvent être sur l'arête en même temps
prop-network-sem_limite = sans limite
prop-network-so_agvs_agv_true_respeitam = Seuls les AGV (agv: true) respectent la capacité ; les autres passent librement.
prop-network-objeto_discreto = Objet discret
prop-network-escolha = (choisir)
prop-network-nao_e_um_objeto_discreto = « { $current } » n'est pas un objet discret du modèle
prop-network-vazao = Débit
prop-network-capacidade = Capacité
prop-network-nivel_inicial = Niveau initial
prop-network-entrada_maxima = Entrée maximale
prop-network-saida_maxima = Sortie maximale
prop-network-vazao_maxima = Débit maximal
prop-network-cada_item_que_entra_no = Chaque article qui entre dans l'objet devient ce volume, livré d'un coup.
prop-network-volume_por_item = Volume par article
prop-network-a_cada_volume_completo_cria = À chaque volume complet, crée un article dans l'objet.
prop-network-problema = Problème : { $message }
prop-network-tipo = Type
prop-network-ligacoes = Liaisons
prop-network-nenhuma_use_a_ferramenta_ligar = Aucune : utilisez l'outil « Relier » et cliquez sur le nœud suivant.
prop-network-remover_ligacao = Supprimer la liaison

## properties/panel.rs

prop-panel-buscar = Rechercher
prop-panel-texto_invalido_nao_aplicado = Texte invalide, non appliqué : { $error }
prop-panel-descartar_texto = abandonner le texte
prop-panel-volta_ao_valor_que_esta = Revient à la valeur du modèle
prop-panel-nao_e_um_numero = « { $s } » n'est pas un nombre
prop-panel-infinita = infinie
prop-panel-valor = valeur
prop-panel-peso = poids
prop-panel-remover = supprimer
prop-panel-adicionar_valor = ajouter une valeur
prop-panel-observacoes_em_ordem_crescente_separadas = Observations en ordre croissant, séparées par des espaces :
prop-panel-deslocamento = décalage
prop-panel-min = min.
prop-panel-max = max.
prop-panel-media = moyenne
prop-panel-moda = mode
prop-panel-desvio = écart-type
prop-panel-forma = forme
prop-panel-escala = échelle
prop-panel-alfa = alpha
prop-panel-beta = bêta
prop-panel-forma_1 = forme 1
prop-panel-forma_2 = forme 2
prop-panel-media-2 = moyenne = { $mean }
prop-panel-sem_limite = sans limite
prop-panel-sem_limite_velocidade_constante = sans limite (vitesse constante)
prop-panel-equipe_transportadores_do_modelo = Équipe (transporteurs du modèle) :
prop-panel-nenhum_transportador_no_modelo_crie = Aucun transporteur dans le modèle. Créez-en un dans la palette.
prop-panel-nao_e_um_transportador = « { $missing } » n'est pas un transporteur
prop-panel-a_equipe_nao_pode_ficar = L'équipe ne peut pas être vide.
prop-panel-precisa_de_operadores = Nécessite des opérateurs
prop-panel-crie_um_transportador_ou_despachante = Créez un Transporteur ou un Répartiteur à utiliser comme équipe.
prop-panel-equipe = équipe
prop-panel-nenhuma = (aucune)
prop-panel-nao_e_um_transportador_nem = « { $team } » n'est ni un transporteur ni un répartiteur
prop-panel-quantidade = quantité
prop-panel-durante_a_preparacao = pendant la préparation
prop-panel-durante_o_processamento = pendant le traitement
prop-panel-transporte_de_saida = Transport de sortie
prop-panel-nenhum = aucun
prop-panel-os_itens_passam_direto_ao = Les articles passent directement au bloc suivant.
prop-panel-nao_e_um_transportador_nem-2 = « { $t } » n'est ni un transporteur ni un répartiteur
prop-panel-sem_rede_de_caminhos_a = Pas de réseau de chemins : la base est la position du transporteur.
prop-panel-base_no_da_rede = Base (nœud du réseau)
prop-panel-fora_da_rede = hors du réseau
prop-panel-escolher_um_no_poe_o = Choisir un nœud y place le transporteur et active « Retour à la base ».
prop-panel-editar_como_texto = modifier comme texte
prop-panel-quantidade_de_itens_exigida_em = Quantité d'articles exigée à chaque entrée de composants :
prop-panel-entrada = entrée { $i }
prop-panel-adicionar_entrada = ajouter une entrée
prop-panel-o_objeto_precisa_de_ligacoes = L'objet a besoin de { $list } liaisons d'entrée (1 conteneur + { $list2 }).
prop-panel-valor_do_rotulo = Valeur de l'étiquette
prop-panel-remover_rotulo = supprimer l'étiquette
prop-panel-nome_do_rotulo = nom de l'étiquette
prop-panel-adicionar_rotulo = ajouter une étiquette
prop-panel-lote_agrupar_itens = Lot (regrouper les articles)
prop-panel-tamanho_do_lote = taille du lot
prop-panel-espera_maxima_0_esperar_sempre = attente maximale (0 = toujours attendre)
prop-panel-fechar_a_entrada_ate_o = fermer l'entrée jusqu'à la sortie du lot
prop-panel-primeira_livre = Première libre
prop-panel-rodizio = Tourniquet
prop-panel-aleatoria = Aléatoire
prop-panel-pesos = Poids
prop-panel-por_rotulo = Par étiquette
prop-panel-script = Script
prop-panel-regra_de_saida = Règle de sortie
prop-panel-peso_de_cada_saida_na = Poids de chaque sortie (dans l'ordre des liaisons) :
prop-panel-saida = sortie { $i } ({ $to })
prop-panel-saida-2 = sortie { $i }
prop-panel-ha_pesos_mas_o_objeto = Il y a { $w } poids, mais l'objet a { $n_out } sorties.
prop-panel-ajustar_ao_numero_de_saidas = ajuster au nombre de sorties
prop-panel-rotulo_com_o_numero_da = étiquette avec le numéro du port
prop-panel-o_nome_do_rotulo_nao = le nom de l'étiquette ne peut pas être vide
prop-panel-script_que_devolve_o_numero = Script qui renvoie le numéro du port (1, 2, ...)
prop-panel-script_ao_entrar_um_item = Script à l'entrée d'un article (on_entry)
prop-panel-roda_para_cada_item_que = S'exécute pour chaque article qui entre, avant toute autre règle.
prop-panel-quebras_mtbf_mttr = Pannes (MTBF/MTTR)
prop-panel-tempo_entre_falhas_mtbf = Temps entre pannes (MTBF)
prop-panel-tempo_de_reparo_mttr = Temps de réparation (MTTR)
prop-panel-transferencia_pela_distancia = Transfert selon la distance
prop-panel-os_itens_levam_atraso_distancia = Les articles mettent délai + distance / vitesse pour atteindre le bloc suivant par les liaisons directes (sans transporteur)
prop-panel-velocidade = Vitesse
prop-panel-atraso = Délai
prop-panel-caminho = Chemin
prop-panel-reta = droit
prop-panel-pela_rede = par le réseau
prop-panel-tempo_entre_chegadas = Temps entre arrivées
prop-panel-primeira_chegada_em_t_0 = Première arrivée à t = 0
prop-panel-maximo_de_chegadas = Nombre maximal d'arrivées
prop-panel-se_a_saida_estiver_bloqueada = Si la sortie est bloquée
prop-panel-esperar = Attendre
prop-panel-descartar_perda = Rejeter (perte)
prop-panel-rotulos = Étiquettes
prop-panel-capacidade_limitada = Capacité limitée
prop-panel-disciplina = Discipline
prop-panel-fifo_primeiro_a_entrar_primeiro = FIFO (premier entré, premier sorti)
prop-panel-lifo_ultimo_a_entrar_primeiro = LIFO (dernier entré, premier sorti)
prop-panel-capacidade = Capacité
prop-panel-preparacao_setup = Préparation (setup)
prop-panel-tempo_de_processamento = Temps de traitement
prop-panel-operadores = Opérateurs
prop-panel-modo = Mode
prop-panel-embalar_so_o_conteiner_sai = Emballer (seul le conteneur sort)
prop-panel-juntar_componentes_destruidos = Joindre (composants détruits)
prop-panel-lote_todos_saem = Lot (tous sortent)
prop-panel-desembalar = Déballer
prop-panel-dividir_em_copias = Diviser en copies
prop-panel-numero_de_itens_gerados = nombre d'articles créés
prop-panel-a_saida_nao_tem_parametros = La sortie n'a pas de paramètres.
prop-panel-velocidade_maxima = Vitesse maximale
prop-panel-aceleracao = Accélération
prop-panel-desaceleracao = Décélération
prop-panel-capacidade_itens = Capacité (articles)
prop-panel-tempo_de_carga = Temps de chargement
prop-panel-tempo_de_descarga = Temps de déchargement
prop-panel-voltar_a_base_quando_ocioso = Retour à la base quand inactif
prop-panel-agv_controle_de_trafego_na = AGV (contrôle du trafic sur le réseau)
prop-panel-comprimento = Longueur
prop-panel-comprimento_por_item = Longueur par article
prop-panel-acumula_itens_quando_a_saida = Accumule les articles quand la sortie bloque
prop-panel-capacidade_itens-2 = Capacité : { $item_length } articles
prop-panel-colunas = Colonnes
prop-panel-niveis = Niveaux
prop-panel-permanencia_minima = Séjour minimal
prop-panel-regra_de_ocupacao = Règle d'occupation
prop-panel-largura_da_coluna = Largeur de la colonne
prop-panel-altura_do_nivel = Hauteur du niveau
prop-panel-capacidade_posicoes = Capacité : { $levels } emplacements
prop-panel-regra_de_escolha = Règle de choix
prop-panel-o_nome_nao_pode_ficar = le nom ne peut pas être vide
prop-panel-ja_existe_um_objeto_chamado = il existe déjà un objet nommé « { $t } »
prop-panel-nome = Nom
prop-panel-problema = Problème : { $message }
prop-panel-padrao_do_tipo = (défaut du type)
prop-panel-aparencia = Apparence
prop-panel-ativo = Ressource :
prop-panel-clipes_de_animacao_ainda_nao = Les clips d'animation ne sont pas encore joués : pose de repos.
prop-panel-biblioteca_do_utsim = Bibliothèque d'UTSim
prop-panel-utsim_gerado_por_utsim_assets = UTSim (généré par utsim-assets)
prop-panel-terceiros_assets_terceiros = Tiers (assets/terceiros)
prop-panel-cor = Couleur
prop-panel-escala-2 = Échelle
prop-panel-giro_graus = Rotation (degrés)
prop-panel-restaurar_padrao = Rétablir le défaut
prop-panel-volta_a_aparencia_padrao_do = Revient à l'apparence par défaut du type d'objet
prop-panel-simulacao = Simulation
prop-panel-tempo_de_simulacao = Durée de simulation
prop-panel-aquecimento = Préchauffage
prop-panel-semente = Graine
prop-panel-replicacoes = Réplications

## properties/position.rs

prop-position-posicao_m = Position (m)
prop-position-giro_graus = Rotation (degrés)

## properties/schedule.rs

prop-schedule-segundos = secondes
prop-schedule-minutos = minutes
prop-schedule-horas = heures
prop-schedule-use_dia_n_hh_mm = utilisez « dia N hh:mm » (reçu « { $t } »)
prop-schedule-dia_invalido_em_a_partir = jour invalide dans « { $t } » (à partir de 1)
prop-schedule-use_um_numero = utilisez un nombre
prop-schedule-hh_mm_so_vale_com = hh:mm ne vaut qu'avec un cycle d'un jour ou d'une semaine (1440 min, 24 h, 86400 s) ; utilisez un nombre
prop-schedule-hora_invalida_em = heure invalide dans « { $t } »
prop-schedule-minuto_invalido_em = minute invalide dans « { $t } »
prop-schedule-minuto_invalido_em_0_a = minute invalide dans « { $t } » (0 à 59)
prop-schedule-nao_e_um_horario_hh = « { $t } » n'est ni un horaire (hh:mm) ni un nombre

## properties/schedule_ui.rs

prop-schedule-ui-media = { $family }, moyenne { $mean }
prop-schedule-ui-chegadas_por_periodo = Arrivées par période
prop-schedule-ui-a_taxa_de_chegada_muda = le taux d'arrivée change avec l'heure
prop-schedule-ui-uma_tabela_de_periodos_que = Une table de périodes qui se répète à chaque cycle (un jour, une semaine). Hors des périodes, il n'y a pas d'arrivées.
prop-schedule-ui-desligado_o_tempo_entre_chegadas = Désactivé : le temps entre arrivées ci-dessus vaut tout le temps.
prop-schedule-ui-ciclo = cycle
prop-schedule-ui-dia = jour
prop-schedule-ui-semana = semaine
prop-schedule-ui-ciclo_de_tempo_do_modelo = cycle de { $v } (temps du modèle en minutes)
prop-schedule-ui-ciclo_de_tempo_do_modelo-2 = Cycle de { $cycle } : temps du modèle en { $name }. Horaires en hh:mm, taux par heure.
prop-schedule-ui-horarios_e_taxas_em_unidades = Horaires et taux en unités du modèle (avec un cycle de 1440, un jour en minutes, la table affiche hh:mm).
prop-schedule-ui-taxa_por_hora = taux par heure
prop-schedule-ui-taxa = taux
prop-schedule-ui-inicio = début
prop-schedule-ui-fim = fin
prop-schedule-ui-modo = mode
prop-schedule-ui-valor = valeur
prop-schedule-ui-esperado = attendu
prop-schedule-ui-copiar_do_padrao = copier le défaut
prop-schedule-ui-usa_no_periodo_uma_copia = Utilise dans la période une copie du temps entre arrivées par défaut, pour l'ajuster seulement ici
prop-schedule-ui-remover_o_periodo = supprimer la période
prop-schedule-ui-adicionar_periodo = ajouter une période
prop-schedule-ui-por_dia = {" "}(par jour)
prop-schedule-ui-por_semana = {" "}(par semaine)
prop-schedule-ui-esperado_por_ciclo_chegadas = Attendu par cycle{ $hint } : { $per_cycle } arrivées
prop-schedule-ui-na_troca_de_periodo_a = Au changement de période, l'arrivée en attente est tirée à nouveau avec le processus de la nouvelle période (des périodes consécutives avec le même processus gardent l'arrivée).
prop-schedule-ui-distribuicoes_dos_periodos = Lois des périodes
prop-schedule-ui-periodo_tempo_entre_chegadas = Période { $i } : temps entre arrivées
prop-schedule-ui-rotulos_por_periodo = Étiquettes par période
prop-schedule-ui-os_rotulos_de_um_periodo = Les étiquettes d'une période remplacent les étiquettes par défaut du même nom (par exemple, plus urgentes la nuit) ; les autres restent valables.
prop-schedule-ui-a_fonte_nao_tem_rotulos = La source n'a pas d'étiquettes par défaut.
prop-schedule-ui-a = { $a } à { $b }
prop-schedule-ui-a-2 = { $start } à { $end }
prop-schedule-ui-periodo = Période { $i } ({ $span })

## properties/shifts_ui.rs

prop-shifts-ui-como_no_calendario = comme dans le calendrier ({ $to_lowercase })
prop-shifts-ui-ao_fim_do_turno = À la fin de l'équipe
prop-shifts-ui-periodo = période
prop-shifts-ui-inicio = début
prop-shifts-ui-duracao = durée
prop-shifts-ui-remover = supprimer
prop-shifts-ui-adicionar_janela = ajouter une fenêtre
prop-shifts-ui-fora_de_turno_do_periodo = hors équipe : { $off_fraction } % de la période
prop-shifts-ui-turnos_janelas_fora_de_turno = Équipes (fenêtres hors équipe)
prop-shifts-ui-usar_calendario = utiliser un calendrier
prop-shifts-ui-crie_calendarios_na_secao_calendarios = Créez des calendriers dans la section Calendriers du panneau Simulation (cliquez sur un point vide de la scène)
prop-shifts-ui-personalizado = personnalisé
prop-shifts-ui-periodo_janela_s_fora_de = période { $period }, { $off } fenêtre(s) hors équipe
prop-shifts-ui-o_calendario_nao_existe = le calendrier '{ $name }' n'existe pas
prop-shifts-ui-nome_do_calendario = nom du calendrier
prop-shifts-ui-salvar_como_calendario = enregistrer comme calendrier
prop-shifts-ui-outros_objetos_podem_usar_o = D'autres objets peuvent utiliser le même calendrier
prop-shifts-ui-calendarios = Calendriers ({ $calendars })
prop-shifts-ui-tabelas_de_turno_com_nome = Tables d'équipes nommées. Un objet en utilise une dans Équipes > utiliser un calendrier ; modifier le calendrier modifie tous les objets qui l'utilisent.
prop-shifts-ui-nenhum_objeto_usa_este_calendario = aucun objet n'utilise ce calendrier
prop-shifts-ui-usado_por = utilisé par : { $users }
prop-shifts-ui-remover_calendario = supprimer le calendrier
prop-shifts-ui-troque_os_objetos_para_outro = Passez d'abord les objets à un autre calendrier ou à personnalisé
prop-shifts-ui-novo_calendario = nouveau calendrier
prop-shifts-ui-o_nome_nao_pode_ficar = le nom ne peut pas être vide
prop-shifts-ui-ja_existe_um_calendario_chamado = il existe déjà un calendrier nommé « { $t } »
prop-shifts-ui-nome = Nom

## properties/spec.rs

prop-spec-continuas = Continues
prop-spec-discretas_e_empirica = Discrètes et empirique
prop-spec-combinacao = Combinaison

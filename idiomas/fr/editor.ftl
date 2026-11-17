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

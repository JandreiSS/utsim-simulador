# Traduzione automatica, in attesa di revisione umana.
# Tradução automática, aguardando revisão humana.
# UTSim: l'editor del simulatore (base: barra, livelli, pannelli e dialoghi generali).

ui-sem_observacoes = nessuna osservazione
ui-resumo = n={ $n }  media={ $media }  ds={ $dp }  min={ $min }  max={ $max }
ui-recebidos = ricevuti: { $n }
ui-simulacao = Simulazione
ui-modelo_replicacao = { $modelo } - replica { $n }
ui-contagem = { $blocos ->
    [one] { $blocos } blocco
   *[other] { $blocos } blocchi
}, { $itens ->
    [one] { $itens } articolo
   *[other] { $itens } articoli
}, { $eventos ->
    [one] { $eventos } evento di traccia
   *[other] { $eventos } eventi di traccia
}
ui-pausar = Pausa
ui-reproduzir = Riproduci
ui-evento_anterior = < Evento
ui-evento_seguinte = Evento >
ui-inicio = Inizio
ui-velocidade = velocità x{ $v }
ui-unidades_por_segundo = unità di tempo simulato al secondo
ui-tempo = tempo
ui-tempo_simulado = tempo simulato: { $t } di { $fim }
ui-ajuda_teclas = Spazio: riproduci/pausa | +/-: velocità | frecce: evento precedente/successivo | F: inquadra
ui-ajuda_mouse = Mouse: il sinistro trascina (sposta) e clicca (seleziona), il destro ruota, la rotella avvicina
ui-ajuda_camera = Telecamera: 1 orbita, 2 segui, 3 prima persona, 4 dalla risorsa | F3: tempi dei fotogrammi
ui-blocos = Blocchi
ui-clique_em_um_bloco = Fai clic su un blocco per vederne le statistiche.
ui-capacidade_rotulo = capacità { $n }
ui-nivel_agora = livello attuale
ui-capacidade = capacità
ui-ocupacao = occupazione
ui-taxa_agora = tasso attuale
ui-nivel_inicial = livello iniziale
ui-entrada_maxima = ingresso massimo
ui-saida_maxima = uscita massima
ui-vazao_da_fonte = portata della sorgente
ui-volume_por_item = volume per articolo
ui-acumulado = accumulato per il prossimo articolo
ui-vazao_agora = portata attuale
ui-vazao_maxima = portata massima
ui-objeto = oggetto
ui-por_unidade_de_tempo = { $taxa } per unità di tempo ({ $sentido })
ui-nivel_interpolado = Livello e tasso interpolati dalla traccia della rete dei fluidi.
ui-seguir_recurso = Segui questa risorsa
ui-seguir_recurso_dica = La telecamera segue la risorsa da dietro e dall'alto (2)
ui-ver_do_recurso = Vedi dalla risorsa
ui-ver_do_recurso_dica = La telecamera va nella posizione del conducente (4)
ui-recebidos_ate_agora = ricevuti finora
ui-estado_agora = stato attuale
ui-conteudo_agora = contenuto attuale
ui-entradas = ingressi
ui-saidas = uscite
ui-conteudo_medio = contenuto medio
ui-conteudo_maximo = contenuto massimo
ui-tempo_de_permanencia = tempo di permanenza
ui-fracao_em = frazione in { $estado }
ui-estatisticas_finais = Statistiche finali della replica (finestra dopo il riscaldamento).
camadas-no_excluido = Nodo { $nome } e i suoi archi eliminati (Ctrl+Z annulla)
camadas-aresta_excluida = Arco { $de } - { $para } eliminato (Ctrl+Z annulla)
camadas-no_fluido_excluido = Nodo { $nome } e i suoi collegamenti eliminati (Ctrl+Z annulla)
camadas-objetos_dica = Oggetti del modello: libreria, collegamenti e proprietà
camadas-rede_dica = Rete di percorsi delle risorse mobili: nodi e archi
camadas-fluidos_dica = Rete dei fluidi: sorgenti, serbatoi, tubi e convertitori
camada-objetos = Oggetti
camada-rede = Rete
camada-fluidos = Fluidi
camadas-rede_de_caminhos = Rete di percorsi
camadas-selecionar = Seleziona
camadas-selecionar_rede_dica = Il clic seleziona un nodo o un arco; trascinare sposta il nodo
camadas-no = Nodo
camadas-no_dica = Un clic sul pavimento posiziona un nodo
camadas-aresta = Arco
camadas-aresta_dica = Fai clic su un nodo e poi su un altro; un clic sul pavimento crea un nodo già collegato
camadas-mao_dupla = Nuovi archi a doppio senso
camadas-ajuda_aresta = Arco: fai clic da nodo a nodo per disegnare il percorso; Esc termina.
camadas-ajuda_del_rede = Canc elimina il nodo (con i suoi archi) o l'arco selezionato.
camadas-ajuda_atendido = Ogni oggetto servito entra nella rete dal nodo più vicino (linea tratteggiata).
camadas-rede_de_fluidos = Rete dei fluidi
camadas-selecionar_fluidos_dica = Il clic seleziona un nodo; trascinare sposta il nodo
camadas-ligar = Collega
camadas-ligar_dica = Fai clic sul nodo di origine e poi su quello di destinazione (la portata segue la freccia)
camadas-colocar = Posiziona
camadas-colocar_dica = Un clic sul pavimento posiziona il nodo (Maiusc mantiene lo strumento)
camadas-ajuda_aciclica = La rete deve essere aciclica. I convertitori collegano la rete a un oggetto.
camadas-ajuda_del_fluidos = Canc elimina il nodo selezionato e i suoi collegamenti.
camadas-aresta_daqui = Arco da qui
camadas-ligar_daqui = Collega da qui
camadas-sem_rede = Il modello non ha una rete di percorsi.
camadas-sem_rede_ajuda = Senza rete, le risorse mobili si muovono in linea retta. Usa "Nodo" per posizionare i nodi e "Arco" per collegarli.
camadas-resumo_rede = { $nos ->
    [one] { $nos } nodo
   *[other] { $nos } nodi
} e { $arestas ->
    [one] { $arestas } arco
   *[other] { $arestas } archi
}. Selezionane uno per modificarlo.
camadas-nos = Nodi ({ $n })
camadas-arestas = Archi ({ $n })
camadas-sem_fluidos = Il modello non ha una rete dei fluidi.
camadas-sem_fluidos_ajuda = Scegli un tipo in "Posiziona" e fai clic sul pavimento; poi usa "Collega".
camadas-resumo_fluidos = Rete "{ $rede }": { $nos ->
    [one] { $nos } nodo
   *[other] { $nos } nodi
} e { $ligacoes ->
    [one] { $ligacoes } collegamento
   *[other] { $ligacoes } collegamenti
}. Seleziona un nodo per modificarlo.
editor-layout_aplicado = Disposizione automatica applicata: salva per mantenere le posizioni
editor-layout_com_rede = Modello con rete di percorsi: posiziona gli oggetti sui nodi della rete (la disposizione automatica non è stata applicata)
editor-sem_titulo = senza titolo.ron
editor-excluidos = { $n } oggetti eliminati (Ctrl+Z annulla)
editor-nada_para_copiar = Niente di selezionato da copiare
editor-copiados = { $objetos ->
    [one] { $objetos } oggetto
   *[other] { $objetos } oggetti
} e { $ligacoes ->
    [one] { $ligacoes } collegamento copiati
   *[other] { $ligacoes } collegamenti copiati
}
editor-recortados = { $n ->
    [one] { $n } oggetto tagliato (Ctrl+V incolla)
   *[other] { $n } oggetti tagliati (Ctrl+V incolla)
}
editor-area_vazia = Gli appunti sono vuoti (Ctrl+C copia)
editor-colados = { $n ->
    [one] { $n } oggetto incollato: { $nomes }
   *[other] { $n } oggetti incollati: { $nomes }
}
editor-nada_para_duplicar = Niente di selezionato da duplicare
editor-salvo_em = Salvato in { $arquivo }
editor-erro_ao_salvar = Errore durante il salvataggio: { $erro }
editor-tempo_invalido = Tempo massimo non valido: usa un numero maggiore di zero
editor-simulando = Simulazione in corso...
editor-sem_resultado = la simulazione è terminata senza risultato
editor-erro_na_simulacao = Errore nella simulazione: { $erro }
editor-na_caixa = { $n ->
    [one] { $n } oggetto nel riquadro
   *[other] { $n } oggetti nel riquadro
}, { $sel ->
    [one] { $sel } selezionato
   *[other] { $sel } selezionati
}
editor-outro_destino = Scegli un altro blocco come destinazione
editor-ligacao_removida = Collegamento { $de } -> { $para } rimosso (Ctrl+Z annulla)
editor-editar = Modifica
editor-simular = ▶ Simula
editor-ate_t = fino a t =
editor-novo = Nuovo
editor-salvar = Salva
editor-salvar_como_botao = Salva con nome...
editor-desfazer = Annulla
editor-refazer = Ripeti
editor-enquadrar = Inquadra (F)
editor-fluxos = Flussi
editor-fluxos_dica = Logica dell'utente: flussi di token
editor-resultados = Risultati
editor-resultados_dica = Repliche, riepilogo con intervalli di confidenza, grafici, scenari e ottimizzazione
editor-atalhos = Scorciatoie
editor-atalhos_dica = Scorciatoie da tastiera e mouse (F1)
editor-salvar_como = Salva con nome
editor-caminho_do_arquivo = Percorso del file (.ron), relativo alla cartella del programma o completo:
editor-sobrescrito = Questo file esiste già e verrà sovrascritto.
editor-cancelar = Annulla
editor-novo_modelo = Nuovo modello
editor-descartar_pergunta = Ci sono modifiche non salvate. Scartarle e iniziare un nuovo modello?
editor-descartar = Scarta
editor-atalhos_do_editor = Scorciatoie dell'editor
editor-f1_fecha = F1 o Esc chiude questa finestra.
editor-nao_simula = Impossibile simulare
editor-corrija = Correggi i problemi del modello prima di simulare:
editor-fechar = Chiudi
editor-valores_gravados = Valori scritti nel modello (Ctrl+Z annulla)
editor-nenhum_problema = Nessun problema: il modello può essere simulato.
editor-modelo = Modello
editor-biblioteca = Libreria
editor-propriedades = Proprietà
editor-selecione_um_bloco = Seleziona un blocco per modificarne le proprietà.
editor-ligar = Collega (C)
editor-excluir = Elimina (Canc)
editor-clique_no_destino = Fai clic sul blocco di destinazione (Esc annulla).
editor-problemas = Problemi ({ $n })
editor-avisos = Avvisi ({ $n })
editor-colocar_dica = { $tipo }: fai clic e poi clic sul pavimento, o trascina fino al pavimento
editor-encaixar = Aggancia alla griglia
editor-passo = Passo (m)
editor-esc_cancela = Esc annulla. Maiusc + clic sul pavimento ne posiziona diversi.
editor-atalhos_f1 = Scorciatoie: F1.
editor-selecionados = { $n } oggetti selezionati
editor-copiar = Copia
editor-duplicar = Duplica
editor-excluir_todos = Elimina tutti
editor-alinhar = Allinea
editor-esquerda = Sinistra
editor-centro_x = Centro X
editor-direita = Destra
editor-topo = Alto
editor-centro_y = Centro Y
editor-base = Basso
editor-distribuir_x = Distribuisci in x
editor-distribuir_y = Distribuisci in y
editor-distribuir_dica = Spazia in modo uniforme tra i due estremi (3 oggetti o più)
editor-propriedades_de_todos = Proprietà di tutti
editor-campos_de_todos = I campi mostrano i valori di { $primeiro }; ogni modifica vale per i { $n } oggetti.
editor-buscar_por_nome = Cerca per nome
editor-todos_os_tipos = Tutti i tipi
editor-de_objetos = { $n } di { $total } oggetti
editor-objeto_dica = { $tipo }. Il clic seleziona, Maiusc aggiunge alla selezione, il doppio clic inquadra
editor-subir = Su
editor-descer = Giù
editor-subir_dica = Scambia di posto con la precedente (Ctrl+Z annulla)
editor-descer_dica = Scambia di posto con la successiva (Ctrl+Z annulla)
editor-remover_ligacao = Rimuovi collegamento
editor-ligacoes = Collegamenti
editor-nenhuma_ligacao = Nessuno. Usa "Collega" e fai clic sulla destinazione.
editor-ordem_importa = L'ordine conta. Uscite: l'ordine è il numero della porta (1, 2, ...) che usano le regole di uscita. Ingressi: il primo ha la priorità quando l'oggetto preleva un articolo.
editor-saidas_por_porta = Uscite, per porta:
editor-porta = porta { $n }: { $de } -> { $para }
editor-entradas_por_prioridade = Ingressi, per priorità:
editor-entrada = { $n }°: { $de } -> { $para }
atalho-01-teclas = Clic
atalho-01 = seleziona il blocco (o rimuove il collegamento cliccato)
atalho-02-teclas = Clic ripetuto nello stesso punto
atalho-02 = alterna tra gli oggetti sovrapposti (un suggerimento mostra quale è selezionato)
atalho-03-teclas = Maiusc + clic
atalho-03 = aggiunge il blocco alla selezione o lo toglie
atalho-04-teclas = Maiusc + trascinare sul pavimento
atalho-04 = selezione a riquadro (aggiunge alla selezione)
atalho-05-teclas = Trascinare un blocco
atalho-05 = sposta il blocco o l'intera selezione
atalho-06-teclas = Trascinare sul pavimento
atalho-06 = sposta la vista
atalho-07-teclas = Tasto destro + trascinare
atalho-07 = ruota la vista
atalho-08-teclas = Rotella del mouse
atalho-08 = avvicina e allontana, verso il cursore
atalho-09-teclas = Scheda Oggetti: clic, Maiusc + clic
atalho-09 = seleziona, aggiunge alla selezione; il doppio clic inquadra l'oggetto
atalho-10-teclas = Libreria: clic e clic sul pavimento
atalho-10 = posiziona un oggetto (Maiusc mantiene lo strumento)
atalho-11-teclas = Libreria: trascinare fino al pavimento
atalho-11 = posiziona un oggetto dove lo si rilascia
atalho-12-teclas = Frecce
atalho-12 = sposta la selezione di un passo della griglia
atalho-13-teclas = Canc
atalho-13 = elimina la selezione
atalho-14-teclas = C
atalho-14 = collega il blocco selezionato al prossimo cliccato
atalho-15-teclas = Esc
atalho-15 = annulla lo strumento, chiude i dialoghi ed esce dalle telecamere segui e prima persona
atalho-16-teclas = F
atalho-16 = inquadra il modello (torna alla telecamera in orbita)
atalho-17-teclas = 1, 2, 3, 4
atalho-17 = telecamera: orbita, segui una risorsa, prima persona, dalla risorsa
atalho-18-teclas = W A S D
atalho-18 = cammina in prima persona (Maiusc corre, la rotella cambia la velocità)
atalho-19-teclas = L
atalho-19 = blocca il mouse in prima persona
atalho-20-teclas = F1
atalho-20 = mostra o nasconde questa finestra
atalho-21-teclas = F3
atalho-21 = mostra o nasconde i tempi dei fotogrammi
atalho-22-teclas = F4
atalho-22 = attiva o disattiva i livelli di dettaglio
atalho-23-teclas = Ctrl + A
atalho-23 = seleziona tutti gli oggetti
atalho-24-teclas = Ctrl + C
atalho-24 = copia la selezione
atalho-25-teclas = Ctrl + X
atalho-25 = taglia la selezione
atalho-26-teclas = Ctrl + V
atalho-26 = incolla (nomi unici, collegamenti interni conservati)
atalho-27-teclas = Ctrl + D
atalho-27 = duplica la selezione
atalho-28-teclas = Ctrl + Z
atalho-28 = annulla
atalho-29-teclas = Ctrl + Y o Ctrl + Maiusc + Z
atalho-29 = ripete
atalho-30-teclas = Ctrl + S
atalho-30 = salva
atalho-31-teclas = Ctrl + Maiusc + S
atalho-31 = salva con un altro nome (Salva con nome)
atalho-32-teclas = Ctrl + N
atalho-32 = nuovo modello

# Parte 2: il pannello delle proprietà (properties/).

## Proprietà: famiglie di distribuzione, regole e tipi

dist-constante = Costante
dist-uniforme = Uniforme
dist-exponencial = Esponenziale
dist-triangular = Triangolare
dist-normal = Normale
dist-lognormal = Lognormale
dist-gama = Gamma
dist-erlang = Erlang
dist-weibull = Weibull
dist-beta = Beta
dist-log-logistica = Log-logistica
dist-pearson-v = Pearson V
dist-pearson-vi = Pearson VI
dist-discreta = Discreta
dist-empirica = Empirica
dist-poisson = Poisson
dist-geometrica = Geometrica
dist-binomial = Binomiale
dist-binomial-negativa = Binomiale negativa
dist-deslocada = Traslata
prop-vaga-primeira-livre = Prima libera
prop-vaga-aleatoria = Casuale
prop-despacho-primeiro-livre = Primo libero
prop-despacho-rodizio = A rotazione
prop-despacho-mais-proximo = Più vicino
tipo-chegada = Arrivo
tipo-fila = Coda
tipo-processo = Processo
tipo-montagem = Assemblaggio
tipo-desmontagem = Disassemblaggio
tipo-saida = Uscita
tipo-transportador = Trasportatore
tipo-ponte-rolante = Carroponte
tipo-despachante = Smistatore
tipo-esteira = Nastro trasportatore
tipo-porta-paletes = Scaffale portapallet
prop-modo-igual-ao-padrao = come il predefinito
prop-modo-distribuicao = distribuzione
prop-modo-taxa = tasso
prop-modo-sem-chegadas = senza arrivi
prop-turno-pausar = Sospendere
prop-turno-terminar = Terminare
prop-turno-pausar-dica = Il lavoro in corso si ferma a fine turno e riprende quando il turno ritorna
prop-turno-terminar-dica = A fine turno l'oggetto non accetta nuovi articoli, termina l'articolo in corso (straordinario) e solo allora esce dal turno; il turno seguente comincia all'ora di sempre

## properties/network.rs

prop-network-o_nome_nao_pode_ficar = il nome non può restare vuoto
prop-network-ja_existe_um_no_chamado = esiste già un nodo chiamato "{ $t }"
prop-network-nome = Nome
prop-network-no_da_rede_de_caminhos = Nodo della rete di percorsi
prop-network-arestas = Archi
prop-network-nenhuma_use_a_ferramenta_aresta = Nessuno: usa lo strumento "Arco" e fai clic su un altro nodo.
prop-network-remover_aresta = Rimuovi arco
prop-network-objetos_ligados_a_este_no = Oggetti collegati a questo nodo
prop-network-nenhum_cada_objeto_atendido_entra = Nessuno (ogni oggetto servito entra dal nodo più vicino).
prop-network-aresta = Arco { $edge }
prop-network-comprimento_m_calculado_pelas_posicoes = Lunghezza: { $len } m (calcolata dalle posizioni dei nodi)
prop-network-mao_dupla = Doppio senso ({ $from } <-> { $to })
prop-network-mao_unica = Senso unico ({ $from } -> { $to })
prop-network-mao_unica-2 = Senso unico ({ $to } -> { $from })
prop-network-sentido = Senso
prop-network-limitar_veiculos_agv = Limita i veicoli (AGV)
prop-network-quantos_agvs_podem_estar_na = Quanti AGV possono stare sull'arco nello stesso momento
prop-network-sem_limite = senza limite
prop-network-so_agvs_agv_true_respeitam = Solo gli AGV (agv: true) rispettano la capacità; gli altri passano liberamente.
prop-network-objeto_discreto = Oggetto discreto
prop-network-escolha = (scegli)
prop-network-nao_e_um_objeto_discreto = "{ $current }" non è un oggetto discreto del modello
prop-network-vazao = Portata
prop-network-capacidade = Capacità
prop-network-nivel_inicial = Livello iniziale
prop-network-entrada_maxima = Ingresso massimo
prop-network-saida_maxima = Uscita massima
prop-network-vazao_maxima = Portata massima
prop-network-cada_item_que_entra_no = Ogni articolo che entra nell'oggetto diventa questo volume, consegnato in una volta.
prop-network-volume_por_item = Volume per articolo
prop-network-a_cada_volume_completo_cria = A ogni volume completo, crea un articolo dentro l'oggetto.
prop-network-problema = Problema: { $message }
prop-network-tipo = Tipo
prop-network-ligacoes = Collegamenti
prop-network-nenhuma_use_a_ferramenta_ligar = Nessuno: usa lo strumento "Collega" e fai clic sul nodo successivo.
prop-network-remover_ligacao = Rimuovi collegamento

## properties/panel.rs

prop-panel-buscar = Cerca
prop-panel-texto_invalido_nao_aplicado = Testo non valido, non applicato: { $error }
prop-panel-descartar_texto = scarta il testo
prop-panel-volta_ao_valor_que_esta = Torna al valore che è nel modello
prop-panel-nao_e_um_numero = "{ $s }" non è un numero
prop-panel-infinita = infinita
prop-panel-valor = valore
prop-panel-peso = peso
prop-panel-remover = rimuovi
prop-panel-adicionar_valor = aggiungi valore
prop-panel-observacoes_em_ordem_crescente_separadas = Osservazioni in ordine crescente, separate da spazi:
prop-panel-deslocamento = traslazione
prop-panel-min = min.
prop-panel-max = max.
prop-panel-media = media
prop-panel-moda = moda
prop-panel-desvio = deviazione
prop-panel-forma = forma
prop-panel-escala = scala
prop-panel-alfa = alfa
prop-panel-beta = beta
prop-panel-forma_1 = forma 1
prop-panel-forma_2 = forma 2
prop-panel-media-2 = media = { $mean }
prop-panel-sem_limite = senza limite
prop-panel-sem_limite_velocidade_constante = senza limite (velocità costante)
prop-panel-equipe_transportadores_do_modelo = Squadra (trasportatori del modello):
prop-panel-nenhum_transportador_no_modelo_crie = Nessun trasportatore nel modello. Creane uno nella tavolozza.
prop-panel-nao_e_um_transportador = "{ $missing }" non è un trasportatore
prop-panel-a_equipe_nao_pode_ficar = La squadra non può restare vuota.
prop-panel-precisa_de_operadores = Richiede operatori
prop-panel-crie_um_transportador_ou_despachante = Crea un Trasportatore o uno Smistatore da usare come squadra.
prop-panel-equipe = squadra
prop-panel-nenhuma = (nessuna)
prop-panel-nao_e_um_transportador_nem = "{ $team }" non è né un trasportatore né uno smistatore
prop-panel-quantidade = quantità
prop-panel-durante_a_preparacao = durante la preparazione
prop-panel-durante_o_processamento = durante la lavorazione
prop-panel-transporte_de_saida = Trasporto in uscita
prop-panel-nenhum = nessuno
prop-panel-os_itens_passam_direto_ao = Gli articoli passano direttamente al blocco successivo.
prop-panel-nao_e_um_transportador_nem-2 = "{ $t }" non è né un trasportatore né uno smistatore
prop-panel-sem_rede_de_caminhos_a = Nessuna rete di percorsi: la base è la posizione del trasportatore.
prop-panel-base_no_da_rede = Base (nodo della rete)
prop-panel-fora_da_rede = fuori dalla rete
prop-panel-escolher_um_no_poe_o = Scegliere un nodo vi colloca il trasportatore e attiva "Torna alla base".
prop-panel-editar_como_texto = modifica come testo
prop-panel-quantidade_de_itens_exigida_em = Quantità di articoli richiesta a ogni ingresso di componenti:
prop-panel-entrada = ingresso { $i }
prop-panel-adicionar_entrada = aggiungi ingresso
prop-panel-o_objeto_precisa_de_ligacoes = L'oggetto richiede { $list } collegamenti in ingresso (1 contenitore + { $list2 }).
prop-panel-valor_do_rotulo = Valore dell'etichetta
prop-panel-remover_rotulo = rimuovi etichetta
prop-panel-nome_do_rotulo = nome dell'etichetta
prop-panel-adicionar_rotulo = aggiungi etichetta
prop-panel-lote_agrupar_itens = Lotto (raggruppa articoli)
prop-panel-tamanho_do_lote = dimensione del lotto
prop-panel-espera_maxima_0_esperar_sempre = attesa massima (0 = attendere sempre)
prop-panel-fechar_a_entrada_ate_o = chiudi l'ingresso finché il lotto non esce
prop-panel-primeira_livre = Prima libera
prop-panel-rodizio = A rotazione
prop-panel-aleatoria = Casuale
prop-panel-pesos = Pesi
prop-panel-por_rotulo = Per etichetta
prop-panel-script = Script
prop-panel-regra_de_saida = Regola di uscita
prop-panel-peso_de_cada_saida_na = Peso di ogni uscita (nell'ordine dei collegamenti):
prop-panel-saida = uscita { $i } ({ $to })
prop-panel-saida-2 = uscita { $i }
prop-panel-ha_pesos_mas_o_objeto = Ci sono { $w } pesi, ma l'oggetto ha { $n_out } uscite.
prop-panel-ajustar_ao_numero_de_saidas = adegua al numero di uscite
prop-panel-rotulo_com_o_numero_da = etichetta con il numero della porta
prop-panel-o_nome_do_rotulo_nao = il nome dell'etichetta non può restare vuoto
prop-panel-script_que_devolve_o_numero = Script che restituisce il numero della porta (1, 2, ...)
prop-panel-script_ao_entrar_um_item = Script all'ingresso di un articolo (on_entry)
prop-panel-roda_para_cada_item_que = Viene eseguito per ogni articolo che entra, prima di qualsiasi altra regola.
prop-panel-quebras_mtbf_mttr = Guasti (MTBF/MTTR)
prop-panel-tempo_entre_falhas_mtbf = Tempo tra guasti (MTBF)
prop-panel-tempo_de_reparo_mttr = Tempo di riparazione (MTTR)
prop-panel-transferencia_pela_distancia = Trasferimento in base alla distanza
prop-panel-os_itens_levam_atraso_distancia = Gli articoli impiegano ritardo + distanza / velocità per raggiungere il blocco successivo attraverso i collegamenti diretti (senza trasportatore)
prop-panel-velocidade = Velocità
prop-panel-atraso = Ritardo
prop-panel-caminho = Percorso
prop-panel-reta = diretto
prop-panel-pela_rede = attraverso la rete
prop-panel-tempo_entre_chegadas = Tempo tra arrivi
prop-panel-primeira_chegada_em_t_0 = Primo arrivo a t = 0
prop-panel-maximo_de_chegadas = Numero massimo di arrivi
prop-panel-se_a_saida_estiver_bloqueada = Se l'uscita è bloccata
prop-panel-esperar = Attendere
prop-panel-descartar_perda = Scartare (perdita)
prop-panel-rotulos = Etichette
prop-panel-capacidade_limitada = Capacità limitata
prop-panel-disciplina = Disciplina
prop-panel-fifo_primeiro_a_entrar_primeiro = FIFO (primo a entrare, primo a uscire)
prop-panel-lifo_ultimo_a_entrar_primeiro = LIFO (ultimo a entrare, primo a uscire)
prop-panel-capacidade = Capacità
prop-panel-preparacao_setup = Preparazione (setup)
prop-panel-tempo_de_processamento = Tempo di lavorazione
prop-panel-operadores = Operatori
prop-panel-modo = Modo
prop-panel-embalar_so_o_conteiner_sai = Imballare (esce solo il contenitore)
prop-panel-juntar_componentes_destruidos = Unire (componenti distrutti)
prop-panel-lote_todos_saem = Lotto (escono tutti)
prop-panel-desembalar = Disimballare
prop-panel-dividir_em_copias = Dividere in copie
prop-panel-numero_de_itens_gerados = numero di articoli generati
prop-panel-a_saida_nao_tem_parametros = L'uscita non ha parametri.
prop-panel-velocidade_maxima = Velocità massima
prop-panel-aceleracao = Accelerazione
prop-panel-desaceleracao = Decelerazione
prop-panel-capacidade_itens = Capacità (articoli)
prop-panel-tempo_de_carga = Tempo di carico
prop-panel-tempo_de_descarga = Tempo di scarico
prop-panel-voltar_a_base_quando_ocioso = Torna alla base quando inattivo
prop-panel-agv_controle_de_trafego_na = AGV (controllo del traffico sulla rete)
prop-panel-comprimento = Lunghezza
prop-panel-comprimento_por_item = Lunghezza per articolo
prop-panel-acumula_itens_quando_a_saida = Accumula articoli quando l'uscita si blocca
prop-panel-capacidade_itens-2 = Capacità: { $item_length } articoli
prop-panel-colunas = Colonne
prop-panel-niveis = Livelli
prop-panel-permanencia_minima = Permanenza minima
prop-panel-regra_de_ocupacao = Regola di occupazione
prop-panel-largura_da_coluna = Larghezza della colonna
prop-panel-altura_do_nivel = Altezza del livello
prop-panel-capacidade_posicoes = Capacità: { $levels } posizioni
prop-panel-regra_de_escolha = Regola di scelta
prop-panel-o_nome_nao_pode_ficar = il nome non può restare vuoto
prop-panel-ja_existe_um_objeto_chamado = esiste già un oggetto chiamato "{ $t }"
prop-panel-nome = Nome
prop-panel-problema = Problema: { $message }
prop-panel-padrao_do_tipo = (predefinito del tipo)
prop-panel-aparencia = Aspetto
prop-panel-ativo = Risorsa:
prop-panel-clipes_de_animacao_ainda_nao = Le clip di animazione non vengono ancora riprodotte: posa di riposo.
prop-panel-biblioteca_do_utsim = Libreria di UTSim
prop-panel-utsim_gerado_por_utsim_assets = UTSim (generato da utsim-assets)
prop-panel-terceiros_assets_terceiros = Terze parti (assets/terceiros)
prop-panel-cor = Colore
prop-panel-escala-2 = Scala
prop-panel-giro_graus = Rotazione (gradi)
prop-panel-restaurar_padrao = Ripristina predefinito
prop-panel-volta_a_aparencia_padrao_do = Torna all'aspetto predefinito del tipo di oggetto
prop-panel-simulacao = Simulazione
prop-panel-tempo_de_simulacao = Tempo di simulazione
prop-panel-aquecimento = Riscaldamento
prop-panel-semente = Seme
prop-panel-replicacoes = Repliche

## properties/position.rs

prop-position-posicao_m = Posizione (m)
prop-position-giro_graus = Rotazione (gradi)

## properties/schedule.rs

prop-schedule-segundos = secondi
prop-schedule-minutos = minuti
prop-schedule-horas = ore
prop-schedule-use_dia_n_hh_mm = usa "dia N hh:mm" (ricevuto "{ $t }")
prop-schedule-dia_invalido_em_a_partir = giorno non valido in "{ $t }" (a partire da 1)
prop-schedule-use_um_numero = usa un numero
prop-schedule-hh_mm_so_vale_com = hh:mm vale solo con un ciclo di un giorno o di una settimana (1440 min, 24 h, 86400 s); usa un numero
prop-schedule-hora_invalida_em = ora non valida in "{ $t }"
prop-schedule-minuto_invalido_em = minuto non valido in "{ $t }"
prop-schedule-minuto_invalido_em_0_a = minuto non valido in "{ $t }" (da 0 a 59)
prop-schedule-nao_e_um_horario_hh = "{ $t }" non è un orario (hh:mm) né un numero

## properties/schedule_ui.rs

prop-schedule-ui-media = { $family }, media { $mean }
prop-schedule-ui-chegadas_por_periodo = Arrivi per periodo
prop-schedule-ui-a_taxa_de_chegada_muda = il tasso di arrivo cambia con l'ora
prop-schedule-ui-uma_tabela_de_periodos_que = Una tabella di periodi che si ripete a ogni ciclo (un giorno, una settimana). Fuori dai periodi non ci sono arrivi.
prop-schedule-ui-desligado_o_tempo_entre_chegadas = Spento: il tempo tra arrivi qui sopra vale sempre.
prop-schedule-ui-ciclo = ciclo
prop-schedule-ui-dia = giorno
prop-schedule-ui-semana = settimana
prop-schedule-ui-ciclo_de_tempo_do_modelo = ciclo di { $v } (tempo del modello in minuti)
prop-schedule-ui-ciclo_de_tempo_do_modelo-2 = Ciclo di { $cycle }: tempo del modello in { $name }. Orari in hh:mm, tassi all'ora.
prop-schedule-ui-horarios_e_taxas_em_unidades = Orari e tassi in unità del modello (con ciclo 1440, un giorno in minuti, la tabella mostra hh:mm).
prop-schedule-ui-taxa_por_hora = tasso all'ora
prop-schedule-ui-taxa = tasso
prop-schedule-ui-inicio = inizio
prop-schedule-ui-fim = fine
prop-schedule-ui-modo = modo
prop-schedule-ui-valor = valore
prop-schedule-ui-esperado = atteso
prop-schedule-ui-copiar_do_padrao = copia dal predefinito
prop-schedule-ui-usa_no_periodo_uma_copia = Usa nel periodo una copia del tempo tra arrivi predefinito, per regolarlo solo qui
prop-schedule-ui-remover_o_periodo = rimuovi il periodo
prop-schedule-ui-adicionar_periodo = aggiungi periodo
prop-schedule-ui-por_dia = {" "}(al giorno)
prop-schedule-ui-por_semana = {" "}(alla settimana)
prop-schedule-ui-esperado_por_ciclo_chegadas = Atteso per ciclo{ $hint }: { $per_cycle } arrivi
prop-schedule-ui-na_troca_de_periodo_a = Al cambio di periodo l'arrivo in sospeso viene estratto di nuovo con il processo del nuovo periodo (periodi consecutivi con lo stesso processo mantengono l'arrivo).
prop-schedule-ui-distribuicoes_dos_periodos = Distribuzioni dei periodi
prop-schedule-ui-periodo_tempo_entre_chegadas = Periodo { $i }: tempo tra arrivi
prop-schedule-ui-rotulos_por_periodo = Etichette per periodo
prop-schedule-ui-os_rotulos_de_um_periodo = Le etichette di un periodo sostituiscono le etichette predefinite con lo stesso nome (per esempio, più urgenti di notte); le altre continuano a valere.
prop-schedule-ui-a_fonte_nao_tem_rotulos = La sorgente non ha etichette predefinite.
prop-schedule-ui-a = da { $a } a { $b }
prop-schedule-ui-a-2 = da { $start } a { $end }
prop-schedule-ui-periodo = Periodo { $i } ({ $span })

## properties/shifts_ui.rs

prop-shifts-ui-como_no_calendario = come nel calendario ({ $to_lowercase })
prop-shifts-ui-ao_fim_do_turno = A fine turno
prop-shifts-ui-periodo = periodo
prop-shifts-ui-inicio = inizio
prop-shifts-ui-duracao = durata
prop-shifts-ui-remover = rimuovi
prop-shifts-ui-adicionar_janela = aggiungi finestra
prop-shifts-ui-fora_de_turno_do_periodo = fuori turno: { $off_fraction }% del periodo
prop-shifts-ui-turnos_janelas_fora_de_turno = Turni (finestre fuori turno)
prop-shifts-ui-usar_calendario = usa calendario
prop-shifts-ui-crie_calendarios_na_secao_calendarios = Crea calendari nella sezione Calendari del pannello Simulazione (fai clic su un punto vuoto della scena)
prop-shifts-ui-personalizado = personalizzato
prop-shifts-ui-periodo_janela_s_fora_de = periodo { $period }, { $off } finestra/e fuori turno
prop-shifts-ui-o_calendario_nao_existe = il calendario '{ $name }' non esiste
prop-shifts-ui-nome_do_calendario = nome del calendario
prop-shifts-ui-salvar_como_calendario = salva come calendario
prop-shifts-ui-outros_objetos_podem_usar_o = Altri oggetti possono usare lo stesso calendario
prop-shifts-ui-calendarios = Calendari ({ $calendars })
prop-shifts-ui-tabelas_de_turno_com_nome = Tabelle dei turni con nome. Un oggetto ne usa una in Turni > usa calendario; cambiare il calendario cambia tutti gli oggetti che lo usano.
prop-shifts-ui-nenhum_objeto_usa_este_calendario = nessun oggetto usa questo calendario
prop-shifts-ui-usado_por = usato da: { $users }
prop-shifts-ui-remover_calendario = rimuovi calendario
prop-shifts-ui-troque_os_objetos_para_outro = Prima passa gli oggetti a un altro calendario o a personalizzato
prop-shifts-ui-novo_calendario = nuovo calendario
prop-shifts-ui-o_nome_nao_pode_ficar = il nome non può restare vuoto
prop-shifts-ui-ja_existe_um_calendario_chamado = esiste già un calendario chiamato "{ $t }"
prop-shifts-ui-nome = Nome

## properties/spec.rs

prop-spec-continuas = Continue
prop-spec-discretas_e_empirica = Discrete ed empirica
prop-spec-combinacao = Combinazione

# Parte 3: os resultados (results.rs, results_panel.rs)
tipo-fluxo = Flusso
tipo-fluidos = Fluidi
tipo-medicao-de-tempo = Misura del tempo
tipo-outro = Altro
res-status-replicacoes = { $n ->
    [one] { $n } replica completata
   *[other] { $n } repliche completate
}
res-status-replicacoes-gargalo = { $n ->
    [one] { $n } replica completata
   *[other] { $n } repliche completate
}; collo di bottiglia: { $gargalo }
res-status-pontos = { $n ->
    [one] { $n } punto valutato
   *[other] { $n } punti valutati
}
res-status-pontos-melhor = { $n ->
    [one] { $n } punto valutato
   *[other] { $n } punti valutati
}; migliore: { $melhor }

## results.rs

res-replicacao_oes_de_t_0 = { $replications ->
    [one] { $replications } replica
   *[other] { $replications } repliche
}, da t = 0 a t = { $stop_time } con riscaldamento fino a t = { $warmup }, seme { $seed }
res-ocupacao = Utilizzo
res-incluindo_bloqueado = Incluso bloccato
res-dentro_do_turno = Nel turno
res-sem_o_tempo_bloqueado = Senza il tempo bloccato
res-fracao_do_tempo_trabalhando_em = Frazione del tempo in lavoro (lavorazione o attrezzaggio). Con capacità maggiore di 1, è la media dei serventi occupati divisa per la capacità.
res-ocupacao_mais_o_tempo_em = Utilizzo più il tempo in cui la risorsa trattiene un pezzo finito che non può uscire.
res-ocupacao_sobre_o_tempo_em = Utilizzo sul tempo di turno: il tempo fuori turno esce dal denominatore.
res-ocupacao_sobre_o_tempo_nao = Utilizzo sul tempo non bloccato: mostra il carico proprio della risorsa.
res-objeto = Oggetto
res-tipo = Tipo
res-entradas = Ingressi
res-saidas = Uscite
res-conteudo_medio = Contenuto medio
res-permanencia_media = Permanenza media
res-c_bloqueio = con blocco
res-no_turno = nel turno
res-s_bloqueio = senza blocco
res-velocidade_da_transferencia = { $name }: velocità del trasferimento
res-simulacao = Simulazione: { $label }
res-simulacao_velocidade_padrao_das_transferencias = Simulazione: velocità predefinita dei trasferimenti
res-cenario = scenario { $n }
res-adicione_pelo_menos_um_cenario = aggiungi almeno uno scenario
res-cenario_de_um_nome = scenario { $i }: dagli un nome
res-cenario_nome_repetido = scenario '{ $name }': nome ripetuto
res-cenario-2 = scenario '{ $name }': { $e }
res-restricao_de_nao_e_um = vincolo su '{ $metric }': { $what } '{ $text }' non è un numero
res-escolha_a_metrica_do_objetivo = scegli la metrica dell’obiettivo
res-minimo = minimo
res-maximo = massimo
res-restricao_de_informe_minimo_ou = vincolo su '{ $metric }': indica minimo o massimo

## results_panel.rs

res-painel-resumo = Riepilogo
res-painel-graficos = Grafici
res-painel-cenarios = Scenari
res-painel-otimizacao = Ottimizzazione
res-painel-ocupacao = Utilizzo
res-painel-estados = Stati
res-painel-serie_temporal = Serie temporale
res-painel-histograma = Istogramma
res-painel-a_execucao_terminou_sem_resultado = l’esecuzione è terminata senza risultato
res-painel-cenario_s_comparados = { $scenarios ->
    [one] { $scenarios } scenario confrontato
   *[other] { $scenarios } scenari confrontati
}
res-painel-cancelado = { $what }: annullato
res-painel-erro = { $what }: errore
res-painel-rodando_replicacoes = Esecuzione delle repliche
res-painel-comparando_cenarios = Confronto degli scenari
res-painel-otimizando = Ottimizzazione in corso
res-painel-resultados = Risultati
res-painel-o_modelo_tem_problema_s = Il modello ha { $blocked ->
    [one] { $blocked } problema
   *[other] { $blocked } problemi
}: correggili in Modifica prima di eseguire.
res-painel-de = { $what }: { $done } di { $total }
res-painel-cancelar = Annulla
res-painel-rodada = Esecuzione
res-painel-replicacoes = Repliche
res-painel-semente = Seme
res-painel-tempo_de_parada = Tempo di arresto
res-painel-aquecimento = Riscaldamento
res-painel-series_temporais_a_cada = Serie temporali ogni
res-painel-histogramas_com_classes = Istogrammi con classi
res-painel-rodar_replicacoes = Esegui le repliche
res-painel-gravar_no_modelo = Salva nel modello
res-painel-grava_replicacoes_tempos_e_semente = Salva repliche, tempi e seme nel modello (Ctrl+Z annulla)
res-painel-restaurar_do_modelo = Ripristina dal modello
res-painel-rode_as_replicacoes_para_ver = Esegui le repliche per vedere il riepilogo per oggetto, il collo di bottiglia e i grafici.
res-painel-cada_valor_media_entre_replicacoes = { $describe }. Ogni valore: media tra le repliche ± semiampiezza dell’IC al 95%.
res-painel-gargalo_geral = Collo di bottiglia (generale):
res-painel-ocupacao_a_maior_entre_processos = (utilizzo { $content }; il più alto tra processi e risorse)
res-painel-gargalo_dentro_do_turno = Collo di bottiglia (nel turno):
res-painel-ocupacao_no_tempo_em_turno = (utilizzo { $content } sul tempo di turno)
res-painel-tipo = Tipo:
res-painel-todos = tutti
res-painel-clique_no_titulo_de_uma = Fai clic sul titolo di una colonna per ordinare.
res-painel-decr = {" "}(decr.)
res-painel-cresc = {" "}(cresc.)
res-painel-metricas_proprias_tempo_no_sistema = Metriche proprie (tempo nel sistema, flussi, livelli dei serbatoi...)
res-painel-pasta_de_exportacao = Cartella di esportazione:
res-painel-exportar_csv = Esporta CSV
res-painel-resumo_csv_objetos_csv_replicacoes = resumo.csv, objetos.csv, replicacoes.csv e, se presenti, series.csv e histogramas.csv
res-painel-exportar_html = Esporta HTML
res-painel-relatorio_com_tabelas_e_graficos = Rapporto con tabelle e grafici (include il confronto degli scenari, se presente)
res-painel-a_exportacao_de_arquivos_existe = L’esportazione dei file esiste solo nell’applicazione desktop.
res-painel-csv_gravado_em = CSV salvato in { $dir }: { $files }
res-painel-nao_foi_possivel_gravar_em = impossibile scrivere in { $dir }: { $e }
res-painel-relatorio_gravado_em = Rapporto salvato in { $file }
res-painel-nao_foi_possivel_gravar = impossibile scrivere { $file }: { $e }
res-painel-rode_as_replicacoes_na_aba = Esegui le repliche nella scheda Riepilogo per vedere i grafici.
res-painel-o_modelo_nao_tem_processos = Il modello non ha processi né risorse mobili.
res-painel-de_cada_recurso_com_o = { $label } di ogni risorsa (%), con l’intervallo al 95%, mai sopra il 100% per servente. In arancione, il collo di bottiglia (utilizzo più alto).
res-painel-fracao_do_tempo_em_cada = Frazione del tempo in ogni stato (%), con i colori del 3D.
res-painel-objeto = Oggetto:
res-painel-este_objeto_nao_tem_observacoes = Questo oggetto non ha osservazioni di permanenza.
res-painel-a_rodada_foi_feita_sem = L’esecuzione è stata fatta senza istogrammi.
res-painel-medida = Misura:
res-painel-ativar_series_temporais_e_rodar = Attiva le serie temporali ed esegui di nuovo
res-painel-ativar_histogramas_e_rodar_de = Attiva gli istogrammi ed esegui di nuovo
res-painel-ic_95 = IC 95%
res-painel-sem_estados_registrados = Nessuno stato registrato.
res-painel-este_objeto_nao_tem_pontos = Questo oggetto non ha punti campionati.
res-painel-a_rodada_foi_feita_sem-2 = L’esecuzione è stata fatta senza serie temporali (run.sample_interval disattivato).
res-painel-conteudo_de_ao_longo_do = Contenuto di { $object } nel tempo: media delle repliche (linea spessa) e ogni replica.
res-painel-observacoes_de_todas_as_replicacoes = { $total } osservazioni da tutte le repliche, da { $h } a { $h2 }.
res-painel-parametro = Parametro:
res-painel-atual = { $key } (attuale: { $current })
res-painel-ou_chave = o chiave:
res-painel-objeto_campo = oggetto.campo
res-painel-adicionar_parametro = Aggiungi parametro
res-painel-cada_linha_e_um_cenario = Ogni riga è uno scenario; ogni colonna, un parametro. Una cella vuota mantiene il valore del modello. Valori in testo RON, come nel campo set degli esperimenti.
res-painel-experimento = Esperimento:
res-painel-cenario = Scenario
res-painel-remover_parametro = Rimuovi parametro
res-painel-remover_cenario = Rimuovi scenario
res-painel-adicionar_cenario = Aggiungi scenario
res-painel-metricas_separadas_por_virgula_vazio = Metriche (separate da virgola; vuoto: tutte):
res-painel-rodar_cenarios = Esegui gli scenari
res-painel-replicacao_oes_por_cenario_aba = { $reps ->
    [one] { $reps } replica
   *[other] { $reps } repliche
} per scenario (scheda Riepilogo), stessi semi per tutti
res-painel-media_ic_de_95_e = { $name }: media ± IC al 95%. Δ è la differenza appaiata (stessi semi) rispetto a '{ $name2 }'; ≈ indica differenza non significativa, in arancione quelle significative.
res-painel-filtrar_metricas = Filtra metriche:
res-painel-metrica = Metrica
res-painel-vs = Δ vs { $name }
res-painel-mostrar_no_grafico = Mostra nel grafico
res-painel-grafico_da_metrica = Grafico della metrica:
res-painel-arquivo_de_experimento = File dell’esperimento:
res-painel-abrir_experimento = Apri esperimento
res-painel-experimento_aberto = Esperimento '{ $name }' aperto
res-painel-experimento_aberto_os_cenarios_valem = Esperimento '{ $name }' aperto; gli scenari valgono per il modello aperto (il file punta a { $model_path })
res-painel-nao_foi_possivel_abrir = impossibile aprire: { $e }
res-painel-salvar_experimento = Salva esperimento
res-painel-salve_o_modelo_antes_o = salva prima il modello: l’esperimento punta al file del modello
res-painel-experimento_salvo_em = Esperimento salvato in { $target }
res-painel-variaveis_de_decisao_mesmas_chaves = Variabili decisionali (stesse chiavi degli scenari), obiettivo e vincoli. Tutti i punti usano gli stessi semi; il migliore è confrontato con gli altri con un test appaiato.
res-painel-variavel = Variabile
res-painel-dominio = Dominio
res-painel-minimo = Minimo
res-painel-maximo = Massimo
res-painel-passo_valores = Passo / valori
res-painel-custo_por_unidade = Costo per unità
res-painel-inteiro = intero
res-painel-continuo = continuo
res-painel-lista = lista
res-painel-objetivo = Obiettivo:
res-painel-minimizar = minimizzare
res-painel-maximizar = massimizzare
res-painel-peso_da_metrica = peso della metrica
res-painel-custos_das_variaveis = + costi delle variabili
res-painel-restricoes_sobre_a_media_da = Vincoli (sulla media della metrica)
res-painel-minimo-2 = minimo
res-painel-maximo-2 = massimo
res-painel-adicionar_restricao = Aggiungi vincolo
res-painel-metodo = Metodo:
res-painel-grade = griglia
res-painel-recozimento_simulado = ricottura simulata
res-painel-genetico = genetico
res-painel-iteracoes = iterazioni
res-painel-populacao = popolazione
res-painel-geracoes = generazioni
res-painel-replicacoes_por_ponto = Repliche per punto
res-painel-orcamento_pontos = Budget (punti)
res-painel-semente_da_busca = Seme della ricerca
res-painel-refinar_os = Affina i
res-painel-melhores_com = migliori con
res-painel-replicacoes-2 = repliche
res-painel-otimizar = Ottimizza
res-painel-melhor_ponto = Punto migliore:
res-painel-objetivo-2 = obiettivo { $objective }
res-painel-aplicar_ao_modelo = Applica al modello
res-painel-grava_estes_valores_no_modelo = Salva questi valori nel modello (Ctrl+Z annulla)
res-painel-nao_atende = non soddisfatto
res-painel-media_observada = { $bound } (media osservata: { $cell })
res-painel-nenhum_ponto_avaliado_atende_as = Nessun punto valutato soddisfa i vincoli.
res-painel-convergencia_melhor_objetivo_viavel_por = Convergenza: miglior obiettivo ammissibile per numero di punti valutati
res-painel-pontos_avaliados = punti valutati
res-painel-melhor_ate_entao = migliore finora
res-painel-comparacao_pareada_com_o_melhor = Confronto appaiato con il migliore (≈: nessuna differenza significativa al 95%)
res-painel-ponto = Punto
res-painel-objetivo-3 = Obiettivo
res-painel-viavel = Ammissibile
res-painel-vs_melhor = Δ vs migliore
res-painel-equivalente = { $delta } equivalente
res-painel-objeto_metrica = oggetto.metrica

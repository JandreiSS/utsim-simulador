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

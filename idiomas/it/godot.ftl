# Traduzione automatica, in attesa di revisione umana.
# Tradução automática, aguardando revisão humana.

## UTSim 3D (cliente Godot): configurações e campos de distribuição

barra-configuracoes = Impostazioni
barra-configuracoes-dica = Preferenze di UTSim 3D su questo computer
configuracoes-titulo = Impostazioni
configuracoes-distribuicoes = I campi di distribuzione si aprono come
configuracoes-formulario = Modulo
configuracoes-codigo = Codice
configuracoes-distribuicoes-ajuda = Vale per tutti i campi di distribuzione (tempo di processo, arrivi, carico, scarico...). Il pulsante {"{"} {"}"} di ogni campo cambia ancora solo quel campo, finché non si sceglie un altro oggetto.
configuracoes-fechar = Chiudi
props-editar_como_texto = Modifica come codice (testo RON)
props-editar_como_formulario = Torna al modulo

## Painéis ao vivo em janela própria

vivo-soltar = Finestra
vivo-soltar-dica = Apre questo pannello in una finestra propria, che può andare su un altro monitor; resta in diretta
vivo-prender = Indietro
vivo-prender-dica = Chiude la finestra e rimette il pannello sopra la vista 3D
vivo-janela-titulo = UTSim 3D: { $objeto }

## Idioma

configuracoes-idioma = Lingua
configuracoes-idioma-ajuda = Per ora cambiano lingua solo la finestra Impostazioni, i pannelli dal vivo e le schede Qualità, Sicurezza e Manutenzione dei nodi; le altre schermate restano in portoghese.

## Curva de fadiga da pessoa

fadiga-secao = Curva di fatica (studio dei tempi)
fadiga-nota = Sola lettura. Le letture e l’adattamento si modificano in UTSim Estatística o con il comando utsim pessoa ajustar.
fadiga-tarefa = Compito { $tarefa }: { $leituras } letture in { $horarios } orari
fadiga-tarefa-metodo = Compito { $tarefa }, metodo { $metodo }: { $leituras } letture in { $horarios } orari
fadiga-resumo = Riposata: { $t0 }. Alla fatica massima, { $acrescimo } % in più. Fatica λ = { $lambda } all’ora; recupero μ = { $mu } all’ora ({ $origem }).
fadiga-estimada = stimata
fadiga-padrao = predefinita
fadiga-hora = ora del turno
fadiga-pausa = Pausa
fadiga-curva = Curva adattata
fadiga-media = Media misurata
fadiga-erro = Non adattata: { $erro }
fadiga-abrir = Apri in UTSim Estatística
fadiga-abrir-ajuda = Apre le letture di questo studio dei tempi in un foglio di UTSim Estatística (una riga per tempo misurato: compito, metodo, ora del turno e tempo) per analizzarle. Il modello non cambia.

## Abas dos nós (Qualidade, Segurança, Manutenção) e painéis ao vivo

aba-so-leitura = { $quem }. Sola lettura finché quella persona non chiude il modello.
aba-abrir-qualidade = Apri in UTSim Qualità
aba-abrir-qualidade-dica = Apre questo nodo in UTSim Qualità (difetti, ispezione, costi, SPC). Il modello viene salvato prima e resta all’altra applicazione per la modifica; UTSim 3D rilegge ciò che essa salva.
aba-abrir-seguranca = Apri in UTSim Sicurezza
aba-abrir-seguranca-dica = Apre questo nodo in UTSim Sicurezza sul lavoro (pericoli, matrice dei rischi, DPI). Il modello viene salvato prima e resta all’altra applicazione per la modifica; UTSim 3D rilegge ciò che essa salva.
aba-calculando = Calcolo dei numeri della simulazione...
aba-nenhuma = nessuna
aba-nenhum-escolha = (nessuno)
aba-nenhum = nessuno
aba-opcoes = Separa con virgole. Opzioni: { $opcoes }.
aba-vazio-nao-usa = vuoto: non usato
aba-adicionar = + Aggiungi
aba-adicionar-dica = Aggiunge un elemento a { $lista }
aba-abrir-item = Apre l’elemento
aba-fechar-item = Chiude l’elemento
aba-sem-nome = (senza nome)
aba-remover = Rimuovi
aba-remover-dica = Toglie questo elemento dall’elenco
vivo-fechar = Chiudi
vivo-fechar-dica = Chiudi il pannello (Esc chiude il più recente)
vivo-fixar = Fissa
vivo-fixado = Fissato
vivo-fixar-dica = Fissa mantiene questo pannello; il prossimo clic su un oggetto ne apre un altro accanto
vivo-sem-numeros = Nessun numero per questo oggetto.
vivo-eixo-tempo = tempo
vivo-eixo-itens = elementi
vivo-sem-dados = Ancora nessun dato.
vivo-estados = Stati nella finestra
vivo-sem-estados = nessuno stato

## Painel ao vivo: linhas (rótulo e ajuda)

vivo-em-processamento-agora = In lavorazione ora
vivo-itens-no-objeto-neste-instante = Elementi nell’oggetto in questo istante (in lavorazione o pronti senza poter uscire).
vivo-itens-em-espera-fila = Elementi in attesa (coda)
vivo-itens-nas-filas-que-alimentam = Elementi nelle code che alimentano l’oggetto, in questo istante.
vivo-entradas = Ingressi
vivo-itens-que-entraram-na-janela = Elementi entrati nella finestra.
vivo-ja-processados = Già lavorati
vivo-itens-que-sairam-do-objeto = Elementi usciti dall’oggetto nella finestra.
vivo-rejeitados-para-descarte = Scartati
vivo-itens-que-chegaram-a-uma = Elementi partiti da qui arrivati a un’uscita scarti (sink_role: Scrap), anche tramite code o trasporto.
vivo-nenhuma-saida-tem-sink-role = Nessuna uscita ha sink_role: Scrap; segna l’uscita scarti nel modello per contare gli scarti.
vivo-rejeitados-para-retrabalho = Scartati per rilavorazione
vivo-itens-que-chegaram-a-um = Elementi partiti da qui arrivati a una postazione di rilavorazione, anche tramite code o trasporto.
vivo-ciclo-medio-permanencia = Ciclo medio (permanenza)
vivo-tempo-medio-no-objeto-dos = Tempo medio nell’oggetto degli elementi usciti nella finestra.
vivo-espera-media-na-fila = Attesa media in coda
vivo-tempo-medio-nas-filas-que = Tempo medio nelle code che alimentano l’oggetto.
vivo-ocupado-processando = Occupato (in lavorazione)
vivo-fracao-da-janela-no-estado = Frazione della finestra nello stato in lavorazione.
vivo-ocioso = Inattivo
vivo-fracao-da-janela-sem-trabalho = Frazione della finestra senza lavoro.
vivo-bloqueado = Bloccato
vivo-pronto-sem-poder-sair-bloqueado = Pronto senza poter uscire (bloccato o in attesa di trasporto).
vivo-parado-quebra = Fermo (guasto)
vivo-quebrado-aguardando-reparo-ou-em = Guasto, in attesa di riparazione o in manutenzione.
vivo-setup = Attrezzaggio
vivo-fracao-em-setup = Frazione in attrezzaggio.
vivo-conteudo-agora = Contenuto ora
vivo-itens-no-objeto-neste-instante-2 = Elementi nell’oggetto in questo istante.
vivo-conteudo-medio = Contenuto medio
vivo-media-no-tempo-do-conteudo = Media nel tempo del contenuto, nella finestra.
vivo-conteudo-maximo = Contenuto massimo
vivo-maior-conteudo-da-janela = Contenuto più alto della finestra.
vivo-espera-media = Attesa media
vivo-saidas = Uscite
vivo-itens-que-sairam-na-janela = Elementi usciti nella finestra.
vivo-fracao-da-janela-sem-poder = Frazione della finestra senza poter rilasciare l’elemento creato.
vivo-recebidos-total = Ricevuti (totale)
vivo-itens-que-chegaram-a-saida = Elementi arrivati all’uscita nella finestra.
vivo-vazao-por-unidade-de-tempo = Portata (per unità di tempo)
vivo-recebidos-divididos-pela-duracao-da = Ricevuti divisi per la durata della finestra.
vivo-tempo-medio-no-sistema = Tempo medio nel sistema
vivo-da-criacao-do-item-a = Dalla creazione dell’elemento all’arrivo qui.
vivo-papel-na-qualidade = Ruolo nella qualità
vivo-o-papel-da-saida-nos = Il ruolo dell’uscita nei rapporti di qualità.
vivo-utilizacao = Utilizzo
vivo-andando-carregando-descarregando-ou-trabalhando = In movimento, carico, scarico o al lavoro in una postazione.
vivo-viagens = Viaggi
vivo-deslocamentos-comecados-na-janela = Spostamenti iniziati nella finestra.
vivo-itens-transportados = Elementi trasportati
vivo-itens-entregues-na-janela = Elementi consegnati nella finestra.
vivo-carregando-agora = Trasporta ora
vivo-itens-consigo-neste-instante = Elementi trasportati in questo istante.
vivo-viajando-carregado = Viaggia carico
vivo-fracao-da-janela-andando-com = Frazione della finestra in movimento con carico.
vivo-viajando-vazio = Viaggia vuoto
vivo-fracao-da-janela-andando-sem = Frazione della finestra in movimento senza carico.
vivo-trabalhando-num-posto = Al lavoro in una postazione
vivo-fracao-da-janela-operando-um = Frazione della finestra a gestire un processo (o un carroponte con pulsantiera o radio).
vivo-descanso-fadiga = Riposo (fatica)
vivo-fracao-da-janela-em-pausa = Frazione della finestra in pausa per recuperare.
vivo-pausas = Pause
vivo-vezes-que-entrou-em-descanso = Volte in cui è andato in riposo nella finestra.
vivo-fadiga-media-maxima-fim-da = Fatica media / massima (fine della replica)
vivo-a-fadiga-nao-esta-no = La fatica non è nella registrazione della riproduzione: valori del rapporto finale.
vivo-esperando-o-operador = Attende l’operatore
vivo-fracao-da-janela-parada-na = Frazione della finestra ferma sul carico finché la persona della pulsantiera o della radio non arriva accanto.

## Painel ao vivo: cabeçalho e seções

godot-ao-vivo-pessoa = persona
godot-ao-vivo-ponte_rolante = carroponte
godot-ao-vivo-empilhadeira = carrello elevatore
godot-ao-vivo-veiculo = veicolo
godot-ao-vivo-aquecimento_ate_numeros_desde_0 = riscaldamento fino a { $inicio_estatisticas }: numeri da 0
godot-ao-vivo-desde_o_fim_do_aquecimento = dalla fine del riscaldamento ({ $inicio_estatisticas })
godot-ao-vivo-desde_o_inicio = dall’inizio
godot-ao-vivo-t_de_replicacao = t = { $t } di { $fim } · replica { $replicacao } · { $janela }
godot-ao-vivo-agora = Ora
godot-ao-vivo-marque_a_saida_de_refugo = segna l’uscita scarti
godot-ao-vivo-acumulado = Cumulato
godot-ao-vivo-tempos = Tempi
godot-ao-vivo-em_processo_wip = In lavorazione (WIP)
godot-ao-vivo-agora_e_na_janela = Ora e nella finestra
godot-ao-vivo-conteudo = Contenuto
godot-ao-vivo-saida = Uscita
godot-ao-vivo-recebidos_acumulado = Ricevuti (cumulato)
godot-ao-vivo-pessoa-2 = Persona
godot-ao-vivo-ponte_rolante-2 = Carroponte
godot-ao-vivo-veiculo-2 = Veicolo
godot-ao-vivo-itens_consigo = Elementi trasportati
godot-ao-vivo-fila = Coda

## Fluxos de tokens

godot-ui-flows-fluxos_de_tokens = Flussi di token
godot-ui-flows-fechar = Chiudi
godot-ui-flows-fechar_os_fluxos = Chiudi i flussi
godot-ui-flows-o_modelo_nao_tem_fluxos = Il modello non ha flussi di token.
godot-ui-flows-tokens_em_cada_atividade_no = Token in ogni attività nell’istante della riproduzione; sotto, i conteggi lungo l’esecuzione (la linea verticale t è l’istante attuale).
godot-ui-flows-zoom_ctrl_roda_no_grafo = Zoom (Ctrl + rotella sul grafo)
godot-ui-flows-diminui = Riduci
godot-ui-flows-aumenta = Ingrandisci
godot-ui-flows-ajustar = Adatta
godot-ui-flows-cabe_na_largura = Sta nella larghezza
godot-ui-flows-sem_contagens = nessun conteggio

## Resultados

godot-ui-results-resumo = Riepilogo
godot-ui-results-graficos = Grafici
godot-ui-results-analise = Analisi
godot-ui-results-cenarios = Scenari
godot-ui-results-otimizacao = Ottimizzazione
godot-ui-results-ocupacao = Utilizzo
godot-ui-results-estados = Stati
godot-ui-results-serie_temporal = Serie temporale
godot-ui-results-histograma = Istogramma
godot-ui-results-nenhum_recurso_ocupado = nessuna risorsa occupata
godot-ui-results-invalido_use_um_numero_inteiro = { $what } "{ $text }" non valido: usa un numero intero
godot-ui-results-serie_invalida_use_objeto_medida = serie "{ $other }" non valida: usa oggetto.misura
godot-ui-results-calcule_antes_quantas_replicacoes_faltam = calcola prima quante repliche mancano
godot-ui-results-simule_o_modelo_antes_de = Simula il modello prima di confrontare gli scenari.
godot-ui-results-simule_o_modelo_antes_de-2 = Simula il modello prima di ottimizzare.
godot-ui-results-trabalho_desconhecido = lavoro sconosciuto: { $other }
godot-ui-results-serie_curta_demais_para_o = Serie troppo corta per l’MSER-5 ({ $points } punti): aumenta il tempo di arresto o il campionamento.
godot-ui-results-corte_sugerido_mas_o_minimo = Taglio suggerito: { $cut }, ma il minimo è nella seconda metà della serie: l’esecuzione è corta o non si stabilizza.
godot-ui-results-corte_sugerido_mser_5_antes = Taglio suggerito (MSER-5): { $cut }. Prima di esso la media sale o scende ancora.
godot-ui-results-gargalo_movel_na_maior_parte = Collo di bottiglia mobile: { $n } per la maggior parte del tempo.
godot-ui-results-nenhum_recurso_ativo = Nessuna risorsa attiva.
godot-ui-results-replicacoes_faltam_para_o_alvo = { $reports } repliche; ne mancano { $more } per l’obiettivo.
godot-ui-results-aquecimento_analisado = Riscaldamento analizzato: { $d }
godot-ui-results-lei_de_little_verde_s = Legge di Little: { $verde } verde/i, { $amarelo } giallo/i, { $vermelho } rosso/i.
godot-ui-results-cenario_s_comparados = { $n ->
    [one] { $n } scenario confrontato.
   *[other] { $n } scenari confrontati.
}
godot-ui-results-otimizacao_melhor_ponto = Ottimizzazione: punto migliore { $ponto }.
godot-ui-results-conteudo_de = contenuto di { $object }
godot-ui-results-mser_5 = MSER-5: { $cut }
godot-ui-results-media_entre_replicacoes = media tra le repliche
godot-ui-results-unico_compartilhado = { $name } (unico { $sole }%, condiviso { $shifting }%)
godot-ui-results-melhor_objetivo = miglior obiettivo
godot-ui-results-pontos_avaliados = punti valutati
godot-ui-results-resultados = Risultati
godot-ui-results-fechar = Chiudi
godot-ui-results-fechar_os_resultados_a_cena = Chiudi i risultati (la scena resta)
godot-ui-results-gargalo_no_turno = collo di bottiglia nel turno
godot-ui-results-objeto = Oggetto
godot-ui-results-tipo = Tipo
godot-ui-results-entradas = Ingressi
godot-ui-results-saidas = Uscite
godot-ui-results-conteudo_medio = Contenuto medio
godot-ui-results-permanencia = Permanenza
godot-ui-results-ocupacao-2 = utilizzo { $valor }
godot-ui-results-replicacoes = Repliche
godot-ui-results-semente = seme { $seed }
godot-ui-results-tempo_de_parada = Tempo di arresto
godot-ui-results-aquecimento = riscaldamento { $warmup }
godot-ui-results-gargalo = Collo di bottiglia
godot-ui-results-gargalo_no_turno-2 = Collo di bottiglia nel turno
godot-ui-results-saidas_por_replicacao = Uscite per replica
godot-ui-results-soma_das_saidas_do_modelo = somma delle uscite del modello
godot-ui-results-o_gargalo_e_o_recurso = Il collo di bottiglia è la risorsa più occupata; nel turno conta solo il tempo in cui lavora (una postazione ferma mezza giornata sembra inattiva in generale, ma può essere satura nel turno).
godot-ui-results-por_objeto = Per oggetto
godot-ui-results-media_semiamplitude_do_ic_de = Media ± semiampiezza dell’IC al 95 % tra le repliche.
godot-ui-results-exportar_csv_para = Esporta CSV in
godot-ui-results-exportar = Esporta
godot-ui-results-grava_resumo_replicacoes_series_e = Salva riepilogo, repliche, serie e istogrammi in CSV
godot-ui-results-grafico = Grafico
godot-ui-results-fracao_do_tempo_trabalhando_turno = Frazione del tempo di lavoro (turno: solo il tempo nel turno; blocco: sommando il tempo bloccato). La barra gialla è il collo di bottiglia; il tratto è l’IC al 95 %.
godot-ui-results-fracao_do_tempo_em_cada = Frazione del tempo in ogni stato, con i colori della scena 3D.
godot-ui-results-conteudo_de_no_tempo_cada = Contenuto di { $object } nel tempo: ogni replica in grigio e la media in giallo.
godot-ui-results-histograma_de_juntando_todas_as = Istogramma di { $object }.{ $tally }, unendo tutte le repliche.
godot-ui-results-o_modelo_nao_grava_ligar = {" "}Il modello non registra { $falta }: "Attiva ed esegui di nuovo" lo attiva e simula di nuovo.
godot-ui-results-falta-serie = la serie del contenuto
godot-ui-results-falta-histogramas = gli istogrammi
godot-ui-results-ligar_e_rodar_de_novo = Attiva ed esegui di nuovo
godot-ui-results-liga_no_modelo_a_serie = Attiva nel modello la serie del contenuto (run.sample_interval, in base al tempo di simulazione) e gli istogrammi (run.histogram_bins = 20), in un passo annullabile con Ctrl+Z, ed esegue di nuovo le repliche
godot-ui-results-sem_serie_ligue_run_sample = nessuna serie: attiva run.sample_interval nel modello
godot-ui-results-sem_histograma_ligue_run_histogram = nessun istogramma: attiva run.histogram_bins nel modello
godot-ui-results-sem_dados = nessun dato
godot-ui-results-ic_95 = IC 95 %
godot-ui-results-a = da { $lo } a { $hi }: { $c }
godot-ui-results-aquecimento-2 = Riscaldamento
godot-ui-results-roda_as_replicacoes_desde_o = Esegue le repliche dal tempo 0, senza riscaldamento, e mostra la loro media (curva di Welch). Il taglio dell’MSER-5 è dove la serie smette di avere l’inizio vuoto; "Applica" scrive quel tempo in run.warmup.
godot-ui-results-serie = Serie
godot-ui-results-wip_ou_objeto_medida = wip o oggetto.misura
godot-ui-results-janela = Finestra
godot-ui-results-campo-janela = finestra
godot-ui-results-campo-replicacoes = repliche
godot-ui-results-campo-orcamento = budget
godot-ui-results-analisar = Analizza
godot-ui-results-roda_e_calcula_welch_e = Esegue e calcola Welch e MSER-5
godot-ui-results-aplicar = Applica
godot-ui-results-grava_o_corte_sugerido_em = Scrive il taglio suggerito in run.warmup del modello
godot-ui-results-sem_serie = nessuna serie
godot-ui-results-replicacoes_necessarias = Repliche necessarie
godot-ui-results-quantas_replicacoes_faltam_para_a = Quante repliche mancano perché la semiampiezza dell’IC al 95 % scenda sotto l’obiettivo (5% è relativo alla media; 0.5 è assoluto). "Esegui altre" aggiunge quelle mancanti a quelle già fatte, con semi nuovi.
godot-ui-results-precisao = Precisione
godot-ui-results-calcular = Calcola
godot-ui-results-calcula_as_replicacoes_necessarias = Calcola le repliche necessarie
godot-ui-results-rodar_mais = Esegui altre { $more }
godot-ui-results-rodar_mais-2 = Esegui altre
godot-ui-results-roda_as_replicacoes_que_faltam = Esegue le repliche mancanti e le unisce a quelle attuali
godot-ui-results-para_todas_as_medidas_chegarem = Perché tutte le misure raggiungano l’obiettivo mancano { $more ->
    [one] { $more } replica.
   *[other] { $more } repliche.
}
godot-ui-results-todas_as_medidas_ja_estao = Tutte le misure sono già entro l’obiettivo.
godot-ui-results-consistencia_lei_de_little = Coerenza: legge di Little
godot-ui-results-confere_l_w_em_cada = Verifica L = λ·W in ogni oggetto e nel sistema. Verde: torna; giallo: la differenza è statisticamente diversa da zero, ma piccola (fino al 2 %); rosso: non torna, il che di solito indica un riscaldamento corto o elementi bloccati alla fine.
godot-ui-results-conferir = Verifica
godot-ui-results-roda_as_replicacoes_e_confere = Esegue le repliche e verifica la legge di Little
godot-ui-results-gargalo_movel = Collo di bottiglia mobile
godot-ui-results-em_cada_instante_o_gargalo = In ogni istante, il collo di bottiglia è la risorsa nel periodo attivo più lungo; quando due periodi si sovrappongono, il collo di bottiglia è condiviso. Mostra la frazione del tempo di ogni risorsa come collo di bottiglia unico e condiviso.
godot-ui-results-roda_as_replicacoes_com_o = Esegue le repliche con la traccia e calcola il collo di bottiglia mobile
godot-ui-results-medida = Misura
godot-ui-results-atual = Attuale
godot-ui-results-alvo = Obiettivo (±)
godot-ui-results-faltam = Mancano
godot-ui-results-diferenca = Differenza
godot-ui-results-semaforo = Semaforo
godot-ui-results-um_cenario_por_bloco_nome = Uno scenario per blocco: [nome] e, sotto, le modifiche (oggetto.campo = valore in RON, o run.campo). Il modello così com’è viene per primo, come base. Tutti usano gli stessi semi (numeri casuali comuni), quindi la differenza dalla base è appaiata; in giallo, le differenze significative.
godot-ui-results-medidas_virgulas_vazio_todas = Misure (virgole; vuoto: tutte)
godot-ui-results-comparar = Confronta
godot-ui-results-roda_todos_os_cenarios_e = Esegue tutti gli scenari e confronta
godot-ui-results-texto = Δ { $valor }
godot-ui-results-procura_os_valores_das_variaveis = Cerca i valori delle variabili che minimizzano (o massimizzano) la media di una metrica. Una variabile per riga: chiave minimo massimo [passo] (interi) o con punto decimale (continua, cinque valori nella griglia). I punti non significativamente diversi dal migliore (test appaiato al 95 %) sono segnati.
godot-ui-results-objetivo = Obiettivo
godot-ui-results-maximizar = massimizzare
godot-ui-results-metodo = Metodo
godot-ui-results-replicacoes_por_ponto = Repliche per punto
godot-ui-results-orcamento_pontos = Budget (punti)
godot-ui-results-otimizar = Ottimizza
godot-ui-results-roda_a_busca = Esegue la ricerca
godot-ui-results-melhor_ponto_ponto_s_simulados = Punto migliore: { $ponto } ({ $simulated ->
    [one] { $simulated } punto simulato
   *[other] { $simulated } punti simulati
})
godot-ui-results-sem_historico = nessuno storico
godot-ui-results-ponto = Punto
godot-ui-results-nenhum = nessuno
godot-ui-results-nenhum-viavel = nessuno ammissibile
godot-ui-results-util-trabalho = lavoro
godot-ui-results-util-turno = turno
godot-ui-results-util-bloqueio = blocco
godot-ui-results-metodo-grade = griglia
godot-ui-results-metodo-recozimento = ricottura
godot-ui-results-metodo-genetico = genetico
godot-ui-results-replicacao_oes_tempo_de_parada = 
    { $model }: { $replications ->
        [one] { $replications } replica
       *[other] { $replications } repliche
    }, tempo di arresto { $stop_time }, riscaldamento { $warmup }, seme { $seed }.
    Collo di bottiglia (generale): { $b }
    Collo di bottiglia (nel turno): { $bs }
godot-ui-results-replicacao_oes = { $model } · { $reports ->
    [one] { $reports } replica
   *[other] { $reports } repliche
}

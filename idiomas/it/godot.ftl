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

## Tipos de bloco nos relatórios (results.rs)

godot-tipo-source = arrivo
godot-tipo-queue = coda
godot-tipo-processor = processo
godot-tipo-combiner = assemblaggio
godot-tipo-separator = disassemblaggio
godot-tipo-sink = uscita
godot-tipo-transporter = trasportatore
godot-tipo-dispatcher = smistatore
godot-tipo-conveyor = nastro trasportatore
godot-tipo-rack = scaffale portapallet
godot-tipo-span = misura del tempo
godot-tipo-flow = flusso
godot-tipo-fluid = fluidi

## Propriedades dos objetos (schema.rs)

godot-campo-movimentacao = Movimentazione
godot-campo-folga-dos-obstaculos = Distanza dagli ostacoli
godot-campo-ponto-de-acesso-proprio = Punto di accesso proprio
godot-campo-x-no-objeto = X sull’oggetto
godot-campo-y-no-objeto = Y sull’oggetto
godot-campo-dimensoes = Dimensioni
godot-campo-dimensoes-proprias = Dimensioni proprie
godot-campo-capacidade-pela-area = Capacità in base all’area
godot-campo-area-por-item = Area per elemento
godot-campo-cabem = Ci stanno
godot-campo-tempo-entre-chegadas = Tempo tra gli arrivi
godot-campo-primeira-chegada-em-t-0 = Primo arrivo a t = 0
godot-campo-maximo-de-chegadas = Massimo di arrivi
godot-campo-se-a-saida-estiver-bloqueada = Se l’uscita è bloccata
godot-opcao-esperar = Attendere
godot-opcao-descartar-perda = Scartare (perdita)
godot-campo-rotulos = Etichette
godot-campo-chegadas-por-periodo = Arrivi per periodo
godot-campo-tabela-de-chegadas = Tabella degli arrivi
godot-campo-capacidade-limitada = Capacità limitata
godot-campo-politica-de-acesso = Politica di accesso
godot-opcao-fifo-primeiro-a-entrar-primeiro = FIFO (il primo a entrare, il primo a uscire)
godot-opcao-lifo-ultimo-a-entrar-primeiro = LIFO (l’ultimo a entrare, il primo a uscire)
godot-opcao-prioridade-maior-rotulo-primeiro = Priorità: prima l’etichetta maggiore
godot-opcao-prioridade-menor-rotulo-primeiro-prazo = Priorità: prima l’etichetta minore (scadenza)
godot-campo-rotulo-da-prioridade = Etichetta della priorità
godot-campo-lote = Lotto
godot-campo-itens-por-lote = Elementi per lotto
godot-campo-espera-maxima = Attesa massima
godot-campo-fecha-a-entrada-ate-o = Chiude l’ingresso finché il lotto non esce
godot-campo-capacidade = Capacità
godot-campo-preparacao-setup = Attrezzaggio (setup)
godot-campo-troca-de-produto = Cambio prodotto
godot-campo-tempo-de-processamento = Tempo di processo
godot-campo-tempo-por-receita = Tempo per ricetta
godot-campo-operadores = Operatori
godot-campo-quem-opera = Chi opera
godot-campo-quantos-por-peca = Quanti per pezzo
godot-campo-ganho-da-equipe = Guadagno della squadra
godot-campo-durante-a-preparacao = Durante l’attrezzaggio
godot-campo-durante-o-processamento = Durante il processo
godot-campo-outras-equipes-ao-mesmo-tempo = Altre squadre contemporaneamente
godot-campo-componentes-por-entrada = Componenti per ingresso
godot-campo-modo = Modalità
godot-opcao-embalar-so-o-conteiner-sai = Imballare (esce solo il contenitore)
godot-opcao-juntar-componentes-destruidos = Unire (componenti distrutti)
godot-opcao-lote-todos-saem = Lotto (escono tutti)
godot-opcao-desembalar = Disimballare
godot-opcao-dividir-em-copias = Dividere in copie
godot-opcao-dividir-pelo-rotulo = Dividere per etichetta
godot-campo-copias = Copie
godot-campo-sem-parametros = Senza parametri
godot-campo-velocidade-maxima = Velocità massima
godot-campo-tempo-de-carga = Tempo di carico
godot-campo-tempo-de-descarga = Tempo di scarico
godot-campo-voltar-a-base-quando-ocioso = Tornare alla base quando inattivo
godot-campo-agv-controle-de-trafego-na = AGV (controllo del traffico sulla rete)
godot-campo-trajeto-desenhado = Percorso disegnato
godot-campo-editar-nos = Modifica nodi
godot-campo-comprimento = Lunghezza
godot-campo-velocidade = Velocità
godot-campo-comprimento-por-item = Lunghezza per elemento
godot-campo-acumula-itens-quando-a-saida = Accumula elementi quando l’uscita si blocca
godot-campo-colunas = Colonne
godot-campo-niveis = Livelli
godot-campo-permanencia-minima = Permanenza minima
godot-campo-regra-de-ocupacao = Regola di occupazione
godot-opcao-primeira-livre = Prima libera
godot-opcao-aleatoria = Casuale
godot-opcao-curva-abc-rotulo-classe = Curva ABC (etichetta classe)
godot-campo-fracao-das-posicoes-da-classe = Frazione delle posizioni della classe A
godot-campo-fracao-das-posicoes-da-classe-2 = Frazione delle posizioni della classe B
godot-campo-largura-da-coluna = Larghezza della colonna
godot-campo-altura-do-nivel = Altezza del livello
godot-campo-equipe = Squadra
godot-campo-regra-de-escolha = Regola di scelta
godot-opcao-primeiro-livre = Primo libero
godot-opcao-rodizio = Rotazione
godot-opcao-mais-proximo = Più vicino
godot-campo-ordem-dos-movimentos = Ordine dei movimenti
godot-opcao-segura-sobe-anda-desce = Tiene (sale, si sposta, scende)
godot-opcao-um-eixo-de-cada-vez = Un asse alla volta
godot-opcao-os-tres-juntos = Tutti e tre insieme
godot-campo-altura-de-passagem-do-gancho = Altezza di passaggio del gancio
godot-campo-rotulo-do-peso-do-item = Etichetta del peso dell’elemento
godot-campo-tempo-de-engate = Tempo di aggancio
godot-campo-tempo-de-desengate = Tempo di sgancio
godot-campo-voltar-a-posicao-propria-quando = Tornare alla posizione propria quando inattivo
godot-campo-controle = Controllo
godot-opcao-cabine-operador-nao-modelado = Cabina (operatore non modellato)
godot-opcao-botoeira-a-pessoa-anda-com = Pulsantiera (la persona cammina con il carico)
godot-opcao-radio-a-pessoa-encontra-a = Radio (la persona incontra il carroponte sul carico)
godot-campo-equipe-do-operador = Squadra dell’operatore
godot-campo-meia-largura-da-area-sob = Mezza larghezza dell’area sotto il carico
godot-campo-afastar-as-pessoas-do-caminho = Allontanare le persone dal percorso del carico
godot-campo-folga-entre-pontes = Distanza tra carriponte
godot-campo-caminho = Percorso
godot-campo-regra-de-saida = Regola di uscita
godot-campo-pesos = Pesi
godot-campo-rotulo-com-o-numero-da = Etichetta con il numero della porta
godot-campo-script = Script
godot-campo-nenhum-passagem-direta = Nessuno (passaggio diretto)
godot-campo-transporte-de-saida = Trasporto in uscita
godot-campo-transferencia-pela-distancia = Trasferimento in base alla distanza
godot-campo-atraso = Ritardo
godot-opcao-reta = retta
godot-opcao-pela-rede = lungo la rete
godot-campo-script-ao-entrar-um-item = Script all’ingresso di un elemento
godot-campo-quebras = Guasti
godot-campo-tempo-entre-falhas-mtbf = Tempo tra i guasti (MTBF)
godot-campo-tempo-de-reparo-mttr = Tempo di riparazione (MTTR)
godot-campo-turnos = Turni
godot-campo-origem = Origine
godot-opcao-personalizado = Personalizzato
godot-opcao-calendario-do-modelo = Calendario del modello
godot-campo-periodo = Periodo
godot-campo-pausas = Pause
godot-campo-no-fim-do-turno = A fine turno
godot-campo-calendario = Calendario
godot-campo-ativo = Attivo
godot-campo-cor = Colore
godot-campo-escala = Scala
godot-campo-modelo-da-peca = Modello del pezzo
godot-campo-giro-do-desenho = Rotazione del disegno
godot-campo-restaurar-padrao = Ripristina predefinito
godot-campo-nome = Nome
godot-campo-tipo = Tipo
godot-campo-rotacao = Rotazione
godot-campo-sexo = Sesso
godot-opcao-nao-informado = Non indicato
godot-opcao-feminino = Femminile
godot-opcao-masculino = Maschile
godot-campo-altura = Altezza
godot-ajuda-a-altura-do-boneco-no = L’altezza del personaggio in 3D. Se disattivata, viene sorteggiata in base al nome.
godot-campo-massa = Massa
godot-ajuda-com-a-altura-da-a = Insieme all’altezza, dà la larghezza del personaggio in 3D tramite l’indice di massa corporea.
godot-campo-idade = Età
godot-campo-experiencia-na-funcao = Esperienza nella mansione
godot-ajuda-so-registro-a-simulacao-nao = Solo registro: la simulazione non lo legge.
godot-campo-carga-maxima-medida = Carico massimo misurato
godot-ajuda-uma-carga-acima-dela-da = Un carico superiore a questo valore genera un avviso nel modello.
godot-campo-forca-maxima-medida = Forza massima misurata
godot-ajuda-empurrar-ou-puxar-so-registro = Spingere o tirare. Solo registro.
godot-campo-tipo-de-espera = Tipo di coda
godot-opcao-fila-simples-sem-lugares = Coda semplice (senza posti)
godot-opcao-mesa = Tavolo
godot-opcao-palete = Pallet
godot-opcao-estante = Scaffale
godot-opcao-chao-area-demarcada = Pavimento (area delimitata)
godot-campo-camadas = Strati
godot-campo-passo-entre-pecas = Passo tra i pezzi
godot-campo-lugares = Posti
godot-campo-opera-as-maquinas = Opera le macchine
godot-campo-leva-os-itens-de = Porta gli elementi da
godot-campo-nome-do-modelo = Nome del modello
godot-campo-descricao = Descrizione
godot-campo-tempo-de-simulacao = Tempo di simulazione
godot-campo-aquecimento = Riscaldamento
godot-campo-semente = Seme
godot-campo-replicacoes = Repliche
godot-campo-movimentacao-padrao = Movimentazione predefinita
godot-campo-perfil-da-marcacao-do-piso = Profilo della segnaletica a pavimento
godot-opcao-brasil-faixa-amarela = Brasile (striscia gialla)
godot-opcao-branco-redacao-original-da-nr = Bianco (testo originale della NR-26)
godot-campo-largura-da-faixa = Larghezza della striscia
godot-campo-parada-no-cruzamento = Sosta all’incrocio
godot-campo-cor-do-chao = Colore del pavimento
godot-ajuda-como-o-recurso-anda-o = Come si sposta la risorsa: il predefinito del modello (Simulazione), in linea retta come nel formato precedente, libero sul pavimento evitando gli ostacoli, lungo la rete di percorsi o solo lungo i corridoi della sua classe.
godot-ajuda-onde-os-recursos-moveis-param = Dove si fermano le risorse mobili per servire questo oggetto, in metri a partire dal suo centro e ruotando con esso. Se disattivato, è il lato -y. Si può anche trascinare nella scena, con “Mostra ostacoli e distanze” attivo.
godot-ajuda-comprimento-largura-e-altura-do = Lunghezza, larghezza e altezza dell’oggetto in metri, nel suo orientamento. L’area a pavimento cambia il punto di accesso e, con esso, le distanze e i tempi di operatori e veicoli; in una coda con area per elemento, anche la capacità. Il 3D adatta il modello a queste misure. Se disattivato, vale l’area predefinita del tipo.
godot-ajuda-a-capacidade-da-espera-sai = La capacità della coda deriva dalla sua area a pavimento divisa per l’area occupata da ogni elemento (solo elementi interi). Se è indicata anche una capacità, vale la minore.
godot-ajuda-intervalo-entre-duas-chegadas-seguidas = Intervallo tra due arrivi consecutivi.
godot-ajuda-valor-sorteado-para-cada-item = Valore sorteggiato per ogni elemento, per nome: {"{"}"tipo": Discrete([(1.0, 3.0), (2.0, 1.0)]){"}"}.
godot-ajuda-ciclo-e-periodos-com-o = Ciclo e periodi con intervallo proprio, tasso o chiusi: (cycle: 1440.0, periods: [(start: 0.0, end: 480.0, arrivals: Rate(0.5))]).
godot-ajuda-horario-instante-quantidade-tipo-e = Orario (istante, quantità, tipo ed etichette di ogni riga) o sequenza di righe: (mode: Times, repeat: Cycle(1440.0), rows: [(at: 480.0, count: 12)]).
godot-ajuda-desligada-a-espera-nao-tem = Se disattivata, la coda non ha limite.
godot-ajuda-qual-item-sai-primeiro-pela = Quale elemento esce per primo. Con la priorità, esce prima l’elemento con il valore maggiore (o minore) dell’etichetta; a pari valore e gli elementi senza etichetta escono in ordine di arrivo.
godot-ajuda-segura-os-itens-e-libera = Trattiene gli elementi e li rilascia a gruppi.
godot-ajuda-libera-um-lote-incompleto-depois = Rilascia un lotto incompleto dopo questo tempo (0 = attende sempre).
godot-ajuda-preparacao-so-quando-o-produto = Attrezzaggio solo quando il prodotto cambia: l’etichetta del prodotto e, se si vuole, il tempo di ogni cambio (da, a); senza riga nella matrice vale l’attrezzaggio indicato sopra. Esempio: (label: "produto", matrix: [(from: 1, to: 2, time: Const(30.0))]). None: attrezzaggio su ogni pezzo.
godot-ajuda-tempo-de-processamento-que-depende = Tempo di processo che dipende dall’elemento: l’etichetta della ricetta e il tempo di ogni valore; gli elementi senza ricetta nella tabella usano il tempo indicato sopra. Esempio: (label: "produto", times: [(value: 1, time: Const(90.0)), (value: 2, time: Const(120.0))]). None: lo stesso tempo per tutti.
godot-ajuda-quem-opera-esta-maquina-marque = Chi opera questa macchina: seleziona e scegli sotto, oppure trascina dal nodo arancione dell’operatore (o della squadra) fino alla macchina. Il processo attende che uno di essi sia libero.
godot-ajuda-operadores-que-a-peca-precisa = Operatori di cui il pezzo ha bisogno contemporaneamente: parte solo quando arrivano tutti, e restano con il pezzo fino alla fine. Con più operatori nella squadra rispetto a capacità × questo numero, quelli in eccesso restano fermi.
godot-ajuda-quantas-vezes-mais-rapido-a = Quante volte più veloce la squadra esegue la fase: il tempo sorteggiato viene diviso per questo numero. 1: nessun guadagno (il tempo è lo stesso con uno o con più operatori).
godot-ajuda-equipes-que-precisam-estar-juntas = Squadre che devono essere presenti insieme agli operatori (un medico e un infermiere nella stessa stanza). Esempio: [OperatorUse(team: "enfermagem")]. La fase parte solo quando arrivano tutti.
godot-ajuda-a-entrada-1-traz-o = L’ingresso 1 porta il contenitore; l’ingresso k + 1 porta il k-esimo numero di elementi. Esempio: [2, 1].
godot-ajuda-desligada-a-velocidade-muda-na = Se disattivata, la velocità cambia subito.
godot-ajuda-abre-a-ferramenta-de-desenho = Apre lo strumento di disegno con questo nastro trasportatore: trascina i nodi, cambia l’altezza e il raggio.
godot-ajuda-transportadores-que-o-despachante-comanda = Trasportatori che lo smistatore comanda.
godot-ajuda-segura-o-gancho-sobe-a = Tiene: il gancio sale all’altezza di passaggio, il carroponte e il carrello si spostano insieme e il gancio scende.
godot-ajuda-rotulo-com-o-peso-de = Etichetta con il peso di ogni elemento, in kg: gli elementi oltre la capacità generano un avviso.
godot-ajuda-na-botoeira-a-ponte-nao = Con la pulsantiera, il carroponte non va più veloce della persona; con la radio, mantiene la propria velocità e attende che la persona arrivi.
godot-ajuda-meio-lado-do-quadrado-sob = Metà lato del quadrato sotto il gancio dove nessuno deve stare con il carico sospeso (analisi di esposizione).
godot-ajuda-antes-de-andar-carregada-a = Prima di muoversi con il carico, il carroponte attende chi attraversa la fascia del carico; e chi stava per entrarvi si ferma al bordo finché il carico non è passato.
godot-ajuda-distancia-mantida-entre-os-ganchos = Distanza mantenuta tra i ganci di due carriponte sullo stesso percorso: uno non supera l’altro.
godot-ajuda-para-qual-ligacao-vai-cada = A quale collegamento va ogni elemento, nell’ordine dei collegamenti in uscita.
godot-ajuda-peso-de-cada-saida-na = Peso di ogni uscita, nell’ordine dei collegamenti. Esempio: [3.0, 1.0].
godot-ajuda-devolve-o-numero-da-porta = Restituisce il numero della porta (1, 2, ...).
godot-ajuda-transportador-ou-despachante-que-leva = Trasportatore o smistatore che porta gli elementi al blocco successivo.
godot-ajuda-os-itens-levam-atraso-distancia = Gli elementi impiegano ritardo + distanza / velocità per arrivare al blocco successivo tramite i collegamenti diretti (senza trasportatore).
godot-ajuda-roda-para-cada-item-que = Viene eseguito per ogni elemento che entra, prima di qualsiasi altra regola.
godot-ajuda-falhas-aleatorias-com-o-tempo = Guasti casuali, con il tempo contato sull’orologio.
godot-ajuda-janelas-fora-do-turno-repetidas = Finestre fuori turno, ripetute a ogni periodo.
godot-ajuda-inicio-e-duracao-de-cada = Inizio e durata di ogni pausa all’interno del periodo, in ordine.
godot-ajuda-corrige-para-onde-o-modelo = Corregge la direzione verso cui guarda il modello 3D; la risorsa si gira verso dove si muove.
godot-ajuda-volta-a-aparencia-padrao-do = Ripristina l’aspetto predefinito del tipo di oggetto.
godot-ajuda-giro-em-graus-anti-horario = Rotazione in gradi, antioraria vista dall’alto (R ruota di 90°, Ctrl+R di 15°).
godot-ajuda-escolhe-o-corpo-no-3d = Sceglie il corpo in 3D (non indicato: il corpo deriva dal nome). Nessun modello di fatica dipende da esso; con Femminile, un carico oltre 20 kg genera l’avviso della CLT, art. 390.
godot-ajuda-numa-espera-real-cada-peca = In una coda reale ogni pezzo occupa un posto (la disposizione del tipo, ricalcolata dalle Dimensioni): esce solo il pezzo senza nulla sopra, il più vecchio per primo, e chi preleva va fino alla parte anteriore del pezzo. Cambia il risultato. Coda semplice: come prima.
godot-ajuda-quantas-pecas-empilhadas-em-cada = Quanti pezzi impilati in ogni posto (sullo scaffale, in ogni ripiano).
godot-ajuda-distancia-entre-os-centros-de = Distanza tra i centri di due pezzi vicini.
godot-ajuda-arraste-do-no-laranja-ate = Trascina dal nodo arancione fino alla macchina per collegare; clicca sulla linea tratteggiata e sulla × per rimuovere.
godot-ajuda-arraste-do-no-laranja-ate-2 = Trascina dal nodo arancione fino all’oggetto (una coda, un arrivo o, con Shift, una macchina) per collegare.
godot-ajuda-texto-livre-sobre-o-modelo = Testo libero sul modello, salvato nel file.
godot-ajuda-as-estatisticas-so-contam-depois = Le statistiche contano solo dopo questo tempo.
godot-ajuda-como-andam-os-recursos-moveis = Come si spostano le risorse mobili che non hanno scelto una modalità propria. Se vuoto, vale Libero (percorso più breve sul pavimento, evitando gli ostacoli).
godot-ajuda-cores-das-faixas-e-setas = Colori delle strisce e delle frecce dei corridoi. Il programma non certifica la conformità: verifica il testo vigente delle norme.
godot-ajuda-a-pratica-usual-vai-de = La pratica usuale va da 0,05 a 0,15 m.
godot-ajuda-o-veiculo-para-esse-tempo = Il veicolo si ferma per questo tempo prima di un incrocio con pedoni.
godot-ajuda-a-cor-do-piso-na = Il colore del pavimento nella scena 3D, salvato nel modello; la griglia cambia tono per restare visibile. Se disattivato, il solito grigio.
godot-opcao-por-rotulo = Per etichetta
godot-opcao-padrao-do-modelo = Predefinito del modello
godot-opcao-legado-linha-reta-ou-rede = Legacy (linea retta o rete)
godot-opcao-livre-menor-caminho-no-piso = Libero (percorso più breve sul pavimento)
godot-opcao-pela-rede-de-caminhos = Lungo la rete di percorsi
godot-opcao-pelos-corredores = Lungo i corridoi


## Propriedades dos objetos: seções, tipos, famílias, parâmetros, turnos e visual (schema.rs)

godot-secao-objeto = Oggetto
godot-secao-posicao = Posizione
godot-secao-parametros = Parametri
godot-secao-saida = Uscita
godot-secao-quebras-e-turnos = Guasti e turni
godot-secao-aparencia = Aspetto
godot-secao-simulacao = Simulazione
godot-secao-movimentacao = Movimentazione
godot-secao-piso-e-corredores = Pavimento e corridoi
godot-secao-modelo = Modello
godot-secao-pessoa = Persona
godot-objeto-chegada = Arrivo
godot-objeto-espera = Coda
godot-objeto-processo = Processo
godot-objeto-montagem = Assemblaggio
godot-objeto-desmontagem = Disassemblaggio
godot-objeto-saida = Uscita
godot-objeto-transportador = Trasportatore
godot-objeto-ponte-rolante = Carroponte
godot-objeto-despachante = Smistatore
godot-objeto-esteira = Nastro trasportatore
godot-objeto-porta-paletes = Scaffale portapallet
godot-dist-constante = Costante
godot-dist-uniforme = Uniforme
godot-dist-exponencial = Esponenziale
godot-dist-triangular = Triangolare
godot-dist-normal = Normale
godot-dist-lognormal = Lognormale
godot-dist-gama = Gamma
godot-dist-erlang = Erlang
godot-dist-weibull = Weibull
godot-dist-beta = Beta
godot-dist-log-logistica = Log-logistica
godot-dist-pearson-v = Pearson V
godot-dist-pearson-vi = Pearson VI
godot-dist-discreta = Discreta
godot-dist-empirica = Empirica
godot-dist-poisson = Poisson
godot-dist-geometrica = Geometrica
godot-dist-binomial = Binomiale
godot-dist-binomial-negativa = Binomiale negativa
godot-dist-deslocada = Traslata
godot-param-valor = valore
godot-param-minimo = minimo
godot-param-maximo = massimo
godot-param-media = media
godot-param-moda = moda
godot-param-desvio-padrao = deviazione standard
godot-param-forma = forma
godot-param-escala = scala
godot-param-fases = fasi (k)
godot-param-forma-1 = forma 1
godot-param-forma-2 = forma 2
godot-param-probabilidade = probabilità (p)
godot-param-tentativas = tentativi (n)
godot-param-sucessos = successi (r)
godot-param-alfa = alfa
godot-param-beta = beta
godot-param-deslocamento = traslazione
godot-dim-comprimento-do-caminho = Lunghezza del percorso
godot-dim-vao = Campata
godot-dim-altura-do-caminho = Altezza del percorso
godot-dim-comprimento = Lunghezza
godot-dim-largura = Larghezza
godot-dim-altura = Altezza
godot-turno-pausar = Mettere in pausa
godot-turno-terminar = Terminare il lavoro
godot-turno-do-calendario = (quello del calendario)
godot-ajuda-folga-papel = Distanza minima da qualsiasi ostacolo. Se disattivata, vale quella del ruolo della risorsa ({ $m } m: mezza larghezza più 0,3 m).
godot-dim-esteira = { $l } × { $w } m: la lunghezza è quella del nastro trasportatore (Parametri)
godot-dim-porta-paletes = { $l } × { $w } × { $h } m: in base a campate e livelli (Parametri)
godot-dim-padrao = { $l } × { $w } m (predefinito)
godot-campo-sem-limite = senza limite
godot-campo-n-itens = { $n ->
    [one] { $n } elemento
   *[other] { $n } elementi
}
godot-campo-nao-existe = { $nome } (non esiste)
godot-campo-n-posicoes = { $n ->
    [one] { $n } posizione
   *[other] { $n } posizioni
}
godot-campo-movimento = Movimento { $o_que }.
godot-campo-n-lugares = { $n ->
    [one] { $n } posto
   *[other] { $n } posti
}
godot-campo-divide-caminho = Condivide il percorso con { $com } (distanza di { $m } m tra i ganci)
godot-campo-pesos-e-saidas = Ci { $pesos ->
    [one] { $pesos } peso
   *[other] { $pesos } pesi
}, ma l’oggetto ha { $n ->
    [one] { $n } uscita
   *[other] { $n } uscite
}. { $ajuda }
godot-turno-sem-calendarios = Il modello non ha calendari: creane uno in "calendars" per usarlo qui.
godot-turno-calendario-ajuda = Un calendario è la stessa tabella di turni usata da più oggetti.
godot-visual-padrao-do-tipo = (predefinito del tipo)
godot-visual-gerado = UTSim (generato da utsim-assets)
godot-visual-grupo-biblioteca = Libreria di UTSim
godot-visual-grupo-terceiros = Terze parti (assets/terceiros)
godot-visual-grupo-elementos = Elementi 3D (assets/modelos3d)
godot-visual-caixa-padrao = scatola (predefinito)
godot-visual-modelo-da-peca-ajuda = Elemento 3D con cui vengono disegnati i pezzi creati qui; la scatola è il predefinito.
godot-visual-no-tamanho-da-peca = { $arquivo } ({ $x } x { $y } x { $z } m), disegnato nella dimensione del pezzo
godot-eixo-z = Z (altezza)


## ui_props.rs

godot-ui-props-modelo_nada_selecionado = Modello (nulla selezionato)
godot-ui-props-propriedades = Proprietà: { $name }
godot-ui-props-problemas = Problemi
godot-ui-props-avisos = Avvisi
godot-ui-props-mostrar_motorista = Mostra conducente
godot-ui-props-desenha_o_operador_sentado_e = Disegna l’operatore seduto e animato sul carrello elevatore. Solo aspetto: non cambia la simulazione né viene salvato nel modello.
godot-ui-props-clique_para_destacar_na_cena = Clicca per evidenziare nella scena ciò che questo avviso cita
godot-ui-props-abre-a-secao = Apre la sezione { $title }
godot-ui-props-fecha-a-secao = Chiude la sezione { $title }
godot-ui-props-liga_ou_desliga = Attiva o disattiva { $campo }
godot-ui-props-aplicar = Applica
godot-ui-props-aplicar_script = Applica script
godot-ui-props-nenhum_transportador_no_modelo = Nessun trasportatore nel modello.
godot-ui-props-este_campo_vem_de_dados = Questo campo deriva da dati reali: { $label }
godot-ui-props-distribuicao = Distribuzione
godot-ui-props-dados_reais = Dati reali
godot-ui-props-fonte = Fonte
godot-ui-props-de_onde_vem_a_distribuicao = Da dove deriva la distribuzione di questo campo: parametri indicati o valori reali, adattati automaticamente
godot-ui-props-media = media { $mean }
godot-ui-props-arquivo_csv = File CSV
godot-ui-props-valores_colados = Valori incollati
godot-ui-props-valores_reais = Valori reali…
godot-ui-props-abre_a_janela_grande_para = Apre la finestra grande per digitare, incollare o importare i valori e confrontare tutte le distribuzioni in una volta
godot-ui-props-origem = Origine
godot-ui-props-arquivo = File
godot-ui-props-arquivo_csv_ou_txt = file .csv o .txt
godot-ui-props-escolher = Scegli...
godot-ui-props-escolhe_o_arquivo_de_dados = Sceglie il file di dati (CSV o TXT)
godot-ui-props-automatica = (automatica)
godot-ui-props-coluna = Colonna
godot-ui-props-coluna_do_arquivo_com_os = Colonna del file con i valori
godot-ui-props-um_valor_por_linha_ou = un valore per riga (o separati da ; o spazio)
godot-ui-props-sao_instantes_usar_os_intervalos = Sono istanti (usare gli intervalli tra di essi)
godot-ui-props-marque_quando_os_valores_sao = Seleziona quando i valori sono istanti crescenti (come gli orari di arrivo): l’adattamento usa le differenze tra di essi. Se deselezionato, i valori sono durate.
godot-ui-props-melhor_automatica = Migliore automatica
godot-ui-props-empirica_os_proprios_valores = Empirica (i valori stessi)
godot-ui-props-so = Solo { $f }
godot-ui-props-escolha = Scelta
godot-ui-props-criterio = Criterio
godot-ui-props-o_que_ordena_as_familias = Ciò che ordina le famiglie accettate dai test
godot-ui-props-reajustar_com_estas_opcoes = Riadatta con queste opzioni
godot-ui-props-ajusta_a_distribuicao_aos_dados = Adatta la distribuzione ai dati (in background) e inizia a usarla in questo campo
godot-ui-props-nao_foi_possivel_ler_os = Impossibile leggere i dati: { $err }
godot-ui-props-os_dados_mudaram_desde_o = I dati sono cambiati dall’ultimo adattamento.
godot-ui-props-reajustar_aos_dados_atuais = Riadatta ai dati attuali
godot-ui-props-o_campo_foi_reajustado_salve = { $c }. Il campo è stato riadattato; salva per registrare.
godot-ui-props-n = { $family } · n = { $n } · { $label }
godot-ui-props-analisando_os_dados = Analisi dei dati...
godot-ui-props-candidatas = Candidate ({ $criterion })
godot-ui-props-aic_bic_usa_esta_familia =
    { $params }
    AIC { $aic }   BIC { $bic }
    Usa questa famiglia in questo campo
godot-ui-props-usar_a_empirica_os_proprios = Usa l’empirica (i valori stessi)
godot-ui-props-relatorio = Rapporto
godot-ui-props-grava_o_relatorio_completo_do = Salva il rapporto completo dell’adattamento (HTML) in una cartella e lo apre nel browser
godot-ui-props-histograma_dos_dados_e_a = Istogramma dei dati e curva della distribuzione scelta
godot-ui-props-buscar_ativo = Cerca asset
godot-ui-props-miniatura_do_objeto_com_o = Miniatura dell’oggetto con l’asset scelto (o quello sotto il mouse nell’elenco)
godot-ui-props-ligacoes_de_saida_ordem_das = Collegamenti in uscita (ordine delle porte)
godot-ui-props-nenhuma_use_a_ferramenta_ligar = Nessuno. Usa lo strumento Collega (L).
godot-ui-props-subir = Su
godot-ui-props-sobe_esta_saida_na_ordem = Sposta in su questa uscita nell’ordine delle porte
godot-ui-props-descer = Giù
godot-ui-props-desce_esta_saida_na_ordem = Sposta in giù questa uscita nell’ordine delle porte
godot-ui-props-remover = Rimuovi
godot-ui-props-remove_esta_ligacao = Rimuove questo collegamento
godot-ui-props-entradas = Ingressi: { $inputs }

## Giro 13: finestre di UTSim 3D

## dados.rs

godot-dados-criterio_desconhecido = criterio sconosciuto: { $other }
godot-dados-use_um_arquivo_ou_valores = usa un file o valori incollati, non entrambi
godot-dados-escolha_um_arquivo_de_dados = scegli un file di dati o incolla i valori
godot-dados-registro_reaproveitado = registro riutilizzato
godot-dados-dados_mudaram = i dati sono cambiati
godot-dados-opcoes_mudaram = le opzioni sono cambiate
godot-dados-dados_ilegiveis = dati illeggibili
godot-dados-os_dados_mudaram_desde_o = i dati sono cambiati dall'ultimo adattamento (era { $old_family }, hash { $old_hash })
godot-dados-nao_foi_possivel_ler = impossibile leggere { $path }: { $e }
godot-dados-nenhum_campo_do_modelo_usa = nessun campo del modello usa dati reali
godot-dados-nao_foi_possivel_gravar_em = impossibile scrivere in { $dir }: { $e }

## document.rs

godot-document-transportadores_e_despachantes_nao_usam = I trasportatori e i dispatcher non usano collegamenti di flusso: trascina dal loro nodo arancione fino alla macchina che servono (o fino all'oggetto di cui trasportano gli elementi).
godot-document-objeto_nao_existe = l'oggetto '{ $n }' non esiste
godot-document-arraste_de_um_objeto_ate = trascina da un oggetto a un altro
godot-document-saiu_da_equipe_de = { $to } è uscito dalla squadra di { $from }.
godot-document-entrou_na_equipe_de = { $to } è entrato nella squadra di { $from }.
godot-document-nao_pede_mais_operadores_de = { $from } non richiede più operatori da { $to }.
godot-document-pede_operadores_de = { $from } richiede operatori da { $to }.
godot-document-nao_pede_mais_transporte_a = { $from } non richiede più trasporto a { $to }.
godot-document-pede_transporte_a = { $from } richiede trasporto a { $to }.
godot-document-arraste_do_despachante_a_um = trascina dal dispatcher a un trasportatore, o da un oggetto al trasportatore o dispatcher che lo serve
godot-document-a_serie_e_os_histogramas = la serie e gli istogrammi sono già collegati nel modello
godot-document-desenhe_clicando_os_nos_no = disegna cliccando i nodi sul pavimento (Enter termina)
godot-document-tipo_desconhecido = tipo sconosciuto: { $id }
godot-document-nada_selecionado = nessuna selezione
godot-document-nao_ha_para_onde_mover = non c'è dove spostare questo collegamento
godot-document-nenhum_arrasto_comecou = nessun trascinamento iniziato
godot-document-objeto_nao_existe-2 = l'oggetto '{ $name }' non esiste
godot-document-e_um_recurso_movel_ele = { $name } è una risorsa mobile: si orienta verso dove cammina
godot-document-recursos_moveis_nao_giram_eles = le risorse mobili non ruotano: si orientano verso dove camminano
godot-document-a_area_de_transferencia_esta = gli appunti sono vuoti (Ctrl+C copia)
godot-document-alinhamento_desconhecido = allineamento sconosciuto: { $how }
godot-document-selecione_dois_objetos_ou_mais = seleziona due o più oggetti da allineare
godot-document-eixo_desconhecido = asse sconosciuto: { $axis }
godot-document-selecione_tres_objetos_ou_mais = seleziona tre o più oggetti da distribuire
godot-document-nao_tem_distribuicao = '{ $field }' non ha una distribuzione
godot-document-modelo = Modello
godot-document-nome_em_vez_de = nome: { $name } invece di { $name2 }
godot-document-descricao_em_vez_de = descrizione: { $description } invece di { $description2 }
godot-document-run_em_vez_de = run: { $run } invece di { $run2 }
godot-document-falta_o_objeto = manca l'oggetto { $name }
godot-document-objeto_em_vez_de = oggetto { $name }: { $g } invece di { $w }
godot-document-objeto_a_mais = oggetto in più: { $name }
godot-document-ligacoes_em_vez_de = collegamenti: { $connections } invece di { $connections2 }
godot-document-outras_partes_do_modelo_calendarios = altre parti del modello (calendari, rete, flussi, fluidi) differiscono
godot-document-clique = Clic
godot-document-seleciona_o_bloco_ou_a = seleziona il blocco o il collegamento
godot-document-clique_de_novo_no_mesmo = Clic di nuovo nello stesso punto
godot-document-passa_para_o_proximo_bloco = passa al blocco sovrapposto successivo
godot-document-poe_ou_tira_o_bloco = aggiunge o toglie il blocco dalla selezione
godot-document-shift_arrastar_no_chao = Shift + trascina sul pavimento
godot-document-selecao_por_caixa_soma_a = selezione a riquadro (si aggiunge alla selezione)
godot-document-arrastar_no_chao = Trascina sul pavimento
godot-document-selecao_por_caixa_troca_a = selezione a riquadro (sostituisce la selezione)
godot-document-arrastar_um_bloco = Trascina un blocco
godot-document-move_o_bloco_ou_a = sposta il blocco o l'intera selezione
godot-document-arrastar_da_porta_de_saida = Trascina dalla porta di uscita (verde)
godot-document-liga_ao_bloco_ou_a = collega al blocco (o alla porta di ingresso) dove rilasci
godot-document-botao_direito_arrastar = Pulsante destro + trascina
godot-document-gira_a_vista = ruota la vista
godot-document-shift_botao_direito_arrastar = Shift + pulsante destro + trascina
godot-document-desloca_a_vista = sposta la vista
godot-document-roda_do_mouse = Rotellina del mouse
godot-document-aproxima_e_afasta = avvicina e allontana
godot-document-biblioteca_clique_e_clique_no = Libreria: clic e clic sul pavimento
godot-document-coloca_um_objeto_shift_mantem = posiziona un oggetto (Shift mantiene lo strumento)
godot-document-biblioteca_arrastar_ate_o_chao = Libreria: trascina fino al pavimento
godot-document-coloca_um_objeto_onde_soltar = posiziona un oggetto dove rilasci
godot-document-setas = Frecce
godot-document-move_a_selecao_um_passo = sposta la selezione di un passo della griglia
godot-document-girar_90_anti_horario_visto = ruota di 90° (antiorario visto dall'alto)
godot-document-girar_90_horario = ruota di −90° (orario)
godot-document-girar_15_15 = ruota di 15° / −15°
godot-document-arrastar_a_alca_de_giro = Trascina la maniglia di rotazione (viola)
godot-document-gira_o_objeto_selecionado_em = ruota l'oggetto selezionato a passi di 15° (con Shift, 90°)
godot-document-exclui_a_selecao_ou_a = elimina la selezione (o il collegamento selezionato)
godot-document-ferramenta_ligar_arraste_da_origem = strumento Collega: trascina dall'origine alla destinazione, oppure clic e clic
godot-document-ferramenta_corredor_clique_pontos_no = strumento Corsia: clicca i punti sul pavimento, Enter o doppio clic termina
godot-document-ferramenta_zona_zonas_de_risco = strumento Zona: zone di rischio sul pavimento; clicca gli angoli, Enter o il primo punto chiude
godot-document-e_ou_alt_com_ligar = E (o Alt con Collega)
godot-document-ferramenta_equipe_arraste_do_despachante = strumento Squadra: trascina dal dispatcher al trasportatore, o dall'oggetto a chi ne trasporta gli elementi (Shift: operatori del processo)
godot-document-cancela_a_ferramenta_a_ligacao = annulla lo strumento, il collegamento o la corsia in corso, oppure la selezione
godot-document-simular_clique_num_objeto = Simula: clic su un oggetto
godot-document-abre_o_painel_de_resultados = apre il pannello dei risultati in tempo reale dell'oggetto nell'angolo della vista
godot-document-simular_esc = Simula: Esc
godot-document-fecha_o_painel_ao_vivo = chiude il pannello in tempo reale più recente (prima quello non fissato)
godot-document-f_editar_ou_home = F (Modifica) o Home
godot-document-enquadra_o_modelo = inquadra il modello
godot-document-simular_f = Simula: F
godot-document-anda_pela_fabrica_em_primeira = cammina per la fabbrica in prima persona: WASD cammina, il mouse guarda, Shift corre, E/Q sale e scende, V vola, la rotellina cambia la velocità, il clic sul mirino apre il pannello, Esc esce
godot-document-liga_ou_desliga_o_encaixe = attiva o disattiva l'aggancio alla griglia
godot-document-alterna_entre_editar_e_simular = alterna tra Modifica e Simula
godot-document-mostra_ou_esconde_esta_janela = mostra o nasconde questa finestra
godot-document-seleciona_todos_os_objetos = seleziona tutti gli oggetti
godot-document-copia_a_selecao = copia la selezione
godot-document-recorta_a_selecao = taglia la selezione
godot-document-cola_nomes_unicos_ligacoes_internas = incolla (nomi univoci, collegamenti interni preservati)
godot-document-duplica_a_selecao = duplica la selezione
godot-document-salva_como = salva con nome
godot-document-abre_um_modelo = apre un modello
godot-document-novo_modelo = nuovo modello

## editor3d.rs

godot-editor3d-opera = { $to } opera { $from }
godot-editor3d-leva_os_itens_de = { $to } trasporta gli elementi di { $from }
godot-editor3d-esta_na_equipe_de = { $to } è nella squadra di { $from }
godot-editor3d-ligacao_de = Collegamento di { $k }: { $from } → { $to }
godot-editor3d-arraste_do_no_laranja_dele = { $name }: trascina dal suo nodo arancione fino a una macchina perché la operi (o fino a un oggetto, per trasportarne gli elementi).
godot-editor3d-de_clique_no_no_de = {" "}({ $next } di { $links }: clicca di nuovo sul nodo per il successivo)
godot-editor3d-ligacao_selecionada_clique_no_ou = Collegamento { $a } → { $b } selezionato{ $extra }. Clicca sulla × o Del lo rimuove.
godot-editor3d-esta_ligacao_se_muda_no = Questo collegamento ({ $kind }) si modifica nel pannello delle proprietà di { $from }.
godot-editor3d-ligacao_removida_ctrl_z_desfaz = Collegamento { $a } → { $b } rimosso (Ctrl+Z annulla).
godot-editor3d-nenhuma_zona_selecionada_para_excluir = Nessuna zona selezionata da eliminare.
godot-editor3d-nada_selecionado_para_excluir = Nessuna selezione da eliminare.
godot-editor3d-zona_cancelada = Zona annullata.
godot-editor3d-ligacao_cancelada = Collegamento annullato.
godot-editor3d-girar_arraste_em_volta_do = Ruota: trascina attorno all'oggetto (passi di 15°; con Shift, 90°).
godot-editor3d-solte_sobre_a_maquina_que = Rilascia sulla macchina che { $res } opererà (o su un oggetto, per trasportarne gli elementi). Esc annulla.
godot-editor3d-ligar_solte_sobre_o_destino = Collega: rilascia sulla destinazione (o clicca su di essa). Esc annulla.
godot-editor3d-ligar_solte_sobre_a_origem = Collega: rilascia sull'origine (o sul suo nodo di uscita). Esc annulla.
godot-editor3d-ligar_clique_em_um_bloco = Collega: clicca su un blocco.
godot-editor3d-ligado_clique_em_outra_origem = Collegato: { $from } → { $to }. Clicca su un'altra origine o Esc.
godot-editor3d-ligacao_selecionada_clique_no_ou-2 = Collegamento { $a } → { $b } selezionato. Clicca sulla × o Del lo rimuove.
godot-editor3d-clique_no_ou_del_remove = { $said }. Clicca sulla × o Del lo rimuove.
godot-editor3d-de_blocos_neste_ponto_clique = { $name } ({ $index } di { $hits } blocchi in questo punto: clicca di nuovo per il successivo).
godot-editor3d-arraste_deste_no_verde_ate = { $from }: trascina da questo nodo verde fino alla destinazione per collegare.
godot-editor3d-ligar_clique_no_destino_esc = Collega: { $from } → clicca sulla destinazione (Esc annulla).
godot-editor3d-ligado = Collegato: { $from } → { $to }.
godot-editor3d-ligacao_cancelada_solte_sobre_o = Collegamento annullato: rilascia sul blocco di destinazione.
godot-editor3d-nao_recebe_de_ninguem_arraste = { $to } non riceve da nessuno: trascina dal nodo di uscita dell'origine fino a qui.
godot-editor3d-ligacao_cancelada_solte_sobre_o-2 = Collegamento annullato: rilascia sul blocco di origine.
godot-editor3d-girado_para = { $name } ruotato a { $degrees }°.
godot-editor3d-arraste_a_alca_em_volta = { $name }: trascina la maniglia attorno all'oggetto per ruotarlo (R ruota di 90°, Ctrl+R di 15°).
godot-editor3d-objeto_s_movidos =
    { $selected } { $selected ->
        [one] oggetto spostato
       *[other] oggetti spostati
    }.
godot-editor3d-objeto_s_na_caixa =
    { $inside } { $inside ->
        [one] oggetto nel riquadro
       *[other] oggetti nel riquadro
    }.
godot-editor3d-nada_mudou_solte_sobre_a = Nulla è cambiato: rilascia sulla macchina (o sull'oggetto) che { $res } servirà.
godot-editor3d-solte_sobre_uma_maquina_ou = Rilascia su una macchina o su un oggetto (un trasportatore entra solo nella squadra di un dispatcher).
godot-editor3d-ja_esta_ligado_para_desfazer = È già collegato: { $to } → { $res }. Per annullare, clicca sulla linea e sulla ×.
godot-editor3d-ligado_ctrl_z_desfaz = Collegato: { $said }. Ctrl+Z annulla.
godot-editor3d-aponte_para_o_chao_para = Punta al pavimento per posizionare l'oggetto.
godot-editor3d-equipe_clique_no_transportador_ou = Squadra: clicca sul trasportatore o dispatcher di destinazione.
godot-editor3d-equipe_comece_num_despachante_ou = Squadra: inizia da un dispatcher o da un oggetto.
godot-editor3d-equipe_clique_no_destino_esc = Squadra: { $from } → clicca sulla destinazione (Esc annulla).
godot-editor3d-equipe_solte_sobre_o_transportador = Squadra: rilascia sul trasportatore o dispatcher.
godot-editor3d-equipe_solte_sobre_outro_objeto = Squadra: rilascia su un altro oggetto.
godot-editor3d-aponte_para_o_chao_para-2 = Punta al pavimento per mettere un punto della corsia.
godot-editor3d-corredor_clique_os_proximos_pontos = Corsia: clicca i punti successivi; Enter o doppio clic termina, Backspace torna indietro di un punto, Esc annulla.
godot-editor3d-ponto_s_enter_ou_duplo =
    { $aisle_points } { $aisle_points ->
        [one] punto
       *[other] punti
    }. Enter o doppio clic termina.
godot-editor3d-vertice_de_selecionado_arraste_para = Vertice { $k } di { $name } selezionato (trascina per spostare).
godot-editor3d-corredor_selecionado_del_exclui_shift = Corsia { $name } selezionata (Del elimina; Shift + clic sulla linea aggiunge un vertice).
godot-editor3d-um_corredor_precisa_de_ao = Una corsia richiede almeno 2 punti.
godot-editor3d-ponto_removido = Punto rimosso ({ $aisle_points }).
godot-editor3d-trecho_selecionado_del_remove = Tratto { $a } → { $b } selezionato (Del rimuove).
godot-editor3d-aponte_para_o_chao_para-3 = Punta al pavimento per posizionare un nodo.
godot-editor3d-zona_ponto_s_clique_no =
    Zona: { $zone_points } { $zone_points ->
        [one] punto
       *[other] punti
    }. Clicca sul primo punto, Enter o doppio clic chiude; Backspace torna indietro; Esc annulla.
godot-editor3d-zona_arraste_para_mover_arraste = Zona { $name }: trascina per spostare, trascina un vertice per cambiare la forma; Del elimina.
godot-editor3d-zona_clique_para_por_os = Zona: clicca per mettere gli altri punti; Enter, doppio clic o il primo punto chiude.
godot-editor3d-zona_alterada_ctrl_z_desfaz = Zona modificata (Ctrl+Z annulla).
godot-editor3d-uma_zona_precisa_de_pelo = Una zona richiede almeno 3 punti.

## indicadores.rs

godot-indicadores-itens_refugados = Elementi scartati
godot-indicadores-itens_que_terminaram_neste_refugo = Elementi terminati in questo scarto, per replica.
godot-indicadores-defeitos_gerados = Difetti generati
godot-indicadores-defeitos_que_este_posto_gerou = Difetti generati da questa postazione, per replica.
godot-indicadores-defeitos_que_escaparam = Difetti sfuggiti
godot-indicadores-defeitos_deste_posto_que_chegaram = Difetti di questa postazione arrivati al cliente (guasto esterno), per replica.
godot-indicadores-rendimento_de_primeira = Resa al primo passaggio
godot-indicadores-itens_inspecionados_uma_so_vez = Elementi ispezionati una sola volta e consegnati, sul totale degli ispezionati.
godot-indicadores-custo_da_qualidade = Costo della qualità
godot-indicadores-avaliacao_falha_interna_e_falha = Valutazione, guasto interno e guasto esterno attribuiti a questa postazione, per replica.
godot-indicadores-tempo_em_zonas = Tempo nelle zone
godot-indicadores-tempo_desta_pessoa_dentro_das = Tempo di questa persona dentro le zone di rischio, per replica.
godot-indicadores-exposicao_nas_zonas = Esposizione nelle zone
godot-indicadores-tempo_de_pessoas_dentro_das = Tempo delle persone dentro le zone in cui si trova questa postazione, sommato, per replica.
godot-indicadores-zonas = Zone
godot-indicadores-zonas_de_risco_onde_o = Zone di rischio in cui si trova la postazione.
godot-indicadores-entradas_sem_epi = Ingressi senza DPI
godot-indicadores-entradas_na_zona_sem_o = Ingressi nella zona senza il dispositivo di protezione richiesto, per replica.
godot-indicadores-quase_conflitos = Quasi-conflitti
godot-indicadores-pessoa_e_veiculo_mais_perto = Persona e veicolo più vicini della soglia, che coinvolgono questa risorsa, per replica. Simulato: serve a confrontare layout, non stima incidenti reali.
godot-indicadores-pessoa_e_veiculo_mais_perto-2 = Persona e veicolo più vicini della soglia entro 3 m da questa postazione, per replica. Simulato: serve a confrontare layout, non stima incidenti reali.
godot-indicadores-quebras = Guasti
godot-indicadores-falhas_do_posto_por_replicacao = Guasti della postazione, per replica.
godot-indicadores-tempo_medio_entre_falhas_tempo = Tempo medio tra i guasti (tempo in funzione sul numero di guasti).
godot-indicadores-tempo_medio_de_reparo_da = Tempo medio di riparazione, dal guasto fino al ritorno alla produzione.
godot-indicadores-tempo_parado = Tempo fermo
godot-indicadores-tempo_em_manutencao_corretiva_por = Tempo in manutenzione correttiva, per replica.
godot-indicadores-disponibilidade = Disponibilità
godot-indicadores-tempo_produzindo_sobre_o_tempo = Tempo in produzione sul tempo pianificato (senza turno e preventive).
godot-indicadores-custo_de_manutencao = Costo di manutenzione
godot-indicadores-tempo_parado_reparos_e_preventivas = Tempo fermo, riparazioni e preventive per i costi del blocco, per replica.
godot-indicadores-parcela_do_tempo_em_quebra = Quota di tempo in guasto, per replica.
godot-indicadores-tempo_fora_de_quebra_sobre = Tempo fuori guasto sul tempo pianificato (senza turno).

## jobs.rs

godot-jobs-rodando_replicacoes = Esecuzione delle repliche
godot-jobs-rodando_mais_replicacoes = Esecuzione di altre repliche
godot-jobs-comparando_cenarios = Confronto degli scenari
godot-jobs-otimizando = Ottimizzazione
godot-jobs-analisando_o_aquecimento = Analisi del riscaldamento
godot-jobs-conferindo_a_lei_de_little = Verifica della legge di Little
godot-jobs-procurando_o_gargalo_movel = Ricerca del collo di bottiglia mobile
godot-jobs-calculando_o_pert_cpm = Calcolo del PERT/CPM
godot-jobs-ajustando_a_distribuicao_aos_dados = Adattamento della distribuzione ai dati
godot-jobs-gravando_o_relatorio_dos_dados = Scrittura del rapporto dei dati
godot-jobs-carregando_a_reproducao = Caricamento della riproduzione
godot-jobs-erro_interno_durante_a_execucao = errore interno durante l'esecuzione
godot-jobs-a_execucao_terminou_sem_resultado = l'esecuzione è terminata senza risultato
godot-jobs-objeto_nao_existe_no_modelo = l'oggetto "{ $object }" non esiste nel modello

## linha.rs


## path_edit.rs

godot-path-edit-nao_existe = '{ $name }' non esiste
godot-path-edit-e_uma_esteira_reta_sem = '{ $name }' è un trasportatore a nastro dritto, senza nodi
godot-path-edit-nao_e_uma_esteira = '{ $name }' non è un trasportatore a nastro
godot-path-edit-a_esteira_precisa_de_ao = il trasportatore a nastro richiede almeno 2 nodi diversi
godot-path-edit-a_biblioteca_nao_tem_esteira = la libreria non ha un trasportatore a nastro
godot-path-edit-o_caminho_precisa_de_ao = il percorso richiede almeno 2 punti diversi
godot-path-edit-nao_existe_o_caminho = il percorso '{ $name }' non esiste
godot-path-edit-nao_tem_o_no = '{ $name }' non ha il nodo { $index }
godot-path-edit-o_raio_deve_ser_um = il raggio deve essere un numero >= 0 (metri)
godot-path-edit-a_distancia_do_paralelo_deve = la distanza del parallelo deve essere diversa da zero
godot-path-edit-o_caminho_novo_sumiu = il nuovo percorso è sparito
godot-path-edit-o_caminho_precisa_de_ao-2 = il percorso richiede almeno 2 punti
godot-path-edit-a_esteira_precisa_de_ao-2 = il trasportatore a nastro richiede almeno 2 nodi
godot-path-edit-nao_e_uma_esteira_desenhada = '{ $name }' non è un trasportatore a nastro disegnato
godot-path-edit-as_duas_pontas_ja_estao = le due estremità sono già sul pavimento
godot-path-edit-tipo_de_esteira_desconhecido = tipo di trasportatore a nastro sconosciuto: { $style }
godot-path-edit-acionamento_desconhecido = azionamento sconosciuto: { $drive }
godot-path-edit-a_largura_deve_ser_0 = la larghezza deve essere > 0 (metri)
godot-path-edit-a_inclinacao_maxima_fica_entre = l'inclinazione massima è compresa tra 0 e 90 gradi
godot-path-edit-divida_num_no_do_meio = dividi in un nodo intermedio del trasportatore a nastro
godot-path-edit-nao_ha_trecho_depois_do = non c'è un tratto dopo l'ultimo nodo
godot-path-edit-o_caminho_so_tem_esse = il percorso ha solo questo tratto: elimina l'intero percorso
godot-path-edit-a_esteira_so_tem_esse = il trasportatore a nastro ha solo questo tratto: elimina l'intero trasportatore a nastro
godot-path-edit-nenhum_trajeto_comeca_onde_este = nessun tragitto inizia dove finisce questo
godot-path-edit-nao_existe_o_caminho-2 = il percorso '{ $other }' non esiste

## tool_panels.rs

godot-tool-panels-ligacao = Collegamento
godot-tool-panels-remover_ligacao_del = Rimuovi collegamento (Del)
godot-tool-panels-rede_de_caminhos = Rete di percorsi
godot-tool-panels-no_s_trecho_s_pista =
    { $nodes } { $nodes ->
        [one] nodo
       *[other] nodi
    }, { $edges } { $edges ->
        [one] tratto
       *[other] tratti
    }, { $controlled } { $controlled ->
        [one] via controllata
       *[other] vie controllate
    }.
godot-tool-panels-clique_no_chao_para_por = Clicca sul pavimento per mettere un nodo; con un nodo armato (verde), il clic successivo lo collega al nuovo con un tratto. Clicca su un nodo per armarlo, trascina per spostarlo. Clicca su un tratto per cambiare il senso e la capacità (con capacità, la via è controllata e diventa rossa). Esc termina il percorso; Del elimina il selezionato.
godot-tool-panels-novos_trechos_em_mao_dupla = Nuovi tratti a doppio senso
godot-tool-panels-no = Nodo { $name }
godot-tool-panels-nome = Nome
godot-tool-panels-posicao_m = Posizione ({ $position }, { $position2 }) m
godot-tool-panels-excluir_no_del = Elimina nodo (Del)
godot-tool-panels-trecho = Tratto { $from } → { $to }
godot-tool-panels-comprimento_m = Lunghezza { $length } m
godot-tool-panels-mao_dupla = Doppio senso
godot-tool-panels-capacidade_veiculos_0_sem_controle = Capacità (veicoli; 0 = senza controllo)
godot-tool-panels-pista_controlada_no_maximo_veiculo =
    Via controllata: al massimo { $cap } { $cap ->
        [one] veicolo
       *[other] veicoli
    } contemporaneamente.
godot-tool-panels-inverter_sentido = Inverti senso
godot-tool-panels-excluir_trecho_del = Elimina tratto (Del)
godot-tool-panels-pedestres = Pedoni
godot-tool-panels-empilhadeiras = Carrelli elevatori
godot-tool-panels-misto = Misto
godot-tool-panels-corredores_de_circulacao = Corsie di circolazione
godot-tool-panels-corredor_es_de_pedestres_de =
    { $list } { $list ->
        [one] corsia
       *[other] corsie
    }: { $pedestrian } pedonali, { $forklift } per carrelli elevatori, { $forklift2 } miste; { $crossings } { $crossings ->
        [one] incrocio
       *[other] incroci
    }.
godot-tool-panels-clique_no_chao_para_por-2 = Clicca sul pavimento per mettere i punti dell'asse; Enter o doppio clic termina, Backspace torna indietro di un punto, Esc annulla. I punti si agganciano alle altre corsie. Clicca su una corsia per modificarla, trascina un vertice per spostarlo, Shift + clic sulla linea aggiunge un vertice. Le strisce gialle, le zebrature degli incroci e le frecce compaiono sul pavimento.
godot-tool-panels-novos_corredores = Nuove corsie
godot-tool-panels-tipo = Tipo
godot-tool-panels-largura_m = Larghezza (m)
godot-tool-panels-corredor = Corsia { $name }
godot-tool-panels-ponto_s_m_de_comprimento =
    { $points } { $points ->
        [one] punto
       *[other] punti
    }, { $length } m di lunghezza
godot-tool-panels-passagem_secundaria_de_pedestres_minimo = Passaggio pedonale secondario (minimo ridotto)
godot-tool-panels-perfil = Profilo
godot-tool-panels-padrao_do_modelo = Predefinito del modello
godot-tool-panels-brasil_amarelo = Brasile (giallo)
godot-tool-panels-brasil = Brasile
godot-tool-panels-branco = Bianco
godot-tool-panels-vertice_selecionado = Vertice { $v } selezionato
godot-tool-panels-remover_vertice = Rimuovi vertice
godot-tool-panels-excluir_corredor_del = Elimina corsia (Del)
godot-tool-panels-zonas_de_risco = Zone di rischio
godot-tool-panels-zona_s_de_risco_no =
    { $zones } { $zones ->
        [one] zona
       *[other] zone
    } di rischio sul pavimento.
godot-tool-panels-clique_no_chao_para_por-3 = Clicca sul pavimento per mettere gli angoli; il primo punto, Enter o doppio clic chiude la zona, Backspace torna indietro di un punto, Esc annulla. Clicca su una zona per modificarla: trascina il centro per spostare, trascina un vertice per cambiare la forma; Del elimina. I punti si agganciano alla griglia.
godot-tool-panels-zona = Zona { $name }
godot-tool-panels-canto_s =
    { $polygon } { $polygon ->
        [one] angolo
       *[other] angoli
    }
godot-tool-panels-nivel_de_risco_1_a = Livello di rischio (da 1 a 5; 0 = senza livello)
godot-tool-panels-epi_exigido_para_entrar = DPI richiesti per entrare
godot-tool-panels-excluir_zona_del = Elimina zona (Del)

## ui_arrivals.rs

godot-ui-arrivals-nao_e_um_numero = "{ $t }" non è un numero
godot-ui-arrivals-use_um_numero = usa un numero
godot-ui-arrivals-hh_mm_so_vale_com = hh:mm vale solo con ciclo di giorno o settimana (1440 min, 24 h, 86400 s); usa un numero
godot-ui-arrivals-hora_invalida_em = ora non valida in "{ $t }"
godot-ui-arrivals-minuto_invalido_em = minuto non valido in "{ $t }"
godot-ui-arrivals-minuto_invalido_em_0_a = minuto non valido in "{ $t }" (da 0 a 59)
godot-ui-arrivals-nao_e_um_horario_hh = "{ $t }" non è un orario (hh:mm) né un numero
godot-ui-arrivals-quantidade_um_inteiro_0 = quantità "{ $t }": un intero >= 0
godot-ui-arrivals-rotulo_use_nome_valor = etichetta "{ $part }": usa nome=valore
godot-ui-arrivals-rotulo_sem_nome = etichetta "{ $part }" senza nome
godot-ui-arrivals-rotulo = etichetta { $name }: { $e }
godot-ui-arrivals-uma_vez = Una volta
godot-ui-arrivals-a_cada_ciclo = A ogni ciclo
godot-ui-arrivals-volta_ao_inicio = Torna all'inizio
godot-ui-arrivals-por_ciclo_de = per ciclo di { $d }
godot-ui-arrivals-por_volta = per giro
godot-ui-arrivals-no_total = in totale
godot-ui-arrivals-em_linha_s =
    { $items } { $items ->
        [one] elemento
       *[other] elementi
    } { $per } in { $rows } { $rows ->
        [one] riga
       *[other] righe
    }
godot-ui-arrivals-tipo_da_fonte = tipo della sorgente
godot-ui-arrivals-um_dia = { $d } (un giorno)
godot-ui-arrivals-uma_semana = { $d } (una settimana)
godot-ui-arrivals-tipo_de_item = tipo di elemento
godot-ui-arrivals-nada_para_colar_copie_as = niente da incollare: copia le righe dal foglio di calcolo
godot-ui-arrivals-linha = riga { $n }: { $e }
godot-ui-arrivals-falta_o_instante_modo_horario = manca l'istante (modalità orario)
godot-ui-arrivals-so_o_cabecalho_nenhuma_linha = solo l'intestazione: nessuna riga da incollare
godot-ui-arrivals-editar_como_texto_ron = Modifica come testo (RON)
godot-ui-arrivals-modo = Modalità
godot-ui-arrivals-horario = Orario
godot-ui-arrivals-sequencia = Sequenza
godot-ui-arrivals-horario_cada_linha_no_seu = Orario: ogni riga nel suo istante. Sequenza: una riga per arrivo, con il tempo tra gli arrivi della sorgente tra l'una e l'altra.
godot-ui-arrivals-repetir = Ripeti
godot-ui-arrivals-ciclo = Ciclo
godot-ui-arrivals-da_fonte = (della sorgente)
godot-ui-arrivals-o_tipo_da_fonte = Il tipo della sorgente
godot-ui-arrivals-tipo_de_item_da_linha = Tipo di elemento della riga
godot-ui-arrivals-subir_a_linha = Sposta su la riga
godot-ui-arrivals-descer_a_linha = Sposta giù la riga
godot-ui-arrivals-remover_a_linha = Rimuovi la riga
godot-ui-arrivals-linha-2 = + Riga
godot-ui-arrivals-acrescenta_uma_linha_no_fim = Aggiunge una riga in fondo
godot-ui-arrivals-ordenar_por_instante = Ordina per istante
godot-ui-arrivals-poe_as_linhas_em_ordem = Mette le righe in ordine di istante
godot-ui-arrivals-as_linhas_ja_estao_em = Le righe sono già in ordine (oppure è una sequenza)
godot-ui-arrivals-colar_da_planilha = Incolla dal foglio di calcolo
godot-ui-arrivals-linhas_copiadas_de_uma_planilha = Righe copiate da un foglio di calcolo (separate da tabulazione o ;)
godot-ui-arrivals-hora_qtd_tipo_rotulo_08 =
    ora; qtà; tipo; etichetta...
    08:00; 12; scatola; priorità=2
godot-ui-arrivals-qtd_tipo_rotulo_2_caixa =
    qtà; tipo; etichetta...
    2; scatola; priorità=2
godot-ui-arrivals-substituir_as_linhas = Sostituisci le righe
godot-ui-arrivals-acrescentar = Aggiungi
godot-ui-arrivals-ver_as_primeiras_chegadas = Vedi i primi arrivi
godot-ui-arrivals-histograma_das_chegadas_de_um = Istogramma degli arrivi di un ciclo (o di un giro), per ora e per tipo
godot-ui-arrivals-simulado_so_a_fonte_ligada = Simulato: solo la sorgente, collegata a un'uscita, per un ciclo.
godot-ui-arrivals-estimado_pela_tabela_a_fonte = Stimato dalla tabella (la sorgente da sola non è stata eseguita: { $e }).
godot-ui-arrivals-estimado_pela_tabela_rodando_a = Stimato dalla tabella; esecuzione della sorgente...
godot-ui-arrivals-maximo_por = massimo { $num_text } per { $bin_label }
godot-ui-arrivals-por_instantes_da_tabela_sem = per { $bin_label } ({ $bin }); istanti della tabella, senza eseguire il modello
godot-ui-arrivals-horarios_em_hh_mm_taxas = { $name } (orari in hh:mm, tassi per ora)
godot-ui-arrivals-unidade_do_modelo = unità del modello
godot-ui-arrivals-fora_dos_periodos_a_fonte = Fuori dai periodi la sorgente non crea elementi
godot-ui-arrivals-periodo_s =
    { $periods } { $periods ->
        [one] periodo
       *[other] periodi
    }
godot-ui-arrivals-itens_h_ou_distribuicao = elementi/h o distribuzione
godot-ui-arrivals-taxa_ou_distribuicao = tasso o distribuzione
godot-ui-arrivals-padrao_o_tempo_entre_chegadas = Predefinito: il tempo tra gli arrivi della sorgente. Distribuzione: un tempo tra gli arrivi proprio. Tasso: arrivi di Poisson. Chiuso: nessun arrivo.
godot-ui-arrivals-distribuicao = distribuzione: { $e }
godot-ui-arrivals-media = media { $mean }
godot-ui-arrivals-sem_chegadas = nessun arrivo
godot-ui-arrivals-remover_o_periodo = Rimuovi il periodo
godot-ui-arrivals-periodo = + Periodo
godot-ui-arrivals-acrescenta_um_periodo_depois_do = Aggiunge un periodo dopo l'ultimo
godot-ui-arrivals-o_ultimo_periodo_ja_vai = L'ultimo periodo arriva già fino alla fine del ciclo
godot-ui-arrivals-esperado_por_ciclo_chegadas = Atteso per ciclo: { $total } arrivi
godot-ui-arrivals-padrao_media = predefinito (media { $mean })
godot-ui-arrivals-distribuicao-2 = distribuzione { $d }
godot-ui-arrivals-taxa_itens_h = tasso { $cycle } elementi/h
godot-ui-arrivals-taxa = tasso { $num_text }
godot-ui-arrivals-periodo_a_esperadas = Periodo { $k }: { $cycle } a { $cycle2 }, { $how }; attesi { $default }
godot-ui-arrivals-chegadas_esperadas_em_cada_periodo = Arrivi attesi in ogni periodo (tasso × durata, o durata ÷ media)
godot-ui-arrivals-barra_s =
    { $exp } { $exp ->
        [one] barra
       *[other] barre
    }
godot-ui-arrivals-nao_existe = '{ $source }' non esiste
godot-ui-arrivals-nao_e_uma_chegada = '{ $source }' non è un arrivo

## ui_bar.rs

godot-ui-bar-equipe_despachante_membro = Squadra (dispatcher → membro)
godot-ui-bar-transporte_pedido = Trasporto richiesto
godot-ui-bar-operadores_pedidos = Operatori richiesti
godot-ui-bar-equipe_de_manutencao = Squadra di manutenzione
godot-ui-bar-local_de_pausa = Luogo di pausa
godot-ui-bar-mais = Altro
godot-ui-bar-os_grupos_que_nao_cabem = I gruppi che non entrano nella barra
godot-ui-bar-arquivo = File
godot-ui-bar-editar = Modifica
godot-ui-bar-organizar = Organizza
godot-ui-bar-grade = Griglia
godot-ui-bar-ajuda = Aiuto
godot-ui-bar-analise = Analisi
godot-ui-bar-novo = Nuovo
godot-ui-bar-novo_modelo_ctrl_n = Nuovo modello (Ctrl+N)
godot-ui-bar-abrir = Apri…
godot-ui-bar-abrir_um_modelo_ou_um = Apri un modello o un replay (Ctrl+O)
godot-ui-bar-salvar = Salva
godot-ui-bar-salvar_ctrl_s = Salva (Ctrl+S)
godot-ui-bar-salvar_como = Salva con nome…
godot-ui-bar-salvar_com_outro_nome_ctrl = Salva con un altro nome (Ctrl+Shift+S)
godot-ui-bar-desfazer = Annulla
godot-ui-bar-desfazer_ctrl_z = Annulla (Ctrl+Z)
godot-ui-bar-refazer = Ripeti
godot-ui-bar-refazer_ctrl_y = Ripeti (Ctrl+Y)
godot-ui-bar-alinhar_a_esquerda = Allinea a sinistra
godot-ui-bar-alinhar_ao_centro_x = Allinea al centro (X)
godot-ui-bar-alinhar_a_direita = Allinea a destra
godot-ui-bar-alinhar_ao_topo = Allinea in alto
godot-ui-bar-alinhar_ao_centro_y = Allinea al centro (Y)
godot-ui-bar-alinhar_a_base = Allinea in basso
godot-ui-bar-distribuir_em_x = Distribuisci in X
godot-ui-bar-distribuir_em_y = Distribuisci in Y
godot-ui-bar-copiar_ctrl_c = Copia (Ctrl+C)
godot-ui-bar-recortar_ctrl_x = Taglia (Ctrl+X)
godot-ui-bar-colar_ctrl_v = Incolla (Ctrl+V)
godot-ui-bar-duplicar_ctrl_d = Duplica (Ctrl+D)
godot-ui-bar-excluir_del = Elimina (Del)
godot-ui-bar-selecionar_tudo_ctrl_a = Seleziona tutto (Ctrl+A)
godot-ui-bar-alinhar_distribuir_copiar_colar_e = Allinea, distribuisci, copia, incolla ed elimina la selezione
godot-ui-bar-encaixar_na_grade_ao_por = Aggancia alla griglia quando posizioni e trascini (G; passo accanto)
godot-ui-bar-passo_da_grade = Passo della griglia
godot-ui-bar-atalhos_f1_a_lista_dos = Scorciatoie (F1): l'elenco delle scorciatoie di tastiera e mouse
godot-ui-bar-resultados = Risultati
godot-ui-bar-tabela_graficos_analises_cenarios_e = Tabella, grafici, analisi, scenari e ottimizzazione dell'ultima esecuzione
godot-ui-bar-pert_cpm = PERT/CPM
godot-ui-bar-pert_cpm_da_selecao_com = PERT/CPM della selezione (con due oggetti, dall'uno all'altro)
godot-ui-bar-pert_cpm_da_selecao = PERT/CPM della selezione
godot-ui-bar-pert_cpm_de_ate_dois = PERT/CPM da… a… (due selezionati)
godot-ui-bar-fluxos_de_tokens = Flussi di token
godot-ui-bar-modo_missao_utsim_ensino = Modalità missione (UTSim Didattica)
godot-ui-bar-abre_as_missoes_do_utsim = Apre le missioni di UTSim Didattica in questo stesso programma
godot-ui-bar-outras_analises_pert_cpm_de = Altre analisi: PERT/CPM da… a…, flussi di token; i collegamenti di squadra e di richiesta nella vista 3D
godot-ui-bar-simular = Simula
godot-ui-bar-montar_o_modelo_tab_alterna = Costruire il modello (Tab alterna)
godot-ui-bar-rodar_as_replicacoes_e_reproduzir = Eseguire le repliche e riprodurre (Tab alterna)
godot-ui-bar-selecionar = Seleziona
godot-ui-bar-selecionar_e_mover_esc = Seleziona e sposta (Esc)
godot-ui-bar-ligar = Collega
godot-ui-bar-ligar_arraste_da_origem_ao = Collega: trascina dall'origine alla destinazione, oppure clicca sull'origine e poi sulla destinazione (L)
godot-ui-bar-rede = Rete
godot-ui-bar-rede_de_caminhos_nos_e = Rete di percorsi: nodi e tratti (N)
godot-ui-bar-corredor = Corsia
godot-ui-bar-corredores_de_circulacao_com_faixas = Corsie di circolazione con strisce di sicurezza: clicca i punti sul pavimento, Enter termina (C)
godot-ui-bar-zona = Zona
godot-ui-bar-zonas_de_risco_no_piso = Zone di rischio sul pavimento: clicca gli angoli, il primo punto o Enter chiude; clicca su una zona per modificarla (Z)
godot-ui-bar-equipe = Squadra
godot-ui-bar-equipe_arraste_do_despachante_a = Squadra: trascina dal dispatcher a un trasportatore (entra o esce dalla squadra), o da un oggetto al trasportatore o dispatcher che ne trasporta gli elementi; con Shift, da un processo a chi fornisce gli operatori (E; Alt con Collega)
godot-ui-bar-ligacoes_na_vista_3d = Collegamenti nella vista 3D
godot-ui-bar-tambem_durante_a_simulacao = Anche durante la simulazione
godot-ui-bar-as_linhas_tracejadas_entre_operadores = Le linee tratteggiate tra operatori, veicoli e macchine in modalità Simula

## ui_fitwin.rs

godot-ui-fitwin-linha_nao_e_um_numero = riga { $k }: "{ $t }" non è un numero
godot-ui-fitwin-os_instantes_precisam_ser_crescentes = gli istanti devono essere crescenti
godot-ui-fitwin-digite_ou_cole_pelo_menos = digita o incolla almeno due valori
godot-ui-fitwin-compare_as_distribuicoes_antes = confronta prima le distribuzioni
godot-ui-fitwin-utsim_3d_valores_reais = UTSim 3D, Valori reali
godot-ui-fitwin-desvio_padrao = deviazione standard
godot-ui-fitwin-valores_reais = Valori reali: { $field }
godot-ui-fitwin-o_que_e = Cos'è: { $title }
godot-ui-fitwin-valores_observados = Valori osservati
godot-ui-fitwin-digite_cole_da_planilha_varias = Digita, incolla dal foglio di calcolo (più righe, virgola decimale) o importa un CSV.
godot-ui-fitwin-cole_aqui_e_clique_adicionar = incolla qui e clicca Aggiungi
godot-ui-fitwin-adicionar = Aggiungi
godot-ui-fitwin-valor_es_adicionados =
    { $n } { $n ->
        [one] valore aggiunto
       *[other] valori aggiunti
    }.
godot-ui-fitwin-importar_csv = Importa CSV…
godot-ui-fitwin-le_os_valores_de_um = Legge i valori da un file CSV o TXT
godot-ui-fitwin-limpar = Svuota
godot-ui-fitwin-apaga_todos_os_valores_da = Cancella tutti i valori della tabella
godot-ui-fitwin-sao_instantes_crescentes_usar_os = Sono istanti crescenti (usa gli intervalli)
godot-ui-fitwin-marque_quando_os_valores_sao = Spunta quando i valori sono orari di arrivo: l'adattamento usa le differenze tra di essi.
godot-ui-fitwin-valor_es =
    { $n } { $n ->
        [one] valore
       *[other] valori
    }
godot-ui-fitwin-apaga_este_valor = Cancella questo valore
godot-ui-fitwin-intervalos = Intervalli
godot-ui-fitwin-estatisticas = Statistiche
godot-ui-fitwin-comparacao_das_distribuicoes = Confronto delle distribuzioni
godot-ui-fitwin-comparar_de_novo = Confronta di nuovo
godot-ui-fitwin-comparar_todas = Confronta tutte
godot-ui-fitwin-ajusta_todas_as_distribuicoes_candidatas = Adatta tutte le distribuzioni candidate e calcola tutte le misure in una volta
godot-ui-fitwin-ajustando_todas_as_candidatas = adattamento di tutte le candidate...
godot-ui-fitwin-depois_de_comparar_a_tabela = Dopo il confronto, la tabella mostra tutte le misure di ogni distribuzione; clicca su una riga per vedere i grafici e scegliere.
godot-ui-fitwin-os_valores_mudaram_compare_de = I valori sono cambiati: confronta di nuovo.
godot-ui-fitwin-ordenada_por_sugestao_inicial_aic = Ordinata per { $label }. Suggerimento iniziale (AIC con la regola di UTSim): { $family }.
godot-ui-fitwin-usar_esta_distribuicao = Usa questa distribuzione ({ $fam })
godot-ui-fitwin-passa_a_usar_esta_distribuicao = Passa a usare questa distribuzione in questo campo, adattata a questi valori (Ctrl+Z annulla)
godot-ui-fitwin-usar_a_empirica = Usa l'empirica
godot-ui-fitwin-usa_os_proprios_valores_observados = Usa i valori osservati stessi, senza famiglia
godot-ui-fitwin-gerar_relatorio = Genera rapporto
godot-ui-fitwin-grava_o_relatorio_html_com = Scrive il rapporto HTML con i dati, il confronto completo, i grafici e la scelta
godot-ui-fitwin-fechar = Chiudi
godot-ui-fitwin-fecha_a_explicacao = Chiude la spiegazione
godot-ui-fitwin-como_ler_quando_usar_cuidados =
    { $what }
    Come leggere: { $read }
    Quando usare: { $when }
    Attenzione: { $care }
godot-ui-fitwin-aceita_pelos_testes_clique_para = Accettata dai test. Clicca per vedere i grafici e scegliere.
godot-ui-fitwin-nao_se_aplica_a_estes = Non si applica a questi dati.
godot-ui-fitwin-rejeitada_por_algum_teste_valor = Rifiutata da qualche test (valore p inferiore a 0,05). Può comunque essere scelta.
godot-ui-fitwin-ordena_por = Ordina per { $label } ({ $melhor })
godot-ui-fitwin-escolhida_por = scelta da
godot-ui-fitwin-histograma_e_curvas_em_destaque = Istogramma e curve ({ $selected_family } in evidenza)
godot-ui-fitwin-poisson = Poisson
godot-ui-fitwin-geometrica = Geometrica
godot-ui-fitwin-binomial = Binomiale
godot-ui-fitwin-binomial_negativa = Binomiale negativa
godot-ui-fitwin-a_empirica_reproduz_os_proprios = L'empirica riproduce i dati stessi: non c'è una curva da confrontare.

## ui_left.rs

godot-ui-left-orbita = Orbita
godot-ui-left-planta_2d = Pianta (2D)
godot-ui-left-seguir = Segui
godot-ui-left-primeira_pessoa = Prima persona
godot-ui-left-do_recurso = Dalla risorsa
godot-ui-left-recentes = Recenti
godot-ui-left-usados_por_ultimo = Usati per ultimi
godot-ui-left-espera = Attesa
godot-ui-left-fluxo = Flusso
godot-ui-left-fluxo_de_itens_e_montagem = Flusso di elementi e assemblaggio
godot-ui-left-mover = Movimentare
godot-ui-left-movimentacao_recursos_e_logistica = Movimentazione, risorse e logistica
godot-ui-left-fabrica = Fabbrica
godot-ui-left-manufatura = Manifattura
godot-ui-left-servicos = Servizi
godot-ui-left-servicos_e_saude = Servizi e sanità
godot-ui-left-escritorio = Ufficio
godot-ui-left-escritorio_e_processos = Ufficio e processi
godot-ui-left-q_e_s = Q e S
godot-ui-left-qualidade_e_seguranca = Qualità e sicurezza
godot-ui-left-biblioteca = Libreria
godot-ui-left-objetos = Oggetti
godot-ui-left-mostrar_obstaculos_e_folgas = Mostra ostacoli e margini
godot-ui-left-mostra_a_area_ocupada_dos = Mostra l'area occupata dagli oggetti, le pareti, il margine delle risorse e il punto di accesso di ogni oggetto (trascinalo con lo strumento Seleziona)
godot-ui-left-nada_com_esse_nome = Niente con questo nome.
godot-ui-left-clique_de_novo_para_recolher = { $title }. Clicca di nuovo per comprimere.
godot-ui-left-os_elementos_que_voce_usar = Gli elementi che userai compaiono qui.
godot-ui-left-buscar_elemento = Cerca elemento
godot-ui-left-limpa_a_busca = Cancella la ricerca
godot-ui-left-clique_e_depois_clique_no = { $label }: clicca e poi clicca sul pavimento, oppure trascina fino al pavimento
godot-ui-left-buscar_por_nome_ou_tipo = Cerca per nome o tipo
godot-ui-left-problema_s =
    { $problems } { $problems ->
        [one] problema
       *[other] problemi
    }
godot-ui-left-aviso_s =
    { $warnings } { $warnings ->
        [one] avviso
       *[other] avvisi
    }
godot-ui-left-simular_de_novo = Simula di nuovo
godot-ui-left-roda_as_replicacoes_outra_vez = Esegue di nuovo le repliche
godot-ui-left-blocos_clique_para_focar = Blocchi (clicca per mettere a fuoco)
godot-ui-left-fluxos = Flussi
godot-ui-left-fluxos_de_tokens_em_2d = Flussi di token in 2D, con i conteggi nel tempo
godot-ui-left-gravar_replay = Registra replay
godot-ui-left-grava_o_replay_desta_replicacao = Registra il replay di questa replica in .json (si riapre con Apri)
godot-ui-left-rotulos = Etichette
godot-ui-left-os_nomes_dos_blocos_sobre = I nomi dei blocchi sulla scena
godot-ui-left-sombras = Ombre
godot-ui-left-as_sombras_do_sol_desligadas = Le ombre del sole (disattivate, la scena è più leggera)
godot-ui-left-seguranca = Sicurezza
godot-ui-left-zonas_de_risco_epi_das = Zone di rischio, DPI delle persone, quasi-conflitti simulati, elementi difettosi e ispezioni
godot-ui-left-calor = Calore
godot-ui-left-mapa_de_calor_da_presenca = Mappa di calore della presenza delle persone e dei quasi-conflitti sul pavimento
godot-ui-left-quase_conflitos_simulados = Quasi-conflitti simulati ({ $quase })
godot-ui-left-replicacoes = Repliche
godot-ui-left-uma_replicacao_a_menos = Una replica in meno
godot-ui-left-uma_replicacao_a_mais = Una replica in più
godot-ui-left-camera = Camera
godot-ui-left-seguir_primeira_pessoa_e_do = Segui, Prima persona e Dalla risorsa accompagnano il blocco scelto nell'elenco
godot-ui-left-os_resultados_aparecem_aqui_depois = I risultati compaiono qui dopo la simulazione.
godot-ui-left-gargalo = Collo di bottiglia: { $bottleneck }
godot-ui-left-a_tabela_completa_os_graficos = La tabella completa, i grafici e le analisi sono in Risultati.
godot-ui-left-gargalo-2 =
    {""}
    Collo di bottiglia: { $bottleneck }

## ui_pert.rs

godot-ui-pert-rede = Rete
godot-ui-pert-gantt = Gantt
godot-ui-pert-distribuicao = Distribuzione
godot-ui-pert-tabela = Tabella
godot-ui-pert-texto = Testo
godot-ui-pert-simulado = Simulato
godot-ui-pert-caminho_critico_observado_na_simulacao = Percorso critico osservato nella simulazione: code, trasporti e risorse inclusi
godot-ui-pert-classico = Classico
godot-ui-pert-pert_classico_estimativas_a_m = PERT classico: stime (a, m, b), CPM sulle medie e P(scadenza) con la normale
godot-ui-pert-monte_carlo = Monte Carlo
godot-ui-pert-sorteios_das_duracoes_sobre_a = Estrazioni delle durate sulla rete: criticità e distribuzione del tempo totale
godot-ui-pert-desvio = , deviazione { $sd }
godot-ui-pert-unidades_sorteios = {" "}({ $units } unità/estrazioni)
godot-ui-pert-tempo_total_cpm_nas_medias = Tempo totale: { $t }{ $spread }{ $units }. CPM sulle medie: { $project }.
godot-ui-pert-ic_a = {" "}(IC { $lo }% a { $hi }%)
godot-ui-pert-p_t = P(T ≤ { $deadline }) = { $p }
godot-ui-pert-caminho_critico = Percorso critico: { $names } ({ $index }%)
godot-ui-pert-aviso = avviso: { $w }
godot-ui-pert-aproximacao_normal_do_pert_classico = approssimazione normale del PERT classico
godot-ui-pert-unidades_observadas = unità osservate
godot-ui-pert-a_linha_amarela_e_o = ; la linea gialla è la scadenza
godot-ui-pert-distribuicao_do_tempo_total = Distribuzione del tempo totale ({ $source }){ $marker }.
godot-ui-pert-pert_cpm_tempo_total = PERT/CPM ({ $mode }): tempo totale { $total }.
godot-ui-pert-atividade = Attività
godot-ui-pert-duracao = Durata
godot-ui-pert-folga_total = Margine totale
godot-ui-pert-folga_livre = Margine libero
godot-ui-pert-criticidade = Criticità
godot-ui-pert-prazo = scadenza { $num }
godot-ui-pert-sem_distribuicao_para_este_modo = nessuna distribuzione per questa modalità
godot-ui-pert-hipotese = ipotesi: { $h }
godot-ui-pert-fechar = Chiudi
godot-ui-pert-fechar_a_janela_pert_cpm = Chiudi la finestra PERT/CPM
godot-ui-pert-prazo-2 = Scadenza
godot-ui-pert-sem_prazo = nessuna scadenza
godot-ui-pert-sorteios_monte_carlo = Estrazioni (Monte Carlo)
godot-ui-pert-recalcular = Ricalcola
godot-ui-pert-calcula_de_novo_no_modo = Calcola di nuovo nella modalità attuale
godot-ui-pert-relatorio_html = rapporto .html
godot-ui-pert-exportar_html = Esporta HTML
godot-ui-pert-grava_o_relatorio_html_do = Scrive il rapporto HTML del PERT/CPM
godot-ui-pert-csv = CSV
godot-ui-pert-grava_as_tabelas_em_csv = Scrive le tabelle in CSV nella cartella del file

## zone_edit.rs

godot-zone-edit-trafego_de_veiculos = Traffico di veicoli
godot-zone-edit-alcance_de_maquina = Raggio d'azione della macchina
godot-zone-edit-carga_suspensa = Carico sospeso
godot-zone-edit-ruido = Rumore
godot-zone-edit-calor = Calore
godot-zone-edit-outro = Altro
godot-zone-edit-a_zona_nao_existe = la zona '{ $name }' non esiste
godot-zone-edit-uma_zona_precisa_de_pelo = una zona richiede almeno 3 punti
godot-zone-edit-a_zona_precisa_de_um = la zona richiede un nome
godot-zone-edit-tipo_de_zona_desconhecido = tipo di zona sconosciuto: { $value }
godot-zone-edit-nivel_um_numero_de_1 = livello: un numero da 1 a 5
godot-zone-edit-campo_de_zona_desconhecido = campo di zona sconosciuto: { $field }

## Giro 13: finestre di dialogo e messaggi di errore

godot-dialogo-descartar-titulo = Scartare le modifiche?
godot-dialogo-descartar-texto = Il modello ha modifiche non salvate. Scartare?
godot-dialogo-descartar = Scarta
godot-dialogo-voltar = Indietro
godot-dialogo-atalhos = Scorciatoie
godot-dialogo-fechar = Chiudi
godot-dialogo-desempenho = Mostra o nasconde le misure di prestazione sulla vista 3D

godot-schema-distribuicao_invalida = distribuzione non valida: { $e }
godot-schema-familia_invalida = famiglia non valida
godot-schema-texto_ron_esperado = testo RON atteso
godot-schema-so_a_distribuicao_deslocada_tem = solo la distribuzione traslata ha una distribuzione interna
godot-schema-a_distribuicao_nao_tem_o = la distribuzione non ha il parametro "{ $key }"
godot-schema-modo_de_movimentacao_desconhecido = modalità di movimentazione sconosciuta: { $name }
godot-schema-modo_de_movimentacao_esperado = modalità di movimentazione attesa
godot-schema-a_velocidade_deve_ser_maior = la velocità deve essere maggiore di zero
godot-schema-campo_desconhecido = campo sconosciuto: { $w }
godot-schema-rotulo_esperado = etichetta attesa
godot-schema-o_rotulo_do_peso_nao = l'etichetta del peso non può restare vuota
godot-schema-ordem_safe_sequential_ou_parallel = ordine: Safe, Sequential o Parallel
godot-schema-crie_antes_uma_pessoa_ou = crea prima una persona (o un dispatcher) per azionare il carroponte
godot-schema-controle_cabin_pendant_ou_radio = controllo: Cabin, Pendant o Radio
godot-schema-escolha_a_equipe_do_operador = scegli la squadra dell'operatore
godot-schema-escolha_antes_botoeira_ou_radio = scegli prima pulsantiera o radio
godot-schema-tipo_de_espera_table_pallet = tipo di attesa: Table, Pallet, Shelf o Floor, non { $k }
godot-schema-camadas_demais = troppi strati
godot-schema-espera_nao_tem_o_campo = attesa non ha il campo "{ $rest }"
godot-schema-espera = attesa: { $e }
godot-schema-sexo_female_male_ou_vazio = sesso: Female, Male o vuoto, non { $other }
godot-schema-idade_grande_demais = età troppo alta
godot-schema-pessoa_nao_tem_o_campo = persona non ha il campo "{ $field }"
godot-schema-pessoa = persona: { $e }
godot-schema-crie_antes_um_transportador_ou = crea prima un trasportatore o un dispatcher per far funzionare il processo
godot-schema-ligue_a_transferencia_antes = attiva prima il trasferimento
godot-schema-caminho_straight_ou_network = percorso: Straight o Network
godot-schema-ligue_as_quebras_antes = attiva prima le pause
godot-schema-ligue_os_turnos_antes = attiva prima i turni
godot-schema-fim_do_turno_pause_ou = fine del turno: Pause o Finish, non { $t }
godot-schema-pausas_invalidas = pause non valide: { $e }
godot-schema-escolha_um_calendario = scegli un calendario
godot-schema-ligue_o_lote_antes = attiva prima il lotto
godot-schema-ligue_antes = attiva prima { $head }
godot-schema-ligue_os_operadores_antes = attiva prima gli operatori
godot-schema-o_ganho_da_equipe_deve = il guadagno della squadra deve essere maggiore di zero
godot-schema-objeto_nao_existe = l'oggetto "{ $object }" non esiste
godot-schema-posicao_x_y_ou_z = posizione: x, y o z
godot-schema-politica_esperada = politica attesa
godot-schema-politica_de_acesso_desconhecida = politica di accesso sconosciuta: { $v }
godot-schema-o_rotulo_da_prioridade_nao = l'etichetta della priorità non può restare vuota
godot-schema-script_esperado = script atteso
godot-schema-regra_firstfree_random_ou_abc = regola: FirstFree, Random o Abc
godot-schema-o_rotulo_nao_pode_ficar = l'etichetta non può restare vuota
godot-schema-modo_unpack_ou_split = modalità: Unpack o Split
godot-schema-o_modelo_nao_tem_calendarios = il modello non ha calendari (creane uno in "calendars")
godot-schema-origem_dos_turnos_inline_ou = origine dei turni: inline o calendar
godot-schema-texto_esperado = testo atteso
godot-schema-modo_esperado = modalità attesa
godot-schema-perfil_brasil_ou_branco = profilo: Brasil o Branco
godot-schema-largura_da_faixa = larghezza della corsia
godot-schema-parada_no_cruzamento = sosta all'incrocio
godot-schema-cor_invalida_use_rrggbb = colore non valido: { $t } (usa #rrggbb)
godot-schema-cor_rrggbb = colore: #rrggbb
godot-schema-run_nao_tem_o_campo = run non ha il campo "{ $path }"
godot-schema-regra_esperada = regola attesa
godot-schema-lista_de_pesos_esperada = elenco di pesi atteso
godot-schema-pesos_invalidos = pesi non validi: { $e }
godot-schema-campo_desconhecido_output = campo sconosciuto: output.{ $rest }
godot-schema-a_escala_precisa_ser_maior = la scala deve essere maggiore di zero
godot-schema-giro_um_numero_de_graus = rotazione: un numero di gradi
godot-schema-campo_desconhecido_visual = campo sconosciuto: visual.{ $rest }

godot-ui-bar-casa = Casa

## Rodada 14: messaggi e pannelli del GDScript (`UtsimDocument.texto`)

godot-gd-obstaculos-visiveis = Ostacoli e spazi liberi visibili.
godot-gd-obstaculos-ocultos = Ostacoli e spazi liberi nascosti.
godot-gd-desempenho-visiveis = Misure di prestazione visibili (F3).
godot-gd-desempenho-ocultas = Misure di prestazione nascoste (F3).
godot-gd-mao-dupla = doppio senso
godot-gd-mao-unica = senso unico
godot-gd-mao-unica-setas = senso unico (frecce a terra)
godot-gd-do-modelo = quello del modello
godot-gd-foi-cancelada = è stata annullata
godot-gd-falhou = non è riuscita
godot-gd-ligada = attivata
godot-gd-desligada = disattivata
godot-gd-ligado = attivato
godot-gd-desligado = disattivato
godot-gd-abrindo-leituras = Apertura delle letture di { $p1 } in UTSim Statistica.
godot-gd-valores-lidos-de = Valori letti da { $p1 }.
godot-gd-nao-leu = Impossibile leggere { $p1 }: { $p2 }
godot-gd-objetos-copiados =
    { $p1 } { $p1 ->
        [one] oggetto copiato
       *[other] oggetti copiati
    }.
godot-gd-nada-para-copiar = Nessuna selezione da copiare.
godot-gd-removida-desfaz = Rimossa: { $p1 } (Ctrl+Z annulla)
godot-gd-nao-removeu-msg = Non rimosso: { $p1 }
godot-gd-nao-ligou-msg = Non collegato: { $p1 }
godot-gd-passagem-cabem = , { $p1 } s di passaggio, ne entrano { $p2 }
godot-gd-esteira-desenhada = Nastro disegnato
godot-gd-esteira-com-canaletas = Trasportatore a nastro con canaline
godot-gd-caminho-no-piso = Percorso a terra
godot-gd-pedestres = Pedoni
godot-gd-empilhadeiras = Carrelli elevatori
godot-gd-misto = Misto
godot-gd-trajeto-inclinacao-pct = Inclinazione (%; negativa scende)
godot-gd-trajeto-acima-maximo = • Oltre il massimo: il pezzo scivola o si ribalta.
godot-gd-trajeto-rampa-ingreme = • Rampa più ripida di 1:12 (8,33 %).
godot-gd-esteira-simples = Trasportatore a nastro semplice
godot-gd-motorizada = Motorizzato
godot-gd-roletes-gravidade = Rulli a gravità
godot-gd-calha = Scivolo
godot-gd-trajeto-caminho-n = Percorso { $p1 }
godot-gd-trajeto-esteira-n = Nastro { $p1 }
godot-gd-trajeto-no-n = Nodo { $p1 }

godot-gd-corredores-corredor_cancelado_clique_no_chao = Corsia annullata. Clicca a terra per iniziarne un'altra, oppure Esc per uscire dallo strumento.
godot-gd-corredores-nao_criou_o_corredor = Corsia non creata: { $p1 }
godot-gd-corredores-corredor_criado_m = Corsia { $p1 } creata ({ $p2 }, { $p3 } m).
godot-gd-corredores-vertice_de_movido = Vertice { $p1 } di { $p2 } spostato.
godot-gd-corredores-nao_moveu = Non spostato: { $p1 }
godot-gd-corredores-vertice_acrescentado_a = Vertice aggiunto a { $p1 }.
godot-gd-corredores-nao_acrescentou = Non aggiunto: { $p1 }
godot-gd-corredores-corredor_excluido = Corsia { $p1 } eliminata.
godot-gd-corredores-nao_excluiu = Non eliminato: { $p1 }
godot-gd-corredores-ponto_de_acesso_de_movido = Punto di accesso di { $p1 } spostato (disattiva "Punto di accesso proprio" nel pannello per tornare a quello predefinito).
godot-gd-corredores-nao_moveu_o_ponto_de = Punto di accesso non spostato: { $p1 }
godot-gd-corredores-corredor_agora_e_de = La corsia { $p1 } ora è { $p2 }.
godot-gd-corredores-corredor_com_m_de_largura = Corsia { $p1 } larga { $p2 } m.
godot-gd-corredores-nao_mudou_a_largura = Larghezza non modificata: { $p1 }
godot-gd-corredores-corredor_em = Corsia { $p1 } in { $p2 }.
godot-gd-corredores-perfil_do_corredor = Profilo della corsia { $p1 }: { $p2 }.
godot-gd-corredores-sentido_do_corredor_invertido = Senso della corsia { $p1 } invertito.
godot-gd-corredores-corredor_agora_se_chama = La corsia { $p1 } ora si chiama { $p2 }.
godot-gd-corredores-nao_renomeou = Non rinominato: { $p1 }
godot-gd-corredores-vertice_removido = Vertice rimosso.
godot-gd-corredores-nao_removeu = Non rimosso: { $p1 }
godot-gd-corredores-esse_aviso_nao_aponta_para = Questo avviso non rimanda ad alcuna corsia o oggetto della scena.
godot-gd-corredores-destacado_na_cena = Evidenziato nella scena: { $p1 }.
godot-gd-editor-equipe_arraste_do_despachante_a = Squadra: trascina dallo smistatore a un trasportatore, oppure da un oggetto a chi porta gli articoli (Shift: da un processo a chi fornisce gli operatori); ripetendo si annulla. Più semplice: trascina dal nodo arancione dell'operatore alla macchina. Esc annulla.
godot-gd-editor-ligar_arraste_do_bloco_de = Collega: trascina dal blocco di origine alla destinazione, oppure clicca su uno e poi sull'altro (Esc annulla).
godot-gd-editor-clique_no_chao_para_colocar = Clicca a terra per posizionare (Shift mantiene lo strumento, Esc annulla).
godot-gd-editor-corredor_clique_no_chao_para = Corsia: clicca a terra per mettere i punti dell'asse (Invio o doppio clic termina), clicca su una corsia per modificarla, trascina un vertice per spostarlo; Esc annulla.
godot-gd-editor-zona_clique_no_chao_para = Zona: clicca a terra per mettere gli angoli della zona di rischio; il primo punto, Invio o doppio clic la chiude; clicca su una zona per modificarla. Esc annulla.
godot-gd-editor-rede_clique_no_chao_para = Rete: clicca a terra per mettere i nodi (ogni clic prosegue il percorso), clicca su un tratto per modificarlo, trascina un nodo per spostarlo; Esc termina il percorso.
godot-gd-editor-nao_mudou_a_zona = Zona non modificata: { $p1 }
godot-gd-editor-nao_ligou = Non collegato: { $p1 }
godot-gd-editor-nao_removeu = Non rimosso: { $p1 }
godot-gd-editor-nao_girou = Non ruotato: { $p1 }
godot-gd-editor-nao_criou_a_zona = Zona non creata: { $p1 }
godot-gd-editor-zona_criada_escolha_o_tipo = Zona { $p1 } creata: scegli il tipo, il livello e il DPI nel pannello (Ctrl+Z annulla).
godot-gd-editor-zona_excluida_ctrl_z_desfaz = Zona { $p1 } eliminata (Ctrl+Z annulla).
godot-gd-editor-nao_excluiu_a_zona = Zona non eliminata: { $p1 }
godot-gd-editor-objeto_s_excluidos =
    { $p1 } { $p1 ->
        [one] oggetto eliminato
       *[other] oggetti eliminati
    }.
godot-gd-editor-colocacao_cancelada_solte_o_objeto = Posizionamento annullato: rilascia l'oggetto sul pavimento.
godot-gd-editor-aponte_para_o_chao_para = Punta il pavimento per posizionare l'oggetto.
godot-gd-editor-nao_foi_possivel_colocar = Impossibile posizionare: { $p1 }
godot-gd-editor-colocado = { $p1 } posizionato.
godot-gd-editor-equipe_cancelado = Squadra: annullato.
godot-gd-editor-nada_selecionado_para_girar = Nessuna selezione da ruotare.
godot-gd-editor-girado_ctrl_z_desfaz = Ruotato { $p1 } (Ctrl+Z annulla).
godot-gd-editor-objetos_girados_cada_um_em = { $p1 } oggetti ruotati { $p2 }, ciascuno attorno al proprio centro (Ctrl+Z annulla).
godot-gd-editor-nenhum_corredor_selecionado_para_excluir = Nessuna corsia selezionata da eliminare.
godot-gd-editor-nenhum_no_selecionado_para_remover = Nessun nodo selezionato da rimuovere.
godot-gd-principal-arraste_um_objeto_da_biblioteca = Trascina un oggetto dalla Libreria sul pavimento, oppure apri un modello (Ctrl+O).
godot-gd-principal-encaixe_na_grade_passo_m = Aggancio alla griglia { $p1 } (passo { $p2 } m).
godot-gd-principal-o_modo_missao_do_utsim = La modalità missione di UTSim Didattica non è in questa versione.
godot-gd-principal-para_andar_pela_fabrica_simule = Per camminare nella fabbrica, simula prima (F in Simula, dopo il giro).
godot-gd-principal-alterado = { $p1 }: { $p2 } modificato.
godot-gd-principal-nao_alterou = { $p1 } non modificato: { $p2 }
godot-gd-principal-novo_modelo = Nuovo modello.
godot-gd-principal-nao_abriu = { $p1 } non aperto: { $p2 }
godot-gd-principal-aberto_mas_salvar_fica_para = { $p1 } aperto, ma { $p2 }: Salva resta per quando la persona chiude (oppure usa Salva con nome).
godot-gd-principal-aberto_objetos_simular_roda_as = { $p1 } aperto: { $p2 } oggetti. Simula esegue le repliche.
godot-gd-principal-aberto_com_problema_s_veja =
    { $p1 } aperto con { $p2 } { $p2 ->
        [one] problema
       *[other] problemi
    }: vedi il pannello delle proprietà.
godot-gd-principal-o_outro_app_fechou_o = L'altra app ha chiuso il modello: le schede Qualità, Sicurezza e Manutenzione sono tornate in modifica.
godot-gd-principal-outro_app_gravou_e_ha = Un'altra app ha salvato { $p1 } e ci sono tue modifiche non salvate: usa Salva con nome per non cancellare il suo lavoro.
godot-gd-principal-nao_releu = { $p1 } non riletto: { $p2 }
godot-gd-principal-foi_gravado_por_outro_app = { $p1 } è stato salvato da un'altra app: ricaricato dal disco.
godot-gd-principal-salve_o_modelo_antes_de = Salva il modello prima di aprirlo in { $p1 }.
godot-gd-principal-abrindo_no_ele_edita_o = Apertura di { $p1 } in { $p2 }: lo modifica lui; UTSim 3D rilegge ciò che salva.
godot-gd-principal-salvo_em = Salvato in { $p1 }.
godot-gd-principal-nao_salvou = Non salvato: { $p1 }
godot-gd-principal-desfeito = Annullato.
godot-gd-principal-nada_para_desfazer = Niente da annullare.
godot-gd-principal-refeito = Ripristinato.
godot-gd-principal-nada_para_refazer = Niente da ripristinare.
godot-gd-principal-nao_foi_possivel_desenhar_o = Impossibile disegnare il modello: { $p1 }
godot-gd-principal-escolha_o_arquivo_de_dados = Scegli il file di dati.
godot-gd-principal-valores_colados = Valori incollati: { $p1 }
godot-gd-principal-aguarde = Attendi: { $p1 }.
godot-gd-principal-nao_ajustou = Non adattato: { $p1 }
godot-gd-principal-agora_usa_dados_reais_ctrl = { $p1 } ora usa dati reali (Ctrl+Z annulla).
godot-gd-principal-nao_aplicou_os_dados = Dati non applicati: { $p1 }
godot-gd-principal-volta_a_usar_uma_distribuicao = { $p1 } torna a usare una distribuzione (Ctrl+Z annulla).
godot-gd-principal-nao_voltou_para_a_distribuicao = Ritorno alla distribuzione non riuscito: { $p1 }
godot-gd-principal-sem_relatorio = Nessun rapporto: { $p1 }
godot-gd-principal-relatorio_gravado_em = Rapporto salvato in { $p1 }.
godot-gd-principal-relatorio_gravado_em_arquivos = Rapporto salvato in { $p1 } ({ $p2 } file).
godot-gd-principal-objetos_alinhados = { $p1 } oggetti allineati.
godot-gd-principal-objetos_distribuidos = { $p1 } oggetti distribuiti.
godot-gd-principal-objeto_s_recortados_ctrl_v =
    { $p1 } { $p1 ->
        [one] oggetto tagliato
       *[other] oggetti tagliati
    } (Ctrl+V incolla).
godot-gd-principal-nada_selecionado_para_recortar = Nessuna selezione da tagliare.
godot-gd-principal-objeto_s_colados =
    { $p1 } { $p1 ->
        [one] oggetto incollato
       *[other] oggetti incollati
    }: { $p2 }.
godot-gd-principal-o_modelo_tem_problema_s =
    Il modello ha { $p1 } { $p1 ->
        [one] problema e
       *[other] problemi e
    } non viene eseguito: { $p2 }
godot-gd-principal-a_simulacao_nao_comecou = La simulazione non è iniziata: { $p1 }
godot-gd-principal-a_simulacao = La simulazione { $p1 }: { $p2 }
godot-gd-principal-nao_foi_possivel_gravar_a = Impossibile salvare la replica 0: { $p1 }
godot-gd-principal-replicacoes_concluidas_reproduzindo_a_replicacao = { $p1 } repliche completate. Riproduzione della replica 0.
godot-gd-principal-texto = { $p1 }...
godot-gd-principal-cancelando = Annullamento: { $p1 }...
godot-gd-principal-simule_o_modelo_para_ver = Simula il modello per vedere i risultati (Tab o il pulsante Simula).
godot-gd-principal-os_fluxos_de_tokens_aparecem = I flussi di token compaiono in Simula.
godot-gd-principal-selecione_os_objetos_da_parte = Seleziona gli oggetti della parte del processo (o due: l'inizio e la fine) e richiedi il PERT/CPM.
godot-gd-principal-pert_cpm_de_ate_selecione = PERT/CPM da... a...: seleziona esattamente due oggetti.
godot-gd-principal-o_modelo_tem_problemas_e = Il modello ha problemi e non viene eseguito: il PERT/CPM richiede un modello valido.
godot-gd-principal-nao_comecou = Non avviato: { $p1 }
godot-gd-principal-um_replay_gravado_nao_roda = Un replay registrato non si esegue di nuovo: apri il modello.
godot-gd-principal-nao_ligou_a_serie_e = Serie e istogrammi non collegati: { $p1 }
godot-gd-principal-ligado_no_modelo_ctrl_z = Collegato al modello ({ $p1 }; Ctrl+Z annulla). Nuova simulazione in corso...
godot-gd-principal-aquecimento_aplicado_ao_modelo_run = Riscaldamento { $p1 } applicato al modello (run.warmup). Simula di nuovo per i numeri con esso.
godot-gd-principal-nao_aplicou_o_aquecimento = Riscaldamento non applicato: { $p1 }
godot-gd-principal-nao_abriu_pedido_na_linha = { $p1 } non aperto (richiesto dalla riga di comando).
godot-gd-principal-nao_foi_possivel_reproduzir = Impossibile riprodurre { $p1 }: { $p2 }
godot-gd-principal-replay_aberto_blocos_so_reproducao = Replay { $p1 } aperto ({ $p2 } blocchi): solo riproduzione; Modifica torna al modello in modifica.
godot-gd-principal-simule_um_modelo_antes_de = Simula un modello prima di registrare il replay.
godot-gd-principal-replay_da_replicacao_0_gravado = Replay della replica 0 salvato in { $p1 }.
godot-gd-principal-nao_gravou_o_replay = Replay non salvato: { $p1 }
godot-gd-principal-camera_de_volta = Telecamera ripristinata.
godot-gd-rede-caminho_encerrado_clique_no_chao = Percorso terminato. Clicca a terra per iniziarne un altro, oppure Esc per uscire dallo strumento.
godot-gd-rede-nao_colocou_o_no = Nodo non posizionato: { $p1 }
godot-gd-rede-no_ligado_a_clique_de = Nodo { $p1 } collegato a { $p2 }. Clicca di nuovo per proseguire il percorso (Esc per fermarsi).
godot-gd-rede-no_colocado_clique_em_outro = Nodo { $p1 } posizionato. Clicca in un altro punto per tracciare un tratto (Esc per fermarsi).
godot-gd-rede-no_movido = Nodo { $p1 } spostato.
godot-gd-rede-nao_moveu = Non spostato: { $p1 }
godot-gd-rede-trecho_criado = Tratto { $p1 } → { $p2 } creato ({ $p3 }).
godot-gd-rede-nao_ligou = Non collegato: { $p1 }
godot-gd-rede-trecho_removido = Tratto { $p1 } → { $p2 } rimosso.
godot-gd-rede-no_removido_com_os_trechos = Nodo { $p1 } rimosso con i suoi tratti.
godot-gd-rede-trecho_em = Tratto { $p1 } → { $p2 } in { $p3 }.
godot-gd-rede-pista_controlada_ate_veiculo_s =
    Via controllata: fino a { $p1 } { $p1 ->
        [one] veicolo
       *[other] veicoli
    } alla volta sul tratto.
godot-gd-rede-trecho_sem_controle_de_trafego = Tratto senza controllo del traffico.
godot-gd-rede-trecho_invertido = Tratto invertito: { $p1 } → { $p2 }.
godot-gd-rede-no_agora_se_chama = Il nodo { $p1 } ora si chiama { $p2 }.
godot-gd-rede-nao_renomeou = Non rinominato: { $p1 }
godot-gd-trajeto-clique_no_chao_para_por = { $p1 }: clicca a terra per mettere i nodi (Invio o doppio clic termina). PageUp/PageDown alza e abbassa il nodo successivo, [ e ] arrotondano la curva. Clicca su un nodo per modificarlo; Esc annulla.
godot-gd-trajeto-arraste_em_volta_do_meio = Trascina attorno al centro per ruotare { $p1 } (di 15 in 15°; Shift: di 1 in 1°).
godot-gd-trajeto-arraste_para_mover_inteiro_r = Trascina per spostare { $p1 } per intero; R e Shift+R ruotano di 15°.
godot-gd-trajeto-arraste_para_curvar_o_trecho = Trascina per curvare il tratto { $p1 } di { $p2 }.
godot-gd-trajeto-no_acrescentado_em_o_trecho = Nodo aggiunto a { $p1 } (il tratto è stato diviso).
godot-gd-trajeto-nao_acrescentou = Non aggiunto: { $p1 }
godot-gd-trajeto-aponte_para_o_chao_para = Punta il pavimento per mettere il nodo.
godot-gd-trajeto-no_em_m_altura_m = { $p1 }: nodo { $p2 } in ({ $p3 }; { $p4 }) m, altezza { $p5 } m ({ $p6 }).
godot-gd-trajeto-comeca_em_m_altura_m = { $p1 }: inizia in ({ $p2 }; { $p3 }) m, altezza { $p4 } m. Clicca i nodi successivi; Invio termina.
godot-gd-trajeto-trecho_de_selecionado_apagar_o = Tratto { $p1 } di { $p2 } selezionato: "Elimina il tratto tra i nodi" nel pannello; trascina il rombo per curvarlo.
godot-gd-trajeto-movido_m_em_x_e = { $p1 } spostato di { $p2 } m in X e { $p3 } m in Y (Ctrl+Z annulla).
godot-gd-trajeto-nao_moveu = Non spostato: { $p1 }
godot-gd-trajeto-trecho_curvado_no_com_raio = Tratto curvato: nodo { $p1 } con raggio di { $p2 } m (Ctrl+Z annulla).
godot-gd-trajeto-nao_curvou = Non curvato: { $p1 }
godot-gd-trajeto-comprimento_do_trecho_m_enter = Lunghezza del tratto: { $p1 } m (Invio mette il nodo nella direzione del cursore, Backspace cancella).
godot-gd-trajeto-proximo_no_a_m_de = Nodo successivo a { $p1 } m di altezza.
godot-gd-trajeto-raio_das_proximas_curvas_m = Raggio delle prossime curve: { $p1 } m.
godot-gd-trajeto-descendo_por_m_o_no = Scendendo { $p1 } % per { $p2 } m, il nodo finirebbe sotto il pavimento.
godot-gd-trajeto-o_trecho_de_m_nao = Il tratto di { $p1 } m non raggiunge l'altezza di { $p2 } m.
godot-gd-trajeto-no_a_m_do_anterior = { $p1 }: nodo { $p2 } a { $p3 } m dal precedente.
godot-gd-trajeto-girado_r_gira_de_novo = { $p1 } ruotato di { $p2 }° (R ruota ancora; Shift+R dall'altra parte).
godot-gd-trajeto-nao_girou = Non ruotato: { $p1 }
godot-gd-trajeto-ultimo_no_desfeito = Ultimo nodo annullato.
godot-gd-trajeto-nao_criou = Non creato: { $p1 }
godot-gd-trajeto-criado_m = { $p1 } { $p2 } creato: { $p3 } m{ $p4 }.
godot-gd-trajeto-desenho_cancelado_clique_no_chao = Disegno annullato. Clicca a terra per iniziarne un altro, oppure Esc per uscire dallo strumento.
godot-gd-trajeto-no_s_removido_s =
    { $p1 } { $p1 ->
        [one] nodo rimosso
       *[other] nodi rimossi
    }.
godot-gd-trajeto-nao_removeu = Non rimosso: { $p1 }
godot-gd-trajeto-no_x_y_altura_m = { $p1 }, nodo { $p2 }: X { $p3 }, Y { $p4 }, altezza { $p5 } m. Frecce e PageUp/PageDown regolano (Shift: 1 cm).
godot-gd-trajeto-nao_e_uma_esteira_desenhada = { $p1 } non è un nastro disegnato.
godot-gd-trajeto-esteira_clique_num_no_para = Nastro { $p1 }: clicca su un nodo per modificarlo; frecce e PageUp/PageDown spostano i nodi selezionati.
godot-gd-trajeto-inclinacao_fixa = Inclinazione fissa { $p1 } ({ $p2 } %).
godot-gd-trajeto-esteira_dividida = Nastro diviso: { $p1 } → { $p2 }.
godot-gd-trajeto-trecho_apagado_de = Tratto eliminato da { $p1 }.
godot-gd-trajeto-e_viraram_um_trajeto_so = { $p1 } e { $p2 } sono diventati un unico percorso (Ctrl+Z annulla).
godot-gd-trajeto-agora_corre_no_sentido_contrario = { $p1 } ora scorre nel senso opposto (anche i collegamenti).
godot-gd-trajeto-desenhado_a_m_de = { $p1 } disegnato a { $p2 } m da { $p3 }.
godot-gd-trajeto-rampa_s_de_acesso_de =
    { $p1 } { $p1 ->
        [one] rampa di accesso 1:12 aggiunta
       *[other] rampe di accesso 1:12 aggiunte
    } a { $p2 }.
godot-gd-trajeto-nao_mudou = Non modificato: { $p1 }
godot-gd-trajeto-no_a_m_de_altura = { $p1 }, nodo { $p2 } a { $p3 } m di altezza (profilo; Ctrl+Z annulla).
godot-gd-trajeto-nao_mudou_a_altura = Altezza non modificata: { $p1 }

godot-gd-trajeto-clique_no_chao_para_por-2 = Clicca a terra per mettere i nodi; il tratto mostra la lunghezza e l'inclinazione. I nodi si agganciano alle porte (estremità dei nastri e oggetti), ai nodi di altri percorsi, di 15 in 15° e alla griglia. PageUp/PageDown: altezza del nodo successivo; [ e ]: raggio della curva; Invio termina; Backspace torna indietro di un nodo; Esc annulla.
godot-gd-trajeto-altura_do_proximo_no_m = Altezza del nodo successivo (m)
godot-gd-trajeto-inclinacao_fixa_no_proximo_trecho = Inclinazione fissa nel tratto successivo
godot-gd-trajeto-raio_das_curvas_m_0 = Raggio delle curve (m; 0 = spigolo)
godot-gd-trajeto-largura_do_caminho_m = Larghezza del percorso (m)
godot-gd-trajeto-esteira_s_desenhada_s_caminho =
    { $p1 } { $p1 ->
        [one] nastro disegnato
       *[other] nastri disegnati
    }, { $p2 } { $p2 ->
        [one] percorso
       *[other] percorsi
    }.
godot-gd-trajeto-maior_inclinacao_maximo = Inclinazione massima { $p1 }° (limite { $p2 }°).
godot-gd-trajeto-girar_graus_anti_horario = Ruota (gradi; + antiorario)
godot-gd-trajeto-girar_o_trajeto = Ruota il percorso
godot-gd-trajeto-girar_15_anti_horario_r = Ruota di 15° (antiorario, R)
godot-gd-trajeto-girar_15_horario_shift_r = Ruota di 15° (orario, Shift+R)
godot-gd-trajeto-juntar_com_num_trajeto_so = Unisci con { $p1 } in un unico percorso
godot-gd-trajeto-paralelo_a_m_a_esquerda = Parallelo a (m; + a sinistra)
godot-gd-trajeto-desenhar_paralelo = Disegna parallelo
godot-gd-trajeto-escada_degraus_so_pedestres = Scala (gradini; solo pedoni)
godot-gd-trajeto-rampas_de_acesso_nas_pontas = Rampe di accesso alle estremità (1:12)
godot-gd-trajeto-tipo = Tipo
godot-gd-trajeto-acionamento = Azionamento
godot-gd-trajeto-inverter_o_sentido = Inverti il senso
godot-gd-trajeto-largura_m = Larghezza (m)
godot-gd-trajeto-altura_m = Altezza (m)
godot-gd-trajeto-raio_da_curva_m_0 = Raggio della curva (m; 0 = spigolo)
godot-gd-trajeto-dividir_a_esteira_neste_no = Dividi il nastro in questo nodo
godot-gd-trajeto-nos_selecionados = { $p1 } nodi selezionati
godot-gd-trajeto-nivelar_na_altura_m = Livella all'altezza (m)
godot-gd-trajeto-inclinacao_constante_entre_as_pontas = Inclinazione costante tra le estremità
godot-gd-trajeto-remover_no_s_del = Rimuovi nodo/i (Del)
godot-gd-trajeto-apagar_o_trecho_entre_os = Elimina il tratto tra i nodi
godot-gd-trajeto-arraste_o_losango_do_no = Trascina il rombo del nodo a terra per spostarlo e la cima dell'asta per cambiare l'altezza. Le frecce spostano (Shift: 1 cm), PageUp/PageDown alzano e abbassano, [ e ] cambiano il raggio. Shift + clic sulla linea aggiunge un nodo; un clic sul rombo arancione (o due nodi vicini con Shift + clic) seleziona il tratto, che può essere eliminato. Trascina il rombo arancione a metà di un tratto per curvarlo, il quadrato verde a metà del percorso per spostarlo per intero e l'anello arancione accanto per ruotarlo; senza nodo selezionato, le frecce spostano e R ruota il percorso.
godot-gd-principal-o_modelo_tem_mudancas_que = Il modello ha modifiche non salvate. Scartarle?

## Rodada 14: finestre e errori restanti

godot-ui-class-sem_replicacao = senza replica
godot-ui-class-sem_rastro_da_replicacao = senza traccia della replica
godot-ui-class-a_janela_de_valores_nao = la finestra dei valori non è aperta
godot-ui-class-valor_es_lidos_de_coluna =
    { $n } { $n ->
        [one] valore letto
       *[other] valori letti
    } da { $file } (colonna { $column }).
godot-ui-class-nao_leu_o_arquivo = File non letto: { $e }
godot-ui-class-relatorio_gravado_em = Rapporto salvato in { $f }.
godot-ui-class-escreva_o_arquivo_do_relatorio = Scrivi il file del rapporto.
godot-ui-class-relatorio_pert_cpm_gravado_em = Rapporto PERT/CPM salvato in { $path }.
godot-ui-class-nao_gravou = { $path } non salvato: { $e }
godot-ui-class-tabela_s_csv_em =
    { $files } { $files ->
        [one] tabella
       *[other] tabelle
    } CSV in { $dir }.
godot-ui-class-nao_gravou_os_csv_em = CSV non salvati in { $dir }: { $e }
godot-ui-class-escreva_a_pasta_da_exportacao = Scrivi la cartella dell'esportazione.
godot-ui-class-arquivo_s_csv_em =
    { $files } { $files ->
        [one] file
       *[other] file
    } CSV in { $dir }.
godot-ui-class-nao_exportou = Non esportato: { $e }
godot-ui-class-so_leitura = { $quem }: sola lettura
godot-ui-class-na_ultima_simulacao_media_semiamplitude =
    Nell'ultima simulazione ({ $replicacoes } { $replicacoes ->
        [one] replica
       *[other] repliche
    }), media ± semiampiezza dell'IC al 95 %:
godot-schema-inteiro_maior_ou_igual_a_zero = { $key } deve essere un intero >= 0
godot-schema-numero_maior_ou_igual_a_zero = { $w }: numero maggiore o uguale a zero
godot-schema-nao_tem_o_campo = { $name } non ha il campo "{ $field }"
godot-schema-campo_maior_que_zero = { $field }: deve essere maggiore di zero
godot-schema-inteiro_minimo = { $what } deve essere un intero >= { $min }
godot-schema-numero_esperado_de = { $what }: numero atteso
godot-schema-verdadeiro_ou_falso_esperado = { $what }: atteso vero o falso

## aba Ambiental

aba-abrir-ambiental = Apri in UTSim Ambiental
aba-abrir-ambiental-dica = Apre questo nodo in UTSim Ambiental (mappa di calore, flussi, impronta, scenari e obiettivi). Il modello viene salvato prima e resta all'altra app per modificarlo; UTSim 3D rilegge ciò che scrive.
godot-indicadores-energia = Energia
godot-indicadores-energia_ajuda = Potenza di ogni gruppo di stati × ore nello stato, nella finestra statistica (media delle repliche).
godot-indicadores-energia_sem_valor = Energia senza valore
godot-indicadores-energia_sem_valor_ajuda = Quota di energia spesa in attesa, ferma o fuori turno.
godot-indicadores-kwh_por_peca = Energia per pezzo
godot-indicadores-kwh_por_peca_ajuda = Energia del nodo divisa per i pezzi usciti da esso.

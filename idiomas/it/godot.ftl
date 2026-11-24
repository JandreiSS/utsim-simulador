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

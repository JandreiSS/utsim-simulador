# Gêmeo digital (sombra e previsão)

O gêmeo digital liga um modelo `.ron` aos dados da fábrica: lê sensores e CLPs por OPC UA,
MQTT ou Sparkplug B, monta o **estado observado** da planta, calcula os KPIs reais em
janelas de tempo e compara cada um com o que o modelo diz que deveria acontecer. Quando o
real sai do esperado, o gêmeo dá um **alerta de desvio** apontando o objeto e o indicador.

O gêmeo tem dois modos nesta versão. O **modo sombra** só observa: nada do real altera a
simulação, e nada é escrito no equipamento. A **previsão** (com `--previsao`) parte do
estado observado e roda replicações à frente para dizer o que deve acontecer nas próximas
horas: a produção até o fim do turno, a chance de cumprir a meta, quando uma fila deve
passar de um limite, e o que muda com mais um operador ou outra regra (seção
[Previsão](#previsão)). O modo sincronizado (o modelo corrigido pelo real) vem depois.

## Experimente com a fábrica sintética

A pasta `examples/gemeo/` traz uma linha (`linha.ron`, máquina com ciclo médio 0,6) e uma
"planta" com a máquina mais lenta (`linha_real.ron`, ciclo médio 0,85). A emulação OPC UA
faz o papel da fábrica:

```bash
# terminal 1: a "fábrica", servidor OPC UA em 127.0.0.1:4840, 50 vezes mais rápida
utsim emular examples/gemeo/linha_real.ron --velocidade 50

# terminal 2: o gêmeo da linha, gravando, com o painel em http://127.0.0.1:8080/gemeo
utsim gemeo examples/gemeo/linha.ron --gravar gravacoes --painel 8080 --ate 4000
```

Em poucas janelas aparecem alertas como

```text
[desvio] utilização de maquina acima do modelo na janela 2 (real 0.912, modelo 0.598 ± 0.111; CUSUM, p = 0.001, z = +2.83)
[desvio] conteúdo médio (WIP) de fila acima do modelo na janela 4 (real 4.229, modelo 0.888 ± 0.446; CUSUM, p = 0.001, z = +7.49)
```

e a sessão pode ser reexecutada depois, sem a fábrica e sem rede:

```bash
utsim gemeo reproduzir gravacoes/ --json relatorio.json
```

## O arquivo `.gemeo.ron`

O mapeamento fica num arquivo à parte, ao lado do modelo (padrão: `<modelo>.gemeo.ron`;
outro com `--config`). Endereços e tópicos mudam de uma instalação para outra; o modelo
continua o mesmo e roda sem a fábrica.

```ron
Twin(
    model: "linha.ron",
    clock: Wall(unit_seconds: 1.0),
    sources: [
        (name: "clp", kind: OpcUa(endpoint: "opc.tcp://10.0.0.5:4840/",
                                  namespace: "urn:fabrica:linha1", security: SignAndEncrypt)),
        (name: "uns", kind: Mqtt(host: "broker.local", topics: ["fabrica/linha1/#"],
                                 credentials_env: "UTSIM_MQTT")),
        (name: "spb", kind: Sparkplug(host: "broker.local", group: "Linha1")),
    ],
    tags: [
        (source: "clp", tag: "ns=2;s=M1.Pecas", object: "maquina", measure: Outputs,
         rollover: 65536),
        (source: "clp", tag: "ns=2;s=M1.Estado", object: "maquina", measure: State,
         states: {"0": Stopped, "1": Running, "3": Down}, debounce: 5.0),
        (source: "uns", tag: "fabrica/linha1/buffer", object: "fila", measure: Content,
         field: "/qtd", deadband: 0.5, max: 500.0),
        (source: "spb", tag: "Linha1/borda1/Empilhadeira/pos", object: "empilhadeira",
         measure: Position, unit: "mm"),
        (source: "uns", tag: "fabrica/rtls/cracha17", object: "operador",
         measure: Position, person: true),
    ],
    shadow: (window: 3600.0, replications: 20),
    record: (anonymize_people: true, keep_days: 30, people: ["rtls", "cracha"]),
)
```

Os campos têm nomes em inglês e aceitam os nomes em português (`modelo`, `relogio`,
`conexoes`, `mapeamento`, `sombra`, `gravacao`, `objeto`, `medida`, `escala`,
`deslocamento`, `unidade`, `banda_morta`, `atraso_estado`, `estados`, `campo`, `pessoa`,
`janela`, `replicacoes`, `retencao_dias`, `anonimizar_pessoas`...).

### Relógio (`clock`)

| Valor | Tempo do gêmeo |
|---|---|
| `Wall(unit_seconds: 1.0)` (padrão) | relógio local; uma unidade de tempo do modelo dura `unit_seconds` segundos. Quando a fonte manda carimbo (OPC UA SourceTimestamp, `timestamp` do Sparkplug) e ele está a menos de 2 s da chegada, vale o carimbo; senão, a hora de chegada, com um aviso |
| `Tag(source: "clp", tag: "...")` | o último valor lido de um tag (um contador de tempo do CLP, o `TempoSimulado` da emulação) |

O instante zero é o início do gêmeo. As leituras esperam 100 ms depois de chegar e são
processadas na ordem do carimbo, para que os valores de uma mesma notificação fiquem na
ordem certa. O tempo do gêmeo nunca volta.

### Conexões (`sources`)

Todas **só leem**.

| Tipo | O que faz |
|---|---|
| `OpcUa(endpoint, namespace, interval_ms: 100, security: None, trust_server: false)` | cliente OPC UA: uma assinatura com um item monitorado por nó mapeado. O tag é um node id completo (`ns=2;s=M1.Estado`, `i=2258`) ou um identificador de texto no `namespace` (URI). `security: SignAndEncrypt` usa Basic256Sha256 assinado e cifrado; o certificado do servidor precisa estar na pasta de confiança, a menos que `trust_server: true`. Reconecta com espera crescente. Login anônimo nesta versão |
| `Mqtt(host, port: 1883, topics, client_id, credentials_env)` | cliente MQTT 3.1.1, QoS 0, só assina. O tag é o tópico; a carga é um número, `true`/`false`, um texto ou um JSON (veja `field`). `credentials_env` é o nome de uma variável de ambiente com `usuario:senha` |
| `Sparkplug(host, port: 1883, group, client_id, credentials_env)` | Sparkplug B sobre MQTT, um grupo: NBIRTH/DBIRTH dão os nomes e apelidos (aliases) das métricas, NDATA/DDATA os valores, NDEATH/DDEATH marcam as métricas do nó ou do dispositivo como qualidade ruim. Lacunas no `seq` são contadas. O tag de uma métrica é `grupo/no/metrica` ou `grupo/no/dispositivo/metrica`. Comandos (NCMD, DCMD, pedido de renascimento) nunca são enviados |

### Mapeamento (`tags`)

Cada regra liga um tag de uma conexão a uma **medida** de um objeto do modelo:

| `measure` | Valor esperado | No gêmeo |
|---|---|---|
| `Inputs` (`Entradas`) | contador de peças que entraram | chegadas por janela |
| `Outputs` (`Saidas`) | contador de peças que saíram | vazão por janela |
| `State` (`Estado`) | código de estado | utilização e tempo quebrado por janela |
| `Content` (`Conteudo`) | quantidade no objeto | conteúdo médio (WIP) por janela |
| `Position` (`Posicao`) | posição `[x, y, z]`, `{"x":…, "y":…}` ou um eixo por tag (`axis: X`) | só o estado observado |

Conversão: o valor usado é `bruto × scale × unidade + offset`. `unit` aceita `m`, `cm`,
`mm` e `km` para posições (convertidas para metros) e `un` para contagens.

Contadores: `counter: Total` (padrão) é um total acumulado; a primeira leitura é a base, e
cada aumento conta. `rollover: 65536` trata a volta do contador; sem ele, uma queda
é um reinício (conta o valor novo). `counter: Pulse` conta cada leitura como incremento (um
pulso `true` vale 1).

Estados: cada leitura vira uma de três classes: **rodando** (`Running`), **parado**
(`Stopped`) e **quebrado** (`Down`). Os nomes de estado do UTSim já são reconhecidos
(`processing`, `setup`, `idle`, `blocked`, `down`, `stopped`, ...) e também `rodando`,
`parado`, `quebrado`; `states` acrescenta os códigos do equipamento (`{"3": Down}`).

Filtros:

| Campo | Efeito |
|---|---|
| `deadband: d` | conteúdo ou posição que mudou menos que `d` é ignorado |
| `min`, `max` | valores fora da faixa são descartados e contados |
| `debounce: t` | um estado novo só vale depois de durar `t` (unidades do modelo); trocas mais curtas são ignoradas |
| `field: "/fila/qtd"` | ponteiro JSON do valor dentro de uma carga JSON; sem ele, um objeto JSON dá o seu membro `valor` ou `value` |

Leituras com qualidade ruim na fonte (OPC UA `Bad`, morte no Sparkplug), não numéricas ou
com estado desconhecido são descartadas e contadas por regra (o painel mostra).

**Validação contra o modelo.** Ao iniciar, o gêmeo recusa: objeto que não existe, conexão
que não existe, o mesmo tag em duas regras, dois contadores da mesma medida de um objeto
(contagem dupla), saídas de um `Sink`, entradas de uma `Source`, unidade que não se aplica,
filtros na medida errada, janela maior que a rodada do modelo. E avisa: objeto sem nenhum
tag, posição de algo que não é recurso móvel, OPC UA sem segurança, MQTT sem TLS e modelo
alterado depois do mapeamento (campo opcional `model_hash`, o `fnv64:...` que o
`utsim gemeo` grava na gravação).

## KPIs por janela

O tempo é cortado em janelas de `shadow.window` unidades a partir de `shadow.warmup`
(tempo real ignorado no início). Em cada janela fechada:

| KPI | Cálculo |
|---|---|
| `objeto.vazao` | peças que saíram na janela |
| `objeto.chegadas` | peças que entraram na janela |
| `objeto.wip` | conteúdo médio ponderado pelo tempo |
| `objeto.utilizacao` | fração da janela rodando |
| `objeto.quebra` | fração da janela quebrado |

Um KPI só entra numa janela quando a medida era conhecida na janela inteira (um total
precisa da primeira leitura antes do início da janela).

## Comparação com o modelo

Antes de começar, o gêmeo roda `shadow.replications` replicações do modelo (padrão 20),
lendo o modelo como um sensor leria (`samples_per_window` amostras por janela, padrão 60),
e corta as mesmas janelas a partir do aquecimento do modelo (`sim_warmup` ou
`run.warmup`). Cada janela de cada replicação é uma amostra de "como o KPI fica numa janela,
segundo o modelo". Para cada janela real e cada KPI:

1. **Teste da janela.** O valor real `y` é posicionado entre as amostras simuladas:
   `p = 2·min(#{x ≥ y} + 1, #{x ≤ y} + 1)/(n + 1)`, sem supor normalidade. Os p-valores da
   janela passam por **Benjamini-Hochberg** com taxa de falsas descobertas `fdr` (padrão
   10 %): uma rejeição é um **aviso**.
2. **Desvios pequenos e persistentes.** O resíduo padronizado real − simulado
   `z = (y − média)/desvio` alimenta uma **EWMA** (`ewma_lambda`, padrão 0,2) e um **CUSUM**
   tabular (`cusum_k`, padrão 0,5) do módulo de qualidade. Um sinal é um **desvio**.
3. **Correção pelo número de KPIs.** Com `m` KPIs observados ao mesmo tempo, o limite `h`
   do CUSUM é escolhido para que cada KPI tenha ARL sob controle (aproximação de Siegmund)
   de `m × arl0`, e a largura da EWMA é `L = Φ⁻¹(1 − 0,0027/(2m))`. O conjunto todo fica
   com cerca de um alarme falso a cada `arl0` janelas (padrão 370).
4. **Histerese.** Um KPI em desvio só volta quando as somas do CUSUM caem abaixo de `h/2`.
   Um alerta é registrado quando o nível de um KPI sobe (ok → aviso → desvio).

O relatório mostra o número de testes feitos e os limites em uso. Com poucas janelas os
testes têm pouco poder: "sem alerta" não prova que o modelo está certo.

As janelas de um sistema com fila são correlacionadas entre si, o que aumenta os alarmes
falsos do CUSUM e da EWMA. Janelas longas (uma hora, um turno) reduzem o efeito.

## Estado observado

O gêmeo guarda, por objeto, o conteúdo, o estado e desde quando, os totais de entradas e
saídas e a posição. Esse estado sai como um arquivo de [estado inicial](estado-inicial.md):
conteúdos como `count`, quebras em curso como `down` (tempo decorrido) e posições de
transportadores; o que o modelo recusa fica de fora, com uma nota. Use
`--estado arquivo.ron` para gravá-lo no fim da sessão e `utsim run modelo.ron --estado
arquivo.ron` para simular a partir da planta como ela estava.

## Painel

Com `--painel PORTA`, o programa serve o [painel](painel.md) do modelo em
`http://127.0.0.1:PORTA/` e a página do gêmeo em `/gemeo`: alertas, KPIs real × modelo com
os resíduos de cada janela, estado observado, conexões (conectada, recebidas, descartadas,
inválidas, lacunas, último erro) e tags (último valor, qualidade, descartes). Só este
computador acessa. As rotas da API:

| Rota | Conteúdo |
|---|---|
| `/api/gemeo` | relatório completo em JSON |
| `/api/gemeo/estado` | o mesmo, com o estado observado como estado inicial |
| `/api/gemeo/estado.ron` | o estado observado em `.ron` |
| `/gemeo/previsao` | a página da previsão (com `--previsao`) |
| `/api/gemeo/previsao` | situação da previsão (ligada, próxima, na fila, erros) e a última previsão |
| `/api/gemeo/previsoes` | todas as previsões guardadas (até 200) |
| `POST /api/gemeo/previsao` | pede uma previsão agora (exige o cabeçalho `X-UTSim`, que o botão da página envia) |

## Gravação e reprodução

Com `--gravar pasta`, cada sessão cria `pasta/sessao-AAAAMMDD-HHMMSS/` com
`sessao.json` (o texto do `.gemeo.ron`, o caminho e o hash do modelo) e um arquivo de linhas
JSON por dia, `observacoes-AAAAMMDD.jsonl`, com **todas** as leituras recebidas:

```json
{"tipo":"obs","t":812.4,"ts":1793691460123,"fonte":"clp","seq":881,"tag":"UTSim/linha/fila/Conteudo","valor":4.0,"qualidade":"Good","regra":2}
{"tipo":"fim","t":2400.0}
```

`t` é o tempo do gêmeo em que a leitura foi aplicada, `ts` a hora (só informativa) e
`regra` a regra do mapeamento que a recebeu. `utsim gemeo reproduzir pasta` lê a sessão
(ou a mais recente da pasta) e passa as mesmas leituras, na mesma ordem e nos mesmos
instantes, pelo mesmo gêmeo: o relatório sai **idêntico** ao da sessão, sem rede e sem
relógio. `--modelo` e `--config` trocam o modelo ou o arquivo do gêmeo gravados (com aviso
quando diferem).

**Pessoas.** Localização e identificação de pessoas são dados pessoais (LGPD). Com
`anonymize_people: true` (padrão), os tags das regras com `person: true` e os tags que
contêm um dos textos de `record.people` viram apelidos estáveis (`pessoa-...`), e os textos
dentro dos valores também; os números (as coordenadas) ficam. Os apelidos vêm de um hash
com chave sorteada a cada gravação e nunca escrita: não dá para voltar ao nome a partir dos
arquivos, e o mesmo crachá tem apelidos diferentes em gravações diferentes. O gêmeo
processa a leitura já anonimizada, ao vivo e na reprodução.

**Retenção.** Os arquivos de dia mais antigos que `keep_days` (padrão 30) são apagados
quando uma gravação começa e uma vez por dia enquanto ela roda; uma sessão sem nenhum dia
restante é removida.

## Segurança

- **Só leitura.** Nenhuma conexão escreve: o cliente OPC UA só cria a assinatura, os
  clientes MQTT só assinam. Escrever no equipamento não existe nesta versão.
- **Credenciais** ficam fora dos arquivos: variável de ambiente (`credentials_env`) e nunca
  vão para a gravação.
- **Transporte.** O OPC UA aceita assinatura e cifra (`SignAndEncrypt`). O MQTT desta
  versão não tem TLS: use só em rede de confiança (o gêmeo avisa).
- **Painel** só em `127.0.0.1`, somente leitura, recusando outros nomes de host.
- **Recursos.** As leituras passam por um canal limitado (descarta e conta se encher);
  cargas MQTT acima de 256 KiB são descartadas; um conector com problema reconecta sem
  derrubar os outros.

## Previsão

Com `--previsao`, o gêmeo tira um **instantâneo** do estado observado a cada intervalo (e
quando alguém clica em **Prever agora** na página `/gemeo/previsao`), monta com ele um
[estado inicial](estado-inicial.md) e roda `R` replicações do modelo até o **horizonte**,
cada uma com sua semente. A previsão roda num fio à parte: o gêmeo continua lendo a planta
enquanto ela calcula.

```bash
utsim gemeo examples/gemeo/linha.ron --previsao --painel 8080 --gravar gravacoes
# Painel do gêmeo em http://127.0.0.1:8080/gemeo ; previsão em http://127.0.0.1:8080/gemeo/previsao
```

A configuração fica na seção `forecast` (ou `previsao`) do `.gemeo.ron`; sem ela valem os
padrões. O mesmo conteúdo, num arquivo próprio `Forecast(...)`, serve ao `utsim prever
--config`:

```ron
Forecast(
    every: 15.0,                          // a cada 15 unidades do gêmeo (aqui, 15 min); 0 = só sob pedido
    horizon: EndOfShift(ends: [480.0, 960.0, 1440.0], period: 1440.0),  // turnos que acabam às 8 h, 16 h e 24 h; ou Hours(4.0), Time(240.0)
    replications: 40,
    time_unit: Minutes,                   // padrão: a do <modelo>.painel.ron, senão segundos
    start_clock: 360.0,                   // minuto do dia no início do gêmeo (aqui, 06h00)
    targets: {"producao": AtLeast(450.0), "lead_time": AtMost(30.0)},
    limits: [(object: "fila", above: 20.0)],
    watch: ["fila"],
    scenarios: [
        (name: "mais um operador", set: {"equipe.team": "[\"op1\", \"op2\", \"op3\"]"}),
        (name: "operador mais próximo", set: {"equipe.rule": "Nearest"}),
    ],
    thresholds: (green: 0.9, yellow: 0.5),
    points: 48,
)
```

| Campo | Padrão | Significado |
|---|---|---|
| `every` (`a_cada`) | 15 min na unidade do modelo | intervalo entre previsões, em tempo do gêmeo; `0` só sob pedido |
| `horizon` (`horizonte`) | `Hours(4.0)` | `Time(t)` em unidades do modelo, `Hours(h)` ou `EndOfShift(ends, period)`: até o próximo fim de turno a partir do relógio do estado |
| `replications` (`replicacoes`) | 40 | replicações por previsão (e por cenário) |
| `seed` (`semente`) | a do modelo | semente base |
| `time_unit` (`unidade_tempo`) | a do painel, senão `Seconds` | o que vale uma unidade de tempo do modelo (`Seconds`, `Minutes`, `Hours`, `Unit`) |
| `start_clock` (`relogio_inicio`) | 0 | instante do calendário no tempo 0 do gêmeo (turnos e `EndOfShift`) |
| `targets` (`metas`) | as do painel | metas por indicador: `AtLeast(v)` ou `AtMost(v)` |
| `limits` (`limites`) | nenhum | `(object, above: v)` ou `(object, below: v)`: quando o conteúdo deve passar do limite |
| `watch` (`acompanhar`) | nenhum | objetos cujo conteúdo aparece no tempo e no horizonte |
| `compare` (`comparar`) | metas, produção, vazão, WIP, lead time | indicadores comparados nos cenários |
| `scenarios` (`cenarios`) | nenhum | variações "e se", com as mesmas chaves `objeto.campo` de um [experimento](experimentos-e-otimizacao.md) |
| `level` (`nivel`), `thresholds` (`limiares`) | 95 %; 90 % e 50 % | nível dos intervalos; limites do semáforo |
| `points` (`pontos`) | 48 | instantes amostrados no horizonte (para a faixa no tempo e os limites) |

O arquivo é conferido ao iniciar: objeto inexistente em `limits` ou `watch`, limite sem
`above`/`below`, cenário que não se aplica ao modelo, menos de 2 replicações, horizonte em
horas com unidade abstrata.

### O que a previsão mostra

| Indicador | Cálculo, por replicação |
|---|---|
| `producao` | itens que chegam às `Sink` entre o instante da previsão e o horizonte |
| `vazao`, `lead_time`, `lead_time_p90`, `wip`, `utilizacao_gargalo`, ... | os mesmos do [painel](painel.md), no período da previsão |
| `conteudo_final:<obj>` | conteúdo do objeto acompanhado no horizonte |

Para cada indicador: a **média** com a **precisão da média** (intervalo de confiança de 95 %),
a **faixa do que pode acontecer** (P5 a P95 entre as replicações) e, quando há meta, a
**probabilidade de cumprir** (fração das replicações que cumprem, com o intervalo de Wilson)
e o **semáforo** por essa probabilidade: verde a partir de 90 %, amarelo de 50 % a 90 %,
vermelho abaixo.

> **Precisão da média ou faixa do que pode acontecer?** São perguntas diferentes. A
> precisão da média responde "quanto produzo, em média, a partir deste estado?" e encolhe
> quando se rodam mais replicações. A faixa responde "quanto vou produzir neste turno?": em
> 9 de cada 10 turnos que começassem assim, o resultado fica nela. Ela não encolhe com mais
> replicações, porque mede a variação da própria fábrica. Uma meta pode estar fora da
> precisão da média e ainda assim dentro da faixa: a fábrica a cumpre às vezes.

No tempo, a página desenha o **leque** da produção acumulada, do WIP e do conteúdo dos
objetos acompanhados: P5 a P95 em tom claro, P25 a P75 em tom mais forte, a mediana, a
média com a precisão e a linha da meta.

Para cada limite, a previsão dá a probabilidade de ele ser passado antes do horizonte (com
Wilson) e, entre as replicações em que isso acontece, o instante P5, P50 e P95 ("a fila
deve passar de 20 por volta de t = 1 230, entre 1 150 e 1 340"). O instante é procurado
nos `points` instantes amostrados.

### "E se"

Cada cenário roda a partir do **mesmo estado** e com as **mesmas sementes** do "como está"
(números aleatórios comuns): a replicação 3 do cenário vê as mesmas chegadas e os mesmos
sorteios que a replicação 3 do "como está". A diferença é calculada replicação a
replicação, com o seu intervalo de confiança; quando o intervalo não contém zero, a
diferença é **significativa** e a tabela diz se **melhora** ou **piora** (pela meta; sem
meta, mais produção e vazão é melhor, mais WIP e lead time é pior). `≈` quer dizer que a
diferença não se distingue do acaso. Um cenário que não cabe no estado (por exemplo,
capacidade menor que o conteúdo observado) aparece com o erro, sem derrubar a previsão.

### Sementes e reprodução

A semente de uma previsão mistura a semente base com o instante do instantâneo: duas
previsões em instantes diferentes são independentes, e a mesma previsão sai sempre igual.
Com `--gravar`, cada instantâneo vira uma linha `{"tipo":"previsao","t":…,"pedido":…}` na
gravação, no ponto exato da sequência de leituras. `utsim gemeo reproduzir pasta
--previsao` tira os instantâneos nessas linhas (não pelo relógio) e refaz as previsões,
idênticas às da sessão; sem linhas gravadas, faz uma previsão no fim. Com `--json`, o
relatório traz as previsões em `previsoes`.

### Sem gêmeo: `utsim prever`

A mesma previsão a partir de um arquivo de estado (ou do sistema vazio), útil para
"simular a partir de segunda às 6 h" sem a fábrica ligada:

```bash
utsim prever examples/turnos.ron --estado examples/estados/turnos_21h30.ron \
    --horizonte 2h --unidade min -n 40 --meta "producao>=10" \
    --cenarios examples/estados/turnos_cenarios.ron --json previsao.json
```

| Opção | Efeito |
|---|---|
| `--estado arquivo.ron` | estado de partida (padrão: o sistema vazio) |
| `--horizonte` | `4h`, `30min`, `90s` ou um número em unidades do modelo |
| `-n` | replicações (padrão 40) |
| `--unidade` | `s`, `min`, `h` ou `u` (padrão: a do `<modelo>.painel.ron`, senão segundos) |
| `--meta` | `KPI>=valor` ou `KPI<=valor`, repetível |
| `--cenarios` | lista RON `[(name: "...", set: {...})]` ou um arquivo de [experimento](experimentos-e-otimizacao.md) |
| `--config` | arquivo `Forecast(...)` com todos os campos acima |
| `--seed`, `--json` | semente; previsão completa em JSON |

Aqui a semente é a do modelo, sem mistura: a previsão repete, replicação a replicação, o
`utsim run modelo.ron --estado estado.ron --stop H`.

## Limitações desta entrega

- Sem o modo sincronizado nem calibração automática; a previsão parte do estado observado
  (contagens, quebras em curso, posições), sem a idade de cada item nem a fase de cada
  estação, que entram como no [estado inicial](estado-inicial.md) por contagem.
- O instante em que um limite é passado é procurado só nos instantes amostrados
  (`points`); a avaliação das previsões passadas contra o real (cobertura, PIT) ainda não
  existe.
- Sem leitura de histórico (OPC UA HistoryRead) nem conector de arquivo, SQL ou Modbus.
- Sem TLS no MQTT e sem login por usuário no OPC UA.
- Contra a emulação muito acelerada, o intervalo mínimo de publicação do servidor OPC UA
  limita a amostragem (estados que mudam mais rápido que isso se perdem); use velocidades
  moderadas para validar o mapeamento.

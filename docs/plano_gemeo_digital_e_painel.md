# Plano: gêmeo digital, painel para gestores e engenheiros e estatística aplicada

Plano técnico, sem implementação, de três pedidos do dono: (1) um ambiente que receba dados de
sensores e funcione como gêmeo digital; (2) um painel completo para gestores e engenheiros;
(3) estatística como foco transversal. Cada item traz como funciona, mudanças no núcleo e no
formato, interface, testes, riscos, estimativa em horas Opus 5.5 e o modelo de IA sugerido. Os
requisitos sugeridos começam em RF-209 e ficam só aqui até o dono aprovar; não estão em
`docs/requisitos.md`.

Documentos relacionados, que este plano não repete:

- `docs/plano_analises_e_dinamica.md`: estatística P1–P3 (RF-157 a RF-166: quantil t geral,
  aquecimento, replicações necessárias, médias em lotes, Little, gargalo móvel, Gantt, WIP com
  banda, ANOVA/Tukey, correlação, mapa de calor, planejamento de experimentos, metamodelo), PERT,
  OEE/MTBF/MTTR observados (RF-208) e a crate `utsim-stats` proposta lá;
- `docs/backlog_v02.md`: levas e critério de escolha do modelo;
- `docs/plano_godot.md`: fases G1–G5;
- `docs/arquitetura.md`, seções 4k (passo a passo e comandos externos), 4o (emulação OPC UA),
  4p (cena ao vivo `utsim-scene/1`, `SceneServer`, `LiveScene`), 4q (resultados) e 4j
  (`utsim-web`).

---

## 0. Ponto de partida e princípios

### O que já existe e será reaproveitado

| Peça | Onde | Uso neste plano |
|---|---|---|
| Ritmo pelo relógio (`Pacer`, `Clock`, `FakeClock`) | `crates/utsim-emu/src/pacing.rs` | modos sombra e sincronizado em tempo real ou acelerado; testes sem relógio real |
| Um só fio de simulação, entradas aplicadas entre dois `run_until` | `crates/utsim-emu/src/runner.rs` (`Emulator`, `Frontend`, `Input`) | as observações dos sensores entram pelo mesmo caminho |
| Log de comandos e `replay` sem rede | `crates/utsim-emu/src/commands.rs` | modelo da gravação de observações |
| Servidor OPC UA (o UTSim expõe variáveis) | `crates/utsim-emu/src/server.rs` | "fábrica sintética" nos testes do gêmeo; o gêmeo precisa do lado cliente, que é novo |
| `stop_block`, `resume_block`, `block_status` | `crates/utsim-core/src/engine.rs` | parada real vira parada no modelo |
| Cena ao vivo por TCP e WebSocket na mesma porta | `crates/utsim-scene/src/server.rs`, `live.rs` | o 3D do gêmeo; o painel ao vivo segue o mesmo padrão de fios e contrapressão |
| Estatísticas por objeto, gargalo, séries, histogramas, diferença pareada | `crates/utsim-model/src/analysis.rs` | base do cálculo dos KPIs |
| Relatório HTML autocontido com SVG em linha | `crates/utsim-model/src/html.rs` | exportação HTML do painel e base dos gráficos |
| Ajuste de distribuições, diagnósticos, funções especiais | `crates/utsim-fit` (`report::analyze`, `diagnostics`, `special`) | calibração a partir dos dados recebidos |
| Simulação no navegador (wasm) | `crates/utsim-web` | o mesmo painel roda sem servidor sobre resultados calculados no navegador |

### Princípios

- **Cálculo no Rust, apresentação fina.** Mesma regra de `plano_analises_e_dinamica.md`, seção 0:
  toda regra testável mora numa crate pura. O painel web e o Godot só desenham números que já
  chegam prontos (JSON).
- **Determinismo preservado.** Dados externos são entradas como os comandos da emulação:
  aplicados entre dois `run_until`, no tempo simulado corrente, e gravados. Com a gravação, a
  rodada se repete sem rede e sem relógio, com o mesmo relatório.
- **Somente leitura por padrão.** Nenhum conector escreve no equipamento real sem opção explícita
  por variável e por execução (seção 1.9).
- **Modelos atuais inalterados.** Tudo que é novo no formato é opcional; ausente, desliga o
  recurso. Snapshots golden de `scripts/verificar` ficam idênticos byte a byte.
- **Custo zero de infraestrutura.** Nada exige servidor externo, nuvem ou banco pago. Broker MQTT,
  historiador e banco de dados são do cliente; o UTSim só se conecta. O painel é servido pelo
  próprio programa na máquina local.
- **Cada número com sua incerteza.** Todo KPI mostrado traz intervalo (de confiança, de predição
  ou de tolerância, com o nome certo) e o tamanho da amostra.

---

## 1. Gêmeo digital

### 1.1 Objetivo e níveis

Ligar um modelo `.ron` a dados reais (sensores, CLPs, MES, historiadores) para:

1. **ver** a planta real no 3D e no painel, com os KPIs calculados dos dados reais;
2. **comparar** o real com o simulado e avisar quando divergem;
3. **prever** as próximas horas a partir do estado atual, com replicações e intervalos;
4. **calibrar** o modelo com os dados, mantendo-o fiel ao longo do tempo.

A literatura separa três níveis: modelo digital (troca manual), sombra digital (dados fluem
sozinhos do real para o modelo) e gêmeo (fluxo automático nos dois sentidos). O UTSim cobre os
dois primeiros por completo; o sentido modelo → real fica restrito a recomendações exibidas ao
usuário e, opcionalmente, à escrita explícita de variáveis (seção 1.9).

### 1.2 Arquitetura

```
 sensores / CLP / MES / historiador
        |  OPC UA (cliente), MQTT, Sparkplug B, arquivo, SQL, Modbus TCP
        v
 [conector]  um fio por conector; converte para Observation; nunca toca a simulação
        |  canal limitado (descarta e conta se encher)
        v
 [utsim-twin::Ingest]  mapeamento tag -> alvo, transformações, qualidade, gravação
        |  Vec<Observation> ordenadas por (t_real, conector, sequência)
        v
 [laço do gêmeo]  mesmo laço de utsim-emu: a cada tick, poll -> aplicar -> run_until(alvo)
        |                    \
        |                     -> [previsão]  snapshot do estado -> N replicações à frente (rayon)
        v
 [estado real]  [estado simulado]  [desvio]  -> painel (HTTP/WebSocket) e cena 3D (utsim-scene/1)
```

**Crates novas**

- `crates/utsim-twin` (pura, compila para wasm32 sem os conectores): tipos `Observation`,
  `TagMap`, transformações, validação do mapeamento, estado real reconstruído
  (`RealState`: conteúdo, estado, contadores por objeto), gravação e leitura das observações,
  detecção de desvio (usa `utsim-stats`), calibração (usa `utsim-fit`), montagem do estado
  inicial para a previsão. Sem rede, sem relógio.
- `crates/utsim-connect` (nativa): conectores atrás de features, cada um implementando

  ```rust
  pub trait Connector: Send {
      fn name(&self) -> &str;
      /// Inicia o fio de leitura; observações vão para `tx`.
      fn start(&mut self, tx: SyncSender<RawObservation>, cancel: Arc<AtomicBool>) -> Result<(), ConnectError>;
      /// Estado da conexão para o painel (conectado, último erro, mensagens descartadas).
      fn health(&self) -> ConnectorHealth;
      /// Só existe quando o conector suporta escrita e ela foi permitida (seção 1.9).
      fn writer(&mut self) -> Option<&mut dyn Writer> { None }
  }
  ```

  | Feature | Conector | Dependência | Notas |
  |---|---|---|---|
  | `file` (padrão) | CSV e JSONL de histórico; reprodução de gravações | nenhuma | formato longo `instante;tag;valor;qualidade`; também lê o CSV exportado por historiadores |
  | `opcua` | cliente OPC UA: assinatura (monitored items) e leitura de histórico (HistoryRead, ReadRaw da Parte 11) | `async-opcua` com feature `client` (o projeto já usa a crate, hoje só `server` em produção e `client` nos testes) | política de segurança configurável; ao iniciar, lê a última janela do histórico para aquecer o estado |
  | `mqtt` | cliente MQTT 3.1.1 e 5, tópicos com curinga, cargas JSON ou número puro | `rumqttc` | TLS por `rustls`; mensagens retidas dão o último valor ao conectar |
  | `sparkplug` | decodificação Sparkplug B sobre o `mqtt` | `prost` com o `.proto` oficial da especificação copiado para o repositório, ou decodificador próprio | trata NBIRTH/DBIRTH (estado completo), NDATA/DDATA, NDEATH/DDEATH (sensor perdido), `seq`/`bdSeq` (lacunas), pedido de rebirth |
  | `sql` | consulta periódica ou de janela a bancos compatíveis com PostgreSQL (inclui os de séries temporais construídos sobre ele) | `postgres` (síncrono) | fora do instalador padrão até haver pedido |
  | `modbus` | leitura de registradores Modbus TCP por varredura | `tokio-modbus` | opcional, para equipamento sem OPC UA nem MQTT |

  O namespace unificado (tópicos MQTT organizados pela hierarquia ISA-95:
  empresa/site/área/linha/célula) é o caso típico; o mapeamento aceita curingas para que uma
  linha inteira seja ligada com poucas regras.

- **`utsim-emu`** ganha um `Frontend` novo, `TwinFrontend`, que embrulha os conectores como o
  `SceneFrontend` embrulha o OPC UA (`crates/utsim-emu/src/cena.rs`). O laço `run_paced` não muda
  de forma: `poll` passa a devolver também as entradas vindas das observações.

**Por que não reaproveitar o servidor OPC UA atual.** O `OpcUaFrontend` é servidor: o UTSim é o
equipamento e o CLP é o cliente (comissionamento virtual). No gêmeo o papel se inverte: o
equipamento real é o servidor e o UTSim é cliente. Os dois convivem: o servidor atual vira a
"fábrica sintética" dos testes (seção 1.11).

### 1.3 Mapeamento sensor → modelo

**Arquivo separado do modelo.** O mapeamento fica em `<modelo>.gemeo.ron`, não no `.ron` do
modelo: endereços, tópicos e credenciais mudam por instalação, e o modelo precisa continuar
portátil e executável sem a fábrica. O `.gemeo.ron` referencia o modelo pelo caminho e pelo hash
do conteúdo (aviso quando o modelo mudou depois do mapeamento). Alternativa em discussão
(decisão D3): campo opcional `twin` no próprio modelo.

```ron
Twin(
    model: "linha_montagem.ron",
    model_hash: "sha256:…",
    time: (unit_seconds: 1.0, timezone: "America/Sao_Paulo"),
    connectors: [
        (name: "uns", kind: Mqtt(url: "mqtts://broker.local:8883", client_id: "utsim-gemeo",
                                  credentials: Env("UTSIM_MQTT"))),
        (name: "clp1", kind: OpcUa(endpoint: "opc.tcp://clp1:4840", security: Basic256Sha256,
                                   credentials: File("~/.utsim/clp1.cred"))),
        (name: "hist", kind: File(path: "dados/historico_semana.csv")),
    ],
    tags: [
        // Contador de peças que entram: cada incremento é uma chegada na fila "buffer_1".
        (source: "uns", tag: "fabrica/linha1/estacao1/entrada/contador",
         target: Arrival(object: "buffer_1"), transform: CounterDelta(rollover: Some(65536))),
        // Estado da máquina: código inteiro mapeado para estados do UTSim.
        (source: "clp1", tag: "ns=2;s=M1.Estado",
         target: State(object: "M1"),
         transform: Enum({0: "idle", 1: "processing", 2: "setup", 3: "failed", 4: "stopped"})),
        // Tempo de ciclo medido (segundos) vira observação da medida "ciclo" de M1.
        (source: "uns", tag: "fabrica/linha1/M1/ciclo_s",
         target: Measure(object: "M1", name: "ciclo"), transform: Scale(1.0)),
        // Conteúdo medido (sensor de presença ou contagem do MES).
        (source: "clp1", tag: "ns=2;s=Buffer1.Qtd", target: Content(object: "buffer_1")),
        // Velocidade da esteira altera um parâmetro do modelo.
        (source: "clp1", tag: "ns=2;s=Est1.Vel", target: Param(path: "esteira_1.speed"),
         transform: Deadband(0.02)),
        // Posição de empilhadeira por localização em tempo real.
        (source: "uns", tag: "fabrica/rtls/emp_1", target: Position(object: "emp_1"),
         transform: Json(x: "/x", y: "/y")),
    ],
    record: (dir: "gravacoes", rotate_mb: 256, keep_days: 30, anonymize_people: true),
    write: [],   // vazio: somente leitura (seção 1.9)
)
```

**Alvos** (`Target`):

| Alvo | Efeito no estado real | Efeito no modo sincronizado |
|---|---|---|
| `Arrival(object)` | conta uma chegada | injeta um item no objeto (fonte guiada pelo real) |
| `Departure(object)` | conta uma saída | confere com a saída simulada; diferença entra no desvio |
| `State(object)` | estado real do objeto | `failed`/`stopped` reais forçam parada (`stop_block`, motivo `REAL`); volta força retomada |
| `Content(object)` | conteúdo real | reconciliação de WIP (seção 1.4) |
| `Measure(object, name)` | amostra de uma medida (ciclo, espera, temperatura) | alimenta calibração e desvio; não altera tempos |
| `Param(path)` | valor real de um parâmetro | aplica como `Set` do editor (mesmo caminho do "Aplicar ao modelo") no instante corrente |
| `Position(object)` | posição real (só visual e mapa de calor real) | não altera tempos |
| `Label(object, label)` | atributo do próximo item (tipo de produto, ordem, prazo) | rótulo do item injetado |

**Transformações** (`transform`): `Scale(k)`, `Linear(k, b)`, `Enum(tabela)`, `CounterDelta`
(com volta do contador), `RisingEdge` (pulso vira evento), `Debounce(t)`, `Deadband(d)`,
`Json(ponteiros)`, `Threshold(limite)`. Cada observação carrega a qualidade da fonte (OPC UA
Good/Uncertain/Bad, morte Sparkplug); `Bad` é descartada e contada, e o painel mostra o sensor
em cinza.

**Validação** (em `utsim-twin`, com avisos no mesmo estilo de `Model::warnings`): tag sem objeto,
objeto sem nenhuma tag, alvo incompatível com o tipo do objeto (estado em `Sink`), caminho de
`Param` inexistente, dois alvos `Arrival` para a mesma fonte (contagem dupla), tópico com
curinga que casa com nada depois de N minutos, modelo alterado depois do mapeamento.

**Assistente de mapeamento** (interface): conecta, lista os tags/tópicos vistos com último valor
e frequência, sugere alvos por nome (casamento aproximado com os nomes dos objetos) e grava o
`.gemeo.ron`.

### 1.4 Modos de operação

Os modos são cumulativos: o sincronizado inclui o que a sombra faz, e a previsão parte de
qualquer um dos dois.

**Sombra (só observa).** As observações montam o `RealState` e as séries reais. Em paralelo,
opcionalmente, o modelo roda no mesmo relógio sem receber nada (gêmeo "livre"), só para comparar
estatisticamente (seção 1.7). Nada do real altera o simulado. É o modo padrão e o mais seguro.
Uso: acompanhar a planta no 3D e no painel, validar o modelo com dados novos.

**Sincronizado (o modelo é corrigido pelo real).** As observações viram `Input` do laço:

- chegadas reais substituem o sorteio das fontes ligadas (`Source` com `driven_by: Real`, campo
  opcional novo; sem ele a fonte sorteia como hoje);
- paradas e quebras reais forçam o estado (motivo de parada `REAL`, ao lado de `EXTERNAL`);
- parâmetros reais (velocidade, número de operadores no turno) entram como `Set`;
- **reconciliação de WIP**: quando o conteúdo real de um objeto difere do simulado por mais que
  uma tolerância durante mais que um tempo de carência, o laço aplica uma correção (cria ou retira
  itens do fim da fila) e registra a correção como evento próprio no rastro e no log. As
  correções contadas são um indicador de qualidade do modelo: muitas correções indicam modelo
  descalibrado;
- o que o modelo não sabe sozinho (tempos de processamento, roteamento) continua sorteado, com a
  semente da rodada.

**Previsão (olhar à frente).** A cada período configurável (padrão 15 min de relógio) ou sob
demanda: tira um instantâneo do estado corrente (real reconciliado no sincronizado; real puro na
sombra), monta um estado inicial (seção 1.5), roda `R` replicações até o horizonte `H` em fios de
trabalho (`rayon`), sem ritmo, e publica: série prevista com banda de predição por KPI,
probabilidade de cumprir metas do turno, gargalo previsto, quando a fila X deve passar do limite.
A previsão nunca altera o laço principal. Sementes derivadas de `(seed, instante do
instantâneo, índice da replicação)`, então cada previsão é reproduzível. "E se": a mesma previsão
pode rodar com alterações (mais um operador, parada programada), mostrando a diferença pareada
com números aleatórios comuns.

**Reprodução de histórico.** Os modos sombra e sincronizado também rodam sobre um arquivo de
histórico ou uma gravação, acelerados (sem relógio, como o `--reproduzir` da emulação). Serve
para validar o modelo com a semana passada e para desenvolver sem acesso à fábrica.

**Tempo real e acelerado.** Com dados ao vivo o relógio é o real (velocidade 1). A unidade de
tempo do modelo vale `time.unit_seconds` (padrão 1 s, como em `utsim-emu`). O instante zero da
simulação corresponde ao instante de início do gêmeo; o calendário de turnos é deslocado para
que `t = 0` caia no horário real (exige o campo `run.start_at` opcional, data e hora do instante
zero, usado também pelos turnos e pelo painel para mostrar horas reais).

### 1.5 Estado inicial a partir do real (núcleo)

*Feito no núcleo, no modelo e na CLI (níveis 1 e 2, adiantado pela decisão D13; manual
"Simular a partir de um estado").* Notas da implementação:
- [x] Formato: arquivo `InitialState` à parte do modelo (`utsim_model::state`), com `clock`
  (instante do calendário em `t = 0`, no lugar de `shifts_clock`), `warmup` opcional (padrão 0)
  e `objects` por nome. Cada objeto é um `BlockState` com campos opcionais (`count`, `items`,
  `down`, `up`, `off_shift`, `position`, `elapsed`, `levels`), em vez de uma variante por tipo;
  os tempos são **decorridos** (`elapsed`, `stay`, `age`), não instantes negativos
  (`entered: -340`), para que gravar e retomar usem a mesma estrutura.
- [x] Kernel: `Simulation::set_start_state` (no lugar de `with_initial_state`),
  `Behavior::restore(&mut self, cx, &BlockState)` que recusa por padrão e
  `Behavior::save_state`; `Simulation::save_state` grava o estado num instante. Blocos:
  `Queue`, `Processor` (setup, processando, pronto; com operadores, a equipe é pedida de
  novo), `Source` (relógio de chegadas e item parado), `Conveyor` (posições), `Rack`
  (posições e permanência), `Transporter` (posição), quebras (`down`, `up`), turnos e
  chegadas por período pelo `clock`, tanques (como `initial`). `Sink`, `Combiner`,
  `Separator`, `Dispatcher` e fluxos recusam itens.
- [x] Residual condicional: `Dist::sample_residual` com o método de cada distribuição
  (`crates/utsim-core/src/residual.rs`); inversão numérica por bisseção sobre a sobrevivência
  exata (funções especiais copiadas para o `utsim-core`, que não pode depender do
  `utsim-stats`), e não Brent; elapsed além do suporte dá residual 0 e aviso.
- [x] CLI: `utsim run --estado arquivo` e `--salvar-estado T arquivo` (replicação 0).
- [x] Testes (`crates/utsim-model/tests/estado_inicial.rs`): casos exatos, retomada exata de
  um modelo determinístico e compatível de um aleatório, M/M/1 cheio converge, validação,
  estado vazio idêntico; residual conferido contra integração numérica (Weibull, Gamma,
  Triangular e as outras); golden inalterados.
- [ ] Rastro `Init`: não feito; os itens pré-carregados aparecem como `Created` em `t = 0`.
- [ ] Nível 3 (itens identificados pelo MES) e conteúdo de `Combiner`, `Separator` e fluxos.

Hoje toda rodada começa com o sistema vazio. A previsão e o início do modo sincronizado exigem
começar de um estado dado. É a maior mudança no núcleo deste plano.

**Formato** (`InitialState`, também gravável em arquivo para estudos "a partir de segunda às
6h"):

```ron
InitialState(
    time: 0.0,
    objects: {
        "buffer_1": Queue(items: [(entered: -340.0, labels: {"tipo": 2.0}), …]),
        "M1": Processor(busy: [(started: -12.0, labels: {})], state: "processing"),
        "M2": Down(since: -300.0, kind: Failure),
        "emp_1": Transporter(position: (12.0, 4.0, 0.0), task: None),
    },
    shifts_clock: 21600.0,   // instante do calendário que corresponde a t = 0
)
```

**Níveis de detalhe** (decisão D6), do mais simples ao mais fiel:

1. **Contagens e estados**: quantos itens em cada fila e processador, estado de cada máquina.
   Itens recebem instante de entrada igual a `t = 0` (permanência subestimada nos primeiros
   itens; aviso no relatório).
2. **Com idades**: instante de início do processamento em curso e da quebra em curso. O tempo
   residual é sorteado da distribuição condicional ao tempo já decorrido:
   `F_res(r) = (F(a + r) - F(a)) / (1 - F(a))`, por inversão numérica (Brent de
   `utsim-fit::special`) sobre a CDF da distribuição; para exponencial é a própria distribuição
   (sem memória); para `Const` é `max(c - a, 0)`; quando `a` passa do suporte (`Triangular` com
   `a > max`), residual zero e aviso de dado inconsistente.
3. **Com itens identificados**: rótulos, ordem e prazo de cada item (exige `Label` no
   mapeamento ou integração com o MES).

**Mudanças no kernel**

- `utsim-core`: `Simulation::with_initial_state(state)` antes do primeiro evento; cada bloco
  implementa `restore(&BlockInit, &mut Ctx)` com padrão que recusa (`Unsupported`), para que blocos
  novos não aceitem estado inicial em silêncio. Primeiro `Queue`, `Processor`, `Source`, `Sink`,
  quebras e turnos; depois `Conveyor`, `Rack`, `Transporter`, fluxos de tokens e fluidos.
- Estatísticas: os itens pré-carregados entram no conteúdo desde `t = 0`; a permanência deles
  conta a partir do instante de entrada informado (pode ser negativo). O aquecimento continua
  valendo (padrão 0 na previsão, porque o estado já é o real).
- Rastro: registro `Init` com o estado aplicado, para que `LiveScene` e o replay desenhem os
  itens desde o primeiro quadro.
- Determinismo: o estado inicial entra antes de qualquer sorteio, os residuais sorteiam do stream
  do próprio bloco; mesma semente e mesmo estado dão o mesmo relatório.

### 1.6 Calibração a partir dos dados

**Conjuntos de dados derivados** das observações, por parâmetro do modelo:

| Parâmetro | Dado derivado |
|---|---|
| intervalo entre chegadas de uma `Source` | diferenças entre chegadas reais (`Arrival`) |
| tempo de processamento | duração dos períodos em `processing` (`State`) ou `Measure` de ciclo |
| setup | duração dos períodos em `setup` |
| MTBF / MTTR | intervalos entre `failed` e duração de `failed` |
| tempo de viagem | intervalos entre posições ou eventos de carga e descarga |
| mix de produtos | frequência dos rótulos `tipo` |

**Como funciona**

1. Janela configurável (padrão 7 dias ou 500 observações).
2. `utsim-fit::report::analyze` sobre cada conjunto, com os diagnósticos de independência e
   tendência que já existem (autocorrelação, corridas, Mann–Kendall). Tendência significativa
   impede a sugestão e avisa ("o tempo de ciclo está subindo; ajustar uma distribuição fixa
   esconderia isso").
3. Dados censurados: um período que começou antes da janela ou ainda não terminou é censurado;
   na versão inicial é descartado e contado (com aviso se passar de 5 %); estimação com censura
   fica como P3.
4. Comparação com o parâmetro atual: teste de aderência dos dados novos à distribuição do modelo
   (Anderson–Darling e Kolmogorov–Smirnov de `utsim-fit::gof`); só sugere troca quando rejeita.
5. **Sugestão, não troca automática** (decisão D7): o painel mostra "o tempo de M1 no modelo é
   Triangular(20, 30, 40); os últimos 600 ciclos sugerem Gamma(k = 9,1, θ = 3,4), média 31,0 ±
   0,4", com o gráfico Q-Q, e o botão aplica como um `Command::Batch` de `Set` (um passo de
   desfazer), gravando a origem no histórico do modelo.
6. **Calibração contínua opcional** para os casos simples: taxa de chegada exponencial e MTBF/MTTR
   exponenciais por atualização conjugada (Gama–Poisson) com esquecimento exponencial (fator
   `λ` por hora), o que dá estimativa e intervalo de credibilidade a cada observação sem rerodar o
   ajuste completo. Os valores entram só na previsão, nunca no modelo gravado, salvo aplicação
   explícita.
7. **Calibração de parâmetros não observáveis** (por exemplo, a regra de despacho que reproduz a
   fila observada): reaproveita `utsim-opt` com objetivo "distância entre KPIs simulados e reais"
   (soma de quadrados padronizados pela variância entre replicações). É o experimento de
   calibração: fica como P3.
8. Filtro de partículas para estimar estado e parâmetros juntos: registrado como pesquisa futura,
   fora deste plano (custo alto; a literatura recente ainda o testa em modelos pequenos).

### 1.7 Detecção de desvio entre real e simulado

**Pergunta**: o real ainda se comporta como o modelo diz? Duas famílias de verificação, ambas em
`utsim-twin::drift` usando `utsim-stats`:

**(a) KPI na janela contra a distribuição prevista.** Para cada KPI monitorado (vazão por hora,
lead time médio, WIP médio, ocupação), em janelas fixas (padrão 1 h):

- o valor real `y` da janela é comparado com a distribuição do mesmo KPI em `R` replicações da
  simulação na mesma janela, partindo do mesmo estado (previsão de uma janela) ou do gêmeo livre;
- estatística: posto de `y` entre as `R` replicações (`p = (1 + #{y_r ≥ y}) / (R + 1)`, bilateral),
  sem suposição de normalidade; com `R = 39` o menor p bilateral é 0,05;
- o resíduo padronizado `z = (y - média) / desvio` alimenta um **CUSUM** e um **EWMA** por KPI
  (seção 3, E4), que acumulam desvios pequenos e persistentes que uma janela sozinha não mostra;
- dois limiares, aviso e desvio, e histerese para não piscar.

**(b) Distribuições de tempos.** Amostras reais de tempo de ciclo, espera ou lead time contra a
amostra simulada: Kolmogorov–Smirnov de duas amostras e Anderson–Darling k-amostras para forma,
Mann–Whitney para posição; contagens por janela contra Poisson ou contra a distribuição simulada
por qui-quadrado (seção 3, E7).

**Controle de alarmes falsos.** Com dezenas de KPIs e objetos, testes simultâneos geram alarmes
por acaso; os p-valores de uma janela passam por Benjamini–Hochberg (taxa de falsas descobertas
padrão 10 %), e a interface mostra quantos testes foram feitos.

**Diagnóstico.** O alerta diz onde: "vazão da linha 1 abaixo do previsto há 3 h (CUSUM);
tempo de processamento de M2 maior que o modelo (KS, p < 0,001; mediana 34 s contra 30 s)" e
aponta para a calibração (seção 1.6). Comparação de sequências de eventos (alinhamento de
sequências), útil com pouco volume de produção, fica como P3.

**Amostras pequenas.** Com poucas observações os testes têm pouco poder; a interface mostra o
tamanho da amostra e o menor desvio detectável (`δ ≈ (z_{1-α/2} + z_{1-β}) · σ / √n`), em vez de
"sem desvio".

### 1.8 Determinismo e reprodutibilidade

- **Gravação**: todas as observações brutas e já mapeadas, com instante real, instante simulado
  de aplicação, conector, sequência e qualidade, em JSONL compactado por blocos (`.utrec`), com
  rotação por tamanho e retenção por dias. As correções de WIP e os `Set` aplicados também vão para
  a gravação (estendem `CommandLog`).
- **Reexecução**: `utsim gemeo reproduzir gravacao.utrec` roda sem rede e sem relógio e dá o mesmo
  relatório, o mesmo rastro e as mesmas previsões (com as mesmas sementes). Teste obrigatório.
- **Quantização**: observações que chegam entre dois ticks são aplicadas no início do tick
  seguinte, na ordem `(t_real, conector, sequência)`; o instante simulado de aplicação é o que fica
  gravado, então o resultado não depende da hora de chegada dos pacotes na reexecução.
- **Relógios**: a fonte pode trazer carimbo próprio (OPC UA SourceTimestamp, Sparkplug
  `timestamp`); o padrão usa o carimbo da fonte quando presente e o relógio local quando não,
  com aviso quando a diferença passa de 2 s (relógios desajustados).

### 1.9 Segurança

- **Somente leitura por padrão.** Nenhum conector abre caminho de escrita sem `write: [...]` no
  `.gemeo.ron` (lista de variáveis permitidas, com faixa de valores) **e** a opção
  `--permitir-escrita` na execução. A previsão e a reprodução nunca escrevem. Escrita de comandos
  no real (decisão D4) fica fora da primeira entrega; o que o gêmeo produz são recomendações no
  painel.
- **Credenciais** fora do `.ron`: variável de ambiente ou arquivo local com permissão restrita,
  nunca gravadas em log ou gravação.
- **Transporte**: MQTT com TLS e verificação do certificado; OPC UA com política de segurança e
  modo assinado e cifrado por padrão, com aviso visível quando o usuário escolhe "sem segurança".
- **Painel e cena**: escutam só em `127.0.0.1` por padrão, como a cena ao vivo hoje; abrir para a
  rede exige opção explícita, imprime aviso e liga um token de acesso de leitura (seção 2.10).
- **Dados pessoais**: localização em tempo real de pessoas e identificação de operadores são
  dados pessoais (LGPD). Padrão `anonymize_people: true`: identificadores trocados por apelidos
  estáveis por gravação; retenção limitada; aviso no assistente de mapeamento.
- **Recursos**: filas limitadas por conector (descarta e conta), limite de tamanho de mensagem,
  reconexão com espera crescente; um conector com problema não derruba o laço.

### 1.10 Interface

**Godot** (depois da G3, que já prevê o cliente da cena ao vivo):

- janela "Gêmeo": conectores com estado (conectado, mensagens por segundo, descartadas, último
  erro), tabela de tags (último valor, idade, qualidade), assistente de mapeamento;
- seletor de modo (sombra, sincronizado, reprodução) e de previsão;
- na cena 3D: objetos com a cor do estado **real**; contorno com a cor do estado simulado quando
  diferem; sensores sem dado em cinza; itens reais e simulados distinguíveis (seção 4, O1);
- marcadores na linha do tempo para alertas de desvio e correções de WIP.

**Painel web** (seção 2): visão "Real × simulado", previsão com banda, alertas.

**CLI**:

```
utsim gemeo mapear modelo.ron --mqtt mqtts://broker:8883 --descobrir 60   # gera o .gemeo.ron
utsim gemeo sombra linha.gemeo.ron --painel 8080 --cena-porta 7878
utsim gemeo sincronizar linha.gemeo.ron --painel 8080 --prever 15m --horizonte 4h --replicacoes 40
utsim gemeo reproduzir gravacoes/2026-11-02.utrec --acelerado
utsim gemeo calibrar linha.gemeo.ron --janela 7d --html calibracao.html
utsim gemeo estado linha.gemeo.ron --salvar estado_6h.ron                 # instantâneo
utsim run modelo.ron --estado-inicial estado_6h.ron --replicacoes 40       # estudo a partir do real
```

### 1.11 Testes

- **Fábrica sintética**: um segundo modelo UTSim roda sob `utsim emular` (servidor OPC UA atual)
  ou publica num broker `rumqttd` embutido nos testes; é o "real". O gêmeo, com o mesmo modelo,
  não deve acusar desvio (taxa de alarmes falsos dentro do nominal em muitas janelas); com o
  "real" alterado (tempo de M2 +15 %), deve acusar em até N janelas, apontando M2. Mede-se poder e
  taxa de alarmes falsos.
- Mapeamento: cada transformação com casos de borda (volta do contador, debounce, deadband,
  JSON ausente, qualidade ruim).
- Sparkplug: cargas de exemplo da especificação decodificadas; lacuna de `seq` detectada; morte
  marca os tags como perdidos.
- Estado inicial: fila pré-carregada no M/M/1 converge para a teoria; residual condicional
  conferido contra integração numérica para Weibull, Gamma e Triangular; com estado vazio o
  relatório é idêntico byte a byte ao de hoje.
- Reprodução: gravação de uma sessão sincronizada reexecutada dá relatório e previsões idênticos.
- Calibração: dados sintéticos de distribuição conhecida recuperam parâmetros dentro do IC; dado
  com tendência bloqueia a sugestão.
- Conectores reais (OPC UA, MQTT) atrás de testes marcados `#[ignore]` na CI, rodados com
  servidores locais.

### 1.12 Riscos

- **Qualidade dos dados reais** (carimbos ruins, contadores que reiniciam, estados mal
  codificados) é o maior risco prático; o assistente de mapeamento e a gravação existem para
  diagnosticar isso cedo.
- **Estado inicial** toca todos os blocos; risco de regressão. Mitigação: `Unsupported` por padrão,
  golden inalterados, entrega por tipo de bloco.
- **Reconciliação de WIP** pode esconder um modelo errado; por isso cada correção é contada e
  exibida.
- **Testes estatísticos on-line** com amostras pequenas têm pouco poder e com muitos KPIs geram
  alarmes falsos; mitigação: posto entre replicações, CUSUM/EWMA, Benjamini–Hochberg, menor desvio
  detectável na interface.
- **Dependências** de rede (async, TLS) aumentam o binário; conectores atrás de features, fora do
  pacote padrão os que ninguém pediu (SQL, Modbus).

### 1.13 Estimativa e modelo sugerido

| Parte | Horas | Modelo |
|---|---|---|
| T1 `utsim-twin`: observação, mapeamento, transformações, validação, `RealState`, gravação `.utrec` | 9–12 | Opus |
| T2 conectores: arquivo/histórico (3–4), MQTT (4–5), Sparkplug B (4–6), cliente OPC UA com assinatura e histórico (5–7), SQL (2–3), Modbus TCP (2–3) | 20–28 | Sonnet (Sparkplug e OPC UA com revisão Opus) |
| T3 modo sombra e gêmeo livre no laço (`TwinFrontend`) | 5–7 | Sonnet |
| T4 modo sincronizado: fonte guiada, paradas reais, `Set` de parâmetros, reconciliação de WIP | 10–14 | Opus |
| T5 estado inicial no kernel (níveis 1 e 2, blocos principais, rastro `Init`) — **feito**, menos o rastro `Init` | 18–26 | Opus |
| T6 previsão à frente com replicações, "e se" pareado, `run.start_at` | 8–11 | Opus |
| T7 calibração: conjuntos derivados, ajuste, sugestão, atualização conjugada | 8–11 | Opus |
| T8 desvio: posto entre replicações, CUSUM/EWMA, KS/AD/Mann–Whitney, BH, menor desvio detectável | 8–11 | Opus |
| T9 segurança: escrita explícita, credenciais, TLS, anonimização | 3–4 | Sonnet |
| T10 CLI `utsim gemeo` | 2–3 | Sonnet |
| T11 Godot: janela Gêmeo, cores real × simulado, marcadores | 8–11 | Sonnet |
| **Total** | **99–138** | |

### 1.14 Requisitos sugeridos

| ID | Requisito |
|---|---|
| RF-209 | ligar um modelo a fontes de dados reais por OPC UA (cliente), MQTT, Sparkplug B, arquivos de histórico e, opcionalmente, SQL e Modbus TCP |
| RF-210 | mapear tags e tópicos para chegadas, saídas, estados, conteúdos, medidas, parâmetros, posições e rótulos de objetos, com transformações e validação, num arquivo separado do modelo |
| RF-211 | descobrir tags e tópicos de uma fonte e sugerir o mapeamento pelos nomes dos objetos |
| RF-212 | acompanhar a planta real no 3D e no painel sem alterar o modelo (modo sombra), com um gêmeo livre opcional para comparação |
| RF-213 | sincronizar o modelo com o real: chegadas e paradas reais, parâmetros reais e reconciliação de WIP contada como correção |
| RF-214 | iniciar uma simulação a partir de um estado dado (contagens, estados, idades dos processamentos e quebras, itens com rótulos) e gravar esse estado em arquivo |
| RF-215 | prever as próximas horas a partir do estado atual com replicações, bandas de predição e probabilidade de cumprir metas, periodicamente ou sob demanda, com variações "e se" comparadas por números aleatórios comuns |
| RF-216 | reproduzir uma sessão gravada ou um histórico sem rede e sem relógio, com resultados idênticos |
| RF-217 | sugerir distribuições e parâmetros a partir dos dados recebidos, com testes de aderência e de tendência, e aplicá-los com um clique e desfazer |
| RF-218 | atualizar continuamente taxas de chegada e de falha para a previsão, sem alterar o modelo gravado |
| RF-219 | detectar desvio entre real e simulado por KPI e por distribuição de tempos, com limiares de aviso e de desvio, controle de alarmes falsos e indicação do objeto provável |
| RF-220 | ver o estado de cada conector e de cada tag (último valor, idade, qualidade, perdas) |
| RF-221 | operar somente em leitura por padrão; escrever em variáveis reais só com lista explícita e opção na execução |
| RF-222 | anonimizar identificadores de pessoas nas gravações e limitar a retenção |
| RF-223 | usar a hora real (fuso, calendário de turnos alinhado ao início) na simulação ligada ao real |
| RF-224 | ver no 3D o estado real e o simulado de cada objeto, com destaque quando diferem |

---

## 2. Painel para gestores e engenheiros

### 2.1 Objetivo

Um painel que responda, para o gestor, "estamos bem? vamos cumprir? onde está o problema?" e,
para o engenheiro, "por quê? desde quando? com que confiança?", sobre três fontes de dados com a
mesma interface: (a) resultados de replicações de um estudo; (b) uma simulação rodando (ao vivo,
replicação a replicação ou no ritmo do relógio); (c) o gêmeo digital (real, simulado e previsto).

### 2.2 Onde roda

**Página web servida pelo próprio programa** (recomendado, decisão D1):

- `crates/utsim-dash` (nativa): servidor HTTP mínimo sobre `std::net` (o mesmo estilo do
  `SceneServer`, que já promove `GET` para WebSocket na mesma porta), servindo a página
  (HTML, CSS e JS embutidos no binário com `include_str!`), uma API JSON e um WebSocket de
  atualização. Sem dependência de servidor web externo e sem nuvem.
- `utsim painel modelo.ron --porta 8080` (estudo), `--painel 8080` nos comandos `run`, `emular`,
  `gemeo` (ao vivo). Abre no navegador padrão do sistema; vários navegadores ao mesmo tempo
  (uma TV na fábrica, o notebook do engenheiro).
- **Mesma página sem servidor**: o JS do painel aceita os mesmos JSON vindos do `utsim-web`
  (wasm), então a página do navegador (`web/`) ganha o painel sobre rodadas feitas ali. E a
  exportação HTML (seção 2.8) é o painel congelado com os dados embutidos.
- **Godot**: a G3 já prevê gráficos de resultados; o painel do Godot usa os mesmos JSON (cálculo
  único em `utsim-kpi`) e desenha os cartões e gráficos principais nativamente; o botão "Abrir
  painel completo" abre a página web no navegador. Não se duplica o painel inteiro no Godot.

**Por que não só no Godot**: o gestor não precisa instalar nem abrir o 3D; uma TV ou um
navegador basta; a página serve também à versão do navegador e ao relatório exportado.

**Gráficos**: SVG gerado no navegador por um módulo próprio pequeno (linhas, barras, áreas,
caixas, pontos, faixas), coerente com o SVG que `html.rs` já gera no Rust (decisão D2:
alternativa é embutir uma biblioteca JS pequena de séries temporais). Sem CDN: tudo embutido,
funciona sem internet.

### 2.3 Cálculo dos KPIs (`utsim-kpi`)

Módulo novo `crates/utsim-model/src/kpi.rs` (ou crate `utsim-kpi` se crescer), puro, que recebe
`RunReport`s, rastro ou `RealState` + séries reais e devolve `KpiSet` serializável:
`{id, nome, unidade, valor, ic: (lo, hi), ip: (lo, hi), n, alvo, situação, série, origem}`.
O painel, o Godot, o HTML e a CLI (`utsim kpi modelo.ron --json`) leem a mesma estrutura.

**KPIs e fórmulas** (por objeto, grupo de objetos e sistema; `T` = tempo observado depois do
aquecimento; entre replicações, média com IC de t de Student e intervalo de predição, seção 3):

| KPI | Fórmula | Fonte no núcleo |
|---|---|---|
| Vazão | `saídas / T` (por hora, turno, dia) | `outputs` do `BlockReport` |
| Vazão no turno | `saídas no turno / tempo de turno` | turnos (`utilization_in_shift`) |
| Atingimento da meta | `vazão / meta`; probabilidade `P(saídas ≥ meta)` entre replicações | E2 |
| Lead time (tempo no sistema) | média, P50, P90, P95 de `time_in_system` | medida da `Sink`, `spans` |
| Tempo de fila | média e P90 da permanência nas filas | `staytime` |
| WIP | conteúdo médio ponderado no tempo, somado no grupo; máximo | `content` |
| Lei de Little (verificação) | `WIP ≈ vazão × lead time` | RF-161 |
| Utilização | fração em `processing` + `setup` (estações) ou viagem/carga (transportadores) | `utilization_of` |
| Bloqueio e falta | fração em `blocked` e `starved`/`idle` | estados |
| Disponibilidade | `(tempo planejado - paradas) / tempo planejado` | quebras, turnos, motivos |
| Desempenho | `(tempo de ciclo ideal × produzidos) / tempo operando` | parâmetro `ideal_cycle` opcional por máquina |
| Qualidade | `boas / produzidas` | saídas de refugo (roteamento para `Sink` marcada `scrap: true`, campo opcional) |
| OEE | `disponibilidade × desempenho × qualidade`, com os três fatores sempre ao lado | RF-208 |
| MTBF / MTTR observados | média dos intervalos entre falhas e das durações | RF-208 |
| Atendimento de prazo (OTD) | `itens com saída ≤ prazo / itens`; atraso médio dos atrasados | rótulo `prazo` (convenção, decisão D11) |
| Nível de serviço | fração com lead time ≤ `L` dado | medida da `Sink` |
| Perdas | `perdidos / chegadas` (desistência, `on_blocked: Drop`) | `lost`, RF-145 |
| Gargalo | maior utilização; gargalo móvel | `bottleneck`, RF-162 |
| Distância e viagens | metros percorridos, viagens vazias e carregadas por transportador | rastro |
| Custo por peça | `Σ custo por estado × tempo + custo por item / saídas` | backlog "custos por estado" |
| Energia por peça | `Σ potência por estado × tempo / saídas` | backlog "energia e emissões" |
| Takt vs ciclo | `tempo disponível / demanda` contra o ciclo médio do gargalo | parâmetros do painel |
| Correções do gêmeo | correções de WIP por hora | seção 1.4 |
| Desvio real × simulado | índice de alarmes por KPI | seção 1.7 |

KPIs que dependem de campos ainda inexistentes (custo, energia, prazo, refugo, ciclo ideal)
aparecem só quando o modelo os define; o painel não inventa número.

**Semáforo.** Cada KPI com alvo (`dashboard.targets` no modelo ou no arquivo do painel) recebe
verde, amarelo ou vermelho **pela probabilidade**, não pela média: verde se a probabilidade de
cumprir é ≥ 90 %, amarelo entre 50 % e 90 %, vermelho abaixo de 50 % (limiares configuráveis).
Quando o IC da média cruza o alvo, o cartão diz "indefinido: rode mais replicações" com o número
que falta (RF-159).

### 2.4 Duas visões

**Visão do gestor** (uma tela, leitura em 10 segundos):

- linha de cartões: vazão, OEE (com os três fatores pequenos), lead time P90, WIP, OTD, custo por
  peça (quando houver); cada cartão com valor, faixa, semáforo e minigráfico da tendência;
- frase-resumo gerada dos números ("Com 40 replicações, a vazão esperada é 118 ± 3 peças/h; em
  9 de cada 10 turnos ela fica entre 104 e 131. A meta de 120 é cumprida em 41 % dos turnos.
  Gargalo: M2, ocupada 94 % do tempo.");
- Pareto de perdas (paradas por motivo, em horas);
- no gêmeo: real agora × previsto até o fim do turno com banda, probabilidade de fechar a meta;
- comparação de cenários: um cartão por cenário com a diferença para o atual e se ela é
  significativa (`delta_cell`, `≈`);
- sem jargão: "intervalo de confiança" aparece como "precisão da estimativa", "intervalo de
  predição" como "faixa esperada num turno", com o termo técnico no detalhe (seção 3, E1).

**Visão do engenheiro** (várias abas, com drill-down):

- **Objetos**: tabela de `object_stats` com ordenação, filtro e exportação; clique abre o objeto;
- **Objeto**: estados (barra empilhada), Gantt de estados (RF-163), histograma e ECDF de
  permanência e de ciclo, série de conteúdo com banda, quebras (MTBF/MTTR), gargalo móvel;
- **Fluxo**: diagrama de Sankey das quantidades entre objetos (a partir das conexões e das saídas
  por porta), espessura pela vazão e cor pela espera média do destino;
- **Distribuições**: caixa "erro e risco" de qualquer KPI entre replicações (seção 3, E9),
  histograma com o ajuste, Q-Q;
- **Séries**: WIP, vazão por janela, lead time por janela, com banda; zoom e seleção de janela;
- **Cenários**: tabela de respostas, gráfico de intervalos lado a lado, ANOVA/Tukey (RF-164),
  seleção do melhor (E9), efeitos e tornado (RF-166);
- **Estatística**: aquecimento, precisão, Little, CEP, capacidade (seção 3);
- **Gêmeo**: real × simulado por KPI, resíduos, CUSUM/EWMA, testes de distribuição, correções,
  conectores e tags;
- **Eventos**: lista filtrável de eventos do rastro (quebras, paradas, alertas), com "ir para" na
  linha do tempo do 3D.

**Drill-down**: cartão do gestor → aba do engenheiro já filtrada → objeto → evento → foco no 3D.
O "foco no 3D" é uma mensagem de extensão nova na cena ao vivo
(`{"tipo":"foco","objeto":"M2","tempo":T}`), que o Godot interpreta movendo a câmera; leitores
antigos ignoram tipos desconhecidos (regra já documentada na seção 4p da arquitetura).

### 2.5 Tipos de gráfico

| Gráfico | Uso |
|---|---|
| Cartão com valor, faixa, semáforo e minigráfico | gestor |
| Linha com banda (IC da média e faixa de predição em tons diferentes) | séries, previsão (leque) |
| Barra empilhada de estados | ocupação por objeto |
| Gantt de estados e de ordens | dinâmica, prazos |
| Histograma, ECDF, Q-Q | distribuições, calibração |
| Caixa "erro e risco": média com IC + P5–P95 + P25–P75 + histograma lateral | comparação de cenários e KPIs |
| Intervalos lado a lado (um por cenário) | comparação |
| Pareto | perdas, paradas, causas de atraso |
| Sankey | fluxo de itens e de pessoas |
| Gráfico de controle (I-MR, X̄-S, EWMA, CUSUM) | CEP, desvio do gêmeo |
| Dispersão e matriz de correlação | relações entre métricas (RF-166) |
| Tornado e gráfico de efeitos | sensibilidade, experimentos |
| Mapa de calor do piso | no 3D (RF-165) e como imagem no painel |

Regras de desenho: paleta dos estados igual à da cena (`style::state_color`), cores acessíveis a
daltônicos, tema claro e escuro, eixos com unidade, nenhuma pizza (barra empilhada é mais
legível), números com casas decimais proporcionais à precisão (não mostrar 118,4372 ± 3,1).

### 2.6 Atualização ao vivo

- **Estudo com replicações**: a cada replicação concluída (`run_replications_with(progress)`), o
  servidor recalcula os KPIs e envia; o usuário vê os IC encolhendo e o contador "faltam k
  replicações para ±2 %". Cancelar usa o `AtomicBool` que já existe.
- **Simulação no ritmo do relógio** (`emular`, `cena --servir`): KPIs acumulados e de janela móvel
  a cada 1 s de relógio, a partir do `Snapshot` do `Emulator` e do rastro.
- **Gêmeo**: real (das observações), simulado e previsto, cada um com sua cor; alertas chegam como
  mensagens próprias.
- **Transporte**: WebSocket com JSON; contrapressão como na cena (fila limitada por cliente,
  descarte contado, desconexão de cliente travado); sem clientes, nada é calculado.
- **Modo quiosque**: `?quiosque=1` esconde menus, aumenta fontes para leitura a distância, alterna
  entre páginas a cada N segundos.

### 2.7 Alertas

Regras no arquivo do painel (seção 2.9):

- **limiar**: KPI acima ou abaixo de um valor por um tempo mínimo, com histerese;
- **estatístico**: sinal de CEP (regras de Western Electric selecionáveis), CUSUM/EWMA, desvio do
  gêmeo (seção 1.7);
- **previsão**: probabilidade de não cumprir a meta do turno acima de x %; fila X prevista acima do
  limite antes de H horas;
- **dados**: sensor sem dado há mais de T, conector desconectado, correções de WIP demais.

Saídas: no painel (lista, som opcional, destaque no cartão), na linha do tempo do 3D, num
arquivo de log, e opcionalmente publicação num tópico MQTT (é escrita: exige a lista explícita da
seção 1.9). E-mail e mensageiros ficam fora (custo e infraestrutura); quem precisa assina o
tópico MQTT com a ferramenta que já usa.

### 2.8 Exportação

| Formato | Como | Dependência |
|---|---|---|
| HTML | o painel congelado: página única com os JSON embutidos e os mesmos scripts; abre sem servidor | nenhuma (estende `html.rs`) |
| PDF | folha de estilo de impressão (A4, cartões em grade, gráficos em SVG vetorial) e impressão pelo navegador; na CLI, `--pdf` chama o navegador do sistema em modo sem janela quando disponível, senão grava o HTML e avisa | nenhuma no binário (decisão D8) |
| Planilha | `.xlsx` com uma aba por tabela (resumo, objetos, replicações, séries, cenários, alertas) | `rust_xlsxwriter` atrás de feature (decisão D9); CSV sempre disponível (`csv.rs`) |
| Imagem | cada gráfico baixa como SVG e PNG (canvas do navegador) | nenhuma |
| JSON | `KpiSet` e séries, para integração | nenhuma |

### 2.9 Configuração do painel

Sem configuração o painel monta-se sozinho: cartões padrão (vazão, lead time, WIP, utilização do
gargalo), abas do engenheiro completas. Para metas e alertas, arquivo opcional
`<modelo>.painel.ron` (ou campo `dashboard` no modelo, decisão D3):

```ron
Dashboard(
    title: "Linha 1",
    period: Shift,                       // agregação padrão dos cartões: hora, turno, dia, rodada
    cards: ["vazao", "oee:M2", "lead_time_p90", "wip:linha", "otd"],
    targets: {"vazao": AtLeast(120.0), "lead_time_p90": AtMost(900.0)},
    groups: {"linha": ["buffer_1", "M1", "buffer_2", "M2"]},
    alerts: [
        (kpi: "vazao", rule: Below(100.0, for_minutes: 30)),
        (kpi: "vazao", rule: ForecastMiss(probability: 0.5)),
        (kpi: "lead_time", rule: Cusum(k: 0.5, h: 5.0)),
    ],
    exposed_params: ["M2.servers", "chegada.interarrival.mean"],   // seção 4, O6
)
```

### 2.10 Acesso e custo

- Escuta em `127.0.0.1` por padrão; `--painel-endereco 0.0.0.0` abre para a rede local, imprime
  aviso e gera um token de leitura incluído no endereço mostrado (`/?t=…`); comandos (rodar
  cenário, "e se") exigem um segundo token.
- Nenhum serviço externo: sem conta, sem nuvem, sem banco. Para acesso fora da fábrica, o
  manual indica o que o cliente já tiver (VPN), sem nada do lado do UTSim.

### 2.11 Testes

- `kpi.rs`: cada fórmula conferida em modelos com resposta conhecida (M/M/1, linha
  determinística de dois postos, OEE com paradas e refugo calculados à mão); KPIs ausentes quando
  o campo não existe.
- Servidor: testes de integração por HTTP e WebSocket (como os do `SceneServer`), cliente lento
  não trava a simulação, token exigido fora de `127.0.0.1`.
- Página: teste de fumaça em Node sem navegador (como `web/smoke.mjs`) carregando JSON de exemplo
  e conferindo que todo gráfico gera SVG válido; regressão visual das páginas exportadas é
  opcional (backlog: testes de regressão visual).
- Exportação: HTML abre sem servidor e contém os dados; `.xlsx` lido de volta com uma crate de
  leitura nos testes; CSV igual ao de `csv.rs`.

### 2.12 Riscos

- **Depósito de números**: muitas abas sem pergunta clara. Mitigação: visão do gestor com no
  máximo seis cartões e a frase-resumo; cada aba do engenheiro começa por uma pergunta.
- **JS crescendo sem testes**: manter o JS só como desenho; todo cálculo no Rust.
- **Desempenho ao vivo** com muitos objetos: agregar no servidor e mandar só o que a aba aberta
  pede (assinatura por aba).
- **Semáforo por probabilidade** pode confundir quem espera semáforo pela média; o detalhe do
  cartão explica, e o critério é configurável.

### 2.13 Estimativa e modelo sugerido

| Parte | Horas | Modelo |
|---|---|---|
| D1 `kpi.rs`: KPIs, fórmulas, IC e faixa, por janela e grupo, semáforo por probabilidade | 8–11 | Opus |
| D2 `utsim-dash`: servidor HTTP, API JSON, WebSocket, tokens, contrapressão | 6–9 | Sonnet |
| D3 página: estrutura, módulo de gráficos SVG, temas, acessibilidade | 10–14 | Sonnet |
| D4 visão do gestor (cartões, frase-resumo, Pareto, cenários) | 5–7 | Sonnet |
| D5 visão do engenheiro (abas, drill-down, Gantt, Sankey, distribuições, eventos) | 10–14 | Sonnet |
| D6 ao vivo (replicações, ritmo do relógio, gêmeo) e modo quiosque | 4–6 | Sonnet |
| D7 alertas (limiar, previsão, dados; os estatísticos usam E4/E5) | 4–6 | Sonnet |
| D8 exportação HTML, PDF por impressão, `.xlsx`, imagens | 6–8 | Sonnet |
| D9 arquivo do painel, metas, grupos, parâmetros expostos | 3–4 | Sonnet |
| D10 Godot: cartões e gráficos principais pelo mesmo JSON, foco no 3D, abrir painel | 5–7 | Sonnet |
| CLI `utsim painel`, `utsim kpi`, `--painel` nos comandos | 2–3 | Haiku |
| **Total** | **63–89** | |

### 2.14 Requisitos sugeridos

| ID | Requisito |
|---|---|
| RF-225 | abrir um painel no navegador servido pelo próprio programa, sem serviço externo, para estudos, simulações em andamento e o gêmeo digital |
| RF-226 | ver a visão do gestor: até seis KPIs com valor, faixa, semáforo pela probabilidade de cumprir a meta, tendência e frase-resumo |
| RF-227 | ver a visão do engenheiro: tabela de objetos, estados, Gantt, distribuições, séries com banda, Sankey de fluxo, cenários, estatística e eventos, com drill-down até o objeto e o instante no 3D |
| RF-228 | calcular vazão, lead time e percentis, WIP, utilização, bloqueio, disponibilidade, desempenho, qualidade, OEE, MTBF/MTTR, OTD, nível de serviço, perdas, distância, custo e energia por peça quando o modelo define os dados necessários |
| RF-229 | ver todo KPI com intervalo de confiança da média, faixa esperada por período e tamanho da amostra |
| RF-230 | acompanhar ao vivo a convergência dos KPIs durante as replicações e os KPIs de janela durante a simulação no ritmo do relógio e no gêmeo |
| RF-231 | configurar metas, grupos de objetos, cartões e alertas por limiar, sinal estatístico, previsão e falta de dados |
| RF-232 | exportar o painel em HTML autocontido, PDF, planilha, CSV, JSON e imagens dos gráficos |
| RF-233 | usar o painel em TV em modo quiosque, com alternância de páginas |
| RF-234 | abrir o painel na rede local só com opção explícita e token de leitura; comandos exigem token próprio |
| RF-235 | ver os KPIs principais dentro do Godot com os mesmos números do painel web |
| RF-236 | ver o Pareto de perdas e paradas por motivo |
| RF-237 | publicar alertas num tópico MQTT, com escrita permitida explicitamente |
| RF-238 | ler os mesmos KPIs pela linha de comando em JSON |

---

## 3. Estatística (foco)

O que segue é **adicional** às prioridades P1–P3 de `plano_analises_e_dinamica.md`, seção 2
(RF-157 a RF-166). Tudo mora em `utsim-stats` (a crate proposta lá), com testes contra
resultados publicados ou calculados à mão. Cada análise tem uma pergunta e um texto curto para o
gestor.

### E1. Intervalo de confiança, intervalo de predição e percentis, explicados

**Pergunta do gestor**: "quanto vou produzir amanhã?" é diferente de "quanto produzo em média?".

- IC da média (já existe): mede o **erro da estimativa**; encolhe com mais replicações.
- **Intervalo de predição** para o valor de uma próxima replicação (um turno, um dia):
  `x̄ ± t(1-α/2; n-1) · s · √(1 + 1/n)`; mede o **risco**; não encolhe com mais replicações.
- **Percentis com IC sem suposição de forma**: o IC do quantil `p` usa estatísticas de ordem,
  `(x_(r), x_(s))` com `r` e `s` pela binomial `B(n, p)` (exato) ou pela aproximação normal;
  avisa quando `n` é pequeno demais para o percentil pedido (P95 com `n < 59` não tem IC de 95 %
  bilateral).
- **Intervalo de tolerância** (opcional): faixa que contém a proporção `P` da população com
  confiança `γ`, para quem pergunta "em 95 % dos dias, com 95 % de confiança".
- Textos padrão para o gestor, revisados com o dono: "precisão da estimativa" (IC) e "faixa
  esperada num período" (predição), com um gráfico que mostra os dois sobre o mesmo eixo.
- **Testes**: cobertura de 95 % por Monte Carlo em amostras normais e exponenciais (a predição
  com t perde cobertura em caudas pesadas; o percentil não), quantis por estatística de ordem
  contra tabela publicada.

### E2. Probabilidade de cumprir uma meta

- `p̂ = #{replicações que cumprem} / n`, com **intervalo de Wilson** (bom para `p` perto de 0 ou
  1 e `n` pequeno); serve ao semáforo (seção 2.3) e à previsão do gêmeo.
- **Risco por pedido** (com rótulo `prazo`): probabilidade de cada pedido sair até o prazo, entre
  replicações, no Gantt de ordens (seção 4, O4).
- **Testes**: Wilson contra valores de livro-texto; cobertura por Monte Carlo.

### E3. Previsão com intervalos e avaliação da previsão

- O leque de previsão (seção 1.4) mostra, por instante, quantis entre as `R` replicações (P5,
  P25, P50, P75, P95) e a média com IC.
- **Avaliação contra o real** (na reprodução de histórico ou ao longo do gêmeo): cobertura
  observada das faixas (a faixa de 90 % contém o real em 90 % das janelas?), histograma da
  transformada integral de probabilidade (PIT: plano se a previsão é bem calibrada), perda de
  quantis (pinball) e CRPS empírico para comparar versões do modelo. Mostra ao gestor um número
  só: "nas últimas 4 semanas, a faixa de 90 % acertou 87 % das vezes".
- **Testes**: previsões geradas pela própria distribuição verdadeira dão PIT uniforme (teste de
  KS) e cobertura nominal.

### E4. Controle estatístico de processo sobre dados reais e simulados

- Gráficos I-MR, X̄-R e X̄-S, p e u (proporção de refugo, contagem de paradas), EWMA e CUSUM
  tabular.
- **Limites a partir do modelo** (o que um simulador acrescenta ao CEP): em vez de estimar os
  limites com 20–25 subgrupos reais, usa-se a distribuição do KPI no modelo validado, entre
  replicações, para o mesmo tamanho de subgrupo; isso dá limites antes de haver histórico e para
  KPIs cuja distribuição não é normal (limites por quantis em vez de ±3σ).
- **Comprimento médio de corrida** (ARL) de cada esquema calculado por simulação: ARL sob
  controle (alarmes falsos) e ARL para um deslocamento dado, para escolher `k` e `h` do CUSUM e
  `λ` e `L` do EWMA com números da própria planta.
- Regras de Western Electric selecionáveis; autocorrelação alta (diagnóstico de `utsim-fit`)
  avisa que os limites clássicos não valem e sugere CEP sobre resíduos ou médias em lotes.
- Sobre dados simulados: o mesmo gráfico aplicado à série de uma replicação longa ajuda a ver
  regime transitório e instabilidade do modelo.
- **Testes**: limites I-MR e X̄-R contra exemplos publicados; ARL do CUSUM e do EWMA contra
  tabelas da literatura (ARL sob controle de 370 para ±3σ em Shewhart como referência).

### E5. Detecção de mudança

- **On-line**: Page–Hinkley e CUSUM (do E4) para mudança de média em séries reais e de
  resíduos real − simulado.
- **Fora de linha** (histórico): segmentação por PELT com custo gaussiano ou de Poisson,
  penalidade BIC; responde "desde quando o tempo de ciclo mudou?" e alimenta a janela da
  calibração (seção 1.6) para não misturar regimes.
- **Bayesiana on-line** (probabilidade de mudança a cada instante): P3.
- **Testes**: séries sintéticas com pontos de mudança conhecidos; taxa de detecção e atraso
  médio; série sem mudança dá zero pontos com a penalidade padrão na maioria das sementes.

### E6. Capacidade

- Cp, Cpk (dentro do subgrupo) e Pp, Ppk (global) de tempos de ciclo, lead time ou medidas,
  contra limites de especificação dados (por exemplo, lead time ≤ 15 min, só limite superior:
  Cpu e Ppu); IC de Cpk por aproximação de Bissell.
- **Não normal**: método dos percentis (`(LSE - P50) / (P99,865 - P50)`) com a distribuição
  ajustada pelo `utsim-fit`; o painel avisa quando o teste de normalidade rejeita.
- **Capacidade do sistema de cumprir o takt**: proporção dos períodos em que o ciclo do gargalo
  fica abaixo do takt, com Wilson (E2).
- Com o gêmeo: capacidade real e simulada lado a lado.
- **Testes**: índices contra exemplos publicados; método dos percentis igual ao clássico em dados
  normais.

### E7. Testes para comparar real e simulado

- Duas amostras: Kolmogorov–Smirnov (com p exato para amostras pequenas), Anderson–Darling
  k-amostras, Mann–Whitney (com correção de empates), t de Welch para médias.
- Contagens: qui-quadrado de aderência à distribuição simulada e teste de razão de taxas de
  Poisson.
- Proporções: diferença de proporções com Newcombe (refugo real × simulado).
- Múltiplos testes: Benjamini–Hochberg (padrão) e Holm.
- Tamanho de efeito sempre ao lado do p-valor (diferença de medianas, δ de Cliff), para que
  "significativo mas irrelevante" seja visível.
- **Testes**: valores publicados de cada teste; p-valores uniformes sob hipótese nula por Monte
  Carlo.

### E8. Relatório de validação modelo × real

- Para um período histórico: tabela de KPIs reais e simulados com IC, diferença, teste, tamanho
  de efeito; gráficos Q-Q real × simulado das distribuições de tempo; veredicto por KPI
  (compatível, diferente mas dentro da tolerância dada, diferente).
- Liga com `docs/validacao.md`: o mesmo relatório serve para os estudos de caso.
- **Testes**: modelo validando contra ele mesmo (gravação de uma rodada) é compatível em todos os
  KPIs.

### E9. Seleção do melhor cenário e gráfico de erro e risco

- **Seleção de subconjunto** (Gupta/Nelson): com `n` replicações por cenário, mantém os cenários
  que podem ser o melhor com probabilidade de seleção correta `1 - α`.
- **Procedimento sequencial de Kim e Nelson** (KN, com zona de indiferença `δ` dada pelo
  usuário): roda replicações em etapas e elimina cenários até sobrar um; usa números aleatórios
  comuns e o "rodar mais k replicações" de RF-159. Integra-se ao `utsim-opt` como etapa final
  ("dos 10 melhores pontos da otimização, qual é o melhor com garantia?").
- **Gráfico de erro e risco**: média com IC (erro), P25–P75 e P5–P95 (risco) e histograma
  lateral, um por cenário, no mesmo eixo.
- **Testes**: configurações de média deslizante com diferenças conhecidas; probabilidade de
  seleção correta empírica ≥ nominal por Monte Carlo.

### E10. Incerteza dos dados de entrada

- Um parâmetro ajustado de 50 observações tem incerteza que o IC entre replicações não mostra.
- **Bootstrap de entrada**: reamostra os dados originais (`examples/dados`, gravações do gêmeo),
  reajusta a distribuição em cada reamostra, simula cada variante com replicações e decompõe a
  variância do KPI em parte estocástica e parte da incerteza de entrada (análise de variância de
  dois níveis). Resultado para o gestor: "a faixa de ±3 peças/h vem ±1 da simulação e ±2,5 da
  pouca quantidade de dados de M2; colete mais dados de M2".
- **Monte Carlo de segunda ordem** para parâmetros sem dados (opinião de especialista como
  distribuição do parâmetro).
- **Testes**: dados sintéticos com `n` crescente mostram a parte de entrada caindo como `1/n`.

### E11. Previsão de demanda e de chegadas

- Chegadas com taxa variável por hora do dia e dia da semana: Poisson não homogêneo estimado das
  chegadas reais (contagens por faixa, com IC), gerando a tabela de taxa por hora (complementa
  "capacidade por hora do dia" do backlog) e alimentando a previsão do gêmeo.
- Tendência e sazonalidade de demanda diária: suavização exponencial de Holt–Winters com
  intervalo de predição, para estudos de capacidade das próximas semanas.
- Teste de sobredispersão (variância/média das contagens) avisa quando Poisson não serve e sugere
  binomial negativa.
- **Testes**: séries sintéticas com sazonalidade conhecida; cobertura das faixas.

### Estimativa e modelo sugerido

| Parte | Horas | Modelo |
|---|---|---|
| E1 predição, percentis por estatística de ordem, tolerância, textos | 3–4 | Opus |
| E2 probabilidade de meta (Wilson), risco por pedido | 2–3 | Opus |
| E3 leque de previsão, cobertura, PIT, pinball, CRPS | 4–6 | Opus |
| E4 CEP com limites pelo modelo e ARL por simulação | 6–8 | Opus |
| E5 Page–Hinkley, PELT | 4–6 | Opus |
| E6 capacidade normal e não normal, takt | 3–4 | Opus |
| E7 testes de duas amostras, contagens, proporções, BH/Holm, tamanho de efeito | 4–5 | Opus |
| E8 relatório de validação | 3–4 | Sonnet |
| E9 seleção de subconjunto, KN, gráfico erro e risco | 6–8 | Opus |
| E10 bootstrap de entrada e Monte Carlo de segunda ordem | 5–7 | Opus |
| E11 Poisson não homogêneo, Holt–Winters, sobredispersão | 4–6 | Opus |
| Abas do painel e textos para o gestor (sobre D3/D5) | 4–6 | Sonnet |
| **Total** | **48–67** | |

### Requisitos sugeridos

| ID | Requisito |
|---|---|
| RF-239 | ver, além do IC da média, o intervalo de predição de um período e percentis com IC sem suposição de forma, com explicação em linguagem do gestor |
| RF-240 | ver a probabilidade de cumprir cada meta, com intervalo, e a probabilidade de cada pedido sair no prazo |
| RF-241 | ver a previsão em leque e a avaliação de previsões passadas (cobertura das faixas, PIT, perda de quantis) |
| RF-242 | aplicar CEP (I-MR, X̄-R, X̄-S, p, u, EWMA, CUSUM) a dados reais e simulados, com limites tirados do modelo e ARL calculado por simulação |
| RF-243 | detectar pontos de mudança em séries, on-line e no histórico |
| RF-244 | calcular índices de capacidade (Cp, Cpk, Pp, Ppk) normais e por percentis, e a capacidade de cumprir o takt |
| RF-245 | comparar amostras reais e simuladas por KS, Anderson–Darling, Mann–Whitney, Welch, qui-quadrado e proporções, com correção de múltiplos testes e tamanho de efeito |
| RF-246 | gerar um relatório de validação do modelo contra um período real |
| RF-247 | selecionar o melhor cenário com garantia estatística (subconjunto e procedimento sequencial), inclusive como etapa final da otimização |
| RF-248 | ver o gráfico de erro e risco por cenário |
| RF-249 | decompor a incerteza de um KPI em variabilidade da simulação e incerteza dos dados de entrada |
| RF-250 | estimar a taxa de chegadas por hora do dia e prever a demanda com intervalos |

---

## 4. Outras ideias da pesquisa, priorizadas

Ordenadas por valor sobre custo. A primeira coluna diz se a ideia entra nesta versão (A), na
seguinte (B) ou fica registrada (C).

| Pri. | Ideia | Como funciona | Horas | Modelo |
|---|---|---|---|---|
| A | **O1 Fantasma do simulado sobre o real no 3D** | no gêmeo, itens e estados simulados desenhados translúcidos sobre os reais (dois `LiveScene` com o mesmo layout); diferenças saltam aos olhos | 4–6 | Sonnet |
| A | **O2 KPIs sobre a cena** | rótulos flutuantes com ocupação, fila e semáforo por objeto; colorir objetos por KPI (ocupação, espera) além de por estado; no Godot, sobre os rótulos que já existem | 3–5 | Sonnet |
| A | **O3 Linha do tempo com marcadores** | quebras, paradas, alertas, correções de WIP e mudanças detectadas como marcas clicáveis na barra de tempo do replay e do gêmeo | 3–4 | Sonnet |
| A | **O6 Painel para quem não modela** | `exposed_params` no arquivo do painel: o gestor muda só esses parâmetros (com faixas), roda um cenário e vê a comparação pareada, sem abrir o editor | 5–7 | Sonnet |
| B | **O4 Agenda com risco por pedido** | pedidos com prazo (rótulo `prazo`) e sequência dada; rodada determinística (variabilidade desligada) gera o Gantt de ordens, e as replicações com variabilidade dão a probabilidade de cada pedido no prazo (E2), marcada no Gantt | 8–11 | Opus |
| B | **O7 Plano do turno** | antes do turno, previsão a partir do estado atual (T5/T6) com a escala de pessoal planejada e alternativas (mais um operador, outra sequência), comparadas pareadas | 5–7 | Opus |
| B | **O8 Sankey de pessoas e de recursos móveis** | fluxo de transportadores e operadores entre pontos (a partir de `Travel` do rastro), no painel e como linhas no piso | 3–4 | Sonnet |
| B | **O5 Modelo a partir de log de eventos** | lê um log de eventos do MES (caso, atividade, recurso, início, fim; CSV ou o formato aberto de log de eventos centrado em objetos), descobre a sequência de atividades e as frequências de desvio, ajusta tempos com `utsim-fit` e gera um `.ron` inicial com fontes, filas, processadores e roteamento por probabilidade; o usuário revisa no editor | 14–20 | Opus |
| C | Experimento de calibração por otimização (seção 1.6, item 7) | `utsim-opt` minimizando a distância a KPIs reais | 5–7 | Opus |
| C | Filtro de partículas para estado e parâmetros | pesquisa; só depois de T4–T8 em uso | 15+ | Opus |
| C | Comparação de sequências de eventos real × simulado | alinhamento de sequências como teste de validação com pouco volume | 6–8 | Opus |
| C | Escrita de recomendações no real | despacho e parâmetros enviados ao CLP; exige análise de segurança própria | — | — |

Itens já no backlog e reforçados pela pesquisa (sem repetir estimativa): comparação de cenários
lado a lado no 3D, mapa de calor (RF-165), energia e emissões por estado, custos por estado,
relatório exportável (coberto pela seção 2.8), vídeo da simulação.

**Total das ideias A e B**: 45–64 h (O1 4–6, O2 3–5, O3 3–4, O6 5–7, O4 8–11, O7 5–7,
O8 3–4, O5 14–20).

### Requisitos sugeridos

| ID | Requisito |
|---|---|
| RF-251 | ver no 3D o simulado translúcido sobre o real no gêmeo |
| RF-252 | ver KPIs e semáforos sobre os objetos da cena e colorir objetos por KPI |
| RF-253 | navegar pela linha do tempo por marcadores de eventos, alertas e correções |
| RF-254 | deixar parâmetros expostos para que quem não modela rode cenários pelo painel |
| RF-255 | ver o Gantt de pedidos com a probabilidade de cada um sair no prazo |
| RF-256 | preparar o turno com uma previsão a partir do estado atual e comparar alternativas de escala |
| RF-257 | ver o fluxo de pessoas e recursos móveis como diagrama de Sankey |
| RF-258 | gerar um modelo inicial a partir de um log de eventos do MES |

---

## 5. Requisitos não funcionais sugeridos

| ID | Requisito |
|---|---|
| RNF-38 | modelos sem `.gemeo.ron`, sem arquivo de painel e sem estado inicial dão resultados idênticos byte a byte aos da versão anterior |
| RNF-39 | uma sessão do gêmeo gravada se reexecuta sem rede e sem relógio com relatório, rastro e previsões idênticos |
| RNF-40 | nenhum conector escreve no equipamento real sem lista explícita e opção na execução; previsão e reprodução nunca escrevem |
| RNF-41 | o laço do gêmeo absorve 2 000 observações por segundo sem atrasar o relógio, e um conector lento ou caído não trava os demais nem a simulação |
| RNF-42 | uma previsão de 40 replicações com horizonte de 4 h de um modelo de 200 objetos termina em menos de 30 s numa máquina de 8 núcleos, sem atrasar o laço principal |
| RNF-43 | o painel atualiza em até 1 s com até 10 navegadores conectados e não exige internet, servidor externo nem conta |
| RNF-44 | todo cálculo estatístico novo tem teste contra valor publicado ou calculado à mão e teste de cobertura ou de taxa de erro por Monte Carlo quando se aplica |

---

## 6. Resumo das estimativas

| Bloco | Núcleo, CLI e servidor | Interfaces (web e Godot) | Total |
|---|---|---|---|
| 1. Gêmeo digital | 91–127 | 8–11 | 99–138 |
| 2. Painel | 19–27 | 44–62 | 63–89 |
| 3. Estatística | 44–61 | 4–6 | 48–67 |
| 4. Outras ideias (A e B) | 30–42 | 15–22 | 45–64 |
| **Total** | **184–257** | **71–101** | **255–358** |

Por modelo, aproximadamente: Opus 137–191 h, Sonnet 116–164 h, Haiku 2–3 h.

---

## 7. Ordem de execução, encaixada nas levas do backlog

Regra do backlog: duas tarefas só rodam juntas se não mexem nos mesmos arquivos. Os pontos de
conflito deste plano são `utsim-core` e `utsim-objects` (T5), `utsim-model/src/lib.rs` (campos
novos: `run.start_at`, `driven_by`, `scrap`, `ideal_cycle`), `utsim-stats` (compartilhada com a
estatística P1–P3) e `utsim-emu/src/runner.rs` (T3/T4).

**Leva 1 (junto com G3 e os itens de núcleo já previstos)**

| Tarefa | Modelo | Depende de | Arquivos |
|---|---|---|---|
| D1 `kpi.rs` (KPIs sobre o que já existe; campos novos ficam para depois) | Opus | nenhuma | `utsim-model/src/kpi.rs` (novo) |
| T1 `utsim-twin` núcleo + conector de arquivo (T2 parte) + gravação | Opus | nenhuma | crates novas `utsim-twin`, `utsim-connect` |
| E1, E2, E6, E7 em `utsim-stats` | Opus | criação da `utsim-stats` (estatística P1 da leva 1) | módulos novos da crate, depois da P1 |
| D2 + D3 servidor e esqueleto da página com dados de `object_stats` | Sonnet | D1 (só o tipo `KpiSet`) | crate nova `utsim-dash` |

**Leva 2 (com a leva 2 do backlog)**

| Tarefa | Modelo | Depende de |
|---|---|---|
| T2 conectores MQTT, Sparkplug B, cliente OPC UA | Sonnet (revisão Opus) | T1 |
| T3 modo sombra + T10 CLI + reprodução de histórico | Sonnet | T1, T2 (arquivo) |
| D4, D5, D8, D9 visões, exportação e configuração do painel | Sonnet | D2, D3 |
| T5 estado inicial no kernel (**feito**, sem mudar o formato do modelo: arquivo à parte) | Opus | formato 2 (ordem do `plano_analises_e_dinamica.md`, item 2), para conferir os golden uma vez; não roda junto com "capacidade por hora do dia" nem com conflitos determinísticos (mesmos arquivos do kernel) |
| E4, E5 CEP e mudança | Opus | E7 |

**Leva 3 (depois da G3 e de T5)**

| Tarefa | Modelo | Depende de |
|---|---|---|
| T4 modo sincronizado | Opus | T3, T5 |
| T6 previsão à frente + E3 | Opus | T5 |
| T8 desvio | Opus | T3, E4, E7 |
| T7 calibração | Opus | T3, `utsim-fit` |
| D6, D7 ao vivo e alertas | Sonnet | D4/D5, T8 |
| T9 segurança, T11 Godot, D10 Godot | Sonnet | T3, G3 |
| O1, O2, O3, O6 | Sonnet | T11, D9 |

**Leva 4**

- E9 seleção do melhor (junto com a otimização de vários objetivos do backlog, que mexe em
  `utsim-opt`: não em paralelo), E10 incerteza de entrada, E11 demanda, E8 relatório de
  validação;
- O4 agenda com risco, O7 plano do turno, O8 Sankey de pessoas;
- O5 modelo a partir de log de eventos (independente, pode adiantar se houver um cliente com
  dados de MES).

Primeira entrega útil ao dono, ao fim da leva 2: painel web completo sobre estudos e simulações
no ritmo do relógio, gêmeo em modo sombra lendo MQTT/OPC UA/histórico, com gravação e reprodução.
Ao fim da leva 3: gêmeo sincronizado com previsão, desvio e calibração, e o 3D mostrando real ×
simulado.

---

## 8. Decisões do dono

**Aprovadas em 2026-11-02:** as 13 escolhas abaixo, nos padrões propostos. Em resumo, o painel
é uma página web servida pelo programa (os principais também no Godot), com gráficos SVG
próprios; sensores e painel em arquivos separados; só leitura do equipamento; Sparkplug B
decodificado a partir da especificação oficial; estado inicial começando por contagens, estados e
tempos decorridos; calibração aplicada com um clique; PDF pela impressão do navegador; `.xlsx` no
instalador padrão; semáforo pela probabilidade (90 % e 50 %); rótulo `prazo` nos itens;
anonimização ligada e retenção de 30 dias; e o estado inicial adiantado na ordem de execução.


1. **D1 Onde roda o painel.** Proposta: página web servida pelo programa (e a mesma página no
   modo navegador e no HTML exportado), com só os cartões e gráficos principais no Godot.
   Alternativa: painel completo dentro do Godot.
2. **D2 Gráficos.** Proposta: módulo SVG próprio. Alternativa: embutir uma biblioteca JS pequena
   de gráficos (mais rápida de fazer, mais um componente de terceiros a acompanhar).
3. **D3 Onde ficam mapeamento e painel.** Proposta: arquivos separados (`.gemeo.ron`,
   `.painel.ron`) ao lado do modelo. Alternativa: campos opcionais `twin` e `dashboard` no
   próprio `.ron`.
4. **D4 Escrita no real.** Proposta: só leitura nesta versão; o gêmeo recomenda, não comanda.
   Alertas em MQTT são a única escrita, e só com lista explícita.
5. **D5 Sparkplug B.** Proposta: `prost` com o `.proto` oficial. Alternativa: decodificador próprio
   sem dependência. A crate pronta encontrada ainda é jovem demais para depender.
6. **D6 Detalhe do estado inicial.** Proposta: níveis 1 e 2 (contagens, estados e idades) na
   primeira entrega; itens identificados (nível 3) depois.
7. **D7 Calibração.** Proposta: sempre sugerida, aplicada só com clique; a atualização contínua
   vale só para a previsão.
8. **D8 PDF.** Proposta: impressão pelo navegador (nada no binário). Alternativa: gerador de PDF
   embutido.
9. **D9 Planilha.** Proposta: `.xlsx` atrás de feature ligada no instalador padrão (dependência
   pequena). Alternativa: só CSV.
10. **D10 Semáforo.** Proposta: pela probabilidade de cumprir a meta (90 % e 50 %). Alternativa:
    pela média contra a meta, com o IC só como informação.
11. **D11 Prazo dos itens.** Proposta: convenção de rótulo `prazo` (instante absoluto) para OTD e
    risco por pedido, sem campo novo no formato.
12. **D12 Dados pessoais.** Confirmar anonimização ligada por padrão e retenção padrão de 30 dias
    nas gravações do gêmeo.
13. **D13 Prioridade.** Confirmar a ordem: painel e gêmeo em sombra primeiro (leva 2), estado
    inicial e previsão depois (leva 3); ou adiantar o estado inicial, que também serve a estudos
    sem gêmeo ("simular a partir de segunda às 6 h").

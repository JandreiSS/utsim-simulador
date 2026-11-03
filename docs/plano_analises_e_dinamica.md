# Plano: análises (PERT/CPM, estatística) e dinâmica do espaço (motorista, desvio, planta CAD, corredores)

Plano técnico de seis recursos pedidos pelo dono, sem implementação. Cada seção traz objetivo,
algoritmos, mudanças no núcleo e no formato do modelo, interface (Godot), testes, riscos,
estimativa em horas Opus 5.5 e requisitos sugeridos (RF-151 em diante; não estão em
`docs/requisitos.md` até o dono aprovar).

## 0. Princípios que valem para todos

- **Cálculo no Rust, desenho no Godot.** Toda regra testável sem motor fica numa crate pura
  (sem Bevy, sem Godot), com testes contra resultados conhecidos. A ponte `crates/utsim-godot`
  devolve dados simples (dicionários e arrays empacotados, coordenadas `(x, z, -y)`), como já
  faz `UtsimPlayback::frame`. Recursos visuais novos vão só para o cliente Godot; o
  `utsim-viewer` (Bevy) fica congelado na versão 0.1, recebendo no máximo correções.
- **Kernel determinístico.** Nada que dependa da taxa de quadros, de física ou de ordem de
  iteração de `HashMap` altera tempos. O que muda tempos passa pelo kernel
  (`utsim-core::engine`), em hooks executados um de cada vez, como o `Traffic` dos AGVs
  (`crates/utsim-objects/src/traffic.rs`). O que é só visual fica no `utsim-scene` e no
  Godot e nunca volta para a simulação.
- **Modelos antigos inalterados.** Todo campo novo do `.ron` é opcional
  (`#[serde(default, skip_serializing_if = ...)]`) e, ausente, desliga o recurso, e os
  snapshots golden de `scripts/verificar` ficam idênticos byte a byte. A única exceção é o modo
  de movimentação (seção 7): para que modelos novos nasçam no modo livre sem mudar os antigos, o
  plano propõe o formato 2 (`FORMAT_VERSION` em `crates/utsim-model/src/lib.rs`), com um passo
  `(1, v1_to_v2)` no gancho `MIGRATIONS` (`migrate`, mesmo arquivo) que grava o comportamento
  atual de forma explícita. Os demais campos novos não exigem migração.
- **Opção explícita para o que muda tempos.** Desvio de obstáculos, pontos de acesso,
  corredores por classe e motorista como recurso só valem quando o modelo pede; sem isso o
  resultado é o de hoje.

---

## 1. PERT/CPM de uma parte do processo

### Objetivo
Depois de montar a fábrica inteira, a pessoa seleciona um trecho (um conjunto de objetos ou
"de A até B") e pede o PERT/CPM dele: rede de atividades, início e fim mais cedo e mais tarde,
folgas total e livre, caminho crítico destacado, distribuição do tempo total e probabilidade de
cumprir um prazo. Substitui o item "PERT/CPM" de `docs/roadmap.md` (seção MVP).

### Como a rede é derivada do modelo
Entrada: `Model` (`objects`, `connections`, `spans`) e a seleção.

1. **Seleção.** Duas formas:
   - conjunto de objetos (a multisseleção do editor, `utsim-edit::arrange`);
   - trecho entre dois pontos, com a mesma semântica de `SpanDef` (`crates/utsim-model/src/spans.rs`):
     todos os objetos que estão em algum caminho de `from` até `to` no grafo de ligações
     (interseção dos alcançáveis a partir de `from` com os que alcançam `to`).
2. **Nós de atividade (atividade no nó, AON).** Um objeto vira uma atividade com duração:

   | Objeto | Duração da atividade | Observação |
   |---|---|---|
   | `Processor` | `setup` (se houver) + `process` | `capacity > 1` não muda a duração de um item |
   | `Combiner` | `setup` + `process` | **junção E**: começa quando a última entrada chega |
   | `Separator` | `process` | **divisão E** quando as saídas vão para ramos distintos |
   | `Conveyor` | `length / speed` | tempo mínimo de percurso |
   | `Rack` | `dwell` | 0 se ausente |
   | `Queue` | 0 no modo clássico; espera medida no modo simulado | ver abaixo |
   | `Source`, `Sink` | 0 | marcos de início e fim |
   | ligação com `transfer` | `delay + distância / speed` | atividade de ligação (aresta vira nó) |
   | ligação com `transport` | viagem vazia estimada + `load_time` + viagem carregada + `unload_time` | distância pelo `Router` (`utsim-objects::network::router`), perfil trapezoidal |
3. **Precedências.** Cada `(from, to)` de `connections` dentro da seleção é uma precedência
   fim-início. Nós sem predecessor na seleção ligam a um marco "início"; sem sucessor, a um
   marco "fim".
4. **Saídas múltiplas que não são divisão.** Num `Processor` ou `Queue` com várias saídas, a
   `OutputRule` (`crates/utsim-core/src/block.rs`) escolhe **um** destino: é um OU, não um E.
   O PERT clássico não representa OU. Regras adotadas:
   - `Weighted`: o usuário escolhe "ramo mais provável" (padrão) ou "cada ramo como cenário";
   - `FirstAvailable`, `RoundRobin`, `Random`, `ByLabel`, `Script`: os ramos são servidores
     paralelos equivalentes; o PERT usa um deles e avisa;
   - só o `Separator` gera um E (o conjunto desmontado segue em paralelo e volta a se juntar
     num `Combiner`). Um `Separator` cujos ramos não convergem é tratado como E até o marco
     "fim".
5. **Ciclos (retrabalho).** Uma ligação de volta gera ciclo, que o CPM não admite. A rede é
   cortada na ligação de volta e a atividade de origem recebe a duração multiplicada pelo
   número esperado de passagens (`1 / (1 - p)`, com `p` vindo de `Weighted`), com aviso. No
   modo simulado o retrabalho aparece naturalmente.
6. **Recursos compartilhados.** O CPM clássico ignora recursos. Operadores (`OperatorUse`),
   transportadores e capacidades aparecem só no modo simulado (abaixo), que mede as esperas
   reais. O relatório diz isso explicitamente para que a pessoa não compare os dois números
   sem saber a diferença.

### Três modos de cálculo
1. **PERT clássico (analítico).**
   - Estimativas `(a, m, b)` por atividade, a partir da `Dist` (`crates/utsim-core/src/dist.rs`):
     `Const(c)` dá `a = m = b = c`; `Triangular` dá os três diretamente; `Uniform` dá
     `m = (a + b) / 2`; as outras usam os quantis de 1 % e 99 % e a moda. Falta hoje um
     `Dist::quantile`: implementar no `utsim-core` (inversa analítica onde existe, bissecção
     com `utsim-fit::special::brent` nas outras) ou reaproveitar as famílias de `utsim-fit`.
   - Duas variantes de momentos: a fórmula clássica `te = (a + 4m + b) / 6`,
     `σ² = ((b - a) / 6)²`, e os momentos exatos da distribuição (`Dist::mean` já existe;
     falta a variância). O padrão é a exata; a clássica fica disponível para o ensino.
   - Passagem para frente (ES, EF) e para trás (LS, LF) em ordem topológica; folga total
     `LS - ES`, folga livre `min(ES dos sucessores) - EF`; caminho crítico = folga total zero
     (com tolerância relativa de `1e-9`), com todos os caminhos críticos quando há empate.
   - `P(T ≤ prazo) = Φ((prazo - μ) / σ)` no caminho crítico (`utsim-fit::special::norm_cdf`),
     com o aviso de que o PERT clássico subestima `T` quando há caminhos quase críticos em
     paralelo (viés da junção).
2. **Monte Carlo sobre a rede.** `N` sorteios (padrão 10 000) das durações com `SimRng`
   (`crates/utsim-core/src/rng.rs`), um stream por atividade, semente do modelo. Por sorteio,
   uma passagem CPM. Resultados:
   - distribuição de `T` (média, IC, quantis 50/80/90/95 %, histograma pelo `Histogram`
     existente) e `P(T ≤ prazo)` com IC binomial;
   - **índice de criticidade** de cada atividade (fração dos sorteios em que está no caminho
     crítico) e de cada caminho;
   - **sensibilidade:** correlação de postos entre a duração da atividade e `T`, e o índice de
     "crucialidade" (criticidade × correlação), em gráfico de tornado.
   É barato (sem kernel) e serve redes de centenas de atividades em menos de um segundo.
3. **Caminho crítico observado na própria simulação.** Roda o modelo inteiro (replicações
   normais) com o rastro ligado (`TraceEvent` em `crates/utsim-core/src/trace.rs`) e, para cada
   unidade que atravessa o trecho:
   - mede, por objeto, espera (entrada até início do trabalho) e trabalho, a partir dos
     eventos `Moved` e `State`;
   - segue a linhagem por `Packed`/`Unpacked` para saber quais componentes formaram cada
     conjunto;
   - em cada `Combiner`, marca como crítica a entrada cujo componente chegou por último (a que
     de fato segurou a montagem); recuando pelos predecessores críticos até o início, obtém o
     caminho crítico daquela unidade.
   O índice de criticidade passa a incluir filas, transportes, operadores, quebras e turnos. A
   distribuição do tempo total é a do `span` do trecho (o mesmo número que um `SpanDef`
   mediria). Este é o modo recomendado para a pergunta "o que atrasa esta parte da fábrica".

### Onde fica no núcleo
- **Crate nova `crates/utsim-pert`**, pura, dependendo de `utsim-core` (Dist, rng, trace,
  stats) e `utsim-model` (Model, grafo, `Router`):
  - `network.rs`: `ActivityNetwork { activities, precedences, milestones, warnings }` e
    `derive(model, selection, options)`;
  - `cpm.rs`: passagens, folgas, caminhos críticos (todos);
  - `pert.rs`: estimativas `(a, m, b)` e momentos, probabilidade de prazo;
  - `monte_carlo.rs`: sorteios, criticidade, sensibilidade (paralelo com `rayon` por blocos de
    sorteios, cada bloco com stream próprio, para manter o determinismo);
  - `observed.rs`: leitura do rastro e caminho crítico observado;
  - `report.rs`: texto, CSV e HTML (seguindo `crates/utsim-model/src/html.rs`).
- **Disposição do diagrama:** `utsim-scene::layout::layers` (já usado no `auto_layout`) dá as
  camadas em ordem topológica; o Rust devolve as coordenadas das caixas e das setas.
- **CLI:** `utsim pert modelo.ron --objetos a,b,c | --de A --ate B --prazo 480
  [--modo classico|montecarlo|simulado] [--csv dir] [--html arquivo]`.

### Formato do modelo
Opcional, para guardar análises que a pessoa quer repetir:

```ron
pert: [(name: "montagem_motor", from: "corte", to: "embalagem", deadline: Some(480.0),
        mode: Simulated, weighted: MostLikely)],
```

Campo novo com `default` e `skip_serializing_if`: não exige passo de migração. Não é lido pela
simulação, como `visual`.

### Interface (Godot)
- Selecionar objetos (G2) e escolher "PERT/CPM da seleção" no menu de contexto, ou "PERT/CPM
  de… até…" clicando em dois objetos.
- Janela com abas:
  - **Rede:** diagrama AON desenhado em `Control` com `draw_*` (a opção de gráficos da G3),
    caixas com ES/EF/LS/LF e folga, caminho crítico em vermelho, espessura da seta pelo
    índice de criticidade;
  - **Gantt:** barras ES→EF, folga em barra clara;
  - **Distribuição:** histograma de `T`, linha do prazo, `P(T ≤ prazo)` com IC;
  - **Sensibilidade:** tornado;
  - **Tabela** e exportação CSV/HTML.
- Na cena 3D, os objetos do caminho crítico ganham contorno vermelho.

### Testes
- Redes de livro-texto com ES/LS/folgas e caminho crítico conhecidos (rede em série, rede com
  dois caminhos, rede com caminhos críticos empatados).
- PERT clássico: `te` e `σ` conferidos à mão para `Triangular` e `Const`.
- Monte Carlo: rede em série converge para a soma das médias; dois ramos iguais em paralelo dão
  criticidade 0,5 ± IC; semente fixa dá resultado idêntico entre execuções e número de threads.
- Modo simulado: linha determinística (`Const`, sem filas) reproduz o CPM exatamente; um
  `Combiner` com um componente mais lento marca esse ramo como crítico em 100 % das unidades.
- Derivação: seleção "de A até B" num modelo com retrabalho gera o aviso e o fator `1/(1-p)`.

### Riscos
- A semântica OU versus E das saídas confunde: o relatório precisa mostrar as hipóteses.
- O modo simulado depende do rastro, que é grande em rodadas longas; ler em fluxo, sem guardar
  tudo, e limitar ao trecho selecionado.
- Linhagem em `Separator` `Split(n)` (cópias, não componentes) precisa de regra própria: as
  cópias são ramos paralelos da mesma unidade.

### Estimativa
| Parte | Horas |
|---|---|
| `utsim-pert`: derivação, CPM, PERT, Monte Carlo, testes | 9–12 |
| Modo simulado pelo rastro, testes | 4–6 |
| CLI, CSV, HTML | 2–3 |
| Godot: menu, rede, Gantt, histograma, tornado | 5–7 |
| **Total** | **20–28** |

### Requisitos sugeridos
| ID | Requisito |
|---|---|
| RF-151 | gerar a rede de atividades de uma seleção de objetos ou de um trecho entre dois objetos, com as hipóteses (E/OU, ciclos) listadas |
| RF-152 | calcular início e fim mais cedo e mais tarde, folga total e livre e todos os caminhos críticos |
| RF-153 | estimar `(a, m, b)` a partir das distribuições do modelo, com momentos clássicos ou exatos, e a probabilidade de cumprir um prazo |
| RF-154 | rodar Monte Carlo sobre a rede com índice de criticidade por atividade e por caminho, distribuição do tempo total e sensibilidade |
| RF-155 | obter o caminho crítico observado na simulação (filas, transportes e recursos incluídos) a partir do rastro |
| RF-156 | ver a rede, o Gantt, o histograma e o tornado na interface, com o caminho crítico destacado também na cena 3D, e exportar CSV/HTML |

---

## 2. Área de estatística avançada

### Objetivo
Ajudar a entender o que está acontecendo e quanto confiar nos números: aquecimento, precisão,
comparação de cenários, verificação de consistência e visões no tempo e no espaço.

### O que já existe
- `Summary::from_samples` com IC de 95 % por t de Student (tabela `t_975` em
  `crates/utsim-core/src/stats.rs`), usado em tudo.
- `utsim-model::analysis`: `object_stats` (entradas, saídas, conteúdo, permanência,
  `utilization`, `utilization_incl_blocked`, `utilization_in_shift`, `utilization_excl_blocked`,
  estados, medidas próprias), `model_stats` (perdas), `bottleneck`, `bottleneck_in_shift`,
  `content_series`, `pooled_histogram`, `paired_difference`, `is_significant`,
  `per_replication`.
- `spans` (tempo entre dois pontos, RF-146), perdas (RF-145), contagens `.count`.
- Cenários com números aleatórios comuns (`experiment.rs::run_scenarios`, mesmas sementes) e
  `utsim-opt` com comparação pareada (`compare`).
- Exportação CSV (`csv.rs`: resumo, replicações, séries, histogramas, objetos) e HTML
  (`html.rs`).
- Aquecimento fixo (`run.warmup`), amostragem do conteúdo (`run.sample_interval`).
- Em `utsim-fit`: autocorrelação, teste de corridas, Mann-Kendall (`diagnostics.rs`) e
  funções especiais (`special.rs`: `ln_gamma`, `beta_inc`, `gamma_p`, `norm_cdf`, `norm_ppf`,
  `brent`). Servem para analisar a saída, não só a entrada.
- `utsim-scene::timeline::Timeline::state_span` (estado de um bloco num instante, a partir
  do rastro).

### O que falta, por prioridade

**P1 (confiança nos números).** *Feito no núcleo e na CLI (`crates/utsim-stats`,
`utsim-model::analysis`, `utsim analisar`) e na aba Análise dos resultados do Godot (G3:
aquecimento com Aplicar, replicações necessárias com Rodar mais, Little com semáforo, gargalo
móvel; as médias em lotes ficam só na CLI).* Notas da
implementação:
- [x] 1: `utsim_stats::t_quantile`/`summary`; em 95 % continua a tabela `t_975` (tornada
  pública), porque as faixas acima de 30 gl da tabela não são o quantil exato e trocá-las
  mudaria resultados. `t_ppf` reproduz a tabela nas três casas (teste de referência).
  `analysis::object_stats_at`, `model_stats_at`, `paired_difference_at`; `--confianca`.
- [x] 2: `warmup.rs` (Welch, MSER/MSER-5) e `analysis::warmup` sobre o WIP amostrado ou as
  observações de uma medida; `Simulation::keep_timed` + `Model::run_replications_timed`.
  A CLI imprime a curva de Welch em texto; o gráfico e o "aplicar" ficam para a interface.
- [x] 3: `precision::replications_needed`/`precision`; `--precisao 2%` ou absoluta.
- [x] 4: `precision::batch_means` (tamanhos dobrando, no mínimo 10 lotes). A autocorrelação
  lag 1 é calculada em `utsim-stats` (mesmo estimador de `utsim-fit::diagnostics`, que não
  pode ser dependência porque `utsim-fit` depende de `utsim-stats`). `--lotes`.
- [x] 5: `little.rs` e `analysis::little`; trânsito de `transfer` pelos eventos `Transit` do
  rastro. Aviso quando o IC da diferença pareada exclui zero **e** a diferença relativa passa
  de 2 % (`DEFAULT_TOLERANCE`): com muitas replicações, efeitos de ponta minúsculos já seriam
  "significativos". Montagens e desmontagens ficam fora (o número de itens muda). `--little`.
- [x] 6: `analysis::active_periods`, `shifting_bottleneck_times`, `shifting_bottleneck`;
  trechos ativos que se tocam no mesmo instante formam um só período. `--gargalo-movel`.

1. **Nível de confiança configurável e quantil t geral.** Trocar a tabela por `t_ppf(p, gl)` a
   partir de `beta_inc` (inversa por `brent`). Padrão continua 95 %, com resultados idênticos
   (a tabela fica como teste de referência).
2. **Detecção de aquecimento.**
   - Welch: média entre replicações da série (conteúdo, WIP ou observações de uma medida), com
     média móvel de janela `w` escolhida pelo usuário; gráfico para escolha visual.
   - MSER-5: agrupa a série em lotes de 5, escolhe o corte `d` que minimiza
     `Σ(x - x̄_d)² / (n - d)²`, rejeitando cortes além da metade da série (sinal de rodada
     curta demais). Resultado: sugestão de `warmup` com um clique para aplicar.
   - Precisa de observações com instante: hoje `Tally::with_raw` guarda valores sem tempo.
     Acrescentar um registro opcional `(t, x)` por medida, ligado só na análise.
3. **Quantas replicações faltam.** Para uma semiamplitude alvo `h*` (absoluta ou relativa à
   média): `n* = min n tal que t(n-1) · s / √n ≤ h*`, iterando a partir do `s` atual; a interface
   oferece "rodar mais k replicações" mantendo as já feitas (as sementes são por replicação, então
   as novas acrescentam sem refazer).
4. **Médias em lotes** para uma rodada longa única: lotes de tamanho crescente até a
   autocorrelação lag 1 dos lotes ficar abaixo de 0,2 (teste de `utsim-fit::diagnostics`), IC
   pelos lotes; aviso quando não há lotes suficientes.
5. **Lei de Little como verificação.** Por objeto e no sistema: `L = λ·W` com `L` = conteúdo
   médio, `λ` = saídas / (tempo - aquecimento), `W` = permanência média; somar os itens em
   trânsito (`transfer`, como já documentado em `docs/arquitetura.md`, seção 4f). Aviso quando a
   diferença relativa passa do IC, o que costuma indicar aquecimento curto ou itens presos no fim.
6. **Gargalo móvel por período ativo.** A partir dos eventos `State` do rastro: período ativo
   = intervalo contínuo em estados de trabalho (`Setup`, `Processing`, `Utilize`, viagens).
   Em cada instante o gargalo é o recurso no período ativo mais longo; sobreposição entre o
   atual e o próximo é gargalo compartilhado. Saída: fração do tempo como gargalo único e
   compartilhado por recurso, complementando `bottleneck` e `bottleneck_in_shift`.

**P2 (entender a dinâmica e comparar)**
7. **Gantt de estados** por objeto (faixas coloridas pelo estado ao longo do tempo), do rastro
   de uma replicação, com zoom; mesma paleta dos estados da cena.
8. **Séries temporais de WIP** (soma dos conteúdos, por grupo de objetos ou do sistema) e
   vazão por janela, média entre replicações com banda de IC, a partir de `content_series`.
9. **Comparação de vários cenários.** ANOVA de um fator (estatística F pela `beta_inc`) e Tukey
   HSD (quantil da amplitude estudentizada por integração numérica, conferido com tabelas
   publicadas); com números aleatórios comuns, comparações pareadas com Bonferroni (já há o par
   em `paired_difference`).
10. **Correlação** entre métricas nas replicações (Pearson e Spearman, matriz com cores) para
    perguntas como "fila longa no posto 2 acompanha vazão menor?".
11. **Mapa de calor de deslocamentos.** Rasteriza os `Travel` e `Transit` do rastro numa grade
    do piso (célula padrão 0,5 m): tempo de ocupação e número de passagens por célula, separado
    por recurso ou classe. Cálculo em `utsim-scene` (já lê o rastro); no Godot, textura no piso.
12. **Exportação por observação.** CSV em formato longo (replicação, instante, objeto, medida,
    valor) e Parquet atrás de uma feature `parquet` (dependências `arrow`/`parquet` são
    pesadas; ficam fora do instalador padrão até haver pedido).

**P3 (experimentos)**
13. **Planejamento de experimentos.** Gera cenários (o formato de `Experiment` já aceita `set`):
    fatorial 2^k, fracionado 2^(k-p) de resolução III/IV/V, hipercubo latino; efeitos principais
    e interações com IC; gráfico de efeitos.
14. **Metamodelo de regressão** sobre os pontos simulados: mínimos quadrados (decomposição QR),
    R², IC dos coeficientes, resíduos; serve de previsão rápida e de sensibilidade (tornado).

### Onde mora o cálculo
- **Crate nova `crates/utsim-stats`**, pura, dependendo só de `utsim-core`:
  - `special.rs` movido de `utsim-fit` (que passa a reexportar) e acrescido de `t_ppf`,
    `f_cdf`, amplitude estudentizada;
  - `warmup.rs` (Welch, MSER-5), `precision.rs` (replicações, médias em lotes), `compare.rs`
    (ANOVA, Tukey, Bonferroni), `little.rs`, `doe.rs`, `regression.rs`, `correlation.rs`.
- **`utsim-model::analysis`** ganha as funções que precisam do `RunReport` ou do rastro
  (gargalo móvel, Gantt, WIP), chamando `utsim-stats`.
- **CLI:** `utsim analisar modelo.ron --aquecimento | --precisao 2% | --little | --gargalo-movel`;
  `utsim experiment` aceita `design: Factorial2k(...)`.

### Interface (Godot, dentro dos resultados da G3)
Aba "Análise" com seções: Confiança (aquecimento com gráfico de Welch e corte MSER, replicações
necessárias com botão "rodar mais"), Consistência (Little, com semáforo por objeto), Dinâmica
(Gantt, WIP, gargalo móvel), Espaço (mapa de calor na cena 3D), Cenários (ANOVA/Tukey, efeitos),
Exportar. Cada número tem um texto curto de "o que isto quer dizer".

### Testes
- MSER-5 numa série sintética com transiente exponencial conhecido acha o corte na faixa
  esperada; série estacionária dá corte perto de zero.
- `t_ppf` igual à tabela `t_975` atual nos 30 primeiros graus de liberdade (4 casas).
- Replicações necessárias: com `s` e `h*` dados, `n*` conferido à mão.
- Médias em lotes: cobertura do IC próxima de 95 % para `W` de um M/M/1 (teoria já usada nos
  testes da fase 0).
- Little exata no M/M/1 e nos modelos de fluxo (`docs/roadmap.md`, fase 6, já valida a lei).
- ANOVA e Tukey contra exemplos publicados de livro-texto; quantis da amplitude estudentizada
  contra a tabela para `α = 0,05`.
- Gargalo móvel: linha de dois postos determinística com períodos ativos calculados à mão.
- Fatorial 2³ com resposta linear sintética recupera os efeitos exatos.

### Riscos
- Guardar observações com instante aumenta memória; ligar só sob demanda e por medida.
- Tukey exige integração numérica cuidadosa; usar a tabela publicada como teste.
- A interface pode virar um depósito de números: cada análise precisa de pergunta clara.

### Estimativa
| Parte | Horas |
|---|---|
| `utsim-stats` P1 (t geral, aquecimento, precisão, lotes, Little) e testes | 8–11 |
| Gargalo móvel, Gantt, WIP (`analysis`) e testes | 4–6 |
| P2: ANOVA/Tukey, correlação, mapa de calor, CSV longo, Parquet opcional | 6–9 |
| P3: planejamento de experimentos e regressão | 5–7 |
| CLI | 2–3 |
| Godot: aba Análise | 8–11 |
| **Total** | **33–47** |

### Requisitos sugeridos
| ID | Requisito |
|---|---|
| RF-157 | escolher o nível de confiança dos intervalos (padrão 95 %, resultados atuais inalterados) |
| RF-158 | sugerir o aquecimento por Welch e MSER-5 e aplicá-lo ao modelo com um clique |
| RF-159 | calcular quantas replicações faltam para uma precisão absoluta ou relativa e rodá-las acrescentando às existentes |
| RF-160 | obter intervalos por médias em lotes numa rodada longa, com teste de autocorrelação dos lotes |
| RF-161 | verificar a lei de Little por objeto e no sistema, com aviso quando não fecha |
| RF-162 | identificar o gargalo móvel pelo método do período ativo (único e compartilhado) |
| RF-163 | ver o Gantt de estados por objeto e a série de WIP com banda de IC |
| RF-164 | comparar vários cenários por ANOVA e Tukey, ou pares com Bonferroni sob números aleatórios comuns |
| RF-165 | ver o mapa de calor de deslocamentos sobre o piso, por recurso ou classe |
| RF-166 | gerar cenários por planejamento fatorial ou hipercubo latino, estimar efeitos e um metamodelo de regressão; exportar observações em CSV longo (e Parquet, opcional) |

---

## 3. Empilhadeira com motorista

### Objetivo
(a) Visual: uma pessoa sentada, animada, dirigindo a empilhadeira. (b) Opcional: a
empilhadeira só anda com um operador de uma equipe, que vai até ela, embarca e desembarca.

### Visual
- **Modelo 3D.** `utsim-assets::vehicles::empilhadeira` ganha um nó âncora `assento` (posição
  e orientação do quadril do motorista) e, se possível, um nó `volante` para girar pela taxa
  de giro. A paleteira (`paleteira`) ganha a âncora `pegador` (operador a pé, empurrando).
- **Clipes.** O catálogo (`assets/terceiros/catalogo.ron`) já mapeia `Driving` →
  `Driving_Loop` e `Sitting` → `Sitting_Idle_Loop` no UAL1 (`quaternius_ual`), e `Pushing` →
  `Push_Loop`. Em `crates/utsim-godot/src/looks.rs`, `Clip` ganha `Drive`, `Sit` e `Push`
  (e `Clip::ALL` cresce; `person_clips()` devolve os nomes novos). Regra:
  `Motion::Moving` → `Drive`; `Idle`, `Blocked`, `Stopped` → `Sit`; `Loading`/`Unloading` →
  `Drive` (mãos nos comandos, garfos pelo `Pose.lift` como hoje).
- **Cena.** `utsim-scene` não precisa de pose nova para o caso só visual: o motorista é um
  acessório do veículo. O `StaticScene` informa, por recurso, `driver: Option<String>` (asset
  da pessoa); o Godot instancia o boneco preso à âncora `assento`, e o `pessoa.gd` recebe o
  clipe calculado pela ponte (`UtsimPlayback::frame`, arrays `clip` e `clip_time`).
- **Formato.** `VisualDef` (`crates/utsim-model/src/lib.rs`) ganha `driver: Option<String>`
  (`Some("operador")` por padrão quando o asset é `empilhadeira`; `Some("")` desliga).
  Campo de apresentação, nunca lido pela simulação.

### Motorista como recurso (opcional)
- **Formato.** `ObjectKind::Transporter` ganha
  `driver: Option<DriverUse { team: String, release: PerTrip | WhenIdle(f64) | Shift }>`.
- **Comportamento** (em `crates/utsim-objects/src/transporter.rs`, reaproveitando o protocolo
  de operadores do `Processor`):
  1. Com tarefa e sem motorista, o transportador fica em `WaitingOperator` (o estado já existe
     em `State`) e pede uma tarefa `Operate { station: <empilhadeira> }` à equipe.
  2. O operador (um `Transporter` pessoa, como hoje) caminha até a posição **atual** da
     empilhadeira e avisa com `MSG_ARRIVED`.
  3. Embarque: o operador entra no estado `Utilize` e deixa de gerar `Travel` próprios; o
     rastro ganha `TraceEvent::Board { carrier, vehicle }` e `Alight { carrier, vehicle, at }`.
  4. A viagem segue normalmente. Liberação conforme `release`: `PerTrip` (desce ao entregar),
     `WhenIdle(t)` (desce depois de `t` ocioso), `Shift` (fica até o fim do turno).
- **Cena.** Entre `Board` e `Alight` o `Timeline` posiciona o operador na âncora do veículo,
  com `Pose.parent = Some(veículo)` e motion `Sitting` (variante nova de `Motion`, ou o par
  `Idle` + parent, a decidir na implementação; a variante nova é mais clara para outros
  motores). O cálculo é feito a partir do rastro, então é determinístico.
- **Métricas.** O operador conta o tempo dirigindo como trabalho (`Utilize`); a empilhadeira
  mostra o tempo em `WaitingOperator`.
- **Impasse.** Operadores que também atendem processadores podem faltar para a empilhadeira e
  vice-versa; a mensagem de impasse existente (`cx.error("impasse: ...")`) cobre o caso de
  espera circular; documentar a recomendação de equipes separadas.

### Interface (Godot)
- Painel de propriedades (G2): "Motorista visível" e, no modo recurso, "Equipe do motorista" e
  "Liberar: por viagem / ocioso por t / fim do turno".
- Câmera "do recurso" (já prevista no Bevy, A3D-4) pode partir do assento.

### Testes
- `looks.rs`: tabela de motion → clipe com `Drive`, `Sit`, `Push`.
- Transportador com motorista: tempo de uma viagem = caminhada do operador + viagem, conferido
  à mão; com `PerTrip`, duas tarefas seguidas geram duas caminhadas; com `Shift`, uma.
- Sem `driver`, golden idênticos.
- Roteiro de captura Godot da empilhadeira com o boneco sentado (comparação visual).

### Riscos
- Encaixe do boneco UAL no assento do modelo gerado por código: ajustar a âncora e a escala.
- O boneco tem 7,6 MB; na VR do Quest (G4) usar a versão reduzida já prevista.

### Estimativa
| Parte | Horas |
|---|---|
| Visual: âncoras, clipes, ponte, Godot, roteiro | 4–6 |
| Recurso: `DriverUse`, protocolo, rastro, `Timeline`, testes | 8–11 |
| **Total** | **12–17** |

### Requisitos sugeridos
| ID | Requisito |
|---|---|
| RF-167 | ver um motorista sentado e animado nas empilhadeiras (e o operador empurrando a paleteira), com opção de esconder |
| RF-168 | exigir um operador de uma equipe para a empilhadeira andar, com caminhada até ela, embarque e desembarque por viagem, por ociosidade ou por turno |
| RF-169 | medir o tempo de espera por motorista e contar o tempo dirigindo na ocupação do operador |

---

## 4. Nenhum objeto passa por dentro de outro; recursos desviam

**Situação (2026-11-03):** a parte (a) está pronta no núcleo: crate `utsim-nav` (áreas ocupadas
pelos tipos e tamanhos, obstáculos da planta e do modelo, folga por papel, grade + A\* com
suavização, pontos de acesso padrão por tipo e `access` no objeto, geração de rede com impressão
digital dos obstáculos em `NetworkDef.generated`), `utsim rede gerar` e os avisos de trecho sobre
obstáculo, ponto de acesso inalcançável e recurso sem caminho. Desvios do texto abaixo: grade +
A\* em vez do grafo de visibilidade (cresce com a área, não com os vértices; ver
`docs/arquitetura.md`); um ponto de acesso por objeto (`access: (x, y)`, sem papel `Pick`/`Drop`
nem vão sorteado do `Rack`), padrão no lado `-y` para todos os tipos; os pontos de acesso valem
nos modos `Free` e `Network` (seção 7) em vez de `run.access_points`. Pendentes: a parte (b) e a
interface no Godot.

### Objetivo
Os recursos móveis contornam máquinas, porta-paletes e paredes, encostam no ponto certo de
cada objeto e, quando dois disputam o mesmo espaço, a disputa é resolvida pela simulação de
forma determinística.

### Estado atual
- Sem rede, a viagem é em linha reta (`StraightLine`, `utsim-core::task`).
- Com rede, menor caminho por Dijkstra pré-calculado (`NetworkRouter`), entrando e saindo pelo
  nó mais próximo em linha reta (o trecho de ligação pode atravessar objetos).
- O destino de uma viagem é a `position` do objeto (o centro).
- Arestas com `capacity` são controladas pelo `Traffic` (FIFO, posse da aresta, impasse).
- Aviso de objeto a mais de 50 m da rede (`Model::warnings`, `crates/utsim-model/src/warnings.rs`).
- As áreas ocupadas existem só no `utsim-scene` (`geometry::Geom::half`, `style::extents`,
  esteiras e porta-paletes pelo tamanho do modelo).

### (a) Obstáculos estáticos e roteamento

1. **Obstáculos.**
   - **Objetos:** retângulo orientado pela área ocupada (`Geom` + `visual.scale` + `visual.yaw`).
     Mover a regra de área de `utsim-scene` para uma função pura acessível ao núcleo (o
     `utsim-scene` já depende de `utsim-model`; a regra vai para `utsim-model` ou para a crate
     nova abaixo, e o `utsim-scene` passa a chamá-la).
   - **Paredes e colunas:** da planta CAD (recurso 5) e de polígonos desenhados à mão
     (`obstacles` no modelo).
   - **Folga:** cada obstáculo é inflado pelo raio do recurso (meia largura do veículo ou
     da carga, mais uma margem padrão de 0,3 m), por classe de recurso (soma de Minkowski de
     polígono convexo com círculo, aproximado por octógono).
2. **Crate nova `crates/utsim-nav`** (pura):
   - **Grafo de visibilidade** sobre os vértices dos obstáculos inflados: caminho mais curto
     exato, determinístico, bom até alguns milhares de vértices.
   - **Grade + A\*** (célula padrão 0,25 m, 8 vizinhos, heurística octil) como alternativa para
     plantas muito detalhadas, seguida de **suavização por linha de visada** (remove vértices
     enquanto o segmento não cruza obstáculo) e, opcionalmente, curvas nos cantos (o
     `utsim-scene::turning` já suaviza o giro no visual).
   - Desempate fixo: menor custo, depois menor índice de vértice; nada de `HashMap` iterado.
   - Testes de interseção segmento × polígono com tolerância fixa.
3. **Integração com a rede existente (decisão central).** O roteador não roda durante a
   simulação. O comando "Gerar rede" do editor produz uma `NetworkDef` comum (nós nos vértices
   úteis e nos pontos de acesso, arestas livres de obstáculos), gravada no `.ron` com
   `generated: true`. A simulação continua usando `NetworkRouter` e `Traffic` como hoje:
   - resultados reprodutíveis mesmo sem o arquivo CAD;
   - a rede gerada pode ser revisada e editada à mão;
   - um hash dos obstáculos guardado junto avisa quando a planta mudou e a rede está velha.
4. **Pontos de acesso por tipo de objeto.** `ObjectDef` ganha
   `access: Option<Vec<AccessPoint { offset: [f64; 2], yaw: f64, role: Pick | Drop | Both }>>`
   (coordenadas locais do objeto). Padrões quando ausente:

   | Objeto | Ponto padrão |
   |---|---|
   | `Rack` | frente do vão (`bay`) da posição sorteada, à distância da folga; a empilhadeira encosta de frente |
   | `Conveyor` | início (entrega) e fim (retirada), pelo lado da esteira |
   | `Processor`, `Combiner`, `Separator` | frente (lado da entrada) e trás (lado da saída) |
   | `Queue` | ponta de saída |
   | `Source`, `Sink` | lado de frente |

   O transportador passa a viajar até o ponto de acesso, e o `Rack` informa o vão do
   `Slot` sorteado para escolher o ponto (o evento `Slot` já existe). Isso muda tempos, então
   só vale com `run.access_points: true` (ou quando a rede é gerada).
5. **Avisos novos** em `Model::warnings`:
   - trecho da rede (ou ligação ponto-a-nó) que cruza a área de um objeto ou parede;
   - ponto de acesso dentro de um obstáculo ou sem caminho até a rede;
   - objeto atendido cujo ponto de acesso não é alcançável pelo recurso que o atende.

### (b) Conflitos entre recursos móveis, de forma determinística

Estende o `Traffic` (`crates/utsim-objects/src/traffic.rs`), que já tem posse de aresta, fila
FIFO e detecção de impasse:

1. **Nós controlados (cruzamentos).** `NodeDef` ganha `capacity: Option<usize>`. Um veículo
   reserva o nó antes de entrar na aresta que termina nele e o libera ao sair. Mesmo protocolo
   das arestas; a detecção de impasse passa a considerar nós.
2. **Zonas.** Polígono com capacidade (ex.: um corredor estreito onde só cabe uma empilhadeira),
   que agrupa as arestas que passam por ele; reservado inteiro.
3. **Distância de segurança em fila.** Arestas longas com `cells: Some(comprimento)` viram uma
   cadeia de sub-trechos de capacidade 1: veículos em sequência mantêm a distância, sem física.
4. **Prioridade por classe** na fila de um recurso de tráfego (pedestres antes de veículos, ou
   o inverso), FIFO entre iguais, com limite de espera para não haver inanição (seção 6).
5. **Tempos.** Reservas e esperas mudam os tempos e só existem quando o modelo declara
   capacidade em nós, zonas ou células. Toda decisão acontece em hooks do kernel, na ordem dos
   eventos.

**O que é só visual** (no `utsim-scene` e no Godot, sem efeito nos tempos):
- deslocamento lateral de duas pessoas que se cruzam numa aresta livre;
- suavização das curvas e do giro;
- frenagem e arranque desenhados nas esperas (a espera em si é da simulação).

### Interface (Godot, G2 e G3)
- Botões "Gerar rede evitando obstáculos" e "Mostrar áreas ocupadas"; obstáculos inflados em
  hachura translúcida; trechos com problema em vermelho, com a mensagem do aviso.
- Edição dos pontos de acesso como alças no objeto selecionado.
- Nós e zonas controlados com a mesma convenção das pistas controladas (vermelho, rótulo
  "capacidade N").

### Testes
- `utsim-nav`: caminho em volta de um quadrado com comprimento exato; corredor estreito mais
  fino que a folga fica fechado; mesmo resultado em execuções repetidas e em ordens diferentes
  de entrada dos obstáculos.
- Rede gerada sem nenhum trecho cruzando obstáculos (propriedade verificada em plantas
  aleatórias com semente fixa).
- Pontos de acesso: empilhadeira no `Rack` chega à frente do vão certo; tempo de viagem
  conferido à mão.
- Cruzamento com capacidade 1 e dois AGVs chegando juntos: o primeiro na ordem de eventos passa,
  o segundo espera o tempo exato; impasse em cruzamento detectado.
- Golden inalterados sem as opções.

### Riscos
- O grafo de visibilidade cresce rápido em plantas com muitas colunas; a grade é a saída, com
  custo de memória.
- Pontos de acesso mudam os tempos de modelos que a pessoa já calibrou; por isso é opção.
- Reservas de nó e zona criam novos impasses; a mensagem precisa mostrar o ciclo na cena.

### Estimativa
| Parte | Horas |
|---|---|
| Área ocupada no núcleo, obstáculos inflados | 3–4 |
| `utsim-nav`: visibilidade, grade + A\*, suavização, testes | 7–9 |
| Geração da `NetworkDef`, hash, avisos | 4–5 |
| Pontos de acesso (formato, transportador, `Rack`), testes | 4–6 |
| `Traffic`: nós, zonas, células, prioridade por classe, impasse, testes | 8–11 |
| Godot: áreas, alças de acesso, geração, avisos na cena | 5–7 |
| **Total** | **31–42** |

### Requisitos sugeridos
| ID | Requisito |
|---|---|
| RF-170 | derivar obstáculos das áreas ocupadas dos objetos, das paredes da planta e de polígonos desenhados, inflados pela folga de cada classe de recurso |
| RF-171 | gerar automaticamente uma rede de caminhos que contorna os obstáculos, gravada no modelo e editável, com aviso quando a planta mudou |
| RF-172 | avisar quando um trecho da rede ou uma ligação cruza um objeto ou parede |
| RF-173 | definir pontos de acesso por objeto, com padrões por tipo (frente do vão do porta-paletes, pontas da esteira) |
| RF-174 | controlar cruzamentos e zonas com capacidade, de forma determinística, com fila, prioridade por classe e detecção de impasse |
| RF-175 | manter distância entre veículos numa mesma aresta por sub-trechos de capacidade 1 |
| RF-176 | distinguir na documentação e na interface o que muda os tempos (reservas) do que é só visual (desvio lateral, curvas) |

---

## 5. Planta CAD como base do layout

**Situação (2026-11-02):** a crate `utsim-cad` e o comando `utsim planta` estão prontos (leitura de
DXF 2D com blocos aninhados, unidades, calibração, simplificação, papéis por camada, obstáculos de
paredes e colunas, SVG e arrays para a interface). Pendentes: encaixe nas linhas (extremidades,
pontos médios, interseções) com índice espacial, contorno de HATCH, o campo `floor_plan` no modelo
(proposta em `docs/arquitetura.md`), a planta por imagem com escala e o diálogo no Godot.
**Atualização (2026-11-03):** `floor_plan` (registro, `Option<Box<FloorPlanDef>>`) e `obstacles`
estão no modelo; `utsim planta arquivo.dxf --obstaculos modelo.ron` grava as paredes e colunas.

### Objetivo
Carregar a planta da fábrica, colocá-la no piso na escala certa e posicionar os objetos sobre
ela; paredes e colunas passam a ser obstáculos do recurso 4.

### Formatos
1. **DXF 2D (principal).** Crate `dxf` (crates.io; lê DXF ASCII e binário de várias versões).
2. **Imagem (PNG/JPG) com escala.** Quad texturizado no piso, calibrado por dois pontos.
3. **PDF.** Primeira versão: a pessoa exporta a página como imagem. Leitura vetorial de PDF fica
   para depois (as bibliotecas disponíveis são grandes ou dependem de binários nativos).
4. **DWG.** Sem leitura direta (formato fechado); o manual orienta converter para DXF no próprio
   programa de CAD ou num conversor gratuito.
5. **3D (etapa futura).** glTF do prédio como malha estática (o `utsim-assets::glb` já carrega
   `.glb`); IFC por conversão para glTF. Na realidade aumentada (G4) a planta ajuda a ancorar a
   cena no piso real.

### Como funciona
- **Crate nova `crates/utsim-cad`** (pura):
  - leitura e achatamento: `LINE`, `LWPOLYLINE`/`POLYLINE` (com bulge), `ARC`, `CIRCLE`,
    `ELLIPSE`, `SPLINE` (aproximada por tolerância), `INSERT` (blocos, recursivo, com
    transformação e atributos), `TEXT`/`MTEXT` (opcional, como rótulos), contorno de `HATCH`;
    tudo vira polilinhas em metros;
  - unidades pelo cabeçalho `$INSUNITS`; se ausente, pergunta (mm é o mais comum em plantas);
  - **calibração por dois pontos:** a pessoa clica dois pontos do desenho e digita a distância
    real; a escala é `distância / |p2 - p1|`; opcionalmente o segundo par de cliques no piso
    define rotação e translação (ajuste de similaridade com dois pares de pontos);
  - simplificação Douglas-Peucker por nível de zoom e índice espacial (grade uniforme ou a crate
    `rstar`) para encaixe e consultas;
  - **encaixe:** extremidades, pontos médios, interseções e ponto mais próximo numa linha, com
    raio em pixels convertido para metros pela câmera;
  - **camadas:** lista com contagem de entidades e caixa envolvente; papel por camada:
    `Fundo`, `Paredes`, `Colunas`, `Corredores` (seção 6), `Ignorar`;
  - **obstáculos:** polilinhas fechadas de `Paredes`/`Colunas` viram polígonos; linhas abertas de
    paredes ganham espessura configurável (padrão 0,15 m). Saída: a mesma lista de polígonos que o
    `utsim-nav` consome.
- **Determinismo.** O arquivo CAD não é lido pela simulação. Ele alimenta a geração da rede e dos
  obstáculos (recurso 4), cujo resultado fica no `.ron`; o hash do CAD guardado avisa quando ele
  mudou.

### Formato do modelo
Campo novo, opcional (não exige passo de migração):

```ron
plant: Some((
    file: "planta/galpao.dxf",          // relativo ao .ron, como visual.asset
    kind: Dxf,                          // ou Image
    units: Millimeters,
    transform: (scale: 0.001, rotation_deg: 0.0, offset: (-12.5, -4.0)),
    calibration: Some((p1: (1000.0, 2000.0), p2: (41000.0, 2000.0), distance: 40.0)),
    layers: [(name: "PAREDES", role: Walls, color: None, visible: true),
             (name: "COTAS", role: Ignore, color: None, visible: false)],
    wall_thickness: 0.15,
    hash: "…",
)),
obstacles: [ (name: "pilar_3", polygon: [(…), (…), (…), (…)]) ],
```

Se um dia `plant` precisar mudar de forma (por exemplo, várias plantas por andar), isso pede
um formato novo (o 3, depois do 2 da seção 7), com o passo correspondente em `MIGRATIONS`.

### Interface (Godot)
- Menu "Planta" > "Importar DXF/imagem": diálogo de camadas (papel, cor, visível), unidades e
  calibração por dois cliques.
- Desenho no piso: linhas em `ArrayMesh` com `PRIMITIVE_LINES` (ou faixas finas para espessura
  constante na tela) a 2 mm acima do piso, uma malha por camada, com nível de detalhe pela
  simplificação do Rust; imagem como `MeshInstance3D` plano com textura.
- Encaixe ao mover objetos e ao desenhar rede, corredores e obstáculos, com marcador do ponto
  encaixado.
- Opacidade da planta e "travar planta" para não movê-la sem querer.

### Testes
- Arquivos DXF pequenos gerados pelos testes (linha, polilinha com bulge, arco, bloco inserido
  com rotação e escala): coordenadas achatadas conferidas à mão.
- Calibração: dois pontos a 40 000 unidades com distância 40 m dão escala 0,001.
- Camadas: contagem e papel; paredes abertas com espessura viram polígonos com área esperada.
- Encaixe: interseção de duas linhas cruzadas encontrada com tolerância.
- Arquivo grande (planta real de dezenas de MB, se o dono puder ceder uma anonimizada): tempo de
  leitura e de simplificação medido (RNF abaixo).

### Riscos
- Plantas reais têm camadas mal organizadas, blocos aninhados, cotas e hachuras pesadas; o
  diálogo de camadas é essencial.
- Unidades ausentes ou erradas são o erro mais provável; a calibração por dois pontos resolve.
- Entidades 3D, `SPLINE` complexas e fontes `SHX` de textos não serão fiéis; textos são
  opcionais.

### Estimativa
| Parte | Horas |
|---|---|
| `utsim-cad`: leitura, achatamento, blocos, unidades, camadas, testes | 6–8 |
| Calibração, simplificação, índice espacial, encaixe | 3–4 |
| Obstáculos a partir de camadas | 2–3 |
| Formato (`plant`, `obstacles`), hash, avisos | 1–2 |
| Godot: importação, diálogo de camadas, desenho, encaixe | 6–8 |
| Imagem com escala | 2–3 |
| **Total** | **20–28** |

### Requisitos sugeridos
| ID | Requisito |
|---|---|
| RF-177 | importar uma planta DXF 2D e vê-la como camada de fundo no piso |
| RF-178 | escolher as camadas visíveis e o papel de cada uma (fundo, paredes, colunas, corredores, ignorar) |
| RF-179 | definir unidades e calibrar a escala por dois pontos com distância conhecida, com rotação e translação |
| RF-180 | encaixar objetos, rede, corredores e obstáculos nas linhas da planta (extremidades, meios, interseções) |
| RF-181 | transformar paredes e colunas da planta em obstáculos do roteamento |
| RF-182 | usar uma imagem com escala como planta alternativa; orientar a conversão de DWG para DXF no manual |

---

## 6. Corredores de circulação e faixas de segurança

Ligada aos recursos 4 (desvio) e 5 (planta CAD).

**Situação (2026-11-03):** pronto no núcleo, sem a interface. `aisles` e `floor_marking`
aditivos no formato 2 (sem nova versão nem migração); perfis Brasil (padrão), Branco e
`Custom`; rede gerada pelas linhas centrais (`utsim_nav::aisle_network`) com cruzamentos e
ligações dos pontos de acesso por classe; modo `Aisles` com roteador por classe e folga;
cruzamentos com parada obrigatória (2 s) e prioridade ao pedestre por reserva em
`utsim_objects::traffic::Crossings`; avisos de largura, obstáculo, alcance e classe sem
corredor; faixas como polígonos (`Model::floor_marks`); `utsim corredores [--svg]`; manual
"Corredores e faixas de segurança". Desvios do texto abaixo: os campos do corredor são `kind`
e `points` (em vez de `class` e `centerline`), `floor_marking` em vez de `marking`; o
`Transporter` não ganhou `class`, `footprint` nem `load_width` (classe pelo papel, `L` pelo
papel ou pela folga); `EdgeDef` não mudou (a classe de cada trecho fica na `AisleNet`); a rede
dos corredores não é gravada no `.ron` (é refeita ao abrir); `crossings` não é um campo editável
(só `crossing_stop`), e só a regra `PedestrianPriority` existe. Pendentes: `floor_marks` de áreas
(armazenagem, a manter livre, incêndio), `speed_limit`, limite de espera e métrica de espera
por cruzamento nos resultados, regras `VehiclePriority`/`Fifo`/`StopBoth`, pedestres livres
atravessando só pelas zebras, aviso de cruzamento sem zebra, interface no Godot.

### Objetivo
- Todos os deslocamentos respeitam paredes e objetos da planta (obstáculos do recurso 4,
  vindos do CAD e das áreas ocupadas).
- A pessoa desenha os caminhos seguros como numa fábrica: corredores de pedestres, de
  empilhadeiras e mistos, com faixas no piso no padrão da indústria.
- Pessoas andam só em corredores de pedestres ou mistos; empilhadeiras, só nos delas ou mistos.
- O corredor desenhado gera a rede de caminhos automaticamente.

### Referências normativas
O programa ajuda a seguir as normas, mas não certifica conformidade: o manual deve dizer que a
responsabilidade é do projeto da fábrica e que o texto vigente das normas deve ser conferido
(as NRs são revisadas com frequência e a numeração dos itens muda).

- **NR-26 (Sinalização de Segurança).** Exige cores na sinalização de segurança nos locais de
  trabalho e, na redação atual, remete às normas técnicas oficiais para a definição das cores. A
  redação original (1978) listava as cores por uso e indicava o branco para passarelas e
  corredores de circulação e para áreas de armazenagem, e o amarelo para sinais de "cuidado".
- **NR-12 (Segurança no Trabalho em Máquinas e Equipamentos).** Na redação de 2019, o capítulo
  de arranjo físico e instalações (itens 12.2.x; 12.6 a 12.8 na redação de 2010) pede áreas de
  circulação demarcadas conforme as normas técnicas oficiais, permanentemente desobstruídas, e
  vias principais de circulação e as que levam às saídas com **largura mínima de 1,20 m**.
- **NR-11 (Transporte, Movimentação, Armazenagem e Manuseio de Materiais).** Trata dos
  equipamentos de transporte motorizados (operador treinado e identificado, sinalização) e da
  armazenagem (afastamento do material empilhado em relação às estruturas laterais do prédio).
  Não fixa largura de corredor de empilhadeira; a largura vem do equipamento e da prática.
- **ABNT NBR 7195 (Cores para segurança).** Define as cores de segurança e seus usos (vermelho
  para combate a incêndio e parada, amarelo para "cuidado", com listras amarelas e pretas para
  riscos de colisão e tropeço, verde para segurança e primeiros socorros, azul para ação
  obrigatória, entre outras). É a referência para os perfis de cor do programa.
- **Referência complementar internacional.** A prática de marcação de piso em fábricas
  (programas de organização do posto de trabalho e de gestão visual) usa amarelo para corredores
  e faixas de trânsito, branco para equipamentos e posições fixas, vermelho para áreas de
  combate a incêndio e de refugo, listras preto e branco para áreas a manter livres, e listras
  amarelo e preto para riscos. As normas internacionais de cores de segurança (ISO 3864) e as
  regras de órgãos de segurança do trabalho de outros países pedem corredores demarcados e
  desobstruídos, com folga para os equipamentos, sem fixar uma largura única de faixa; a faixa
  de 5 a 10 cm é a usual.

### Padrões do programa (perfil inicial "Brasil", configurável)

| Elemento | Padrão inicial | Origem |
|---|---|---|
| Delimitação de corredores | faixa amarela contínua, 10 cm (aceita 5 a 15 cm) | escolha do dono; prática usual; NBR 7195 (amarelo = cuidado) |
| Sentido de mão única | setas amarelas a cada 10 m | prática usual |
| Travessia de pedestres | zebrado amarelo e preto, listras de 40 cm, largura do corredor de pedestres | NBR 7195 (listras amarelo/preto); prática usual |
| Corredor de pedestres | largura mínima 1,20 m (via principal); 0,90 m em passagem secundária (aviso, não erro) | NR-12 (1,20 m); 0,90 m como prática |
| Corredor de empilhadeira, mão única | `L + 2 × 0,50 m`, sendo `L` a maior largura entre veículo e carga | prática usual (configurável) |
| Corredor de empilhadeira, mão dupla | `2L + 3 × 0,40 m` | prática usual (configurável) |
| Corredor misto | soma da exigência de veículo e de pedestre, ou separação física | prática usual |
| Corredor de manobra em porta-paletes | largura de giro do veículo informada no recurso (padrão 3,5 m para contrabalançada) | dado do equipamento |
| Áreas de armazenagem | contorno branco | NR-26 (1978); prática usual |
| Área em frente a equipamento de combate a incêndio | quadrado vermelho com borda, a manter livre | NBR 7195 (vermelho = incêndio) |
| Área proibida ou a manter livre | listras preto e branco (ou vermelho e branco) | prática usual |
| Risco de colisão (colunas, quinas) | listras amarelo e preto | NBR 7195 |

- Perfis prontos: "Brasil (padrão)", "NR-26 1978 (corredores em branco)" e "Personalizado",
  todos editáveis (cores, larguras de faixa, larguras mínimas, espaçamento das setas).
- As larguras mínimas de empilhadeira usam `L` do recurso; o `Transporter` ganha
  `footprint: Option<(largura, comprimento)>` e `load_width: Option<f64>` (padrões pelo asset).

### Formato do modelo
Campos novos, opcionais (sem passo de migração):

```ron
aisles: [
    (name: "corredor_A", class: Forklift, centerline: [(0.0, 0.0), (40.0, 0.0)],
     width: 3.5, two_way: true, speed_limit: Some(2.2)),
    (name: "passeio_1", class: Pedestrian, centerline: [(0.0, 3.0), (40.0, 3.0)],
     width: 1.2, two_way: true),
],
floor_marks: [ (kind: Storage, polygon: [...]), (kind: KeepClear, polygon: [...]) ],
marking: Some((profile: Brasil, line_width: 0.10, overrides: {})),
crossings: [ (at: "corredor_A x passeio_1", rule: PedestrianPriority, stop: 2.0) ],
```

- `EdgeDef` (`crates/utsim-objects/src/network.rs`) ganha `class: Option<TrafficClass>`
  (`Pedestrian`, `Forklift`, `Mixed`; ausente = livre para todos, como hoje) e `aisle:
  Option<String>` (de qual corredor veio).
- `Transporter` ganha `class: Option<TrafficClass>`; o padrão vem do asset (`operador` →
  pedestre; `empilhadeira`, `paleteira`, `agv` → veículo).
- `crossings` é gerado e pode ser editado (regra e tempo de parada).

### Corredor desenhado gera a rede
1. Cada corredor vira uma polilinha de eixo com largura. Nós nos vértices e nas interseções
   entre eixos (interseção segmento × segmento, com tolerância fixa); arestas ao longo do eixo,
   com `two_way` e `class` do corredor; um corredor de mão dupla largo pode gerar duas arestas de
   mão única (uma por faixa) quando a pessoa pede.
2. **Ligações aos pontos de acesso** (recurso 4): para cada ponto de acesso, a ligação mais curta
   até o eixo de um corredor de classe compatível que não cruze obstáculo (`utsim-nav`), como
   aresta curta não controlada. Substitui a regra atual de "nó mais próximo em linha reta" quando
   há corredores.
3. **Roteamento por classe.** Hoje `NetworkRouter` calcula um Dijkstra de todos os pares para a
   rede inteira. Passa a ter uma tabela por classe (subgrafo com as arestas da classe e as
   mistas e livres). Com 2 ou 3 classes o custo de memória é o dobro ou o triplo; aceitável até
   alguns milhares de nós (medir; se preciso, Dijkstra sob demanda com cache determinístico).
4. **Cruzamentos.** Na interseção de corredores de classes diferentes nasce um nó controlado
   (recurso 4b) com regra:
   - `PedestrianPriority` (padrão): o veículo para `stop` segundos antes do cruzamento (parada
     obrigatória, constante, padrão 2 s) e só entra se o nó estiver livre; pedestres esperando
     têm prioridade na fila; limite de espera configurável para o veículo não esperar sem fim
     (depois dele, FIFO);
   - `VehiclePriority`, `Fifo`, `StopBoth`.
   Tudo em hooks do kernel: a ordem é a dos eventos, sem dependência de quadro.
5. **Respeito aos obstáculos.** O corredor (eixo inflado pela largura) é testado contra os
   obstáculos; um corredor que cruza uma parede ou um objeto gera aviso e o trecho não vira aresta
   até ser corrigido.

### Validação (avisos em `Model::warnings`)
- corredor mais estreito que o mínimo do perfil para a sua classe e sentido (com a largura `L`
  do maior veículo que o usa);
- corredor que cruza parede, coluna ou objeto;
- recurso que não alcança um ponto de acesso de um objeto que atende, dentro da sua classe;
- pessoa ou veículo cuja rota atravessa área `KeepClear` ou a área do equipamento de incêndio;
- cruzamento de corredores de classes diferentes sem travessia zebrada (quando a geração
  automática foi desligada).

### Visual no Godot
- Faixas geradas no Rust como malhas finas: polilinha deslocada para os dois lados do eixo (com
  junção em esquadria limitada), triângulos a 2 mm do piso, uma `ArrayMesh` por cor; muito mais
  barato que um `Decal` por trecho em plantas grandes. `Decal` fica como opção de acabamento
  (desgaste, textura) no modo realista.
- Setas de sentido por `MultiMesh` ao longo das arestas de mão única.
- Zebrado como quads alternados no cruzamento, alinhados ao corredor de pedestres.
- Áreas (`floor_marks`) com contorno e, nas proibidas, listras diagonais.
- Edição (G2): desenhar o eixo por cliques com encaixe na planta (recurso 5), alças de largura,
  troca de classe, sentido e perfil.

### Testes
- Geração: dois corredores em cruz geram 5 nós, 4 arestas e um cruzamento com a regra padrão;
  corredor em L gera o canto certo.
- Roteamento por classe: pessoa não usa corredor de empilhadeira mesmo quando é mais curto;
  empilhadeira dá a volta pelo corredor dela; o comprimento conferido à mão.
- Cruzamento com `PedestrianPriority`: empilhadeira chega, para 2 s, espera o pedestre passar;
  tempos exatos; repetição dá o mesmo rastro.
- Validação: corredor de pedestres de 1,0 m gera aviso; corredor de empilhadeira mão dupla com
  `L = 1,2 m` exige `2 × 1,2 + 1,2 = 3,6 m`.
- Malhas das faixas: área de uma faixa reta igual a comprimento × largura; esquadria limitada em
  ângulos agudos.
- Modelos sem `aisles`: golden inalterados.

### Riscos
- Normas: a numeração e os textos mudam; o perfil e o manual devem trazer a data de conferência.
  As larguras de empilhadeira são prática de mercado, não norma; precisam estar marcadas assim.
- Prioridade a pedestres pode gerar esperas longas em cruzamentos movimentados; o limite de
  espera e a métrica de espera por cruzamento ajudam a decidir.
- Corredores mistos estreitos com regra de passagem única viram gargalos que a pessoa pode não
  esperar; o mapa de calor (recurso 2) e o gargalo móvel ajudam a mostrar.

### Estimativa
| Parte | Horas |
|---|---|
| Formato (`aisles`, `floor_marks`, `marking`, `crossings`, classes) e perfis | 3–4 |
| Geração da rede a partir dos corredores, ligações aos acessos | 5–7 |
| Roteamento por classe, cruzamentos com regra, testes | 6–8 |
| Validação (larguras, obstáculos, alcance, áreas) | 3–4 |
| Malhas das faixas, setas e zebrado no Rust (ponte) | 3–4 |
| Godot: desenho e edição de corredores e áreas, perfil | 5–7 |
| **Total** | **25–34** |

### Requisitos sugeridos
| ID | Requisito |
|---|---|
| RF-183 | desenhar corredores de pedestres, de empilhadeiras e mistos, com largura, sentido e limite de velocidade |
| RF-184 | ver as faixas no piso, setas de sentido, travessias zebradas e áreas de armazenagem, proibidas e de equipamento de incêndio, nas cores do perfil escolhido (Brasil por padrão, configurável) |
| RF-185 | gerar a rede de caminhos a partir dos corredores, com ligações aos pontos de acesso sem cruzar obstáculos |
| RF-186 | restringir cada recurso aos corredores da sua classe (pessoas em pedestres e mistos, veículos nos seus e mistos) |
| RF-187 | resolver cruzamentos entre classes por regra determinística (prioridade ao pedestre com parada obrigatória, por padrão), com métrica de espera por cruzamento |
| RF-188 | avisar quando um corredor fica abaixo da largura mínima do perfil, cruza um obstáculo ou quando um recurso não alcança um ponto de acesso |
| RF-189 | manter no manual as normas de referência (NR-26, NR-12, NR-11, NBR 7195) com a data da conferência e o aviso de que o programa não certifica conformidade |

---

## 7. Modos de movimentação dos recursos

**Situação (2026-11-03):** formato 2 com a migração `(1, v1_to_v2)` pronto; `Legacy`, `Free` e
`Network` funcionam, com roteador por recurso (`Transporter::router`, montados uma vez por folga
em `Model::nav_set` e compartilhados pelas replicações) e `utsim check` listando o modo de cada
recurso. Desvios: a migração só grava `movement: Legacy` quando o modelo tem transportadores
(sem eles o modo não significa nada, e o modelo migrado fica igual ao novo); `Aisles` funciona
desde 2026-11-03 (seção 6: pessoas só em corredores de pedestres e mistos, veículos só nos de
empilhadeiras e mistos, roteador por classe e folga, prioridade ao pedestre nos cruzamentos);
AGVs com rede seguem o `Traffic` em qualquer modo; nos modos `Free` e `Network` o roteador é
por folga, não por classe; o cache é por
par exato origem-destino (bits das coordenadas, sem quantização em milímetros, para não depender
da ordem das consultas). Pendentes: `TransferPath::Free`, convivência com corredores e a
interface no Godot.

Os corredores não são a única forma de deslocamento. Cada recurso móvel tem um modo de
movimentação, com um padrão no modelo e sobrescrita por recurso.

### Os modos
| Modo | Como anda | Precisa de |
|---|---|---|
| `Legacy` | como hoje: linha reta sem rede; com rede, nó mais próximo em linha reta e menor caminho na rede | nada (é o comportamento atual) |
| `Free` | menor caminho no piso desviando de paredes e objetos (obstáculos inflados do recurso 4, grafo de visibilidade ou grade + A\*, com suavização) | obstáculos (objetos e, se houver, planta); nenhuma rede |
| `Network` | pela rede de caminhos (`NodeDef`, `EdgeDef`, mão única ou dupla, pistas controladas); ligação ao ponto de acesso pela rota livre mais curta, sem cruzar obstáculo | `network` |
| `Aisles` | só nos corredores da sua classe e nos mistos (rede gerada pelos corredores, seção 6) | `aisles` |

### Formato e migração
- `RunSettings` ganha `movement: Option<Movement>` (o padrão do modelo) e `Transporter` ganha
  `movement: Option<Movement>` (sobrescrita); `class` (pedestre, empilhadeira, outro veículo)
  vem da seção 6.
- **Migração para o formato 2.** Hoje um campo ausente não pode significar ao mesmo tempo
  "comportamento atual" (modelos antigos) e "livre" (modelos novos). Proposta:
  - `FORMAT_VERSION = 2`, com o passo `(1, v1_to_v2)` em `MIGRATIONS`, que acrescenta
    `movement: Some(Legacy)` em `run` e muda `format: 1` para `format: 2`;
  - no formato 2, `movement` ausente vale `Free`;
  - modelos antigos abertos continuam com o comportamento de hoje e são salvos já com
    `movement: Legacy` explícito; modelos novos do editor nascem com `Free` (ou `Network`
    quando a pessoa desenha uma rede antes de qualquer recurso, a confirmar);
  - os golden de `scripts/verificar` rodam o modelo migrado e precisam continuar idênticos;
  - o mesmo passo de migração é o lugar de desligar explicitamente as características físicas
    da seção 8 (que, ausentes, já ficam desligadas).
- `TransferPath` (`crates/utsim-core/src/block.rs`, hoje `Straight` e `Network`) ganha `Free`,
  para que o tempo das ligações diretas com `transfer` também possa desviar de obstáculos.

### Convivência na mesma simulação
- O roteador passa a ser **por recurso**: hoje há um `Arc<dyn Router>` para o modelo
  (`utsim-objects::network::router`); passa a haver um por par (modo, classe), criado uma vez
  em `Model::build` e compartilhado pelos recursos que usam o mesmo par. O trait `Router`
  (`route(from, to) -> Vec<[f64; 3]>`, `crates/utsim-core/src/task.rs`) já é a fronteira certa:
  `NavRouter` (livre) e `ClassNetworkRouter` (rede ou corredores filtrados por classe) o
  implementam.
- Exemplo típico: pessoas em `Free` e empilhadeiras em `Aisles`. Regra para não cruzarem às
  cegas: no modo livre, os corredores de veículos podem ser tratados como obstáculos para
  pedestres com aberturas só nas travessias zebradas (opção `cross_vehicle_aisles: AtCrossings`,
  padrão quando há corredores); o pedestre então passa pelo nó do cruzamento e participa da
  reserva com prioridade (seção 6), de forma determinística.
- Conflito entre dois recursos em modo livre: não há reserva (passam um pelo outro no tempo da
  simulação); o desvio lateral é só visual. Reservas só existem em nós, zonas, células e arestas
  controladas.

### Distância e tempo em cada modo (determinístico)
- **Legacy e Network:** Dijkstra de todos os pares já pré-calculado (`NetworkRouter::new`),
  agora por classe.
- **Aisles:** o mesmo, sobre a rede gerada pelos corredores.
- **Free:**
  - em `Model::build`, uma vez por modelo (não por replicação), monta-se o grafo de navegação
    por classe (obstáculos inflados pela folga da classe);
  - os pontos conhecidos (pontos de acesso, posições `home`, posições iniciais) são
    pré-calculados par a par, com o caminho e o comprimento guardados num `BTreeMap` ordenado;
  - pontos que surgem durante a rodada (parada no meio de uma volta para casa, quebra no meio da
    viagem) são calculados sob demanda e guardados num cache cuja chave é o ponto quantizado em
    milímetros; o resultado é função pura dos dados, então cache e ordem de consulta não mudam
    nada;
  - o cache é compartilhado entre replicações em paralelo por `Arc` com dados imutáveis
    pré-calculados e um cache sob demanda protegido por `Mutex`, sem efeito no resultado.
- Em todos os modos, o tempo é o perfil trapezoidal atual (velocidade, aceleração e
  desaceleração do recurso) sobre o comprimento da polilinha; com curvas, o limite de velocidade
  em curva da seção 8 (raio de giro) entra como trechos de velocidade menor.

### Validação e interface
- `utsim check` e `Model::warnings` listam o modo de cada recurso (`empilhadeira_1:
  movimentação pelos corredores (empilhadeira)`), e avisam quando:
  - o modo pede algo que falta (`Network` sem rede, `Aisles` sem corredores da classe);
  - um ponto de acesso não é alcançável no modo e na classe do recurso;
  - um modelo em `Legacy` tem ligações em linha reta que atravessam objetos (sugere `Free`).
- Godot: o modo aparece no painel do recurso (lista com os quatro modos e "usar o padrão do
  modelo"), como ícone ao lado do nome na lista de objetos e como prévia da rota (polilinha
  colorida pelo modo) ao selecionar o recurso e um destino.

### Testes
- Modelo antigo migrado com `Legacy`: golden idênticos.
- `Free`: rota em volta de uma máquina com comprimento exato; mesma rota em replicações paralelas
  e sequenciais; cache ligado e desligado dão o mesmo rastro.
- Mistos: pessoa livre atravessa o corredor da empilhadeira só pela zebra; empilhadeira fica no
  corredor.
- `TransferPath::Free`: tempo da ligação igual ao comprimento da rota sobre a velocidade.

### Estimativa
| Parte | Horas |
|---|---|
| Formato, migração para o formato 2, testes de migração e golden | 2–3 |
| Roteador por recurso (modo, classe), `NavRouter` com pré-cálculo e cache | 4–6 |
| Convivência (corredores como obstáculos com aberturas, pedestre no cruzamento) | 2–3 |
| Validação e listagem dos modos | 1–2 |
| Godot: seletor de modo, ícone, prévia da rota | 2–3 |
| **Total** | **11–17** |

(O `utsim-nav` em si está contado no recurso 4.)

### Requisitos sugeridos
| ID | Requisito |
|---|---|
| RF-190 | escolher o modo de movimentação (livre, pela rede, pelos corredores, legado) no modelo e por recurso |
| RF-191 | mover recursos no modo livre pelo menor caminho no piso, desviando de paredes e objetos, sem rede nem corredor |
| RF-192 | abrir modelos antigos com o comportamento de hoje (migração para o formato 2 com `movement: Legacy`) e criar modelos novos no modo livre |
| RF-193 | combinar modos na mesma simulação (pessoas livres, empilhadeiras em corredores), com pedestres livres atravessando corredores de veículos só nas travessias |
| RF-194 | ver o modo de cada recurso na validação, na lista de objetos e na prévia da rota |

---

## 8. Características físicas de pessoas e máquinas

### Objetivo
Representar o que muda o desempenho real de pessoas e equipamentos (fadiga, aprendizado,
limites de carga, habilidades, manutenção, desgaste, energia, especificações dos veículos), de
forma determinística, opcional e refletida nas estatísticas.

### Estado atual
- Quebras por `downtime: (mtbf, mttr)` no relógio, tratadas pelo kernel
  (`utsim-core::block::Downtime`, `docs/arquitetura.md` seção 4b), com stream próprio por bloco;
  o reparo não usa recurso.
- Turnos e calendários (`shifts`, `calendars`, `on_end: Finish`), também para transportadores e
  equipes (RF-142, RF-143).
- `Transporter` com `speed`, `acceleration`, `deceleration`, `capacity`, `load_time`,
  `unload_time`; operadores por `OperatorUse` (`crates/utsim-objects/src/processor.rs`).
- Pessoas animadas com o ritmo do clipe pela velocidade (`looks.rs`, `clip_time`).

### Pessoas
Todos os efeitos ficam num estado por pessoa atualizado **só nos eventos** (início e fim de
tarefa, de pausa, de turno), por fórmulas fechadas; nada de passo de tempo.

1. **Fadiga e recuperação.**
   - Modelo simples (padrão quando ligado): **tolerâncias de descanso** da engenharia de
     métodos, um percentual que multiplica os tempos de operação (tabelas de fadiga da OIT,
     *Introduction to Work Study*, Kanawaty (ed.), 4.ª ed., 1992; Niebel e Freivalds,
     *Methods, Standards, and Work Design*). Determinístico e sem estado.
   - Modelo dinâmico: nível de fadiga `F ∈ [0, 1)` com curva exponencial de acúmulo no trabalho,
     `F(t) = 1 - (1 - F0)·e^(-λ·t)`, e de recuperação no descanso, `F(t) = F0·e^(-μ·t)` (forma
     usada por Jaber, Givi e Neumann, "Incorporating human fatigue and recovery into the
     learning–forgetting process", *Applied Mathematical Modelling* 37, 2013; base fisiológica em
     Rohmert (1960) e em Konz, "Work/rest", *International Journal of Industrial Ergonomics*,
     1998). `λ` depende da atividade: trabalho no posto, caminhada vazia, caminhada carregando
     (proporcional à carga em kg e ao ritmo).
   - Efeito: tempo de operação multiplicado por `1 + β·F` no início de cada tarefa; velocidade
     de caminhada multiplicada por `1 - α·F`.
   - Pausas: intervalos de turno e paradas programadas recuperam; opção de micropausa
     automática quando `F` passa de um limite (estado novo `Resting`), com duração para voltar a
     um nível alvo, calculada em forma fechada.
2. **Curva de aprendizado.** Tempo da `n`-ésima repetição `T_n = T_1·n^(-b)`, com taxa de
   aprendizado `r` (padrão 85 %, `b = -log2 r`), Wright (1936), com piso no tempo padrão;
   esquecimento opcional entre turnos (modelo de Jaber e Bonney, 1996). Contagem por pessoa e por
   posto.
3. **Limites de carga manual.** O peso do item vem de um rótulo (`weight`, em kg). Validação:
   - equação revisada de levantamento do NIOSH (Waters et al., *Ergonomics* 36(7), 1993):
     `RWL = 23 kg × HM × VM × DM × AM × FM × CM` e índice de levantamento `LI = carga / RWL`, com
     aviso para `LI > 1` e alerta para `LI > 3`; os fatores vêm do posto (alturas e distâncias
     informadas ou padrão do objeto);
   - NR-17 (Ergonomia): o transporte manual não pode comprometer a saúde ou a segurança e exige
     treinamento; limites legais de referência na CLT (art. 198: 60 kg por empregado; art. 390:
     20 kg em trabalho contínuo e 25 kg ocasional para mulheres). O manual cita as fontes com a
     data da conferência.
4. **Velocidade de caminhada.** Padrão 1,3 m/s vazio (faixa usual 1,2 a 1,5 m/s), reduzida com a
   carga por um fator configurável (padrão: −1 % por kg acima de 5 kg, mínimo 0,8 m/s), mais a
   fadiga.
5. **Habilidades e qualificações.** Cada pessoa da equipe ganha `skills: ["torno", "empilhadeira"]`;
   `OperatorUse` e `DriverUse` (recurso 3) ganham `requires: Option<String>`. O `Dispatcher`
   (`crates/utsim-objects/src/dispatcher.rs`) só entrega a tarefa a quem tem a habilidade, mantendo
   as regras `FirstAvailable`, `RoundRobin`, `Nearest` entre os qualificados; aviso quando
   ninguém da equipe é qualificado.
6. **Variação individual.** Multiplicador de ritmo por pessoa, sorteado no início de cada
   replicação de uma distribuição (padrão `Normal(1, 0,1)` truncada em 0,7 a 1,3) num stream
   próprio da pessoa, para que ligar a variação não mude os outros sorteios.

### Máquinas e veículos
1. **Manutenção corretiva com equipe.** `Downtime` ganha `team: Option<String>` e
   `requires: Option<String>`: na quebra, o kernel pede uma tarefa `Operate` à equipe de
   manutenção (técnicos são `Transporter` pessoas, como os operadores); o estado é `WaitingRepair`
   até o técnico chegar, e o `mttr` conta a partir da chegada. Sem `team`, como hoje.
2. **Manutenção preventiva.** `maintenance: [(trigger, duration, team, cost, policy)]` por
   objeto, com `trigger`:
   - `Every(t)` no relógio, `Calendar("nome")` pelos calendários existentes;
   - `Usage(t)` por tempo trabalhando (estados de trabalho do `StateTracker`);
   - `Cycles(n)` por itens processados.
   `policy: Finish | Interrupt` (como `on_end` dos turnos). Estado `Maintenance`.
3. **Desgaste.** `downtime.basis: Clock | Busy | Cycles` e `downtime.aging: Weibull(shape, scale)`:
   o tempo até a falha é sorteado condicionado à idade `a` acumulada,
   `t = η·((a/η)^β - ln U)^(1/β) - a`, a partir do mesmo stream de quebras. A preventiva zera a
   idade (tão bom quanto novo); a corretiva mantém a idade (reparo mínimo), ou zera, conforme
   `repair: Minimal | AsNew`.
4. **Energia de veículos.** `Transporter.energy: (capacity, per_meter_empty, per_meter_loaded,
   per_kg_meter, per_lift_meter, idle_rate, threshold, station, mode)`:
   - consumo calculado por viagem, em forma fechada, no fim de cada trecho;
   - antes de aceitar uma tarefa, o veículo confere se a energia cobre a tarefa mais a ida ao
     posto; se não, vai recarregar primeiro (estado `Charging`, ou `TravelEmpty` até o posto);
   - objeto novo `ChargingStation { capacity, charge_rate | swap_time }`: fila FIFO de veículos,
     recarga até o nível alvo (tempo em forma fechada) ou troca de bateria em tempo fixo.
5. **Especificação por modelo de empilhadeira.** Catálogo em `utsim-assets` (e sobrescrita no
   recurso): velocidade vazia e carregada, aceleração, raio de giro, capacidade de carga (kg),
   altura máxima de elevação e velocidade do mastro.
   - Validação: item mais pesado que a capacidade; nível do `Rack` (`levels × level_height`)
     acima da elevação máxima; corredor mais estreito que o necessário para o raio de giro
     (seção 6).
   - Tempo: subida e descida dos garfos (`altura / velocidade do mastro`) somadas a carga e
     descarga; limite de velocidade em curva `v = √(a_lat · r)` nos cantos da rota.
6. **Aquecimento e partida.** `startup: (after_idle: t, duration: Dist)`: depois de ocioso ou
   fora de turno por mais de `t`, a máquina passa pelo estado `Warmup` antes da próxima peça;
   também no início de cada turno.

### Estatísticas
- **Pessoas:** fadiga média e máxima, tempo acima do limite, tempo em pausa e em micropausa,
  repetições e tempo médio pela curva de aprendizado, índice de levantamento máximo por posto.
- **OEE** por máquina: disponibilidade (tempo produzindo / tempo planejado, descontando turnos e
  paradas programadas) × desempenho (ciclo ideal × quantidade / tempo produzindo) × qualidade
  (boas / total). Precisa de `ideal_cycle` no objeto (padrão: a média de `process`) e de uma
  forma de marcar refugo: `quality: Reject` numa `Sink` ou numa porta de saída.
- **Manutenção:** MTBF e MTTR observados (dos intervalos `down`), tempo esperando técnico,
  número de preventivas e corretivas, custo (por intervenção, por hora de técnico e de peças).
- **Energia:** recargas por veículo, tempo carregando, fila no posto, energia consumida.
- Todas viram métricas do relatório (`<bloco>.<medida>`), usáveis em experimentos, otimização e na
  estatística avançada (recurso 2).

### Determinismo e compatibilidade
- Cada efeito com sorteio tem stream próprio (variação individual, desgaste), como já é feito com
  as quebras; ligar um não altera os sorteios dos outros.
- Atualização de estado só nos eventos, por fórmulas fechadas: mesma semente, mesmo rastro, em
  qualquer número de threads.
- Tudo ausente por padrão; o passo de migração para o formato 2 (seção 7) não liga nada.

### Visual no Godot
- Barra de fadiga sobre a pessoa (verde a vermelho), ligável na camada de rótulos.
- Ritmo da animação de trabalho dividido por `1 + β·F`; caminhada já segue a velocidade.
  A ponte recebe `fatigue` por pessoa: `Pose` ganha `fatigue: Option<f32>` (campo opcional; o
  protocolo `utsim-scene` passa a uma versão menor nova, e leitores antigos ignoram o campo).
- Empilhadeira indo ao posto de recarga (viagem comum), com indicador de bateria; posto de recarga
  como modelo novo em `utsim-assets`.
- Técnico de manutenção caminhando até a máquina, máquina com ícone de manutenção e cor própria
  dos estados `WaitingRepair`, `Maintenance` e `Warmup`.

### Testes
- Fadiga: com `λ`, `μ` e tempos dados, nível ao fim de cada tarefa conferido à mão; tolerância de
  15 % multiplica exatamente o tempo.
- Aprendizado: `T_8 = T_1 × r³` com `r = 0,85`.
- NIOSH: exemplos resolvidos da publicação (RWL e LI) reproduzidos.
- Habilidades: tarefa só vai para o qualificado; sem qualificado, aviso.
- Weibull condicional: com `β = 1` reduz-se à exponencial sem memória (teste estatístico); MTBF
  observado dentro do IC do teórico; preventiva periódica reduz falhas com `β > 1`.
- Energia: consumo de uma rota conhecida; veículo vai recarregar no ponto previsto; fila no posto.
- OEE: linha determinística com refugo de 10 % e uma parada programada dá os três fatores
  calculados à mão.
- Modelos atuais e migrados: golden idênticos.

### Riscos
- Muitos parâmetros sem dados reais; cada um precisa de padrão defensável, fonte citada e aviso de
  que é estimativa.
- Fadiga e aprendizado interagem com a escolha de quem faz a tarefa (`Nearest`), o que pode dar
  comportamentos difíceis de explicar; o Gantt e as métricas por pessoa ajudam.
- Energia e manutenção com equipe criam novas esperas circulares (técnico que é também operador);
  reaproveitar a detecção de impasse e documentar.

### Estimativa
| Parte | Horas |
|---|---|
| Pessoas: tolerâncias e fadiga dinâmica com pausas | 6–8 |
| Pessoas: aprendizado, carga (NIOSH, avisos), caminhada com carga, variação individual | 4–6 |
| Habilidades no despachante e nos pedidos | 3–4 |
| Manutenção corretiva com equipe e preventiva por tempo, uso, ciclos e calendário | 7–9 |
| Desgaste Weibull com idade | 3–4 |
| Energia e posto de recarga | 7–9 |
| Especificações de empilhadeira (validação, mastro, curvas) | 4–5 |
| Aquecimento e partida | 2–3 |
| Estatísticas (OEE, MTBF/MTTR observados, custo, fadiga, energia) | 4–6 |
| Godot: barra de fadiga, ritmo, posto de recarga, técnico, estados | 5–7 |
| **Total** | **45–61** |

### Requisitos sugeridos
| ID | Requisito |
|---|---|
| RF-195 | aplicar tolerâncias de fadiga aos tempos das pessoas ou um modelo dinâmico de fadiga e recuperação que reduz a velocidade e aumenta os tempos |
| RF-196 | recuperar a fadiga em intervalos de turno, paradas e micropausas automáticas por limite |
| RF-197 | reduzir os tempos pela curva de aprendizado por pessoa e posto, com esquecimento opcional |
| RF-198 | informar o peso dos itens e avisar quando o levantamento manual passa do limite recomendado (equação do NIOSH, NR-17) |
| RF-199 | variar a velocidade de caminhada com a carga e a fadiga |
| RF-200 | definir habilidades por pessoa e exigir uma habilidade para operar uma máquina ou dirigir um veículo |
| RF-201 | sortear um ritmo individual por pessoa em cada replicação, num stream próprio |
| RF-202 | exigir um técnico de uma equipe para o reparo, com espera pelo técnico medida à parte |
| RF-203 | programar manutenção preventiva por relógio, calendário, tempo de uso ou ciclos, com política de terminar ou interromper |
| RF-204 | modelar desgaste com taxa de falha crescente (Weibull pela idade), zerado pela preventiva |
| RF-205 | modelar bateria ou combustível dos veículos, com ida ao posto de recarga ou troca, e o posto como objeto |
| RF-206 | definir por modelo de empilhadeira velocidades, acelerações, raio de giro, capacidade de carga e elevação máxima, com validação contra itens, porta-paletes e corredores |
| RF-207 | exigir aquecimento da máquina depois de ociosidade longa ou no início do turno |
| RF-208 | medir OEE, MTBF e MTTR observados, custo de manutenção, fadiga, pausas e energia, e ver no Godot a fadiga sobre a pessoa, o ritmo mais lento, o técnico indo à máquina e o veículo indo recarregar |

---

## 9. Requisitos não funcionais sugeridos

| ID | Requisito |
|---|---|
| RNF-32 | modelos atuais, abertos pela migração para o formato 2, dão resultados idênticos byte a byte aos da versão anterior (snapshots golden); os campos novos ausentes (`pert`, `plant`, `obstacles`, `access`, `aisles`, `driver`, fadiga, manutenção, energia) desligam o recurso |
| RNF-33 | nada que é só visual (desvio lateral, curvas, faixas, motorista sem `DriverUse`, barra de fadiga) altera tempos; o que altera tempos é decidido em hooks do kernel por fórmulas fechadas, reprodutível com a mesma semente em qualquer número de threads |
| RNF-34 | a simulação não lê o arquivo CAD: obstáculos e rede gerados ficam no `.ron`, com aviso quando a planta muda |
| RNF-35 | uma planta DXF de 20 MB é lida e simplificada em menos de 5 s em release; Monte Carlo PERT de 200 atividades com 10 000 sorteios em menos de 2 s; o modo livre não deixa a rodada mais de 20 % mais lenta que o modo pela rede no mesmo modelo, graças ao pré-cálculo e ao cache |
| RNF-36 | todo cálculo novo (PERT, estatística, navegação, CAD, corredores, características físicas) mora em crate pura com testes contra resultados conhecidos, sem dependência de motor gráfico |
| RNF-37 | todo padrão tirado de norma ou de literatura (cores, larguras, NIOSH, tolerâncias, velocidades) traz a fonte no manual e na interface, com a data da conferência |

---

## 10. Resumo das estimativas

| Recurso | Núcleo e CLI | Godot | Total |
|---|---|---|---|
| 1. PERT/CPM da seleção | 15–21 | 5–7 | 20–28 |
| 2. Estatística avançada | 25–36 | 8–11 | 33–47 |
| 3. Empilhadeira com motorista | 9–13 | 3–4 | 12–17 |
| 4. Desvio e conflitos | 26–35 | 5–7 | 31–42 |
| 5. Planta CAD | 14–20 | 6–8 | 20–28 |
| 6. Corredores e faixas | 20–27 | 5–7 | 25–34 |
| 7. Modos de movimentação | 9–14 | 2–3 | 11–17 |
| 8. Características físicas | 40–54 | 5–7 | 45–61 |
| **Total** | **158–220** | **39–54** | **197–274** |

## 11. Ordem sugerida de execução

Contexto: G2 (edição) e G5 (empacotamento) em andamento; G3 (rede, fluidos, resultados) e G4
(VR/AR) depois. Recursos visuais novos só no Godot.

1. **Agora, em paralelo com G2/G5 (só núcleo, sem interface):**
   - [x] `utsim-stats` P1 (quantil t geral, aquecimento, replicações necessárias, médias em lotes,
     Little) e a CLI `utsim analisar`; não toca na interface nem no formato. Feito, com o
     gargalo móvel (item 6 da seção 2).
   - `utsim-pert` (derivação, CPM, PERT, Monte Carlo) e `utsim pert` com HTML; dá valor imediato
     pela linha de comando e pelo relatório.
2. [x] **Formato 2 cedo.** A migração (seção 7, `movement: Legacy`) é pequena, mas toca todos os
   modelos e exemplos; fazê-la antes de qualquer recurso que mude tempos evita duas migrações e
   deixa os golden conferidos uma vez só. Combinar com a G2, que já grava modelos pelo Godot.
3. [x] **Ainda no núcleo:** `utsim-cad` (leitura, camadas, calibração, obstáculos) e `utsim-nav`
   (obstáculos inflados, visibilidade, A\*), com os avisos de cruzamento de objeto, e o modo
   livre com roteador por recurso (seção 7). Independem da interface e destravam 4, 5, 6 e 7.
4. **Motorista visual** (recurso 3, parte visual): pequeno, depende só da ponte da G1; pode
   entrar junto com o fim da G2.
5. **Com a G2 pronta (edição no Godot):** importação da planta com encaixe, desenho de corredores
   e áreas, pontos de acesso, geração da rede, seletor do modo de movimentação (recursos 4a, 5, 6
   e 7 na interface). A G3 já previa a edição da rede; corredores e rede gerada entram como parte
   dela.
6. **Na G3 (resultados):** aba Análise (estatística P1 e P2), janela PERT/CPM, mapa de calor,
   Gantt de estados; o modo simulado do PERT e o gargalo móvel entram aqui, porque dependem do
   rastro e dos gráficos.
7. **Depois da G3:** conflitos determinísticos (nós, zonas, células, prioridade por classe),
   cruzamentos com regra e motorista como recurso (`DriverUse`); em seguida, as características
   físicas, começando pelas de máquinas (manutenção com equipe, preventiva, desgaste, OEE, que
   reaproveitam o kernel de quebras) e depois energia, especificações de empilhadeira e as de
   pessoas (habilidades, fadiga, aprendizado, carga). São as partes que mudam tempos e pedem mais
   validação.
8. **Por último:** estatística P3 (planejamento de experimentos, metamodelo), Parquet, planta por
   imagem e, na G4, a planta como referência para ancorar a realidade aumentada e o glTF do
   prédio.

## 12. Decisões do dono

**Aprovadas em 2026-11-02, como propostas abaixo:**

- Formato 2, com os modelos atuais migrados para `movement: Legacy`, os novos nascendo em `Free`
  e a rede gerada gravada no `.ron`.
- Perfil de piso "Brasil" com faixa amarela de 10 cm, larguras `L + 1,0 m` (mão única) e
  `2L + 1,2 m` (mão dupla) e prioridade ao pedestre com parada de 2 s; perfil branco disponível.
- A janela do PERT abre no caminho crítico observado na simulação, com o CPM clássico e o Monte
  Carlo a um clique.

Conferência das normas (pendente antes de publicar no manual): a NR-26 vigente é a redação da
Portaria MTP nº 2.770, de 5 de setembro de 2022 ("Sinalização e Identificação de Segurança"), e
as cores de segurança estão na ABNT NBR 7195:2018. Fontes secundárias divergem entre branco e
amarelo para corredores de pedestres, e o valor de largura mínima das vias da NR-12 não foi
confirmado no texto oficial. Por isso os dois perfis de cor continuam no programa, e os números
só entram no manual depois da leitura do texto oficial.

Texto original das decisões:

1. **Modo de movimentação padrão e formato 2.** O plano propõe subir o formato para 2, com os
   modelos atuais migrados para `movement: Legacy` (comportamento de hoje) e os modelos novos
   nascendo em `Free` (menor caminho no piso desviando de obstáculos). Junto disso, a rede gerada
   por obstáculos ou corredores fica gravada no `.ron` (reprodutível sem o CAD, com aviso pelo
   hash), em vez de regenerada a cada rodada.
2. **Padrões de faixas e larguras.** Confirmar o amarelo como cor padrão dos corredores (a
   redação original da NR-26 indicava branco; o plano deixa os dois perfis), a faixa de 10 cm e as
   fórmulas de largura de corredor de empilhadeira (`L + 1,0 m` mão única, `2L + 1,2 m` mão dupla),
   que são prática de mercado e não norma; e a regra padrão dos cruzamentos (prioridade ao
   pedestre com parada de 2 s).
3. **O que o PERT considera atividade por padrão.** Só os tempos de trabalho (CPM clássico, o
   que se ensina) ou trabalho mais esperas, transportes e recursos (caminho crítico observado na
   simulação, o que responde "o que atrasa esta parte da fábrica"). O plano oferece os três modos;
   falta escolher o que abre primeiro na janela.

Outras escolhas menores, com padrão já proposto no texto: modelo de fadiga padrão (tolerâncias
fixas antes do dinâmico), taxa de aprendizado de 85 %, Parquet fora do instalador padrão.

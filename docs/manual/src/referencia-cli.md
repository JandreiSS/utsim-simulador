# Comandos da linha de comando

O programa se chama `utsim`. Todos os comandos aceitam `--help`. Códigos de saída: 0 em caso de sucesso e 1 em caso de erro (a mensagem vai para a saída de erro, começando com `erro:`).

| Comando | Para que serve | Capítulo |
|---|---|---|
| `utsim run` | roda um modelo e mostra as estatísticas (também a partir de um estado, e grava o estado num instante) | [Resultados](resultados.md), [Simular a partir de um estado](estado-inicial.md) |
| `utsim check` | valida um modelo sem rodar e mostra o modo de movimentação de cada recurso | [Formato `.ron`](referencia-ron.md), [Movimentação](movimentacao.md) |
| `utsim analisar` | aquecimento, replicações necessárias, médias em lotes, lei de Little, gargalo móvel, comparação de cenários, correlação, mapa de calor e exportação das observações | [Resultados](resultados.md) |
| `utsim replay` | grava o rastro de uma replicação (entrada do visualizador) | [Primeiros passos](primeiros-passos.md) |
| `utsim cena` | grava a cena 3D para outros motores | [3D realista](visual-3d.md) |
| `utsim fit` | ajusta distribuições a dados | [Ajuste de distribuições](ajuste-de-distribuicoes.md) |
| `utsim dados` | ajusta os campos do modelo alimentados por dados reais e gera os relatórios | [Dados reais e ajuste automático](dados-reais.md) |
| `utsim experiment` | compara cenários | [Experimentos e otimização](experimentos-e-otimizacao.md) |
| `utsim otimizar` | procura os melhores parâmetros | [Experimentos e otimização](experimentos-e-otimizacao.md) |
| `utsim emular` | emula em tempo real com OPC UA | [Emulação OPC UA](emulacao-opcua.md) |
| `utsim gemeo` | gêmeo digital em modo sombra: lê sensores (OPC UA, MQTT, Sparkplug B), só leitura, compara com o modelo, grava; `reproduzir` reexecuta sem rede | [Gêmeo digital](gemeo-digital.md) |
| `utsim fluxograma` | desenha o fluxograma do processo e a carta de processo | [Fluxograma do processo](fluxograma.md) |
| `utsim gerar-modelo` | gera modelos sintéticos grandes (testes de desempenho) | `docs/desempenho.md` |
| `utsim pert` | PERT/CPM de uma seleção de objetos ou de um trecho | [PERT/CPM](pert-cpm.md) |
| `utsim qualidade` | cartas de controle, capacidade e análise do sistema de medição sobre um CSV | [Qualidade](qualidade.md) |
| `utsim planta` | lê uma planta DXF 2D (camadas, calibração, SVG) | [Planta CAD](planta-cad.md) |
| `utsim planta` | lê uma planta DXF 2D (camadas, calibração, SVG) e grava paredes e colunas como obstáculos do modelo | [Planta CAD](planta-cad.md) |
| `utsim rede gerar` | gera a rede de caminhos pelo caminho livre, desviando dos obstáculos, e grava no modelo | [Movimentação e obstáculos](movimentacao.md) |
| `utsim corredores` | resumo dos corredores, da rede gerada e dos cruzamentos, avisos e planta em SVG (`--svg`) | [Corredores e faixas de segurança](corredores.md) |

O `utsim run` e o `utsim check` imprimem, antes do resultado, **avisos** (`aviso: ...`) sobre coisas válidas mas provavelmente não pretendidas, como ligações longas e instantâneas. Eles não impedem a rodada.

## Exemplos

```bash
utsim check examples/mm1.ron
utsim run examples/mm1.ron -n 20 --json resultado.json
utsim run examples/mm1.ron --sample 100 --bins 20 --csv saida/
utsim run examples/turnos.ron --estado examples/estados/turnos_21h30.ron
utsim run examples/turnos.ron --salvar-estado 480 estado_14h.ron
utsim analisar examples/mm1.ron --precisao 2% --little --confianca 90%
utsim replay examples/mm1.ron --out replay.json
utsim fit examples/dados/tempos_atendimento.csv --html relatorio.html
utsim dados examples/dados_reais.ron --relatorio relatorios_dados
utsim experiment examples/experimento_mm1.ron
utsim otimizar examples/otimizacao_servidores.ron --html relatorio.html
utsim emular examples/linha_com_bloqueio.ron --velocidade 10
utsim gemeo examples/gemeo/linha.ron --gravar gravacoes --painel 8080
utsim gemeo reproduzir gravacoes/ --json relatorio.json
utsim cena examples/agvs.ron --fps 10 --ate 600 --saida agvs.jsonl
utsim fluxograma examples/centro_de_distribuicao.ron --html fluxograma.html --com-resultados
utsim gerar-modelo --objetos 100 --tipo linha --itens 1000 --saida grande.ron
utsim pert examples/montagem_paletes.ron --de paletes --ate saida_caixas --prazo 10
utsim qualidade carta examples/qualidade/diametros.csv --tipo xbarr --regras we --html carta.html
utsim qualidade capacidade examples/qualidade/lead_time.csv --lse 18 --metodo todos
utsim qualidade msa examples/qualidade/grr.csv --tolerancia 6
utsim planta galpao.dxf --obstaculos modelo.ron
utsim rede gerar modelo.ron --usar
utsim corredores modelo.ron --svg planta.svg
```

## Observações sobre as opções

- `run`: as opções `-n`, `--seed`, `--stop`, `--warmup`, `--sample` e `--bins` substituem os campos de `run` do arquivo, só nesta execução. Com `--estado`, a rodada parte do estado do arquivo, sem aquecimento a menos que o estado tenha `warmup` ou que se use `--warmup`. `--salvar-estado T ARQUIVO` grava o estado da replicação 0 em `T` antes da rodada pedida.
- `analisar`: `-n` e `--stop` substituem `run.replications` e `run.stop_time`. Sem nenhuma das opções de análise, roda aquecimento, precisão a 2%, Little e gargalo móvel. `--comparar experimento.ron` (ANOVA, Tukey, Bonferroni), `--correlacao`, `--mapa-calor arquivo.svg` (com `--celula` e `--por recurso|classe`) e `--exportar-observacoes arquivo.csv` só rodam quando pedidas.
- `replay` e `cena`: `--rep` escolhe a replicação (a primeira é a 0).
- `gerar-modelo`: `--tipo` aceita `linha`, `rede` e `armazem`.
- `check`: depois de `ok`, uma linha `movimentação: recurso: modo (papel, folga)` por recurso móvel.
- `rede gerar`: sobrescreve o modelo (ou grava em `--saida`), mantendo os comentários do topo; `--folga` troca a folga (padrão: a maior entre os recursos); `--usar` grava `movement: Network` em `run`; uma rede desenhada à mão só é trocada com `--substituir`.
- `pert`: use `--objetos` ou `--de` com `--ate`. O modo padrão é `simulado`; `-n` só vale
  para ele e `--sorteios` só para `montecarlo`.
- `gemeo`: o arquivo do gêmeo é `<modelo>.gemeo.ron` (ou `--config`); `--ate` é o tempo do gêmeo em que a sessão termina (sem ele, até Ctrl+C); `--estado` grava o estado observado como estado inicial; `reproduzir` aceita a pasta de uma sessão ou a pasta de gravações (usa a mais recente).
- `qualidade`: tem quatro subcomandos (`carta`, `capacidade`, `msa`, `arl`), cada um com o seu `--help`. Na `carta`, `--excluir` e `--revisar` são da fase I e `--limites` da fase II; `--regras` aceita `1`, `we`, `nelson` ou uma lista como `1,2,5`. Na `capacidade`, `--deslocamento` é o deslocamento somado ao Z no nível sigma (1,5 por convenção).
- `fluxograma`: `--tipo` escolhe entre dez diagramas (`carta`, `blocos`, `operacoes`, `raias`, `sipoc`, `mapofluxograma`, `espaguete`, `de-para`, `homem-maquina`, `vsm`) ou `todos` (índice em `--html`); `--replica`, `--de`/`--ate`, `--comparar` e `--raia` ajustam os tipos da simulação e das raias; `--svg`, `--html`, `--mermaid` e `--csv` escolhem as saídas (sem nenhuma, imprime o Mermaid); `--simbolo objeto=simbolo` troca o símbolo de um objeto; `--horizontal`, `--sem-tempos` e `--sem-distancias` ajustam o desenho; `--com-resultados` roda o modelo e põe os números nas caixas.

## Saída de `--help`

Esta é a ajuda de cada comando, copiada da versão atual do programa.
### `utsim --help`

```text
UTSim: simulador de eventos discretos

Usage: utsim <COMMAND>

Commands:
  run           Executa um modelo e mostra as estatísticas
  check         Valida um modelo sem executar
  analisar      Análises de saída: aquecimento, precisão, lotes, lei de Little e gargalo móvel
  replay        Grava o rastro completo de uma replicação para o visualizador 3D
  cena          Grava a cena 3D de uma replicação (protocolo utsim-scene/1, JSON em linhas)
  gerar-modelo  Gera um modelo sintético de qualquer tamanho (testes de desempenho e de escala)
  experiment    Compara cenários de um arquivo de experimento
  otimizar      Otimiza parâmetros do modelo por simulação (busca com comparação pareada)
  emular        Emula o modelo em tempo real, com servidor OPC UA (comissionamento virtual)
  fit           Ajusta distribuições a dados reais (CSV/TXT) e justifica a escolha
  pert          PERT/CPM de uma parte do modelo (seleção de objetos ou trecho de A até B)
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### `utsim run --help`

```text
Executa um modelo e mostra as estatísticas

Usage: utsim run [OPTIONS] <MODEL>

Arguments:
  <MODEL>  

Options:
  -n, --reps <REPS>                  Número de replicações (sobrescreve run.replications)
      --seed <SEED>                  
      --stop <STOP>                  Tempo de parada (sobrescreve run.stop_time)
      --warmup <WARMUP>              Tempo de aquecimento (sobrescreve run.warmup)
      --json <JSON>                  Grava os relatórios completos em JSON
      --csv <CSV>                    Grava resumo, replicações, séries e histogramas em CSV neste diretório
      --sample <SAMPLE>              Intervalo de amostragem das séries temporais (sobrescreve run.sample_interval)
      --bins <BINS>                  Número de classes dos histogramas (sobrescreve run.histogram_bins)
      --estado <ARQUIVO>             Parte deste estado inicial (arquivo RON) em vez do sistema vazio
      --salvar-estado <T> <ARQUIVO>  Grava o estado da replicação 0 no instante T neste arquivo, para encadear rodadas
  -h, --help                         Print help
```

### `utsim check --help`

```text
Valida um modelo sem executar

Usage: utsim check <MODEL>

Arguments:
  <MODEL>  

Options:
  -h, --help  Print help
```

### `utsim rede gerar --help`

```text
Gera a rede de caminhos pelo caminho livre (desviando dos obstáculos) e grava no modelo

Usage: utsim rede gerar [OPTIONS] <MODEL>

Arguments:
  <MODEL>

Options:
      --saida <SAIDA>  Grava neste arquivo em vez de sobrescrever o modelo
      --folga <FOLGA>  Folga em metros (padrão: a maior entre os recursos móveis)
      --usar           Também muda `run.movement` para Network (os recursos passam a usar a rede)
      --substituir     Substitui uma rede desenhada à mão (sem isso, só uma rede gerada é trocada)
  -h, --help           Print help
```

### `utsim corredores --help`

```text
Corredores de circulação: resumo, avisos e planta em SVG com as faixas

Usage: utsim corredores [OPTIONS] <MODEL>

Arguments:
  <MODEL>  

Options:
      --svg <SVG>  Grava a planta com os corredores, as faixas e a rede gerada em SVG
  -h, --help       Print help
```

### `utsim analisar --help`

```text
Análises de saída: aquecimento, precisão, lotes, lei de Little e gargalo móvel

Sem nenhuma opção de análise, roda todas menos as médias em lotes. Comparação de cenários,
correlação, mapa de calor e exportação das observações só rodam quando pedidas.

Usage: utsim analisar [OPTIONS] <MODEL>

Arguments:
  <MODEL>  

Options:
      --aquecimento                 Sugere o aquecimento (Welch e MSER-5) a partir do WIP ou de --medida
      --precisao <ALVO>             Replicações necessárias para esta semiamplitude: 2% (da média) ou 0.5 (absoluta)
      --lotes                       Intervalo por médias em lotes numa rodada única (padrão: tempo no sistema)
      --little                      Verifica a lei de Little por objeto e no sistema
      --gargalo-movel               Gargalo móvel pelo período ativo (único e compartilhado)
      --confianca <CONFIANCA>       Nível de confiança dos intervalos [default: 95%]
      --medida <MEDIDA>             Medida para aquecimento e lotes: objeto.medida (ex.: saida.time_in_system)
      --metrica <METRICA>           Métrica para --precisao, --comparar e --correlacao (repetível; padrão: ocupações, conteúdos e tempos)
      --janela <JANELA>             Janela da média móvel de Welch [default: 5]
  -n, --reps <REPS>                 Número de replicações (sobrescreve run.replications)
      --stop <STOP>                 Tempo de parada (sobrescreve run.stop_time)
      --json <JSON>                 Grava os resultados em JSON
      --comparar <EXPERIMENTO>      Compara os cenários de um experimento: ANOVA, Tukey e pares com Bonferroni
      --correlacao                  Correlação (Pearson e Spearman) entre as métricas nas replicações
      --mapa-calor <SVG>            Grava o mapa de calor de deslocamentos da replicação 0 em SVG
      --celula <CELULA>             Lado da célula do mapa de calor, em metros [default: 0.5]
      --por <POR>                   Camadas do mapa de calor: recurso ou classe [default: recurso]
      --exportar-observacoes <CSV>  Grava cada observação em CSV longo (replicação, instante, objeto, medida, valor)
  -h, --help                        Print help
```

### `utsim replay --help`

```text
Grava o rastro completo de uma replicação para o visualizador 3D

Usage: utsim replay [OPTIONS] <MODEL>

Arguments:
  <MODEL>  

Options:
  -o, --out <OUT>  Arquivo JSON de saída [default: replay.json]
      --rep <REP>  [default: 0]
  -h, --help       Print help
```

### `utsim cena --help`

```text
Grava a cena 3D de uma replicação (protocolo utsim-scene/1, JSON em linhas)

Usage: utsim cena [OPTIONS] <MODEL>

Arguments:
  <MODEL>  

Options:
      --rep <REP>      [default: 0]
      --fps <FPS>      Quadros por unidade de tempo simulado [default: 10]
      --ate <ATE>      Último instante gravado (padrão: o fim da rodada)
      --saida <SAIDA>  Arquivo de saída (padrão: a saída padrão)
  -h, --help           Print help
```

### `utsim fit --help`

```text
Ajusta distribuições a dados reais (CSV/TXT) e justifica a escolha

Usage: utsim fit [OPTIONS] <FILE>

Arguments:
  <FILE>  

Options:
      --coluna <COLUNA>        Coluna: nome do cabeçalho ou posição (a partir de 1); sem ela, escolhe a
                               única coluna numérica que não parece numeração (1, 2, 3...) e recusa se
                               restar mais de uma
      --timestamps             A coluna traz instantes crescentes; ajusta os intervalos entre eles
      --alpha <ALPHA>          Nível de significância dos testes [default: 0.05]
      --bootstrap <BOOTSTRAP>  Réplicas do bootstrap paramétrico (p-valores de KS e Anderson-Darling) [default: 300]
      --seed <SEED>            [default: 1]
      --html <HTML>            Grava o relatório completo, com gráficos, em um HTML
      --json <JSON>            Grava o relatório em JSON
      --relatorio <RELATORIO>  Grava o relatório da escolha automática (o mesmo de `utsim dados`) neste
                               HTML, com um CSV das candidatas ao lado
      --criterio <CRITERIO>    Critério que ordena as candidatas aceitas no relatório: aic, bic, ks ou
                               ad [default: aic]
  -h, --help                   Print help
```

### `utsim dados --help`

```text
Ajusta os campos de um modelo alimentados por dados reais e gera os relatórios

Usage: utsim dados [OPTIONS] <MODEL>

Arguments:
  <MODEL>  Modelo (.ron) com campos alimentados por dados reais (`data: [..]`)

Options:
      --relatorio <RELATORIO>  Pasta dos relatórios: um HTML e um CSV por campo, mais index.html
      --gravar                 Grava no modelo o resultado de cada ajuste (família, distribuição e
                               hash dos dados), para reprodutibilidade e para avisar quando os dados
                               mudarem
  -h, --help                   Print help
```

### `utsim emular --help`

```text
Emula o modelo em tempo real, com servidor OPC UA (comissionamento virtual)

Usage: utsim emular [OPTIONS] <MODEL>

Arguments:
  <MODEL>  

Options:
      --porta <PORTA>            Porta TCP do servidor OPC UA (0 escolhe uma livre) [default: 4840]
      --velocidade <VELOCIDADE>  Fator de velocidade: 1.0 = tempo real, 10.0 = dez vezes mais rápido [default: 1]
      --ate <ATE>                Tempo simulado em que a emulação termina (padrão: run.stop_time)
      --gravar <GRAVAR>          Grava os comandos recebidos (tempo, variável, valor) neste arquivo RON
      --reproduzir <REPRODUZIR>  Reaplica os comandos de um arquivo RON, sem OPC UA e sem esperar o relógio
  -h, --help                     Print help
```

### `utsim gemeo --help`

```text
Gêmeo digital em modo sombra: lê sensores (OPC UA, MQTT, Sparkplug B), só leitura, compara com o modelo e grava as observações; `reproduzir` reexecuta sem rede

Usage: utsim gemeo [OPTIONS] <MODEL>
       utsim gemeo <COMMAND>

Commands:
  reproduzir  Reexecuta uma gravação sem rede e sem relógio (mesmos resultados da sessão)
  help        Print this message or the help of the given subcommand(s)

Arguments:
  <MODEL>  Modelo (.ron) que o gêmeo acompanha

Options:
      --config <CONFIG>   Arquivo do gêmeo: conexões e mapeamento (padrão: <modelo>.gemeo.ron)
      --gravar <PASTA>    Grava todas as observações nesta pasta (uma subpasta por sessão)
      --painel <PORTA>    Serve o painel em 127.0.0.1 nesta porta, com a página /gemeo
      --ate <ATE>         Tempo do gêmeo em que a sessão termina (padrão: até Ctrl+C)
      --json <JSON>       Grava o relatório final (KPIs, alertas, estado) em JSON
      --estado <ARQUIVO>  Grava o estado observado no fim como estado inicial (.ron, para `utsim run --estado`)
  -h, --help              Print help
```

### `utsim gemeo reproduzir --help`

```text
Reexecuta uma gravação sem rede e sem relógio (mesmos resultados da sessão)

Usage: utsim gemeo reproduzir [OPTIONS] <GRAVACAO>

Arguments:
  <GRAVACAO>  Pasta da sessão (sessao-...) ou pasta de gravações (usa a mais recente)

Options:
      --modelo <MODELO>   Modelo (padrão: o da gravação)
      --config <CONFIG>   Arquivo do gêmeo (padrão: o texto guardado na gravação)
      --json <JSON>       Grava o relatório (KPIs, alertas, estado) em JSON
      --estado <ARQUIVO>  Grava o estado observado no fim como estado inicial (.ron)
  -h, --help              Print help
```

### `utsim experiment --help`

```text
Usage: utsim experiment [OPTIONS] <FILE>

Arguments:
  <FILE>  

Options:
      --json <JSON>  Grava o resultado completo em JSON
  -h, --help         Print help
```

### `utsim otimizar --help`

```text
Otimiza parâmetros do modelo por simulação (busca com comparação pareada)

Usage: utsim otimizar [OPTIONS] <PROBLEMA>

Arguments:
  <PROBLEMA>  

Options:
      --html <HTML>            Grava o relatório completo, com gráficos, em um HTML
      --json <JSON>            Grava o resultado completo em JSON
      --reps <REPS>            Replicações por ponto (sobrescreve o arquivo do problema)
      --orcamento <ORCAMENTO>  Máximo de pontos avaliados (sobrescreve o arquivo do problema)
  -h, --help                   Print help
```

### `utsim gerar-modelo --help`

```text
Gera um modelo sintético de qualquer tamanho (testes de desempenho e de escala)

Usage: utsim gerar-modelo [OPTIONS]

Options:
      --objetos <OBJETOS>  Número de objetos do modelo (no mínimo 4) [default: 100]
      --tipo <TIPO>        Forma do modelo: linha, rede ou armazem [default: linha]
      --itens <ITENS>      Itens que a fonte cria (exatamente) [default: 1000]
      --saida <SAIDA>      Arquivo de saída (padrão: a saída padrão)
  -h, --help               Print help
```

### `utsim pert --help`

```text
PERT/CPM de uma parte do modelo (seleção de objetos ou trecho de A até B)

Usage: utsim pert [OPTIONS] <MODEL>

Arguments:
  <MODEL>  

Options:
      --objetos <OBJETOS>    Objetos da seleção, separados por vírgula
      --de <DE>              Início do trecho (todos os objetos em algum caminho de --de até --ate)
      --ate <ATE>            Fim do trecho
      --prazo <PRAZO>        Prazo: calcula a probabilidade de terminar até ele
      --modo <MODO>          simulado (caminho crítico observado na simulação), classico ou montecarlo [default: simulado]
      --momentos <MOMENTOS>  Momentos do PERT clássico: exatos (das distribuições) ou classicos (beta) [default: exatos]
      --sorteios <SORTEIOS>  Sorteios do Monte Carlo [default: 10000]
      --semente <SEMENTE>    Semente (padrão: a do modelo)
  -n, --reps <REPS>          Replicações do modo simulado (padrão: as do modelo)
      --csv <CSV>            Grava atividades, caminhos, tempo total, diagrama e sensibilidade em CSV neste diretório
      --html <HTML>          Grava o relatório completo, com a rede, o Gantt e os gráficos, em um HTML
  -h, --help                 Print help
```

### `utsim qualidade carta --help`

```text
Carta de controle (I-MR, X̄-R, X̄-S, p, np, c, u, EWMA ou CUSUM)

Usage: utsim qualidade carta [OPTIONS] <ARQUIVO>

Arguments:
  <ARQUIVO>  

Options:
      --tipo <TIPO>
          imr (padrão), xbarr, xbars, p, np, c, u, ewma ou cusum; com --limites, o tipo gravado nos limites
      --subgrupo <SUBGRUPO>
          Corta uma coluna de valores em subgrupos consecutivos deste tamanho
      --regras <REGRAS>
          Regras: 1 (só fora dos limites), we (Western Electric), nelson, ou lista como 1,2,5 ou we1,we4 [default: 1]
      --excluir <EXCLUIR>
          Fase I: subgrupos com causa atribuível fora da estimativa (a partir de 1), ex. 3,7
      --revisar
          Fase I revista: retira os pontos fora dos limites e reestima até não sobrar nenhum
      --limites <LIMITES>
          Fase II: limites congelados de um estudo anterior (JSON de --salvar-limites)
      --salvar-limites <SALVAR_LIMITES>
          Grava os limites estimados em JSON (para a fase II)
      --media <MEDIA>
          Média conhecida (padrão dado; EWMA e CUSUM: alvo)
      --sigma <SIGMA>
          σ conhecido do processo (padrão dado; EWMA e CUSUM: σ do valor individual)
      --lambda <LAMBDA>
          EWMA: peso λ [default: 0.2]
      --l <L>
          EWMA: largura L dos limites em σ [default: 3]
      --k <K>
          CUSUM: folga k em σ [default: 0.5]
      --h <H>
          CUSUM: intervalo de decisão h em σ [default: 5]
      --html <HTML>
          Grava o relatório com os gráficos em HTML
      --json <JSON>
          Grava a carta em JSON
  -h, --help
          Print help
```

### `utsim qualidade capacidade --help`

```text
Capacidade do processo (Cp, Cpk, Pp, Ppk, Cpm, PPM, nível sigma)

Usage: utsim qualidade capacidade [OPTIONS] <ARQUIVO>

Arguments:
  <ARQUIVO>  

Options:
      --lie <LIE>                    Limite inferior de especificação
      --lse <LSE>                    Limite superior de especificação
      --alvo <ALVO>                  Alvo (padrão: meio da especificação)
      --subgrupo <SUBGRUPO>          Corta uma coluna de valores em subgrupos consecutivos deste tamanho
      --sigma-dentro <SIGMA_DENTRO>  σ dentro: mr (amplitude móvel), rbar (R̄/d2) ou sbar (S̄/c4)
      --metodo <METODO>              normal, boxcox, percentis ou todos [default: normal]
      --arredondar                   Box-Cox: arredonda λ para um valor usual dentro do IC
      --deslocamento <DESLOCAMENTO>  Deslocamento somado ao Z no nível sigma (convenção: 1,5; 0 desliga) [default: 1.5]
      --confianca <CONFIANCA>        Nível de confiança dos intervalos [default: 95%]
      --html <HTML>                  Grava o relatório com o histograma em HTML
      --json <JSON>                  Grava os índices em JSON
  -h, --help                         Print help
```

### `utsim qualidade msa --help`

```text
Análise do sistema de medição (Gage R&R, atributos, viés e linearidade)

Usage: utsim qualidade msa [OPTIONS] <ARQUIVO>

Arguments:
  <ARQUIVO>  

Options:
      --tipo <TIPO>
          grr (peça, operador, valor), atributos (item, avaliador, resposta[, padrão]), vies ou linearidade (referência, valor) [default: grr]
      --tolerancia <TOLERANCIA>
          Tolerância LSE − LIE (para a %tolerância)
      --lie <LIE>
          Limite inferior de especificação (com --lse, dá a tolerância)
      --lse <LSE>
          Limite superior de especificação
      --k <K>
          Multiplicador de σ da variação do estudo (6 ou 5,15) [default: 6]
      --variacao-processo <VARIACAO_PROCESSO>
          Linearidade: variação do processo (6σ) para a linearidade absoluta
      --html <HTML>
          Grava o relatório com os gráficos em HTML
      --json <JSON>
          Grava o resultado em JSON
  -h, --help
          Print help
```

### `utsim qualidade arl --help`

```text
Comprimento médio de corrida (ARL) de um esquema, por simulação

Usage: utsim qualidade arl [OPTIONS]

Options:
      --tipo <TIPO>                  shewhart, we, nelson, ewma ou cusum [default: shewhart]
      --deslocamento <DESLOCAMENTO>  Deslocamento da média em σ (0: alarmes falsos) [default: 0]
      --lambda <LAMBDA>              EWMA: peso λ [default: 0.2]
      --l <L>                        EWMA: largura L dos limites em σ [default: 3]
      --k <K>                        CUSUM: folga k em σ [default: 0.5]
      --h <H>                        CUSUM: intervalo de decisão h em σ [default: 5]
      --corridas <CORRIDAS>          Corridas simuladas [default: 10000]
      --semente <SEMENTE>            Semente dos números aleatórios [default: 1]
  -h, --help                         Print help
```

# Comandos da linha de comando

O programa se chama `utsim`. Todos os comandos aceitam `--help`. Códigos de saída: 0 em caso de sucesso e 1 em caso de erro (a mensagem vai para a saída de erro, começando com `erro:`).

| Comando | Para que serve | Capítulo |
|---|---|---|
| `utsim run` | roda um modelo e mostra as estatísticas (também a partir de um estado, e grava o estado num instante) | [Resultados](resultados.md), [Simular a partir de um estado](estado-inicial.md) |
| `utsim check` | valida um modelo sem rodar | [Formato `.ron`](referencia-ron.md) |
| `utsim analisar` | aquecimento, replicações necessárias, médias em lotes, lei de Little e gargalo móvel | [Resultados](resultados.md) |
| `utsim replay` | grava o rastro de uma replicação (entrada do visualizador) | [Primeiros passos](primeiros-passos.md) |
| `utsim cena` | grava a cena 3D para outros motores | [3D realista](visual-3d.md) |
| `utsim fit` | ajusta distribuições a dados | [Ajuste de distribuições](ajuste-de-distribuicoes.md) |
| `utsim experiment` | compara cenários | [Experimentos e otimização](experimentos-e-otimizacao.md) |
| `utsim otimizar` | procura os melhores parâmetros | [Experimentos e otimização](experimentos-e-otimizacao.md) |
| `utsim emular` | emula em tempo real com OPC UA | [Emulação OPC UA](emulacao-opcua.md) |
| `utsim fluxograma` | desenha o fluxograma do processo e a carta de processo | [Fluxograma do processo](fluxograma.md) |
| `utsim gerar-modelo` | gera modelos sintéticos grandes (testes de desempenho) | `docs/desempenho.md` |
| `utsim pert` | PERT/CPM de uma seleção de objetos ou de um trecho | [PERT/CPM](pert-cpm.md) |
| `utsim planta` | lê uma planta DXF 2D (camadas, calibração, SVG) | [Planta CAD](planta-cad.md) |

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
utsim experiment examples/experimento_mm1.ron
utsim otimizar examples/otimizacao_servidores.ron --html relatorio.html
utsim emular examples/linha_com_bloqueio.ron --velocidade 10
utsim cena examples/agvs.ron --fps 10 --ate 600 --saida agvs.jsonl
utsim fluxograma examples/centro_de_distribuicao.ron --html fluxograma.html --com-resultados
utsim gerar-modelo --objetos 100 --tipo linha --itens 1000 --saida grande.ron
utsim pert examples/montagem_paletes.ron --de paletes --ate saida_caixas --prazo 10
```

## Observações sobre as opções

- `run`: as opções `-n`, `--seed`, `--stop`, `--warmup`, `--sample` e `--bins` substituem os campos de `run` do arquivo, só nesta execução. Com `--estado`, a rodada parte do estado do arquivo, sem aquecimento a menos que o estado tenha `warmup` ou que se use `--warmup`. `--salvar-estado T ARQUIVO` grava o estado da replicação 0 em `T` antes da rodada pedida.
- `analisar`: `-n` e `--stop` substituem `run.replications` e `run.stop_time`. Sem nenhuma das opções de análise, roda aquecimento, precisão a 2%, Little e gargalo móvel.
- `replay` e `cena`: `--rep` escolhe a replicação (a primeira é a 0).
- `gerar-modelo`: `--tipo` aceita `linha`, `rede` e `armazem`.
- `pert`: use `--objetos` ou `--de` com `--ate`. O modo padrão é `simulado`; `-n` só vale
  para ele e `--sorteios` só para `montecarlo`.
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

### `utsim analisar --help`

```text
Análises de saída: aquecimento, precisão, lotes, lei de Little e gargalo móvel

Sem nenhuma opção de análise, roda todas menos as médias em lotes.

Usage: utsim analisar [OPTIONS] <MODEL>

Arguments:
  <MODEL>  

Options:
      --aquecimento            Sugere o aquecimento (Welch e MSER-5) a partir do WIP ou de --medida
      --precisao <ALVO>        Replicações necessárias para esta semiamplitude: 2% (da média) ou 0.5 (absoluta)
      --lotes                  Intervalo por médias em lotes numa rodada única (padrão: tempo no sistema)
      --little                 Verifica a lei de Little por objeto e no sistema
      --gargalo-movel          Gargalo móvel pelo período ativo (único e compartilhado)
      --confianca <CONFIANCA>  Nível de confiança dos intervalos [default: 95%]
      --medida <MEDIDA>        Medida para aquecimento e lotes: objeto.medida (ex.: saida.time_in_system)
      --metrica <METRICA>      Métrica para --precisao (repetível; padrão: ocupações, conteúdos e tempos)
      --janela <JANELA>        Janela da média móvel de Welch [default: 5]
  -n, --reps <REPS>            Número de replicações (sobrescreve run.replications)
      --stop <STOP>            Tempo de parada (sobrescreve run.stop_time)
      --json <JSON>            Grava os resultados em JSON
  -h, --help                   Print help
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


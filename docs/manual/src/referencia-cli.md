# Comandos da linha de comando

O programa se chama `utsim`. Todos os comandos aceitam `--help`. Códigos de saída: 0 em caso de sucesso e 1 em caso de erro (a mensagem vai para a saída de erro, começando com `erro:`).

| Comando | Para que serve | Capítulo |
|---|---|---|
| `utsim run` | roda um modelo e mostra as estatísticas | [Resultados](resultados.md) |
| `utsim check` | valida um modelo sem rodar | [Formato `.ron`](referencia-ron.md) |
| `utsim replay` | grava o rastro de uma replicação (entrada do visualizador) | [Primeiros passos](primeiros-passos.md) |
| `utsim cena` | grava a cena 3D para outros motores | [3D realista](visual-3d.md) |
| `utsim fit` | ajusta distribuições a dados | [Ajuste de distribuições](ajuste-de-distribuicoes.md) |
| `utsim experiment` | compara cenários | [Experimentos e otimização](experimentos-e-otimizacao.md) |
| `utsim otimizar` | procura os melhores parâmetros | [Experimentos e otimização](experimentos-e-otimizacao.md) |
| `utsim emular` | emula em tempo real com OPC UA | [Emulação OPC UA](emulacao-opcua.md) |
| `utsim fluxograma` | desenha o fluxograma do processo e a carta de processo | [Fluxograma do processo](fluxograma.md) |
| `utsim gerar-modelo` | gera modelos sintéticos grandes (testes de desempenho) | `docs/desempenho.md` |

O `utsim run` e o `utsim check` imprimem, antes do resultado, **avisos** (`aviso: ...`) sobre coisas válidas mas provavelmente não pretendidas, como ligações longas e instantâneas. Eles não impedem a rodada.

## Exemplos

```bash
utsim check examples/mm1.ron
utsim run examples/mm1.ron -n 20 --json resultado.json
utsim run examples/mm1.ron --sample 100 --bins 20 --csv saida/
utsim replay examples/mm1.ron --out replay.json
utsim fit examples/dados/tempos_atendimento.csv --html relatorio.html
utsim experiment examples/experimento_mm1.ron
utsim otimizar examples/otimizacao_servidores.ron --html relatorio.html
utsim emular examples/linha_com_bloqueio.ron --velocidade 10
utsim cena examples/agvs.ron --fps 10 --ate 600 --saida agvs.jsonl
utsim fluxograma examples/centro_de_distribuicao.ron --html fluxograma.html --com-resultados
utsim gerar-modelo --objetos 100 --tipo linha --itens 1000 --saida grande.ron
```

## Observações sobre as opções

- `run`: as opções `-n`, `--seed`, `--stop`, `--warmup`, `--sample` e `--bins` substituem os campos de `run` do arquivo, só nesta execução.
- `replay` e `cena`: `--rep` escolhe a replicação (a primeira é a 0).
- `gerar-modelo`: `--tipo` aceita `linha`, `rede` e `armazem`.
- `fluxograma`: `--svg`, `--html`, `--mermaid` e `--csv` escolhem as saídas (sem nenhuma, imprime o Mermaid); `--simbolo objeto=simbolo` troca o símbolo de um objeto; `--horizontal`, `--sem-tempos` e `--sem-distancias` ajustam o desenho; `--com-resultados` roda o modelo e põe os números nas caixas.

## Saída de `--help`

Esta é a ajuda de cada comando, copiada da versão atual do programa.
### `utsim --help`

```text
UTSim: simulador de eventos discretos

Usage: utsim <COMMAND>

Commands:
  run           Executa um modelo e mostra as estatísticas
  check         Valida um modelo sem executar
  replay        Grava o rastro completo de uma replicação para o visualizador 3D
  cena          Grava a cena 3D de uma replicação (protocolo utsim-scene/1, JSON em linhas)
  gerar-modelo  Gera um modelo sintético de qualquer tamanho (testes de desempenho e de escala)
  experiment    Compara cenários de um arquivo de experimento
  otimizar      Otimiza parâmetros do modelo por simulação (busca com comparação pareada)
  emular        Emula o modelo em tempo real, com servidor OPC UA (comissionamento virtual)
  fit           Ajusta distribuições a dados reais (CSV/TXT) e justifica a escolha
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
  -n, --reps <REPS>      Número de replicações (sobrescreve run.replications)
      --seed <SEED>      
      --stop <STOP>      Tempo de parada (sobrescreve run.stop_time)
      --warmup <WARMUP>  Tempo de aquecimento (sobrescreve run.warmup)
      --json <JSON>      Grava os relatórios completos em JSON
      --csv <CSV>        Grava resumo, replicações, séries e histogramas em CSV neste diretório
      --sample <SAMPLE>  Intervalo de amostragem das séries temporais (sobrescreve run.sample_interval)
      --bins <BINS>      Número de classes dos histogramas (sobrescreve run.histogram_bins)
  -h, --help             Print help
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


mod analisar;
mod cena;
mod corredores;
mod dados;
mod emulate;
mod estado;
mod estatistica;
mod experiment;
mod fit;
mod fluxograma;
mod gemeo;
mod gerar;
mod linha;
mod optimize;
mod painel;
mod pert;
mod planta;
mod prever;
mod qualidade;
mod rede;
mod replay;
mod tempos;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use utsim_core::stats::Summary;
use utsim_core::{RunReport, summarize};
use utsim_model::Model;

#[derive(Parser)]
#[command(
    name = "utsim",
    version,
    about = "UTSim: simulador de eventos discretos"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Executa um modelo e mostra as estatísticas
    Run {
        model: PathBuf,
        /// Número de replicações (sobrescreve run.replications)
        #[arg(short = 'n', long)]
        reps: Option<u32>,
        #[arg(long)]
        seed: Option<u64>,
        /// Tempo de parada (sobrescreve run.stop_time)
        #[arg(long)]
        stop: Option<f64>,
        /// Tempo de aquecimento (sobrescreve run.warmup)
        #[arg(long)]
        warmup: Option<f64>,
        /// Grava os relatórios completos em JSON
        #[arg(long)]
        json: Option<PathBuf>,
        /// Grava resumo, replicações, séries e histogramas em CSV neste diretório
        #[arg(long)]
        csv: Option<PathBuf>,
        /// Intervalo de amostragem das séries temporais (sobrescreve run.sample_interval)
        #[arg(long)]
        sample: Option<f64>,
        /// Número de classes dos histogramas (sobrescreve run.histogram_bins)
        #[arg(long)]
        bins: Option<usize>,
        /// Parte deste estado inicial (arquivo RON) em vez do sistema vazio
        #[arg(long, value_name = "ARQUIVO")]
        estado: Option<PathBuf>,
        /// Grava o estado da replicação 0 no instante T neste arquivo, para encadear rodadas
        #[arg(long = "salvar-estado", num_args = 2, value_names = ["T", "ARQUIVO"])]
        salvar_estado: Option<Vec<String>>,
        /// Mostra também os indicadores desta linha especializada (por exemplo lean_office)
        #[arg(long)]
        linha: Option<String>,
    },
    /// Valida um modelo sem executar
    Check { model: PathBuf },
    /// Análises de saída: aquecimento, precisão, lotes, lei de Little e gargalo móvel
    ///
    /// Sem nenhuma opção de análise, roda todas menos as médias em lotes. Comparação de cenários,
    /// correlação, mapa de calor e exportação das observações só rodam quando pedidas.
    Analisar {
        model: PathBuf,
        /// Sugere o aquecimento (Welch e MSER-5) a partir do WIP ou de --medida
        #[arg(long)]
        aquecimento: bool,
        /// Replicações necessárias para esta semiamplitude: 2% (da média) ou 0.5 (absoluta)
        #[arg(long, value_name = "ALVO")]
        precisao: Option<String>,
        /// Intervalo por médias em lotes numa rodada única (padrão: tempo no sistema)
        #[arg(long)]
        lotes: bool,
        /// Verifica a lei de Little por objeto e no sistema
        #[arg(long)]
        little: bool,
        /// Gargalo móvel pelo período ativo (único e compartilhado)
        #[arg(long = "gargalo-movel")]
        gargalo_movel: bool,
        /// Nível de confiança dos intervalos
        #[arg(long, default_value = "95%")]
        confianca: String,
        /// Medida para aquecimento e lotes: objeto.medida (ex.: saida.time_in_system)
        #[arg(long)]
        medida: Option<String>,
        /// Métrica para --precisao, --comparar e --correlacao (repetível; padrão: ocupações, conteúdos e tempos)
        #[arg(long)]
        metrica: Vec<String>,
        /// Janela da média móvel de Welch
        #[arg(long, default_value_t = 5)]
        janela: usize,
        /// Número de replicações (sobrescreve run.replications)
        #[arg(short = 'n', long)]
        reps: Option<u32>,
        /// Tempo de parada (sobrescreve run.stop_time)
        #[arg(long)]
        stop: Option<f64>,
        /// Grava os resultados em JSON
        #[arg(long)]
        json: Option<PathBuf>,
        /// Compara os cenários de um experimento: ANOVA, Tukey e pares com Bonferroni
        #[arg(long, value_name = "EXPERIMENTO")]
        comparar: Option<PathBuf>,
        /// Correlação (Pearson e Spearman) entre as métricas nas replicações
        #[arg(long)]
        correlacao: bool,
        /// Grava o mapa de calor de deslocamentos da replicação 0 em SVG
        #[arg(long = "mapa-calor", value_name = "SVG")]
        mapa_calor: Option<PathBuf>,
        /// Lado da célula do mapa de calor, em metros
        #[arg(long, default_value_t = 0.5)]
        celula: f64,
        /// Camadas do mapa de calor: recurso ou classe
        #[arg(long, default_value = "recurso")]
        por: String,
        /// Grava cada observação em CSV longo (replicação, instante, objeto, medida, valor)
        #[arg(long = "exportar-observacoes", value_name = "CSV")]
        exportar_observacoes: Option<PathBuf>,
    },
    /// Grava o rastro completo de uma replicação para o visualizador 3D
    Replay {
        model: PathBuf,
        /// Arquivo JSON de saída
        #[arg(short, long, default_value = "replay.json")]
        out: PathBuf,
        #[arg(long, default_value_t = 0)]
        rep: u32,
    },
    /// Grava a cena 3D de uma replicação (protocolo utsim-scene/1, JSON em linhas)
    Cena {
        model: PathBuf,
        #[arg(long, default_value_t = 0)]
        rep: u32,
        /// Quadros por unidade de tempo simulado
        #[arg(long, default_value_t = 10.0)]
        fps: f64,
        /// Último instante gravado (padrão: o fim da rodada)
        #[arg(long)]
        ate: Option<f64>,
        /// Arquivo de saída (padrão: a saída padrão)
        #[arg(long)]
        saida: Option<PathBuf>,
        /// Em vez de gravar, serve a cena ao vivo nesta porta (TCP e WebSocket), com a
        /// rodada no ritmo do relógio e sem OPC UA
        #[arg(long, value_name = "PORTA")]
        servir: Option<u16>,
        /// Com --servir: fator de velocidade (1.0 = tempo real)
        #[arg(long, default_value_t = 1.0)]
        velocidade: f64,
        /// Com --servir: endereço de escuta (padrão: só esta máquina)
        #[arg(long, default_value = "127.0.0.1")]
        cena_endereco: String,
        /// Com --servir: quadros por segundo de relógio enviados aos clientes
        #[arg(long, default_value_t = 20.0)]
        cena_fps: f64,
    },
    /// Gera um modelo sintético de qualquer tamanho (testes de desempenho e de escala)
    GerarModelo {
        /// Número de objetos do modelo (no mínimo 4)
        #[arg(long, default_value_t = 100)]
        objetos: usize,
        /// Forma do modelo: linha, rede ou armazem
        #[arg(long, default_value = "linha")]
        tipo: String,
        /// Itens que a fonte cria (exatamente)
        #[arg(long, default_value_t = 1000)]
        itens: u64,
        /// Arquivo de saída (padrão: a saída padrão)
        #[arg(long)]
        saida: Option<PathBuf>,
    },
    /// Lê uma planta DXF 2D: camadas, calibração por dois pontos e SVG
    Planta {
        arquivo: PathBuf,
        /// Lista as camadas com contagem e papel sugerido
        #[arg(long)]
        camadas: bool,
        /// Grava a planta em SVG
        #[arg(long)]
        svg: Option<PathBuf>,
        /// Calibra: x1,y1,x2,y2,distancia (pontos nas unidades do desenho, distância em m)
        #[arg(long, value_name = "X1,Y1,X2,Y2,DIST", allow_hyphen_values = true)]
        calibrar: Option<String>,
        /// Move a origem para este ponto (x,y, em metros após a calibração)
        #[arg(long, value_name = "X,Y", allow_hyphen_values = true)]
        origem: Option<String>,
        /// Força a unidade do desenho: mm, cm, dm, m, km, pol ou pe
        #[arg(long)]
        unidades: Option<String>,
        /// Grava as paredes e colunas como `obstacles` deste modelo (e registra a planta em `floor_plan`)
        #[arg(long, value_name = "MODELO")]
        obstaculos: Option<PathBuf>,
    },
    /// Corredores de circulação: resumo, avisos e planta em SVG com as faixas
    Corredores {
        model: PathBuf,
        /// Grava a planta com os corredores, as faixas e a rede gerada em SVG
        #[arg(long)]
        svg: Option<PathBuf>,
    },
    /// Rede de caminhos: gera pelo caminho livre, desviando dos obstáculos
    Rede {
        #[command(subcommand)]
        acao: rede::Acao,
    },
    /// Gera o fluxograma do processo (SVG, HTML, Mermaid) e a carta de processo (HTML, CSV)
    Fluxograma {
        model: PathBuf,
        /// Grava o fluxograma em SVG
        #[arg(long)]
        svg: Option<PathBuf>,
        /// Grava a página HTML com legenda, fluxograma e carta de processo
        #[arg(long)]
        html: Option<PathBuf>,
        /// Grava o fluxograma em Mermaid (`flowchart LR`)
        #[arg(long)]
        mermaid: Option<PathBuf>,
        /// Grava a carta de processo (caminho principal) em CSV
        #[arg(long)]
        csv: Option<PathBuf>,
        /// Roda o modelo e põe ocupação, fila média e espera média nas caixas
        #[arg(long)]
        com_resultados: bool,
        /// Desenha da esquerda para a direita (padrão: de cima para baixo)
        #[arg(long)]
        horizontal: bool,
        /// Não mostra tempos médios
        #[arg(long)]
        sem_tempos: bool,
        /// Não mostra distâncias das transferências
        #[arg(long)]
        sem_distancias: bool,
        /// Troca o símbolo de um objeto: objeto=simbolo (inicio, fim, operacao, transporte,
        /// inspecao, espera, armazenagem); pode repetir
        #[arg(long = "simbolo", value_name = "OBJETO=SIMBOLO")]
        simbolos: Vec<String>,
        /// Tipo de diagrama: carta, blocos, operacoes, raias, sipoc, mapofluxograma, espaguete,
        /// de-para, homem-maquina, vsm ou todos (HTML índice em --html, um arquivo por tipo).
        /// Sem --tipo: fluxograma de blocos com a carta de processo
        #[arg(long)]
        tipo: Option<String>,
        /// Replicação gravada nos diagramas da simulação (a primeira é a 0)
        #[arg(long, default_value_t = 0)]
        replica: u32,
        /// Início do intervalo do gráfico homem-máquina
        #[arg(long)]
        de: Option<f64>,
        /// Fim do intervalo do gráfico homem-máquina
        #[arg(long)]
        ate: Option<f64>,
        /// Segundo modelo (o "depois") do diagrama de espaguete; o primeiro é o "antes"
        #[arg(long)]
        comparar: Option<PathBuf>,
        /// Raia de um objeto no fluxograma multifuncional: objeto=nome; pode repetir
        #[arg(long = "raia", value_name = "OBJETO=NOME")]
        raias: Vec<String>,
    },
    /// Compara cenários de um arquivo de experimento
    Experiment {
        file: PathBuf,
        /// Grava o resultado completo em JSON
        #[arg(long)]
        json: Option<PathBuf>,
    },
    /// Otimiza parâmetros do modelo por simulação (busca com comparação pareada)
    Otimizar {
        problema: PathBuf,
        /// Grava o relatório completo, com gráficos, em um HTML
        #[arg(long)]
        html: Option<PathBuf>,
        /// Grava o resultado completo em JSON
        #[arg(long)]
        json: Option<PathBuf>,
        /// Replicações por ponto (sobrescreve o arquivo do problema)
        #[arg(long)]
        reps: Option<u32>,
        /// Máximo de pontos avaliados (sobrescreve o arquivo do problema)
        #[arg(long)]
        orcamento: Option<usize>,
    },
    /// Emula o modelo em tempo real, com servidor OPC UA (comissionamento virtual)
    Emular {
        model: PathBuf,
        /// Porta TCP do servidor OPC UA (0 escolhe uma livre)
        #[arg(long, default_value_t = 4840)]
        porta: u16,
        /// Fator de velocidade: 1.0 = tempo real, 10.0 = dez vezes mais rápido
        #[arg(long, default_value_t = 1.0)]
        velocidade: f64,
        /// Tempo simulado em que a emulação termina (padrão: run.stop_time)
        #[arg(long)]
        ate: Option<f64>,
        /// Grava os comandos recebidos (tempo, variável, valor) neste arquivo RON
        #[arg(long)]
        gravar: Option<PathBuf>,
        /// Reaplica os comandos de um arquivo RON, sem OPC UA e sem esperar o relógio
        #[arg(long)]
        reproduzir: Option<PathBuf>,
        /// Serve também a cena 3D ao vivo nesta porta (TCP e WebSocket, utsim-scene/1),
        /// no mesmo relógio do OPC UA (0 escolhe uma livre)
        #[arg(long)]
        cena_porta: Option<u16>,
        /// Endereço de escuta da cena (padrão: só esta máquina; 0.0.0.0 abre para a rede)
        #[arg(long, default_value = "127.0.0.1")]
        cena_endereco: String,
        /// Quadros por segundo de relógio enviados aos clientes da cena
        #[arg(long, default_value_t = 20.0)]
        cena_fps: f64,
    },
    /// Gêmeo digital em modo sombra: lê sensores (OPC UA, MQTT, Sparkplug B), só leitura,
    /// compara com o modelo e grava as observações; `reproduzir` reexecuta sem rede
    Gemeo(gemeo::Args),
    /// Previsão a partir de um estado, sem gêmeo: R replicações até o horizonte, com metas,
    /// limites e cenários "e se" comparados com números aleatórios comuns
    Prever(prever::Args),
    /// Painel web para gestores e engenheiros: roda as replicações e serve a página (127.0.0.1)
    Painel {
        model: PathBuf,
        /// Porta do servidor local
        #[arg(long, default_value_t = 8080)]
        porta: u16,
        /// Arquivo do painel (padrão: <modelo>.painel.ron, se existir)
        #[arg(long)]
        config: Option<PathBuf>,
        /// Número de replicações (sobrescreve run.replications)
        #[arg(short = 'n', long)]
        reps: Option<u32>,
        /// Não serve: grava o painel congelado (HTML único, CSV, JSON, .xlsx) nesta pasta
        #[arg(long, value_name = "PASTA")]
        exportar: Option<PathBuf>,
        /// Abre o navegador padrão no endereço do painel
        #[arg(long)]
        abrir: bool,
    },
    /// Ajusta distribuições a dados reais (CSV/TXT) e justifica a escolha
    Fit {
        file: PathBuf,
        /// Coluna: nome do cabeçalho ou posição (a partir de 1); sem ela, escolhe a única coluna
        /// numérica que não parece numeração (1, 2, 3...) e recusa se restar mais de uma
        #[arg(long)]
        coluna: Option<String>,
        /// A coluna traz instantes crescentes; ajusta os intervalos entre eles
        #[arg(long)]
        timestamps: bool,
        /// Nível de significância dos testes
        #[arg(long, default_value_t = 0.05)]
        alpha: f64,
        /// Réplicas do bootstrap paramétrico (p-valores de KS e Anderson-Darling)
        #[arg(long, default_value_t = 300)]
        bootstrap: usize,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Grava o relatório completo, com gráficos, em um HTML
        #[arg(long)]
        html: Option<PathBuf>,
        /// Grava o relatório em JSON
        #[arg(long)]
        json: Option<PathBuf>,
        /// Grava o relatório da escolha automática (o mesmo de `utsim dados`) neste HTML,
        /// com um CSV das candidatas ao lado
        #[arg(long)]
        relatorio: Option<PathBuf>,
        /// Critério que ordena as candidatas aceitas no relatório: aic, bic, ks ou ad
        #[arg(long, default_value = "aic")]
        criterio: String,
    },
    // Arguments parsed by `qualidade::run_args`, so the main parser stays small (its
    // debug build overflowed the main-thread stack on Windows).
    /// Controle estatístico de processo, capacidade e análise do sistema de medição
    #[command(disable_help_flag = true)]
    Qualidade {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Estatística descritiva e testes de hipótese (t, ANOVA, qui-quadrado, proporções,
    /// Mann-Whitney, Wilcoxon) sobre as colunas de um CSV
    #[command(disable_help_flag = true)]
    Estatistica {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Estudo de tempos e métodos (homem-máquina): ciclo de trabalho, tempos por elemento,
    /// máquinas por operador, tempo padrão e comparação de métodos
    #[command(disable_help_flag = true)]
    Tempos {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Ajusta os campos de um modelo alimentados por dados reais e gera os relatórios
    Dados(dados::Args),
    /// PERT/CPM de uma parte do modelo (seleção de objetos ou trecho de A até B)
    Pert {
        model: PathBuf,
        /// Objetos da seleção, separados por vírgula
        #[arg(long, conflicts_with_all = ["de", "ate"])]
        objetos: Option<String>,
        /// Início do trecho (todos os objetos em algum caminho de --de até --ate)
        #[arg(long, requires = "ate")]
        de: Option<String>,
        /// Fim do trecho
        #[arg(long, requires = "de")]
        ate: Option<String>,
        /// Prazo: calcula a probabilidade de terminar até ele
        #[arg(long)]
        prazo: Option<f64>,
        /// simulado (caminho crítico observado na simulação), classico ou montecarlo
        #[arg(long, default_value = "simulado")]
        modo: String,
        /// Momentos do PERT clássico: exatos (das distribuições) ou classicos (beta)
        #[arg(long, default_value = "exatos")]
        momentos: String,
        /// Sorteios do Monte Carlo
        #[arg(long, default_value_t = 10_000)]
        sorteios: usize,
        /// Semente (padrão: a do modelo)
        #[arg(long)]
        semente: Option<u64>,
        /// Replicações do modo simulado (padrão: as do modelo)
        #[arg(short = 'n', long)]
        reps: Option<u32>,
        /// Grava atividades, caminhos, tempo total, diagrama e sensibilidade em CSV neste diretório
        #[arg(long)]
        csv: Option<PathBuf>,
        /// Grava o relatório completo, com a rede, o Gantt e os gráficos, em um HTML
        #[arg(long)]
        html: Option<PathBuf>,
    },
    /// Linhas especializadas (Lean Office): predefinições, vocabulário, modelos prontos,
    /// indicadores e relatório da linha
    #[command(disable_help_flag = true)]
    Linha {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

fn main() -> ExitCode {
    // The argument parser for all subcommands is large; in debug builds it overflows the 1 MB
    // main-thread stack of Windows, so the program runs on a thread with a bigger stack.
    let worker = std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024)
        .spawn(|| match run(Cli::parse()) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("erro: {e}");
                ExitCode::FAILURE
            }
        })
        .expect("thread principal do utsim");
    worker.join().unwrap_or(ExitCode::FAILURE)
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.cmd {
        Cmd::Qualidade { args } => qualidade::run_args(args)?,
        Cmd::Estatistica { args } => estatistica::run_args(args)?,
        Cmd::Tempos { args } => tempos::run_args(args)?,
        Cmd::Linha { args } => linha::run_args(args)?,
        Cmd::Pert {
            model,
            objetos,
            de,
            ate,
            prazo,
            modo,
            momentos,
            sorteios,
            semente,
            reps,
            csv,
            html,
        } => pert::run(pert::Args {
            model,
            objects: objetos,
            from: de,
            to: ate,
            deadline: prazo,
            mode: modo,
            moments: momentos,
            samples: sorteios,
            seed: semente,
            reps,
            csv,
            html,
        })?,
        Cmd::Painel {
            model,
            porta,
            config,
            reps,
            exportar,
            abrir,
        } => painel::run(painel::Args {
            model,
            port: porta,
            config,
            reps,
            export: exportar,
            open: abrir,
        })?,
        Cmd::Fit {
            file,
            coluna,
            timestamps,
            alpha,
            bootstrap,
            seed,
            html,
            json,
            relatorio,
            criterio,
        } => fit::run(fit::Args {
            file,
            column: coluna,
            timestamps,
            alpha,
            bootstrap,
            seed,
            html,
            json,
            report: relatorio,
            criterion: criterio,
        })?,
        Cmd::Dados(args) => dados::run(args)?,
        Cmd::Gemeo(args) => gemeo::run(args)?,
        Cmd::Prever(args) => prever::run(args)?,
        Cmd::Otimizar {
            problema,
            html,
            json,
            reps,
            orcamento,
        } => optimize::run(optimize::Args {
            file: problema,
            html,
            json,
            reps,
            budget: orcamento,
        })?,
        Cmd::Emular {
            model,
            porta,
            velocidade,
            ate,
            gravar,
            reproduzir,
            cena_porta,
            cena_endereco,
            cena_fps,
        } => emulate::run(emulate::Args {
            model,
            port: porta,
            speed: velocidade,
            until: ate,
            record: gravar,
            replay: reproduzir,
            scene_port: cena_porta,
            scene_addr: cena_endereco,
            scene_fps: cena_fps,
        })?,
        Cmd::Check { model } => {
            let m = Model::load(&model)?;
            m.validate()?;
            print_warnings(&m);
            println!(
                "{}: ok ({} objetos, {} conexões)",
                m.name,
                m.objects.len(),
                m.connections.len()
            );
            for line in m.movement_lines() {
                println!("movimentação: {line}");
            }
            for line in m.maintenance_lines() {
                println!("manutenção: {line}");
            }
            for line in m.person_lines() {
                println!("pessoa: {line}");
            }
            for line in m.arrival_lines() {
                println!("chegadas: {line}");
            }
        }
        Cmd::Rede { acao } => rede::run(acao)?,
        Cmd::Corredores { model, svg } => corredores::run(&model, svg.as_deref())?,
        Cmd::Analisar {
            model,
            aquecimento,
            precisao,
            lotes,
            little,
            gargalo_movel,
            confianca,
            medida,
            metrica,
            janela,
            reps,
            stop,
            json,
            comparar,
            correlacao,
            mapa_calor,
            celula,
            por,
            exportar_observacoes,
        } => analisar::run(analisar::Args {
            model,
            warmup: aquecimento,
            precision: precisao,
            batches: lotes,
            little,
            bottleneck: gargalo_movel,
            level: confianca,
            measure: medida,
            metrics: metrica,
            window: janela,
            reps,
            stop,
            json,
            compare: comparar,
            correlation: correlacao,
            heatmap: mapa_calor,
            cell: celula,
            group: por,
            observations: exportar_observacoes,
        })?,
        Cmd::Fluxograma {
            model,
            svg,
            html,
            mermaid,
            csv,
            com_resultados,
            horizontal,
            sem_tempos,
            sem_distancias,
            simbolos,
            tipo,
            replica,
            de,
            ate,
            comparar,
            raias,
        } => fluxograma::run(fluxograma::Args {
            model,
            svg,
            html,
            mermaid,
            csv,
            with_results: com_resultados,
            horizontal,
            no_times: sem_tempos,
            no_distances: sem_distancias,
            symbols: simbolos,
            kind: tipo,
            replication: replica,
            from: de,
            to: ate,
            compare: comparar,
            lanes: raias,
        })?,
        Cmd::Experiment { file, json } => experiment::run(&file, json.as_deref())?,
        Cmd::Planta {
            arquivo,
            camadas,
            svg,
            calibrar,
            origem,
            unidades,
            obstaculos,
        } => planta::run(
            &arquivo,
            camadas,
            svg.as_deref(),
            calibrar.as_deref(),
            origem.as_deref(),
            unidades.as_deref(),
            obstaculos.as_deref(),
        )?,
        Cmd::GerarModelo {
            objetos,
            tipo,
            itens,
            saida,
        } => gerar::run(&tipo, objetos, itens, saida.as_deref())?,
        Cmd::Replay { model, out, rep } => replay::run(&model, rep, &out)?,
        Cmd::Cena {
            model,
            rep,
            fps,
            ate,
            saida,
            servir,
            velocidade,
            cena_endereco,
            cena_fps,
        } => match servir {
            Some(porta) => cena::serve(
                &model,
                rep,
                porta,
                &cena_endereco,
                cena_fps,
                velocidade,
                ate,
            )?,
            None => cena::run(&model, rep, fps, ate, saida.as_deref())?,
        },
        Cmd::Run {
            model,
            reps,
            seed,
            stop,
            warmup,
            json,
            csv,
            sample,
            bins,
            estado,
            salvar_estado,
            linha,
        } => {
            if let Some(l) = &linha {
                linha::pack(l)?;
            }
            let mut m = Model::load(&model)?;
            if let Some(v) = reps {
                m.run.replications = v;
            }
            if let Some(v) = seed {
                m.run.seed = v;
            }
            if let Some(v) = stop {
                m.run.stop_time = v;
            }
            if let Some(v) = warmup {
                m.run.warmup = v;
            }
            if let Some(v) = sample {
                m.run.sample_interval = Some(v);
            }
            if let Some(v) = bins {
                m.run.histogram_bins = Some(v);
            }
            let state = match &estado {
                Some(path) => Some(estado::load(&mut m, path, warmup)?),
                None => None,
            };
            m.validate()?;
            print_warnings(&m);
            if let Some(args) = &salvar_estado {
                estado::save(&m, state.as_ref(), args)?;
            }
            let started = std::time::Instant::now();
            let reports = match &state {
                Some(st) => m.run_replications_from(st)?,
                None => m.run_replications()?,
            };
            let elapsed = started.elapsed();
            print_summary(&m, &reports);
            let events: u64 = reports.iter().map(|r| r.events).sum();
            println!("\n{events} eventos em {:.2?}", elapsed);
            if let Some(dir) = csv {
                let files = utsim_model::csv::export(&dir, &reports)?;
                println!("CSV gravado em {}: {}", dir.display(), files.join(", "));
            }
            if let Some(path) = json {
                let out = serde_json::json!({ "model": m.name, "summary": summarize(&reports), "replications": reports });
                std::fs::write(
                    &path,
                    serde_json::to_string_pretty(&out).expect("report serializes"),
                )?;
                println!("relatório gravado em {}", path.display());
            }
            if let Some(l) = &linha {
                linha::after_run(&m, l)?;
            }
        }
    }
    Ok(())
}

/// Non-fatal model warnings, on stderr so they never mix with results.
fn print_warnings(model: &Model) {
    for warning in model.warnings() {
        eprintln!("aviso: {warning}");
    }
}

fn print_summary(model: &Model, reports: &[RunReport]) {
    let summary = summarize(reports);
    let first = &reports[0];
    println!(
        "{}: {} replicação(ões), estatísticas de t={} até t={}\n",
        model.name,
        reports.len(),
        first.stats_from,
        first.end_time
    );
    let cell = |key: String| summary.get(&key).map_or_else(|| "-".to_string(), fmt);
    println!(
        "{:<14} {:<10} {:>22} {:>22} {:>22} {:>22}",
        "objeto", "tipo", "entradas", "saídas", "conteúdo médio", "permanência média"
    );
    for b in &first.blocks {
        let n = &b.name;
        println!(
            "{:<14} {:<10} {:>22} {:>22} {:>22} {:>22}",
            n,
            b.kind,
            cell(format!("{n}.input")),
            cell(format!("{n}.output")),
            cell(format!("{n}.content.mean")),
            cell(format!("{n}.staytime.mean")),
        );
    }

    print_utilization(&summary, first);

    println!();
    for b in &first.blocks {
        let n = &b.name;
        let prefix = format!("{n}.state.");
        let states: Vec<String> = summary
            .iter()
            .filter_map(|(k, s)| k.strip_prefix(&prefix).map(|state| (state, s)))
            .filter(|(_, s)| s.mean > 0.0)
            .map(|(state, s)| format!("{state} {}", fmt_pct(s)))
            .collect();
        // A tally shows its mean; `lost` is always 1 per item, so it shows how many
        // items were lost per replication instead.
        let extras: Vec<String> = b
            .extra
            .keys()
            .map(|k| match k.as_str() {
                "lost" => format!("lost {} por replicação", cell(format!("{n}.lost.count"))),
                _ => format!("{k} {}", cell(format!("{n}.{k}.mean"))),
            })
            .collect();
        // Metrics computed by the block itself (e.g. tank levels of a fluid network).
        let custom: Vec<String> = b
            .custom
            .keys()
            // `servers.*` only feed the utilisation table above.
            .filter(|k| !k.starts_with("servers."))
            .map(|k| format!("{k} {}", cell(format!("{n}.{k}"))))
            .collect();
        if !states.is_empty() || !extras.is_empty() {
            println!(
                "{n:<14} {}",
                states
                    .into_iter()
                    .chain(extras)
                    .collect::<Vec<_>>()
                    .join(" | ")
            );
        }
        // One per line: a fluid network may have several tanks.
        for m in custom {
            println!("{n:<14} {m}");
        }
    }
}

/// Utilisation of every resource (processes, assemblies, mobile resources) and the
/// model-wide loss of sources that drop items. Nothing is printed without them.
fn print_utilization(summary: &std::collections::BTreeMap<String, Summary>, first: &RunReport) {
    let resources: Vec<&str> = first
        .blocks
        .iter()
        .filter(|b| summary.contains_key(&format!("{}.utilization", b.name)))
        .map(|b| b.name.as_str())
        .collect();
    if !resources.is_empty() {
        println!(
            "\n{:<14} {:>20} {:>20} {:>20} {:>20}",
            "ocupação", "trabalhando", "incl. bloqueio", "dentro do turno", "sem bloqueio"
        );
        for &n in &resources {
            let cell = |suffix: &str| {
                summary
                    .get(&format!("{n}.{suffix}"))
                    .map_or_else(|| "-".to_string(), fmt_pct)
            };
            println!(
                "{n:<14} {:>20} {:>20} {:>20} {:>20}",
                cell("utilization"),
                cell("utilization_incl_blocked"),
                cell("utilization_in_shift"),
                cell("utilization_excl_blocked"),
            );
        }
        // The bottleneck overall and within shift: they differ when resources work
        // different shifts.
        let best = |suffix: &str| {
            resources
                .iter()
                .filter_map(|n| {
                    let s = summary
                        .get(&format!("{n}.{suffix}"))
                        .or_else(|| summary.get(&format!("{n}.utilization")))?;
                    (s.mean > 0.0).then_some((*n, s))
                })
                .fold(None::<(&str, &Summary)>, |acc, (n, s)| match acc {
                    Some((_, b)) if b.mean >= s.mean => acc,
                    _ => Some((n, s)),
                })
        };
        if let Some((n, s)) = best("utilization") {
            println!("gargalo (geral): {n} ({})", fmt_pct(s));
        }
        if let Some((n, s)) = best("utilization_in_shift") {
            println!("gargalo (dentro do turno): {n} ({})", fmt_pct(s));
        }
    }
    if let (Some(lost), Some(fraction)) = (
        summary.get("model.lost.count"),
        summary.get("model.loss_fraction"),
    ) {
        println!(
            "\nperdas do modelo (fontes com Drop): {} perdidos por replicação, fração perdida {}",
            fmt(lost),
            fmt_pct(fraction)
        );
    }
}

fn fmt(s: &Summary) -> String {
    if s.half_width.is_nan() {
        format!("{:.4}", s.mean)
    } else {
        format!("{:.4} ± {:.4}", s.mean, s.half_width)
    }
}

fn fmt_pct(s: &Summary) -> String {
    if s.half_width.is_nan() {
        format!("{:.2}%", 100.0 * s.mean)
    } else {
        format!("{:.2}% ± {:.2}", 100.0 * s.mean, 100.0 * s.half_width)
    }
}

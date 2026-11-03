use std::path::{Path, PathBuf};

use clap::Subcommand;
use utsim_model::{Model, Movement};

#[derive(Subcommand)]
pub enum Acao {
    /// Gera a rede de caminhos pelo caminho livre (desviando dos obstáculos) e grava no modelo
    Gerar {
        model: PathBuf,
        /// Grava neste arquivo em vez de sobrescrever o modelo
        #[arg(long)]
        saida: Option<PathBuf>,
        /// Folga em metros (padrão: a maior entre os recursos móveis)
        #[arg(long)]
        folga: Option<f64>,
        /// Também muda `run.movement` para Network (os recursos passam a usar a rede)
        #[arg(long)]
        usar: bool,
        /// Substitui uma rede desenhada à mão (sem isso, só uma rede gerada é trocada)
        #[arg(long)]
        substituir: bool,
    },
}

/// Comentários antes de `Model(`, mantidos ao gravar (como no editor).
pub(crate) fn header(text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        let t = line.trim_start();
        if t.starts_with("Model") || t.starts_with('(') || t.starts_with("#!") {
            break;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

pub(crate) fn to_text(model: &Model, header: &str) -> String {
    let config = ron::ser::PrettyConfig::new()
        .struct_names(true)
        .indentor("    ".to_string())
        .extensions(ron::extensions::Extensions::IMPLICIT_SOME);
    let body = ron::ser::to_string_pretty(model, config).expect("model serializes");
    format!("{header}{body}\n")
}

pub fn run(acao: Acao) -> Result<(), Box<dyn std::error::Error>> {
    match acao {
        Acao::Gerar {
            model,
            saida,
            folga,
            usar,
            substituir,
        } => gerar(&model, saida.as_deref(), folga, usar, substituir),
    }
}

fn gerar(
    path: &Path,
    saida: Option<&Path>,
    folga: Option<f64>,
    usar: bool,
    substituir: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(path)?;
    let mut m = Model::parse(&text)?;
    m.validate()?;
    if let Some(c) = folga {
        if !(c.is_finite() && c >= 0.0) {
            return Err("--folga deve ser um número >= 0 (metros)".into());
        }
    }
    if m.network.as_ref().is_some_and(|n| n.generated.is_none()) && !substituir {
        return Err(
            "o modelo já tem uma rede desenhada à mão; use --substituir para trocá-la pela gerada"
                .into(),
        );
    }
    let g = m.generate_network(folga);
    for name in &g.unreachable {
        eprintln!(
            "aviso: '{name}' não tem caminho livre até os outros pontos e ficou fora da rede"
        );
    }
    let info = g.network.generated.clone().expect("generated network");
    println!(
        "rede gerada: {} nós, {} trechos, folga {} m ({})",
        g.network.nodes.len(),
        g.network.edges.len(),
        format!("{:.2}", info.clearance).replace('.', ","),
        info.fingerprint
    );
    m.network = Some(g.network);
    if usar {
        m.run.movement = Some(Movement::Network);
    }
    m.validate()?;
    for w in m.warnings() {
        eprintln!("aviso: {w}");
    }
    let out = saida.unwrap_or(path);
    std::fs::write(out, to_text(&m, &header(&text)))?;
    println!("gravado em {}", out.display());
    if !usar {
        println!(
            "para os recursos usarem a rede: `movement: Network` em run (ou no recurso), ou rode de novo com --usar"
        );
    }
    Ok(())
}

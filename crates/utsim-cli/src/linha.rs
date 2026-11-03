//! `utsim linha`: the specialised lines (`utsim_linhas`): list them, show a line's
//! presets, vocabulary and ready models, place a preset in a model, and run the line's
//! indicators and report. Also the `--linha` option of `utsim run`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use utsim_edit::{Command, Document};
use utsim_linhas::lean_office::{Indicadores, diagramas, relatorio};
use utsim_linhas::{Pack, Placement, builtin, find};
use utsim_model::Model;

/// The `linha` command line, parsed on its own (as `qualidade` and `tempos`, to keep the
/// main parser small).
#[derive(Parser)]
#[command(
    name = "utsim linha",
    about = "Linhas especializadas (Lean Office): predefinições, vocabulário, modelos \
             prontos, indicadores e relatório da linha"
)]
struct Args {
    /// Linha (por exemplo lean_office), ou "lista" para ver as linhas instaladas
    linha: String,
    #[command(subcommand)]
    acao: Option<Acao>,
}

#[derive(Subcommand)]
enum Acao {
    /// Predefinições da biblioteca da linha, com os campos e os valores padrão
    Predefinicoes,
    /// Vocabulário da linha (termo do núcleo -> termo da linha)
    Vocabulario,
    /// Modelos prontos da linha
    Modelos,
    /// Indicadores da linha (com IC de 95% entre replicações) e, com --html, o relatório
    Relatorio {
        /// Modelo (.ron)
        model: PathBuf,
        /// Número de replicações (sobrescreve run.replications)
        #[arg(short = 'n', long)]
        reps: Option<u32>,
        /// Grava o relatório completo (indicadores, gráficos, VSM de escritório, raias)
        #[arg(long)]
        html: Option<PathBuf>,
        /// Cenário do estado futuro (outro .ron): VSM e indicadores lado a lado
        #[arg(long)]
        comparar: Option<PathBuf>,
        /// Grava os indicadores de cada replicação em JSON
        #[arg(long)]
        json: Option<PathBuf>,
    },
    /// Coloca uma predefinição num modelo (cria o arquivo se não existir) e grava
    Colocar {
        /// Modelo (.ron)
        model: PathBuf,
        /// Predefinição (veja `utsim linha <linha> predefinicoes`)
        predefinicao: String,
        /// Nome do objeto (padrão: o nome base da predefinição, numerado se preciso)
        #[arg(long)]
        nome: Option<String>,
        /// Valor de um campo em RON (repetível), por exemplo --campo 'tempo=Exp(mean: 20.0)'
        #[arg(long = "campo", value_name = "CHAVE=VALOR")]
        campos: Vec<String>,
        /// Posição (x y) em metros
        #[arg(long, num_args = 2, value_names = ["X", "Y"], allow_negative_numbers = true)]
        posicao: Option<Vec<f64>>,
        /// Área (departamento) dos objetos criados
        #[arg(long)]
        area: Option<String>,
        /// Grava em outro arquivo (padrão: o próprio modelo)
        #[arg(long)]
        saida: Option<PathBuf>,
    },
}

/// Parses the arguments after `utsim linha` and runs the action.
pub fn run_args(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let a = Args::try_parse_from(std::iter::once("utsim linha".to_string()).chain(args))
        .unwrap_or_else(|e| e.exit());
    run(a)
}

/// The built-in line `id`, or an error that lists the installed ones.
pub fn pack(id: &str) -> Result<Pack, String> {
    find(id).ok_or_else(|| {
        let ids: Vec<String> = builtin().iter().map(|p| p.id().to_string()).collect();
        format!(
            "linha '{id}' desconhecida (linhas instaladas: {})",
            ids.join(", ")
        )
    })
}

fn run(a: Args) -> Result<(), Box<dyn std::error::Error>> {
    if a.linha == "lista" {
        for p in builtin() {
            println!("{}  {}: {}", p.id(), p.library.name, p.library.description);
        }
        return Ok(());
    }
    let p = pack(&a.linha)?;
    match a.acao {
        None => {
            println!("{} ({})", p.library.name, p.id());
            println!("{}", p.library.description);
            println!(
                "{} predefinições, {} termos de vocabulário, {} modelo(s) pronto(s)",
                p.library.presets.len(),
                p.vocabulary.terms.len(),
                p.library.models.len()
            );
        }
        Some(Acao::Predefinicoes) => {
            for pr in &p.library.presets {
                println!(
                    "{}  {} ({}, papel: {})",
                    pr.id,
                    pr.name,
                    pr.icon,
                    pr.role.nome()
                );
                if !pr.description.is_empty() {
                    println!("    {}", pr.description);
                }
                for f in &pr.fields {
                    println!(
                        "    --campo {}=...  {} (padrão: {})",
                        f.key, f.label, f.default
                    );
                }
            }
        }
        Some(Acao::Vocabulario) => {
            let w = p
                .vocabulary
                .terms
                .keys()
                .map(|k| k.len())
                .max()
                .unwrap_or(0);
            for (k, v) in &p.vocabulary.terms {
                println!("{k:w$}  {v}");
            }
        }
        Some(Acao::Modelos) => {
            for m in &p.library.models {
                println!("{}  {}: {}", m.file, m.name, m.description);
            }
        }
        Some(Acao::Relatorio {
            model,
            reps,
            html,
            comparar,
            json,
        }) => relatorio_cmd(&p, &model, reps, html, comparar, json)?,
        Some(Acao::Colocar {
            model,
            predefinicao,
            nome,
            campos,
            posicao,
            area,
            saida,
        }) => {
            let mut doc = if model.exists() {
                Document::open(&model)?
            } else {
                let stem = model
                    .file_stem()
                    .map_or("modelo".into(), |s| s.to_string_lossy().to_string());
                Document::new(&stem)
            };
            if doc.model().line.is_none() {
                doc.apply(Command::SetLine(Some(p.id().to_string())))?;
            }
            let mut values = BTreeMap::new();
            for c in &campos {
                let (k, v) = c
                    .split_once('=')
                    .ok_or_else(|| format!("--campo espera chave=valor, recebi '{c}'"))?;
                values.insert(k.trim().to_string(), v.trim().to_string());
            }
            let pos = posicao.unwrap_or_else(|| vec![0.0, 0.0]);
            let names = p.apply(
                &mut doc,
                &Placement {
                    preset: &predefinicao,
                    name: nome.as_deref(),
                    values,
                    position: [pos[0], pos[1], 0.0],
                    area: area.as_deref(),
                },
            )?;
            let out = saida.unwrap_or(model);
            doc.save_as(&out)?;
            println!("{}: criado(s) {}", out.display(), names.join(", "));
            for issue in doc.issues() {
                println!(
                    "pendência: {}{}",
                    issue.object.map(|o| format!("{o}: ")).unwrap_or_default(),
                    issue.message
                );
            }
        }
    }
    Ok(())
}

fn load(path: &Path, reps: Option<u32>) -> Result<Model, Box<dyn std::error::Error>> {
    let mut m = Model::load(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if let Some(n) = reps {
        m.run.replications = n;
    }
    Ok(m)
}

fn relatorio_cmd(
    p: &Pack,
    path: &Path,
    reps: Option<u32>,
    html: Option<PathBuf>,
    comparar: Option<PathBuf>,
    json: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let m = load(path, reps)?;
    if m.line.as_deref().is_some_and(|l| l != p.id()) {
        eprintln!(
            "aviso: o modelo é da linha '{}'; os indicadores são os de '{}'",
            m.line.as_deref().unwrap_or_default(),
            p.id()
        );
    }
    let k = Indicadores::rodar(&m, Some(p))?;
    print!("{}", relatorio::texto(&k));
    let futuro = match &comparar {
        Some(f) => {
            let m2 = load(f, reps)?;
            let k2 = Indicadores::rodar(&m2, Some(p))?;
            println!();
            print!("{}", relatorio::texto(&k2));
            Some((m2, k2))
        }
        None => None,
    };
    if let Some(out) = html {
        let secoes = diagramas::secoes(&m, Some(p), &k, futuro.as_ref().map(|(m2, k2)| (m2, k2)));
        std::fs::write(&out, relatorio::html(&k, &p.vocabulary, &secoes))?;
        println!("relatório gravado em {}", out.display());
    }
    if let Some(out) = json {
        let v = serde_json::json!({
            "linha": p.id(),
            "modelo": k.modelo,
            "replicacoes": k.reps,
            "futuro": futuro.as_ref().map(|(_, k2)| serde_json::json!({
                "modelo": k2.modelo,
                "replicacoes": k2.reps,
            })),
        });
        std::fs::write(&out, serde_json::to_string_pretty(&v)?)?;
        println!("indicadores gravados em {}", out.display());
    }
    Ok(())
}

/// `utsim run --linha <id>`: after the usual results, the line's indicators.
pub fn after_run(model: &Model, id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let p = pack(id)?;
    let k = Indicadores::rodar(model, Some(&p))?;
    println!();
    print!("{}", relatorio::texto(&k));
    Ok(())
}

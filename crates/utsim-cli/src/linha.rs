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
use utsim_model::quality::{ZoneDef, ZoneKind};

/// The `linha` command line, parsed on its own (as `qualidade` and `tempos`, to keep the
/// main parser small).
#[derive(Parser)]
#[command(
    name = "utsim linha",
    about = "Linhas especializadas (Lean Office, processos administrativos): predefinições, \
             vocabulário, modelos prontos, indicadores e relatório da linha"
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
        /// Grava o relatório completo (indicadores, gráficos, VSM administrativo, raias)
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
        /// Departamento dos objetos criados (campo `department`; raia do fluxograma)
        #[arg(long)]
        departamento: Option<String>,
        /// Grava em outro arquivo (padrão: o próprio modelo)
        #[arg(long)]
        saida: Option<PathBuf>,
    },
    /// Põe uma zona de risco no modelo (campo `zones`, que as predefinições não criam)
    Zona {
        /// Modelo (.ron)
        model: PathBuf,
        /// Nome da zona
        nome: String,
        /// Tipo: veiculo, maquina, carga_suspensa, ruido, calor ou outro
        #[arg(long, default_value = "veiculo")]
        tipo: String,
        /// Nível da matriz de risco (1 em diante)
        #[arg(long)]
        nivel: Option<u8>,
        /// EPI exigido para entrar (repetível)
        #[arg(long = "epi")]
        epi: Vec<String>,
        /// Vértices do polígono em metros: x1 y1 x2 y2 x3 y3 ...
        #[arg(long, num_args = 6.., value_names = ["X", "Y"], allow_negative_numbers = true, required = true)]
        poligono: Vec<f64>,
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
            let tipo = if p.is_cross_cutting() {
                " [soma-se à linha pelo campo packs]"
            } else {
                ""
            };
            println!(
                "{}  {}{tipo}: {}",
                p.id(),
                p.library.title(),
                p.library.description
            );
        }
        return Ok(());
    }
    let p = pack(&a.linha)?;
    match a.acao {
        None => {
            println!("{} [{}]", p.library.title(), p.id());
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
        Some(Acao::Zona {
            model,
            nome,
            tipo,
            nivel,
            epi,
            poligono,
        }) => {
            if !p.is_cross_cutting() {
                return Err(format!("'{}' não é um pacote transversal", p.id()).into());
            }
            let mut doc = Document::open(&model)?;
            let zone = zona(&nome, &tipo, nivel, epi, &poligono)?;
            let mut packs = doc.model().packs.clone();
            if !packs.iter().any(|x| x == p.id()) {
                packs.push(p.id().to_string());
            }
            doc.apply(Command::Batch(vec![
                Command::SetPacks(packs),
                Command::AddZone(zone),
            ]))?;
            doc.save()?;
            println!("{}: zona '{nome}' criada", model.display());
        }
        Some(Acao::Colocar {
            model,
            predefinicao,
            nome,
            campos,
            posicao,
            departamento,
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
            if doc.model().line.is_none() && !p.is_cross_cutting() {
                doc.apply(Command::SetLine(Some(p.id().to_string())))?;
            }
            let values = valores(&campos)?;
            let pos = posicao.unwrap_or_else(|| vec![0.0, 0.0]);
            let names = p.apply(
                &mut doc,
                &Placement {
                    preset: &predefinicao,
                    name: nome.as_deref(),
                    values,
                    position: [pos[0], pos[1], 0.0],
                    department: departamento.as_deref(),
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

/// A risk zone from the `zona` options.
fn zona(
    nome: &str,
    tipo: &str,
    nivel: Option<u8>,
    ppe: Vec<String>,
    poligono: &[f64],
) -> Result<ZoneDef, String> {
    if poligono.len() % 2 != 0 {
        return Err("--poligono espera pares x y".into());
    }
    let kind = match tipo {
        "veiculo" => ZoneKind::Vehicle,
        "maquina" => ZoneKind::Machine,
        "carga_suspensa" => ZoneKind::SuspendedLoad,
        "ruido" => ZoneKind::Noise,
        "calor" => ZoneKind::Heat,
        "outro" => ZoneKind::Other,
        t => return Err(format!("tipo de zona desconhecido: '{t}'")),
    };
    Ok(ZoneDef {
        name: nome.to_string(),
        kind,
        polygon: poligono.chunks(2).map(|c| [c[0], c[1]]).collect(),
        level: nivel,
        ppe,
    })
}

/// The `--campo chave=valor` options as a map (the value is kept as RON text).
fn valores(campos: &[String]) -> Result<BTreeMap<String, String>, String> {
    let mut values = BTreeMap::new();
    for c in campos {
        let (k, v) = c
            .split_once('=')
            .ok_or_else(|| format!("--campo espera chave=valor, recebi '{c}'"))?;
        let k = k.trim();
        if k.is_empty() {
            return Err(format!("--campo sem chave: '{c}'"));
        }
        values.insert(k.to_string(), v.trim().to_string());
    }
    Ok(values)
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
            "replicacoes": &k.reps,
            "futuro": futuro.as_ref().map(|(_, k2)| serde_json::json!({
                "modelo": k2.modelo,
                "replicacoes": &k2.reps,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Args, clap::Error> {
        Args::try_parse_from(std::iter::once("utsim linha").chain(args.iter().copied()))
    }

    #[test]
    fn colocar_takes_fields_negative_positions_and_department() {
        let a = parse(&[
            "lean_office",
            "colocar",
            "m.ron",
            "posto",
            "--campo",
            "tempo=Exp(mean: 20.0)",
            "--campo",
            "ca=0.9",
            "--posicao",
            "-2",
            "1.5",
            "--departamento",
            "Compras",
        ])
        .unwrap();
        let Some(Acao::Colocar {
            predefinicao,
            campos,
            posicao,
            departamento,
            ..
        }) = a.acao
        else {
            panic!("ação colocar");
        };
        assert_eq!(predefinicao, "posto");
        assert_eq!(posicao, Some(vec![-2.0, 1.5]));
        assert_eq!(departamento.as_deref(), Some("Compras"));
        let v = valores(&campos).unwrap();
        assert_eq!(v["tempo"], "Exp(mean: 20.0)");
        assert_eq!(v["ca"], "0.9");
    }

    #[test]
    fn relatorio_options_and_bad_input() {
        let a = parse(&[
            "lean_office",
            "relatorio",
            "m.ron",
            "-n",
            "3",
            "--html",
            "r.html",
        ])
        .unwrap();
        let Some(Acao::Relatorio { reps, html, .. }) = a.acao else {
            panic!("ação relatorio");
        };
        assert_eq!(reps, Some(3));
        assert_eq!(html, Some(PathBuf::from("r.html")));
        assert!(parse(&["lean_office", "colocar", "m.ron"]).is_err());
        assert!(parse(&["lean_office", "relatorio"]).is_err());
        assert!(valores(&["sem_igual".into()]).is_err());
        assert!(valores(&["=1".into()]).is_err());
        assert!(pack("nada").unwrap_err().contains("lean_office"));
    }
}

//! `utsim pessoa`: commands about the people of a model. `ajustar` fits the fatigue curve
//! and the rested-time distribution of one person from its time study (`person.study`,
//! `utsim_fit::fatigue`), prints the result and, with `--gravar`, writes it into the
//! profile (`person.times` and `person.fatigue`), so the run uses it.

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use utsim_edit::{Command, Document};
use utsim_fit::auto::AutoConfig;
use utsim_fit::fatigue::{PersonFit, fit_person};
use utsim_model::Model;

/// The `pessoa` command line, parsed on its own (as `tempos`, to keep the main parser
/// small).
#[derive(Parser)]
#[command(name = "utsim pessoa", about = "Pessoas do modelo")]
struct Args {
    #[command(subcommand)]
    acao: Acao,
}

#[derive(Subcommand)]
enum Acao {
    /// Ajusta a curva de fadiga e o tempo descansado de uma pessoa a partir das
    /// cronometragens dela (person.study); com --gravar, grava o resultado no perfil
    Ajustar(Ajustar),
}

#[derive(Parser)]
struct Ajustar {
    /// Modelo (.ron)
    model: PathBuf,
    /// Pessoa (o nome do Transporter com o perfil e o estudo)
    #[arg(long)]
    pessoa: String,
    /// Só esta tarefa (padrão: todas as do estudo)
    #[arg(long)]
    tarefa: Option<String>,
    /// Só este método (padrão: todos os da tarefa; "" é o método sem nome)
    #[arg(long)]
    metodo: Option<String>,
    /// Unidade de tempo do modelo: s, min ou h (as taxas do ajuste saem por hora)
    #[arg(long, default_value = "s")]
    unidade: String,
    /// Tarefa cuja curva vira a fadiga do perfil, quando há mais de uma (padrão: a que
    /// tem mais leituras)
    #[arg(long)]
    fadiga_de: Option<String>,
    /// Grava os tempos próprios e a fadiga no perfil da pessoa (sem isso, só mostra)
    #[arg(long)]
    gravar: bool,
    /// Com --gravar, grava neste arquivo em vez de regravar o modelo
    #[arg(long, requires = "gravar")]
    saida: Option<PathBuf>,
    /// Grava o resultado em JSON
    #[arg(long)]
    json: Option<PathBuf>,
}

/// Parses the arguments after `utsim pessoa` and runs the command.
pub fn run_args(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let a = Args::try_parse_from(std::iter::once("utsim pessoa".to_string()).chain(args))
        .unwrap_or_else(|e| e.exit());
    match a.acao {
        Acao::Ajustar(a) => ajustar(a),
    }
}

fn units_per_hour(unit: &str) -> Result<f64, String> {
    match unit {
        "s" => Ok(3600.0),
        "min" => Ok(60.0),
        "h" => Ok(1.0),
        other => Err(format!("unidade '{other}' desconhecida: use s, min ou h")),
    }
}

/// Number with up to `d` decimals, without trailing zeros.
fn num(x: f64, d: usize) -> String {
    let s = format!("{x:.d$}");
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

fn label(task: &str, method: &str) -> String {
    if method.is_empty() {
        format!("'{task}'")
    } else {
        format!("'{task}', método '{method}'")
    }
}

fn texto(fit: &PersonFit, unit: &str) -> String {
    let c = &fit.curve;
    let d = &fit.distribution;
    let mut out = format!(
        "Tarefa {}: {} leituras em {} horários\n",
        label(&c.task, &c.method),
        c.n,
        c.points.len()
    );
    out.push_str(&format!(
        "  tempo descansado t0 = {} {unit}; fadiga máxima acrescenta {} % (β = {})\n",
        num(c.t0, 3),
        num(c.beta * 100.0, 1),
        num(c.beta, 4)
    ));
    out.push_str(&format!(
        "  cansaço λ = {} por hora; recuperação μ = {} por hora ({})\n",
        num(c.work_per_hour, 4),
        num(c.recovery_per_hour, 4),
        if c.recovery_estimated {
            "estimada"
        } else {
            "padrão"
        }
    ));
    if let Some(ci) = &c.intervals {
        out.push_str(&format!(
            "  IC {} %: t0 entre {} e {}; acréscimo t0·β entre {} e {}\n",
            num(ci.level * 100.0, 0),
            num(ci.t0.0, 3),
            num(ci.t0.1, 3),
            num(ci.increase.0, 3),
            num(ci.increase.1, 3)
        ));
    }
    out.push_str("  hora  leituras  média  curva  fadiga\n");
    for p in &c.points {
        out.push_str(&format!(
            "  {:>4}  {:>8}  {:>5}  {:>5}  {:>6}\n",
            num(p.hour, 2),
            p.n,
            num(p.mean, 2),
            num(p.fitted, 2),
            num(p.level, 3)
        ));
    }
    out.push_str(&format!(
        "  distribuição do tempo descansado: {} — {:?}\n",
        d.family, d.dist
    ));
    // Why this one (the comparison with every candidate goes to the JSON).
    if let Some(j) = d.justification.first() {
        out.push_str(&format!("    {j}\n"));
    }
    for w in c.warnings.iter().chain(&d.warnings) {
        out.push_str(&format!("  aviso: {w}\n"));
    }
    out
}

fn ajustar(a: Ajustar) -> Result<(), Box<dyn std::error::Error>> {
    let per_hour = units_per_hour(&a.unidade)?;
    let model = Model::load(&a.model).map_err(|e| format!("{}: {e}", a.model.display()))?;
    let obj = model
        .objects
        .iter()
        .find(|o| o.name == a.pessoa)
        .ok_or_else(|| format!("objeto \"{}\" não existe", a.pessoa))?;
    let person = obj
        .person
        .as_deref()
        .ok_or_else(|| format!("{} não tem perfil de pessoa (person)", a.pessoa))?;
    let study = person
        .study
        .as_ref()
        .ok_or_else(|| format!("{} não tem estudo de tempos (study)", a.pessoa))?;

    // The (task, method) pairs to fit, in the order of the study.
    let mut pairs: Vec<(String, String)> = Vec::new();
    for r in &study.readings {
        let pair = (r.task.clone(), r.method.clone());
        let wanted = a.tarefa.as_ref().is_none_or(|t| *t == r.task)
            && a.metodo.as_ref().is_none_or(|m| *m == r.method);
        if wanted && !pairs.contains(&pair) {
            pairs.push(pair);
        }
    }
    if pairs.is_empty() {
        return Err(format!(
            "{}: nenhuma leitura{}{}",
            a.pessoa,
            a.tarefa
                .as_ref()
                .map(|t| format!(" da tarefa '{t}'"))
                .unwrap_or_default(),
            a.metodo
                .as_ref()
                .map(|m| format!(" pelo método '{m}'"))
                .unwrap_or_default()
        )
        .into());
    }
    // One own time per task: with several methods of a task, it is the methods'
    // comparison, and only one of them can go into the profile.
    if a.gravar {
        for (task, _) in &pairs {
            let methods: Vec<&str> = pairs
                .iter()
                .filter(|(t, _)| t == task)
                .map(|(_, m)| m.as_str())
                .collect();
            if methods.len() > 1 {
                return Err(format!(
                    "a tarefa '{task}' tem leituras de mais de um método ({}): escolha um com \
                     --metodo para gravar",
                    methods.join(", ")
                )
                .into());
            }
        }
    }

    let config = AutoConfig::default();
    let mut fits: Vec<PersonFit> = Vec::new();
    let mut failed: Vec<String> = Vec::new();
    println!("Pessoa {}: estudo de tempos", a.pessoa);
    for (task, method) in &pairs {
        match fit_person(person, task, method, &config) {
            Ok(fit) => {
                print!("{}", texto(&fit, &a.unidade));
                fits.push(fit);
            }
            Err(e) => {
                println!("Tarefa {}: não ajustada: {e}", label(task, method));
                failed.push(format!("{}: {e}", label(task, method)));
            }
        }
    }

    if let Some(p) = &a.json {
        let items: Vec<serde_json::Value> = fits
            .iter()
            .map(|f| {
                serde_json::json!({
                    "curva": f.curve,
                    "distribuicao": {
                        "familia": f.distribution.family,
                        "dist": f.distribution.dist,
                        "justificativa": f.distribution.justification,
                        "avisos": f.distribution.warnings,
                    },
                    "fadiga": f.curve.fatigue(per_hour),
                })
            })
            .collect();
        let v = serde_json::json!({
            "pessoa": a.pessoa,
            "unidade": a.unidade,
            "ajustes": items,
            "falhas": failed,
        });
        std::fs::write(p, serde_json::to_string_pretty(&v)?)?;
        println!("gravado em {}", p.display());
    }

    if !a.gravar {
        if !fits.is_empty() {
            println!("(nada foi gravado no modelo; use --gravar para gravar no perfil)");
        }
        return if fits.is_empty() {
            Err("nenhuma tarefa foi ajustada".into())
        } else {
            Ok(())
        };
    }
    if fits.is_empty() {
        return Err("nenhuma tarefa foi ajustada: nada a gravar".into());
    }

    // The curve that becomes the fatigue of the profile: the chosen task, or the one with
    // the most readings. Every fit gives its own time; the fatigue one goes last.
    let fatigue_at = match &a.fadiga_de {
        Some(t) => fits
            .iter()
            .position(|f| f.curve.task == *t)
            .ok_or_else(|| format!("--fadiga-de '{t}': a tarefa não foi ajustada"))?,
        None => (0..fits.len())
            .max_by_key(|&i| (fits[i].curve.n, std::cmp::Reverse(i)))
            .expect("at least one fit"),
    };
    // Every fit gives its own time; only the chosen one sets the fatigue, applied over
    // the fatigue the profile had.
    let mut new = person.clone();
    for f in &fits {
        f.apply(&mut new, per_hour);
    }
    new.fatigue = person.fatigue;
    fits[fatigue_at].apply(&mut new, per_hour);

    let mut doc = Document::open(&a.model)?;
    let value = format!("Some({})", ron::to_string(&new)?);
    doc.apply(Command::Set {
        key: format!("{}.person", a.pessoa),
        value,
    })?;
    let out = a.saida.unwrap_or(a.model);
    doc.save_as(&out)?;
    println!(
        "gravado em {}: tempos próprios de {}; fadiga pela tarefa '{}'",
        out.display(),
        fits.iter()
            .map(|f| format!("'{}'", f.curve.task))
            .collect::<Vec<_>>()
            .join(", "),
        fits[fatigue_at].curve.task
    );
    for issue in doc.issues() {
        println!(
            "pendência: {}{}",
            issue.object.map(|o| format!("{o}: ")).unwrap_or_default(),
            issue.message
        );
    }
    Ok(())
}

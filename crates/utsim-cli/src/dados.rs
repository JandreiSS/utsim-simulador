//! `utsim dados`: fits every distribution field of a model that is fed by real data
//! (`data: [..]`), prints the choice for each one and writes the reports.

use std::path::PathBuf;

use utsim_model::Model;
use utsim_model::dados::{self, DataStatus};

#[derive(clap::Args)]
pub struct Args {
    /// Modelo (.ron) com campos alimentados por dados reais (`data: [..]`)
    model: PathBuf,
    /// Pasta dos relatórios: um HTML e um CSV por campo, mais index.html
    #[arg(long)]
    relatorio: Option<PathBuf>,
    /// Grava no modelo o resultado de cada ajuste (família, distribuição e hash dos
    /// dados), para reprodutibilidade e para avisar quando os dados mudarem
    #[arg(long)]
    gravar: bool,
}

fn status_text(status: &DataStatus) -> &'static str {
    match status {
        DataStatus::Pending => "pendente",
        DataStatus::Fitted => "ajustado agora",
        DataStatus::Reused => "registro reaproveitado (dados iguais)",
        DataStatus::DataChanged { .. } => "DADOS MUDARAM: reajustado",
        DataStatus::SettingsChanged => "opções mudaram: reajustado",
        DataStatus::RecordOnly { .. } => "dados ilegíveis: usado o registro",
    }
}

pub fn run(args: Args) -> Result<(), String> {
    let base = args.model.parent();
    // With --gravar the model is opened as a document (same fit) and saved with the records.
    let model = if args.gravar {
        let mut doc = utsim_edit::Document::open(&args.model).map_err(|e| e.to_string())?;
        if !doc.model().data.is_empty() {
            doc.save().map_err(|e| e.to_string())?;
        }
        doc.model().clone()
    } else {
        Model::load(&args.model).map_err(|e| e.to_string())?
    };
    if model.data.is_empty() {
        println!(
            "{} não tem campos alimentados por dados reais (`data: [..]`). Campos de \
             distribuição que podem usar dados:",
            args.model.display()
        );
        for f in dados::dist_fields(&model) {
            println!("  {f}");
        }
        return Ok(());
    }
    println!(
        "Dados reais: {} ({} campo(s))\n",
        model.name,
        model.data.len()
    );
    for input in &model.data {
        let (family, ron) = input.fitted.as_ref().map_or(("-".into(), "-".into()), |r| {
            (
                r.family.clone(),
                ron::ser::to_string(&r.dist).unwrap_or_default(),
            )
        });
        println!("{}", input.field);
        println!("  dados:        {}", input.source_label());
        println!("  situação:     {}", status_text(&input.status));
        println!("  distribuição: {family}");
        println!("  RON:          {ron}");
        if let Some(rec) = &input.fitted {
            println!("  n = {}, hash {}", rec.n, rec.hash);
        }
        println!();
    }
    for w in model.warnings() {
        eprintln!("aviso: {w}");
    }

    if let Some(dir) = &args.relatorio {
        let (written, failed) = dados::write_reports(&model, base, dir)
            .map_err(|e| format!("não foi possível gravar em '{}': {e}", dir.display()))?;
        println!(
            "relatórios gravados em {} (abra {})",
            dir.display(),
            written[0].display()
        );
        if failed > 0 {
            eprintln!("aviso: {failed} campo(s) não puderam ser analisados (veja o índice)");
        }
    }
    if args.gravar {
        println!("ajustes registrados em {}", args.model.display());
    }
    Ok(())
}

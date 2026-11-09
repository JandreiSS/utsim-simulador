//! The flow of a background job seen from the interface (passo 7, parte 3): the progress
//! bar while it runs and, when it ends, the status message and whether the main scene goes
//! on with the result. `UtsimUi::poll_job` uses it; the main scene only keeps the job and
//! what to do with its result.

/// The progress bar of a job: `(fraction, with percent)`. A job that cannot count
/// (`total` 0) gets a bar that goes back and forth with the clock (`ms`).
pub fn barra(done: i64, total: i64, ms: u64) -> (f64, bool) {
    if total > 0 {
        ((done as f64 / total as f64).clamp(0.0, 1.0), true)
    } else {
        ((ms as f64 / 400.0).sin().abs(), false)
    }
}

/// How a job ended: the status message (empty: none) and whether the main scene calls
/// what it does with the result. A failed or cancelled run of replications still goes on
/// (the main scene goes back to Editar and says why); other jobs stop at the message.
pub fn fim(what: &str, kind: &str, ok: bool, cancelled: bool, error: &str) -> (String, bool) {
    if ok {
        return (String::new(), true);
    }
    let msg = if cancelled {
        format!("{what}: cancelado.")
    } else {
        format!("{what}: erro: {error}")
    };
    (msg, kind == "replicacoes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bar_counts_or_swings() {
        assert_eq!(barra(3, 4, 0), (0.75, true));
        assert_eq!(barra(9, 4, 0), (1.0, true));
        let (p, percent) = barra(0, 0, 628);
        assert!(!percent && (0.0..=1.0).contains(&p));
        assert_ne!(barra(0, 0, 100).0, barra(0, 0, 500).0, "it moves");
    }

    #[test]
    fn the_end_says_what_happened() {
        assert_eq!(
            fim("Rodando", "replicacoes", true, false, ""),
            (String::new(), true)
        );
        assert_eq!(
            fim("Varredura", "varredura", false, true, "cancelado"),
            ("Varredura: cancelado.".to_owned(), false)
        );
        assert_eq!(
            fim(
                "Rodando replicações",
                "replicacoes",
                false,
                false,
                "falta memória"
            ),
            ("Rodando replicações: erro: falta memória".to_owned(), true)
        );
    }
}

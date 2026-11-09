//! The one-writer lock of the model open in Editar (`docs/plano_integracao_familia.md`,
//! section 3.2), the same `utsim_pasta::trava` UTSim Qualidade and Segurança use, with app
//! `utsim`. Opening or saving a file takes its lock; the main scene renews it every minute;
//! closing, opening another or a new model releases it. When another app or person holds
//! it, the model stays open, the Qualidade, Segurança and Manutenção tabs are read only and
//! "Salvar" refuses (with who edits), so the other writer's work is not overwritten;
//! "Salvar como" to another file is still possible. A lock the other side lets go is taken
//! at the next renewal.

use std::path::{Path, PathBuf};

use utsim_pasta::trava::{self, Estado, Trava};

/// The app name written in the lock.
pub const APP: &str = "utsim";

/// The person and the computer, as the lock records them (from the environment).
pub fn pessoa() -> String {
    std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .ok()
        .filter(|p| !p.trim().is_empty())
        .unwrap_or_else(|| "pessoa".into())
}

pub fn maquina() -> String {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .ok()
        .filter(|m| !m.trim().is_empty())
        .unwrap_or_else(|| "maquina".into())
}

/// The name of an app as people read it.
pub fn nome_app(app: &str) -> &str {
    match app {
        "utsim" => "UTSim",
        "qualidade" => "UTSim Qualidade",
        "seguranca" => "UTSim Segurança do Trabalho",
        outro => outro,
    }
}

/// `HH:MM` (UTC) of an instant in milliseconds.
fn hora(ms: u64) -> String {
    let s = ms / 1000 % 86_400;
    format!("{:02}:{:02}", s / 3600, s / 60 % 60)
}

/// The lock of the model open in Editar.
#[derive(Debug)]
pub struct TravaModelo {
    pessoa: String,
    maquina: String,
    arquivo: Option<PathBuf>,
    estado: Option<Estado>,
    /// The lock handed to another app (`ceder`): the app and until when we wait for it to
    /// take it, without taking it back.
    cedida: Option<(String, u64)>,
}

/// How long a lock handed to another app waits for it (the app may take a while to open).
pub const ESPERA_CEDIDA_MS: u64 = 60_000;

impl Default for TravaModelo {
    fn default() -> Self {
        TravaModelo::nova(&pessoa(), &maquina())
    }
}

impl TravaModelo {
    pub fn nova(pessoa: &str, maquina: &str) -> TravaModelo {
        TravaModelo {
            pessoa: pessoa.into(),
            maquina: maquina.into(),
            arquivo: None,
            estado: None,
            cedida: None,
        }
    }

    /// Takes the lock of `arquivo` (releasing the one held before). An error writing the
    /// lock leaves the model editable: the lock protects, it never blocks the work.
    pub fn abrir(&mut self, arquivo: &Path) {
        if self.arquivo.as_deref() == Some(arquivo) && self.pode_gravar() {
            self.renovar();
            return;
        }
        self.soltar();
        self.arquivo = Some(arquivo.to_path_buf());
        self.estado =
            trava::travar(arquivo, APP, &self.pessoa, &self.maquina, trava::agora_ms()).ok();
    }

    /// Releases our lock (a new model, another file, closing).
    pub fn soltar(&mut self) {
        if let (Some(a), Some(Estado::Minha)) = (&self.arquivo, &self.estado) {
            let _ = trava::soltar(a, APP, &self.pessoa, &self.maquina);
        }
        self.arquivo = None;
        self.estado = None;
        self.cedida = None;
    }

    /// Hands the lock to `app` (`qualidade`, `seguranca`), which is opening the model: ours is
    /// released and not taken back at the renewals until that app holds it (then the model
    /// is read only, as with any other writer) or [`ESPERA_CEDIDA_MS`] goes by.
    pub fn ceder(&mut self, app: &str, agora_ms: u64) {
        if let (Some(a), Some(Estado::Minha)) = (&self.arquivo, &self.estado) {
            let _ = trava::soltar(a, APP, &self.pessoa, &self.maquina);
        }
        if self.arquivo.is_some() {
            self.estado = None;
            self.cedida = Some((app.to_owned(), agora_ms + ESPERA_CEDIDA_MS));
        }
    }

    /// Renews our lock, or takes it when the other side let it go.
    pub fn renovar(&mut self) {
        let Some(a) = self.arquivo.clone() else {
            return;
        };
        if let Some((_, ate)) = &self.cedida {
            let agora = trava::agora_ms();
            match trava::ler(&a, agora) {
                Some(t) => {
                    self.cedida = None;
                    self.estado = Some(Estado::DeOutro(t));
                    return;
                }
                None if agora < *ate => return,
                None => self.cedida = None,
            }
        }
        if let Ok(e) = trava::renovar(&a, APP, &self.pessoa, &self.maquina, trava::agora_ms()) {
            self.estado = Some(e);
        }
    }

    /// Who edits the file, when it is not us.
    pub fn de_outro(&self) -> Option<&Trava> {
        match &self.estado {
            Some(Estado::DeOutro(t)) => Some(t),
            _ => None,
        }
    }

    /// The model can be saved over its file (ours, or no lock to speak of).
    pub fn pode_gravar(&self) -> bool {
        self.de_outro().is_none() && self.cedida.is_none()
    }

    /// "Em edição por ana no UTSim Qualidade desde 14:05", when someone else holds it.
    pub fn aviso(&self) -> Option<String> {
        if let Some((app, _)) = &self.cedida {
            return Some(format!(
                "Aberto no {}, que vai editar o modelo",
                nome_app(app)
            ));
        }
        self.de_outro().map(|t| {
            format!(
                "Em edição por {} no {} desde {}",
                t.pessoa,
                nome_app(&t.app),
                hora(t.desde_ms)
            )
        })
    }

    /// Saving to `arquivo`: refused when someone else holds its lock (with who), else the
    /// lock is ours (taken when the file is a new one, renewed when it is the same).
    pub fn antes_de_gravar(&mut self, arquivo: &Path) -> Result<(), String> {
        if self.arquivo.as_deref() == Some(arquivo) {
            self.renovar();
        } else {
            self.abrir(arquivo);
        }
        match self.aviso() {
            Some(a) => Err(format!(
                "{a}. Para não apagar o trabalho de quem edita, salve com outro nome (Salvar como) ou espere a pessoa fechar o modelo."
            )),
            None => Ok(()),
        }
    }
}

impl Drop for TravaModelo {
    fn drop(&mut self) {
        self.soltar();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn modelo(dir: &Path) -> PathBuf {
        let m = dir.join("linha.ron");
        std::fs::write(&m, "Model()").unwrap();
        m
    }

    #[test]
    fn a_lock_handed_over_waits_for_the_other_app() {
        let dir = tempfile::tempdir().unwrap();
        let m = modelo(dir.path());
        let mut t = TravaModelo::nova("ana", "pc1");
        t.abrir(&m);
        t.ceder("qualidade", trava::agora_ms());
        assert!(trava::ler(&m, trava::agora_ms()).is_none(), "released");
        assert!(!t.pode_gravar());
        assert!(t.aviso().unwrap().contains("UTSim Qualidade"));
        // Not taken back while the other app opens.
        t.renovar();
        assert!(trava::ler(&m, trava::agora_ms()).is_none());
        // The other app takes it: read only, with who.
        trava::travar(&m, "qualidade", "ana", "pc1", trava::agora_ms()).unwrap();
        t.renovar();
        assert!(t.de_outro().is_some());
        assert!(t.aviso().unwrap().starts_with("Em edição por ana"));
        // It closes: ours again at the next renewal.
        trava::soltar(&m, "qualidade", "ana", "pc1").unwrap();
        t.renovar();
        assert!(t.pode_gravar());
        // A handover nobody takes ends after the wait.
        t.ceder("seguranca", 0);
        t.renovar();
        assert!(t.pode_gravar());
    }

    #[test]
    fn opening_takes_the_lock_and_closing_lets_it_go() {
        let dir = tempfile::tempdir().unwrap();
        let m = modelo(dir.path());
        let mut t = TravaModelo::nova("ana", "pc1");
        t.abrir(&m);
        assert!(t.pode_gravar());
        let lido = trava::ler(&m, trava::agora_ms()).unwrap();
        assert_eq!((lido.app.as_str(), lido.pessoa.as_str()), ("utsim", "ana"));
        t.soltar();
        assert!(trava::ler(&m, trava::agora_ms()).is_none());
        // Dropping releases it too.
        {
            let mut t = TravaModelo::nova("ana", "pc1");
            t.abrir(&m);
        }
        assert!(trava::ler(&m, trava::agora_ms()).is_none());
    }

    #[test]
    fn another_writer_makes_saving_refuse_and_its_release_is_noticed() {
        let dir = tempfile::tempdir().unwrap();
        let m = modelo(dir.path());
        assert_eq!(
            trava::travar(&m, "qualidade", "bruno", "pc2", trava::agora_ms()).unwrap(),
            Estado::Minha
        );
        let mut t = TravaModelo::nova("ana", "pc1");
        t.abrir(&m);
        assert!(!t.pode_gravar());
        let aviso = t.aviso().unwrap();
        assert!(
            aviso.starts_with("Em edição por bruno no UTSim Qualidade desde "),
            "{aviso}"
        );
        let e = t.antes_de_gravar(&m).unwrap_err();
        assert!(e.contains("Salvar como"), "{e}");
        // Saving as another file is ours.
        let outro = dir.path().join("copia.ron");
        assert!(t.antes_de_gravar(&outro).is_ok());
        assert!(
            trava::ler(&m, trava::agora_ms()).is_some(),
            "bruno's lock stays"
        );
        // Back on the first file: once bruno lets go, the next renewal takes it.
        t.abrir(&m);
        assert!(!t.pode_gravar());
        trava::soltar(&m, "qualidade", "bruno", "pc2").unwrap();
        t.renovar();
        assert!(t.pode_gravar());
        assert!(t.antes_de_gravar(&m).is_ok());
    }
}

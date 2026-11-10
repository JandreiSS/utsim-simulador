//! "Abrir no UTSim Qualidade" and "Abrir no UTSim Segurança" of a node's tabs (front C of
//! the family integration, `docs/plano_integracao_familia.md`, section 2.2): the program is
//! looked up through the launcher's installation, then next to this program, then on the
//! `PATH` (section 2.1), and opened with the model and the node:
//!
//! ```text
//! utsim-qualidade --utsim-abrir=<absolute .ron> --utsim-no=<object name>
//! ```
//!
//! Each argument goes whole to the process (no shell), so spaces and accents need no
//! quoting. The other app opens on its Modelo screen with the node selected and answers
//! nothing; it takes the model's lock after the UTSim 3D lets it go.

use std::path::{Path, PathBuf};

/// One of the apps a node can open in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum App {
    Qualidade,
    Seguranca,
}

impl App {
    pub fn de(nome: &str) -> Option<App> {
        match nome {
            "qualidade" => Some(App::Qualidade),
            "seguranca" => Some(App::Seguranca),
            _ => None,
        }
    }

    /// The product id in the launcher's catalog.
    pub fn produto(self) -> &'static str {
        match self {
            App::Qualidade => "qualidade",
            App::Seguranca => "seguranca",
        }
    }

    /// The program's file name (without `.exe`).
    pub fn programa(self) -> &'static str {
        match self {
            App::Qualidade => "utsim-qualidade",
            App::Seguranca => "utsim-seguranca",
        }
    }

    pub fn nome(self) -> &'static str {
        match self {
            App::Qualidade => "UTSim Qualidade",
            App::Seguranca => "UTSim Segurança do Trabalho",
        }
    }
}

/// The program, its default arguments and the folder to run it in.
pub type Comando = (PathBuf, Vec<String>, PathBuf);

fn arquivo(programa: &str) -> String {
    if cfg!(windows) {
        format!("{programa}.exe")
    } else {
        programa.to_owned()
    }
}

/// Looks for `programa` in `dirs` (next to this program, then each folder of the `PATH`).
pub fn procurar(programa: &str, dirs: &[PathBuf]) -> Option<Comando> {
    let nome = arquivo(programa);
    dirs.iter()
        .map(|d| d.join(&nome))
        .find(|p| p.is_file())
        .map(|p| {
            let dir = p.parent().map(Path::to_path_buf).unwrap_or_default();
            (p, Vec::new(), dir)
        })
}

/// Where the app is: the launcher's active version, then next to this program, then the
/// `PATH`.
pub fn localizar(app: App) -> Option<Comando> {
    use utsim_launcher_core::install::{Layout, launch_command};
    if let Some(raiz) = Layout::default_root()
        && let Ok(c) = launch_command(&Layout::new(raiz), app.produto())
    {
        return Some(c);
    }
    let mut dirs = Vec::new();
    if let Some(d) = std::env::current_exe()
        .ok()
        .and_then(|e| e.parent().map(Path::to_path_buf))
    {
        dirs.push(d);
    }
    if let Some(path) = std::env::var_os("PATH") {
        dirs.extend(std::env::split_paths(&path));
    }
    procurar(app.programa(), &dirs)
}

/// The arguments: the program's own, then the model and the node.
pub fn argumentos(padrao: &[String], modelo: &Path, no: &str) -> Vec<String> {
    let mut a = padrao.to_vec();
    a.push(format!("--utsim-abrir={}", modelo.display()));
    a.push(format!("--utsim-no={no}"));
    a
}

/// What is missing, said to the user.
pub fn falta(app: App) -> String {
    format!(
        "O {} não está instalado neste computador: instale pelo launcher do UTSim.",
        app.nome()
    )
}

/// Opens `app` on `no` of the model saved at `modelo` (absolute). Does not wait.
pub fn abrir(app: App, modelo: &Path, no: &str) -> Result<(), String> {
    let (exe, padrao, dir) = localizar(app).ok_or_else(|| falta(app))?;
    std::process::Command::new(&exe)
        .args(argumentos(&padrao, modelo, no))
        .current_dir(dir)
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Não consegui abrir {}: {e}", exe.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_arguments_go_whole() {
        let m = Path::new("C:/Modelos/linha de montagem.ron");
        assert_eq!(
            argumentos(&["--x".into()], m, "prensa 2"),
            [
                "--x",
                "--utsim-abrir=C:/Modelos/linha de montagem.ron",
                "--utsim-no=prensa 2"
            ]
        );
        assert_eq!(App::de("seguranca"), Some(App::Seguranca));
        assert_eq!(App::de("x"), None);
        assert!(falta(App::Qualidade).contains("UTSim Qualidade"));
    }

    #[test]
    fn the_program_is_found_in_the_first_folder_that_has_it() {
        let a = tempfile::tempdir().unwrap();
        let b = tempfile::tempdir().unwrap();
        let dirs = [a.path().to_path_buf(), b.path().to_path_buf()];
        assert!(procurar("utsim-qualidade", &dirs).is_none());
        let exe = b.path().join(arquivo("utsim-qualidade"));
        std::fs::write(&exe, b"").unwrap();
        let (p, args, dir) = procurar("utsim-qualidade", &dirs).unwrap();
        assert_eq!(p, exe);
        assert!(args.is_empty());
        assert_eq!(dir, b.path());
    }
}

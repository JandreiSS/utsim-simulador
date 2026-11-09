//! Whether the model file changed on disk since the UTSim 3D opened or saved it
//! (`docs/plano_integracao_familia.md`, section 3.2): UTSim Qualidade and UTSim Segurança
//! do Trabalho edit the same `.ron` while they hold its lock. Without a local change the 3D
//! reloads it; with one, saving over the file is refused so the other app's work is not
//! lost ("Salvar como" still works). The mark is the size and a hash of the contents, so a
//! file touched without a change does not count.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

/// What the file held when it was last read or written by us.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Marca {
    tamanho: u64,
    hash: u64,
}

/// The mark of `path` now (`None`: missing or unreadable).
pub fn marca(path: &Path) -> Option<Marca> {
    let bytes = std::fs::read(path).ok()?;
    let mut h = DefaultHasher::new();
    bytes.hash(&mut h);
    Some(Marca {
        tamanho: bytes.len() as u64,
        hash: h.finish(),
    })
}

/// What to do about the file on disk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disco {
    /// Same as we left it (or nothing to compare: a new model, a file that went away).
    Igual,
    /// Changed by someone else and nothing local to lose: read it again.
    Recarregar,
    /// Changed by someone else while there are local changes: warn, and refuse "Salvar".
    Conflito,
}

/// Compares the mark we kept with the file's mark now; `pendente`: local changes not
/// saved.
pub fn decidir(nossa: Option<Marca>, agora: Option<Marca>, pendente: bool) -> Disco {
    match (nossa, agora) {
        (Some(a), Some(b)) if a != b => {
            if pendente {
                Disco::Conflito
            } else {
                Disco::Recarregar
            }
        }
        _ => Disco::Igual,
    }
}

/// The refusal of "Salvar" over a file someone else changed.
pub const RECUSA: &str = "Outro app gravou este modelo depois que você o abriu. Para não apagar o trabalho dele, use Salvar como (outro arquivo), ou abra o modelo de novo para partir do que está no disco.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_change_on_disk_reloads_or_conflicts() {
        let dir = tempfile::tempdir().unwrap();
        let f = dir.path().join("linha.ron");
        std::fs::write(&f, "Model(name: \"a\")").unwrap();
        let nossa = marca(&f);
        assert!(nossa.is_some());
        assert_eq!(decidir(nossa, marca(&f), true), Disco::Igual);
        // The same contents written again do not count.
        std::fs::write(&f, "Model(name: \"a\")").unwrap();
        assert_eq!(decidir(nossa, marca(&f), false), Disco::Igual);
        std::fs::write(&f, "Model(name: \"b\")").unwrap();
        assert_eq!(decidir(nossa, marca(&f), false), Disco::Recarregar);
        assert_eq!(decidir(nossa, marca(&f), true), Disco::Conflito);
        // A new model, or the file gone: nothing to do.
        assert_eq!(decidir(None, marca(&f), true), Disco::Igual);
        std::fs::remove_file(&f).unwrap();
        assert_eq!(marca(&f), None);
        assert_eq!(decidir(nossa, None, false), Disco::Igual);
    }
}

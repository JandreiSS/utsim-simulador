//! Scripted UI tests: the script format and the pure assertion logic.
//!
//! A script (`roteiro`) is a RON list of steps (`Passo`) that the `utsim-view`
//! binary replays with synthetic mouse and keyboard input
//! (`utsim-view modelo.ron --roteiro roteiro.ron --capturas dir`). This module only
//! holds what needs no window: parsing, the editor-state snapshot the assertions run
//! against, and the summary line written to `log.txt`. The runner is `src/script.rs`
//! in the binary. See `docs/testes.md`, section "Testes roteirizados da interface".
//!
//! ```text
//! [
//!     Esperar(quadros: 10),
//!     EsperarSegundos(1.5),
//!     EsperarAte(Modo(Simular), 30.0),        // polls a condition, fails on timeout
//!     MoverPara(Objeto("fila")),              // or Mundo(x, y), Tela(px, py), Botao("Salvar")
//!     Clicar(Esquerdo),                       // also Pressionar(..), Soltar(..); Direito
//!     Arrastar(de: Objeto("torno"), para: Mundo(8.0, 2.0)),
//!     Rolar(3.0),                             // mouse wheel, in lines (positive zooms in)
//!     Tecla("Delete"),                        // "C", "Esc", "Enter", "F5", "SetaCima", ...
//!     Atalho(["Ctrl", "Z"]),
//!     Digitar("texto"),                       // into the focused egui field; {capturas} = output dir
//!     ClicarBotao("▶ Simular"),               // visible egui label
//!     Captura("nome"),                        // <capturas>/nome.png
//!     Afirmar(ObjetosIgual(5)),
//!     IrPara(1806.0),                         // Simular: pause and jump the clock
//!     Focar(objeto: "empilhadeira_1", distancia: 9.0, giro: Some(0.6), inclinacao: None),
//! ]
//! ```
//!
//! Targets (`Alvo`): `Objeto(nome)` is the center of the block, `Mundo(x, y)` is a
//! model ground coordinate, `Tela(px, py)` is a logical window pixel, `Botao(rótulo)`
//! is the center of an egui text with that label, and `Deslocado(alvo, dx, dy)` moves
//! another target by pixels (to reach the field next to a label).

use serde::Deserialize;

/// Mouse button of a script step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Botao {
    Esquerdo,
    Direito,
}

/// Where the pointer goes.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Alvo {
    /// Center of the block with this name (projected with the active camera).
    Objeto(String),
    /// Model ground coordinates `(x, y)`.
    Mundo(f64, f64),
    /// Logical window pixels.
    Tela(f32, f32),
    /// Center of the visible egui text equal to (or containing) this label.
    Botao(String),
    /// Another target moved by `(dx, dy)` pixels.
    Deslocado(Box<Alvo>, f32, f32),
}

/// Editor mode in assertions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ModoEditor {
    Editar,
    Simular,
}

/// A check on the editor state (used by `Afirmar` and `EsperarAte`).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Condicao {
    ObjetosIgual(usize),
    LigacoesIgual(usize),
    ProblemasIgual(usize),
    Selecionado(Option<String>),
    Modo(ModoEditor),
    Sujo(bool),
    /// Tool: `"Selecionar"`, `"Colocar:Queue"` or `"Ligar:fila"`.
    Ferramenta(String),
    ObjetoExiste(String),
    /// Model ground position `(x, y)` of an object, tolerance 0.05.
    PosicaoDe(String, (f64, f64)),
    /// Same with an explicit tolerance.
    PosicaoPerto(String, (f64, f64), f64),
    StatusContem(String),
    TituloContem(String),
    /// Current simulated time (in `Simular`) is at least this.
    TempoSimuladoMin(f64),
    /// Number of token flows in the model.
    FluxosIgual(usize),
    /// Number of activities of the named flow.
    AtividadesIgual(String, usize),
    /// Selected activity in the Fluxos panel.
    AtividadeSelecionada(Option<String>),
    /// Whether the Fluxos panel is open.
    PainelFluxos(bool),
    Nao(Box<Condicao>),
}

/// One step of a script.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Passo {
    Esperar {
        quadros: u32,
    },
    EsperarSegundos(f64),
    /// Waits until the condition holds; fails after this many seconds.
    EsperarAte(Condicao, f64),
    MoverPara(Alvo),
    Clicar(Botao),
    Pressionar(Botao),
    Soltar(Botao),
    Arrastar {
        de: Alvo,
        para: Alvo,
    },
    Rolar(f32),
    Tecla(String),
    Atalho(Vec<String>),
    Digitar(String),
    ClicarBotao(String),
    Captura(String),
    Afirmar(Condicao),
    /// Pauses the playback and jumps to this simulated time (`Simular`).
    IrPara(f64),
    /// Points the camera at the block called `objeto` (where it is now), at `distancia`
    /// metres; `giro` and `inclinacao` (radians) set the orbit angles when given.
    Focar {
        objeto: String,
        distancia: f32,
        #[serde(default)]
        giro: Option<f32>,
        #[serde(default)]
        inclinacao: Option<f32>,
    },
}

/// Parses a script.
pub fn parse(text: &str) -> Result<Vec<Passo>, String> {
    ron::from_str(text).map_err(|e| format!("roteiro inválido: {e}"))
}

/// Snapshot of the editor, what the assertions look at.
#[derive(Debug, Clone, Default)]
pub struct Estado {
    pub modo: String,
    pub selecionado: Option<String>,
    pub ferramenta: String,
    pub objetos: usize,
    pub ligacoes: usize,
    pub problemas: usize,
    pub sujo: bool,
    pub titulo: String,
    pub status: String,
    /// Simulated time, when in `Simular`.
    pub tempo: Option<f64>,
    /// Model positions by object name.
    pub posicoes: Vec<(String, [f64; 3])>,
    /// Token flows: name and number of activities.
    pub fluxos: Vec<(String, usize)>,
    /// Activity selected in the Fluxos panel.
    pub atividade: Option<String>,
    pub painel_fluxos: bool,
}

fn igual<T: PartialEq + std::fmt::Display>(rotulo: &str, tem: T, quer: T) -> Result<(), String> {
    if tem == quer {
        Ok(())
    } else {
        Err(format!("{rotulo}: esperado {quer}, encontrado {tem}"))
    }
}

impl Estado {
    /// Checks one condition; the error says what was expected and what was found.
    pub fn verificar(&self, c: &Condicao) -> Result<(), String> {
        match c {
            Condicao::ObjetosIgual(n) => igual("objetos", self.objetos, *n),
            Condicao::LigacoesIgual(n) => igual("ligações", self.ligacoes, *n),
            Condicao::ProblemasIgual(n) => igual("problemas", self.problemas, *n),
            Condicao::Selecionado(s) => {
                if &self.selecionado == s {
                    Ok(())
                } else {
                    Err(format!(
                        "selecionado: esperado {s:?}, encontrado {:?}",
                        self.selecionado
                    ))
                }
            }
            Condicao::Modo(m) => {
                let quer = match m {
                    ModoEditor::Editar => "Editar",
                    ModoEditor::Simular => "Simular",
                };
                igual("modo", self.modo.as_str(), quer)
            }
            Condicao::Sujo(b) => igual("sujo", self.sujo, *b),
            Condicao::Ferramenta(t) => igual("ferramenta", self.ferramenta.as_str(), t.as_str()),
            Condicao::ObjetoExiste(n) => {
                if self.posicoes.iter().any(|(m, _)| m == n) {
                    Ok(())
                } else {
                    Err(format!("objeto \"{n}\" não existe"))
                }
            }
            Condicao::PosicaoDe(n, p) => self.posicao(n, *p, 0.05),
            Condicao::PosicaoPerto(n, p, tol) => self.posicao(n, *p, *tol),
            Condicao::StatusContem(t) => {
                if self.status.contains(t.as_str()) {
                    Ok(())
                } else {
                    Err(format!("status \"{}\" não contém \"{t}\"", self.status))
                }
            }
            Condicao::TituloContem(t) => {
                if self.titulo.contains(t.as_str()) {
                    Ok(())
                } else {
                    Err(format!("título \"{}\" não contém \"{t}\"", self.titulo))
                }
            }
            Condicao::TempoSimuladoMin(min) => match self.tempo {
                Some(t) if t >= *min => Ok(()),
                Some(t) => Err(format!("tempo simulado {t:.3} < {min}")),
                None => Err("fora do modo Simular".to_string()),
            },
            Condicao::FluxosIgual(n) => igual("fluxos", self.fluxos.len(), *n),
            Condicao::AtividadesIgual(fluxo, n) => {
                match self.fluxos.iter().find(|(f, _)| f == fluxo) {
                    Some((_, tem)) => igual(&format!("atividades de {fluxo}"), *tem, *n),
                    None => Err(format!("fluxo \"{fluxo}\" não existe")),
                }
            }
            Condicao::AtividadeSelecionada(a) => {
                if &self.atividade == a {
                    Ok(())
                } else {
                    Err(format!(
                        "atividade selecionada: esperado {a:?}, encontrado {:?}",
                        self.atividade
                    ))
                }
            }
            Condicao::PainelFluxos(b) => igual("painel de fluxos aberto", self.painel_fluxos, *b),
            Condicao::Nao(inner) => match self.verificar(inner) {
                Ok(()) => Err(format!("a condição negada é verdadeira: {inner:?}")),
                Err(_) => Ok(()),
            },
        }
    }

    fn posicao(&self, nome: &str, quer: (f64, f64), tol: f64) -> Result<(), String> {
        let Some((_, p)) = self.posicoes.iter().find(|(m, _)| m == nome) else {
            return Err(format!("objeto \"{nome}\" não existe"));
        };
        if (p[0] - quer.0).abs() <= tol && (p[1] - quer.1).abs() <= tol {
            Ok(())
        } else {
            Err(format!(
                "posição de {nome}: esperado ({}, {}) ±{tol}, encontrado ({}, {})",
                quer.0, quer.1, p[0], p[1]
            ))
        }
    }

    /// The state part of a `log.txt` line.
    pub fn resumo(&self) -> String {
        let tempo = self
            .tempo
            .map_or(String::new(), |t| format!(" tempo={t:.3}"));
        format!(
            "modo={} selecionado={} ferramenta={} objetos={} ligacoes={} problemas={} sujo={} titulo=\"{}\" status=\"{}\"{tempo}",
            self.modo,
            self.selecionado.as_deref().unwrap_or("-"),
            self.ferramenta,
            self.objetos,
            self.ligacoes,
            self.problemas,
            self.sujo,
            self.titulo,
            self.status
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_every_kind_of_step() {
        let text = r#"[
            Esperar(quadros: 10),
            EsperarSegundos(1.5),
            EsperarAte(Modo(Simular), 30.0),
            MoverPara(Objeto("fila")),
            MoverPara(Mundo(1.0, -2.5)),
            MoverPara(Tela(100.0, 200.0)),
            MoverPara(Deslocado(Botao("tempo"), 90.0, 0.0)),
            Clicar(Esquerdo),
            Pressionar(Direito),
            Soltar(Direito),
            Arrastar(de: Objeto("torno"), para: Mundo(8.0, 2.0)),
            Rolar(3.0),
            Tecla("Delete"),
            Atalho(["Ctrl", "Z"]),
            Digitar("texto"),
            ClicarBotao("▶ Simular"),
            Captura("nome"),
            Afirmar(Selecionado(Some("fila"))),
            Afirmar(PosicaoDe("fila", (1.0, 2.0))),
            Afirmar(Nao(Sujo(true))),
        ]"#;
        let steps = parse(text).unwrap();
        assert_eq!(steps.len(), 20);
        assert_eq!(steps[0], Passo::Esperar { quadros: 10 });
        assert_eq!(
            steps[10],
            Passo::Arrastar {
                de: Alvo::Objeto("torno".into()),
                para: Alvo::Mundo(8.0, 2.0)
            }
        );
    }

    #[test]
    fn parses_playback_and_camera_steps() {
        let steps = parse(
            r#"[IrPara(1806.0), Focar(objeto: "agv_1", distancia: 6.0, giro: Some(0.5)),
                Focar(objeto: "x", distancia: 2.0, giro: None, inclinacao: Some(0.3))]"#,
        )
        .unwrap();
        assert_eq!(steps[0], Passo::IrPara(1806.0));
        assert_eq!(
            steps[1],
            Passo::Focar {
                objeto: "agv_1".into(),
                distancia: 6.0,
                giro: Some(0.5),
                inclinacao: None,
            }
        );
        assert!(matches!(
            steps[2],
            Passo::Focar {
                inclinacao: Some(_),
                ..
            }
        ));
    }

    #[test]
    fn rejects_unknown_steps() {
        assert!(parse("[Dancar]").unwrap_err().contains("roteiro inválido"));
    }

    fn state() -> Estado {
        Estado {
            modo: "Editar".into(),
            selecionado: Some("fila".into()),
            ferramenta: "Selecionar".into(),
            objetos: 4,
            ligacoes: 3,
            problemas: 0,
            sujo: true,
            titulo: "UTSim - novo.ron*".into(),
            status: "Salvo em novo.ron".into(),
            tempo: None,
            posicoes: vec![("fila".into(), [2.0, 1.0, 0.0])],
            fluxos: vec![("recebimento".into(), 7)],
            atividade: Some("decide".into()),
            painel_fluxos: true,
        }
    }

    #[test]
    fn flow_assertions() {
        let s = state();
        assert!(s.verificar(&Condicao::FluxosIgual(1)).is_ok());
        assert!(s.verificar(&Condicao::FluxosIgual(2)).is_err());
        assert!(
            s.verificar(&Condicao::AtividadesIgual("recebimento".into(), 7))
                .is_ok()
        );
        assert!(
            s.verificar(&Condicao::AtividadesIgual("recebimento".into(), 8))
                .is_err()
        );
        assert!(
            s.verificar(&Condicao::AtividadesIgual("nada".into(), 1))
                .is_err()
        );
        assert!(
            s.verificar(&Condicao::AtividadeSelecionada(Some("decide".into())))
                .is_ok()
        );
        assert!(s.verificar(&Condicao::AtividadeSelecionada(None)).is_err());
        assert!(s.verificar(&Condicao::PainelFluxos(true)).is_ok());
    }

    #[test]
    fn assertions_pass_and_fail_with_a_reason() {
        let s = state();
        assert!(s.verificar(&Condicao::ObjetosIgual(4)).is_ok());
        let e = s.verificar(&Condicao::ObjetosIgual(5)).unwrap_err();
        assert!(
            e.contains("esperado 5") && e.contains("encontrado 4"),
            "{e}"
        );
        assert!(
            s.verificar(&Condicao::Selecionado(Some("fila".into())))
                .is_ok()
        );
        assert!(s.verificar(&Condicao::Selecionado(None)).is_err());
        assert!(s.verificar(&Condicao::Modo(ModoEditor::Editar)).is_ok());
        assert!(s.verificar(&Condicao::Modo(ModoEditor::Simular)).is_err());
        assert!(s.verificar(&Condicao::StatusContem("Salvo".into())).is_ok());
        assert!(s.verificar(&Condicao::TituloContem("*".into())).is_ok());
        assert!(s.verificar(&Condicao::TempoSimuladoMin(1.0)).is_err());
    }

    #[test]
    fn position_uses_a_tolerance() {
        let s = state();
        let ok = Condicao::PosicaoDe("fila".into(), (2.04, 0.97));
        assert!(s.verificar(&ok).is_ok());
        let far = Condicao::PosicaoDe("fila".into(), (2.5, 1.0));
        assert!(s.verificar(&far).is_err());
        let wide = Condicao::PosicaoPerto("fila".into(), (2.5, 1.0), 1.0);
        assert!(s.verificar(&wide).is_ok());
        assert!(
            s.verificar(&Condicao::PosicaoDe("x".into(), (0.0, 0.0)))
                .is_err()
        );
    }

    #[test]
    fn negation_inverts() {
        let s = state();
        assert!(
            s.verificar(&Condicao::Nao(Box::new(Condicao::Sujo(false))))
                .is_ok()
        );
        assert!(
            s.verificar(&Condicao::Nao(Box::new(Condicao::Sujo(true))))
                .is_err()
        );
    }

    #[test]
    fn summary_lists_the_state() {
        let r = state().resumo();
        assert!(
            r.contains("objetos=4") && r.contains("selecionado=fila"),
            "{r}"
        );
    }
}

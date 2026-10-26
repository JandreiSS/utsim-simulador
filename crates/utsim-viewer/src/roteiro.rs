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
//!     Camera(modo: "Seguir", objeto: Some("empilhadeira_1")),   // Orbita, Seguir, PrimeiraPessoa, DoRecurso
//!     Lod(false),                             // levels of detail off/on (F4)
//!     Tempos(true),                           // frame-time overlay (F3)
//!     Medir(segundos: 4.0, nome: "lod_ligado"), // frame times over 4 s, written to log.txt
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
    /// Number of selected objects (the primary one and the others).
    SelecionadosIgual(usize),
    /// The object is among the selected ones.
    SelecaoContem(String),
    /// `visual.asset` of an object (`None`: the default look of its kind).
    AtivoDe(String, Option<String>),
    /// Number of objects whose name starts with this text.
    ObjetosComPrefixo(String, usize),
    /// Whether the connection `de -> para` exists.
    Ligado(String, String),
    /// Layer of `Editar`: `"Objetos"`, `"Rede"` or `"Fluidos"`.
    Camada(String),
    /// Number of nodes of the path network.
    NosIgual(usize),
    /// Number of edges of the path network.
    ArestasIgual(usize),
    /// An edge between these nodes exists (stored either way round).
    Aresta(String, String),
    /// Model ground position `(x, y)` of a network node, tolerance 0.05.
    NoEm(String, (f64, f64)),
    /// Selection of the `Rede` or `Fluidos` layer: `"no:a"`, `"aresta:a->b"`,
    /// `"fluido:tanque"` (`None`: nothing selected there).
    SelecaoCamada(Option<String>),
    /// Number of nodes of the fluid network.
    NosFluidoIgual(usize),
    /// Number of links of the fluid network.
    LigacoesFluidoIgual(usize),
    /// Number of warnings (`Avisos`).
    AvisosIgual(usize),
    /// Level of a tank at the current time (`Simular`; the initial level in `Editar`)
    /// between `min` and `max`.
    NivelEntre(String, f64, f64),
    /// Whether the "Resultados" window is open.
    PainelResultados(bool),
    /// Tab of the "Resultados" window: `"Resumo"`, `"Gráficos"`, `"Cenários"` or
    /// `"Otimização"`.
    AbaResultados(String),
    /// A run, scenario comparison or optimisation of "Resultados" is in progress.
    AnaliseRodando(bool),
    /// Number of replications of the latest results (0: nothing run yet).
    ReplicacoesNoResultado(usize),
    /// Object highlighted as the bottleneck of the latest results.
    Gargalo(Option<String>),
    /// Number of scenarios of the latest comparison.
    CenariosComparados(usize),
    /// Some file written by the "Resultados" exports ends with this text.
    Exportado(String),
    /// The best point of the latest optimisation, as `chave = valor, ...`, contains this.
    MelhorPontoContem(String),
    /// Camera mode: `"Órbita"`, `"Seguir"`, `"Primeira pessoa"` or `"Do recurso"`.
    CameraModo(String),
    /// Block the follow or ride camera tracks (`None`: none).
    CameraAlvo(Option<String>),
    /// Height of the camera above the ground (metres) between `min` and `max`.
    CameraAlturaEntre(f64, f64),
    /// The camera is at most this far (metres) from the named block.
    CameraPertoDe(String, f64),
    /// The levels of detail are on.
    LodAtivo(bool),
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
    /// Presses a key and keeps it down (e.g. `Segurar("Shift")` before a click or a
    /// drag); `LargarTecla` releases it.
    Segurar(String),
    LargarTecla(String),
    Digitar(String),
    ClicarBotao(String),
    Captura(String),
    Afirmar(Condicao),
    /// Pauses the playback and jumps to this simulated time (`Simular`).
    IrPara(f64),
    /// Counts the frames drawn over this many seconds and writes the frame rate to
    /// `log.txt` (`quadros por segundo: ...`).
    MedirQuadros(f64),
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
    /// Switches the camera mode (`Orbita`, `Seguir`, `PrimeiraPessoa`, `DoRecurso`, with
    /// or without accents). `Seguir` and `DoRecurso` track `objeto` (else the selected
    /// vehicle, else the first one). Needs `Simular`.
    Camera {
        modo: String,
        #[serde(default)]
        objeto: Option<String>,
    },
    /// Turns the levels of detail on or off (F4).
    Lod(bool),
    /// Shows or hides the frame-time overlay (F3).
    Tempos(bool),
    /// Records the frame times for this many seconds (after half a second of warm-up)
    /// and writes the mean, median, 95th percentile and worst frame to `log.txt`.
    Medir {
        segundos: f64,
        nome: String,
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
    /// Every selected object, the primary one first.
    pub selecionados: Vec<String>,
    /// `visual.asset` by object name.
    pub ativos: Vec<(String, Option<String>)>,
    /// Connections `(de, para)`.
    pub ligacoes_lista: Vec<(String, String)>,
    pub camada: String,
    /// Path network nodes and their model positions.
    pub nos: Vec<(String, [f64; 3])>,
    /// Path network edges `(from, to)` as stored.
    pub arestas: Vec<(String, String)>,
    /// Selection of the current network layer (see [`Condicao::SelecaoCamada`]).
    pub selecao_camada: Option<String>,
    pub nos_fluido: usize,
    pub ligacoes_fluido: usize,
    pub avisos: usize,
    /// Tank levels now, by node name.
    pub niveis: Vec<(String, f64)>,
    pub painel_resultados: bool,
    pub aba_resultados: String,
    pub analise_rodando: bool,
    pub replicacoes_resultado: usize,
    pub gargalo: Option<String>,
    pub cenarios_comparados: usize,
    /// Files written by the exports, as full paths.
    pub exportados: Vec<String>,
    /// Best point of the latest optimisation (`chave = valor, ...`).
    pub melhor_ponto: Option<String>,
    /// Camera mode label.
    pub camera_modo: String,
    /// Name of the block the follow or ride camera tracks.
    pub camera_alvo: Option<String>,
    /// Camera position in the scene (Bevy space, metres).
    pub camera_pos: [f64; 3],
    /// Distance from the camera to each block (`Simular`).
    pub camera_distancias: Vec<(String, f64)>,
    pub lod_ativo: bool,
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
            Condicao::SelecionadosIgual(n) => igual("selecionados", self.selecionados.len(), *n),
            Condicao::SelecaoContem(n) => {
                if self.selecionados.contains(n) {
                    Ok(())
                } else {
                    Err(format!(
                        "\"{n}\" não está na seleção {:?}",
                        self.selecionados
                    ))
                }
            }
            Condicao::AtivoDe(n, quer) => match self.ativos.iter().find(|(m, _)| m == n) {
                Some((_, tem)) if tem == quer => Ok(()),
                Some((_, tem)) => Err(format!(
                    "ativo de {n}: esperado {quer:?}, encontrado {tem:?}"
                )),
                None => Err(format!("objeto \"{n}\" não existe")),
            },
            Condicao::ObjetosComPrefixo(p, n) => igual(
                &format!("objetos começando com \"{p}\""),
                self.posicoes
                    .iter()
                    .filter(|(m, _)| m.starts_with(p.as_str()))
                    .count(),
                *n,
            ),
            Condicao::Ligado(a, b) => {
                if self.ligacoes_lista.iter().any(|(x, y)| x == a && y == b) {
                    Ok(())
                } else {
                    Err(format!("não há ligação {a} -> {b}"))
                }
            }
            Condicao::Camada(c) => igual("camada", self.camada.as_str(), c.as_str()),
            Condicao::NosIgual(n) => igual("nós da rede", self.nos.len(), *n),
            Condicao::ArestasIgual(n) => igual("arestas da rede", self.arestas.len(), *n),
            Condicao::Aresta(a, b) => {
                if self
                    .arestas
                    .iter()
                    .any(|(x, y)| (x == a && y == b) || (x == b && y == a))
                {
                    Ok(())
                } else {
                    Err(format!("não há aresta entre {a} e {b}"))
                }
            }
            Condicao::NoEm(n, quer) => match self.nos.iter().find(|(m, _)| m == n) {
                Some((_, p)) if (p[0] - quer.0).abs() <= 0.05 && (p[1] - quer.1).abs() <= 0.05 => {
                    Ok(())
                }
                Some((_, p)) => Err(format!(
                    "posição do nó {n}: esperado ({}, {}), encontrado ({}, {})",
                    quer.0, quer.1, p[0], p[1]
                )),
                None => Err(format!("nó \"{n}\" não existe")),
            },
            Condicao::SelecaoCamada(s) => {
                if &self.selecao_camada == s {
                    Ok(())
                } else {
                    Err(format!(
                        "seleção da camada: esperado {s:?}, encontrado {:?}",
                        self.selecao_camada
                    ))
                }
            }
            Condicao::NosFluidoIgual(n) => igual("nós de fluido", self.nos_fluido, *n),
            Condicao::LigacoesFluidoIgual(n) => {
                igual("ligações de fluido", self.ligacoes_fluido, *n)
            }
            Condicao::AvisosIgual(n) => igual("avisos", self.avisos, *n),
            Condicao::NivelEntre(n, min, max) => match self.niveis.iter().find(|(m, _)| m == n) {
                Some((_, l)) if l >= min && l <= max => Ok(()),
                Some((_, l)) => Err(format!("nível de {n}: {l:.3} fora de [{min}, {max}]")),
                None => Err(format!("tanque \"{n}\" não existe")),
            },
            Condicao::PainelResultados(b) => {
                igual("janela Resultados aberta", self.painel_resultados, *b)
            }
            Condicao::AbaResultados(a) => igual(
                "aba de Resultados",
                self.aba_resultados.as_str(),
                a.as_str(),
            ),
            Condicao::AnaliseRodando(b) => igual("análise rodando", self.analise_rodando, *b),
            Condicao::ReplicacoesNoResultado(n) => {
                igual("replicações no resultado", self.replicacoes_resultado, *n)
            }
            Condicao::Gargalo(g) => {
                if &self.gargalo == g {
                    Ok(())
                } else {
                    Err(format!(
                        "gargalo: esperado {g:?}, encontrado {:?}",
                        self.gargalo
                    ))
                }
            }
            Condicao::CenariosComparados(n) => {
                igual("cenários comparados", self.cenarios_comparados, *n)
            }
            Condicao::Exportado(fim) => {
                if self.exportados.iter().any(|f| f.ends_with(fim.as_str())) {
                    Ok(())
                } else {
                    Err(format!(
                        "nenhum arquivo exportado termina com \"{fim}\" ({:?})",
                        self.exportados
                    ))
                }
            }
            Condicao::MelhorPontoContem(t) => match &self.melhor_ponto {
                Some(p) if p.contains(t.as_str()) => Ok(()),
                Some(p) => Err(format!("melhor ponto \"{p}\" não contém \"{t}\"")),
                None => Err("nenhuma otimização concluída".to_string()),
            },
            Condicao::CameraModo(m) => {
                let quer = crate::cameras::CameraKind::parse(m)
                    .ok_or_else(|| format!("modo de câmera desconhecido: {m}"))?
                    .label();
                igual("modo da câmera", self.camera_modo.as_str(), quer)
            }
            Condicao::CameraAlvo(a) => {
                if &self.camera_alvo == a {
                    Ok(())
                } else {
                    Err(format!(
                        "alvo da câmera: esperado {a:?}, encontrado {:?}",
                        self.camera_alvo
                    ))
                }
            }
            Condicao::CameraAlturaEntre(min, max) => {
                let h = self.camera_pos[1];
                if h >= *min && h <= *max {
                    Ok(())
                } else {
                    Err(format!("altura da câmera {h:.2} fora de {min}..{max}"))
                }
            }
            Condicao::CameraPertoDe(nome, max) => {
                match self.camera_distancias.iter().find(|(n, _)| n == nome) {
                    Some((_, d)) if d <= max => Ok(()),
                    Some((_, d)) => Err(format!("câmera a {d:.1} m de \"{nome}\" (máximo {max})")),
                    None => Err(format!("bloco \"{nome}\" não existe na simulação")),
                }
            }
            Condicao::LodAtivo(b) => igual("LOD ativo", self.lod_ativo, *b),
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
            "modo={} camada={} selecionado={} ferramenta={} objetos={} ligacoes={} nos={} arestas={} fluidos={} problemas={} avisos={} sujo={} titulo=\"{}\" status=\"{}\"{tempo}",
            self.modo,
            self.camada,
            self.selecionado.as_deref().unwrap_or("-"),
            self.ferramenta,
            self.objetos,
            self.ligacoes,
            self.nos.len(),
            self.arestas.len(),
            self.nos_fluido,
            self.problemas,
            self.avisos,
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
            selecionados: vec!["fila".into(), "maquina".into()],
            ativos: vec![
                ("fila".into(), None),
                ("maquina".into(), Some("maquina_kenney".into())),
            ],
            ligacoes_lista: vec![("fila".into(), "maquina".into())],
            camada: "Rede".into(),
            nos: vec![("a".into(), [1.0, 2.0, 0.0]), ("b".into(), [5.0, 2.0, 0.0])],
            arestas: vec![("a".into(), "b".into())],
            selecao_camada: Some("no:a".into()),
            nos_fluido: 3,
            ligacoes_fluido: 2,
            avisos: 1,
            niveis: vec![("tanque".into(), 12.5)],
            painel_resultados: true,
            aba_resultados: "Resumo".into(),
            analise_rodando: false,
            replicacoes_resultado: 5,
            gargalo: Some("furacao".into()),
            cenarios_comparados: 2,
            exportados: vec!["C:/tmp/linha_resultados/resumo.csv".into()],
            melhor_ponto: Some("maquina.capacity = 3".into()),
            camera_modo: "Seguir".into(),
            camera_alvo: Some("empilhadeira_1".into()),
            camera_pos: [3.0, 6.5, -2.0],
            camera_distancias: vec![("empilhadeira_1".into(), 9.0), ("doca".into(), 40.0)],
            lod_ativo: true,
        }
    }

    #[test]
    fn camera_steps_and_assertions() {
        let steps = parse(
            r#"[Camera(modo: "Seguir", objeto: Some("empilhadeira_1")), Camera(modo: "Orbita"),
                Lod(false), Tempos(true), Medir(segundos: 2.0, nome: "x")]"#,
        )
        .unwrap();
        assert!(
            matches!(&steps[0], Passo::Camera { objeto: Some(o), .. } if o == "empilhadeira_1")
        );
        assert!(matches!(&steps[1], Passo::Camera { objeto: None, .. }));
        assert_eq!(steps[2], Passo::Lod(false));
        let s = state();
        assert!(s.verificar(&Condicao::CameraModo("seguir".into())).is_ok());
        assert!(s.verificar(&Condicao::CameraModo("Órbita".into())).is_err());
        assert!(s.verificar(&Condicao::CameraModo("voo".into())).is_err());
        assert!(
            s.verificar(&Condicao::CameraAlvo(Some("empilhadeira_1".into())))
                .is_ok()
        );
        assert!(s.verificar(&Condicao::CameraAlturaEntre(5.0, 7.0)).is_ok());
        assert!(s.verificar(&Condicao::CameraAlturaEntre(1.5, 1.7)).is_err());
        assert!(
            s.verificar(&Condicao::CameraPertoDe("empilhadeira_1".into(), 10.0))
                .is_ok()
        );
        assert!(
            s.verificar(&Condicao::CameraPertoDe("doca".into(), 10.0))
                .is_err()
        );
        assert!(s.verificar(&Condicao::LodAtivo(true)).is_ok());
    }

    #[test]
    fn results_assertions() {
        let s = state();
        let ok = |c: Condicao| assert!(s.verificar(&c).is_ok(), "{c:?}");
        let bad = |c: Condicao| assert!(s.verificar(&c).is_err(), "{c:?}");
        ok(Condicao::PainelResultados(true));
        ok(Condicao::AbaResultados("Resumo".into()));
        bad(Condicao::AbaResultados("Cenários".into()));
        ok(Condicao::AnaliseRodando(false));
        ok(Condicao::ReplicacoesNoResultado(5));
        bad(Condicao::ReplicacoesNoResultado(4));
        ok(Condicao::Gargalo(Some("furacao".into())));
        bad(Condicao::Gargalo(None));
        ok(Condicao::CenariosComparados(2));
        ok(Condicao::Exportado("resumo.csv".into()));
        bad(Condicao::Exportado("relatorio.html".into()));
        ok(Condicao::MelhorPontoContem("capacity = 3".into()));
        bad(Condicao::MelhorPontoContem("capacity = 4".into()));
        let steps =
            parse(r#"[Afirmar(Gargalo(Some("furacao"))), Afirmar(Exportado("resumo.csv"))]"#)
                .unwrap();
        assert_eq!(steps.len(), 2);
    }

    #[test]
    fn network_and_fluid_assertions() {
        let s = state();
        let ok = |c: Condicao| assert!(s.verificar(&c).is_ok(), "{c:?}");
        let bad = |c: Condicao| assert!(s.verificar(&c).is_err(), "{c:?}");
        ok(Condicao::Camada("Rede".into()));
        bad(Condicao::Camada("Fluidos".into()));
        ok(Condicao::NosIgual(2));
        ok(Condicao::ArestasIgual(1));
        ok(Condicao::Aresta("b".into(), "a".into()));
        bad(Condicao::Aresta("a".into(), "c".into()));
        ok(Condicao::NoEm("b".into(), (5.0, 2.0)));
        bad(Condicao::NoEm("b".into(), (5.5, 2.0)));
        bad(Condicao::NoEm("z".into(), (0.0, 0.0)));
        ok(Condicao::SelecaoCamada(Some("no:a".into())));
        bad(Condicao::SelecaoCamada(None));
        ok(Condicao::NosFluidoIgual(3));
        ok(Condicao::LigacoesFluidoIgual(2));
        ok(Condicao::AvisosIgual(1));
        ok(Condicao::NivelEntre("tanque".into(), 10.0, 15.0));
        bad(Condicao::NivelEntre("tanque".into(), 13.0, 15.0));
        bad(Condicao::NivelEntre("nada".into(), 0.0, 1.0));
        let steps = parse(
            r#"[Afirmar(Camada("Rede")), Afirmar(NivelEntre("t", 0.0, 1.0)), Afirmar(SelecaoCamada(None))]"#,
        )
        .unwrap();
        assert_eq!(steps.len(), 3);
        assert!(s.resumo().contains("camada=Rede"));
    }

    #[test]
    fn selection_and_appearance_assertions() {
        let s = state();
        assert!(s.verificar(&Condicao::SelecionadosIgual(2)).is_ok());
        assert!(s.verificar(&Condicao::SelecionadosIgual(1)).is_err());
        assert!(
            s.verificar(&Condicao::SelecaoContem("maquina".into()))
                .is_ok()
        );
        assert!(s.verificar(&Condicao::SelecaoContem("x".into())).is_err());
        let ativo = |n: &str, a: Option<&str>| Condicao::AtivoDe(n.into(), a.map(Into::into));
        assert!(
            s.verificar(&ativo("maquina", Some("maquina_kenney")))
                .is_ok()
        );
        assert!(s.verificar(&ativo("fila", None)).is_ok());
        assert!(s.verificar(&ativo("fila", Some("area"))).is_err());
        assert!(s.verificar(&ativo("nada", None)).is_err());
        assert!(
            s.verificar(&Condicao::ObjetosComPrefixo("fi".into(), 1))
                .is_ok()
        );
        assert!(
            s.verificar(&Condicao::Ligado("fila".into(), "maquina".into()))
                .is_ok()
        );
        assert!(
            s.verificar(&Condicao::Ligado("maquina".into(), "fila".into()))
                .is_err()
        );
        let steps = parse(
            r#"[Segurar("Shift"), LargarTecla("Shift"), Afirmar(AtivoDe("p", Some("bancada_kenney")))]"#,
        )
        .unwrap();
        assert_eq!(steps[0], Passo::Segurar("Shift".into()));
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

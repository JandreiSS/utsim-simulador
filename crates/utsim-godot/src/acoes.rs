//! What each action of the interface does (passo 7, parte 3): the clicks of the top bar,
//! the left panels and the playback bar (`UtsimUi.action`), the rows of the properties
//! panel (fields separated by tabs), the tool panels (`form:<ação>:<valor>`) and the keys
//! (`shortcuts.rs`). Here the text is read and the decisions taken (which tool a click
//! leaves armed, which link kinds stay hidden, the limits of the replications, the instant
//! a near miss jumps to); the main scene only runs the [`Passo`]s, which call the
//! document, the 3D scene, the editor and the Godot dialogs.

/// What the decisions need of the main scene.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Estado {
    /// The editor's tool and the library entry armed with `colocar`.
    pub ferramenta: String,
    pub colocar: String,
    /// Snap to the grid and its step.
    pub encaixe: bool,
    pub passo: f64,
    /// The link kinds hidden in the 3D view.
    pub ocultas: Vec<String>,
}

/// One thing for the main scene to do.
#[derive(Clone, Debug, PartialEq)]
pub enum Passo {
    // Document and modes.
    Modo(String),
    AlternarModo,
    Novo,
    Abrir,
    Salvar,
    SalvarComo,
    Desfazer,
    Refazer,
    Dialogo {
        descartar: bool,
    },
    Atalhos,
    Desempenho,
    Ensino,
    Analisar(String),
    /// "Abrir no UTSim Qualidade / Segurança" of the node tabs: the app (`qualidade`,
    /// `seguranca`) and the node.
    AbrirApp {
        app: String,
        no: String,
    },
    Resultados(String),
    Pert(String),
    // Editar.
    Ferramenta {
        nome: String,
        id: String,
    },
    Organizar(String),
    Encaixe {
        ligado: bool,
        passo: f64,
    },
    Ocultas(Vec<String>),
    ArrastarBiblioteca(String),
    Selecionar(String),
    SelecionarTudo,
    Focar(String),
    Obstaculos(bool),
    Enquadrar,
    Excluir,
    CancelarEditor,
    Mover(i64, i64),
    /// Turns the selection by these degrees (counter-clockwise seen from above).
    Girar(i64),
    FecharZona,
    FecharCorredor,
    // Simular.
    Replicacoes(i64),
    SimularDeNovo,
    Bloco(i64),
    Camera(String),
    GravarReplay,
    Rotulos(bool),
    Seguranca(bool),
    Calor(bool),
    Sombras(bool),
    /// Pauses and jumps (without crossfade) to this instant.
    IrPara(f64),
    Reproduzir,
    /// The clock's time jumped: show it.
    Relogio,
    FluxosTempo,
    CancelarTrabalho,
    // The properties panel.
    PropOk {
        objeto: String,
        caminho: String,
        valor: String,
    },
    PropErro {
        objeto: String,
        motivo: String,
    },
    PropFim,
    Problema(String),
    SaidaOrdem(i64, i64),
    SaidaRemover(String),
    Motorista {
        nome: String,
        visivel: bool,
    },
    Dados {
        acao: String,
        caminho: String,
        rascunho: String,
    },
    Miniatura(String),
    // The tool panels.
    RedeNovosMaoDupla(bool),
    RedeNome(String),
    RedeExcluir,
    RedeMaoDupla(bool),
    RedeCapacidade(i64),
    RedeInverter,
    CorredorExcluir,
    CorredorTipo(String),
    CorredorLargura(f64),
    CorredorMaoDupla(bool),
    CorredorSecundaria(bool),
    CorredorNome(String),
    CorredorPerfil(String),
    CorredorRemoverVertice,
    CorredorInverter,
    /// A field of the selected zone (`name`, `kind`, `level`, `ppe`).
    ZonaCampo {
        campo: String,
        valor: String,
    },
    MostrarPropriedades,
}

/// The fields a [`Passo`] carries to the main scene, by kind.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Args {
    pub s: String,
    pub s2: String,
    pub s3: String,
    pub n: i64,
    pub n2: i64,
    pub x: f64,
    pub b: bool,
    pub lista: Vec<String>,
}

impl Passo {
    /// The name the main scene matches on (`op`) and the fields.
    pub fn campos(&self) -> (&'static str, Args) {
        use Passo::*;
        let mut a = Args::default();
        let op = match self {
            Modo(m) => {
                a.s = m.clone();
                "modo"
            }
            AlternarModo => "alternar_modo",
            Novo => "novo",
            Abrir => "abrir",
            Salvar => "salvar",
            SalvarComo => "salvar_como",
            Desfazer => "desfazer",
            Refazer => "refazer",
            Dialogo { descartar } => {
                a.b = *descartar;
                "dialogo"
            }
            Atalhos => "atalhos",
            Desempenho => "desempenho",
            Ensino => "ensino",
            Analisar(s) => {
                a.s = s.clone();
                "analisar"
            }
            AbrirApp { app, no } => {
                a.s = app.clone();
                a.s2 = no.clone();
                "abrir_app"
            }
            Resultados(s) => {
                a.s = s.clone();
                "resultados"
            }
            Pert(s) => {
                a.s = s.clone();
                "pert"
            }
            Ferramenta { nome, id } => {
                a.s = nome.clone();
                a.s2 = id.clone();
                "ferramenta"
            }
            Organizar(s) => {
                a.s = s.clone();
                "organizar"
            }
            Encaixe { ligado, passo } => {
                a.b = *ligado;
                a.x = *passo;
                "encaixe"
            }
            Ocultas(l) => {
                a.lista = l.clone();
                "ocultas"
            }
            ArrastarBiblioteca(s) => {
                a.s = s.clone();
                "arrastar_biblioteca"
            }
            Selecionar(s) => {
                a.s = s.clone();
                "selecionar"
            }
            SelecionarTudo => "selecionar_tudo",
            Focar(s) => {
                a.s = s.clone();
                "focar"
            }
            Obstaculos(b) => {
                a.b = *b;
                "obstaculos"
            }
            Enquadrar => "enquadrar",
            Excluir => "excluir",
            CancelarEditor => "cancelar_editor",
            Mover(x, z) => {
                a.n = *x;
                a.n2 = *z;
                "mover"
            }
            Girar(g) => {
                a.n = *g;
                "girar"
            }
            FecharZona => "fechar_zona",
            FecharCorredor => "fechar_corredor",
            Replicacoes(n) => {
                a.n = *n;
                "replicacoes"
            }
            SimularDeNovo => "simular_de_novo",
            Bloco(n) => {
                a.n = *n;
                "bloco"
            }
            Camera(s) => {
                a.s = s.clone();
                "camera"
            }
            GravarReplay => "gravar_replay",
            Rotulos(b) => {
                a.b = *b;
                "rotulos"
            }
            Seguranca(b) => {
                a.b = *b;
                "seguranca"
            }
            Calor(b) => {
                a.b = *b;
                "calor"
            }
            Sombras(b) => {
                a.b = *b;
                "sombras"
            }
            IrPara(x) => {
                a.x = *x;
                "ir_para"
            }
            Reproduzir => "reproduzir",
            Relogio => "relogio",
            FluxosTempo => "fluxos_tempo",
            CancelarTrabalho => "cancelar_trabalho",
            PropOk {
                objeto,
                caminho,
                valor,
            } => {
                a.s = objeto.clone();
                a.s2 = caminho.clone();
                a.s3 = valor.clone();
                "prop_ok"
            }
            PropErro { objeto, motivo } => {
                a.s = objeto.clone();
                a.s2 = motivo.clone();
                "prop_erro"
            }
            PropFim => "prop_fim",
            Problema(s) => {
                a.s = s.clone();
                "problema"
            }
            SaidaOrdem(de, para) => {
                a.n = *de;
                a.n2 = *para;
                "saida_ordem"
            }
            SaidaRemover(s) => {
                a.s = s.clone();
                "saida_remover"
            }
            Motorista { nome, visivel } => {
                a.s = nome.clone();
                a.b = *visivel;
                "motorista"
            }
            Dados {
                acao,
                caminho,
                rascunho,
            } => {
                a.s = acao.clone();
                a.s2 = caminho.clone();
                a.s3 = rascunho.clone();
                "dados"
            }
            Miniatura(s) => {
                a.s = s.clone();
                "miniatura"
            }
            RedeNovosMaoDupla(b) => {
                a.b = *b;
                "rede_novos_mao_dupla"
            }
            RedeNome(s) => {
                a.s = s.clone();
                "rede_nome"
            }
            RedeExcluir => "rede_excluir",
            RedeMaoDupla(b) => {
                a.b = *b;
                "rede_mao_dupla"
            }
            RedeCapacidade(n) => {
                a.n = *n;
                "rede_capacidade"
            }
            RedeInverter => "rede_inverter",
            CorredorExcluir => "corredor_excluir",
            CorredorTipo(s) => {
                a.s = s.clone();
                "corredor_tipo"
            }
            CorredorLargura(x) => {
                a.x = *x;
                "corredor_largura"
            }
            CorredorMaoDupla(b) => {
                a.b = *b;
                "corredor_mao_dupla"
            }
            CorredorSecundaria(b) => {
                a.b = *b;
                "corredor_secundaria"
            }
            CorredorNome(s) => {
                a.s = s.clone();
                "corredor_nome"
            }
            CorredorPerfil(s) => {
                a.s = s.clone();
                "corredor_perfil"
            }
            CorredorRemoverVertice => "corredor_remover_vertice",
            CorredorInverter => "corredor_inverter",
            ZonaCampo { campo, valor } => {
                a.s = campo.clone();
                a.s2 = valor.clone();
                "zona_campo"
            }
            MostrarPropriedades => "mostrar_propriedades",
        };
        (op, a)
    }
}

fn int(s: &str) -> Option<i64> {
    let s = s.trim();
    s.parse::<i64>().ok().or_else(|| {
        s.parse::<f64>()
            .ok()
            .filter(|x| x.is_finite())
            .map(|x| x as i64)
    })
}

fn num(s: &str) -> Option<f64> {
    s.trim().parse::<f64>().ok().filter(|x| x.is_finite())
}

/// What a click (top bar, left panels, playback bar, properties panel, tool panels) does.
/// Unknown actions do nothing.
pub fn rotear(what: &str, st: &Estado) -> Vec<Passo> {
    use Passo::*;
    if what.contains('\t') {
        return painel(&what.split('\t').collect::<Vec<_>>());
    }
    let (verbo, arg) = what.split_once(':').unwrap_or((what, ""));
    let liga = arg == "ligar";
    let p = match verbo {
        "modo" => Modo(arg.into()),
        "novo" => Novo,
        "abrir" => Abrir,
        "salvar" => Salvar,
        "salvar_como" => SalvarComo,
        "desfazer" => Desfazer,
        "refazer" => Refazer,
        "ligacoes" => {
            let (tipo, como) = arg.split_once(':').unwrap_or((arg, ""));
            let mut l: Vec<String> = st.ocultas.iter().filter(|k| *k != tipo).cloned().collect();
            if como == "desligar" {
                l.push(tipo.into());
            }
            Ocultas(l)
        }
        "ferramenta" => {
            let nome = if arg == "selecionar" || arg == "ligar" || st.ferramenta != arg {
                arg
            } else {
                "selecionar"
            };
            Ferramenta {
                nome: nome.into(),
                id: String::new(),
            }
        }
        "organizar" => Organizar(arg.into()),
        "grade" => Encaixe {
            ligado: liga,
            passo: st.passo,
        },
        "passo" => match num(arg) {
            Some(p) if p > 0.0 => Encaixe {
                ligado: st.encaixe,
                passo: p,
            },
            _ => return Vec::new(),
        },
        "analisar" => Analisar(arg.into()),
        "abrir_app" => {
            let (app, no) = arg.split_once(':').unwrap_or((arg, ""));
            AbrirApp {
                app: app.into(),
                no: no.into(),
            }
        }
        "ensino" => Ensino,
        "resultados" => Resultados(arg.into()),
        "pert" => Pert(arg.into()),
        "form" => return formulario(arg),
        "fluxos" => FluxosTempo,
        "dialogo" => Dialogo {
            descartar: arg == "descartar",
        },
        "atalhos" => Atalhos,
        "biblioteca" => {
            if st.ferramenta == "colocar" && st.colocar == arg {
                Ferramenta {
                    nome: "selecionar".into(),
                    id: String::new(),
                }
            } else {
                Ferramenta {
                    nome: "colocar".into(),
                    id: arg.into(),
                }
            }
        }
        "biblioteca_arrastar" => ArrastarBiblioteca(arg.into()),
        "objeto" => Selecionar(arg.into()),
        "objeto_focar" => Focar(arg.into()),
        "obstaculos" => Obstaculos(liga),
        "replicacoes" => match int(arg) {
            Some(n) => Replicacoes(n.clamp(1, 1000)),
            None => return Vec::new(),
        },
        "rodar_de_novo" => SimularDeNovo,
        "bloco" => match int(arg) {
            Some(k) if k >= 0 => Bloco(k),
            _ => return Vec::new(),
        },
        "camera" => Camera(arg.into()),
        "gravar_replay" => GravarReplay,
        "rotulos" => Rotulos(liga),
        "seguranca" => Seguranca(liga),
        "calor" => Calor(liga),
        "sombras" => Sombras(liga),
        // A near miss: a little before it, to see it coming.
        "quase" => match num(arg) {
            Some(x) => IrPara((x - 2.0).max(0.0)),
            None => return Vec::new(),
        },
        "cancelar" => CancelarTrabalho,
        "relogio" => Relogio,
        _ => return Vec::new(),
    };
    vec![p]
}

/// The rows of the properties panel: `<verbo>\t<campo>\t...`.
fn painel(a: &[&str]) -> Vec<Passo> {
    use Passo::*;
    let at = |k: usize| a.get(k).copied().unwrap_or("").to_owned();
    let p = match a[0] {
        "prop_ok" => PropOk {
            objeto: at(1),
            caminho: at(2),
            valor: at(3),
        },
        "prop_erro" => PropErro {
            objeto: at(1),
            motivo: at(2),
        },
        "prop_fim" => PropFim,
        "problema" => Problema(at(1)),
        "saida_ordem" => match (int(&at(1)), int(&at(2))) {
            (Some(de), Some(para)) => SaidaOrdem(de, para),
            _ => return Vec::new(),
        },
        "saida_remover" => SaidaRemover(at(1)),
        "motorista" => Motorista {
            nome: at(1),
            visivel: at(2) == "true",
        },
        "dados" => Dados {
            acao: at(1),
            caminho: at(2),
            rascunho: at(3),
        },
        "miniatura" => Miniatura(at(1)),
        _ => return Vec::new(),
    };
    vec![p]
}

/// The tool panels (Rede, Corredor, Zona, the selected link): `<ação>:<valor>`. The panel
/// is shown again after any of them.
fn formulario(what: &str) -> Vec<Passo> {
    use Passo::*;
    let (acao, valor) = what.split_once(':').unwrap_or((what, ""));
    let sim = valor == "1";
    let campo = |c: &str| ZonaCampo {
        campo: c.into(),
        valor: valor.into(),
    };
    let p = match acao {
        "ligacao_remover" | "zona_excluir" => Some(Excluir),
        "rede_novos_mao_dupla" => Some(RedeNovosMaoDupla(sim)),
        "rede_no_nome" => Some(RedeNome(valor.into())),
        "rede_excluir" => Some(RedeExcluir),
        "rede_mao_dupla" => Some(RedeMaoDupla(sim)),
        "rede_capacidade" => int(valor).map(RedeCapacidade),
        "rede_inverter" => Some(RedeInverter),
        "corredor_excluir" => Some(CorredorExcluir),
        "corredor_tipo" => Some(CorredorTipo(valor.into())),
        "corredor_largura" => num(valor).map(CorredorLargura),
        "corredor_mao_dupla" => Some(CorredorMaoDupla(sim)),
        "corredor_secundaria" => Some(CorredorSecundaria(sim)),
        "corredor_nome" => Some(CorredorNome(valor.into())),
        "corredor_perfil" => Some(CorredorPerfil(valor.into())),
        "corredor_vertice_remover" => Some(CorredorRemoverVertice),
        "corredor_inverter" => Some(CorredorInverter),
        "zona_nome" => Some(campo("name")),
        "zona_tipo" => Some(campo("kind")),
        "zona_nivel" => Some(campo("level")),
        _ => acao.strip_prefix("zona_epi_").map(|epi| ZonaCampo {
            campo: "ppe".into(),
            valor: epi.into(),
        }),
    };
    p.into_iter().chain([MostrarPropriedades]).collect()
}

/// What a key does (the action `shortcuts.rs` named). Empty: the key is not used.
pub fn rotear_tecla(what: &str, st: &Estado) -> Vec<Passo> {
    use Passo::*;
    let mut partes = what.split(':');
    let verbo = partes.next().unwrap_or("");
    let arg = partes.next().unwrap_or("");
    let p = match verbo {
        "desfazer" => Desfazer,
        "refazer" => Refazer,
        "salvar" => Salvar,
        "salvar_como" => SalvarComo,
        "abrir" => Abrir,
        "novo" => Novo,
        "organizar" => Organizar(arg.into()),
        "selecionar_tudo" => SelecionarTudo,
        "atalhos" => Atalhos,
        "desempenho" => Desempenho,
        "modo" => AlternarModo,
        "enquadrar" => Enquadrar,
        "reproduzir" => Reproduzir,
        "excluir" => Excluir,
        "cancelar" => CancelarEditor,
        "mover" => match (int(arg), int(partes.next().unwrap_or(""))) {
            (Some(x), Some(z)) => Mover(x, z),
            _ => return Vec::new(),
        },
        "ferramenta" => Ferramenta {
            nome: arg.into(),
            id: String::new(),
        },
        "girar" => match int(arg) {
            Some(g) => Girar(g),
            None => return Vec::new(),
        },
        "zona_fechar" => FecharZona,
        "corredor_fechar" => FecharCorredor,
        "grade" => Encaixe {
            ligado: !st.encaixe,
            passo: st.passo,
        },
        _ => return Vec::new(),
    };
    vec![p]
}

#[cfg(test)]
mod tests {
    use super::Passo::*;
    use super::*;

    fn st() -> Estado {
        Estado {
            ferramenta: "selecionar".into(),
            colocar: String::new(),
            encaixe: true,
            passo: 0.5,
            ocultas: vec!["operadores".into()],
        }
    }

    #[test]
    fn the_tool_buttons_toggle_and_the_library_arms_and_disarms() {
        let mut s = st();
        let tool = |p: &[Passo]| match p {
            [Ferramenta { nome, id }] => (nome.clone(), id.clone()),
            o => panic!("{o:?}"),
        };
        assert_eq!(
            tool(&rotear("ferramenta:rede", &s)),
            ("rede".into(), "".into())
        );
        s.ferramenta = "rede".into();
        assert_eq!(
            tool(&rotear("ferramenta:rede", &s)).0,
            "selecionar",
            "second click"
        );
        assert_eq!(tool(&rotear("ferramenta:ligar", &s)).0, "ligar");
        s.ferramenta = "ligar".into();
        assert_eq!(
            tool(&rotear("ferramenta:ligar", &s)).0,
            "ligar",
            "ligar stays"
        );
        assert_eq!(
            tool(&rotear("biblioteca:Processor", &s)),
            ("colocar".into(), "Processor".into())
        );
        s.ferramenta = "colocar".into();
        s.colocar = "Processor".into();
        assert_eq!(tool(&rotear("biblioteca:Processor", &s)).0, "selecionar");
        assert_eq!(tool(&rotear("biblioteca:Sink", &s)).1, "Sink");
    }

    #[test]
    fn the_bar_reads_its_arguments() {
        let s = st();
        assert_eq!(
            rotear("ligacoes:operadores:ligar", &s),
            vec![Ocultas(vec![])]
        );
        assert_eq!(
            rotear("ligacoes:fluxo:desligar", &s),
            vec![Ocultas(vec!["operadores".into(), "fluxo".into()])]
        );
        assert_eq!(
            rotear("grade:desligar", &s),
            vec![Encaixe {
                ligado: false,
                passo: 0.5
            }]
        );
        assert_eq!(
            rotear("passo:2", &s),
            vec![Encaixe {
                ligado: true,
                passo: 2.0
            }]
        );
        assert!(rotear("passo:0", &s).is_empty() && rotear("passo:x", &s).is_empty());
        assert_eq!(rotear("replicacoes:5000", &s), vec![Replicacoes(1000)]);
        assert_eq!(rotear("replicacoes:0", &s), vec![Replicacoes(1)]);
        assert_eq!(rotear("quase:1.5", &s), vec![IrPara(0.0)]);
        assert_eq!(rotear("quase:30", &s), vec![IrPara(28.0)]);
        assert_eq!(
            rotear("organizar:alinhar:esquerda", &s),
            vec![Organizar("alinhar:esquerda".into())]
        );
        assert_eq!(rotear("modo:simular", &s), vec![Modo("simular".into())]);
        assert_eq!(rotear("rotulos:ligar", &s), vec![Rotulos(true)]);
        assert_eq!(
            rotear("dialogo:descartar", &s),
            vec![Dialogo { descartar: true }]
        );
        assert_eq!(rotear("cancelar", &s), vec![CancelarTrabalho]);
        assert!(rotear("nada", &s).is_empty());
    }

    #[test]
    fn the_panel_rows_and_the_tool_panels() {
        let s = st();
        assert_eq!(
            rotear("prop_ok\tprocesso\tname\tprensa", &s),
            vec![PropOk {
                objeto: "processo".into(),
                caminho: "name".into(),
                valor: "prensa".into()
            }]
        );
        assert_eq!(
            rotear("prop_erro\tprocesso", &s),
            vec![PropErro {
                objeto: "processo".into(),
                motivo: String::new()
            }]
        );
        assert_eq!(rotear("saida_ordem\t2\t0", &s), vec![SaidaOrdem(2, 0)]);
        assert!(rotear("saida_ordem\tx\t0", &s).is_empty());
        assert_eq!(
            rotear("motorista\tempilhadeira_1\ttrue", &s),
            vec![Motorista {
                nome: "empilhadeira_1".into(),
                visivel: true
            }]
        );
        assert_eq!(
            rotear("form:rede_capacidade:3.0", &s),
            vec![RedeCapacidade(3), MostrarPropriedades]
        );
        assert_eq!(
            rotear("form:corredor_largura:1,5", &s),
            vec![MostrarPropriedades],
            "a bad number changes nothing"
        );
        assert_eq!(
            rotear("form:zona_nivel:alto", &s),
            vec![
                ZonaCampo {
                    campo: "level".into(),
                    valor: "alto".into()
                },
                MostrarPropriedades
            ]
        );
        assert_eq!(
            rotear("form:zona_epi_luvas", &s),
            vec![
                ZonaCampo {
                    campo: "ppe".into(),
                    valor: "luvas".into()
                },
                MostrarPropriedades
            ]
        );
        assert_eq!(
            rotear("form:ligacao_remover", &s),
            vec![Excluir, MostrarPropriedades]
        );
    }

    #[test]
    fn the_keys() {
        let s = st();
        assert_eq!(rotear_tecla("modo:alternar", &s), vec![AlternarModo]);
        assert_eq!(rotear_tecla("mover:-1:0", &s), vec![Mover(-1, 0)]);
        assert_eq!(rotear_tecla("girar:-90", &s), vec![Girar(-90)]);
        assert!(rotear_tecla("girar:x", &s).is_empty());
        assert_eq!(
            rotear_tecla("organizar:colar", &s),
            vec![Organizar("colar".into())]
        );
        assert_eq!(rotear_tecla("cancelar", &s), vec![CancelarEditor]);
        assert_eq!(
            rotear_tecla("grade", &s),
            vec![Encaixe {
                ligado: false,
                passo: 0.5
            }]
        );
        assert!(rotear_tecla("tecla_nova", &s).is_empty());
    }

    #[test]
    fn every_step_has_a_name() {
        let (op, a) = Encaixe {
            ligado: true,
            passo: 0.25,
        }
        .campos();
        assert_eq!((op, a.b, a.x), ("encaixe", true, 0.25));
        let (op, a) = Mover(1, -1).campos();
        assert_eq!((op, a.n, a.n2), ("mover", 1, -1));
        let (op, a) = Girar(15).campos();
        assert_eq!((op, a.n), ("girar", 15));
    }
}

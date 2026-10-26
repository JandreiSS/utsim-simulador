//! Scripted mode (`--roteiro`): drives the real app with synthetic input.
//!
//! The runner replays the steps of a script (format in
//! [`utsim_viewer::roteiro`]) one at a time, writing the same Bevy messages the
//! windowing backend writes (`CursorMoved`, `MouseButtonInput`, `MouseWheel`,
//! `MouseMotion`, `KeyboardInput`, plus the `WindowEvent` union that `bevy_egui`
//! reads for the pointer), so the editor systems and the egui widgets react exactly
//! as they do to a person. World coordinates are projected with the active camera,
//! so scripts survive camera changes.
//!
//! **Finding egui widgets.** egui 0.36 / `bevy_egui` 0.43 are built here without
//! `accesskit`, so there is no widget tree. Instead, after each egui pass (in
//! `PostUpdate`, between `EndPass` and `ProcessOutput`) every text shape of the
//! frame is recorded with its on-screen rectangle (`TextLabels`). `Botao("rótulo")`
//! and `ClicarBotao("rótulo")` look up the topmost visible text equal to (else
//! containing) the label and aim at its center, which is inside the widget that owns
//! the text (buttons, selectable labels, links, checkboxes).
//!
//! Each step lasts a few frames (so Bevy and egui process the events between
//! steps); after every step a line goes to `<capturas>/log.txt`. The run ends with
//! exit code 0, or 1 at the first failed step.

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use bevy::ecs::system::SystemParam;
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyCode, KeyboardInput};
use bevy::input::mouse::{MouseButton, MouseButtonInput, MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::math::DVec2;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::{CursorMoved, PrimaryWindow, WindowEvent};
use bevy_egui::{EguiFullOutput, EguiPostUpdateSet, egui};
use utsim_viewer::layout;
use utsim_viewer::roteiro::{Alvo, Botao, Estado, Passo};

use crate::editor::{Editor, Mode, Tool};
use crate::playback::{Data, Playback};

/// Frames the pointer needs to travel during `Arrastar`.
const DRAG_FRAMES: u32 = 8;
/// Whole-run safety limit, in seconds.
const RUN_LIMIT: f64 = 600.0;
/// Limit of a screenshot, in seconds.
const CAPTURE_LIMIT: f64 = 20.0;

/// Script, output directory and the window size to use.
pub struct ScriptPlugin {
    pub steps: Vec<Passo>,
    pub dir: PathBuf,
}

impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        let _ = fs::create_dir_all(&self.dir);
        let _ = fs::write(self.dir.join("log.txt"), "");
        app.insert_resource(Runner::new(self.steps.clone(), self.dir.clone()))
            .init_resource::<TextLabels>()
            .add_systems(
                PostUpdate,
                record_texts
                    .after(EguiPostUpdateSet::EndPass)
                    .before(EguiPostUpdateSet::ProcessOutput),
            )
            .add_systems(
                Update,
                (run, apply_control).chain().after(crate::editor::title),
            );
    }
}

/// Applies an `IrPara` (jump the clock) or `Focar` (aim the camera) step.
fn apply_control(
    mut runner: ResMut<Runner>,
    mut pb: ResMut<Playback>,
    mut cam: ResMut<crate::camera::OrbitCam>,
    data: Res<Data>,
) {
    match runner.control.take() {
        Some(Passo::IrPara(t)) => {
            pb.playing = false;
            pb.t = t.clamp(0.0, data.end);
        }
        Some(Passo::Focar {
            objeto,
            distancia,
            giro,
            inclinacao,
        }) => {
            if let Some(b) = data.replay.blocks.iter().position(|b| b.name == objeto) {
                let h = data.geoms[b].half[1];
                cam.target = Vec3::from(data.base_at(b, pb.t)) + Vec3::Y * h;
                cam.dist = distancia.clamp(1.5, 5000.0);
                if let Some(y) = giro {
                    cam.yaw = y;
                }
                if let Some(p) = inclinacao {
                    cam.pitch = p.clamp(0.05, 1.55);
                }
            }
        }
        _ => {}
    }
}

/// Visible egui texts of the last frame, in paint order (topmost last).
#[derive(Resource, Default)]
pub struct TextLabels(Vec<TextLabel>);

struct TextLabel {
    text: String,
    rect: egui::Rect,
    clip: egui::Rect,
}

fn collect(shape: &egui::Shape, clip: egui::Rect, out: &mut Vec<TextLabel>) {
    match shape {
        egui::Shape::Vec(v) => v.iter().for_each(|s| collect(s, clip, out)),
        egui::Shape::Text(t) => {
            let text = t.galley.text().to_string();
            if !text.trim().is_empty() {
                out.push(TextLabel {
                    text,
                    rect: t.galley.rect.translate(t.pos.to_vec2()),
                    clip,
                });
            }
        }
        _ => {}
    }
}

fn record_texts(outputs: Query<&EguiFullOutput>, mut labels: ResMut<TextLabels>) {
    let Some(output) = outputs.iter().find_map(|o| o.0.as_ref()) else {
        return;
    };
    labels.0.clear();
    for clipped in &output.shapes {
        collect(&clipped.shape, clipped.clip_rect, &mut labels.0);
    }
}

impl TextLabels {
    /// Center of the topmost visible text equal to `label`, else containing it.
    fn find(&self, label: &str) -> Option<Vec2> {
        let pick = |exact: bool| {
            self.0.iter().rfind(|l| {
                l.clip.contains(l.rect.center())
                    && if exact {
                        l.text.trim() == label
                    } else {
                        l.text.contains(label)
                    }
            })
        };
        let found = pick(true).or_else(|| pick(false))?;
        let c = found.rect.center();
        Some(Vec2::new(c.x, c.y))
    }
}

/// Key by name (English or Portuguese): physical key code and logical key.
fn parse_key(name: &str) -> Option<(KeyCode, Key)> {
    let n = name.trim();
    let lower = n.to_lowercase();
    if n.chars().count() == 1 {
        let c = n.chars().next()?;
        let code = match c.to_ascii_uppercase() {
            'A' => KeyCode::KeyA,
            'B' => KeyCode::KeyB,
            'C' => KeyCode::KeyC,
            'D' => KeyCode::KeyD,
            'E' => KeyCode::KeyE,
            'F' => KeyCode::KeyF,
            'G' => KeyCode::KeyG,
            'H' => KeyCode::KeyH,
            'I' => KeyCode::KeyI,
            'J' => KeyCode::KeyJ,
            'K' => KeyCode::KeyK,
            'L' => KeyCode::KeyL,
            'M' => KeyCode::KeyM,
            'N' => KeyCode::KeyN,
            'O' => KeyCode::KeyO,
            'P' => KeyCode::KeyP,
            'Q' => KeyCode::KeyQ,
            'R' => KeyCode::KeyR,
            'S' => KeyCode::KeyS,
            'T' => KeyCode::KeyT,
            'U' => KeyCode::KeyU,
            'V' => KeyCode::KeyV,
            'W' => KeyCode::KeyW,
            'X' => KeyCode::KeyX,
            'Y' => KeyCode::KeyY,
            'Z' => KeyCode::KeyZ,
            '0' => KeyCode::Digit0,
            '1' => KeyCode::Digit1,
            '2' => KeyCode::Digit2,
            '3' => KeyCode::Digit3,
            '4' => KeyCode::Digit4,
            '5' => KeyCode::Digit5,
            '6' => KeyCode::Digit6,
            '7' => KeyCode::Digit7,
            '8' => KeyCode::Digit8,
            '9' => KeyCode::Digit9,
            '.' => KeyCode::Period,
            ',' => KeyCode::Comma,
            '-' => KeyCode::Minus,
            '+' | '=' => KeyCode::Equal,
            '/' => KeyCode::Slash,
            '_' => KeyCode::Minus,
            ' ' => return Some((KeyCode::Space, Key::Space)),
            _ => KeyCode::Unidentified(bevy::input::keyboard::NativeKeyCode::Unidentified),
        };
        let text = c.to_lowercase().to_string();
        return Some((code, Key::Character(text.as_str().into())));
    }
    let fixed = |code, key| Some((code, key));
    match lower.as_str() {
        "ctrl" | "control" => fixed(KeyCode::ControlLeft, Key::Control),
        "shift" => fixed(KeyCode::ShiftLeft, Key::Shift),
        "alt" => fixed(KeyCode::AltLeft, Key::Alt),
        "delete" | "del" => fixed(KeyCode::Delete, Key::Delete),
        "esc" | "escape" => fixed(KeyCode::Escape, Key::Escape),
        "enter" | "return" => fixed(KeyCode::Enter, Key::Enter),
        "tab" => fixed(KeyCode::Tab, Key::Tab),
        "space" | "espaço" | "espaco" => fixed(KeyCode::Space, Key::Space),
        "backspace" => fixed(KeyCode::Backspace, Key::Backspace),
        "home" => fixed(KeyCode::Home, Key::Home),
        "end" => fixed(KeyCode::End, Key::End),
        "setacima" | "up" | "arrowup" => fixed(KeyCode::ArrowUp, Key::ArrowUp),
        "setabaixo" | "down" | "arrowdown" => fixed(KeyCode::ArrowDown, Key::ArrowDown),
        "setaesquerda" | "left" | "arrowleft" => fixed(KeyCode::ArrowLeft, Key::ArrowLeft),
        "setadireita" | "right" | "arrowright" => fixed(KeyCode::ArrowRight, Key::ArrowRight),
        "f1" => fixed(KeyCode::F1, Key::F1),
        "f2" => fixed(KeyCode::F2, Key::F2),
        "f3" => fixed(KeyCode::F3, Key::F3),
        "f4" => fixed(KeyCode::F4, Key::F4),
        "f5" => fixed(KeyCode::F5, Key::F5),
        _ => None,
    }
}

/// Everything the runner writes to.
#[derive(SystemParam)]
struct Out<'w, 's> {
    commands: Commands<'w, 's>,
    window_events: MessageWriter<'w, WindowEvent>,
    moved: MessageWriter<'w, CursorMoved>,
    motion: MessageWriter<'w, MouseMotion>,
    buttons: MessageWriter<'w, MouseButtonInput>,
    wheel: MessageWriter<'w, MouseWheel>,
    keyboard: MessageWriter<'w, KeyboardInput>,
    exit: MessageWriter<'w, AppExit>,
}

/// Read-only view of the app the steps need.
#[derive(SystemParam)]
struct World3<'w, 's> {
    editor: Res<'w, Editor>,
    data: Res<'w, Data>,
    playback: Res<'w, Playback>,
    labels: Res<'w, TextLabels>,
    camera: Single<'w, 's, (&'static Camera, &'static GlobalTransform), With<Camera3d>>,
}

#[derive(Resource)]
pub struct Runner {
    steps: Vec<Passo>,
    dir: PathBuf,
    index: usize,
    /// Calls made to the current step so far.
    frame: u32,
    /// Seconds spent in the current step.
    secs: f64,
    /// Seconds since the start.
    total: f64,
    /// Logical pointer position.
    cursor: Vec2,
    /// Drag endpoints, resolved at the first frame of an `Arrastar`.
    drag: Option<(Vec2, Vec2)>,
    /// A click or move target resolved at frame 0.
    aim: Option<Vec2>,
    started: bool,
    /// Playback or camera change asked by `IrPara`/`Focar`, applied by [`apply_control`].
    control: Option<Passo>,
}

impl Runner {
    fn new(steps: Vec<Passo>, dir: PathBuf) -> Self {
        Runner {
            steps,
            dir,
            index: 0,
            frame: 0,
            secs: 0.0,
            total: 0.0,
            cursor: Vec2::new(10.0, 10.0),
            drag: None,
            aim: None,
            started: false,
            control: None,
        }
    }
}

/// The editor state as the assertions see it.
fn snapshot(world: &World3, window: &Window) -> Estado {
    let ed = &world.editor;
    let model = ed.doc.model();
    Estado {
        modo: match ed.mode {
            Mode::Edit => "Editar",
            Mode::Sim => "Simular",
        }
        .to_string(),
        selecionado: ed.selected.clone(),
        ferramenta: match &ed.tool {
            Tool::Select => "Selecionar".to_string(),
            Tool::Place(k) => format!("Colocar:{k}"),
            Tool::Connect(f) => format!("Ligar:{f}"),
        },
        objetos: model.objects.len(),
        ligacoes: model.connections.len(),
        problemas: ed.issues.len(),
        sujo: ed.doc.is_dirty(),
        titulo: window.title.clone(),
        status: ed.status.clone(),
        tempo: (ed.mode == Mode::Sim).then_some(world.playback.t),
        posicoes: model
            .objects
            .iter()
            .map(|o| (o.name.clone(), o.position))
            .collect(),
        fluxos: model
            .flows
            .iter()
            .map(|f| (f.name.clone(), f.activities.len()))
            .collect(),
        atividade: ed.flows.activity.clone(),
        painel_fluxos: ed.flows.open,
        selecionados: ed.selection(),
        ativos: model
            .objects
            .iter()
            .map(|o| {
                let asset = o.visual.as_ref().and_then(|v| v.asset.clone());
                (o.name.clone(), asset)
            })
            .collect(),
        ligacoes_lista: model.connections.clone(),
    }
}

fn project(world: &World3, point: Vec3, window: &Window) -> Result<Vec2, String> {
    let (cam, tf) = *world.camera;
    let p = cam
        .world_to_viewport(tf, point)
        .map_err(|e| format!("projeção falhou: {e}"))?;
    if p.x < 0.0 || p.y < 0.0 || p.x > window.width() || p.y > window.height() {
        return Err(format!(
            "ponto fora da janela ({:.0}, {:.0}) de {:.0}x{:.0}",
            p.x,
            p.y,
            window.width(),
            window.height()
        ));
    }
    Ok(p)
}

/// Window position of a target.
fn resolve(target: &Alvo, world: &World3, window: &Window) -> Result<Vec2, String> {
    match target {
        Alvo::Tela(x, y) => Ok(Vec2::new(*x, *y)),
        Alvo::Deslocado(inner, dx, dy) => Ok(resolve(inner, world, window)? + Vec2::new(*dx, *dy)),
        Alvo::Botao(label) => world
            .labels
            .find(label)
            .ok_or_else(|| format!("rótulo \"{label}\" não encontrado na tela")),
        Alvo::Mundo(x, y) => {
            let p = Vec3::from(layout::to_bevy([*x, *y, 0.0]));
            project(world, p, window)
        }
        Alvo::Objeto(name) => {
            let (base, half) = if world.editor.mode == Mode::Edit {
                let i = world
                    .editor
                    .view
                    .index_of(name)
                    .ok_or_else(|| format!("objeto \"{name}\" não existe"))?;
                (
                    world.editor.view.positions[i],
                    world.editor.view.geoms[i].half,
                )
            } else {
                let i = world
                    .data
                    .replay
                    .blocks
                    .iter()
                    .position(|b| &b.name == name)
                    .ok_or_else(|| format!("bloco \"{name}\" não existe na simulação"))?;
                (world.data.positions[i], world.data.geoms[i].half)
            };
            project(world, Vec3::from(base) + Vec3::Y * half[1], window)
        }
    }
}

impl Out<'_, '_> {
    fn move_cursor(&mut self, runner: &mut Runner, window: &mut Window, to: Vec2, entity: Entity) {
        let delta = to - runner.cursor;
        runner.cursor = to;
        let scale = window.scale_factor() as f64;
        window.set_physical_cursor_position(Some(DVec2::new(
            to.x as f64 * scale,
            to.y as f64 * scale,
        )));
        let moved = CursorMoved {
            window: entity,
            position: to,
            delta: Some(delta),
        };
        self.window_events
            .write(WindowEvent::CursorMoved(moved.clone()));
        self.moved.write(moved);
        if delta != Vec2::ZERO {
            let motion = MouseMotion { delta };
            self.window_events.write(WindowEvent::MouseMotion(motion));
            self.motion.write(motion);
        }
    }

    fn button(&mut self, button: MouseButton, state: ButtonState, entity: Entity) {
        let event = MouseButtonInput {
            button,
            state,
            window: entity,
        };
        self.window_events
            .write(WindowEvent::MouseButtonInput(event));
        self.buttons.write(event);
    }

    fn key(&mut self, code: KeyCode, key: Key, state: ButtonState, entity: Entity) {
        let text = match (&key, state) {
            (Key::Character(c), ButtonState::Pressed) => Some(c.clone()),
            (Key::Space, ButtonState::Pressed) => Some(" ".into()),
            _ => None,
        };
        let event = KeyboardInput {
            key_code: code,
            logical_key: key,
            state,
            text,
            repeat: false,
            window: entity,
        };
        self.window_events
            .write(WindowEvent::KeyboardInput(event.clone()));
        self.keyboard.write(event);
    }

    fn wheel(&mut self, lines: f32, entity: Entity) {
        let event = MouseWheel {
            unit: MouseScrollUnit::Line,
            x: 0.0,
            y: lines,
            phase: bevy::input::touch::TouchPhase::Moved,
            window: entity,
        };
        self.window_events.write(WindowEvent::MouseWheel(event));
        self.wheel.write(event);
    }
}

fn mouse_button(b: Botao) -> MouseButton {
    match b {
        Botao::Esquerdo => MouseButton::Left,
        Botao::Direito => MouseButton::Right,
    }
}

/// Outcome of one call to a step.
enum Flow {
    Pending,
    Done,
    Fail(String),
}

fn describe(step: &Passo) -> String {
    format!("{step:?}")
}

fn log_line(dir: &Path, line: &str) {
    if let Ok(mut f) = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("log.txt"))
    {
        let _ = writeln!(f, "{line}");
    }
}

/// The runner system: one call per frame advances the current step.
fn run(
    time: Res<Time<Real>>,
    mut runner: ResMut<Runner>,
    world: World3,
    mut out: Out,
    mut window: Single<(Entity, &mut Window), With<PrimaryWindow>>,
) {
    let dt = time.delta_secs_f64();
    let (entity, window) = &mut *window;
    let entity = *entity;
    runner.total += dt;
    runner.secs += dt;

    // Let the first frames render so the camera and egui have a layout.
    if !runner.started {
        if runner.total < 2.5 {
            return;
        }
        runner.started = true;
        runner.secs = 0.0;
    }

    if runner.index >= runner.steps.len() {
        log_line(&runner.dir, "fim: todos os passos passaram");
        println!("roteiro concluído: {} passos", runner.steps.len());
        out.exit.write(AppExit::Success);
        runner.index = usize::MAX / 2;
        return;
    }
    if runner.index > runner.steps.len() {
        return;
    }
    if runner.total > RUN_LIMIT {
        fail(
            &mut runner,
            &mut out,
            "limite de tempo do roteiro excedido".into(),
            &world,
            window,
        );
        return;
    }

    let step = runner.steps[runner.index].clone();
    let state = snapshot(&world, window);
    let flow = advance(&step, &mut runner, &world, &mut out, window, entity, &state);
    runner.frame += 1;
    match flow {
        Flow::Pending => {}
        Flow::Done => {
            let line = format!(
                "[{:03}] {} -> ok | {}",
                runner.index + 1,
                describe(&step),
                state.resumo()
            );
            log_line(&runner.dir, &line);
            runner.index += 1;
            runner.frame = 0;
            runner.secs = 0.0;
            runner.drag = None;
            runner.aim = None;
        }
        Flow::Fail(reason) => fail(&mut runner, &mut out, reason, &world, window),
    }
}

fn fail(runner: &mut Runner, out: &mut Out, reason: String, world: &World3, window: &Window) {
    let step = runner
        .steps
        .get(runner.index)
        .map_or("-".to_string(), describe);
    let state = snapshot(world, window);
    let line = format!(
        "[{:03}] {} -> FALHOU: {} | {}",
        runner.index + 1,
        step,
        reason,
        state.resumo()
    );
    log_line(&runner.dir, &line);
    eprintln!(
        "roteiro falhou no passo {} ({}): {}",
        runner.index + 1,
        step,
        reason
    );
    out.exit.write(AppExit::from_code(1));
    runner.index = runner.steps.len() + 1;
}

/// One call to a step; `runner.frame` counts the calls so far.
fn advance(
    step: &Passo,
    runner: &mut Runner,
    world: &World3,
    out: &mut Out,
    window: &mut Window,
    entity: Entity,
    state: &Estado,
) -> Flow {
    let n = runner.frame;
    match step {
        Passo::Esperar { quadros } => {
            if n >= *quadros {
                Flow::Done
            } else {
                Flow::Pending
            }
        }
        Passo::EsperarSegundos(s) => {
            if runner.secs >= *s {
                Flow::Done
            } else {
                Flow::Pending
            }
        }
        Passo::EsperarAte(cond, limit) => match state.verificar(cond) {
            Ok(()) => Flow::Done,
            Err(e) if runner.secs > *limit => {
                Flow::Fail(format!("tempo esgotado ({limit} s) esperando: {e}"))
            }
            Err(_) => Flow::Pending,
        },
        Passo::MoverPara(target) => match n {
            0 => match resolve(target, world, window) {
                Ok(p) => {
                    out.move_cursor(runner, window, p, entity);
                    Flow::Pending
                }
                Err(e) => Flow::Fail(e),
            },
            1 => Flow::Pending,
            _ => Flow::Done,
        },
        Passo::Clicar(b) => match n {
            0 => {
                out.button(mouse_button(*b), ButtonState::Pressed, entity);
                Flow::Pending
            }
            1 => {
                out.button(mouse_button(*b), ButtonState::Released, entity);
                Flow::Pending
            }
            _ => Flow::Done,
        },
        Passo::Pressionar(b) => match n {
            0 => {
                out.button(mouse_button(*b), ButtonState::Pressed, entity);
                Flow::Pending
            }
            _ => Flow::Done,
        },
        Passo::Soltar(b) => match n {
            0 => {
                out.button(mouse_button(*b), ButtonState::Released, entity);
                Flow::Pending
            }
            _ => Flow::Done,
        },
        Passo::Arrastar { de, para } => drag(runner, world, out, window, entity, de, para),
        Passo::Rolar(lines) => match n {
            0 => {
                out.wheel(*lines, entity);
                Flow::Pending
            }
            1 | 2 => Flow::Pending,
            _ => Flow::Done,
        },
        Passo::Tecla(name) => {
            let Some((code, key)) = parse_key(name) else {
                return Flow::Fail(format!("tecla desconhecida: \"{name}\""));
            };
            match n {
                0 => {
                    out.key(code, key, ButtonState::Pressed, entity);
                    Flow::Pending
                }
                1 => {
                    out.key(code, key, ButtonState::Released, entity);
                    Flow::Pending
                }
                _ => Flow::Done,
            }
        }
        Passo::Atalho(names) => {
            let mut keys = Vec::new();
            for name in names {
                match parse_key(name) {
                    Some(k) => keys.push(k),
                    None => return Flow::Fail(format!("tecla desconhecida: \"{name}\"")),
                }
            }
            match n {
                0 => {
                    for (code, key) in &keys {
                        out.key(*code, key.clone(), ButtonState::Pressed, entity);
                    }
                    Flow::Pending
                }
                1 => {
                    for (code, key) in keys.iter().rev() {
                        out.key(*code, key.clone(), ButtonState::Released, entity);
                    }
                    Flow::Pending
                }
                _ => Flow::Done,
            }
        }
        Passo::Segurar(name) | Passo::LargarTecla(name) => {
            let Some((code, key)) = parse_key(name) else {
                return Flow::Fail(format!("tecla desconhecida: \"{name}\""));
            };
            let state = if matches!(step, Passo::Segurar(_)) {
                ButtonState::Pressed
            } else {
                ButtonState::Released
            };
            match n {
                0 => {
                    out.key(code, key, state, entity);
                    Flow::Pending
                }
                1 => Flow::Pending,
                _ => Flow::Done,
            }
        }
        Passo::Digitar(text) => match n {
            0 => {
                let text = text.replace("{capturas}", &runner.dir.to_string_lossy());
                for c in text.chars() {
                    let Some((code, key)) = parse_key(&c.to_string()) else {
                        return Flow::Fail(format!("caractere não digitável: {c:?}"));
                    };
                    out.key(code, key.clone(), ButtonState::Pressed, entity);
                    out.key(code, key, ButtonState::Released, entity);
                }
                Flow::Pending
            }
            1 => Flow::Pending,
            _ => Flow::Done,
        },
        Passo::ClicarBotao(label) => match n {
            0 => match resolve(&Alvo::Botao(label.clone()), world, window) {
                Ok(p) => {
                    out.move_cursor(runner, window, p, entity);
                    Flow::Pending
                }
                Err(e) => Flow::Fail(e),
            },
            1 => Flow::Pending,
            2 => {
                out.button(MouseButton::Left, ButtonState::Pressed, entity);
                Flow::Pending
            }
            3 => {
                out.button(MouseButton::Left, ButtonState::Released, entity);
                Flow::Pending
            }
            4 => Flow::Pending,
            _ => Flow::Done,
        },
        Passo::Captura(name) => capture(runner, out, name, n),
        Passo::IrPara(_) | Passo::Focar { .. } => match n {
            0 => {
                if let Passo::Focar { objeto, .. } = step
                    && !world.data.replay.blocks.iter().any(|b| &b.name == objeto)
                {
                    return Flow::Fail(format!("bloco \"{objeto}\" não existe na simulação"));
                }
                if world.editor.mode != Mode::Sim {
                    return Flow::Fail("IrPara/Focar exigem o modo Simular".into());
                }
                runner.control = Some(step.clone());
                Flow::Pending
            }
            1..=3 => Flow::Pending,
            _ => Flow::Done,
        },
        Passo::Afirmar(cond) => match state.verificar(cond) {
            Ok(()) => Flow::Done,
            Err(e) => Flow::Fail(e),
        },
    }
}

/// Press, several moves, release. Endpoints are resolved at the first frame.
fn drag(
    runner: &mut Runner,
    world: &World3,
    out: &mut Out,
    window: &mut Window,
    entity: Entity,
    from: &Alvo,
    to: &Alvo,
) -> Flow {
    let n = runner.frame;
    if n == 0 {
        let a = match resolve(from, world, window) {
            Ok(p) => p,
            Err(e) => return Flow::Fail(format!("origem do arrasto: {e}")),
        };
        let b = match resolve(to, world, window) {
            Ok(p) => p,
            Err(e) => return Flow::Fail(format!("destino do arrasto: {e}")),
        };
        runner.drag = Some((a, b));
        out.move_cursor(runner, window, a, entity);
        return Flow::Pending;
    }
    let Some((a, b)) = runner.drag else {
        return Flow::Fail("arrasto sem pontos".into());
    };
    if n == 1 {
        out.button(MouseButton::Left, ButtonState::Pressed, entity);
        return Flow::Pending;
    }
    if n <= 1 + DRAG_FRAMES {
        let k = (n - 1) as f32 / DRAG_FRAMES as f32;
        out.move_cursor(runner, window, a.lerp(b, k), entity);
        return Flow::Pending;
    }
    if n == 2 + DRAG_FRAMES {
        out.button(MouseButton::Left, ButtonState::Released, entity);
        return Flow::Pending;
    }
    if n <= 4 + DRAG_FRAMES {
        return Flow::Pending;
    }
    Flow::Done
}

/// Screenshot of the primary window, written to `<dir>/<name>.png`.
fn capture(runner: &mut Runner, out: &mut Out, name: &str, n: u32) -> Flow {
    let path = runner.dir.join(format!("{name}.png"));
    match n {
        0 => {
            let _ = fs::remove_file(&path);
            Flow::Pending
        }
        1 | 2 => Flow::Pending, // let the last input show on screen
        3 => {
            out.commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path));
            Flow::Pending
        }
        _ => {
            let ready = fs::metadata(&path).is_ok_and(|m| m.len() > 0);
            if ready {
                Flow::Done
            } else if runner.secs > CAPTURE_LIMIT {
                Flow::Fail(format!("a captura {name}.png não foi gravada"))
            } else {
                Flow::Pending
            }
        }
    }
}

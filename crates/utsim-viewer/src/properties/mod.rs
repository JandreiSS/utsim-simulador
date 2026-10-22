//! Property editor for the selected object (phase 3). Pure egui: it reads the
//! [`Document`](utsim_edit::Document) and returns [`Command`](utsim_edit::Command)s;
//! it never mutates the model itself, so the editor applies them through the
//! document (undo/redo, validation).
//!
//! The logic that does not need a UI lives in [`spec`] (which fields exist, their
//! current value, distribution families) and [`ron_text`] (value to RON text), and
//! is unit-tested without a GPU. [`panel`] only draws.

pub(crate) mod panel;
pub mod ron_text;
pub mod spec;

pub use panel::{properties_panel, run_panel};

#[cfg(test)]
mod tests {
    use std::path::Path;

    use bevy_egui::egui;
    use utsim_core::{Dist, Downtime, OffWindow, OutputRule, ShiftTable};
    use utsim_edit::{Command, Document};
    use utsim_model::{Model, ObjectDef, ObjectKind};

    use super::spec::{self, FAMILIES};
    use super::*;

    const KITCHEN_SINK: &str = r#"Model(
        name: "tudo",
        run: (stop_time: 500.0, warmup: 10.0, seed: 7, replications: 3),
        objects: [
            (name: "emp1", position: (1.0, 2.0, 0.0), kind: Transporter(
                speed: 2.5,
                acceleration: 1.0,
                deceleration: 1.5,
                capacity: 2,
                load_time: Const(0.5),
                unload_time: Uniform(min: 0.1, max: 0.3),
            )),
            (name: "emp2", kind: Transporter(speed: 1.0)),
            (name: "equipe", kind: Dispatcher(team: ["emp1", "emp2"], rule: Nearest)),
            (name: "src", output: RoundRobin, transport: "equipe", kind: Source(
                interarrival: Exp(mean: 2.0),
                first_at_zero: true,
                max_arrivals: 50,
                on_blocked: Drop,
                labels: {
                    "tipo \"a\"": Gamma(shape: 2.0, scale: 1.5),
                    "b": Empirical([1.0, 2.0, 5.0]),
                },
            )),
            (name: "q", output: Random, kind: Queue(
                capacity: 8,
                discipline: Lifo,
                batch: (size: 4, max_wait: 2.5, flush: true),
            )),
            (name: "maq", output: Weighted([1.0, 2.0, 0.5]),
                kind: Processor(
                    capacity: 3,
                    setup: Uniform(min: 1.0, max: 2.0),
                    process: Weibull(shape: 1.5, scale: 4.0),
                    operators: (team: "emp1", count: 2, setup: false, process: true),
                ),
                downtime: (mtbf: Exp(mean: 100.0), mttr: Triangular(min: 1.0, mode: 2.0, max: 5.0)),
                shifts: (period: 1440.0, off: [
                    (start: 360.0, duration: 60.0),
                    (start: 960.0, duration: 480.0),
                ])),
            (name: "mont", output: ByLabel("porta \\ 2"), kind: Combiner(
                components: [1, 3],
                mode: Join,
                setup: Const(0.5),
                process: Normal(mean: 5.0, sd: 1.0),
            )),
            (name: "sep", kind: Separator(mode: Split(3), process: Poisson(mean: 2.0))),
            (name: "esteira", kind: Conveyor(
                length: 12.5,
                speed: 0.75,
                item_length: 0.5,
                accumulating: false,
            )),
            (name: "estante", kind: Rack(
                bays: 5,
                levels: 3,
                dwell: Uniform(min: 1.0, max: 2.0),
                placement: Random,
                bay_width: 1.2,
                level_height: 1.5,
            )),
            (name: "fim", kind: Sink),
        ],
        connections: [],
    )"#;

    fn apply_set(doc: &mut Document, key: &str, value: &str) {
        doc.apply(Command::Set {
            key: key.into(),
            value: value.into(),
        })
        .unwrap_or_else(|e| panic!("{key} = {value}: {e}"));
    }

    fn all_families() -> Vec<Dist> {
        let mut dists: Vec<Dist> = (0..FAMILIES.len())
            .map(|i| spec::family_default(i, 3.0))
            .collect();
        dists.push(Dist::Shifted {
            offset: 1.5,
            dist: Box::new(Dist::Shifted {
                offset: -0.25,
                dist: Box::new(Dist::Discrete(vec![(1.0, 2.0), (4.0, 1e-7)])),
            }),
        });
        dists.push(Dist::Const(-1.0e21));
        dists
    }

    fn kitchen_sink() -> Document {
        let model = Model::parse(KITCHEN_SINK).unwrap();
        let mut doc = Document::new("tudo");
        for object in model.objects {
            doc.apply(Command::Add(Box::new(object))).unwrap();
        }
        doc.apply(Command::Set {
            key: "run.stop_time".into(),
            value: "500.0".into(),
        })
        .unwrap();
        doc
    }

    fn object(doc: &Document, name: &str) -> ObjectDef {
        doc.model()
            .objects
            .iter()
            .find(|o| o.name == name)
            .unwrap()
            .clone()
    }

    fn field(doc: &Document, name: &str, key: &str) -> String {
        spec::read_field(&object(doc, name), key).unwrap()
    }

    fn documents() -> Vec<(String, Document)> {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
        let mut docs = vec![("tudo".to_string(), kitchen_sink())];
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            let stem = path.file_stem().unwrap().to_string_lossy().to_string();
            if path.extension().is_some_and(|e| e == "ron") && !stem.starts_with("experimento") {
                docs.push((stem, Document::open(&path).unwrap()));
            }
        }
        assert!(docs.len() > 5);
        docs
    }

    #[test]
    fn reading_a_field_and_writing_it_back_changes_nothing() {
        for (name, mut doc) in documents() {
            let objects: Vec<ObjectDef> = doc.model().objects.clone();
            for obj in &objects {
                for f in spec::all_fields(&obj.kind) {
                    let value = spec::read_field(obj, f.key)
                        .unwrap_or_else(|| panic!("{name}.{}: no value", f.key));
                    let before = doc.model().clone();
                    apply_set(&mut doc, &format!("{}.{}", obj.name, f.key), &value);
                    assert_eq!(doc.model(), &before, "{name}.{}.{}", obj.name, f.key);
                }
            }
            for f in spec::run_fields() {
                let value = spec::read_run(&doc.model().run, f.key).unwrap();
                let before = doc.model().clone();
                apply_set(&mut doc, &format!("run.{}", f.key), &value);
                assert_eq!(doc.model(), &before, "{name} run.{}", f.key);
            }
        }
    }

    #[test]
    fn the_spec_lists_every_field_of_every_kind() {
        // A field missing from the spec would never be editable.
        let doc = kitchen_sink();
        for obj in &doc.model().objects {
            let keys: Vec<&str> = spec::all_fields(&obj.kind).iter().map(|f| f.key).collect();
            let expected: &[&str] = match &obj.kind {
                ObjectKind::Source { .. } => &[
                    "interarrival",
                    "first_at_zero",
                    "max_arrivals",
                    "labels",
                    "on_blocked",
                ],
                ObjectKind::Queue { .. } => &["capacity", "discipline", "batch"],
                ObjectKind::Processor { .. } => &["capacity", "setup", "process", "operators"],
                ObjectKind::Combiner { .. } => &["components", "mode", "setup", "process"],
                ObjectKind::Separator { .. } => &["mode", "process"],
                ObjectKind::Sink => &[],
                ObjectKind::Transporter { .. } => &[
                    "speed",
                    "acceleration",
                    "deceleration",
                    "capacity",
                    "load_time",
                    "unload_time",
                    "home",
                ],
                ObjectKind::Dispatcher { .. } => &["team", "rule"],
                ObjectKind::Conveyor { .. } => &["length", "speed", "item_length", "accumulating"],
                ObjectKind::Rack { .. } => &[
                    "bays",
                    "levels",
                    "dwell",
                    "placement",
                    "bay_width",
                    "level_height",
                ],
            };
            // Mobile resources have no ports: no output rule and no `transport`.
            let common: &[&str] = if utsim_model::is_mobile(&obj.kind) {
                &["downtime", "shifts"]
            } else {
                &["output", "downtime", "shifts", "transport"]
            };
            for k in expected.iter().chain(common) {
                assert!(keys.contains(k), "{}: missing {k}", obj.name);
            }
            assert_eq!(keys.len(), expected.len() + common.len(), "{}", obj.name);
        }
    }

    #[test]
    fn every_distribution_family_survives_text_and_lands_in_the_model() {
        let mut doc = kitchen_sink();
        for d in all_families() {
            for key in ["maq.process", "src.interarrival", "maq.setup"] {
                apply_set(&mut doc, key, &ron_text::dist(&d));
            }
            let ObjectKind::Processor { process, setup, .. } = object(&doc, "maq").kind else {
                unreachable!()
            };
            assert_eq!(process, d);
            assert_eq!(setup, Some(d.clone()));
            let ObjectKind::Source { interarrival, .. } = object(&doc, "src").kind else {
                unreachable!()
            };
            assert_eq!(interarrival, d);
        }
    }

    #[test]
    fn edited_scalars_options_and_enums_land_in_the_model() {
        let mut doc = kitchen_sink();
        let edits = [
            ("maq.capacity", "7"),
            ("q.capacity", "None"),
            ("q.discipline", "Fifo"),
            ("src.max_arrivals", "None"),
            ("src.first_at_zero", "false"),
            ("src.on_blocked", "Wait"),
            ("mont.mode", "Batch"),
            ("mont.components", "[2]"),
            ("mont.setup", "None"),
            ("sep.mode", "Split(5)"),
            ("sep.mode", "Unpack"),
            ("sep.mode", "Split(6)"),
        ];
        for (key, value) in edits {
            apply_set(&mut doc, key, value);
            let (name, f) = key.split_once('.').unwrap();
            assert_eq!(field(&doc, name, f), value, "{key}");
        }
        assert!(matches!(
            object(&doc, "maq").kind,
            ObjectKind::Processor { capacity: 7, .. }
        ));
        assert!(matches!(
            object(&doc, "mont").kind,
            ObjectKind::Combiner { setup: None, .. }
        ));

        apply_set(&mut doc, "run.stop_time", &ron_text::float(123.5));
        apply_set(&mut doc, "run.warmup", "0.0");
        apply_set(&mut doc, "run.seed", "99");
        apply_set(&mut doc, "run.replications", "4");
        let run = &doc.model().run;
        assert_eq!(
            (run.stop_time, run.warmup, run.seed, run.replications),
            (123.5, 0.0, 99, 4)
        );
    }

    #[test]
    fn mobile_resource_fields_land_in_the_model() {
        let mut doc = kitchen_sink();
        for (key, value) in [
            ("emp1.speed", ron_text::float(3.5)),
            ("emp1.acceleration", "None".to_string()),
            ("emp1.deceleration", ron_text::float(0.75)),
            ("emp1.capacity", "4".to_string()),
            ("emp1.load_time", ron_text::dist(&Dist::Exp { mean: 2.0 })),
            ("emp1.unload_time", ron_text::dist(&Dist::Const(0.0))),
            ("equipe.team", ron_text::strings(&["emp2".to_string()])),
            ("equipe.rule", "RoundRobin".to_string()),
            (
                "maq.operators",
                ron_text::operators("equipe", 1, true, false),
            ),
            ("src.transport", ron_text::string("emp2")),
        ] {
            apply_set(&mut doc, key, &value);
            let (name, f) = key.split_once('.').unwrap();
            assert_eq!(field(&doc, name, f), value, "{key}");
        }
        let ObjectKind::Transporter {
            speed,
            acceleration,
            deceleration,
            capacity,
            load_time,
            unload_time,
            ..
        } = object(&doc, "emp1").kind
        else {
            unreachable!()
        };
        assert_eq!(
            (speed, acceleration, deceleration, capacity),
            (3.5, None, Some(0.75), 4)
        );
        assert_eq!(load_time, Dist::Exp { mean: 2.0 });
        assert_eq!(unload_time, Dist::Const(0.0));
        let ObjectKind::Dispatcher { team, rule } = object(&doc, "equipe").kind else {
            unreachable!()
        };
        assert_eq!(team, vec!["emp2".to_string()]);
        assert_eq!(format!("{rule:?}"), "RoundRobin");
        let ObjectKind::Processor { operators, .. } = object(&doc, "maq").kind else {
            unreachable!()
        };
        let op = operators.unwrap();
        assert_eq!(
            (op.team.as_str(), op.count, op.setup, op.process),
            ("equipe", 1, true, false)
        );
        assert_eq!(object(&doc, "src").transport.as_deref(), Some("emp2"));
    }

    #[test]
    fn conveyor_and_rack_fields_land_in_the_model() {
        let mut doc = kitchen_sink();
        for (key, value) in [
            ("esteira.length", ron_text::float(20.0)),
            ("esteira.speed", ron_text::float(1.25)),
            ("esteira.item_length", ron_text::float(0.8)),
            ("esteira.accumulating", "true".to_string()),
            ("estante.bays", "8".to_string()),
            ("estante.levels", "2".to_string()),
            ("estante.dwell", "None".to_string()),
            ("estante.placement", "FirstFree".to_string()),
            ("estante.bay_width", ron_text::float(2.5)),
            ("estante.level_height", ron_text::float(0.75)),
        ] {
            apply_set(&mut doc, key, &value);
            let (name, f) = key.split_once('.').unwrap();
            assert_eq!(field(&doc, name, f), value, "{key}");
        }
        let ObjectKind::Conveyor {
            length,
            speed,
            item_length,
            accumulating,
        } = object(&doc, "esteira").kind
        else {
            unreachable!()
        };
        assert_eq!(
            (length, speed, item_length, accumulating),
            (20.0, 1.25, 0.8, true)
        );
        let ObjectKind::Rack {
            bays,
            levels,
            dwell,
            placement,
            bay_width,
            level_height,
        } = object(&doc, "estante").kind
        else {
            unreachable!()
        };
        assert_eq!((bays, levels, dwell), (8, 2, None));
        assert_eq!(format!("{placement:?}"), "FirstFree");
        assert_eq!((bay_width, level_height), (2.5, 0.75));
        // The dwell switches back on with a distribution.
        let d = Dist::Exp { mean: 5.0 };
        apply_set(
            &mut doc,
            "estante.dwell",
            &ron_text::option(&Some(d.clone()), ron_text::dist),
        );
        assert_eq!(
            field(&doc, "estante", "dwell"),
            ron_text::option(&Some(d), ron_text::dist)
        );
        for (text, _) in spec::RACK_PLACEMENTS {
            apply_set(&mut doc, "estante.placement", text);
            assert_eq!(field(&doc, "estante", "placement"), text);
        }
        // Values the model rejects are reported, not applied.
        let before = doc.model().clone();
        assert!(
            doc.apply(Command::Set {
                key: "estante.bays".into(),
                value: "-3".into(),
            })
            .is_err()
        );
        assert_eq!(doc.model(), &before);
    }

    #[test]
    fn drawing_a_conveyor_and_a_rack_emits_nothing() {
        let doc = kitchen_sink();
        assert_eq!(render(&doc, "esteira"), Vec::new());
        assert_eq!(render(&doc, "estante"), Vec::new());
    }

    #[test]
    fn mobile_resources_are_listed_for_the_pickers() {
        let doc = kitchen_sink();
        let objects = &doc.model().objects;
        assert_eq!(spec::mobile_names(objects), vec!["emp1", "emp2", "equipe"]);
        assert_eq!(spec::transporter_names(objects), vec!["emp1", "emp2"]);
        for (text, _) in spec::DISPATCH_RULES {
            let mut d = kitchen_sink();
            apply_set(&mut d, "equipe.rule", text);
            assert_eq!(field(&d, "equipe", "rule"), text);
        }
        // Optional sections switch off with `None`.
        let mut d = kitchen_sink();
        apply_set(&mut d, "maq.operators", "None");
        apply_set(&mut d, "src.transport", "None");
        assert_eq!(field(&d, "maq", "operators"), "None");
        assert_eq!(field(&d, "src", "transport"), "None");
    }

    #[test]
    fn edited_sections_land_in_the_model() {
        let mut doc = kitchen_sink();

        apply_set(&mut doc, "q.batch", &ron_text::batch(9, 0.0, false));
        match object(&doc, "q").kind {
            ObjectKind::Queue { batch: Some(b), .. } => {
                assert_eq!((b.size, b.max_wait, b.flush), (9, 0.0, false));
            }
            _ => unreachable!(),
        }
        apply_set(&mut doc, "q.batch", "None");
        assert_eq!(field(&doc, "q", "batch"), "None");

        let dt = Downtime {
            mtbf: Dist::Const(50.0),
            mttr: Dist::Erlang { k: 3, mean: 4.0 },
        };
        apply_set(&mut doc, "maq.downtime", &ron_text::downtime(&dt));
        assert_eq!(object(&doc, "maq").downtime, Some(dt));
        apply_set(&mut doc, "maq.downtime", "None");
        assert_eq!(object(&doc, "maq").downtime, None);

        let table = ShiftTable {
            period: 100.0,
            off: vec![OffWindow {
                start: 10.0,
                duration: 5.0,
            }],
        };
        apply_set(&mut doc, "maq.shifts", &ron_text::shifts(&table));
        assert_eq!(object(&doc, "maq").shifts, Some(table));
        apply_set(&mut doc, "maq.shifts", "None");
        assert_eq!(object(&doc, "maq").shifts, None);

        let rules = [
            OutputRule::FirstAvailable,
            OutputRule::RoundRobin,
            OutputRule::Random,
            OutputRule::Weighted(vec![0.0, 1.0, 2.5]),
            OutputRule::ByLabel("tipo \"x\" \\ y".into()),
        ];
        for rule in rules {
            apply_set(&mut doc, "maq.output", &ron_text::output(&rule));
            assert_eq!(object(&doc, "maq").output, rule);
        }

        let mut labels = std::collections::BTreeMap::new();
        labels.insert("peso".to_string(), Dist::Const(2.0));
        labels.insert("dois \"x\"".to_string(), Dist::Exp { mean: 0.5 });
        apply_set(&mut doc, "src.labels", &ron_text::labels(&labels));
        match object(&doc, "src").kind {
            ObjectKind::Source { labels: l, .. } => assert_eq!(l, labels),
            _ => unreachable!(),
        }
        apply_set(&mut doc, "src.labels", "{}");
        assert_eq!(field(&doc, "src", "labels"), "{}");
    }

    #[test]
    fn written_text_is_what_the_saved_file_parses_back_to() {
        let doc = kitchen_sink();
        assert_eq!(&Model::parse(&doc.to_text()).unwrap(), doc.model());
    }

    #[test]
    fn rejected_text_leaves_the_document_alone() {
        let mut doc = kitchen_sink();
        let before = doc.model().clone();
        let bad = Command::Set {
            key: "maq.process".into(),
            value: "Exp(mean:".into(),
        };
        assert!(doc.apply(bad).is_err());
        assert_eq!(doc.model(), &before);
    }

    #[test]
    fn on_entry_and_script_rules_draw_without_commands_and_round_trip() {
        let mut doc = Document::new("s");
        doc.apply(Command::Add(Box::new(
            utsim_edit::template("Queue", "a", [0.0; 3]).unwrap(),
        )))
        .unwrap();
        apply_set(&mut doc, "a.on_entry", "\"item.x = 1;\"");
        apply_set(&mut doc, "a.output", "Script(\"2\")");
        let commands = render(&doc, "a");
        assert_eq!(commands, Vec::new());
        let obj = &doc.model().objects[0];
        assert_eq!(
            spec::read_field(obj, "on_entry").as_deref(),
            Some("\"item.x = 1;\"")
        );
        // Writing back what the panel reads changes nothing; "None" clears it.
        let before = doc.model().clone();
        apply_set(&mut doc, "a.on_entry", "\"item.x = 1;\"");
        assert_eq!(doc.model(), &before);
        apply_set(&mut doc, "a.on_entry", "None");
        assert_eq!(doc.model().objects[0].on_entry, None);
    }

    fn render(doc: &Document, selected: &str) -> Vec<Command> {
        let ctx = egui::Context::default();
        let mut commands = Vec::new();
        // Two frames: the second one runs with the memory the first one left.
        for _ in 0..2 {
            ctx.run_ui(egui::RawInput::default(), |ui| {
                commands = properties_panel(ui, doc, selected);
            })
            .drop_without_applying_deltas();
        }
        commands
    }

    #[test]
    fn drawing_every_object_and_the_run_settings_emits_nothing() {
        for (name, doc) in documents() {
            for obj in &doc.model().objects {
                let commands = render(&doc, &obj.name);
                assert_eq!(commands, Vec::new(), "{name}.{}", obj.name);
            }
            assert_eq!(render(&doc, ""), Vec::new(), "{name} run");
        }
    }

    #[test]
    fn drawing_every_distribution_family_does_not_panic() {
        let mut doc = kitchen_sink();
        for d in all_families() {
            apply_set(&mut doc, "maq.process", &ron_text::dist(&d));
            apply_set(&mut doc, "src.interarrival", &ron_text::dist(&d));
            assert_eq!(render(&doc, "maq"), Vec::new());
            assert_eq!(render(&doc, "src"), Vec::new());
        }
        // Invalid parameters are shown, not rejected.
        apply_set(&mut doc, "maq.process", "Uniform(min: 5.0, max: 1.0)");
        assert_eq!(render(&doc, "maq"), Vec::new());
    }
}

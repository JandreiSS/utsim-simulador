## Main scene of the Godot interface (phases G1 to G3): build a model by placing and
## connecting blocks in 3D (Editar), then run the replications in Rust and play the
## replay of replication 0 (Simular).
##
## G3: the path network is drawn and edited with the Rede tool (`rede.gd`); runs go to a
## thread of the bridge (`UtsimJob`, polled every frame, with progress and Cancelar), the
## "Resultados" window (egui, `ui_results.rs` in the bridge) shows the table, the charts, the output analyses,
## scenarios and optimisation; the PERT/CPM window of the selection (egui, `ui_pert.rs`); the
## token flows in 2D (egui, `ui_flows.rs`); tanks with their levels (`cena.gd`); the cameras
## Seguir, Primeira pessoa and Do recurso; and recorded replays (`.json`) can be opened.
##
## The model being edited is a `UtsimDocument` (the bridge over `utsim_edit::Document`):
## every change is a command, validated and undoable. Editar draws the document with the
## same scene code as the playback (`UtsimDocument.edit_scene`) and the 3D interaction is
## `editor.gd`.
##
## Everything visible is built here in code (lights, floor, grid, camera, panels), so the
## `.tscn` stays a single node and the layout is easy to read in one file. Panels are
## opaque, so the 3D labels never show through them.
##
## The interface around the 3D view is drawn in Rust with egui (`UtsimUi`,
## `crates/utsim-godot/src/ui_*.rs`): the top bar (`_sync_bar`), the left panel with the
## library, the objects and the run (`_sync_edit`, `_sync_sim`), the properties, the status
## bar (`set_status`, the job's progress) and the playback bar (`_sync_play`). Its clicks
## come back as `bar_action(what)`, and its widgets keep the names of the Godot controls
## they replaced (`Modo_Editar`, `Biblioteca_Queue`, `Reproducao_Play`...), which the
## scripted runs find through the bridge. The Godot panels that remain (Resultados,
## PERT/CPM, Fluxos, the Rede and Corredor subpanels, the dialogs) come in later steps.
extends Node3D

## The widths of the side panels and the heights of the bars, as the egui interface draws
## them (`ui_left::WIDTH`, `ui_props::WIDTH`, the bar, `ui_bottom::STATUS_HEIGHT`): the
## Godot panels that remain are placed by them.
const LEFT_WIDTH := 290
const TOP_HEIGHT := 46
const BOTTOM_HEIGHT := 40
const RED := Color(1.0, 0.45, 0.42)
const YELLOW := Color(0.98, 0.80, 0.35)
const CAMERAS := [["orbita", "Órbita"], ["planta", "Planta (2D)"], ["seguir", "Seguir"], ["primeira_pessoa", "Primeira pessoa"], ["recurso", "Do recurso"]]

var doc: Object
var model: Object
var playback: Object
var cena: CenaUtsim
var editor: EditorUtsim
var camera: CameraOrbita
var sun: DirectionalLight3D
var t := 0.0
var playing := false
var speed := 1.0
var model_path := ""
## `editar` or `simular`.
var mode := "editar"
var snap_on := true
var grid_step := 1.0

var _ui: CanvasLayer
## The interface around the 3D view drawn in Rust with egui (`UtsimUi`, the bridge): the
## top bar for now; the other panels move to it in the next steps (plano_interface_rust.md).
var ui: Object
## Whether the right panel may show at all (Editar, no window covering it).
var _right_allowed := true
## The asset the egui panel's miniature shows (`miniatura` of the scripted checks).
var mini_asset := "-"
var _mini_view: SubViewport
var _mini_scene: CenaUtsim
var _mini_camera: Camera3D
## The status line (the egui status bar shows it; the scripted runs read it).
var status_text := ""
## The status line of the Simular panel ("Simulando..." while the run goes).
var _sim_status := ""
## Only when the bridge did not load: the line that says so.
var _status_fallback: Label
## The results summary of the Simular panel (`UtsimUi.set_sim_state`).
var _summary := {}
var _block_labels := PackedStringArray()
var labels_on := true
## Camadas de segurança e qualidade (camada_seguranca.gd): zonas, EPI, quase-conflitos e
## itens com defeito; e o mapa de calor.
var seguranca_on := false
var calor_on := false
var _open_dialog: FileDialog
var _save_dialog: FileDialog
var _grid: MeshInstance3D
## The performance line over the 3D view (F3), drawn by egui.
var _perf_on := false
var _revision := -1
var _pending := Callable()
## The replications of Simular (the field of the Simular panel).
var _run_reps := 5
## The run shown in Resultados (`UtsimResults`), and the job running now (`UtsimJob`).
var results: Object
var job: Object
var _job_done := Callable()
## The selection of the PERT/CPM window (the first name is the start of "de A até B").
var _pert_names := PackedStringArray()
var _pert_between := false
## `orbita`, `seguir`, `primeira_pessoa` or `recurso`.
var camera_mode := "orbita"
## Point the following cameras look at (scripted checks: nothing may stand between).
var camera_aim := Vector3.ZERO
## Block followed by the cameras other than the orbit one.
var follow_name := ""
var _frames := 0
var _replay_dialog: FileDialog
var _data_dialog: FileDialog
var _report_dialog: FileDialog
var _data_pick_path := ""
## Where the data file and report dialogs answer: the panel's form or the "Valores
## reais…" window (`painel`/`janela`, `modelo`/`janela`).
var _data_target := "painel"
var _report_target := "modelo"
## The last report of the real data written: `{index, files, failed}`.
var last_report := {}
## A recorded replay (`.json`) is being played instead of the document.
var playing_replay := false
## A scripted run (`roteiro.gd`) drives the window: no system dialogs, no browser.
var scripted := false
## The kinds of team and request lines hidden (the "Ligações" toggles of the bar).
var links_hidden: Array = []


func _ready() -> void:
	_build_world()
	_build_ui()
	for cls in [&"UtsimModel", &"UtsimDocument"]:
		if not ClassDB.class_exists(cls):
			set_status("A ponte utsim_godot não carregou. Rode scripts/godot.sh e abra o projeto de novo.")
			return
	doc = ClassDB.instantiate(&"UtsimDocument")
	doc.new_model("novo_modelo")
	if ui != null:
		ui.set_document(doc)
		ui.set_library_icons(_library_icons())
		_build_miniature()
	editor = EditorUtsim.new()
	editor.name = "Editor"
	add_child(editor)
	editor.setup(self, _ui)
	set_mode("editar")
	camera.frame_box(Vector3(-8, 0, -8), Vector3(8, 0, 8))
	set_status("Arraste um objeto da Biblioteca para o chão, ou abra um modelo (Ctrl+O).")


# ---------------------------------------------------------------------------
# World
# ---------------------------------------------------------------------------


func _build_world() -> void:
	var env := WorldEnvironment.new()
	var e := Environment.new()
	e.background_mode = Environment.BG_COLOR
	e.background_color = Color(0.10, 0.11, 0.14)
	e.ambient_light_source = Environment.AMBIENT_SOURCE_COLOR
	e.ambient_light_color = Color(0.80, 0.84, 0.92)
	e.ambient_light_energy = 0.7
	e.tonemap_mode = Environment.TONE_MAPPER_AGX
	e.ssao_enabled = true
	e.ssao_radius = 0.6
	e.ssao_intensity = 1.2
	env.environment = e
	add_child(env)
	sun = DirectionalLight3D.new()
	sun.light_energy = 1.5
	sun.shadow_enabled = true
	sun.shadow_bias = 0.03
	sun.directional_shadow_mode = DirectionalLight3D.SHADOW_PARALLEL_4_SPLITS
	sun.transform = Transform3D(Basis.IDENTITY, Vector3(8, 14, 6)).looking_at(Vector3.ZERO, Vector3.UP)
	add_child(sun)
	camera = CameraOrbita.new()
	camera.over_ui = over_ui
	camera.free_span = free_span
	add_child(camera)
	camera.current = true
	var ground := MeshInstance3D.new()
	var plane := PlaneMesh.new()
	plane.size = Vector2(2000, 2000)
	ground.mesh = plane
	var gm := StandardMaterial3D.new()
	gm.albedo_color = Color(0.30, 0.31, 0.32)
	gm.roughness = 0.95
	ground.material_override = gm
	ground.position = Vector3(0, -0.004, 0)
	ground.name = "Piso"
	add_child(ground)
	cena = CenaUtsim.new()
	cena.name = "Cena"
	cena.show_floor = false
	add_child(cena)


# ---------------------------------------------------------------------------
# Interface
# ---------------------------------------------------------------------------


func _build_ui() -> void:
	_ui = CanvasLayer.new()
	_ui.name = "Interface"
	add_child(_ui)

	_open_dialog = FileDialog.new()
	_open_dialog.name = "Dialogo_Abrir"
	_open_dialog.access = FileDialog.ACCESS_FILESYSTEM
	_open_dialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE
	_open_dialog.filters = PackedStringArray(["*.ron ; Modelos do UTSim", "*.json ; Replays gravados"])
	_open_dialog.title = "Abrir modelo"
	_open_dialog.file_selected.connect(func(p): open_model(p))
	_ui.add_child(_open_dialog)
	_save_dialog = FileDialog.new()
	_save_dialog.name = "Dialogo_Salvar"
	_save_dialog.access = FileDialog.ACCESS_FILESYSTEM
	_save_dialog.file_mode = FileDialog.FILE_MODE_SAVE_FILE
	_save_dialog.filters = PackedStringArray(["*.ron ; Modelos do UTSim"])
	_save_dialog.title = "Salvar modelo como"
	_save_dialog.file_selected.connect(func(p): save_as(p))
	_ui.add_child(_save_dialog)
	_data_dialog = FileDialog.new()
	_data_dialog.name = "Dialogo_Dados"
	_data_dialog.access = FileDialog.ACCESS_FILESYSTEM
	_data_dialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE
	_data_dialog.filters = PackedStringArray(["*.csv, *.txt, *.tsv ; Dados (CSV ou texto)"])
	_data_dialog.title = "Arquivo com os dados reais"
	_data_dialog.file_selected.connect(func(p): _on_data_file(p))
	_ui.add_child(_data_dialog)
	_report_dialog = FileDialog.new()
	_report_dialog.name = "Dialogo_Relatorio"
	_report_dialog.access = FileDialog.ACCESS_FILESYSTEM
	_report_dialog.file_mode = FileDialog.FILE_MODE_OPEN_DIR
	_report_dialog.title = "Pasta para o relatório dos dados"
	_report_dialog.dir_selected.connect(func(p):
		if _report_target == "janela":
			write_fit_report(p)
		else:
			write_data_report(p))
	_ui.add_child(_report_dialog)
	_replay_dialog = FileDialog.new()
	_replay_dialog.name = "Dialogo_Replay"
	_replay_dialog.access = FileDialog.ACCESS_FILESYSTEM
	_replay_dialog.file_mode = FileDialog.FILE_MODE_SAVE_FILE
	_replay_dialog.filters = PackedStringArray(["*.json ; Replays do UTSim"])
	_replay_dialog.title = "Gravar replay"
	_replay_dialog.file_selected.connect(func(p): save_replay(p))
	_ui.add_child(_replay_dialog)
	# The file dialogs are the system's own on a screen; a scripted run (`roteiro.gd`)
	# answers them itself, so it keeps the Godot ones.
	scripted = Array(OS.get_cmdline_user_args()).any(func(a): return String(a).begins_with("--utsim-roteiro="))
	_open_from_args.call_deferred()
	if DisplayServer.get_name() != "headless" and not scripted:
		for d in [_open_dialog, _save_dialog, _data_dialog, _report_dialog, _replay_dialog]:
			d.use_native_dialog = true
	if ClassDB.class_exists(&"UtsimUi"):
		ui = ClassDB.instantiate(&"UtsimUi")
		ui.name = "InterfaceRust"
		ui.action.connect(bar_action)
		_ui.add_child(ui)
	else:
		# Without the bridge there is no interface: one line says why.
		_status_fallback = Label.new()
		_status_fallback.name = "Status"
		_status_fallback.anchor_top = 1.0
		_status_fallback.anchor_right = 1.0
		_status_fallback.anchor_bottom = 1.0
		_status_fallback.offset_left = 12
		_status_fallback.offset_top = -BOTTOM_HEIGHT
		_ui.add_child(_status_fallback)


## The icons of the library (`res://icones/<id>.svg`), for the egui panel.
func _library_icons() -> Dictionary:
	var icons := {}
	for e in doc.palette():
		var path := "res://icones/%s.svg" % e["id"]
		if ResourceLoader.exists(path):
			icons[e["id"]] = load(path)
	return icons


## The performance line over the 3D view (frames per second, bridge and scene times):
## hidden by default, F3 shows and hides it.
func toggle_performance() -> void:
	_perf_on = not _perf_on
	if ui != null and not _perf_on:
		ui.set_performance("")
	set_status("Medidas de desempenho %s (F3)." % ("visíveis" if _perf_on else "ocultas"))


## The shortcuts window (F1): an egui dialog with the list of the bridge.
func toggle_shortcuts() -> void:
	if ui == null:
		return
	if ui.dialog_open():
		ui.show_dialog("")
		return
	var pairs: Array = Array(doc.shortcuts())
	pairs.append(["F3", "Mostra ou oculta as medidas de desempenho sobre a vista 3D"])
	ui.show_dialog(JSON.stringify({"name": "Janela_Atalhos", "title": "Atalhos", "lines": pairs,
		"buttons": [["Atalhos_Fechar", "Fechar", "fechar"]]}))


## Left and right x of the 3D view not covered by the side panels (`CameraOrbita.free_span`).
func free_span() -> Vector2:
	var w := get_viewport().get_visible_rect().size.x
	var lo := 0.0
	var hi := w
	if ui != null:
		# The free area egui leaves (the library or the run on the left, the properties).
		var free: Rect2 = ui.central_rect()
		if free.size.x > 0.0:
			lo = maxf(lo, free.position.x)
			hi = minf(hi, free.end.x)
	return Vector2(lo, hi)


## Whether screen point `p` is over a panel (the 3D view does not take it).
func over_ui(p: Vector2) -> bool:
	if ui != null and ui.is_over(p):
		return true
	return false


## The Resultados and PERT/CPM windows cover the view down to the playback bar in
## Simular and down to the status line in Editar (no bar there); while one is open in
## Editar, the properties panel, which they would half cover, is hidden.
func _sync_panels() -> void:
	var covering: bool = ui != null and (ui.pert_open() or ui.results_open())
	if mode == "editar":
		_set_right(not covering)


## Shows or hides the right panel (properties, or the panel of a tool).
func _set_right(on: bool) -> void:
	_right_allowed = on
	_apply_right()


func _apply_right() -> void:
	if ui != null:
		ui.set_properties_visible(_right_allowed)


func is_editing() -> bool:
	return mode == "editar" and doc != null


func snap_step() -> float:
	return grid_step if snap_on else 0.0


func set_snap(on: bool, step: float) -> void:
	snap_on = on
	grid_step = step
	_sync_bar()
	_draw_edit_grid()
	set_status("Encaixe na grade %s (passo %s m)." % ["ligado" if on else "desligado", str(step)])


func set_status(text: String) -> void:
	status_text = text
	if ui != null:
		ui.set_status_bar({"text": text})
	elif _status_fallback != null:
		_status_fallback.text = text
	if mode == "simular":
		_sim_status = text
		_sync_sim()


func on_tool_changed() -> void:
	_sync_bar()
	_sync_edit()
	_show_properties()


## The node or edge chosen with the Rede tool changed.
func on_network_selection() -> void:
	_show_properties()


## The aisle or vertex chosen with the Corredor tool changed.
func on_aisle_selection() -> void:
	_show_properties()


func on_selection_changed() -> void:
	_show_properties()
	_sync_edit()


func on_dragging() -> void:
	pass


# ---------------------------------------------------------------------------
# Top bar (egui, in Rust)
# ---------------------------------------------------------------------------


## Sends what the top bar shows to `UtsimUi` (it redraws only when something changed).
func _sync_bar() -> void:
	if ui == null or doc == null:
		return
	var dirty := " *" if doc.is_dirty() else ""
	var file: String = doc.path() if doc.path() != "" else "(sem arquivo)"
	ui.set_bar({
		"editing": mode == "editar",
		"tool": editor.tool if editor != null else "selecionar",
		"snap": snap_on,
		"step": grid_step,
		"can_undo": doc.can_undo(),
		"can_redo": doc.can_redo(),
		"title": "%s%s" % [doc.name(), dirty],
		"title_tip": "%s%s\n%s" % [doc.name(), dirty, file],
		"links_hidden": PackedStringArray(links_hidden),
	})


## The clicks of the top bar (`UtsimUi.action`): the same as the Godot buttons did.
func bar_action(what: String) -> void:
	if doc == null:
		return
	if what.contains("\t"):
		_panel_action(what.split("\t"))
		return
	var arg := what.substr(what.find(":") + 1) if what.contains(":") else ""
	match what.get_slice(":", 0):
		"modo":
			set_mode(arg)
		"novo":
			new_document()
		"abrir":
			_confirm_discard(func(): _open_dialog.popup_centered_ratio(0.7))
		"salvar":
			save()
		"salvar_como":
			save_as_dialog()
		"desfazer":
			undo()
		"refazer":
			redo()
		"ligacoes":
			var kind := arg.get_slice(":", 0)
			links_hidden.erase(kind)
			if arg.get_slice(":", 1) == "desligar":
				links_hidden.append(kind)
		"ferramenta":
			if arg == "selecionar" or arg == "ligar":
				editor.set_tool(arg)
			else:
				editor.set_tool("selecionar" if editor.tool == arg else arg)
		"organizar":
			arrange_action(what.substr("organizar:".length()))
		"grade":
			set_snap(arg == "ligar", grid_step)
		"passo":
			set_snap(snap_on, float(arg))
		"analisar":
			analysis_action(arg)
		"resultados":
			results_action(what.substr("resultados:".length()))
		"pert":
			pert_action(what.substr("pert:".length()))
		"form":
			_form_action(what.substr("form:".length()))
		"fluxos":
			ui.flows_time(t)
		"dialogo":
			if arg == "descartar" and _pending.is_valid():
				var go := _pending
				_pending = Callable()
				go.call()
			else:
				_pending = Callable()
		"atalhos":
			toggle_shortcuts()
		# The left panel of Editar.
		"biblioteca":
			if editor.tool == "colocar" and editor.place_id == arg:
				editor.set_tool("selecionar")
			else:
				editor.set_tool("colocar", arg)
		"biblioteca_arrastar":
			# The drag started in the egui library; the editor follows the mouse and the
			# drop is resolved over the 3D view (the free central area).
			editor.start_palette_drag(arg)
		"objeto":
			editor.select_only(arg)
		"objeto_focar":
			focus_block(arg, 8.0)
		"obstaculos":
			editor.corredores.set_obstacles_visible(arg == "ligar")
			_sync_edit()
		# The left panel of Simular.
		"replicacoes":
			_run_reps = clampi(int(arg), 1, 1000)
			_sync_sim()
		"rodar_de_novo":
			simulate(_run_reps)
		"bloco":
			var k := int(arg)
			if k >= 0 and k < cena.blocks.size():
				follow_name = cena.blocks[k]["name"]
				if camera_mode == "orbita":
					focus_block(follow_name, 8.0)
				_sync_sim()
		"camera":
			set_camera_mode(arg)
		"gravar_replay":
			_replay_dialog.popup_centered_ratio(0.7)
		"rotulos":
			labels_on = arg == "ligar"
			cena.set_labels_visible(labels_on)
			_sync_sim()
		"seguranca":
			seguranca_on = arg == "ligar"
			_aplicar_camadas()
			_sync_sim()
		"calor":
			calor_on = arg == "ligar"
			_aplicar_camadas()
			_sync_sim()
		"quase":
			# Salto da reprodução até o instante de um quase-conflito.
			set_playing(false)
			seek(maxf(0.0, float(arg) - 2.0), true)
		"sombras":
			sun.shadow_enabled = arg == "ligar"
			_sync_sim()
		# The status bar and the playback bar.
		"cancelar":
			cancel_job()
		"evento":
			step_event(int(arg))
		"reproduzir":
			set_playing(not playing)
		"velocidade":
			speed = float(arg)
			_sync_play()
		"tempo":
			seek(float(arg), true)
	_sync_bar()


## What the left panel of Editar shows besides the library and the objects (which the
## bridge reads from the document): the entry armed, the selection, the obstacles.
func _sync_edit() -> void:
	if ui == null or editor == null:
		return
	ui.set_edit_state({
		"armed": editor.place_id if editor.tool == "colocar" else "",
		"selected": PackedStringArray(editor.selected),
		"obstacles": editor.corredores.show_obstacles,
	})


## What the left panel of Simular shows.
func _sync_sim() -> void:
	if ui == null:
		return
	ui.set_sim_state({
		"reps": _run_reps,
		"status": _sim_status,
		"blocks": _block_labels,
		"follow": cena.block_index(follow_name) if follow_name != "" else -1,
		"camera": camera_mode,
		"labels": labels_on,
		"seguranca": seguranca_on,
		"calor": calor_on,
		"quase": cena.camada.lista_quase() if cena.camada != null else [],
		"shadows": sun.shadow_enabled,
		"summary": _summary,
	})


## What the playback bar shows (only in Simular).
func _sync_play() -> void:
	if ui == null:
		return
	ui.set_playback_bar({
		"visible": mode == "simular",
		"playing": playing,
		"t": t,
		"end": playback.end() if playback != null else 0.0,
		"speed": speed,
	})


## What the egui properties panel did (the edits are already applied by the bridge).
func _panel_action(a: PackedStringArray) -> void:
	match a[0]:
		"prop_ok":
			var path := a[2]
			if path == "name" and not editor.selected.is_empty():
				editor.selected[0] = a[3].strip_edges()
			after_edit.call_deferred()
			set_status("%s: %s alterado." % [a[1], path])
		"prop_erro":
			set_status("Não alterou %s: %s" % [a[1], a[2] if a.size() > 2 else ""])
			_show_properties.call_deferred()
		"prop_fim":
			after_edit.call_deferred()
		"problema":
			editor.corredores.highlight_issue(a[1])
		"saida_ordem":
			if not editor.selected.is_empty() and doc.move_connection(editor.selected[0], int(a[1]), int(a[2])):
				after_edit.call_deferred()
		"saida_remover":
			if not editor.selected.is_empty() and doc.disconnect_objects(editor.selected[0], a[1]):
				after_edit.call_deferred()
		"motorista":
			cena.set_driver_visible(a[1], a[2] == "true")
			_show_properties()
		"dados":
			# a: dados, what, path, draft (JSON)
			var draft = JSON.parse_string(a[3]) if a.size() > 3 else null
			var d: Dictionary = draft if draft is Dictionary else {}
			match a[1]:
				"aplicar":
					_on_data_apply(a[2], d)
				"analisar":
					_on_data_analyze(a[2], d)
				"para_dist":
					_on_data_to_dist(a[2])
				"relatorio":
					_report_target = "modelo"
					_report_dialog.popup_centered_ratio(0.6)
				"escolher_arquivo":
					_data_pick_path = a[2]
					_data_target = "painel"
					_data_dialog.popup_centered_ratio(0.7)
				"janela_csv":
					_data_target = "janela"
					_data_dialog.popup_centered_ratio(0.7)
				"janela_relatorio":
					_report_target = "janela"
					_report_dialog.popup_centered_ratio(0.6)
		"miniatura":
			show_miniature(a[1])


# ---------------------------------------------------------------------------
# Document
# ---------------------------------------------------------------------------


func _confirm_discard(then: Callable) -> void:
	if doc != null and doc.is_dirty():
		_pending = then
		if ui != null:
			ui.show_dialog(JSON.stringify({"name": "Dialogo_Descartar", "title": "Descartar mudanças?",
				"text": "O modelo tem mudanças que não foram salvas. Descartar?",
				"buttons": [["Dialogo_Descartar_Ok", "Descartar", "descartar"], ["Dialogo_Descartar_Voltar", "Voltar", "voltar"]]}))
	else:
		then.call()


func new_document(force := false) -> void:
	var go := func():
		doc.new_model("novo_modelo")
		model_path = ""
		editor.set_tool("selecionar")
		editor.clear_selection()
		set_mode("editar")
		after_edit(true)
		camera.frame_box(Vector3(-8, 0, -8), Vector3(8, 0, 8))
		set_status("Novo modelo.")
	if force:
		go.call()
	else:
		_confirm_discard(go)


## Opens a model (it may be incomplete: its problems are listed). True when opened.
func open_model(path: String) -> bool:
	if doc == null:
		return false
	if path.get_extension().to_lower() == "json":
		return open_replay(path)
	set_playing(false)
	playing_replay = false
	var r: Dictionary = doc.open(path)
	if not r["ok"]:
		set_status("Não abriu %s: %s" % [path.get_file(), r["error"]])
		return false
	model_path = path
	editor.set_tool("selecionar")
	editor.clear_selection()
	if mode != "editar":
		set_mode("editar")
	after_edit(true)
	_frame_model()
	var issues: Array = doc.issues()
	if issues.is_empty():
		set_status("%s aberto: %d objetos. Simular roda as replicações." % [path.get_file(), doc.objects().size()])
	else:
		set_status("%s aberto com %d problema(s): veja o painel de propriedades." % [path.get_file(), issues.size()])
	return true


func save() -> bool:
	if doc.path() == "":
		save_as_dialog()
		return false
	var r: Dictionary = doc.save()
	_report_save(r)
	return r["ok"]


func save_as_dialog() -> void:
	if doc.path() != "":
		_save_dialog.current_path = doc.path()
	else:
		_save_dialog.current_file = doc.name() + ".ron"
	_save_dialog.popup_centered_ratio(0.7)


func save_as(path: String) -> bool:
	if not path.ends_with(".ron"):
		path += ".ron"
	var r: Dictionary = doc.save_as(path)
	_report_save(r)
	return r["ok"]


func _report_save(r: Dictionary) -> void:
	if r["ok"]:
		model_path = doc.path()
		set_status("Salvo em %s." % doc.path())
	else:
		set_status("Não salvou: %s" % r["error"])
	_update_title()


func undo() -> void:
	if doc.undo():
		after_edit()
		set_status("Desfeito.")
	else:
		set_status("Nada para desfazer.")


func redo() -> void:
	if doc.redo():
		after_edit()
		set_status("Refeito.")
	else:
		set_status("Nada para refazer.")


## Refreshes everything drawn from the document after a change (scene, overlay, lists,
## properties, title). Cheap when the revision did not change, unless `force`.
func after_edit(force := false) -> void:
	if doc == null:
		return
	var rev: int = doc.revision()
	if rev == _revision and not force:
		_show_properties()
		return
	_revision = rev
	var objects: Array = doc.objects()
	editor.sync(objects, doc.connections())
	editor.rede.sync(doc.network())
	editor.corredores.sync()
	if mode == "editar":
		_build_edit_scene()
	_sync_edit()
	_show_properties()
	_update_title()


func _build_edit_scene() -> void:
	var pb: Object = doc.edit_scene(CenaUtsim.skeletal_available())
	cena.show_connections = false
	if pb == null:
		set_status("Não foi possível desenhar o modelo: %s" % doc.last_error())
		return
	playback = null
	cena.build(pb, true)
	cena.apply(0.0, 0.0, true)
	cena.set_labels_visible(true)
	_draw_edit_grid()


func _update_title() -> void:
	var file: String = doc.path().get_file() if doc.path() != "" else "(sem arquivo)"
	var dirty := " *" if doc.is_dirty() else ""
	if is_inside_tree():
		get_window().title = "UTSim 3D: %s%s (%s)" % [doc.name(), dirty, file]
	_sync_bar()


func _show_properties() -> void:
	if doc == null or mode != "editar" or ui == null:
		return
	if not editor.selected_link.is_empty():
		_show_link(editor.selected_link)
		return
	if editor.tool == "rede":
		_show_network()
		return
	if editor.tool == "corredor":
		_show_aisles()
		return
	if editor.tool == "zona":
		_show_zones()
		return
	ui.show_form("")
	var sel: Array = editor.selected
	var name: String = sel[0] if not sel.is_empty() else "run"
	var others := PackedStringArray()
	for k in range(1, sel.size()):
		others.append(sel[k])
	var extra_line := ""
	if sel.size() > 1:
		extra_line = "%d objetos selecionados: as mudanças valem para todos do mesmo tipo." % sel.size()
	ui.show_properties(name, others, extra_line, _driver_state(name))


## The seated driver of a forklift: -1 none, 0 hidden, 1 shown.
func _driver_state(object: String) -> int:
	if cena == null or not cena.has_driver(object):
		return -1
	return 1 if cena.driver_visible(object) else 0


## The miniature of the asset picker of the egui panel: the object alone in a small 3D
## world of its own, drawn by the same scene code; egui shows the texture.
func _build_miniature() -> void:
	_mini_view = SubViewport.new()
	_mini_view.name = "Miniatura"
	_mini_view.size = Vector2i(396, 150)
	_mini_view.own_world_3d = true
	_mini_view.msaa_3d = Viewport.MSAA_2X
	_mini_view.render_target_update_mode = SubViewport.UPDATE_ALWAYS
	add_child(_mini_view)
	var env := WorldEnvironment.new()
	var e := Environment.new()
	e.background_mode = Environment.BG_COLOR
	e.background_color = Color(0.16, 0.17, 0.21)
	e.ambient_light_source = Environment.AMBIENT_SOURCE_COLOR
	e.ambient_light_color = Color(0.80, 0.84, 0.92)
	e.ambient_light_energy = 0.8
	e.tonemap_mode = Environment.TONE_MAPPER_AGX
	env.environment = e
	_mini_view.add_child(env)
	var light := DirectionalLight3D.new()
	light.light_energy = 1.4
	light.transform = Transform3D(Basis.IDENTITY, Vector3(4, 8, 5)).looking_at(Vector3.ZERO, Vector3.UP)
	_mini_view.add_child(light)
	_mini_camera = Camera3D.new()
	_mini_camera.current = true
	_mini_view.add_child(_mini_camera)
	_mini_scene = CenaUtsim.new()
	_mini_scene.show_floor = false
	_mini_scene.show_connections = false
	_mini_view.add_child(_mini_scene)
	ui.set_miniature(_mini_view)


## Draws the selected object with `asset` in the miniature.
func show_miniature(asset: String) -> void:
	mini_asset = asset
	if _mini_scene == null or editor == null or editor.selected.is_empty():
		if _mini_scene != null:
			_mini_scene.clear()
		return
	var pb: Object = doc.look_scene(editor.selected[0], asset, CenaUtsim.skeletal_available())
	if pb == null:
		_mini_scene.clear()
		return
	_mini_scene.build(pb)
	_mini_scene.apply(0.0, 0.0, true)
	_mini_scene.set_labels_visible(false)
	var lo: Vector3 = _mini_scene.info.get("bounds_min", Vector3(-1, 0, -1))
	var hi: Vector3 = _mini_scene.info.get("bounds_max", Vector3(1, 1, 1))
	var size := maxf((hi - lo).length(), 1.0)
	var center := (lo + hi) * 0.5
	var eye := center + Vector3(0.9, 0.7, 1.1).normalized() * size * 1.1
	_mini_camera.transform = Transform3D(Basis.IDENTITY, eye).looking_at(center, Vector3.UP)


# ---------------------------------------------------------------------------
# Real data in the distribution fields (G3)
# ---------------------------------------------------------------------------


## The bridge's config from the form of the panel, or `{}` (the reason goes to the status).
func _data_config(draft: Dictionary) -> Dictionary:
	var config := {
		"timestamps": bool(draft.get("timestamps", false)),
		"choose": String(draft.get("choose", "Best")),
		"criterion": String(draft.get("criterion", "aic")),
	}
	if draft.get("origem", "arquivo") == "arquivo":
		var file := String(draft.get("source", "")).strip_edges()
		if file == "":
			set_status("Escolha o arquivo de dados.")
			return {}
		config["source"] = doc.data_relative(file)
		config["column"] = String(draft.get("column", ""))
	else:
		var r: Dictionary = doc.data_parse_values(String(draft.get("text", "")))
		if not r["ok"]:
			set_status("Valores colados: %s" % r["error"])
			return {}
		config["values"] = r["values"]
	return config


func _selected_field(path: String) -> String:
	var sel: Array = editor.selected
	return "%s.%s" % [sel[0], path] if not sel.is_empty() else ""


func _on_data_apply(path: String, draft: Dictionary) -> void:
	var field := _selected_field(path)
	var config := _data_config(draft)
	if field == "" or config.is_empty():
		return
	if busy():
		set_status("Aguarde: %s." % job.what())
		return
	var j: Object = doc.data_fit(field, config)
	if j == null:
		set_status("Não ajustou: %s" % doc.last_error())
		return
	start_job(j, func(_j):
		if doc.data_apply(field, config):
			doc.end_gesture()
			after_edit.call_deferred()
			set_status("%s agora usa dados reais (Ctrl+Z desfaz)." % field)
		else:
			set_status("Não aplicou os dados: %s" % doc.last_error()))


## The analysis of an applied entry that has none yet (the panel asks once).
func _on_data_analyze(path: String, draft: Dictionary) -> void:
	var field := _selected_field(path)
	var config := _data_config(draft)
	if field == "" or config.is_empty() or busy():
		return
	var j: Object = doc.data_fit(field, config)
	if j == null:
		return
	start_job(j, func(_j):
		after_edit.call_deferred())


func _on_data_to_dist(path: String) -> void:
	var field := _selected_field(path)
	if field == "":
		return
	if doc.data_to_dist(field):
		doc.end_gesture()
		after_edit.call_deferred()
		set_status("%s volta a usar uma distribuição (Ctrl+Z desfaz)." % field)
	else:
		set_status("Não voltou para a distribuição: %s" % doc.last_error())
		_show_properties.call_deferred()


func _on_data_file(file: String) -> void:
	if ui != null and _data_target == "janela":
		var err := String(ui.fitwin_csv(file))
		set_status("Valores lidos de %s." % file.get_file() if err == "" else "Não leu %s: %s" % [file.get_file(), err])
		return
	if ui != null:
		ui.set_data_file(_data_pick_path, doc.data_relative(file))


## Writes the report of the "Valores reais…" window (all the measures and the choice)
## into `dir` and opens it in the system browser.
func write_fit_report(dir: String) -> void:
	var r: Dictionary = ui.fitwin_report(dir)
	if not r["ok"]:
		set_status("Sem relatório: %s" % r["error"])
		return
	last_report = {"index": r["file"], "files": 1}
	set_status("Relatório gravado em %s." % r["file"])
	if DisplayServer.get_name() != "headless" and not scripted:
		OS.shell_open(String(r["file"]))


## Writes the HTML reports of the fields with data into `dir` (on the bridge's thread)
## and opens `index.html` in the system browser.
func write_data_report(dir: String) -> void:
	var j: Object = doc.data_report(dir)
	if j == null:
		set_status("Sem relatório: %s" % doc.last_error())
		return
	start_job(j, func(done):
		last_report = done.data()
		var index := String(last_report.get("index", ""))
		set_status("Relatório gravado em %s (%d arquivos)." % [dir, int(last_report.get("files", 0))])
		# A scripted run checks the files; it does not open the browser.
		if index != "" and DisplayServer.get_name() != "headless" and not scripted:
			OS.shell_open(index))


## The actions of the "Organizar" menu and of the keyboard shortcuts.
func arrange_action(what: String) -> void:
	var sel := editor.selection()
	if what.begins_with("alinhar:"):
		if doc.align(sel, what.substr(8)):
			after_edit()
			set_status("%d objetos alinhados." % sel.size())
		else:
			set_status(doc.last_error())
		return
	if what.begins_with("distribuir:"):
		if doc.distribute(sel, what.substr(11)):
			after_edit()
			set_status("%d objetos distribuídos." % sel.size())
		else:
			set_status(doc.last_error())
		return
	match what:
		"copiar":
			var n: int = doc.copy(sel)
			set_status("%d objeto(s) copiados." % n if n > 0 else "Nada selecionado para copiar.")
		"recortar":
			var n: int = doc.cut(sel)
			if n > 0:
				editor.clear_selection()
				after_edit()
				set_status("%d objeto(s) recortados (Ctrl+V cola)." % n)
			else:
				set_status("Nada selecionado para recortar.")
		"colar", "duplicar":
			var names: PackedStringArray = doc.paste() if what == "colar" else doc.duplicate(sel)
			if names.is_empty():
				set_status(doc.last_error())
			else:
				editor.select_names(Array(names), false)
				after_edit()
				set_status("%d objeto(s) colados: %s." % [names.size(), ", ".join(names)])
		"excluir":
			editor.delete_selected()
		"tudo":
			editor.select_all()


func _frame_model() -> void:
	var lo := Vector3(INF, 0, INF)
	var hi := Vector3(-INF, 0, -INF)
	var any := false
	for o in doc.objects():
		var p: Vector3 = o["position"]
		lo = Vector3(minf(lo.x, p.x), 0, minf(lo.z, p.z))
		hi = Vector3(maxf(hi.x, p.x), 0, maxf(hi.z, p.z))
		any = true
	# What the scene draws beyond the objects (the fluid network, the paths).
	if cena.info.has("bounds_min"):
		var blo: Vector3 = cena.info["bounds_min"]
		var bhi: Vector3 = cena.info["bounds_max"]
		lo = Vector3(minf(lo.x, blo.x), 0, minf(lo.z, blo.z))
		hi = Vector3(maxf(hi.x, bhi.x), 0, maxf(hi.z, bhi.z))
		any = true
	if not any:
		lo = Vector3(-8, 0, -8)
		hi = Vector3(8, 0, 8)
	camera.frame_box(lo - Vector3(2, 0, 2), hi + Vector3(2, 0, 2))
	_update_shadows(lo, hi)


# ---------------------------------------------------------------------------
# Modes
# ---------------------------------------------------------------------------


## `editar` or `simular`. Simular needs a model without problems; it runs the
## replications and plays replication 0. True when the mode changed.
func set_mode(which: String) -> bool:
	if doc == null:
		return false
	if which == "simular":
		return simulate(_run_reps)
	set_playing(false)
	mode = "editar"
	playing_replay = false
	_set_right(true)
	_sync_play()
	_sync_panels()
	_sync_bar()
	_revision = -1
	after_edit(true)
	return true


## Runs `reps` replications of the document (in Rust, in parallel), shows the summary and
## starts the playback of replication 0. False (and stays in Editar) when the model has
## problems.
func simulate(reps: int) -> bool:
	if doc == null:
		return false
	set_playing(false)
	_run_reps = reps
	model = doc.as_model()
	if not model.is_loaded():
		var issues: Array = doc.issues()
		var first: String = issues[0]["message"] if not issues.is_empty() else ""
		set_status("O modelo tem %d problema(s) e não roda: %s" % [issues.size(), first])
		_sync_bar()
		return false
	mode = "simular"
	_set_right(false)
	_sync_play()
	_sync_panels()
	_sync_bar()
	playing_replay = false
	if job != null:
		set_status("Aguarde: %s." % job.what())
		return false
	var j: Object = model.start_run(reps)
	if j == null:
		set_status("A simulação não começou: %s" % model.last_error())
		return false
	_sim_status = "Simulando %d replicações..." % reps
	_sync_sim()
	cena.clear(true)
	playback = null
	start_job(j, func(done): _on_run(done))
	return true


func _on_run(j: Object) -> void:
	if not j.ok():
		set_mode("editar")
		set_status("A simulação %s: %s" % ["foi cancelada" if j.cancelled() else "falhou", j.error()])
		return
	on_results(j.results())
	playback = model.playback(0, CenaUtsim.skeletal_available())
	if playback == null:
		set_status("Não foi possível gravar a replicação 0: %s" % model.last_error())
		return
	_show_playback()
	set_status("%d replicações concluídas. Reproduzindo a replicação 0." % results.replications())


## Liga ou desliga as camadas de segurança e de calor da cena.
func _aplicar_camadas() -> void:
	if cena.camada != null:
		cena.camada.ligar(seguranca_on)
		cena.camada.ligar_calor(calor_on)


## Builds the scene of `playback` and starts at 0.
func _show_playback() -> void:
	cena.show_connections = true
	cena.build(playback, true)
	cena.camada = CamadaSeguranca.criar(cena, model) if model != null and model.is_loaded() else null
	_aplicar_camadas()
	var info := cena.info
	_update_shadows(info["bounds_min"], info["bounds_max"])
	_draw_grid(info)
	_block_labels = PackedStringArray()
	for b in cena.blocks:
		# A model of a specialised line names the kinds with the line's words.
		var term: String = doc.kind_term(b["kind"])
		_block_labels.append("%s (%s)" % [b["name"], term if term != "" else b["kind"]])
	if ui != null:
		ui.set_flows_playback(playback)
	if follow_name == "" or cena.block_index(follow_name) < 0:
		follow_name = ""
		for b in cena.blocks:
			if b["mobile"]:
				follow_name = b["name"]
				break
	_sync_sim()
	seek(0.0, true)


## A new run to show (`UtsimResults`): the summary on the left and the Resultados panel.
func on_results(res: Object) -> void:
	if res == null:
		return
	results = res
	_show_summary()
	if ui != null:
		var base := model_path.get_basename() if model_path != "" else OS.get_user_data_dir().path_join("resultados")
		ui.set_results(res, not playing_replay, base + "_resultados")


func _show_summary() -> void:
	var s: Dictionary = results.summary()
	var lines := PackedStringArray()
	for r in results.table():
		lines.append("%s: %s saídas" % [r["name"], _mh(r["output"])])
	var b = s.get("bottleneck")
	_summary = {
		"title": "Resultados (%d replicações)" % s["replications"],
		"bottleneck": str(b) if b != null else "",
		"lines": lines,
	}
	_sync_sim()


## `mean ± half_width` with about four significant digits.
static func _mh(s: Variant) -> String:
	if s == null or not (s is Dictionary) or s.get("mean") == null:
		return "-"
	var h = s.get("half_width")
	if h == null:
		return _num(float(s["mean"]))
	return "%s ± %s" % [_num(float(s["mean"])), _num(float(h))]


## A number with about four significant digits.
static func _num(x: float) -> String:
	if x == 0.0 or is_nan(x):
		return str(x)
	var digits := clampi(3 - int(floor(log(absf(x)) / log(10.0))), 0, 6)
	return String.num(x, digits)


## Shadows sized by the scene, as the Bevy viewer does: up to the camera distance plus
## half the diagonal.
func _update_shadows(lo: Vector3, hi: Vector3) -> void:
	sun.directional_shadow_max_distance = camera.dist + (hi - lo).length() * 0.5 + 10.0


## The floor grid of Editar: lines every snapping step around the model (at least 40 m
## across), a little darker than the floor.
func _draw_edit_grid() -> void:
	if mode != "editar":
		return
	var lo := Vector3(-20, 0, -20)
	var hi := Vector3(20, 0, 20)
	for o in doc.objects():
		var p: Vector3 = o["position"]
		lo = Vector3(minf(lo.x, p.x - 10), 0, minf(lo.z, p.z - 10))
		hi = Vector3(maxf(hi.x, p.x + 10), 0, maxf(hi.z, p.z + 10))
	var step := grid_step
	while (hi.x - lo.x) / step > 400 or (hi.z - lo.z) / step > 400:
		step *= 2.0
	_grid_lines(lo, hi, step)


## The floor grid of the Bevy viewer (`draw_world`) for Simular: about 40 cells across
## the scene, the spacing rounded to 1, 2 or 5 x 10^k metres.
func _draw_grid(info: Dictionary) -> void:
	var lo: Vector3 = info["bounds_min"]
	var hi: Vector3 = info["bounds_max"]
	var span := maxf(maxf(hi.x - lo.x, hi.z - lo.z), 10.0) + 16.0
	var raw := span / 40.0
	var mag := pow(10.0, floor(log(raw) / log(10.0)))
	var step := raw
	for m in [1.0, 2.0, 5.0, 10.0]:
		if m * mag >= raw:
			step = m * mag
			break
	var c := (lo + hi) * 0.5
	_grid_lines(c - Vector3(span, 0, span) * 0.5, c + Vector3(span, 0, span) * 0.5, step)


func _grid_lines(lo: Vector3, hi: Vector3, step: float) -> void:
	if _grid != null:
		_grid.queue_free()
	var x0 := floorf(lo.x / step) * step
	var x1 := ceilf(hi.x / step) * step
	var z0 := floorf(lo.z / step) * step
	var z1 := ceilf(hi.z / step) * step
	var c := Color(0.22, 0.24, 0.28)
	var im := ImmediateMesh.new()
	im.surface_begin(Mesh.PRIMITIVE_LINES)
	var x := x0
	while x <= x1 + 1e-6:
		for v in [Vector3(x, 0.002, z0), Vector3(x, 0.002, z1)]:
			im.surface_set_color(c)
			im.surface_add_vertex(v)
		x += step
	var z := z0
	while z <= z1 + 1e-6:
		for v in [Vector3(x0, 0.002, z), Vector3(x1, 0.002, z)]:
			im.surface_set_color(c)
			im.surface_add_vertex(v)
		z += step
	im.surface_end()
	_grid = MeshInstance3D.new()
	_grid.name = "Grade"
	_grid.mesh = im
	_grid.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	var m := StandardMaterial3D.new()
	m.shading_mode = BaseMaterial3D.SHADING_MODE_UNSHADED
	m.vertex_color_use_as_albedo = true
	m.vertex_color_is_srgb = true
	_grid.material_override = m
	add_child(_grid)


# ---------------------------------------------------------------------------
# Playback (Simular)
# ---------------------------------------------------------------------------


func set_playing(on: bool) -> void:
	playing = on and playback != null and mode == "simular"
	_sync_play()


## Jumps to `time` (no crossfade when `snap`).
func seek(time: float, snap := false) -> void:
	if playback == null:
		return
	t = clampf(time, 0.0, playback.end())
	cena.apply(t, 0.0, snap)
	_show_time()
	if ui != null:
		ui.flows_time(t)


func step_event(direction: int) -> void:
	if playback == null:
		return
	set_playing(false)
	var e: float = playback.next_event(t) if direction > 0 else playback.prev_event(t)
	if e >= 0.0:
		seek(e, true)


func _show_time() -> void:
	_sync_play()


func _process(dt: float) -> void:
	_poll_job()
	if playback != null and playing:
		t += dt * speed
		if t >= playback.end():
			t = playback.end()
			set_playing(false)
		cena.apply(t, dt)
		_show_time()
		if ui != null:
			ui.flows_time(t)
	_frames += 1
	if mode == "simular" and playback != null:
		_drive_camera()
		if _frames % 10 == 0:
			cena.limit_labels(camera)
	if _perf_on and ui != null and _frames % 15 == 0:
		ui.set_performance("%d qps | ponte %.2f ms | cena %.2f ms" % [
			Engine.get_frames_per_second(), cena.last_bridge_us / 1000.0, cena.last_apply_us / 1000.0])


func _unhandled_key_input(event: InputEvent) -> void:
	var k := event as InputEventKey
	if k == null or not k.pressed or doc == null:
		return
	var ctrl := k.ctrl_pressed or k.meta_pressed
	var handled := true
	if ctrl:
		match k.keycode:
			KEY_Z:
				if mode == "editar" and k.shift_pressed:
					redo()
				elif mode == "editar":
					undo()
			KEY_Y:
				if mode == "editar":
					redo()
			KEY_S:
				if k.shift_pressed:
					save_as_dialog()
				else:
					save()
			KEY_O:
				_confirm_discard(func(): _open_dialog.popup_centered_ratio(0.7))
			KEY_N:
				new_document()
			KEY_C:
				arrange_action("copiar")
			KEY_X:
				arrange_action("recortar")
			KEY_V:
				arrange_action("colar")
			KEY_D:
				arrange_action("duplicar")
			KEY_A:
				editor.select_all()
			_:
				handled = false
	else:
		match k.keycode:
			KEY_F1:
				toggle_shortcuts()
			KEY_F3:
				toggle_performance()
			KEY_TAB:
				set_mode("editar" if mode == "simular" else "simular")
			KEY_F:
				if mode == "simular" and cena.playback != null:
					camera.frame_box(cena.info["bounds_min"], cena.info["bounds_max"])
				else:
					_frame_model()
			KEY_SPACE:
				if mode == "simular":
					set_playing(not playing)
			_:
				handled = mode == "editar" and _edit_key(k)
	if handled:
		get_viewport().set_input_as_handled()


## The keys of Editar alone: Del, Esc, arrows, L and G.
func _edit_key(k: InputEventKey) -> bool:
	match k.keycode:
		KEY_DELETE, KEY_BACKSPACE:
			editor.delete_selected()
		KEY_ESCAPE:
			editor.cancel()
		KEY_LEFT:
			editor.nudge(-1, 0)
		KEY_RIGHT:
			editor.nudge(1, 0)
		KEY_UP:
			editor.nudge(0, 1)
		KEY_DOWN:
			editor.nudge(0, -1)
		KEY_L:
			editor.set_tool("selecionar" if editor.tool == "ligar" else "ligar")
		KEY_N:
			editor.set_tool("selecionar" if editor.tool == "rede" else "rede")
		KEY_C:
			editor.set_tool("selecionar" if editor.tool == "corredor" else "corredor")
		KEY_E:
			editor.set_tool("selecionar" if editor.tool == "equipe" else "equipe")
		KEY_Z:
			if k.ctrl_pressed or k.meta_pressed:
				return false
			editor.set_tool("selecionar" if editor.tool == "zona" else "zona")
		KEY_ENTER, KEY_KP_ENTER:
			if editor.tool == "zona":
				editor.zone_finish()
			elif editor.tool != "corredor" or not editor.corredores.finish():
				return false
		KEY_G:
			set_snap(not snap_on, grid_step)
		_:
			return false
	return true


## Aims the orbit camera at block `name` (the viewer's `Focar` step).
func focus_block(name: String, distance: float, yaw: Variant = null, pitch: Variant = null) -> bool:
	var i := cena.block_index(name)
	if i < 0:
		return false
	camera.focus(cena.block_focus(i), distance, yaw, pitch)
	return true



# ---------------------------------------------------------------------------
# Jobs on a thread of the bridge (G3)
# ---------------------------------------------------------------------------


## Whether a job is running (scripts wait for it).
func busy() -> bool:
	return job != null


## Runs `j` (a `UtsimJob`) and calls `done(j)` when it ends, keeping the interface
## drawing; the progress bar and Cancelar show meanwhile. One job at a time.
func start_job(j: Object, done: Callable) -> void:
	if j == null:
		return
	if job != null:
		set_status("Aguarde: %s." % job.what())
		return
	job = j
	_job_done = done
	if ui != null:
		ui.set_status_bar({"progress": 0.0, "percent": true})
	set_status("%s..." % j.what())


func cancel_job() -> void:
	if job != null:
		job.cancel()
		set_status("Cancelando: %s..." % job.what())


func _poll_job() -> void:
	if job == null:
		return
	var p: Dictionary = job.progress()
	var total := int(p["total"])
	if ui != null:
		if total > 0:
			ui.set_status_bar({"progress": float(p["done"]) / total, "percent": true})
		else:
			# Unknown length: a bar that goes back and forth.
			ui.set_status_bar({"progress": absf(sin(Time.get_ticks_msec() / 400.0)), "percent": false})
	if not p["finished"]:
		return
	var j := job
	var done := _job_done
	job = null
	_job_done = Callable()
	if ui != null:
		ui.set_status_bar({"progress": -1.0})
	if not j.ok():
		if j.cancelled():
			set_status("%s: cancelado." % j.what())
		else:
			set_status("%s: erro: %s" % [j.what(), j.error()])
		if j.kind() != "replicacoes":
			return
	if done.is_valid():
		done.call(j)


# ---------------------------------------------------------------------------
# Analyses (G3)
# ---------------------------------------------------------------------------


## The document as a model that can run, or null when it has problems.
func document_model() -> Object:
	if doc == null:
		return null
	var m: Object = doc.as_model()
	return m if m.is_loaded() else null


## The actions of the "Analisar" menu and buttons.
func analysis_action(what: String) -> void:
	match what:
		"resultados":
			if results == null:
				set_status("Simule o modelo para ver os resultados (Tab ou o botão Simular).")
				return
			show_results(not ui.results_open())
		"fluxos":
			if mode != "simular" or playback == null:
				set_status("Os fluxos de tokens aparecem no Simular.")
				return
			ui.set_flows_open(not ui.flows_open())
			ui.flows_time(t)
		"pert", "pert_trecho", "pert_auto":
			open_pert(what == "pert_trecho" or (what == "pert_auto" and editor.selected.size() == 2))


## Opens the PERT/CPM window for the selection (`between`: from the first object
## selected to the second; the primary selection is the last one clicked).
func open_pert(between: bool) -> void:
	var sel: Array = editor.selected
	if sel.is_empty():
		set_status("Selecione os objetos da parte do processo (ou dois: o início e o fim) e peça o PERT/CPM.")
		return
	var names := PackedStringArray()
	if between:
		if sel.size() != 2:
			set_status("PERT/CPM de... até...: selecione exatamente dois objetos.")
			return
		names = PackedStringArray([sel[1], sel[0]])
	else:
		names = PackedStringArray(sel)
	if document_model() == null:
		set_status("O modelo tem problemas e não roda: o PERT/CPM precisa de um modelo válido.")
		return
	show_results(false)
	_pert_names = names
	_pert_between = between
	var what := ("de %s até %s" % [names[0], names[1]]) if between and names.size() >= 2 else ", ".join(names)
	ui.open_pert("PERT/CPM: %s" % what)
	_sync_panels()
	pert_calculate("simulado")


## Calculates the PERT/CPM of the selection in `mode` (`simulado`, `classico`,
## `montecarlo`) with the deadline and samples of the window, on the bridge's thread.
func pert_calculate(mode_id: String) -> void:
	var m: Object = document_model()
	if m == null:
		ui.set_pert_info("O modelo tem problemas e não roda: corrija no Editar.", true)
		return
	var f: Dictionary = ui.pert_form()
	var j: Object = m.start_pert(_pert_names, _pert_between, mode_id, float(f["deadline"]), int(f["samples"]))
	if j == null:
		ui.set_pert_info(m.last_error(), true)
		return
	if job != null:
		set_status("Aguarde: %s." % job.what())
		return
	ui.set_pert_info("Calculando (%s)..." % mode_id, false)
	start_job(j, func(done):
		var p: Object = done.pert()
		if p == null:
			ui.set_pert_info("Não calculou: %s" % done.error(), true)
			return
		var base := model_path.get_basename() if model_path != "" else OS.get_user_data_dir().path_join("pert")
		ui.set_pert(p, base + "_pert.html")
		editor.critical = Array(ui.pert_critical())
		set_status(ui.pert_status()))


## The actions of the PERT/CPM window (`pert:<what>`).
func pert_action(what: String) -> void:
	match what.get_slice(":", 0):
		"calcular":
			pert_calculate(what.get_slice(":", 1))
		"fechar":
			editor.critical = []
			_sync_panels()
		"exportar_html":
			set_status(ui.pert_export(true))
		"exportar_csv":
			set_status(ui.pert_export(false))


## Opens or closes the "Resultados" window (egui, over the free area of the view).
func show_results(on: bool) -> void:
	if ui == null:
		return
	ui.set_results_open(on)
	if ui.results_open():
		ui.close_pert()
		editor.critical = []
	_sync_panels()


## The actions of the "Resultados" window (`resultados:<what>`).
func results_action(what: String) -> void:
	var arg := what.substr(what.find(":") + 1) if what.contains(":") else ""
	match what.get_slice(":", 0):
		"fechar":
			_sync_panels()
		"exportar":
			set_status(ui.results_export())
		"ligar_amostras":
			enable_samples_and_rerun()
		"aplicar_aquecimento":
			apply_warmup(float(arg))
		"trabalho":
			if job != null:
				set_status("Aguarde: %s." % job.what())
				return
			var j: Object = ui.results_job(arg)
			if j == null:
				set_status("Não começou: %s" % ui.results_error())
				return
			start_job(j, func(done):
				if done.kind() == "replicacoes":
					on_results(done.results())
				var line: String = ui.results_done(done)
				if line != "":
					set_status(line))


## Writes `warmup` to `run.warmup` of the document ("Aplicar" of the warm-up analysis).
## Switches on the content series and the histograms in the model (one undo step, from
## the Resultados charts) and runs the replications again, so the charts have data.
func enable_samples_and_rerun() -> void:
	if playing_replay:
		set_status("Um replay gravado não roda de novo: abra o modelo.")
		return
	var r: Dictionary = doc.enable_samples()
	if not r["ok"]:
		set_status("Não ligou a série e os histogramas: %s" % doc.last_error())
		return
	after_edit()
	var parts := PackedStringArray()
	if float(r["sample_interval"]) > 0.0:
		parts.append("run.sample_interval = %s" % _num(float(r["sample_interval"])))
	if int(r["histogram_bins"]) > 0:
		parts.append("run.histogram_bins = %d" % int(r["histogram_bins"]))
	simulate(_run_reps)
	set_status("Ligado no modelo (%s; Ctrl+Z desfaz). Simulando de novo..." % ", ".join(parts))


func apply_warmup(warmup: float) -> void:
	if doc.set_property("run", "warmup", warmup, PackedStringArray()):
		doc.end_gesture()
		after_edit()
		set_status("Aquecimento %s aplicado ao modelo (run.warmup). Simule de novo para os números com ele." % _num(warmup))
	else:
		set_status("Não aplicou o aquecimento: %s" % doc.last_error())


# ---------------------------------------------------------------------------
# Recorded replays (G3)
# ---------------------------------------------------------------------------


## Opens a replay recorded by `utsim run --replay` or "Gravar replay": it plays in
## Simular (with the statistics of that replication); the document is not touched.
## `--utsim-abrir=<arquivo>` (the Ensino's "Ver em 3D", or any caller): opens that model
## (`.ron`) or replay (`.json`, `utsim_model::replay::Replay` as JSON) when the scene starts.
func _open_from_args() -> void:
	for a in OS.get_cmdline_user_args():
		var arg := String(a)
		if arg.begins_with("--utsim-abrir="):
			var path := arg.trim_prefix("--utsim-abrir=")
			if not open_model(path):
				set_status("Não abriu %s (pedido na linha de comando)." % path.get_file())
			return


func open_replay(path: String) -> bool:
	set_playing(false)
	var m: Object = ClassDB.instantiate(&"UtsimModel")
	var r: Dictionary = m.load(path)
	if not r["ok"]:
		set_status("Não abriu %s: %s" % [path.get_file(), ", ".join(r["errors"])])
		return false
	model = m
	playback = model.playback(0, CenaUtsim.skeletal_available())
	if playback == null:
		set_status("Não foi possível reproduzir %s: %s" % [path.get_file(), model.last_error()])
		return false
	playing_replay = true
	mode = "simular"
	_set_right(false)
	_sync_play()
	_sync_panels()
	_sync_bar()
	on_results(model.replay_results())
	_show_playback()
	camera.frame_box(cena.info["bounds_min"], cena.info["bounds_max"])
	set_status("Replay %s aberto (%d blocos): só reprodução; Editar volta ao modelo em edição." % [path.get_file(), int(r["objects"])])
	return true


## Writes the replay of replication 0 of the last run as `.json`.
func save_replay(path: String) -> bool:
	if model == null or not model.is_loaded():
		set_status("Simule um modelo antes de gravar o replay.")
		return false
	if not path.ends_with(".json"):
		path += ".json"
	if model.save_replay(0, path):
		set_status("Replay da replicação 0 gravado em %s." % path)
		return true
	set_status("Não gravou o replay: %s" % model.last_error())
	return false


# ---------------------------------------------------------------------------
# Cameras (G3)
# ---------------------------------------------------------------------------


func set_camera_mode(which: String) -> void:
	camera_mode = which
	if which not in ["primeira_pessoa", "recurso"]:
		cena.restore_view()
	camera.set_plan(which == "planta")
	var label := which
	for c in CAMERAS:
		if c[0] == which:
			label = c[1]
	_sync_sim()
	if which == "orbita":
		camera.update()
		set_status("Câmera de órbita.")
	elif which == "planta":
		set_status("Planta (2D): vista de cima, sem perspectiva; arraste com o botão direito para mover.")
	elif follow_name == "":
		set_status("Escolha na lista o bloco que a câmera acompanha.")
	else:
		set_status("Câmera %s: %s." % [label.to_lower(), follow_name])


## Places the camera for the modes that follow a block (called every frame in Simular).
func _drive_camera() -> void:
	if camera_mode in ["orbita", "planta"] or follow_name == "":
		return
	var i := cena.block_index(follow_name)
	if i < 0:
		return
	var root: Node3D = cena.rigs[i]["root"]
	var at := root.global_position
	var forward := root.global_basis.x.normalized()
	forward.y = 0.0
	if forward.length() < 1e-3:
		forward = Vector3.RIGHT
	forward = forward.normalized()
	match camera_mode:
		"seguir":
			camera.target = cena.block_focus(i)
			camera.update()
		"primeira_pessoa":
			var eye := at + Vector3(0, 1.6, 0) + forward * 0.35
			camera_aim = eye + forward * 10.0 - Vector3(0, 1.2, 0)
			camera.transform = Transform3D(Basis.IDENTITY, eye).looking_at(camera_aim, Vector3.UP)
			cena.clear_view(eye, camera_aim, i)
		"recurso":
			# The driving position: above the vehicle (an AGV has no cab; a forklift's
			# seat is about its mast's height), looking ahead and a little down. The
			# parts of the vehicle on the sight line and any block the eye goes through
			# are hidden while the camera is there.
			var eye_y := clampf(float(cena.rigs[i].get("top", 1.0)) + 0.6, 1.4, 2.4)
			var eye := at + Vector3(0, eye_y, 0)
			# A forklift with a driver: the camera is at the driver's head, and the
			# driver is hidden from there.
			var head = cena.driver_eye(i)
			if head != null:
				eye = head
				eye_y = eye.y - at.y
			camera_aim = eye + forward * 10.0 - Vector3(0, eye_y * 0.8, 0)
			camera.transform = Transform3D(Basis.IDENTITY, eye).looking_at(camera_aim, Vector3.UP)
			cena.clear_view(eye, camera_aim, i)


# ---------------------------------------------------------------------------
# The Corredor panel (G4)
# ---------------------------------------------------------------------------


## The right panel of a selected link (built by `UtsimDocument.tool_panel`).
func _show_link(link: Array) -> void:
	_tool_panel("ligacao", {"from": link[0], "to": link[1]})


## A tool's right panel, built in Rust from the document (`tool_panels.rs`).
func _tool_panel(tool: String, state: Dictionary) -> void:
	if ui != null:
		ui.show_form(doc.tool_panel(tool, state))
## Shows `rows` (see `ui_form.rs`) as the right panel; the edits come back in `_form_action`.
func _show_form(rows: Array) -> void:
	if ui != null:
		ui.show_form(JSON.stringify(rows))


## What the tool panels (`form:<action>:<value>`) asked.
func _form_action(what: String) -> void:
	var action := what.get_slice(":", 0)
	var value := what.substr(action.length() + 1)
	var rede: EditorRede = editor.rede
	var ai: EditorCorredores = editor.corredores
	match action:
		"ligacao_remover":
			editor.delete_selected()
		"rede_novos_mao_dupla":
			rede.two_way = value == "1"
		"rede_no_nome":
			rede.rename(value)
		"rede_excluir", "corredor_excluir":
			(rede if action == "rede_excluir" else ai).delete_selected()
		"rede_mao_dupla":
			rede.set_two_way(value == "1")
		"rede_capacidade":
			rede.set_capacity(int(float(value)))
		"rede_inverter":
			rede.reverse()
		"corredor_tipo":
			ai.set_kind(value)
		"corredor_largura":
			ai.set_width(float(value))
		"corredor_mao_dupla":
			ai.set_two_way(value == "1")
		"corredor_secundaria":
			ai.set_secondary(value == "1")
		"corredor_nome":
			ai.rename(value)
		"corredor_perfil":
			ai.set_profile(value)
		"corredor_vertice_remover":
			ai.remove_vertex()
		"corredor_inverter":
			ai.reverse()
		"zona_nome", "zona_tipo", "zona_nivel":
			editor.set_zone_field(action.trim_prefix("zona_").replace("nome", "name").replace("tipo", "kind").replace("nivel", "level"), value)
		"zona_excluir":
			editor.delete_selected()
		_ when action.begins_with("zona_epi_"):
			editor.set_zone_field("ppe", action.trim_prefix("zona_epi_"))
	_show_properties.call_deferred()



func _show_aisles() -> void:
	var ai: EditorCorredores = editor.corredores
	_tool_panel("corredor", {"selected": ai.selected, "vertex": ai.selected_vertex,
		"new_kind": ai.new_kind, "new_width": ai.new_width, "new_two_way": ai.new_two_way})
## The right panel of the Zona tool.
func _show_zones() -> void:
	_tool_panel("zona", {"selected": editor.selected_zone()})
func _show_network() -> void:
	var rede: EditorRede = editor.rede
	_tool_panel("rede", {"node": rede.selected_node, "edge": rede.selected_edge, "two_way": rede.two_way})
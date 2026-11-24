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

var doc: Object
var model: Object
## The replication played in Simular; the clock of the bridge follows it.
var playback: Object:
	set(v):
		playback = v
		if ui != null:
			ui.clock_load(v)
var cena: CenaUtsim
var editor: EditorUtsim
var camera: CameraOrbita
var sun: DirectionalLight3D
## The playback clock is the bridge's (`relogio.rs`, kept by the `UtsimUi`): the playback
## bar changes it by itself; these read it (the roteiros too).
var t: float:
	get:
		return ui.clock_t() if ui != null else 0.0
var playing: bool:
	get:
		return ui != null and ui.clock_playing()
var speed: float:
	get:
		return ui.clock_speed() if ui != null else 1.0
	set(v):
		if ui != null:
			ui.set_clock_speed(v)
var model_path := ""
## The status already said that another app saved the model over local changes.
var _disk_warned := false
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
## The floor plane; its colour is the model's (`floor_marking.floor_color`).
var _ground: MeshInstance3D
## The grid lines' colour on that floor (`grid.rs`).
var _grid_color := Color(0.22, 0.24, 0.28)
## The performance line over the 3D view (F3), drawn by egui.
var _perf_on := false
var _revision := -1
var _pending := Callable()
## What closing the window does once it may close (the roteiros put a check here instead
## of quitting); empty: quit.
var close_hook := Callable()
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
## The free first-person camera of Simular (F): `UtsimAndar` (`andar.rs`), active while
## walking; its crosshair and hint over the view.
var andar: Object
var _andar_hud: CanvasLayer
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
## `simulacao`: none of them in Simular (the default; they stay in Editar).
var links_hidden: Array = ["simulacao"]


func _ready() -> void:
	# The X of the window asks first when the model has unsaved changes (`request_close`).
	get_tree().set_auto_accept_quit(false)
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
	# The one-writer lock of the open file (UTSim Qualidade and Segurança read it): renewed
	# every minute; taken again when the other app lets it go.
	var lock_timer := Timer.new()
	lock_timer.wait_time = 60.0
	lock_timer.autostart = true
	lock_timer.timeout.connect(_renew_lock)
	add_child(lock_timer)
	camera.frame_box(Vector3(-8, 0, -8), Vector3(8, 0, 8))
	set_status(T("godot-gd-principal-arraste_um_objeto_da_biblioteca"))


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
	_ground = ground
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
	_maximize_window()
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
	set_status(T("godot-gd-desempenho-visiveis") if _perf_on else T("godot-gd-desempenho-ocultas"))


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
	set_status(T("godot-gd-principal-encaixe_na_grade_passo_m", [T("godot-gd-ligado") if on else T("godot-gd-desligado"), str(step)]))


## A text of the interface in the language of now (`UtsimDocument.texto`); the key itself
## while the bridge is not loaded.
func T(key: String, args: Array = []) -> String:
	if doc == null:
		return key
	return doc.texto(key, args)


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
	if doc == null or ui == null:
		return
	for p in ui.route_action(what, _route_state()):
		_run_step(p)
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
	if ui != null:
		ui.set_playback_bar({"visible": mode == "simular"})


## What the routing decisions of the bridge need (`acoes.rs`).
func _route_state() -> Dictionary:
	return {
		"tool": editor.tool if editor != null else "selecionar",
		"place_id": editor.place_id if editor != null else "",
		"snap": snap_on,
		"step": grid_step,
		"links_hidden": PackedStringArray(links_hidden),
	}


## Runs one step the bridge decided (`acoes.rs`: the clicks, the panel rows, the tool
## panels and the keys). False only when a key had nothing to do.
func _run_step(p: Dictionary) -> bool:
	var s: String = p["s"]
	var on: bool = p["b"]
	var rede: EditorRede = editor.rede
	var ai: EditorCorredores = editor.corredores
	match String(p["op"]):
		# Document and modes.
		"modo":
			set_mode(s)
		"alternar_modo":
			set_mode("editar" if mode == "simular" else "simular")
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
		"dialogo":
			var go := _pending
			_pending = Callable()
			if on and go.is_valid():
				go.call()
		"atalhos":
			toggle_shortcuts()
		"vivo_janela":
			open_live_window(s)
		"desempenho":
			toggle_performance()
		"ensino":
			# The UTSim Ensino's mission mode, in its own scene (`ensino.tscn`).
			if ResourceLoader.exists("res://cenas/ensino.tscn"):
				get_tree().change_scene_to_file.call_deferred("res://cenas/ensino.tscn")
			else:
				set_status(T("godot-gd-principal-o_modo_missao_do_utsim"))
		"analisar":
			analysis_action(s)
		"abrir_app":
			_open_in_app(s, p["s2"])
		"resultados":
			results_action(s)
		"pert":
			pert_action(s)
		# Editar.
		"ferramenta":
			editor.set_tool(s, p["s2"])
		"organizar":
			arrange_action(s)
		"encaixe":
			set_snap(on, float(p["x"]))
		"ocultas":
			links_hidden = Array(p["lista"])
		"arrastar_biblioteca":
			# The drag started in the egui library; the editor follows the mouse and the
			# drop is resolved over the 3D view (the free central area).
			editor.start_palette_drag(s)
		"selecionar":
			editor.select_only(s)
		"selecionar_tudo":
			editor.select_all()
		"focar":
			focus_block(s, 8.0)
		"obstaculos":
			editor.corredores.set_obstacles_visible(on)
			_sync_edit()
		"andar":
			if walking():
				stop_walk()
			elif not start_walk():
				set_status(T("godot-gd-principal-para_andar_pela_fabrica_simule"))
		"enquadrar":
			stop_walk()
			if mode == "simular" and cena.playback != null:
				camera.frame_box(cena.info["bounds_min"], cena.info["bounds_max"])
			else:
				_frame_model()
		"excluir":
			editor.delete_selected()
		"cancelar_editor":
			editor.cancel()
		"mover":
			editor.nudge(int(p["n"]), int(p["n2"]))
		"girar":
			editor.rotate_selection(int(p["n"]))
		"fechar_zona":
			editor.zone_finish()
		"fechar_corredor":
			return editor.corredores.finish()
		# Simular.
		"replicacoes":
			_run_reps = int(p["n"])
			_sync_sim()
		"simular_de_novo":
			simulate(_run_reps)
		"bloco":
			var k := int(p["n"])
			if k < cena.blocks.size():
				follow_name = cena.blocks[k]["name"]
				if camera_mode == "orbita":
					focus_block(follow_name, 8.0)
				_sync_sim()
		"camera":
			set_camera_mode(s)
		"gravar_replay":
			_replay_dialog.popup_centered_ratio(0.7)
		"rotulos":
			labels_on = on
			cena.set_labels_visible(on)
			_sync_sim()
		"seguranca":
			seguranca_on = on
			_aplicar_camadas()
			_sync_sim()
		"calor":
			calor_on = on
			_aplicar_camadas()
			_sync_sim()
		"sombras":
			sun.shadow_enabled = on
			_sync_sim()
		"ir_para":
			set_playing(false)
			seek(float(p["x"]), true)
		"reproduzir":
			set_playing(not playing)
		"relogio":
			_apply_clock(true)
		"fluxos_tempo":
			ui.flows_time(t)
		"cancelar_trabalho":
			cancel_job()
		"fechar_painel_vivo":
			# Esc in Simular: the live panel not pinned, or else the most recent one; the
			# key goes on when none is open.
			return mode == "simular" and ui.live_close_top()
		# The properties panel.
		"prop_ok":
			if p["s2"] == "name" and not editor.selected.is_empty():
				editor.selected[0] = String(p["s3"]).strip_edges()
			after_edit.call_deferred()
			set_status(T("godot-gd-principal-alterado", [s, p["s2"]]))
		"prop_erro":
			set_status(T("godot-gd-principal-nao_alterou", [s, p["s2"]]))
			_show_properties.call_deferred()
		"prop_fim":
			after_edit.call_deferred()
		"problema":
			editor.corredores.highlight_issue(s)
		"saida_ordem":
			if not editor.selected.is_empty() and doc.move_connection(editor.selected[0], int(p["n"]), int(p["n2"])):
				after_edit.call_deferred()
		"saida_remover":
			if not editor.selected.is_empty() and doc.disconnect_objects(editor.selected[0], s):
				after_edit.call_deferred()
		"motorista":
			cena.set_driver_visible(s, on)
			_show_properties()
		"dados":
			_data_step(s, p["s2"], p["s3"])
		"miniatura":
			show_miniature(s)
		# The tool panels.
		"rede_novos_mao_dupla":
			rede.two_way = on
		"rede_nome":
			rede.rename(s)
		"rede_excluir":
			rede.delete_selected()
		"rede_mao_dupla":
			rede.set_two_way(on)
		"rede_capacidade":
			rede.set_capacity(int(p["n"]))
		"rede_inverter":
			rede.reverse()
		"corredor_excluir":
			ai.delete_selected()
		"corredor_tipo":
			ai.set_kind(s)
		"corredor_largura":
			ai.set_width(float(p["x"]))
		"corredor_mao_dupla":
			ai.set_two_way(on)
		"corredor_secundaria":
			ai.set_secondary(on)
		"corredor_nome":
			ai.rename(s)
		"corredor_perfil":
			ai.set_profile(s)
		"corredor_remover_vertice":
			ai.remove_vertex()
		"corredor_inverter":
			ai.reverse()
		"zona_campo":
			editor.set_zone_field(s, p["s2"])
		"trajeto":
			editor.trajetos.form(s, p["s2"])
		"mostrar_propriedades":
			_show_properties.call_deferred()
	return true


## The real-data actions of the properties panel and of the fit window.
func _data_step(what: String, path: String, draft_json: String) -> void:
	var draft = JSON.parse_string(draft_json) if draft_json != "" else null
	var d: Dictionary = draft if draft is Dictionary else {}
	match what:
		"aplicar":
			_on_data_apply(path, d)
		"analisar":
			_on_data_analyze(path, d)
		"para_dist":
			_on_data_to_dist(path)
		"relatorio":
			_report_target = "modelo"
			_report_dialog.popup_centered_ratio(0.6)
		"escolher_arquivo":
			_data_pick_path = path
			_data_target = "painel"
			_data_dialog.popup_centered_ratio(0.7)
		"janela_csv":
			_data_target = "janela"
			_data_dialog.popup_centered_ratio(0.7)
		"janela_relatorio":
			_report_target = "janela"
			_report_dialog.popup_centered_ratio(0.6)


# ---------------------------------------------------------------------------
# Document
# ---------------------------------------------------------------------------


func _confirm_discard(then: Callable) -> void:
	if doc != null and doc.is_dirty():
		_pending = then
		if ui != null:
			ui.show_dialog(JSON.stringify({"name": "Dialogo_Descartar", "title": "Descartar mudanças?",
				"text": T("godot-gd-principal-o_modelo_tem_mudancas_que"),
				"buttons": [["Dialogo_Descartar_Ok", "Descartar", "descartar"], ["Dialogo_Descartar_Voltar", "Voltar", "voltar"]]}))
	else:
		then.call()


## Closing the window: with unsaved changes, the same question as New and Open
## (Descartar or Voltar); otherwise it closes.
func request_close() -> void:
	_confirm_discard(func():
		if close_hook.is_valid():
			close_hook.call()
		else:
			get_tree().quit())


func new_document(force := false) -> void:
	var go := func():
		doc.new_model("novo_modelo")
		model_path = ""
		if ui != null:
			ui.clear_indicators()
			ui.live_close_all()
		editor.set_tool("selecionar")
		editor.clear_selection()
		set_mode("editar")
		after_edit(true)
		camera.frame_box(Vector3(-8, 0, -8), Vector3(8, 0, 8))
		set_status(T("godot-gd-principal-novo_modelo"))
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
		set_status(T("godot-gd-principal-nao_abriu", [path.get_file(), r["error"]]))
		return false
	model_path = path
	_disk_warned = false
	if ui != null:
		# The numbers of the node tabs and the live panels were of the model closed.
		ui.clear_indicators()
		ui.live_close_all()
	editor.set_tool("selecionar")
	editor.clear_selection()
	if mode != "editar":
		set_mode("editar")
	after_edit(true)
	_frame_model()
	var issues: Array = doc.issues()
	var lock: String = doc.lock_notice()
	if lock != "":
		set_status(T("godot-gd-principal-aberto_mas_salvar_fica_para", [path.get_file(), lock]))
	elif issues.is_empty():
		set_status(T("godot-gd-principal-aberto_objetos_simular_roda_as", [path.get_file(), doc.objects().size()]))
	else:
		set_status(T("godot-gd-principal-aberto_com_problema_s_veja", [path.get_file(), issues.size()]))
	return true


func _renew_lock() -> void:
	if doc == null:
		return
	var before: String = doc.lock_notice()
	doc.renew_lock()
	if before != "" and doc.lock_notice() == "":
		set_status(T("godot-gd-principal-o_outro_app_fechou_o"))
		if ui != null:
			ui.repaint()
	_check_disk()


func _notification(what: int) -> void:
	if what == NOTIFICATION_WM_CLOSE_REQUEST:
		request_close()
	elif what == NOTIFICATION_APPLICATION_FOCUS_IN:
		_check_disk()
	elif what == NOTIFICATION_APPLICATION_FOCUS_OUT and walking() and not scripted:
		# The cursor is let go when the window loses the focus: the walk ends with it.
		stop_walk()


## Another app (UTSim Qualidade, UTSim Segurança do Trabalho) may have saved the model
## (`disco.rs`): without local changes it is read again; with them, "Salvar" over it is
## refused and the status says why, once.
func _check_disk() -> void:
	if doc == null:
		return
	match String(doc.disk_check()):
		"recarregar":
			_reload_from_disk()
		"conflito":
			if not _disk_warned:
				_disk_warned = true
				set_status(T("godot-gd-principal-outro_app_gravou_e_ha", [model_path.get_file()]))


## Reads the model again from its file, keeping the mode, the camera and the selection.
func _reload_from_disk() -> void:
	var keep: Array = editor.selected.duplicate()
	var r: Dictionary = doc.reload()
	if not r["ok"]:
		set_status(T("godot-gd-principal-nao_releu", [model_path.get_file(), r["error"]]))
		return
	_disk_warned = false
	after_edit(true)
	var names := []
	for o in doc.objects():
		if o["name"] in keep:
			names.append(o["name"])
	editor.select_names(names, false)
	set_status(T("godot-gd-principal-foi_gravado_por_outro_app", [model_path.get_file()]))


## "Abrir no UTSim Qualidade / Segurança" of a node: the other app reads the file, so the
## model is saved first (a new one asks where); then it opens on the node and edits the
## model, which the UTSim 3D reads again when it saves (`_check_disk`).
func _open_in_app(app: String, node: String) -> void:
	if app == "estatistica":
		# Only the readings of the person's time study go, as a spreadsheet: no save and
		# no lock handed over (the UTSim Estatística does not edit the model).
		var err: String = doc.open_in_app(app, node)
		set_status(err if err != "" else T("godot-gd-abrindo-leituras", [node]))
		if ui != null:
			ui.repaint()
		return
	var nome := "UTSim Qualidade" if app == "qualidade" else "UTSim Segurança do Trabalho"
	if doc.path() == "":
		set_status(T("godot-gd-principal-salve_o_modelo_antes_de", [nome]))
		save_as_dialog()
		return
	if doc.is_dirty() and not save():
		return
	var e: String = doc.open_in_app(app, node)
	if e != "":
		set_status(e)
	else:
		set_status(T("godot-gd-principal-abrindo_no_ele_edita_o", [node, nome]))
	if ui != null:
		ui.repaint()


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
		_disk_warned = false
		set_status(T("godot-gd-principal-salvo_em", [doc.path()]))
	else:
		set_status(T("godot-gd-principal-nao_salvou", [r["error"]]))
	_update_title()


func undo() -> void:
	if doc.undo():
		after_edit()
		set_status(T("godot-gd-principal-desfeito"))
	else:
		set_status(T("godot-gd-principal-nada_para_desfazer"))


func redo() -> void:
	if doc.redo():
		after_edit()
		set_status(T("godot-gd-principal-refeito"))
	else:
		set_status(T("godot-gd-principal-nada_para_refazer"))


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
	editor.trajetos.sync()
	if mode == "editar":
		_build_edit_scene()
	_sync_edit()
	_show_properties()
	_update_title()


func _build_edit_scene() -> void:
	var pb: Object = doc.edit_scene(CenaUtsim.skeletal_available())
	cena.show_connections = false
	if pb == null:
		set_status(T("godot-gd-principal-nao_foi_possivel_desenhar_o", [doc.last_error()]))
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
	if editor.tool == "trajeto":
		_show_form(editor.trajetos.panel())
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
			set_status(T("godot-gd-principal-escolha_o_arquivo_de_dados"))
			return {}
		config["source"] = doc.data_relative(file)
		config["column"] = String(draft.get("column", ""))
	else:
		var r: Dictionary = doc.data_parse_values(String(draft.get("text", "")))
		if not r["ok"]:
			set_status(T("godot-gd-principal-valores_colados", [r["error"]]))
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
		set_status(T("godot-gd-principal-aguarde", [job.what()]))
		return
	var j: Object = doc.data_fit(field, config)
	if j == null:
		set_status(T("godot-gd-principal-nao_ajustou", [doc.last_error()]))
		return
	start_job(j, func(_j):
		if doc.data_apply(field, config):
			doc.end_gesture()
			after_edit.call_deferred()
			set_status(T("godot-gd-principal-agora_usa_dados_reais_ctrl", [field]))
		else:
			set_status(T("godot-gd-principal-nao_aplicou_os_dados", [doc.last_error()])))


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
		set_status(T("godot-gd-principal-volta_a_usar_uma_distribuicao", [field]))
	else:
		set_status(T("godot-gd-principal-nao_voltou_para_a_distribuicao", [doc.last_error()]))
		_show_properties.call_deferred()


func _on_data_file(file: String) -> void:
	if ui != null and _data_target == "janela":
		var err := String(ui.fitwin_csv(file))
		set_status(T("godot-gd-valores-lidos-de", [file.get_file()]) if err == "" else T("godot-gd-nao-leu", [file.get_file(), err]))
		return
	if ui != null:
		ui.set_data_file(_data_pick_path, doc.data_relative(file))


## Writes the report of the "Valores reais…" window (all the measures and the choice)
## into `dir` and opens it in the system browser.
func write_fit_report(dir: String) -> void:
	var r: Dictionary = ui.fitwin_report(dir)
	if not r["ok"]:
		set_status(T("godot-gd-principal-sem_relatorio", [r["error"]]))
		return
	last_report = {"index": r["file"], "files": 1}
	set_status(T("godot-gd-principal-relatorio_gravado_em", [r["file"]]))
	if DisplayServer.get_name() != "headless" and not scripted:
		OS.shell_open(String(r["file"]))


## Writes the HTML reports of the fields with data into `dir` (on the bridge's thread)
## and opens `index.html` in the system browser.
func write_data_report(dir: String) -> void:
	var j: Object = doc.data_report(dir)
	if j == null:
		set_status(T("godot-gd-principal-sem_relatorio", [doc.last_error()]))
		return
	start_job(j, func(done):
		last_report = done.data()
		var index := String(last_report.get("index", ""))
		set_status(T("godot-gd-principal-relatorio_gravado_em_arquivos", [dir, int(last_report.get("files", 0))]))
		# A scripted run checks the files; it does not open the browser.
		if index != "" and DisplayServer.get_name() != "headless" and not scripted:
			OS.shell_open(index))


## The actions of the "Organizar" menu and of the keyboard shortcuts.
func arrange_action(what: String) -> void:
	var sel := editor.selection()
	if what.begins_with("alinhar:"):
		if doc.align(sel, what.substr(8)):
			after_edit()
			set_status(T("godot-gd-principal-objetos_alinhados", [sel.size()]))
		else:
			set_status(doc.last_error())
		return
	if what.begins_with("distribuir:"):
		if doc.distribute(sel, what.substr(11)):
			after_edit()
			set_status(T("godot-gd-principal-objetos_distribuidos", [sel.size()]))
		else:
			set_status(doc.last_error())
		return
	match what:
		"copiar":
			var n: int = doc.copy(sel)
			set_status(T("godot-gd-objetos-copiados", [n]) if n > 0 else T("godot-gd-nada-para-copiar"))
		"recortar":
			var n: int = doc.cut(sel)
			if n > 0:
				editor.clear_selection()
				after_edit()
				set_status(T("godot-gd-principal-objeto_s_recortados_ctrl_v", [n]))
			else:
				set_status(T("godot-gd-principal-nada_selecionado_para_recortar"))
		"colar", "duplicar":
			var names: PackedStringArray = doc.paste() if what == "colar" else doc.duplicate(sel)
			if names.is_empty():
				set_status(doc.last_error())
			else:
				editor.select_names(Array(names), false)
				after_edit()
				set_status(T("godot-gd-principal-objeto_s_colados", [names.size(), ", ".join(names)]))
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
	stop_walk()
	mode = "editar"
	playing_replay = false
	if ui != null:
		# The live panels are of the playback: Editar closes them.
		ui.live_close_all()
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
		set_status(T("godot-gd-principal-o_modelo_tem_problema_s", [issues.size(), first]))
		_sync_bar()
		return false
	mode = "simular"
	_set_right(false)
	_sync_play()
	_sync_panels()
	_sync_bar()
	playing_replay = false
	if job != null:
		set_status(T("godot-gd-principal-aguarde", [job.what()]))
		return false
	var j: Object = model.start_run(reps)
	if j == null:
		set_status(T("godot-gd-principal-a_simulacao_nao_comecou", [model.last_error()]))
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
		set_status(T("godot-gd-principal-a_simulacao", [T("godot-gd-foi-cancelada") if j.cancelled() else T("godot-gd-falhou"), j.error()]))
		return
	on_results(j.results())
	# Replication 0 is recorded again and its scene built on a thread of the bridge: the
	# window keeps drawing ("Carregando a reprodução...").
	var reps: int = results.replications()
	start_job(model.start_playback(0, CenaUtsim.skeletal_available()), func(pj):
		if mode != "simular":
			return  # back to Editar while it loaded
		playback = pj.playback()
		if playback == null:
			set_status(T("godot-gd-principal-nao_foi_possivel_gravar_a", [pj.error()]))
			return
		_show_playback()
		set_status(T("godot-gd-principal-replicacoes_concluidas_reproduzindo_a_replicacao", [reps])))


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
	if ui != null:
		follow_name = ui.camera_pick(follow_name, cena.blocks)
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


## The floor in the model's colour, and the grid in a tone that stays visible on it; a
## replay opened alone has its model's colour, not the document's.
func _apply_floor() -> void:
	if _ground == null:
		return
	var c := PackedColorArray()
	if playing_replay and model != null and model.is_replay():
		c = model.replay_floor_colors()
	elif doc != null:
		c = doc.floor_colors()
	if c.size() < 2:
		return
	(_ground.material_override as StandardMaterial3D).albedo_color = c[0]
	_grid_color = c[1]


## The floor grid of Editar: lines every snapping step around the model (at least 40 m
## across), in a tone apart from the floor's.
func _draw_edit_grid() -> void:
	_apply_floor()
	if mode != "editar" or ui == null:
		return
	var at := PackedVector3Array()
	for o in doc.objects():
		at.append(o["position"])
	_grid_lines(ui.edit_grid(at, grid_step))


## The floor grid of Simular: about 40 cells across the scene (`grid.rs`).
func _draw_grid(info: Dictionary) -> void:
	_apply_floor()
	if ui != null:
		_grid_lines(ui.sim_grid(info["bounds_min"], info["bounds_max"]))


## The grid lines computed in Rust (`grid.rs`), as one mesh of lines.
func _grid_lines(points: PackedVector3Array) -> void:
	if _grid != null:
		_grid.queue_free()
	var mesh := ArrayMesh.new()
	if not points.is_empty():
		var colors := PackedColorArray()
		colors.resize(points.size())
		colors.fill(_grid_color)
		var arrays := []
		arrays.resize(Mesh.ARRAY_MAX)
		arrays[Mesh.ARRAY_VERTEX] = points
		arrays[Mesh.ARRAY_COLOR] = colors
		mesh.add_surface_from_arrays(Mesh.PRIMITIVE_LINES, arrays)
	_grid = MeshInstance3D.new()
	_grid.name = "Grade"
	_grid.mesh = mesh
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
	if ui != null:
		ui.clock_play(on and mode == "simular")


## Jumps to `time` (no crossfade when `snap`).
func seek(time: float, snap := false) -> void:
	if ui != null and ui.clock_seek(time):
		_apply_clock(snap)


## Shows the clock's time in the scene (no crossfade when `snap`).
func _apply_clock(snap: bool) -> void:
	cena.apply(t, 0.0, snap)
	if ui != null:
		ui.flows_time(t)


func step_event(direction: int) -> void:
	if ui != null and ui.clock_step(direction):
		_apply_clock(true)


func _process(dt: float) -> void:
	_poll_job()
	if ui != null and ui.clock_tick(dt):
		cena.apply(t, dt)
		ui.flows_time(t)
	_frames += 1
	if walking():
		_walk_frame(dt)
		if _frames % 10 == 0:
			cena.limit_labels(camera)
	elif mode == "simular" and playback != null:
		_drive_camera()
		if _frames % 10 == 0:
			cena.limit_labels(camera)
	if _perf_on and ui != null and _frames % 15 == 0:
		ui.set_performance("%d qps | ponte %.2f ms | cena %.2f ms" % [
			Engine.get_frames_per_second(), cena.last_bridge_us / 1000.0, cena.last_apply_us / 1000.0])


## Simular: a left click on an object of the 3D view opens its live results panel (the
## egui panels took the clicks over them already; the camera turns with the other buttons).
func _unhandled_input(event: InputEvent) -> void:
	var mb := event as InputEventMouseButton
	if mb == null or not mb.pressed or mb.button_index != MOUSE_BUTTON_LEFT:
		return
	if mode != "simular" or playback == null or ui == null or over_ui(mb.position):
		return
	if pick_live_panel(mb.position):
		get_viewport().set_input_as_handled()


## Opens the live results panel of the object under the screen point `p` (Simular). The
## ray test is the bridge's (`ao_vivo::escolher`); false when it hits nothing.
func pick_live_panel(p: Vector2) -> bool:
	var cam := get_viewport().get_camera_3d()
	# Built only when the scene has a rig per block (as the editor's `_feed`).
	if cam == null or cena.blocks.is_empty() or cena.rigs.size() != cena.blocks.size():
		return false
	var centers := PackedVector3Array()
	var sizes := PackedVector3Array()
	var yaws := PackedFloat32Array()
	var hidden := PackedByteArray()
	for i in cena.blocks.size():
		# The box of the scene's geometry around the root (the asset is fitted into it by
		# its offset and scale), turned with the root; hidden when the camera's view hid it.
		var root: Node3D = cena.rigs[i]["root"]
		centers.append(cena.block_focus(i))
		sizes.append(cena.blocks[i]["size"])
		yaws.append(root.global_rotation.y)
		hidden.append(1 if cena.hidden_for_view(i) else 0)
	var hit: int = ui.live_pick(cam.project_ray_origin(p), cam.project_ray_normal(p), centers, sizes, yaws, hidden)
	if hit < 0:
		return false
	return open_live_panel(String(cena.blocks[hit]["name"]))


## Opens the live results panel of `name` at the playback time (Simular only).
func open_live_panel(name: String) -> bool:
	if mode != "simular" or playback == null or ui == null:
		return false
	return ui.live_open(name)


func _unhandled_key_input(event: InputEvent) -> void:
	var k := event as InputEventKey
	if k == null or not k.pressed or doc == null or ui == null:
		return
	# Which action the key asks for is Rust's (`shortcuts.rs`, with tests).
	var action := String(ui.key_action(k.keycode, k.ctrl_pressed or k.meta_pressed,
		k.shift_pressed, mode == "editar", editor.tool))
	if action != "" and run_key_action(action):
		get_viewport().set_input_as_handled()


## Runs a keyboard action (see `shortcuts.rs`); false when it did nothing (the key goes on).
## What a key does (the action `UtsimUi.key_action` named), routed by the bridge.
func run_key_action(action: String) -> bool:
	if ui == null:
		return false
	var steps: Array = ui.route_key(action, _route_state())
	if steps.is_empty():
		return false
	var done := true
	for p in steps:
		done = _run_step(p)
	return done


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
		set_status(T("godot-gd-principal-aguarde", [job.what()]))
		return
	job = j
	_job_done = done
	if ui != null:
		ui.set_status_bar({"progress": 0.0, "percent": true})
	set_status(T("godot-gd-principal-texto", [j.what()]))


func cancel_job() -> void:
	if job != null:
		job.cancel()
		set_status(T("godot-gd-principal-cancelando", [job.what()]))


## The job's progress and how it ended are the bridge's (`andamento.rs`).
func _poll_job() -> void:
	if job == null or ui == null:
		return
	var r: Dictionary = ui.poll_job(job)
	if not r["finished"]:
		return
	var j := job
	var done := _job_done
	job = null
	_job_done = Callable()
	if r["message"] != "":
		set_status(r["message"])
	if r["call"] and done.is_valid():
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


# ---------------------------------------------------------------------------
# Live panels in windows of their own
# ---------------------------------------------------------------------------

## The windows of the live panels taken out of the 3D view, by object.
var live_windows := {}


## Opens the live panel of `nome` in a window of its own (`UtsimPainelVivo`): a window of
## the operating system, not embedded, so it can go to another monitor; it keeps being
## updated with the clock. Without a display (the scripted tests) it is embedded.
func open_live_window(nome: String) -> void:
	if live_windows.has(nome):
		(live_windows[nome] as Window).grab_focus()
		return
	var w := Window.new()
	w.name = "JanelaVivo_%s" % nome.validate_node_name()
	w.title = ui.live_window_title(nome)
	w.size = Vector2i(440, 380)
	w.min_size = Vector2i(320, 240)
	var painel = ClassDB.instantiate("UtsimPainelVivo")
	painel.setup(ui, nome)
	w.add_child(painel)
	w.close_requested.connect(func(): _close_live_window(nome, true))
	painel.connect("fechada", func(): _close_live_window(nome, false))
	live_windows[nome] = w
	var root := get_tree().root
	var native := DisplayServer.get_name() != "headless"
	var embed := root.gui_embed_subwindows
	if native:
		# A real window only for this one: the dialogs stay as they are.
		root.gui_embed_subwindows = false
	root.add_child(w)
	w.popup_centered()
	if native:
		root.gui_embed_subwindows = embed


## Closes the window of the live panel of `nome`; `drop`: the user closed the window, so
## the panel goes too.
func _close_live_window(nome: String, drop: bool) -> void:
	if not live_windows.has(nome):
		return
	var w: Window = live_windows[nome]
	live_windows.erase(nome)
	if drop and ui != null:
		ui.live_close(nome)
	w.queue_free()


## The actions of the "Analisar" menu and buttons.
func analysis_action(what: String) -> void:
	match what:
		"resultados":
			if results == null:
				set_status(T("godot-gd-principal-simule_o_modelo_para_ver"))
				return
			show_results(not ui.results_open())
		"fluxos":
			if mode != "simular" or playback == null:
				set_status(T("godot-gd-principal-os_fluxos_de_tokens_aparecem"))
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
		set_status(T("godot-gd-principal-selecione_os_objetos_da_parte"))
		return
	var names := PackedStringArray()
	if between:
		if sel.size() != 2:
			set_status(T("godot-gd-principal-pert_cpm_de_ate_selecione"))
			return
		names = PackedStringArray([sel[1], sel[0]])
	else:
		names = PackedStringArray(sel)
	if document_model() == null:
		set_status(T("godot-gd-principal-o_modelo_tem_problemas_e"))
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
		set_status(T("godot-gd-principal-aguarde", [job.what()]))
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
				set_status(T("godot-gd-principal-aguarde", [job.what()]))
				return
			var j: Object = ui.results_job(arg)
			if j == null:
				set_status(T("godot-gd-principal-nao_comecou", [ui.results_error()]))
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
		set_status(T("godot-gd-principal-um_replay_gravado_nao_roda"))
		return
	var r: Dictionary = doc.enable_samples()
	if not r["ok"]:
		set_status(T("godot-gd-principal-nao_ligou_a_serie_e", [doc.last_error()]))
		return
	after_edit()
	var parts := PackedStringArray()
	if float(r["sample_interval"]) > 0.0:
		parts.append("run.sample_interval = %s" % _num(float(r["sample_interval"])))
	if int(r["histogram_bins"]) > 0:
		parts.append("run.histogram_bins = %d" % int(r["histogram_bins"]))
	simulate(_run_reps)
	set_status(T("godot-gd-principal-ligado_no_modelo_ctrl_z", [", ".join(parts)]))


func apply_warmup(warmup: float) -> void:
	if doc.set_property("run", "warmup", warmup, PackedStringArray()):
		doc.end_gesture()
		after_edit()
		set_status(T("godot-gd-principal-aquecimento_aplicado_ao_modelo_run", [_num(warmup)]))
	else:
		set_status(T("godot-gd-principal-nao_aplicou_o_aquecimento", [doc.last_error()]))


# ---------------------------------------------------------------------------
# Recorded replays (G3)
# ---------------------------------------------------------------------------


## Every tool opens maximized. A scripted run (`roteiro.gd`) keeps the size asked on the
## command line (`--resolution WxH`, as the windowed roteiros do), so its captures keep
## their size. (`window/size/mode=2` in project.godot would ignore `--resolution`.)
func _maximize_window() -> void:
	if DisplayServer.get_name() == "headless" or scripted:
		return
	get_window().mode = Window.MODE_MAXIMIZED


## Opens a replay recorded by `utsim run --replay` or "Gravar replay": it plays in
## Simular (with the statistics of that replication); the document is not touched.
## `--utsim-abrir=<arquivo>` (the Ensino's "Ver em 3D", or any caller): opens that model
## (`.ron`) or replay (`.json`, `utsim_model::replay::Replay` as JSON) when the scene starts.
func _open_from_args() -> void:
	for a in OS.get_cmdline_user_args():
		var arg := String(a)
		# Modo missão do UTSim Ensino (cena própria, `ensino.gd`).
		if arg == "--utsim-ensino" and not Engine.has_meta(&"utsim_ensino_aberto"):
			get_tree().change_scene_to_file.call_deferred("res://cenas/ensino.tscn")
			return
		if arg.begins_with("--utsim-abrir="):
			var path := arg.trim_prefix("--utsim-abrir=")
			if not open_model(path):
				set_status(T("godot-gd-principal-nao_abriu_pedido_na_linha", [path.get_file()]))
			return


func open_replay(path: String) -> bool:
	set_playing(false)
	var m: Object = ClassDB.instantiate(&"UtsimModel")
	var r: Dictionary = m.load(path)
	if not r["ok"]:
		set_status(T("godot-gd-principal-nao_abriu", [path.get_file(), ", ".join(r["errors"])]))
		return false
	if ui != null:
		ui.live_close_all()
	if job != null:
		set_status(T("godot-gd-principal-aguarde", [job.what()]))
		return false
	model = m
	playing_replay = true
	mode = "simular"
	_set_right(false)
	_sync_play()
	_sync_panels()
	_sync_bar()
	on_results(model.replay_results())
	cena.clear(true)
	playback = null
	# The scene is built on a thread of the bridge ("Carregando a reprodução...").
	var objects := int(r["objects"])
	start_job(model.start_playback(0, CenaUtsim.skeletal_available()), func(pj):
		if mode != "simular":
			return  # back to Editar while it loaded
		playback = pj.playback()
		if playback == null:
			set_status(T("godot-gd-principal-nao_foi_possivel_reproduzir", [path.get_file(), pj.error()]))
			return
		_show_playback()
		camera.frame_box(cena.info["bounds_min"], cena.info["bounds_max"])
		set_status(T("godot-gd-principal-replay_aberto_blocos_so_reproducao", [path.get_file(), objects])))
	return true


## Writes the replay of replication 0 of the last run as `.json`.
func save_replay(path: String) -> bool:
	if model == null or not model.is_loaded():
		set_status(T("godot-gd-principal-simule_um_modelo_antes_de"))
		return false
	if not path.ends_with(".json"):
		path += ".json"
	if model.save_replay(0, path):
		set_status(T("godot-gd-principal-replay_da_replicacao_0_gravado", [path]))
		return true
	set_status(T("godot-gd-principal-nao_gravou_o_replay", [model.last_error()]))
	return false


# ---------------------------------------------------------------------------
# Cameras (G3)
# ---------------------------------------------------------------------------


func set_camera_mode(which: String) -> void:
	stop_walk()
	camera_mode = which
	var c: Dictionary = ui.camera_choice(which, follow_name) if ui != null else {"restore": true, "notice": ""}
	if c["restore"]:
		cena.restore_view()
	camera.set_plan(which == "planta")
	_sync_sim()
	if which == "orbita":
		camera.update()
	set_status(c["notice"])


## Places the camera for the modes that follow a block (called every frame in Simular).
func _drive_camera() -> void:
	if ui == null:
		return
	var how: String = ui.camera_drive(camera_mode, follow_name)
	if how == "livre":
		return
	var i := cena.block_index(follow_name)
	if i < 0:
		return
	if how == "alvo":
		camera.target = cena.block_focus(i)
		camera.update()
		return
	# Where the eye goes and what it looks at is Rust's (`camera_follow.rs`); the parts of
	# the vehicle on the sight line and any block the eye goes through are hidden.
	var root: Node3D = cena.rigs[i]["root"]
	var head = cena.driver_eye(i) if camera_mode == "recurso" else null
	var v: PackedVector3Array = ui.follow_view(camera_mode, root.global_position, root.global_basis.x,
		float(cena.rigs[i].get("top", 1.0)), head)
	if v.size() < 2:
		return
	camera_aim = v[1]
	camera.transform = Transform3D(Basis.IDENTITY, v[0]).looking_at(camera_aim, Vector3.UP)
	cena.clear_view(v[0], camera_aim, i)


# ---------------------------------------------------------------------------
# Andar: the free first-person camera of Simular (F)
# ---------------------------------------------------------------------------


func walking() -> bool:
	return andar != null and andar.active()


## Starts walking from where the orbit camera is (`andar.rs`): at eye height, facing its
## target. The cursor is captured (not in a scripted run: the owner may be using the
## mouse); the crosshair and the hint show over the view.
func start_walk() -> bool:
	if mode != "simular" or playback == null or not ClassDB.class_exists(&"UtsimAndar"):
		return false
	if andar == null:
		andar = ClassDB.instantiate(&"UtsimAndar")
	# The orbit's own eye and target (a following camera may have moved the camera).
	camera.update()
	# A drag of the orbit cut by F would go on after the walk (its release is taken here).
	camera.set("_dragging", false)
	andar.begin(camera.global_position, camera.target, camera.yaw)
	# Walking needs perspective, also from the plan view; `set_plan` gives it back.
	camera.projection = Camera3D.PROJECTION_PERSPECTIVE
	cena.restore_view()
	if not scripted:
		Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
	_walk_hud(true)
	walk_step(0.0, 0.0, 0.0, false, 0.0)
	return true


## Back to the camera chosen before (the orbit keeps its target, turn and distance).
func stop_walk() -> void:
	if not walking():
		return
	andar.end()
	if Input.mouse_mode == Input.MOUSE_MODE_CAPTURED:
		Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
	_walk_hud(false)
	camera.set_plan(camera.plan)
	camera.update()
	set_status(T("godot-gd-principal-camera_de_volta"))


## One frame of the walk: the keys held move the eye (W/S, A/D, E/Q or Space/C, Shift).
## With Ctrl held nothing moves: the keys are a shortcut (Ctrl+S saves).
func _walk_frame(dt: float) -> void:
	if Input.is_physical_key_pressed(KEY_CTRL) or Input.is_physical_key_pressed(KEY_META):
		walk_step(0.0, 0.0, 0.0, false, dt)
		return
	var forward := _held([KEY_W, KEY_UP]) - _held([KEY_S, KEY_DOWN])
	var side := _held([KEY_D, KEY_RIGHT]) - _held([KEY_A, KEY_LEFT])
	var up := _held([KEY_E, KEY_SPACE]) - _held([KEY_Q, KEY_C])
	walk_step(forward, side, up, Input.is_physical_key_pressed(KEY_SHIFT), dt)


func _held(keys: Array) -> float:
	for k in keys:
		if Input.is_physical_key_pressed(k):
			return 1.0
	return 0.0


## Moves the walk `dt` seconds and places the camera (the scripts call it directly).
func walk_step(forward: float, side: float, up: float, run: bool, dt: float) -> void:
	if not walking():
		return
	var was_flying: bool = andar.flying()
	andar.step(forward, side, up, run, dt)
	camera_aim = andar.aim()
	camera.transform = Transform3D(Basis.IDENTITY, andar.eye()).looking_at(camera_aim, Vector3.UP)
	if was_flying != andar.flying():
		_walk_hint()


## V: flying or back on the floor.
func walk_toggle_flight() -> void:
	if not walking():
		return
	andar.toggle_flight()
	walk_step(0.0, 0.0, 0.0, false, 0.0)
	_walk_hint()


## The mouse looks, the wheel sets the speed, a click opens the live panel of what is under
## the crosshair, V flies or lands and Esc stops. The keys that move are taken here, so
## Space does not play; with Ctrl they go on as shortcuts.
func _input(event: InputEvent) -> void:
	if not walking():
		return
	var handled := true
	if event is InputEventMouseMotion:
		var mm := event as InputEventMouseMotion
		if Input.mouse_mode == Input.MOUSE_MODE_CAPTURED:
			andar.look(mm.relative.x, mm.relative.y)
	elif event is InputEventMouseButton:
		var mb := event as InputEventMouseButton
		if mb.pressed:
			match mb.button_index:
				MOUSE_BUTTON_WHEEL_UP:
					andar.wheel(1)
					_walk_hint()
				MOUSE_BUTTON_WHEEL_DOWN:
					andar.wheel(-1)
					_walk_hint()
				MOUSE_BUTTON_LEFT:
					pick_live_panel(get_viewport().get_visible_rect().size * 0.5)
	elif event is InputEventKey:
		var k := event as InputEventKey
		var code := k.physical_keycode if k.physical_keycode != KEY_NONE else k.keycode
		if code == KEY_ESCAPE:
			if k.pressed and not k.echo:
				stop_walk()
		elif code == KEY_V:
			if k.pressed and not k.echo:
				walk_toggle_flight()
		elif k.ctrl_pressed or k.meta_pressed or not [KEY_W, KEY_A, KEY_S, KEY_D, KEY_E, KEY_Q,
				KEY_C, KEY_SPACE, KEY_SHIFT, KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT].has(code):
			handled = false
	else:
		handled = false
	if handled:
		get_viewport().set_input_as_handled()


func _walk_hint() -> void:
	if not walking():
		return
	var text: String = andar.hint()
	set_status(text)
	if _andar_hud != null:
		(_andar_hud.get_node("Dica") as Label).text = text


## The crosshair in the middle of the view and the hint at the top.
func _walk_hud(on: bool) -> void:
	if not on:
		if _andar_hud != null:
			_andar_hud.queue_free()
			_andar_hud = null
		return
	if _andar_hud == null:
		_andar_hud = CanvasLayer.new()
		_andar_hud.name = "Andar"
		_andar_hud.layer = 5
		var mira := Label.new()
		mira.name = "Mira"
		mira.text = "+"
		mira.add_theme_font_size_override("font_size", 28)
		mira.add_theme_color_override("font_color", Color(1, 1, 1, 0.9))
		mira.add_theme_color_override("font_outline_color", Color(0, 0, 0, 0.8))
		mira.add_theme_constant_override("outline_size", 4)
		mira.set_anchors_preset(Control.PRESET_CENTER)
		mira.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
		mira.vertical_alignment = VERTICAL_ALIGNMENT_CENTER
		mira.grow_horizontal = Control.GROW_DIRECTION_BOTH
		mira.grow_vertical = Control.GROW_DIRECTION_BOTH
		mira.mouse_filter = Control.MOUSE_FILTER_IGNORE
		_andar_hud.add_child(mira)
		var dica := Label.new()
		dica.name = "Dica"
		dica.add_theme_font_size_override("font_size", 15)
		dica.add_theme_color_override("font_color", Color(1, 1, 1))
		dica.add_theme_color_override("font_outline_color", Color(0, 0, 0, 0.85))
		dica.add_theme_constant_override("outline_size", 5)
		dica.set_anchors_preset(Control.PRESET_CENTER_TOP)
		dica.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
		dica.grow_horizontal = Control.GROW_DIRECTION_BOTH
		dica.position.y = 56
		dica.mouse_filter = Control.MOUSE_FILTER_IGNORE
		_andar_hud.add_child(dica)
		add_child(_andar_hud)
	_walk_hint()


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
## Shows `rows` (see `ui_form.rs`) as the right panel; the edits come back as `form:` actions (`acoes.rs`).
func _show_form(rows: Array) -> void:
	if ui != null:
		ui.show_form(JSON.stringify(rows))


## What the tool panels (`form:<action>:<value>`) asked.




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
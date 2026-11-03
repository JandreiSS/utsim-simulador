## Main scene of the Godot interface (phases G1 to G3): build a model by placing and
## connecting blocks in 3D (Editar), then run the replications in Rust and play the
## replay of replication 0 (Simular).
##
## G3: the path network is drawn and edited with the Rede tool (`rede.gd`); runs go to a
## thread of the bridge (`UtsimJob`, polled every frame, with progress and Cancelar), the
## "Resultados" panel (`resultados.gd`) shows the table, the charts, the output analyses,
## scenarios and optimisation; the PERT/CPM window of the selection (`pert.gd`); the
## token flows in 2D (`fluxos.gd`); tanks with their levels (`cena.gd`); the cameras
## Seguir, Primeira pessoa and Do recurso; and recorded replays (`.json`) can be opened.
##
## The model being edited is a `UtsimDocument` (the bridge over `utsim_edit::Document`):
## every change is a command, validated and undoable. Editar draws the document with the
## same scene code as the playback (`UtsimDocument.edit_scene`), the 3D interaction is
## `editor.gd`, the library `biblioteca.gd` and the properties `painel_propriedades.gd`.
##
## Everything visible is built here in code (lights, floor, grid, camera, panels), so the
## `.tscn` stays a single node and the layout is easy to read in one file. Panels are
## opaque, so the 3D labels never show through them.
##
## The top bar is drawn in Rust with egui (`UtsimUi`, `crates/utsim-godot/src/ui_bar.rs`):
## it shows `_sync_bar()`'s state and its clicks come back as `bar_action(what)`. Its
## widgets keep the names of the Godot buttons they replaced (`Modo_Editar`,
## `Botao_Novo`...), which the scripted runs find through the bridge.
extends Node3D

const SPEEDS := [0.25, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0]
const LEFT_WIDTH := 290
const RIGHT_WIDTH := 420
const TOP_HEIGHT := 46
const BOTTOM_HEIGHT := 40
const PANEL := Color(0.12, 0.13, 0.16)
const RED := Color(1.0, 0.45, 0.42)
const YELLOW := Color(0.98, 0.80, 0.35)
const CAMERAS := [["orbita", "Órbita"], ["seguir", "Seguir"], ["primeira_pessoa", "Primeira pessoa"], ["recurso", "Do recurso"]]

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
## The properties panel: the egui one (Rust) unless a subpanel still in Godot is needed
## (Rede, Corredor, a link, the real-data form), then the Godot one (`_props`).
var _props_godot := true
## Whether the right panel may show at all (Editar, no window covering it).
var _right_allowed := true
## The asset the egui panel's miniature shows (`miniatura` of the scripted checks).
var mini_asset := "-"
var _mini_view: SubViewport
var _mini_scene: CenaUtsim
var _mini_camera: Camera3D
var _left: PanelContainer
var _right: PanelContainer
var _bottom: PanelContainer
var _bar: PanelContainer
var _edit_left: VBoxContainer
var _sim_left: VBoxContainer
var _library: Biblioteca
var _objects_list: ItemList
var _objects_search: LineEdit
var _props: PainelPropriedades
var _props_scroll: ScrollContainer
var _status: Label
var _sim_status: Label
var _summary: RichTextLabel
var _reps: SpinBox
var _play: Button
var _slider: HSlider
var _time: Label
var _blocks: ItemList
var _open_dialog: FileDialog
var _save_dialog: FileDialog
var _discard: ConfirmationDialog
var _shortcuts: AcceptDialog
var _grid: MeshInstance3D
var _slider_lock := false
var _fps: Label
var _revision := -1
var _listed: Array = []
var _pending := Callable()
var _run_reps := 5
## The run shown in Resultados (`UtsimResults`), and the job running now (`UtsimJob`).
var results: Object
var job: Object
var _job_done := Callable()
var _progress: ProgressBar
var _cancel: Button
var _show_obstacles: CheckBox
var _results_panel: PainelResultados
var _pert_panel: PainelPert
var _flows_panel: PainelFluxos
var _camera_pick: OptionButton
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
## The last report of the real data written: `{index, files, failed}`.
var last_report := {}
## A recorded replay (`.json`) is being played instead of the document.
var playing_replay := false


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
		ui.set_reserved_bottom(BOTTOM_HEIGHT)
		_build_miniature()
	editor = EditorUtsim.new()
	editor.name = "Editor"
	add_child(editor)
	editor.setup(self, _ui)
	_library.build(doc.palette(), doc.categories())
	_props.families = doc.families()
	_props.data_families = doc.data_families()
	_fill_shortcuts()
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


func _panel(name: String) -> PanelContainer:
	var p := PanelContainer.new()
	p.name = name
	p.mouse_filter = Control.MOUSE_FILTER_STOP
	_ui.add_child(p)
	return p


func _button(parent: Node, name: String, text: String, tip: String, action: Callable) -> Button:
	var b := Button.new()
	b.name = name
	b.text = text
	b.tooltip_text = tip
	b.focus_mode = Control.FOCUS_NONE
	b.pressed.connect(action)
	parent.add_child(b)
	return b


func _build_ui() -> void:
	_ui = CanvasLayer.new()
	_ui.name = "Interface"
	add_child(_ui)

	_left = _panel("PainelEsquerdo")
	_left.anchor_bottom = 1.0
	_left.offset_top = TOP_HEIGHT
	_left.offset_right = LEFT_WIDTH
	_left.offset_bottom = -BOTTOM_HEIGHT
	var left := VBoxContainer.new()
	_left.add_child(left)
	_edit_left = VBoxContainer.new()
	_edit_left.size_flags_vertical = Control.SIZE_EXPAND_FILL
	left.add_child(_edit_left)
	_library = Biblioteca.new()
	_library.name = "Biblioteca"
	_library.chosen.connect(func(id):
		if editor.tool == "colocar" and editor.place_id == id:
			editor.set_tool("selecionar")
		else:
			editor.set_tool("colocar", id))
	_library.drag_started.connect(func(id): editor.start_palette_drag(id))
	_edit_left.add_child(_library)
	_edit_left.add_child(HSeparator.new())
	var olabel := Label.new()
	olabel.text = "Objetos"
	olabel.theme_type_variation = &"TituloPainel"
	_edit_left.add_child(olabel)
	_objects_search = LineEdit.new()
	_objects_search.name = "Objetos_Busca"
	_objects_search.placeholder_text = "Buscar por nome ou tipo"
	_objects_search.clear_button_enabled = true
	_objects_search.text_changed.connect(func(_t): _fill_objects())
	_edit_left.add_child(_objects_search)
	_objects_list = ItemList.new()
	_objects_list.name = "Objetos_Lista"
	_objects_list.size_flags_vertical = Control.SIZE_EXPAND_FILL
	_objects_list.focus_mode = Control.FOCUS_NONE
	_objects_list.item_clicked.connect(func(k, _at, _button):
		editor.select_only(_listed[k]))
	_objects_list.item_activated.connect(func(k): focus_block(_listed[k], 8.0))
	_edit_left.add_child(_objects_list)
	_show_obstacles = CheckBox.new()
	_show_obstacles.name = "Ver_Obstaculos"
	_show_obstacles.text = "Mostrar obstáculos e folgas"
	_show_obstacles.tooltip_text = "Mostra a área ocupada dos objetos, as paredes, a folga dos recursos e o ponto de acesso de cada objeto (arraste-o com a ferramenta Selecionar)"
	_show_obstacles.focus_mode = Control.FOCUS_NONE
	_show_obstacles.toggled.connect(func(on): editor.corredores.set_obstacles_visible(on))
	_edit_left.add_child(_show_obstacles)

	_sim_left = VBoxContainer.new()
	_sim_left.size_flags_vertical = Control.SIZE_EXPAND_FILL
	left.add_child(_sim_left)
	var row := HBoxContainer.new()
	_sim_left.add_child(row)
	var reps_label := Label.new()
	reps_label.text = "Replicações"
	row.add_child(reps_label)
	_reps = SpinBox.new()
	_reps.name = "Simular_Replicacoes"
	_reps.min_value = 1
	_reps.max_value = 1000
	_reps.value = 5
	row.add_child(_reps)
	_button(_sim_left, "Botao_RodarDeNovo", "▶ Simular de novo", "Roda as replicações outra vez", func(): simulate(int(_reps.value)))
	_sim_status = Label.new()
	_sim_status.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	_sim_status.custom_minimum_size = Vector2(260, 0)
	_sim_left.add_child(_sim_status)
	var blocks_label := Label.new()
	blocks_label.text = "Blocos (clique para focar)"
	_sim_left.add_child(blocks_label)
	_blocks = ItemList.new()
	_blocks.name = "Simular_Blocos"
	_blocks.custom_minimum_size = Vector2(260, 200)
	_blocks.size_flags_vertical = Control.SIZE_EXPAND_FILL
	_blocks.item_selected.connect(func(k):
		follow_name = cena.blocks[k]["name"]
		if camera_mode == "orbita":
			focus_block(follow_name, 8.0))
	_sim_left.add_child(_blocks)
	var cam_row := HBoxContainer.new()
	_sim_left.add_child(cam_row)
	var cam_label := Label.new()
	cam_label.text = "Câmera"
	cam_row.add_child(cam_label)
	_camera_pick = OptionButton.new()
	_camera_pick.name = "Simular_Camera"
	_camera_pick.focus_mode = Control.FOCUS_NONE
	_camera_pick.tooltip_text = "Seguir, Primeira pessoa e Do recurso acompanham o bloco escolhido na lista"
	for k in CAMERAS.size():
		_camera_pick.add_item(CAMERAS[k][1], k)
	_camera_pick.item_selected.connect(func(k): set_camera_mode(CAMERAS[k][0]))
	cam_row.add_child(_camera_pick)
	var sim_row := HBoxContainer.new()
	_sim_left.add_child(sim_row)
	_button(sim_row, "Botao_Fluxos", "Fluxos", "Fluxos de tokens em 2D, com as contagens no tempo", func(): analysis_action("fluxos"))
	_button(sim_row, "Botao_GravarReplay", "Gravar replay", "Grava o replay desta replicação em .json (abre de novo com Abrir)", func(): _replay_dialog.popup_centered_ratio(0.7))
	var labels := CheckBox.new()
	labels.text = "Rótulos"
	labels.button_pressed = true
	labels.toggled.connect(func(on): cena.set_labels_visible(on))
	_sim_left.add_child(labels)
	var shadows := CheckBox.new()
	shadows.text = "Sombras"
	shadows.button_pressed = true
	shadows.toggled.connect(func(on): sun.shadow_enabled = on)
	_sim_left.add_child(shadows)
	_summary = RichTextLabel.new()
	_summary.custom_minimum_size = Vector2(260, 180)
	_summary.bbcode_enabled = true
	_summary.scroll_active = true
	_sim_left.add_child(_summary)

	_right = _panel("PainelPropriedades")
	_right.anchor_left = 1.0
	_right.anchor_right = 1.0
	_right.anchor_bottom = 1.0
	_right.offset_left = -RIGHT_WIDTH
	_right.offset_top = TOP_HEIGHT
	_right.offset_bottom = -BOTTOM_HEIGHT
	_props_scroll = ScrollContainer.new()
	_props_scroll.horizontal_scroll_mode = ScrollContainer.SCROLL_MODE_DISABLED
	_right.add_child(_props_scroll)
	_props = PainelPropriedades.new()
	_props.name = "Propriedades"
	_props.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	_props.edited.connect(_on_property_edited)
	_props.issue_clicked.connect(func(m): editor.corredores.highlight_issue(m))
	_props.look_provider = func(asset: String) -> Object:
		if doc == null or editor == null or editor.selected.is_empty():
			return null
		return doc.look_scene(editor.selected[0], asset, CenaUtsim.skeletal_available())
	_props.driver_provider = func(object: String) -> int:
		if cena == null or not cena.has_driver(object):
			return -1
		return 1 if cena.driver_visible(object) else 0
	_props.driver_toggled.connect(func(object: String, on: bool): cena.set_driver_visible(object, on))
	_props.action.connect(func(what): arrange_action(what))
	_props.columns_provider = func(file: String) -> Array:
		if doc == null:
			return []
		var r: Dictionary = doc.data_columns(file)
		return Array(r["columns"]) if r["ok"] else []
	_props.data_apply.connect(_on_data_apply)
	_props.data_analyze.connect(_on_data_analyze)
	_props.data_to_dist.connect(_on_data_to_dist)
	_props.data_report.connect(func(): _report_dialog.popup_centered_ratio(0.6))
	_props.data_pick_file.connect(func(path):
		_data_pick_path = path
		_data_dialog.popup_centered_ratio(0.7))
	_props.gesture_ended.connect(func():
		doc.end_gesture()
		after_edit.call_deferred())
	_props.reorder.connect(func(k, delta):
		if doc.move_connection(editor.selected[0], k, delta):
			after_edit.call_deferred())
	_props.disconnect_output.connect(func(to):
		if doc.disconnect_objects(editor.selected[0], to):
			after_edit.call_deferred())
	_props_scroll.add_child(_props)

	_bottom = _panel("BarraInferior")
	_bottom.anchor_top = 1.0
	_bottom.anchor_right = 1.0
	_bottom.anchor_bottom = 1.0
	_bottom.offset_top = -BOTTOM_HEIGHT
	var bottom := HBoxContainer.new()
	_bottom.add_child(bottom)
	_status = Label.new()
	_status.name = "Status"
	_status.clip_text = true
	_status.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	bottom.add_child(_status)
	_progress = ProgressBar.new()
	_progress.name = "Progresso"
	_progress.custom_minimum_size = Vector2(220, 0)
	_progress.visible = false
	bottom.add_child(_progress)
	_cancel = _button(bottom, "Botao_Cancelar", "Cancelar", "Para na próxima replicação", func(): cancel_job())
	_cancel.visible = false

	# Playback bar (Simular), above the status bar.
	_bar = _panel("BarraReproducao")
	_bar.anchor_top = 1.0
	_bar.anchor_right = 1.0
	_bar.anchor_bottom = 1.0
	_bar.offset_left = LEFT_WIDTH + 10
	_bar.offset_right = -10
	_bar.offset_top = -BOTTOM_HEIGHT - 52
	_bar.offset_bottom = -BOTTOM_HEIGHT - 6
	var h := HBoxContainer.new()
	_bar.add_child(h)
	_button(h, "Reproducao_Anterior", "◀ evento", "Evento anterior", func(): step_event(-1))
	_play = _button(h, "Reproducao_Play", "Reproduzir", "Reproduzir ou pausar (espaço)", func(): set_playing(not playing))
	_play.custom_minimum_size = Vector2(110, 0)
	_button(h, "Reproducao_Proximo", "evento ▶", "Próximo evento", func(): step_event(1))
	var sp := OptionButton.new()
	sp.name = "Reproducao_Velocidade"
	sp.focus_mode = Control.FOCUS_NONE
	for k in SPEEDS.size():
		sp.add_item("%sx" % str(SPEEDS[k]), k)
	sp.selected = SPEEDS.find(1.0)
	sp.item_selected.connect(func(k): speed = SPEEDS[k])
	h.add_child(sp)
	_slider = HSlider.new()
	_slider.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	_slider.size_flags_vertical = Control.SIZE_SHRINK_CENTER
	_slider.step = 0.0
	_slider.focus_mode = Control.FOCUS_NONE
	_slider.value_changed.connect(func(v):
		if not _slider_lock:
			seek(v, true))
	h.add_child(_slider)
	_time = Label.new()
	_time.custom_minimum_size = Vector2(170, 0)
	h.add_child(_time)

	_fps = Label.new()
	_fps.offset_left = LEFT_WIDTH + 10
	_fps.offset_right = LEFT_WIDTH + 330
	_fps.offset_top = TOP_HEIGHT + 6
	_fps.theme_type_variation = &"Ajuda"
	_fps.mouse_filter = Control.MOUSE_FILTER_IGNORE
	_fps.name = "Desempenho"
	# Measurement for development, not for the user: hidden until F3.
	_fps.visible = false
	_ui.add_child(_fps)

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
	_discard = ConfirmationDialog.new()
	_discard.name = "Dialogo_Descartar"
	_discard.title = "Descartar mudanças?"
	_discard.dialog_text = "O modelo tem mudanças que não foram salvas. Descartar?"
	_discard.ok_button_text = "Descartar"
	_discard.cancel_button_text = "Voltar"
	_discard.confirmed.connect(func():
		if _pending.is_valid():
			_pending.call()
		_pending = Callable())
	_ui.add_child(_discard)
	_results_panel = PainelResultados.new()
	_ui.add_child(_results_panel)
	_results_panel.build(self)
	_pert_panel = PainelPert.new()
	_ui.add_child(_pert_panel)
	_pert_panel.build(self)
	_flows_panel = PainelFluxos.new()
	_ui.add_child(_flows_panel)
	_flows_panel.build(self)
	for panel in [_results_panel, _pert_panel, _flows_panel]:
		panel.anchor_right = 1.0
		panel.anchor_bottom = 1.0
		panel.offset_left = LEFT_WIDTH + 12
		panel.offset_top = TOP_HEIGHT + 30
		panel.offset_right = -12
		panel.offset_bottom = -BOTTOM_HEIGHT - 60
		panel.mouse_filter = Control.MOUSE_FILTER_STOP
		panel.visible = false
	_flows_panel.offset_left = -560
	_flows_panel.anchor_left = 1.0
	for panel in [_results_panel, _pert_panel]:
		panel.visibility_changed.connect(_sync_panels)
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
	_report_dialog.dir_selected.connect(func(p): write_data_report(p))
	_ui.add_child(_report_dialog)
	_replay_dialog = FileDialog.new()
	_replay_dialog.name = "Dialogo_Replay"
	_replay_dialog.access = FileDialog.ACCESS_FILESYSTEM
	_replay_dialog.file_mode = FileDialog.FILE_MODE_SAVE_FILE
	_replay_dialog.filters = PackedStringArray(["*.json ; Replays do UTSim"])
	_replay_dialog.title = "Gravar replay"
	_replay_dialog.file_selected.connect(func(p): save_replay(p))
	_ui.add_child(_replay_dialog)
	_shortcuts = AcceptDialog.new()
	_shortcuts.name = "Janela_Atalhos"
	_shortcuts.title = "Atalhos"
	_shortcuts.ok_button_text = "Fechar"
	_ui.add_child(_shortcuts)
	if ClassDB.class_exists(&"UtsimUi"):
		ui = ClassDB.instantiate(&"UtsimUi")
		ui.name = "InterfaceRust"
		ui.action.connect(bar_action)
		_ui.add_child(ui)


func _fill_shortcuts() -> void:
	var scroll := ScrollContainer.new()
	scroll.custom_minimum_size = Vector2(640, 520)
	var grid := GridContainer.new()
	grid.columns = 2
	grid.add_theme_constant_override("h_separation", 18)
	var pairs: Array = Array(doc.shortcuts())
	pairs.append(["F3", "Mostra ou oculta as medidas de desempenho sobre a vista 3D"])
	for pair in pairs:
		var k := Label.new()
		k.text = pair[0]
		k.add_theme_color_override("font_color", Color(0.62, 0.76, 0.98))
		grid.add_child(k)
		var v := Label.new()
		v.text = pair[1]
		grid.add_child(v)
	scroll.add_child(grid)
	_shortcuts.add_child(scroll)


## The performance line over the 3D view (frames per second, bridge and scene times):
## hidden by default, F3 shows and hides it.
func toggle_performance() -> void:
	_fps.visible = not _fps.visible
	set_status("Medidas de desempenho %s (F3)." % ("visíveis" if _fps.visible else "ocultas"))


func toggle_shortcuts() -> void:
	if _shortcuts.visible:
		_shortcuts.hide()
	else:
		_shortcuts.popup_centered()


## Left and right x of the 3D view not covered by the side panels (`CameraOrbita.free_span`).
func free_span() -> Vector2:
	var w := get_viewport().get_visible_rect().size.x
	var lo := float(LEFT_WIDTH) if _left != null and _left.visible else 0.0
	var hi := w - float(RIGHT_WIDTH) if _right != null and _right.visible else w
	if ui != null:
		# The free area egui leaves (its side panels, once they move to Rust).
		var free: Rect2 = ui.central_rect()
		if free.size.x > 0.0:
			lo = maxf(lo, free.position.x)
			hi = minf(hi, free.end.x)
	return Vector2(lo, hi)


## Whether screen point `p` is over a panel (the 3D view does not take it).
func over_ui(p: Vector2) -> bool:
	if ui != null and ui.is_over(p):
		return true
	for c in [_left, _right, _bottom, _bar, _results_panel, _pert_panel, _flows_panel]:
		if c != null and c.visible and c.get_global_rect().has_point(p):
			return true
	return false


## The Resultados and PERT/CPM windows cover the view down to the playback bar in
## Simular and down to the status line in Editar (no bar there); while one is open in
## Editar, the properties panel, which they would half cover, is hidden.
func _sync_panels() -> void:
	var covering: bool = _results_panel.visible or _pert_panel.visible
	for panel in [_results_panel, _pert_panel]:
		panel.offset_bottom = -BOTTOM_HEIGHT - (60 if mode == "simular" else 12)
	if mode == "editar":
		_set_right(not covering)


## Shows or hides the right panel (the egui one or the Godot one, see `_props_godot`).
func _set_right(on: bool) -> void:
	_right_allowed = on
	_apply_right()


func _apply_right() -> void:
	_right.visible = _right_allowed and (_props_godot or ui == null)
	if ui != null:
		ui.set_properties_visible(_right_allowed and not _props_godot)


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
	_status.text = text
	if _sim_status != null and mode == "simular":
		_sim_status.text = text


func on_tool_changed() -> void:
	_sync_bar()
	_library.mark(editor.place_id)
	_show_properties()


## The node or edge chosen with the Rede tool changed.
func on_network_selection() -> void:
	_show_properties()


## The aisle or vertex chosen with the Corredor tool changed.
func on_aisle_selection() -> void:
	_show_properties()


func on_selection_changed() -> void:
	_show_properties()
	_mark_listed()


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
	})


## The clicks of the top bar (`UtsimUi.action`): the same as the Godot buttons did.
func bar_action(what: String) -> void:
	if doc == null:
		return
	if what.contains("\t"):
		_panel_action(what.split("\t"))
		return
	var arg := what.get_slice(":", 1) if what.contains(":") else ""
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
		"atalhos":
			toggle_shortcuts()
	_sync_bar()


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
		"dados_reais":
			if not editor.selected.is_empty():
				_props._data_mode["%s|%s" % [editor.selected[0], a[1]]] = true
			_show_properties()
		"miniatura":
			show_miniature(a[1])


# ---------------------------------------------------------------------------
# Document
# ---------------------------------------------------------------------------


func _confirm_discard(then: Callable) -> void:
	if doc != null and doc.is_dirty():
		_pending = then
		_discard.popup_centered()
	else:
		then.call()


func new_document(force := false) -> void:
	var go := func():
		doc.new_model("novo_modelo")
		model_path = ""
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
	_fill_objects()
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


func _fill_objects() -> void:
	if _objects_list == null or doc == null:
		return
	_objects_list.clear()
	_listed.clear()
	var wanted := _objects_search.text.strip_edges().to_lower()
	for o in doc.objects():
		var text := "%s (%s)" % [o["name"], o["kind_label"]]
		if wanted != "" and not text.to_lower().contains(wanted):
			continue
		var k := _objects_list.add_item(text)
		if int(o["problems"]) > 0:
			_objects_list.set_item_custom_fg_color(k, RED)
			_objects_list.set_item_tooltip(k, "%d problema(s)" % o["problems"])
		elif int(o["warnings"]) > 0:
			_objects_list.set_item_custom_fg_color(k, YELLOW)
			_objects_list.set_item_tooltip(k, "%d aviso(s)" % o["warnings"])
		_listed.append(o["name"])
	_mark_listed()


func _mark_listed() -> void:
	_objects_list.deselect_all()
	for k in _listed.size():
		if editor.selected.has(_listed[k]):
			_objects_list.select(k, false)


func _show_properties() -> void:
	if doc == null or mode != "editar":
		return
	var scroll := _props_scroll.scroll_vertical
	var godot := ui == null or not editor.selected_link.is_empty() or editor.tool == "rede" or editor.tool == "corredor"
	var sel: Array = editor.selected
	var name: String = sel[0] if not sel.is_empty() else "run"
	if not godot and _needs_data_form(name):
		godot = true
	if _props_godot != godot:
		_props_godot = godot
		_apply_right()
	if not godot:
		# The egui panel (Rust) reads the schema from the document itself.
		_props.clear()
		var others := PackedStringArray()
		for k in range(1, sel.size()):
			others.append(sel[k])
		var extra_line := ""
		if sel.size() > 1:
			extra_line = "%d objetos selecionados: as mudanças valem para todos do mesmo tipo." % sel.size()
		ui.show_properties(name, others, extra_line, _driver_state(name))
		return
	if not editor.selected_link.is_empty():
		_show_link(editor.selected_link)
		return
	if editor.tool == "rede":
		_show_network()
		_props_scroll.scroll_vertical = scroll
		return
	if editor.tool == "corredor":
		_show_aisles()
		_props_scroll.scroll_vertical = scroll
		return
	var props: Dictionary = doc.properties(name)
	var extra := ""
	if sel.size() > 1:
		extra = "%d objetos selecionados: as mudanças valem para todos do mesmo tipo." % sel.size()
	_props.show_properties(props, extra)
	_props_scroll.scroll_vertical = scroll


## Whether the object's panel needs the real-data form (still in Godot): a field fed by
## data, or one switched to "Dados reais" and not applied yet.
func _needs_data_form(name: String) -> bool:
	for k in _props._data_mode:
		if String(k).begins_with(name + "|"):
			return true
	var data: Dictionary = doc.properties(name).get("data", {})
	for v in data.values():
		if String((v as Dictionary).get("source", "")) == "dados":
			return true
	return false


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


func _show_link(link: Array) -> void:
	_props.clear()
	var title := Label.new()
	title.text = "Ligação"
	title.theme_type_variation = &"TituloPainel"
	_props.add_child(title)
	var l := Label.new()
	l.text = "%s → %s" % link
	_props.add_child(l)
	var rm := Button.new()
	rm.name = "Ligacao_Remover"
	rm.text = "Remover ligação (Del)"
	rm.pressed.connect(func(): editor.delete_selected())
	_props.add_child(rm)


func _on_property_edited(path: String, value: Variant) -> void:
	var sel: Array = editor.selected
	var name: String = sel[0] if not sel.is_empty() else "run"
	var others := PackedStringArray()
	for k in range(1, sel.size()):
		others.append(sel[k])
	if doc.set_property(name, path, value, others):
		if path == "name":
			editor.selected[0] = String(value).strip_edges()
		if not path.begins_with("visual.color"):
			doc.end_gesture()
		after_edit.call_deferred()
		set_status("%s: %s alterado." % [name, path])
	else:
		set_status("Não alterou %s: %s" % [path, doc.last_error()])
		_show_properties.call_deferred()


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
			_props.drafts_seen.erase(_props.draft_key(path))
			_props.clear_requests()
			after_edit.call_deferred()
			set_status("%s agora usa dados reais (Ctrl+Z desfaz)." % field)
		else:
			set_status("Não aplicou os dados: %s" % doc.last_error()))


## The analysis of an applied entry that has none yet (the panel asks once).
func _on_data_analyze(path: String, draft: Dictionary) -> void:
	var field := _selected_field(path)
	var config := _data_config(draft)
	_props.clear_requests()
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
		_props.clear_requests()
		after_edit.call_deferred()
		set_status("%s volta a usar uma distribuição (Ctrl+Z desfaz)." % field)
	else:
		set_status("Não voltou para a distribuição: %s" % doc.last_error())
		_show_properties.call_deferred()


func _on_data_file(file: String) -> void:
	_props.set_data_file(_data_pick_path, file)


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
		if index != "" and DisplayServer.get_name() != "headless":
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
	_edit_left.visible = true
	_sim_left.visible = false
	_set_right(true)
	_bar.visible = false
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
	_edit_left.visible = false
	_sim_left.visible = true
	_set_right(false)
	_bar.visible = true
	_sync_panels()
	_sync_bar()
	_reps.value = reps
	playing_replay = false
	if job != null:
		set_status("Aguarde: %s." % job.what())
		return false
	var j: Object = model.start_run(reps)
	if j == null:
		set_status("A simulação não começou: %s" % model.last_error())
		return false
	_sim_status.text = "Simulando %d replicações..." % reps
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


## Builds the scene of `playback` and starts at 0.
func _show_playback() -> void:
	cena.show_connections = true
	cena.build(playback, true)
	var info := cena.info
	_update_shadows(info["bounds_min"], info["bounds_max"])
	_draw_grid(info)
	_blocks.clear()
	for b in cena.blocks:
		_blocks.add_item("%s (%s)" % [b["name"], b["kind"]])
	_slider_lock = true
	_slider.max_value = playback.end()
	_slider_lock = false
	_flows_panel.set_playback(playback)
	if follow_name == "" or cena.block_index(follow_name) < 0:
		follow_name = ""
		for b in cena.blocks:
			if b["mobile"]:
				follow_name = b["name"]
				break
	seek(0.0, true)


## A new run to show (`UtsimResults`): the summary on the left and the Resultados panel.
func on_results(res: Object) -> void:
	if res == null:
		return
	results = res
	_show_summary()
	_results_panel.show_results(res)


func _show_summary() -> void:
	var lines := PackedStringArray()
	var s: Dictionary = results.summary()
	lines.append("[b]Resultados (%d replicações)[/b]" % s["replications"])
	var b = s.get("bottleneck")
	if b != null:
		lines.append("[color=#ff9a3d]Gargalo: %s[/color]" % b)
	for r in results.table():
		lines.append("%s: %s saídas" % [r["name"], PainelResultados.mh(r["output"])])
	lines.append("[color=#aab2be]A tabela completa, os gráficos e as análises estão em Resultados.[/color]")
	_summary.text = "\n".join(lines)


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
	if _play != null:
		_play.text = "Pausar" if playing else "Reproduzir"


## Jumps to `time` (no crossfade when `snap`).
func seek(time: float, snap := false) -> void:
	if playback == null:
		return
	t = clampf(time, 0.0, playback.end())
	cena.apply(t, 0.0, snap)
	_show_time()
	_flows_panel.show_time(t)


func step_event(direction: int) -> void:
	if playback == null:
		return
	set_playing(false)
	var e: float = playback.next_event(t) if direction > 0 else playback.prev_event(t)
	if e >= 0.0:
		seek(e, true)


func _show_time() -> void:
	_slider_lock = true
	_slider.value = t
	_slider_lock = false
	_time.text = "t = %.2f / %.2f" % [t, playback.end()]


func _process(dt: float) -> void:
	_poll_job()
	if playback != null and playing:
		t += dt * speed
		if t >= playback.end():
			t = playback.end()
			set_playing(false)
		cena.apply(t, dt)
		_show_time()
		_flows_panel.show_time(t)
	_frames += 1
	if mode == "simular" and playback != null:
		_drive_camera()
		if _frames % 10 == 0:
			cena.limit_labels(camera)
	if _fps.visible:
		_fps.text = "%d qps | ponte %.2f ms | cena %.2f ms" % [
			Engine.get_frames_per_second(), cena.last_bridge_us / 1000.0, cena.last_apply_us / 1000.0]


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
		KEY_ENTER, KEY_KP_ENTER:
			if editor.tool != "corredor" or not editor.corredores.finish():
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
	_progress.visible = true
	_progress.value = 0
	_cancel.visible = true
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
	if total > 0:
		_progress.max_value = total
		_progress.value = int(p["done"])
		_progress.show_percentage = true
	else:
		# Unknown length: a bar that goes back and forth.
		_progress.max_value = 100
		_progress.value = absf(sin(Time.get_ticks_msec() / 400.0)) * 100.0
		_progress.show_percentage = false
	if not p["finished"]:
		return
	var j := job
	var done := _job_done
	job = null
	_job_done = Callable()
	_progress.visible = false
	_cancel.visible = false
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
			_results_panel.visible = not _results_panel.visible
			if _results_panel.visible:
				_pert_panel.visible = false
		"fluxos":
			if mode != "simular" or playback == null:
				set_status("Os fluxos de tokens aparecem no Simular.")
				return
			_flows_panel.visible = not _flows_panel.visible
			if _flows_panel.visible:
				_flows_panel.set_playback(playback)
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
	_results_panel.visible = false
	_pert_panel.open(names, between)


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
	simulate(int(_reps.value))
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
	_edit_left.visible = false
	_sim_left.visible = true
	_set_right(false)
	_bar.visible = true
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
	for k in CAMERAS.size():
		if CAMERAS[k][0] == which:
			_camera_pick.select(k)
	if which == "orbita":
		camera.update()
		set_status("Câmera de órbita.")
	elif follow_name == "":
		set_status("Escolha na lista o bloco que a câmera acompanha.")
	else:
		set_status("Câmera %s: %s." % [CAMERAS[_camera_pick.selected][1].to_lower(), follow_name])


## Places the camera for the modes that follow a block (called every frame in Simular).
func _drive_camera() -> void:
	if camera_mode == "orbita" or follow_name == "":
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


func _show_aisles() -> void:
	_props.clear()
	var ai: EditorCorredores = editor.corredores
	var title := Label.new()
	title.text = "Corredores de circulação"
	title.theme_type_variation = &"TituloPainel"
	_props.add_child(title)
	var c := ai.counts()
	var info := Label.new()
	info.name = "Corredor_Resumo"
	info.text = "%d corredor(es): %d de pedestres, %d de empilhadeiras, %d mistos; %d cruzamento(s)." % [c["total"], c["pedestre"], c["empilhadeira"], c["misto"], c["cruzamentos"]]
	info.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	info.custom_minimum_size = Vector2(RIGHT_WIDTH - 40, 0)
	_props.add_child(info)
	var help := Label.new()
	help.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	help.custom_minimum_size = Vector2(RIGHT_WIDTH - 40, 0)
	help.theme_type_variation = &"Ajuda"
	help.text = "Clique no chão para pôr os pontos do eixo; Enter ou duplo clique termina, Backspace volta um ponto, Esc cancela. Os pontos encaixam nos outros corredores. Clique num corredor para editá-lo, arraste um vértice para movê-lo, Shift + clique na linha acrescenta um vértice. As faixas amarelas, a zebra dos cruzamentos e as setas aparecem no piso."
	_props.add_child(help)
	var a := ai._aisle(ai.selected)
	if a.is_empty():
		var head := Label.new()
		head.text = "Novos corredores"
		head.theme_type_variation = &"RotuloSecao"
		_props.add_child(head)
		_aisle_kind_row("Corredor_NovoTipo", ai.new_kind, func(k): ai.set_kind.call_deferred(k); _show_properties.call_deferred())
		_aisle_width_row("Corredor_NovaLargura", ai.new_width, func(v): ai.set_width.call_deferred(v))
		_aisle_check("Corredor_NovosMaoDupla", "Mão dupla", ai.new_two_way, func(on): ai.set_two_way.call_deferred(on))
	else:
		var head := Label.new()
		head.text = "Corredor %s" % a["name"]
		head.theme_type_variation = &"RotuloSecao"
		_props.add_child(head)
		var row := HBoxContainer.new()
		var l := Label.new()
		l.text = "Nome"
		row.add_child(l)
		var e := LineEdit.new()
		e.name = "Corredor_Nome"
		e.text = a["name"]
		e.size_flags_horizontal = Control.SIZE_EXPAND_FILL
		e.text_submitted.connect(func(tx): ai.rename.call_deferred(tx))
		row.add_child(e)
		_props.add_child(row)
		var len_label := Label.new()
		len_label.name = "Corredor_Comprimento"
		len_label.text = "%d ponto(s), %s m de comprimento" % [(a["points"] as PackedVector3Array).size(), _num(float(a["length"]))]
		_props.add_child(len_label)
		_aisle_kind_row("Corredor_Tipo", a["kind"], func(k): ai.set_kind.call_deferred(k))
		_aisle_width_row("Corredor_Largura", float(a["width"]), func(v): ai.set_width.call_deferred(v))
		_aisle_check("Corredor_MaoDupla", "Mão dupla", a["two_way"], func(on): ai.set_two_way.call_deferred(on))
		if a["kind"] != "forklift":
			_aisle_check("Corredor_Secundaria", "Passagem secundária de pedestres (mínimo menor)", a["secondary"], func(on): ai.set_secondary.call_deferred(on))
		var prow := HBoxContainer.new()
		var pl := Label.new()
		pl.text = "Perfil"
		prow.add_child(pl)
		var pick := OptionButton.new()
		pick.name = "Corredor_Perfil"
		pick.focus_mode = Control.FOCUS_NONE
		for item in [["Padrão do modelo", ""], ["Brasil (amarelo)", "Brasil"], ["Branco", "Branco"]]:
			pick.add_item(item[0])
			pick.set_item_metadata(pick.item_count - 1, item[1])
			if a["profile"] == item[1]:
				pick.select(pick.item_count - 1)
		pick.item_selected.connect(func(k): ai.set_profile.call_deferred(String(pick.get_item_metadata(k))))
		prow.add_child(pick)
		_props.add_child(prow)
		if ai.selected_vertex >= 0:
			var v := Label.new()
			v.name = "Corredor_Vertice"
			v.text = "Vértice %d selecionado" % (ai.selected_vertex + 1)
			_props.add_child(v)
			var rv := Button.new()
			rv.name = "Corredor_VerticeRemover"
			rv.text = "Remover vértice"
			rv.disabled = (a["points"] as PackedVector3Array).size() <= 2
			rv.pressed.connect(func(): ai.remove_vertex.call_deferred())
			_props.add_child(rv)
		var rev := Button.new()
		rev.name = "Corredor_Inverter"
		rev.text = "Inverter sentido"
		rev.disabled = a["two_way"]
		rev.pressed.connect(func(): ai.reverse.call_deferred())
		_props.add_child(rev)
		var del := Button.new()
		del.name = "Corredor_Excluir"
		del.text = "Excluir corredor (Del)"
		del.pressed.connect(func(): ai.delete_selected.call_deferred())
		_props.add_child(del)
		for w in doc.warnings():
			if String(w["message"]).contains("'%s'" % a["name"]):
				var wl := Label.new()
				wl.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
				wl.custom_minimum_size = Vector2(RIGHT_WIDTH - 40, 0)
				wl.text = "• " + String(w["message"])
				wl.add_theme_color_override("font_color", YELLOW)
				wl.name = "Corredor_Aviso"
				_props.add_child(wl)
	for i in doc.issues():
		var m: String = i["message"]
		if m.contains("corredor") or m.contains("Aisles"):
			var w := Label.new()
			w.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
			w.custom_minimum_size = Vector2(RIGHT_WIDTH - 40, 0)
			w.text = m
			w.add_theme_color_override("font_color", RED)
			_props.add_child(w)


func _aisle_kind_row(node_name: String, kind: String, on_pick: Callable) -> void:
	var row := HBoxContainer.new()
	var l := Label.new()
	l.text = "Tipo"
	row.add_child(l)
	var pick := OptionButton.new()
	pick.name = node_name
	pick.focus_mode = Control.FOCUS_NONE
	for k in EditorCorredores.KINDS.size():
		pick.add_item(EditorCorredores.KIND_LABELS[k])
	pick.select(EditorCorredores.KINDS.find(kind))
	pick.item_selected.connect(func(k): on_pick.call(EditorCorredores.KINDS[k]))
	row.add_child(pick)
	_props.add_child(row)


func _aisle_width_row(node_name: String, width: float, on_change: Callable) -> void:
	var row := HBoxContainer.new()
	var l := Label.new()
	l.text = "Largura (m)"
	row.add_child(l)
	var spin := SpinBox.new()
	spin.name = node_name
	spin.min_value = 0.3
	spin.max_value = 20.0
	spin.step = 0.05
	spin.value = width
	spin.value_changed.connect(func(v): on_change.call(v))
	row.add_child(spin)
	_props.add_child(row)


func _aisle_check(node_name: String, text: String, on: bool, on_toggle: Callable) -> void:
	var box := CheckBox.new()
	box.name = node_name
	box.text = text
	box.button_pressed = on
	box.toggled.connect(func(v): on_toggle.call(v))
	_props.add_child(box)


# ---------------------------------------------------------------------------
# The Rede panel (G3)
# ---------------------------------------------------------------------------


func _show_network() -> void:
	_props.clear()
	var rede: EditorRede = editor.rede
	var title := Label.new()
	title.text = "Rede de caminhos"
	title.theme_type_variation = &"TituloPainel"
	_props.add_child(title)
	var c := rede.counts()
	var info := Label.new()
	info.name = "Rede_Resumo"
	info.text = "%d nó(s), %d trecho(s), %d pista(s) controlada(s)." % [c["nos"], c["trechos"], c["controlados"]]
	_props.add_child(info)
	var help := Label.new()
	help.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	help.custom_minimum_size = Vector2(RIGHT_WIDTH - 40, 0)
	help.theme_type_variation = &"Ajuda"
	help.text = "Clique no chão para pôr um nó; com um nó armado (verde), o próximo clique o liga ao novo por um trecho. Clique num nó para armá-lo, arraste para movê-lo. Clique num trecho para mudar o sentido e a capacidade (com capacidade, a pista é controlada e fica em vermelho). Esc encerra o caminho; Del exclui o selecionado."
	_props.add_child(help)
	var both := CheckBox.new()
	both.name = "Rede_NovosMaoDupla"
	both.text = "Novos trechos em mão dupla"
	both.button_pressed = rede.two_way
	both.toggled.connect(func(on): rede.two_way = on)
	_props.add_child(both)
	if rede.selected_node != "":
		var head := Label.new()
		head.text = "Nó %s" % rede.selected_node
		head.theme_type_variation = &"RotuloSecao"
		_props.add_child(head)
		var row := HBoxContainer.new()
		var l := Label.new()
		l.text = "Nome"
		row.add_child(l)
		var e := LineEdit.new()
		e.name = "Rede_NoNome"
		e.text = rede.selected_node
		e.size_flags_horizontal = Control.SIZE_EXPAND_FILL
		e.text_submitted.connect(func(tx): rede.rename.call_deferred(tx))
		row.add_child(e)
		_props.add_child(row)
		for n in rede.net["nodes"]:
			if n["name"] == rede.selected_node:
				var mp: Vector3 = n["model_position"]
				var pl := Label.new()
				pl.text = "Posição (%s, %s) m" % [_num(mp.x), _num(mp.y)]
				_props.add_child(pl)
		var del := Button.new()
		del.name = "Rede_NoExcluir"
		del.text = "Excluir nó (Del)"
		del.pressed.connect(func(): rede.delete_selected.call_deferred())
		_props.add_child(del)
	if rede.selected_edge.size() == 2:
		var e: Dictionary = rede._edge(rede.selected_edge[0], rede.selected_edge[1])
		var head := Label.new()
		head.text = "Trecho %s → %s" % [e["from"], e["to"]]
		head.theme_type_variation = &"RotuloSecao"
		_props.add_child(head)
		var len_label := Label.new()
		len_label.text = "Comprimento %s m" % _num(float(e["length"]))
		_props.add_child(len_label)
		var two := CheckBox.new()
		two.name = "Rede_TrechoMaoDupla"
		two.text = "Mão dupla"
		two.button_pressed = e["two_way"]
		two.toggled.connect(func(on): rede.set_two_way.call_deferred(on))
		_props.add_child(two)
		var row := HBoxContainer.new()
		var l := Label.new()
		l.text = "Capacidade (veículos; 0 = sem controle)"
		row.add_child(l)
		var cap := SpinBox.new()
		cap.name = "Rede_TrechoCapacidade"
		cap.min_value = 0
		cap.max_value = 100
		cap.value = int(e["capacity"])
		cap.value_changed.connect(func(v): rede.set_capacity.call_deferred(int(v)))
		row.add_child(cap)
		_props.add_child(row)
		if e["controlled"]:
			var red := Label.new()
			red.text = "Pista controlada: no máximo %d veículo(s) ao mesmo tempo." % int(e["capacity"])
			red.add_theme_color_override("font_color", RED)
			_props.add_child(red)
		var rev := Button.new()
		rev.name = "Rede_TrechoInverter"
		rev.text = "Inverter sentido"
		rev.disabled = e["two_way"]
		rev.pressed.connect(func(): rede.reverse.call_deferred())
		_props.add_child(rev)
		var del := Button.new()
		del.name = "Rede_TrechoExcluir"
		del.text = "Excluir trecho (Del)"
		del.pressed.connect(func(): rede.delete_selected.call_deferred())
		_props.add_child(del)
	for i in doc.issues():
		var m: String = i["message"]
		if m.contains("network") or m.contains("rede") or m.contains("nó"):
			var w := Label.new()
			w.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
			w.custom_minimum_size = Vector2(RIGHT_WIDTH - 40, 0)
			w.text = m
			w.add_theme_color_override("font_color", RED)
			_props.add_child(w)

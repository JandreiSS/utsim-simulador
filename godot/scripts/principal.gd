## Main scene of the Godot interface (phase G1): open a model, run the replications in
## Rust, and play the replay of replication 0 with the library models.
##
## Everything visible is built here in code (lights, floor grid, camera, panels), so the
## `.tscn` stays a single node and the layout is easy to read in one file.
extends Node3D

const SPEEDS := [0.25, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0]

var model: Object
var playback: Object
var cena: CenaUtsim
var camera: CameraOrbita
var sun: DirectionalLight3D
var t := 0.0
var playing := false
var speed := 1.0
var model_path := ""

var _status: Label
var _summary: RichTextLabel
var _reps: SpinBox
var _play: Button
var _slider: HSlider
var _time: Label
var _blocks: ItemList
var _dialog: FileDialog
var _simulate: Button
var _grid: MeshInstance3D
var _slider_lock := false
var _fps: Label


func _ready() -> void:
	_build_world()
	_build_ui()
	if not ClassDB.class_exists(&"UtsimModel"):
		_set_status("A ponte utsim_godot não carregou. Rode scripts/godot.sh e abra o projeto de novo.")
		_simulate.disabled = true
		return
	model = ClassDB.instantiate(&"UtsimModel")
	_set_status("Abra um modelo (.ron) para começar.")


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
	add_child(camera)
	camera.current = true
	cena = CenaUtsim.new()
	cena.name = "Cena"
	add_child(cena)


func _build_ui() -> void:
	var ui := CanvasLayer.new()
	add_child(ui)
	var panel := PanelContainer.new()
	panel.position = Vector2(10, 10)
	panel.custom_minimum_size = Vector2(320, 0)
	ui.add_child(panel)
	var box := VBoxContainer.new()
	panel.add_child(box)
	var title := Label.new()
	title.text = "UTSim"
	title.add_theme_font_size_override("font_size", 20)
	box.add_child(title)
	var open := Button.new()
	open.text = "Abrir modelo..."
	open.pressed.connect(func(): _dialog.popup_centered_ratio(0.7))
	box.add_child(open)
	var row := HBoxContainer.new()
	box.add_child(row)
	var reps_label := Label.new()
	reps_label.text = "Replicações"
	row.add_child(reps_label)
	_reps = SpinBox.new()
	_reps.min_value = 1
	_reps.max_value = 1000
	_reps.value = 5
	row.add_child(_reps)
	_simulate = Button.new()
	_simulate.text = "▶ Simular"
	_simulate.disabled = true
	_simulate.pressed.connect(func(): simulate(int(_reps.value)))
	box.add_child(_simulate)
	_status = Label.new()
	_status.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	_status.custom_minimum_size = Vector2(300, 0)
	box.add_child(_status)
	var blocks_label := Label.new()
	blocks_label.text = "Blocos (clique para focar)"
	box.add_child(blocks_label)
	_blocks = ItemList.new()
	_blocks.custom_minimum_size = Vector2(300, 200)
	_blocks.item_selected.connect(_on_block_selected)
	box.add_child(_blocks)
	var labels := CheckBox.new()
	labels.text = "Rótulos"
	labels.button_pressed = true
	labels.toggled.connect(func(on): cena.set_labels_visible(on))
	box.add_child(labels)
	var shadows := CheckBox.new()
	shadows.text = "Sombras"
	shadows.button_pressed = true
	shadows.toggled.connect(func(on): sun.shadow_enabled = on)
	box.add_child(shadows)
	_summary = RichTextLabel.new()
	_summary.custom_minimum_size = Vector2(300, 180)
	_summary.bbcode_enabled = true
	_summary.scroll_active = true
	box.add_child(_summary)

	# Playback bar at the bottom.
	var bar := PanelContainer.new()
	bar.anchor_left = 0.0
	bar.anchor_right = 1.0
	bar.anchor_top = 1.0
	bar.anchor_bottom = 1.0
	bar.offset_left = 340
	bar.offset_right = -10
	bar.offset_top = -50
	bar.offset_bottom = -10
	ui.add_child(bar)
	var h := HBoxContainer.new()
	bar.add_child(h)
	var back := Button.new()
	back.text = "◀ evento"
	back.tooltip_text = "Evento anterior"
	back.pressed.connect(func(): step_event(-1))
	h.add_child(back)
	_play = Button.new()
	_play.text = "Reproduzir"
	_play.custom_minimum_size = Vector2(110, 0)
	_play.pressed.connect(func(): set_playing(not playing))
	h.add_child(_play)
	var fwd := Button.new()
	fwd.text = "evento ▶"
	fwd.tooltip_text = "Próximo evento"
	fwd.pressed.connect(func(): step_event(1))
	h.add_child(fwd)
	var sp := OptionButton.new()
	for k in SPEEDS.size():
		sp.add_item("%sx" % str(SPEEDS[k]), k)
	sp.selected = SPEEDS.find(1.0)
	sp.item_selected.connect(func(k): speed = SPEEDS[k])
	h.add_child(sp)
	_slider = HSlider.new()
	_slider.size_flags_horizontal = Control.SIZE_EXPAND_FILL
	_slider.size_flags_vertical = Control.SIZE_SHRINK_CENTER
	_slider.step = 0.0
	_slider.value_changed.connect(func(v):
		if not _slider_lock:
			seek(v, true))
	h.add_child(_slider)
	_time = Label.new()
	_time.custom_minimum_size = Vector2(170, 0)
	h.add_child(_time)
	_fps = Label.new()
	_fps.position = Vector2(-10, 10)
	_fps.anchor_left = 1.0
	_fps.anchor_right = 1.0
	_fps.offset_left = -260
	_fps.horizontal_alignment = HORIZONTAL_ALIGNMENT_RIGHT
	ui.add_child(_fps)

	_dialog = FileDialog.new()
	_dialog.access = FileDialog.ACCESS_FILESYSTEM
	_dialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE
	_dialog.filters = PackedStringArray(["*.ron ; Modelos do UTSim"])
	_dialog.title = "Abrir modelo"
	_dialog.file_selected.connect(func(p): open_model(p))
	ui.add_child(_dialog)


func _set_status(text: String) -> void:
	_status.text = text


## Loads and validates a model; shows its errors and warnings. True when it is valid.
func open_model(path: String) -> bool:
	if model == null:
		return false
	set_playing(false)
	var r: Dictionary = model.load(path)
	model_path = path
	if not r["ok"]:
		_set_status("Modelo inválido:\n- " + "\n- ".join(r["errors"]))
		_simulate.disabled = true
		return false
	var text := "%s: %d objetos. Clique em Simular." % [r["name"], r["objects"]]
	var warnings: PackedStringArray = r["warnings"]
	if not warnings.is_empty():
		text += "\nAvisos:\n- " + "\n- ".join(warnings)
	_set_status(text)
	_reps.value = maxi(1, int(r["replications"]))
	_simulate.disabled = false
	return true


## Runs `reps` replications (in Rust, in parallel), shows the summary and starts the
## playback of replication 0.
func simulate(reps: int) -> bool:
	if model == null or not model.is_loaded():
		return false
	set_playing(false)
	_set_status("Simulando %d replicações..." % reps)
	var r: Dictionary = model.run(reps)
	if not r["ok"]:
		_set_status("A simulação falhou: %s" % r.get("error", ""))
		return false
	_show_summary(r)
	playback = model.playback(0, CenaUtsim.skeletal_available())
	if playback == null:
		_set_status("Não foi possível gravar a replicação 0: %s" % model.last_error())
		return false
	cena.build(playback)
	var info := cena.info
	camera.frame_box(info["bounds_min"], info["bounds_max"])
	_update_shadows(info)
	_draw_grid(info)
	_blocks.clear()
	for b in cena.blocks:
		_blocks.add_item("%s (%s)" % [b["name"], b["kind"]])
	_slider_lock = true
	_slider.max_value = playback.end()
	_slider_lock = false
	_set_status("%d replicações concluídas. Reproduzindo a replicação 0." % r["replications"])
	seek(0.0, true)
	return true


func _show_summary(r: Dictionary) -> void:
	var metrics: Dictionary = r["metrics"]
	var lines := PackedStringArray()
	lines.append("[b]Resultados (%d replicações)[/b]" % r["replications"])
	var keys := metrics.keys()
	keys.sort()
	for k in keys:
		var key := String(k)
		if not (key.ends_with(".output") or key.ends_with(".content.mean") or key.ends_with(".staytime.mean")):
			continue
		var s: Dictionary = metrics[k]
		var hw: float = s["half_width"]
		var hw_text := "" if is_nan(hw) else " ± %s" % _num(hw)
		lines.append("%s: %s%s" % [key, _num(s["mean"]), hw_text])
	_summary.text = "\n".join(lines)


## A number with about four significant digits.
static func _num(x: float) -> String:
	if x == 0.0 or is_nan(x):
		return str(x)
	var digits := clampi(3 - int(floor(log(absf(x)) / log(10.0))), 0, 6)
	return String.num(x, digits)


## Shadows sized by the scene, as the Bevy viewer does: up to the camera distance plus
## half the diagonal.
func _update_shadows(info: Dictionary) -> void:
	var lo: Vector3 = info["bounds_min"]
	var hi: Vector3 = info["bounds_max"]
	sun.directional_shadow_max_distance = camera.dist + (hi - lo).length() * 0.5 + 10.0


## The floor grid of the Bevy viewer (`draw_world`): about 40 cells across the scene,
## the spacing rounded to 1, 2 or 5 x 10^k metres, lines a little darker than the floor.
func _draw_grid(info: Dictionary) -> void:
	if _grid != null:
		_grid.queue_free()
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
	var cx := roundf((lo.x + hi.x) * 0.5 / step) * step
	var cz := roundf((lo.z + hi.z) * 0.5 / step) * step
	var half := ceili(span / step / 2.0)
	var length := half * step
	var c := Color(0.22, 0.24, 0.28)
	var im := ImmediateMesh.new()
	im.surface_begin(Mesh.PRIMITIVE_LINES)
	for k in range(-half, half + 1):
		var o := k * step
		for v in [Vector3(cx + o, 0.002, cz - length), Vector3(cx + o, 0.002, cz + length),
				Vector3(cx - length, 0.002, cz + o), Vector3(cx + length, 0.002, cz + o)]:
			im.surface_set_color(c)
			im.surface_add_vertex(v)
	im.surface_end()
	_grid = MeshInstance3D.new()
	_grid.mesh = im
	_grid.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	var m := StandardMaterial3D.new()
	m.shading_mode = BaseMaterial3D.SHADING_MODE_UNSHADED
	m.vertex_color_use_as_albedo = true
	m.vertex_color_is_srgb = true
	_grid.material_override = m
	add_child(_grid)


func set_playing(on: bool) -> void:
	playing = on and playback != null
	_play.text = "Pausar" if playing else "Reproduzir"


## Jumps to `time` (no crossfade when `snap`).
func seek(time: float, snap := false) -> void:
	if playback == null:
		return
	t = clampf(time, 0.0, playback.end())
	cena.apply(t, 0.0, snap)
	_show_time()


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
	if playback != null and playing:
		t += dt * speed
		if t >= playback.end():
			t = playback.end()
			set_playing(false)
		cena.apply(t, dt)
		_show_time()
	_fps.text = "%d qps | ponte %.2f ms | cena %.2f ms" % [
		Engine.get_frames_per_second(), cena.last_bridge_us / 1000.0, cena.last_apply_us / 1000.0]


func _unhandled_key_input(event: InputEvent) -> void:
	var k := event as InputEventKey
	if k == null or not k.pressed:
		return
	match k.keycode:
		KEY_SPACE:
			set_playing(not playing)
		KEY_F:
			if cena.playback != null:
				camera.frame_box(cena.info["bounds_min"], cena.info["bounds_max"])


func _on_block_selected(index: int) -> void:
	focus_block(cena.blocks[index]["name"], 8.0)


## Aims the orbit camera at block `name` (the viewer's `Focar` step).
func focus_block(name: String, distance: float, yaw: Variant = null, pitch: Variant = null) -> bool:
	var i := cena.block_index(name)
	if i < 0:
		return false
	camera.focus(cena.block_focus(i), distance, yaw, pitch)
	return true

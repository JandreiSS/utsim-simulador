## Scripted runs for visual QA (autoload). Does nothing unless the project is started
## with the user argument `--utsim-roteiro=<file.json>`:
##
##   godot --path godot -- --utsim-roteiro=roteiros/capturas.json --utsim-capturas=<pasta>
##
## Two kinds of script, both JSON. Paths are relative to the script file; `{saida}` in a
## path is the capture folder. Quits at the end (exit code 1 on any failure) and writes
## the log to `<pasta>/log.txt`.
##
## Without a window (`godot --headless ...`, as `scripts/godot_teste.sh` runs the UI
## scripts) the steps run the same way, with the same input events, but nothing is drawn:
## `capturar` is skipped (logged) and only the checks count.
##
## 1. Captures of models (phase G1): `modelos` lists models; for each one it opens the
##    model, runs the replications, then for each capture jumps to `t`, optionally aims the
##    camera (`focar`, the same numbers as the `Focar` step of the Bevy viewer's scripts)
##    and saves the viewport as `<pasta>/<nome>.png`. A `medir` entry plays for some
##    seconds and writes the frame times.
##
##    {"modelos": [{"modelo": "../../examples/agvs.ron", "replicacoes": 1,
##      "capturas": [{"nome": "agvs_01", "t": 30.0,
##                    "focar": {"objeto": "estacao_1", "distancia": 7.0, "giro": 0.5, "inclinacao": 0.4}}],
##      "medir": {"t": 0.0, "segundos": 4.0, "velocidade": 10.0}}]}
##
## 2. A UI driver (phase G2): `passos` is a list of steps that act on the interface with
##    real input events (`Input.parse_input_event`), so clicks go through the panels and
##    the 3D view as a user's would. Each step is one dictionary:
##
##    - `{"clicar": <alvo>, "shift": false, "duplo": false, "botao": "esquerdo"}`
##    - `{"arrastar": {"de": <alvo>, "para": <alvo>, "shift": false, "passos": 12}}`
##    - `{"mover": <alvo>}` (mouse only)
##    - `{"rolar": <alvo>, "vezes": 3, "sentido": "baixo"}` (the mouse wheel over it)
##    - `{"digitar": "texto", "em": <alvo>, "enter": true}` (clicks `em` first, if given)
##    - `{"escolher": {"controle": "Prop_visual_asset", "item": "bancada_kenney"}}`: picks
##      the item with that text in an OptionButton, as choosing it in the open list does
##      (the list is a separate window the events do not reach without a screen)
##    - `{"tecla": "ctrl+z"}` (`ctrl`, `shift`, `alt` plus a key name: `a`, `f1`, `delete`,
##      `escape`, `enter`, `left`, `tab`...)
##    - `{"esperar": 10}` (frames) or `{"esperar_s": 0.5}`
##    - `{"capturar": "nome"}`
##    - `{"dialogo": "{saida}/modelo.ron"}`: accepts the open file dialog with that path
##    - `{"camera": {"alvo": [x, y], "distancia": 18, "giro": 0.3, "inclinacao": 0.9}}`
##      (model plan coordinates): the target is drawn in the middle of the free area
##      between the panels, and in a window whose free area shows less floor than at
##      1600x900 the camera backs off in proportion (the `camera_distancia` checks too), so
##      the floor points of a script written for 1600x900 stay out of the panels
##    - `{"focar": {"objeto": "empilhadeira_1", "distancia": 4.0, "giro": 0.5, "inclinacao": 0.3}}`:
##      the orbit camera on a block (as the `focar` of the model captures)
##    - `{"abrir": "../../examples/agvs.ron"}`, `{"novo": true}`, `{"modo": "simular"}`
##    - `{"conferir": {...}}`: assertions on the document and the interface (see `_check`).
##    - `{"nota": "texto"}`: written to the log.
##    - `{"esperar_valores": true}`: waits for the comparison of the "Valores reais…" window
##    - `{"esperar_indicadores": true}`: waits for the simulated numbers of the node tabs
##    - `{"esperar_trabalho": 120}` (G3): waits until the run on the bridge's thread ends
##      (at most that many seconds; a failure if it does not). `modo` and the captures of
##      models wait by themselves.
##    - `{"tempo": 120.0}`: jumps the playback to that instant.
##    - `{"outro_app": {"arquivo": "{saida}/m.ron", "trocar": ["de", "para"]}}`: another app
##      (UTSim Qualidade, Segurança) saves the model: the text is replaced in the file, and
##      the main scene looks at the disk as when the window gets the focus back.
##
##    A target (<alvo>) is one of: `{"ponto": [x, y]}` (pixels), `{"controle": "Nome"}`
##    (a control by node name, its centre), `{"chao": [x, y]}` (a floor point in model
##    plan coordinates), `{"objeto": "nome"}` (where a block is drawn),
##    `{"item": ["Lista", 0]}` (an item of an ItemList),
##    `{"ligacao": ["de", "para"]}` (the middle of a connection's line) or
##    `{"porta_saida": "nome"}` / `{"porta_entrada": "nome"}` (a block's nodes),
##    `{"zona": "zona_1"}` (the middle of a risk zone), `{"vertice_zona": ["zona_1", 0]}`,
##    `{"remover_ligacao": true}` (the × of the selected connection or team line),
##    `{"alca_giro": true}` (the rotate handle of the only object selected),
##    `{"no_atende": "operador_1"}` (the orange service node of a transporter or dispatcher),
##    `{"linha_equipe": ["operadores", "processo", "operador_1"]}` (the middle of a team line),
##    `{"no": "nome"}` (a node of the path network), `{"trecho": ["de", "para"]}` (the
##    middle of an edge), `{"vertice": ["corredor_1", 0]}` (a vertex of an aisle),
##    `{"corredor": "corredor_1"}` (the middle of its first segment), `{"acesso": "objeto"}`
##    (the access point of an object, with the obstacles shown) or
##    `{"aba": ["Abas", "Título"]}` (a tab of a TabContainer).
##
## The widgets drawn by egui in Rust (`UtsimUi`, the top bar first) keep the names of the
## Godot nodes they replaced: `{"controle": "Modo_Editar"}` finds the Godot control by that
## name or, when there is none, the egui widget (`_widget`), and the steps act on it with
## the same input events. `escolher`, `texto`, `visivel` and `oculto` work on both.
extends Node

var _out := ""
var _base := ""
var _log := PackedStringArray()
var _failed := false
var _main: Node
## How much the last `camera` step backed off (1 at 1600x900, more in a smaller window):
## the `camera_distancia` checks are scaled by it.
var _cam_scale := 1.0
var _buttons := 0


func _ready() -> void:
	var script_path := ""
	for a in OS.get_cmdline_user_args():
		if a.begins_with("--utsim-roteiro="):
			script_path = a.substr("--utsim-roteiro=".length())
		elif a.begins_with("--utsim-capturas="):
			_out = a.substr("--utsim-capturas=".length())
	if script_path == "":
		return
	_run.call_deferred(script_path)


func _say(text: String) -> void:
	print("[roteiro] ", text)
	_log.append(text)


func _fail(text: String) -> void:
	_failed = true
	_say("FALHOU: " + text)


func _resolve(base: String, path: String) -> String:
	path = path.replace("{saida}", _out)
	if path.is_absolute_path():
		return path.simplify_path()
	return base.path_join(path).simplify_path()


func _frames(n: int) -> void:
	for k in n:
		await get_tree().process_frame


func _run(script_path: String) -> void:
	if script_path.is_relative_path():
		script_path = ProjectSettings.globalize_path("res://").path_join(script_path).simplify_path()
	var text := FileAccess.get_file_as_string(script_path)
	var data = JSON.parse_string(text)
	if typeof(data) != TYPE_DICTIONARY:
		_fail("roteiro inválido: %s" % script_path)
		get_tree().quit(1)
		return
	_base = script_path.get_base_dir()
	if _out == "":
		_out = _resolve(_base, data.get("saida", "capturas"))
	DirAccess.make_dir_recursive_absolute(_out)
	if headless():
		# Without a window the root viewport is tiny: give it the project's size, so the
		# panels and the 3D view are laid out as on screen and the targets land the same.
		get_tree().root.size = Vector2i(
			int(ProjectSettings.get_setting("display/window/size/viewport_width", 1600)),
			int(ProjectSettings.get_setting("display/window/size/viewport_height", 900)))
	await _frames(5)
	_main = get_tree().current_scene
	if _main == null or not _main.has_method("open_model"):
		_fail("a cena principal não está carregada")
		get_tree().quit(1)
		return
	if data.has("modelos"):
		await _run_models(data["modelos"])
	if data.has("passos"):
		Input.use_accumulated_input = false
		var k := 0
		for step in data["passos"]:
			k += 1
			if typeof(step) != TYPE_DICTIONARY:
				_fail("passo %d inválido" % k)
				continue
			await _step(step, k)
			if _failed and data.get("parar_na_falha", true):
				_say("parou no passo %d" % k)
				break
	var log := FileAccess.open(_out.path_join("log.txt"), FileAccess.WRITE)
	if log != null:
		log.store_string("\n".join(_log) + "\n")
	_say("roteiro %s: %s" % [script_path.get_file(), "FALHOU" if _failed else "tudo certo"])
	get_tree().quit(1 if _failed else 0)


# ---------------------------------------------------------------------------
# Captures of models (G1)
# ---------------------------------------------------------------------------


func _run_models(models: Array) -> void:
	for m in models:
		var path := _resolve(_base, m["modelo"])
		_say("modelo %s" % path)
		if not _main.open_model(path):
			_fail("modelo inválido: %s" % path)
			continue
		if not _main.simulate(int(m.get("replicacoes", 1))):
			_fail("simulação falhou: %s" % path)
			continue
		if not await _wait_idle(300.0):
			_fail("simulação não terminou: %s" % path)
			continue
		# Let the imported scenes settle (shaders, shadows).
		await _frames(20)
		for c in m.get("capturas", []):
			_main.seek(float(c.get("t", 0.0)), true)
			if c.has("focar"):
				var f: Dictionary = c["focar"]
				if not _main.focus_block(f["objeto"], float(f.get("distancia", 8.0)), f.get("giro"), f.get("inclinacao")):
					_fail("objeto não encontrado: %s" % f["objeto"])
			# Apply once more so carried loads follow the hands of the posed skeletons.
			await _frames(2)
			_main.seek(float(c.get("t", 0.0)), true)
			await _frames(6)
			await _capture(String(c["nome"]))
		if m.has("medir"):
			await _measure(_main, m["medir"], path.get_file())


## Whether the project runs without a window (`--headless`): nothing to capture.
static func headless() -> bool:
	return DisplayServer.get_name() == "headless"


func _capture(name: String) -> void:
	if headless():
		_say("captura %s pulada (sem janela)" % name)
		return
	await RenderingServer.frame_post_draw
	var img := get_viewport().get_texture().get_image()
	var file := _out.path_join(name + ".png")
	if img == null or img.save_png(file) != OK:
		_fail("não salvou %s" % file)
	else:
		_say("captura %s" % file)


## Plays for `segundos` of real time at `velocidade` and logs the frame times.
func _measure(main: Node, spec: Dictionary, label: String) -> void:
	main.seek(float(spec.get("t", 0.0)), true)
	main.speed = float(spec.get("velocidade", 1.0))
	main.set_playing(true)
	var start := Time.get_ticks_usec()
	var frames := 0
	var bridge := 0
	var apply := 0
	var worst := 0
	var last := start
	var seconds := float(spec.get("segundos", 4.0))
	while Time.get_ticks_usec() - start < int(seconds * 1e6):
		await get_tree().process_frame
		var now := Time.get_ticks_usec()
		worst = maxi(worst, now - last)
		last = now
		frames += 1
		bridge += main.cena.last_bridge_us
		apply += main.cena.last_apply_us
	main.set_playing(false)
	var total := (Time.get_ticks_usec() - start) / 1e6
	var n := maxi(frames, 1)
	_say("medida %s: %d quadros em %.2f s (%.0f qps), quadro pior %.2f ms, ponte %.3f ms, cena %.3f ms, itens %d, blocos %d" % [
		label, frames, total, frames / total, worst / 1000.0, bridge / 1000.0 / n,
		apply / 1000.0 / n, main.cena.items.size(), main.cena.blocks.size()])


# ---------------------------------------------------------------------------
# UI driver (G2)
# ---------------------------------------------------------------------------


## Scrolls a control target into view and waits for the layout, so its rectangle is
## where it will be clicked.
func _reveal(t: Variant) -> void:
	if typeof(t) != TYPE_DICTIONARY or not (t as Dictionary).has("controle"):
		return
	var c := _control(t["controle"])
	if c == null or not c.is_visible_in_tree():
		var u := _egui()
		if u != null:
			u.update_now()
			if u.reveal(String(t["controle"])):
				for i in 4:
					await _frames(1)
					u.update_now()
		return
	var scroll := _scroll_of(c)
	if scroll != null:
		scroll.ensure_control_visible(c)
		await _frames(3)


func _step(s: Dictionary, k: int) -> void:
	for key in ["clicar", "mover", "em", "escolher"]:
		if s.has(key):
			await _reveal(s[key])
	if s.has("arrastar"):
		await _reveal(s["arrastar"]["de"])
	if s.has("nota"):
		_say("— %s" % s["nota"])
	elif s.has("clicar"):
		var p = _target(s["clicar"])
		if p == null:
			_fail("passo %d: alvo não encontrado: %s" % [k, s["clicar"]])
			return
		var button := MOUSE_BUTTON_RIGHT if s.get("botao", "esquerdo") == "direito" else MOUSE_BUTTON_LEFT
		await _click(p, button, s.get("shift", false), s.get("duplo", false))
	elif s.has("arrastar"):
		var d: Dictionary = s["arrastar"]
		var a = _target(d["de"])
		var b = _target(d["para"])
		if a == null or b == null:
			_fail("passo %d: alvo do arrasto não encontrado: %s" % [k, d])
			return
		await _drag(a, b, d.get("shift", false), int(d.get("passos", 12)))
	elif s.has("mover"):
		var p = _target(s["mover"])
		if p == null:
			_fail("passo %d: alvo não encontrado: %s" % [k, s["mover"]])
			return
		_motion(p, p - _last_pos(), 0, false)
		await _frames(2)
	elif s.has("rolar"):
		var p = _target(s["rolar"])
		if p == null:
			_fail("passo %d: alvo não encontrado: %s" % [k, s["rolar"]])
			return
		_motion(p, p - _last_pos(), 0, false)
		await _frames(2)
		var wheel := MOUSE_BUTTON_WHEEL_UP if s.get("sentido", "baixo") == "cima" else MOUSE_BUTTON_WHEEL_DOWN
		for i in int(s.get("vezes", 3)):
			_button(p, wheel, true, false)
			_button(p, wheel, false, false)
			await _frames(2)
	elif s.has("digitar"):
		if s.has("em"):
			var p = _target(s["em"])
			if p == null:
				_fail("passo %d: campo não encontrado: %s" % [k, s["em"]])
				return
			await _click(p, MOUSE_BUTTON_LEFT, false, false)
			var c := _control(s["em"].get("controle", "")) if typeof(s["em"]) == TYPE_DICTIONARY else null
			if c is LineEdit:
				(c as LineEdit).select_all()
			elif c is SpinBox:
				(c as SpinBox).get_line_edit().select_all()
			elif c is TextEdit:
				(c as TextEdit).select_all()
		for ch in String(s["digitar"]).replace("{saida}", _out):
			if ch == "
":
				await _key(KEY_ENTER, 0, false, false, false)
			else:
				await _key(KEY_NONE, ch.unicode_at(0), false, false, false)
		if s.get("enter", true):
			await _key(KEY_ENTER, 0, false, false, false)
		await _frames(3)
	elif s.has("escolher"):
		var e: Dictionary = s["escolher"]
		var o := _control(String(e.get("controle", ""))) as OptionButton
		if (o == null or not o.is_visible_in_tree()) and _egui() != null and _widget_visible(String(e.get("controle", ""))):
			if not _egui().choose(String(e["controle"]), String(e["item"])):
				_fail("passo %d: %s não tem o item %s" % [k, e["controle"], e["item"]])
				return
			await _frames(4)
			return
		if o == null or not o.is_visible_in_tree():
			_fail("passo %d: lista não encontrada: %s" % [k, e])
			return
		var found := -1
		for i in o.item_count:
			if o.get_item_text(i) == String(e["item"]) and not o.is_item_separator(i):
				found = i
		if found < 0:
			_fail("passo %d: %s não tem o item %s" % [k, o.name, e["item"]])
			return
		# The miniature follows the entry under the mouse, then the choice.
		o.get_popup().id_focused.emit(o.get_item_id(found))
		await _frames(2)
		o.select(found)
		o.item_selected.emit(found)
		await _frames(4)
	elif s.has("tecla"):
		await _combo(String(s["tecla"]))
	elif s.has("esperar"):
		await _frames(int(s["esperar"]))
	elif s.has("esperar_s"):
		await get_tree().create_timer(float(s["esperar_s"])).timeout
	elif s.has("capturar"):
		await _frames(4)
		await _capture(String(s["capturar"]))
	elif s.has("dialogo"):
		await _accept_dialog(_resolve(_base, String(s["dialogo"])), k)
	elif s.has("camera"):
		var c: Dictionary = s["camera"]
		var at: Array = c.get("alvo", [0, 0])
		var p := Vector3(float(at[0]), 0, -float(at[1]))
		# The scripts are written for 1600x900: in a smaller window the camera backs off so
		# the free area shows as much floor, and the target goes to the middle of it.
		await _frames(1)
		_cam_scale = _view_scale()
		_main.camera.focus(p, float(c.get("distancia", 20.0)) * _cam_scale, c.get("giro"), c.get("inclinacao"))
		_center_in_free_area(p)
		await _frames(2)
	elif s.has("focar"):
		var f: Dictionary = s["focar"]
		if not _main.focus_block(String(f["objeto"]), float(f.get("distancia", 8.0)), f.get("giro"), f.get("inclinacao")):
			_fail("passo %d: objeto não encontrado: %s" % [k, f["objeto"]])
		await _frames(2)
	elif s.has("abrir"):
		var path := _resolve(_base, String(s["abrir"]))
		if not _main.open_model(path):
			_fail("passo %d: não abriu %s" % [k, path])
		await _frames(5)
	elif s.has("novo"):
		_main.new_document(true)
		await _frames(3)
	elif s.has("modo"):
		if not _main.set_mode(String(s["modo"])):
			_fail("passo %d: não entrou no modo %s" % [k, s["modo"]])
		if not await _wait_idle(300.0):
			_fail("passo %d: a simulação não terminou" % k)
		await _frames(5)
	elif s.has("esperar_valores"):
		# The comparison of the "Valores reais…" window runs on its own thread.
		if _egui() == null or not _egui().fitwin_wait():
			_fail("passo %d: a comparação dos valores reais não terminou" % k)
		await _frames(4)
	elif s.has("esperar_indicadores"):
		# Quality and safety of the node tabs come from one more run on a thread.
		if _egui() == null or not _egui().indicators_wait():
			_fail("passo %d: os números da simulação nas abas não vieram" % k)
		await _frames(4)
	elif s.has("esperar_trabalho"):
		if not await _wait_idle(float(s["esperar_trabalho"])):
			_fail("passo %d: o trabalho não terminou em %s s" % [k, s["esperar_trabalho"]])
		await _frames(3)
	elif s.has("tempo"):
		_main.seek(float(s["tempo"]), true)
		await _frames(3)
	elif s.has("outro_app"):
		var o: Dictionary = s["outro_app"]
		var path := _resolve(_base, String(o["arquivo"]))
		var text := FileAccess.get_file_as_string(path)
		var swap: Array = o.get("trocar", [])
		if text == "" or swap.size() != 2 or not text.contains(String(swap[0])):
			_fail("passo %d: o outro app não achou \"%s\" em %s" % [k, swap[0] if swap.size() > 0 else "", path])
		else:
			var f := FileAccess.open(path, FileAccess.WRITE)
			f.store_string(text.replace(String(swap[0]), String(swap[1])))
			f.close()
			_main._check_disk()
		await _frames(5)
	elif s.has("conferir"):
		_check(s["conferir"], k)
	else:
		_fail("passo %d desconhecido: %s" % [k, s])


## The ratio of the free area (between the side panels) to the window's height, at
## 1600x900 with the library and the properties open: 890 / 900.
const FREE_RATIO := 890.0 / 900.0


## How much farther the camera must be for the free area of this window to show as much
## floor as at 1600x900 (1 when it is as wide or wider).
func _view_scale() -> float:
	var screen := get_viewport().get_visible_rect().size
	var span: Vector2 = _main.free_span()
	var width := span.y - span.x
	if screen.y <= 0.0 or width <= 1.0:
		return 1.0
	return maxf(1.0, FREE_RATIO / (width / screen.y))


## Moves the camera sideways so floor point `p` is drawn in the middle of the free area
## (not under a side panel).
func _center_in_free_area(p: Vector3) -> void:
	var cam: CameraOrbita = _main.camera
	var screen := get_viewport().get_visible_rect().size
	var span: Vector2 = _main.free_span()
	if cam.is_position_behind(p) or screen.y <= 0.0:
		return
	var off := cam.unproject_position(p).x - (span.x + span.y) * 0.5
	var per_px := 2.0 * cam.dist * tan(deg_to_rad(cam.fov) * 0.5) / screen.y
	var right := cam.global_basis.x
	right.y = 0.0
	cam.target += right.normalized() * off * per_px
	cam.update()


func _control(name: String) -> Control:
	if name == "":
		return null
	var n := get_tree().root.find_child(name, true, false)
	return n as Control


## The egui interface of the main scene (`UtsimUi`), or null.
func _egui() -> Object:
	if _main == null:
		return null
	var u = _main.get("ui")
	return u if u is Object and u != null else null


## The rectangle of the egui widget `name` on screen (pixels), or an empty one.
func _widget(name: String) -> Rect2:
	var u := _egui()
	if u == null or name == "":
		return Rect2()
	u.update_now()
	return u.widget_rect(name)


func _widget_visible(name: String) -> bool:
	var u := _egui()
	if u == null:
		return false
	u.update_now()
	# Drawn in the last frame, on screen or scrolled out of a panel (as a Godot control
	# inside a ScrollContainer is still "visible").
	return u.has_widget(name) or u.knows_widget(name)


## The screen point of a target, or null.
func _target(t: Variant) -> Variant:
	if typeof(t) != TYPE_DICTIONARY:
		return null
	var d: Dictionary = t
	if d.has("ponto"):
		return Vector2(float(d["ponto"][0]), float(d["ponto"][1]))
	if d.has("controle"):
		var c := _control(d["controle"])
		if c == null or not c.is_visible_in_tree():
			var r := _widget(String(d["controle"]))
			return r.get_center() if r.has_area() else null
		return c.get_global_rect().get_center()
	if d.has("item"):
		var list := _control(d["item"][0]) as ItemList
		if list == null or int(d["item"][1]) >= list.item_count:
			return null
		var r := list.get_item_rect(int(d["item"][1]))
		return list.get_global_transform() * r.get_center()
	var cam: Camera3D = _main.camera
	if d.has("chao"):
		var p := Vector3(float(d["chao"][0]), 0, -float(d["chao"][1]))
		if cam.is_position_behind(p):
			return null
		return cam.unproject_position(p)
	if d.has("objeto"):
		return _main.editor.block_screen(d["objeto"]) if _main.is_editing() else null
	if d.has("porta_saida") or d.has("porta_entrada"):
		var out := d.has("porta_saida")
		var name: String = d["porta_saida"] if out else d["porta_entrada"]
		var at = _main.editor.port_out(name) if out else _main.editor.port_in(name)
		if at == null or cam.is_position_behind(at):
			return null
		return cam.unproject_position(at)
	if d.has("no"):
		return _main.editor.rede.node_screen(String(d["no"]))
	if d.has("trecho"):
		return _main.editor.rede.edge_screen(String(d["trecho"][0]), String(d["trecho"][1]))
	if d.has("vertice"):
		return _main.editor.corredores.vertex_screen(String(d["vertice"][0]), int(d["vertice"][1]))
	if d.has("corredor"):
		return _main.editor.corredores.line_screen(String(d["corredor"]))
	if d.has("acesso"):
		return _main.editor.corredores.access_screen(String(d["acesso"]))
	if d.has("aba"):
		var tabs := _control(String(d["aba"][0])) as TabContainer
		if tabs == null or not tabs.is_visible_in_tree():
			# The tabs of an egui window: one widget per tab, `<abas>_<título>`.
			var r := _widget("%s_%s" % [d["aba"][0], d["aba"][1]])
			return r.get_center() if r.has_area() else null
		var bar := tabs.get_tab_bar()
		for i in tabs.get_tab_count():
			if tabs.get_tab_title(i) == String(d["aba"][1]):
				return bar.get_global_transform() * bar.get_tab_rect(i).get_center()
		return null
	if d.has("no_atende"):
		var at = _main.editor.port_service(String(d["no_atende"]))
		if at == null or cam.is_position_behind(at):
			return null
		return cam.unproject_position(at)
	if d.has("linha_equipe"):
		var l: Array = d["linha_equipe"]
		return _main.editor.team_line_screen(String(l[0]), String(l[1]), String(l[2]))
	if d.has("zona"):
		_main.editor._feed()
		return _main.editor.ed.zone_screen(String(d["zona"]))
	if d.has("vertice_zona"):
		_main.editor._feed()
		return _main.editor.ed.zone_vertex_screen(String(d["vertice_zona"][0]), int(d["vertice_zona"][1]))
	if d.has("alca_giro"):
		_main.editor._feed()
		return _main.editor.ed.rotate_handle_screen()
	if d.has("remover_ligacao"):
		return _main.editor.remove_screen()
	if d.has("ligacao"):
		var l: Array = d["ligacao"]
		var a = _main.editor.block_position(l[0])
		var b = _main.editor.block_position(l[1])
		if a == null or b == null:
			return null
		var m: Vector3 = ((a as Vector3) + (b as Vector3)) * 0.5
		m.y = 0.06
		return cam.unproject_position(m)
	return null


## Waits until no run is going on the bridge's thread; false after `seconds`.
func _wait_idle(seconds: float) -> bool:
	var start := Time.get_ticks_msec()
	while _main.busy():
		if Time.get_ticks_msec() - start > int(seconds * 1000.0):
			return false
		await get_tree().process_frame
	return true


## Text shown by a control (labels, rich text, buttons, fields).
func _text_of(c: Control) -> String:
	if c is RichTextLabel:
		return (c as RichTextLabel).get_parsed_text()
	if c is Label:
		return (c as Label).text
	if c is Button:
		return (c as Button).text
	if c is LineEdit:
		return (c as LineEdit).text
	if c is TextEdit:
		return (c as TextEdit).text
	if c is SpinBox:
		return str((c as SpinBox).value)
	if c is OptionButton:
		return (c as OptionButton).get_item_text((c as OptionButton).selected) if (c as OptionButton).selected >= 0 else ""
	return ""


func _scroll_of(c: Control) -> ScrollContainer:
	var n := c.get_parent()
	while n != null:
		if n is ScrollContainer:
			return n
		n = n.get_parent()
	return null


var _pos := Vector2.ZERO


func _last_pos() -> Vector2:
	return _pos


func _motion(p: Vector2, rel: Vector2, mask: int, shift: bool) -> void:
	var e := InputEventMouseMotion.new()
	e.position = p
	e.global_position = p
	e.relative = rel
	e.button_mask = mask
	e.shift_pressed = shift
	Input.parse_input_event(e)
	_pos = p


func _button(p: Vector2, button: int, pressed: bool, shift: bool, double := false) -> void:
	var e := InputEventMouseButton.new()
	e.position = p
	e.global_position = p
	e.button_index = button
	e.pressed = pressed
	e.shift_pressed = shift
	e.double_click = double
	e.button_mask = (1 << (button - 1)) if pressed else 0
	Input.parse_input_event(e)
	_pos = p


func _click(p: Vector2, button: int, shift: bool, double: bool) -> void:
	_motion(p, p - _pos, 0, shift)
	await _frames(2)
	_button(p, button, true, shift)
	await _frames(2)
	_button(p, button, false, shift)
	await _frames(2)
	if double:
		_button(p, button, true, shift, true)
		await _frames(1)
		_button(p, button, false, shift)
		await _frames(2)
	await _frames(2)


func _drag(a: Vector2, b: Vector2, shift: bool, steps: int) -> void:
	_motion(a, a - _pos, 0, shift)
	await _frames(2)
	_button(a, MOUSE_BUTTON_LEFT, true, shift)
	await _frames(2)
	var last := a
	for i in range(1, steps + 1):
		var p := a.lerp(b, float(i) / steps)
		_motion(p, p - last, MOUSE_BUTTON_MASK_LEFT, shift)
		last = p
		await _frames(1)
	await _frames(2)
	_button(b, MOUSE_BUTTON_LEFT, false, shift)
	await _frames(4)


func _key(code: Key, unicode: int, ctrl: bool, shift: bool, alt: bool) -> void:
	for pressed in [true, false]:
		var e := InputEventKey.new()
		e.keycode = code
		e.physical_keycode = code
		e.unicode = unicode if pressed else 0
		e.pressed = pressed
		e.ctrl_pressed = ctrl
		e.shift_pressed = shift
		e.alt_pressed = alt
		Input.parse_input_event(e)
		await _frames(1)


func _combo(text: String) -> void:
	var parts := text.to_lower().split("+")
	var ctrl := parts.has("ctrl")
	var shift := parts.has("shift")
	var alt := parts.has("alt")
	var name := parts[parts.size() - 1]
	var names := {"del": KEY_DELETE, "delete": KEY_DELETE, "esc": KEY_ESCAPE, "escape": KEY_ESCAPE,
		"enter": KEY_ENTER, "tab": KEY_TAB, "left": KEY_LEFT, "right": KEY_RIGHT, "up": KEY_UP,
		"down": KEY_DOWN, "space": KEY_SPACE, "backspace": KEY_BACKSPACE}
	var code: Key = names.get(name, OS.find_keycode_from_string(name))
	if code == KEY_NONE:
		_fail("tecla desconhecida: %s" % text)
		return
	var unicode := name.unicode_at(0) if name.length() == 1 and not ctrl else 0
	await _key(code, unicode, ctrl, shift, alt)
	await _frames(3)


## Accepts the visible file dialog with `path`, as if the user typed it and pressed the
## confirm button.
func _accept_dialog(path: String, k: int) -> void:
	await _frames(3)
	var dialog: FileDialog = null
	for name in ["Dialogo_Salvar", "Dialogo_Abrir", "Dialogo_Replay", "Dialogo_Dados", "Dialogo_Relatorio"]:
		var d := get_tree().root.find_child(name, true, false) as FileDialog
		if d != null and d.visible:
			dialog = d
	if dialog == null:
		_fail("passo %d: nenhuma janela de arquivo aberta" % k)
		return
	dialog.current_dir = path.get_base_dir()
	dialog.current_file = path.get_file()
	await _frames(2)
	dialog.hide()
	if dialog.file_mode == FileDialog.FILE_MODE_OPEN_DIR:
		dialog.dir_selected.emit(path)
	else:
		dialog.file_selected.emit(path)
	await _frames(5)


## Assertions: `objetos` (count), `ligacoes` (count), `existe` / `nao_existe` (names),
## `ligado` ([de, para]), `nao_ligado`, `problemas` (count), `sujo` (bool), `selecao`
## (names, primary first), `modo`, `campo` ({objeto, campo, valor}: the RON text),
## `posicao` ({objeto, x, y, tolerancia}: model plan coordinates), `arquivo` ({caminho,
## contem: [...]}), `igual_a` ({arquivo, modelo}: the saved file holds the same model as
## the reference, comments and formatting aside; each difference is a failure),
## `status_contem` (text), `reproduzindo` (bool), `tempo_maior_que`, `pode_desfazer`,
## `pode_refazer`, `ferramenta`, `ligando` (the origin of a connection being made, or ""),
## `zonas` (count), `zona_selecionada` (name), `zona_def` ({nome, tipo, nivel, epi, pontos,
## x0, y0}: a risk zone of the document, model plan), `ligacao_selecionada` (`[de, para]`, or []), `janela_valores` ({aberta, linhas,
## linhas_maior_que, comparado, escolhida, ordem, candidatas_maior_que}: the "Valores
## reais…" window), `no_destacado` (the node under the mouse,
## `[bloco, "out"|"in"|"service"]`, or []), `linha_equipe` / `sem_linha_equipe`
## (`[tipo, de, para]`: a team line exists or not), `linha_equipe_selecionada`,
## `ativo` ({objeto, valor}: `visual.asset`, "" for the default look), `miniatura` (the
## asset the miniature shows), `foco` (name of the control with the keyboard focus) and
## `giro` ({objeto, graus, tolerancia}: how the block is turned as drawn, its root).
##
## G4: `corredores` ({total, pedestre, empilhadeira, misto, cruzamentos}), `faixas` ({continuas_maior_que, zebras_maior_que, setas_maior_que}: polygons of the floor marking in the scene), `corredor` ({nome, tipo, largura, mao_dupla, pontos, perfil}: an aisle of the document), `corredor_selecionado` (name), `desenhando` (points of the aisle being drawn), `obstaculos_visiveis`, `acesso` ({objeto, proprio, x, y, tolerancia}: the access point of an object, model plan), `destaque` (names outlined), `campo_painel` ({objeto, caminho, valor}: a field of the properties schema; `objeto` "run" for the model) and `movel_em` ({objeto, t, x, y, tolerancia}: where a mobile is drawn at time `t`, a range `[min, max]` for `x` and `y`).
##
## G3: `dados` ({campo, fonte, familia, familia_contem, n_maior_que, defasado, analisado}: the real data of a field, `objeto.campo`), `grafico_dados` ({controle, barras_maior_que, curva}: the mini histogram of a field), `rede` ({nos, trechos, controlados}), `no_posicao` ({no, x, y, tolerancia}), `texto` ({controle, contem}: the text of a
## label, rich text or field), `visivel` / `oculto` (control names), `replicacoes`
## (replications of the results shown), `execucao` ({campo, maior_que}: a run setting of
## the document), `ocupado` (a run on the bridge's thread), `grafico` ({controle,
## maior_que}: things drawn by an egui chart), `fluidos` ({nos, tanques, nivel_entre: [a,
## b], altura_no_maximo: metres of the network as drawn, enquadrados: the whole network
## on screen}), `fluxos` ({atividades_maior_que, tokens_pico_maior_que, legivel: boxes
## apart, text whole, zoom legible}), `pert` ({modo, caixas_maior_que, tem_histograma}),
## `motorista` ({objeto, existe, visivel, t, clipe}: the seated driver of a forklift),
## `camera_nos_olhos` (a forklift: the camera is at its driver's eyes), `critico` (names outlined in red), `camera` (mode), `camera_distancia` ([a, b]: the orbit distance), `camera_livre` ({margem,
## distancia}: no mesh within `margem` metres of a following camera nor on the first
## `distancia` metres of its sight line), `modelo_contem` (pieces of the model's RON
## text), `rotulos_no_maximo` (labels drawn) and `replay` (a recorded replay is playing).
func _check(c: Dictionary, k: int) -> void:
	var doc: Object = _main.doc
	var objects: Array = doc.objects()
	var names := []
	for o in objects:
		names.append(o["name"])
	var links: Array = doc.connections()
	var has_link := func(pair: Array) -> bool:
		for l in links:
			if l["from"] == pair[0] and l["to"] == pair[1]:
				return true
		return false
	# Lambdas capture by value: the outcome goes in a dictionary.
	var state := {"ok": true}
	var report := func(what: String, good: bool, detail: String) -> void:
		if not good:
			state["ok"] = false
			_fail("passo %d: %s (%s)" % [k, what, detail])
	for key in c:
		var want = c[key]
		match key:
			"objetos":
				report.call("objetos", objects.size() == int(want), "há %d: %s" % [objects.size(), names])
			"ligacoes":
				report.call("ligações", links.size() == int(want), "há %d" % links.size())
			"existe":
				for n in want:
					report.call("existe %s" % n, names.has(n), str(names))
			"nao_existe":
				for n in want:
					report.call("não existe %s" % n, not names.has(n), str(names))
			"ligado":
				report.call("ligado %s" % [want], has_link.call(want), str(links))
			"nao_ligado":
				report.call("não ligado %s" % [want], not has_link.call(want), str(links))
			"problemas":
				var n: int = doc.issues().size()
				report.call("problemas", n == int(want), "há %d: %s" % [n, doc.issues()])
			"sujo":
				report.call("sujo", doc.is_dirty() == bool(want), str(doc.is_dirty()))
			"selecao":
				report.call("seleção", _main.editor.selected == want, str(_main.editor.selected))
			"modo":
				report.call("modo", _main.mode == want, _main.mode)
			"ferramenta":
				report.call("ferramenta", _main.editor.tool == want, _main.editor.tool)
			"campo":
				var got: String = doc.field_text(want["objeto"], want["campo"])
				report.call("campo %s.%s" % [want["objeto"], want["campo"]], got == want["valor"], got)
			"posicao":
				var found := false
				for o in objects:
					if o["name"] == want["objeto"]:
						found = true
						var p: Vector3 = o["model_position"]
						var tol := float(want.get("tolerancia", 1e-6))
						var good := absf(p.x - float(want.get("x", p.x))) <= tol and absf(p.y - float(want.get("y", p.y))) <= tol
						report.call("posição de %s" % want["objeto"], good, "está em (%s, %s)" % [p.x, p.y])
				report.call("objeto %s" % want["objeto"], found, str(names))
			"arquivo":
				var path := _resolve(_base, String(want["caminho"]))
				var text := FileAccess.get_file_as_string(path)
				report.call("arquivo %s" % path, text != "", "vazio ou ausente")
				for piece in want.get("contem", []):
					report.call("arquivo contém %s" % piece, text.contains(piece), path)
			"igual_a":
				var got := _resolve(_base, String(want["arquivo"]))
				var ref := _resolve(_base, String(want["modelo"]))
				var diff: PackedStringArray = _main.doc.file_differences(got, ref)
				for d in diff:
					report.call("%s igual a %s" % [got.get_file(), ref.get_file()], false, d)
			"ativo":
				var props: Dictionary = doc.properties(want["objeto"])
				var asset := "?"
				for f in props.get("fields", []):
					if f["path"] == "visual.asset":
						asset = str(f["value"])
				report.call("ativo de %s" % want["objeto"], asset == want["valor"], asset)
			"miniatura":
				var view := _control("Prop_visual_asset_miniatura")
				var shown: String = _main._props._mini_asset if view != null else "(sem miniatura)"
				if view == null and _egui() != null and _egui().knows_widget("Prop_visual_asset_miniatura"):
					# The egui panel: the miniature the main scene drew last.
					shown = _main.mini_asset
				report.call("miniatura", shown == want, shown)
			"foco":
				var owner := get_viewport().gui_get_focus_owner()
				var focused: String = String(owner.name) if owner != null else ""
				report.call("foco", focused == want, focused)
			"ligando":
				report.call("ligando", _main.editor.connect_from == want, _main.editor.connect_from)
			"ligacao_selecionada":
				var sel: Array = _main.editor.selected_link
				report.call("ligação selecionada", sel == Array(want), str(sel))
			"zonas":
				report.call("zonas", _main.doc.zones().size() == int(want), str(_main.doc.zones().size()))
			"zona_selecionada":
				report.call("zona selecionada", _main.editor.selected_zone() == String(want), _main.editor.selected_zone())
			"zona_def":
				var found := {}
				for z in _main.doc.zones():
					if z["name"] == want["nome"]:
						found = z
				report.call("zona %s" % want["nome"], not found.is_empty(), "não existe")
				if not found.is_empty():
					if want.has("tipo"):
						report.call("tipo da zona", found["kind"] == want["tipo"], str(found["kind"]))
					if want.has("nivel"):
						report.call("nível da zona", int(found["level"]) == int(want["nivel"]), str(found["level"]))
					if want.has("epi"):
						report.call("EPI da zona", Array(found["ppe"]) == Array(want["epi"]), str(found["ppe"]))
					if want.has("pontos"):
						report.call("pontos da zona", (found["points"] as PackedVector3Array).size() == int(want["pontos"]), str((found["points"] as PackedVector3Array).size()))
					if want.has("x0"):
						var p0: Vector3 = (found["points"] as PackedVector3Array)[0]
						report.call("primeiro ponto da zona", absf(p0.x - float(want["x0"])) < 1e-4 and absf(-p0.z - float(want["y0"])) < 1e-4, "(%s, %s)" % [p0.x, -p0.z])
			"janela_valores":
				var info: Dictionary = _egui().fitwin_info() if _egui() != null else {}
				for wk in want:
					var w = want[wk]
					if wk == "linhas_maior_que":
						report.call("valores: linhas > %s" % w, int(info.get("linhas", 0)) > int(w), str(info))
					elif wk == "candidatas_maior_que":
						report.call("valores: candidatas > %s" % w, int(info.get("candidatas", 0)) > int(w), str(info))
					else:
						var got = info.get(wk, "")
						var same: bool = (float(got) == float(w)) if typeof(w) in [TYPE_INT, TYPE_FLOAT] else str(got) == str(w)
						report.call("valores: %s = %s" % [wk, w], same, str(info))
			"linha_equipe":
				var l: Array = want
				report.call("linha %s" % str(l), _main.editor.has_team_line(l[0], l[1], l[2]), "")
			"sem_linha_equipe":
				var l: Array = want
				report.call("sem linha %s" % str(l), not _main.editor.has_team_line(l[0], l[1], l[2]), "")
			"linha_equipe_selecionada":
				var sel: Dictionary = _main.editor.selected_team
				var got := [sel.get("kind", ""), sel.get("from", ""), sel.get("to", "")] if not sel.is_empty() else []
				report.call("linha de equipe selecionada", got == Array(want), str(got))
			"no_destacado":
				var lit: Array = _main.editor._hover_port
				report.call("nó destacado", lit == Array(want), str(lit))
			"status_contem":
				var status: String = _main.status_text
				report.call("status contém %s" % want, status.contains(want), status)
			"reproduzindo":
				report.call("reproduzindo", _main.playing == bool(want), str(_main.playing))
			"tempo_maior_que":
				report.call("tempo > %s" % want, _main.t > float(want), str(_main.t))
			"rede":
				var got: Dictionary = _main.editor.rede.counts()
				for key2 in want:
					report.call("rede.%s" % key2, int(got.get(key2, -1)) == int(want[key2]), str(got))
			"corredores":
				var got: Dictionary = _main.editor.corredores.counts()
				for key2 in want:
					report.call("corredores.%s" % key2, int(got.get(key2, -1)) == int(want[key2]), str(got))
			"faixas":
				var marks: Dictionary = _main.cena.mark_counts
				for key2 in want:
					var pattern: String = String(key2).trim_suffix("_maior_que")
					pattern = {"continuas": "continuous", "zebras": "zebra", "setas": "arrow"}.get(pattern, pattern)
					report.call("faixas %s > %s" % [pattern, want[key2]], int(marks.get(pattern, 0)) > int(want[key2]), str(marks))
			"corredor":
				var found := false
				for a in doc.aisles():
					if a["name"] != want["nome"]:
						continue
					found = true
					if want.has("tipo"):
						report.call("tipo de %s" % a["name"], a["kind"] == want["tipo"], str(a["kind"]))
					if want.has("largura"):
						report.call("largura de %s" % a["name"], absf(float(a["width"]) - float(want["largura"])) < 1e-6, str(a["width"]))
					if want.has("mao_dupla"):
						report.call("mão dupla de %s" % a["name"], bool(a["two_way"]) == bool(want["mao_dupla"]), str(a["two_way"]))
					if want.has("pontos"):
						report.call("pontos de %s" % a["name"], (a["points"] as PackedVector3Array).size() == int(want["pontos"]), str((a["points"] as PackedVector3Array).size()))
					if want.has("perfil"):
						report.call("perfil de %s" % a["name"], a["profile"] == want["perfil"], str(a["profile"]))
					if want.has("ponto"):
						var mp: Vector2 = (a["model_points"] as PackedVector2Array)[int(want["ponto"][0])]
						var tol := float(want.get("tolerancia", 1e-6))
						report.call("ponto %d de %s" % [int(want["ponto"][0]), a["name"]], absf(mp.x - float(want["ponto"][1])) <= tol and absf(mp.y - float(want["ponto"][2])) <= tol, str(mp))
				report.call("corredor %s" % want["nome"], found, str(doc.aisles().size()))
			"nao_existe_corredor":
				for n in want:
					var exists := false
					for a in doc.aisles():
						if a["name"] == n:
							exists = true
					report.call("corredor %s não existe" % n, not exists, "")
			"corredor_selecionado":
				report.call("corredor selecionado", _main.editor.corredores.selected == want, _main.editor.corredores.selected)
			"desenhando":
				report.call("pontos do corredor em desenho", _main.editor.corredores.points.size() == int(want), str(_main.editor.corredores.points.size()))
			"obstaculos_visiveis":
				report.call("obstáculos visíveis", _main.editor.corredores.show_obstacles == bool(want), str(_main.editor.corredores.show_obstacles))
			"destaque":
				var got: Array = _main.editor.corredores.highlight
				for n in want:
					report.call("destaque de %s" % n, got.has(n), str(got))
			"acesso":
				var found := false
				for a in doc.access_points():
					if a["object"] != want["objeto"]:
						continue
					found = true
					if want.has("proprio"):
						report.call("ponto de acesso próprio de %s" % a["object"], bool(a["custom"]) == bool(want["proprio"]), str(a["custom"]))
					var pt: Vector3 = a["point"]
					var tol := float(want.get("tolerancia", 1e-6))
					if want.has("x"):
						report.call("x do acesso de %s" % a["object"], absf(pt.x - float(want["x"])) <= tol, str(pt.x))
					if want.has("y"):
						report.call("y do acesso de %s" % a["object"], absf(-pt.z - float(want["y"])) <= tol, str(-pt.z))
				report.call("acesso de %s" % want["objeto"], found, "")
			"giro":
				var i: int = _main.cena.block_index(String(want["objeto"]))
				report.call("bloco %s" % want["objeto"], i >= 0, "")
				if i >= 0:
					var drawn := fposmod(rad_to_deg((_main.cena.rigs[i]["root"] as Node3D).rotation.y), 360.0)
					var diff := absf(fposmod(drawn - float(want["graus"]) + 180.0, 360.0) - 180.0)
					report.call("giro de %s" % want["objeto"], diff <= float(want.get("tolerancia", 0.01)), "%.3f°" % drawn)
			"campo_painel":
				var value: Variant = "(sem o campo)"
				for f in doc.properties(String(want["objeto"])).get("fields", []):
					if f["path"] == want["caminho"]:
						value = f["value"]
				report.call("campo %s.%s" % [want["objeto"], want["caminho"]], str(value) == str(want["valor"]), str(value))
			"movel_em":
				_main.seek(float(want["t"]), true)
				var i: int = _main.cena.block_index(String(want["objeto"]))
				report.call("bloco %s" % want["objeto"], i >= 0, "")
				if i >= 0:
					var at: Vector3 = (_main.cena.rigs[i]["root"] as Node3D).global_position
					if want.has("x"):
						report.call("x de %s em t=%s" % [want["objeto"], want["t"]], at.x >= float(want["x"][0]) and at.x <= float(want["x"][1]), "x = %.2f" % at.x)
					if want.has("y"):
						report.call("y de %s em t=%s" % [want["objeto"], want["t"]], -at.z >= float(want["y"][0]) and -at.z <= float(want["y"][1]), "y = %.2f" % -at.z)
			"no_posicao":
				var found := false
				for n in _main.editor.rede.net["nodes"]:
					if n["name"] == want["no"]:
						found = true
						var mp: Vector3 = n["model_position"]
						var tol := float(want.get("tolerancia", 1e-6))
						var good := absf(mp.x - float(want.get("x", mp.x))) <= tol and absf(mp.y - float(want.get("y", mp.y))) <= tol
						report.call("posição do nó %s" % want["no"], good, "está em (%s, %s)" % [mp.x, mp.y])
				report.call("nó %s" % want["no"], found, str(_main.editor.rede.net["nodes"]))
			"texto":
				var ctl := _control(String(want["controle"]))
				var shown := _text_of(ctl) if ctl != null else "(controle ausente)"
				if (ctl == null or not ctl.is_visible_in_tree()) and _widget_visible(String(want["controle"])):
					shown = String(_egui().widget_info(String(want["controle"])).get("text", ""))
				report.call("texto de %s contém %s" % [want["controle"], want["contem"]], shown.contains(String(want["contem"])), shown.substr(0, 300))
			"visivel":
				for n in want:
					var ctl := _control(String(n))
					report.call("%s visível" % n, (ctl != null and ctl.is_visible_in_tree()) or _widget_visible(String(n)), "")
			"oculto":
				for n in want:
					var ctl := _control(String(n))
					report.call("%s oculto" % n, (ctl == null or not ctl.is_visible_in_tree()) and not _widget_visible(String(n)), "")
			"replicacoes":
				var n: int = _main.results.replications() if _main.results != null else 0
				report.call("replicações dos resultados", n == int(want), str(n))
			"execucao":
				var value := NAN
				for f in doc.properties("run").get("fields", []):
					if f["path"] == want["campo"]:
						value = float(f["value"])
				report.call("run.%s > %s" % [want["campo"], want["maior_que"]], value > float(want["maior_que"]), str(value))
			"ocupado":
				report.call("ocupado", _main.busy() == bool(want), str(_main.busy()))
			"grafico":
				# An egui chart: it records how many things it draws.
				var n := -1
				if _widget_visible(String(want["controle"])):
					n = int(_egui().widget_info(String(want["controle"])).get("count", -1))
				report.call("gráfico %s" % want["controle"], n > int(want.get("maior_que", 0)), str(n))
			"fluidos":
				var cena: CenaUtsim = _main.cena
				var fill := cena.fluid_fill()
				var tanks := 0
				for f in fill:
					if f >= 0.0:
						tanks += 1
				if want.has("nos"):
					report.call("nós de fluido", cena.fluid_nodes.size() == int(want["nos"]), str(cena.fluid_nodes.size()))
				if want.has("tanques"):
					report.call("tanques", tanks == int(want["tanques"]), str(tanks))
				if want.has("nivel_entre"):
					var lo := float(want["nivel_entre"][0])
					var hi := float(want["nivel_entre"][1])
					var inside := false
					for f in fill:
						if f >= lo and f <= hi:
							inside = true
					report.call("algum tanque com nível entre %s e %s" % [lo, hi], inside, str(fill))
				var box: Array = cena.fluid_bounds()
				if want.has("altura_no_maximo") and not box.is_empty():
					var h: float = (box[1] as Vector3).y - (box[0] as Vector3).y
					report.call("altura dos fluidos até %s m" % want["altura_no_maximo"], h <= float(want["altura_no_maximo"]), "%.2f m" % h)
				if want.get("enquadrados", false):
					var cam: Camera3D = _main.camera
					var out := 0
					if not box.is_empty():
						var aabb := AABB(box[0], (box[1] as Vector3) - (box[0] as Vector3))
						for corner in 8:
							if not cam.is_position_in_frustum(aabb.get_endpoint(corner)):
								out += 1
					report.call("fluidos inteiros na tela", not box.is_empty() and out == 0, "%d canto(s) fora; caixa %s" % [out, box])
			"fluxos":
				# The egui graph records "atividades= pico= sobreposicoes= cortadas= zoom=".
				var info := {}
				for part in String(_egui().widget_info("Fluxos_Grafo").get("text", "")).split(" "):
					info[part.get_slice("=", 0)] = part.get_slice("=", 1)
				var acts := int(info.get("atividades", "0"))
				report.call("atividades do fluxo", acts > int(want.get("atividades_maior_que", 0)), str(acts))
				if want.has("tokens_pico_maior_que"):
					var peak := int(info.get("pico", "0"))
					report.call("pico de tokens", peak > int(want["tokens_pico_maior_que"]), str(peak))
				if want.get("legivel", false):
					report.call("caixas do fluxo sem sobreposição", info.get("sobreposicoes", "?") == "0", str(info.get("sobreposicoes")))
					report.call("texto do fluxo sem corte", info.get("cortadas", "?") == "0", str(info.get("cortadas")))
					report.call("zoom do fluxo legível", float(info.get("zoom", "0")) >= 0.75 - 1e-3, str(info.get("zoom")))
			"pert":
				var pd = _egui().pert_data() if _egui() != null else null
				var d: Dictionary = pd if pd is Dictionary else {}
				if want.has("modo"):
					report.call("modo do PERT", d.get("mode", "") == want["modo"], str(d.get("mode", "(nenhum)")))
				if want.has("caixas_maior_que"):
					var n: int = (d.get("diagram", {}).get("boxes", []) as Array).size()
					report.call("caixas do diagrama", n > int(want["caixas_maior_que"]), str(n))
				if want.has("tem_histograma"):
					report.call("histograma do PERT", (d.get("histogram") != null) == bool(want["tem_histograma"]), str(d.get("histogram") != null))
			"critico":
				for n in want:
					report.call("%s no caminho crítico" % n, _main.editor.critical.has(n), str(_main.editor.critical))
			"camera":
				report.call("câmera", _main.camera_mode == want, _main.camera_mode)
			"modelo_contem":
				var text: String = doc.text()
				for piece in want:
					report.call("modelo contém %s" % piece, text.contains(piece), text.substr(maxi(text.find("run:"), 0), 240))
			"camera_distancia":
				# Distances of a 1600x900 script, scaled as the last `camera` step was.
				var dd: float = _main.camera.dist / _cam_scale
				report.call("distância da câmera entre %s e %s" % [want[0], want[1]], dd >= float(want[0]) and dd <= float(want[1]), str(dd))
			"motorista":
				# {"objeto", "existe", "visivel", "t", "clipe"}: the seated driver of a forklift
				# (`clipe`: the animation playing, without the "_Loop" suffix).
				if want.has("t"):
					_main.seek(float(want["t"]), true)
				var cena: CenaUtsim = _main.cena
				var i: int = cena.block_index(String(want["objeto"]))
				var has: bool = i >= 0 and cena.rigs[i].has("driver")
				if want.has("existe"):
					report.call("motorista de %s" % want["objeto"], has == bool(want["existe"]), str(has))
				if has:
					if want.has("visivel"):
						var shown: bool = (cena.rigs[i]["driver_node"] as Node3D).is_visible_in_tree()
						report.call("motorista de %s visível" % want["objeto"], shown == bool(want["visivel"]), str(shown))
					if want.has("clipe"):
						var clip: String = (cena.rigs[i]["driver"] as Pessoa).clip_name()
						report.call("clipe do motorista de %s" % want["objeto"], clip == String(want["clipe"]), clip)
			"elemento":
				# {"objeto", "t", "clipe", "fase_entre": [a, b], "aberto"/"fechado": "no"}: the
				# clip a Blender element plays at `t` ("" for none), its phase, and whether a
				# node is out of its rest pose.
				if want.has("t"):
					_main.seek(float(want["t"]), true)
				var cena: CenaUtsim = _main.cena
				var i: int = cena.block_index(String(want["objeto"]))
				report.call("bloco %s" % want["objeto"], i >= 0, "")
				if i >= 0:
					var playing: Array = cena.element_clip(i)
					if want.has("clipe"):
						report.call("clipe de %s" % want["objeto"], String(playing[0]) == String(want["clipe"]), str(playing))
					if want.has("fase_entre"):
						var fase := float(playing[1])
						report.call("fase de %s entre %s e %s" % [want["objeto"], want["fase_entre"][0], want["fase_entre"][1]], fase >= float(want["fase_entre"][0]) and fase <= float(want["fase_entre"][1]), "%.3f" % fase)
					if want.has("aberto") or want.has("fechado"):
						# {"aberto": "porta"} / {"fechado": "porta"}: that node out of / at rest.
						var want_open: bool = want.has("aberto")
						var node_name := String(want["aberto"] if want_open else want["fechado"])
						var moved = cena.element_node_moved(i, node_name)
						report.call("%s de %s %s" % [node_name, want["objeto"], "aberto" if want_open else "fechado"], moved != null and bool(moved) == want_open, str(moved))
			"item_em_ponto":
				# {"objeto", "t", "ponto", "altura_entre": [a, b]}: an item sits on that point
				# of the block's Blender element (the part it works on, the load on its forks
				# or deck), and the point's height.
				if want.has("t"):
					_main.seek(float(want["t"]), true)
				var cena: CenaUtsim = _main.cena
				var i: int = cena.block_index(String(want["objeto"]))
				var n: int = cena.items_at_point(i, String(want["ponto"]))
				report.call("item em %s de %s" % [want["ponto"], want["objeto"]], n > 0, "%d item(ns)" % n)
				if want.has("altura_entre"):
					# The point itself (the forks past the clip's travel reach the cell).
					var h: float = cena.point_height(i, String(want["ponto"]))
					report.call("altura de %s de %s" % [want["ponto"], want["objeto"]], h >= float(want["altura_entre"][0]) and h <= float(want["altura_entre"][1]), "%.2f m" % h)
			"pessoa3d":
				# {"objeto", "corpo_blender": true, "epis": [...], "animada": true}: a person
				# drawn with a Blender body, the protective equipment shown, and a clip playing.
				var cena: CenaUtsim = _main.cena
				var i: int = cena.block_index(String(want["objeto"]))
				report.call("bloco %s" % want["objeto"], i >= 0, "")
				if i >= 0:
					var rig: Dictionary = cena.rigs[i]
					if want.has("corpo_blender"):
						report.call("corpo do Blender em %s" % want["objeto"], rig.has("pessoa") == bool(want["corpo_blender"]), str(rig.get("pessoa", {})))
					if want.has("epis") and rig.has("asset"):
						var shown := Pessoa.epis_visiveis(rig["asset"])
						report.call("EPI de %s" % want["objeto"], Array(shown) == Array(want["epis"]), str(shown))
					if want.get("animada", false):
						var clip: String = (rig["person"] as Pessoa).clip_name() if rig.has("person") else ""
						report.call("%s animada" % want["objeto"], clip != "", clip)
			"camera_nos_olhos":
				var cena: CenaUtsim = _main.cena
				var eye = cena.driver_eye(cena.block_index(String(want)))
				report.call("a câmera nos olhos do motorista de %s" % want, eye != null and (eye as Vector3).distance_to(_main.camera.global_position) < 0.02, str(eye))
			"camera_livre":
				var cam: Camera3D = _main.camera
				var eye := cam.global_position
				var near: Vector3 = eye + (_main.camera_aim - eye).normalized() * float(want.get("distancia", 2.0))
				var hit: PackedStringArray = _main.cena.meshes_blocking(eye, near, float(want.get("margem", 0.5)))
				report.call("nada entre a câmera e o alvo", hit.is_empty(), ", ".join(hit))
			"rotulos_no_maximo":
				var n: int = _main.cena.labels_shown()
				report.call("rótulos desenhados", n <= int(want), str(n))
			"replay":
				report.call("replay", _main.playing_replay == bool(want), str(_main.playing_replay))
			"dados":
				# {campo, fonte: "dados" | "dist", familia, n_maior_que, defasado, selo}
				var entry: Dictionary = doc.data_entry(want["campo"])
				var uses := not entry.is_empty()
				if want.has("fonte"):
					report.call("fonte de %s" % want["campo"], uses == (want["fonte"] == "dados"), "usa dados: %s" % uses)
				if want.has("familia"):
					report.call("família de %s" % want["campo"], String(entry.get("family", "")) == String(want["familia"]), str(entry.get("family", "(sem dados)")))
				if want.has("familia_contem"):
					report.call("família de %s contém %s" % [want["campo"], want["familia_contem"]], String(entry.get("family", "")).contains(String(want["familia_contem"])), str(entry.get("family", "(sem dados)")))
				if want.has("n_maior_que"):
					report.call("n de %s" % want["campo"], int(entry.get("n", 0)) > int(want["n_maior_que"]), str(entry.get("n", 0)))
				if want.has("defasado"):
					report.call("dados defasados de %s" % want["campo"], bool(entry.get("stale", false)) == bool(want["defasado"]), str(entry.get("stale", false)))
				if want.has("analisado"):
					report.call("análise de %s" % want["campo"], (typeof(entry.get("view")) == TYPE_DICTIONARY) == bool(want["analisado"]), "")
			"grafico_dados":
				# The egui mini histogram records "barras=<n> curva=<m>".
				var info: Dictionary = _egui().widget_info(String(want["controle"])) if _egui() != null else {}
				var parts := String(info.get("text", "")).split(" ")
				var bars := int(parts[0].get_slice("=", 1)) if parts.size() > 1 else -1
				var curve := int(parts[1].get_slice("=", 1)) if parts.size() > 1 else 0
				report.call("barras de %s" % want["controle"], bars > int(want.get("barras_maior_que", 0)), str(bars))
				if want.get("curva", false):
					report.call("curva de %s" % want["controle"], curve > 1, str(curve))
			"pode_desfazer":
				report.call("pode desfazer", doc.can_undo() == bool(want), str(doc.can_undo()))
			"pode_refazer":
				report.call("pode refazer", doc.can_redo() == bool(want), str(doc.can_redo()))
			_:
				report.call("conferência desconhecida %s" % key, false, "")
	if state["ok"]:
		_say("passo %d: conferido %s" % [k, c.keys()])

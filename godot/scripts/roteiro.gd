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
##      (model plan coordinates)
##    - `{"abrir": "../../examples/agvs.ron"}`, `{"novo": true}`, `{"modo": "simular"}`
##    - `{"conferir": {...}}`: assertions on the document and the interface (see `_check`).
##    - `{"nota": "texto"}`: written to the log.
##    - `{"esperar_trabalho": 120}` (G3): waits until the run on the bridge's thread ends
##      (at most that many seconds; a failure if it does not). `modo` and the captures of
##      models wait by themselves.
##    - `{"tempo": 120.0}`: jumps the playback to that instant.
##
##    A target (<alvo>) is one of: `{"ponto": [x, y]}` (pixels), `{"controle": "Nome"}`
##    (a control by node name, its centre), `{"chao": [x, y]}` (a floor point in model
##    plan coordinates), `{"objeto": "nome"}` (where a block is drawn),
##    `{"item": ["Lista", 0]}` (an item of an ItemList),
##    `{"ligacao": ["de", "para"]}` (the middle of a connection's line) or
##    `{"porta_saida": "nome"}` / `{"porta_entrada": "nome"}` (a block's ports),
##    `{"no": "nome"}` (a node of the path network), `{"trecho": ["de", "para"]}` (the
##    middle of an edge) or `{"aba": ["Abas", "Título"]}` (a tab of a TabContainer).
extends Node

var _out := ""
var _base := ""
var _log := PackedStringArray()
var _failed := false
var _main: Node
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
	if c == null:
		return
	var scroll := _scroll_of(c)
	if scroll != null:
		scroll.ensure_control_visible(c)
		await _frames(3)


func _step(s: Dictionary, k: int) -> void:
	for key in ["clicar", "mover", "em"]:
		if s.has(key):
			await _reveal(s[key])
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
		_main.camera.focus(Vector3(float(at[0]), 0, -float(at[1])), float(c.get("distancia", 20.0)), c.get("giro"), c.get("inclinacao"))
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
	elif s.has("esperar_trabalho"):
		if not await _wait_idle(float(s["esperar_trabalho"])):
			_fail("passo %d: o trabalho não terminou em %s s" % [k, s["esperar_trabalho"]])
		await _frames(3)
	elif s.has("tempo"):
		_main.seek(float(s["tempo"]), true)
		await _frames(3)
	elif s.has("conferir"):
		_check(s["conferir"], k)
	else:
		_fail("passo %d desconhecido: %s" % [k, s])


func _control(name: String) -> Control:
	if name == "":
		return null
	var n := get_tree().root.find_child(name, true, false)
	return n as Control


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
			return null
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
	if d.has("aba"):
		var tabs := _control(String(d["aba"][0])) as TabContainer
		if tabs == null or not tabs.is_visible_in_tree():
			return null
		var bar := tabs.get_tab_bar()
		for i in tabs.get_tab_count():
			if tabs.get_tab_title(i) == String(d["aba"][1]):
				return bar.get_global_transform() * bar.get_tab_rect(i).get_center()
		return null
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
	for name in ["Dialogo_Salvar", "Dialogo_Abrir", "Dialogo_Replay"]:
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
## `ativo` ({objeto, valor}: `visual.asset`, "" for the default look), `miniatura` (the
## asset the miniature shows) and `foco` (name of the control with the keyboard focus).
##
## G3: `rede` ({nos, trechos, controlados}), `no_posicao` ({no, x, y, tolerancia}), `texto` ({controle, contem}: the text of a
## label, rich text or field), `visivel` / `oculto` (control names), `replicacoes`
## (replications of the results shown), `execucao` ({campo, maior_que}: a run setting of
## the document), `ocupado` (a run on the bridge's thread), `grafico` ({controle,
## maior_que}: things drawn by a `Grafico`), `fluidos` ({nos, tanques, nivel_entre: [a,
## b], altura_no_maximo: metres of the network as drawn, enquadrados: the whole network
## on screen}), `fluxos` ({atividades_maior_que, tokens_pico_maior_que, legivel: boxes
## apart, text whole, zoom legible}), `pert` ({modo, caixas_maior_que, tem_histograma}),
## `critico` (names outlined in red), `camera` (mode), `camera_distancia` ([a, b]: the orbit distance), `camera_livre` ({margem,
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
				report.call("miniatura", shown == want, shown)
			"foco":
				var owner := get_viewport().gui_get_focus_owner()
				var focused: String = String(owner.name) if owner != null else ""
				report.call("foco", focused == want, focused)
			"ligando":
				report.call("ligando", _main.editor.connect_from == want, _main.editor.connect_from)
			"status_contem":
				var status: String = _main._status.text
				report.call("status contém %s" % want, status.contains(want), status)
			"reproduzindo":
				report.call("reproduzindo", _main.playing == bool(want), str(_main.playing))
			"tempo_maior_que":
				report.call("tempo > %s" % want, _main.t > float(want), str(_main.t))
			"rede":
				var got: Dictionary = _main.editor.rede.counts()
				for key2 in want:
					report.call("rede.%s" % key2, int(got.get(key2, -1)) == int(want[key2]), str(got))
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
				report.call("texto de %s contém %s" % [want["controle"], want["contem"]], shown.contains(String(want["contem"])), shown.substr(0, 300))
			"visivel":
				for n in want:
					var ctl := _control(String(n))
					report.call("%s visível" % n, ctl != null and ctl.is_visible_in_tree(), "")
			"oculto":
				for n in want:
					var ctl := _control(String(n))
					report.call("%s oculto" % n, ctl == null or not ctl.is_visible_in_tree(), "")
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
				var g := _control(String(want["controle"])) as Grafico
				var n := g.count() if g != null else -1
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
				var panel: PainelFluxos = _main._flows_panel
				var acts := 0
				if not panel.flows.is_empty():
					acts = (panel.flows[panel.flow]["nodes"] as Array).size()
				report.call("atividades do fluxo", acts > int(want.get("atividades_maior_que", 0)), str(acts))
				if want.has("tokens_pico_maior_que"):
					report.call("pico de tokens", panel.peak_tokens() > int(want["tokens_pico_maior_que"]), str(panel.peak_tokens()))
				if want.get("legivel", false):
					var graph: PainelFluxos.Grafo = panel._graph
					report.call("caixas do fluxo sem sobreposição", graph.overlaps() == 0, str(graph.overlaps()))
					report.call("texto do fluxo sem corte", graph.clipped() == 0, str(graph.clipped()))
					report.call("zoom do fluxo legível", graph.zoom >= PainelFluxos.Grafo.MIN_FIT - 1e-3, str(graph.zoom))
			"pert":
				var panel: PainelPert = _main._pert_panel
				var d: Dictionary = panel.data
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
				var dd: float = _main.camera.dist
				report.call("distância da câmera entre %s e %s" % [want[0], want[1]], dd >= float(want[0]) and dd <= float(want[1]), str(dd))
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
			"pode_desfazer":
				report.call("pode desfazer", doc.can_undo() == bool(want), str(doc.can_undo()))
			"pode_refazer":
				report.call("pode refazer", doc.can_redo() == bool(want), str(doc.can_redo()))
			_:
				report.call("conferência desconhecida %s" % key, false, "")
	if state["ok"]:
		_say("passo %d: conferido %s" % [k, c.keys()])

## Scripted capture for visual QA (autoload). Does nothing unless the project is started
## with the user argument `--utsim-roteiro=<file.json>`:
##
##   godot --path godot -- --utsim-roteiro=roteiros/capturas.json --utsim-capturas=<pasta>
##
## The script lists models; for each one it opens the model, runs the replications, then
## for each capture jumps to `t`, optionally aims the camera (`focar`, the same numbers
## as the `Focar` step of the Bevy viewer's scripts) and saves the viewport as
## `<pasta>/<nome>.png`. A `medir` entry plays for some seconds and writes the frame
## times to `<pasta>/log.txt`. Paths are relative to the script file. Quits at the end
## (exit code 1 on any failure).
##
## {
##   "modelos": [
##     {"modelo": "../../examples/agvs.ron", "replicacoes": 1,
##      "capturas": [
##        {"nome": "agvs_01", "t": 30.0},
##        {"nome": "agvs_02", "t": 36.0,
##         "focar": {"objeto": "estacao_1", "distancia": 7.0, "giro": 0.5, "inclinacao": 0.4}}],
##      "medir": {"t": 0.0, "segundos": 4.0, "velocidade": 10.0}}
##   ]
## }
extends Node

var _out := ""
var _log := PackedStringArray()
var _failed := false


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
	if path.is_absolute_path():
		return path
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
	var base := script_path.get_base_dir()
	if _out == "":
		_out = _resolve(base, data.get("saida", "capturas"))
	DirAccess.make_dir_recursive_absolute(_out)
	await _frames(5)
	var main := get_tree().current_scene
	if main == null or not main.has_method("open_model"):
		_fail("a cena principal não está carregada")
		get_tree().quit(1)
		return
	for m in data.get("modelos", []):
		var path := _resolve(base, m["modelo"])
		_say("modelo %s" % path)
		if not main.open_model(path):
			_fail("modelo inválido: %s" % path)
			continue
		if not main.simulate(int(m.get("replicacoes", 1))):
			_fail("simulação falhou: %s" % path)
			continue
		# Let the imported scenes settle (shaders, shadows).
		await _frames(20)
		for c in m.get("capturas", []):
			main.seek(float(c.get("t", 0.0)), true)
			if c.has("focar"):
				var f: Dictionary = c["focar"]
				if not main.focus_block(f["objeto"], float(f.get("distancia", 8.0)), f.get("giro"), f.get("inclinacao")):
					_fail("objeto não encontrado: %s" % f["objeto"])
			# Apply once more so carried loads follow the hands of the posed skeletons.
			await _frames(2)
			main.seek(float(c.get("t", 0.0)), true)
			await _frames(6)
			await RenderingServer.frame_post_draw
			var img := get_viewport().get_texture().get_image()
			var file := _out.path_join(String(c["nome"]) + ".png")
			if img == null or img.save_png(file) != OK:
				_fail("não salvou %s" % file)
			else:
				_say("captura %s (t = %.2f)" % [file, float(c.get("t", 0.0))])
		if m.has("medir"):
			await _measure(main, m["medir"], path.get_file())
	var log := FileAccess.open(_out.path_join("log.txt"), FileAccess.WRITE)
	if log != null:
		log.store_string("\n".join(_log) + "\n")
	get_tree().quit(1 if _failed else 0)


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

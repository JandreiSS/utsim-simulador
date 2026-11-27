## Headless check of the Godot interface (scripts/godot_teste.sh):
##
##   godot --headless --path godot --script res://testes/teste.gd -- --utsim-modelo=<model.ron> ...
##
## For each model: the extension is loaded (the UtsimModel and UtsimPlayback classes
## exist), the model loads and validates, two replications run (also on a thread of the
## bridge, `UtsimJob`, giving the table of the results panel), the replay of replication
## 0 builds the 3D scene (library models, people, items) and is played from 0 to its end
## in 400 steps. Prints the mean cost per frame (bridge and scene) and exits with 1 on
## any failure.
extends SceneTree

var _failed := false


func _fail(text: String) -> void:
	_failed = true
	printerr("FALHOU: ", text)


func _initialize() -> void:
	_run.call_deferred()


func _run() -> void:
	var models := PackedStringArray()
	for a in OS.get_cmdline_user_args():
		if a.begins_with("--utsim-modelo="):
			models.append(a.substr("--utsim-modelo=".length()))
	if models.is_empty():
		_fail("nenhum modelo (use --utsim-modelo=<arquivo.ron>)")
		quit(1)
		return
	for cls in [&"UtsimModel", &"UtsimPlayback"]:
		if not ClassDB.class_exists(cls):
			_fail("a extensão não registrou %s (godot/bin/ tem a biblioteca?)" % cls)
			quit(1)
			return
	print("extensão carregada: UtsimModel e UtsimPlayback")
	print("boneco animado importado: %s" % CenaUtsim.skeletal_available())
	for path in models:
		_check(path)
	print("Godot: %s" % ("FALHOU" if _failed else "tudo certo"))
	quit(1 if _failed else 0)


func _check(path: String) -> void:
	var model: Object = ClassDB.instantiate(&"UtsimModel")
	var r: Dictionary = model.load(path)
	if not r["ok"]:
		_fail("%s: %s" % [path, r["errors"]])
		return
	var run: Dictionary = model.run(2)
	if not run["ok"] or (run["metrics"] as Dictionary).is_empty():
		_fail("%s: replicações: %s" % [path, run.get("error", "sem métricas")])
		return
	# G3: the same run on a thread of the bridge, and the results panel's numbers.
	var job: Object = model.start_run(2)
	if job == null or not job.wait():
		_fail("%s: replicações em thread: %s" % [path, job.error() if job != null else "sem tarefa"])
		return
	var res: Object = job.results()
	if res == null or (res.table() as Array).is_empty() or res.replications() != 2:
		_fail("%s: resultados da thread vazios" % path)
		return
	var pb: Object = model.playback(0, CenaUtsim.skeletal_available())
	if pb == null:
		_fail("%s: replay: %s" % [path, model.last_error()])
		return
	var cena := CenaUtsim.new()
	root.add_child(cena)
	cena.build(pb)
	var end: float = pb.end()
	if end <= 0.0 or cena.blocks.is_empty():
		_fail("%s: cena vazia (fim %f)" % [path, end])
		return
	var people := 0
	var realistic := 0
	for i in cena.rigs.size():
		var rig: Dictionary = cena.rigs[i]
		if rig.has("person"):
			people += 1
		elif cena.blocks[i].get("skeletal", false):
			_fail("%s: %s deveria ser uma pessoa animada" % [path, cena.blocks[i]["name"]])
		if rig.has("asset"):
			realistic += 1
	var steps := 400
	var bridge := 0
	var apply := 0
	var max_items := 0
	var moved := false
	var first := {}
	for k in steps + 1:
		var t := end * k / steps
		cena.apply(t, 1.0 / 60.0)
		bridge += cena.last_bridge_us
		apply += cena.last_apply_us
		max_items = maxi(max_items, cena.items.size())
		for i in cena.mobiles:
			var p: Vector3 = (cena.rigs[i]["root"] as Node3D).position
			if not first.has(i):
				first[i] = p
			elif p.distance_to(first[i]) > 0.5:
				moved = true
	if not cena.mobiles.is_empty() and not moved:
		_fail("%s: nenhum recurso móvel saiu do lugar" % path)
	if max_items == 0:
		_fail("%s: nenhum item apareceu" % path)
	print("%s: %d blocos (%d com modelo, %d pessoas animadas, %d móveis), até %d itens, fim %.1f" % [
		path.get_file(), cena.blocks.size(), realistic, people, cena.mobiles.size(), max_items, end])
	print("  custo médio por quadro: ponte %.3f ms, cena %.3f ms (%d quadros, depuração)" % [
		bridge / 1000.0 / (steps + 1), apply / 1000.0 / (steps + 1), steps + 1])
	# QS6: a camada de segurança e qualidade liga, desenha e acompanha a reprodução.
	cena.camada = CamadaSeguranca.criar(cena, model)
	if cena.camada == null:
		_fail("%s: a camada de segurança não foi criada" % path)
	else:
		cena.camada.ligar(true)
		cena.camada.ligar_calor(true)
		for k in 21:
			cena.apply(end * k / 20.0, 1.0 / 60.0)
		print("  camada de segurança: %d quase-conflitos, %d pessoas" % [
			cena.camada.lista_quase().size(), cena.camada.seg.people().size()])
	# AM13: a camada Ambiental, só nos modelos com bloco ambiental. O medidor mostra a mesma
	# potência que a ponte (a curva de carga do UTSim Ambiental) e a soma das colunas.
	var tr := func(chave: String, args: Array) -> String: return "%s %s" % [chave, args]
	cena.camada_ambiental = CamadaAmbiental.criar(cena, model, tr)
	if cena.camada_ambiental != null:
		var ca := cena.camada_ambiental
		ca.ligar(true)
		var vistas := 0
		for k in 41:
			var t := end * k / 40.0
			cena.apply(t, 1.0 / 60.0)
			var m: Dictionary = ca.amb.meter(t)
			if not m["in_window"]:
				continue
			var soma := 0.0
			for x in ca.amb.powers(t):
				soma += x
			if absf(soma - float(m["kw"])) > 1e-6:
				_fail("%s: soma das colunas %f != medidor %f em t=%f" % [path, soma, m["kw"], t])
			if not ca.texto_medidor().contains("%.1f" % float(m["kw"])):
				_fail("%s: o medidor não mostra %.1f kW: %s" % [path, m["kw"], ca.texto_medidor()])
			vistas += 1
		var refugo: PackedInt64Array = ca.amb.scrap_items()
		if vistas == 0 or refugo.is_empty() or not ca.abrir_cartao(int(refugo[0])):
			_fail("%s: camada Ambiental sem instantes (%d) ou sem mochila (%d peças)" % [path, vistas, refugo.size()])
		print("  camada Ambiental: %d nós, %d peças refugadas, %d instantes conferidos" % [
			ca.amb.nodes().size(), refugo.size(), vistas])
	cena.free()

extends Node
## Modo missão do UTSim Ensino dentro do cliente 3D (plano do ensino, seção 5.4).
## O nó UtsimEnsino da ponte hospeda as mesmas telas da janela do Ensino.
## Argumentos: --utsim-pacote=<pasta|pacote.utse> e --utsim-missao=<id>.

const PRINCIPAL := "res://cenas/principal.tscn"


func _ready() -> void:
	# A cena principal não volta a pedir o modo missão ao recarregar.
	Engine.set_meta(&"utsim_ensino_aberto", true)
	var ensino: Node = ClassDB.instantiate(&"UtsimEnsino")
	if ensino == null:
		push_error("A ponte utsim_godot não tem o UtsimEnsino.")
		get_tree().change_scene_to_file(PRINCIPAL)
		return
	$Camada.add_child(ensino)
	ensino.connect(&"sair", _voltar)
	for a in OS.get_cmdline_user_args():
		var arg := String(a)
		if arg.begins_with("--utsim-pacote="):
			var erro: String = ensino.abrir_pacote(arg.trim_prefix("--utsim-pacote="))
			if erro != "":
				push_warning("Pacote não abriu: %s" % erro)
	for a in OS.get_cmdline_user_args():
		var arg := String(a)
		if arg.begins_with("--utsim-missao="):
			ensino.abrir_missao(arg.trim_prefix("--utsim-missao="))


func _voltar() -> void:
	get_tree().change_scene_to_file(PRINCIPAL)

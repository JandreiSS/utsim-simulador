# Gera o texto de avisos de terceiros do motor Godot (copyright e licenças), lido do próprio
# executável. Chamado por scripts/godot_exportar.sh:
#   godot --headless --path godot --script <este arquivo> -- <arquivo_de_saída>
# Não é a licença do UTSim: o projeto não tem licença definida.
extends SceneTree


func _init() -> void:
	var args := OS.get_cmdline_user_args()
	if args.is_empty():
		push_error("FALHOU: informe o arquivo de saída")
		quit(1)
		return
	var v := Engine.get_version_info()
	var t := ""
	t += "AVISOS DE TERCEIROS DO UTSim 3D (prévia)\n"
	t += "=======================================\n\n"
	t += "O UTSim 3D roda sobre o motor Godot %s (%s). Este arquivo reproduz os avisos de\n" % [v.string, v.hash]
	t += "copyright e as licenças do motor, lidos do próprio executável. Os créditos e as\n"
	t += "licenças dos modelos 3D estão em assets/terceiros/CREDITOS.md e nas pastas\n"
	t += "assets/terceiros/*/LICENCA*.txt. Este arquivo não é a licença do UTSim.\n\n"
	t += "=== Licença do motor Godot ===\n\n"
	t += Engine.get_license_text() + "\n\n"
	t += "=== Licenças de componentes do Godot ===\n\n"
	var licencas := Engine.get_license_info()
	for nome in licencas.keys():
		t += "--- %s ---\n%s\n\n" % [nome, licencas[nome]]
	t += "=== Copyright dos componentes de terceiros do Godot ===\n\n"
	for parte in Engine.get_copyright_info():
		t += "%s\n" % parte.name
		for item in parte.parts:
			t += "  Arquivos: %s\n" % ", ".join(item.files)
			t += "  Copyright: %s\n" % "; ".join(item.copyright)
			t += "  Licença: %s\n" % item.license
		t += "\n"
	var f := FileAccess.open(args[0], FileAccess.WRITE)
	if f == null:
		push_error("FALHOU: não consegui gravar %s" % args[0])
		quit(1)
		return
	f.store_string(t)
	f.close()
	print("Avisos gravados em %s (%d bytes)" % [args[0], t.length()])
	quit(0)

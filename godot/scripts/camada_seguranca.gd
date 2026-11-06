## Camada "Segurança" da reprodução (etapa QS6 de docs/plano_qualidade_e_seguranca.md).
##
## Só desenha: toda a conta está na ponte (crates/utsim-godot/src/seguranca.rs, classe
## UtsimSeguranca). A camada mostra
##  - as zonas de risco no piso, com a cor do nível e o nome;
##  - o EPI que cada pessoa veste (o próprio mais o do posto que atende), trocado quando
##    ela chega a outro posto, e um contorno vermelho na pessoa que está numa zona sem o
##    EPI que ela pede;
##  - marcadores nos quase-conflitos simulados (a lista e o salto ficam no painel);
##  - uma marca vermelha nos itens com defeito e uma lupa nos postos com inspeção;
##  - o mapa de calor de presença e quase-conflitos como textura no piso (liga à parte).
##
## É filha da cena (CenaUtsim) e a cena chama `aplicar(t)` a cada quadro.
class_name CamadaSeguranca
extends Node3D

const NOMES_EPI := {
	"capacete": "capacete", "oculos": "óculos", "protetor_auricular": "protetor auricular",
	"luvas": "luvas", "botina": "botina", "colete": "colete", "mascara": "máscara",
	"avental": "avental",
}
const VERMELHO := Color(0.95, 0.12, 0.10)
const ZONA_ALTURA := 0.02
const MAX_MARCADORES := 300

var seg: Object
var cena: CenaUtsim
var ligada := false
var calor_ligado := false

var _zonas: Node3D
var _marcadores: Node3D
var _lupas: Node3D
var _calor: MeshInstance3D
var _pessoas := {}
var _defeitos := {}
var _quase: Array = []
var _modelos := {}


## Cria a camada de `model` (UtsimModel) na replicação 0 e a pendura em `cena`. Nulo se a
## ponte não calculou (modelo de replay, extensão sem a classe).
static func criar(cena_: CenaUtsim, model: Object, replicacao := 0) -> CamadaSeguranca:
	if not ClassDB.class_exists(&"UtsimSeguranca"):
		return null
	var s: Object = ClassDB.instantiate(&"UtsimSeguranca")
	if not s.build(model, replicacao):
		push_warning("camada de segurança: %s" % s.last_error())
		return null
	var c := CamadaSeguranca.new()
	c.name = "CamadaSeguranca"
	c.seg = s
	c.cena = cena_
	cena_.add_child(c)
	c._montar()
	return c


func _notification(what: int) -> void:
	if what == NOTIFICATION_PREDELETE:
		# Os modelos-base das peças não estão na árvore: liberá-los à mão.
		for m in _modelos.values():
			if m is Node and is_instance_valid(m):
				(m as Node).free()
		_modelos.clear()


func _montar() -> void:
	_zonas = Node3D.new()
	_zonas.name = "Zonas"
	add_child(_zonas)
	_marcadores = Node3D.new()
	_marcadores.name = "QuaseConflitos"
	add_child(_marcadores)
	_lupas = Node3D.new()
	_lupas.name = "Inspecoes"
	add_child(_lupas)
	for z in seg.zones():
		_zonas.add_child(_zona(z))
	_quase = seg.near_misses()
	for k in mini(_quase.size(), MAX_MARCADORES):
		_marcadores.add_child(_marcador(_quase[k]))
	for nome in seg.inspections():
		_lupa(nome)
	for id in seg.defective_items():
		_defeitos[int(id)] = true
	_calor = _mapa_de_calor(seg.heat())
	if _calor != null:
		add_child(_calor)
	_ajustar()


## Liga ou desliga a camada (zonas, EPI, marcadores, itens com defeito, lupas).
func ligar(on: bool) -> void:
	ligada = on
	_ajustar()


func ligar_calor(on: bool) -> void:
	calor_ligado = on
	_ajustar()


func _ajustar() -> void:
	_zonas.visible = ligada
	_marcadores.visible = ligada
	_lupas.visible = ligada
	if _calor != null:
		_calor.visible = calor_ligado
	for nome in _pessoas:
		var p: Dictionary = _pessoas[nome]
		(p["epi"] as Node3D).visible = ligada
		(p["alerta"] as Node3D).visible = ligada and p["fora_de_regra"]


## Os quase-conflitos para a lista do painel: `[[instante, texto], ...]`.
func lista_quase() -> Array:
	var out := []
	for q in _quase:
		out.append([q["t"], q["text"]])
	return out


# ---------------------------------------------------------------------------
# A cada quadro
# ---------------------------------------------------------------------------


func aplicar(t: float) -> void:
	if ligada:
		_vestir(t)
		_itens_com_defeito()
		_girar_lupas(t)
		_pulsar(t)


func _vestir(t: float) -> void:
	for s in seg.status(t):
		var nome: String = s["name"]
		var i := cena.block_index(nome)
		if i < 0:
			continue
		var p: Dictionary = _pessoas.get(nome, {})
		if p.is_empty():
			p = _nova_pessoa(i)
			_pessoas[nome] = p
		var chave := ",".join(s["ppe"])
		if chave != p["chave"]:
			p["chave"] = chave
			_trocar_epi(p, i, s["ppe"])
		var fora: bool = s["zone"] != ""
		var texto := ""
		if fora:
			var faltas := []
			for e in s["missing"]:
				faltas.append(NOMES_EPI.get(e, e))
			texto = "sem EPI em %s: %s" % [s["zone"], ", ".join(faltas)]
		if fora != p["fora_de_regra"] or texto != p["texto"]:
			p["fora_de_regra"] = fora
			p["texto"] = texto
			(p["alerta"] as Node3D).visible = fora
			(p["rotulo"] as Label3D).text = texto


func _nova_pessoa(i: int) -> Dictionary:
	var rig: Dictionary = cena.rigs[i]
	var root: Node3D = rig["root"]
	var epi := Node3D.new()
	epi.name = "EPI"
	root.add_child(epi)
	var alerta := Node3D.new()
	alerta.name = "AlertaSemEPI"
	alerta.visible = false
	alerta.add_child(_anel(0.5, 0.68, VERMELHO))
	var rotulo := Label3D.new()
	rotulo.billboard = BaseMaterial3D.BILLBOARD_ENABLED
	rotulo.fixed_size = true
	rotulo.no_depth_test = true
	rotulo.pixel_size = 0.0006
	rotulo.font_size = 24
	rotulo.modulate = VERMELHO
	rotulo.outline_size = 8
	rotulo.outline_modulate = Color(0.1, 0.0, 0.0)
	rotulo.position = Vector3(0, float(rig.get("top", 1.85)) + 0.7, 0)
	alerta.add_child(rotulo)
	root.add_child(alerta)
	return {"epi": epi, "alerta": alerta, "rotulo": rotulo, "chave": "\u0000",
		"fora_de_regra": false, "texto": "", "nos": []}


## Tira as peças antigas e veste as de `ppe` (nomes de EPI).
func _trocar_epi(p: Dictionary, i: int, ppe: PackedStringArray) -> void:
	for n in p["nos"]:
		if is_instance_valid(n):
			(n as Node).queue_free()
	p["nos"] = []
	var rig: Dictionary = cena.rigs[i]
	for nome in ppe:
		if rig.has("person"):
			_vestir_manequim(p, rig, nome)
		elif rig.has("asset") and (rig["asset"] as Node).find_child("tronco", true, false) != null:
			_vestir_simples(p, rig["asset"], nome)


func _peca(nome: String, osso: bool) -> Node3D:
	var chave := "%s/%s" % [nome, osso]
	if not _modelos.has(chave):
		var bytes: PackedByteArray = seg.epi_glb(nome, osso)
		_modelos[chave] = cena._from_gltf(bytes, "") if not bytes.is_empty() else null
	var base = _modelos[chave]
	return (base as Node3D).duplicate() as Node3D if base != null else null


## Operador simples: cada nó da peça passa para o nó de mesmo nome do operador (mesmo pivô,
## então a peça acompanha o braço, a perna e a cabeça).
func _vestir_simples(p: Dictionary, asset: Node, nome: String) -> void:
	var peca := _peca(nome, false)
	if peca == null:
		return
	for no_nome in seg.epi_nodes(nome):
		var origem := peca.find_child(no_nome, true, false) as Node3D
		var destino := asset.find_child(no_nome, true, false) as Node3D
		if origem == null or destino == null:
			continue
		origem.get_parent().remove_child(origem)
		origem.transform = Transform3D.IDENTITY
		destino.add_child(origem)
		p["nos"].append(origem)
	peca.free()


## Manequim esquelético: como a roupa de Pessoa._dress, presa a ossos. Cabeça e tronco
## levam a peça da biblioteca (girada para a frente do manequim, +Z do arquivo); luvas e
## botina levam a versão centrada, no osso da mão e do pé.
func _vestir_manequim(p: Dictionary, rig: Dictionary, nome: String) -> void:
	var pessoa: Pessoa = rig["person"]
	var skel := pessoa.skeleton
	var asset: Node3D = rig["asset"]
	var para_osso := nome == "luvas" or nome == "botina"
	var ossos: Array
	match nome:
		"luvas":
			ossos = ["hand_l", "hand_r"]
		"botina":
			ossos = ["foot_l", "foot_r"]
		"colete", "avental":
			ossos = ["spine_02"]
		_:
			ossos = ["Head"]
	var para_esqueleto := Pessoa._local_to(asset, skel)
	for osso in ossos:
		var b := skel.find_bone(osso)
		if b < 0:
			continue
		var peca := _peca(nome, para_osso)
		if peca == null:
			continue
		var holder := BoneAttachment3D.new()
		holder.bone_name = skel.get_bone_name(b)
		skel.add_child(holder)
		if para_osso:
			holder.add_child(peca)
		else:
			var rest := para_esqueleto * skel.get_bone_global_rest(b)
			peca.transform = rest.affine_inverse() * Transform3D(Basis(Vector3.UP, -PI / 2.0), Vector3.ZERO)
			holder.add_child(peca)
		p["nos"].append(holder)


# ---------------------------------------------------------------------------
# Zonas, marcadores, lupas, itens e calor
# ---------------------------------------------------------------------------


func _material(cor: Color, alfa := 1.0) -> StandardMaterial3D:
	var m := StandardMaterial3D.new()
	m.shading_mode = BaseMaterial3D.SHADING_MODE_UNSHADED
	m.albedo_color = Color(cor.r, cor.g, cor.b, alfa)
	if alfa < 1.0:
		m.transparency = BaseMaterial3D.TRANSPARENCY_ALPHA
	m.cull_mode = BaseMaterial3D.CULL_DISABLED
	return m


func _zona(z: Dictionary) -> Node3D:
	var raiz := Node3D.new()
	raiz.name = "Zona_" + String(z["name"])
	var cor: Color = z["color"]
	var pts: PackedVector3Array = z["points"]
	var plano := PackedVector2Array()
	for v in pts:
		plano.append(Vector2(v.x, v.z))
	var idx := Geometry2D.triangulate_polygon(plano)
	if idx.is_empty():
		# Polígono que o triangulador recusa: leque a partir do primeiro vértice.
		for k in range(1, pts.size() - 1):
			idx.append_array(PackedInt32Array([0, k, k + 1]))
	var verts := PackedVector3Array()
	for v in pts:
		verts.append(Vector3(v.x, ZONA_ALTURA, v.z))
	var arrays := []
	arrays.resize(Mesh.ARRAY_MAX)
	arrays[Mesh.ARRAY_VERTEX] = verts
	arrays[Mesh.ARRAY_INDEX] = idx
	var mesh := ArrayMesh.new()
	mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, arrays)
	var piso := MeshInstance3D.new()
	piso.mesh = mesh
	piso.material_override = _material(cor, 0.32)
	piso.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	raiz.add_child(piso)
	var borda := ImmediateMesh.new()
	borda.surface_begin(Mesh.PRIMITIVE_LINE_STRIP)
	for v in pts:
		borda.surface_add_vertex(Vector3(v.x, ZONA_ALTURA + 0.005, v.z))
	borda.surface_add_vertex(Vector3(pts[0].x, ZONA_ALTURA + 0.005, pts[0].z))
	borda.surface_end()
	var contorno := MeshInstance3D.new()
	contorno.mesh = borda
	contorno.material_override = _material(cor, 1.0)
	contorno.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	raiz.add_child(contorno)
	var nome := Label3D.new()
	var nivel := int(z["level"])
	nome.text = "%s (%s%s)" % [z["name"], z["kind"], ", nível %d" % nivel if nivel > 0 else ""]
	nome.billboard = BaseMaterial3D.BILLBOARD_ENABLED
	nome.fixed_size = true
	nome.no_depth_test = true
	nome.pixel_size = 0.0006
	nome.font_size = 24
	nome.modulate = cor.lightened(0.35)
	nome.outline_size = 8
	nome.outline_modulate = Color(0.05, 0.05, 0.05)
	nome.position = z["center"] + Vector3(0, 0.5, 0)
	raiz.add_child(nome)
	return raiz


func _anel(r0: float, r1: float, cor: Color) -> MeshInstance3D:
	var im := ImmediateMesh.new()
	im.surface_begin(Mesh.PRIMITIVE_TRIANGLES)
	var passos := 32
	for k in passos:
		var a0 := TAU * k / passos
		var a1 := TAU * (k + 1) / passos
		var p := [Vector3(cos(a0) * r0, 0, sin(a0) * r0), Vector3(cos(a0) * r1, 0, sin(a0) * r1),
			Vector3(cos(a1) * r1, 0, sin(a1) * r1), Vector3(cos(a1) * r0, 0, sin(a1) * r0)]
		for j in [0, 1, 2, 0, 2, 3]:
			im.surface_add_vertex(p[j])
	im.surface_end()
	var mi := MeshInstance3D.new()
	mi.mesh = im
	mi.material_override = _material(cor)
	mi.position = Vector3(0, 0.04, 0)
	mi.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	return mi


func _marcador(q: Dictionary) -> Node3D:
	var n := Node3D.new()
	n.name = "QuaseConflito"
	n.position = q["position"]
	n.set_meta("t", float(q["t"]))
	n.set_meta("fim", float(q["t"]) + float(q["duration"]))
	var cone := CylinderMesh.new()
	cone.top_radius = 0.0
	cone.bottom_radius = 0.28
	cone.height = 0.7
	var mi := MeshInstance3D.new()
	mi.mesh = cone
	mi.material_override = _material(Color(1.0, 0.45, 0.05), 0.85)
	mi.rotation = Vector3(PI, 0, 0)
	mi.position = Vector3(0, 0.85, 0)
	mi.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	n.add_child(mi)
	n.add_child(_anel(0.3, 0.42, Color(1.0, 0.45, 0.05)))
	return n


## O marcador pulsa (cresce) enquanto o quase-conflito acontece em `t`.
func _pulsar(t: float) -> void:
	for m in _marcadores.get_children():
		var agora: bool = t >= m.get_meta("t") - 1.0 and t <= m.get_meta("fim") + 3.0
		(m as Node3D).scale = Vector3.ONE * (1.6 if agora else 1.0)


## A lupa da inspeção, girando sobre o posto.
func _lupa(nome: String) -> void:
	var i := cena.block_index(nome)
	if i < 0:
		return
	var rig: Dictionary = cena.rigs[i]
	var raiz := Node3D.new()
	raiz.name = "Lupa_" + nome
	raiz.position = (rig["root"] as Node3D).position + Vector3(0, float(rig.get("top", 1.5)) + 1.0, 0)
	var cor := Color(0.2, 0.75, 1.0)
	var aro := TorusMesh.new()
	aro.inner_radius = 0.1
	aro.outer_radius = 0.15
	var a := MeshInstance3D.new()
	a.mesh = aro
	a.rotation = Vector3(PI / 2.0, 0, 0)
	a.material_override = _material(cor)
	raiz.add_child(a)
	var cabo := CylinderMesh.new()
	cabo.top_radius = 0.03
	cabo.bottom_radius = 0.03
	cabo.height = 0.22
	var c := MeshInstance3D.new()
	c.mesh = cabo
	c.rotation = Vector3(0, 0, PI / 4.0)
	c.position = Vector3(0.2, -0.2, 0)
	c.material_override = _material(cor)
	raiz.add_child(c)
	for mi in [a, c]:
		(mi as MeshInstance3D).cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	_lupas.add_child(raiz)


func _girar_lupas(t: float) -> void:
	for l in _lupas.get_children():
		(l as Node3D).rotation.y = t * 1.2


## Marca vermelha (caixa translúcida) nos itens vivos que terminam com defeito.
func _itens_com_defeito() -> void:
	if _defeitos.is_empty():
		return
	for id in cena.items:
		if not _defeitos.has(int(id)):
			continue
		var item: Node3D = cena.items[id]
		if item.has_meta("seg_defeito"):
			continue
		item.set_meta("seg_defeito", true)
		var caixa := BoxMesh.new()
		caixa.size = Vector3.ONE * (CenaUtsim.ITEM_SIZE * 1.12)
		var mi := MeshInstance3D.new()
		mi.name = "MarcaDefeito"
		mi.mesh = caixa
		mi.material_override = _material(VERMELHO, 0.55)
		mi.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
		item.add_child(mi)


func _mapa_de_calor(h: Dictionary) -> MeshInstance3D:
	if h.is_empty():
		return null
	var nx := int(h["nx"])
	var ny := int(h["ny"])
	var valores: PackedFloat32Array = h["values"]
	if nx <= 0 or ny <= 0 or valores.size() != nx * ny:
		return null
	var bytes := PackedByteArray()
	bytes.resize(nx * ny * 4)
	# Linha 0 da imagem é o maior y do modelo (o menor z do Godot).
	for iy in ny:
		var linha := ny - 1 - iy
		for ix in nx:
			var v := valores[iy * nx + ix]
			var cor := _cor_calor(v)
			var o := (linha * nx + ix) * 4
			bytes[o] = int(cor.r * 255.0)
			bytes[o + 1] = int(cor.g * 255.0)
			bytes[o + 2] = int(cor.b * 255.0)
			bytes[o + 3] = int(cor.a * 255.0)
	var img := Image.create_from_data(nx, ny, false, Image.FORMAT_RGBA8, bytes)
	var tex := ImageTexture.create_from_image(img)
	var cel: float = h["cell"]
	var origem: Vector3 = h["origin"]
	var plano := PlaneMesh.new()
	plano.size = Vector2(nx * cel, ny * cel)
	var m := StandardMaterial3D.new()
	m.shading_mode = BaseMaterial3D.SHADING_MODE_UNSHADED
	m.transparency = BaseMaterial3D.TRANSPARENCY_ALPHA
	m.albedo_texture = tex
	m.texture_filter = BaseMaterial3D.TEXTURE_FILTER_NEAREST
	var mi := MeshInstance3D.new()
	mi.name = "MapaDeCalor"
	mi.mesh = plano
	mi.material_override = m
	# O canto de menor x e menor y do modelo é (origem.x, -origem.z); o centro do piso fica
	# meio piso para +x e meio piso para -z (maior y).
	mi.position = Vector3(origem.x + nx * cel * 0.5, 0.015, origem.z - ny * cel * 0.5)
	mi.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	return mi


## Do transparente ao amarelo e ao vermelho, conforme o valor de 0 a 1.
static func _cor_calor(v: float) -> Color:
	if v <= 0.001:
		return Color(0, 0, 0, 0)
	var x := clampf(v, 0.0, 1.0)
	return Color(1.0, 1.0 - x, 0.0, 0.25 + 0.5 * x)

## Camada "Ambiental" da reprodução (fase AM13 de docs/plano_ambiental.md do UTSim Ambiental).
##
## Só desenha: toda a conta é a do UTSim Ambiental, pela ponte (crates/utsim-godot/src/
## ambiental.rs, classe UtsimAmbiental). A camada mostra
##  - o brilho no piso em volta de cada máquina com bloco ambiental, na cor da energia dela na
##    janela (escala de um só matiz, do claro ao escuro);
##  - uma coluna translúcida sobre a máquina, com a altura proporcional à potência no instante;
##  - um contorno nas peças que vão para o refugo e, ao clicar numa delas, o cartão da mochila;
##  - o medidor da fábrica no canto: a potência no instante (a mesma da curva de carga do app),
##    o pico da curva e se o instante cai na janela de pico da tarifa.
##
## É filha da cena (CenaUtsim) e a cena chama `aplicar(t)` a cada quadro.
class_name CamadaAmbiental
extends Node3D

const ALTURA_MAX := 3.0
const RAIO_COLUNA := 0.22
const COR_COLUNA := Color(0.35, 0.85, 0.55)
const COR_REFUGO := Color(1.0, 0.62, 0.10)

var amb: Object
var cena: CenaUtsim
var ligada := false
## Tradução dos textos: `tr_(chave, [args])` (o T de principal.gd).
var tr_: Callable

var _nos: Array = []
var _brilhos: Node3D
var _colunas: Node3D
var _barras: Array = []
var _rotulos: Array = []
var _refugo := {}
var _hud: CanvasLayer
var _medidor: Label
var _cartao: PanelContainer
var _cartao_texto: Label
var _item_aberto := -1
var _t := 0.0


## Cria a camada de `model` (UtsimModel) na replicação 0 e a pendura em `cena`. Nulo se o
## modelo não tem bloco ambiental (ou a extensão não tem a classe).
static func criar(cena_: CenaUtsim, model: Object, tradutor: Callable, replicacao := 0) -> CamadaAmbiental:
	if not ClassDB.class_exists(&"UtsimAmbiental"):
		return null
	var a: Object = ClassDB.instantiate(&"UtsimAmbiental")
	if not a.build(model, replicacao):
		return null
	var c := CamadaAmbiental.new()
	c.name = "CamadaAmbiental"
	c.amb = a
	c.cena = cena_
	c.tr_ = tradutor
	cena_.add_child(c)
	c._montar()
	return c


func _T(chave: String, args: Array = []) -> String:
	return tr_.call(chave, args) if tr_.is_valid() else chave


func _montar() -> void:
	_brilhos = Node3D.new()
	_brilhos.name = "Brilhos"
	add_child(_brilhos)
	_colunas = Node3D.new()
	_colunas.name = "Colunas"
	add_child(_colunas)
	_nos = amb.nodes()
	for n in _nos:
		_montar_no(n)
	for id in amb.scrap_items():
		_refugo[int(id)] = true
	_montar_hud()
	_ajustar()


func _montar_no(n: Dictionary) -> void:
	var i := int(n["index"])
	var tem := i >= 0 and i < cena.rigs.size()
	var pos: Vector3 = (cena.rigs[i]["root"] as Node3D).position if tem else Vector3.ZERO
	var tam: Vector3 = cena.blocks[i]["size"] if tem and i < cena.blocks.size() else Vector3.ONE
	var topo := float(cena.rigs[i].get("top", 1.5)) if tem else 1.5
	# O brilho: um disco no piso, um pouco maior que a máquina.
	var disco := CylinderMesh.new()
	var r := maxf(tam.x, tam.z) * 0.75 + 0.5
	disco.top_radius = r
	disco.bottom_radius = r
	disco.height = 0.01
	var brilho := MeshInstance3D.new()
	brilho.name = "Brilho_" + String(n["name"])
	brilho.mesh = disco
	var cor: Color = n["color"]
	var m := _material(cor, 0.6)
	m.emission_enabled = true
	m.emission = cor
	brilho.material_override = m
	brilho.position = Vector3(pos.x, 0.012, pos.z)
	brilho.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	_brilhos.add_child(brilho)
	# A coluna: um cilindro de altura 1, escalado em y a cada quadro.
	var cil := CylinderMesh.new()
	cil.top_radius = RAIO_COLUNA
	cil.bottom_radius = RAIO_COLUNA
	cil.height = 1.0
	var raiz := Node3D.new()
	raiz.name = "Coluna_" + String(n["name"])
	raiz.position = Vector3(pos.x, topo + 0.15, pos.z)
	var barra := MeshInstance3D.new()
	barra.mesh = cil
	barra.material_override = _material(COR_COLUNA, 0.45)
	barra.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	raiz.add_child(barra)
	var rotulo := Label3D.new()
	rotulo.billboard = BaseMaterial3D.BILLBOARD_ENABLED
	rotulo.fixed_size = true
	rotulo.no_depth_test = true
	rotulo.pixel_size = 0.0006
	rotulo.font_size = 22
	rotulo.modulate = COR_COLUNA.lightened(0.4)
	rotulo.outline_size = 8
	rotulo.outline_modulate = Color(0.02, 0.08, 0.04)
	raiz.add_child(rotulo)
	_colunas.add_child(raiz)
	_barras.append(barra)
	_rotulos.append(rotulo)


func _montar_hud() -> void:
	_hud = CanvasLayer.new()
	_hud.name = "MedidorAmbiental"
	_hud.layer = 5
	# Uma tela por cima da vista 3D (a CanvasLayer não segue a posição da camada).
	add_child(_hud)
	var caixa := VBoxContainer.new()
	caixa.anchor_left = 1.0
	caixa.anchor_right = 1.0
	caixa.offset_left = -330.0
	caixa.offset_right = -12.0
	caixa.offset_top = 64.0
	caixa.mouse_filter = Control.MOUSE_FILTER_IGNORE
	_hud.add_child(caixa)
	var painel := _painel()
	painel.mouse_filter = Control.MOUSE_FILTER_IGNORE
	_medidor = Label.new()
	_medidor.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	painel.add_child(_medidor)
	caixa.add_child(painel)
	_cartao = _painel()
	_cartao.visible = false
	_cartao_texto = Label.new()
	_cartao_texto.autowrap_mode = TextServer.AUTOWRAP_WORD_SMART
	_cartao.add_child(_cartao_texto)
	caixa.add_child(_cartao)


func _painel() -> PanelContainer:
	var p := PanelContainer.new()
	var s := StyleBoxFlat.new()
	s.bg_color = Color(0.06, 0.10, 0.08, 0.86)
	s.border_color = COR_COLUNA.darkened(0.2)
	s.set_border_width_all(1)
	s.set_corner_radius_all(6)
	s.set_content_margin_all(10)
	p.add_theme_stylebox_override("panel", s)
	return p


## Liga ou desliga a camada (brilhos, colunas, contornos, medidor e cartão).
func ligar(on: bool) -> void:
	ligada = on
	_ajustar()


func _ajustar() -> void:
	_brilhos.visible = ligada
	_colunas.visible = ligada
	if _hud != null:
		_hud.visible = ligada
	if not ligada:
		_fechar_cartao()
	for id in cena.items:
		var item: Node3D = cena.items[id]
		var marca := item.get_node_or_null("ContornoRefugo") as Node3D
		if marca != null:
			marca.visible = ligada


# ---------------------------------------------------------------------------
# A cada quadro
# ---------------------------------------------------------------------------


func aplicar(t: float) -> void:
	_t = t
	if not ligada:
		return
	var alturas: PackedFloat32Array = amb.heights(t)
	var kw: PackedFloat64Array = amb.powers(t)
	for k in mini(_barras.size(), alturas.size()):
		var h := maxf(alturas[k] * ALTURA_MAX, 0.01)
		var barra: MeshInstance3D = _barras[k]
		barra.scale = Vector3(1.0, h, 1.0)
		barra.position = Vector3(0, h * 0.5, 0)
		var rotulo: Label3D = _rotulos[k]
		rotulo.position = Vector3(0, h + 0.35, 0)
		rotulo.text = _T("godot-gd-ambiental-kw", ["%.1f" % kw[k]])
	_contornar_refugo()
	_medir(t)


func _medir(t: float) -> void:
	var m: Dictionary = amb.meter(t)
	if m.is_empty():
		return
	var linhas := [_T("godot-gd-ambiental-medidor_titulo")]
	if m["in_window"]:
		linhas.append(_T("godot-gd-ambiental-kw", ["%.1f" % float(m["kw"])]))
	else:
		linhas.append(_T("godot-gd-ambiental-medidor_aquecimento"))
	linhas.append(_T("godot-gd-ambiental-medidor_pico", ["%.1f" % float(m["peak_kw"]), "%.1f" % float(m["peak_t"])]))
	if m["tariff"]:
		linhas.append(_T("godot-gd-ambiental-medidor_na_janela_de_pico") if m["in_peak"] else _T("godot-gd-ambiental-medidor_fora_da_janela_de_pico"))
	var co2: String = "%.1f" % float(m["co2e_kg"]) if m["has_co2e"] else _T("godot-gd-ambiental-falta_fator")
	linhas.append(_T("godot-gd-ambiental-medidor_energia", ["%.1f" % float(m["energy_kwh"]), co2]))
	_medidor.text = "\n".join(linhas)


## Contorno nas peças vivas que vão para o refugo (a casca de trás de uma caixa maior).
func _contornar_refugo() -> void:
	if _refugo.is_empty():
		return
	for id in cena.items:
		if not _refugo.has(int(id)):
			continue
		var item: Node3D = cena.items[id]
		if item.has_node("ContornoRefugo"):
			continue
		var caixa := BoxMesh.new()
		caixa.size = Vector3.ONE * CenaUtsim.ITEM_SIZE
		var m := _material(COR_REFUGO)
		m.cull_mode = BaseMaterial3D.CULL_FRONT
		m.grow = true
		m.grow_amount = 0.035
		var mi := MeshInstance3D.new()
		mi.name = "ContornoRefugo"
		mi.mesh = caixa
		mi.material_override = m
		mi.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
		item.add_child(mi)


# ---------------------------------------------------------------------------
# A mochila da peça
# ---------------------------------------------------------------------------


## O clique na cena: abre o cartão da peça refugada sob o raio (`origem`, `direcao`), ou
## fecha o cartão aberto. Verdadeiro quando o clique era para a camada.
func clicar(origem: Vector3, direcao: Vector3) -> bool:
	if not ligada:
		return false
	var melhor := -1
	var melhor_d := INF
	var raio := CenaUtsim.ITEM_SIZE * 0.9
	for id in cena.items:
		if not _refugo.has(int(id)):
			continue
		var item: Node3D = cena.items[id]
		if not item.visible:
			continue
		var p := item.global_position
		var ao_longo := (p - origem).dot(direcao)
		if ao_longo <= 0.0:
			continue
		var d := (origem + direcao * ao_longo).distance_to(p)
		if d < raio and ao_longo < melhor_d:
			melhor = int(id)
			melhor_d = ao_longo
	if melhor < 0:
		if _cartao.visible:
			_fechar_cartao()
			return true
		return false
	abrir_cartao(melhor)
	return true


## Mostra o cartão da mochila da peça `item`; falso se ela não foi refugada na janela.
func abrir_cartao(item: int) -> bool:
	var b: Dictionary = amb.backpack(item)
	if not b.get("found", false):
		return false
	_item_aberto = item
	var linhas := [_T("godot-gd-ambiental-cartao_titulo", [str(item)])]
	linhas.append(_T("godot-gd-ambiental-cartao_caminho", [" → ".join(b["path"])]))
	linhas.append(_T("godot-gd-ambiental-cartao_energia", ["%.2f" % float(b["kwh"])]))
	for par in b["flows"]:
		linhas.append(_T("godot-gd-ambiental-cartao_fluxo", [String(par[0]), "%.3f" % float(par[1])]))
	if b["has_co2e"]:
		linhas.append(_T("godot-gd-ambiental-cartao_co2", ["%.2f" % float(b["co2e_kg"])]))
	else:
		linhas.append(_T("godot-gd-ambiental-cartao_falta", [", ".join(b["missing"])]))
	linhas.append(_T("godot-gd-ambiental-cartao_fechar"))
	_cartao_texto.text = "\n".join(linhas)
	_cartao.visible = true
	return true


func _fechar_cartao() -> void:
	_item_aberto = -1
	if _cartao != null:
		_cartao.visible = false


## O texto do medidor e do cartão (para o teste sem janela).
func texto_medidor() -> String:
	return _medidor.text if _medidor != null else ""


func texto_cartao() -> String:
	return _cartao_texto.text if _cartao != null and _cartao.visible else ""


func _material(cor: Color, alfa := 1.0) -> StandardMaterial3D:
	var m := StandardMaterial3D.new()
	m.shading_mode = BaseMaterial3D.SHADING_MODE_UNSHADED
	m.albedo_color = Color(cor.r, cor.g, cor.b, alfa)
	if alfa < 1.0:
		m.transparency = BaseMaterial3D.TRANSPARENCY_ALPHA
	m.cull_mode = BaseMaterial3D.CULL_DISABLED
	return m

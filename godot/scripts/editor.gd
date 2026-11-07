## The 3D interaction of Editar: tools, selection, moving, connecting and placing from
## the library, drawn over the scene built by `CenaUtsim` from
## `UtsimDocument.edit_scene()`. Every change goes through the document (`principal.gd`
## owns it), so it is validated and undoable; after a change the main scene rebuilds
## the scene and the panels (`after_edit`).
##
## Picking projects the mouse on the floor with a camera ray, without physics bodies: a
## block is hit when the ray crosses the box of its footprint and height. Clicking again
## at the same point walks through the blocks under it. Connections are hit within a
## few pixels of their line on screen.
##
## Connecting: each block that can send items has an output node (a green disc on its
## `+x` side) and each one that can receive has an input node (a blue disc on its `-x`
## side); a source has only the output, a sink only the input. The nodes show on every
## block in Editar and grow under the mouse. Dragging from an output node to another
## block (or to its input node) connects them, with a preview arrow while dragging; from
## an input node to another block (or its output node) connects that block to this one.
## A click on a node selects its connections (again: the next one); a selected connection
## shows a red × in its middle: clicking it (or Del) removes it. The Ligar tool (L) still
## connects by dragging from block to block, or by clicking the origin and then the
## destination. The selection is marked only at the corners, so nothing covers the nodes.
##
## Since step 6 of `docs/plano_interface_rust.md`, the picking and the gestures of Selecionar
## and Ligar are Rust's (`UtsimEditor`, `crates/utsim-godot/src/editor3d.rs`, with unit
## tests): this script hands it the camera, the scene and the mouse, applies the effects it
## answers to the document and draws what it describes. Rede, Corredor, Colocar, the Equipe
## tool and the team lines of Simular stay here.
##
## Selecionar is the tool in use by default: when a model opens, after placing an object,
## after a connection and after Esc in any other tool. A click hits a block by its box or,
## when the box is small on screen, within a few pixels of its centre; dragging a block
## moves the selection and dragging on the floor draws a box that selects what is inside.
##
## Who serves whom (operators): a transporter or dispatcher has an orange service node.
## Dragging from it to a processor makes that resource operate the processor (the
## processor's `operators`); to a dispatcher, the transporter joins its team; to any other
## object, the resource carries that object's items (its `transport`; Shift on a processor
## does this too). The links are the dashed lines; a click on one selects it and its × (or
## Del) removes it.
##
## The path network has its own tool, Rede (N, `rede.gd`), drawn in this same overlay, and
## so do the circulation aisles: the Corredor tool (C, `corredores.gd`), which also draws
## the obstacles, the clearances and the access points when they are shown.
## Objects on the critical path of the PERT/CPM window get a red outline (`critical`).
##
## Teams and requests (`docs/plano_chegadas_e_equipes_3d.md`, E1b and E8b): the lines of
## `UtsimPlayback.team_links()` (dispatcher → member, object → who carries its items, a
## processor → its operators, the repair crew, home and break places) are drawn in both
## modes, dashed or dotted by kind; in Simular they are faint and the ones of the block
## followed light up, the members get a ring in the colour of their state and the one
## followed a line to where its task takes it. The Equipe tool (E, or Alt with Ligar)
## edits them by dragging (`UtsimDocument.team_link`): from a dispatcher to a transporter
## (joins or leaves the team), from an object to a transporter or dispatcher (who carries
## its items), with Shift from a processor (who supplies its operators).
class_name EditorUtsim
extends Node3D

const PICK_PIXELS := 8.0
const DRAG_PIXELS := 4.0
const PRIMARY := Color(1.0, 1.0, 1.0)
const OTHERS := Color(0.35, 0.65, 1.0)
const PROBLEM := Color(1.0, 0.35, 0.30)
const LINK := Color(0.62, 0.72, 0.88)
const LINK_LOGICAL := Color(0.50, 0.55, 0.62)
const LINK_SELECTED := Color(1.0, 0.85, 0.30)
const CONNECTING := Color(0.45, 0.95, 0.55)
const PORT_OUT := Color(0.45, 0.95, 0.55)
const PORT_IN := Color(0.55, 0.80, 1.0)
const PORT_PIXELS := 14.0
const PORT_HOVER := Color(1.0, 1.0, 1.0)
const PORT_RING := Color(0.06, 0.07, 0.09, 0.9)
const REMOVE := Color(0.92, 0.26, 0.22)
## The service node of a transporter or dispatcher (the colour of the operator lines).
const PORT_SERVICE := Color(1.0, 0.58, 0.12)
## Kinds with a service node.
const SERVICE := ["Transporter", "Dispatcher"]
## Screen radius around a block's centre that still picks it (small blocks far away).
const PICK_CENTRE := 18.0
const CRITICAL := Color(1.0, 0.30, 0.22)
## The team and request lines by kind: colour and dash (metres drawn, metres skipped).
const TEAM_LINES := {
	"equipe": [Color(0.78, 0.58, 0.98), 0.5, 0.25],
	"transporte": [Color(0.40, 0.86, 0.92), 0.14, 0.16],
	"operadores": [Color(0.98, 0.70, 0.35), 0.14, 0.16],
	"manutencao": [Color(1.0, 0.45, 0.42), 0.3, 0.2],
	"casa": [Color(0.45, 0.90, 0.55), 0.2, 0.2],
	"pausa": [Color(0.55, 0.80, 1.0), 0.2, 0.2],
}
const TASK_LINE := Color(1.0, 1.0, 1.0)
## Kinds without an output port (a sink only receives; mobile resources are not wired
## by ports) and without an input port (a source only sends).
const NO_OUTPUT := ["Sink", "Transporter", "Dispatcher"]
const NO_INPUT := ["Source", "Transporter", "Dispatcher"]

var main: Node
## Selected objects, the primary one first.
var selected: Array = []
## Selected connection `[from, to]`, or empty.
var selected_link: Array = []
## `selecionar`, `ligar`, `colocar`, `rede`, `corredor` or `equipe`.
var tool := "selecionar"
## Objects on the critical path of the PERT/CPM window (red outline).
var critical: Array = []
## The path network tool.
var rede: EditorRede
## The circulation aisles tool, and the overlay of obstacles and access points.
var corredores: EditorCorredores
var place_id := ""
var connect_from := ""
## The start of a team link begun by a click (the next click ends it).
var team_from := ""
## Mouse position on screen (kept from the events, so scripted input works the same).
var mouse := Vector2.ZERO

var _press := {}
var _drag_names: Array = []
var _drag_offsets := {}
var _box_rect: ColorRect
var _palette_drag := ""
var _lines: MeshInstance3D
var _mesh: ImmediateMesh
var _ports: Array[Label3D] = []
var _ghosts := {}
var _ghost: Node3D
var _connections: Array = []
var _objects: Array = []
var _hover := ""
## The node under the mouse: `[block, "out"|"in"]`, or [].
var _hover_port: Array = []
## The block an input node was dragged from (the connection ends there), or "".
var connect_to := ""
var _remove_label: Label3D
## Selected team or request line: `{kind, from, to}` (block names), or empty.
var selected_team := {}
## The resource a service link is dragged from, or "".
var serve_from := ""
var _team_lines: MeshInstance3D
var _team_mesh: ImmediateMesh
var _links: Array = []
var _links_of: Object
var _alt := false
## The editor in Rust (`UtsimEditor`) and what it was last fed.
var ed: Object
var _zone_mesh: ImmediateMesh
var _zone_fill: MeshInstance3D
var _zone_labels: Array[Label3D] = []
var _scene_dirty := true
var _fed_playback: Object
var _fed_hidden: Array = []


func setup(owner_main: Node, canvas: CanvasLayer) -> void:
	main = owner_main
	_mesh = ImmediateMesh.new()
	_lines = MeshInstance3D.new()
	_lines.name = "Sobreposicao"
	_lines.mesh = _mesh
	_lines.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	var mat := StandardMaterial3D.new()
	mat.shading_mode = BaseMaterial3D.SHADING_MODE_UNSHADED
	mat.vertex_color_use_as_albedo = true
	mat.vertex_color_is_srgb = true
	mat.cull_mode = BaseMaterial3D.CULL_DISABLED
	mat.no_depth_test = true
	mat.render_priority = 5
	_lines.material_override = mat
	add_child(_lines)
	_team_mesh = ImmediateMesh.new()
	_team_lines = MeshInstance3D.new()
	_team_lines.name = "LigacoesEquipe"
	_team_lines.mesh = _team_mesh
	_team_lines.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	_team_lines.material_override = mat
	add_child(_team_lines)
	# Risk zones: a translucent fill on the floor (hidden by what stands on it).
	_zone_mesh = ImmediateMesh.new()
	var zones := MeshInstance3D.new()
	zones.name = "ZonasRisco"
	zones.mesh = _zone_mesh
	zones.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	var zm := StandardMaterial3D.new()
	zm.shading_mode = BaseMaterial3D.SHADING_MODE_UNSHADED
	zm.vertex_color_use_as_albedo = true
	zm.vertex_color_is_srgb = true
	zm.cull_mode = BaseMaterial3D.CULL_DISABLED
	zm.transparency = BaseMaterial3D.TRANSPARENCY_ALPHA
	zones.material_override = zm
	add_child(zones)
	_zone_fill = zones
	_box_rect = ColorRect.new()
	_box_rect.name = "CaixaSelecao"
	_box_rect.color = Color(0.35, 0.65, 1.0, 0.18)
	_box_rect.mouse_filter = Control.MOUSE_FILTER_IGNORE
	_box_rect.visible = false
	canvas.add_child(_box_rect)
	ed = ClassDB.instantiate("UtsimEditor")
	ed.set_tool("selecionar")
	rede = EditorRede.new()
	rede.name = "Rede"
	add_child(rede)
	rede.setup(owner_main, self)
	corredores = EditorCorredores.new()
	corredores.name = "Corredores"
	add_child(corredores)
	corredores.setup(owner_main, self)


func doc() -> Object:
	return main.doc


func camera() -> Camera3D:
	return main.camera


func cena() -> CenaUtsim:
	return main.cena


func sync(objects: Array, connections: Array) -> void:
	_objects = objects
	_connections = connections
	var names := {}
	for o in objects:
		names[o["name"]] = true
	var kept := []
	for n in selected:
		if names.has(n):
			kept.append(n)
	selected = kept
	# The script may have changed the selection (a rename): Rust follows it.
	ed.set_selected(PackedStringArray(selected))
	_scene_dirty = true
	_feed()
	_pull()


func set_tool(name: String, id := "") -> void:
	tool = name
	place_id = id if name == "colocar" else ""
	team_from = ""
	ed.set_tool(name if name in ["selecionar", "ligar", "zona", "rede"] else "selecionar")
	_pull()
	_show_ghost(place_id)
	main.on_tool_changed()
	match name:
		"equipe":
			main.set_status("Equipe: arraste do despachante a um transportador, ou de um objeto a quem leva os itens (Shift: de um processo a quem fornece os operadores); de novo desfaz. Mais fácil: arraste do nó laranja do operador até a máquina. Esc cancela.")
		"ligar":
			main.set_status("Ligar: arraste do bloco de origem até o destino, ou clique em um e depois no outro (Esc cancela).")
		"colocar":
			main.set_status("Clique no chão para colocar (Shift mantém a ferramenta, Esc cancela).")
		"corredor":
			main.set_status("Corredor: clique no chão para pôr os pontos do eixo (Enter ou duplo clique termina), clique num corredor para editá-lo, arraste um vértice para movê-lo; Esc cancela.")
		"zona":
			main.set_status("Zona: clique no chão para pôr os cantos da zona de risco; o primeiro ponto, Enter ou duplo clique fecha; clique numa zona para editá-la. Esc cancela.")
		"rede":
			main.set_status("Rede: clique no chão para pôr nós (cada clique continua o caminho), clique num trecho para editá-lo, arraste um nó para movê-lo; Esc encerra o caminho.")
		_:
			main.set_status("")


func select_only(name: String) -> void:
	_feed()
	_apply(ed.select_only(name))
func select_names(names: Array, add: bool) -> void:
	ed.select_names(PackedStringArray(names), add)
	_pull()
	main.on_selection_changed()


func toggle(name: String) -> void:
	ed.toggle(name)
	_pull()
	main.on_selection_changed()


func clear_selection() -> void:
	ed.clear_selection()
	_pull()
	main.on_selection_changed()


func select_all() -> void:
	var names := []
	for o in _objects:
		names.append(o["name"])
	select_names(names, false)


func selection() -> PackedStringArray:
	return PackedStringArray(selected)


# ---------------------------------------------------------------------------
# Picking
# ---------------------------------------------------------------------------


## Floor point under screen point `p`, or null when the ray misses the floor.
func floor_at(p: Vector2) -> Variant:
	var cam := camera()
	var o := cam.project_ray_origin(p)
	var d := cam.project_ray_normal(p)
	if absf(d.y) < 1e-6:
		return null
	var t := -o.y / d.y
	if t < 0.0:
		return null
	return o + d * t


func snap_point(p: Vector3) -> Vector3:
	var step: float = main.snap_step()
	if step <= 0.0:
		return Vector3(p.x, 0, p.z)
	return Vector3(roundf(p.x / step) * step, 0, roundf(p.z / step) * step)


func _box_of(i: int) -> AABB:
	var b: Dictionary = cena().blocks[i]
	var root: Node3D = cena().rigs[i]["root"]
	var s: Vector3 = b["size"]
	var hx := maxf(s.x * 0.5, 0.4) + 0.05
	var hz := maxf(s.z * 0.5, 0.4) + 0.05
	var h := maxf(s.y, 0.5)
	if b.get("skeletal", false):
		h = 1.8
	var p := root.global_position
	return AABB(Vector3(p.x - hx, 0.0, p.z - hz), Vector3(2.0 * hx, h, 2.0 * hz))


## Names of the blocks under screen point `p`, nearest first.
func hits_at(p: Vector2) -> Array:
	_feed()
	return Array(ed.hits_at(p))
func block_position(name: String) -> Variant:
	var i := cena().block_index(name)
	if i < 0:
		return null
	return (cena().rigs[i]["root"] as Node3D).global_position


## The block's centre on screen, or null when it is behind the camera.
func block_screen(name: String) -> Variant:
	_feed()
	return ed.block_screen(name)


func _kind_of(i: int) -> String:
	return String(cena().blocks[i].get("kind", ""))


## The output node of block `name` (a floor point on its `+x` side), or null.
func port_out(name: String) -> Variant:
	_feed()
	return ed.node_pos(name, "out")
## The input node of block `name` (on its `-x` side), or null.
func port_in(name: String) -> Variant:
	_feed()
	return ed.node_pos(name, "in")
## The service node of a transporter or dispatcher (on its `+x` side), or null.
func port_service(name: String) -> Variant:
	_feed()
	return ed.node_pos(name, "service")
## The node under screen point `p`: `[block, "out"|"in"|"service"]`, or [].
func node_at(p: Vector2) -> Array:
	_feed()
	return ed.node_at(p)


## Where the × that removes the selected connection or line is drawn, or null.
func remove_point() -> Variant:
	return ed.remove_point()
## The × of the selected connection on screen, or null.
func remove_screen() -> Variant:
	_feed()
	return ed.remove_screen()


func team_lines() -> Array:
	var out := []
	var c := cena()
	if c == null:
		return out
	for l in _links:
		var from := int(l["from"])
		var to := int(l["to"])
		if to < 0 or from >= c.blocks.size() or to >= c.blocks.size():
			continue
		out.append({"kind": String(l["kind"]), "from": String(c.blocks[from]["name"]),
			"to": String(c.blocks[to]["name"])})
	return out


## The team line under screen point `p`, or {}.


## Hands Rust the camera and, when it changed, the scene (blocks where they are drawn,
## connections, team lines, the kinds hidden).
func _feed() -> void:
	var cam := camera()
	if cam != null and is_inside_tree():
		ed.set_camera(cam.global_transform.affine_inverse(), cam.get_camera_projection(),
			get_viewport().get_visible_rect().size)
	var c := cena()
	if c == null or c.playback == null or c.rigs.size() != c.blocks.size():
		return
	var hidden: Array = main.links_hidden
	if not _scene_dirty and _fed_playback == c.playback and hidden == _fed_hidden:
		return
	if _links_of != c.playback:
		_links_of = c.playback
		_links = c.playback.team_links() if c.playback.has_method("team_links") else []
	var problems := {}
	for o in _objects:
		if int(o["problems"]) > 0:
			problems[o["name"]] = true
	var blocks := []
	for i in c.blocks.size():
		var b: Dictionary = c.blocks[i]
		blocks.append({"name": String(b["name"]), "kind": String(b.get("kind", "")),
			"pos": (c.rigs[i]["root"] as Node3D).global_position, "size": b["size"],
			"skeletal": bool(b.get("skeletal", false)), "problems": problems.has(b["name"])})
	ed.set_scene(blocks, _connections, team_lines(), PackedStringArray(hidden))
	_feed_zones()
	_scene_dirty = false
	_fed_playback = c.playback
	_fed_hidden = hidden.duplicate()
	_pull()


## The risk zones of the document for the Zona tool, and the grid step.
func _feed_zones() -> void:
	ed.set_snap(main.snap_step())
	ed.set_zones(doc().zones())


## The zone selected with the Zona tool, or "".
func selected_zone() -> String:
	return String(ed.selected_zone())


## Enter with the Zona tool: closes the zone being drawn.
func zone_finish() -> void:
	_apply(ed.zone_finish())


## A field of the zone selected (the panel): `name`, `kind`, `level` or `ppe`.
func set_zone_field(field: String, value: String) -> void:
	var name := selected_zone()
	if name == "":
		return
	if doc().set_zone_field(name, field, value):
		if field == "name":
			ed.select_zone(value.strip_edges())
		main.after_edit()
	else:
		main.set_status("Não mudou a zona: %s" % doc().last_error())


## Reads back what Rust keeps: the selection, the gesture under way and the node under the
## mouse (the panels, the roteiros and the drawing read these).
func _pull() -> void:
	selected = Array(ed.selected())
	selected_link = Array(ed.selected_link())
	selected_team = ed.selected_team()
	connect_from = String(ed.connect_from())
	connect_to = String(ed.connect_to())
	serve_from = String(ed.serve_from())
	_hover = String(ed.hover_name())
	_hover_port = ed.hover_node()


## Applies what a gesture asked of the document (see `UtsimEditor`).
func _apply(effects: Array) -> void:
	_pull()
	for e in effects:
		match String(e["op"]):
			"status":
				main.set_status(String(e["text"]))
			"selecao":
				main.on_selection_changed()
			"ligar":
				if doc().connect_objects(e["from"], e["to"]):
					main.after_edit()
					main.set_status(String(e["ok"]))
				else:
					main.set_status("Não ligou: %s" % doc().last_error())
			"desligar":
				if doc().disconnect_objects(e["from"], e["to"]):
					main.after_edit()
					main.set_status(String(e["ok"]))
				else:
					main.set_status("Não removeu: %s" % doc().last_error())
			"equipe":
				var r: Dictionary = doc().team_link(e["from"], e["to"], bool(e["operators"]))
				if r["ok"]:
					main.after_edit()
					if bool(e["removed"]):
						main.set_status("Removida: " + String(r["message"]) + " (Ctrl+Z desfaz)")
					else:
						main.set_status(String(e["ok"]) if String(e["ok"]) != "" else String(r["message"]))
				else:
					main.set_status(("Não removeu: " if bool(e["removed"]) else "Não ligou: ") + String(r["message"]))
			"mover_inicio":
				_drag_names = Array(e["names"])
				doc().begin_drag(PackedStringArray(_drag_names))
				_drag_offsets.clear()
				var at := _doc_positions()
				for n in _drag_names:
					var bp = block_position(n)
					if bp != null and at.has(n):
						_drag_offsets[n] = (bp as Vector3) - (at[n] as Vector3)
			"mover":
				var delta: Vector3 = e["delta"]
				doc().drag(delta, main.snap_step())
				var at := _doc_positions()
				for n in _drag_names:
					var i := cena().block_index(n)
					if i < 0 or not at.has(n):
						continue
					var pos: Vector3 = (at[n] as Vector3) + _drag_offsets.get(n, Vector3.ZERO)
					(cena().rigs[i]["root"] as Node3D).global_position = pos
					ed.set_position(n, pos)
				main.on_dragging()
			"mover_fim":
				doc().end_drag()
				main.after_edit()
			"zona_nova":
				var name: String = doc().add_zone(e["points"])
				if name == "":
					main.set_status("Não criou a zona: %s" % doc().last_error())
				else:
					ed.select_zone(name)
					main.after_edit()
					main.set_status("Zona %s criada: escolha o tipo, o nível e o EPI no painel (Ctrl+Z desfaz)." % name)
			"zona_forma":
				if doc().set_zone_points(e["name"], e["points"]):
					main.after_edit()
				else:
					main.set_status("Não mudou a zona: %s" % doc().last_error())
			"zona_excluir":
				if doc().remove_zone(e["name"]):
					main.after_edit()
					main.set_status("Zona %s excluída (Ctrl+Z desfaz)." % e["name"])
				else:
					main.set_status("Não excluiu a zona: %s" % doc().last_error())
			"remover":
				var names := PackedStringArray(e["names"])
				if doc().remove(names):
					main.after_edit()
					main.set_status("%d objeto(s) excluídos." % names.size())
	_pull()
func team_line_at(p: Vector2) -> Dictionary:
	_feed()
	return ed.team_line_at(p)
## The middle of team line `l` on screen (the roteiros' target), or null.
func team_line_screen(kind: String, from: String, to: String) -> Variant:
	_feed()
	return ed.team_line_screen(kind, from, to)
## Whether line `kind` from `from` to `to` exists.
func has_team_line(kind: String, from: String, to: String) -> bool:
	_feed()
	return ed.has_team_line(kind, from, to)


## The connection whose line passes near `p`: `[from, to]` or [].
func link_at(p: Vector2) -> Array:
	_feed()
	return Array(ed.link_at(p))
## The connection lines as drawn ({from, to, port, logical, a, b}).
func _segments() -> Array:
	return ed.segments()


# ---------------------------------------------------------------------------
# Input
# ---------------------------------------------------------------------------


## The library button was dragged out of its panel: follow the mouse until release.
func start_palette_drag(id: String) -> void:
	set_tool("colocar", id)
	_palette_drag = id


func _input(event: InputEvent) -> void:
	if not main.is_editing() or _palette_drag == "":
		return
	if event is InputEventMouseMotion:
		mouse = (event as InputEventMouseMotion).position
	var mb := event as InputEventMouseButton
	if mb != null and mb.button_index == MOUSE_BUTTON_LEFT and not mb.pressed:
		mouse = mb.position
		var id := _palette_drag
		_palette_drag = ""
		if main.over_ui(mouse):
			set_tool("selecionar")
			main.set_status("Colocação cancelada: solte o objeto sobre o chão.")
		else:
			_place(id, mouse, mb.shift_pressed)
		# The release still goes to the GUI: the library button that took the press keeps
		# the mouse until it sees the release (its tooltip would stay up and eat the next
		# Esc). It does not arm the entry again, the drag already cleared it.


func _unhandled_input(event: InputEvent) -> void:
	if not main.is_editing():
		return
	if event is InputEventMouseMotion:
		var mm := event as InputEventMouseMotion
		mouse = mm.position
		_feed()
		ed.hover(mm.position)
		_pull()
		if tool == "rede":
			if (mm.button_mask & MOUSE_BUTTON_MASK_LEFT) != 0:
				rede.drag(mm.position)
			return
		if tool == "corredor":
			if (mm.button_mask & MOUSE_BUTTON_MASK_LEFT) != 0:
				corredores.drag(mm.position)
			return
		if corredores.access_active():
			if (mm.button_mask & MOUSE_BUTTON_MASK_LEFT) != 0:
				corredores.access_drag(mm.position)
			return
		if (mm.button_mask & MOUSE_BUTTON_MASK_LEFT) != 0:
			if not _press.is_empty():
				_on_drag(mm.position)
			else:
				_apply(ed.drag(mm.position))
		return
	var mb := event as InputEventMouseButton
	if mb == null or mb.button_index != MOUSE_BUTTON_LEFT:
		return
	mouse = mb.position
	_alt = mb.alt_pressed
	if tool == "rede":
		if mb.pressed:
			get_viewport().gui_release_focus()
			rede.press(mb.position, mb.shift_pressed)
		else:
			rede.release(mb.position)
		get_viewport().set_input_as_handled()
		return
	if tool == "corredor":
		if mb.pressed:
			get_viewport().gui_release_focus()
			corredores.press(mb.position, mb.shift_pressed)
		else:
			corredores.release(mb.position)
		get_viewport().set_input_as_handled()
		return
	if tool == "selecionar":
		# An access point (shown with the obstacles) is dragged before any object.
		if mb.pressed and corredores.access_press(mb.position):
			get_viewport().gui_release_focus()
			get_viewport().set_input_as_handled()
			return
		if not mb.pressed and corredores.access_active():
			corredores.access_release(mb.position)
			get_viewport().set_input_as_handled()
			return
	if mb.pressed:
		get_viewport().gui_release_focus()
		ed.set_double(mb.double_click)
		_on_press(mb.position, mb.shift_pressed)
	else:
		_on_release(mb.position)
	get_viewport().set_input_as_handled()


func _place(id: String, p: Vector2, keep: bool) -> void:
	var at = floor_at(p)
	if at == null:
		main.set_status("Aponte para o chão para colocar o objeto.")
		return
	var name: String = doc().add(id, at, main.snap_step())
	if name == "":
		main.set_status("Não foi possível colocar: %s" % doc().last_error())
		return
	if not keep:
		set_tool("selecionar")
	selected = [name]
	ed.set_selected(PackedStringArray(selected))
	main.after_edit()
	main.set_status("%s colocado." % name)
	_feed()
	_apply(ed.select_only(name).filter(func(x): return x["op"] == "status"))


func _on_press(p: Vector2, shift: bool) -> void:
	if tool == "colocar":
		_place(place_id, p, shift)
		return
	if tool == "equipe" or (tool == "ligar" and _alt):
		_team_press(p, hits_at(p), shift)
		return
	_feed()
	_apply(ed.press(p, shift))
func _team_press(p: Vector2, hits: Array, shift: bool) -> void:
	if team_from != "":
		var from := team_from
		team_from = ""
		if hits.is_empty():
			main.set_status("Equipe: clique no transportador ou despachante de destino.")
			return
		_team_link(from, hits[0], shift)
		return
	if hits.is_empty():
		main.set_status("Equipe: comece num despachante ou num objeto.")
		return
	_press = {"pos": p, "shift": shift, "kind": "team", "name": hits[0]}
	team_from = hits[0]


func _team_link(from: String, to: String, operators: bool) -> void:
	if from == to:
		main.set_status("Equipe: solte sobre outro objeto.")
		return
	var r: Dictionary = doc().team_link(from, to, operators)
	if r["ok"]:
		main.after_edit()
	main.set_status(("" if r["ok"] else "Não mudou: ") + String(r["message"]))


## A drag of the Equipe tool (the others are Rust's).
func _on_drag(p: Vector2) -> void:
	var start: Vector2 = _press["pos"]
	if p.distance_to(start) >= DRAG_PIXELS:
		_press["moved"] = true


func _doc_positions() -> Dictionary:
	var out := {}
	for o in doc().objects():
		out[o["name"]] = o["position"]
	return out


func _on_release(p: Vector2) -> void:
	if _press.is_empty():
		_apply(ed.release(p))
		return
	# The Equipe tool.
	var press := _press
	_press = {}
	if not press.get("moved", false):
		# A click: the next click is the destination.
		main.set_status("Equipe: %s → clique no destino (Esc cancela)." % press["name"])
		return
	team_from = ""
	var hits := hits_at(p)
	if hits.is_empty():
		main.set_status("Equipe: solte sobre o transportador ou despachante.")
		return
	_team_link(press["name"], hits[0], press["shift"])
## Cancels, in order: placing, a connection begun, the Ligar tool, the selection.
func cancel() -> void:
	if tool == "rede":
		if not rede.cancel():
			set_tool("selecionar")
		return
	if tool == "corredor":
		if not corredores.cancel():
			set_tool("selecionar")
		return
	if tool == "colocar":
		set_tool("selecionar")
	elif team_from != "":
		team_from = ""
		_press = {}
		main.set_status("Equipe: cancelado.")
	elif tool == "equipe":
		set_tool("selecionar")
	else:
		var r: Dictionary = ed.cancel()
		if r["sair_da_ferramenta"]:
			set_tool("selecionar")
		_apply(r["efeitos"])
func nudge(dx: int, dy: int) -> void:
	if selected.is_empty():
		return
	var step: float = main.snap_step()
	if step <= 0.0:
		step = 1.0
	if doc().nudge(selection(), Vector3(dx * step, 0, -dy * step)):
		main.after_edit()


func delete_selected() -> void:
	if tool == "rede" and rede.delete_selected():
		return
	if tool == "corredor":
		if not corredores.delete_selected():
			main.set_status("Nenhum corredor selecionado para excluir.")
		return
	if tool == "zona" and int(ed.zone_drawing_count()) > 0:
		ed.zone_back()
		return
	_apply(ed.delete())


# ---------------------------------------------------------------------------
# Drawing
# ---------------------------------------------------------------------------


func _show_ghost(id: String) -> void:
	if _ghost != null:
		_ghost.visible = false
	_ghost = null
	if id == "":
		return
	if not _ghosts.has(id):
		var pb: Object = doc().palette_scene(id, CenaUtsim.skeletal_available())
		if pb == null:
			return
		var g := CenaUtsim.new()
		g.name = "Fantasma_" + id
		g.show_floor = false
		g.show_connections = false
		add_child(g)
		g.build(pb)
		g.apply(0.0, 0.0, true)
		g.set_ghost(0.5)
		_ghosts[id] = g
	_ghost = _ghosts[id]
	_ghost.visible = true


func _process(_dt: float) -> void:
	if main == null:
		return
	_draw_team()
	if not main.is_editing():
		_lines.visible = false
		for l in _ports:
			l.visible = false
		if _remove_label != null:
			_remove_label.visible = false
		_zone_fill.visible = false
		for l in _zone_labels:
			l.visible = false
		if _ghost != null:
			_ghost.visible = false
		return
	_lines.visible = true
	_zone_fill.visible = true
	if _ghost != null:
		var at = floor_at(mouse)
		_ghost.visible = at != null and not main.over_ui(mouse)
		if at != null:
			_ghost.position = snap_point(at)
	_redraw()


func _ribbon(a: Vector3, b: Vector3, width: float, color: Color) -> void:
	var d := b - a
	d.y = 0.0
	if d.length() < 1e-5:
		return
	var side := Vector3(-d.z, 0, d.x).normalized() * width * 0.5
	for v in [a - side, a + side, b + side, a - side, b + side, b - side]:
		_mesh.surface_set_color(color)
		_mesh.surface_add_vertex(v)


func _arrow(a: Vector3, b: Vector3, color: Color, width := 0.06) -> void:
	var d := b - a
	var length := d.length()
	if length < 1e-3:
		return
	d /= length
	var head := minf(0.45, length * 0.4)
	var base := b - d * head
	_ribbon(a, base, width, color)
	var side := Vector3(-d.z, 0, d.x) * head * 0.45
	for v in [base - side, base + side, b]:
		_mesh.surface_set_color(color)
		_mesh.surface_add_vertex(v)


func _rect(box: AABB, color: Color, width: float) -> void:
	var y := 0.03
	var p := [Vector3(box.position.x, y, box.position.z), Vector3(box.end.x, y, box.position.z),
		Vector3(box.end.x, y, box.end.z), Vector3(box.position.x, y, box.end.z)]
	for k in 4:
		var a: Vector3 = p[k]
		var b: Vector3 = p[(k + 1) % 4]
		var d := (b - a).normalized() * width * 0.5
		_ribbon(a - d, b + d, width, color)


func _redraw() -> void:
	_mesh.clear_surfaces()
	_mesh.surface_begin(Mesh.PRIMITIVE_TRIANGLES)
	# Keep the surface valid when there is nothing to draw.
	for k in 3:
		_mesh.surface_set_color(Color(0, 0, 0, 0))
		_mesh.surface_add_vertex(Vector3.ZERO)
	var c := cena()
	if c != null and c.playback != null:
		_feed()
		var o: Dictionary = ed.overlay()
		for k in o["corners"]:
			var box := AABB(k[0], (k[1] as Vector3) - (k[0] as Vector3))
			match String(k[2]):
				"primary":
					_corners(box.grow(0.1), PRIMARY, 0.09)
				"other":
					_corners(box.grow(0.1), OTHERS, 0.09)
				_:
					_corners(box.grow(0.04), PROBLEM, 0.04)
		for i in c.blocks.size():
			if critical.has(c.blocks[i]["name"]):
				_rect(_box_of(i).grow(0.3), CRITICAL, 0.1)
		corredores.draw(self)
		rede.draw(self)
		var segs := _segments()
		var outs := {}
		for s in segs:
			outs[s["from"]] = outs.get(s["from"], 0) + 1
		var k := 0
		for s in segs:
			var color := LINK_LOGICAL if s["logical"] else LINK
			if selected_link.size() == 2 and selected_link[0] == s["from"] and selected_link[1] == s["to"]:
				color = LINK_SELECTED
			_arrow(s["a"], s["b"], color, 0.09 if color == LINK_SELECTED else 0.06)
			if int(outs[s["from"]]) > 1:
				_port_label(k, s["a"].lerp(s["b"], 0.25), str(s["port"]), color)
				k += 1
		for j in range(k, _ports.size()):
			_ports[j].visible = false
		var colors := {"out": PORT_OUT, "in": PORT_IN, "service": PORT_SERVICE}
		for n in o["nodes"]:
			_port_mark(n[0], colors[String(n[1])], bool(n[2]), float(n[3]))
		_draw_remove(o["remove"])
		var pv: Array = o["preview"]
		if pv.size() == 3 and String(pv[2]) == "flow":
			_arrow(pv[0], pv[1], CONNECTING, 0.07)
		_draw_zones(o)
		var bx = o["box"]
		_box_rect.visible = bx != null
		if bx != null:
			_box_rect.position = (bx as Rect2).position
			_box_rect.size = (bx as Rect2).size
	_mesh.surface_end()


## The team and request lines, in both modes (see the header).
func _draw_team() -> void:
	_team_mesh.clear_surfaces()
	var c := cena()
	if c == null or c.playback == null or c.rigs.is_empty():
		_team_lines.visible = false
		return
	if _links_of != c.playback:
		_links_of = c.playback
		_links = c.playback.team_links() if c.playback.has_method("team_links") else []
	_team_lines.visible = true
	_team_mesh.surface_begin(Mesh.PRIMITIVE_TRIANGLES)
	for k in 3:
		_team_mesh.surface_set_color(Color(0, 0, 0, 0))
		_team_mesh.surface_add_vertex(Vector3.ZERO)
	var editing: bool = main.is_editing()
	var focus := {}
	if editing:
		for n in selected:
			focus[c.block_index(n)] = true
	elif main.follow_name != "":
		focus[c.block_index(main.follow_name)] = true
	var hidden: Array = main.links_hidden
	var members := {}
	for l in _links:
		var kind := String(l["kind"])
		var from := int(l["from"])
		var to := int(l["to"])
		if kind == "equipe" and to >= 0:
			members[to] = true
		if hidden.has(kind) or from >= c.rigs.size() or to >= c.rigs.size():
			continue
		var a: Vector3 = (c.rigs[from]["root"] as Node3D).global_position
		var b: Vector3 = (c.rigs[to]["root"] as Node3D).global_position if to >= 0 else l["point"]
		var style: Array = TEAM_LINES.get(kind, [Color.WHITE, 0.3, 0.2])
		var color: Color = style[0]
		var lit := focus.has(from) or focus.has(to)
		if editing and not selected_team.is_empty() and to >= 0 and String(c.blocks[from]["name"]) == selected_team["from"] and String(c.blocks[to]["name"]) == selected_team["to"] and kind == selected_team["kind"]:
			lit = true
		if editing:
			color.a = 1.0 if lit or focus.is_empty() else 0.55
		else:
			color.a = 1.0 if lit else 0.22
		_dashed(a + Vector3(0, 0.09, 0), b + Vector3(0, 0.09, 0), color, 0.09 if lit else 0.06, style[1], style[2])
	if not editing:
		# E8b: a ring in the colour of the state under each member, and the task of the one
		# followed.
		for i in members:
			if i < c.rigs.size() and i < c._last_color.size():
				var col: Color = c._last_color[i]
				col.a = 0.9
				_ring((c.rigs[i]["root"] as Node3D).global_position + Vector3(0, 0.04, 0), 0.62, col)
		for i in focus:
			if i >= 0 and i < c.task_target.size():
				var t := c.task_target[i]
				if not is_nan(t.x):
					var a: Vector3 = (c.rigs[i]["root"] as Node3D).global_position
					_dashed(a + Vector3(0, 0.12, 0), t + Vector3(0, 0.12, 0), TASK_LINE, 0.07, 0.35, 0.2)
	elif serve_from != "":
		var pv: Array = ed.overlay()["preview"]
		if pv.size() == 3 and String(pv[2]) == "service":
			_dashed((pv[0] as Vector3) + Vector3(0, 0.1, 0), (pv[1] as Vector3) + Vector3(0, 0.1, 0), PORT_SERVICE, 0.08, 0.14, 0.12)
	elif team_from != "":
		var a = block_position(team_from)
		var b = floor_at(mouse)
		if a != null and b != null:
			_dashed((a as Vector3) + Vector3(0, 0.1, 0), (b as Vector3) + Vector3(0, 0.1, 0), TEAM_LINES["equipe"][0], 0.08, 0.3, 0.15)
	_team_mesh.surface_end()


## The risk zones (fill, outline, name; the corners of the one selected with the Zona
## tool) and the zone being drawn.
func _draw_zones(o: Dictionary) -> void:
	_zone_mesh.clear_surfaces()
	_zone_mesh.surface_begin(Mesh.PRIMITIVE_TRIANGLES)
	for k in 3:
		_zone_mesh.surface_set_color(Color(0, 0, 0, 0))
		_zone_mesh.surface_add_vertex(Vector3.ZERO)
	var n := 0
	for z in o["zones"]:
		var pts: PackedVector3Array = z["points"]
		if pts.size() < 3:
			continue
		var col: Color = z["color"]
		var sel: bool = z["selected"]
		var flat := PackedVector2Array()
		var centre := Vector3.ZERO
		for v in pts:
			flat.append(Vector2(v.x, v.z))
			centre += v
		centre /= pts.size()
		var fill := Color(col, 0.34 if sel else 0.22)
		for i in Geometry2D.triangulate_polygon(flat):
			_zone_mesh.surface_set_color(fill)
			_zone_mesh.surface_add_vertex(Vector3(pts[i].x, 0.02, pts[i].z))
		for k in pts.size():
			var a := pts[k] + Vector3(0, 0.035, 0)
			var b := pts[(k + 1) % pts.size()] + Vector3(0, 0.035, 0)
			_ribbon(a, b, 0.1 if sel else 0.06, col)
		if sel and tool == "zona":
			for v in pts:
				_disc(v + Vector3(0, 0.04, 0), 0.14, PORT_HOVER)
				_disc(v + Vector3(0, 0.045, 0), 0.1, col)
		while _zone_labels.size() <= n:
			var l := Label3D.new()
			l.billboard = BaseMaterial3D.BILLBOARD_ENABLED
			l.fixed_size = true
			l.no_depth_test = true
			l.pixel_size = 0.0006
			l.font_size = 28
			l.outline_size = 10
			l.outline_modulate = Color(0.06, 0.07, 0.09, 0.95)
			l.render_priority = 12
			l.outline_render_priority = 11
			add_child(l)
			_zone_labels.append(l)
		var lab := _zone_labels[n]
		lab.visible = true
		lab.text = String(z["name"])
		lab.modulate = col.lightened(0.3)
		lab.position = centre + Vector3(0, 0.3, 0)
		n += 1
	for j in range(n, _zone_labels.size()):
		_zone_labels[j].visible = false
	var drawing: PackedVector3Array = o["zone_drawing"]
	for k in drawing.size() - 1:
		_ribbon(drawing[k] + Vector3(0, 0.04, 0), drawing[k + 1] + Vector3(0, 0.04, 0), 0.07, CONNECTING)
	for k in drawing.size() - 1:
		_disc(drawing[k] + Vector3(0, 0.045, 0), 0.12, CONNECTING)
	_zone_mesh.surface_end()


## A dashed ribbon from `a` to `b` (`on` metres drawn, `off` skipped).
func _dashed(a: Vector3, b: Vector3, color: Color, width: float, on: float, off: float) -> void:
	var d := b - a
	var length := d.length()
	if length < 1e-3:
		return
	var dir := d / length
	var side := Vector3(-dir.z, 0, dir.x).normalized() * width * 0.5
	if side.length() < 1e-6:
		side = Vector3(width * 0.5, 0, 0)
	var s := 0.0
	while s < length:
		var e := minf(s + on, length)
		var p := a + dir * s
		var q := a + dir * e
		for v in [p - side, p + side, q + side, p - side, q + side, q - side]:
			_team_mesh.surface_set_color(color)
			_team_mesh.surface_add_vertex(v)
		s = e + off


## A ring on the floor around `c`.
func _ring(c: Vector3, r: float, color: Color) -> void:
	var n := 20
	var w := 0.07
	for k in n:
		var a0 := TAU * k / n
		var a1 := TAU * (k + 1) / n
		var p0 := c + Vector3(cos(a0), 0, sin(a0)) * r
		var p1 := c + Vector3(cos(a1), 0, sin(a1)) * r
		var q0 := c + Vector3(cos(a0), 0, sin(a0)) * (r + w)
		var q1 := c + Vector3(cos(a1), 0, sin(a1)) * (r + w)
		for v in [p0, q0, q1, p0, q1, p1]:
			_team_mesh.surface_set_color(color)
			_team_mesh.surface_add_vertex(v)


## A node: a disc on the floor with a dark rim, joined to the side of its block (at
## `edge_x`) by a short stub; bigger and white-rimmed when `lit` (under the mouse, or the
## end of the connection being made).
func _port_mark(at: Variant, color: Color, lit := false, edge_x := NAN) -> void:
	if at == null:
		return
	var c: Vector3 = at
	if not is_nan(edge_x):
		_ribbon(Vector3(edge_x, c.y, c.z), c, 0.04, Color(color, 0.8))
	var r := 0.17 if lit else 0.12
	_disc(c, r + 0.035, PORT_HOVER if lit else PORT_RING)
	_disc(c, r, color.lightened(0.25) if lit else color)


## A filled disc on the floor plane.
func _disc(c: Vector3, r: float, color: Color) -> void:
	var n := 16
	for k in n:
		var a0 := TAU * k / n
		var a1 := TAU * (k + 1) / n
		for v in [c, c + Vector3(cos(a0), 0, sin(a0)) * r, c + Vector3(cos(a1), 0, sin(a1)) * r]:
			_mesh.surface_set_color(color)
			_mesh.surface_add_vertex(v)


## The red × in the middle of the selected connection (a click on it removes it).
func _draw_remove(at: Variant) -> void:
	if at == null:
		if _remove_label != null:
			_remove_label.visible = false
		return
	var sp = ed.project(at)
	var lit: bool = sp != null and (sp as Vector2).distance_to(mouse) <= PORT_PIXELS
	for seg in (_segments() if selected_team.is_empty() else []):
		if seg["from"] == selected_link[0] and seg["to"] == selected_link[1]:
			var m: Vector3 = (seg["a"] as Vector3).lerp(seg["b"], 0.5)
			if m.distance_to(at) > 0.05:
				_ribbon(m, at, 0.03, Color(REMOVE, 0.8))
	_disc(at, 0.2 if lit else 0.17, PORT_HOVER)
	_disc(at, 0.17 if lit else 0.14, REMOVE)
	if _remove_label == null:
		_remove_label = Label3D.new()
		_remove_label.name = "RemoverLigacao"
		_remove_label.billboard = BaseMaterial3D.BILLBOARD_ENABLED
		_remove_label.fixed_size = true
		_remove_label.no_depth_test = true
		_remove_label.pixel_size = 0.0006
		_remove_label.font_size = 34
		_remove_label.modulate = Color.WHITE
		_remove_label.render_priority = 13
		_remove_label.text = "×"
		add_child(_remove_label)
	_remove_label.visible = true
	_remove_label.position = (at as Vector3) + Vector3(0, 0.02, 0)


## The selection: L marks at the four corners of the footprint (no frame over the nodes).
func _corners(box: AABB, color: Color, width: float) -> void:
	var y := 0.03
	var arm := clampf(minf(box.size.x, box.size.z) * 0.3, 0.25, 0.8)
	var x0 := box.position.x
	var x1 := box.end.x
	var z0 := box.position.z
	var z1 := box.end.z
	for corner in [[x0, z0, 1, 1], [x1, z0, -1, 1], [x1, z1, -1, -1], [x0, z1, 1, -1]]:
		var p := Vector3(corner[0], y, corner[1])
		_ribbon(p, p + Vector3(corner[2] * arm, 0, 0), width, color)
		_ribbon(p, p + Vector3(0, 0, corner[3] * arm), width, color)


func _port_label(k: int, at: Vector3, text: String, color: Color) -> void:
	while _ports.size() <= k:
		var l := Label3D.new()
		l.billboard = BaseMaterial3D.BILLBOARD_ENABLED
		l.fixed_size = true
		l.no_depth_test = true
		l.pixel_size = 0.0006
		l.font_size = 30
		l.outline_size = 10
		l.outline_modulate = Color(0.06, 0.07, 0.09, 0.95)
		l.render_priority = 12
		l.outline_render_priority = 11
		add_child(l)
		_ports.append(l)
	var l := _ports[k]
	l.visible = true
	l.text = text
	l.modulate = color
	l.position = at + Vector3(0, 0.25, 0)

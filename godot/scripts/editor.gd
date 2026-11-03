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
## Connecting: each block that can send items has an output port (a green mark on its
## `+x` side) and each one that can receive has an input port (on its `-x` side). The
## ports show on the selected blocks and on the block under the mouse; dragging from an
## output port to another block (or to its input port) connects them. The Ligar tool (L)
## does the same by dragging from block to block, or by clicking the origin and then the
## destination.
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
var _dragging := false
var _drag_names: Array = []
var _drag_floor := Vector3.ZERO
var _drag_offsets := {}
var _box := false
var _box_rect: ColorRect
var _palette_drag := ""
var _last_click := Vector2(-1000, -1000)
var _last_hits: Array = []
var _last_index := 0
var _lines: MeshInstance3D
var _mesh: ImmediateMesh
var _ports: Array[Label3D] = []
var _ghosts := {}
var _ghost: Node3D
var _connections: Array = []
var _objects: Array = []
var _hover := ""
var _team_lines: MeshInstance3D
var _team_mesh: ImmediateMesh
var _links: Array = []
var _links_of: Object
var _alt := false


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
	_box_rect = ColorRect.new()
	_box_rect.name = "CaixaSelecao"
	_box_rect.color = Color(0.35, 0.65, 1.0, 0.18)
	_box_rect.mouse_filter = Control.MOUSE_FILTER_IGNORE
	_box_rect.visible = false
	canvas.add_child(_box_rect)
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


## Called after every change of the document: keeps only the names that still exist.
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
	if not selected_link.is_empty():
		var found := false
		for c in connections:
			if c["from"] == selected_link[0] and c["to"] == selected_link[1]:
				found = true
		if not found:
			selected_link = []
	if connect_from != "" and not names.has(connect_from):
		connect_from = ""


func set_tool(name: String, id := "") -> void:
	tool = name
	place_id = id if name == "colocar" else ""
	connect_from = ""
	team_from = ""
	_show_ghost(place_id)
	main.on_tool_changed()
	match name:
		"equipe":
			main.set_status("Equipe: arraste do despachante a um transportador, ou de um objeto a quem leva os itens (Shift: de um processo a quem fornece os operadores); de novo desfaz. Esc cancela.")
		"ligar":
			main.set_status("Ligar: arraste do bloco de origem até o destino, ou clique em um e depois no outro (Esc cancela).")
		"colocar":
			main.set_status("Clique no chão para colocar (Shift mantém a ferramenta, Esc cancela).")
		"corredor":
			main.set_status("Corredor: clique no chão para pôr os pontos do eixo (Enter ou duplo clique termina), clique num corredor para editá-lo, arraste um vértice para movê-lo; Esc cancela.")
		"rede":
			main.set_status("Rede: clique no chão para pôr nós (cada clique continua o caminho), clique num trecho para editá-lo, arraste um nó para movê-lo; Esc encerra o caminho.")
		_:
			main.set_status("")


func select_only(name: String) -> void:
	selected = [name]
	selected_link = []
	main.on_selection_changed()


func select_names(names: Array, add: bool) -> void:
	if not add:
		selected = []
	for n in names:
		if not selected.has(n):
			selected.append(n)
	selected_link = []
	main.on_selection_changed()


func toggle(name: String) -> void:
	if selected.has(name):
		selected.erase(name)
	else:
		selected.push_front(name)
	selected_link = []
	main.on_selection_changed()


func clear_selection() -> void:
	selected = []
	selected_link = []
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


static func _ray_box(o: Vector3, d: Vector3, lo: Vector3, hi: Vector3) -> float:
	var tmin := -INF
	var tmax := INF
	for k in 3:
		if absf(d[k]) < 1e-9:
			if o[k] < lo[k] or o[k] > hi[k]:
				return -1.0
		else:
			var t1 := (lo[k] - o[k]) / d[k]
			var t2 := (hi[k] - o[k]) / d[k]
			tmin = maxf(tmin, minf(t1, t2))
			tmax = minf(tmax, maxf(t1, t2))
	if tmax < maxf(tmin, 0.0):
		return -1.0
	return maxf(tmin, 0.0)


## The box a block is picked by, in world space: its footprint (at least 0.8 m) and its
## height (at least 0.5 m).
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
	var cam := camera()
	var o := cam.project_ray_origin(p)
	var d := cam.project_ray_normal(p)
	var found := []
	var c := cena()
	if c == null or c.playback == null:
		return []
	for i in c.blocks.size():
		var box := _box_of(i)
		var t := _ray_box(o, d, box.position, box.end)
		if t >= 0.0:
			found.append([t, String(c.blocks[i]["name"])])
	found.sort_custom(func(a, b): return a[0] < b[0])
	var names := []
	for f in found:
		names.append(f[1])
	return names


## Where block `name` is drawn now (its root), or null.
func block_position(name: String) -> Variant:
	var i := cena().block_index(name)
	if i < 0:
		return null
	return (cena().rigs[i]["root"] as Node3D).global_position


## The block's centre on screen, or null when it is behind the camera.
func block_screen(name: String) -> Variant:
	var i := cena().block_index(name)
	if i < 0:
		return null
	var box := _box_of(i)
	var c := box.get_center()
	if camera().is_position_behind(c):
		return null
	return camera().unproject_position(c)


func _kind_of(i: int) -> String:
	return String(cena().blocks[i].get("kind", ""))


## The output port of block `name` (a floor point on its `+x` side), or null when the
## block has none.
func port_out(name: String) -> Variant:
	var i := cena().block_index(name)
	if i < 0 or NO_OUTPUT.has(_kind_of(i)):
		return null
	var box := _box_of(i)
	return Vector3(box.end.x + 0.25, 0.06, box.get_center().z)


## The input port of block `name` (on its `-x` side), or null.
func port_in(name: String) -> Variant:
	var i := cena().block_index(name)
	if i < 0 or NO_INPUT.has(_kind_of(i)):
		return null
	var box := _box_of(i)
	return Vector3(box.position.x - 0.25, 0.06, box.get_center().z)


## Blocks whose ports are drawn and can be grabbed: the selection and the block under
## the mouse.
func _port_blocks() -> Array:
	var out := selected.duplicate()
	if _hover != "" and not out.has(_hover):
		out.append(_hover)
	# C7: sources and sinks always show their ports in Editar.
	var c := cena()
	if c != null:
		for b in c.blocks:
			if String(b["kind"]) in ["Source", "Sink"] and not out.has(b["name"]):
				out.append(b["name"])
	return out


func _near(world: Variant, p: Vector2) -> bool:
	if world == null or camera().is_position_behind(world):
		return false
	return camera().unproject_position(world).distance_to(p) <= PORT_PIXELS


## The block under the mouse, or whose output port is under it (the port sticks out of
## the block), or "".
func _hover_at(p: Vector2) -> String:
	var hits := hits_at(p)
	if not hits.is_empty():
		return hits[0]
	for o in _objects:
		if _near(port_out(o["name"]), p):
			return o["name"]
	return ""


## The block whose output port is under screen point `p`, or "".
func port_at(p: Vector2) -> String:
	for n in _port_blocks():
		if _near(port_out(n), p):
			return n
	return ""


## The block a connection dragged from `from` lands on at screen point `p`: an input port
## under the point, else the nearest block under it that is not `from`; "" for none.
func drop_target(p: Vector2, from: String) -> String:
	for o in _objects:
		var n: String = o["name"]
		if n != from and _near(port_in(n), p):
			return n
	for n in hits_at(p):
		if n != from:
			return n
	return ""


static func _segment_distance(p: Vector2, a: Vector2, b: Vector2) -> float:
	var ab := b - a
	var len2 := ab.length_squared()
	if len2 < 1e-6:
		return p.distance_to(a)
	var t := clampf((p - a).dot(ab) / len2, 0.0, 1.0)
	return p.distance_to(a + ab * t)


## The connection whose line passes within PICK_PIXELS of `p`: `[from, to]` or [].
func link_at(p: Vector2) -> Array:
	var best := PICK_PIXELS
	var hit := []
	for seg in _segments():
		var a: Vector3 = seg["a"]
		var b: Vector3 = seg["b"]
		if camera().is_position_behind(a) or camera().is_position_behind(b):
			continue
		var dist := _segment_distance(p, camera().unproject_position(a), camera().unproject_position(b))
		if dist < best:
			best = dist
			hit = [seg["from"], seg["to"]]
	return hit


## Connection segments between the edges of the blocks' footprints, so the arrows show.
func _segments() -> Array:
	var out := []
	for c in _connections:
		var pa = block_position(c["from"])
		var pb = block_position(c["to"])
		if pa == null or pb == null:
			continue
		var a: Vector3 = pa
		var b: Vector3 = pb
		a.y = 0.06
		b.y = 0.06
		var dir := b - a
		var length := dir.length()
		if length < 1e-3:
			continue
		dir /= length
		var ra := _radius(c["from"], dir)
		var rb := _radius(c["to"], -dir)
		if ra + rb >= length - 0.2:
			ra = length * 0.3
			rb = length * 0.3
		out.append({"from": c["from"], "to": c["to"], "port": c["port"], "logical": c["logical"],
			"a": a + dir * ra, "b": b - dir * rb})
	return out


## Distance from a block's centre to the edge of its footprint along `dir`.
func _radius(name: String, dir: Vector3) -> float:
	var i := cena().block_index(name)
	if i < 0:
		return 0.0
	var s: Vector3 = cena().blocks[i]["size"]
	var hx := maxf(s.x * 0.5, 0.3)
	var hz := maxf(s.z * 0.5, 0.3)
	var kx := hx / maxf(absf(dir.x), 1e-6)
	var kz := hz / maxf(absf(dir.z), 1e-6)
	return minf(kx, kz) + 0.15


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
		_hover = _hover_at(mm.position)
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
		if not _press.is_empty() and (mm.button_mask & MOUSE_BUTTON_MASK_LEFT) != 0:
			_on_drag(mm.position)
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
	selected_link = []
	main.after_edit()
	main.set_status("%s colocado." % name)


func _on_press(p: Vector2, shift: bool) -> void:
	if tool == "colocar":
		_place(place_id, p, shift)
		return
	var hits := hits_at(p)
	if tool == "equipe" or (tool == "ligar" and _alt):
		_team_press(p, hits, shift)
		return
	var port := port_at(p) if connect_from == "" else ""
	if port != "":
		_begin_link(p, port, false)
		return
	if tool == "ligar":
		if connect_from == "" and not hits.is_empty():
			_begin_link(p, hits[0], true)
		else:
			_connect_click(hits)
		return
	_press = {"pos": p, "shift": shift, "kind": "floor", "name": ""}
	if hits.is_empty():
		var link := link_at(p)
		if not link.is_empty():
			_press["kind"] = "link"
			selected = []
			selected_link = link
			main.on_selection_changed()
			main.set_status("Ligação %s → %s selecionada (Del remove)." % link)
		_last_hits = []
		return
	# Clicking again at the same point walks through the overlapping blocks.
	var index := 0
	if p.distance_to(_last_click) < 5.0 and hits == _last_hits and hits.size() > 1:
		index = (_last_index + 1) % hits.size()
	_last_click = p
	_last_hits = hits
	_last_index = index
	var name: String = hits[index]
	_press["kind"] = "block"
	_press["name"] = name
	if hits.size() > 1:
		main.set_status("%s (%d de %d blocos neste ponto: clique de novo para o próximo)." % [name, index + 1, hits.size()])
	if not shift and (not selected.has(name) or index > 0):
		select_only(name)


## A connection begun at block `from` (its output port, or the block itself with the Ligar
## tool): it is made where the mouse is released, or, after a plain click, on the next
## click (the Ligar tool stays on).
func _begin_link(p: Vector2, from: String, from_tool: bool) -> void:
	_press = {"pos": p, "shift": false, "kind": "link", "name": from, "tool": from_tool}
	connect_from = from
	main.set_status("Ligar: solte sobre o destino (ou clique nele). Esc cancela.")


func _finish_link(p: Vector2, press: Dictionary) -> void:
	var from: String = press["name"]
	if not press.get("moved", false):
		# A click: wait for the destination with the Ligar tool.
		if tool != "ligar":
			set_tool("ligar")
		connect_from = from
		main.set_status("Ligar: %s → clique no destino (Esc cancela)." % from)
		return
	var to := drop_target(p, from)
	connect_from = ""
	if to == "":
		main.set_status("Ligação cancelada: solte sobre o bloco de destino.")
		return
	if doc().connect_objects(from, to):
		main.after_edit()
		main.set_status("Ligado: %s → %s." % [from, to])
	else:
		main.set_status("Não ligou: %s" % doc().last_error())


func _connect_click(hits: Array) -> void:
	if hits.is_empty():
		main.set_status("Ligar: clique em um bloco.")
		return
	var name: String = hits[0]
	if connect_from == "":
		connect_from = name
		main.set_status("Ligar: %s → clique no destino (Esc cancela)." % name)
		return
	var from := connect_from
	connect_from = ""
	if doc().connect_objects(from, name):
		main.after_edit()
		main.set_status("Ligado: %s → %s. Clique em outra origem ou Esc." % [from, name])
	else:
		main.set_status("Não ligou: %s" % doc().last_error())


## A press of the Equipe tool: starts a drag from the block under the mouse, or ends a link
## begun by a click.
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


func _on_drag(p: Vector2) -> void:
	var start: Vector2 = _press["pos"]
	if not _dragging and not _box and p.distance_to(start) < DRAG_PIXELS:
		return
	match _press["kind"]:
		"link", "team":
			_press["moved"] = true
		"block":
			if _press["shift"]:
				return
			if not _dragging:
				_begin_move(start)
			_move_to(p)
		"floor":
			_box = true
			var r := Rect2(start, Vector2.ZERO).expand(p)
			_box_rect.position = r.position
			_box_rect.size = r.size
			_box_rect.visible = true


func _begin_move(start: Vector2) -> void:
	var f = floor_at(start)
	if f == null or selected.is_empty():
		return
	_dragging = true
	_drag_floor = f
	_drag_names = selected.duplicate()
	doc().begin_drag(PackedStringArray(_drag_names))
	_drag_offsets.clear()
	var at := _doc_positions()
	for n in _drag_names:
		var bp = block_position(n)
		if bp != null and at.has(n):
			_drag_offsets[n] = (bp as Vector3) - (at[n] as Vector3)


func _doc_positions() -> Dictionary:
	var out := {}
	for o in doc().objects():
		out[o["name"]] = o["position"]
	return out


func _move_to(p: Vector2) -> void:
	var f = floor_at(p)
	if f == null:
		return
	var delta: Vector3 = (f as Vector3) - _drag_floor
	delta.y = 0.0
	doc().drag(delta, main.snap_step())
	var at := _doc_positions()
	for n in _drag_names:
		var i := cena().block_index(n)
		if i < 0 or not at.has(n):
			continue
		(cena().rigs[i]["root"] as Node3D).global_position = (at[n] as Vector3) + _drag_offsets.get(n, Vector3.ZERO)
	main.on_dragging()


func _on_release(p: Vector2) -> void:
	if _press.is_empty():
		return
	var press := _press
	_press = {}
	if press["kind"] == "link":
		_finish_link(p, press)
		return
	if press["kind"] == "team":
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
		return
	if _dragging:
		_dragging = false
		doc().end_drag()
		main.after_edit()
		main.set_status("%d objeto(s) movidos." % _drag_names.size())
		return
	if _box:
		_box = false
		_box_rect.visible = false
		var r := Rect2(press["pos"], Vector2.ZERO).expand(p)
		var inside := []
		for b in cena().blocks:
			var s = block_screen(b["name"])
			if s != null and r.has_point(s):
				inside.append(b["name"])
		select_names(inside, press["shift"])
		main.set_status("%d objeto(s) na caixa." % inside.size())
		return
	match press["kind"]:
		"block":
			if press["shift"]:
				toggle(press["name"])
		"floor":
			if not press["shift"]:
				clear_selection()


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
		main.set_status("Equipe: cancelado.")
	elif tool == "equipe":
		set_tool("selecionar")
	elif connect_from != "":
		connect_from = ""
		main.set_status("Ligação cancelada.")
	elif tool == "ligar":
		set_tool("selecionar")
	else:
		clear_selection()


## Moves the selection one grid step with the arrow keys (`dx`, `dy` in model plan axes).
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
	if not selected_link.is_empty():
		var link := selected_link
		if doc().disconnect_objects(link[0], link[1]):
			selected_link = []
			main.after_edit()
			main.set_status("Ligação %s → %s removida." % link)
		return
	if selected.is_empty():
		main.set_status("Nada selecionado para excluir.")
		return
	var n := selected.size()
	if doc().remove(selection()):
		selected = []
		main.after_edit()
		main.set_status("%d objeto(s) excluídos." % n)


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
		if _ghost != null:
			_ghost.visible = false
		return
	_lines.visible = true
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
		var problems := {}
		for o in _objects:
			if int(o["problems"]) > 0:
				problems[o["name"]] = true
		for i in c.blocks.size():
			var name: String = c.blocks[i]["name"]
			var box := _box_of(i)
			if selected.has(name):
				_rect(box.grow(0.12), PRIMARY if selected[0] == name else OTHERS, 0.08)
			if problems.has(name):
				_rect(box.grow(0.04), PROBLEM, 0.04)
			if name == connect_from:
				_rect(box.grow(0.2), CONNECTING, 0.08)
			if critical.has(name):
				_rect(box.grow(0.3), CRITICAL, 0.1)
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
		for n in _port_blocks():
			_port_mark(port_out(n), PORT_OUT)
			_port_mark(port_in(n), PORT_IN)
		if connect_from != "":
			for o in _objects:
				if o["name"] != connect_from:
					_port_mark(port_in(o["name"]), PORT_IN)
			var a = port_out(connect_from)
			if a == null:
				a = block_position(connect_from)
			var b = floor_at(mouse)
			if a != null and b != null:
				var pa: Vector3 = a
				pa.y = 0.06
				var pb: Vector3 = b
				pb.y = 0.06
				_arrow(pa, pb, CONNECTING, 0.07)
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
	elif team_from != "":
		var a = block_position(team_from)
		var b = floor_at(mouse)
		if a != null and b != null:
			_dashed((a as Vector3) + Vector3(0, 0.1, 0), (b as Vector3) + Vector3(0, 0.1, 0), TEAM_LINES["equipe"][0], 0.08, 0.3, 0.15)
	_team_mesh.surface_end()


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


## A port: a small diamond on the floor.
func _port_mark(at: Variant, color: Color) -> void:
	if at == null:
		return
	var c: Vector3 = at
	var r := 0.18
	var pts := [c + Vector3(r, 0, 0), c + Vector3(0, 0, r), c + Vector3(-r, 0, 0), c + Vector3(0, 0, -r)]
	for k in [[0, 1, 2], [0, 2, 3]]:
		for j in k:
			_mesh.surface_set_color(color)
			_mesh.surface_add_vertex(pts[j])


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

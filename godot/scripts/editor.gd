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

var main: Node
## Selected objects, the primary one first.
var selected: Array = []
## Selected connection `[from, to]`, or empty.
var selected_link: Array = []
## `selecionar`, `ligar` or `colocar`.
var tool := "selecionar"
var place_id := ""
var connect_from := ""
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
	_box_rect = ColorRect.new()
	_box_rect.name = "CaixaSelecao"
	_box_rect.color = Color(0.35, 0.65, 1.0, 0.18)
	_box_rect.mouse_filter = Control.MOUSE_FILTER_IGNORE
	_box_rect.visible = false
	canvas.add_child(_box_rect)


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
	_show_ghost(place_id)
	main.on_tool_changed()
	match name:
		"ligar":
			main.set_status("Ligar: clique no bloco de origem e depois no de destino (Esc cancela).")
		"colocar":
			main.set_status("Clique no chão para colocar (Shift mantém a ferramenta, Esc cancela).")
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
		get_viewport().set_input_as_handled()


func _unhandled_input(event: InputEvent) -> void:
	if not main.is_editing():
		return
	if event is InputEventMouseMotion:
		var mm := event as InputEventMouseMotion
		mouse = mm.position
		if not _press.is_empty() and (mm.button_mask & MOUSE_BUTTON_MASK_LEFT) != 0:
			_on_drag(mm.position)
		return
	var mb := event as InputEventMouseButton
	if mb == null or mb.button_index != MOUSE_BUTTON_LEFT:
		return
	mouse = mb.position
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
	if tool == "ligar":
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


func _on_drag(p: Vector2) -> void:
	var start: Vector2 = _press["pos"]
	if not _dragging and not _box and p.distance_to(start) < DRAG_PIXELS:
		return
	match _press["kind"]:
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
	if tool == "colocar":
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
		if tool == "ligar" and connect_from != "":
			var a = block_position(connect_from)
			var b = floor_at(mouse)
			if a != null and b != null:
				var pa: Vector3 = a
				pa.y = 0.06
				var pb: Vector3 = b
				pb.y = 0.06
				_arrow(pa, pb, CONNECTING, 0.07)
	_mesh.surface_end()


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

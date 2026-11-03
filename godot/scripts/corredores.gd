## Circulation aisles in Editar (phase G4): the "Corredor" tool (C) draws and edits the
## aisles of `Model.aisles` through `UtsimDocument` (`add_aisle`, `move_aisle_point`,
## `set_aisle_*`...), so every change is a command with undo. The floor marking itself
## (yellow lines, zebra at the crossings, arrows) is drawn by the scene (`cena.gd`), in
## Editar and in Simular; this file draws only what the editor adds over it.
##
## - Clicking the floor puts the points of the centre line, one click each; a point lands
##   on the vertex or the line of another aisle when one is within `SNAP_METRES`, else on
##   the grid. Enter, or a second click on the last point (a double click), ends the
##   aisle; Backspace takes the last point back; Esc cancels. Shift + click starts a new
##   aisle even over the line of an existing one.
## - Clicking an aisle selects it: the panel changes kind (pedestrian, forklift, mixed),
##   width, direction, profile and name. Dragging a vertex moves it (one undo step);
##   Shift + click on the line of the selected aisle adds a vertex; Del excludes it.
##
## "Mostrar obstáculos e folgas" (any tool) draws the areas of the objects and the walls of
## the model, the clearance kept from them, and the access point of every object: a
## diamond to drag with the Select tool (it edits `access` of the object).
class_name EditorCorredores
extends Node

const PICK_PIXELS := 12.0
const LINE_PIXELS := 9.0
const DRAG_PIXELS := 4.0
const SNAP_METRES := 0.45
const LINE := Color(0.95, 0.85, 0.30)
const SELECTED := Color(1.0, 0.97, 0.60)
const DRAWING := Color(0.45, 0.95, 0.55)
const NODE := Color(0.92, 0.94, 0.98)
const WARN := Color(1.0, 0.55, 0.10)
const OBJECT := Color(1.0, 0.45, 0.40)
const WALL := Color(0.75, 0.55, 1.0)
const HALO := Color(1.0, 0.75, 0.30)
const ACCESS := Color(0.35, 0.90, 0.95)
const ACCESS_OWN := Color(1.0, 0.45, 0.95)
const KINDS := ["pedestrian", "forklift", "mixed"]
const KIND_LABELS := ["Pedestres", "Empilhadeiras", "Misto"]
## Width a new aisle of each kind starts with (the minimums of the Brasil profile).
const KIND_WIDTHS := {"pedestrian": 1.2, "forklift": 3.6, "mixed": 4.8}

var main: Node
var editor: EditorUtsim
## `UtsimDocument.aisles()`, `crossings()`, `obstacles()` and `access_points()`.
var list: Array = []
var crossings: Array = []
var obstacles: Array = []
var access: Array = []
var show_obstacles := false
var selected := ""
var selected_vertex := -1
## The aisle being drawn: floor points in Godot coordinates.
var points := PackedVector3Array()
## What the next aisle starts as.
var new_kind := "pedestrian"
var new_width := 1.2
var new_two_way := true
## Aisles and objects outlined in orange (a warning was clicked).
var highlight: Array = []

var _press := {}
var _dragging := false
var _drag_at := Vector3.ZERO
var _apress := {}
var _adragging := false
var _adrag_at := Vector3.ZERO


func setup(owner_main: Node, owner_editor: EditorUtsim) -> void:
	main = owner_main
	editor = owner_editor


func doc() -> Object:
	return main.doc


func active() -> bool:
	return editor.tool == "corredor"


## After every change of the document.
func sync() -> void:
	list = doc().aisles()
	crossings = doc().crossings()
	obstacles = doc().obstacles()
	access = doc().access_points()
	highlight = []
	if _aisle(selected).is_empty():
		selected = ""
		selected_vertex = -1
	elif selected_vertex >= (_aisle(selected)["points"] as PackedVector3Array).size():
		selected_vertex = -1


func _aisle(name: String) -> Dictionary:
	for a in list:
		if a["name"] == name:
			return a
	return {}


func counts() -> Dictionary:
	var out := {"total": list.size(), "pedestre": 0, "empilhadeira": 0, "misto": 0,
		"cruzamentos": crossings.size()}
	for a in list:
		match a["kind"]:
			"pedestrian":
				out["pedestre"] += 1
			"forklift":
				out["empilhadeira"] += 1
			_:
				out["misto"] += 1
	return out


func clear() -> void:
	selected = ""
	selected_vertex = -1
	points = PackedVector3Array()
	highlight = []
	_press = {}
	_apress = {}
	_dragging = false
	_adragging = false


func set_obstacles_visible(on: bool) -> void:
	show_obstacles = on
	main.set_status("Obstáculos e folgas %s." % ("visíveis" if on else "ocultos"))


# ---------------------------------------------------------------------------
# Picking
# ---------------------------------------------------------------------------


func _screen(w: Vector3) -> Variant:
	var cam: Camera3D = main.camera
	if cam.is_position_behind(w):
		return null
	return cam.unproject_position(w)


## `[aisle, index]` of the vertex nearest to screen point `p` within `PICK_PIXELS`.
func vertex_at(p: Vector2) -> Array:
	var best: Array = []
	var best_d := PICK_PIXELS
	for a in list:
		var pts: PackedVector3Array = a["points"]
		for i in pts.size():
			var s = _screen(_vertex_world(a["name"], i, pts[i]))
			if s == null:
				continue
			var d: float = (s as Vector2).distance_to(p)
			# The selected aisle wins ties.
			if d < best_d or (d == best_d and a["name"] == selected):
				best_d = d
				best = [a["name"], i]
	return best


func _vertex_world(name: String, index: int, w: Vector3) -> Vector3:
	if _dragging and _press.get("vertex", "") == name and _press.get("index", -1) == index:
		return _drag_at
	return w


## `[aisle, segment]` of the centre line nearest to screen point `p`.
func line_at(p: Vector2) -> Array:
	var best: Array = []
	var best_d := LINE_PIXELS
	for a in list:
		var pts: PackedVector3Array = a["points"]
		for i in pts.size() - 1:
			var sa = _screen(pts[i])
			var sb = _screen(pts[i + 1])
			if sa == null or sb == null:
				continue
			var q := Geometry2D.get_closest_point_to_segment(p, sa, sb)
			var d := q.distance_to(p)
			if d < best_d or (d == best_d and a["name"] == selected):
				best_d = d
				best = [a["name"], i]
	return best


## Screen point of a vertex, or null (scripts).
func vertex_screen(name: String, index: int) -> Variant:
	var a := _aisle(name)
	if a.is_empty() or index >= (a["points"] as PackedVector3Array).size():
		return null
	return _screen((a["points"] as PackedVector3Array)[index])


## Screen point of the middle of the first segment of an aisle, or null (scripts).
func line_screen(name: String) -> Variant:
	var a := _aisle(name)
	if a.is_empty():
		return null
	var pts: PackedVector3Array = a["points"]
	return _screen(pts[0].lerp(pts[1], 0.5))


## Screen point of the access point of an object, or null (scripts).
func access_screen(object: String) -> Variant:
	for a in access:
		if a["object"] == object:
			return _screen(a["point"])
	return null


func _snapped(at: Vector3, skip := "", skip_index := -1) -> Vector3:
	return doc().aisle_snap(at, main.snap_step(), SNAP_METRES, skip, skip_index)


# ---------------------------------------------------------------------------
# Input of the Corredor tool (called by the editor while it is active)
# ---------------------------------------------------------------------------


func press(p: Vector2, shift: bool) -> void:
	_press = {}
	var at = editor.floor_at(p)
	if not points.is_empty():
		if at != null:
			_press = {"floor": at, "pos": p}
		return
	var v := vertex_at(p)
	if not v.is_empty() and not shift:
		_press = {"vertex": v[0], "index": v[1], "pos": p}
		return
	var l := line_at(p)
	if not l.is_empty() and not shift:
		_press = {"line": l[0], "pos": p}
		return
	if not l.is_empty() and shift and l[0] == selected and at != null:
		# Shift on the selected line: a bend in the middle of it.
		_press = {"insert": l[0], "segment": l[1], "floor": at, "pos": p}
		return
	if at == null:
		main.set_status("Aponte para o chão para pôr um ponto do corredor.")
		return
	_press = {"floor": at, "pos": p}


func drag(p: Vector2) -> void:
	if not _press.has("vertex"):
		return
	if not _dragging and p.distance_to(_press["pos"]) < DRAG_PIXELS:
		return
	var at = editor.floor_at(p)
	if at == null:
		return
	_dragging = true
	_drag_at = _snapped(at, _press["vertex"], _press["index"])


func release(_p: Vector2) -> void:
	if _press.is_empty():
		return
	var pr := _press
	_press = {}
	if pr.has("vertex"):
		var name: String = pr["vertex"]
		if _dragging:
			_dragging = false
			if doc().move_aisle_point(name, pr["index"], _drag_at):
				selected = name
				selected_vertex = pr["index"]
				main.after_edit()
				main.on_aisle_selection()
				main.set_status("Vértice %d de %s movido." % [int(pr["index"]) + 1, name])
			else:
				main.set_status("Não moveu: %s" % doc().last_error())
			return
		_select(name, pr["index"])
		return
	if pr.has("line"):
		_select(pr["line"], -1)
		return
	if pr.has("insert"):
		var at: Vector3 = _snapped(pr["floor"])
		if doc().insert_aisle_point(pr["insert"], pr["segment"], at):
			selected_vertex = int(pr["segment"]) + 1
			main.after_edit()
			main.on_aisle_selection()
			main.set_status("Vértice acrescentado a %s." % pr["insert"])
		else:
			main.set_status("Não acrescentou: %s" % doc().last_error())
		return
	var at: Vector3 = _snapped(pr["floor"])
	if not points.is_empty() and at.distance_to(points[points.size() - 1]) < 0.02:
		finish()
		return
	selected = ""
	selected_vertex = -1
	points.append(at)
	main.on_aisle_selection()
	if points.size() == 1:
		main.set_status("Corredor: clique os próximos pontos; Enter ou duplo clique termina, Backspace volta um ponto, Esc cancela.")
	else:
		main.set_status("%d ponto(s). Enter ou duplo clique termina." % points.size())


func _select(name: String, vertex: int) -> void:
	selected = name
	selected_vertex = vertex
	highlight = []
	main.on_aisle_selection()
	if vertex >= 0:
		main.set_status("Vértice %d de %s selecionado (arraste para mover)." % [vertex + 1, name])
	else:
		main.set_status("Corredor %s selecionado (Del exclui; Shift + clique na linha acrescenta um vértice)." % name)


## Ends the aisle being drawn (Enter, or a double click on its last point).
func finish() -> bool:
	if points.is_empty():
		return false
	var line := points
	points = PackedVector3Array()
	if line.size() < 2:
		main.set_status("Um corredor precisa de ao menos 2 pontos.")
		return true
	var name: String = doc().add_aisle(line, new_kind, new_width, new_two_way)
	if name == "":
		main.set_status("Não criou o corredor: %s" % doc().last_error())
		return true
	selected = name
	selected_vertex = -1
	main.after_edit()
	main.on_aisle_selection()
	main.set_status("Corredor %s criado (%s, %s m)." % [name, KIND_LABELS[KINDS.find(new_kind)].to_lower(), _num(new_width)])
	return true


## Esc: cancels the aisle being drawn, else the selection; true when there was one.
func cancel() -> bool:
	if not points.is_empty():
		points = PackedVector3Array()
		main.on_aisle_selection()
		main.set_status("Corredor cancelado. Clique no chão para começar outro, ou Esc para sair da ferramenta.")
		return true
	if selected != "":
		selected = ""
		selected_vertex = -1
		main.on_aisle_selection()
		return true
	return false


## Del or Backspace: the last point while drawing, else the selected aisle.
func delete_selected() -> bool:
	if not points.is_empty():
		points.remove_at(points.size() - 1)
		main.on_aisle_selection()
		main.set_status("Ponto removido (%d)." % points.size())
		return true
	if selected != "":
		var name := selected
		if doc().remove_aisle(name):
			selected = ""
			selected_vertex = -1
			main.after_edit()
			main.on_aisle_selection()
			main.set_status("Corredor %s excluído." % name)
		else:
			main.set_status("Não excluiu: %s" % doc().last_error())
		return true
	return false


# ---------------------------------------------------------------------------
# Access points (Select tool, with the obstacles shown)
# ---------------------------------------------------------------------------


func access_active() -> bool:
	return not _apress.is_empty()


## A press on an access point starts dragging it; false when none is there.
func access_press(p: Vector2) -> bool:
	if not show_obstacles:
		return false
	var best := ""
	var best_d := PICK_PIXELS
	for a in access:
		var s = _screen(a["point"])
		if s == null:
			continue
		var d: float = (s as Vector2).distance_to(p)
		if d <= best_d:
			best_d = d
			best = a["object"]
	if best == "":
		return false
	# A resource standing on the point (a person at her machine) is what a plain click
	# picks; dragging from there moves the point.
	var click := best
	for n in editor.hits_at(p):
		var i: int = editor.cena().block_index(n)
		if i >= 0 and editor.cena().blocks[i]["mobile"]:
			click = n
			break
	_apress = {"object": best, "click": click, "pos": p}
	return true


func access_drag(p: Vector2) -> void:
	if _apress.is_empty():
		return
	if not _adragging and p.distance_to(_apress["pos"]) < DRAG_PIXELS:
		return
	var at = editor.floor_at(p)
	if at == null:
		return
	_adragging = true
	_adrag_at = editor.snap_point(at)


func access_release(_p: Vector2) -> void:
	if _apress.is_empty():
		return
	var pr := _apress
	_apress = {}
	var object: String = pr["object"]
	if not _adragging:
		editor.select_only(pr["click"])
		return
	_adragging = false
	if doc().set_access_at(object, _adrag_at, main.snap_step()):
		main.after_edit()
		main.set_status("Ponto de acesso de %s movido (desligue \"Ponto de acesso próprio\" no painel para voltar ao padrão)." % object)
	else:
		main.set_status("Não moveu o ponto de acesso: %s" % doc().last_error())


# ---------------------------------------------------------------------------
# Edits from the panel
# ---------------------------------------------------------------------------


func set_kind(kind: String) -> void:
	if selected == "":
		new_kind = kind
		new_width = KIND_WIDTHS[kind]
		return
	if doc().set_aisle_kind(selected, kind):
		main.after_edit()
		main.set_status("Corredor %s agora é de %s." % [selected, KIND_LABELS[KINDS.find(kind)].to_lower()])


func set_width(width: float) -> void:
	if selected == "":
		new_width = width
		return
	if doc().set_aisle_width(selected, width):
		main.after_edit()
		main.set_status("Corredor %s com %s m de largura." % [selected, _num(width)])
	else:
		main.set_status("Não mudou a largura: %s" % doc().last_error())


func set_two_way(on: bool) -> void:
	if selected == "":
		new_two_way = on
		return
	if doc().set_aisle_two_way(selected, on):
		main.after_edit()
		main.set_status("Corredor %s em %s." % [selected, "mão dupla" if on else "mão única (setas no piso)"])


func set_secondary(on: bool) -> void:
	if selected != "" and doc().set_aisle_secondary(selected, on):
		main.after_edit()


func set_profile(profile: String) -> void:
	if selected != "" and doc().set_aisle_profile(selected, profile):
		main.after_edit()
		main.set_status("Perfil do corredor %s: %s." % [selected, profile if profile != "" else "o do modelo"])


func reverse() -> void:
	if selected != "" and doc().reverse_aisle(selected):
		main.after_edit()
		main.set_status("Sentido do corredor %s invertido." % selected)


func rename(to: String) -> void:
	if selected == "":
		return
	var from := selected
	if doc().rename_aisle(from, to):
		selected = to.strip_edges()
		main.after_edit()
		main.set_status("Corredor %s agora se chama %s." % [from, selected])
	else:
		main.set_status("Não renomeou: %s" % doc().last_error())


func remove_vertex() -> void:
	if selected == "" or selected_vertex < 0:
		return
	if doc().remove_aisle_point(selected, selected_vertex):
		selected_vertex = -1
		main.after_edit()
		main.on_aisle_selection()
		main.set_status("Vértice removido.")
	else:
		main.set_status("Não removeu: %s" % doc().last_error())


## Outlines in the scene what a warning talks about: the aisles and the objects named
## between single quotes (or at the start) in `message`. Returns what was found.
func highlight_issue(message: String) -> Array:
	var found := []
	for a in list:
		if message.contains("'%s'" % a["name"]):
			found.append(a["name"])
	for o in editor._objects:
		var n: String = o["name"]
		if message.contains("'%s'" % n) or message.begins_with(n + ":") or message.begins_with(n + "."):
			found.append(n)
	highlight = found
	if found.is_empty():
		main.set_status("Esse aviso não aponta para nenhum corredor ou objeto da cena.")
	else:
		main.set_status("Destacado na cena: %s." % ", ".join(found))
	return found


func _num(v: float) -> String:
	return ("%.2f" % v).replace(".", ",")


# ---------------------------------------------------------------------------
# Drawing (inside the editor's overlay surface)
# ---------------------------------------------------------------------------


func draw(e: EditorUtsim) -> void:
	if show_obstacles:
		_draw_obstacles(e)
	if active():
		for a in list:
			_draw_aisle(e, a)
		_draw_drawing(e)
	_draw_highlight(e)


func _lift(w: Vector3, y: float) -> Vector3:
	return Vector3(w.x, y, w.z)


## A closed outline, as thin ribbons.
func _loop(e: EditorUtsim, poly: PackedVector3Array, y: float, width: float, color: Color) -> void:
	for i in poly.size():
		var a := _lift(poly[i], y)
		var b := _lift(poly[(i + 1) % poly.size()], y)
		var d := (b - a).normalized() * width * 0.5
		e._ribbon(a - d, b + d, width, color)


## The two edges of the band of width `width` along `pts`.
func _band(e: EditorUtsim, pts: PackedVector3Array, width: float, y: float, line: float, color: Color) -> void:
	for i in pts.size() - 1:
		var a := _lift(pts[i], y)
		var b := _lift(pts[i + 1], y)
		var d := b - a
		d.y = 0.0
		if d.length() < 1e-5:
			continue
		var side := Vector3(-d.z, 0, d.x).normalized() * width * 0.5
		e._ribbon(a + side, b + side, line, color)
		e._ribbon(a - side, b - side, line, color)


func _draw_aisle(e: EditorUtsim, a: Dictionary) -> void:
	var pts: PackedVector3Array = a["points"]
	var name: String = a["name"]
	var sel := name == selected
	if _dragging and _press.get("vertex", "") == name:
		pts = pts.duplicate()
		pts[_press["index"]] = _drag_at
	var color := SELECTED if sel else LINE
	for i in pts.size() - 1:
		e._ribbon(_lift(pts[i], 0.05), _lift(pts[i + 1], 0.05), 0.16 if sel else 0.10, color)
		if not a["two_way"]:
			var m := _lift(pts[i], 0.06).lerp(_lift(pts[i + 1], 0.06), 0.5)
			var d := (pts[i + 1] - pts[i])
			d.y = 0.0
			if d.length() > 0.8:
				d = d.normalized()
				e._arrow(m - d * 0.45, m + d * 0.45, color, 0.12)
	if sel:
		_band(e, pts, float(a["width"]), 0.04, 0.05, color)
	for i in pts.size():
		var big := sel and i == selected_vertex
		_diamond(e, pts[i], 0.34 if big else (0.24 if sel else 0.16), ARMED_VERTEX if big else NODE)


const ARMED_VERTEX := Color(0.45, 0.95, 0.55)


func _draw_drawing(e: EditorUtsim) -> void:
	if points.is_empty():
		return
	var pts := PackedVector3Array(points)
	var at = e.floor_at(e.mouse)
	if at != null and not main.over_ui(e.mouse):
		pts.append(_snapped(at))
	for i in pts.size() - 1:
		e._ribbon(_lift(pts[i], 0.07), _lift(pts[i + 1], 0.07), 0.12, DRAWING)
	_band(e, pts, new_width, 0.05, 0.05, DRAWING)
	for i in points.size():
		_diamond(e, points[i], 0.28, DRAWING)
	if pts.size() > points.size():
		_diamond(e, pts[pts.size() - 1], 0.16, DRAWING)


func _draw_obstacles(e: EditorUtsim) -> void:
	for o in obstacles:
		var poly: PackedVector3Array = o["polygon"]
		if poly.size() < 3:
			continue
		var wall: bool = o["kind"] == "parede"
		_loop(e, poly, 0.045, 0.07, WALL if wall else OBJECT)
		# The clearance kept from it: the outline grown by it.
		var flat := PackedVector2Array()
		for p in poly:
			flat.append(Vector2(p.x, p.z))
		var grown := Geometry2D.offset_polygon(flat, float(o["clearance"]), Geometry2D.JOIN_ROUND)
		if grown.size() > 0:
			var g := PackedVector3Array()
			for q in grown[0]:
				g.append(Vector3(q.x, 0.0, q.y))
			_loop(e, g, 0.035, 0.035, HALO)
	for a in access:
		var at: Vector3 = a["point"]
		if _adragging and _apress.get("object", "") == a["object"]:
			at = _adrag_at
		var c: Vector3 = a["center"]
		e._ribbon(_lift(c, 0.06), _lift(at, 0.06), 0.03, ACCESS_OWN if a["custom"] else ACCESS)
		_diamond(e, at, 0.26, ACCESS_OWN if a["custom"] else ACCESS)


func _draw_highlight(e: EditorUtsim) -> void:
	if highlight.is_empty():
		return
	var pulse := 0.10 + 0.05 * sin(Time.get_ticks_msec() * 0.008)
	for n in highlight:
		var a := _aisle(n)
		if not a.is_empty():
			var pts: PackedVector3Array = a["points"]
			_band(e, pts, float(a["width"]) + 0.5, 0.08, pulse, WARN)
			continue
		var i: int = e.cena().block_index(n)
		if i >= 0:
			e._rect(e._box_of(i).grow(0.3), WARN, pulse)


func _diamond(e: EditorUtsim, at: Vector3, r: float, color: Color) -> void:
	var c := Vector3(at.x, 0.09, at.z)
	var pts := [c + Vector3(r, 0, 0), c + Vector3(0, 0, r), c + Vector3(-r, 0, 0), c + Vector3(0, 0, -r)]
	for k in [[0, 1, 2], [0, 2, 3]]:
		for j in k:
			e._mesh.surface_set_color(color)
			e._mesh.surface_add_vertex(pts[j])

## The path network in Editar (phase G3): the "Rede" tool (N) draws and edits the nodes
## and edges of `Model.network` through `UtsimDocument` (`add_node`, `move_node`,
## `add_edge`, `set_edge`...), so every change is a command with undo.
##
## - Clicking the floor adds a node; while a node is "armed" (the last one placed or
##   clicked, green), the new node is joined to it by an edge, so a path is drawn one
##   click at a time. Clicking another node joins the armed node to it. Esc disarms
##   (a second Esc leaves the tool).
## - Dragging a node moves it (one undo step, snapped to the grid).
## - Clicking an edge selects it: the panel changes direction (one way or both),
##   capacity (vehicles at the same time: a controlled lane, drawn in red as in the
##   Bevy viewer) and turns it around; Del removes the selected node or edge.
##
## The network is drawn over the scene by the editor's overlay (`draw`), in every tool:
## edges as ribbons (amber, red when controlled, an arrow in the middle when one way),
## nodes as diamonds.
class_name EditorRede
extends Node

const PICK_PIXELS := 14.0
const EDGE_PIXELS := 9.0
const DRAG_PIXELS := 4.0
const FREE := Color(0.95, 0.75, 0.25)
const CONTROLLED := Color(1.0, 0.30, 0.25)
const SELECTED := Color(1.0, 0.95, 0.55)
const NODE := Color(0.92, 0.94, 0.98)
const ARMED := Color(0.45, 0.95, 0.55)

var main: Node
var editor: EditorUtsim
## `{nodes: [...], edges: [...]}` of `UtsimDocument.network()`.
var net: Dictionary = {"nodes": [], "edges": []}
## Node the next click joins from ("" when none).
var armed := ""
var selected_node := ""
## `[from, to]` of the selected edge, or empty.
var selected_edge: Array = []
## New edges are two-way.
var two_way := true

var _press := {}
var _dragging := false
var _drag_at := Vector3.ZERO


func setup(owner_main: Node, owner_editor: EditorUtsim) -> void:
	main = owner_main
	editor = owner_editor


func doc() -> Object:
	return main.doc


func active() -> bool:
	return editor.tool == "rede"


## After every change of the document.
func sync(network: Dictionary) -> void:
	net = network
	var names := {}
	for n in net["nodes"]:
		names[n["name"]] = true
	if not names.has(armed):
		armed = ""
	if not names.has(selected_node):
		selected_node = ""
	if not selected_edge.is_empty() and _edge(selected_edge[0], selected_edge[1]).is_empty():
		selected_edge = []


func _edge(from: String, to: String) -> Dictionary:
	for e in net["edges"]:
		if e["from"] == from and e["to"] == to:
			return e
	return {}


func node_position(name: String) -> Variant:
	for n in net["nodes"]:
		if n["name"] == name:
			if _dragging and _press.get("node", "") == name:
				return _drag_at
			return n["position"]
	return null


func counts() -> Dictionary:
	var controlled := 0
	for e in net["edges"]:
		if e["controlled"]:
			controlled += 1
	return {"nos": net["nodes"].size(), "trechos": net["edges"].size(), "controlados": controlled}


func clear() -> void:
	armed = ""
	selected_node = ""
	selected_edge = []
	_press = {}
	_dragging = false


# ---------------------------------------------------------------------------
# Picking
# ---------------------------------------------------------------------------


func node_at(p: Vector2) -> String:
	var cam: Camera3D = main.camera
	var best := ""
	var best_d := PICK_PIXELS
	for n in net["nodes"]:
		var w: Vector3 = n["position"]
		if cam.is_position_behind(w):
			continue
		var d := cam.unproject_position(w).distance_to(p)
		if d <= best_d:
			best_d = d
			best = n["name"]
	return best


func edge_at(p: Vector2) -> Array:
	var cam: Camera3D = main.camera
	var best: Array = []
	var best_d := EDGE_PIXELS
	for e in net["edges"]:
		var a = node_position(e["from"])
		var b = node_position(e["to"])
		if a == null or b == null or cam.is_position_behind(a) or cam.is_position_behind(b):
			continue
		var sa := cam.unproject_position(a)
		var sb := cam.unproject_position(b)
		var q := Geometry2D.get_closest_point_to_segment(p, sa, sb)
		var d := q.distance_to(p)
		if d <= best_d:
			best_d = d
			best = [e["from"], e["to"]]
	return best


## Screen point of a node, or null (scripts).
func node_screen(name: String) -> Variant:
	var w = node_position(name)
	if w == null or main.camera.is_position_behind(w):
		return null
	return main.camera.unproject_position(w)


## Screen point of the middle of an edge, or null (scripts).
func edge_screen(from: String, to: String) -> Variant:
	var a = node_position(from)
	var b = node_position(to)
	if a == null or b == null:
		return null
	var m: Vector3 = ((a as Vector3) + (b as Vector3)) * 0.5
	if main.camera.is_position_behind(m):
		return null
	return main.camera.unproject_position(m)


# ---------------------------------------------------------------------------
# Input (called by the editor while the tool is active)
# ---------------------------------------------------------------------------


func press(p: Vector2, _shift: bool) -> void:
	var n := node_at(p)
	if n != "":
		_press = {"node": n, "pos": p}
		return
	var e := edge_at(p)
	if not e.is_empty():
		selected_edge = e
		selected_node = ""
		_press = {}
		main.on_network_selection()
		main.set_status("Trecho %s → %s selecionado (Del remove)." % e)
		return
	var at = editor.floor_at(p)
	if at == null:
		main.set_status("Aponte para o chão para colocar um nó.")
		_press = {}
		return
	_press = {"floor": at, "pos": p}


func drag(p: Vector2) -> void:
	if not _press.has("node"):
		return
	if not _dragging and p.distance_to(_press["pos"]) < DRAG_PIXELS:
		return
	var at = editor.floor_at(p)
	if at == null:
		return
	_dragging = true
	_drag_at = editor.snap_point(at)


func release(p: Vector2) -> void:
	if _press.is_empty():
		return
	var pr := _press
	_press = {}
	if pr.has("node"):
		var n: String = pr["node"]
		if _dragging:
			_dragging = false
			if doc().move_node(n, _drag_at, main.snap_step()):
				main.after_edit()
				main.set_status("Nó %s movido." % n)
			else:
				main.set_status("Não moveu: %s" % doc().last_error())
			return
		if armed != "" and armed != n:
			if doc().add_edge(armed, n, two_way):
				main.after_edit()
				main.set_status("Trecho %s → %s criado (%s)." % [armed, n, "mão dupla" if two_way else "mão única"])
			else:
				main.set_status("Não ligou: %s" % doc().last_error())
		armed = n
		selected_node = n
		selected_edge = []
		main.on_network_selection()
		return
	var at: Vector3 = pr["floor"]
	var name: String = doc().add_node(at, main.snap_step(), armed, two_way)
	if name == "":
		main.set_status("Não colocou o nó: %s" % doc().last_error())
		return
	var from := armed
	armed = name
	selected_node = name
	selected_edge = []
	main.after_edit()
	main.on_network_selection()
	if from != "":
		main.set_status("Nó %s ligado a %s. Clique de novo para continuar o caminho (Esc para)." % [name, from])
	else:
		main.set_status("Nó %s colocado. Clique em outro ponto para traçar um trecho (Esc para)." % name)


## Esc: stops the path being drawn; true when there was one.
func cancel() -> bool:
	if armed != "":
		armed = ""
		main.set_status("Caminho encerrado. Clique no chão para começar outro, ou Esc para sair da ferramenta.")
		return true
	return false


func delete_selected() -> bool:
	if not selected_edge.is_empty():
		var e := selected_edge
		if doc().remove_edge(e[0], e[1]):
			selected_edge = []
			main.after_edit()
			main.on_network_selection()
			main.set_status("Trecho %s → %s removido." % e)
		return true
	if selected_node != "":
		var n := selected_node
		if doc().remove_nodes(PackedStringArray([n])):
			selected_node = ""
			main.after_edit()
			main.on_network_selection()
			main.set_status("Nó %s removido com os trechos dele." % n)
		return true
	return false


# ---------------------------------------------------------------------------
# Edits from the panel
# ---------------------------------------------------------------------------


func set_two_way(on: bool) -> void:
	if selected_edge.is_empty():
		return
	var e := _edge(selected_edge[0], selected_edge[1])
	if doc().set_edge(selected_edge[0], selected_edge[1], on, int(e.get("capacity", 0))):
		main.after_edit()
		main.set_status("Trecho %s → %s em %s." % [selected_edge[0], selected_edge[1], "mão dupla" if on else "mão única"])


func set_capacity(capacity: int) -> void:
	if selected_edge.is_empty():
		return
	var e := _edge(selected_edge[0], selected_edge[1])
	if doc().set_edge(selected_edge[0], selected_edge[1], bool(e.get("two_way", true)), capacity):
		main.after_edit()
		if capacity > 0:
			main.set_status("Pista controlada: até %d veículo(s) ao mesmo tempo no trecho." % capacity)
		else:
			main.set_status("Trecho sem controle de tráfego.")


func reverse() -> void:
	if selected_edge.is_empty():
		return
	var e := selected_edge
	if doc().reverse_edge(e[0], e[1]):
		selected_edge = [e[1], e[0]]
		main.after_edit()
		main.set_status("Trecho invertido: %s → %s." % [e[1], e[0]])


func rename(to: String) -> void:
	if selected_node == "":
		return
	var from := selected_node
	if doc().rename_node(from, to):
		selected_node = to.strip_edges()
		if armed == from:
			armed = selected_node
		main.after_edit()
		main.set_status("Nó %s agora se chama %s." % [from, selected_node])
	else:
		main.set_status("Não renomeou: %s" % doc().last_error())


# ---------------------------------------------------------------------------
# Drawing (inside the editor's overlay surface)
# ---------------------------------------------------------------------------


func draw(e: EditorUtsim) -> void:
	var lift := Vector3(0, 0.05, 0)
	var width := 0.22 if active() else 0.14
	for edge in net["edges"]:
		var a = node_position(edge["from"])
		var b = node_position(edge["to"])
		if a == null or b == null:
			continue
		var color := CONTROLLED if edge["controlled"] else FREE
		if selected_edge.size() == 2 and selected_edge[0] == edge["from"] and selected_edge[1] == edge["to"]:
			color = SELECTED
		var pa: Vector3 = (a as Vector3) + lift
		var pb: Vector3 = (b as Vector3) + lift
		e._ribbon(pa, pb, width, color)
		if not edge["two_way"]:
			var mid := pa.lerp(pb, 0.5)
			var d := (pb - pa).normalized()
			e._arrow(mid - d * 0.35, mid + d * 0.35, color, width * 0.6)
	if not active() and net["nodes"].size() > 0:
		for n in net["nodes"]:
			_diamond(e, node_position(n["name"]), 0.16, NODE)
		return
	for n in net["nodes"]:
		var name: String = n["name"]
		var color := NODE
		if name == armed:
			color = ARMED
		elif name == selected_node:
			color = SELECTED
		_diamond(e, node_position(name), 0.32 if name == armed or name == selected_node else 0.24, color)
	if armed != "" and _press.is_empty():
		var a = node_position(armed)
		var b = e.floor_at(e.mouse)
		if a != null and b != null and not main.over_ui(e.mouse):
			var pa: Vector3 = a
			var pb: Vector3 = e.snap_point(b)
			pa.y = 0.07
			pb.y = 0.07
			e._ribbon(pa, pb, 0.06, ARMED)
			_diamond(e, pb, 0.16, ARMED)


func _diamond(e: EditorUtsim, at: Variant, r: float, color: Color) -> void:
	if at == null:
		return
	var c: Vector3 = (at as Vector3) + Vector3(0, 0.08, 0)
	var pts := [c + Vector3(r, 0, 0), c + Vector3(0, 0, r), c + Vector3(-r, 0, 0), c + Vector3(0, 0, -r)]
	for k in [[0, 1, 2], [0, 2, 3]]:
		for j in k:
			e._mesh.surface_set_color(color)
			e._mesh.surface_add_vertex(pts[j])

## Belts and floor paths drawn by nodes in Editar, the way roads are laid out in a
## city-builder: the "Esteira desenhada", "Esteira com canaletas" and "Caminho no piso"
## entries of the library arm the "trajeto" tool. Every edit goes through `UtsimTrajetos`
## (`path_godot.rs`, `path_edit.rs`) as a command with undo; the 3D look of the belts and
## ramps is the scene's (`trajeto_malha.gd`).
##
## Drawing:
## - each click puts a node: on a port (the end of a belt, at its height, or the centre of
##   an object), on a node of another route or a point of a floor path, else at an angle
##   in steps of 15° from the last node, else on the grid. The segment being drawn shows
##   its length and slope live; red when it is steeper than the tool allows;
## - PageUp / PageDown raise or lower the next node 10 cm (with Shift, 1 cm): belts start
##   at 0,8 m (a workbench), paths on the floor (a ramp when they go up);
## - [ and ] round the next corner (the radius of the curve, 0,25 m a step);
## - Enter, or a second click on the last node, ends it; Backspace takes the last node
##   back; Esc cancels. A belt that starts or ends on a port is connected to it.
##
## Editing (no route being drawn):
## - a click on a node selects it (Shift adds nodes of the same route); a click on a
##   route's line with Shift splits that segment with a new node;
## - the gizmo of a selected node: drag the diamond on the floor to move it in X and Y
##   (snapping as when drawing), drag the top of its post to raise or lower it;
## - arrows move the selected nodes one grid step (Shift: 1 cm), PageUp / PageDown raise
##   or lower them (10 cm; Shift: 1 cm), [ and ] change the radius; Del removes them;
## - the right panel takes exact X, Y and height, the radius, the type, the drive, the
##   width, and levels the selection or puts it at a constant slope.
class_name EditorTrajetos
extends Node

const BELT_SIMPLE := "EsteiraDesenhada"
const BELT_CHANNELED := "EsteiraCanaletas"
const FLOOR_PATH := "CaminhoPiso"
const IDS := [BELT_SIMPLE, BELT_CHANNELED, FLOOR_PATH]
const LABELS := {BELT_SIMPLE: "Esteira desenhada", BELT_CHANNELED: "Esteira com canaletas",
	FLOOR_PATH: "Caminho no piso"}
const BELT_HEIGHT := 0.8
const PICK_PIXELS := 12.0
const DRAG_PIXELS := 4.0
const SNAP_METRES := 0.45
const LINE_METRES := 0.5
const HEIGHT_STEP := 0.1
const FINE_STEP := 0.01
const RADIUS_STEP := 0.25
const DRAWING := Color(0.45, 0.95, 0.55)
const ROUTE := Color(0.55, 0.80, 1.0)
const PATH := Color(0.95, 0.85, 0.30)
const SELECTED := Color(1.0, 0.97, 0.60)
const STEEP := Color(1.0, 0.35, 0.25)
const STICK := Color(0.85, 0.88, 0.95, 0.8)
## The curve handles in the middle of the segments of the selected route.
const CURVE := Color(1.0, 0.62, 0.20)

var main: Node
var editor: EditorUtsim
var tr: Object
## The tool armed (one of `IDS`).
var mode := BELT_SIMPLE
## The route being drawn: nodes in Godot coordinates (`y` is the height) and radii.
var points := PackedVector3Array()
var radii := PackedFloat64Array()
## Ports where the route being drawn starts and ends (objects to connect).
var start_port := {}
var end_port := {}
## Height of the next node and radius of the next corner.
var draw_z := BELT_HEIGHT
var new_radius := 0.0
var path_width := 1.2
## Distance (m) of a parallel route; positive is to the left of the way.
var parallel_dist := 1.0
## Fixed slope while drawing: the next node's height follows from its distance in plan.
var grade_on := false
var grade_pct := 5.0
## `UtsimTrajetos.esteiras()` and `caminhos()`.
var belts: Array = []
var paths: Array = []
## Selection: one route (a belt or a path) and some of its nodes.
var sel_name := ""
var sel_path := false
var sel_nodes := PackedInt32Array()
var _press := {}
var _dragging := false
var _drag_at := Vector3.ZERO
var _readout: Label3D
## The length of the next segment being typed (digits and a comma), "" when none.
var typed := ""
## Where the mouse points while drawing (the direction a typed length goes).
var _cursor: Variant = null
## Direction of the route a new one goes on from (its last segment), null when none.
var _start_tangent: Variant = null


func setup(owner_main: Node, owner_editor: EditorUtsim) -> void:
	main = owner_main
	editor = owner_editor
	if ClassDB.class_exists("UtsimTrajetos"):
		tr = ClassDB.instantiate("UtsimTrajetos")
	_readout = Label3D.new()
	_readout.name = "LeituraTrajeto"
	_readout.billboard = BaseMaterial3D.BILLBOARD_ENABLED
	_readout.fixed_size = true
	_readout.no_depth_test = true
	_readout.pixel_size = 0.0006
	_readout.font_size = 26
	_readout.outline_size = 10
	_readout.outline_modulate = Color(0.06, 0.07, 0.09, 0.95)
	_readout.render_priority = 12
	_readout.outline_render_priority = 11
	_readout.visible = false
	add_child(_readout)


func doc() -> Object:
	return main.doc


func active() -> bool:
	return editor.tool == "trajeto" and tr != null


static func is_route_tool(id: String) -> bool:
	return id in IDS


func is_path() -> bool:
	return mode == FLOOR_PATH


## Arms the tool for library entry `id`.
func start(id: String) -> void:
	mode = id if id in IDS else BELT_SIMPLE
	_clear_drawing()
	draw_z = 0.0 if is_path() else BELT_HEIGHT
	sync()
	main.set_status("%s: clique no chão para pôr os nós (Enter ou duplo clique termina). PageUp/PageDown sobe e desce o próximo nó, [ e ] arredondam a curva. Clique num nó para editá-lo; Esc cancela." % LABELS[mode])


## After every change of the document.
func sync() -> void:
	if tr == null or doc() == null:
		return
	tr.ligar(doc())
	belts = tr.esteiras()
	paths = tr.caminhos()
	var r := _route(sel_name, sel_path)
	if r.is_empty():
		_select("", false, PackedInt32Array())
	else:
		var n := (r["nodes"] as PackedVector3Array).size()
		var keep := PackedInt32Array()
		for i in sel_nodes:
			if i < n:
				keep.append(i)
		sel_nodes = keep


func _route(name: String, path: bool) -> Dictionary:
	if name == "":
		return {}
	for r in (paths if path else belts):
		if r["name"] == name:
			return r
	return {}


func _clear_drawing() -> void:
	points = PackedVector3Array()
	radii = PackedFloat64Array()
	start_port = {}
	end_port = {}
	typed = ""
	_start_tangent = null
	_readout.visible = false


func _select(name: String, path: bool, nodes: PackedInt32Array) -> void:
	sel_name = name
	sel_path = path
	sel_nodes = nodes
	if main != null:
		main.on_aisle_selection()


func _num(v: float, digits := 2) -> String:
	return (("%." + str(digits) + "f") % v).replace(".", ",")


func _screen(w: Vector3) -> Variant:
	var cam: Camera3D = main.camera
	if cam.is_position_behind(w):
		return null
	return cam.unproject_position(w)


func _snap(at: Vector3, prev: Variant, skip := "") -> Dictionary:
	if prev != null:
		return tr.encaixe_tangente(at, main.snap_step(), SNAP_METRES, prev, _tangent(), skip)
	return tr.encaixe(at, main.snap_step(), SNAP_METRES, prev, skip)


## The way the route being drawn is going: its last segment, or the route it goes on from.
func _tangent() -> Variant:
	if points.size() >= 2:
		return points[points.size() - 1] - points[points.size() - 2]
	return _start_tangent


## Floor point under the mouse at the drawing height.
func _aim(p: Vector2) -> Variant:
	var at = editor.floor_at(p)
	if at == null:
		return null
	var v: Vector3 = at
	v.y = draw_z
	return v


# ---------------------------------------------------------------------------
# Mouse (called by the editor while the tool is active)
# ---------------------------------------------------------------------------


## The node handle under screen point `p`: {name, path, index, handle ("xy" | "z")}.
func _handle_at(p: Vector2) -> Dictionary:
	var best := {}
	var best_d := PICK_PIXELS
	for path in [false, true]:
		for r in (paths if path else belts):
			var nodes: PackedVector3Array = r["nodes"]
			for i in nodes.size():
				var top: Vector3 = nodes[i]
				var spots := [[top, "z" if top.y > 0.05 else "xy"]]
				if top.y > 0.05:
					spots.append([Vector3(top.x, 0.0, top.z), "xy"])
				for s in spots:
					var sp = _screen(s[0])
					if sp == null:
						continue
					var d: float = (sp as Vector2).distance_to(p)
					if d <= best_d:
						best_d = d
						best = {"name": r["name"], "path": path, "index": i, "handle": s[1]}
			# The curve handles: the middle of each segment of the selected route.
			if r["name"] != sel_name or path != sel_path:
				continue
			for i in nodes.size() - 1:
				var sp = _screen(nodes[i].lerp(nodes[i + 1], 0.5))
				if sp == null:
					continue
				var d: float = (sp as Vector2).distance_to(p)
				if d <= best_d:
					best_d = d
					best = {"name": r["name"], "path": path, "index": i, "handle": "curva"}
	return best


func press(p: Vector2, shift: bool) -> void:
	editor._feed()
	if not points.is_empty():
		_add_point(p)
		return
	var h := _handle_at(p)
	if not h.is_empty() and h["handle"] == "curva":
		var rc := _route(h["name"], h["path"])
		var ns: PackedVector3Array = rc["nodes"]
		_press = h.duplicate()
		_press["pos"] = p
		_press["from"] = ns[int(h["index"])].lerp(ns[int(h["index"]) + 1], 0.5)
		_dragging = false
		main.set_status("Arraste para curvar o trecho %d de %s." % [int(h["index"]) + 1, sel_name])
		return
	if not h.is_empty():
		var same: bool = h["name"] == sel_name and h["path"] == sel_path
		if shift and same:
			var nodes := PackedInt32Array(sel_nodes)
			var k := nodes.find(int(h["index"]))
			if k >= 0:
				nodes.remove_at(k)
			else:
				nodes.append(int(h["index"]))
			_select(sel_name, sel_path, nodes)
			return
		if not (same and int(h["index"]) in sel_nodes):
			_select(h["name"], h["path"], PackedInt32Array([int(h["index"])]))
		var r := _route(h["name"], h["path"])
		_press = h.duplicate()
		_press["pos"] = p
		_press["from"] = (r["nodes"] as PackedVector3Array)[int(h["index"])]
		_dragging = false
		_status_node()
		return
	var at = _aim(p)
	if at == null:
		return
	if shift and sel_name != "":
		var seg: Dictionary = tr.trecho_perto(at, LINE_METRES)
		if not seg.is_empty() and seg["name"] == sel_name and seg["path"] == sel_path:
			var r := _route(sel_name, sel_path)
			var nodes: PackedVector3Array = r["nodes"]
			var i := int(seg["index"])
			# The new node at the height the segment has there.
			var a := nodes[i]
			var b := nodes[i + 1]
			var ab := Vector2(b.x - a.x, b.z - a.z)
			var t := clampf(Vector2((at as Vector3).x - a.x, (at as Vector3).z - a.z).dot(ab) / maxf(ab.length_squared(), 1e-9), 0.0, 1.0)
			var q := a.lerp(b, t)
			if tr.inserir_no(sel_name, sel_path, i, q):
				main.after_edit()
				_select(sel_name, sel_path, PackedInt32Array([i + 1]))
				main.set_status("Nó acrescentado em %s (o trecho foi dividido)." % sel_name)
			else:
				main.set_status("Não acrescentou: %s" % tr.last_error())
			return
	# A new route.
	_select("", false, PackedInt32Array())
	_add_point(p)


func _add_point(p: Vector2) -> void:
	var at = _aim(p)
	if at == null:
		main.set_status("Aponte para o chão para pôr o nó.")
		return
	var prev: Variant = points[points.size() - 1] if not points.is_empty() else null
	var s := _snap(at, prev)
	var q: Vector3 = s["at"]
	# A second click on the last node ends the route.
	if not points.is_empty() and q.distance_to(points[points.size() - 1]) < 0.05:
		finish()
		return
	if not (s["what"] in ["porta", "no", "caminho"]):
		q = _graded(q)
		if grade_on:
			draw_z = q.y
	var port := {}
	if s["what"] == "porta":
		port = {"object": s["object"], "output": int(s["output"])}
		# A port at another height (the end of a belt) sets the height.
		draw_z = q.y
	elif s["what"] == "linha" and not is_path():
		# On the line of another belt: a T joint (merge at the end, divert at the start).
		port = {"object": s["object"], "output": -1, "line": true}
		draw_z = q.y
	if points.is_empty():
		start_port = port
	end_port = port
	points.append(q)
	radii.append(0.0)
	if points.size() >= 2 and new_radius > 0.0:
		radii[points.size() - 2] = new_radius
	main.set_status("%s: nó %d em (%s; %s) m, altura %s m (%s)." % [LABELS[mode], points.size(),
		_num(q.x), _num(-q.z), _num(q.y), _what(s["what"])])


## Starts a new route at `at` (the end of another one), snapped like a click there.
func _start_at(at: Vector3) -> void:
	var s := _snap(at, null)
	var q: Vector3 = s["at"]
	_select("", false, PackedInt32Array())
	_clear_drawing()
	draw_z = q.y
	if s["what"] == "porta":
		start_port = {"object": s["object"], "output": int(s["output"])}
	points.append(q)
	radii.append(0.0)
	main.set_status("%s: começa em (%s; %s) m, altura %s m. Clique os próximos nós; Enter termina." % [
		LABELS[mode], _num(q.x), _num(-q.z), _num(q.y)])


func _what(w: String) -> String:
	match w:
		"linha":
			return "na linha de outra esteira (junção em T)"
		"porta":
			return "encaixado na porta"
		"no":
			return "encaixado num nó"
		"caminho":
			return "encaixado no caminho"
		"tangente":
			return "seguindo o trecho anterior"
		"angulo":
			return "ângulo de 15 em 15°"
		"grade":
			return "na grade"
	return "livre"


func drag(p: Vector2) -> void:
	if _press.is_empty():
		return
	if not _dragging and p.distance_to(_press["pos"]) < DRAG_PIXELS:
		return
	_dragging = true
	var from: Vector3 = _press["from"]
	if _press["handle"] == "z":
		_drag_at = Vector3(from.x, _height_under(p, from), from.z)
	elif _press["handle"] == "curva":
		var at = editor.floor_at(p)
		if at == null:
			return
		var step: float = main.snap_step()
		var v: Vector3 = at
		if step > 0.0:
			v = Vector3(snappedf(v.x, step), 0.0, snappedf(v.z, step))
		_drag_at = Vector3(v.x, from.y, v.z)
	else:
		var at = editor.floor_at(p)
		if at == null:
			return
		var v: Vector3 = at
		v.y = from.y
		var lock := ""
		if Input.is_key_pressed(KEY_CTRL) or Input.is_key_pressed(KEY_META):
			lock = "x" if absf(v.x - from.x) >= absf(v.z - from.z) else "y"
		if lock != "":
			_drag_at = lock_axis(from, v, lock, main.snap_step())
		else:
			var s := _snap(v, null, "" if _press["path"] else String(_press["name"]))
			_drag_at = s["at"]
			if s["what"] != "porta" and s["what"] != "no":
				_drag_at.y = from.y
		_press["lock"] = lock
	var text := "X %s  Y %s  altura %s m" % [_num(_drag_at.x), _num(-_drag_at.z), _num(_drag_at.y)]
	match String(_press.get("lock", "")):
		"x":
			text += "\neixo X travado (Ctrl)"
		"y":
			text += "\neixo Y travado (Ctrl)"
	_readout_at(_drag_at, text, false)


## `to` moved from `from` only along the model's X axis (`axis` "x") or Y axis ("y"), the
## distance on the grid step; the height stays.
static func lock_axis(from: Vector3, to: Vector3, axis: String, step: float) -> Vector3:
	var out := from
	if axis == "x":
		out.x = from.x + (snappedf(to.x - from.x, step) if step > 0.0 else to.x - from.x)
	else:
		out.z = from.z + (snappedf(to.z - from.z, step) if step > 0.0 else to.z - from.z)
	return out


## Height the mouse points at on the vertical plane through `at` that faces the camera.
func _height_under(p: Vector2, at: Vector3) -> float:
	var cam: Camera3D = main.camera
	var o := cam.project_ray_origin(p)
	var d := cam.project_ray_normal(p)
	var n := cam.global_transform.basis.z
	n.y = 0.0
	if n.length() < 1e-4:
		return at.y
	n = n.normalized()
	var den := d.dot(n)
	if absf(den) < 1e-6:
		return at.y
	var t := (at - o).dot(n) / den
	var y := (o + d * t).y
	var step := FINE_STEP if Input.is_key_pressed(KEY_SHIFT) else 0.05
	return maxf(snappedf(y, step), 0.0)


func release(_p: Vector2) -> void:
	if _press.is_empty():
		return
	var pr := _press
	_press = {}
	_readout.visible = false
	if not _dragging:
		# A click (no drag) on the first or last node of a route starts a new one there,
		# as a road goes on from where another one ends.
		var r := _route(pr["name"], pr["path"])
		var n := (r["nodes"] as PackedVector3Array).size() if not r.is_empty() else 0
		var i := int(pr["index"])
		if n > 0 and (i == 0 or i == n - 1):
			var nodes: PackedVector3Array = r["nodes"]
			var tangent: Vector3 = nodes[n - 1] - nodes[n - 2] if i == n - 1 else nodes[0] - nodes[1]
			_start_at(pr["from"])
			_start_tangent = tangent
		return
	_dragging = false
	var from: Vector3 = pr["from"]
	var name: String = pr["name"]
	var path: bool = pr["path"]
	var ok: bool
	if pr["handle"] == "curva":
		ok = tr.curvar(name, path, int(pr["index"]), _drag_at)
		if ok:
			main.after_edit()
			_select(name, path, PackedInt32Array([int(pr["index"]) + 1]))
			main.set_status("Trecho curvado: nó %d com raio de %s m (Ctrl+Z desfaz)." % [int(pr["index"]) + 2,
				_num((_route(name, path)["radii"] as PackedFloat64Array)[int(pr["index"]) + 1])])
		else:
			main.set_status("Não curvou: %s" % tr.last_error())
		return
	if sel_nodes.size() > 1 and int(pr["index"]) in sel_nodes:
		ok = tr.deslocar(name, path, sel_nodes, _drag_at - from)
	else:
		ok = tr.por_no(name, path, int(pr["index"]), _drag_at)
	if ok:
		main.after_edit()
		_status_node()
	else:
		main.set_status("Não moveu: %s" % tr.last_error())


# ---------------------------------------------------------------------------
# Keys
# ---------------------------------------------------------------------------


func _unhandled_key_input(event: InputEvent) -> void:
	var k := event as InputEventKey
	if k == null or not k.pressed or not active() or not main.is_editing():
		return
	if k.ctrl_pressed or k.meta_pressed:
		return
	if key(k.keycode, k.shift_pressed):
		get_viewport().set_input_as_handled()


## A key of the tool; true when it was used (scripts call it too).
func key(code: int, shift: bool) -> bool:
	var drawing := not points.is_empty()
	var fine := FINE_STEP if shift else HEIGHT_STEP
	# Typing a length while drawing: digits and a comma, Enter puts the node.
	if drawing:
		var ch := _digit(code)
		if ch != "":
			if ch != "," or not typed.contains(","):
				typed += ch
			main.set_status("Comprimento do trecho: %s m (Enter põe o nó na direção do cursor, Backspace apaga)." % typed)
			return true
	match code:
		KEY_ENTER, KEY_KP_ENTER:
			if drawing and typed != "":
				return _place_typed()
			return finish()
		KEY_BACKSPACE:
			if drawing and typed != "":
				typed = typed.left(typed.length() - 1)
				return true
			if drawing:
				_back()
				return true
		KEY_PAGEUP, KEY_PAGEDOWN:
			var dz := fine if code == KEY_PAGEUP else -fine
			if drawing or sel_nodes.is_empty():
				draw_z = maxf(snappedf(draw_z + dz, FINE_STEP), 0.0)
				main.set_status("Próximo nó a %s m de altura." % _num(draw_z))
				return true
			return nudge(Vector3(0, dz, 0))
		KEY_BRACKETLEFT, KEY_BRACKETRIGHT:
			var dr := (0.05 if shift else RADIUS_STEP) * (1.0 if code == KEY_BRACKETRIGHT else -1.0)
			if drawing or sel_nodes.is_empty():
				new_radius = maxf(snappedf(new_radius + dr, 0.05), 0.0)
				main.set_status("Raio das próximas curvas: %s m." % _num(new_radius))
				return true
			return _radius_step(dr)
		KEY_LEFT, KEY_RIGHT, KEY_UP, KEY_DOWN:
			if sel_nodes.is_empty():
				return false
			var step: float = FINE_STEP if shift else (main.snap_step() if main.snap_step() > 0.0 else HEIGHT_STEP)
			var d := Vector3.ZERO
			match code:
				KEY_LEFT:
					d.x = -step
				KEY_RIGHT:
					d.x = step
				KEY_UP:
					d.z = -step
				KEY_DOWN:
					d.z = step
			return nudge(d)
	return false


## `q` at the height the fixed slope gives it from the last node (when it is on).
func _graded(q: Vector3) -> Vector3:
	if not grade_on or points.is_empty():
		return q
	var last := points[points.size() - 1]
	var run := Vector2(q.x - last.x, q.z - last.z).length()
	q.y = maxf(last.y + grade_pct / 100.0 * run, 0.0)
	return q


static func _digit(code: int) -> String:
	if code >= KEY_0 and code <= KEY_9:
		return str(code - KEY_0)
	if code >= KEY_KP_0 and code <= KEY_KP_9:
		return str(code - KEY_KP_0)
	if code in [KEY_COMMA, KEY_PERIOD, KEY_KP_PERIOD]:
		return ","
	return ""


## The node `typed` metres (3D, along the segment) from the last one, towards the cursor,
## at the height of the next node.
func _place_typed() -> bool:
	var length := typed.replace(",", ".").to_float()
	typed = ""
	var last := points[points.size() - 1]
	var dir := Vector3.RIGHT
	if _cursor != null:
		dir = (_cursor as Vector3) - last
	elif points.size() >= 2:
		dir = last - points[points.size() - 2]
	dir.y = 0.0
	if dir.length() < 1e-6:
		dir = Vector3.RIGHT
	dir = dir.normalized()
	var dz := draw_z - last.y
	var run: float
	if grade_on:
		run = length / sqrt(1.0 + pow(grade_pct / 100.0, 2.0))
		dz = grade_pct / 100.0 * run
		if last.y + dz < 0.0:
			main.set_status("Descendo %s %% por %s m, o nó ficaria abaixo do piso." % [_num(grade_pct, 1), _num(length)])
			return true
		draw_z = last.y + dz
	elif length <= absf(dz):
		main.set_status("O trecho de %s m não alcança a altura de %s m." % [_num(length), _num(draw_z)])
		return true
	else:
		run = sqrt(length * length - dz * dz)
	var q := last + dir * run
	q.y = draw_z
	points.append(q)
	radii.append(0.0)
	if points.size() >= 2 and new_radius > 0.0:
		radii[points.size() - 2] = new_radius
	end_port = {}
	main.set_status("%s: nó %d a %s m do anterior." % [LABELS[mode], points.size(), _num(length)])
	return true


## Moves the selected nodes by `d` (Godot), one undo step.
func nudge(d: Vector3) -> bool:
	if sel_nodes.is_empty():
		return false
	if tr.deslocar(sel_name, sel_path, sel_nodes, d):
		main.after_edit()
		_status_node()
	else:
		main.set_status("Não moveu: %s" % tr.last_error())
	return true


func _radius_step(dr: float) -> bool:
	var r := _route(sel_name, sel_path)
	if r.is_empty():
		return false
	var rs: PackedFloat64Array = r["radii"]
	for i in sel_nodes:
		tr.por_raio(sel_name, sel_path, i, maxf(rs[i] + dr, 0.0))
	main.after_edit()
	_status_node()
	return true


func _back() -> void:
	points.remove_at(points.size() - 1)
	radii.remove_at(radii.size() - 1)
	if points.is_empty():
		_clear_drawing()
	main.set_status("Último nó desfeito.")


## Ends the route being drawn (Enter, or a second click on the last node).
func finish() -> bool:
	if points.size() < 2:
		return false
	var pts := PackedVector3Array(points)
	var rs := PackedFloat64Array(radii)
	var name: String
	if is_path():
		name = tr.novo_caminho(pts, rs, path_width)
	else:
		# From an object or a belt's output end; to an object or a belt's input end.
		var from := ""
		var to := ""
		var from_line := ""
		var to_line := ""
		if not start_port.is_empty() and start_port.get("line", false):
			from_line = start_port["object"]
		elif not start_port.is_empty() and int(start_port["output"]) != 0:
			from = start_port["object"]
		if not end_port.is_empty() and end_port.get("line", false):
			to_line = end_port["object"]
		elif not end_port.is_empty() and int(end_port["output"]) != 1:
			to = end_port["object"]
		name = tr.nova_esteira_t(mode, pts, rs, from, to, from_line, to_line)
	_clear_drawing()
	if name == "":
		main.set_status("Não criou: %s" % tr.last_error())
		return true
	main.after_edit()
	_select(name, is_path(), PackedInt32Array())
	var r := _route(name, is_path())
	if not r.is_empty():
		main.set_status("%s %s criado: %s m%s." % [LABELS[mode], name, _num(float(r["length"])),
			"" if is_path() else ", %s s de passagem, cabem %d" % [_num(float(r["transit"]), 1), int(r["capacity"])]])
	return true


## Esc: the route being drawn, else the selection; true when there was one.
func cancel() -> bool:
	if not points.is_empty():
		_clear_drawing()
		main.set_status("Desenho cancelado. Clique no chão para começar outro, ou Esc para sair da ferramenta.")
		return true
	if sel_name != "":
		_select("", false, PackedInt32Array())
		return true
	return false


## Del: the last node while drawing, else the selected nodes.
func delete_selected() -> bool:
	if not points.is_empty():
		_back()
		return true
	if sel_nodes.is_empty():
		return false
	var nodes := Array(sel_nodes)
	nodes.sort()
	nodes.reverse()
	var done := 0
	for i in nodes:
		if tr.tirar_no(sel_name, sel_path, i):
			done += 1
	if done > 0:
		main.after_edit()
		_select(sel_name, sel_path, PackedInt32Array())
		main.set_status("%d nó(s) removido(s)." % done)
	else:
		main.set_status("Não removeu: %s" % tr.last_error())
	return true


func _status_node() -> void:
	var r := _route(sel_name, sel_path)
	if r.is_empty() or sel_nodes.is_empty():
		return
	var n: Vector3 = (r["nodes"] as PackedVector3Array)[sel_nodes[0]]
	main.set_status("%s, nó %d: X %s, Y %s, altura %s m. Setas e PageUp/PageDown ajustam (Shift: 1 cm)." % [
		sel_name, sel_nodes[0] + 1, _num(n.x), _num(-n.z), _num(n.y)])


# ---------------------------------------------------------------------------
# Right panel (rows of `ui_form.rs`; the edits come back as `form:trajeto_*`)
# ---------------------------------------------------------------------------


func panel() -> Array:
	var rows := [{"kind": "title", "text": LABELS[mode] if sel_name == "" else ("Caminho " if sel_path else "Esteira ") + sel_name}]
	var r := _route(sel_name, sel_path)
	if r.is_empty():
		rows.append({"kind": "help", "text": "Clique no chão para pôr os nós; o trecho mostra o comprimento e a inclinação. Os nós encaixam nas portas (pontas das esteiras e objetos), nos nós de outros trajetos, de 15 em 15° e na grade. PageUp/PageDown: altura do próximo nó; [ e ]: raio da curva; Enter termina; Backspace volta um nó; Esc cancela."})
		rows.append({"kind": "number", "name": "Trajeto_AlturaDesenho", "label": "Altura do próximo nó (m)", "value": draw_z, "min": 0.0, "max": 30.0, "step": 0.01, "action": "trajeto_altura_desenho"})
		rows.append({"kind": "check", "name": "Trajeto_InclinacaoFixa", "text": "Inclinação fixa no próximo trecho", "on": grade_on, "action": "trajeto_inclinacao_fixa"})
		rows.append({"kind": "number", "name": "Trajeto_Inclinacao", "label": "Inclinação (%; negativa desce)", "value": grade_pct, "min": -100.0, "max": 100.0, "step": 0.5, "action": "trajeto_inclinacao"})
		rows.append({"kind": "number", "name": "Trajeto_RaioDesenho", "label": "Raio das curvas (m; 0 = quina)", "value": new_radius, "min": 0.0, "max": 50.0, "step": 0.05, "action": "trajeto_raio_desenho"})
		if is_path():
			rows.append({"kind": "number", "name": "Trajeto_LarguraCaminho", "label": "Largura do caminho (m)", "value": path_width, "min": 0.6, "max": 10.0, "step": 0.05, "action": "trajeto_largura_caminho"})
		rows.append({"kind": "info", "name": "Trajeto_Resumo", "text": "%d esteira(s) desenhada(s), %d caminho(s)." % [belts.size(), paths.size()]})
		return rows
	var nodes: PackedVector3Array = r["nodes"]
	var rs: PackedFloat64Array = r["radii"]
	var info := "%d nós, %s m" % [nodes.size(), _num(float(r["length"]))]
	if not sel_path:
		info += ", %s s de passagem a %s m/s, cabem %d" % [_num(float(r["transit"]), 1), _num(float(r["speed"])), int(r["capacity"])]
	rows.append({"kind": "info", "name": "Trajeto_Resumo", "text": info + "."})
	rows.append({"kind": "info", "name": "Trajeto_Inclinacao", "text": "Maior inclinação %s° (máximo %s°)." % [_num(float(r["max_degrees"]), 1), _num(float(r["limit"]), 1)]})
	if r["too_steep"]:
		rows.append({"kind": "warning", "name": "Trajeto_Ingreme", "text": "• Acima do máximo: a peça escorrega ou tomba." if not sel_path else "• Rampa mais íngreme que 1:12 (8,33 %)."})
	rows.append({"kind": "number", "name": "Trajeto_ParaleloDist", "label": "Paralelo a (m; + à esquerda)", "value": parallel_dist, "min": -50.0, "max": 50.0, "step": 0.05, "action": "trajeto_paralelo_dist"})
	rows.append({"kind": "button", "name": "Trajeto_Paralelo", "text": "Desenhar paralelo", "action": "trajeto_paralelo"})
	if sel_path:
		rows.append({"kind": "check", "name": "Trajeto_Escada", "text": "Escada (degraus; só pedestres)", "on": bool(r.get("stair", false)), "action": "trajeto_escada"})
	if sel_path and (nodes[0].y > 0.001 or nodes[nodes.size() - 1].y > 0.001):
		rows.append({"kind": "button", "name": "Trajeto_Rampas", "text": "Rampas de acesso nas pontas (1:12)", "action": "trajeto_rampas"})
	if not sel_path:
		rows.append({"kind": "choice", "name": "Trajeto_Tipo", "label": "Tipo", "items": [["Esteira simples", "simples"], ["Esteira com canaletas", "canaletas"]], "value": r["style"], "action": "trajeto_tipo"})
		rows.append({"kind": "choice", "name": "Trajeto_Acionamento", "label": "Acionamento", "items": [["Motorizada", "motorizada"], ["Roletes por gravidade", "gravidade"]], "value": r["drive"], "action": "trajeto_acionamento"})
		rows.append({"kind": "number", "name": "Trajeto_Largura", "label": "Largura (m)", "value": float(r["width"]), "min": 0.1, "max": 3.0, "step": 0.05, "action": "trajeto_largura"})
	if sel_nodes.size() == 1:
		var i := sel_nodes[0]
		var n := nodes[i]
		rows.append({"kind": "section", "text": "Nó %d" % (i + 1)})
		rows.append({"kind": "number", "name": "Trajeto_X", "label": "X (m)", "value": n.x, "min": -10000.0, "max": 10000.0, "step": 0.01, "action": "trajeto_x"})
		rows.append({"kind": "number", "name": "Trajeto_Y", "label": "Y (m)", "value": -n.z, "min": -10000.0, "max": 10000.0, "step": 0.01, "action": "trajeto_y"})
		rows.append({"kind": "number", "name": "Trajeto_Z", "label": "Altura (m)", "value": n.y, "min": 0.0, "max": 100.0, "step": 0.01, "action": "trajeto_z"})
		if i > 0 and i < nodes.size() - 1:
			rows.append({"kind": "number", "name": "Trajeto_Raio", "label": "Raio da curva (m; 0 = quina)", "value": rs[i], "min": 0.0, "max": 100.0, "step": 0.05, "action": "trajeto_raio"})
			if not sel_path:
				rows.append({"kind": "button", "name": "Trajeto_Dividir", "text": "Dividir a esteira neste nó", "action": "trajeto_dividir"})
	elif sel_nodes.size() > 1:
		rows.append({"kind": "section", "text": "%d nós selecionados" % sel_nodes.size()})
	if sel_nodes.size() > 1 or sel_nodes.is_empty():
		rows.append({"kind": "number", "name": "Trajeto_Nivel", "label": "Nivelar na altura (m)", "value": nodes[sel_nodes[0]].y if not sel_nodes.is_empty() else nodes[0].y, "min": 0.0, "max": 100.0, "step": 0.01, "action": "trajeto_nivelar"})
		rows.append({"kind": "button", "name": "Trajeto_Constante", "text": "Inclinação constante entre as pontas", "action": "trajeto_constante"})
	if not sel_nodes.is_empty():
		rows.append({"kind": "button", "name": "Trajeto_ExcluirNo", "text": "Remover nó(s) (Del)", "enabled": nodes.size() - sel_nodes.size() >= 2, "action": "trajeto_excluir_no"})
	rows.append({"kind": "help", "text": "Arraste o losango do nó no chão para movê-lo e o topo da haste para mudar a altura. Setas movem (Shift: 1 cm), PageUp/PageDown sobem e descem, [ e ] mudam o raio. Shift + clique na linha acrescenta um nó. Arraste o losango laranja no meio de um trecho para curvá-lo."})
	return rows


## The range of nodes the level and constant-slope buttons act on: the selected ones, or
## the whole route.
func _span() -> Array:
	var r := _route(sel_name, sel_path)
	var n := (r["nodes"] as PackedVector3Array).size()
	if sel_nodes.size() < 2:
		return [0, n - 1]
	var a := Array(sel_nodes)
	a.sort()
	return [a[0], a[a.size() - 1]]


## "Editar nós" of the properties panel: the tool of the belt's type, with the belt
## selected (all its nodes, so the arrows and PageUp/PageDown move it whole).
func edit_belt(name: String) -> void:
	sync()
	var r := _route(name, false)
	if r.is_empty():
		main.set_status("%s não é uma esteira desenhada." % name)
		return
	editor.set_tool("colocar", BELT_CHANNELED if r["style"] == "canaletas" else BELT_SIMPLE)
	var all := PackedInt32Array()
	for i in (r["nodes"] as PackedVector3Array).size():
		all.append(i)
	_select(name, false, all)
	main.set_status("Esteira %s: clique num nó para editá-lo; setas e PageUp/PageDown movem os nós selecionados." % name)


## An edit from the panel (`form:trajeto_<what>:<value>`).
func form(what: String, value: String) -> void:
	var v := value.replace(",", ".").to_float()
	if what == "trajeto_editar":
		edit_belt(value)
		return
	var r := _route(sel_name, sel_path)
	match what:
		"trajeto_altura_desenho":
			draw_z = maxf(v, 0.0)
			return
		"trajeto_raio_desenho":
			new_radius = maxf(v, 0.0)
			return
		"trajeto_largura_caminho":
			path_width = maxf(v, 0.3)
			return
		"trajeto_paralelo_dist":
			parallel_dist = v
			return
		"trajeto_inclinacao_fixa":
			grade_on = value == "1" or value == "true"
			main.set_status("Inclinação fixa %s (%s %%)." % ["ligada" if grade_on else "desligada", _num(grade_pct, 1)])
			return
		"trajeto_inclinacao":
			grade_pct = v
			return
	if r.is_empty():
		return
	var ok := true
	var nodes: PackedVector3Array = r["nodes"]
	match what:
		"trajeto_x", "trajeto_y", "trajeto_z":
			if sel_nodes.size() != 1:
				return
			var n := nodes[sel_nodes[0]]
			if what == "trajeto_x":
				n.x = v
			elif what == "trajeto_y":
				n.z = -v
			else:
				n.y = maxf(v, 0.0)
			ok = tr.por_no(sel_name, sel_path, sel_nodes[0], n)
		"trajeto_raio":
			if sel_nodes.size() == 1:
				ok = tr.por_raio(sel_name, sel_path, sel_nodes[0], v)
		"trajeto_tipo":
			ok = tr.por_tipo(sel_name, value)
		"trajeto_acionamento":
			ok = tr.por_acionamento(sel_name, value)
		"trajeto_largura":
			ok = tr.por_largura(sel_name, v)
		"trajeto_nivelar":
			var s := _span()
			ok = tr.nivelar(sel_name, sel_path, s[0], s[1], v)
		"trajeto_constante":
			var s := _span()
			ok = tr.inclinacao_constante(sel_name, sel_path, s[0], s[1])
		"trajeto_dividir":
			if sel_nodes.size() == 1:
				var other: String = tr.dividir(sel_name, sel_nodes[0])
				ok = other != ""
				if ok:
					main.set_status("Esteira dividida: %s → %s." % [sel_name, other])
		"trajeto_excluir_no":
			delete_selected()
			return
		"trajeto_paralelo":
			var other: String = tr.paralelo(sel_name, sel_path, parallel_dist)
			ok = other != ""
			if ok:
				main.after_edit()
				_select(other, sel_path, PackedInt32Array())
				main.set_status("%s desenhado a %s m de %s." % [other, _num(parallel_dist), r["name"]])
				return
		"trajeto_escada":
			ok = tr.por_escada(sel_name, value == "1" or value == "true")
		"trajeto_rampas":
			var k: int = tr.rampas_de_acesso(sel_name)
			ok = k > 0
			if ok:
				main.set_status("%d rampa(s) de acesso de 1:12 acrescentada(s) a %s." % [k, sel_name])
		_:
			return
	if ok:
		main.after_edit()
	else:
		main.set_status("Não mudou: %s" % tr.last_error())


# ---------------------------------------------------------------------------
# Drawing (inside the editor's overlay surface)
# ---------------------------------------------------------------------------


func draw(e: EditorUtsim) -> void:
	if not active():
		_readout.visible = false
		return
	for r in belts:
		_draw_route(e, r, false)
	for r in paths:
		_draw_route(e, r, true)
	_draw_drawing(e)


func _draw_route(e: EditorUtsim, r: Dictionary, path: bool) -> void:
	var sel: bool = r["name"] == sel_name and path == sel_path
	var line: PackedVector3Array = r["line"]
	var color := SELECTED if sel else (PATH if path else ROUTE)
	if r["too_steep"]:
		color = STEEP
	var lift := Vector3(0, 0.03, 0)
	for i in line.size() - 1:
		e._ribbon(line[i] + lift, line[i + 1] + lift, 0.10 if sel else 0.06, color)
	var nodes: PackedVector3Array = r["nodes"]
	var bending: bool = sel and _dragging and _press.get("handle", "") == "curva"
	if sel:
		for i in nodes.size() - 1:
			var mid := nodes[i].lerp(nodes[i + 1], 0.5)
			if bending and i == int(_press["index"]):
				# The bend being dragged: the two legs to the new node.
				e._ribbon(nodes[i] + lift, _drag_at + lift, 0.08, CURVE)
				e._ribbon(_drag_at + lift, nodes[i + 1] + lift, 0.08, CURVE)
				mid = _drag_at
			_diamond(e, mid + Vector3(0, 0.02, 0), 0.16, CURVE)
	for i in nodes.size():
		var n := nodes[i]
		if _dragging and sel and not bending and i == int(_press.get("index", -1)):
			n = _drag_at
		var picked := sel and i in sel_nodes
		var c := DRAWING if picked else (SELECTED if sel else Color(0.92, 0.94, 0.98))
		_handle(e, n, 0.30 if picked else 0.18, c)


## A node: a diamond on the floor, a post up to the node and a diamond at its height.
func _handle(e: EditorUtsim, n: Vector3, r: float, color: Color) -> void:
	var floor := Vector3(n.x, 0.06, n.z)
	_diamond(e, floor, r, color)
	if n.y > 0.05:
		_post(e, floor, n, 0.03, STICK)
		_diamond(e, n + Vector3(0, 0.02, 0), r * 0.8, color)


func _diamond(e: EditorUtsim, c: Vector3, r: float, color: Color) -> void:
	var pts := [c + Vector3(r, 0, 0), c + Vector3(0, 0, r), c + Vector3(-r, 0, 0), c + Vector3(0, 0, -r)]
	for k in [[0, 1, 2], [0, 2, 3]]:
		for j in k:
			e._mesh.surface_set_color(color)
			e._mesh.surface_add_vertex(pts[j])


## A vertical bar from `a` to `b`, `w` wide, facing the camera.
func _post(e: EditorUtsim, a: Vector3, b: Vector3, w: float, color: Color) -> void:
	var cam: Camera3D = main.camera
	var side := cam.global_transform.basis.x
	side.y = 0.0
	side = side.normalized() * w * 0.5
	for v in [a - side, a + side, b + side, a - side, b + side, b - side]:
		e._mesh.surface_set_color(color)
		e._mesh.surface_add_vertex(v)


func _draw_drawing(e: EditorUtsim) -> void:
	if points.is_empty():
		if not _dragging:
			_readout.visible = false
		return
	var pts := PackedVector3Array(points)
	var rs := PackedFloat64Array(radii)
	var cursor: Variant = null
	var at = _aim(e.mouse)
	if at != null and not main.over_ui(e.mouse):
		cursor = _snap(at, points[points.size() - 1])["at"]
	if cursor != null and grade_on:
		cursor = _graded(cursor)
	_cursor = cursor
	# A typed length puts the preview node at that distance.
	if cursor != null and typed != "":
		var l := typed.replace(",", ".").to_float()
		var last := points[points.size() - 1]
		var d := (cursor as Vector3) - last
		d.y = 0.0
		var dz := draw_z - last.y
		if d.length() > 1e-6 and grade_on:
			var run := l / sqrt(1.0 + pow(grade_pct / 100.0, 2.0))
			cursor = last + d.normalized() * run
			cursor.y = maxf(last.y + grade_pct / 100.0 * run, 0.0)
		elif d.length() > 1e-6 and l > absf(dz):
			cursor = last + d.normalized() * sqrt(l * l - dz * dz)
			cursor.y = draw_z
	if cursor != null:
		pts.append(cursor)
		rs.append(0.0)
		if new_radius > 0.0 and rs.size() >= 3:
			rs[rs.size() - 2] = new_radius
	var draft: Dictionary = tr.rascunho(pts, rs)
	var line: PackedVector3Array = draft["line"]
	var steep := false
	var text := ""
	if cursor != null:
		var pv: Dictionary = tr.previa(mode, points[points.size() - 1], cursor)
		steep = pv["too_steep"]
		text = "Trecho %s m · %s° (%s %%)\nTotal %s m · altura %s m" % [_num(float(pv["length"])),
			_num(float(pv["degrees"]), 1), _num(float(pv["percent"]), 1),
			_num(float(draft["length"])), _num((cursor as Vector3).y)]
		if steep:
			text += "\nAcima do máximo de %s°" % _num(float(pv["limit"]), 1)
		if typed != "":
			text = "Digitado: %s m\n" % typed + text
		_readout_at(cursor, text, steep)
	else:
		_readout.visible = false
	var color := STEEP if steep else DRAWING
	for i in line.size() - 1:
		e._ribbon(line[i] + Vector3(0, 0.04, 0), line[i + 1] + Vector3(0, 0.04, 0), 0.12, color)
	if is_path():
		_edges(e, line, path_width, color)
	for p in points:
		_handle(e, p, 0.24, DRAWING)
	if cursor != null:
		_handle(e, cursor, 0.14, color)


func _edges(e: EditorUtsim, line: PackedVector3Array, width: float, color: Color) -> void:
	for i in line.size() - 1:
		var a := line[i]
		var b := line[i + 1]
		var d := b - a
		d.y = 0.0
		if d.length() < 1e-5:
			continue
		var side := Vector3(-d.z, 0, d.x).normalized() * width * 0.5
		e._ribbon(a + side, b + side, 0.04, color)
		e._ribbon(a - side, b - side, 0.04, color)


func _readout_at(at: Vector3, text: String, warn: bool) -> void:
	_readout.visible = true
	_readout.position = at + Vector3(0, 0.6, 0)
	_readout.text = text
	_readout.modulate = STEEP if warn else Color(0.95, 0.97, 1.0)

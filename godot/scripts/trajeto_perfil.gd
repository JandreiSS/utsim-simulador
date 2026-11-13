## The profile of the route selected with the "trajeto" tool: a cut along it, height
## against the distance walked in plan, with the grade of every segment (red above the
## limit of the belt or of a ramp) and its nodes as handles. Dragging a node up or down
## changes only its height (snapped to 5 cm; with Shift, 1 cm), one undo step on release.
class_name TrajetoPerfil
extends Control

const PAD := 34.0
const NODE_PIXELS := 9.0
const BACK := Color(0.10, 0.11, 0.13, 0.92)
const GRID := Color(0.30, 0.32, 0.36)
const LINE := Color(0.55, 0.80, 1.0)
const STEEP := Color(1.0, 0.35, 0.25)
const NODE := Color(0.92, 0.94, 0.98)
const PICKED := Color(0.45, 0.95, 0.55)
const TEXT := Color(0.85, 0.87, 0.90)

## Called with (index, new height) when a node is dropped.
signal node_height(index: int, height: float)

var route: Dictionary = {}
## Per point of the real line: (distance in plan, height); per node: its point.
var _pts := PackedVector2Array()
var _node_at := PackedInt32Array()
var _limit_grade := 0.0
var _drag := -1
var _drag_y := 0.0
## The scale while a node is dragged (it would follow the node otherwise).
var _fixed := Rect2()
var selected := PackedInt32Array()


func _ready() -> void:
	mouse_filter = Control.MOUSE_FILTER_STOP
	custom_minimum_size = Vector2(520, 170)


## Shows `r` (a row of `UtsimTrajetos.esteiras()` or `caminhos()`), or hides with {}.
func show_route(r: Dictionary, picked: PackedInt32Array) -> void:
	route = r
	selected = picked
	visible = not r.is_empty()
	if r.is_empty():
		return
	var line: PackedVector3Array = r["line"]
	_pts = PackedVector2Array()
	var run := 0.0
	for k in line.size():
		if k > 0:
			run += Vector2(line[k].x - line[k - 1].x, line[k].z - line[k - 1].z).length()
		_pts.append(Vector2(run, line[k].y))
	var nodes: PackedVector3Array = r["nodes"]
	_node_at = PackedInt32Array()
	for n in nodes:
		var best := 0
		var bd := INF
		for k in line.size():
			var d := line[k].distance_squared_to(n)
			if d < bd:
				bd = d
				best = k
		_node_at.append(best)
	_limit_grade = tan(deg_to_rad(float(r.get("limit", 90.0))))
	queue_redraw()


func _bounds() -> Rect2:
	if _drag >= 0:
		return _fixed
	var max_x := 1.0
	var max_y := 1.0
	for p in _pts:
		max_x = maxf(max_x, p.x)
		max_y = maxf(max_y, p.y)
	return Rect2(0, 0, max_x, max_y * 1.15 + 0.2)


func _to_screen(p: Vector2) -> Vector2:
	var b := _bounds()
	var w := size.x - 2.0 * PAD
	var h := size.y - 2.0 * PAD
	return Vector2(PAD + p.x / b.size.x * w, size.y - PAD - p.y / b.size.y * h)


func _height_at(y: float) -> float:
	var b := _bounds()
	var h := size.y - 2.0 * PAD
	return (size.y - PAD - y) / h * b.size.y


func _num(v: float, d := 2) -> String:
	return (("%." + str(d) + "f") % v).replace(".", ",")


func _draw() -> void:
	if route.is_empty() or _pts.size() < 2:
		return
	draw_rect(Rect2(Vector2.ZERO, size), BACK)
	var font := get_theme_default_font()
	var b := _bounds()
	# The floor and a line every metre of height.
	var m := 0.0
	while m <= b.size.y:
		var y := _to_screen(Vector2(0, m)).y
		draw_line(Vector2(PAD, y), Vector2(size.x - PAD, y), GRID if m > 0.0 else TEXT, 1.0)
		draw_string(font, Vector2(4, y + 4), "%s m" % _num(m, 0), HORIZONTAL_ALIGNMENT_LEFT, -1, 11, TEXT)
		m += 1.0 if b.size.y < 8.0 else 2.0
	var pts := _pts.duplicate()
	if _drag >= 0:
		pts[_node_at[_drag]].y = _drag_y
	for k in pts.size() - 1:
		var run := pts[k + 1].x - pts[k].x
		var g := absf(pts[k + 1].y - pts[k].y) / run if run > 1e-6 else 0.0
		draw_line(_to_screen(pts[k]), _to_screen(pts[k + 1]), STEEP if g > _limit_grade + 1e-9 else LINE, 3.0)
	# Each node, with the grade of the segment after it.
	for i in _node_at.size():
		var p := pts[_node_at[i]]
		var s := _to_screen(p)
		var c := PICKED if (i in selected or i == _drag) else NODE
		draw_circle(s, 5.0, c)
		draw_string(font, s + Vector2(-14, -10), "%s m" % _num(p.y), HORIZONTAL_ALIGNMENT_LEFT, -1, 11, c)
		if i + 1 < _node_at.size():
			var q := pts[_node_at[i + 1]]
			var run := q.x - p.x
			if run > 1e-6:
				var pct := (q.y - p.y) / run * 100.0
				var mid := _to_screen((p + q) * 0.5)
				draw_string(font, mid + Vector2(-16, 16), "%s %%" % _num(pct, 1), HORIZONTAL_ALIGNMENT_LEFT, -1, 11, TEXT)
	draw_string(font, Vector2(PAD, 16), "Perfil de %s: %s m · arraste um nó para cima ou para baixo (Shift: 1 cm)" % [
		route["name"], _num(float(route["length"]))], HORIZONTAL_ALIGNMENT_LEFT, -1, 12, TEXT)


func _gui_input(event: InputEvent) -> void:
	var mb := event as InputEventMouseButton
	if mb != null and mb.button_index == MOUSE_BUTTON_LEFT:
		if mb.pressed:
			_drag = -1
			var found := -1
			for i in _node_at.size():
				if _to_screen(_pts[_node_at[i]]).distance_to(mb.position) <= NODE_PIXELS:
					found = i
			if found >= 0:
				_fixed = _bounds()
				_fixed.size.y = maxf(_fixed.size.y * 1.5, 1.0)
				_drag = found
				_drag_y = _pts[_node_at[found]].y
			accept_event()
		elif _drag >= 0:
			var i := _drag
			_drag = -1
			node_height.emit(i, _drag_y)
			accept_event()
		queue_redraw()
		return
	var mm := event as InputEventMouseMotion
	if mm != null and _drag >= 0:
		var step := 0.01 if mm.shift_pressed else 0.05
		_drag_y = clampf(snappedf(_height_at(mm.position.y), step), 0.0, _fixed.size.y)
		queue_redraw()
		accept_event()

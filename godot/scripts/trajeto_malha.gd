## The 3D look of a belt drawn by nodes (`UtsimTrajetos.da_cena`) and of the ramps of the
## floor paths (`UtsimTrajetos.caminhos_da_cena`), built along the real line of the route
## (curves sampled, heights as drawn): nothing here is a scaled asset.
##
## Belt: the line is the top of the conveying surface. Under it, the two side frames
## (`FRAME_DEPTH` deep), a belt that scrolls while the clock runs (powered) or a row of
## rollers (gravity), and two legs to the floor at every node, braced when tall. The belt
## with side channels (`style == "canaletas"`) has high guides on both sides.
##
## Ramp of a floor path: a deck of the path's width along the parts above the floor, with
## a yellow edge line and handrails where it is higher than `RAIL_FROM`.
class_name TrajetoMalha
extends RefCounted

const FRAME_DEPTH := 0.14
const FRAME_THICK := 0.045
const GUIDE_HEIGHT := 0.16
const GUIDE_THICK := 0.03
const LEG := 0.05
const ROLLER_PITCH := 0.12
const ROLLER_RADIUS := 0.028
## Length of one stripe period of the belt texture (m): the scroll is in these units.
const STRIPE := 0.5
const DECK := 0.08
const RAIL_FROM := 0.5
const RAIL_HEIGHT := 1.0

const BELT_COLOR := Color(0.13, 0.14, 0.15)
const FRAME_COLOR := Color(0.33, 0.45, 0.60)
const GUIDE_COLOR := Color(0.92, 0.66, 0.18)
const LEG_COLOR := Color(0.55, 0.58, 0.62)
const ROLLER_COLOR := Color(0.78, 0.80, 0.83)
const DECK_COLOR := Color(0.46, 0.47, 0.48)
const EDGE_COLOR := Color(0.95, 0.80, 0.18)


## Side vector (horizontal, to the right of the direction) at every point of `line`,
## averaged at the joints so the swept shapes do not tear at the corners.
static func _sides(line: PackedVector3Array) -> PackedVector3Array:
	var n := line.size()
	var seg := PackedVector3Array()
	for k in n - 1:
		var d := line[k + 1] - line[k]
		d.y = 0.0
		var s := Vector3(-d.z, 0, d.x).normalized() if d.length() > 1e-6 else Vector3(0, 0, 1)
		seg.append(-s)
	var out := PackedVector3Array()
	for k in n:
		var a: Vector3 = seg[maxi(k - 1, 0)] if seg.size() > 0 else Vector3(0, 0, 1)
		var b: Vector3 = seg[mini(k, seg.size() - 1)] if seg.size() > 0 else Vector3(0, 0, 1)
		var s := (a + b)
		if s.length() < 1e-6:
			s = b
		s = s.normalized()
		# Keep the width constant across a corner (miter), at most twice.
		var c := s.dot(b)
		out.append(s / clampf(c, 0.5, 1.0))
	return out


## Distance along the line at every point.
static func _along(line: PackedVector3Array) -> PackedFloat32Array:
	var out := PackedFloat32Array([0.0])
	for k in range(1, line.size()):
		out.append(out[k - 1] + line[k].distance_to(line[k - 1]))
	return out


## A box of cross-section [s0, s1] × [u0, u1] (side × height, metres from the line)
## swept along the line, into `st` (vertex colour `color`), with end caps.
static func _sweep(st: SurfaceTool, line: PackedVector3Array, sides: PackedVector3Array,
		s0: float, s1: float, u0: float, u1: float, color: Color) -> void:
	var n := line.size()
	if n < 2:
		return
	var ring := func(k: int) -> Array:
		var p := line[k]
		var s := sides[k]
		return [p + s * s0 + Vector3.UP * u0, p + s * s1 + Vector3.UP * u0,
			p + s * s1 + Vector3.UP * u1, p + s * s0 + Vector3.UP * u1]
	st.set_color(color)
	var prev: Array = ring.call(0)
	_quad(st, prev[0], prev[3], prev[2], prev[1])
	for k in range(1, n):
		var cur: Array = ring.call(k)
		for f in 4:
			var g := (f + 1) % 4
			_quad(st, prev[f], prev[g], cur[g], cur[f])
		prev = cur
	_quad(st, prev[0], prev[1], prev[2], prev[3])


static func _quad(st: SurfaceTool, a: Vector3, b: Vector3, c: Vector3, d: Vector3) -> void:
	for v in [a, b, c, a, c, d]:
		st.add_vertex(v)


static func _material(color: Color, rough := 0.6, metal := 0.0) -> StandardMaterial3D:
	var m := StandardMaterial3D.new()
	m.albedo_color = color
	m.roughness = rough
	m.metallic = metal
	m.vertex_color_use_as_albedo = true
	m.cull_mode = BaseMaterial3D.CULL_DISABLED
	return m


## The texture of the belt: dark rubber with a lighter cleat every `STRIPE`.
static func _belt_texture() -> ImageTexture:
	var img := Image.create(64, 8, false, Image.FORMAT_RGB8)
	img.fill(Color(0.15, 0.16, 0.17))
	for x in 6:
		for y in 8:
			img.set_pixel(x, y, Color(0.32, 0.33, 0.35))
	return ImageTexture.create_from_image(img)


## A belt: `{line, nodes, width, style, drive}` as `UtsimTrajetos.da_cena` gives it, in
## absolute Godot coordinates. Returns a top-level node; `node.get_meta("belt_material")`
## is the material whose `uv1_offset.x` scrolls the belt (null with rollers).
static func belt(r: Dictionary) -> Node3D:
	var root := Node3D.new()
	root.name = "Trajeto"
	root.top_level = true
	var line: PackedVector3Array = r["line"]
	if line.size() < 2:
		return root
	var w := float(r.get("width", 0.6))
	var half := w * 0.5
	var sides := _sides(line)
	var gravity := String(r.get("drive", "")) == "gravidade"
	var channels := String(r.get("style", "")) == "canaletas"
	var parts := _parts_available()
	# Frames, guides and legs: one mesh, coloured by vertex.
	var st := SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_TRIANGLES)
	_sweep(st, line, sides, -half - FRAME_THICK, -half, -FRAME_DEPTH, 0.02, FRAME_COLOR)
	_sweep(st, line, sides, half, half + FRAME_THICK, -FRAME_DEPTH, 0.02, FRAME_COLOR)
	if channels:
		_sweep(st, line, sides, -half - GUIDE_THICK, -half + 0.005, 0.02, GUIDE_HEIGHT, GUIDE_COLOR)
		_sweep(st, line, sides, half - 0.005, half + GUIDE_THICK, 0.02, GUIDE_HEIGHT, GUIDE_COLOR)
	var nodes: PackedVector3Array = r.get("nodes", line)
	for k in nodes.size():
		# Under the line: at a rounded corner the node itself is off the arc, so the legs
		# go under the point of the arc nearest to it.
		var j := _nearest(line, nodes[k])
		var p := line[j]
		var bottom := p.y - FRAME_DEPTH
		if bottom < 0.12:
			continue
		var s: Vector3 = sides[j]
		var feet: Array[Vector3] = []
		for side in [-1.0, 1.0]:
			var c: Vector3 = p + s * side * (half + FRAME_THICK * 0.5)
			feet.append(c)
			if parts:
				_foot(root, Vector3(c.x, 0.0, c.z), _ahead(line, _nearest(line, p)), bottom)
			else:
				_post(st, Vector3(c.x, 0.0, c.z), bottom, LEG, LEG_COLOR)
		# A cross bar under the frame, and one low down on tall legs.
		_bar(st, Vector3(feet[0].x, bottom - 0.03, feet[0].z), Vector3(feet[1].x, bottom - 0.03, feet[1].z), 0.035, LEG_COLOR)
		if bottom > 1.2:
			_bar(st, Vector3(feet[0].x, 0.25, feet[0].z), Vector3(feet[1].x, 0.25, feet[1].z), 0.035, LEG_COLOR)
	if parts:
		_end_drums(root, line, w)
		if channels:
			_guide_brackets(root, line, sides, half)
	st.generate_normals()
	var frame := MeshInstance3D.new()
	frame.name = "Estrutura"
	frame.mesh = st.commit()
	frame.material_override = _material(Color.WHITE, 0.45, 0.55)
	root.add_child(frame)
	if gravity:
		root.add_child(_rollers(line, sides, w))
		root.set_meta("belt_material", null)
	else:
		var along := _along(line)
		var bt := SurfaceTool.new()
		bt.begin(Mesh.PRIMITIVE_TRIANGLES)
		for k in range(1, line.size()):
			var a := line[k - 1]
			var b := line[k]
			var sa := sides[k - 1] * half
			var sb := sides[k] * half
			var ua := along[k - 1] / STRIPE
			var ub := along[k] / STRIPE
			var quad := [[a - sa, Vector2(ua, 0)], [a + sa, Vector2(ua, 1)], [b + sb, Vector2(ub, 1)],
				[a - sa, Vector2(ua, 0)], [b + sb, Vector2(ub, 1)], [b - sb, Vector2(ub, 0)]]
			for q in quad:
				bt.set_normal(Vector3.UP)
				bt.set_uv(q[1])
				bt.add_vertex((q[0] as Vector3) + Vector3(0, 0.005, 0))
		var surface := MeshInstance3D.new()
		surface.name = "Correia"
		surface.mesh = bt.commit()
		var m := StandardMaterial3D.new()
		m.albedo_texture = _belt_texture()
		m.texture_filter = BaseMaterial3D.TEXTURE_FILTER_LINEAR_WITH_MIPMAPS
		m.texture_repeat = true
		m.roughness = 0.85
		m.cull_mode = BaseMaterial3D.CULL_DISABLED
		surface.material_override = m
		root.add_child(surface)
		root.set_meta("belt_material", m)
	return root


# ---------------------------------------------------------------------------
# The Blender parts (`docs/elementos_3d_blender.md`, "Peças da esteira por trajeto"): a foot
# with an adjustable pad, the end drum and the bracket of the side channel. Without the
# imported files the shapes above stand for them.
# ---------------------------------------------------------------------------

const PARTS := "res://importados/modelos3d/"
const PAD := 0.06
const DRUM_RADIUS := 0.07
## Metres between the brackets of a side channel.
const BRACKET_PITCH := 1.0


static func _parts_available() -> bool:
	for n in ["esteira_pe", "esteira_ponteira", "esteira_canaleta"]:
		if not ResourceLoader.exists(PARTS + n + ".glb"):
			return false
	return true


static func _part(name: String) -> Node3D:
	var scene := load(PARTS + name + ".glb") as PackedScene
	return scene.instantiate() as Node3D if scene != null else null


static func _find(n: Node, name: String) -> Node3D:
	if n.name == name and n is Node3D:
		return n as Node3D
	for c in n.get_children():
		var f := _find(c, name)
		if f != null:
			return f
	return null


## Horizontal direction of the line at point `k` (towards the next point).
static func _ahead(line: PackedVector3Array, k: int) -> Vector3:
	var a := line[mini(k, line.size() - 2)]
	var b := line[mini(k, line.size() - 2) + 1]
	var d := b - a
	d.y = 0.0
	return d.normalized() if d.length() > 1e-6 else Vector3.RIGHT


## The part's frame: `x` along `fwd`, `y` up, `z` to the right of the way.
static func _frame(fwd: Vector3, at: Vector3) -> Transform3D:
	var x := fwd.normalized()
	var z := x.cross(Vector3.UP).normalized()
	var y := z.cross(x).normalized()
	return Transform3D(Basis(x, y, z), at)


## A foot on the floor at `at`, its leg up to `top` (the bottom of the frame).
static func _foot(root: Node3D, at: Vector3, fwd: Vector3, top: float) -> void:
	var pe := _part("esteira_pe")
	if pe == null:
		return
	pe.transform = _frame(fwd, at)
	var leg := _find(pe, "perna")
	if leg != null:
		leg.scale.y = maxf(top - PAD, 0.01)
	root.add_child(pe)


## The drums at both ends, turned outwards, as wide as the belt. Their clip `rolar` is
## seeked by the scene as the belt runs (`set_meta("drums")`).
static func _end_drums(root: Node3D, line: PackedVector3Array, w: float) -> void:
	var n := line.size()
	var players: Array = []
	for end in [[line[n - 1], line[n - 1] - line[n - 2]], [line[0], line[0] - line[1]]]:
		var drum := _part("esteira_ponteira")
		if drum == null:
			continue
		var fwd: Vector3 = end[1]
		if fwd.length() < 1e-6:
			fwd = Vector3.RIGHT
		var x := fwd.normalized()
		var z := x.cross(Vector3.UP).normalized()
		drum.transform = Transform3D(Basis(x, z.cross(x).normalized(), z), end[0])
		var t := _find(drum, "tambor")
		if t != null:
			t.scale.z = w
		for m in ["mancal_e", "mancal_d"]:
			var b := _find(drum, m)
			if b != null:
				b.position.z = signf(b.position.z) * w * 0.5 if b.position.z != 0.0 else b.position.z
		root.add_child(drum)
		var player := _find_player(drum)
		if player != null and player.has_animation(&"rolar"):
			players.append(player)
	root.set_meta("drums", players)


static func _find_player(n: Node) -> AnimationPlayer:
	if n is AnimationPlayer:
		return n as AnimationPlayer
	for c in n.get_children():
		var f := _find_player(c)
		if f != null:
			return f
	return null


## Turns the end drums for a belt that moved `distance` metres.
static func roll(players: Array, distance: float) -> void:
	for p in players:
		var player := p as AnimationPlayer
		var anim := player.get_animation(&"rolar")
		if anim == null:
			continue
		if player.current_animation != "rolar":
			player.play(&"rolar")
			player.pause()
		var turns := distance / (TAU * DRUM_RADIUS)
		player.seek(fposmod(turns, 1.0) * anim.length, true)


## The brackets of the side channels, one every `BRACKET_PITCH` on both sides.
static func _guide_brackets(root: Node3D, line: PackedVector3Array, sides: PackedVector3Array, half: float) -> void:
	var along := _along(line)
	var total: float = along[along.size() - 1]
	var k := 0
	var s := BRACKET_PITCH * 0.5
	while s < total:
		while k < along.size() - 2 and along[k + 1] < s:
			k += 1
		var seg := maxf(along[k + 1] - along[k], 1e-6)
		var p := line[k].lerp(line[k + 1], clampf((s - along[k]) / seg, 0.0, 1.0))
		var fwd := _ahead(line, k)
		var out := half + FRAME_THICK
		for side in [1.0, -1.0]:
			var b := _part("esteira_canaleta")
			if b == null:
				return
			# `sides` points left: the right one is at −side, the left one turned 180°.
			var at: Vector3 = p - sides[k].normalized() * side * out + Vector3(0, 0.02, 0)
			b.transform = _frame(fwd * side, at)
			root.add_child(b)
		s += BRACKET_PITCH


## Rise of a step of a stair (m).
const STEP_RISE := 0.18


## The steps of a stair: on each piece of the line that climbs, as many steps as fit the
## rise, each a block from the floor of the piece to its tread; level pieces are landings.
static func _steps(st: SurfaceTool, line: PackedVector3Array, sides: PackedVector3Array, half: float) -> void:
	for k in line.size() - 1:
		var a := line[k]
		var b := line[k + 1]
		var rise := b.y - a.y
		if absf(rise) < 0.01:
			_sweep(st, PackedVector3Array([a, b]), PackedVector3Array([sides[k], sides[k + 1]]), -half, half, -DECK, 0.0, DECK_COLOR)
			continue
		var n := maxi(int(ceil(absf(rise) / STEP_RISE)), 1)
		for j in n:
			# Tread j: from j/n to (j+1)/n along the piece, at the height of its upper end.
			var p0 := a.lerp(b, float(j) / n)
			var p1 := a.lerp(b, float(j + 1) / n)
			var top := maxf(p0.y, p1.y) if rise > 0.0 else minf(p0.y, p1.y) + absf(rise) / n
			var base := minf(p0.y, p1.y) - DECK
			var q0 := Vector3(p0.x, top, p0.z)
			var q1 := Vector3(p1.x, top, p1.z)
			_sweep(st, PackedVector3Array([q0, q1]), PackedVector3Array([sides[k], sides[k + 1]]), -half, half, base - top, 0.0, DECK_COLOR)


static func _nearest(line: PackedVector3Array, p: Vector3) -> int:
	var best := 0
	var bd := INF
	for k in line.size():
		var d := line[k].distance_squared_to(p)
		if d < bd:
			bd = d
			best = k
	return best


## A square post from `base` up to `top` (m), `size` wide.
static func _post(st: SurfaceTool, base: Vector3, top: float, size: float, color: Color) -> void:
	var line := PackedVector3Array([base, Vector3(base.x, top, base.z)])
	# A vertical sweep: the "side" is X and the "up" offsets become Z.
	var h := size * 0.5
	st.set_color(color)
	var corners := [Vector3(-h, 0, -h), Vector3(h, 0, -h), Vector3(h, 0, h), Vector3(-h, 0, h)]
	for f in 4:
		var a: Vector3 = corners[f]
		var b: Vector3 = corners[(f + 1) % 4]
		_quad(st, line[0] + a, line[0] + b, line[1] + b, line[1] + a)
	_quad(st, line[1] + corners[0], line[1] + corners[1], line[1] + corners[2], line[1] + corners[3])


## A square bar from `a` to `b`.
static func _bar(st: SurfaceTool, a: Vector3, b: Vector3, size: float, color: Color) -> void:
	var line := PackedVector3Array([a, b])
	var sides := _sides(line)
	_sweep(st, line, sides, -size * 0.5, size * 0.5, -size * 0.5, size * 0.5, color)


## The rollers of a gravity belt, one every `ROLLER_PITCH` along the line.
static func _rollers(line: PackedVector3Array, sides: PackedVector3Array, w: float) -> MultiMeshInstance3D:
	var along := _along(line)
	var total: float = along[along.size() - 1]
	var count := maxi(int(total / ROLLER_PITCH), 1)
	var cyl := CylinderMesh.new()
	cyl.top_radius = ROLLER_RADIUS
	cyl.bottom_radius = ROLLER_RADIUS
	cyl.height = w
	cyl.radial_segments = 10
	cyl.rings = 1
	var mm := MultiMesh.new()
	mm.transform_format = MultiMesh.TRANSFORM_3D
	mm.mesh = cyl
	mm.instance_count = count
	var k := 0
	for i in count:
		var s := (i + 0.5) * total / count
		while k < along.size() - 2 and along[k + 1] < s:
			k += 1
		var seg := maxf(along[k + 1] - along[k], 1e-6)
		var t := (s - along[k]) / seg
		var p := line[k].lerp(line[k + 1], t)
		var side := sides[k].lerp(sides[k + 1], t).normalized()
		var fwd := (line[k + 1] - line[k]).normalized()
		var up := fwd.cross(side).normalized()
		if up.y < 0.0:
			# Keep the basis right-handed with its Z up.
			side = -side
			up = -up
		# The cylinder's own axis is Y: turn it onto the side vector.
		var basis := Basis(fwd, side, up)
		mm.set_instance_transform(i, Transform3D(basis, p - up * ROLLER_RADIUS))
	var mi := MultiMeshInstance3D.new()
	mi.name = "Roletes"
	mi.multimesh = mm
	mi.material_override = _material(ROLLER_COLOR, 0.35, 0.8)
	return mi


## The deck of a floor path where it is above the floor (a ramp, a mezzanine walkway):
## `{line, width}` in absolute Godot coordinates. Null when the whole path is on the floor.
static func ramp(r: Dictionary) -> Node3D:
	var line: PackedVector3Array = r["line"]
	var up := false
	for p in line:
		if p.y > 0.01:
			up = true
	if line.size() < 2 or not up:
		return null
	var half := float(r.get("width", 1.2)) * 0.5
	var sides := _sides(line)
	var st := SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_TRIANGLES)
	if bool(r.get("stair", false)):
		_steps(st, line, sides, half)
	else:
		_sweep(st, line, sides, -half, half, -DECK, 0.0, DECK_COLOR)
	_sweep(st, line, sides, -half, -half + 0.08, 0.0, 0.004, EDGE_COLOR)
	_sweep(st, line, sides, half - 0.08, half, 0.0, 0.004, EDGE_COLOR)
	var tall := false
	for p in line:
		if p.y > RAIL_FROM:
			tall = true
	if tall:
		for side in [-1.0, 1.0]:
			var s0 := -half if side < 0.0 else half - 0.04
			_sweep(st, line, sides, s0, s0 + 0.04, RAIL_HEIGHT - 0.04, RAIL_HEIGHT, EDGE_COLOR)
		var along := _along(line)
		var total: float = along[along.size() - 1]
		var k := 0
		var s := 0.0
		while s <= total:
			while k < along.size() - 2 and along[k + 1] < s:
				k += 1
			var seg := maxf(along[k + 1] - along[k], 1e-6)
			var p := line[k].lerp(line[k + 1], clampf((s - along[k]) / seg, 0.0, 1.0))
			if p.y > RAIL_FROM * 0.5:
				for side in [-1.0, 1.0]:
					var c: Vector3 = p + sides[k] * side * (half - 0.02)
					_post(st, c, c.y + RAIL_HEIGHT, 0.04, EDGE_COLOR)
			# Posts under the deck, down to the floor.
			if p.y > 0.3:
				_post(st, Vector3(p.x, 0.0, p.z), p.y - DECK, 0.1, LEG_COLOR)
			s += 1.5
	st.generate_normals()
	var mi := MeshInstance3D.new()
	mi.name = "Rampa"
	mi.mesh = st.commit()
	mi.material_override = _material(Color.WHITE, 0.8)
	var root := Node3D.new()
	root.name = "Caminho"
	root.top_level = true
	root.add_child(mi)
	return root

## Orbit camera with the conventions of the Bevy viewer (`crates/utsim-viewer/src/camera.rs`):
## the eye is `target + Basis.from_euler((-pitch, yaw, 0), YXZ) * (0, 0, dist)`, looking at
## the target; `yaw` 0 looks along -Z. Right or middle drag orbits, Shift + drag (or the
## middle button with Shift) pans, the wheel zooms.
class_name CameraOrbita
extends Camera3D

var target := Vector3.ZERO
var yaw := 0.0
var pitch := 0.75
var dist := 20.0
var _dragging := false


func _ready() -> void:
	fov = 45.0
	near = 0.05
	far = 4000.0
	update()


## Frames a box `lo..hi` (Godot space) like the viewer's first view.
func frame_box(lo: Vector3, hi: Vector3) -> void:
	var a := lo
	var b := hi
	for k in [0, 2]:
		var mid := (a[k] + b[k]) * 0.5
		if b[k] - a[k] < 16.0:
			a[k] = mid - 8.0
			b[k] = mid + 8.0
	target = (a + b) * 0.5
	target.y = 0.0
	var radius := maxf((b - a).length() * 0.5, 4.0)
	yaw = 0.0
	pitch = 0.75
	# The view fits the box: half its diagonal over tan(fov / 2), plus a margin.
	dist = clampf(radius / tan(deg_to_rad(fov) * 0.5) * 0.85, 1.5, 5000.0)
	update()


func focus(at: Vector3, distance: float, new_yaw: Variant = null, new_pitch: Variant = null) -> void:
	target = at
	dist = clampf(distance, 1.5, 5000.0)
	if new_yaw != null:
		yaw = new_yaw
	if new_pitch != null:
		pitch = clampf(new_pitch, 0.05, 1.55)
	update()


func update() -> void:
	var rot := Basis.from_euler(Vector3(-pitch, yaw, 0.0), EULER_ORDER_YXZ)
	var eye := target + rot * Vector3(0, 0, dist)
	transform = Transform3D(Basis.IDENTITY, eye).looking_at(target, Vector3.UP)


func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		var mb := event as InputEventMouseButton
		match mb.button_index:
			MOUSE_BUTTON_WHEEL_UP:
				dist = maxf(1.5, dist * 0.9)
				update()
			MOUSE_BUTTON_WHEEL_DOWN:
				dist = minf(5000.0, dist / 0.9)
				update()
			MOUSE_BUTTON_RIGHT, MOUSE_BUTTON_MIDDLE:
				_dragging = mb.pressed
	elif event is InputEventMouseMotion and _dragging:
		var mm := event as InputEventMouseMotion
		if mm.shift_pressed or (mm.button_mask & MOUSE_BUTTON_MASK_MIDDLE) != 0:
			var right := global_basis.x
			var fwd := Vector3(-sin(yaw), 0, -cos(yaw))
			var k := dist * 0.0015
			target += (-right * mm.relative.x + fwd * mm.relative.y) * k
		else:
			yaw -= mm.relative.x * 0.006
			pitch = clampf(pitch + mm.relative.y * 0.006, 0.05, 1.55)
		update()

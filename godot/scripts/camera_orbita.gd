## Orbit camera with the conventions of the Bevy viewer (`crates/utsim-viewer/src/camera.rs`):
## the eye is `target + Basis.from_euler((-pitch, yaw, 0), YXZ) * (0, 0, dist)`, looking at
## the target; `yaw` 0 looks along -Z. Right or middle drag orbits, Shift + drag (or the
## middle button with Shift) pans, the wheel zooms.
class_name CameraOrbita
extends Camera3D

## Distance factor of one notch of the wheel.
const WHEEL_STEP := 0.85

var target := Vector3.ZERO
var yaw := 0.0
var pitch := 0.75
var dist := 20.0
var _dragging := false
## `func(screen_point) -> bool`: whether a panel covers that point (the main scene's
## `over_ui`), for events the GUI let go of.
var over_ui := Callable()
## `func() -> Vector2`: left and right x (pixels) of the part of the screen not covered by
## the side panels. `frame_box` centres the box there and fits its width in it, so the
## objects and their labels are not cut by a panel.
var free_span := Callable()


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
	var half_tan := tan(deg_to_rad(fov) * 0.5)
	dist = clampf(radius / half_tan * 0.85, 1.5, 5000.0)
	var screen := get_viewport().get_visible_rect().size if is_inside_tree() else Vector2.ZERO
	if free_span.is_valid() and screen.x > 0.0 and screen.y > 0.0:
		var span: Vector2 = free_span.call()
		var width := span.y - span.x
		if width > 100.0:
			# The box's half width, with a margin for the labels, fits in the free part.
			var half_x := (b.x - a.x) * 0.5 + 1.5
			dist = clampf(maxf(dist, half_x / (half_tan * width / screen.y)), 1.5, 5000.0)
			# Move the target so the box's centre falls in the middle of the free part
			# (yaw is 0 here: the screen's right is +X).
			var shift_px := screen.x * 0.5 - (span.x + span.y) * 0.5
			target.x += shift_px * 2.0 * dist * half_tan / screen.y
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


## Whether the pointer is over a control (a panel, a list, a field): the wheel and the
## drags that start there belong to it, never to the 3D view, even when it lets them go
## (a ScrollContainer at the end of its scroll, a label).
func _over_gui(at: Vector2) -> bool:
	if get_viewport().gui_get_hovered_control() != null:
		return true
	return over_ui.is_valid() and bool(over_ui.call(at))


func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		var mb := event as InputEventMouseButton
		if mb.pressed and _over_gui(mb.position):
			return
		match mb.button_index:
			# A notch of the wheel is a press and a release: zoom once, on the press (the
			# release of a notch over a panel must not reach the view either).
			MOUSE_BUTTON_WHEEL_UP:
				if mb.pressed:
					dist = maxf(1.5, dist * WHEEL_STEP)
					update()
			MOUSE_BUTTON_WHEEL_DOWN:
				if mb.pressed:
					dist = minf(5000.0, dist / WHEEL_STEP)
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

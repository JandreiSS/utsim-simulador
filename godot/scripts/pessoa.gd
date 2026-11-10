## Skeletal person: the CC0 mannequin of the Universal Animation Library, driven by the
## clip index and clip time that the bridge computes from `Pose.motion` (the mapping and
## the strides of `crates/utsim-viewer/src/people.rs`, ported to `utsim-godot/looks.rs`).
##
## The clips of the second file (`pessoa_operador_carga`: walking with a box) are added
## as the animation library "ual2"; both files share the skeleton and the node paths, so
## its tracks drive this mannequin as they are.
##
## The bodies made in Blender (`pessoa_homem.glb`, `pessoa_mulher.glb`,
## `docs/elementos_3d_blender.md`) have the UAL1 skeleton and no clips: [method bind] takes
## the UAL1 clips too and names the skeleton's path as the tracks expect, and
## [method vestir] shows the person's protective equipment and puts its colours on.
##
## An AnimationTree in manual mode, every clip seeked to the time the scene gives, so
## jumping to `t` gives the pose of `t`:
##   slot a and slot b: clip (TimeSeek) blended, with a filter on the upper body, with
##   `Walk_Carry_Loop` held at 0 (the arms of the IdleCarry clip); `mix` crossfades a to b
##   in FADE seconds of real time.
class_name Pessoa
extends RefCounted

const FADE := 0.25
## Clip indices of `UtsimPlayback.person_clips()`.
const IDLE_CARRY := 6

## Work clothes, helmet and vest (sRGB), as in the Bevy viewer.
const SUIT := Color(0.231, 0.431, 0.647)
const JOINTS := Color(0.17, 0.24, 0.31)
const HELMET := Color(1.0, 0.82, 0.0)
const VEST := Color(0.96, 0.49, 0.0)
const STRIPE := Color(0.87, 0.89, 0.91)

var tree: AnimationTree
var skeleton: Skeleton3D
var hand_l := -1
var hand_r := -1
var durations := PackedFloat32Array()
var _names: PackedStringArray
var _anim_a: AnimationNodeAnimation
var _anim_b: AnimationNodeAnimation
var _clip_a := -1
var _clip_b := -1
var _fade := 1.0
var _time_a := 0.0
var _time_b := 0.0

static var _materials := {}


## Binds the mannequin instance `asset` (the imported scene of UAL1); `clips2` is the
## AnimationLibrary of UAL2; `names` the clip names in index order. `clips1`: the
## AnimationLibrary of UAL1, for a body without clips of its own (a Blender body); then
## the mannequin is not dressed (the body has its own clothes and equipment). Returns
## false when the scene is not the expected one (no skeleton, or no clips).
func bind(asset: Node3D, clips2: AnimationLibrary, names: PackedStringArray, clips1: AnimationLibrary = null) -> bool:
	_names = names
	var player := _find(asset, "AnimationPlayer") as AnimationPlayer
	skeleton = _find(asset, "Skeleton3D") as Skeleton3D
	if skeleton == null:
		return false
	var root: Node = null
	var lib: AnimationLibrary = null
	var own_clips := player != null and player.has_animation_library(&"")
	if own_clips:
		root = player.get_node(player.root_node)
		lib = player.get_animation_library(&"")
		player.active = false
	elif clips1 != null:
		lib = clips1
		root = _align(asset, clips1)
	if root == null or lib == null:
		return false
	tree = AnimationTree.new()
	tree.name = "Clipes"
	root.add_child(tree)
	tree.root_node = tree.get_path_to(root)
	tree.add_animation_library(&"", lib)
	if clips2 != null:
		tree.add_animation_library(&"ual2", clips2)
	tree.callback_mode_process = AnimationMixer.ANIMATION_CALLBACK_MODE_PROCESS_MANUAL
	# Godot's importer drops the "_Loop" suffix of looping clips ("Idle_Loop" is "Idle").
	_names = PackedStringArray()
	durations.resize(names.size())
	for k in names.size():
		var resolved := _resolve(names[k])
		if resolved == "":
			push_warning("pessoa: clipe %s não encontrado; o boneco fica sem animação" % names[k])
			tree.queue_free()
			if own_clips:
				player.active = true
			return false
		_names.append(resolved)
		durations[k] = tree.get_animation(StringName(resolved)).length
	var skel_path := String(root.get_path_to(skeleton))
	var bt := AnimationNodeBlendTree.new()
	_anim_a = _slot(bt, "a", skel_path)
	_anim_b = _slot(bt, "b", skel_path)
	var mix := AnimationNodeBlend2.new()
	bt.add_node(&"mix", mix)
	bt.connect_node(&"mix", 0, &"a_arms")
	bt.connect_node(&"mix", 1, &"b_arms")
	bt.connect_node(&"output", 0, &"mix")
	tree.tree_root = bt
	tree.active = true
	hand_l = skeleton.find_bone("hand_l")
	hand_r = skeleton.find_bone("hand_r")
	if own_clips:
		_dress(asset)
	return true


## A body without clips: names the skeleton and its parents as the bone tracks of `clips`
## address them (`Armature/Skeleton3D:pelvis` in the UAL files) and returns the node they
## are relative to, or null when the path does not fit inside `asset`.
func _align(asset: Node3D, clips: AnimationLibrary) -> Node:
	var prefix := ""
	for clip_name in clips.get_animation_list():
		var anim := clips.get_animation(clip_name)
		for k in anim.get_track_count():
			var path := anim.track_get_path(k)
			if path.get_subname_count() > 0:
				prefix = String(path.get_concatenated_names())
				break
		if prefix != "":
			break
	if prefix == "":
		return null
	var parts := prefix.split("/")
	var n: Node = skeleton
	for k in range(parts.size() - 1, -1, -1):
		if n == null or not (n == asset or asset.is_ancestor_of(n)):
			push_warning("pessoa: o caminho %s dos clipes não cabe no corpo" % prefix)
			return null
		if String(n.name) != parts[k]:
			n.name = parts[k]
		if k > 0:
			n = n.get_parent()
	return n.get_parent()


## Dresses a Blender body (`block.pessoa` of the static scene): only the `epi_<name>`
## meshes of its `epis` are shown, and the materials of `cores` (`pele`, `cabelo`,
## `camisa`...) get those colours.
static func vestir(asset: Node3D, info: Dictionary) -> void:
	var epis: PackedStringArray = info.get("epis", PackedStringArray())
	var cores: Dictionary = info.get("cores", {})
	for mi in _meshes(asset):
		var nm := String(mi.name)
		if nm.begins_with("epi_"):
			mi.visible = epis.has(nm.trim_prefix("epi_"))
		for s in mi.mesh.get_surface_count():
			var m := mi.mesh.surface_get_material(s)
			if m != null and cores.has(m.resource_name):
				mi.set_surface_override_material(s, _tint(m, Color.html(String(cores[m.resource_name]))))


## Names of the protective equipment shown on a Blender body (the visible `epi_*` meshes).
static func epis_visiveis(asset: Node3D) -> PackedStringArray:
	var out := PackedStringArray()
	for mi in _meshes(asset):
		var nm := String(mi.name)
		if nm.begins_with("epi_") and mi.visible:
			out.append(nm.trim_prefix("epi_"))
	out.sort()
	return out


## The animation name of a glTF clip name (`[library/]name`) in this tree, or "".
func _resolve(clip: String) -> String:
	if tree.has_animation(StringName(clip)):
		return clip
	if clip.ends_with("_Loop"):
		var trimmed := clip.trim_suffix("_Loop")
		if tree.has_animation(StringName(trimmed)):
			return trimmed
	return ""


## One slot: the clip with its TimeSeek, and the arms layer for IdleCarry.
func _slot(bt: AnimationNodeBlendTree, s: String, skel_path: String) -> AnimationNodeAnimation:
	var anim := AnimationNodeAnimation.new()
	anim.animation = StringName(_names[0])
	bt.add_node(StringName(s + "_anim"), anim)
	bt.add_node(StringName(s + "_seek"), AnimationNodeTimeSeek.new())
	bt.connect_node(StringName(s + "_seek"), 0, StringName(s + "_anim"))
	var hold := AnimationNodeAnimation.new()
	hold.animation = StringName(_names[IDLE_CARRY])
	bt.add_node(StringName(s + "_hold"), hold)
	bt.add_node(StringName(s + "_hseek"), AnimationNodeTimeSeek.new())
	bt.connect_node(StringName(s + "_hseek"), 0, StringName(s + "_hold"))
	var arms := AnimationNodeBlend2.new()
	arms.filter_enabled = true
	for b in skeleton.get_bone_count():
		if _upper(b):
			arms.set_filter_path(NodePath(skel_path + ":" + skeleton.get_bone_name(b)), true)
	bt.add_node(StringName(s + "_arms"), arms)
	bt.connect_node(StringName(s + "_arms"), 0, StringName(s + "_seek"))
	bt.connect_node(StringName(s + "_arms"), 1, StringName(s + "_hseek"))
	return anim


## The spine and everything above it (arms, head): the IdleCarry arms layer.
func _upper(bone: int) -> bool:
	var b := bone
	while b >= 0:
		if skeleton.get_bone_name(b).begins_with("spine_"):
			return true
		b = skeleton.get_bone_parent(b)
	return false


## Plays `clip` at `time` (seconds into it). `dt` is real time since the last call;
## `snap` skips the crossfade (jumps, captures). The clip fading out keeps its last time.
func drive(clip: int, time: float, dt: float, snap: bool) -> void:
	if tree == null or clip < 0 or clip >= _names.size():
		return
	if clip != _clip_b:
		_clip_a = _clip_b if _clip_b >= 0 else clip
		_time_a = _time_b
		_anim_a.animation = _anim_b.animation
		_clip_b = clip
		# IdleCarry plays the idle legs; its arms come from the hold layer.
		_anim_b.animation = StringName(_names[0] if clip == IDLE_CARRY else _names[clip])
		_fade = 0.0
	_time_b = time
	_fade = 1.0 if snap else minf(1.0, _fade + dt / FADE)
	tree.set(&"parameters/a_seek/seek_request", _time_a)
	tree.set(&"parameters/b_seek/seek_request", _time_b)
	tree.set(&"parameters/a_hseek/seek_request", 0.0)
	tree.set(&"parameters/b_hseek/seek_request", 0.0)
	tree.set(&"parameters/a_arms/blend_amount", 1.0 if _clip_a == IDLE_CARRY else 0.0)
	tree.set(&"parameters/b_arms/blend_amount", 1.0 if _clip_b == IDLE_CARRY else 0.0)
	tree.set(&"parameters/mix/blend_amount", _fade)
	tree.advance(0.0)


## Name of the clip playing now (the Godot animation name, without "_Loop"), or "".
func clip_name() -> String:
	return _names[_clip_b] if _clip_b >= 0 and _clip_b < _names.size() else ""


## Midpoint of the hands in world space, or null before the skeleton is bound.
func hands() -> Variant:
	if skeleton == null or hand_l < 0 or hand_r < 0:
		return null
	var g := skeleton.global_transform
	var l := g * skeleton.get_bone_global_pose(hand_l).origin
	var r := g * skeleton.get_bone_global_pose(hand_r).origin
	return (l + r) * 0.5


## Work clothes on the untextured mannequin, a helmet on the head and a vest on the chest
## (placed in the rest pose, file units, Y up, facing +Z, like the Bevy viewer).
func _dress(asset: Node3D) -> void:
	for mi in _meshes(asset):
		for s in mi.mesh.get_surface_count():
			var m := mi.mesh.surface_get_material(s)
			if m == null:
				continue
			match m.resource_name:
				"M_Main":
					mi.set_surface_override_material(s, _tint(m, SUIT))
				"M_Joints":
					mi.set_surface_override_material(s, _tint(m, JOINTS))
	var to_skel := _local_to(asset, skeleton)
	var head := skeleton.find_bone("Head")
	var chest := skeleton.find_bone("spine_02")
	if head >= 0:
		var helmet := CylinderMesh.new()
		helmet.top_radius = 0.075
		helmet.bottom_radius = 0.118
		helmet.height = 0.085
		_attach(head, to_skel, Vector3(0, 1.778, 0.008), Vector3.ONE, helmet, HELMET, 0.4)
		var crown := SphereMesh.new()
		crown.radius = 0.075
		crown.height = 0.15
		_attach(head, to_skel, Vector3(0, 1.818, 0.008), Vector3(1, 0.45, 1), crown, HELMET, 0.4)
		var brim := CylinderMesh.new()
		brim.top_radius = 0.122
		brim.bottom_radius = 0.122
		brim.height = 0.012
		_attach(head, to_skel, Vector3(0, 1.738, 0.03), Vector3.ONE, brim, HELMET, 0.4)
	if chest >= 0:
		var vest := BoxMesh.new()
		vest.size = Vector3(0.355, 0.34, 0.27)
		_attach(chest, to_skel, Vector3(0, 1.235, 0), Vector3.ONE, vest, VEST, 0.7)
		var stripe := BoxMesh.new()
		stripe.size = Vector3(0.365, 0.035, 0.28)
		_attach(chest, to_skel, Vector3(0, 1.17, 0), Vector3.ONE, stripe, STRIPE, 0.3)
		_attach(chest, to_skel, Vector3(0, 1.30, 0), Vector3.ONE, stripe, STRIPE, 0.3)


func _attach(bone: int, to_skel: Transform3D, at: Vector3, scale: Vector3, mesh: Mesh, color: Color, rough: float) -> void:
	var holder := BoneAttachment3D.new()
	holder.bone_name = skeleton.get_bone_name(bone)
	skeleton.add_child(holder)
	var rest := to_skel * skeleton.get_bone_global_rest(bone)
	var mi := MeshInstance3D.new()
	mi.mesh = mesh
	mi.material_override = _plain(color, rough)
	mi.transform = rest.affine_inverse() * Transform3D(Basis.from_scale(scale), at)
	holder.add_child(mi)


## Transform of `node` in the frame of `ancestor` (product of the local transforms).
static func _local_to(ancestor: Node, node: Node3D) -> Transform3D:
	var t := Transform3D.IDENTITY
	var at: Node = node
	while at != null and at != ancestor:
		if at is Node3D:
			t = (at as Node3D).transform * t
		at = at.get_parent()
	return t


static func _plain(color: Color, rough: float) -> StandardMaterial3D:
	var key := "%s/%f" % [color.to_html(), rough]
	if not _materials.has(key):
		var m := StandardMaterial3D.new()
		m.albedo_color = color
		m.roughness = rough
		_materials[key] = m
	return _materials[key]


static func _tint(original: Material, color: Color) -> Material:
	var key := "%d/%s" % [original.get_instance_id(), color.to_html()]
	if not _materials.has(key):
		var m := original.duplicate() as Material
		if m is BaseMaterial3D:
			(m as BaseMaterial3D).albedo_color = color
		_materials[key] = m
	return _materials[key]


static func _find(node: Node, type: String) -> Node:
	if node.is_class(type):
		return node
	for c in node.get_children():
		var f := _find(c, type)
		if f != null:
			return f
	return null


static func _meshes(node: Node) -> Array[MeshInstance3D]:
	var out: Array[MeshInstance3D] = []
	if node is MeshInstance3D and (node as MeshInstance3D).mesh != null:
		out.append(node)
	for c in node.get_children():
		out.append_array(_meshes(c))
	return out


## Whether a catalogue name is the skeletal mannequin (either file of the library).
static func _is_person(name: String) -> bool:
	var n := name.strip_edges().to_lower()
	return n == "pessoa_operador" or n == "pessoa_operador_carga"

"""Cena do enunciado de uma missão do UTSim Ensino: monta no Blender os elementos 3D de uma
planta (JSON gerado pelo `utsim-ensino-core`, `cena::Planta`), renderiza a imagem e grava
as âncoras, o ponto da imagem de cada objeto, para o app prender os cartões do enunciado.

    blender -b --factory-startup -P tools/blender/enunciado.py -- \
        --planta planta.json --saida missao.cena.png [--pasta assets/modelos3d]

A planta tem `objetos` (nome, elemento, x, y, giro, fases, pessoa), `pecas` (elemento, x, y,
giro), `largura`, `altura` e `vista` (direção da câmera, de cima em três quartos). As âncoras
vão para `<saida sem .png>.ancoras.json`: `{"largura", "altura", "nos": {nome: [x, y]}}`, com
o ponto acima de cada objeto (`nome`) e o pé dele (`nome@pe`).
"""

import argparse
import json
import math
import os
import sys

import bpy
from bpy_extras.object_utils import world_to_camera_view
from mathutils import Vector

RAIZ = os.environ.get("UTSIM_RAIZ") or os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
CLIPES = os.path.join(RAIZ, "assets", "terceiros", "quaternius_ual", "UAL1_Standard.glb")

argv = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
p = argparse.ArgumentParser()
p.add_argument("--planta", required=True)
p.add_argument("--saida", required=True)
p.add_argument("--pasta", default=os.path.join(RAIZ, "assets", "modelos3d"))
a = p.parse_args(argv)
P = json.load(open(a.planta, encoding="utf-8"))
QUADRO = 100

bpy.ops.wm.read_factory_settings(use_empty=True)
cena = bpy.context.scene


def mat(nome, rgb, rough=0.9):
    m = bpy.data.materials.new(nome)
    m.use_nodes = True
    b = m.node_tree.nodes["Principled BSDF"]
    b.inputs["Base Color"].default_value = (*rgb, 1)
    b.inputs["Roughness"].default_value = rough
    return m


def plano(nome, x0, y0, x1, y1, z, m):
    me = bpy.data.meshes.new(nome)
    me.from_pydata([(x0, y0, z), (x1, y0, z), (x1, y1, z), (x0, y1, z)], [], [(0, 1, 2, 3)])
    me.materials.append(m)
    o = bpy.data.objects.new(nome, me)
    cena.collection.objects.link(o)


def arquivo(elemento):
    for nome in (elemento + "_medio", elemento):
        c = os.path.join(a.pasta, nome + ".glb")
        if os.path.exists(c):
            return c
    raise SystemExit(f"enunciado: elemento '{elemento}' não existe em {a.pasta}")


def importar(elemento, x, y, giro, fases, escala=1.0):
    antes = set(bpy.data.objects)
    bpy.ops.import_scene.gltf(filepath=arquivo(elemento))
    novos = [o for o in bpy.data.objects if o not in antes]
    raiz = [o for o in novos if o.parent is None][0]
    for o in novos:
        ad = o.animation_data
        if not ad:
            continue
        ad.action = None
        for tr in ad.nla_tracks:
            tr.mute = tr.name not in fases
            for st in tr.strips:
                if tr.name in fases:
                    st.extrapolation = "HOLD"
                    st.frame_start_ui = QUADRO - fases[tr.name] * 24
    raiz.location = (x, y, raiz.location.z)
    raiz.rotation_mode = "XYZ"
    raiz.rotation_euler = (0, 0, math.radians(giro))
    raiz.scale = (escala, escala, escala)
    return {o.name.split(".")[0]: o for o in novos}, novos


# Os clipes do boneco UAL1 (as pessoas usam o mesmo esqueleto): importa uma vez e apaga a
# malha, ficando só as ações.
antes = set(bpy.data.objects)
bpy.ops.import_scene.gltf(filepath=CLIPES)
for o in [o for o in bpy.data.objects if o not in antes]:
    bpy.data.objects.remove(o, do_unlink=True)


def pessoa(sexo, ponto, fase):
    antes = set(bpy.data.objects)
    bpy.ops.import_scene.gltf(filepath=arquivo("pessoa_" + sexo))
    novos = [o for o in bpy.data.objects if o not in antes]
    for o in novos:
        if o.name.startswith("epi_"):
            o.hide_render = True
    arm = [o for o in novos if o.type == "ARMATURE"][0]
    act = bpy.data.actions.get("Interact")
    ad = arm.animation_data or arm.animation_data_create()
    for tr in list(ad.nla_tracks):
        ad.nla_tracks.remove(tr)
    ad.action = None
    ini, fim = act.frame_range
    tr = ad.nla_tracks.new()
    st = tr.strips.new("Interact", int(ini), act)
    st.frame_start_ui = QUADRO - fase * (fim - ini)
    st.extrapolation = "HOLD"
    mw = ponto.matrix_world
    fx, fy = mw.col[0].x, mw.col[0].y
    pai = bpy.data.objects.new("pessoa", None)
    cena.collection.objects.link(pai)
    pai.location = (mw.translation.x, mw.translation.y, 0.0)
    pai.rotation_euler = (0, 0, math.atan2(fy, fx) + math.pi / 2)
    for o in novos:
        if o.parent is None:
            o.parent = pai


nos = {}
caixa_min = [math.inf, math.inf]
caixa_max = [-math.inf, -math.inf]
for ob in P["objetos"]:
    n, novos = importar(ob["elemento"], ob["x"], ob["y"], ob.get("giro", 0.0), ob.get("fases", {}))
    nos[ob["nome"]] = (ob, n, novos)
    for k, v in ((0, ob["x"]), (1, ob["y"])):
        caixa_min[k] = min(caixa_min[k], v)
        caixa_max[k] = max(caixa_max[k], v)
for pc in P.get("pecas", []):
    importar(pc["elemento"], pc["x"], pc["y"], pc.get("giro", 0.0), {}, pc.get("escala", 1.0))

cena.frame_set(QUADRO)
for i, ob in enumerate(P["objetos"]):
    sexo = ob.get("pessoa")
    if not sexo:
        continue
    ponto = nos[ob["nome"]][1].get("ponto_operador")
    if ponto is not None:
        pessoa(sexo, ponto, 0.15 + 0.11 * (i % 6))
cena.frame_set(QUADRO)

# Piso do simulador e a faixa amarela do corredor, à frente da linha.
m = 30.0
plano("piso", caixa_min[0] - m, caixa_min[1] - m, caixa_max[0] + m, caixa_max[1] + m, 0,
      mat("piso", (0.50, 0.53, 0.57)))
y_faixa = caixa_min[1] - 2.3
plano("faixa", caixa_min[0] - m, y_faixa - 0.06, caixa_max[0] + m, y_faixa + 0.06, 0.002,
      mat("faixa", (0.95, 0.78, 0.05), 0.6))

sol = bpy.data.lights.new("sol", "SUN")
sol.energy = 1.9
sol.angle = math.radians(8)
os_ = bpy.data.objects.new("sol", sol)
os_.rotation_euler = (math.radians(42), math.radians(10), math.radians(25))
cena.collection.objects.link(os_)
mundo = bpy.data.worlds.new("mundo")
mundo.use_nodes = True
mundo.node_tree.nodes["Background"].inputs["Color"].default_value = (0.93, 0.95, 0.97, 1)
mundo.node_tree.nodes["Background"].inputs["Strength"].default_value = 0.45
cena.world = mundo

# Câmera ortográfica, de cima em três quartos, enquadrando a planta com folga em cima para
# os cartões.
L, H = P.get("largura", 2200), P.get("altura", 860)
cena.render.resolution_x = L
cena.render.resolution_y = H
cam = bpy.data.cameras.new("cam")
cam.type = "ORTHO"
oc = bpy.data.objects.new("cam", cam)
cena.collection.objects.link(oc)
direcao = Vector(P.get("vista", [-0.16, -1.0, 0.80])).normalized()
centro = Vector(((caixa_min[0] + caixa_max[0]) / 2, (caixa_min[1] + caixa_max[1]) / 2, 0.0))
oc.location = centro + direcao * 80
oc.rotation_euler = (-direcao).to_track_quat("-Z", "Y").to_euler()
cena.camera = oc
pontos = []
for ob, n, novos in nos.values():
    for o in novos:
        if o.type == "MESH":
            pontos += [o.matrix_world @ Vector(c) for c in o.bound_box]
cam.ortho_scale = 10.0
bpy.context.view_layer.update()
proj = [world_to_camera_view(cena, oc, v) for v in pontos]
lx = max(v.x for v in proj) - min(v.x for v in proj)
ly = max(v.y for v in proj) - min(v.y for v in proj)
# Ocupa 94 % da largura, ou 55 % da altura (o resto fica para os cartões, em cima).
cam.ortho_scale = 10.0 * max(lx / 0.94, ly / 0.55)
bpy.context.view_layer.update()
proj = [world_to_camera_view(cena, oc, v) for v in pontos]
cx = (max(v.x for v in proj) + min(v.x for v in proj)) / 2
cy_baixo = min(v.y for v in proj)
dx = (cx - 0.5) * cam.ortho_scale
dy = (cy_baixo - 0.06) * cam.ortho_scale * (H / L)
oc.location += oc.matrix_world.to_3x3() @ Vector((dx, dy, 0))
bpy.context.view_layer.update()

cena.render.engine = "BLENDER_EEVEE"
cena.view_settings.view_transform = "Standard"
cena.render.filepath = os.path.abspath(a.saida)

ancoras = {}
for nome, (ob, n, novos) in nos.items():
    malhas = [o for o in novos if o.type == "MESH"]
    topo = max((o.matrix_world @ Vector(c)).z for o in malhas for c in o.bound_box)
    for chave, z in ((nome, topo + 0.1), (nome + "@pe", 0.0)):
        v = world_to_camera_view(cena, oc, Vector((ob["x"], ob["y"], z)))
        ancoras[chave] = [round(v.x * L, 1), round((1 - v.y) * H, 1)]
base = os.path.splitext(os.path.abspath(a.saida))[0]
with open(base + ".ancoras.json", "w", encoding="utf-8") as f:
    json.dump({"largura": L, "altura": H, "nos": ancoras}, f, indent=1, sort_keys=True)

bpy.ops.render.render(write_still=True)
print("enunciado:", cena.render.filepath)

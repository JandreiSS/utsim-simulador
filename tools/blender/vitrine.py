"""Vitrine: monta os `.glb` gerados numa pequena linha de produção e renderiza uma
imagem, para conferir o visual e as interações (peça na garra, palete no garfo,
operador no posto, portão aberto).

    blender -b --factory-startup -P tools/blender/vitrine.py -- --saida vitrine.png \
        [--pasta assets/modelos3d] [--sufixo _medio] [--cam=x,y,z,rx,rz]

Usa o boneco CC0 de `assets/terceiros/quaternius_ual` para as pessoas.
"""

import argparse
import math
import os
import sys

import bpy

AQUI = os.path.dirname(os.path.abspath(__file__))
RAIZ = os.path.normpath(os.path.join(AQUI, "..", ".."))
BONECO = os.path.join(RAIZ, "assets", "terceiros", "quaternius_ual", "UAL1_Standard.glb")

argv = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
p = argparse.ArgumentParser()
p.add_argument("--pasta", default=os.path.join(RAIZ, "assets", "modelos3d"))
p.add_argument("--saida", required=True)
p.add_argument("--sufixo", default="")
p.add_argument("--cam", default="")
p.add_argument("--largura", type=int, default=1600)
p.add_argument("--altura", type=int, default=900)
a = p.parse_args(argv)

QUADRO = 100

# (id, elemento, x, y, giro, {clipe: fase}, (preso_a_id, ponto) ou None); com `preso`,
# x e y são o deslocamento a partir do ponto.
CENA = [
    ("pp", "porta_paletes", -11.0, 5.0, 0, {}, None),
    ("pal1", "palete", 0, 0, 0, {}, ("pp", "ponto_vao_0_1")),
    ("pal2", "palete", 0, 0, 0, {}, ("pp", "ponto_vao_2_2")),
    ("emp", "empilhadeira", -9.6, 1.6, 90, {"garfo_subir": 0.55}, None),
    ("pal3", "palete", 0, 0, 0, {}, ("emp", "ponto_carga")),
    ("est", "esteira", -4.6, 0.0, 0, {"rolar": 0.3}, None),
    ("cx1", "caixa", 0.6, 0, 0, {}, ("est", "ponto_entrada")),
    ("pc1", "peca", 2.4, 0, 0, {}, ("est", "ponto_entrada")),
    ("rob", "robo", -2.0, 0.0, 90, {"levar": 0.2}, None),
    ("pc2", "peca", 0, 0, 0, {}, ("rob", "ponto_garra")),
    ("cnc", "cnc", 0.3, 0.95, 0, {"porta_abrir": 1.0}, None),
    ("prensa", "prensa", 4.0, 0.95, 0, {"ciclo": 0.5}, None),
    ("inj", "injetora", 8.6, 0.95, 0, {"ciclo": 0.1}, None),
    ("posto", "posto", 4.2, -3.6, 180, {"ciclo": 0.5}, None),
    ("agv", "agv", 0.6, -3.2, 0, {"mesa_subir": 1.0}, None),
    ("cx2", "caixa", 0, 0, 0, {}, ("agv", "ponto_carga")),
    ("estante", "estante", 8.6, -3.8, 180, {}, None),
    ("espera", "espera", 6.6, -3.6, 0, {}, None),
    ("cel", "celula", -6.0, -5.0, 0, {"porta_abrir": 1.0}, None),
    ("rob2", "robo", -6.0, -4.6, 0, {"ciclo": 0.15}, None),
]
# Pessoas: (id do elemento, ponto, clipe do boneco, fase).
PESSOAS = [
    ("cnc", "ponto_operador", "Interact", 0.4),
    ("posto", "ponto_operador", "Interact", 0.7),
    ("cel", "ponto_porta", "Walk_Loop", 0.3),
]

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


plano("piso", -16, -10, 14, 10, 0, mat("piso", (0.40, 0.42, 0.44)))
faixa = mat("faixa_piso", (0.95, 0.78, 0.05), 0.6)
for x0, y0, x1, y1 in ((-14, -1.9, 12, -1.8), (-14, -1.0, 12, -0.9)):
    plano("faixa", x0, y0, x1, y1, 0.002, faixa)

objetos = {}
for ident, nome, x, y, giro, fases, preso in CENA:
    arq = os.path.join(a.pasta, nome + a.sufixo + ".glb")
    antes = set(bpy.data.objects)
    bpy.ops.import_scene.gltf(filepath=arq)
    novos = [o for o in bpy.data.objects if o not in antes]
    raiz = [o for o in novos if o.parent is None][0]
    objetos[ident] = {o.name.split(".")[0]: o for o in novos}
    for o in novos:
        ad = o.animation_data
        if not ad:
            continue
        # O importador põe cada clipe numa faixa NLA: deixa só os pedidos, na fase pedida.
        ad.action = None
        for tr in ad.nla_tracks:
            tr.mute = tr.name not in fases
            for st in tr.strips:
                if tr.name in fases:
                    st.extrapolation = "HOLD"
                    st.frame_start_ui = QUADRO - fases[tr.name] * 24
    if preso:
        raiz.parent = objetos[preso[0]][preso[1]]
        raiz.matrix_parent_inverse.identity()
        raiz.location = (x, y, 0)
        raiz.rotation_mode = "XYZ"
        raiz.rotation_euler = (0, 0, 0)
    else:
        raiz.location = (x, y, 0)
        raiz.rotation_mode = "XYZ"
        raiz.rotation_euler = (0, 0, math.radians(giro))

cena.frame_set(QUADRO)

for ident, ponto, clipe, fase in PESSOAS:
    antes = set(bpy.data.objects)
    bpy.ops.import_scene.gltf(filepath=BONECO)
    novos = [o for o in bpy.data.objects if o not in antes]
    arm = [o for o in novos if o.type == "ARMATURE"][0]
    for o in novos:
        if o.type == "MESH" and o.parent is None:
            o.hide_render = True  # esfera de referência do arquivo
    act = bpy.data.actions.get(clipe)
    ad = arm.animation_data or arm.animation_data_create()
    for tr in list(ad.nla_tracks):
        ad.nla_tracks.remove(tr)
    ini, fim = act.frame_range
    ad.action = None
    tr = ad.nla_tracks.new()
    st = tr.strips.new(clipe, int(ini), act)
    st.frame_start_ui = QUADRO - fase * (fim - ini)
    st.extrapolation = "HOLD"
    pt = objetos[ident][ponto]
    mw = pt.matrix_world
    fx, fy = mw.col[0].x, mw.col[0].y
    # A animação do boneco mexe no próprio nó: o lugar e o giro vão num nó pai.
    pai = bpy.data.objects.new("pessoa", None)
    cena.collection.objects.link(pai)
    pai.location = (mw.translation.x, mw.translation.y, 0.0)
    # O boneco olha para -y do Blender depois da importação: gira para o +x do ponto.
    pai.rotation_euler = (0, 0, math.atan2(fy, fx) + math.pi / 2)
    for o in novos:
        if o.parent is None:
            o.parent = pai

cena.frame_set(QUADRO)

sol = bpy.data.lights.new("sol", "SUN")
sol.energy = 3.2
sol.angle = math.radians(6)
os_ = bpy.data.objects.new("sol", sol)
os_.rotation_euler = (math.radians(48), math.radians(12), math.radians(30))
cena.collection.objects.link(os_)
mundo = bpy.data.worlds.new("mundo")
mundo.use_nodes = True
mundo.node_tree.nodes["Background"].inputs["Color"].default_value = (0.62, 0.68, 0.75, 1)
mundo.node_tree.nodes["Background"].inputs["Strength"].default_value = 0.9
cena.world = mundo

cam = bpy.data.cameras.new("cam")
cam.lens = 28
oc = bpy.data.objects.new("cam", cam)
cena.collection.objects.link(oc)
cx, cy, cz, rx, rz = (float(v) for v in (a.cam or "-1.5,-17,10.5,57,0").split(","))
oc.location = (cx, cy, cz)
oc.rotation_euler = (math.radians(rx), 0, math.radians(rz))
cena.camera = oc

cena.render.engine = "BLENDER_EEVEE"
cena.render.resolution_x = a.largura
cena.render.resolution_y = a.altura
cena.view_settings.view_transform = "Standard"
cena.render.filepath = os.path.abspath(a.saida)
bpy.ops.render.render(write_still=True)
print("vitrine:", cena.render.filepath)

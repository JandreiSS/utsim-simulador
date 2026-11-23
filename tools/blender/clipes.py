"""Clipes de pessoa corrigidos (`pessoa_clipes.glb`).

Os clipes das pessoas vêm da Universal Animation Library (CC0, `assets/terceiros`). Alguns
foram feitos para um boneco de jogo e ficam errados num corpo humano. Este módulo lê o clipe
original, corrige a pose quadro a quadro e grava só o esqueleto com os clipes corrigidos, com
os mesmos nomes de nó e de clipe: quem toca o clipe troca o original por este, sem mais nada.

`Walk_Carry_Loop` (andar com caixa): no original, o tronco deita uns 25° para trás e a cabeça
olha para o teto. A correção dobra a coluna, o pescoço e a cabeça para a frente. Os braços vão
junto com o tórax, e a caixa fica na frente da barriga.
"""

import math
import os

import bpy
from mathutils import Matrix

AQUI = os.path.dirname(os.path.abspath(__file__))
RAIZ = os.path.abspath(os.path.join(AQUI, "..", ".."))
UAL1 = os.path.join(RAIZ, "assets", "terceiros", "quaternius_ual", "UAL1_Standard.glb")
UAL2 = os.path.join(RAIZ, "assets", "terceiros", "quaternius_ual2", "UAL2_Standard.glb")

# Clipes do UAL1 que o UTSim usa sem alteração (`pessoa_clipes_base.glb`): clipe -> laço.
# Com este arquivo e o `pessoa_clipes.glb`, as pessoas não precisam das duas bibliotecas
# inteiras (86 clipes, 15,7 MB) no pacote.
BASE = {
    "Driving_Loop": True,
    "Idle_Loop": True,
    "Interact": False,
    "Jog_Fwd_Loop": True,
    "PickUp_Table": False,
    "Sitting_Idle_Loop": True,
    "Walk_Loop": True,
}

# clipe: (arquivo de origem, laço, ((osso, graus para a frente), ...)), do quadril para a cabeça.
CLIPES = {
    "Walk_Carry_Loop": (
        UAL2,
        True,
        (("spine_01", 8.0), ("spine_02", 8.0), ("spine_03", 6.0), ("neck_01", 8.0), ("Head", 12.0)),
    ),
}


def _corrigir(arm, act, dobras):
    ad = arm.animation_data
    ad.action = act
    if hasattr(ad, "action_slot") and getattr(act, "slots", None):
        ad.action_slot = act.slots[0]
    cena = bpy.context.scene
    ini, fim = (int(round(v)) for v in act.frame_range)
    mw = arm.matrix_world
    mwi = mw.inverted()
    for f in range(ini, fim + 1):
        cena.frame_set(f)
        for nome, graus in dobras:
            pb = arm.pose.bones[nome]
            m = mw @ pb.matrix
            j = m.translation
            # A pessoa olha para -y do Blender: girar em torno de +x leva a cabeça para a frente.
            r = Matrix.Translation(j) @ Matrix.Rotation(math.radians(graus), 4, "X") @ Matrix.Translation(-j)
            pb.matrix = mwi @ (r @ m)
            bpy.context.view_layer.update()
            pb.keyframe_insert("rotation_quaternion", frame=f)
    ad.action = None


def gerar(pasta, nome="pessoa_clipes"):
    bpy.ops.wm.read_factory_settings(use_empty=True)
    arm = None
    feitos = {}
    for clipe, (origem, laco, dobras) in CLIPES.items():
        if arm is None:
            bpy.ops.import_scene.gltf(filepath=origem)
            arm = [o for o in bpy.data.objects if o.type == "ARMATURE"][0]
            for o in [o for o in bpy.data.objects if o.type != "ARMATURE"]:
                bpy.data.objects.remove(o)
            ad = arm.animation_data or arm.animation_data_create()
            ad.action = None
            for tr in list(ad.nla_tracks):
                ad.nla_tracks.remove(tr)
        act = bpy.data.actions[clipe]
        _corrigir(arm, act, dobras)
        feitos[clipe] = (act, laco)
    return _gravar(arm, feitos, pasta, nome, {"esqueleto": "UAL1", "substitui": sorted(feitos)})


def gerar_base(pasta, nome="pessoa_clipes_base"):
    """Os clipes de `BASE`, copiados do UAL1 sem alteração, só com o esqueleto."""
    bpy.ops.wm.read_factory_settings(use_empty=True)
    bpy.ops.import_scene.gltf(filepath=UAL1)
    arm = [o for o in bpy.data.objects if o.type == "ARMATURE"][0]
    for o in [o for o in bpy.data.objects if o.type != "ARMATURE"]:
        bpy.data.objects.remove(o)
    ad = arm.animation_data or arm.animation_data_create()
    ad.action = None
    for tr in list(ad.nla_tracks):
        ad.nla_tracks.remove(tr)
    feitos = {c: (bpy.data.actions[c], laco) for c, laco in BASE.items()}
    return _gravar(arm, feitos, pasta, nome, {"esqueleto": "UAL1", "origem": "UAL1", "copia": sorted(feitos)})


def _gravar(arm, feitos, pasta, nome, dados):
    """Grava só o esqueleto `arm` com os clipes de `feitos` (nome -> (ação, laço))."""
    for a in list(bpy.data.actions):
        if a.name not in feitos:
            bpy.data.actions.remove(a)
    for clipe, (act, _) in feitos.items():
        tr = arm.animation_data.nla_tracks.new()
        tr.name = clipe
        st = tr.strips.new(clipe, int(act.frame_range[0]), act)
        st.name = clipe
    for pb in arm.pose.bones:
        pb.matrix_basis.identity()
    bpy.context.scene.frame_set(0)
    bpy.ops.export_scene.gltf(
        filepath=os.path.join(pasta, nome + ".glb"),
        export_format="GLB",
        export_yup=True,
        export_texcoords=False,
        export_tangents=False,
        export_attributes=False,
        export_extras=False,
        export_cameras=False,
        export_lights=False,
        export_animations=True,
        export_animation_mode="NLA_TRACKS",
        export_force_sampling=True,
        export_optimize_animation_size=False,
        export_frame_range=False,
        export_anim_slide_to_zero=False,
        export_def_bones=False,
        export_skins=True,
        export_morph=False,
    )
    return {
        "arquivo": nome + ".glb",
        "tamanho": [0.0, 0.0, 0.0],
        "minimo": [0.0, 0.0, 0.0],
        "triangulos": 0,
        "nos": [],
        "pontos": {},
        "clipes": {c: {"laco": laco, "nos": []} for c, (_, laco) in sorted(feitos.items())},
        "estados": {},
        "peca": {},
        "dados": dados,
    }

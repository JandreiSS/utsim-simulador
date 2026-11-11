"""Pessoas (homem e mulher) para o 3D: humanos do MakeHuman (MPFB) no esqueleto do UAL1.

    blender -b --factory-startup --python-exit-code 1 -P tools/blender/pessoas.py -- \
        [--saida assets/modelos3d] [--so homem]

Os humanos vêm do MPFB (o MakeHuman para o Blender) e do pacote de assets de sistema do
MakeHuman, os dois CC0 nos humanos gerados e nos assets. Eles ficam fora do repositório, na
pasta de `UTSIM_MPFB` (veja `assets/modelos3d/LEIA-ME.md`, que diz de onde baixar):

    <UTSIM_MPFB>/ext/mpfb      o complemento MPFB 2.0.18, descompactado
    <UTSIM_MPFB>/assets        o pacote `makehuman_system_assets_cc0`, descompactado

Cada arquivo `pessoa_<homem|mulher>.glb` traz o esqueleto do UAL1 (mesmos ossos, mesma pose
de repouso, sem animações: o 3D usa as do UAL1 e do UAL2) e:

- `corpo`: o humano do MakeHuman (corpo, olhos, sobrancelhas e cabelo) com camiseta, macacão
  de trabalho e sapato fechado. O rig `game_engine` do MPFB tem os mesmos nomes de ossos do
  UAL1; cada osso é girado (e os longos, esticados) até a pose do UAL1, e a malha vai junto,
  com os pesos do próprio MPFB. Sem textura: a cor vem só dos materiais, e o 3D recolore
  `pele`, `cabelo`, `camisa` e `calca` para variar as pessoas;
- `epi_<nome>`: uma malha por EPI de `PPE_NAMES` (capacete, óculos, protetor auricular,
  luvas, botina, colete, máscara, avental), escondida ou mostrada conforme o EPI que a pessoa
  veste no modelo. Cada EPI é feito sobre a superfície do corpo vestido (raios até a pele ou a
  roupa, com folga), então veste o corpo e deixa o rosto à mostra.

Metros, Z para cima; a pessoa olha para -y do Blender, como o UAL1.
"""

import argparse
import importlib
import math
import os
import sys

import addon_utils
import bmesh
import bpy
from mathutils import Matrix, Vector
from mathutils.bvhtree import BVHTree

AQUI = os.path.dirname(os.path.abspath(__file__))
RAIZ = os.path.normpath(os.path.join(AQUI, "..", ".."))
UAL1 = os.path.join(RAIZ, "assets", "terceiros", "quaternius_ual", "UAL1_Standard.glb")
MPFB = os.environ.get("UTSIM_MPFB") or os.path.join(
    os.path.expanduser("~"), "Documents", "Codes", "UTSim-origem-3d", "mpfb"
)
MPFB_VERSAO = (2, 0, 18)

# Cores (sRGB, 0..255), as mesmas convenções de `comum.PALETA`.
CORES = {
    "pele": (0xC6, 0x8E, 0x6B),
    "cabelo": (0x3B, 0x2A, 0x20),
    "camisa": (0x8C, 0x96, 0xA0),
    "calca": (0x2F, 0x4A, 0x6E),
    "calcado": (0x22, 0x20, 0x1F),
    "sola": (0x15, 0x14, 0x14),
    "fivela": (0xB4, 0xB8, 0xBD),
    "olho": (0x3A, 0x2A, 0x20),
    "olho_branco": (0xEE, 0xEA, 0xE4),
    "labio": (0xA8, 0x62, 0x58),
    "faixa": (0xD9, 0xDD, 0xE0),
    "epi_amarelo": (0xF2, 0xC1, 0x1D),
    "epi_lente": (0x5C, 0x6E, 0x7A),
    "epi_vermelho": (0xC8, 0x3A, 0x2E),
    "epi_laranja": (0xF0, 0x7A, 0x1E),
    "epi_cinza": (0x8A, 0x91, 0x99),
    "epi_branco": (0xEE, 0xEF, 0xF0),
    "epi_couro": (0x6B, 0x4A, 0x2E),
}


def lin(c):
    """sRGB 0..255 para linear (o glTF guarda o fator em linear)."""
    v = c / 255.0
    return v / 12.92 if v <= 0.04045 else ((v + 0.055) / 1.055) ** 2.4


def material(nome):
    if nome in bpy.data.materials:
        return bpy.data.materials[nome]
    m = bpy.data.materials.new(nome)
    m.use_nodes = True
    b = m.node_tree.nodes["Principled BSDF"]
    r, g, bl = CORES[nome]
    b.inputs["Base Color"].default_value = (lin(r), lin(g), lin(bl), 1.0)
    b.inputs["Roughness"].default_value = 0.3 if nome in ("olho", "olho_branco", "epi_lente") else 0.8
    if nome == "epi_lente":
        b.inputs["Alpha"].default_value = 0.4
        m.surface_render_method = "BLENDED"
    if nome == "fivela":
        b.inputs["Metallic"].default_value = 0.8
        b.inputs["Roughness"].default_value = 0.4
    return m


# ---------------------------------------------------------------------------
# MPFB
# ---------------------------------------------------------------------------

# Os humanos: macros do MakeHuman (0..1) e os assets do pacote de sistema.
HUMANOS = {
    "homem": {
        "macro": {"gender": 1.0, "age": 0.5, "muscle": 0.6, "weight": 0.55, "proportions": 0.6,
                  "height": 0.55, "cupsize": 0.5, "firmness": 0.5},
        "cabelo": "hair/short02/short02.mhclo",
        "sobrancelha": "eyebrows/eyebrow001/eyebrow001.mhclo",
    },
    "mulher": {
        "macro": {"gender": 0.0, "age": 0.45, "muscle": 0.5, "weight": 0.5, "proportions": 0.6,
                  "height": 0.5, "cupsize": 0.55, "firmness": 0.6},
        "cabelo": "hair/braid01/braid01.mhclo",
        "sobrancelha": "eyebrows/eyebrow010/eyebrow010.mhclo",
    },
}
RACA = {"african": 0.33, "asian": 0.33, "caucasian": 0.34}
OLHOS = "eyes/low-poly/low-poly.mhclo"
ROUPA = "clothes/male_worksuit01/male_worksuit01.mhclo"
SAPATO = "clothes/shoes04/shoes04.mhclo"
BOTINA = "clothes/shoes03/shoes03.mhclo"


def carregar_mpfb():
    """Liga o MPFB de `UTSIM_MPFB` como extensão local e devolve (HumanService, TargetService)."""
    ext = os.path.join(MPFB, "ext")
    if not os.path.isfile(os.path.join(ext, "mpfb", "__init__.py")) or not os.path.isdir(
        os.path.join(MPFB, "assets", "clothes")
    ):
        raise RuntimeError(
            f"MPFB e o pacote de assets do MakeHuman não estão em {MPFB} "
            "(defina UTSIM_MPFB; veja assets/modelos3d/LEIA-ME.md)"
        )
    repos = bpy.context.preferences.extensions.repos
    if not any(r.module == "utsim" for r in repos):
        repos.new(name="utsim", module="utsim", custom_directory=ext, source="USER")
    addon_utils.enable("bl_ext.utsim.mpfb", default_set=True)
    raiz = importlib.import_module("bl_ext.utsim.mpfb")
    if tuple(raiz.VERSION) != MPFB_VERSAO:
        raise RuntimeError(f"MPFB {raiz.VERSION} em {ext}; o UTSim espera {MPFB_VERSAO}")
    hs = importlib.import_module("bl_ext.utsim.mpfb.services.humanservice").HumanService
    ts = importlib.import_module("bl_ext.utsim.mpfb.services.targetservice").TargetService
    return hs, ts


def asset(hs, humano, caminho, tipo):
    """Veste um asset do pacote no humano e devolve o objeto novo."""
    antes = set(bpy.data.objects)
    hs.add_mhclo_asset(os.path.join(MPFB, "assets", caminho), humano, asset_type=tipo,
                       subdiv_levels=0, material_type="MAKESKIN")
    novos = [o for o in bpy.data.objects if o not in antes and o.type == "MESH"]
    assert len(novos) == 1, caminho
    return novos[0]


def humano(sexo):
    """Gera o humano no MPFB: devolve o rig e as malhas {papel: objeto}."""
    hs, ts = carregar_mpfb()
    macro = ts.get_default_macro_info_dict()
    macro.update(HUMANOS[sexo]["macro"])
    macro["race"] = dict(RACA)
    h = hs.create_human(macro_detail_dict=macro)
    hs.add_builtin_rig(h, "game_engine")
    partes = {"pele": h}
    partes["roupa"] = asset(hs, h, ROUPA, "Clothes")
    partes["calcado"] = asset(hs, h, SAPATO, "Clothes")
    partes["botina"] = asset(hs, h, BOTINA, "Clothes")
    partes["cabelo"] = asset(hs, h, HUMANOS[sexo]["cabelo"], "Hair")
    partes["sobrancelha"] = asset(hs, h, HUMANOS[sexo]["sobrancelha"], "Eyebrows")
    partes["olhos"] = asset(hs, h, OLHOS, "Eyes")
    rig = h.parent
    assert rig and rig.type == "ARMATURE"
    for ob in partes.values():
        # Os alvos (formas do corpo) e as máscaras viram malha; o esqueleto do MPFB sai.
        if ob.data.shape_keys:
            bpy.context.view_layer.objects.active = ob
            with bpy.context.temp_override(object=ob, active_object=ob):
                bpy.ops.object.shape_key_remove(all=True, apply_mix=True)
        for mod in list(ob.modifiers):
            if mod.type == "ARMATURE":
                ob.modifiers.remove(mod)
        for mod in list(ob.modifiers):
            with bpy.context.temp_override(object=ob, active_object=ob):
                bpy.ops.object.modifier_apply(modifier=mod.name)
    return rig, partes


# ---------------------------------------------------------------------------
# Da pose do MPFB para a do UAL1
# ---------------------------------------------------------------------------

# Nomes que mudam do rig `game_engine` para o UAL1; o resto é igual.
RENOMEAR = {"head": "Head", "Root": "root"}
# Ossos dos membros, esticados até o comprimento do osso do UAL1; mão e pé só giram.
ESTICA = ("thigh_", "calf_", "upperarm_", "lowerarm_")
# O tronco vai inteiro, sem se deformar: bacia, coluna e clavículas.
TRONCO_RIGIDO = ("Root", "pelvis", "spine_01", "spine_02", "spine_03", "clavicle_l", "clavicle_r")
DEDOS = ("index", "middle", "ring", "pinky", "thumb")


def ossos(arm):
    mw = arm.matrix_world
    return {b.name: (mw @ b.head_local, mw @ b.tail_local, mw @ b.matrix_local) for b in arm.data.bones}


def quadro(nome, o):
    """Base de um osso: y na direção do osso (da mão, até o dedo médio) e x de referência
    (para o lado; nos braços, para a frente; na mão e nos dedos, do mínimo ao indicador)."""
    h, t, _ = o[nome]
    lado = nome[-1]
    d = o[f"middle_01_{lado}"][0] - h if nome.startswith("hand_") else t - h
    if nome.startswith(("hand_",) + DEDOS):
        r = o[f"index_01_{lado}"][0] - o[f"pinky_01_{lado}"][0]
    elif nome.startswith(("clavicle", "upperarm", "lowerarm")):
        r = Vector((0.0, -1.0, 0.0))
    else:
        r = Vector((1.0, 0.0, 0.0))
    y = d.normalized()
    x = (r - r.dot(y) * y).normalized()
    return Matrix((x, y, x.cross(y))).transposed()


def meio(o, a, b):
    return (o[a][0] + o[b][0]) / 2


def matrizes(rig, ual):
    """Para cada osso do MPFB, a matriz que leva a malha da pose dele à do UAL1.

    O tronco vai inteiro (girado em torno do eixo x e escalado) para que o meio do quadril e
    o meio dos ombros caiam nos do UAL1; a cabeça vai reta para a junta da cabeça do UAL1 e o
    pescoço fica no meio-termo. Braços e pernas giram até a direção dos ossos do UAL1 (de
    braços abertos) e os ossos longos esticam até o comprimento deles."""
    m, u = ossos(rig), ossos(ual)
    mq, uq = meio(m, "thigh_l", "thigh_r"), meio(u, "thigh_l", "thigh_r")
    mo, uo = meio(m, "upperarm_l", "upperarm_r"), meio(u, "upperarm_l", "upperarm_r")
    vm, vu = (mo - mq), (uo - uq)
    s = vu.length / vm.length
    teta = math.atan2(vu.y, vu.z) - math.atan2(vm.y, vm.z)

    def rigido(de, para, ang):
        return (Matrix.Translation(para) @ Matrix.Rotation(-ang, 4, "X") @ Matrix.Scale(s, 4)
                @ Matrix.Translation(-de))

    tronco = rigido(mq, uq, teta)
    out = {n: tronco for n in TRONCO_RIGIDO}
    out["neck_01"] = rigido(m["neck_01"][0], u["neck_01"][0], teta / 2)
    out["head"] = rigido(m["head"][0], u["Head"][0], 0.0)

    def calc(nome):
        if nome in out:
            return out[nome]
        base = nome.split("_")[0]
        if base in DEDOS:
            out[nome] = calc(f"hand_{nome[-1]}")
        elif base == "ball":
            out[nome] = calc(f"foot_{nome[-1]}")
        else:
            h, t, R = m[nome]
            uh, ut, _ = u[nome]
            rot = quadro(nome, u) @ quadro(nome, m).transposed() @ R.to_3x3().normalized()
            esc = (s, (ut - uh).length / (t - h).length, s) if nome.startswith(ESTICA) else (s, s, s)
            out[nome] = Matrix.Translation(uh) @ rot.to_4x4() @ Matrix.Diagonal((*esc, 1.0)) @ R.inverted()
            if base == "foot":
                # A junta do pé do UAL1 é mais alta: o pé desce até o chão do MPFB virar o chão.
                chao = out[nome] @ Vector((h.x, h.y, 0.0))
                out[nome] = Matrix.Translation((0.0, 0.0, -chao.z)) @ out[nome]
        return out[nome]

    for b in rig.data.bones:
        calc(b.name)
    return out


def deformar(ob, M):
    """Pele linear: cada vértice vai com a média das matrizes dos seus ossos."""
    mw = ob.matrix_world.copy()
    nomes = {g.index: g.name for g in ob.vertex_groups}
    for v in ob.data.vertices:
        p = mw @ v.co
        acc, tot = Vector((0.0, 0.0, 0.0)), 0.0
        for e in v.groups:
            mat = M.get(nomes[e.group])
            if mat is not None and e.weight > 0.0:
                acc += e.weight * (mat @ p)
                tot += e.weight
        v.co = acc / tot if tot > 1e-6 else M["pelvis"] @ p
    ob.parent = None
    ob.matrix_world = Matrix.Identity(4)
    ob.data.update()


# ---------------------------------------------------------------------------
# Materiais
# ---------------------------------------------------------------------------


def dominante(ob):
    """Osso de maior peso de cada vértice."""
    nomes = {g.index: g.name for g in ob.vertex_groups}
    out = []
    for v in ob.data.vertices:
        melhor, w = "", 0.0
        for e in v.groups:
            n = RENOMEAR.get(nomes[e.group], nomes[e.group])
            if e.weight > w and n in OSSOS_UAL:
                melhor, w = n, e.weight
        out.append(melhor)
    return out


def no_grupo(ob, nome, limite=0.5):
    g = ob.vertex_groups.get(nome)
    if g is None:
        return set()
    return {v.index for v in ob.data.vertices for e in v.groups if e.group == g.index and e.weight >= limite}


def cor_da_textura(ob, caminho):
    """Cor (0..1) da textura no centro de cada face, pelas coordenadas UV."""
    import numpy as np

    img = bpy.data.images.load(caminho)
    w, h = img.size
    px = np.empty(w * h * img.channels, dtype=np.float32)
    img.pixels.foreach_get(px)
    px = px.reshape(h, w, img.channels)
    uv = ob.data.uv_layers.active.data
    out = []
    for f in ob.data.polygons:
        u = sum(uv[i].uv.x for i in f.loop_indices) / f.loop_total
        v = sum(uv[i].uv.y for i in f.loop_indices) / f.loop_total
        x = min(w - 1, max(0, int(u * w)))
        y = min(h - 1, max(0, int(v * h)))
        out.append(tuple(float(c) for c in px[y, x, :3]))
    bpy.data.images.remove(img)
    return out


def pintar(ob, escolher):
    """Troca os materiais do MPFB por cores lisas: `escolher(face)` dá o nome de cada face."""
    nomes = [escolher(f) for f in ob.data.polygons]
    lista = sorted(set(nomes))
    ob.data.materials.clear()
    for n in lista:
        ob.data.materials.append(material(n))
    for f, n in zip(ob.data.polygons, nomes):
        f.material_index = lista.index(n)
    while ob.data.uv_layers:
        ob.data.uv_layers.remove(ob.data.uv_layers[0])


def pintar_partes(partes, ual):
    u = ossos(ual)
    pele = partes["pele"]
    dom = dominante(pele)
    labios = no_grupo(pele, "lips")
    tronco = {"pelvis", "spine_01", "spine_02", "spine_03", "clavicle_l", "clavicle_r"}

    def da_pele(f):
        vs = list(f.vertices)
        if all(i in labios for i in vs):
            return "labio"
        ossos_f = [dom[i] for i in vs]
        if all(o in tronco for o in ossos_f):
            return "camisa"
        # Manga curta: metade do braço, do ombro para fora.
        if all(o in tronco or o.startswith("upperarm") for o in ossos_f):
            lado = "l" if f.center.x > 0 else "r"
            h, t, _ = u[f"upperarm_{lado}"]
            if abs(f.center.x) < abs(h.x) + 0.45 * (t - h).length:
                return "camisa"
        return "pele"

    pintar(pele, da_pele)
    # O macacão traz a camiseta pintada na textura: branco é camiseta, escuro é fivela.
    roupa = partes["roupa"]
    cores = cor_da_textura(roupa, os.path.join(MPFB, "assets", os.path.dirname(ROUPA),
                                               "male_worksuit01_diffuse.png"))

    def da_roupa(f):
        r, g, b = cores[f.index]
        if min(r, g, b) > 0.75:
            return "camisa"
        if r + g + b < 0.45:
            return "fivela"
        return "calca"

    pintar(roupa, da_roupa)
    for papel in ("calcado", "botina"):
        ob = partes[papel]
        cor = "calcado" if papel == "calcado" else "epi_couro"
        pintar(ob, lambda f, c=cor: "sola" if f.center.z < 0.022 and f.normal.z < -0.3 else c)
    pintar(partes["cabelo"], lambda f: "cabelo")
    pintar(partes["sobrancelha"], lambda f: "cabelo")
    # Olhos: a íris é o miolo da frente de cada olho.
    ob = partes["olhos"]
    vs = ob.data.vertices
    frente = {}
    for lado in (1, -1):
        meus = [v.co for v in vs if v.co.x * lado > 0]
        frente[lado] = (min(meus, key=lambda c: c.y), (max(c.x for c in meus) - min(c.x for c in meus)) / 2)

    def do_olho(f):
        p, r = frente[1 if f.center.x > 0 else -1]
        return "olho" if (f.center - p).length < 0.5 * r else "olho_branco"

    pintar(ob, do_olho)


# ---------------------------------------------------------------------------
# Malhas auxiliares (EPIs)
# ---------------------------------------------------------------------------


class Malha:
    """Acumula vértices e faces com material e vira um objeto."""

    def __init__(self, nome):
        self.nome = nome
        self.vs = []
        self.fs = []
        self.mats = []

    def vert(self, p):
        self.vs.append(tuple(p))
        return len(self.vs) - 1

    def face(self, idx, mat):
        if mat not in self.mats:
            self.mats.append(mat)
        self.fs.append((tuple(idx), self.mats.index(mat)))

    def grade(self, linhas, mats, fechar=False):
        """Linhas de pontos de mesmo tamanho ligadas em quadrados; `mats` é um nome ou um
        por faixa entre linhas. `fechar` liga o último ponto de cada linha ao primeiro."""
        ids = [[self.vert(p) for p in ln] for ln in linhas]
        n = len(ids[0])
        for k, (a, b) in enumerate(zip(ids, ids[1:])):
            mat = mats if isinstance(mats, str) else mats[k]
            for i in range(n if fechar else n - 1):
                j = (i + 1) % n
                self.face((a[i], a[j], b[j], b[i]), mat)
        return ids

    def leque(self, anel, centro, mat):
        c = self.vert(centro)
        n = len(anel)
        for i in range(n):
            self.face((anel[i], anel[(i + 1) % n], c), mat)

    def elipsoide(self, c, r, mat, nu=14, nv=7, base=None):
        """Elipsoide de raios r nos eixos de `base` (colunas; o padrão é x, y, z)."""
        base = base or Matrix.Identity(3)
        linhas = []
        for j in range(1, nv):
            t = math.pi * j / nv
            linhas.append([Vector(c) + base @ Vector((r[0] * math.sin(t) * math.cos(2 * math.pi * i / nu),
                                                      r[1] * math.sin(t) * math.sin(2 * math.pi * i / nu),
                                                      r[2] * math.cos(t))) for i in range(nu)])
        ids = self.grade(linhas, mat, fechar=True)
        self.leque(ids[0][::-1], Vector(c) + base @ Vector((0, 0, r[2])), mat)
        self.leque(ids[-1], Vector(c) - base @ Vector((0, 0, r[2])), mat)

    def tubo(self, pontos, raio, mat, n=6):
        """Tubo de seção elíptica (raio = (largura, espessura) ou número) pelos pontos."""
        if isinstance(raio, (int, float)):
            raio = (raio, raio)
        linhas = []
        for k, p in enumerate(pontos):
            a = pontos[max(k - 1, 0)]
            b = pontos[min(k + 1, len(pontos) - 1)]
            d = (Vector(b) - Vector(a)).normalized()
            ref = Vector((0, 0, 1)) if abs(d.z) < 0.9 else Vector((1, 0, 0))
            u = d.cross(ref).normalized()
            w = u.cross(d)
            linhas.append([Vector(p) + u * raio[0] * math.cos(2 * math.pi * i / n)
                           + w * raio[1] * math.sin(2 * math.pi * i / n) for i in range(n)])
        ids = self.grade(linhas, mat, fechar=True)
        self.leque(ids[0][::-1], pontos[0], mat)
        self.leque(ids[-1], pontos[-1], mat)

    def objeto(self):
        me = bpy.data.meshes.new(self.nome)
        me.from_pydata(self.vs, [], [f for f, _ in self.fs])
        for m in self.mats:
            me.materials.append(material(m))
        for poly, (_, mi) in zip(me.polygons, self.fs):
            poly.material_index = mi
        me.validate()
        me.update()
        ob = bpy.data.objects.new(self.nome, me)
        bpy.context.scene.collection.objects.link(ob)
        return ob


class Superficie:
    """Raios de fora para dentro até o corpo vestido: a primeira coisa que bate é a de fora."""

    def __init__(self, ob):
        dg = bpy.context.evaluated_depsgraph_get()
        self.bvh = BVHTree.FromObject(ob, dg)

    def ate(self, alvo, d, folga, longe=0.5, perto=None):
        """Ponto da superfície na direção `d` (unitário) a partir de `alvo`, com folga. Com
        `perto`, o que bate mais longe que isso de `alvo` (o braço aberto ao lado do tronco)
        não conta: o raio segue até a superfície de dentro."""
        d = Vector(d).normalized()
        origem = Vector(alvo) + d * longe
        resta = longe
        while resta > 1e-4:
            hit = self.bvh.ray_cast(origem, -d, resta)
            if hit[0] is None:
                return None
            if perto is None or (hit[0] - Vector(alvo)).dot(d) <= perto:
                return hit[0] + d * folga
            resta -= hit[3] + 1e-4
            origem = hit[0] - d * 1e-4
        return None

    def anel(self, centro, folga, angulos, plano="xy", longe=0.5, perto=None):
        """Pontos em volta de `centro`, um por ângulo (graus; 0 é a frente, -y, e 90 é +x)."""
        out = []
        ultimo = None
        for a in angulos:
            r = math.radians(a)
            if plano == "xy":
                d = Vector((math.sin(r), -math.cos(r), 0.0))
            elif plano == "yz":  # 0 é a frente, 90 é para cima
                d = Vector((0.0, -math.cos(r), math.sin(r)))
            else:  # "xz": 0 é +x, 90 é para cima
                d = Vector((math.cos(r), 0.0, math.sin(r)))
            p = self.ate(centro, d, folga, longe, perto)
            if p is None:
                p = ultimo if ultimo is not None else Vector(centro) + d * 0.1
            out.append(p)
            ultimo = p
        return out


def faixa(a, b, n):
    return [a + (b - a) * i / (n - 1) for i in range(n)]


# ---------------------------------------------------------------------------
# EPIs
# ---------------------------------------------------------------------------


def medidas(corpo, partes, ual):
    """Pontos de referência do corpo vestido, já na pose do UAL1."""
    u = ossos(ual)
    olhos = partes["olhos"]
    pe = [v.co for v in olhos.data.vertices]
    olho = {}
    for lado in (1, -1):
        meus = [c for c in pe if c.x * lado > 0]
        centro = sum(meus, Vector()) / len(meus)
        olho[lado] = (centro, min(c.y for c in meus), (max(c.x for c in meus) - min(c.x for c in meus)) / 2)
    cab = [v.co for ob in (partes["pele"], partes["cabelo"]) for v in ob.data.vertices
           if v.co.z > u["neck_01"][1].z]
    labios = [(partes["pele"].matrix_world @ partes["pele"].data.vertices[i].co)
              for i in no_grupo(partes["pele"], "lips")]
    return {
        "olho": olho,
        "olho_z": (olho[1][0].z + olho[-1][0].z) / 2,
        "topo": max(c.z for c in cab),
        "cab_y": (min(c.y for c in cab) + max(c.y for c in cab)) / 2,
        "boca_z": sum(c.z for c in labios) / len(labios),
        "boca_y": min(c.y for c in labios),
        "u": u,
    }


def epis(corpo, partes, ual):
    """As peças de EPI, cada uma num objeto `epi_<nome>`, sobre o corpo vestido."""
    s = Superficie(corpo)
    md = medidas(corpo, partes, ual)
    u = md["u"]
    oz, cy = md["olho_z"], md["cab_y"]
    out = {}

    # Capacete: casca sobre o cabelo, mais curta na testa, com aba e bico na frente.
    m = Malha("epi_capacete")
    c = Vector((0.0, cy, oz + 0.03))
    na = 36
    angs = [360.0 * i / na for i in range(na)]
    topo = s.ate(c, (0, 0, 1), 0.016)
    linhas = []
    for k in range(1, 9):
        linha = []
        for a in angs:
            lim = 76.0 - 18.0 * math.cos(math.radians(a))
            fi = math.radians(lim * k / 8)
            r = math.radians(a)
            d = Vector((math.sin(fi) * math.sin(r), -math.sin(fi) * math.cos(r), math.cos(fi)))
            linha.append(s.ate(c, d, 0.016) or c + d * 0.12)
        linhas.append(linha)
    aba = []
    for p, a in zip(linhas[-1], angs):
        h = Vector((p.x - c.x, p.y - c.y, 0.0)).normalized()
        bico = max(0.0, math.cos(math.radians(a))) ** 3
        aba.append(p + h * (0.018 + 0.045 * bico) + Vector((0, 0, -0.006 - 0.004 * bico)))
    ids = m.grade(linhas + [aba], "epi_amarelo", fechar=True)
    m.leque(ids[0][::-1], topo, "epi_amarelo")
    out["capacete"] = m.objeto()

    # Óculos: duas lentes claras na frente dos olhos, ponte e hastes até as orelhas.
    m = Malha("epi_oculos")
    lentes = {}
    for lado in (1, -1):
        centro, yf, r = md["olho"][lado]
        lc = Vector((centro.x, yf - 0.013, centro.z + 0.002))
        lentes[lado] = (lc, r)
        m.elipsoide(lc, (r * 1.75, 0.0025, r * 1.25), "epi_lente", 16, 4)
        haste = [lc + Vector((lado * r * 1.75, 0.0, 0.0))]
        for alvo, d, f in ((Vector((0, cy, lc.z)), (lado, -0.25, 0), 0.008),
                           (Vector((0, cy + 0.03, lc.z - 0.01)), (lado, 0.15, 0), 0.006)):
            p = s.ate(alvo, d, f)
            if p is not None:
                haste.append(p)
        m.tubo(haste, 0.0022, "epi_cinza", 4)
    a, b = lentes[1], lentes[-1]
    m.tubo([a[0] - Vector((a[1] * 1.75, 0, -0.004)), b[0] + Vector((b[1] * 1.75, 0, 0.004))], 0.0025,
           "epi_cinza", 4)
    out["oculos"] = m.objeto()

    # Protetor auricular: conchas nas orelhas e arco por cima da cabeça.
    m = Malha("epi_protetor_auricular")
    ce = Vector((0.0, cy + 0.012, oz - 0.025))
    conchas = {}
    for lado in (1, -1):
        p = s.ate(ce, (lado, 0, 0), 0.0) or ce + Vector((lado * 0.08, 0, 0))
        conchas[lado] = p + Vector((lado * 0.016, 0, 0))
        m.elipsoide(conchas[lado], (0.017, 0.031, 0.038), "epi_vermelho", 12, 6)
    arco = s.anel(Vector((0.0, ce.y, ce.z)), 0.012, [10 + 160 * i / 12 for i in range(13)], plano="xz",
                  longe=0.3, perto=0.16)
    arco = ([conchas[1] + Vector((0, 0, 0.03))] + [p for p in arco if p.z > ce.z + 0.03]
            + [conchas[-1] + Vector((0, 0, 0.03))])
    m.tubo(arco, (0.009, 0.004), "epi_cinza", 6)
    out["protetor_auricular"] = m.objeto()

    # Máscara: concha só sobre o nariz e a boca, com dois elásticos por trás.
    m = Malha("epi_mascara")
    eixo_y = cy + 0.01
    zs = faixa(md["boca_z"] - 0.042, oz - 0.034, 7)
    linhas = [s.anel(Vector((0.0, eixo_y, z)), 0.007, [-62 + 124 * i / 12 for i in range(13)], longe=0.3,
                     perto=0.14) for z in zs]
    # A concha não encosta no nariz nem na boca: no meio, cada coluna fica à frente do ponto
    # mais saliente dela; nas bordas, volta a encostar no rosto.
    for j in range(13):
        frente = min(ln[j].y for ln in linhas)
        for k, ln in enumerate(linhas):
            meio_k = 1.0 - abs(2.0 * k / (len(linhas) - 1) - 1.0)
            ln[j].y = min(ln[j].y, frente - 0.006 + 0.016 * (1.0 - meio_k) ** 2)
    m.grade(linhas, "epi_branco")
    for z in (zs[1], zs[-2]):
        angs = [62 + 236 * i / 16 for i in range(17)]
        a1 = s.anel(Vector((0.0, eixo_y, z)), 0.004, angs, longe=0.3, perto=0.12)
        a2 = s.anel(Vector((0.0, eixo_y, z + 0.006)), 0.004, angs, longe=0.3, perto=0.12)
        m.grade([a1, a2], "epi_branco")
    out["mascara"] = m.objeto()

    # Luvas: a mão com folga e o punho largo.
    m = Malha("epi_luvas")
    pele = partes["pele"]
    dom = dominante(pele)
    mao = {i for i, o in enumerate(dom) if o.startswith(("hand_",) + DEDOS)}
    novos = {}
    for v in pele.data.vertices:
        if v.index in mao:
            novos[v.index] = m.vert(v.co + v.normal * 0.0035)
    for f in pele.data.polygons:
        if all(i in mao for i in f.vertices):
            m.face([novos[i] for i in f.vertices], "epi_laranja")
    for lado in (1, -1):
        hx = u[f"hand_{'l' if lado > 0 else 'r'}"][0]
        linhas = []
        for k, dx in enumerate((-0.075, -0.045, -0.015, 0.012)):
            cen = Vector((hx.x + lado * dx, hx.y, hx.z))
            angs = [360.0 * i / 16 for i in range(16)]
            pts = []
            for a in angs:
                r = math.radians(a)
                d = Vector((0.0, -math.cos(r), math.sin(r)))
                pts.append(s.ate(cen, d, 0.006 + 0.006 * (k == 0), longe=0.2) or cen + d * 0.04)
            linhas.append(pts)
        m.grade(linhas, "epi_laranja", fechar=True)
    out["luvas"] = m.objeto()

    # Botina: a bota do pacote (cano curto), um pouco por fora do sapato.
    out["botina"] = partes["botina"]

    # Colete: veste o tronco da cintura às axilas, com alças nos ombros e duas faixas.
    m = Malha("epi_colete")
    z0 = u["spine_01"][0].z
    z1 = u["upperarm_l"][0].z - 0.075
    zs = faixa(z0, z1, 11)
    angs = [360.0 * i / 40 for i in range(40)]
    linhas = [s.anel(Vector((0.0, 0.0, z)), 0.013, angs, longe=0.6, perto=0.24) for z in zs]
    mats = ["faixa" if k in (2, 7) else "epi_laranja" for k in range(len(zs) - 1)]
    m.grade(linhas, mats, fechar=True)
    for lado in (1, -1):
        alca = []
        for x in (0.075, 0.12):
            cen = Vector((lado * x, 0.0, z1 - 0.04))
            alca.append(s.anel(cen, 0.012, [15 + 150 * i / 12 for i in range(13)], plano="yz", longe=0.3,
                               perto=0.2))
        m.grade(alca, "epi_laranja")
    out["colete"] = m.objeto()

    # Avental: peitilho e saia que caem da frente do corpo, com alça no pescoço e tira nas costas.
    m = Malha("epi_avental")
    fo = 0.02
    zc = u["thigh_l"][0].z - 0.06
    zs = faixa(0.52, 1.40, 17)
    linhas = []
    for z in zs:
        larg = 0.20 if z < 1.13 else 0.13
        xs = faixa(-larg, larg, 13)
        ys = []
        for x in xs:
            p = s.ate(Vector((x, 0.0, z)), (0, -1, 0), 0.0, longe=0.6)
            ys.append(p.y if p is not None else None)
        frente = min(y for y in ys if y is not None)
        linha = []
        for x, y in zip(xs, ys):
            if z < zc or y is None:
                y = frente  # abaixo da virilha o avental cai reto, sem entrar entre as pernas
            linha.append(Vector((x, y - fo, z)))
        linhas.append(linha)
    ids = m.grade(linhas, "epi_cinza")
    pescoco = u["neck_01"][0]
    gola = s.anel(Vector((0.0, pescoco.y, pescoco.z + 0.02)), 0.012, [50 + 260 * i / 13 for i in range(14)],
                  longe=0.3, perto=0.1)
    m.tubo([linhas[-1][-1], gola[0] + Vector((0, 0, 0.01))] + gola + [gola[-1] + Vector((0, 0, 0.01)), linhas[-1][0]],
           (0.010, 0.003), "epi_cinza", 4)
    zt = 1.085
    t1 = s.anel(Vector((0.0, 0.0, zt)), 0.016, [70 + 220 * i / 16 for i in range(17)], longe=0.6, perto=0.24)
    t2 = s.anel(Vector((0.0, 0.0, zt + 0.02)), 0.016, [70 + 220 * i / 16 for i in range(17)], longe=0.6,
                perto=0.24)
    m.grade([t1, t2], "epi_cinza")
    out["avental"] = m.objeto()

    for nome, ob in out.items():
        ob.name = f"epi_{nome}"
        ob.data.name = f"epi_{nome}"
    return out


# ---------------------------------------------------------------------------
# Esqueleto e pesos
# ---------------------------------------------------------------------------

OSSOS_UAL = set()


def esqueleto():
    """Importa o UAL1 e deixa só o esqueleto (sem animações)."""
    bpy.ops.import_scene.gltf(filepath=UAL1)
    arm = bpy.data.objects["Armature"]
    arm.name = "ual"
    for o in list(bpy.data.objects):
        if o.parent == arm or (o.type == "MESH" and o.find_armature() == arm):
            bpy.data.objects.remove(o)
    if arm.animation_data:
        arm.animation_data.action = None
        for tr in list(arm.animation_data.nla_tracks):
            arm.animation_data.nla_tracks.remove(tr)
    for a in list(bpy.data.actions):
        bpy.data.actions.remove(a)
    OSSOS_UAL.clear()
    OSSOS_UAL.update(b.name for b in arm.data.bones)
    return arm


def limpar_pesos(ob, reserva="pelvis"):
    """Grupos com os nomes do UAL1, só ossos, no máximo 4 por vértice, somando 1."""
    for g in list(ob.vertex_groups):
        novo = RENOMEAR.get(g.name, g.name)
        if novo not in OSSOS_UAL:
            ob.vertex_groups.remove(g)
        else:
            g.name = novo
    if reserva not in ob.vertex_groups:
        ob.vertex_groups.new(name=reserva)
    grupos = {g.index: g for g in ob.vertex_groups}
    res = ob.vertex_groups[reserva]
    for v in ob.data.vertices:
        pesos = sorted(((e.weight, e.group) for e in v.groups if e.weight > 1e-4), reverse=True)
        for e in list(v.groups):
            grupos[e.group].remove([v.index])
        pesos = pesos[:4]
        tot = sum(w for w, _ in pesos)
        if tot <= 1e-6:
            res.add([v.index], 1.0, "REPLACE")
            continue
        for w, gi in pesos:
            grupos[gi].add([v.index], w / tot, "REPLACE")


def transferir(ob, fonte, pode):
    """Copia os pesos de `fonte` pelo ponto mais próximo, só dos ossos que `pode` aceita."""
    mod = ob.modifiers.new("pesos", "DATA_TRANSFER")
    mod.object = fonte
    mod.use_vert_data = True
    mod.data_types_verts = {"VGROUP_WEIGHTS"}
    mod.vert_mapping = "POLYINTERP_NEAREST"
    mod.layers_vgroup_select_src = "ALL"
    mod.layers_vgroup_select_dst = "NAME"
    bpy.context.view_layer.objects.active = ob
    with bpy.context.temp_override(object=ob, active_object=ob):
        bpy.ops.object.datalayout_transfer(modifier=mod.name)
        bpy.ops.object.modifier_apply(modifier=mod.name)
    for g in list(ob.vertex_groups):
        if not pode(g.name):
            ob.vertex_groups.remove(g)
    limpar_pesos(ob, reserva=next((g.name for g in ob.vertex_groups), "pelvis"))


def no_osso(ob, osso):
    """Peça rígida presa a um osso só."""
    for g in list(ob.vertex_groups):
        ob.vertex_groups.remove(g)
    g = ob.vertex_groups.new(name=osso)
    g.add(list(range(len(ob.data.vertices))), 1.0, "REPLACE")


def prender(ob, arm):
    ob.parent = arm
    a = ob.modifiers.new("esqueleto", "ARMATURE")
    a.object = arm


def juntar(obs, nome):
    bpy.ops.object.select_all(action="DESELECT")
    for o in obs:
        o.select_set(True)
    bpy.context.view_layer.objects.active = obs[0]
    bpy.ops.object.join()
    ob = bpy.context.view_layer.objects.active
    ob.name = nome
    ob.data.name = nome
    return ob


TRONCO = ("pelvis", "spine", "neck", "clavicle", "thigh")


def familia(nome):
    if nome == "tronco":
        return lambda o: o.startswith(TRONCO)
    if nome == "bracos":
        return lambda o: o.startswith(("upperarm", "lowerarm", "hand") + DEDOS)
    return lambda o: o.startswith(("thigh", "calf", "foot", "ball"))


# ---------------------------------------------------------------------------
# Principal
# ---------------------------------------------------------------------------


# Variações que o 3D sorteia por pessoa (tons de pele e de cabelo, sRGB) e a faixa de altura
# (m) de cada sexo; `altura_base` é a altura do arquivo, do chão ao topo da cabeça.
PELES = ["#f1c7a5", "#e0ac84", "#c68e6b", "#a86f4c", "#8a5638", "#5f3a25"]
CABELOS = ["#1f1a17", "#3b2a20", "#6b4a2b", "#a07040", "#c9a36a", "#8c8c8c"]
ALTURAS = {"homem": [1.65, 1.90], "mulher": [1.55, 1.78]}
# IMC (kg/m²) estimado do corpo gerado (peso médio do MakeHuman, um pouco acima no homem) e a
# faixa do fator de largura: com o peso da pessoa, o 3D alarga ou afina o corpo por
# sqrt(IMC / imc_base), dentro da faixa.
IMC_BASE = {"homem": 23.5, "mulher": 22.0}
LARGURA = [0.85, 1.25]
EPIS = ["avental", "botina", "capacete", "colete", "luvas", "mascara", "oculos", "protetor_auricular"]


# Fração dos triângulos que fica em cada parte (o MakeHuman é denso demais para o 3D em tempo
# real, com várias pessoas na cena); a redução é simétrica, igual dos dois lados.
ENXUGAR = {"pele": 0.35, "roupa": 0.45, "calcado": 0.35, "botina": 0.3, "cabelo": 0.5}


def enxugar(ob, fracao):
    mod = ob.modifiers.new("enxugar", "DECIMATE")
    mod.ratio = fracao
    mod.use_symmetry = True
    mod.symmetry_axis = "X"
    with bpy.context.temp_override(object=ob, active_object=ob):
        bpy.ops.object.modifier_apply(modifier=mod.name)


def gerar(sexo, saida):
    """Gera `pessoa_<sexo>.glb` e devolve o item do catálogo."""
    bpy.ops.wm.read_factory_settings(use_empty=True)
    rig, partes = humano(sexo)
    ual = esqueleto()
    M = matrizes(rig, ual)
    for ob in partes.values():
        deformar(ob, M)
    bpy.data.objects.remove(rig)
    pintar_partes(partes, ual)
    for papel, fracao in ENXUGAR.items():
        enxugar(partes[papel], fracao)
    pecas = epis_e_corpo(partes, ual)
    for ob in list(bpy.data.objects):
        # O que sobrou do MPFB (ajudantes escondidos) não vai para o arquivo.
        if ob.type != "ARMATURE" and ob not in pecas:
            bpy.data.objects.remove(ob)
    for ob in pecas:
        prender(ob, ual)
    for ob in bpy.data.objects:
        if ob.type == "MESH":
            # Triângulos fixos aqui, como em `comum.Elemento`.
            bm = bmesh.new()
            bm.from_mesh(ob.data)
            bmesh.ops.triangulate(bm, faces=bm.faces[:], quad_method="FIXED", ngon_method="EAR_CLIP")
            bm.verts.index_update()
            bm.faces.index_update()
            chave = {f.index: (f.material_index, tuple(sorted(v.index for v in f.verts))) for f in bm.faces}
            ordem = {i: n for n, i in enumerate(sorted(chave, key=chave.get))}
            bm.faces.sort(key=lambda f: ordem[f.index])
            bm.to_mesh(ob.data)
            bm.free()
    nome = f"pessoa_{sexo}"
    # A raiz tem o nome do elemento, como nos outros arquivos.
    ual.name = nome
    arq = os.path.join(saida, nome + ".glb")
    bpy.ops.object.select_all(action="DESELECT")
    bpy.ops.export_scene.gltf(
        filepath=arq,
        export_format="GLB",
        export_yup=True,
        export_texcoords=False,
        export_tangents=False,
        export_attributes=False,
        export_extras=False,
        export_cameras=False,
        export_lights=False,
        export_animations=False,
        export_morph=False,
        export_apply=False,
    )
    corpo = bpy.data.objects["corpo"]
    pts = [corpo.matrix_world @ v.co for v in corpo.data.vertices]
    lo = [min(p[i] for p in pts) for i in range(3)]
    hi = [max(p[i] for p in pts) for i in range(3)]
    # Medidas no sistema do UTSim (+x frente, +y esquerda): a pessoa olha para -y do Blender.
    tamanho = [hi[1] - lo[1], hi[0] - lo[0], hi[2] - lo[2]]
    minimo = [-hi[1], lo[0], lo[2]]
    malhas = sorted(o.name for o in bpy.data.objects if o.type == "MESH")
    tri = sum(len(p.vertices) - 2 for o in bpy.data.objects if o.type == "MESH" for p in o.data.polygons)
    return {
        "arquivo": nome + ".glb",
        "tamanho": [round(v, 3) for v in tamanho],
        "minimo": [round(v, 3) for v in minimo],
        "triangulos": tri,
        "nos": malhas,
        "pontos": {},
        "clipes": {},
        "estados": {},
        "peca": {},
        "dados": {
            "altura_base": round(hi[2] - lo[2], 2),
            "alturas": ALTURAS[sexo],
            "imc_base": IMC_BASE[sexo],
            "largura": LARGURA,
            "cabelos": CABELOS,
            "epis": EPIS,
            "esqueleto": "UAL1",
            "peles": PELES,
            "recolorir": ["cabelo", "calca", "camisa", "pele"],
        },
    }


def epis_e_corpo(partes, ual):
    """Junta o corpo vestido, faz os EPIs sobre ele e acerta os pesos de todos."""
    corpo_partes = [partes[p] for p in ("pele", "roupa", "calcado", "cabelo", "sobrancelha", "olhos")]
    # Os EPIs medem o corpo antes da junção (olhos, lábios); a superfície é o corpo vestido.
    vestido = juntar([_copia(o) for o in corpo_partes], "vestido")
    pecas = epis(vestido, partes, ual)
    bpy.data.objects.remove(vestido)
    botina = pecas["botina"]
    for v in botina.data.vertices:
        v.co += v.normal * 0.004
    for ob in corpo_partes + [botina]:
        limpar_pesos(ob)
    corpo = juntar(corpo_partes, "corpo")
    botina.name = botina.data.name = "epi_botina"
    for nome in ("capacete", "oculos", "protetor_auricular", "mascara"):
        no_osso(pecas[nome], "Head")
    transferir(pecas["luvas"], corpo, familia("bracos"))
    transferir(pecas["colete"], corpo, familia("tronco"))
    transferir(pecas["avental"], corpo, familia("tronco"))
    return [corpo] + list(pecas.values())


def _copia(ob):
    c = ob.copy()
    c.data = ob.data.copy()
    bpy.context.scene.collection.objects.link(c)
    return c


def main():
    argv = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    p = argparse.ArgumentParser()
    p.add_argument("--saida", default=os.path.join(RAIZ, "assets", "modelos3d"))
    p.add_argument("--so", nargs="*", default=["homem", "mulher"])
    a = p.parse_args(argv)
    os.makedirs(a.saida, exist_ok=True)
    for sexo in a.so:
        item = gerar(sexo, a.saida)
        print(f"utsim-blender: {item['arquivo']} {item['triangulos']} triângulos")


if __name__ == "__main__":
    main()

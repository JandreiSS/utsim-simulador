"""Pessoas de verdade (homem e mulher) para o 3D, no esqueleto do boneco livre UAL1.

    blender -b --factory-startup --python-exit-code 1 -P tools/blender/pessoas.py -- \
        [--saida assets/modelos3d] [--so homem]

Cada arquivo `pessoa_<homem|mulher>.glb` traz o esqueleto do UAL1 (mesmos ossos, mesma pose
de repouso, sem animações: o 3D usa as do UAL1 e do UAL2) e:

- `corpo`: uma malha lisa, feita de metaball por família (tronco, cada braço, cada perna),
  com rosto simples (olhos, sobrancelhas, nariz, orelhas, boca) e cabelo. A cor vem só dos
  materiais `pele`, `cabelo`, `camisa`, `calca`, `calcado`, `olho` e `labio`, sem textura:
  o 3D recolore `pele`, `cabelo`, `camisa` e `calca` para variar as pessoas;
- `epi_<nome>`: uma malha por EPI de `PPE_NAMES` (capacete, óculos, protetor auricular,
  luvas, botina, colete, máscara, avental), escondida ou mostrada conforme o EPI que a pessoa
  veste no modelo.

Os pesos de pele vêm do boneco UAL1 (transferência pelo ponto mais próximo), então todas as
animações dele servem. Metros, Z para cima; a pessoa olha para -y do Blender, como o UAL1.
"""

import argparse
import math
import os
import sys

import bmesh
import bpy
from mathutils import Matrix, Vector

AQUI = os.path.dirname(os.path.abspath(__file__))
RAIZ = os.path.normpath(os.path.join(AQUI, "..", ".."))
UAL1 = os.path.join(RAIZ, "assets", "terceiros", "quaternius_ual", "UAL1_Standard.glb")

# Raio da superfície de uma bola de metaball / raio do elemento (rigidez 2, limiar 0,6).
SUPERFICIE = 0.672

# Cores (sRGB, 0..255), as mesmas convenções de `comum.PALETA`.
CORES = {
    "pele": (0xC6, 0x8E, 0x6B),
    "cabelo": (0x3B, 0x2A, 0x20),
    "camisa": (0x2F, 0x5D, 0x8C),
    "calca": (0x2B, 0x33, 0x40),
    "calcado": (0x26, 0x22, 0x20),
    "olho": (0x1C, 0x1C, 0x22),
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
    b.inputs["Roughness"].default_value = 0.35 if nome in ("olho", "epi_lente") else 0.8
    return m


# ---------------------------------------------------------------------------
# Metaball
# ---------------------------------------------------------------------------


class Massa:
    """Uma família de metaball (os elementos de uma família se fundem; famílias diferentes
    não). Medidas em raio de superfície (metros)."""

    def __init__(self, nome, resolucao=0.011):
        mb = bpy.data.metaballs.new(nome)
        mb.resolution = resolucao
        mb.render_resolution = resolucao
        mb.threshold = 0.6
        self.ob = bpy.data.objects.new(nome, mb)
        bpy.context.scene.collection.objects.link(self.ob)
        self.mb = mb

    def bola(self, c, rx, ry=None, rz=None, rigidez=2.0):
        ry = rx if ry is None else ry
        rz = rx if rz is None else rz
        e = self.mb.elements.new(type="ELLIPSOID")
        e.co = c
        e.radius = 1.0 / SUPERFICIE
        e.size_x, e.size_y, e.size_z = rx, ry, rz
        e.stiffness = rigidez
        return e

    def capsula(self, a, b, r, rigidez=2.0):
        a, b = Vector(a), Vector(b)
        d = b - a
        e = self.mb.elements.new(type="CAPSULE")
        e.co = (a + b) / 2
        e.radius = r / SUPERFICIE
        e.size_x = d.length / 2
        e.rotation = d.to_track_quat("X", "Z")
        e.stiffness = rigidez
        return e

    def malha(self, nome):
        """Converte a família em malha (um objeto novo) e apaga o metaball."""
        dg = bpy.context.evaluated_depsgraph_get()
        me = bpy.data.meshes.new_from_object(self.ob.evaluated_get(dg))
        me.name = nome
        ob = bpy.data.objects.new(nome, me)
        bpy.context.scene.collection.objects.link(ob)
        bpy.data.objects.remove(self.ob)
        return ob


# ---------------------------------------------------------------------------
# Corpo
# ---------------------------------------------------------------------------

# Medidas que mudam entre homem e mulher (metros).
MEDIDAS = {
    "homem": {
        "ombro": 0.185, "peito": (0.165, 0.105, 0.16), "cintura": (0.145, 0.095, 0.12),
        "quadril": (0.155, 0.105, 0.11), "braco": 0.058, "antebraco": 0.047,
        "coxa": 0.084, "panturrilha": 0.060, "pescoco": 0.058,
        "cabeca": (0.088, 0.104, 0.118), "seios": 0.0, "mao": (0.085, 0.046, 0.024),
    },
    "mulher": {
        "ombro": 0.160, "peito": (0.145, 0.095, 0.15), "cintura": (0.125, 0.085, 0.11),
        "quadril": (0.170, 0.11, 0.115), "braco": 0.050, "antebraco": 0.040,
        "coxa": 0.086, "panturrilha": 0.056, "pescoco": 0.050,
        "cabeca": (0.084, 0.100, 0.114), "seios": 0.058, "mao": (0.078, 0.042, 0.021),
    },
}


def tronco(m):
    t = Massa("tronco")
    cx, cy, cz = m["cabeca"]
    t.bola((0, 0.0, 1.705), cx, cy, cz)
    # Queixo e mandíbula: o rosto afina para baixo.
    t.bola((0, -0.022, 1.655), cx * 0.72, cy * 0.68, cz * 0.45)
    t.capsula((0, 0.014, 1.52), (0, 0.004, 1.61), m["pescoco"])
    px, py, pz = m["peito"]
    t.bola((0, 0.012, 1.34), px, py, pz)
    t.capsula((-m["ombro"] + 0.07, 0.03, 1.425), (m["ombro"] - 0.07, 0.03, 1.425), 0.062)
    wx, wy, wz = m["cintura"]
    t.bola((0, 0.008, 1.15), wx, wy, wz)
    hx, hy, hz = m["quadril"]
    t.bola((0, 0.012, 0.97), hx, hy, hz)
    if m["seios"]:
        for s in (-1, 1):
            t.bola((s * 0.062, -0.06, 1.32), m["seios"], m["seios"] * 0.9, m["seios"] * 0.85)
    return t


def braco(m, s):
    b = Massa(f"braco_{'l' if s > 0 else 'r'}")
    x0 = s * (m["ombro"] - 0.035)
    b.capsula((x0, 0.05, 1.435), (s * 0.465, 0.07, 1.44), m["braco"])
    b.capsula((s * 0.465, 0.07, 1.44), (s * 0.735, 0.066, 1.44), m["antebraco"])
    mx, my, mz = m["mao"]
    b.bola((s * (0.745 + mx * 0.95), 0.062, 1.44), mx, my, mz)
    b.capsula((s * 0.77, 0.03, 1.42), (s * 0.86, -0.012, 1.375), 0.016)
    return b


def perna(m, s):
    p = Massa(f"perna_{'l' if s > 0 else 'r'}")
    x = s * 0.09
    p.capsula((x, 0.008, 0.95), (x, 0.004, 0.53), m["coxa"])
    p.capsula((x, 0.004, 0.53), (x, 0.03, 0.12), m["panturrilha"])
    p.capsula((x, 0.035, 0.05), (x, -0.165, 0.042), 0.046)
    return p


def cabelo(m, sexo):
    c = Massa("cabelo")
    cx, cy, cz = m["cabeca"]
    c.bola((0, 0.012, 1.735), cx + 0.008, cy + 0.008, cz * 0.82)
    if sexo == "mulher":
        c.bola((0, 0.045, 1.68), cx + 0.01, cy - 0.01, cz * 0.95)
        c.capsula((0, 0.11, 1.70), (0, 0.13, 1.56), 0.04)
        c.bola((0, 0.13, 1.54), 0.045, 0.04, 0.05)
    return c


def rosto(m, partes):
    """Olhos, sobrancelhas, nariz, orelhas e boca (malhas pequenas)."""
    cx, cy, cz = m["cabeca"]
    frente = -cy
    for s in (-1, 1):
        partes.append(esfera(f"olho_{s}", (s * 0.033, frente + 0.016, 1.712), (0.011, 0.005, 0.010), "olho"))
        partes.append(caixa(f"sobrancelha_{s}", (s * 0.034, frente + 0.012, 1.735), (0.018, 0.004, 0.0045), "cabelo"))
        partes.append(esfera(f"orelha_{s}", (s * (cx + 0.002), 0.006, 1.70), (0.010, 0.022, 0.028), "pele"))
    partes.append(esfera("nariz", (0, frente + 0.004, 1.69), (0.012, 0.012, 0.02), "pele"))
    partes.append(esfera("boca", (0, frente + 0.018, 1.655), (0.019, 0.005, 0.005), "labio"))


def esfera(nome, c, r, mat, seg=10, aneis=7):
    me = bpy.data.meshes.new(nome)
    bm = bmesh.new()
    bmesh.ops.create_uvsphere(bm, u_segments=seg, v_segments=aneis, radius=1.0)
    bmesh.ops.transform(bm, matrix=Matrix.Diagonal((*r, 1.0)), verts=bm.verts)
    bmesh.ops.translate(bm, vec=Vector(c), verts=bm.verts)
    bm.to_mesh(me)
    bm.free()
    me.materials.append(material(mat))
    ob = bpy.data.objects.new(nome, me)
    bpy.context.scene.collection.objects.link(ob)
    return ob


def caixa(nome, c, r, mat):
    me = bpy.data.meshes.new(nome)
    bm = bmesh.new()
    bmesh.ops.create_cube(bm, size=2.0)
    bmesh.ops.transform(bm, matrix=Matrix.Diagonal((*r, 1.0)), verts=bm.verts)
    bmesh.ops.translate(bm, vec=Vector(c), verts=bm.verts)
    bm.to_mesh(me)
    bm.free()
    me.materials.append(material(mat))
    ob = bpy.data.objects.new(nome, me)
    bpy.context.scene.collection.objects.link(ob)
    return ob


def regiao_corpo(c, familia):
    """Material de uma face do corpo pela posição do centro."""
    x, y, z = c
    if familia.startswith("perna"):
        return "calcado" if z < 0.115 else "calca"
    if familia.startswith("braco"):
        # Manga curta até o meio do braço; antebraço e mão de pele.
        return "camisa" if abs(x) < 0.36 else "pele"
    if z > 1.515:
        return "pele"
    return "camisa" if z > 1.03 else "calca"


def reduzir(ob, proporcao):
    if proporcao >= 1.0:
        return
    mod = ob.modifiers.new("reduzir", "DECIMATE")
    mod.ratio = proporcao
    bpy.context.view_layer.objects.active = ob
    with bpy.context.temp_override(object=ob, active_object=ob):
        bpy.ops.object.modifier_apply(modifier=mod.name)


# Planos (ponto, normal) onde a cor muda, por família: a costura fica reta.
COSTURAS = {
    "tronco": [((0, 0, 1.03), (0, 0, 1)), ((0, 0, 1.515), (0, 0, 1))],
    "braco_l": [((0.36, 0, 0), (1, 0, 0))],
    "braco_r": [((-0.36, 0, 0), (1, 0, 0))],
    "perna_l": [((0, 0, 0.115), (0, 0, 1))],
    "perna_r": [((0, 0, 0.115), (0, 0, 1))],
}


def costurar(ob, familia):
    bm = bmesh.new()
    bm.from_mesh(ob.data)
    for co, no in COSTURAS.get(familia, []):
        geom = bm.verts[:] + bm.edges[:] + bm.faces[:]
        bmesh.ops.bisect_plane(bm, geom=geom, plane_co=co, plane_no=no, dist=1e-5)
    bmesh.ops.triangulate(bm, faces=bm.faces[:], quad_method="FIXED", ngon_method="EAR_CLIP")
    bm.to_mesh(ob.data)
    bm.free()


def pintar(ob, familia):
    costurar(ob, familia)
    mats = ["pele", "cabelo", "camisa", "calca", "calcado", "olho", "labio"]
    for n in mats:
        ob.data.materials.append(material(n))
    for p in ob.data.polygons:
        p.material_index = mats.index(regiao_corpo(p.center, familia))


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


def ossos_da(familia):
    """Ossos que podem mover cada família: o tronco não segue o braço, o braço não segue a
    coluna (o boneco tem esferas nas juntas, e o ponto mais próximo erraria o osso)."""
    lado = familia[-2:]
    if familia == "tronco":
        return lambda o: o.startswith(("pelvis", "spine", "neck", "Head", "clavicle", "thigh"))
    if familia.startswith("braco"):
        return lambda o: o.endswith(lado) and not o.startswith(("thigh", "calf", "foot", "ball"))
    return lambda o: o == "pelvis" or (o.endswith(lado) and o.startswith(("thigh", "calf", "foot", "ball")))


def corpo(sexo, boneco):
    m = MEDIDAS[sexo]
    partes = []
    familias = [("tronco", tronco(m))]
    for s in (1, -1):
        familias.append((f"braco_{'l' if s > 0 else 'r'}", braco(m, s)))
        familias.append((f"perna_{'l' if s > 0 else 'r'}", perna(m, s)))
    for nome, massa in familias:
        ob = massa.malha(nome)
        reduzir(ob, 0.045)
        pintar(ob, nome)
        transferir(ob, boneco, ossos_da(nome))
        if nome.startswith("braco"):
            ombro(ob, nome[-1])
        partes.append(ob)
    c = cabelo(m, sexo).malha("cabelo")
    reduzir(c, 0.12)
    c.data.materials.append(material("cabelo"))
    cabeca = [c]
    rosto(m, cabeca)
    for o in cabeca:
        no_osso(o, "Head")
    return juntar(partes + cabeca, "corpo")


# ---------------------------------------------------------------------------
# EPIs
# ---------------------------------------------------------------------------


def tubo(nome, centro, raio, r_tubo, a0, a1, mat, n=12, lados=6):
    """Arco de tubo no plano xz (em volta do eixo y), de `a0` a `a1` graus (0 = +x)."""
    me = bpy.data.meshes.new(nome)
    bm = bmesh.new()
    aneis = []
    for k in range(n + 1):
        a = math.radians(a0 + (a1 - a0) * k / n)
        radial = Vector((math.cos(a), 0.0, math.sin(a)))
        c = Vector(centro) + raio * radial
        anel = []
        for j in range(lados):
            b = 2 * math.pi * j / lados
            d = math.cos(b) * radial + math.sin(b) * Vector((0.0, 1.0, 0.0))
            anel.append(bm.verts.new(c + r_tubo * d))
        aneis.append(anel)
    for k in range(n):
        for j in range(lados):
            j2 = (j + 1) % lados
            bm.faces.new((aneis[k][j], aneis[k][j2], aneis[k + 1][j2], aneis[k + 1][j]))
    bm.to_mesh(me)
    bm.free()
    me.materials.append(material(mat))
    ob = bpy.data.objects.new(nome, me)
    bpy.context.scene.collection.objects.link(ob)
    return ob


def cortar(ob, co, no):
    """Corta a malha no plano e apaga o lado para onde a normal não aponta."""
    bm = bmesh.new()
    bm.from_mesh(ob.data)
    geom = bm.verts[:] + bm.edges[:] + bm.faces[:]
    bmesh.ops.bisect_plane(bm, geom=geom, plane_co=co, plane_no=no, dist=1e-5, clear_inner=True)
    bm.to_mesh(ob.data)
    bm.free()


def faixas(ob, alturas):
    """Corta a malha em planos horizontais (para pintar faixas retas)."""
    bm = bmesh.new()
    bm.from_mesh(ob.data)
    for z in alturas:
        geom = bm.verts[:] + bm.edges[:] + bm.faces[:]
        bmesh.ops.bisect_plane(bm, geom=geom, plane_co=(0, 0, z), plane_no=(0, 0, 1), dist=1e-5)
    bm.to_mesh(ob.data)
    bm.free()


def epis(m, boneco):
    """As peças de EPI, cada uma num objeto `epi_<nome>` com os pesos prontos."""
    cx, cy, cz = m["cabeca"]
    topo = 1.705
    frente = -cy
    out = {}

    # Capacete: casco (meia elipse), aba com bico na frente e nervura no alto.
    base = topo + 0.03
    casco = esfera("casco", (0, 0.004, base), (cx + 0.016, cy + 0.016, cz * 0.82), "epi_amarelo", 12, 7)
    cortar(casco, (0, 0, base), (0, 0, 1))
    aba = esfera("aba", (0, -0.012, base), (cx + 0.03, cy + 0.045, 0.006), "epi_amarelo", 16, 4)
    out["capacete"] = juntar([casco, aba], "epi_capacete")
    no_osso(out["capacete"], "Head")

    # Óculos: lente única e hastes até as orelhas.
    lente = caixa("lente", (0, frente + 0.006, 1.713), (0.07, 0.004, 0.02), "epi_lente")
    hastes = [
        caixa(f"haste_{s}", (s * (cx + 0.002), frente * 0.45, 1.716), (0.003, -frente * 0.55, 0.003), "epi_cinza")
        for s in (-1, 1)
    ]
    out["oculos"] = juntar([lente] + hastes, "epi_oculos")
    no_osso(out["oculos"], "Head")

    # Protetor auricular: conchas nas orelhas e arco por cima da cabeça.
    conchas = [
        esfera(f"concha_{s}", (s * (cx + 0.014), 0.006, 1.70), (0.016, 0.032, 0.038), "epi_vermelho", 8, 5)
        for s in (-1, 1)
    ]
    arco = tubo("arco", (0, 0.006, 1.70), cx + 0.016, 0.007, 8, 172, "epi_cinza")
    out["protetor_auricular"] = juntar(conchas + [arco], "epi_protetor_auricular")
    no_osso(out["protetor_auricular"], "Head")

    # Máscara: concha sobre o nariz e a boca (a metade de trás fica dentro do rosto).
    mascara = esfera("epi_mascara", (0, frente + 0.018, 1.668), (0.05, 0.03, 0.038), "epi_branco", 10, 6)
    no_osso(mascara, "Head")
    out["mascara"] = mascara

    # Luvas: mão, polegar e punho mais largos que os do corpo, presos a cada mão.
    luvas = []
    mx, my, mz = m["mao"]
    for s, lado in ((1, "l"), (-1, "r")):
        luva = Massa(f"luva_{lado}", 0.008)
        luva.bola((s * (0.745 + mx * 0.95), 0.062, 1.44), mx + 0.01, my + 0.01, mz + 0.012)
        luva.capsula((s * 0.77, 0.03, 1.42), (s * 0.86, -0.012, 1.375), 0.022)
        luva.capsula((s * 0.70, 0.066, 1.44), (s * 0.76, 0.064, 1.44), m["antebraco"] + 0.012)
        ob = luva.malha(f"luva_{lado}")
        reduzir(ob, 0.06)
        ob.data.materials.append(material("epi_laranja"))
        no_osso(ob, f"hand_{lado}")
        luvas.append(ob)
    out["luvas"] = juntar(luvas, "epi_luvas")

    # Botina: cano até o tornozelo.
    botas = []
    for s, lado in ((1, "l"), (-1, "r")):
        x = s * 0.09
        bota = Massa(f"bota_{lado}", 0.009)
        bota.capsula((x, 0.04, 0.052), (x, -0.17, 0.046), 0.054)
        bota.capsula((x, 0.03, 0.06), (x, 0.03, 0.17), m["panturrilha"] + 0.008)
        ob = bota.malha(f"bota_{lado}")
        reduzir(ob, 0.05)
        ob.data.materials.append(material("epi_couro"))
        transferir(ob, boneco, ossos_da(f"perna_{lado}"))
        botas.append(ob)
    out["botina"] = juntar(botas, "epi_botina")

    # Colete: casca do tronco, laranja com duas faixas refletivas.
    colete = Massa("colete", 0.011)
    px, py, pz = m["peito"]
    colete.bola((0, 0.012, 1.34), px + 0.014, py + 0.016, pz + 0.01)
    wx, wy, wz = m["cintura"]
    colete.bola((0, 0.008, 1.15), wx + 0.016, wy + 0.016, wz + 0.01)
    if m["seios"]:
        r = m["seios"]
        for s in (-1, 1):
            colete.bola((s * 0.062, -0.06, 1.32), r + 0.012, r * 0.9 + 0.012, r * 0.85 + 0.01)
    ob = colete.malha("epi_colete")
    cortar(ob, (0, 0, 1.06), (0, 0, 1))
    cortar(ob, (0, 0, 1.47), (0, 0, -1))
    reduzir(ob, 0.05)
    listras = ((1.16, 1.19), (1.30, 1.33))
    faixas(ob, [z for par in listras for z in par])
    ob.data.materials.append(material("epi_laranja"))
    ob.data.materials.append(material("faixa"))
    for p in ob.data.polygons:
        z = p.center.z
        p.material_index = 1 if any(a < z < b for a, b in listras) else 0
    transferir(ob, boneco, ossos_da("tronco"))
    out["colete"] = ob

    # Avental: placa na frente, do peito ao meio da coxa. Em cima segue o peito, embaixo a
    # bacia, com a passagem no meio.
    saliencia = m["peito"][1] + (m["seios"] * 0.9 if m["seios"] else 0.0)
    avental = caixa("epi_avental", (0, -saliencia - 0.022, 1.10), (0.15, 0.004, 0.33), "epi_cinza")
    faixas(avental, [0.95, 1.05, 1.15, 1.25])
    peito = avental.vertex_groups.new(name="spine_02")
    bacia = avental.vertex_groups.new(name="pelvis")
    for v in avental.data.vertices:
        w = min(1.0, max(0.0, (v.co.z - 0.95) / 0.30))
        peito.add([v.index], w, "REPLACE")
        bacia.add([v.index], 1.0 - w, "REPLACE")
    out["avental"] = avental
    return out


# ---------------------------------------------------------------------------
# Esqueleto e pesos
# ---------------------------------------------------------------------------


def esqueleto():
    """Importa o UAL1 e deixa só o esqueleto (sem animações) e o boneco (fonte dos pesos)."""
    bpy.ops.import_scene.gltf(filepath=UAL1)
    arm = bpy.data.objects["Armature"]
    boneco = bpy.data.objects["Mannequin"]
    for o in list(bpy.data.objects):
        if o not in (arm, boneco):
            bpy.data.objects.remove(o)
    if arm.animation_data:
        arm.animation_data.action = None
        for tr in list(arm.animation_data.nla_tracks):
            arm.animation_data.nla_tracks.remove(tr)
    for a in list(bpy.data.actions):
        bpy.data.actions.remove(a)
    return arm, boneco


def transferir(ob, boneco, pode):
    """Copia os pesos do boneco pelo ponto mais próximo, só dos ossos que `pode` aceita, e
    normaliza (um vértice que ficasse sem peso vai para o primeiro osso aceito)."""
    mod = ob.modifiers.new("pesos", "DATA_TRANSFER")
    mod.object = boneco
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
    reserva = ob.vertex_groups[0]
    for v in ob.data.vertices:
        total = sum(e.weight for e in v.groups)
        if total <= 1e-6:
            reserva.add([v.index], 1.0, "REPLACE")
            continue
        for e in v.groups:
            e.weight = e.weight / total


def ombro(ob, lado, x0=0.17, largura=0.12):
    """Perto do ombro, o braço passa aos poucos da clavícula para o braço: a ponta de dentro
    não gira para fora do ombro quando o braço desce."""
    cla = ob.vertex_groups.get(f"clavicle_{lado}") or ob.vertex_groups.new(name=f"clavicle_{lado}")
    sup = ob.vertex_groups.get(f"upperarm_{lado}") or ob.vertex_groups.new(name=f"upperarm_{lado}")
    for v in ob.data.vertices:
        d = abs(v.co.x)
        if d >= x0 + largura:
            continue
        w = min(1.0, max(0.0, (d - x0) / largura))
        w = w * w * (3 - 2 * w)
        for g in [ob.vertex_groups[e.group] for e in v.groups]:
            g.remove([v.index])
        sup.add([v.index], w, "REPLACE")
        cla.add([v.index], 1.0 - w, "REPLACE")


def no_osso(ob, osso):
    """Peça rígida presa a um osso só."""
    g = ob.vertex_groups.new(name=osso)
    g.add(list(range(len(ob.data.vertices))), 1.0, "REPLACE")


def prender(ob, arm):
    ob.parent = arm
    a = ob.modifiers.new("esqueleto", "ARMATURE")
    a.object = arm


# ---------------------------------------------------------------------------
# Principal
# ---------------------------------------------------------------------------


# Variações que o 3D sorteia por pessoa (tons de pele e de cabelo, sRGB) e a faixa de altura
# (m) de cada sexo; o esqueleto tem 1,82 m do chão ao topo da cabeça.
PELES = ["#f1c7a5", "#e0ac84", "#c68e6b", "#a86f4c", "#8a5638", "#5f3a25"]
CABELOS = ["#1f1a17", "#3b2a20", "#6b4a2b", "#a07040", "#c9a36a", "#8c8c8c"]
ALTURAS = {"homem": [1.65, 1.90], "mulher": [1.55, 1.78]}
ALTURA_BASE = 1.82
EPIS = ["avental", "botina", "capacete", "colete", "luvas", "mascara", "oculos", "protetor_auricular"]


def gerar(sexo, saida):
    """Gera `pessoa_<sexo>.glb` e devolve o item do catálogo."""
    bpy.ops.wm.read_factory_settings(use_empty=True)
    arm, boneco = esqueleto()
    c = corpo(sexo, boneco)
    prender(c, arm)
    for ob in epis(MEDIDAS[sexo], boneco).values():
        prender(ob, arm)
    bpy.data.objects.remove(boneco)
    for ob in bpy.data.objects:
        if ob.type == "MESH":
            # Triângulos fixos aqui, como em `comum.Elemento`.
            bm = bmesh.new()
            bm.from_mesh(ob.data)
            bmesh.ops.triangulate(bm, faces=bm.faces[:], quad_method="FIXED", ngon_method="EAR_CLIP")
            # A ordem das faces que os cortes e as junções deixam muda entre execuções.
            bm.verts.index_update()
            bm.faces.index_update()
            chave = {f.index: (f.material_index, tuple(sorted(v.index for v in f.verts))) for f in bm.faces}
            ordem = {i: n for n, i in enumerate(sorted(chave, key=chave.get))}
            bm.faces.sort(key=lambda f: ordem[f.index])
            bm.to_mesh(ob.data)
            bm.free()
    nome = f"pessoa_{sexo}"
    # A raiz tem o nome do elemento, como nos outros arquivos.
    arm.name = nome
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
    malhas = sorted(o.name for o in bpy.data.objects if o.type == "MESH")
    lo = [min(v[i] for o in bpy.data.objects if o.name == "corpo" for v in [o.matrix_world @ x.co for x in o.data.vertices]) for i in range(3)]
    hi = [max(v[i] for o in bpy.data.objects if o.name == "corpo" for v in [o.matrix_world @ x.co for x in o.data.vertices]) for i in range(3)]
    # Medidas no sistema do UTSim (+x frente, +y esquerda): a pessoa olha para -y do Blender.
    tamanho = [hi[1] - lo[1], hi[0] - lo[0], hi[2] - lo[2]]
    minimo = [-hi[1], lo[0], lo[2]]
    tri = sum(len(p.vertices) - 2 for o in bpy.data.objects if o.type == "MESH" for p in o.data.polygons)
    return {
        "arquivo": nome + ".glb",
        "tamanho": [round(v, 3) for v in tamanho],
        "minimo": [round(v, 3) for v in minimo],
        "triangulos": tri,
        "nos": malhas,
        "pontos": {},
        "clipes": {},
        "dados": {
            "altura_base": ALTURA_BASE,
            "alturas": ALTURAS[sexo],
            "cabelos": CABELOS,
            "epis": EPIS,
            "esqueleto": "UAL1",
            "peles": PELES,
            "recolorir": ["cabelo", "calca", "camisa", "pele"],
        },
    }


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

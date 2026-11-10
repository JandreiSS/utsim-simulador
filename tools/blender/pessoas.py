"""Pessoas de verdade (homem e mulher) para o 3D, no esqueleto do boneco livre UAL1.

    blender -b --factory-startup --python-exit-code 1 -P tools/blender/pessoas.py -- \
        [--saida assets/modelos3d] [--so homem]

Cada arquivo `pessoa_<homem|mulher>.glb` traz o esqueleto do UAL1 (mesmos ossos, mesma pose
de repouso, sem animações: o 3D usa as do UAL1 e do UAL2) e:

- `corpo`: uma malha lisa feita de anéis sobre os ossos (tronco, cada braço com a mão e os
  cinco dedos, cada perna com o sapato, cabeça). O rosto tem olhos, sobrancelhas, nariz,
  boca e orelhas; o homem tem cabelo curto e a mulher, até os ombros, e os dois têm corpo
  e rosto diferentes (`TRONCO`, `CABECA`, `MEDIDAS`). A cor vem só dos materiais, sem
  textura: o 3D recolore `pele`, `cabelo`, `camisa` e `calca` para variar as pessoas;
- `epi_<nome>`: uma malha por EPI de `PPE_NAMES` (capacete, óculos, protetor auricular,
  luvas, botina, colete, máscara, avental), escondida ou mostrada conforme o EPI que a pessoa
  veste no modelo. Cada EPI segue a superfície do corpo, com folga, e deixa o rosto à mostra.

Os pesos de pele vêm do boneco UAL1 (transferência pelo ponto mais próximo), então todas as
animações dele servem. Metros, Z para cima; a pessoa olha para -y do Blender, como o UAL1.
"""

import argparse
import math
import os
import sys

import bmesh
import bpy
from mathutils import Vector

AQUI = os.path.dirname(os.path.abspath(__file__))
RAIZ = os.path.normpath(os.path.join(AQUI, "..", ".."))
UAL1 = os.path.join(RAIZ, "assets", "terceiros", "quaternius_ual", "UAL1_Standard.glb")

# Cores (sRGB, 0..255), as mesmas convenções de `comum.PALETA`.
CORES = {
    "pele": (0xC6, 0x8E, 0x6B),
    "cabelo": (0x3B, 0x2A, 0x20),
    "camisa": (0x2F, 0x5D, 0x8C),
    "calca": (0x2B, 0x33, 0x40),
    "calcado": (0x26, 0x22, 0x20),
    "olho": (0x2A, 0x1C, 0x16),
    "olho_branco": (0xF1, 0xEE, 0xE8),
    "cinto": (0x35, 0x27, 0x1D),
    "fivela": (0xB4, 0xB8, 0xBD),
    "sola": (0x15, 0x14, 0x14),
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
    b.inputs["Roughness"].default_value = 0.35 if nome in ("olho", "olho_branco", "epi_lente") else 0.8
    if nome == "epi_lente":
        b.inputs["Alpha"].default_value = 0.4
        m.surface_render_method = "BLENDED"
    if nome == "fivela":
        b.inputs["Metallic"].default_value = 0.8
        b.inputs["Roughness"].default_value = 0.4
    return m


# ---------------------------------------------------------------------------
# Malha por anéis
# ---------------------------------------------------------------------------


def sp(c, e):
    """Potência que mantém o sinal (superelipse)."""
    return math.copysign(abs(c) ** e, c)


def gauss(d2, s):
    return math.exp(-d2 / (2 * s * s))


class Malha:
    """Malha em construção, feita de anéis ligados por faixas. A ordem dos pontos de cada
    anel e a ordem dos anéis fixam o lado de fora, sem depender do Blender."""

    def __init__(self, nome):
        self.nome = nome
        self.bm = bmesh.new()
        self.mats = []

    def mat(self, nome):
        if nome not in self.mats:
            self.mats.append(nome)
        return self.mats.index(nome)

    def anel(self, pts):
        return [self.bm.verts.new(p) for p in pts]

    def faixa(self, a, b, mat, fechado=True):
        """Liga o anel `a` ao anel `b` (o seguinte no sentido do eixo)."""
        k = self.mat(mat)
        n = len(a)
        for j in range(n if fechado else n - 1):
            j2 = (j + 1) % n
            f = self.bm.faces.new((a[j], a[j2], b[j2], b[j]))
            f.material_index = k
            f.smooth = True

    def tampa(self, anel, mat, ponta, adiante=True):
        """Fecha o anel num ponto; `adiante` quando a ponta fica no sentido do eixo."""
        k = self.mat(mat)
        c = self.bm.verts.new(ponta)
        n = len(anel)
        for j in range(n):
            j2 = (j + 1) % n
            f = self.bm.faces.new((anel[j], anel[j2], c) if adiante else (anel[j2], anel[j], c))
            f.material_index = k
            f.smooth = True

    def tubo(self, centros, raios, u, mats, n=8, p=2.0, pontas=(True, True), arred=0.5):
        """Tubo pelos `centros`, com um anel em cada um. `raios` = (ru, rv) por anel: `ru`
        na direção `u`, `rv` na direção eixo × u. `mats` = um material, ou um por trecho."""
        centros = [Vector(c) for c in centros]
        d = (centros[-1] - centros[0]).normalized()
        u = Vector(u)
        u = (u - u.dot(d) * d).normalized()
        v = d.cross(u)
        if isinstance(mats, str):
            mats = [mats] * (len(centros) - 1)
        aneis = []
        for c, (ru, rv) in zip(centros, raios):
            pts = []
            for j in range(n):
                t = 2 * math.pi * j / n
                pts.append(c + ru * sp(math.cos(t), 2 / p) * u + rv * sp(math.sin(t), 2 / p) * v)
            aneis.append(self.anel(pts))
        for i in range(len(aneis) - 1):
            self.faixa(aneis[i], aneis[i + 1], mats[i])
        if pontas[0]:
            self.tampa(aneis[0], mats[0], centros[0] - d * arred * min(raios[0]), False)
        if pontas[1]:
            self.tampa(aneis[-1], mats[-1], centros[-1] + d * arred * min(raios[-1]), True)
        return aneis

    def elipsoide(self, c, r, mat, n=10, aneis=5):
        c = Vector(c)
        lista = []
        for i in range(1, aneis + 1):
            a = -math.pi / 2 + math.pi * i / (aneis + 1)
            pts = [
                c + Vector((r[0] * math.cos(a) * math.cos(2 * math.pi * j / n),
                            r[1] * math.cos(a) * math.sin(2 * math.pi * j / n), r[2] * math.sin(a)))
                for j in range(n)
            ]
            lista.append(self.anel(pts))
        for i in range(len(lista) - 1):
            self.faixa(lista[i], lista[i + 1], mat)
        self.tampa(lista[0], mat, c - Vector((0, 0, r[2])), False)
        self.tampa(lista[-1], mat, c + Vector((0, 0, r[2])), True)

    def casca(self, perfil, zs, mats, n=20, folga=0.0, a0=0.0, a1=360.0, cupula=False):
        """Faixas que seguem a superfície do `perfil` nas alturas `zs` (de baixo para cima),
        afastadas `folga` (número, ou função de z e do ângulo). Com `a0`..`a1` (graus)
        menor que a volta inteira, a casca é aberta."""
        fechado = a1 - a0 >= 360.0
        sobe = sobe_cupula if cupula else (lambda z: 0.0)
        if isinstance(mats, str):
            mats = [mats] * (len(zs) - 1)
        aneis = []
        for z in zs:
            pts = []
            for j in range(n if fechado else n + 1):
                ang = a0 + (a1 - a0) * j / n
                f = folga(z, ang) if callable(folga) else folga
                pts.append(perfil.ponto(z, ang, f) + Vector((0.0, 0.0, f * sobe(z))))
            aneis.append(self.anel(pts))
        for i in range(len(aneis) - 1):
            self.faixa(aneis[i], aneis[i + 1], mats[i], fechado)
        return aneis

    def objeto(self):
        me = bpy.data.meshes.new(self.nome)
        self.bm.to_mesh(me)
        self.bm.free()
        for n in self.mats:
            me.materials.append(material(n))
        ob = bpy.data.objects.new(self.nome, me)
        bpy.context.scene.collection.objects.link(ob)
        return ob


def sobe_cupula(z):
    """Quanto da folga vai para cima: nada até a testa, tudo no alto da cabeça."""
    t = min(1.0, max(0.0, (z - 1.745) / (TOPO - 1.745)))
    return t * t * (3 - 2 * t)


class Perfil:
    """Superfície dada por cortes horizontais: (z, meia largura, fundo à frente, fundo atrás,
    centro em y, expoente da superelipse). A pessoa olha para -y; o ângulo 0 é +x (lado
    esquerdo dela), 90 é atrás e 270 é a frente."""

    def __init__(self, linhas, desloca=None):
        self.linhas = sorted(linhas)
        self.desloca = desloca

    def em(self, z):
        ls = self.linhas
        if z <= ls[0][0]:
            return ls[0][1:]
        for a, b in zip(ls, ls[1:]):
            if z <= b[0]:
                t = (z - a[0]) / (b[0] - a[0])
                return tuple(a[i] + (b[i] - a[i]) * t for i in range(1, 6))
        return ls[-1][1:]

    def ponto(self, z, ang, folga=0.0):
        rx, rf, rb, cy, p = self.em(z)
        c, s = math.cos(math.radians(ang)), math.sin(math.radians(ang))
        x = (rx + folga) * sp(c, 2 / p)
        y = cy + ((rb if s > 0 else rf) + folga) * sp(s, 2 / p)
        if self.desloca:
            y += self.desloca(x, y - cy, z)
        return Vector((x, y, z))

    def frente(self, x, z, folga=0.0):
        """O y da superfície na frente, em (x, z)."""
        rx, rf, rb, cy, p = self.em(z)
        t = min(1.0, abs(x) / (rx + folga))
        y = cy - (rf + folga) * (1 - t ** p) ** (1 / p)
        if self.desloca:
            y += self.desloca(x, y - cy, z)
        return y


# ---------------------------------------------------------------------------
# Corpo
# ---------------------------------------------------------------------------

# Cortes do tronco (do meio das pernas ao pescoço), em metros.
TRONCO = {
    "homem": [
        (0.845, 0.070, 0.055, 0.065, 0.022, 2.2), (0.880, 0.135, 0.085, 0.095, 0.020, 2.4),
        (0.930, 0.166, 0.098, 0.108, 0.020, 2.5), (0.980, 0.168, 0.100, 0.108, 0.018, 2.5),
        (1.030, 0.160, 0.100, 0.100, 0.016, 2.5), (1.090, 0.150, 0.098, 0.092, 0.012, 2.4),
        (1.150, 0.146, 0.095, 0.088, 0.010, 2.4), (1.220, 0.152, 0.100, 0.092, 0.012, 2.4),
        (1.300, 0.164, 0.108, 0.100, 0.016, 2.4), (1.360, 0.174, 0.112, 0.104, 0.022, 2.4),
        (1.410, 0.180, 0.106, 0.104, 0.030, 2.3), (1.450, 0.172, 0.090, 0.098, 0.038, 2.2),
        (1.480, 0.125, 0.074, 0.086, 0.032, 2.1), (1.505, 0.072, 0.060, 0.068, 0.022, 2.0),
        (1.530, 0.056, 0.053, 0.058, 0.014, 2.0), (1.600, 0.053, 0.052, 0.055, 0.010, 2.0),
    ],
    "mulher": [
        (0.845, 0.070, 0.055, 0.065, 0.022, 2.2), (0.880, 0.140, 0.085, 0.098, 0.020, 2.4),
        (0.930, 0.172, 0.096, 0.112, 0.020, 2.5), (0.980, 0.170, 0.096, 0.110, 0.018, 2.5),
        (1.030, 0.152, 0.090, 0.096, 0.016, 2.4), (1.090, 0.130, 0.084, 0.084, 0.012, 2.3),
        (1.150, 0.122, 0.080, 0.078, 0.010, 2.3), (1.220, 0.128, 0.084, 0.080, 0.012, 2.3),
        (1.300, 0.140, 0.090, 0.086, 0.016, 2.3), (1.360, 0.148, 0.092, 0.090, 0.022, 2.3),
        (1.410, 0.152, 0.088, 0.090, 0.030, 2.2), (1.450, 0.146, 0.078, 0.086, 0.038, 2.2),
        (1.480, 0.105, 0.064, 0.076, 0.032, 2.1), (1.505, 0.060, 0.052, 0.060, 0.022, 2.0),
        (1.530, 0.047, 0.045, 0.050, 0.014, 2.0), (1.600, 0.045, 0.044, 0.047, 0.010, 2.0),
    ],
}

# Cortes da cabeça (do pescoço ao alto), o topo e o que muda no rosto.
CABECA = {
    "homem": [
        (1.575, 0.044, 0.044, 0.046, 0.012, 2.0), (1.600, 0.046, 0.050, 0.050, 0.010, 2.0),
        (1.612, 0.047, 0.078, 0.062, 0.006, 1.9), (1.622, 0.057, 0.090, 0.066, 0.004, 1.9),
        (1.640, 0.067, 0.095, 0.074, 0.003, 2.0), (1.660, 0.073, 0.097, 0.082, 0.003, 2.1),
        (1.680, 0.076, 0.097, 0.088, 0.003, 2.2), (1.700, 0.078, 0.096, 0.092, 0.003, 2.2),
        (1.715, 0.079, 0.095, 0.094, 0.003, 2.2), (1.735, 0.079, 0.099, 0.096, 0.003, 2.2),
        (1.760, 0.078, 0.097, 0.098, 0.004, 2.2), (1.785, 0.073, 0.089, 0.094, 0.005, 2.1),
        (1.805, 0.061, 0.072, 0.080, 0.006, 2.0), (1.820, 0.040, 0.046, 0.054, 0.007, 2.0),
        (1.827, 0.020, 0.023, 0.027, 0.007, 2.0),
    ],
    "mulher": [
        (1.575, 0.037, 0.037, 0.039, 0.012, 2.0), (1.600, 0.039, 0.044, 0.044, 0.010, 2.0),
        (1.612, 0.038, 0.072, 0.058, 0.006, 1.8), (1.622, 0.047, 0.084, 0.062, 0.004, 1.8),
        (1.640, 0.058, 0.090, 0.070, 0.003, 1.9), (1.660, 0.066, 0.092, 0.078, 0.003, 2.0),
        (1.680, 0.071, 0.092, 0.084, 0.003, 2.1), (1.700, 0.074, 0.091, 0.088, 0.003, 2.2),
        (1.715, 0.075, 0.090, 0.090, 0.003, 2.2), (1.735, 0.075, 0.092, 0.092, 0.003, 2.2),
        (1.760, 0.074, 0.092, 0.094, 0.004, 2.2), (1.785, 0.070, 0.085, 0.090, 0.005, 2.1),
        (1.805, 0.058, 0.069, 0.077, 0.006, 2.0), (1.820, 0.038, 0.044, 0.052, 0.007, 2.0),
        (1.827, 0.019, 0.022, 0.026, 0.007, 2.0),
    ],
}
TOPO = 1.829

# O que muda entre homem e mulher fora dos cortes.
MEDIDAS = {
    "homem": {
        "membros": 1.0, "panturrilha": 1.0, "olho": 1.0, "olho_x": 0.031, "sobrancelha": (0.0032, 0.0026, 0.0),
        "nariz": 1.0, "boca": (0.024, 0.0030), "labio": (0x9A, 0x68, 0x5C), "cilios": False,
    },
    "mulher": {
        "membros": 0.86, "panturrilha": 0.92, "olho": 1.14, "olho_x": 0.030, "sobrancelha": (0.0019, 0.0020, 0.004),
        "nariz": 0.84, "boca": (0.022, 0.0042), "labio": (0xB0, 0x56, 0x5A), "cilios": True,
    },
}


def perfil_tronco(sexo):
    def seios(x, dy, z):
        if dy >= 0:
            return 0.0
        return -0.042 * gauss((abs(x) - 0.066) ** 2, 0.040) * gauss((z - 1.325) ** 2, 0.042)

    return Perfil(TRONCO[sexo], seios if sexo == "mulher" else None)


def perfil_cabeca(sexo):
    ox = MEDIDAS[sexo]["olho_x"]

    def orbitas(x, dy, z):
        if dy >= 0:
            return 0.0
        return 0.007 * gauss((abs(x) - ox) ** 2 + (z - 1.715) ** 2, 0.013)

    return Perfil(CABECA[sexo], orbitas)


def tronco(sexo):
    """Tronco e pescoço, com a calça abaixo do cinto, a camisa até a gola e o cinto."""
    t = perfil_tronco(sexo)
    m = Malha("tronco")
    zs = [l[0] for l in TRONCO[sexo]]
    mats = ["calca" if (a + b) / 2 < 1.03 else "camisa" if (a + b) / 2 < 1.505 else "pele" for a, b in zip(zs, zs[1:])]
    aneis = m.casca(t, zs, mats)
    m.tampa(aneis[0], "calca", (0.0, 0.022, 0.828), False)
    m.tampa(aneis[-1], "pele", (0.0, 0.010, 1.602), True)
    # Gola e cinto com fivela.
    gola = m.casca(t, [1.490, 1.508, 1.526], "camisa", folga=lambda z, a: 0.007 + (z - 1.490) * 0.25)
    m.casca(t, [1.526, 1.5261], "camisa", folga=lambda z, a: 0.016 if z < 1.52605 else 0.002)
    del gola
    m.casca(t, [1.018, 1.050], "cinto", folga=0.0045)
    m.casca(t, [1.050, 1.0501], "cinto", folga=lambda z, a: 0.0045 if z < 1.05005 else 0.0)
    y = t.frente(0.0, 1.034, 0.0045)
    m.elipsoide((0.0, y, 1.034), (0.017, 0.005, 0.013), "fivela", 8, 3)
    return m.objeto()


def mao(m, s, mat, folga=0.0, esc=1.0):
    """Palma e cinco dedos sobre os ossos da mão (a palma olha para baixo)."""
    def p(x, y, z):
        return (s * x, y, z)

    f = folga
    palma = [(0.735, 0.066, 1.441, 0.027, 0.019), (0.775, 0.068, 1.441, 0.040, 0.016),
             (0.820, 0.070, 1.440, 0.046, 0.0145), (0.862, 0.071, 1.440, 0.045, 0.013)]
    m.tubo([p(*l[:3]) for l in palma], [(l[3] * esc + f, l[4] * esc + f) for l in palma], (0, 1, 0), mat, 10, 3.2)
    dedos = [
        ([(0.850, 0.035, 1.439), (0.899, 0.035, 1.439), (0.934, 0.035, 1.439), (0.964, 0.035, 1.439),
          (0.980, 0.035, 1.439)], [0.0105, 0.0100, 0.0095, 0.0088, 0.0070]),
        ([(0.850, 0.060, 1.441), (0.903, 0.062, 1.441), (0.937, 0.064, 1.441), (0.971, 0.065, 1.441),
          (0.989, 0.066, 1.441)], [0.0110, 0.0105, 0.0098, 0.0090, 0.0072]),
        ([(0.850, 0.083, 1.441), (0.897, 0.084, 1.441), (0.928, 0.085, 1.441), (0.960, 0.086, 1.441),
          (0.977, 0.087, 1.441)], [0.0102, 0.0098, 0.0092, 0.0085, 0.0068]),
        ([(0.842, 0.106, 1.439), (0.887, 0.109, 1.439), (0.914, 0.111, 1.439), (0.942, 0.113, 1.439),
          (0.956, 0.114, 1.439)], [0.0090, 0.0086, 0.0080, 0.0074, 0.0060]),
        ([(0.752, 0.046, 1.430), (0.766, 0.032, 1.418), (0.800, 0.014, 1.399), (0.839, -0.006, 1.377),
          (0.872, -0.023, 1.359), (0.884, -0.029, 1.352)], [0.0170, 0.0160, 0.0135, 0.0120, 0.0110, 0.0080]),
    ]
    for pts, rs in dedos:
        m.tubo([p(*q) for q in pts], [(r * esc + f, r * esc + f) for r in rs], (0, 0, 1), mat, 6)


def braco(sexo, s):
    """Braço com manga curta, antebraço e mão."""
    e = MEDIDAS[sexo]["membros"]
    m = Malha(f"braco_{'l' if s > 0 else 'r'}")
    linhas = [
        (0.150, 0.040, 0.020, "camisa"), (0.210, 0.054, 0.044, "camisa"), (0.255, 0.052, 0.052, "camisa"),
        (0.320, 0.049, 0.051, "camisa"), (0.372, 0.050, 0.052, "camisa"), (0.372, 0.043, 0.045, "pele"),
        (0.420, 0.041, 0.042, "pele"), (0.466, 0.038, 0.040, "pele"), (0.520, 0.042, 0.042, "pele"),
        (0.620, 0.036, 0.036, "pele"), (0.715, 0.029, 0.027, "pele"), (0.745, 0.028, 0.021, "pele"),
    ]
    m.tubo([(s * l[0], 0.067, 1.441) for l in linhas], [(l[1] * e, l[2] * e) for l in linhas], (0, 1, 0),
           [l[3] for l in linhas[:-1]], 10)
    mao(m, s, "pele", 0.0, 0.88 if sexo == "mulher" else 1.0)
    return m.objeto()


def pe(m, s, mat, folga=0.0, cano=0.20, sola="sola"):
    """Calçado: cano, pé e sola. Com `folga`, cresce para os lados e para cima (a sola
    continua no chão)."""
    x = s * 0.089
    f = folga
    m.tubo([(x, 0.035, cano), (x, 0.035, 0.06)], [(0.041 + f, 0.042 + f)] * 2, (1, 0, 0), mat, 10, pontas=(False, True))
    linhas = [(0.085, 0.030, 0.070), (0.060, 0.041, 0.098), (0.000, 0.045, 0.094), (-0.055, 0.047, 0.074),
              (-0.115, 0.048, 0.058), (-0.165, 0.044, 0.048), (-0.198, 0.030, 0.036)]
    m.tubo([(x, y, (h + f) / 2) for y, w, h in linhas], [(w + f, (h + f) / 2) for y, w, h in linhas],
           (1, 0, 0), mat, 12, 3.0)
    m.tubo([(x, y, 0.008) for y, w, h in linhas], [(w + f + 0.003, 0.008) for y, w, h in linhas],
           (1, 0, 0), sola, 12, 3.0)


def perna(sexo, s):
    """Perna de calça até a barra e o sapato."""
    md = MEDIDAS[sexo]
    m = Malha(f"perna_{'l' if s > 0 else 'r'}")
    linhas = [
        (0.985, 0.088, 0.016, 0.078, 0.088, 1.0), (0.900, 0.089, 0.012, 0.086, 0.094, 1.0),
        (0.800, 0.089, 0.008, 0.081, 0.087, 1.0), (0.680, 0.089, 0.004, 0.070, 0.075, 1.0),
        (0.560, 0.089, 0.003, 0.058, 0.062, md["panturrilha"]), (0.500, 0.089, 0.006, 0.055, 0.058, md["panturrilha"]),
        (0.420, 0.089, 0.016, 0.058, 0.062, md["panturrilha"]), (0.300, 0.089, 0.027, 0.050, 0.052, md["panturrilha"]),
        (0.165, 0.089, 0.035, 0.048, 0.049, 1.0), (0.165, 0.089, 0.035, 0.040, 0.041, 1.0),
    ]
    m.tubo([(s * l[1], l[2], l[0]) for l in linhas], [(l[3] * l[5], l[4] * l[5]) for l in linhas], (1, 0, 0), "calca", 12)
    pe(m, s, "calcado")
    return m.objeto()


def linha_cabelo(sexo, ang):
    """Altura até onde o cabelo desce, pelo ângulo em volta da cabeça."""
    fi = abs((ang - 270.0 + 180.0) % 360.0 - 180.0)  # 0 na frente, 180 atrás
    def rampa(a, b, za, zb):
        t = (fi - a) / (b - a)
        return za + (zb - za) * t * t * (3 - 2 * t)

    if sexo == "homem":
        if fi < 50:
            return 1.776
        if fi < 78:
            return rampa(50, 78, 1.776, 1.728)
        if fi < 96:
            return rampa(78, 96, 1.728, 1.690)
        if fi < 125:
            return rampa(96, 125, 1.690, 1.640)
        return 1.640
    if fi < 42:
        return 1.781
    if fi < 68:
        return rampa(42, 68, 1.781, 1.600)
    if fi < 92:
        return rampa(68, 92, 1.600, 1.515)
    return 1.515


def cabelo(m, h, sexo, n=28):
    """Casca de cabelo sobre a cabeça: curto no homem, até os ombros na mulher."""
    if sexo == "homem":
        zs = [1.640 + 0.0125 * i for i in range(15)] + [1.827]

        def folga(z, ang):
            t = min(1.0, max(0.0, (z - 1.745) / 0.06))
            return 0.007 + 0.010 * t * t * (3 - 2 * t)
    else:
        zs = [1.515 + 0.0135 * i for i in range(23)] + [1.827]

        def folga(z, ang):
            t = min(1.0, max(0.0, (1.675 - z) / 0.15))
            return 0.015 + 0.016 * t
    vs = {}

    def v(i, j):
        j %= n
        if (i, j) not in vs:
            ang = 360.0 * j / n
            f = folga(zs[i], ang)
            vs[(i, j)] = m.bm.verts.new(h.ponto(zs[i], ang, f) + Vector((0.0, 0.0, f * sobe_cupula(zs[i]))))
        return vs[(i, j)]

    k = m.mat("cabelo")
    for i in range(len(zs) - 1):
        for j in range(n):
            if (zs[i] + zs[i + 1]) / 2 < linha_cabelo(sexo, 360.0 * (j + 0.5) / n):
                continue
            f = m.bm.faces.new((v(i, j), v(i, j + 1), v(i + 1, j + 1), v(i + 1, j)))
            f.material_index = k
            f.smooth = True
    topo = [v(len(zs) - 1, j) for j in range(n)]
    m.tampa(topo, "cabelo", (0.0, 0.007, TOPO + folga(TOPO, 0.0) + 0.002), True)


def cabeca(sexo):
    """Cabeça com rosto (olhos, sobrancelhas, nariz, boca, orelhas) e cabelo."""
    md = MEDIDAS[sexo]
    h = perfil_cabeca(sexo)
    m = Malha("cabeca")
    zs = [l[0] for l in CABECA[sexo]]
    aneis = m.casca(h, zs, "pele", 24)
    m.tampa(aneis[0], "pele", (0.0, 0.012, 1.570), False)
    m.tampa(aneis[-1], "pele", (0.0, 0.007, TOPO), True)
    eo = md["olho"]
    ox = md["olho_x"]
    for s in (-1, 1):
        ye = h.frente(ox, 1.715)
        m.elipsoide((s * ox, ye + 0.0042, 1.715), (0.0135 * eo, 0.0065, 0.0082 * eo), "olho_branco", 10, 4)
        m.elipsoide((s * ox, ye - 0.0008, 1.7148), (0.0063 * eo, 0.0026, 0.0063 * eo), "olho", 8, 3)
        if md["cilios"]:
            m.tubo([(s * (ox + dx), h.frente(ox + dx, 1.715) - 0.0012, 1.7235 - 0.0030 * (dx / 0.013) ** 2)
                    for dx in (-0.013, -0.006, 0.002, 0.009, 0.015)],
                   [(0.0012, 0.0016)] * 5, (0, 0, 1), "olho", 4)
        rz, ry, arco = md["sobrancelha"]
        xs = (0.013, 0.026, 0.040, 0.052)
        alt = (1.7345, 1.7375 + arco, 1.7370 + arco, 1.7325 + arco * 0.5)
        m.tubo([(s * x, h.frente(x, z) - 0.0012, z) for x, z in zip(xs, alt)],
               [(rz, ry), (rz, ry), (rz * 0.9, ry), (rz * 0.5, ry * 0.7)], (0, 0, 1), "cabelo", 6)
        rx = h.em(1.703)[0]
        m.elipsoide((s * (rx + 0.001), 0.013, 1.703), (0.0065, 0.014, 0.027), "pele", 8, 4)
    # Nariz: do alto, entre os olhos, até a base.
    en = md["nariz"]
    nz = [(1.728, 0.0075, 0.0050), (1.708, 0.0085, 0.0095), (1.688, 0.0115, 0.0170), (1.674, 0.0155, 0.0235),
          (1.666, 0.0150, 0.0170), (1.660, 0.0090, 0.0060)]
    m.tubo([(0.0, h.frente(0.0, z) + 0.002, z) for z, w, pr in nz], [(w * en, pr * en) for z, w, pr in nz],
           (1, 0, 0), "pele", 8, arred=0.3)
    # Boca: lábio de cima e de baixo.
    lw, lr = md["boca"]
    for z, r in ((1.6525, lr * 0.85), (1.6525 - lr * 1.5, lr)):
        m.tubo([(lw * t, h.frente(lw * t, z) + 0.0012, z - 0.0015 * abs(t)) for t in (-1.0, -0.6, 0.0, 0.6, 1.0)],
               [(r * q, r * q * 0.9) for q in (0.25, 0.85, 1.0, 0.85, 0.25)], (0, 0, 1), "labio", 6, arred=0.3)
    cabelo(m, h, sexo)
    ob = m.objeto()
    no_osso(ob, "Head")
    return ob


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
    partes = []
    t = tronco(sexo)
    transferir(t, boneco, ossos_da("tronco"))
    partes.append(t)
    for s, lado in ((1, "l"), (-1, "r")):
        b = braco(sexo, s)
        transferir(b, boneco, ossos_da(f"braco_{lado}"))
        ombro(b, lado)
        p = perna(sexo, s)
        transferir(p, boneco, ossos_da(f"perna_{lado}"))
        partes += [b, p]
    partes.append(cabeca(sexo))
    return juntar(partes, "corpo")


# ---------------------------------------------------------------------------
# EPIs
# ---------------------------------------------------------------------------


def epis(sexo, boneco):
    """As peças de EPI, cada uma num objeto `epi_<nome>` com os pesos prontos. Todas seguem
    a superfície do corpo (os mesmos cortes, com folga)."""
    md = MEDIDAS[sexo]
    t = perfil_tronco(sexo)
    h = perfil_cabeca(sexo)
    out = {}

    # Capacete: casco sobre o cabelo, com aba e bico na frente; a testa e o rosto ficam à mostra.
    m = Malha("epi_capacete")
    zs = [1.758, 1.778, 1.798, 1.812, 1.822, 1.827]
    aneis = m.casca(h, zs, "epi_amarelo", 24, folga=0.026, cupula=True)
    m.tampa(aneis[-1], "epi_amarelo", (0.0, 0.007, TOPO + 0.029), True)

    def aba(z, ang):
        fi = abs((ang - 270.0 + 180.0) % 360.0 - 180.0)
        bico = max(0.0, math.cos(math.radians(min(fi, 90.0) * 1.5))) ** 2
        return (0.026 if z > 1.757 else 0.038 + 0.040 * bico)

    m.casca(h, [1.754, 1.758], "epi_amarelo", 24, folga=aba)
    m.tubo([(0.0, -0.070, 1.842), (0.0, -0.03, 1.857), (0.0, 0.03, 1.858), (0.0, 0.080, 1.842)],
           [(0.012, 0.004), (0.014, 0.006), (0.014, 0.006), (0.012, 0.004)], (1, 0, 0), "epi_amarelo", 6)
    out["capacete"] = m.objeto()
    no_osso(out["capacete"], "Head")

    # Óculos: duas lentes claras, ponte e hastes finas. Os olhos aparecem por trás.
    m = Malha("epi_oculos")
    ox = md["olho_x"] + 0.001
    yl = h.frente(ox, 1.715) - 0.011
    for s in (-1, 1):
        m.elipsoide((s * ox, yl, 1.714), (0.0215, 0.0030, 0.0140), "epi_lente", 14, 3)
        rx = h.em(1.712)[0]
        m.tubo([(s * (ox + 0.021), yl + 0.002, 1.716), (s * (rx + 0.006), yl + 0.035, 1.716),
                (s * (rx + 0.008), 0.016, 1.713)], [(0.0020, 0.0024)] * 3, (0, 0, 1), "epi_cinza", 4)
    m.tubo([(-0.011, yl - 0.001, 1.717), (0.011, yl - 0.001, 1.717)], [(0.0022, 0.0022)] * 2, (0, 0, 1), "epi_cinza", 4)
    out["oculos"] = m.objeto()
    no_osso(out["oculos"], "Head")

    # Protetor auricular: conchas nas orelhas e arco por trás da nuca.
    m = Malha("epi_protetor_auricular")
    rx = h.em(1.703)[0]
    for s in (-1, 1):
        m.elipsoide((s * (rx + 0.017), 0.013, 1.703), (0.018, 0.030, 0.036), "epi_vermelho", 10, 4)
    m.casca(h, [1.690, 1.708], "epi_cinza", 14, folga=0.034 if sexo == "mulher" else 0.020, a0=8.0, a1=172.0)
    out["protetor_auricular"] = m.objeto()
    no_osso(out["protetor_auricular"], "Head")

    # Máscara: concha só sobre o nariz e a boca, com dois elásticos.
    m = Malha("epi_mascara")
    m.elipsoide((0.0, h.frente(0.0, 1.664) + 0.004, 1.664), (0.040, 0.036, 0.032), "epi_branco", 12, 5)
    for z in (1.648, 1.682):
        m.casca(h, [z, z + 0.006], "epi_branco", 16, folga=0.004, a0=-62.0, a1=242.0)
    out["mascara"] = m.objeto()
    no_osso(out["mascara"], "Head")

    # Luvas: a mão com folga, dedo por dedo, e o punho.
    luvas = []
    for s, lado in ((1, "l"), (-1, "r")):
        m = Malha(f"luva_{lado}")
        mao(m, s, "epi_laranja", 0.0035, 0.88 if sexo == "mulher" else 1.0)
        e = md["membros"]
        m.tubo([(s * 0.672, 0.067, 1.441), (s * 0.690, 0.067, 1.441), (s * 0.745, 0.066, 1.441)],
               [(0.034 * e + 0.006, 0.033 * e + 0.006), (0.033 * e + 0.007, 0.031 * e + 0.007),
                (0.031 * e + 0.005, 0.024 * e + 0.005)], (0, 1, 0), "epi_laranja", 10)
        ob = m.objeto()
        transferir(ob, boneco, ossos_da(f"braco_{lado}"))
        luvas.append(ob)
    out["luvas"] = juntar(luvas, "epi_luvas")

    # Botina: o calçado com folga e cano acima da barra da calça.
    botas = []
    for s, lado in ((1, "l"), (-1, "r")):
        m = Malha(f"bota_{lado}")
        pe(m, s, "epi_couro", 0.013, 0.255)
        ob = m.objeto()
        transferir(ob, boneco, ossos_da(f"perna_{lado}"))
        botas.append(ob)
    out["botina"] = juntar(botas, "epi_botina")

    # Colete: veste o tronco da cintura aos ombros, com duas faixas refletivas.
    m = Malha("epi_colete")
    zs = [1.070, 1.110, 1.160, 1.190, 1.240, 1.300, 1.330, 1.370, 1.410, 1.450, 1.480, 1.500]
    mats = ["faixa" if (a, b) in ((1.160, 1.190), (1.300, 1.330)) else "epi_laranja" for a, b in zip(zs, zs[1:])]
    m.casca(t, zs, mats, folga=0.013)
    out["colete"] = m.objeto()
    transferir(out["colete"], boneco, ossos_da("tronco"))

    # Avental: peitilho e saia que seguem a frente do corpo, com alça no pescoço e tira nas costas.
    m = Malha("epi_avental")
    fo = 0.022
    m.casca(t, [1.215, 1.245, 1.275, 1.305, 1.335, 1.365, 1.405], "epi_cinza", 14, folga=fo, a0=228.0, a1=312.0)
    saia = [0.560, 0.660, 0.760, 0.860, 0.960, 1.030, 1.090, 1.150, 1.215]
    aneis = []
    for z in saia:
        pts = []
        for j in range(17):
            ang = 197.0 + (343.0 - 197.0) * j / 16
            p = t.ponto(max(z, 0.960), ang, fo)
            pts.append(Vector((p.x, p.y, z)))
        aneis.append(m.anel(pts))
    for a, b in zip(aneis, aneis[1:]):
        m.faixa(a, b, "epi_cinza", False)
    for lado, a0, a1 in ((1, 312.0, 330.0), (-1, 228.0, 210.0)):
        m.tubo([t.ponto(1.400, a0, fo), t.ponto(1.455, (a0 + a1) / 2, fo - 0.003), t.ponto(1.503, a1, 0.015)],
               [(0.010, 0.003)] * 3, (lado, 0, 0), "epi_cinza", 4)
    m.casca(t, [1.496, 1.510], "epi_cinza", 12, folga=0.015, a0=-30.0, a1=210.0)
    m.casca(t, [1.120, 1.142], "epi_cinza", 14, folga=0.015, a0=-17.0, a1=197.0)
    out["avental"] = m.objeto()
    transferir(out["avental"], boneco, ossos_da("tronco"))
    for nome, ob in out.items():
        ob.name = f"epi_{nome}"
        ob.data.name = f"epi_{nome}"
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


def ombro(ob, lado, x0=0.165, largura=0.06):
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
    CORES["labio"] = MEDIDAS[sexo]["labio"]
    c = corpo(sexo, boneco)
    prender(c, arm)
    for ob in epis(sexo, boneco).values():
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
        "estados": {},
        "peca": {},
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

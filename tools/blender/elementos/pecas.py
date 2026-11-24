"""Peças e cargas comuns de fábrica, para o campo "Modelo da peça" das origens.

Todas em escala real, com a origem no chão, no centro da base, e um nó só (`corpo`). Quem
desenha a peça ajusta o tamanho pelo lado maior. `dados.tamanho` repete as medidas nominais.
"""

import math

from comum import Elemento

TAU = 2 * math.pi


def _anel(e, no, mat, centro, raio_dentro, raio_fora, y0, y1, lados, liso=True):
    """Anel (tubo) com o eixo em y, feito de `lados` gomos: um prisma por gomo."""
    cx, cz = centro
    for k in range(lados):
        a0, a1 = TAU * k / lados, TAU * (k + 1) / lados
        perfil = [(cx + raio_dentro * math.cos(a0), cz + raio_dentro * math.sin(a0)),
                  (cx + raio_fora * math.cos(a0), cz + raio_fora * math.sin(a0)),
                  (cx + raio_fora * math.cos(a1), cz + raio_fora * math.sin(a1)),
                  (cx + raio_dentro * math.cos(a1), cz + raio_dentro * math.sin(a1))]
        e.prisma(no, mat, perfil, y0, y1, eixo=1, liso=liso)


def engradado(detalhe=0):
    """Engradado plástico empilhável, 0,60 × 0,40 × 0,30 m, aberto em cima, com alças."""
    e = Elemento("engradado", detalhe)
    d = detalhe
    e.material("plastico", 0x2B6CB0, 0.0, 0.55)
    e.material("sombra", 0x1D4A7A, 0.0, 0.7)
    e.no("corpo")
    lx, ly, h, p = 0.30, 0.20, 0.30, 0.02
    e.caixa("corpo", "plastico", (-lx, -ly, 0.0), (lx, ly, p), chanfro=0)
    for s in (-1, 1):
        e.caixa("corpo", "plastico", (-lx, min(s * ly, s * (ly - p)), p), (lx, max(s * ly, s * (ly - p)), h), chanfro=0)
        e.caixa("corpo", "plastico", (min(s * lx, s * (lx - p)), -ly + p, p), (max(s * lx, s * (lx - p)), ly - p, h), chanfro=0)
        # Alça: o rebaixo escuro nas duas cabeceiras.
        x = s * (lx + 0.001)
        e.caixa("corpo", "sombra", (min(x, x - s * 0.002), -0.07, h - 0.09), (max(x, x - s * 0.002), 0.07, h - 0.05), chanfro=0)
    # Borda reforçada em cima e, no detalhe médio, as nervuras das paredes.
    for s in (-1, 1):
        y = s * (ly + 0.008)
        e.caixa("corpo", "plastico", (-lx - 0.008, min(y, s * ly), h - 0.03), (lx + 0.008, max(y, s * ly), h), chanfro=0)
        x = s * (lx + 0.008)
        e.caixa("corpo", "plastico", (min(x, s * lx), -ly, h - 0.03), (max(x, s * lx), ly, h), chanfro=0)
    if d:
        for s in (-1, 1):
            for k in range(5):
                x = -0.24 + k * 0.12
                y = s * (ly + 0.005)
                e.caixa("corpo", "sombra", (x - 0.008, min(y, s * ly), 0.02), (x + 0.008, max(y, s * ly), h - 0.03), chanfro=0)
    e.dados = {"tamanho": [0.60, 0.40, 0.30]}
    return e


def tambor(detalhe=0):
    """Tambor de aço de 200 litros: 0,58 m de diâmetro e 0,88 m de altura, com dois aros."""
    e = Elemento("tambor", detalhe)
    d = detalhe
    e.material("tambor", 0x1F5FA8, 0.4, 0.45)
    lados = 20 if d else 12
    e.no("corpo")
    r, h = 0.285, 0.88
    e.cilindro("corpo", "tambor", (0, 0, 0.0), (0, 0, h), r, lados)
    for z in (0.0, 0.29, 0.585, h - 0.02):  # borda de baixo, dois aros e borda de cima
        e.cilindro("corpo", "tambor", (0, 0, z), (0, 0, z + 0.02), r + 0.006, lados)
    # Tampa rebaixada com os dois bujões.
    e.cilindro("corpo", "aco_escuro", (0, 0, h - 0.004), (0, 0, h + 0.001), r - 0.02, lados)
    e.cilindro("corpo", "aco", (0.17, 0.0, h), (0.17, 0.0, h + 0.012), 0.035, 8)
    e.cilindro("corpo", "aco", (-0.17, 0.05, h), (-0.17, 0.05, h + 0.010), 0.02, 8)
    e.dados = {"tamanho": [0.58, 0.58, 0.88]}
    return e


def bobina(detalhe=0):
    """Bobina de aço deitada (olho na horizontal, eixo em y): 1,20 m de diâmetro, 0,50 m de
    olho e 0,80 m de largura, sobre dois calços de madeira, com as cintas."""
    e = Elemento("bobina", detalhe)
    d = detalhe
    lados = 24 if d else 16
    e.no("corpo")
    ro, ri, w = 0.60, 0.25, 0.40
    cz = ro + 0.004  # as cintas passam 4 mm do aço: a bobina sobe isso para não furar o piso
    _anel(e, "corpo", "aco", (0.0, cz), ri, ro, -w, w, lados)
    # Cintas em volta e os calços.
    for y in ((-0.22, 0.22) if d else (0.0,)):
        _anel(e, "corpo", "aco_escuro", (0.0, cz), ro, ro + 0.004, y - 0.02, y + 0.02, lados)
    for s in (-1, 1):
        x0, x1 = sorted((s * 0.20, s * 0.50))
        alto = x1 if s > 0 else x0
        baixo = x0 if s > 0 else x1
        e.prisma("corpo", "madeira", [(baixo, 0.0), (alto, 0.0), (alto, 0.14)], -w + 0.05, w - 0.05, eixo=1)
    e.dados = {"tamanho": [1.20, 0.80, 1.20], "olho": 0.50}
    return e


def saco(detalhe=0):
    """Saco de 50 kg deitado, 0,70 × 0,45 × 0,16 m, de papel kraft com uma faixa."""
    e = Elemento("saco", detalhe)
    e.material("kraft", 0xC9A36B, 0.0, 0.9)
    e.no("corpo")
    # O miolo cheio e as duas pontas mais finas (a costura).
    e.caixa("corpo", "kraft", (-0.28, -0.225, 0.0), (0.28, 0.225, 0.16), chanfro=0.05)
    for s in (-1, 1):
        x0, x1 = sorted((s * 0.26, s * 0.35))
        e.caixa("corpo", "kraft", (x0, -0.21, 0.03), (x1, 0.21, 0.11), chanfro=0.02)
    e.caixa("corpo", "faixa", (-0.10, -0.227, 0.02), (0.10, 0.227, 0.162), chanfro=0.05)
    e.dados = {"tamanho": [0.70, 0.45, 0.16]}
    return e


def palete_caixas(detalhe=0):
    """Palete carregado: o palete PBR (1,20 × 1,00 m) com duas camadas de quatro caixas de
    papelão e as cintas, 0,96 m de altura."""
    e = Elemento("palete_caixas", detalhe)
    d = detalhe
    e.material("fita", 0xD9C7A3, 0.0, 0.6)
    e.no("corpo")
    for y in (-0.45, 0.0, 0.45):
        e.caixa("corpo", "madeira", (-0.60, y - 0.05, 0.0), (0.60, y + 0.05, 0.022), chanfro=0)
        for x in (-0.55, 0.0, 0.55):
            e.caixa("corpo", "madeira", (x - 0.05, y - 0.05, 0.022), (x + 0.05, y + 0.05, 0.122), chanfro=0)
    e.caixa("corpo", "madeira", (-0.60, -0.50, 0.122), (0.60, 0.50, 0.144), chanfro=0)
    # Caixas de 0,57 × 0,47 × 0,40 m, quatro por camada.
    for camada in range(2):
        z = 0.144 + camada * 0.405
        for sx in (-1, 1):
            for sy in (-1, 1):
                x0, x1 = sorted((sx * 0.005, sx * 0.575))
                y0, y1 = sorted((sy * 0.005, sy * 0.475))
                e.caixa("corpo", "papelao", (x0, y0, z), (x1, y1, z + 0.40))
                if d:
                    xm = (x0 + x1) / 2
                    e.caixa("corpo", "fita", (xm - 0.03, y0 - 0.002, z), (xm + 0.03, y1 + 0.002, z + 0.402), chanfro=0)
    # Duas cintas pretas por cima da carga.
    for y in (-0.24, 0.24):
        e.caixa("corpo", "borracha", (-0.582, y - 0.012, 0.144), (0.582, y + 0.012, 0.958), chanfro=0)
    e.dados = {"tamanho": [1.20, 1.00, 0.96]}
    return e


def engrenagem(detalhe=0):
    """Engrenagem usinada deitada: 0,30 m de diâmetro e 0,06 m de espessura, com cubo e furo."""
    e = Elemento("engrenagem", detalhe)
    d = detalhe
    e.material("interior", 0x1E2125, 0.3, 0.7)
    dentes = 16 if d else 12
    e.no("corpo")
    r, h = 0.125, 0.05
    e.cilindro("corpo", "aco", (0, 0, 0.0), (0, 0, h), r, dentes * 2)
    for k in range(dentes):
        a = TAU * k / dentes
        m = TAU / dentes * 0.22  # meia largura do dente na base
        t = TAU / dentes * 0.12  # e na ponta
        perfil = [(r * 0.98 * math.cos(a - m), r * 0.98 * math.sin(a - m)),
                  (0.15 * math.cos(a - t), 0.15 * math.sin(a - t)),
                  (0.15 * math.cos(a + t), 0.15 * math.sin(a + t)),
                  (r * 0.98 * math.cos(a + m), r * 0.98 * math.sin(a + m))]
        e.prisma("corpo", "aco", perfil, 0.0, h, eixo=2)
    # Cubo mais alto, com o furo do eixo.
    e.cilindro("corpo", "aco", (0, 0, h), (0, 0, 0.06), 0.055, 12)
    e.cilindro("corpo", "interior", (0, 0, 0.0595), (0, 0, 0.0605), 0.025, 12)
    e.dados = {"tamanho": [0.30, 0.30, 0.06]}
    return e

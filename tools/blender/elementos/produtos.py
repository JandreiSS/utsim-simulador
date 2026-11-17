"""Produtos de exemplo para o Ensino: ventilador de mesa, ventilador de coluna e a caixa
embalada de cada um. Escala real, frente em `+x`, origem no chão, no centro da base.

O ventilador tem dois nós: `corpo` (base, haste, motor e grade) e `helice`, que gira em torno
de x no clipe `girar`. A caixa embalada tem um nó só, `corpo`.
"""

import math

from comum import Elemento

TAU = 2 * math.pi
AZUL = 0x2B6CB0


def _anel(e, no, mat, x0, x1, cy, cz, ri, ro, n):
    """Anel no plano yz, centro em (`cy`, `cz`), de `x0` a `x1`: um prisma por trecho."""
    pts = [(math.cos(TAU * k / n), math.sin(TAU * k / n)) for k in range(n + 1)]
    for (ca, sa), (cb, sb) in zip(pts, pts[1:]):
        e.prisma(no, mat, [(cy + ro * ca, cz + ro * sa), (cy + ro * cb, cz + ro * sb),
                           (cy + ri * cb, cz + ri * sb), (cy + ri * ca, cz + ri * sa)], x0, x1, eixo=0)


def _raio(e, no, mat, x0, x1, cy, cz, r0, r1, ang, larg):
    """Vareta da grade: do raio `r0` ao `r1` no ângulo `ang`, com `larg` de largura."""
    c, s = math.cos(ang), math.sin(ang)
    h = larg / 2
    e.prisma(no, mat, [(cy + r0 * c + h * s, cz + r0 * s - h * c), (cy + r1 * c + h * s, cz + r1 * s - h * c),
                       (cy + r1 * c - h * s, cz + r1 * s + h * c), (cy + r0 * c - h * s, cz + r0 * s + h * c)], x0, x1, eixo=0)


def _cabeca(e, d, z, raio, fundo):
    """Motor, grade e hélice de um ventilador com o eixo em x, na altura `z`. A grade vai de
    x = 0 a x = `fundo`; o motor fica atrás."""
    lados = 12 if d else 8
    n = 16 if d else 10
    # Motor atrás da grade.
    e.cilindro("corpo", "pintura", (-fundo * 1.1, 0, z), (0.0, 0, z), raio * 0.30, lados, raio2=raio * 0.36)
    e.cilindro("corpo", "painel", (-fundo * 1.25, 0, z), (-fundo * 1.1, 0, z), raio * 0.22, lados)
    # Grade: aro de trás, aro da frente, aro do meio e varetas na frente e atrás.
    _anel(e, "corpo", "aco", 0.0, 0.008, 0, z, raio - 0.012, raio, n)
    _anel(e, "corpo", "aco", fundo - 0.008, fundo, 0, z, raio - 0.012, raio, n)
    _anel(e, "corpo", "aco", fundo * 0.45, fundo * 0.55, 0, z, raio - 0.004, raio, n)
    varetas = 12 if d else 8
    for k in range(varetas):
        ang = TAU * k / varetas
        _raio(e, "corpo", "aco", fundo - 0.006, fundo, 0, z, raio * 0.22, raio - 0.006, ang, 0.006)
        if d or k % 2 == 0:
            _raio(e, "corpo", "aco", 0.0, 0.006, 0, z, raio * 0.36, raio - 0.006, ang, 0.006)
    # Emblema no centro da grade da frente.
    e.cilindro("corpo", "marca", (fundo - 0.004, 0, z), (fundo + 0.004, 0, z), raio * 0.22, lados)
    # Hélice: cubo e três pás, girando em torno de x.
    e.no("helice", "corpo", (fundo * 0.5, 0.0, z))
    e.cilindro("helice", "painel", (fundo * 0.30, 0, z), (fundo * 0.70, 0, z), raio * 0.16, lados)
    for k in range(3):
        a0 = TAU * k / 3
        pa = []
        for r, da in ((raio * 0.14, -0.10), (raio * 0.60, -0.42), (raio * 0.86, -0.20), (raio * 0.86, 0.22), (raio * 0.50, 0.30), (raio * 0.14, 0.14)):
            pa.append((r * math.cos(a0 + da), z + r * math.sin(a0 + da)))
        e.prisma("helice", "pa", pa, fundo * 0.42, fundo * 0.58, eixo=0)
    e.clipe("girar", True, {"helice": [(0, "rotation_euler", 0, 0.0), (1, "rotation_euler", 0, TAU)]})


def _materiais(e):
    e.material("pintura", 0xECEEF0, 0.1, 0.45)
    e.material("pa", 0x5FA8E0, 0.0, 0.35, alfa=0.80)
    e.material("marca", AZUL, 0.1, 0.4)


def ventilador_mesa(detalhe=0):
    """Ventilador de mesa, 0,45 m de altura, grade de 0,32 m. Nós `corpo` e `helice`; clipe
    `girar`."""
    e = Elemento("ventilador_mesa", detalhe)
    d = detalhe
    _materiais(e)
    lados = 16 if d else 10
    z = 0.28
    e.no("corpo")
    e.cilindro("corpo", "pintura", (-0.04, 0, 0.0), (-0.04, 0, 0.022), 0.115, lados)
    e.cilindro("corpo", "pintura", (-0.04, 0, 0.022), (-0.04, 0, 0.034), 0.115, lados, raio2=0.07)
    e.caixa("corpo", "marca", (0.02, -0.035, 0.022), (0.05, 0.035, 0.032), chanfro=0)  # botões
    e.cilindro("corpo", "pintura", (-0.04, 0, 0.034), (-0.055, 0, z - 0.03), 0.022, 8)
    e.cilindro("corpo", "painel", (-0.055, 0, z - 0.05), (-0.055, 0, z), 0.03, 8)  # articulação
    _cabeca(e, d, z, 0.16, 0.09)
    e.dados = {"altura": 0.44, "diametro": 0.32, "eixo": z}
    return e


def ventilador_coluna(detalhe=0):
    """Ventilador de coluna, 1,20 m de altura, grade de 0,40 m. Nós `corpo` e `helice`; clipe
    `girar`."""
    e = Elemento("ventilador_coluna", detalhe)
    d = detalhe
    _materiais(e)
    lados = 16 if d else 10
    z = 1.0
    e.no("corpo")
    e.cilindro("corpo", "painel", (-0.05, 0, 0.0), (-0.05, 0, 0.03), 0.21, lados)
    e.cilindro("corpo", "painel", (-0.05, 0, 0.03), (-0.05, 0, 0.06), 0.21, lados, raio2=0.05)
    e.cilindro("corpo", "aco", (-0.05, 0, 0.06), (-0.05, 0, 0.62), 0.022, 8)
    e.cilindro("corpo", "pintura", (-0.05, 0, 0.58), (-0.05, 0, 0.64), 0.03, 8)  # trava da altura
    e.cilindro("corpo", "aco", (-0.05, 0, 0.62), (-0.065, 0, z - 0.06), 0.016, 8)
    e.caixa("corpo", "pintura", (-0.10, -0.045, z - 0.14), (-0.03, 0.045, z - 0.04), chanfro=0)  # comando
    e.caixa("corpo", "marca", (-0.03, -0.03, z - 0.12), (-0.025, 0.03, z - 0.06), chanfro=0)
    _cabeca(e, d, z, 0.20, 0.11)
    e.dados = {"altura": 1.20, "diametro": 0.40, "eixo": z}
    return e


def _embalado(nome, detalhe, cx, cy, cz):
    """Caixa de papelão `cx` × `cy` × `cz` com fita, faixa azul e o desenho do ventilador."""
    e = Elemento(nome, detalhe)
    d = detalhe
    e.material("fita", 0xD9C7A3, 0.0, 0.6)
    e.material("marca", AZUL, 0.1, 0.4)
    e.material("pintura", 0xECEEF0, 0.1, 0.45)
    x, y = cx / 2, cy / 2
    e.no("corpo")
    e.caixa("corpo", "papelao", (-x, -y, 0.0), (x, y, cz))
    e.caixa("corpo", "fita", (-x - 0.002, -0.025, 0.0), (x + 0.002, 0.025, cz + 0.002), chanfro=0)
    # Faixa azul em volta, embaixo, e o desenho do ventilador (um disco) nas duas faces largas.
    e.caixa("corpo", "marca", (-x - 0.001, -y - 0.001, cz * 0.10), (x + 0.001, y + 0.001, cz * 0.24), chanfro=0)
    r = min(cx, cz) * 0.24
    for s in (-1, 1):
        yy = s * y
        e.cilindro("corpo", "pintura", (0, yy, cz * 0.60), (0, yy + s * 0.002, cz * 0.60), r, 16 if d else 10)
        e.cilindro("corpo", "marca", (0, yy + s * 0.002, cz * 0.60), (0, yy + s * 0.004, cz * 0.60), r * 0.28, 12 if d else 8)
    e.dados = {"tamanho_caixa": [cx, cy, cz]}
    return e


def ventilador_mesa_caixa(detalhe=0):
    """Ventilador de mesa embalado: caixa de 0,40 × 0,24 × 0,46 m. Nó `corpo`."""
    return _embalado("ventilador_mesa_caixa", detalhe, 0.40, 0.24, 0.46)


def ventilador_coluna_caixa(detalhe=0):
    """Ventilador de coluna embalado (desmontado): caixa de 0,48 × 0,22 × 0,50 m. Nó `corpo`."""
    return _embalado("ventilador_coluna_caixa", detalhe, 0.48, 0.22, 0.50)

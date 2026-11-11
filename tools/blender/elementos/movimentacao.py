"""Esteira de rolos, empilhadeira e AGV.

A empilhadeira e o AGV mantêm os nomes de nós do `utsim-assets` (`chassi`, `luz`,
`torre`, `carro`, `garfos`, `roda_*`), que a interface já procura. A empilhadeira tem
mastro telescópico: os estágios `mastro_m` e `mastro_i` sobem junto com o `carro`.
"""

import math

from comum import Elemento

TAU = 2 * math.pi

# Esteira padrão (m), a mesma medida da `esteira` do `utsim-assets`.
ESTEIRA_COMPRIMENTO = 3.0
ESTEIRA_LARGURA = 0.8
ESTEIRA_ALTURA = 0.76
RAIO_ROLO = 0.03
PASSO_ROLO = 0.15

# Empilhadeira: centro dos garfos em x, garfo em repouso e curso do clipe.
GARFO_CENTRO = 1.555
GARFO_REPOUSO = 0.10
# O curso cobre o vão mais alto do porta-paletes (6,08 m): três estágios de mastro.
GARFO_CURSO = 6.0
MASTRO_TOPO = 2.62
# AGV: altura da mesa e curso do elevador.
AGV_MESA = 0.35
AGV_CURSO = 0.06


def esteira(detalhe=0, comprimento=ESTEIRA_COMPRIMENTO, largura=ESTEIRA_LARGURA, altura=ESTEIRA_ALTURA):
    """Esteira de rolos, peças andando em `+x`. Nós: `estrutura`, `rolo_00`… (cada um
    gira em y). Clipe `rolar`: uma volta por rolo (a fase anda `distância / (2π·raio)`)."""
    e = Elemento("esteira", detalhe)
    c2, w2 = comprimento / 2, largura / 2
    topo = altura - RAIO_ROLO
    e.no("estrutura")
    for s in (-1, 1):
        y0 = s * w2
        e.caixa("estrutura", "faixa", (-c2, min(y0, y0 - s * 0.05), topo - 0.10), (c2, max(y0, y0 - s * 0.05), altura))
    n_pernas = max(2, int(comprimento / 1.5) + 1)
    for k in range(n_pernas):
        x = -c2 + 0.08 + k * (comprimento - 0.16) / (n_pernas - 1)
        for s in (-1, 1):
            y = s * (w2 - 0.06)
            e.caixa("estrutura", "aco_escuro", (x - 0.03, y - 0.03, 0.0), (x + 0.03, y + 0.03, topo - 0.10))
            e.caixa("estrutura", "base", (x - 0.06, y - 0.06, 0.0), (x + 0.06, y + 0.06, 0.02))
        e.caixa("estrutura", "aco_escuro", (x - 0.02, -w2 + 0.06, 0.20), (x + 0.02, w2 - 0.06, 0.24))
    n = int(comprimento / PASSO_ROLO)
    for k in range(n):
        x = -c2 + PASSO_ROLO / 2 + k * PASSO_ROLO
        nome = f"rolo_{k:02d}"
        e.no(nome, "estrutura", (x, 0.0, topo))
        e.cilindro(nome, "aco", (x, -w2 + 0.05, topo), (x, w2 - 0.05, topo), RAIO_ROLO, 8 if detalhe else 6)
    e.ponto("ponto_entrada", (-c2, 0.0, altura), 0)
    e.ponto("ponto_saida", (c2, 0.0, altura), 0)
    e.dados = {"raio_rolo": RAIO_ROLO, "passo_rolo": PASSO_ROLO}
    e.clipe("rolar", True, {f"rolo_{k:02d}": [(0, "rotation_euler", 1, 0.0), (1, "rotation_euler", 1, TAU)]
                            for k in range(n)})
    return e


def _roda(e, nome, x, y, r, largura, pai="chassi"):
    e.no(nome, pai, (x, y, r))
    e.cilindro(nome, "pneu", (x, y - largura / 2, r), (x, y + largura / 2, r), r)
    e.cilindro(nome, "aco", (x, y - largura / 2 - 0.005, r), (x, y + largura / 2 + 0.005, r), r * 0.55, 8)


def _arco(cx, cz, r, a0, a1, n):
    """Pontos de um arco (graus), de `a0` a `a1`, em `n` trechos."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)),
             cz + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def _roda_emp(e, nome, x, y, r, largura):
    """Roda da empilhadeira: pneu redondo, aro claro e cubo."""
    d = e.detalhe
    s = 1 if y > 0 else -1
    e.no(nome, "chassi", (x, y, r))
    e.cilindro(nome, "pneu", (x, y - largura / 2, r), (x, y + largura / 2, r), r, 24 if d else 16)
    e.cilindro(nome, "aro", (x, y + s * (largura / 2 - 0.02), r), (x, y + s * (largura / 2 + 0.006), r), r * 0.58, 16 if d else 10)
    if d:
        e.cilindro(nome, "aco_escuro", (x, y + s * (largura / 2), r), (x, y + s * (largura / 2 + 0.02), r), r * 0.22, 8)


def empilhadeira(detalhe=0):
    """Empilhadeira contrabalançada com mastro telescópico de três estágios. Nós: `chassi`,
    `cabine`, `luz`, `torre` (estágio de fora, fixo), `mastro_m` e `mastro_i` (estágios do
    meio e de dentro), `carro` (filho de `torre`, origem na altura do topo do garfo),
    `garfos`, `roda_de`, `roda_dd`, `roda_te`, `roda_td`. Clipe `garfo_subir`: o carro sobe
    `GARFO_CURSO`, o estágio do meio sobe um terço disso e o de dentro, dois terços."""
    e = Elemento("empilhadeira", detalhe)
    d = detalhe
    e.material("pintura", 0xF2B705, 0.1, 0.55)
    e.material("contrapeso", 0x2E3135, 0.3, 0.6)
    e.material("aro", 0xD9DCE0, 0.4, 0.5)
    e.material("gas", 0xC4CAD1, 0.5, 0.4)
    e.material("farol", 0xF6F2DC, 0.0, 0.3)
    e.no("chassi")
    # Contrapeso: traseira arredondada (vista de cima) sobre um miolo mais estreito.
    n = 3 if d else 1
    contorno = [(-0.78, -0.56)]
    contorno += _arco(-1.14, -0.30, 0.26, 270, 180, n)
    contorno += _arco(-1.14, 0.30, 0.26, 180, 90, n)
    contorno += [(-0.78, 0.56)]
    e.prisma("chassi", "contrapeso", contorno, 0.52, 1.10, eixo=2)
    if d:
        e.prisma("chassi", "contrapeso", [(-0.80 + (x + 0.80) * 0.93, y * 0.93) for x, y in contorno], 1.10, 1.15, eixo=2)
        for y0, y1 in ((0.30, 0.44), (-0.44, -0.30)):
            e.caixa("chassi", "vermelho", (-1.412, y0, 0.90), (-1.39, y1, 0.97), chanfro=0)
    e.caixa("chassi", "contrapeso", (-1.36, -0.28, 0.20), (-0.78, 0.28, 0.52))
    # Corpo: laterais, miolo entre as rodas da frente, capô sob o banco e painel.
    e.caixa("chassi", "pintura", (-0.80, -0.52, 0.40), (0.08, 0.52, 0.66), chanfro=0.02)
    e.caixa("chassi", "contrapeso", (-0.80, -0.30, 0.18), (0.08, 0.30, 0.40))
    e.caixa("chassi", "contrapeso", (0.08, -0.33, 0.24), (0.78, 0.33, 0.66))
    e.caixa("chassi", "borracha", (0.08, -0.33, 0.66), (0.50, 0.33, 0.68), chanfro=0)
    e.prisma("chassi", "pintura", [(-0.80, 0.66), (0.02, 0.66), (0.02, 0.92), (-0.10, 0.98), (-0.80, 0.98)], -0.44, 0.44)
    e.prisma("chassi", "pintura", [(0.50, 0.66), (0.78, 0.66), (0.78, 1.02), (0.70, 1.14), (0.56, 1.14), (0.50, 1.00)], -0.44, 0.44)
    e.caixa("chassi", "tela", (0.57, -0.20, 1.14), (0.69, 0.20, 1.146), chanfro=0)
    # Para-lamas em arco sobre as rodas da frente.
    n = 6 if d else 3
    arco = _arco(0.45, 0.33, 0.43, 10, 170, n) + list(reversed(_arco(0.45, 0.33, 0.38, 10, 170, n)))
    for y0, y1 in ((0.33, 0.58), (-0.58, -0.33)):
        e.prisma("chassi", "pintura", arco, y0, y1)
    # Volante, alavancas, degraus e os cilindros que inclinam o mastro.
    e.cilindro("chassi", "aco_escuro", (0.58, 0.0, 1.10), (0.38, 0.0, 1.30), 0.028, 8)
    e.cilindro("chassi", "borracha", (0.385, 0.0, 1.295), (0.365, 0.0, 1.315), 0.17, 16 if d else 10)
    for y in (-0.30, 0.30):
        e.cilindro("chassi", "aco", (0.56, y, 0.74), (0.82, y, 0.98), 0.03, 8)
    if d:
        for y in (-0.26, -0.33):
            e.cilindro("chassi", "aco_escuro", (0.62, y, 1.12), (0.54, y, 1.30), 0.012, 6)
            e.cilindro("chassi", "vermelho", (0.54, y, 1.30), (0.53, y, 1.33), 0.022, 6)
        for y0, y1 in ((0.52, 0.62), (-0.62, -0.52)):
            e.caixa("chassi", "contrapeso", (-0.30, y0, 0.34), (0.04, y1, 0.38), chanfro=0)
    # Botijão de gás deitado sobre o contrapeso.
    e.cilindro("chassi", "gas", (-1.08, -0.34, 1.31), (-1.08, 0.34, 1.31), 0.15, 16 if d else 10)
    if d:
        e.cilindro("chassi", "gas", (-1.08, 0.34, 1.31), (-1.08, 0.40, 1.31), 0.09, 8)
        for y in (-0.20, 0.20):
            e.caixa("chassi", "aco_escuro", (-1.24, y - 0.025, 1.14), (-0.92, y + 0.025, 1.19), chanfro=0)
    # Cabine: banco e grade de proteção em tubos.
    e.no("cabine", pivo=(0.0, 0.0, 0.95))
    e.caixa("cabine", "borracha", (-0.64, -0.24, 0.98), (-0.16, 0.24, 1.06), chanfro=0.02)
    e.prisma("cabine", "borracha", [(-0.70, 1.04), (-0.60, 1.04), (-0.66, 1.58), (-0.75, 1.58)], -0.24, 0.24)
    lados = 8 if d else 6
    for y in (-0.50, 0.50):
        e.cilindro("cabine", "aco_escuro", (0.74, y * 0.94, 0.70), (0.50, y, 2.14), 0.028, lados)
        e.cilindro("cabine", "aco_escuro", (-0.84, y, 1.08), (-0.78, y, 2.14), 0.028, lados)
        e.cilindro("cabine", "aco_escuro", (-0.80, y, 2.14), (0.52, y, 2.14), 0.028, lados)
    for x in (-0.78, 0.50):
        e.cilindro("cabine", "aco_escuro", (x, -0.50, 2.14), (x, 0.50, 2.14), 0.028, lados)
    for k in range(5 if d else 3):
        y = (-0.34 + k * 0.17) if d else (-0.26 + k * 0.26)
        e.caixa("cabine", "aco_escuro", (-0.78, y - 0.02, 2.13), (0.50, y + 0.02, 2.15), chanfro=0)
    if d:
        for y in (-0.50, 0.50):
            e.cilindro("cabine", "aco_escuro", (0.60, y, 1.55), (0.66, y, 1.55), 0.06, 10)
            e.cilindro("cabine", "farol", (0.66, y, 1.55), (0.67, y, 1.55), 0.05, 10)
        e.cilindro("cabine", "aco_escuro", (-0.70, 0.0, 2.14), (-0.70, 0.0, 2.17), 0.07, 10)
    e.material("luz", 0xFF9D00, 0.0, 0.4, emissao=True)
    e.no("luz", "cabine", (-0.70, 0.0, 2.17))
    e.cilindro("luz", "luz", (-0.70, 0.0, 2.17), (-0.70, 0.0, 2.27), 0.05)
    # Mastro de três estágios. O de fora (`torre`) é fixo; os outros sobem com o garfo.
    topo = MASTRO_TOPO
    e.no("torre", pivo=(0.85, 0.0, 0.10))
    for y in (-0.37, 0.37):
        e.caixa("torre", "aco_escuro", (0.79, y - 0.04, 0.12), (0.91, y + 0.04, topo))
    for z0, z1 in ((0.12, 0.24), (1.30, 1.40), (topo - 0.14, topo)):
        e.caixa("torre", "aco_escuro", (0.765, -0.41, z0), (0.80, 0.41, z1), chanfro=0)
    for y in (-0.13, 0.13):
        e.cilindro("torre", "aco_escuro", (0.82, y, 0.14), (0.82, y, 2.30), 0.04, 8)
    e.no("mastro_m", "torre", (0.85, 0.0, 0.10))
    for y in (-0.295, 0.295):
        e.caixa("mastro_m", "aco", (0.80, y - 0.03, 0.14), (0.90, y + 0.03, topo), chanfro=0)
    e.caixa("mastro_m", "aco", (0.80, -0.325, topo - 0.10), (0.84, 0.325, topo), chanfro=0)
    for y in (-0.13, 0.13):
        e.cilindro("mastro_m", "aro", (0.82, y, 0.25), (0.82, y, topo - 0.08), 0.022, 6)
    e.no("mastro_i", "torre", (0.85, 0.0, 0.10))
    for y in (-0.235, 0.235):
        e.caixa("mastro_i", "aco_escuro", (0.81, y - 0.025, 0.16), (0.89, y + 0.025, topo), chanfro=0)
    for z0, z1 in ((0.16, 0.24), (topo - 0.08, topo)):
        e.caixa("mastro_i", "aco_escuro", (0.845, -0.26, z0), (0.885, 0.26, z1), chanfro=0)
    if d:
        for y in (-0.07, 0.07):
            e.caixa("mastro_i", "aco", (0.893, y - 0.012, 0.40), (0.905, y + 0.012, topo - 0.04), chanfro=0)
    # Carro porta-garfos com o encosto da carga.
    r0 = GARFO_REPOUSO
    e.no("carro", "torre", (0.95, 0.0, r0))
    for y in (-0.23, 0.23):
        e.caixa("carro", "aco_escuro", (0.905, y - 0.03, r0 + 0.02), (0.955, y + 0.03, r0 + 0.52), chanfro=0)
    for z in (r0 + 0.04, r0 + 0.38):
        e.caixa("carro", "aco_escuro", (0.93, -0.50, z), (0.98, 0.50, z + 0.08))
    for y in (-0.485, 0.485):
        e.caixa("carro", "aco_escuro", (0.94, y - 0.015, r0 + 0.46), (0.97, y + 0.015, r0 + 1.15), chanfro=0)
    e.caixa("carro", "aco_escuro", (0.94, -0.50, r0 + 1.12), (0.97, 0.50, r0 + 1.15), chanfro=0)
    for y in ((-0.29, -0.10, 0.10, 0.29) if d else (-0.16, 0.16)):
        e.caixa("carro", "aco_escuro", (0.945, y - 0.01, r0 + 0.46), (0.965, y + 0.01, r0 + 1.12), chanfro=0)
    e.no("garfos", "carro", (0.98, 0.0, r0))
    lamina = [(0.98, r0 - 0.045), (1.90, r0 - 0.045), (2.13, r0 - 0.012), (2.13, r0), (0.98, r0)]
    for y in (-0.25, 0.25):
        e.prisma("garfos", "borracha", lamina, y - 0.05, y + 0.05)
        e.caixa("garfos", "borracha", (0.98, y - 0.05, r0), (1.03, y + 0.05, r0 + 0.50), chanfro=0)
    e.ponto("ponto_carga", (GARFO_CENTRO, 0.0, GARFO_REPOUSO), 0, pai="garfos")
    _roda_emp(e, "roda_de", 0.45, 0.46, 0.33, 0.22)
    _roda_emp(e, "roda_dd", 0.45, -0.46, 0.33, 0.22)
    _roda_emp(e, "roda_te", -0.95, 0.40, 0.25, 0.18)
    _roda_emp(e, "roda_td", -0.95, -0.40, 0.25, 0.18)
    e.ponto("ponto_operador", (-0.40, 0.0, 1.06), 0, pai="cabine")
    e.dados = {"garfo_repouso": GARFO_REPOUSO, "garfo_curso": GARFO_CURSO, "garfo_centro": GARFO_CENTRO}
    # Ao carregar, o garfo tira o palete do chão (0,10 m); no porta-paletes a ponte usa a
    # altura do vão (fase = altura / `garfo_curso`).
    e.dados["levante"] = round(0.10 / GARFO_CURSO, 6)
    e.clipe("garfo_subir", False, {
        "carro": [(0, "location", 2, 0.0), (1, "location", 2, GARFO_CURSO)],
        "mastro_i": [(0, "location", 2, 0.0), (1, "location", 2, GARFO_CURSO * 2 / 3)],
        "mastro_m": [(0, "location", 2, 0.0), (1, "location", 2, GARFO_CURSO / 3)],
    })
    return e


def agv(detalhe=0):
    """AGV com mesa elevatória. Nós: `chassi`, `mesa` (sobe `AGV_CURSO`), `luz`,
    `roda_e`, `roda_d`. Clipe `mesa_subir`."""
    e = Elemento("agv", detalhe)
    e.material("pintura", 0xDFE3E8, 0.1, 0.5)
    e.no("chassi")
    e.caixa("chassi", "borracha", (-0.50, -0.34, 0.02), (0.50, 0.34, 0.12))
    e.caixa("chassi", "pintura", (-0.55, -0.40, 0.12), (0.55, 0.40, AGV_MESA - 0.03))
    for s, mat in ((1, "amarelo"), (-1, "borracha")):
        x = s * 0.555
        e.caixa("chassi", mat, (x - 0.015, -0.38, 0.10), (x + 0.015, 0.38, 0.18))
    e.caixa("chassi", "tela", (0.553, -0.15, 0.24), (0.565, 0.15, 0.29))
    for x in (-0.42, 0.42):
        for y in (-0.28, 0.28):
            e.cilindro("chassi", "pneu", (x, y - 0.02, 0.025), (x, y + 0.02, 0.025), 0.025, 8)
    e.no("mesa", pivo=(0.0, 0.0, AGV_MESA - 0.03))
    e.caixa("mesa", "borracha", (-0.50, -0.36, AGV_MESA - 0.03), (0.50, 0.36, AGV_MESA))
    for y in (-0.12, 0.12):
        e.caixa("mesa", "amarelo", (0.10, y - 0.02, AGV_MESA), (0.30, y + 0.02, AGV_MESA + 0.003))
    e.ponto("ponto_carga", (0.0, 0.0, AGV_MESA), 0, pai="mesa")
    e.material("luz", 0xE8F4FF, 0.0, 0.4, emissao=True)
    e.no("luz", pivo=(0.0, 0.0, 0.22))
    e.caixa("luz", "luz", (-0.556, -0.40, 0.20), (0.556, 0.40, 0.23))
    _roda(e, "roda_e", 0.0, 0.33, 0.06, 0.05)
    _roda(e, "roda_d", 0.0, -0.33, 0.06, 0.05)
    e.dados = {"mesa": AGV_MESA, "mesa_curso": AGV_CURSO}
    e.clipe("mesa_subir", False, {"mesa": [(0, "location", 2, 0.0), (1, "location", 2, AGV_CURSO)]})
    return e

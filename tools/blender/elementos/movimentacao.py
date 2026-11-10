"""Esteira de rolos, empilhadeira e AGV.

A empilhadeira e o AGV mantêm os nomes de nós do `utsim-assets` (`chassi`, `luz`,
`torre`, `carro`, `garfos`, `roda_*`), que a interface já procura.
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
GARFO_CURSO = 2.0
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


def empilhadeira(detalhe=0):
    """Empilhadeira contrabalançada. Nós: `chassi`, `cabine`, `luz`, `torre`, `carro`
    (filho de `torre`, origem na altura do topo do garfo), `garfos`, `roda_de`,
    `roda_dd`, `roda_te`, `roda_td`. Clipe `garfo_subir`: o carro sobe `GARFO_CURSO`."""
    e = Elemento("empilhadeira", detalhe)
    e.material("pintura", 0xF2B705, 0.1, 0.55)
    e.material("contrapeso", 0x2E3135, 0.3, 0.6)
    e.no("chassi")
    e.caixa("chassi", "contrapeso", (-1.22, -0.28, 0.10), (0.70, 0.28, 0.60))
    e.caixa("chassi", "pintura", (-1.25, -0.55, 0.60), (0.75, 0.55, 0.95))
    e.caixa("chassi", "contrapeso", (-1.30, -0.55, 0.55), (-0.82, 0.55, 1.15))
    e.caixa("chassi", "pintura", (0.35, -0.45, 0.95), (0.75, 0.45, 1.12))
    for y0, y1 in ((0.30, 0.55), (-0.55, -0.30)):
        e.caixa("chassi", "pintura", (0.20, y0, 0.66), (0.72, y1, 0.70))
    e.cilindro("chassi", "aco_escuro", (0.32, 0.0, 1.10), (0.22, 0.0, 1.38), 0.03, 8)
    e.cilindro("chassi", "borracha", (0.24, 0.0, 1.37), (0.20, 0.0, 1.41), 0.17)
    e.no("cabine", pivo=(0.0, 0.0, 0.95))
    e.caixa("cabine", "borracha", (-0.62, -0.25, 0.95), (-0.18, 0.25, 1.06))
    e.caixa("cabine", "borracha", (-0.66, -0.25, 1.06), (-0.56, 0.25, 1.55))
    for x in (-0.80, 0.55):
        for y in (-0.50, 0.50):
            e.caixa("cabine", "aco_escuro", (x - 0.03, y - 0.03, 0.95), (x + 0.03, y + 0.03, 2.10))
    e.caixa("cabine", "aco_escuro", (-0.83, -0.53, 2.10), (0.58, 0.53, 2.14))
    for k in range(5):
        y = -0.40 + k * 0.20
        e.caixa("cabine", "aco_escuro", (-0.80, y - 0.02, 2.14), (0.55, y + 0.02, 2.16))
    e.material("luz", 0xFF9D00, 0.0, 0.4, emissao=True)
    e.no("luz", "cabine", (-0.70, 0.0, 2.16))
    e.cilindro("luz", "luz", (-0.70, 0.0, 2.16), (-0.70, 0.0, 2.25), 0.05)
    # Torre (mastro) e carro com os garfos.
    e.no("torre", pivo=(0.85, 0.0, 0.10))
    for y in (-0.33, 0.33):
        e.caixa("torre", "aco_escuro", (0.80, y - 0.05, 0.10), (0.90, y + 0.05, 2.20))
    e.caixa("torre", "aco_escuro", (0.80, -0.33, 2.12), (0.90, 0.33, 2.20))
    e.cilindro("torre", "aco", (0.86, 0.0, 0.40), (0.86, 0.0, 1.60), 0.04, 8)
    e.no("carro", "torre", (0.95, 0.0, GARFO_REPOUSO))
    e.caixa("carro", "aco_escuro", (0.92, -0.45, GARFO_REPOUSO), (0.97, 0.45, GARFO_REPOUSO + 0.55))
    for k in range(4):
        z = GARFO_REPOUSO + 0.10 + k * 0.12
        e.caixa("carro", "aco_escuro", (0.95, -0.45, z), (0.98, 0.45, z + 0.03))
    e.no("garfos", "carro", (0.98, 0.0, GARFO_REPOUSO))
    for y in (-0.25, 0.25):
        e.caixa("garfos", "borracha", (0.98, y - 0.05, GARFO_REPOUSO - 0.045), (2.13, y + 0.05, GARFO_REPOUSO))
        e.caixa("garfos", "borracha", (0.98, y - 0.05, GARFO_REPOUSO), (1.02, y + 0.05, GARFO_REPOUSO + 0.45))
    e.ponto("ponto_carga", (GARFO_CENTRO, 0.0, GARFO_REPOUSO), 0, pai="garfos")
    _roda(e, "roda_de", 0.45, 0.43, 0.33, 0.22)
    _roda(e, "roda_dd", 0.45, -0.43, 0.33, 0.22)
    _roda(e, "roda_te", -0.95, 0.40, 0.25, 0.18)
    _roda(e, "roda_td", -0.95, -0.40, 0.25, 0.18)
    e.ponto("ponto_operador", (-0.40, 0.0, 1.06), 0, pai="cabine")
    e.dados = {"garfo_repouso": GARFO_REPOUSO, "garfo_curso": GARFO_CURSO, "garfo_centro": GARFO_CENTRO}
    e.clipe("garfo_subir", False, {"carro": [(0, "location", 2, 0.0), (1, "location", 2, GARFO_CURSO)]})
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

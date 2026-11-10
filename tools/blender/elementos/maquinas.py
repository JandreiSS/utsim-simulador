"""Máquinas: torno CNC, prensa e injetora.

Lado de trabalho em `-y` (porta e painel), como a `maquina` do `utsim-assets`.
Clipes: `ciclo` (laço, processando), `porta_abrir` (0 fechada, 1 aberta), `setup`
(laço, preparando) e `quebra` (0 normal, 1 tampa de manutenção aberta).
"""

import math

from comum import Elemento

TAU = 2 * math.pi


def _torre_luz(e, x, y, z):
    e.cilindro("chassi", "painel", (x, y, z), (x, y, z + 0.15), 0.02, 8)
    e.material("luz", 0x46D160, 0.0, 0.4, emissao=True)
    e.no("luz", pivo=(x, y, z + 0.15))
    e.cilindro("luz", "luz", (x, y, z + 0.15), (x, y, z + 0.35), 0.055)


def _painel(e, x0, x1, z0, z1, y):
    """Painel de comando no lado de trabalho, com tela e três botões."""
    e.caixa("chassi", "painel", (x0, y - 0.12, z0), (x1, y, z1))
    e.caixa("chassi", "tela", (x0 + 0.04, y - 0.125, (z0 + z1) / 2), (x1 - 0.04, y - 0.12, z1 - 0.05))
    for k, cor in enumerate(("verde", "vermelho", "amarelo")):
        xb = x0 + 0.07 + k * 0.09
        e.cilindro("chassi", cor, (xb, y - 0.12, z0 + 0.08), (xb, y - 0.14, z0 + 0.08), 0.022, 8)


def cnc(detalhe=0):
    """Torno CNC fechado, 2,0 × 1,5 × 2,2 m. Nós: `chassi`, `porta` (corrediça),
    `fuso` (placa girando em x), `torre` (torre de ferramentas), `tampa` (manutenção,
    lado `+x`), `luz`."""
    e = Elemento("cnc", detalhe)
    e.no("chassi")
    e.material("vidro", 0x6F8FA8, 0.1, 0.15, alfa=0.45)
    e.material("interior", 0x1E2125, 0.3, 0.7)
    # Base e corpo em volta da câmara de trabalho (x -0.55..0.25, z 0.60..1.60).
    e.caixa("chassi", "base", (-1.0, -0.75, 0.0), (1.0, 0.75, 0.15))
    e.caixa("chassi", "pintura", (-0.95, -0.70, 0.15), (0.95, 0.70, 0.60))
    e.caixa("chassi", "pintura", (-0.95, -0.70, 1.60), (0.95, 0.70, 1.85))
    e.caixa("chassi", "pintura", (-0.95, -0.70, 0.60), (-0.55, 0.70, 1.60))
    e.caixa("chassi", "pintura", (0.25, -0.70, 0.60), (0.95, 0.70, 1.60))
    e.caixa("chassi", "interior", (-0.55, -0.20, 0.60), (0.25, 0.70, 1.60))
    e.caixa("chassi", "faixa", (-0.96, -0.71, 1.62), (0.96, 0.71, 1.70))
    e.caixa("chassi", "base", (-0.90, -0.65, 1.85), (0.90, 0.65, 1.95))
    # Esteira de cavaco atrás, à direita.
    e.caixa("chassi", "aco_escuro", (0.55, 0.70, 0.15), (0.95, 1.00, 0.90))
    _painel(e, 0.45, 0.90, 0.95, 1.50, -0.70)
    # Placa (fuso) e castanhas, girando em torno de x.
    e.no("fuso", pivo=(-0.55, -0.45, 1.10))
    e.cilindro("fuso", "aco", (-0.55, -0.45, 1.10), (-0.40, -0.45, 1.10), 0.13)
    for k in range(3):
        a = TAU * k / 3
        cy, cz = -0.45 + 0.08 * math.cos(a), 1.10 + 0.08 * math.sin(a)
        e.caixa("fuso", "aco_escuro", (-0.40, cy - 0.025, cz - 0.025), (-0.36, cy + 0.025, cz + 0.025))
    # Torre de ferramentas, à direita da câmara.
    e.no("torre", pivo=(0.05, -0.45, 1.10))
    e.caixa("torre", "aco_escuro", (0.0, -0.60, 0.95), (0.20, -0.30, 1.25))
    e.caixa("torre", "aco", (-0.12, -0.47, 1.08), (0.0, -0.43, 1.12))
    # Porta corrediça com visor.
    e.no("porta", pivo=(-0.15, -0.72, 0.60))
    e.caixa("porta", "pintura", (-0.57, -0.76, 0.58), (0.27, -0.72, 0.68))
    e.caixa("porta", "pintura", (-0.57, -0.76, 1.52), (0.27, -0.72, 1.62))
    e.caixa("porta", "pintura", (-0.57, -0.76, 0.68), (-0.49, -0.72, 1.52))
    e.caixa("porta", "pintura", (0.19, -0.76, 0.68), (0.27, -0.72, 1.52))
    e.caixa("porta", "vidro", (-0.49, -0.745, 0.68), (0.19, -0.735, 1.52))
    e.caixa("porta", "aco", (0.12, -0.80, 1.00), (0.15, -0.76, 1.25))  # puxador
    # Tampa de manutenção no lado +x (dobradiça vertical na aresta da frente).
    e.no("tampa", pivo=(0.95, -0.55, 0.30))
    e.caixa("tampa", "pintura", (0.95, -0.55, 0.30), (0.98, 0.40, 1.40))
    e.caixa("tampa", "faixa", (0.98, -0.45, 1.20), (0.99, -0.30, 1.30))
    _torre_luz(e, 0.75, 0.45, 1.95)
    # Pontos.
    e.ponto("ponto_operador", (-0.15, -1.20, 0.0), 90)
    e.ponto("ponto_porta", (-0.15, -0.85, 0.0), 90)
    e.ponto("ponto_peca", (-0.30, -0.45, 1.10), 0)
    e.ponto("ponto_entrada", (-1.30, -0.95, 0.90), 0)
    e.ponto("ponto_saida", (1.30, -0.95, 0.90), 0)
    # Clipes.
    e.clipe("ciclo", True, {
        "fuso": [(0, "rotation_euler", 0, 0.0), (1, "rotation_euler", 0, 3 * TAU)],
        "torre": [(0, "location", 0, 0.0), (0.2, "location", 0, 0.0), (0.45, "location", 0, -0.12),
                  (0.8, "location", 0, -0.12), (1, "location", 0, 0.0)],
    })
    e.clipe("porta_abrir", False, {"porta": [(0, "location", 0, 0.0), (1, "location", 0, -0.80)]})
    e.clipe("setup", True, {
        "torre": [(0, "rotation_euler", 0, 0.0), (0.3, "rotation_euler", 0, 0.0),
                  (0.7, "rotation_euler", 0, math.pi / 2), (1, "rotation_euler", 0, math.pi / 2)],
    })
    e.clipe("quebra", False, {"tampa": [(0, "rotation_euler", 2, 0.0), (1, "rotation_euler", 2, math.radians(-100))]})
    return e


def prensa(detalhe=0):
    """Prensa de coluna, 1,4 × 1,3 × 2,7 m. Nós: `chassi`, `martelo` (desce em z),
    `volante` (gira em y), `porta` (grade frontal, sobe), `matriz` (sai em `-y` no
    setup), `tampa` (do volante), `luz`."""
    e = Elemento("prensa", detalhe)
    e.no("chassi")
    e.material("interior", 0x1E2125, 0.3, 0.7)
    e.caixa("chassi", "base", (-0.70, -0.60, 0.0), (0.70, 0.65, 0.15))
    e.caixa("chassi", "pintura", (-0.60, -0.40, 0.15), (0.60, 0.55, 0.85))  # mesa
    e.caixa("chassi", "aco_escuro", (-0.45, -0.45, 0.85), (0.45, 0.20, 0.92))  # placa
    for x in (-0.60, 0.35):
        e.caixa("chassi", "pintura", (x, 0.15, 0.85), (x + 0.25, 0.55, 2.10))  # colunas
    e.caixa("chassi", "pintura", (-0.62, -0.20, 2.10), (0.62, 0.60, 2.55))  # coroa
    e.caixa("chassi", "faixa", (-0.63, -0.21, 2.40), (0.63, 0.61, 2.46))
    e.caixa("chassi", "interior", (-0.35, 0.15, 0.92), (0.35, 0.40, 2.10))
    # Cortina de luz (postes amarelos) na frente.
    for x in (-0.66, 0.66):
        e.caixa("chassi", "amarelo", (x - 0.03, -0.55, 0.85), (x + 0.03, -0.49, 1.90))
    _painel(e, 0.70, 1.0, 1.00, 1.45, -0.20)
    # Matriz sobre a placa.
    e.no("matriz", pivo=(0.0, -0.10, 0.92))
    e.caixa("matriz", "aco", (-0.30, -0.35, 0.92), (0.30, 0.15, 1.05))
    # Martelo (desce até a matriz).
    e.no("martelo", pivo=(0.0, -0.10, 1.45))
    e.caixa("martelo", "aco_escuro", (-0.33, -0.38, 1.45), (0.33, 0.18, 1.70))
    e.caixa("martelo", "aco", (-0.28, -0.33, 1.32), (0.28, 0.13, 1.45))
    e.cilindro("martelo", "aco", (0.0, -0.10, 1.70), (0.0, -0.10, 2.10), 0.08)
    # Volante na lateral +x da coroa, com tampa.
    e.no("volante", pivo=(0.66, 0.20, 2.30))
    e.cilindro("volante", "aco_escuro", (0.64, 0.20, 2.30), (0.74, 0.20, 2.30), 0.24)
    e.caixa("volante", "amarelo", (0.745, 0.17, 2.36), (0.75, 0.23, 2.52))
    e.no("tampa", pivo=(0.80, -0.10, 2.05))
    e.caixa("tampa", "pintura", (0.78, -0.10, 2.05), (0.82, 0.50, 2.58))
    # Grade frontal (porta): sobe para trocar a matriz.
    e.no("porta", pivo=(0.0, -0.52, 0.95))
    e.caixa("porta", "amarelo", (-0.60, -0.54, 0.95), (0.60, -0.50, 1.00))
    e.caixa("porta", "amarelo", (-0.60, -0.54, 1.40), (0.60, -0.50, 1.45))
    for k in range(7):
        x = -0.55 + k * 0.183
        e.caixa("porta", "aco_escuro", (x - 0.008, -0.53, 1.00), (x + 0.008, -0.51, 1.40))
    _torre_luz(e, 0.45, 0.45, 2.55)
    e.ponto("ponto_operador", (0.0, -1.00, 0.0), 90)
    e.ponto("ponto_porta", (0.0, -0.75, 0.0), 90)
    e.ponto("ponto_peca", (0.0, -0.10, 1.06), 0)
    e.ponto("ponto_entrada", (-1.0, -0.40, 0.95), 0)
    e.ponto("ponto_saida", (1.0, -0.40, 0.95), 0)
    e.clipe("ciclo", True, {
        "martelo": [(0, "location", 2, 0.0), (0.3, "location", 2, 0.0), (0.45, "location", 2, -0.26),
                    (0.55, "location", 2, -0.26), (0.85, "location", 2, 0.0), (1, "location", 2, 0.0)],
        "volante": [(0, "rotation_euler", 0, 0.0), (1, "rotation_euler", 0, 2 * TAU)],
    })
    e.clipe("porta_abrir", False, {"porta": [(0, "location", 2, 0.0), (1, "location", 2, 0.55)]})
    e.clipe("setup", True, {
        "matriz": [(0, "location", 1, 0.0), (0.2, "location", 1, 0.0), (0.5, "location", 1, -0.30),
                   (0.6, "location", 1, -0.30), (0.9, "location", 1, 0.0), (1, "location", 1, 0.0)],
    })
    e.clipe("quebra", False, {"tampa": [(0, "rotation_euler", 2, 0.0), (1, "rotation_euler", 2, math.radians(-95))]})
    return e


def injetora(detalhe=0):
    """Injetora horizontal, 4,0 × 1,3 × 2,0 m. Nós: `chassi`, `molde` (placa móvel,
    anda em x), `unidade` (unidade de injeção, recua no setup), `porta` (corrediça de
    segurança), `tampa` (sobre a unidade de injeção), `luz`."""
    e = Elemento("injetora", detalhe)
    e.no("chassi")
    e.material("vidro", 0x6F8FA8, 0.1, 0.15, alfa=0.45)
    e.caixa("chassi", "base", (-2.0, -0.55, 0.0), (2.0, 0.55, 0.15))
    e.caixa("chassi", "pintura", (-1.95, -0.50, 0.15), (1.95, 0.50, 0.95))
    e.caixa("chassi", "faixa", (-1.96, -0.51, 0.80), (1.96, 0.51, 0.88))
    # Placa fixa, placa traseira, colunas.
    e.caixa("chassi", "aco_escuro", (0.0, -0.45, 0.95), (0.15, 0.45, 1.75))
    e.caixa("chassi", "aco_escuro", (-1.75, -0.45, 0.95), (-1.60, 0.45, 1.75))
    for y in (-0.33, 0.33):
        for z in (1.07, 1.63):
            e.cilindro("chassi", "aco", (-1.75, y, z), (0.15, y, z), 0.04)
    _painel(e, 0.30, 0.80, 1.00, 1.50, -0.50)
    # Molde móvel.
    e.no("molde", pivo=(-0.40, 0.0, 0.95))
    e.caixa("molde", "aco_escuro", (-0.55, -0.42, 0.95), (-0.40, 0.42, 1.75))
    e.caixa("molde", "aco", (-0.40, -0.25, 1.15), (-0.30, 0.25, 1.55))
    e.cilindro("molde", "aco", (-1.60, 0.0, 1.35), (-0.55, 0.0, 1.35), 0.09)
    # Unidade de injeção: cilindro, funil e capô.
    e.no("unidade", pivo=(0.15, 0.0, 1.35))
    e.cilindro("unidade", "aco", (0.15, 0.0, 1.35), (1.70, 0.0, 1.35), 0.11)
    e.caixa("unidade", "pintura", (1.10, -0.30, 1.05), (1.90, 0.30, 1.30))
    e.cilindro("unidade", "aco", (0.90, 0.0, 1.46), (0.90, 0.0, 1.70), 0.06)
    e.cilindro("unidade", "laranja", (0.90, 0.0, 1.70), (0.90, 0.0, 1.98), 0.10, raio2=0.22)
    e.no("tampa", pivo=(1.10, 0.30, 1.30), pai="unidade")
    e.caixa("tampa", "pintura", (0.20, -0.30, 1.30), (1.10, 0.30, 1.38))
    # Porta de segurança sobre a área do molde.
    e.no("porta", pivo=(-0.80, -0.56, 0.95))
    e.caixa("porta", "pintura", (-1.55, -0.60, 0.95), (-0.05, -0.56, 1.02))
    e.caixa("porta", "pintura", (-1.55, -0.60, 1.78), (-0.05, -0.56, 1.85))
    e.caixa("porta", "pintura", (-1.55, -0.60, 1.02), (-1.48, -0.56, 1.78))
    e.caixa("porta", "pintura", (-0.12, -0.60, 1.02), (-0.05, -0.56, 1.78))
    e.caixa("porta", "vidro", (-1.48, -0.585, 1.02), (-0.12, -0.575, 1.78))
    _torre_luz(e, -1.80, 0.40, 0.95)
    e.ponto("ponto_operador", (-0.80, -1.05, 0.0), 90)
    e.ponto("ponto_porta", (-0.80, -0.75, 0.0), 90)
    e.ponto("ponto_peca", (-0.15, 0.0, 1.35), 0)
    e.ponto("ponto_entrada", (2.30, 0.0, 1.0), 180)
    e.ponto("ponto_saida", (-0.20, -0.85, 0.90), -90)
    e.clipe("ciclo", True, {
        "molde": [(0, "location", 0, 0.0), (0.2, "location", 0, 0.24), (0.7, "location", 0, 0.24),
                  (0.9, "location", 0, 0.0), (1, "location", 0, 0.0)],
    })
    e.clipe("porta_abrir", False, {"porta": [(0, "location", 0, 0.0), (1, "location", 0, 1.30)]})
    e.clipe("setup", True, {
        "unidade": [(0, "location", 0, 0.0), (0.25, "location", 0, 0.30), (0.75, "location", 0, 0.30),
                    (1, "location", 0, 0.0)],
    })
    e.clipe("quebra", False, {"tampa": [(0, "rotation_euler", 0, 0.0), (1, "rotation_euler", 0, math.radians(-80))]})
    return e

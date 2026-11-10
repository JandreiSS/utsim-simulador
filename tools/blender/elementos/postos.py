"""Posto manual, robô e célula cercada."""

import math

from comum import Elemento

# Robô: alturas e comprimentos (m).
PEDESTAL = 0.50
OMBRO = 0.78  # altura da junta do ombro
BRACO = 0.70
ANTEBRACO = 0.65
GARRA = 0.22  # do punho à ponta dos dedos
ALCANCE = 1.0  # distância horizontal do eixo à peça, em ponto_entrada e ponto_saida
PEGA = 0.76  # altura da base da peça (topo da esteira padrão)


def posto(detalhe=0):
    """Bancada de montagem, 1,5 × 0,75 m, tampo a 0,90 m, lado de trabalho em `-y`.
    Nós: `estrutura`, `grampo` (alavanca do gabarito), `luz` (andon)."""
    e = Elemento("posto", detalhe)
    e.no("estrutura")
    e.caixa("estrutura", "tampo", (-0.75, -0.375, 0.86), (0.75, 0.375, 0.90))
    for x in (-0.70, 0.70):
        for y in (-0.32, 0.32):
            e.caixa("estrutura", "faixa", (x - 0.025, y - 0.025, 0.0), (x + 0.025, y + 0.025, 0.86))
    e.caixa("estrutura", "faixa", (-0.70, -0.32, 0.14), (0.70, 0.32, 0.17))
    e.caixa("estrutura", "painel", (-0.75, 0.33, 0.90), (0.75, 0.37, 1.55))
    e.caixa("estrutura", "aco_escuro", (-0.75, 0.30, 1.55), (0.75, 0.37, 1.60))
    # Caixas de peças: entrada à esquerda (-x), saída à direita (+x).
    for x0, cor in ((-0.70, "laranja"), (0.40, "verde")):
        e.caixa("estrutura", cor, (x0, -0.05, 0.90), (x0 + 0.30, 0.25, 1.02))
    # Gabarito no centro.
    e.caixa("estrutura", "aco_escuro", (-0.18, -0.25, 0.90), (0.18, 0.05, 0.94))
    e.no("grampo", pivo=(0.14, -0.10, 0.96))
    e.caixa("grampo", "aco", (0.12, -0.12, 0.94), (0.16, -0.08, 0.98))
    e.caixa("grampo", "vermelho", (0.13, -0.11, 0.98), (0.15, -0.09, 1.12))
    # Andon.
    e.cilindro("estrutura", "painel", (0.68, 0.35, 1.60), (0.68, 0.35, 1.80), 0.015, 8)
    e.material("luz", 0x46D160, 0.0, 0.4, emissao=True)
    e.no("luz", pivo=(0.68, 0.35, 1.80))
    e.cilindro("luz", "luz", (0.68, 0.35, 1.80), (0.68, 0.35, 1.92), 0.04)
    e.ponto("ponto_operador", (0.0, -0.70, 0.0), 90)
    e.ponto("ponto_porta", (0.0, -0.70, 0.0), 90)
    e.ponto("ponto_peca", (0.0, -0.10, 0.94), 0)
    e.ponto("ponto_entrada", (-0.55, 0.10, 1.02), 0)
    e.ponto("ponto_saida", (0.55, 0.10, 1.02), 0)
    e.clipe("ciclo", True, {
        "grampo": [(0, "rotation_euler", 1, 0.0), (0.15, "rotation_euler", 1, math.radians(-70)),
                   (0.85, "rotation_euler", 1, math.radians(-70)), (1, "rotation_euler", 1, 0.0)],
    })
    # Setup: o grampo abre todo para trocar o gabarito e volta.
    e.clipe("setup", True, {
        "grampo": [(0, "rotation_euler", 1, 0.0), (0.3, "rotation_euler", 1, math.radians(-120)),
                   (0.7, "rotation_euler", 1, math.radians(-120)), (1, "rotation_euler", 1, 0.0)],
    })
    return e


def _ik(r, z):
    """Ângulos (ombro, cotovelo, punho) para a ponta dos dedos em (r, z), garra para
    baixo. Giro positivo em y inclina para a frente (+x)."""
    wx, wz = r, z + GARRA - OMBRO  # punho, relativo ao ombro
    d = math.hypot(wx, wz)
    c2 = (d * d - BRACO * BRACO - ANTEBRACO * ANTEBRACO) / (2 * BRACO * ANTEBRACO)
    q2 = math.acos(max(-1.0, min(1.0, c2)))  # dobra do cotovelo
    # Ângulo do braço a partir da vertical, cotovelo para cima.
    phi = math.atan2(wx, wz)
    psi = math.atan2(ANTEBRACO * math.sin(q2), BRACO + ANTEBRACO * math.cos(q2))
    a = phi - psi
    # Antebraço em repouso aponta +x (90° da vertical); direção final = a + 90° + b.
    b = (a + q2) - (a + math.pi / 2)
    c = -(a + b)
    return a, b, c


def robo(detalhe=0):
    """Robô articulado sobre pedestal. Em repouso: braço vertical, antebraço para a
    frente (+x), garra para baixo. Nós: `pedestal`, `base` (gira em z), `braco`
    (ombro, gira em y), `antebraco` (cotovelo), `garra` (punho), `dedo_e`, `dedo_d`.
    Pega em `ponto_entrada` (+y) e solta em `ponto_saida` (-y).
    Clipes: `pegar` (de repouso até a peça, fecha a garra), `levar` (sobe e gira até
    a saída), `soltar` (desce, abre e volta ao repouso), `ciclo` (os três seguidos),
    `setup` (a base gira e os dedos abrem e fecham na troca de ferramenta) e `quebra`
    (o braço cai e fica parado)."""
    e = Elemento("robo", detalhe)
    e.material("robo", 0xF08A00, 0.15, 0.45)
    e.no("pedestal")
    e.caixa("pedestal", "base", (-0.35, -0.35, 0.0), (0.35, 0.35, 0.05))
    e.cilindro("pedestal", "aco_escuro", (0.0, 0.0, 0.05), (0.0, 0.0, PEDESTAL), 0.20)
    e.no("base", pivo=(0.0, 0.0, PEDESTAL))
    e.cilindro("base", "robo", (0.0, 0.0, PEDESTAL), (0.0, 0.0, PEDESTAL + 0.18), 0.24)
    e.caixa("base", "robo", (-0.14, -0.16, PEDESTAL + 0.18), (0.14, 0.16, OMBRO + 0.10))
    e.no("braco", "base", (0.0, 0.0, OMBRO))
    e.cilindro("braco", "aco_escuro", (0.0, -0.17, OMBRO), (0.0, 0.17, OMBRO), 0.11)
    e.caixa("braco", "robo", (-0.08, -0.10, OMBRO), (0.08, 0.10, OMBRO + BRACO))
    cot = OMBRO + BRACO
    e.no("antebraco", "braco", (0.0, 0.0, cot))
    e.cilindro("antebraco", "aco_escuro", (0.0, -0.13, cot), (0.0, 0.13, cot), 0.08)
    e.caixa("antebraco", "robo", (0.0, -0.07, cot - 0.07), (ANTEBRACO, 0.07, cot + 0.07))
    e.no("garra", "antebraco", (ANTEBRACO, 0.0, cot))
    e.cilindro("garra", "aco_escuro", (ANTEBRACO, -0.06, cot), (ANTEBRACO, 0.06, cot), 0.06)
    e.caixa("garra", "aco", (ANTEBRACO - 0.05, -0.08, cot - 0.12), (ANTEBRACO + 0.05, 0.08, cot - 0.04))
    for nome, s in (("dedo_e", 1), ("dedo_d", -1)):
        e.no(nome, "garra", (ANTEBRACO, s * 0.06, cot - 0.12))
        e.caixa(nome, "aco_escuro", (ANTEBRACO - 0.02, s * 0.06 - 0.01, cot - GARRA),
                (ANTEBRACO + 0.02, s * 0.06 + 0.01, cot - 0.12))
    # Na ponta dos dedos: a base da peça presa fica aqui.
    e.ponto("ponto_garra", (ANTEBRACO, 0.0, cot - GARRA), 0, pai="garra")
    e.ponto("ponto_entrada", (0.0, ALCANCE, PEGA), 90)
    e.ponto("ponto_saida", (0.0, -ALCANCE, PEGA), -90)
    e.ponto("ponto_operador", (-1.20, 0.0, 0.0), 0)
    e.ponto("ponto_porta", (-1.20, 0.0, 0.0), 0)

    # Poses (relativas ao repouso): giro da base, ombro, cotovelo, punho, dedos.
    a, b, c = _ik(ALCANCE, PEGA)
    a2, b2, c2 = _ik(ALCANCE, PEGA + 0.35)
    aberto, fechado = 0.04, 0.0
    rep = (0.0, 0.0, 0.0, 0.0, aberto)
    sobre_e = (math.pi / 2, a2, b2, c2, aberto)
    pega_ab = (math.pi / 2, a, b, c, aberto)
    pega_fe = (math.pi / 2, a, b, c, fechado)
    sobe_e = (math.pi / 2, a2, b2, c2, fechado)
    sobre_s = (-math.pi / 2, a2, b2, c2, fechado)
    solta_fe = (-math.pi / 2, a, b, c, fechado)
    solta_ab = (-math.pi / 2, a, b, c, aberto)
    sobe_s = (-math.pi / 2, a2, b2, c2, aberto)

    def chaves(seq):
        k = {"base": [], "braco": [], "antebraco": [], "garra": [], "dedo_e": [], "dedo_d": []}
        for fase, (g, o, co, p, d) in seq:
            k["base"].append((fase, "rotation_euler", 2, g))
            k["braco"].append((fase, "rotation_euler", 1, o))
            k["antebraco"].append((fase, "rotation_euler", 1, co))
            k["garra"].append((fase, "rotation_euler", 1, p))
            k["dedo_e"].append((fase, "location", 1, d - aberto))
            k["dedo_d"].append((fase, "location", 1, aberto - d))
        return k

    pegar = [(0, rep), (0.5, sobre_e), (0.8, pega_ab), (1, pega_fe)]
    levar = [(0, pega_fe), (0.25, sobe_e), (0.85, sobre_s), (1, solta_fe)]
    soltar = [(0, solta_fe), (0.25, solta_ab), (0.5, sobe_s), (1, rep)]
    e.clipe("pegar", False, chaves(pegar))
    e.clipe("levar", False, chaves(levar))
    e.clipe("soltar", False, chaves(soltar))
    tudo = [(f / 3, p) for f, p in pegar] + [((1 + f) / 3, p) for f, p in levar[1:]] + [
        ((2 + f) / 3, p) for f, p in soltar[1:]]
    e.clipe("ciclo", True, chaves(tudo))
    # Só os eixos que os outros clipes já usam (o exportador completa os outros eixos de
    # um nó com o valor de outro clipe).
    e.clipe("setup", True, {
        "base": [(0, "rotation_euler", 2, 0.0), (0.4, "rotation_euler", 2, math.radians(-30)),
                 (0.6, "rotation_euler", 2, math.radians(-30)), (1, "rotation_euler", 2, 0.0)],
        "dedo_e": [(0, "location", 1, 0.0), (0.4, "location", 1, -aberto), (0.6, "location", 1, -aberto),
                   (1, "location", 1, 0.0)],
        "dedo_d": [(0, "location", 1, 0.0), (0.4, "location", 1, aberto), (0.6, "location", 1, aberto),
                   (1, "location", 1, 0.0)],
    })
    e.clipe("quebra", False, {
        "braco": [(0, "rotation_euler", 1, 0.0), (1, "rotation_euler", 1, math.radians(25))],
        "antebraco": [(0, "rotation_euler", 1, 0.0), (1, "rotation_euler", 1, math.radians(40))],
        "garra": [(0, "rotation_euler", 1, 0.0), (1, "rotation_euler", 1, math.radians(20))],
    })
    return e


def celula(detalhe=0):
    """Cerca de célula robotizada, 3,0 × 3,0 × 2,0 m, com portão no lado `-y`.
    Nós: `cerca`, `portao` (dobradiça vertical em x = -0,5), `luz` (sinaleiro).
    `ponto_porta` fica fora do portão e `ponto_operador` dentro."""
    e = Elemento("celula", detalhe)
    e.material("tela_cerca", 0x9AA3AD, 0.5, 0.5, alfa=0.35)
    lado, alt = 1.5, 2.0
    e.no("cerca")
    postes = [(-lado, -lado), (-lado, lado), (lado, lado), (lado, -lado), (-0.5, -lado), (0.5, -lado),
              (0.0, lado), (-lado, 0.0), (lado, 0.0)]
    for x, y in postes:
        e.caixa("cerca", "amarelo", (x - 0.03, y - 0.03, 0.0), (x + 0.03, y + 0.03, alt))
    paineis = [((-lado, lado), (lado, lado)), ((-lado, -lado), (-lado, lado)), ((lado, -lado), (lado, lado)),
               ((-lado, -lado), (-0.5, -lado)), ((0.5, -lado), (lado, -lado))]
    for (x0, y0), (x1, y1) in paineis:
        e.caixa("cerca", "tela_cerca", (min(x0, x1) - 0.005, min(y0, y1) - 0.005, 0.15),
                (max(x0, x1) + 0.005, max(y0, y1) + 0.005, alt - 0.05))
        e.caixa("cerca", "amarelo", (min(x0, x1) - 0.01, min(y0, y1) - 0.01, alt - 0.05),
                (max(x0, x1) + 0.01, max(y0, y1) + 0.01, alt))
    e.no("portao", pivo=(-0.47, -lado, 0.0))
    e.caixa("portao", "amarelo", (-0.47, -lado - 0.02, 0.10), (0.47, -lado + 0.02, 0.15))
    e.caixa("portao", "amarelo", (-0.47, -lado - 0.02, alt - 0.10), (0.47, -lado + 0.02, alt - 0.05))
    e.caixa("portao", "amarelo", (0.42, -lado - 0.02, 0.15), (0.47, -lado + 0.02, alt - 0.10))
    e.caixa("portao", "tela_cerca", (-0.47, -lado - 0.005, 0.15), (0.42, -lado + 0.005, alt - 0.10))
    e.caixa("portao", "vermelho", (0.30, -lado - 0.06, 1.0), (0.40, -lado - 0.02, 1.10))  # trava
    e.cilindro("cerca", "painel", (-0.60, -lado - 0.05, alt), (-0.60, -lado - 0.05, alt + 0.15), 0.02, 8)
    e.material("luz", 0x46D160, 0.0, 0.4, emissao=True)
    e.no("luz", pivo=(-0.60, -lado - 0.05, alt + 0.15))
    e.cilindro("luz", "luz", (-0.60, -lado - 0.05, alt + 0.15), (-0.60, -lado - 0.05, alt + 0.30), 0.05)
    e.ponto("ponto_porta", (0.0, -lado - 0.50, 0.0), 90)
    e.ponto("ponto_operador", (0.0, -lado + 0.60, 0.0), 90)
    e.clipe("porta_abrir", False, {"portao": [(0, "rotation_euler", 2, 0.0), (1, "rotation_euler", 2, math.radians(-100))]})
    return e

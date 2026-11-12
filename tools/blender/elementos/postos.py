"""Posto manual, robô e célula cercada."""

import math

from comum import Elemento

# Robô: alturas e comprimentos (m).
PEDESTAL = 0.50
OMBRO = 0.78  # altura da junta do ombro
BRACO = 0.70
ANTEBRACO = 0.65
GARRA = 0.27  # do punho à ponta dos dedos
DEDO = 0.11  # dedo aberto, do centro da garra
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
    # Gaveteiro sob o tampo, à direita, com três gavetas.
    e.caixa("estrutura", "painel", (0.26, -0.33, 0.44), (0.66, 0.30, 0.84))
    for k in range(3):
        z = 0.47 + k * 0.125
        e.caixa("estrutura", "aco", (0.28, -0.34, z), (0.64, -0.33, z + 0.105), chanfro=0)
        e.caixa("estrutura", "base", (0.38, -0.355, z + 0.045), (0.54, -0.34, z + 0.06), chanfro=0)
    # Ferramentas no painel.
    e.caixa("estrutura", "vermelho", (-0.50, 0.315, 1.15), (-0.46, 0.33, 1.40), chanfro=0)
    e.caixa("estrutura", "aco", (-0.38, 0.315, 1.12), (-0.35, 0.33, 1.42), chanfro=0)
    e.caixa("estrutura", "amarelo", (-0.27, 0.315, 1.20), (-0.15, 0.33, 1.32), chanfro=0)
    e.caixa("estrutura", "aco", (-0.02, 0.315, 1.18), (0.22, 0.33, 1.21), chanfro=0)
    # Luminária sobre a bancada, em dois braços.
    for x in (-0.60, 0.50):
        e.caixa("estrutura", "faixa", (x - 0.02, 0.31, 1.60), (x + 0.02, 0.35, 1.84), chanfro=0)
    e.caixa("estrutura", "aco_escuro", (-0.64, 0.02, 1.84), (0.54, 0.37, 1.88))
    e.caixa("estrutura", "pintura", (-0.58, 0.06, 1.832), (0.48, 0.30, 1.84), chanfro=0)
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
    d = detalhe
    e.material("robo", 0xF08A00, 0.15, 0.45)
    e.material("motor", 0x3A3F45, 0.4, 0.5)
    lados = 20 if d else 12
    # Pedestal: chapa de base com parafusos, coluna e flange.
    e.no("pedestal")
    e.caixa("pedestal", "base", (-0.35, -0.35, 0.0), (0.35, 0.35, 0.05), chanfro=0.015)
    if d:
        for x in (-0.28, 0.28):
            for y in (-0.28, 0.28):
                e.cilindro("pedestal", "aco", (x, y, 0.05), (x, y, 0.07), 0.018, 6)
    e.cilindro("pedestal", "aco_escuro", (0.0, 0.0, 0.05), (0.0, 0.0, PEDESTAL - 0.04), 0.19, lados)
    e.cilindro("pedestal", "aco_escuro", (0.0, 0.0, PEDESTAL - 0.04), (0.0, 0.0, PEDESTAL), 0.25, lados)
    # Eixo 1: torre que gira, com o motor vertical atrás e o apoio do ombro.
    e.no("base", pivo=(0.0, 0.0, PEDESTAL))
    e.cilindro("base", "robo", (0.0, 0.0, PEDESTAL), (0.0, 0.0, PEDESTAL + 0.14), 0.27, lados, raio2=0.25)
    e.prisma("base", "robo", [(-0.22, PEDESTAL + 0.14), (0.20, PEDESTAL + 0.14), (0.15, OMBRO - 0.02),
                              (0.06, OMBRO + 0.12), (-0.08, OMBRO + 0.12), (-0.17, OMBRO - 0.02)], -0.15, 0.15)
    e.cilindro("base", "motor", (-0.24, 0.0, PEDESTAL + 0.06), (-0.24, 0.0, PEDESTAL + 0.34), 0.075, 12 if d else 8)
    e.cilindro("base", "aco_escuro", (-0.24, 0.0, PEDESTAL + 0.34), (-0.24, 0.0, PEDESTAL + 0.37), 0.055, 12 if d else 8)
    # Eixo 2 (ombro): cubo largo, motor do lado e o braço afinando até o cotovelo.
    e.no("braco", "base", (0.0, 0.0, OMBRO))
    e.cilindro("braco", "robo", (0.0, -0.17, OMBRO), (0.0, 0.17, OMBRO), 0.15, lados)
    e.cilindro("braco", "motor", (0.0, 0.17, OMBRO), (0.0, 0.29, OMBRO), 0.10, 14 if d else 8)
    e.cilindro("braco", "aco_escuro", (0.0, 0.29, OMBRO), (0.0, 0.31, OMBRO), 0.07, 14 if d else 8)
    cot = OMBRO + BRACO
    e.prisma("braco", "robo", [(-0.13, OMBRO), (0.13, OMBRO), (0.10, cot), (-0.10, cot)], -0.11, 0.11)
    if d:
        e.caixa("braco", "aco_escuro", (-0.09, -0.115, OMBRO + 0.20), (0.09, 0.115, OMBRO + 0.24), chanfro=0)
    # Eixo 3 (cotovelo): carcaça com o motor atrás e o antebraço redondo para a frente.
    e.no("antebraco", "braco", (0.0, 0.0, cot))
    e.cilindro("antebraco", "robo", (0.0, -0.13, cot), (0.0, 0.13, cot), 0.12, lados)
    e.caixa("antebraco", "robo", (-0.20, -0.10, cot - 0.09), (0.06, 0.10, cot + 0.11), chanfro=0.03)
    e.cilindro("antebraco", "motor", (-0.20, 0.0, cot + 0.01), (-0.33, 0.0, cot + 0.01), 0.075, 12 if d else 8)
    e.cilindro("antebraco", "robo", (0.06, 0.0, cot), (ANTEBRACO - 0.08, 0.0, cot), 0.08, lados, raio2=0.065)
    # Eixo 4: anel do giro do antebraço.
    e.cilindro("antebraco", "aco_escuro", (0.28, 0.0, cot), (0.30, 0.0, cot), 0.084, lados)
    # Eixos 5 e 6: punho, flange e a garra de dois dedos.
    e.no("garra", "antebraco", (ANTEBRACO, 0.0, cot))
    e.cilindro("garra", "robo", (ANTEBRACO, -0.075, cot), (ANTEBRACO, 0.075, cot), 0.07, lados)
    e.cilindro("garra", "robo", (ANTEBRACO - 0.08, 0.0, cot), (ANTEBRACO, 0.0, cot), 0.06, lados)
    e.cilindro("garra", "aco", (ANTEBRACO, 0.0, cot - 0.06), (ANTEBRACO, 0.0, cot - 0.09), 0.055, lados)
    e.caixa("garra", "aco_escuro", (ANTEBRACO - 0.05, -0.14, cot - 0.14), (ANTEBRACO + 0.05, 0.14, cot - 0.09), chanfro=0.01)
    for nome, s in (("dedo_e", 1), ("dedo_d", -1)):
        y = s * DEDO
        e.no(nome, "garra", (ANTEBRACO, y, cot - 0.14))
        e.caixa(nome, "aco", (ANTEBRACO - 0.03, y - 0.012, cot - 0.16), (ANTEBRACO + 0.03, y + 0.012, cot - 0.12), chanfro=0)
        e.caixa(nome, "aco_escuro", (ANTEBRACO - 0.025, y - 0.010, cot - GARRA), (ANTEBRACO + 0.025, y + 0.010, cot - 0.16), chanfro=0.004)
        # Almofada de borracha do lado de dentro.
        y0, y1 = sorted((y - s * 0.016, y - s * 0.010))
        e.caixa(nome, "borracha", (ANTEBRACO - 0.022, y0, cot - GARRA + 0.01), (ANTEBRACO + 0.022, y1, cot - GARRA + 0.10), chanfro=0)
    # Na ponta dos dedos: a base da peça presa fica aqui.
    e.ponto("ponto_garra", (ANTEBRACO, 0.0, cot - GARRA), 0, pai="garra")
    e.ponto("ponto_entrada", (0.0, ALCANCE, PEGA), 90)
    e.ponto("ponto_saida", (0.0, -ALCANCE, PEGA), -90)
    e.ponto("ponto_operador", (-1.20, 0.0, 0.0), 0)
    e.ponto("ponto_porta", (-1.20, 0.0, 0.0), 0)

    # Poses (relativas ao repouso): giro da base, ombro, cotovelo, punho, dedos.
    a, b, c = _ik(ALCANCE, PEGA)
    a2, b2, c2 = _ik(ALCANCE, PEGA + 0.35)
    # Dedos: abertos a `DEDO` do centro; fechados, encostam na peça (0,12 m de diâmetro).
    aberto, fechado = round(DEDO - 0.076, 6), 0.0
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
    # A peça no ciclo: parada na entrada até a garra fechar (1/3), presa na garra até ela
    # abrir na saída (2/3 + 1/4 do `soltar`), e na saída até o fim.
    solta = round((2 + soltar[1][0]) / 3, 6)
    e.peca = {
        "processing": [[0.0, round(1 / 3, 6), "ponto_entrada"], [round(1 / 3, 6), solta, "ponto_garra"],
                       [solta, 1.0, "ponto_saida"]],
        "blocked": [[0.0, 1.0, "ponto_saida"]],
    }
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


def _malha(e, no, a, b, z0, z1, passo=0.25, fio=0.012):
    """Fios da tela de um painel de cerca, de `a` a `b` (x, y) entre `z0` e `z1`."""
    h = fio / 2
    (x0, y0), (x1, y1) = (min(a[0], b[0]), min(a[1], b[1])), (max(a[0], b[0]), max(a[1], b[1]))
    n = max(1, round(max(x1 - x0, y1 - y0) / passo))
    for k in range(1, n):
        f = k / n
        x, y = x0 + (x1 - x0) * f, y0 + (y1 - y0) * f
        e.caixa(no, "aco_escuro", (x - h, y - h, z0), (x + h, y + h, z1), chanfro=0)
    m = max(1, round((z1 - z0) / passo))
    for k in range(1, m):
        z = z0 + (z1 - z0) * k / m
        e.caixa(no, "aco_escuro", (x0 - h, y0 - h, z - h), (x1 + h, y1 + h, z + h), chanfro=0)


def espera(detalhe=0):
    """Mesa de espera (fila ou buffer), 1,20 × 0,80 m, tampo a 0,76 m (a altura da esteira).
    A peça entra por `-x` e sai por `+x`; as guias baixas ficam nas bordas compridas.
    Nós: `estrutura` e `faixa` (a faixa amarela no chão em volta, 1,40 × 1,00 m)."""
    e = Elemento("espera", detalhe)
    e.material("tampo_aco", 0xC9CED4, 0.6, 0.45)
    e.no("estrutura")
    for x in (-0.55, 0.55):
        for y in (-0.35, 0.35):
            e.caixa("estrutura", "aco_escuro", (x - 0.025, y - 0.025, 0.04), (x + 0.025, y + 0.025, 0.68))
            e.cilindro("estrutura", "aco", (x, y, 0.015), (x, y, 0.04), 0.012, 8)
            e.cilindro("estrutura", "aco", (x, y, 0.0), (x, y, 0.015), 0.04, 12)
    e.caixa("estrutura", "aco_escuro", (-0.60, -0.40, 0.68), (0.60, 0.40, 0.72))
    e.caixa("estrutura", "tampo_aco", (-0.60, -0.40, 0.72), (0.60, 0.40, 0.76))
    for y in (-0.40, 0.38):
        e.caixa("estrutura", "aco_escuro", (-0.60, y, 0.76), (0.60, y + 0.02, 0.82))
    # Prateleira de baixo, sobre travessas.
    for y in (-0.35, 0.35):
        e.caixa("estrutura", "aco_escuro", (-0.55, y - 0.02, 0.20), (0.55, y + 0.02, 0.24), chanfro=0)
    e.caixa("estrutura", "aco", (-0.57, -0.37, 0.24), (0.57, 0.37, 0.26))
    # Faixa amarela no chão, 5 cm de largura.
    e.no("faixa")
    for y in (-0.50, 0.45):
        e.caixa("faixa", "amarelo", (-0.70, y, 0.003), (0.70, y + 0.05, 0.006), chanfro=0)
    for x in (-0.70, 0.65):
        e.caixa("faixa", "amarelo", (x, -0.45, 0.003), (x + 0.05, 0.45, 0.006), chanfro=0)
    e.ponto("ponto_entrada", (-0.45, 0.0, 0.76), 0)
    e.ponto("ponto_saida", (0.45, 0.0, 0.76), 0)
    e.dados = {"tampo": 0.76, "area": [1.10, 0.66]}
    return e


def celula(detalhe=0):
    """Cerca de célula robotizada, 3,0 × 3,0 × 2,0 m, com portão no lado `-y`.
    Nós: `cerca`, `portao` (dobradiça vertical em x = -0,5), `luz` (sinaleiro).
    `ponto_porta` fica fora do portão e `ponto_operador` dentro."""
    e = Elemento("celula", detalhe)
    e.material("tela_cerca", 0x596068, 0.5, 0.5, alfa=0.30)
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
        e.caixa("cerca", "amarelo", (min(x0, x1) - 0.01, min(y0, y1) - 0.01, 0.12),
                (max(x0, x1) + 0.01, max(y0, y1) + 0.01, 0.15))
        _malha(e, "cerca", (x0, y0), (x1, y1), 0.15, alt - 0.05)
    e.no("portao", pivo=(-0.47, -lado, 0.0))
    e.caixa("portao", "amarelo", (-0.47, -lado - 0.02, 0.10), (0.47, -lado + 0.02, 0.15))
    e.caixa("portao", "amarelo", (-0.47, -lado - 0.02, alt - 0.10), (0.47, -lado + 0.02, alt - 0.05))
    e.caixa("portao", "amarelo", (0.42, -lado - 0.02, 0.15), (0.47, -lado + 0.02, alt - 0.10))
    e.caixa("portao", "tela_cerca", (-0.47, -lado - 0.005, 0.15), (0.42, -lado + 0.005, alt - 0.10))
    _malha(e, "portao", (-0.47, -lado), (0.42, -lado), 0.15, alt - 0.10)
    e.caixa("portao", "vermelho", (0.30, -lado - 0.06, 1.0), (0.40, -lado - 0.02, 1.10))  # trava
    e.cilindro("cerca", "painel", (-0.60, -lado - 0.05, alt), (-0.60, -lado - 0.05, alt + 0.15), 0.02, 8)
    e.material("luz", 0x46D160, 0.0, 0.4, emissao=True)
    e.no("luz", pivo=(-0.60, -lado - 0.05, alt + 0.15))
    e.cilindro("luz", "luz", (-0.60, -lado - 0.05, alt + 0.15), (-0.60, -lado - 0.05, alt + 0.30), 0.05)
    e.ponto("ponto_porta", (0.0, -lado - 0.50, 0.0), 90)
    e.ponto("ponto_operador", (0.0, -lado + 0.60, 0.0), 90)
    e.clipe("porta_abrir", False, {"portao": [(0, "rotation_euler", 2, 0.0), (1, "rotation_euler", 2, math.radians(-100))]})
    return e

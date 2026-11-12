"""Porta-paletes, estante e cargas (peça, palete, caixa).

Os vãos têm um ponto cada, `ponto_vao_<b>_<n>` (vão `b` da esquerda para a direita,
nível `n` a partir do piso), no centro do vão, no topo da longarina; o `+x` local
aponta para o corredor (`-y`), de onde a empilhadeira chega.
"""

from comum import Elemento

VAOS, NIVEIS = 3, 4
LARG_VAO, ALT_NIVEL, PROF, LONG = 1.4, 1.5, 1.0, 0.08


def porta_paletes(detalhe=0, vaos=VAOS, niveis=NIVEIS):
    """Porta-paletes: montantes azuis com travessas e diagonais, longarinas laranja e
    protetor amarelo no pé das colunas do corredor. Nó `estrutura`, sem clipe."""
    e = Elemento("porta_paletes", detalhe)
    e.material("montante", 0x2B6CB0, 0.3, 0.5)
    e.no("estrutura")
    larg = vaos * LARG_VAO
    x0, p2 = -larg / 2, PROF / 2
    topo = niveis * ALT_NIVEL + LONG
    for b in range(vaos + 1):
        x = x0 + b * LARG_VAO
        for y in (-p2, p2):
            e.caixa("estrutura", "montante", (x - 0.04, y - 0.04, 0.0), (x + 0.04, y + 0.04, topo))
            e.caixa("estrutura", "base", (x - 0.07, y - 0.07, 0.0), (x + 0.07, y + 0.07, 0.01))
        # Protetor de coluna, só do lado do corredor.
        e.caixa("estrutura", "amarelo", (x - 0.07, -p2 - 0.07, 0.01), (x + 0.07, -p2 + 0.07, 0.40))
        # Travessas e diagonais do montante (sem chanfro: são finas).
        zs = [0.3 + k * 0.9 for k in range(int((topo - 0.3) / 0.9) + 1)]
        for z in zs:
            e.caixa("estrutura", "montante", (x - 0.015, -p2, z), (x + 0.015, p2, z + 0.03), chanfro=0)
        for k, (za, zb) in enumerate(zip(zs, zs[1:])):
            ya, yb = (-p2 + 0.04, p2 - 0.04) if k % 2 == 0 else (p2 - 0.04, -p2 + 0.04)
            e.prisma("estrutura", "montante", [(ya, za + 0.03), (ya, za + 0.07), (yb, zb), (yb, zb - 0.04)],
                     x - 0.012, x + 0.012, eixo=0)
    for n in range(1, niveis + 1):
        z = n * ALT_NIVEL
        for y in (-p2, p2):
            e.caixa("estrutura", "laranja", (x0, y - 0.03, z), (x0 + larg, y + 0.03, z + LONG))
    for b in range(vaos):
        xc = x0 + (b + 0.5) * LARG_VAO
        for n in range(niveis + 1):
            z = 0.0 if n == 0 else n * ALT_NIVEL + LONG
            e.ponto(f"ponto_vao_{b}_{n}", (xc, 0.0, z), -90)
    return e


def estante(detalhe=0, niveis=4):
    """Estante de caixas 1,0 × 0,5 × 2,0 m, prateleiras de aço. Nó `estrutura`."""
    e = Elemento("estante", detalhe)
    e.no("estrutura")
    for x in (-0.48, 0.48):
        for y in (-0.23, 0.23):
            e.caixa("estrutura", "aco_escuro", (x - 0.02, y - 0.02, 0.0), (x + 0.02, y + 0.02, 2.0))
    for n in range(niveis + 1):
        z = 0.10 + n * (1.85 / niveis)
        e.caixa("estrutura", "aco", (-0.50, -0.25, z), (0.50, 0.25, z + 0.025))
        e.caixa("estrutura", "aco_escuro", (-0.50, -0.25, z - 0.03), (0.50, -0.235, z), chanfro=0)  # aba
        if n < niveis:
            e.ponto(f"ponto_vao_0_{n}", (0.0, 0.0, z + 0.025), -90)
    # Contraventamento em X no fundo.
    for xa, xb in ((-0.46, 0.46), (0.46, -0.46)):
        e.prisma("estrutura", "aco_escuro", [(xa, 0.15), (xa, 0.19), (xb, 1.95), (xb, 1.91)], 0.238, 0.248)
    return e


def peca(detalhe=0):
    """Peça usinada (tarugo com ressalto), 0,12 m de diâmetro. Nó `corpo`."""
    e = Elemento("peca", detalhe)
    e.no("corpo")
    e.cilindro("corpo", "aco", (-0.06, 0.0, 0.06), (0.03, 0.0, 0.06), 0.06)
    e.cilindro("corpo", "aco", (0.03, 0.0, 0.06), (0.07, 0.0, 0.06), 0.035)
    return e


def palete(detalhe=0):
    """Palete PBR 1,20 × 1,00 × 0,144 m. Nó `corpo`."""
    e = Elemento("palete", detalhe)
    e.no("corpo")
    for y in (-0.45, 0.0, 0.45):
        e.caixa("corpo", "madeira", (-0.60, y - 0.05, 0.0), (0.60, y + 0.05, 0.022))
        for x in (-0.55, 0.0, 0.55):
            e.caixa("corpo", "madeira", (x - 0.05, y - 0.05, 0.022), (x + 0.05, y + 0.05, 0.122))
    for x in (-0.55, 0.0, 0.55):
        e.caixa("corpo", "madeira", (x - 0.05, -0.50, 0.0), (x + 0.05, 0.50, 0.022))
    for k in range(7):
        x = -0.55 + k * (1.10 / 6)
        e.caixa("corpo", "madeira", (x - 0.05, -0.50, 0.122), (x + 0.05, 0.50, 0.144))
    return e


def caixa(detalhe=0):
    """Caixa de papelão 0,40 × 0,30 × 0,30 m, com fita. Nó `corpo`."""
    e = Elemento("caixa", detalhe)
    e.no("corpo")
    e.caixa("corpo", "papelao", (-0.20, -0.15, 0.0), (0.20, 0.15, 0.30))
    e.material("fita", 0xD9C7A3, 0.0, 0.6)
    e.caixa("corpo", "fita", (-0.203, -0.03, 0.0), (0.203, 0.03, 0.303))
    return e

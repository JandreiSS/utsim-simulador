"""Peças pequenas da esteira desenhada por trajeto, para repetir ao longo dela.

A esteira por trajeto é feita na cena, ao longo da linha (`godot/scripts/trajeto_malha.gd`).
Estas peças completam o desenho: o pé com sapata regulável, a ponteira (o tambor de ponta com
os mancais) e o suporte da canaleta. Cada uma tem a origem no ponto de apoio, `+x` ao longo da
esteira e `z` para cima. As medidas seguem as da malha do trajeto: longarina de 0,14 m, perna
de 0,05 m e guia de 0,16 m.
"""

import math

from comum import Elemento

TAU = 2 * math.pi
PERNA = 0.05  # lado do tubo da perna
SAPATA = 0.06  # altura da sapata, do chão à base da perna
PERNA_BASE = 1.0  # comprimento da perna no arquivo (estica em z)
TAMBOR = 0.07  # raio do tambor de ponta: metade da longarina
LARGURA_BASE = 1.0  # comprimento do tambor no arquivo (estica em y)
GUIA = 0.14  # o topo da guia da canaleta acima do topo da longarina
GUIA_FORA = 0.015  # a face de fora da guia fica isto para dentro da face da longarina
GUIA_ESPESSURA = 0.035


def esteira_pe(detalhe=0):
    """Pé de esteira com sapata regulável. Origem no chão, no centro da sapata. Nós: `sapata`
    (disco de borracha, haste roscada e porca; fixo) e `perna` (tubo quadrado; origem na base
    dele, estica em z)."""
    e = Elemento("esteira_pe", detalhe)
    d = detalhe
    lados = 12 if d else 8
    e.no("sapata")
    e.cilindro("sapata", "borracha", (0, 0, 0.0), (0, 0, 0.012), 0.045, lados)
    e.cilindro("sapata", "aco", (0, 0, 0.012), (0, 0, 0.020), 0.038, lados, raio2=0.020)
    e.cilindro("sapata", "aco", (0, 0, 0.020), (0, 0, SAPATA + 0.02), 0.008, 6)
    e.cilindro("sapata", "aco_escuro", (0, 0, SAPATA - 0.016), (0, 0, SAPATA - 0.004), 0.016, 6)
    e.no("perna", pivo=(0.0, 0.0, SAPATA))
    h = PERNA / 2
    e.caixa("perna", "aco", (-h, -h, SAPATA), (h, h, SAPATA + PERNA_BASE), chanfro=0)
    e.dados = {"sapata_altura": SAPATA, "perna_base": PERNA_BASE, "lado": PERNA, "estica": {"perna": "z"}}
    return e


def esteira_ponteira(detalhe=0):
    """Ponteira da esteira: o tambor de ponta com os dois mancais. Origem na linha (o topo da
    superfície), no fim da esteira. Nós: `tambor` (ao longo de y, estica em y), `mancal_e` e
    `mancal_d` (origem no eixo do tambor, em `y = ±largura/2`). Clipe `rolar`: o tambor gira."""
    e = Elemento("esteira_ponteira", detalhe)
    d = detalhe
    lados = 16 if d else 10
    w2 = LARGURA_BASE / 2
    eixo = (0.0, 0.0, -TAMBOR)
    e.no("tambor", pivo=eixo)
    e.cilindro("tambor", "aco", (0, -w2, -TAMBOR), (0, w2, -TAMBOR), TAMBOR, lados)
    # Uma marca no topo do tambor, para o giro aparecer.
    e.caixa("tambor", "aco_escuro", (-0.006, -w2 + 0.02, -TAMBOR + TAMBOR * 0.96), (0.006, w2 - 0.02, -TAMBOR + TAMBOR * 1.005), chanfro=0)
    for s, lado in ((1, "e"), (-1, "d")):
        y = s * w2
        no = e.no("mancal_" + lado, pivo=(0.0, y, -TAMBOR))
        # Caixa do mancal por fora da longarina, com o esticador da correia.
        e.caixa(no, "aco_escuro", (-0.06, min(y, y + s * 0.05), -TAMBOR - 0.05), (0.06, max(y, y + s * 0.05), -TAMBOR + 0.05), chanfro=0)
        e.cilindro(no, "aco", (0, y + s * 0.05, -TAMBOR), (0, y + s * 0.065, -TAMBOR), 0.022, 8)
        e.caixa(no, "aco", (-0.20, min(y + s * 0.02, y + s * 0.032), -TAMBOR - 0.008), (-0.06, max(y + s * 0.02, y + s * 0.032), -TAMBOR + 0.008), chanfro=0)
        if d:
            e.caixa(no, "amarelo", (0.06, min(y, y + s * 0.05), -TAMBOR - 0.05), (0.066, max(y, y + s * 0.05), -TAMBOR + 0.05), chanfro=0)
    e.dados = {"raio": TAMBOR, "largura_base": LARGURA_BASE, "estica": {"tambor": "y"}}
    e.clipe("rolar", True, {"tambor": [(0, "rotation_euler", 1, 0.0), (1, "rotation_euler", 1, TAU)]})
    return e


def esteira_canaleta(detalhe=0):
    """Suporte da guia lateral (canaleta). Origem no topo da longarina, na face de fora, do
    lado direito do sentido (`-y`); do outro lado, gira 180°. Nó `suporte`: sapata aparafusada
    na longarina, haste por fora da guia e gancho que abraça o topo dela. A guia fica
    `GUIA_FORA` para dentro da face da longarina e tem `GUIA_ESPESSURA`."""
    e = Elemento("esteira_canaleta", detalhe)
    d = detalhe
    e.no("suporte")
    # Sapata na face de fora da longarina, com dois parafusos.
    e.caixa("suporte", "aco_escuro", (-0.035, -0.008, -0.07), (0.035, 0.0, 0.0), chanfro=0)
    for x in (-0.02, 0.02):
        e.cilindro("suporte", "aco", (x, -0.008, -0.035), (x, -0.014, -0.035), 0.008, 6)
    # Haste que sobe por fora da guia, com um calço que encosta nela.
    e.caixa("suporte", "aco_escuro", (-0.012, -0.008, 0.0), (0.012, 0.0, GUIA + 0.008), chanfro=0)
    e.caixa("suporte", "aco", (-0.02, 0.0, GUIA * 0.45), (0.02, GUIA_FORA, GUIA * 0.45 + 0.015), chanfro=0)
    # Gancho: passa por cima da guia e desce por dentro.
    dentro = GUIA_FORA + GUIA_ESPESSURA
    e.caixa("suporte", "aco_escuro", (-0.02, -0.008, GUIA), (0.02, dentro + 0.006, GUIA + 0.008), chanfro=0)
    e.caixa("suporte", "aco_escuro", (-0.02, dentro, GUIA - 0.04), (0.02, dentro + 0.006, GUIA), chanfro=0)
    if d:
        e.cilindro("suporte", "aco", (0.0, -0.008, GUIA * 0.45 + 0.0075), (0.0, -0.018, GUIA * 0.45 + 0.0075), 0.009, 6)
    e.dados = {"altura": GUIA, "guia_fora": GUIA_FORA, "guia_espessura": GUIA_ESPESSURA}
    return e

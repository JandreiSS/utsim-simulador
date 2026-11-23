"""Logística: a doca, por onde as cargas entram e saem da fábrica.

Serve para a origem e para o destino das peças (recebimento e expedição), que não tinham
elemento próprio. O lado de dentro da fábrica fica em `-y`; o caminhão encosta em `+y`.
"""

from comum import Elemento

# Vão da porta (largura, altura), espessura da parede e a área de carga marcada no piso.
VAO = (2.6, 3.2)
PAREDE = 0.25
AREA = (2.6, 2.2)
# A porta de enrolar fechada vale escala 1 em z; aberta, fica só a barra de baixo no rolo.
ABERTA = 0.04


def doca(detalhe=0):
    """Doca de carga, 3,6 × 3,3 × 4,0 m: um trecho de parede com porta de enrolar, abrigo de
    lona por fora, niveladora, batentes, balizadores, semáforo e a área de carga marcada no
    piso. Nós: `parede`, `porta` (escala em z a partir do rolo, no alto), `luz`. Clipe
    `porta_abrir` (0 fechada, 1 aberta). `ponto_peca` é o centro da área de carga, no piso,
    do lado de dentro; `ponto_fora` é onde a carga está no caminhão; `ponto_operador` fica ao
    lado da área, olhando para ela."""
    e = Elemento("doca", detalhe)
    d = detalhe
    e.material("lona", 0x1B1C1F, 0.0, 0.95)
    e.material("vidro", 0x6F8FA8, 0.1, 0.15, alfa=0.45)
    e.material("luz", 0x46D160, 0.0, 0.4, emissao=True)
    lv, av = VAO[0] / 2, VAO[1]
    e.no("parede")
    # Parede: dois pilares e a verga, com a faixa e a placa do número da doca.
    e.caixa("parede", "pintura", (-1.8, 0.0, 0.0), (-lv, PAREDE, 4.0))
    e.caixa("parede", "pintura", (lv, 0.0, 0.0), (1.8, PAREDE, 4.0))
    e.caixa("parede", "pintura", (-lv, 0.0, av), (lv, PAREDE, 4.0))
    e.caixa("parede", "faixa", (-1.81, -0.01, 3.62), (1.81, PAREDE + 0.01, 3.74), chanfro=0)
    e.caixa("parede", "faixa", (-0.30, -0.03, 3.78), (0.30, -0.01, 3.96), chanfro=0)
    # Caixa do rolo da porta, por dentro, e as guias laterais.
    e.caixa("parede", "aco_escuro", (-lv - 0.08, -0.26, av), (lv + 0.08, 0.0, av + 0.36))
    for s in (-1, 1):
        x = s * (lv + 0.03)
        e.caixa("parede", "aco", (x - 0.03, -0.06, 0.0), (x + 0.03, 0.0, av), chanfro=0)
    # Balizadores amarelos por dentro, um de cada lado do vão.
    for s in (-1, 1):
        x = s * (lv + 0.22)
        e.cilindro("parede", "amarelo", (x, -0.40, 0.0), (x, -0.40, 1.05), 0.07, 10 if d else 6)
        e.cilindro("parede", "borracha", (x, -0.40, 0.70), (x, -0.40, 0.82), 0.072, 10 if d else 6)
    # Por fora: abrigo de lona em volta do vão, batentes de borracha e a niveladora.
    f = PAREDE
    e.caixa("parede", "lona", (-lv - 0.30, f, 0.55), (-lv, f + 0.45, av + 0.30), chanfro=0)
    e.caixa("parede", "lona", (lv, f, 0.55), (lv + 0.30, f + 0.45, av + 0.30), chanfro=0)
    e.caixa("parede", "lona", (-lv, f, av), (lv, f + 0.45, av + 0.30), chanfro=0)
    for s in (-1, 1):
        x = s * (lv - 0.20)
        e.caixa("parede", "borracha", (x - 0.12, f, 0.0), (x + 0.12, f + 0.12, 0.50))
    e.caixa("parede", "aco_escuro", (-1.0, -0.10, 0.0), (1.0, f + 0.75, 0.02), chanfro=0)
    e.caixa("parede", "amarelo", (-1.0, f + 0.69, 0.02), (1.0, f + 0.75, 0.024), chanfro=0)
    if d:
        for s in (-1, 1):  # bordas da niveladora e as faixas refletivas do abrigo
            e.caixa("parede", "amarelo", (s * 0.97 - 0.03, -0.10, 0.02), (s * 0.97 + 0.03, f + 0.69, 0.024), chanfro=0)
            x = s * (lv + 0.15)
            for k in range(4):
                z = 0.80 + k * 0.62
                e.caixa("parede", "amarelo", (x - 0.10, f + 0.45, z), (x + 0.10, f + 0.455, z + 0.22), chanfro=0)
    # Área de carga marcada no piso, do lado de dentro (contorno amarelo).
    ax, ay, fx = AREA[0] / 2, AREA[1], 0.08
    e.caixa("parede", "amarelo", (-ax, -0.20 - ay, 0.0), (ax, -0.20 - ay + fx, 0.004), chanfro=0)
    for s in (-1, 1):
        e.caixa("parede", "amarelo", (min(s * ax, s * (ax - fx)), -0.20 - ay, 0.0),
                (max(s * ax, s * (ax - fx)), -0.20, 0.004), chanfro=0)
    # Semáforo da doca por dentro, à direita: a `luz` acesa (verde) e a de cima apagada.
    e.caixa("parede", "painel", (lv + 0.34, -0.10, 2.05), (lv + 0.56, 0.0, 2.60))
    e.cilindro("parede", "vermelho", (lv + 0.45, -0.10, 2.46), (lv + 0.45, -0.115, 2.46), 0.075, 10 if d else 8)
    e.no("luz", pivo=(lv + 0.45, -0.10, 2.20))
    e.cilindro("luz", "luz", (lv + 0.45, -0.10, 2.20), (lv + 0.45, -0.115, 2.20), 0.075, 10 if d else 8)
    # Porta de enrolar: lâminas horizontais com uma fileira de visores; enrola para cima.
    e.no("porta", pivo=(0.0, 0.10, av))
    laminas = 8 if d else 4
    h = av / laminas
    visor = laminas // 2
    for k in range(laminas):
        z = k * h
        if k == visor and d:
            e.caixa("porta", "aco", (-lv, 0.08, z), (lv, 0.12, z + h * 0.25), chanfro=0)
            e.caixa("porta", "aco", (-lv, 0.08, z + h * 0.75), (lv, 0.12, z + h - 0.008), chanfro=0)
            for j in range(4):
                x = -lv + 0.25 + j * (VAO[0] - 0.5) / 4
                e.caixa("porta", "vidro", (x, 0.09, z + h * 0.25), (x + 0.42, 0.11, z + h * 0.75), chanfro=0)
                e.caixa("porta", "aco", (x + 0.42, 0.08, z + h * 0.25), (x + (VAO[0] - 0.5) / 4, 0.12, z + h * 0.75), chanfro=0)
            e.caixa("porta", "aco", (-lv, 0.08, z + h * 0.25), (-lv + 0.25, 0.12, z + h * 0.75), chanfro=0)
            e.caixa("porta", "aco", (lv - 0.25, 0.08, z + h * 0.25), (lv, 0.12, z + h * 0.75), chanfro=0)
        else:
            e.caixa("porta", "aco", (-lv, 0.08, z), (lv, 0.12, z + h - 0.008), chanfro=0)
    e.caixa("porta", "amarelo", (-lv, 0.07, 0.0), (lv, 0.13, 0.06), chanfro=0)  # barra de baixo
    # Pontos.
    centro = -0.20 - ay / 2
    e.ponto("ponto_peca", (0.0, centro, 0.0), 0)
    e.ponto("ponto_fora", (0.0, PAREDE + 1.20, 0.0), 0)
    e.ponto("ponto_operador", (ax + 0.55, centro, 0.0), 180)
    # Os valores do clipe somam ao repouso (escala 1): aberta, a escala em z é `ABERTA`.
    e.clipe("porta_abrir", False, {"porta": [(0, "scale", 2, 0.0), (1, "scale", 2, ABERTA - 1.0)]})
    e.dados = {"vao": list(VAO), "area": list(AREA), "parede": PAREDE, "porta_aberta": ABERTA}
    return e

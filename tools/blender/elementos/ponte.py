"""Ponte rolante birrail sobre caminho de rolamento em colunas.

Eixos: `+x` é o caminho de rolamento (a ponte anda em x), `y` é o vão (o carro anda em y) e
`z` é a altura. Origem no chão, no centro. Cada parte que estica com o tamanho do elemento é
um nó próprio, com a origem onde a escala não a tira do lugar (ver `docs/elementos_3d_blender.md`).
"""

import math

from comum import Elemento

COMPRIMENTO = 12.0  # caminho de rolamento (x)
VAO = 8.0  # entre os trilhos (y)
TRILHO = 6.0  # topo do trilho (z)
CABECEIRA = 3.2  # comprimento da cabeceira (x)
CURSO_X = COMPRIMENTO - CABECEIRA - 0.4
CURSO_Y = VAO - 2.4
VIGA_X = 0.8  # as duas vigas ficam em x = ±VIGA_X
VIGA_TOPO = TRILHO + 0.95
CARRO_Z = VIGA_TOPO + 0.05  # origem do carro: o topo do trilho das vigas
CABO_TOPO = CARRO_Z + 0.25  # de onde o cabo sai (o tambor)
GANCHO_Z = 5.2  # origem do gancho recolhido: o eixo das polias do moitão
PONTO_GANCHO = 0.62  # o ponto da carga fica isto abaixo da origem do gancho
CURSO_GANCHO = GANCHO_Z - PONTO_GANCHO - 0.4  # até 0,4 m do chão
BOTOEIRA_Z = 1.25


def _faixas(e, no, x0, x1, y0, y1, z0, z1, n, eixo=0, so_preto=False):
    """Padrão amarelo e preto: `n` blocos alternados ao longo de `eixo` (0 = x, 1 = y). Com
    `so_preto`, só os blocos pretos (o fundo já é amarelo)."""
    for k in range(n):
        cor = "amarelo" if k % 2 == 0 else "preto"
        if so_preto and cor == "amarelo":
            continue
        if eixo == 0:
            a, b = x0 + (x1 - x0) * k / n, x0 + (x1 - x0) * (k + 1) / n
            e.caixa(no, cor, (a, y0, z0), (b, y1, z1), chanfro=0)
        else:
            a, b = y0 + (y1 - y0) * k / n, y0 + (y1 - y0) * (k + 1) / n
            e.caixa(no, cor, (x0, a, z0), (x1, b, z1), chanfro=0)


def ponte_rolante(detalhe=0):
    """Ponte rolante birrail. Nós: `trilho_e`/`trilho_d` (esticam em x), `coluna_e0..2` e
    `coluna_d0..2` (esticam em z), `ponte` (anda em x) com `viga_a`/`viga_b` (esticam em y),
    `cabeceira_e`/`cabeceira_d`, `luz`, `cabine`, `cabo_botoeira` (estica em z) e `botoeira`;
    `carro` (filho de `ponte`, anda em y) com `cabo` (estica em z) e `gancho` (desce em z).
    Em repouso a ponte e o carro ficam no começo do curso e o gancho recolhido.
    Clipes `ponte_andar`, `carro_andar` e `gancho_descer`."""
    e = Elemento("ponte_rolante", detalhe)
    d = detalhe
    e.material("preto", 0x17181A, 0.1, 0.6)
    e.material("estrutura", 0x5E6B78, 0.4, 0.55)
    e.material("vidro", 0x6F8FA8, 0.1, 0.15, alfa=0.45)
    e.material("luz", 0xFF9D00, 0.0, 0.4, emissao=True)
    L2, V2 = COMPRIMENTO / 2, VAO / 2
    lados = 12 if d else 8

    # Caminho de rolamento: viga com trilho em cada lado, sobre três colunas com console.
    for s, lado in ((1, "e"), (-1, "d")):
        y = s * V2
        no = e.no("trilho_" + lado, pivo=(0.0, y, TRILHO))
        if d:  # perfil em I: mesas e alma
            e.caixa(no, "estrutura", (-L2, y - 0.15, TRILHO - 0.55), (L2, y + 0.15, TRILHO - 0.50), chanfro=0)
            e.caixa(no, "estrutura", (-L2, y - 0.03, TRILHO - 0.50), (L2, y + 0.03, TRILHO - 0.11), chanfro=0)
            e.caixa(no, "estrutura", (-L2, y - 0.15, TRILHO - 0.11), (L2, y + 0.15, TRILHO - 0.06), chanfro=0)
        else:
            e.caixa(no, "estrutura", (-L2, y - 0.15, TRILHO - 0.55), (L2, y + 0.15, TRILHO - 0.06), chanfro=0)
        e.caixa(no, "aco_escuro", (-L2, y - 0.03, TRILHO - 0.06), (L2, y + 0.03, TRILHO), chanfro=0)
        for sx in ((-1, 1) if d else ()):  # batentes nas pontas
            x = sx * (L2 - 0.08)
            e.caixa(no, "vermelho", (x - 0.08, y - 0.10, TRILHO), (x + 0.08, y + 0.10, TRILHO + 0.22), chanfro=0)
        for k, x in enumerate((-L2 + 0.25, 0.0, L2 - 0.25)):
            no = e.no(f"coluna_{lado}{k}", pivo=(x, y, 0.0))
            # A coluna fica sob a viga, com uma chapa de apoio em cima (o console).
            topo = TRILHO - 0.55
            e.caixa(no, "estrutura", (x - 0.16, y - 0.16, 0.0), (x + 0.16, y + 0.16, topo - (0.04 if d else 0.0)), chanfro=0)
            if d:
                e.caixa(no, "aco_escuro", (x - 0.22, y - 0.24, topo - 0.04), (x + 0.22, y + 0.24, topo), chanfro=0)
            # Proteção amarela e preta no pé.
            e.caixa(no, "amarelo", (x - 0.17, y - 0.17, 0.0), (x + 0.17, y + 0.17, 1.20), chanfro=0)
            for z in ((0.20, 0.60, 1.00) if d else (0.45,)):
                h = 0.20 if d else 0.30
                e.caixa(no, "preto", (x - 0.175, y - 0.175, z), (x + 0.175, y + 0.175, z + h), chanfro=0)

    # Ponte: duas vigas-caixão, cabeceiras com rodas, passadiço, luz de aviso.
    x0 = -CURSO_X / 2
    e.no("ponte", pivo=(x0, 0.0, TRILHO))
    for nome, s in (("viga_a", -1), ("viga_b", 1)):
        x = x0 + s * VIGA_X
        no = e.no(nome, "ponte", (x, 0.0, TRILHO))
        e.caixa(no, "amarelo", (x - 0.20, -V2 - 0.25, TRILHO + 0.20), (x + 0.20, V2 + 0.25, VIGA_TOPO), chanfro=0)
        e.caixa(no, "aco_escuro", (x - 0.03, -V2 + 0.3, VIGA_TOPO), (x + 0.03, V2 - 0.3, VIGA_TOPO + 0.05), chanfro=0)
    for s, lado in ((1, "e"), (-1, "d")):
        y = s * V2
        no = e.no("cabeceira_" + lado, "ponte", (x0, y, TRILHO))
        xa, xb = x0 - CABECEIRA / 2, x0 + CABECEIRA / 2
        e.caixa(no, "amarelo", (xa, y - 0.16, TRILHO + 0.04), (xb, y + 0.16, TRILHO + 0.40), chanfro=None if d else 0)
        _faixas(e, no, xa, xb, y + s * 0.16, y + s * 0.17, TRILHO + 0.06, TRILHO + 0.38, 16 if d else 8, so_preto=True)
        for x in (xa + 0.35, xb - 0.35):
            e.cilindro(no, "aco", (x, y - 0.05, TRILHO + 0.16), (x, y + 0.05, TRILHO + 0.16), 0.16, lados)
            if d:  # motorredutor da roda
                e.caixa(no, "aco_escuro", (x - 0.14, y - s * 0.16, TRILHO + 0.12), (x + 0.14, y - s * 0.34, TRILHO + 0.36), chanfro=0)
        for x in (xa - 0.06, xb):  # para-choques
            e.caixa(no, "preto", (x, y - 0.08, TRILHO + 0.12), (x + 0.06, y + 0.08, TRILHO + 0.30), chanfro=0)
    if d:  # passadiço com guarda-corpo ao longo da viga de trás
        xp = x0 - VIGA_X - 0.20
        e.caixa("ponte", "estrutura", (xp - 0.55, -V2 + 0.4, TRILHO + 0.60), (xp, V2 - 0.4, TRILHO + 0.63), chanfro=0)
        for k in range(7):
            y = (-V2 + 0.4) + k * (VAO - 0.8) / 6
            e.caixa("ponte", "amarelo", (xp - 0.55, y - 0.015, TRILHO + 0.63), (xp - 0.52, y + 0.015, TRILHO + 1.65), chanfro=0)
        for z in (TRILHO + 1.15, TRILHO + 1.65):
            e.caixa("ponte", "amarelo", (xp - 0.55, -V2 + 0.4, z - 0.02), (xp - 0.52, V2 - 0.4, z + 0.02), chanfro=0)
    e.no("luz", "ponte", (x0 + VIGA_X + 0.20, V2 - 0.6, TRILHO + 0.20))
    xl, yl = x0 + VIGA_X + 0.26, V2 - 0.6
    e.caixa("luz", "aco_escuro", (xl - 0.06, yl - 0.08, TRILHO + 0.30), (xl + 0.08, yl + 0.08, TRILHO + 0.36), chanfro=0)
    e.cilindro("luz", "luz", (xl + 0.02, yl, TRILHO + 0.36), (xl + 0.02, yl, TRILHO + 0.52), 0.07, lados)
    # Cabine pendurada sob a ponte, num dos lados (opcional: quem desenha pode esconder).
    yc = V2 - 1.5
    e.no("cabine", "ponte", (x0, yc, TRILHO))
    e.caixa("cabine", "amarelo", (x0 - 0.75, yc - 0.65, TRILHO - 1.95), (x0 + 0.75, yc + 0.65, TRILHO - 1.05), chanfro=None if d else 0)
    e.caixa("cabine", "vidro", (x0 - 0.73, yc - 0.63, TRILHO - 1.05), (x0 + 0.73, yc + 0.63, TRILHO - 0.25), chanfro=0)
    e.caixa("cabine", "amarelo", (x0 - 0.78, yc - 0.68, TRILHO - 0.25), (x0 + 0.78, yc + 0.68, TRILHO - 0.17), chanfro=0)
    e.caixa("cabine", "preto", (x0 - 0.76, yc - 0.66, TRILHO - 1.10), (x0 + 0.76, yc + 0.66, TRILHO - 1.04), chanfro=0)  # peitoril
    if d:  # painel de comando e banco, vistos pelo vidro
        e.caixa("cabine", "painel", (x0 + 0.35, yc - 0.40, TRILHO - 1.05), (x0 + 0.65, yc + 0.40, TRILHO - 0.80), chanfro=0)
        e.caixa("cabine", "borracha", (x0 - 0.45, yc - 0.22, TRILHO - 1.05), (x0 - 0.05, yc + 0.22, TRILHO - 0.95), chanfro=0)
        e.caixa("cabine", "borracha", (x0 - 0.52, yc - 0.22, TRILHO - 1.05), (x0 - 0.44, yc + 0.22, TRILHO - 0.50), chanfro=0)
    for sx in (-1, 1):
        for sy in (-1, 1):
            x, y = x0 + sx * 0.72, yc + sy * 0.62
            e.caixa("cabine", "aco_escuro", (x - 0.03, y - 0.03, TRILHO - 1.05), (x + 0.03, y + 0.03, TRILHO + 0.20), chanfro=0)
    # Botoeira pendente, do outro lado.
    xb_, yb = x0 + VIGA_X + 0.35, -V2 + 1.2
    e.no("cabo_botoeira", "ponte", (xb_, yb, TRILHO + 0.20))
    e.caixa("cabo_botoeira", "preto", (xb_ - 0.012, yb - 0.012, BOTOEIRA_Z + 0.16), (xb_ + 0.012, yb + 0.012, TRILHO + 0.20), chanfro=0)
    e.no("botoeira", "ponte", (xb_, yb, BOTOEIRA_Z))
    e.caixa("botoeira", "amarelo", (xb_ - 0.045, yb - 0.04, BOTOEIRA_Z - 0.16), (xb_ + 0.045, yb + 0.04, BOTOEIRA_Z + 0.16), chanfro=0)
    for k, cor in enumerate(("vermelho", "verde", "preto", "preto") if d else ("vermelho", "verde")):
        z = BOTOEIRA_Z + 0.10 - k * 0.065
        e.caixa("botoeira", cor, (xb_ + 0.045, yb - 0.02, z - 0.02), (xb_ + 0.055, yb + 0.02, z + 0.02), chanfro=0)
    e.ponto("ponto_operador", (xb_ - 0.55, yb, 0.0), 0, pai="ponte")

    # Carro (trole) sobre as vigas: chassi, rodas, tambor, motor e redutor.
    y0 = -CURSO_Y / 2
    e.no("carro", "ponte", (x0, y0, CARRO_Z))
    for s in (-1, 1):
        e.caixa("carro", "aco_escuro", (x0 - 1.0, y0 + s * 0.55 - 0.08, CARRO_Z + 0.10), (x0 + 1.0, y0 + s * 0.55 + 0.08, CARRO_Z + 0.26), chanfro=0)
        for sx in (-1, 1):
            x = x0 + sx * VIGA_X
            e.cilindro("carro", "aco", (x - 0.04, y0 + s * 0.55, CARRO_Z + 0.11), (x + 0.04, y0 + s * 0.55, CARRO_Z + 0.11), 0.11, lados)
    e.caixa("carro", "amarelo", (x0 - 0.62, y0 - 0.55, CARRO_Z + 0.12), (x0 + 0.62, y0 + 0.55, CARRO_Z + 0.20), chanfro=0)
    e.cilindro("carro", "aco", (x0 - 0.45, y0, CABO_TOPO + 0.15), (x0 + 0.45, y0, CABO_TOPO + 0.15), 0.20, lados)
    e.caixa("carro", "painel", (x0 + 0.45, y0 - 0.24, CARRO_Z + 0.20), (x0 + 0.75, y0 + 0.24, CARRO_Z + 0.68), chanfro=0)
    e.cilindro("carro", "faixa", (x0 + 0.75, y0, CARRO_Z + 0.44), (x0 + 1.10, y0, CARRO_Z + 0.44), 0.15, lados)
    e.caixa("carro", "painel", (x0 - 0.80, y0 - 0.30, CARRO_Z + 0.20), (x0 - 0.50, y0 + 0.30, CARRO_Z + 0.75), chanfro=0)
    if d:
        for s in (-1, 1):
            _faixas(e, "carro", x0 - 0.62, x0 + 0.62, y0 + s * 0.55, y0 + s * 0.56, CARRO_Z + 0.12, CARRO_Z + 0.20, 8, so_preto=True)
    # Cabo: do tambor ao moitão. A origem fica em cima; estica por escala em z.
    e.no("cabo", "carro", (x0, y0, CABO_TOPO))
    for x in (-0.13, -0.05, 0.05, 0.13) if d else (-0.10, 0.10):
        e.caixa("cabo", "aco_escuro", (x0 + x - 0.008, y0 - 0.008, GANCHO_Z), (x0 + x + 0.008, y0 + 0.008, CABO_TOPO), chanfro=0)
    # Moitão com gancho, amarelo e preto.
    e.no("gancho", "carro", (x0, y0, GANCHO_Z))
    e.cilindro("gancho", "aco", (x0 - 0.17, y0, GANCHO_Z), (x0 + 0.17, y0, GANCHO_Z), 0.15, lados)
    for s in (-1, 1):
        x = x0 + s * 0.19
        e.prisma("gancho", "amarelo", [(y0 - 0.19, GANCHO_Z + 0.17), (y0 + 0.19, GANCHO_Z + 0.17), (y0 + 0.19, GANCHO_Z - 0.12),
                                         (y0 + 0.07, GANCHO_Z - 0.30), (y0 - 0.07, GANCHO_Z - 0.30), (y0 - 0.19, GANCHO_Z - 0.12)],
                 x - 0.02, x + 0.02, eixo=0)
        e.caixa("gancho", "preto", (x + s * 0.02, y0 - 0.19, GANCHO_Z - 0.06), (x + s * 0.025, y0 + 0.19, GANCHO_Z + 0.02), chanfro=0)
    e.caixa("gancho", "aco_escuro", (x0 - 0.17, y0 - 0.06, GANCHO_Z - 0.32), (x0 + 0.17, y0 + 0.06, GANCHO_Z - 0.24), chanfro=0)
    # O gancho em si: um C forjado no plano yz, aberto para a frente (+y) em cima. O fundo da
    # sela (por dentro) é `ponto_gancho`; a haste desce do moitão até o topo do C.
    zg = GANCHO_Z - PONTO_GANCHO  # o fundo da sela, onde a carga apoia
    ri, ro = 0.10, 0.16
    zc = zg + ri
    n = 14 if d else 8
    fora, dentro = [], []
    for k in range(n + 1):
        ang = math.radians(70.0 + 305.0 * k / n)
        # A ponta afina no último trecho.
        fino = 0.022 * max(0.0, k - (n - 2)) / 2
        fora.append((y0 + (ro - fino) * math.cos(ang), zc + (ro - fino) * math.sin(ang)))
        dentro.append((y0 + (ri + fino) * math.cos(ang), zc + (ri + fino) * math.sin(ang)))
    meia = 0.035 if d else 0.03
    # Um trecho por segmento: a tampa de um perfil em C inteiro não sai certa na exportação.
    for k in range(n):
        e.prisma("gancho", "aco_escuro", [fora[k], fora[k + 1], dentro[k + 1], dentro[k]], x0 - meia, x0 + meia, eixo=0)
    e.caixa("gancho", "aco_escuro", (x0 - meia, y0 - 0.04, zc + ri), (x0 + meia, y0 + 0.04, GANCHO_Z - 0.32), chanfro=0)
    e.caixa("gancho", "aco", (x0 - 0.05, y0 - 0.055, GANCHO_Z - 0.36), (x0 + 0.05, y0 + 0.055, GANCHO_Z - 0.32), chanfro=0)  # porca
    if d:  # trava de segurança, da haste até a ponta
        pa, pb = (y0 + 0.04, zc + ri + 0.03), (y0 + 0.118, zc + 0.045)
        e.prisma("gancho", "amarelo", [pa, pb, (pb[0] + 0.008, pb[1] + 0.010), (pa[0] + 0.008, pa[1] + 0.010)],
                 x0 - 0.012, x0 + 0.012, eixo=0)
    e.ponto("ponto_gancho", (x0, y0, zg), 0, pai="gancho")

    e.dados = {
        "comprimento": COMPRIMENTO,
        "vao": VAO,
        "altura_trilho": TRILHO,
        "curso_x": round(CURSO_X, 4),
        "curso_y": round(CURSO_Y, 4),
        "curso_gancho": round(CURSO_GANCHO, 4),
        "gancho_repouso": round(zg, 4),
        "cabo_base": round(CABO_TOPO - GANCHO_Z, 4),
        "botoeira_altura": BOTOEIRA_Z,
        "cabo_botoeira_base": round(TRILHO + 0.20 - BOTOEIRA_Z - 0.16, 4),
        "margem_x": round((COMPRIMENTO - CURSO_X) / 2, 4),
        "margem_y": round((VAO - CURSO_Y) / 2, 4),
        # Os nós que esticam com o tamanho, e o eixo de cada um; os outros só mudam de lugar.
        "estica": {
            **{f"coluna_{lado}{k}": "z" for lado in "ed" for k in range(3)},
            "trilho_e": "x",
            "trilho_d": "x",
            "viga_a": "y",
            "viga_b": "y",
            "cabo": "z",
            "cabo_botoeira": "z",
        },
    }
    e.clipe("ponte_andar", False, {"ponte": [(0, "location", 0, 0.0), (1, "location", 0, CURSO_X)]})
    e.clipe("carro_andar", False, {"carro": [(0, "location", 1, 0.0), (1, "location", 1, CURSO_Y)]})
    base = CABO_TOPO - GANCHO_Z
    e.clipe("gancho_descer", False, {
        "gancho": [(0, "location", 2, 0.0), (1, "location", 2, -CURSO_GANCHO)],
        "cabo": [(0, "scale", 2, 0.0), (1, "scale", 2, CURSO_GANCHO / base)],
    })
    return e


def controle_remoto(detalhe=0):
    """Controle remoto (rádio) da ponte rolante, do tipo caixa de barriga: 0,26 × 0,15 × 0,08 m,
    com dois manetes, botão de emergência, chave, antena e as alças laterais onde as mãos
    seguram. Origem entre as duas mãos, no centro da caixa; `+x` é para a frente do operador.
    Nós: `corpo` e `alca` (a correia que sobe pelos ombros e passa atrás do pescoço; quem
    desenha pode esconder)."""
    e = Elemento("controle_remoto", detalhe)
    d = detalhe
    e.material("preto", 0x17181A, 0.1, 0.6)
    e.material("correia", 0x2B2D31, 0.0, 0.9)
    lados = 10 if d else 6
    e.no("corpo")
    # Caixa amarela com a tampa preta dos comandos e o para-choque de borracha em volta.
    e.caixa("corpo", "amarelo", (-0.075, -0.13, -0.04), (0.075, 0.13, 0.025), chanfro=None if d else 0)
    e.caixa("corpo", "preto", (-0.068, -0.123, 0.025), (0.068, 0.123, 0.032), chanfro=0)
    e.caixa("corpo", "borracha", (-0.08, -0.135, -0.012), (0.08, 0.135, 0.002), chanfro=0)
    # Dois manetes (ponte e carro de um lado, gancho do outro), com a coifa de borracha.
    for y in (-0.07, 0.07):
        e.cilindro("corpo", "borracha", (0.0, y, 0.032), (0.0, y, 0.045), 0.024, lados, raio2=0.012)
        e.cilindro("corpo", "aco", (0.0, y, 0.045), (0.008, y, 0.075), 0.006, 6)
        e.cilindro("corpo", "preto", (0.008, y, 0.075), (0.008, y, 0.092), 0.012, lados)
    # Botão de emergência (cogumelo vermelho), chave e três botões.
    e.cilindro("corpo", "amarelo", (0.035, 0.0, 0.032), (0.035, 0.0, 0.040), 0.022, lados)
    e.cilindro("corpo", "vermelho", (0.035, 0.0, 0.040), (0.035, 0.0, 0.058), 0.018, lados)
    e.cilindro("corpo", "aco", (-0.04, 0.0, 0.032), (-0.04, 0.0, 0.044), 0.008, 6)
    if d:
        for k, cor in enumerate(("verde", "amarelo", "preto")):
            y = -0.025 + k * 0.025
            e.cilindro("corpo", cor, (-0.005, y, 0.032), (-0.005, y, 0.038), 0.007, 6)
    # Antena curta e as alças laterais (o arco de proteção onde as mãos seguram).
    e.cilindro("corpo", "preto", (0.06, 0.11, 0.025), (0.06, 0.11, 0.085), 0.006, 6)
    for s in (-1, 1):
        y = s * 0.13
        for x in (-0.06, 0.06):
            e.caixa("corpo", "aco_escuro", (x - 0.006, min(y, y + s * 0.03), 0.0), (x + 0.006, max(y, y + s * 0.03), 0.012), chanfro=0)
        e.caixa("corpo", "aco_escuro", (-0.066, min(y + s * 0.024, y + s * 0.036), -0.002), (0.066, max(y + s * 0.024, y + s * 0.036), 0.014), chanfro=0)
    # Correia: sobe pelo peito, passa por cima dos ombros e fecha atrás do pescoço.
    e.no("alca")
    linha = [(-0.075, 0.0), (-0.115, 0.30), (-0.19, 0.50), (-0.30, 0.47)]
    esp = 0.006
    for s in (-1, 1):
        y = s * 0.10
        for (xa, za), (xb, zb) in zip(linha, linha[1:]):
            dx, dz = xb - xa, zb - za
            n = (dx * dx + dz * dz) ** 0.5
            ox, oz = -dz / n * esp / 2, dx / n * esp / 2
            e.prisma("alca", "correia", [(xa + ox, za + oz), (xb + ox, zb + oz), (xb - ox, zb - oz), (xa - ox, za - oz)],
                     y - 0.016, y + 0.016, eixo=1)
    e.caixa("alca", "correia", (-0.303, -0.116, 0.454), (-0.297, 0.116, 0.486), chanfro=0)
    e.dados = {"largura": 0.26, "pega": 0.32, "alca_altura": 0.50}
    return e

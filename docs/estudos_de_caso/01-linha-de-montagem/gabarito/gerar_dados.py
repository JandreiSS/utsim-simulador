"""Gera os CSVs de dados brutos do estudo de caso 1 (semente fixa).

Uso: python gerar_dados.py   (de dentro de gabarito/; grava em ../dados/)

As distribuições de origem ficam só aqui, no gabarito:
  tempos_inspecao.csv      300 valores  Gamma(shape 4, scale 25)           média 100 s
  tempos_calibracao.csv    250 valores  LogNormal(média 44, dp 6)          média 44 s
  tempos_reparo_solda.csv   60 valores  Exp(média 1500)                    MTTR 25 min
  entre_falhas_solda.csv    60 valores  Exp(média 7200)                    MTBF 2 h
"""
import math
import os
import random

pasta = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "dados")
os.makedirs(pasta, exist_ok=True)


def grava(nome, cabecalho, valores):
    with open(os.path.join(pasta, nome), "w", encoding="utf-8", newline="\n") as f:
        f.write(cabecalho + "\n")
        for i, v in enumerate(valores, 1):
            f.write("%d;%s\n" % (i, ("%.1f" % v).replace(".", ",")))


r = random.Random(105)
grava("tempos_inspecao.csv", "medida;tempo_s", [r.gammavariate(4.0, 25.0) for _ in range(300)])

r = random.Random(102)
media, dp = 44.0, 6.0
sigma2 = math.log(1.0 + (dp / media) ** 2)
mu = math.log(media) - sigma2 / 2.0
grava("tempos_calibracao.csv", "medida;tempo_s", [r.lognormvariate(mu, math.sqrt(sigma2)) for _ in range(250)])

r = random.Random(103)
grava("tempos_reparo_solda.csv", "parada;reparo_s", [r.expovariate(1.0 / 1500.0) for _ in range(60)])

r = random.Random(104)
grava("entre_falhas_solda.csv", "parada;entre_falhas_s", [r.expovariate(1.0 / 7200.0) for _ in range(60)])

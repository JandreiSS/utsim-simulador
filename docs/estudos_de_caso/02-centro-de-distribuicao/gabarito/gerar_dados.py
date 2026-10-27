"""Gera os CSVs de dados brutos do estudo de caso 2 (semente fixa).

Uso: python gerar_dados.py   (de dentro de gabarito/; grava em ../dados/)

As distribuições de origem ficam só aqui, no gabarito:
  tempos_picking.csv       300 valores  Triangular(35, 50, 80)        média 55 s
  chegadas_caminhoes.csv   120 instantes  Exp(média 1800) acumulada   (1 caminhão a cada 30 min)
  tempos_doca.csv          200 valores  Uniform(900, 1500)            média 1200 s
"""
import os
import random

pasta = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "dados")
os.makedirs(pasta, exist_ok=True)


def grava(nome, cabecalho, valores, casas=1):
    with open(os.path.join(pasta, nome), "w", encoding="utf-8", newline="\n") as f:
        f.write(cabecalho + "\n")
        for i, v in enumerate(valores, 1):
            f.write("%d;%s\n" % (i, (("%." + str(casas) + "f") % v).replace(".", ",")))


r = random.Random(201)
grava("tempos_picking.csv", "caixa;tempo_s", [r.triangular(35.0, 80.0, 50.0) for _ in range(300)])

r = random.Random(202)
t = 0.0
instantes = []
for _ in range(120):
    t += r.expovariate(1.0 / 1800.0)
    instantes.append(t)
grava("chegadas_caminhoes.csv", "caminhao;instante_s", instantes, casas=0)

r = random.Random(203)
grava("tempos_doca.csv", "caminhao;descarga_s", [r.uniform(900.0, 1500.0) for _ in range(200)], casas=0)

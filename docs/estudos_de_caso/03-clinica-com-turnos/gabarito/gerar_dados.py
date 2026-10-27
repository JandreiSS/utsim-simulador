"""Gera os CSVs de dados brutos do estudo de caso 3 (semente fixa).

Uso: python gerar_dados.py   (de dentro de gabarito/; grava em ../dados/)

As distribuições de origem ficam só aqui, no gabarito:
  consultas.csv   400 valores  mistura: 88% LogNormal(média 12, dp 5) e 12% LogNormal(média 25, dp 10)
                  (média da mistura 13,6 min; os urgentes demoram mais, mas o arquivo não diz quais são)
  triagem.csv     250 valores  Triangular(3, 5, 9) minutos, média 5,67 min
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


def lognormal(r, media, dp):
    sigma2 = math.log(1.0 + (dp / media) ** 2)
    mu = math.log(media) - sigma2 / 2.0
    return r.lognormvariate(mu, math.sqrt(sigma2))


r = random.Random(301)
consultas = []
for _ in range(400):
    if r.random() < 0.12:
        consultas.append(lognormal(r, 25.0, 10.0))
    else:
        consultas.append(lognormal(r, 12.0, 5.0))
grava("consultas.csv", "consulta;duracao_min", consultas)

r = random.Random(305)
grava("triagem.csv", "paciente;duracao_min", [r.triangular(3.0, 9.0, 5.0) for _ in range(250)])

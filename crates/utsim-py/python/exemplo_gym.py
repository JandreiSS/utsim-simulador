"""Exemplo: laço estilo Gym sobre o modelo roteamento_rl.ron.

Compara uma política aleatória com a heurística "menor atraso esperado" em 10 episódios.
Pré-requisito: compilar as ligações (veja a seção "Aprendizado por reforço" do README):
    cd crates/utsim-py && maturin develop --release
"""
import random
import statistics
from pathlib import Path

import utsim

MODELO = Path(__file__).resolve().parents[3] / "examples" / "roteamento_rl.ron"
BLOCOS = ["fila_rapida", "maquina_rapida", "fila_lenta", "maquina_lenta"]

env = utsim.Env(
    str(MODELO),
    # Recompensa = -(área sob o número de itens no sistema) entre decisões.
    reward=[(f"{b}.content.mean", -1.0, True) for b in BLOCOS],
    # Observação final: [relógio, itens na linha rápida, itens na linha lenta].
    observe_blocks=["fila_rapida+maquina_rapida", "fila_lenta+maquina_lenta"],
    episode_length=100.0,
)


def aleatoria(obs, rng):
    return rng.randrange(2)


def menor_atraso(obs, rng):
    rapida, lenta = obs[1], obs[2]
    atraso_rapido = (rapida + 1) * 0.8
    atraso_lento = 2.0 if lenta < 2 else lenta + 1
    return int(atraso_lento < atraso_rapido)


def episodio(politica, semente):
    rng = random.Random(semente)
    obs, info = env.reset(seed=semente)
    total, done = 0.0, False
    while not done:
        obs, recompensa, done, info = env.step(politica(obs, rng))
        total += recompensa
    return total


for nome, politica in [("aleatória", aleatoria), ("menor atraso", menor_atraso)]:
    retornos = [episodio(politica, s) for s in range(10_000, 10_010)]
    print(f"{nome:>12}: retorno médio {statistics.mean(retornos):.0f} "
          f"(desvio {statistics.stdev(retornos):.0f})")

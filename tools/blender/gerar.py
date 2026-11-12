"""Gera os elementos 3D do UTSim no Blender, sem janela.

    blender -b --factory-startup -P tools/blender/gerar.py -- [--saida PASTA] [--so NOME,...]

ou `bash scripts/blender.sh` (acha o Blender e usa a pasta padrão `assets/modelos3d`).
Grava `<nome>.glb` (detalhe baixo, o padrão), `<nome>_medio.glb` e `catalogo.json`
(tamanho, nós, pontos e clipes de cada arquivo, o que os testes conferem).
"""

import argparse
import os
import sys

AQUI = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, AQUI)
sys.path.insert(0, os.path.join(AQUI, "elementos"))

import comum  # noqa: E402
import maquinas  # noqa: E402
import movimentacao  # noqa: E402
import armazenagem  # noqa: E402
import postos  # noqa: E402
import pessoas  # noqa: E402
import clipes  # noqa: E402

ELEMENTOS = {
    "cnc": maquinas.cnc,
    "prensa": maquinas.prensa,
    "injetora": maquinas.injetora,
    "posto": postos.posto,
    "robo": postos.robo,
    "celula": postos.celula,
    "espera": postos.espera,
    "esteira": movimentacao.esteira,
    "empilhadeira": movimentacao.empilhadeira,
    "agv": movimentacao.agv,
    "porta_paletes": armazenagem.porta_paletes,
    "estante": armazenagem.estante,
    "peca": armazenagem.peca,
    "palete": armazenagem.palete,
    "caixa": armazenagem.caixa,
}

DETALHES = {0: "", 1: "_medio"}

# Pessoas (`pessoas.py`): um nível de detalhe só, no esqueleto do boneco UAL1.
PESSOAS = {"pessoa_homem": "homem", "pessoa_mulher": "mulher"}
# Clipes de pessoa corrigidos (`clipes.py`): só o esqueleto e as animações.
CLIPES = "pessoa_clipes"


def main():
    argv = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    p = argparse.ArgumentParser(prog="gerar.py")
    p.add_argument("--saida", default=os.path.join(AQUI, "..", "..", "assets", "modelos3d"))
    p.add_argument("--so", default="")
    a = p.parse_args(argv)
    pasta = os.path.abspath(a.saida)
    os.makedirs(pasta, exist_ok=True)
    nomes = [n for n in a.so.split(",") if n] or list(ELEMENTOS) + list(PESSOAS) + [CLIPES]
    caminho_cat = os.path.join(pasta, "catalogo.json")
    catalogo = {}
    if a.so and os.path.exists(caminho_cat):
        import json

        with open(caminho_cat, encoding="utf-8") as f:
            catalogo = json.load(f)
    for nome in nomes:
        if nome == CLIPES:
            item = clipes.gerar(pasta, nome)
            catalogo[nome] = item
            print(f"utsim-blender: {item['arquivo']} {sorted(item['clipes'])}")
            continue
        if nome in PESSOAS:
            item = pessoas.gerar(PESSOAS[nome], pasta)
            catalogo[nome] = item
            print(f"utsim-blender: {item['arquivo']} {item['triangulos']} triângulos, {item['tamanho']} m")
            continue
        for detalhe, sufixo in DETALHES.items():
            e = ELEMENTOS[nome](detalhe)
            item = e.gravar(pasta, nome + sufixo)
            catalogo[nome + sufixo] = item
            print(f"utsim-blender: {item['arquivo']} {item['triangulos']} triângulos, {item['tamanho']} m")
    comum.gravar_catalogo(pasta, catalogo)


main()

#!/usr/bin/env python3
"""Confere as regras editoriais do guia na prosa dos capitulos.

Ignora comentarios, listagens, tikz, tabelas, figuras, equacoes, matematica,
\\texttt e argumentos de \\label, \\ref, \\cite. Procura travessoes, ponto e
virgula, dois-pontos seguidos de letra e adjetivos proibidos.
"""
import glob
import os
import re
import sys

BASE = os.path.dirname(os.path.abspath(__file__))
PROIBIDAS = [
    "perfeito", "perfeita", "perfeitamente", "elegante", "enxuto", "enxuta",
    "poderoso", "poderosa", "absolutamente", "verdadeiramente", "extremamente",
    "incrivelmente", "fantástico", "fantástica", "maravilhoso", "maravilhosa",
    "brilhante", "sensacional", "espetacular", "magnífico", "magnífica",
]
ENVS = ["lstlisting", "tikzpicture", "tabular", "table", "figure", "equation",
        "align", "verbatim"]


def limpar(texto):
    texto = re.sub(r"(?<!\\)%.*", "", texto)
    for env in ENVS:
        texto = re.sub(r"\\begin\{%s\*?\}.*?\\end\{%s\*?\}" % (env, env), " ", texto, flags=re.S)
    texto = re.sub(r"\$[^$]*\$", " ", texto)
    texto = re.sub(r"\\texttt\{[^{}]*\}", " ", texto)
    texto = re.sub(r"\\(label|ref|cite|textcite|input|pagecolor)\{[^{}]*\}", " ", texto)
    return texto


def main():
    arquivos = sorted(glob.glob(os.path.join(BASE, "capitulos", "*.tex")))
    total = {"travessao": 0, "ponto_virgula": 0, "dois_pontos": 0, "adjetivo": 0}
    for arq in arquivos:
        texto = limpar(open(arq, encoding="utf-8").read())
        for n, linha in enumerate(texto.splitlines(), 1):
            achados = []
            if "---" in linha or "--" in linha or "\u2014" in linha or "\u2013" in linha:
                achados.append("travessao")
            if ";" in linha:
                achados.append("ponto_virgula")
            if re.search(r":\s*[A-Za-zÀ-ú]", linha):
                achados.append("dois_pontos")
            if any(re.search(r"\b%s\b" % p, linha, re.I) for p in PROIBIDAS):
                achados.append("adjetivo")
            for a in achados:
                total[a] += 1
                print("%s:%d [%s] %s" % (os.path.basename(arq), n, a, linha.strip()[:100]))
    print("Contagens na prosa:", total)
    return 0 if sum(total.values()) == 0 else 1


if __name__ == "__main__":
    sys.exit(main())

# Como o UTSim funciona

Guia em LaTeX, em português, que explica as ideias do UTSim com esquemas simples (TikZ). Não é o manual de consulta, que fica em `docs/manual/`.

## Compilar

Na pasta `docs/guia/`, com uma distribuição TeX (TeX Live ou MiKTeX) que tenha `biber`:

```bash
latexmk -pdf -outdir=build main.tex
```

Sem `latexmk` (ele precisa de Perl), rode a sequência manual:

```bash
pdflatex main.tex
biber main
pdflatex main.tex
pdflatex main.tex
```

No Overleaf, envie a pasta inteira, escolha `main.tex` como documento principal e deixe o compilador em pdfLaTeX. A bibliografia usa `biblatex` com `biber`, que o Overleaf executa sozinho.

O PDF pronto é `como_o_utsim_funciona.pdf`.

## Estrutura

| Arquivo | Conteúdo |
|---|---|
| `main.tex` | capa em TikZ, sumário, capítulos e referências |
| `config.tex` | pacotes, cores, estilos TikZ, `\epigrafe` e as caixas `sintese`, `analogia` e `definicao` |
| `capitulos/` | um arquivo por capítulo |
| `referencias.bib` | bibliografia |
| `verificar_estilo.py` | confere a prosa dos capítulos (sem travessões, ponto e vírgula, dois-pontos ou adjetivos proibidos) |

Os arquivos auxiliares e a pasta `build/` não vão para o Git.

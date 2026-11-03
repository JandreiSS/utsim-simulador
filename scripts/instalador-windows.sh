#!/usr/bin/env bash
# Gera o instalador do Windows (NSIS) a partir de target/release.
#
# Uso: bash scripts/instalador-windows.sh [versão] [pasta_de_saída]
#   versão          padrão: a de Cargo.toml (ex.: 0.1.0). Um prefixo "v" e sufixos como "-rc1" são ignorados.
#   pasta_de_saída  padrão: dist
#
# Antes: cargo build --release -p utsim-cli -p utsim-viewer
# Opcional: mdbook build docs/manual   (para levar o manual em HTML)
# Opcional: bash scripts/godot_exportar.sh   (leva o componente "UTSim 3D (prévia)", de
#           target/godot-export/windows; sem ele, o instalador sai sem esse componente)
# O makensis é procurado em $MAKENSIS, no PATH e nas pastas padrão do NSIS.
# O instalador só é gerado aqui, nunca executado.

set -eu

cd "$(dirname "$0")/.." || exit 1

VERSAO=${1:-$(awk -F'"' '/^version = /{print $2; exit}' Cargo.toml)}
VERSAO=${VERSAO#v}
ROTULO=$VERSAO
VERSAO=${VERSAO%%-*}
SAIDA=${2:-dist}

MAKENSIS=${MAKENSIS:-}
if [ -z "$MAKENSIS" ]; then
    if command -v makensis >/dev/null 2>&1; then
        MAKENSIS=$(command -v makensis)
    else
        for c in "/c/Program Files (x86)/NSIS/makensis.exe" "/c/Program Files/NSIS/makensis.exe"; do
            if [ -f "$c" ]; then
                MAKENSIS=$c
                break
            fi
        done
    fi
fi
[ -n "$MAKENSIS" ] || { echo "erro: makensis não encontrado (defina MAKENSIS)" >&2; exit 1; }

PASTA=$(mktemp -d)
trap 'rm -rf "$PASTA"' EXIT
bash scripts/preparar-pacote.sh "$PASTA/utsim" target/release target/godot-export/windows

mkdir -p "$SAIDA"
ARQUIVO="utsim-v${ROTULO}-windows-x64-instalador.exe"

# O makensis do Windows entende caminhos do Windows, não os do Git Bash.
nativo() { if command -v cygpath >/dev/null 2>&1; then cygpath -w "$1"; else echo "$1"; fi; }

MSYS2_ARG_CONV_EXCL='*' "$MAKENSIS" -V2 \
    "-DVERSAO=$VERSAO" \
    "-DORIGEM=$(nativo "$PASTA/utsim")" \
    "-DICONE=$(nativo "$PWD/assets/icone/utsim.ico")" \
    "-DSAIDA=$(nativo "$PWD/$SAIDA/$ARQUIVO")" \
    "$(nativo "$PWD/empacotamento/windows/utsim.nsi")"

echo "Instalador gerado: $SAIDA/$ARQUIVO"

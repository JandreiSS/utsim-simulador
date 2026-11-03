#!/usr/bin/env bash
# Publica o editor UTSim pelo launcher: monta a pasta do programa (a mesma do instalador),
# cifra, assina e grava o pacote e o manifesto na pasta de publicação (a pasta do Google Drive).
# Não envia nada à internet: o Google Drive para computador sincroniza a pasta.
#
# Uso: bash scripts/publicar-utsim.sh <pasta_de_publicação> [canal] [versão]
#   canal   padrão: testes (ou estavel)
#   versão  padrão: a de Cargo.toml
#
# Antes: cargo build --release -p utsim-cli -p utsim-viewer -p utsim-publicar
# Notas da versão: a seção do CHANGELOG.md (scripts/notas-versao.sh).

set -eu

cd "$(dirname "$0")/.." || exit 1

[ $# -ge 1 ] || { echo "uso: $0 <pasta_de_publicação> [canal] [versão]" >&2; exit 2; }
SAIDA=$1
CANAL=${2:-testes}
VERSAO=${3:-$(awk -F'"' '/^version = /{print $2; exit}' Cargo.toml)}
VERSAO=${VERSAO#v}

PUBLICAR=target/release/utsim-publicar
[ -x "$PUBLICAR" ] || PUBLICAR=target/release/utsim-publicar.exe
[ -x "$PUBLICAR" ] || { echo "erro: rode antes cargo build --release -p utsim-publicar" >&2; exit 1; }

case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*) PLATAFORMA=windows-x64; EXE=bin/utsim-view.exe ;;
    Linux) PLATAFORMA=linux-x64; EXE=bin/utsim-view ;;
    *) echo "erro: plataforma não suportada" >&2; exit 1 ;;
esac

PASTA=$(mktemp -d)
trap 'rm -rf "$PASTA"' EXIT
# A interface 3D entra quando já foi exportada (scripts/godot_exportar.sh).
GODOT_EXPORT=target/godot-export/${PLATAFORMA%%-*}
if [ -f "$GODOT_EXPORT/utsim-3d.pck" ]; then
    bash scripts/preparar-pacote.sh "$PASTA/utsim" target/release "$GODOT_EXPORT"
else
    bash scripts/preparar-pacote.sh "$PASTA/utsim" target/release
fi
bash scripts/notas-versao.sh "$VERSAO" > "$PASTA/notas.md" || echo "(sem notas)" > "$PASTA/notas.md"

"$PUBLICAR" publicar \
    --saida "$SAIDA" \
    --produto utsim \
    --nome "UTSim" \
    --descricao "Editor de modelos de simulação a eventos discretos" \
    --versao "$VERSAO" \
    --canal "$CANAL" \
    --plataforma "$PLATAFORMA" \
    --pasta "$PASTA/utsim" \
    --executavel "$EXE" \
    --arg=--novo \
    --notas "$PASTA/notas.md"

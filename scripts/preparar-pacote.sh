#!/usr/bin/env bash
# Monta a árvore de arquivos que vai dentro dos pacotes (instalador do Windows, .deb, .tar.gz):
#
#   <destino>/bin/utsim[.exe]  bin/utsim-view[.exe]
#   <destino>/examples/        modelos de exemplo
#   <destino>/assets/          modelos 3D, terceiros (com licenças) e ícones
#   <destino>/manual/          manual em HTML (se docs/manual/book existir) ou em Markdown
#   <destino>/LICENSE  README.md  CHANGELOG.md
#
# Uso: bash scripts/preparar-pacote.sh <destino> [pasta_dos_binarios]
# A pasta dos binários padrão é target/release (rode antes:
#   cargo build --release -p utsim-cli -p utsim-viewer).

set -eu

cd "$(dirname "$0")/.." || exit 1

if [ $# -lt 1 ]; then
    echo "uso: $0 <destino> [pasta_dos_binarios]" >&2
    exit 2
fi
DESTINO=$1
BINS=${2:-target/release}

if [ -e "$DESTINO" ] && [ -n "$(ls -A "$DESTINO" 2>/dev/null)" ]; then
    echo "erro: $DESTINO já existe e não está vazio" >&2
    exit 1
fi
mkdir -p "$DESTINO/bin"

achou=0
for nome in utsim utsim-view; do
    if [ -f "$BINS/$nome.exe" ]; then
        cp "$BINS/$nome.exe" "$DESTINO/bin/"
    elif [ -f "$BINS/$nome" ]; then
        cp "$BINS/$nome" "$DESTINO/bin/"
    else
        echo "erro: $nome não encontrado em $BINS" >&2
        exit 1
    fi
    achou=$((achou + 1))
done

cp -r examples "$DESTINO/examples"
cp -r assets "$DESTINO/assets"

if [ -f docs/manual/book/index.html ]; then
    mkdir -p "$DESTINO/manual"
    cp -r docs/manual/book/. "$DESTINO/manual/"
else
    echo "aviso: manual em HTML não encontrado (mdbook build docs/manual); levando o Markdown" >&2
    cp -r docs/manual/src "$DESTINO/manual"
fi

cp LICENSE README.md CHANGELOG.md "$DESTINO/"
echo "Pacote montado em $DESTINO ($achou programas)"

#!/usr/bin/env bash
# Monta a árvore de arquivos que vai dentro dos pacotes (instalador do Windows, .deb, .tar.gz):
#
#   <destino>/bin/utsim[.exe]  bin/utsim-view[.exe]
#   <destino>/examples/        modelos de exemplo
#   <destino>/assets/          modelos 3D, terceiros (com licenças) e ícones
#   <destino>/manual/          manual em HTML (se docs/manual/book existir) ou em Markdown
#   <destino>/3d/              UTSim 3D (prévia), se houver a exportação do Godot (veja abaixo)
#   <destino>/README.md  CHANGELOG.md
#
# Uso: bash scripts/preparar-pacote.sh <destino> [pasta_dos_binarios] [pasta_da_exportacao_godot]
# A exportação do Godot (scripts/godot_exportar.sh) é opcional: sem a pasta, o pacote sai sem a
# pasta 3d/. Ela entra como está (utsim-3d, .pck, biblioteca da ponte e AVISOS-TERCEIROS.txt).
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
GODOT_EXPORT=${3:-}

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

if [ -n "$GODOT_EXPORT" ]; then
    if [ -f "$GODOT_EXPORT/utsim-3d.pck" ] && [ -f "$GODOT_EXPORT/AVISOS-TERCEIROS.txt" ]; then
        mkdir -p "$DESTINO/3d"
        cp "$GODOT_EXPORT"/* "$DESTINO/3d/"
        echo "UTSim 3D (prévia) incluído de $GODOT_EXPORT"
    else
        echo "aviso: $GODOT_EXPORT sem exportação completa do Godot; pacote sem a pasta 3d/" >&2
    fi
fi

cp README.md CHANGELOG.md "$DESTINO/"
echo "Pacote montado em $DESTINO ($achou programas)"

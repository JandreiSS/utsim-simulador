#!/usr/bin/env bash
# Usa uma cópia local da família em vez da tag do GitHub, para mudar o núcleo e o app juntos.
#
#   bash scripts/familia-local.sh liga [pasta]   pasta padrão: ../utsim-familia
#   bash scripts/familia-local.sh desliga
#
# "liga" grava .cargo/config.toml (fora do git) com um [patch] que troca cada crate da família
# pelo da pasta. Quando o núcleo estiver pronto: verificar e commit na família, tag nova,
# depois "desliga" aqui e bash scripts/atualizar-familia.sh <tag>. O verificar.sh recusa
# rodar com a família local ligada, para nada ir ao GitHub apontando para a sua pasta.

set -eu
cd "$(dirname "$0")/.." || exit 1
CFG=.cargo/config.toml
case "${1:-}" in
liga)
    PASTA=${2:-../utsim-familia}
    [ -f "$PASTA/Cargo.toml" ] || { echo "não achei a família em $PASTA (git clone https://github.com/JandreiSS/utsim-familia $PASTA)" >&2; exit 1; }
    PASTA=$(cd "$PASTA" && (pwd -W 2>/dev/null || pwd))
    mkdir -p .cargo
    {
        echo '# Família local (scripts/familia-local.sh). Não vai para o git.'
        echo '[patch."https://github.com/JandreiSS/utsim-familia"]'
        for c in $(grep -oE '^utsim-[a-z0-9-]+ = \{ git = "https://github.com/JandreiSS/utsim-familia"' Cargo.toml | cut -d' ' -f1); do
            echo "$c = { path = \"$PASTA/crates/$c\" }"
        done
    } > "$CFG"
    echo "família local ligada: $PASTA"
    ;;
desliga)
    rm -f "$CFG"
    rmdir .cargo 2>/dev/null || true
    echo "família local desligada: volta a tag do Cargo.toml"
    ;;
*)
    echo "uso: $0 liga [pasta] | desliga" >&2
    exit 2
    ;;
esac

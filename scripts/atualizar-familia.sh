#!/usr/bin/env bash
# Passa este repositório para outra tag da família (utsim-familia) e atualiza o Cargo.lock.
# Uso: bash scripts/atualizar-familia.sh v0.2.0
# Depois: bash scripts/verificar.sh, e um commit "Família v0.2.0".

set -eu
cd "$(dirname "$0")/.." || exit 1
[ $# -eq 1 ] || { echo "uso: $0 <tag da família, ex.: v0.2.0>" >&2; exit 2; }
TAG=$1
git ls-remote --exit-code --tags https://github.com/JandreiSS/utsim-familia "refs/tags/$TAG" >/dev/null \
    || { echo "a tag $TAG não existe em utsim-familia" >&2; exit 1; }
sed -i -E "s#(git = \"https://github.com/JandreiSS/utsim-familia\", tag = \")[^\"]*\"#\1$TAG\"#" Cargo.toml
cargo update -q -p utsim-idiomas
grep -c "tag = \"$TAG\"" Cargo.toml | xargs echo "dependências da família na $TAG:"

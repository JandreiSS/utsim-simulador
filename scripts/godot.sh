#!/usr/bin/env bash
# Prepara a interface Godot (pasta godot/):
#   1. compila a ponte em Rust (crates/utsim-godot) e copia a biblioteca para godot/bin/;
#   2. copia os modelos .glb do repositório para godot/importados/ (a biblioteca de
#      assets/modelos, os elementos do Blender de assets/modelos3d e o boneco animado de
#      assets/terceiros/quaternius_ual*);
#   3. roda a importação do Godot (gera o cache em godot/.godot/).
# As cópias e o cache ficam fora do git (godot/.gitignore): os originais continuam só em
# assets/, e o Godot pode escrever seus arquivos .import sem tocar neles.
#
# Uso: scripts/godot.sh [--release] [--sem-importar] [--abrir]
#   --release       compila a ponte otimizada (reprodução mais leve);
#   --sem-importar  pula o passo 3;
#   --abrir         abre o projeto no Godot no fim.
# O executável do Godot vem de GODOT; sem ela, de `godot` no PATH ou da pasta
# ~/Documents/Godot_v4.7*.

set -u

cd "$(dirname "$0")/.." || exit 1

falha() {
    echo "FALHOU: $1" >&2
    exit 1
}

PERFIL=debug
FLAGS=()
IMPORTAR=1
ABRIR=0
for arg in "$@"; do
    case "$arg" in
        --release) PERFIL=release; FLAGS=(--release) ;;
        --sem-importar) IMPORTAR=0 ;;
        --abrir) ABRIR=1 ;;
        *) falha "opção desconhecida: $arg" ;;
    esac
done

# O executável do console (Windows) escreve no terminal; o outro abre a janela.
acha_godot() {
    local console="${1:-1}"
    if [ -n "${GODOT:-}" ]; then
        echo "$GODOT"
        return
    fi
    if command -v godot >/dev/null 2>&1; then
        command -v godot
        return
    fi
    local nome='Godot_v4.7*_win64.exe'
    [ "$console" = 1 ] && nome='Godot_v4.7*_console.exe'
    find "$HOME/Documents" -maxdepth 2 -name "$nome" 2>/dev/null | sort | tail -n 1
}

echo "==> 1/3 compilando a ponte (cargo build -p utsim-godot, $PERFIL)"
cargo build -p utsim-godot "${FLAGS[@]}" || falha "a ponte utsim-godot não compilou"
mkdir -p godot/bin
case "$(uname -s)" in
    MINGW* | MSYS* | CYGWIN*) LIB="utsim_godot.dll" ;;
    Darwin) LIB="libutsim_godot.dylib" ;;
    *) LIB="libutsim_godot.so" ;;
esac
cp "target/$PERFIL/$LIB" "godot/bin/$LIB" ||
    falha "não consegui copiar $LIB (o Godot está aberto com o projeto? feche-o e rode de novo)"

echo "==> 2/3 copiando os modelos para godot/importados"
mkdir -p godot/importados/modelos
cp -u assets/modelos/*.glb godot/importados/modelos/ || falha "cópia de assets/modelos"
mkdir -p godot/importados/modelos3d
cp -u assets/modelos3d/*.glb godot/importados/modelos3d/ || falha "cópia de assets/modelos3d"
for pacote in quaternius_ual quaternius_ual2; do
    mkdir -p "godot/importados/terceiros/$pacote"
    cp -u "assets/terceiros/$pacote/"*.glb "assets/terceiros/$pacote/"LICENCA*.txt \
        "godot/importados/terceiros/$pacote/" || falha "cópia de assets/terceiros/$pacote"
done

GODOT_BIN="$(acha_godot 1)"
if [ "$IMPORTAR" = 1 ]; then
    echo "==> 3/3 importação do Godot"
    [ -n "$GODOT_BIN" ] || falha "Godot não encontrado: defina GODOT com o caminho do executável"
    "$GODOT_BIN" --headless --path godot --import || falha "a importação do Godot falhou"
else
    echo "==> 3/3 importação pulada"
fi

if [ "$ABRIR" = 1 ]; then
    JANELA="$(acha_godot 0)"
    [ -n "$JANELA" ] || JANELA="$GODOT_BIN"
    "$JANELA" --path godot &
fi

echo
echo "Pronto: godot/bin/$LIB"

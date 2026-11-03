#!/usr/bin/env bash
# Baixa o Godot 4.7.2 para o CI (a versão do console, sem janela), confere o SHA512 publicado
# no release e deixa o caminho do executável em GODOT (no $GITHUB_ENV, se existir, e na
# última linha da saída).
#
# Uso: bash scripts/godot_instalar_ci.sh <linux|windows> [pasta_de_destino]
# Os modelos de exportação são baixados por scripts/godot_exportar.sh --instalar-modelos.

set -eu

PLATAFORMA=${1:?uso: $0 <linux|windows> [pasta_de_destino]}
DESTINO=${2:-.godot-ci}
VERSAO=4.7.2
BASE="https://github.com/godotengine/godot/releases/download/$VERSAO-stable"

case "$PLATAFORMA" in
    linux) ZIP="Godot_v$VERSAO-stable_linux.x86_64.zip"; EXE="Godot_v$VERSAO-stable_linux.x86_64" ;;
    windows) ZIP="Godot_v$VERSAO-stable_win64.exe.zip"; EXE="Godot_v$VERSAO-stable_win64_console.exe" ;;
    *) echo "plataforma desconhecida: $PLATAFORMA" >&2; exit 2 ;;
esac

descompactar() {
    if command -v unzip >/dev/null 2>&1; then
        unzip -q -o "$1" -d "$2"
    else
        # Git Bash do Windows pode não trazer o unzip.
        "$(command -v python3 || command -v python)" -m zipfile -e "$1" "$2"
    fi
}

mkdir -p "$DESTINO"
curl -fL --retry 3 -o "$DESTINO/$ZIP" "$BASE/$ZIP"
curl -fL --retry 3 -o "$DESTINO/SHA512-SUMS.txt" "$BASE/SHA512-SUMS.txt"
ESPERADO=$(grep -F " $ZIP" "$DESTINO/SHA512-SUMS.txt" | awk '{print $1}')
[ -n "$ESPERADO" ] || { echo "hash de $ZIP não encontrado" >&2; exit 1; }
OBTIDO=$(sha512sum "$DESTINO/$ZIP" | awk '{print $1}')
[ "$ESPERADO" = "$OBTIDO" ] || { echo "SHA512 de $ZIP não confere" >&2; exit 1; }
echo "SHA512 de $ZIP conferido"

descompactar "$DESTINO/$ZIP" "$DESTINO"
[ -f "$DESTINO/$EXE" ] || { echo "$EXE não está em $ZIP" >&2; exit 1; }
chmod +x "$DESTINO/$EXE"
CAMINHO="$(cd "$DESTINO" && pwd)/$EXE"
[ -z "${GITHUB_ENV:-}" ] || echo "GODOT=$CAMINHO" >> "$GITHUB_ENV"
echo "$CAMINHO"

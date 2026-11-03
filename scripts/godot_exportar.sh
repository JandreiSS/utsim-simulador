#!/usr/bin/env bash
# Exporta a interface 3D (pasta godot/) como programa independente, sem abrir janela:
#   1. compila a ponte em release, copia os .glb e importa (scripts/godot.sh --release);
#   2. exporta com `--headless --export-release` (presets em godot/export_presets.cfg);
#   3. gera o AVISOS-TERCEIROS.txt (copyright e licenças do motor Godot, lidos do executável);
#   4. deixa tudo em target/godot-export/<plataforma>/:
#        utsim-3d[.exe | .x86_64]  utsim-3d.pck  utsim_godot.dll | libutsim_godot.so
#        AVISOS-TERCEIROS.txt
#
# Uso: scripts/godot_exportar.sh [--plataforma windows|linux] [--instalar-modelos]
#   --plataforma        padrão: a do computador. A ponte é compilada para o sistema atual, então
#                       cada plataforma se exporta no seu próprio sistema (o CI faz as duas).
#   --instalar-modelos  baixa os modelos de exportação do Godot (~1 GB), confere o SHA512 do
#                       release e instala (só se ainda não estiverem instalados).
# O executável do Godot vem de GODOT (o do console, no Windows), de `godot` no PATH ou de
# ~/Documents/Godot_v4.7*. Os modelos de exportação vêm da versão do Godot encontrado.

set -u

cd "$(dirname "$0")/.." || exit 1

falha() {
    echo "FALHOU: $1" >&2
    exit 1
}

VERSAO_GODOT=4.7.2
PLATAFORMA=""
INSTALAR=0
while [ $# -gt 0 ]; do
    case "$1" in
        --plataforma) shift; PLATAFORMA="${1:-}" ;;
        --instalar-modelos) INSTALAR=1 ;;
        *) falha "opção desconhecida: $1" ;;
    esac
    shift
done

case "$(uname -s)" in
    MINGW* | MSYS* | CYGWIN*) HOST=windows ;;
    *) HOST=linux ;;
esac
[ -n "$PLATAFORMA" ] || PLATAFORMA=$HOST
[ "$PLATAFORMA" = "$HOST" ] ||
    falha "a ponte é compilada para $HOST: exporte '$PLATAFORMA' num sistema $PLATAFORMA"
case "$PLATAFORMA" in
    windows) PRESET="Windows Desktop"; EXE=utsim-3d.exe; LIB=utsim_godot.dll ;;
    linux) PRESET="Linux"; EXE=utsim-3d.x86_64; LIB=libutsim_godot.so ;;
    *) falha "plataforma desconhecida: $PLATAFORMA (use windows ou linux)" ;;
esac

# O console (Windows) escreve no terminal; fora do Windows é o mesmo executável.
GODOT_BIN="${GODOT:-}"
[ -n "$GODOT_BIN" ] || GODOT_BIN="$(command -v godot 2>/dev/null || true)"
[ -n "$GODOT_BIN" ] ||
    GODOT_BIN="$(find "$HOME/Documents" -maxdepth 2 -name 'Godot_v4.7*_console.exe' 2>/dev/null | sort | tail -n 1)"
[ -n "$GODOT_BIN" ] || falha "Godot não encontrado: defina GODOT com o caminho do executável"

# Pasta dos modelos de exportação: <dados>/Godot/export_templates/<versão>.stable.
if [ "$HOST" = windows ]; then
    PASTA_MODELOS="${APPDATA:-$HOME/AppData/Roaming}/Godot/export_templates/$VERSAO_GODOT.stable"
else
    PASTA_MODELOS="${XDG_DATA_HOME:-$HOME/.local/share}/godot/export_templates/$VERSAO_GODOT.stable"
fi

descompactar() {
    if command -v unzip >/dev/null 2>&1; then
        unzip -q -o "$1" -d "$2"
    else
        # Git Bash do Windows pode não trazer o unzip.
        "$(command -v python3 || command -v python)" -m zipfile -e "$1" "$2"
    fi
}

if [ ! -f "$PASTA_MODELOS/version.txt" ]; then
    [ "$INSTALAR" = 1 ] ||
        falha "modelos de exportação do Godot $VERSAO_GODOT não instalados em $PASTA_MODELOS (rode com --instalar-modelos ou use Editor > Gerenciar modelos de exportação)"
    BASE="https://github.com/godotengine/godot/releases/download/$VERSAO_GODOT-stable"
    TPZ="Godot_v$VERSAO_GODOT-stable_export_templates.tpz"
    TMP="$(mktemp -d)"
    trap 'rm -rf "$TMP"' EXIT
    echo "==> baixando os modelos de exportação ($TPZ)"
    curl -fL --retry 3 -o "$TMP/$TPZ" "$BASE/$TPZ" || falha "download dos modelos"
    curl -fL --retry 3 -o "$TMP/SHA512-SUMS.txt" "$BASE/SHA512-SUMS.txt" || falha "download do SHA512-SUMS.txt"
    ESPERADO="$(grep -F " $TPZ" "$TMP/SHA512-SUMS.txt" | awk '{print $1}')"
    [ -n "$ESPERADO" ] || falha "hash de $TPZ não encontrado em SHA512-SUMS.txt"
    OBTIDO="$(sha512sum "$TMP/$TPZ" | awk '{print $1}')"
    [ "$ESPERADO" = "$OBTIDO" ] || falha "SHA512 dos modelos não confere (esperado $ESPERADO, obtido $OBTIDO)"
    echo "SHA512 conferido"
    mkdir -p "$TMP/x" "$PASTA_MODELOS"
    descompactar "$TMP/$TPZ" "$TMP/x" || falha "não consegui abrir $TPZ"
    cp -r "$TMP/x/templates/." "$PASTA_MODELOS/" || falha "cópia dos modelos"
fi

echo "==> 1/3 ponte em release e importação"
bash scripts/godot.sh --release || falha "preparo do projeto Godot"

SAIDA="target/godot-export/$PLATAFORMA"
rm -rf "$SAIDA"
mkdir -p "$SAIDA"

echo "==> 2/3 exportando ($PRESET)"
LOG="$(mktemp)"
"$GODOT_BIN" --headless --path godot --export-release "$PRESET" "../$SAIDA/$EXE" 2>&1 | tee "$LOG"
STATUS=${PIPESTATUS[0]}
# O Godot sai com 0 mesmo com alguns erros de exportação: procure-os na saída.
if grep -qE "^ERROR:|SCRIPT ERROR|Parse Error" "$LOG"; then
    STATUS=1
fi
rm -f "$LOG"
[ "$STATUS" -eq 0 ] || falha "a exportação do Godot falhou"
[ -f "$SAIDA/$EXE" ] || falha "$SAIDA/$EXE não foi gerado"
[ -f "$SAIDA/utsim-3d.pck" ] || falha "$SAIDA/utsim-3d.pck não foi gerado"
# O Godot copia a biblioteca da ponte ao lado do executável; se não o fez, copie.
[ -f "$SAIDA/$LIB" ] || cp "godot/bin/$LIB" "$SAIDA/$LIB" || falha "biblioteca da ponte ausente"

echo "==> 3/3 avisos de terceiros do motor"
"$GODOT_BIN" --headless --path godot --script res://../scripts/godot_avisos.gd -- \
    "$(cd "$SAIDA" && pwd -W 2>/dev/null || pwd)/AVISOS-TERCEIROS.txt" ||
    falha "não consegui gerar os avisos de terceiros"
[ -s "$SAIDA/AVISOS-TERCEIROS.txt" ] || falha "AVISOS-TERCEIROS.txt vazio"

echo
echo "Pronto: $SAIDA"
ls -l "$SAIDA"

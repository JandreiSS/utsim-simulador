#!/usr/bin/env bash
# Teste sem janela da interface Godot: prepara o projeto (scripts/godot.sh) e roda
# godot/testes/teste.gd no console do Godot, que confere se a extensão carrega e se cada
# modelo abre, simula, monta a cena e é reproduzido até o fim. Sai com 1 se algo falhar.
#
# Uso: scripts/godot_teste.sh [modelo.ron ...]
#   sem argumentos: examples/centro_de_distribuicao.ron, examples/agvs.ron e
#   examples/pessoas_carregando.ron.
# Precisa do Godot 4.7 (GODOT, `godot` no PATH ou ~/Documents/Godot_v4.7*).

set -u

cd "$(dirname "$0")/.." || exit 1

falha() {
    echo "FALHOU: $1" >&2
    exit 1
}

bash scripts/godot.sh || falha "preparo do projeto Godot"

GODOT_BIN="${GODOT:-}"
[ -n "$GODOT_BIN" ] || GODOT_BIN="$(command -v godot 2>/dev/null || true)"
[ -n "$GODOT_BIN" ] ||
    GODOT_BIN="$(find "$HOME/Documents" -maxdepth 2 -name 'Godot_v4.7*_console.exe' 2>/dev/null | sort | tail -n 1)"
[ -n "$GODOT_BIN" ] || falha "Godot não encontrado: defina GODOT com o caminho do executável"

if [ "$#" -eq 0 ]; then
    set -- examples/centro_de_distribuicao.ron examples/agvs.ron examples/pessoas_carregando.ron
fi
ARGS=()
for m in "$@"; do
    [ -f "$m" ] || falha "modelo não encontrado: $m"
    ARGS+=("--utsim-modelo=$(cd "$(dirname "$m")" && pwd -W 2>/dev/null || pwd)/$(basename "$m")")
done

echo "==> teste sem janela (godot/testes/teste.gd)"
SAIDA="$(mktemp)"
"$GODOT_BIN" --headless --path godot --script res://testes/teste.gd -- "${ARGS[@]}" 2>&1 | tee "$SAIDA"
STATUS=${PIPESTATUS[0]}
# Erros de script do Godot não mudam o código de saída: procure-os também na saída.
if grep -qE "SCRIPT ERROR|Parse Error|^ERROR:|FALHOU" "$SAIDA"; then
    STATUS=1
fi
rm -f "$SAIDA"
[ "$STATUS" -eq 0 ] || falha "o teste do Godot falhou"
echo
echo "Godot: tudo certo"

#!/usr/bin/env bash
# Teste sem janela da interface Godot: prepara o projeto (scripts/godot.sh) e roda no
# console do Godot, sempre com --headless:
#   1. godot/testes/teste.gd, que confere se a extensão carrega e se cada modelo abre,
#      simula, monta a cena e é reproduzido até o fim;
#   2. os roteiros da interface (godot/roteiros/g2_*.json): a cena principal inteira,
#      com cliques, arrastos e teclas de verdade, conferindo o documento a cada passo (as
#      capturas de tela são puladas sem janela).
# Sai com 1 se algo falhar.
#
# Uso: scripts/godot_teste.sh [modelo.ron ...] [roteiro.json ...]
#   sem argumentos: examples/centro_de_distribuicao.ron, examples/agvs.ron,
#   examples/pessoas_carregando.ron e todos os godot/roteiros/g2_*.json.
#   Só roteiros (ou só modelos) nos argumentos: roda só eles.
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

# Caminho absoluto no formato do sistema (no Windows, C:/...).
absoluto() {
    echo "$(cd "$(dirname "$1")" && pwd -W 2>/dev/null || pwd)/$(basename "$1")"
}

MODELOS=()
ROTEIROS=()
for a in "$@"; do
    [ -f "$a" ] || falha "arquivo não encontrado: $a"
    case "$a" in
        *.json) ROTEIROS+=("$a") ;;
        *) MODELOS+=("$a") ;;
    esac
done
if [ "$#" -eq 0 ]; then
    MODELOS=(examples/centro_de_distribuicao.ron examples/agvs.ron examples/pessoas_carregando.ron)
    for r in godot/roteiros/g2_*.json; do
        [ -f "$r" ] && ROTEIROS+=("$r")
    done
fi

# Erros de script do Godot não mudam o código de saída: procure-os também na saída.
confere_saida() {
    ! grep -qE "SCRIPT ERROR|Parse Error|^ERROR:|FALHOU" "$1"
}

if [ "${#MODELOS[@]}" -gt 0 ]; then
    ARGS=()
    for m in "${MODELOS[@]}"; do
        ARGS+=("--utsim-modelo=$(absoluto "$m")")
    done
    echo "==> teste sem janela (godot/testes/teste.gd)"
    SAIDA="$(mktemp)"
    "$GODOT_BIN" --headless --path godot --script res://testes/teste.gd -- "${ARGS[@]}" 2>&1 | tee "$SAIDA"
    STATUS=${PIPESTATUS[0]}
    confere_saida "$SAIDA" || STATUS=1
    rm -f "$SAIDA"
    [ "$STATUS" -eq 0 ] || falha "o teste do Godot falhou"
fi

for r in "${ROTEIROS[@]}"; do
    echo "==> roteiro sem janela: $r"
    PASTA="$(mktemp -d)"
    SAIDA="$PASTA/saida.txt"
    "$GODOT_BIN" --headless --path godot -- "--utsim-roteiro=$(absoluto "$r")" \
        "--utsim-capturas=$(cd "$PASTA" && pwd -W 2>/dev/null || pwd)" 2>&1 | tee "$SAIDA"
    STATUS=${PIPESTATUS[0]}
    confere_saida "$SAIDA" || STATUS=1
    rm -rf "$PASTA"
    [ "$STATUS" -eq 0 ] || falha "o roteiro $r falhou"
done

echo
echo "Godot: tudo certo"

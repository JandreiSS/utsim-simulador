#!/usr/bin/env bash
# Verificação completa deste repositório: formatação, lints, testes, build de release e modelos.
# Para no primeiro passo que falhar.

set -u

cd "$(dirname "$0")/.." || exit 1
REPO=$(basename "$(git rev-parse --show-toplevel 2>/dev/null || pwd)")

TRAVA=/c/Users/Jandrei/AppData/Local/utsim-trava/trava.sh
TRAVA_DONO=/c/Users/Jandrei/AppData/Local/Temp/utsim-verificar.lock/dono
# Quem chamou já tem a trava quando o token dele (exportado por trava.sh) é o dono dela.
if [ -f "$TRAVA" ] && [ "$(cat "$TRAVA_DONO" 2>/dev/null)" != "${UTSIM_TRAVA_TOKEN:-sem-token}" ]; then
    # shellcheck disable=SC1090
    source "$TRAVA" "$REPO:verificar"
fi


# Verificação é de uma vez só: os dados incrementais não servem para nada e ocupam disco.
export CARGO_INCREMENTAL=0
# Cache de compilação compartilhado entre os worktrees (sccache), quando instalado.
if command -v sccache >/dev/null 2>&1; then
    export RUSTC_WRAPPER=sccache
    export SCCACHE_CACHE_SIZE="${SCCACHE_CACHE_SIZE:-25G}"
fi

# Com UTSIM_GODOT=1, a ponte utsim-godot entra no clippy e nos testes do workspace: uma
# chamada separada recompilaria várias dependências com outro conjunto de features.
FORA="--exclude utsim-viewer --exclude utsim-godot"
if [ "${UTSIM_GODOT:-}" = "1" ]; then
    FORA="--exclude utsim-viewer"
fi

falha() {
    echo "FALHOU: $1" >&2
    exit 1
}

passo() {
    echo
    echo "==> $1"
}

passo "1/7 Formatação (cargo fmt --check)"
# Pacote por pacote: com o workspace grande, `cargo fmt --all` passa da linha de comando
# máxima do Windows (os error 206).
for toml in crates/*/Cargo.toml; do
    # Crates fora do workspace (o `exclude` do Cargo.toml da raiz) ficam de fora.
    grep -q "\"$(dirname "$toml")\"" <(grep '^exclude' Cargo.toml) && continue
    pacote=$(awk -F'"' '/^name = /{print $2; exit}' "$toml")
    cargo fmt -p "$pacote" -- --check || falha "formatação fora do padrão em $pacote. Rode: cargo fmt -p $pacote"
done

passo "1b/7 Família (uma tag só, sem a família local ligada, sem dependência duplicada)"
if [ -f .cargo/config.toml ] && grep -q "utsim-familia" .cargo/config.toml; then
    falha "a família local está ligada (bash scripts/familia-local.sh desliga)"
fi
TAGS=$(grep -ho 'utsim-familia", tag = "[^"]*"' Cargo.toml | sort -u | wc -l)
[ "$TAGS" -le 1 ] || falha "o Cargo.toml usa mais de uma tag da família"
DUP=$(cargo tree -d -e normal --workspace 2>/dev/null | grep -E '^(utsim-[a-z-]+|egui|eframe) v' || true)
[ -z "$DUP" ] || falha "dependência em duas versões (tipos não batem com a família):
$DUP"

passo "2/7 Lints (cargo clippy -D warnings)"
# shellcheck disable=SC2086
cargo clippy --workspace $FORA --all-targets -- -D warnings || falha "clippy encontrou avisos ou erros"

# shellcheck disable=SC2086
if cargo nextest --version >/dev/null 2>&1; then
    passo "3/7 Testes (cargo nextest run --workspace $FORA, em paralelo, e os doctests)"
    # --retries 1: um teste que só passa na segunda vez aparece como FLAKY no relatório.
    cargo nextest run --workspace $FORA --retries 1 --no-fail-fast || falha "testes falharam"
    cargo test --doc --workspace $FORA || falha "doctests falharam"
else
    passo "3/7 Testes (cargo test --workspace $FORA)"
    cargo test --workspace $FORA || falha "testes falharam"
fi

# A versão de distribuição (release) só é gerada na hora de empacotar: aqui basta o
# binário de desenvolvimento, que reaproveita o passo 3 e ainda confere estouros.
passo "4/7 Build do utsim-cli (dev)"
cargo build -p utsim-cli || falha "build do utsim-cli falhou"

passo "5/7 Validação dos modelos (utsim check)"
BIN="target/debug/utsim"
if [ ! -x "$BIN" ]; then
    BIN="target/debug/utsim.exe"
fi
[ -x "$BIN" ] || falha "binário utsim não encontrado em target/debug"
for modelo in examples/*.ron; do
    case "$(basename "$modelo")" in
        experimento*|otimizacao*) continue ;;
    esac
    "$BIN" check "$modelo" || falha "modelo inválido: $modelo"
done

# Elementos 3D do Blender: os testes do passo 3 conferem os arquivos versionados; com
# UTSIM_BLENDER=1, gera tudo de novo e exige que nada mude (precisa do Blender 5.2).
if [ "${UTSIM_BLENDER:-}" = "1" ]; then
    echo
    echo "==> Elementos 3D do Blender (UTSIM_BLENDER=1)"
    bash scripts/blender.sh >/dev/null || falha "scripts/blender.sh falhou"
    git diff --quiet -- assets/modelos3d || falha "assets/modelos3d mudou ao gerar de novo: versione o resultado"
fi

passo "6/7 Visualizador 3D (opcional: UTSIM_VIEWER=1)"
if [ "${UTSIM_VIEWER:-}" = "1" ]; then
    cargo clippy -p utsim-viewer --all-targets -- -D warnings || falha "clippy do utsim-viewer encontrou avisos ou erros"
    cargo test -p utsim-viewer || falha "testes do utsim-viewer falharam"
else
    echo "(pulado; defina UTSIM_VIEWER=1 para incluir o utsim-viewer)"
fi

passo "7/7 Interface Godot (opcional: UTSIM_GODOT=1)"
if [ "${UTSIM_GODOT:-}" = "1" ]; then
    # O clippy e os testes da ponte já rodaram nos passos 2 e 3 (FORA sem utsim-godot).
    # O teste sem janela precisa do Godot 4.7 (GODOT, `godot` no PATH ou ~/Documents/Godot_v4.7*).
    bash scripts/godot_teste.sh || falha "teste sem janela do Godot falhou"
else
    echo "(pulado; defina UTSIM_GODOT=1 para incluir a ponte utsim-godot e o teste no Godot)"
fi

echo
echo "Tudo certo"

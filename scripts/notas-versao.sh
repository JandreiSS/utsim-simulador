#!/usr/bin/env bash
# Imprime as notas de uma versão a partir do CHANGELOG.md.
#
# Uso: bash scripts/notas-versao.sh <tag ou versão>   (ex.: v0.1.0 ou 0.1.0)
# Procura a seção "## [versão]". Se não existir, usa "## [Não lançado]" (útil na simulação
# antes de criar a tag). Sai com erro se não achar nenhuma das duas ou se a seção estiver vazia.

set -eu

cd "$(dirname "$0")/.." || exit 1

[ $# -eq 1 ] || { echo "uso: $0 <tag ou versão>" >&2; exit 2; }
VERSAO=${1#v}

extrair() {
    awk -v alvo="$1" '
        /^## \[/ {
            if (dentro) exit
            cab = $0
            sub(/^## \[/, "", cab)
            sub(/\].*$/, "", cab)
            if (cab == alvo) { dentro = 1; next }
        }
        dentro { print }
    ' CHANGELOG.md
}

NOTAS=$(extrair "$VERSAO")
if [ -z "$(printf '%s' "$NOTAS" | tr -d '[:space:]')" ]; then
    echo "aviso: sem seção [$VERSAO] no CHANGELOG.md; usando [Não lançado]" >&2
    NOTAS=$(extrair "Não lançado")
fi
if [ -z "$(printf '%s' "$NOTAS" | tr -d '[:space:]')" ]; then
    echo "erro: CHANGELOG.md não tem notas para $VERSAO" >&2
    exit 1
fi
printf '%s\n' "$NOTAS"

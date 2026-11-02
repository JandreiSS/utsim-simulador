#!/usr/bin/env bash
# Roda a validação do piloto e imprime a tabela de resultados em Markdown
# (caso, métrica, esperado, simulado ± IC95%, veredito). Uso:
#   bash scripts/validacao_tabela.sh > tabela.md
# Os valores vêm das linhas "VALIDACAO|..." impressas por
# crates/utsim-model/tests/validacao_piloto.rs (sementes fixas, resultado reproduzível).

set -u
cd "$(dirname "$0")/.." || exit 1

saida=$(cargo test --release -p utsim-model --test validacao_piloto -- --nocapture --test-threads=1 2>&1) || {
    echo "$saida" | tail -n 40 >&2
    echo "FALHOU: a validação do piloto tem testes falhando (veja acima)" >&2
    exit 1
}

echo "| Caso | Métrica | Esperado | Simulado ± IC95% | Resultado |"
echo "|---|---|---|---|---|"
echo "$saida" | grep -o 'VALIDACAO|.*' | sort -t'|' -k2,2V -s | awk -F'|' '{
    hw = ($6 + 0 == 0) ? "(exato)" : sprintf("± %.4f", $6)
    printf "| %s | %s | %.4f | %.4f %s | %s |\n", $2, $3, $4, $5, hw, $7
}'

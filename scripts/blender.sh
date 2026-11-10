#!/usr/bin/env bash
# Gera os elementos 3D (assets/modelos3d/*.glb e catalogo.json) com os scripts de
# tools/blender, no Blender sem janela. Argumentos vão para o gerar.py, por exemplo:
#   bash scripts/blender.sh --so cnc,robo
# O Blender vem de $BLENDER, do PATH ou da pasta padrão do Windows.
set -euo pipefail
raiz="$(cd "$(dirname "$0")/.." && pwd)"
b="${BLENDER:-}"
if [ -z "$b" ]; then
  if command -v blender >/dev/null 2>&1; then
    b="$(command -v blender)"
  else
    for c in "/c/Program Files/Blender Foundation"/Blender*/blender.exe "/Applications/Blender.app/Contents/MacOS/Blender"; do
      [ -x "$c" ] && b="$c"
    done
  fi
fi
[ -n "$b" ] || { echo "Blender não encontrado (defina BLENDER)" >&2; exit 1; }
saida=$("$b" -b --factory-startup --python-exit-code 1 -P "$raiz/tools/blender/gerar.py" -- "$@" 2>&1) || { echo "$saida" >&2; exit 1; }
echo "$saida" | grep -E "^utsim-blender:|Error|Traceback" || true

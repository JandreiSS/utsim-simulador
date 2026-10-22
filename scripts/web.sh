#!/usr/bin/env bash
# Builds UTSim for the browser:
#   web/dist/utsim.js + utsim_bg.wasm   (use with web/index.html through a local server)
#   web/dist/utsim.html                 (single self-contained page: wasm in base64, JS and CSS inlined)
# Needs: rustup target wasm32-unknown-unknown and wasm-bindgen-cli with the SAME version
# as the wasm-bindgen crate in Cargo.lock (installed here if missing).

set -eu

cd "$(dirname "$0")/.." || exit 1

rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true

# Version of the wasm-bindgen crate that Cargo.lock pins.
WANT=$(awk '/^name = "wasm-bindgen"$/ {getline; gsub(/[^0-9.]/, "", $0); print; exit}' Cargo.lock)
HAVE=$(wasm-bindgen --version 2>/dev/null | awk '{print $2}' || true)
if [ "$HAVE" != "$WANT" ]; then
    echo "==> instalando wasm-bindgen-cli $WANT (encontrado: ${HAVE:-nenhum})"
    cargo install wasm-bindgen-cli --version "$WANT" --locked --force
fi

echo "==> compilando utsim-web (perfil web, wasm32)"
cargo build --profile web --target wasm32-unknown-unknown -p utsim-web

echo "==> wasm-bindgen"
rm -rf web/dist
wasm-bindgen target/wasm32-unknown-unknown/web/utsim_web.wasm \
    --target no-modules --no-typescript --out-dir web/dist --out-name utsim

echo "==> gerando web/dist/utsim.html (autocontido)"
B64=$(base64 < web/dist/utsim_bg.wasm | tr -d '\n\r')
OUT=web/dist/utsim.html
: > "$OUT"
while IFS= read -r line || [ -n "$line" ]; do
    case "$line" in
        *'href="style.css"'*)
            { echo "<style>"; cat web/style.css; echo "</style>"; } >> "$OUT" ;;
        *'src="dist/utsim.js"'*)
            { echo "<script>"; cat web/dist/utsim.js; echo "</script>"; } >> "$OUT" ;;
        *'src="app.js"'*)
            { printf '<script>window.UTSIM_WASM_B64="%s";</script>\n' "$B64"
              echo "<script>"; cat web/app.js; echo "</script>"; } >> "$OUT" ;;
        *) printf '%s\n' "$line" >> "$OUT" ;;
    esac
done < web/index.html

ls -l web/dist/utsim_bg.wasm web/dist/utsim.html | awk '{printf "%10d bytes  %s\n", $5, $9}'
echo "Pronto. Abra web/dist/utsim.html no navegador (ou sirva a pasta web/: python3 -m http.server -d web)."

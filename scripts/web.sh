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

# The 3D viewer (Bevy + WebGL2) is large: it goes to web/dist/viewer/ and is NOT inlined
# in utsim.html. Pass --sem-3d to skip it (fast builds). web/viewer.html loads it.
if [ "${1:-}" != "--sem-3d" ]; then
    echo "==> compilando utsim-viewer (perfil web-3d, wasm32; demora)"
    cargo build --profile web-3d --target wasm32-unknown-unknown -p utsim-viewer
    echo "==> wasm-bindgen (visualizador)"
    wasm-bindgen target/wasm32-unknown-unknown/web-3d/utsim-view.wasm         --target web --no-typescript --out-dir web/dist/viewer --out-name utsim_viewer
fi

echo "==> gerando web/dist/utsim.html (autocontido)"
# The engine runs in a Web Worker. Here the worker is created from a Blob made of the
# wasm-bindgen glue + web/worker.js, kept in inert <script type="text/plain"> elements.
for part in web/dist/utsim.js web/worker.js web/app.js; do
    if grep -q '</script' "$part"; then
        echo "FALHOU: $part contém '</script' e não pode ser embutido" >&2
        exit 1
    fi
done
B64=$(base64 < web/dist/utsim_bg.wasm | tr -d '\n\r')
OUT=web/dist/utsim.html
: > "$OUT"
while IFS= read -r line || [ -n "$line" ]; do
    case "$line" in
        *'href="style.css"'*)
            { echo "<style>"; cat web/style.css; echo "</style>"; } >> "$OUT" ;;
        *'src="app.js"'*)
            { echo '<script type="text/plain" id="utsim-glue">'; cat web/dist/utsim.js; echo "</script>"
              echo '<script type="text/plain" id="utsim-worker-src">'; cat web/worker.js; echo "</script>"
              printf '<script>window.UTSIM_WASM_B64="%s";</script>\n' "$B64"
              echo "<script>"; cat web/app.js; echo "</script>"; } >> "$OUT" ;;
        *) printf '%s\n' "$line" >> "$OUT" ;;
    esac
done < web/index.html

ls -l web/dist/utsim_bg.wasm web/dist/utsim.html web/dist/viewer/utsim_viewer_bg.wasm 2>/dev/null | awk '{printf "%10d bytes  %s\n", $5, $9}'
echo "Pronto. Abra web/dist/utsim.html no navegador (ou sirva a pasta web/: python3 -m http.server -d web; o 3D só funciona servido)."

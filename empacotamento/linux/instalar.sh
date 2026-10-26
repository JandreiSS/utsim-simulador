#!/usr/bin/env bash
# Instala o UTSim para o seu usuário (sem sudo), a partir da pasta do .tar.gz descompactado.
#
#   ./instalar.sh                 instala em ~/.local/share/utsim
#   ./instalar.sh --desinstalar   remove o que foi instalado
#   PREFIXO=/outra/pasta ./instalar.sh   troca a pasta dos arquivos do programa
#
# Cria: os arquivos do programa em $PREFIXO, os comandos utsim e utsim-view em ~/.local/bin,
# os atalhos "UTSim" e "UTSim (exemplos)" no menu de aplicativos e o ícone.

set -eu

ORIGEM=$(cd "$(dirname "$0")" && pwd)
DADOS=${XDG_DATA_HOME:-$HOME/.local/share}
PREFIXO=${PREFIXO:-$DADOS/utsim}
BIN_USUARIO=${BIN_USUARIO:-$HOME/.local/bin}
TAMANHOS="16 32 48 64 128 256 512"

desinstalar() {
    rm -f "$BIN_USUARIO/utsim" "$BIN_USUARIO/utsim-view"
    rm -f "$DADOS/applications/utsim.desktop" "$DADOS/applications/utsim-exemplos.desktop"
    rm -f "$DADOS/icons/hicolor/scalable/apps/utsim.svg"
    for t in $TAMANHOS; do
        rm -f "$DADOS/icons/hicolor/${t}x${t}/apps/utsim.png"
    done
    # Só apaga a pasta do programa se ela for mesmo uma instalação do UTSim.
    if [ -f "$PREFIXO/bin/utsim-view" ]; then
        rm -rf "$PREFIXO"
    fi
    echo "UTSim removido."
}

if [ "${1:-}" = "--desinstalar" ]; then
    desinstalar
    exit 0
fi

if [ ! -f "$ORIGEM/bin/utsim-view" ]; then
    echo "erro: rode este script de dentro da pasta descompactada (não achei bin/utsim-view)" >&2
    exit 1
fi

# Reinstalar por cima: limpa a versão anterior.
if [ -f "$PREFIXO/bin/utsim-view" ]; then
    rm -rf "$PREFIXO"
fi
mkdir -p "$PREFIXO" "$BIN_USUARIO" "$DADOS/applications"
for item in bin examples assets manual LICENSE README.md CHANGELOG.md; do
    cp -r "$ORIGEM/$item" "$PREFIXO/"
done

# Os programas procuram assets/terceiros acima de bin/, então o link aponta para a pasta real.
ln -sf "$PREFIXO/bin/utsim" "$BIN_USUARIO/utsim"
ln -sf "$PREFIXO/bin/utsim-view" "$BIN_USUARIO/utsim-view"

for arq in utsim.desktop utsim-exemplos.desktop; do
    sed "s|@PREFIXO@|$PREFIXO|g" "$ORIGEM/empacotamento/$arq" > "$DADOS/applications/$arq"
done

for t in $TAMANHOS; do
    mkdir -p "$DADOS/icons/hicolor/${t}x${t}/apps"
    cp "$ORIGEM/assets/icone/utsim-$t.png" "$DADOS/icons/hicolor/${t}x${t}/apps/utsim.png"
done
mkdir -p "$DADOS/icons/hicolor/scalable/apps"
cp "$ORIGEM/assets/icone/utsim.svg" "$DADOS/icons/hicolor/scalable/apps/utsim.svg"

command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "$DADOS/applications" || true
command -v gtk-update-icon-cache >/dev/null 2>&1 && gtk-update-icon-cache -q -t "$DADOS/icons/hicolor" || true

echo "UTSim instalado em $PREFIXO"
case ":$PATH:" in
    *":$BIN_USUARIO:"*) ;;
    *) echo "Aviso: $BIN_USUARIO não está no PATH; adicione-o para usar o comando utsim." ;;
esac

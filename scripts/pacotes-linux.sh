#!/usr/bin/env bash
# Gera os pacotes do Linux a partir de target/release:
#   utsim-v<versão>-linux-x64.tar.gz   pasta portátil com bin/, examples/, assets/, manual/ e instalar.sh
#   utsim_<versão>_amd64.deb           instala em /opt/utsim, com comandos em /usr/bin, atalhos e ícones
#
# Uso: bash scripts/pacotes-linux.sh [versão] [pasta_de_saída]
# Antes: cargo build --release -p utsim-cli -p utsim-viewer
# Opcional: mdbook build docs/manual   (para levar o manual em HTML)
# O .deb exige dpkg-deb (Debian, Ubuntu); sem ele, só o .tar.gz é gerado.

set -eu

cd "$(dirname "$0")/.." || exit 1

VERSAO=${1:-$(awk -F'"' '/^version = /{print $2; exit}' Cargo.toml)}
VERSAO=${VERSAO#v}
SAIDA=${2:-dist}
# Versões de teste como 0.2.0-rc1 ordenam antes de 0.2.0 no dpkg com "~".
VERSAO_DEB=${VERSAO//-/\~}

mkdir -p "$SAIDA"
SAIDA=$(cd "$SAIDA" && pwd)
PASTA=$(mktemp -d)
trap 'rm -rf "$PASTA"' EXIT

bash scripts/preparar-pacote.sh "$PASTA/base" target/release

# ---------- tar.gz portátil ----------
NOME="utsim-v${VERSAO}-linux-x64"
mkdir -p "$PASTA/$NOME/empacotamento"
cp -r "$PASTA/base/." "$PASTA/$NOME/"
cp empacotamento/linux/instalar.sh "$PASTA/$NOME/instalar.sh"
cp empacotamento/linux/utsim.desktop empacotamento/linux/utsim-exemplos.desktop "$PASTA/$NOME/empacotamento/"
chmod +x "$PASTA/$NOME/instalar.sh"
tar -C "$PASTA" -czf "$SAIDA/$NOME.tar.gz" "$NOME"
echo "Gerado: $SAIDA/$NOME.tar.gz"

# ---------- .deb ----------
if ! command -v dpkg-deb >/dev/null 2>&1; then
    echo "aviso: dpkg-deb não encontrado; .deb não gerado" >&2
    exit 0
fi

DEB="$PASTA/deb"
mkdir -p "$DEB/opt" "$DEB/usr/bin" "$DEB/usr/share/applications" "$DEB/usr/share/doc/utsim" "$DEB/DEBIAN"
cp -r "$PASTA/base" "$DEB/opt/utsim"
ln -s /opt/utsim/bin/utsim "$DEB/usr/bin/utsim"
ln -s /opt/utsim/bin/utsim-view "$DEB/usr/bin/utsim-view"

for arq in utsim.desktop utsim-exemplos.desktop; do
    sed "s|@PREFIXO@|/opt/utsim|g" "empacotamento/linux/$arq" > "$DEB/usr/share/applications/$arq"
done
for t in 16 32 48 64 128 256 512; do
    mkdir -p "$DEB/usr/share/icons/hicolor/${t}x${t}/apps"
    cp "assets/icone/utsim-$t.png" "$DEB/usr/share/icons/hicolor/${t}x${t}/apps/utsim.png"
done
mkdir -p "$DEB/usr/share/icons/hicolor/scalable/apps"
cp assets/icone/utsim.svg "$DEB/usr/share/icons/hicolor/scalable/apps/utsim.svg"

# Documentação e licença no lugar padrão do Debian (os arquivos de /opt/utsim ficam como estão).
cp LICENSE "$DEB/usr/share/doc/utsim/copyright"
cp CHANGELOG.md "$DEB/usr/share/doc/utsim/changelog"
gzip -9n "$DEB/usr/share/doc/utsim/changelog"

TAMANHO=$(du -sk "$DEB" | cut -f1)
cat > "$DEB/DEBIAN/control" <<EOF
Package: utsim
Version: $VERSAO_DEB
Section: science
Priority: optional
Architecture: amd64
Installed-Size: $TAMANHO
Depends: libc6, libasound2t64 | libasound2, libudev1, libxkbcommon0, libwayland-client0, libvulkan1
Maintainer: JandreiSS <JandreiSS@users.noreply.github.com>
Homepage: https://github.com/JandreiSS/utsim
Description: simulador de processos a eventos discretos com editor 3D
 O UTSim simula filas, linhas de produção, armazéns e logística interna.
 Traz o comando utsim (linha de comando) e o utsim-view (editor e
 visualizador 3D), com exemplos, manual e ativos 3D de terceiros.
EOF

# Permissões padrão: diretórios 755, arquivos 644, programas 755.
find "$DEB" -type d -exec chmod 755 {} +
find "$DEB" -type f -exec chmod 644 {} +
chmod 755 "$DEB/opt/utsim/bin/utsim" "$DEB/opt/utsim/bin/utsim-view"

dpkg-deb --root-owner-group --build "$DEB" "$SAIDA/utsim_${VERSAO_DEB}_amd64.deb"
echo "Gerado: $SAIDA/utsim_${VERSAO_DEB}_amd64.deb"

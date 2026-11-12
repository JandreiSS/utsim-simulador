# Elementos 3D (`assets/modelos3d`)

Os arquivos desta pasta são gerados por `bash scripts/blender.sh`, com os scripts de
`tools/blender`. Como cada um é feito, os nós, os pontos e os clipes estão em
`docs/elementos_3d_blender.md`.

## Pessoas: humanos do MakeHuman (MPFB)

`pessoa_homem.glb` e `pessoa_mulher.glb` são humanos do MakeHuman, gerados no Blender pelo
complemento MPFB e encaixados no esqueleto do boneco UAL1 por `tools/blender/pessoas.py`. No
repositório ficam só os `.glb` e o script. Os arquivos de origem ficam fora dele, na pasta de
`UTSIM_MPFB` (padrão: `~/Documents/Codes/UTSim-origem-3d/mpfb`).

| O quê | Versão | Fonte oficial | Licença |
|---|---|---|---|
| MPFB, complemento do Blender que gera os humanos | 2.0.18 | https://extensions.blender.org/add-ons/mpfb/ (`add-on-mpfb-v2.0.18.zip`, sha256 `c075b3e9b94b8e5556a5f03f4653602d02ea306de08126e8f55ed6dee28a2dad`) | Código: GPL-3.0 ou posterior. Ele só roda na máquina de quem gera; não vai no repositório nem no app. |
| Malha base, alvos do corpo e pesos do rig `game_engine` (dentro do MPFB) | a do MPFB 2.0.18 | a mesma | CC0 1.0 (o MakeHuman liberou os assets em CC0 em 2020; está no cabeçalho dos arquivos) |
| Pacote de assets de sistema do MakeHuman | `makehuman_system_assets_cc0`, baixado em 2026-11-10 | https://files.makehumancommunity.org/asset_packs/makehuman_system_assets/makehuman_system_assets_cc0.zip (página: https://static.makehumancommunity.org/assets/assetpacks/makehuman_system_assets.html; sha256 `b542127a8e25547c7c29c19f2d1d2adb9a664c80396ecd694095dbc8028a0107`) | CC0 1.0 |

Do pacote, o UTSim usa: o macacão `male_worksuit01` (com a camiseta), o sapato `shoes04`
(também o molde do EPI botina), os cabelos `short02` (homem) e `ponytail01` (mulher), as
sobrancelhas `eyebrow001` e `eyebrow010` e os olhos `low-poly`. As texturas não vão para o
arquivo: cada parte vira uma cor lisa, que o 3D pode recolorir.

### Como gerar de novo

1. Baixe os dois zips das fontes acima (confira o sha256).
2. Descompacte o MPFB em `<UTSIM_MPFB>/ext/mpfb` (a pasta que tem o `__init__.py`) e o pacote
   em `<UTSIM_MPFB>/assets` (a pasta que tem `clothes`, `hair`, `eyes`).
3. Rode `bash scripts/blender.sh --so pessoa_homem,pessoa_mulher` (ou `bash scripts/blender.sh`
   para todos os elementos).

O script liga o MPFB como extensão local só durante a execução: nada é instalado no Blender
do usuário. As pastas de trabalho do MPFB (cache e logs) ficam em `<UTSIM_MPFB>/blender`. Sem a
pasta do MPFB, a geração das pessoas para com uma mensagem que aponta para este arquivo; os
outros elementos não dependem dela. O mesmo vale para `UTSIM_BLENDER=1 bash scripts/verificar.sh`,
que gera tudo de novo.

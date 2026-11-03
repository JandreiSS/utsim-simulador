# Planta CAD (DXF)

O UTSim lê a planta da fábrica em **DXF 2D** (texto) para servir de fundo ao layout e para virar paredes e colunas que os recursos não atravessam. Esta página descreve o núcleo (comando `utsim planta`); o diálogo de importação na interface vem depois.

## Formatos

- **DXF de texto (ASCII)**: suportado. Entidades lidas: `LINE`, `LWPOLYLINE`, `POLYLINE` (com arcos por *bulge*), `CIRCLE`, `ARC`, `TEXT`, `MTEXT` e `INSERT` (blocos, inclusive aninhados, com escala, rotação e espelhamento). Elipses, *splines*, hachuras, cotas e entidades 3D são ignoradas.
- **DWG**: não é lido (formato fechado). Converta para DXF no seu programa de CAD ou em um conversor gratuito.
- **DXF binário**: salve novamente como DXF de texto.
- **Imagem e PDF**: por enquanto, exporte a página como imagem; o uso de imagem com escala fica para a interface.

## Unidades e calibração

A unidade vem do cabeçalho do arquivo (`$INSUNITS`). Se o arquivo não informar, o UTSim assume **milímetros** e avisa. Confira sempre o tamanho da caixa que o comando mostra. Para garantir a escala, use a **calibração por dois pontos**: dê dois pontos do desenho (nas unidades do arquivo) e a distância real entre eles, em metros. Exemplo: dois pontos a 40 000 unidades, com 40 m reais, dão a escala 0,001.

## Camadas e papéis

Cada camada tem um papel: `fundo`, `parede`, `coluna`, `porta` ou `ignorar`. O programa sugere o papel pelo nome comum da camada (`PAREDES`, `WALL`, `COLUNAS`, `PILARES`, `PORTAS`, `COTAS`, `HACHURA` e parecidos); você pode trocar qualquer um. Polilinhas fechadas de parede e coluna viram polígonos de obstáculo. Linhas abertas de parede ganham espessura (padrão 0,15 m). Camadas de porta marcam aberturas e não bloqueiam.

## Comando

```text
utsim planta arquivo.dxf [--camadas] [--svg saida.svg] [--calibrar x1,y1,x2,y2,distancia]
                         [--origem x,y] [--unidades mm|cm|dm|m|km|pol|pe]
                         [--obstaculos modelo.ron]
```

- sem opções: mostra o hash do arquivo, a unidade, a caixa envolvente (em metros) e o número de camadas e segmentos;
- `--camadas`: tabela com entidades, segmentos, rótulos e o papel sugerido de cada camada;
- `--svg`: grava a planta em SVG (as camadas ignoradas não aparecem), para conferir a leitura num navegador;
- `--calibrar`: lê o desenho sem converter unidades e aplica a escala;
- `--origem`: desloca a planta para que o ponto dado vire (0, 0);
- `--obstaculos modelo.ron`: grava as paredes e colunas (com a calibração e a origem dadas)
  como `obstacles` do modelo, com os nomes `planta_1`, `planta_2`... (os da importação anterior
  saem; os desenhados à mão ficam), e registra a planta em `floor_plan` (arquivo, hash,
  transformação, calibração e papéis das camadas). A simulação não lê o DXF: os obstáculos ficam
  no `.ron`. Os recursos nos modos livre e pela rede desviam deles (veja
  [Movimentação e obstáculos](movimentacao.md)); se o modelo tiver uma rede gerada, gere-a de
  novo com `utsim rede gerar`.

## Limites conhecidos

- Plantas reais têm camadas desorganizadas; revise os papéis sugeridos.
- Paredes feitas de linhas soltas que formam um recinto não viram polígono fechado; só polilinhas fechadas (ou abertas com as pontas coincidentes) viram.
- Paredes abertas viram um retângulo por segmento; nos cantos pode sobrar uma pequena falha do tamanho da espessura.
- Textos usam apenas o conteúdo e a posição.

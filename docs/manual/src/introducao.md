# Introdução

Este manual ensina a usar o UTSim, um simulador de processos produtivos feito para quem trabalha com fábricas, armazéns e serviços, e não precisa saber programar.

Se você prefere aprender fazendo, vá direto aos [tutoriais](tutoriais/index.md). Se quer consultar um detalhe, use a barra lateral.

## O que é simulação a eventos discretos

Imagine uma linha de produção. Uma peça chega, espera na fila, é usinada, segue para a pintura e sai. O estado dessa linha (quantas peças esperam, quais máquinas estão ocupadas) só muda em instantes bem definidos: quando uma peça chega, quando uma máquina termina o trabalho, quando uma máquina quebra. Cada um desses instantes é um **evento**.

Entre dois eventos nada acontece. Por isso o simulador não precisa "andar" segundo a segundo. Ele mantém uma lista de eventos futuros, pula direto para o próximo e atualiza o estado. É assim que ele simula semanas de fábrica em milésimos de segundo.

Os tempos (entre chegadas, de processamento, de reparo) quase nunca são fixos. Por isso o modelo sorteia cada tempo a partir de uma **distribuição de probabilidade**. Como o resultado de uma rodada depende desses sorteios, uma só rodada não basta. O UTSim roda várias **replicações** independentes e mostra a média com um **intervalo de confiança**.

Em resumo, a simulação a eventos discretos serve para responder perguntas do tipo "e se":

- Quantas máquinas eu preciso para dar conta da demanda?
- Onde está o gargalo?
- Quanto uma peça espera, em média, antes de ser atendida?
- Vale a pena comprar mais uma empilhadeira?
- O que muda se eu afastar duas estações em 30 metros?

## O que o UTSim faz

O UTSim reúne:

- **Objetos de processo**: chegadas, filas, processadores, montagem e desmontagem, esteiras, porta-paletes e saídas. Veja [Objetos](objetos.md).
- **Recursos móveis**: operadores, empilhadeiras e AGVs que andam pelo layout, com despachantes e rede de caminhos. Veja [Recursos móveis](recursos-moveis.md).
- **Quebras e turnos**: falhas aleatórias e janelas fora de turno. Veja [Quebras e turnos](quebras-e-turnos.md).
- **Lógica do usuário**: fluxos de tokens e scripts simples. Veja [Fluxos de tokens e scripts](fluxos-e-scripts.md).
- **Fluidos**: tanques, tubos e envase. Veja [Fluidos](fluidos.md).
- **Estatística**: replicações, aquecimento, intervalos de confiança e comparação pareada de cenários. Veja [Resultados e estatística](resultados.md).
- **Ajuste de distribuições**: a partir de dados reais, o comando `utsim fit` escolhe a distribuição e explica por quê. Veja [Ajuste de distribuições](ajuste-de-distribuicoes.md).
- **Experimentos e otimização**: compara cenários e procura a melhor combinação de parâmetros. Veja [Experimentos e otimização](experimentos-e-otimizacao.md).
- **Editor e visualização 3D**: você monta o modelo e assiste à simulação em 3D.
- **Emulação OPC UA** e **aprendizado por reforço**, para comissionamento virtual e decisões automáticas.

O modelo é um arquivo de texto (extensão `.ron`). Você pode editá-lo no editor 3D ou em qualquer editor de texto, guardá-lo no Git e comparar versões. Veja o [formato `.ron`](referencia-ron.md).

O UTSim foi escrito do zero. O [glossário](glossario.md) traz os termos em português e os equivalentes em inglês da literatura de simulação.

## Instalação

Há três jeitos de usar o UTSim. Escolha o que for mais simples para você.

### 1. Programas prontos (instaladores e pacotes)

Cada versão publicada na página de [Releases](https://github.com/JandreiSS/utsim/releases) do GitHub traz:

| Arquivo | Para quê |
|---|---|
| `utsim-v<versão>-windows-x64-instalador.exe` | Instalador do Windows (recomendado) |
| `utsim-v<versão>-windows-x64.zip` | Versão portátil do Windows: descompacte e use |
| `utsim_<versão>_amd64.deb` | Pacote para Debian, Ubuntu e derivados |
| `utsim-v<versão>-linux-x64.tar.gz` | Pasta portátil do Linux, com `instalar.sh` |
| `utsim-v<versão>-web.zip` | A página que roda no navegador |
| `SHA256SUMS.txt` | Somas de verificação dos arquivos acima |

#### Windows

Baixe e abra o instalador. Ele **não pede administrador**: instala só para o seu usuário em `%LOCALAPPDATA%\Programs\UTSim` (você pode trocar a pasta). Como o instalador ainda não é assinado digitalmente, o Windows pode mostrar o aviso do SmartScreen; escolha "Mais informações" e "Executar assim mesmo".

O instalador oferece até quatro componentes:

- **UTSim** (obrigatório): `bin\utsim.exe` (linha de comando), `bin\utsim-view.exe` (editor e visualizador 3D), a pasta `examples\` (modelos de exemplo), `assets\` (modelos 3D e ativos de terceiros, cada pacote com a sua licença e os créditos em `assets\terceiros\CREDITOS.md`), `manual\` (este manual em HTML, abra `manual\index.html`) e os arquivos `README.md` e `CHANGELOG.md`.
- **Atalhos no Menu Iniciar**: a pasta "UTSim" com "UTSim" (abre o editor com um modelo novo), "UTSim (exemplos)" (abre a pasta dos exemplos; copie um para outro lugar antes de editar, pois o editor salva por cima do arquivo aberto), "Manual do UTSim" e "Desinstalar o UTSim".
- **UTSim 3D (prévia)** (só nos instaladores gerados com a exportação do Godot): a nova interface 3D, em `3d\utsim-3d.exe`, com o atalho "UTSim 3D (prévia)" no Menu Iniciar e o arquivo `3d\AVISOS-TERCEIROS.txt` (avisos de copyright e licenças do motor Godot). Convive com o editor `utsim-view`, que continua sendo o programa principal da versão 0.1.
- **Adicionar o comando `utsim` ao PATH** (desmarcado por padrão): acrescenta `bin\` ao `PATH` do seu usuário, para digitar `utsim` em qualquer terminal novo.

Para remover, use Configurações > Aplicativos > UTSim, ou "Desinstalar o UTSim" no Menu Iniciar. A desinstalação apaga só o que o instalador criou e tira o `PATH`, se ele foi mexido. Para instalar sem janelas, rode o instalador com `/S` (e `/D=C:\pasta` para escolher a pasta, sempre como último argumento).

O instalador **não associa extensões de arquivo**. O Windows associa pela última parte do nome, então `modelo.utsim.ron` cairia em `.ron`, uma extensão genérica demais para o UTSim tomar para si. Para abrir um modelo, arraste o arquivo sobre `utsim-view.exe` ou rode `utsim-view modelo.ron`.

#### Linux

**Debian, Ubuntu e derivados** (`.deb`):

```bash
sudo apt install ./utsim_0.1.0_amd64.deb
```

Os arquivos ficam em `/opt/utsim` (`bin/`, `examples/`, `assets/`, `manual/`), os comandos `utsim` e `utsim-view` em `/usr/bin`, os atalhos "UTSim" e "UTSim (exemplos)" no menu de aplicativos, os ícones em `/usr/share/icons/hicolor` e a licença em `/usr/share/doc/utsim`. Remova com `sudo apt remove utsim`.

**Qualquer distribuição** (`.tar.gz`, sem `sudo`):

```bash
tar xzf utsim-v0.1.0-linux-x64.tar.gz
cd utsim-v0.1.0-linux-x64
./instalar.sh
```

O script copia o programa para `~/.local/share/utsim`, cria os comandos em `~/.local/bin`, os atalhos no menu e os ícones. Para remover, rode `./instalar.sh --desinstalar`. Também dá para usar direto da pasta, sem instalar: `./bin/utsim-view --novo`.

Nos pacotes que trazem o UTSim 3D (prévia), a pasta `3d/` fica ao lado de `bin/`, o comando `utsim-3d` aponta para ela e o atalho "UTSim 3D (prévia)" aparece no menu de aplicativos. Os avisos do motor Godot estão em `3d/AVISOS-TERCEIROS.txt`.

Em ambos, o visualizador usa as bibliotecas de áudio e de janela do sistema (ALSA, udev, xkbcommon, Wayland ou X11, Vulkan). Numa distribuição de mesa comum, elas já estão presentes.

#### Conferindo

```bash
utsim --version
utsim run examples/mm1.ron
```

No Windows, se você não marcou a opção do `PATH`, use o caminho completo, por exemplo `%LOCALAPPDATA%\Programs\UTSim\bin\utsim.exe`.

### 2. Compilando com o cargo

Você precisa do [Rust](https://www.rust-lang.org/tools/install) (versão 1.85 ou mais nova). Na pasta do projeto:

```bash
# linha de comando
cargo build --release -p utsim-cli
# resultado: target/release/utsim (no Windows, utsim.exe)

# editor e visualizador 3D (demora mais, pois usa o motor gráfico Bevy)
cargo build --release -p utsim-viewer
# resultado: target/release/utsim-view
```

No Linux, o visualizador pede alguns pacotes do sistema: `libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev`.

Para rodar a suíte de verificação do projeto, use `bash scripts/verificar.sh`. Ela formata, roda os lints, os testes e valida todos os modelos de `examples/`.

### 3. A página no navegador

A simulação, o ajuste de distribuições e o 3D também rodam no navegador, sem instalar nada. Para gerar a página:

```bash
bash scripts/web.sh          # no Windows: scripts/web.ps1
python3 -m http.server -d web
```

Abra `http://localhost:8000/`. O arquivo `web/dist/utsim.html` é autocontido e abre direto no navegador, mas sem a aba 3D. Veja [Página no navegador](navegador.md).

## Como este manual está organizado

- **Primeiros passos**: abrir o visualizador e se mexer nele.
- **Construindo modelos**: cada tipo de objeto, conexões, recursos móveis, esteiras, quebras, lógica e fluidos.
- **Analisando resultados**: como ler o relatório, ajustar distribuições, comparar cenários e otimizar.
- **Recursos avançados**: OPC UA, aprendizado por reforço, 3D realista e navegador.
- **Tutoriais**: seis exercícios guiados, cada um com um modelo pronto em `examples/`.
- **Referência**: todos os campos do `.ron`, todos os comandos e o glossário.

Os exemplos usam tempos em minutos ou segundos e distâncias em metros. O UTSim não impõe unidades: o que importa é usar a mesma unidade de tempo em todo o modelo e, para distâncias, a mesma unidade das velocidades.
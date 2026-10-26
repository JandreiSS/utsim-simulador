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

O UTSim é software livre (licença GPL-3.0 ou posterior). Ele reúne:

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

O UTSim foi escrito do zero. A inspiração conceitual vem de simuladores comerciais como o simuladores comerciais, e o [glossário](glossario.md) traz a correspondência entre os termos.

## Instalação

Há três jeitos de usar o UTSim. Escolha o que for mais simples para você.

### 1. Programas prontos (binários)

Cada versão publicada no GitHub traz três pacotes:

| Pacote | Conteúdo |
|---|---|
| `utsim-<versão>-windows-x64` | `utsim.exe` (linha de comando) e `utsim-view.exe` (editor e visualizador 3D) |
| `utsim-<versão>-linux-x64` | `utsim` e `utsim-view` |
| `utsim-<versão>-web` | a página que roda no navegador |

Descompacte e pronto. Não há instalador. Para usar `utsim` de qualquer pasta, coloque o programa numa pasta que esteja no `PATH`.

Confira se funciona:

```bash
utsim --version
utsim run examples/mm1.ron
```

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
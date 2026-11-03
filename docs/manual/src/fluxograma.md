# Fluxograma do processo

O comando `utsim fluxograma` desenha o **processo inteiro** a partir de um modelo, sem rodar nada: cada objeto vira uma etapa com o símbolo padrão do fluxograma de processo da engenharia de métodos, cada ligação vira uma seta, e os recursos móveis (equipes e frotas) aparecem como anotações laterais. É o desenho que se cola num relatório, numa proposta ou no material de uma aula.

```bash
utsim fluxograma examples/centro_de_distribuicao.ron --html fluxograma.html
utsim fluxograma examples/centro_de_distribuicao.ron --svg fluxograma.svg --mermaid fluxograma.mmd --csv carta.csv
utsim fluxograma examples/centro_de_distribuicao.ron --html fluxograma.html --com-resultados
```

## Saídas

| Opção | O que grava |
|---|---|
| `--html arq` | página única e autocontida: legenda, fluxograma, painel de recursos e carta de processo |
| `--svg arq` | só o fluxograma, em SVG (abre em qualquer navegador e entra em Word, PowerPoint e LibreOffice) |
| `--mermaid arq` | o fluxograma em texto Mermaid (`flowchart LR`), para colar em documentos que desenham Mermaid |
| `--csv arq` | a carta de processo do caminho principal, em CSV (vírgula, ponto decimal) |

Sem nenhuma opção de saída, o comando imprime o texto Mermaid na tela. As cores acompanham o tema claro ou escuro do navegador.

Opções de desenho: `--horizontal` (da esquerda para a direita; o padrão é de cima para baixo), `--sem-tempos`, `--sem-distancias` e `--simbolo objeto=simbolo` (veja abaixo). `--com-resultados` roda o modelo (com as replicações do arquivo) e põe números nas caixas.

## Os símbolos

| Símbolo | Forma | Significa | Objetos |
|---|---|---|---|
| Início | cápsula | entrada dos itens | `Source` |
| Fim | cápsula | saída dos itens | `Sink` |
| Operação | círculo | o item é transformado | `Processor`, `Combiner`, `Separator` |
| Transporte | seta larga | o item é movido | `Conveyor` e as ligações com transporte (veja abaixo) |
| Inspeção | quadrado | o item é conferido | `Processor`, `Combiner` ou `Separator` cujo nome indica inspeção |
| Espera | "D" | o item aguarda | `Queue` |
| Armazenagem | triângulo | o item fica guardado | `Rack` |

`Transporter` e `Dispatcher` não são etapas: eles carregam os itens. Por isso aparecem como **recursos** (painel à direita e uma linha na caixa de quem os usa).

### Como o programa decide que é uma inspeção

O formato do modelo não tem um campo "símbolo". Por isso a regra é por **nome**: um objeto de processamento cujo nome contém um destes trechos (sem diferenciar maiúsculas nem acentos) vira inspeção: `inspec`, `confer`, `verific`, `qualidade`, `teste`, `audit`, `triagem`. Assim `inspecao`, `Inspeção_final` e `conferencia_saida` ficam com o quadrado.

Para escolher o símbolo de um objeto à mão, use `--simbolo`, quantas vezes precisar:

```bash
utsim fluxograma modelo.ron --html f.html --simbolo torno=inspecao --simbolo balanca=espera
```

Os nomes aceitos são `inicio`, `fim`, `operacao`, `transporte`, `inspecao`, `espera` e `armazenagem`. O que vale para uma biblioteca Rust é a mesma regra: `Options::overrides` (por objeto) e `Options::inspection_keywords` (os trechos).

> **Proposta, ainda não adotada:** um campo opcional `symbol` no objeto do `.ron` (como o `visual`) guardaria a escolha dentro do arquivo. Mudaria o formato do modelo, então ficou para uma decisão do dono; veja `docs/backlog_v02.md`.

## O que o desenho mostra

- **Caixas**: nome, tipo, tempo médio da etapa (processo mais preparação; comprimento da esteira dividido pela velocidade; permanência mínima no porta-paletes; intervalo médio entre chegadas), capacidade e avisos (quebras, turnos, perda de chegadas, divisão em partes, lotes).
- **Ramificações**: quando um objeto tem mais de uma saída, cada seta leva a regra: a porcentagem dos pesos (`Weighted`), `alternado` ou `aleatório` (partes iguais), `prioridade n` (primeira livre), `rótulo x = n` ou `script`. O objeto ganha o aviso `ramifica em n saídas`.
- **Junções**: objetos com mais de uma entrada ganham `junção de n entradas`. Nas montagens (`Combiner`), as setas dizem qual é o contêiner e quais são os componentes.
- **Retrabalho**: uma seta que volta a uma etapa anterior (um laço no fluxo) é tracejada, em laranja, com o rótulo `retrabalho`. O desenho não conta a porcentagem de retrabalho de uma rodada; ela vem da regra de saída (por exemplo, 10%).
- **Transporte nas setas**: uma ligação cujo objeto de origem tem `transport` (empilhadeira, AGV, equipe) ou tempo de transferência (`transfer`, `run.transfer`) leva o símbolo de transporte, a **distância** (linha reta ou rede de caminhos, conforme o modelo), o **tempo** e quem transporta. No transporte por empilhadeira ou AGV, o tempo é uma estimativa: distância dividida pela velocidade, mais carga e descarga, sem contar a espera por um veículo livre. Se todos os objetos estão na posição `(0, 0, 0)`, não há distância a mostrar.
- **Recursos** (painel à direita): equipes e frotas (`Dispatcher`) com a regra e os membros, e transportadores soltos (AGV, empilhadeira, pessoa), com os objetos que os usam.

## A carta de processo

A carta é a tabela do **caminho principal**: começa na primeira chegada e segue, a cada ramificação, a seta mais provável (a de maior peso; sem pesos, a primeira), sem entrar em retrabalho. Cada etapa tem o símbolo, o tempo médio, a distância e os recursos. Entre duas etapas ligadas por transporte entra uma linha própria de transporte.

```text
etapa,simbolo,descricao,objeto,tempo_medio,distancia,recursos,observacao
4,Transporte,esteira_entrada (Esteira),esteira_entrada,16,,,acumula itens no fim
5,Inspeção,inspecao (Processo),inspecao,80,,transporte: frota; operadores: ...
6,Transporte,Transporte de inspecao até porta_paletes,,46.67,40,frota,
```

No fim da tabela do HTML há o total de tempo e de distância e a contagem por símbolo (quantas operações, transportes, inspeções, esperas e armazenagens). As esperas não têm tempo definido no modelo: sem resultados o total de tempo é um mínimo e a página avisa. Com `--com-resultados`, a linha de uma espera traz a espera média medida.

## Com resultados de uma rodada

`--com-resultados` roda as replicações do arquivo e acrescenta às caixas, em laranja: a **ocupação** dos recursos (processos, montagens, desmontagens), a **fila média** e a **espera média** das filas, o conteúdo médio dos porta-paletes e os **itens por replicação** de cada objeto. São médias entre as replicações. É o jeito mais rápido de ver o gargalo em cima do desenho.

## Mermaid

O texto sai com uma forma por símbolo (cápsula, círculo, bandeira, retângulo, retângulo arredondado e hexágono, que é o mais próximo do triângulo que o Mermaid desenha) e as cores do HTML:

```text
flowchart LR
    n0(["chegada_caminhoes"])
    n1(("descarga<br/>tempo 240"))
    ...
    n4 -->|"dist. 40 · tempo 46,67 · frota"| n5
```

Cole o texto num bloco ` ```mermaid ` do Markdown (GitHub, GitLab, Obsidian) ou no editor online do Mermaid.

## Como a disposição é calculada

As etapas vão para camadas: a camada de uma etapa é o maior caminho até ela, sem contar os retrabalhos. Dentro da camada, a ordem reduz os cruzamentos (médias dos vizinhos, algumas varreduras) e as caixas são alinhadas com as vizinhas. Uma seta que pulasse camadas passa por pontos virtuais em cada camada, para contornar as caixas. O resultado depende só do modelo: o mesmo arquivo dá sempre o mesmo desenho.

## Os dez tipos de diagrama (`--tipo`)

`utsim fluxograma modelo.ron --tipo <tipo>` escolhe o tipo; `--tipo todos --html indice.html` gera todos, com uma página por tipo ao lado do índice (`indice-carta.html`, `indice-vsm.svg`, `indice-carta.csv`...). Os cinco primeiros saem só do modelo; os cinco últimos precisam de uma rodada (use `--replica n` para escolher a replicação; a primeira é a 0). Toda página termina com **Origem dos dados**: o modelo, ou a simulação com a semente e a replicação.

| `--tipo` | Diagrama | Origem | Saídas |
|---|---|---|---|
| `carta` | carta de processo (fluxograma vertical) | modelo | HTML, CSV |
| `blocos` | fluxograma de blocos por camadas (o desenho de cima) | modelo | SVG, HTML, Mermaid |
| `operacoes` | gráfico de operações do processo | modelo | SVG, HTML, Mermaid |
| `raias` | fluxograma multifuncional | modelo | SVG, HTML, Mermaid |
| `sipoc` | diagrama SIPOC | modelo | SVG, HTML, CSV |
| `mapofluxograma` | mapofluxograma | simulação | SVG, HTML, CSV |
| `espaguete` | diagrama de espaguete | simulação | SVG, HTML, CSV |
| `de-para` | matriz de-para e diagrama de cordas | simulação | SVG, HTML, CSV |
| `homem-maquina` | gráfico homem-máquina | simulação | SVG, HTML, CSV |
| `vsm` | mapa do fluxo de valor, estado atual | simulação | SVG, HTML, CSV |

Sem `--tipo`, o comando mantém o comportamento descrito acima (blocos mais carta, numa página só).

### Carta de processo (`carta`)
**O que é:** a sequência de etapas do caminho principal, uma por linha, com o símbolo, o tempo médio, a distância, os recursos, as observações e os totais por símbolo. **Quando usar:** para documentar o método de trabalho e contar quantas operações, transportes, esperas, inspeções e armazenagens o item sofre. **Como gerar:** `utsim fluxograma m.ron --tipo carta --html carta.html --csv carta.csv`.

### Fluxograma de blocos (`blocos`)
**O que é:** o diagrama de caixas e setas por camadas descrito no início deste capítulo; `--horizontal` desenha da esquerda para a direita. **Quando usar:** para ver ramificações, junções, retrabalho e recursos de uma vez. **Como gerar:** `--tipo blocos --svg f.svg --mermaid f.mmd`.

### Gráfico de operações do processo (`operacoes`)
**O que é:** só as operações (`O-1`, `O-2`...) e as inspeções (`I-1`...), numeradas na ordem do processo, com as chegadas (material) entrando nas montagens e as saídas das desmontagens; esperas, transportes e armazenagens ficam fora. **Quando usar:** para enxergar a estrutura de montagem do produto e fazer o estudo de operações. **Como gerar:** `--tipo operacoes --svg o.svg --mermaid o.mmd`.

### Fluxograma multifuncional (`raias`)
**O que é:** as etapas dentro da raia de quem as executa, com as passagens entre raias em laranja e uma tabela que as conta. A raia sai da equipe de operadores da etapa (`Equipe x`), do tipo do objeto (`Estações`, `Esteiras`, `Filas e armazenagem`, `Fornecedor e cliente`) e, para o transporte por frota ou transferência, de uma raia própria (`Frota x`). **Quando usar:** para achar entregas entre áreas e responsáveis. **Como gerar:** `--tipo raias --html r.html`; `--raia objeto=nome` põe um objeto numa área à escolha (o formato do modelo não tem campo de área).

### Diagrama SIPOC (`sipoc`)
**O que é:** cinco colunas: fornecedores (as chegadas), entradas (o que chega e com que frequência), processo (as etapas principais do caminho), saídas e clientes (as saídas do modelo). **Quando usar:** na abertura de um projeto de melhoria, para delimitar o processo. **Como gerar:** `--tipo sipoc --html s.html --csv s.csv`.

### Mapofluxograma (`mapofluxograma`)
**O que é:** a planta, com os objetos nas suas posições, o símbolo da carta de processo em cada etapa e o percurso mais usado pelos itens, em setas numeradas na ordem. **Quando usar:** para ligar o fluxo do processo ao layout físico. **Como gerar:** `--tipo mapofluxograma --svg m.svg`. Um modelo sem posições (tudo em 0, 0, 0) usa a disposição do fluxograma de blocos e não mostra distâncias. Em código, `Options::plant_background` aceita um fragmento SVG que é desenhado por baixo (gancho para a planta CAD).

### Diagrama de espaguete (`espaguete`)
**O que é:** todos os deslocamentos da rodada sobre a planta: os itens entre os objetos e os percursos reais das empilhadeiras, AGVs e pessoas (uma cor por recurso, espessura pela frequência), com a distância total e a média por viagem de cada recurso e dos itens. **Comparação:** `--comparar depois.ron` desenha os dois modelos lado a lado (o primeiro é o "antes") e mostra as distâncias e a variação. **Como gerar:** `--tipo espaguete --html e.html --csv e.csv`. A distância de um item é a do trajeto das ligações com tempo (`transfer`) ou, sem ele, a reta entre os dois objetos.

### Matriz de-para e diagrama de cordas (`de-para`)
**O que é:** para cada par de objetos, o número de viagens dos itens e a distância total (distância x viagens), em tabelas com sombreado e num diagrama de cordas (a espessura é o número de viagens). **Quando usar:** para achar os fluxos intensos e decidir quais objetos ficam perto. **Como gerar:** `--tipo de-para --html d.html --csv d.csv`.

### Gráfico homem-máquina (`homem-maquina`)
**O que é:** uma coluna de tempo para cada processo, montagem, desmontagem e recurso móvel, colorida pelo que ele faz (trabalhando, preparando, andando, esperando, ocioso, parado), com a ocupação (trabalhando mais preparando) num intervalo. **Como gerar:** `--tipo homem-maquina --de 4000 --ate 5000 --html h.html`; sem o intervalo, usa um décimo da rodada depois do aquecimento. Para o ciclo de trabalho de um operador com as suas máquinas, os tempos por elemento, quantas máquinas ele atende e o tempo padrão, use `utsim tempos` (capítulo [Estudo de tempos e métodos](tempos-e-metodos.md)).

### Mapa do fluxo de valor (`vsm`)
**O que é:** os processos do caminho principal com tempo de ciclo, preparação, disponibilidade (MTBF/(MTBF+MTTR)) e operadores; os estoques entre eles (fila média da rodada) e a linha do tempo: tempo de agregação de valor (só as operações; inspeções, esperas e transportes não agregam) contra o lead time medido (`time_in_system` da saída), com o índice de agregação de valor. **Como gerar:** `--tipo vsm --html v.html --csv v.csv`. O desenho cobre o fluxo de material; o fluxo de informação não existe no modelo.

## Limitações

- O desenho é do **modelo**, não da rodada: filas, tempos de espera e escolhas de caminho aleatórias só aparecem com `--com-resultados`.
- Fluxos de tokens (`flows`), fluidos e `spans` não entram no fluxograma.
- Na interface Godot, o fluxograma ganhará um botão "Fluxograma" (veja `docs/backlog_v02.md`).

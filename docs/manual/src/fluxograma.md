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

## Limitações

- O desenho é do **modelo**, não da rodada: filas, tempos de espera e escolhas de caminho aleatórias só aparecem com `--com-resultados`.
- Fluxos de tokens (`flows`), fluidos e `spans` não entram no fluxograma.
- Na interface Godot, o fluxograma ganhará um botão "Fluxograma" (veja `docs/backlog_v02.md`).

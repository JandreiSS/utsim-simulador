# Processos contínuos e batelada

A sexta **linha especializada** do UTSim estuda processos químicos e de alimentos feitos em
lotes: lotes de batelada com uma receita por produto, reatores que limpam entre um produto e
outro, tanques pulmão, envase e a parte contínua (tanques, bombas e envase contínuo) na rede de
fluidos. A linha usa o mesmo núcleo, o mesmo `.ron` e as mesmas replicações do resto do
programa.

A linha traz:

- uma **biblioteca** de 9 predefinições em `linhas/processos/biblioteca.ron`: programação de
  lotes, fila de lotes, reator de batelada, equipamento de processo, tanque pulmão de lotes,
  envase do lote, tanque de produto, equipe de operadores e um operador;
- um **vocabulário** (o item é o **lote**, o processo é o **reator**, a preparação é a
  **limpeza**), em `linhas/processos/vocabulario.ron`;
- uma peça nova do núcleo, que serve a qualquer modelo: o **tempo por receita** (`recipe`), o
  tempo de processamento que depende de um rótulo do item;
- os **indicadores** do processo com intervalo de confiança: lotes por dia, volume produzido
  por dia, rendimento, lotes fora de especificação, atravessamento do lote, o tempo de cada
  reator (produzindo, em limpeza, quebrado, bloqueado, sem lote), limpezas por dia, nível médio
  dos tanques, unidades envasadas por dia e ocupação dos operadores;
- três **modelos prontos**.

A limpeza entre produtos (CIP) é a troca de produto (`changeover`) da linha
[Manufatura discreta](../manufatura/index.md). Inspeção, reprocesso e descarte vêm do pacote
[Qualidade](../pacotes-transversais.md): o modelo pede `packs: ["qualidade"]` e usa as
predefinições de lá.

## Primeiro contato

```bash
utsim linha lista
utsim linha processos predefinicoes
utsim linha processos relatorio examples/batelada_cip.ron --html relatorio.html
utsim linha processos relatorio examples/xarope_envase.ron --rendimento-alvo 0.97
```

O `--rendimento-alvo` é o rendimento que os lotes deveriam alcançar, de 0 a 1 (padrão 0,95).
`utsim run modelo.ron --linha processos` mostra os indicadores depois do resultado de costume,
com a meta padrão.

## O lote

O lote é um item com rótulos, sorteados na programação:

| Rótulo | O que é |
|---|---|
| `produto` | a receita do lote (1, 2, 3...), por exemplo `Discrete([(1.0, 0.5), (2.0, 0.3), (3.0, 0.2)])` |
| `volume` | litros carregados no lote |
| `rendimento` | fração do volume que vira produto; sem o rótulo, conta 1 |

O volume produzido de um lote é `volume × rendimento`. O lote que termina numa saída com
`sink_role: Some(Scrap)` está **fora de especificação** e não conta como produto.

## Receita e limpeza

O reator de batelada é um `Processor` com dois campos:

```text
(name: "reator", kind: Processor(
    setup: Some(Const(60.0)),                         // limpeza padrão
    process: Triangular(min: 150.0, mode: 180.0, max: 220.0),   // produto sem receita
    changeover: Some((label: "produto", matrix: [
        (from: 1.0, to: 3.0, time: Const(40.0)),      // do claro para o escuro: curto
        (from: 3.0, to: 1.0, time: Const(120.0)),     // do escuro para o claro: longo
    ])),
    recipe: Some((label: "produto", times: [
        (value: 1.0, time: Triangular(min: 150.0, mode: 180.0, max: 220.0)),
        (value: 2.0, time: Triangular(min: 190.0, mode: 220.0, max: 260.0)),
    ])),
)),
```

- `recipe` dá o tempo de cada receita. O lote cujo `produto` não aparece na tabela usa o
  `process`.
- `changeover` faz a limpeza só quando o produto muda, com o tempo da matriz (de, para) ou, sem
  linha na matriz, o `setup`. O primeiro lote não limpa.

Os dois campos são do núcleo e servem a qualquer processo, não só a esta linha. O campo
`recipe` aparece no painel do UTSim 3D como "Tempo por receita".

## A parte contínua

Tanques de matéria-prima e de produto, bombas e envase contínuo ficam na **rede de fluidos** do
modelo (`fluids`, veja [Fluidos](../../fluidos.md)). Dois nós ligam os lotes à rede:

- `FromItems(object: "descarga", volume: 2000.0)`: cada lote que entra na saída `descarga`
  vira 2000 L de uma vez no tanque seguinte;
- `ToItems(object: "fila_fardos", volume: 12.0, max_rate: Some(11.0))`: o envase contínuo puxa
  até 11 L por minuto e cria um fardo a cada 12 L.

As unidades criadas pelo `ToItems` não são lotes: contam como **unidades envasadas por dia**.
O `FromItems` para o modelo com erro quando o tanque não tem lugar para o lote inteiro. Dê ao
tanque capacidade para alguns lotes, ou um envase mais rápido que a batelada.

## Indicadores

Todos com intervalo de confiança de 95 % entre replicações:

| Indicador | Como sai |
|---|---|
| Lotes por dia | lotes bons que saíram na janela de estatística ÷ dias |
| Volume produzido por dia | soma de `volume × rendimento` dos lotes bons ÷ dias (só com o rótulo `volume`) |
| Rendimento médio | média do rótulo `rendimento` dos lotes bons, comparada com o alvo |
| Unidades envasadas por dia | unidades do envase contínuo (`ToItems`) que saíram ÷ dias |
| Tempo de atravessamento do lote | da programação até a saída, no total e por produto |
| Lotes fora de especificação | lotes no descarte ÷ todos os que terminaram |
| Tempo dos equipamentos | frações do tempo de turno produzindo, em limpeza, quebrado, bloqueado (lote pronto sem lugar adiante) e sem lote |
| Limpezas por dia | limpezas de cada reator com `changeover` ÷ dias |
| Ciclo do lote | tempo médio do lote dentro do equipamento (limpeza, receita e espera para sair) |
| Nível médio dos tanques | nível médio de cada tanque da rede de fluidos ÷ capacidade |
| Ocupação dos operadores | fração do turno andando ou operando |

Os equipamentos são os processos do modelo, exceto as estações de inspeção e de reprocesso.

## Modelos prontos

| Modelo | O que mostra |
|---|---|
| `examples/batelada_cip.ron` | dois reatores com fila única, três receitas, limpeza que depende da sequência, quebras, tanque pulmão e envase do lote |
| `examples/xarope_envase.ron` | um reator de dois sabores que descarrega num tanque de 20 000 L, e um envase contínuo que faz fardos de 12 L |
| `examples/tintas_lotes.ron` | dispersor com receita e limpeza por cor, completagem, laboratório que confere o tom, ajuste de cor e descarte, operadores com habilidades |

Resultados com 5 replicações (média):

- **Batelada com limpeza.** Cerca de 7,5 lotes e 14 300 L por dia, rendimento de 95,4 % e
  atravessamento de 480 min. Os reatores ficam 52 a 60 % do tempo produzindo e cerca de 10 % em
  limpeza (2,4 limpezas por dia cada). Com a fila em campanha (`Some(Asc("produto"))`), o
  atravessamento cai para cerca de 460 min e quase nada mais muda: a fila raramente guarda
  vários lotes, então há pouco para juntar. A campanha compensa quando a fila é longa.
- **Xarope e envase contínuo.** Cerca de 6,9 lotes por dia e 1150 fardos por dia. O reator
  fica 73 % do tempo produzindo e 10 % em limpeza. O tanque de xarope tem nível médio de 8,5 %,
  porque o envase (11 L por minuto) é mais rápido que a batelada (cerca de 9 L por minuto) e
  passa parte do tempo parado à espera do próximo lote.
- **Tintas.** Cerca de 13,3 lotes por dia, rendimento de 96,7 % e 0,8 % de lotes descartados.
  O dispersor fica 61 % do tempo produzindo e 16 % em limpeza (8,6 limpezas por dia). O primeiro
  operador, o único que sabe tingir, fica 74 % ocupado, e o segundo, 8 %.

## Decisões em aberto

As decisões C-D1 a C-D6 da seção 11 de `docs/plano_linhas_especializadas.md` (nome da linha,
rendimento sorteado na programação, volume fixo do `FromItems`, transbordo como erro, tempo do
lote dentro do reator, modelos prontos) seguem com as propostas até o dono responder.

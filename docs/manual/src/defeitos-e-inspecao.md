# Defeitos, inspeção e retrabalho

Um processo pode gerar defeitos e medir características de cada peça. Uma estação de inspeção decide se a peça segue, volta para o retrabalho ou vai para o refugo. Tudo isso fica no próprio modelo `.ron`, em blocos opcionais. O formato completo está em `docs/formato_qualidade_seguranca.md`, e o exemplo pronto é `examples/inspecao_e_retrabalho.ron`.

## Defeito por processo

```ron
(name: "torno", kind: Processor(process: Const(1.5)),
 quality: Some((
    defects: [
        (name: "risco", probability: 0.02, factors: [Cycle(every: 100, first: 5, times: 3.0)]),
    ],
    characteristics: [
        (name: "diametro", value: Normal(mean: 25.0, sd: 0.012), drift: 0.0002,
         cycle: Some(100), lsl: Some(24.95), usl: Some(25.05)),
    ],
 ))),
```

Cada peça que entra no torno sorteia o defeito `risco` com 2 % de chance. O fator `Cycle` triplica essa chance nas 5 primeiras peças de cada ciclo de 100 (logo depois da troca de ferramenta). O fator `Label(label: "turno", equals: 3, times: 1.5)` multiplica a chance quando a peça tem o rótulo `turno` igual a 3.

A característica `diametro` é sorteada da normal, cresce `drift` a cada peça do ciclo e recomeça a cada `cycle` peças. Fora dos limites `lsl` e `usl`, a peça ganha o defeito `diametro`.

A peça carrega rótulos: `defeito.risco` vale 1 quando o defeito está presente e 0 quando foi corrigido, `defeitos` conta os presentes e `diametro` guarda o valor medido.

## Inspeção

```ron
(name: "inspecao", kind: Processor(process: Const(0.8)), output: ByLabel("inspecao"),
 inspection: Some((
    detect: 0.9, false_reject: 0.005,
    measure: [(characteristic: "diametro", sigma: 0.004)],
    on_reject: Rework, max_rework: Some(2),
 ))),
```

A inspeção escreve no rótulo `inspecao` a porta de saída da peça: 1 aprovada, 2 retrabalho, 3 refugo depois de `max_rework` passagens. Por isso a saída é `ByLabel("inspecao")` e as ligações que saem da inspeção seguem essa ordem. Com `on_reject: Scrap`, a porta 2 é o refugo.

- `every: n` inspeciona uma peça a cada n. As outras passam.
- `detect` é a chance de ver um defeito presente, e `detect_by` muda essa chance por tipo.
- `false_reject` é a chance de rejeitar uma peça boa.
- `measure` mede a característica somando um erro normal de desvio `sigma`, o desvio do sistema de medição que sai da MSA. Perto dos limites, a inspeção aprova peça ruim e rejeita peça boa.

## Retrabalho e refugo

```ron
(name: "retrabalho", kind: Processor(process: Exp(mean: 3.0)), rework: Some((fix: 0.9))),
(name: "refugo", kind: Sink, sink_role: Some(Scrap)),
(name: "cliente", kind: Sink, sink_role: Some(Customer)),
```

O retrabalho soma 1 no rótulo `passagens` e corrige cada defeito com a chance `fix`. A peça volta para a inspeção pela ligação comum. Nenhuma peça passa do limite de retrabalho, porque a inspeção manda para o refugo quem já usou as passagens.

`sink_role` diz ao relatório qual saída é o refugo e qual é o cliente. Sem papel, a saída conta como cliente.

## Relatório de qualidade

```bash
utsim qualidade simulado examples/inspecao_e_retrabalho.ron --html qualidade.html --csv medicoes.csv
```

O comando roda as replicações e mostra, com intervalo de 95 %, o rendimento de primeira passagem, o rendimento acumulado, o refugo, o retrabalho, os defeitos por unidade, o DPMO, o escape para o cliente e a eficácia da inspeção. O HTML traz ainda o Pareto por tipo de defeito e por posto, a capacidade de cada característica e a carta de valores individuais da primeira replicação. O CSV tem a saída longa (replicação, item, objeto, característica, instante e valor), que `utsim qualidade carta` e `utsim qualidade capacidade` leem.

Os números são de origem **Simulado**. Eles servem para estudar onde o defeito nasce, onde é detectado e o efeito de mudar a inspeção, o método ou a máquina.

## Custo da qualidade

Os blocos aceitam custos unitários, que a simulação nunca lê e que o relatório de custos multiplica pelas contagens de cada replicação:

| Campo | Categoria | Como entra |
|---|---|---|
| `inspection.cost` | avaliação | por peça inspecionada |
| `rework.cost` | falha interna | por passagem pelo retrabalho |
| `quality.unit_cost` | falha interna | valor que o processo agrega, perdido por peça refugada que passou por ele |
| `quality.escape_cost` | falha externa | por defeito do processo que chega ao cliente |

```bash
utsim qualidade custos examples/inspecao_e_retrabalho.ron --lancamentos lancamentos.csv --comparar outro_cenario.ron --html custos.html
```

Os lançamentos digitados (treinamento, calibração, garantia, devoluções) ficam num CSV `categoria;descricao;valor`, com as categorias `prevencao`, `avaliacao`, `falha_interna` e `falha_externa`. O relatório mostra o total por categoria (média e intervalo de 95 % do que vem do modelo, mais os lançamentos), por posto e por tipo de defeito. Com `--comparar`, o outro cenário roda com as mesmas sementes e a diferença do total é calculada replicação a replicação.

## Planos de amostragem por atributos

```bash
utsim qualidade amostragem --p1 0.01 --alfa 0.05 --p2 0.06 --beta 0.10 --lote 1000 --html plano.html
utsim qualidade amostragem --n 80 --c 2 --lote 500
```

O comando projeta o plano simples de menor amostra que atende aos dois riscos: aceitar com probabilidade de pelo menos `1 - alfa` um lote com fração defeituosa `p1` (o nível de qualidade aceitável) e com probabilidade de no máximo `beta` um lote com `p2` (a qualidade limite). Com `--duplo`, projeta um plano de duas amostras iguais com o menor tamanho médio de amostra em `p1`. Com `--n` e `--c`, avalia um plano dado. A distribuição é a binomial (padrão), a hipergeométrica (`--dist hiper`, lote finito) ou a de Poisson.

O relatório mostra a curva característica de operação, a qualidade média que sai com inspeção retificadora (AOQ) e o seu limite (AOQL), a inspeção total média por lote (ATI) e, no plano duplo, o tamanho médio da amostra (ASN). Os planos são calculados pelo programa. As tabelas das normas de amostragem não são embutidas: quem tem direito de uso cadastra os planos por CSV (`nome;n;c` ou `nome;n1;c1;r1;n2;c2`).

Com `--aplicar modelo.ron --estacao inspecao --saida novo.ron`, o plano vira a fração amostral da estação de inspeção do modelo (uma peça a cada `lote / n`) e o simulador mostra o efeito sobre o escape, o custo e o fluxo. A decisão por lote e a inspeção completa dos lotes rejeitados não são simuladas peça a peça, e o relatório do plano as dá pelas fórmulas.

Os planos de controle (característica, especificação, método, sistema de medição, frequência, amostra, carta e reação) e de inspeção (o que, como, quando, quem e o plano de amostragem) ficam por posto na pasta da unidade, no app UTSim Qualidade, e podem ser sugeridos a partir do modelo.

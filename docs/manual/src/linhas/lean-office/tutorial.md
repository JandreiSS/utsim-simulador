# Tutorial: do mapa ao modelo

Este tutorial leva um processo administrativo do mapa no quadro branco até a comparação entre o
estado atual e o estado futuro. O processo é o **reembolso de despesas**: o funcionário pede o
reembolso, o financeiro lança o pedido, junta os pedidos num lote, o gestor aprova, o malote
leva o lote à tesouraria e a tesouraria paga.

O resultado está pronto em `examples/reembolso_atual.ron` e `examples/reembolso_futuro.ron`,
para conferir cada passo.

## 1. O mapa

Antes do modelo, o mapa: as etapas, quem faz cada uma, quanto trabalho ela dá e onde o caso
espera. Para o reembolso, a equipe levantou:

| Etapa | Departamento | Quem faz | Trabalho por caso | O que se sabe |
|---|---|---|---|---|
| Pedido | Solicitante | o funcionário | | um pedido a cada 45 min, em média; prazo de 2 dias |
| Lançamento | Financeiro | 2 analistas | 5 a 20 min, 10 o mais comum | 10 % saem com algum erro |
| Lote | Financeiro | | | 10 pedidos, ou o que houver em 8 h |
| Aprovação | Gerência | 2 gestores | 5 a 20 min | 10 % voltam ao financeiro |
| Malote | Financeiro | | | de 1 a 8 h até a tesouraria |
| Pagamento | Tesouraria | os analistas | 3 a 10 min, 5 o mais comum | |

Cada linha da tabela vira uma predefinição: o pedido é uma **entrada de pedidos**, antes do
lançamento há uma **caixa de entrada**, o lançamento e o pagamento são **postos de trabalho**,
o lote é um **agrupamento em lote**, a aprovação é uma **aprovação**, o malote é uma
**tramitação** e o fim é uma **conclusão**. Os analistas e os gestores são duas **equipes**.

## 2. O modelo, uma predefinição por vez

Cada comando coloca uma predefinição em `reembolso.ron` (o primeiro cria o arquivo). A
posição só serve para o desenho; o departamento vira a raia do fluxograma.

```bash
utsim linha lean_office colocar reembolso.ron entrada --nome pedidos --campo 'intervalo=Exp(mean: 45.0)' --campo 'rotulos={"sla": Const(2880.0)}' --departamento Solicitante
utsim linha lean_office colocar reembolso.ron caixa_entrada --posicao 2 0 --departamento Financeiro
utsim linha lean_office colocar reembolso.ron posto --nome lancamento --posicao 4 0 --campo 'tempo=Triangular(min: 5.0, mode: 10.0, max: 20.0)' --campo ca=0.9 --departamento Financeiro
utsim linha lean_office colocar reembolso.ron lote --posicao 6 0 --campo tamanho=10 --departamento Financeiro
utsim linha lean_office colocar reembolso.ron aprovacao --posicao 8 0 --campo 'aprovador="gestores"' --departamento Gerência
utsim linha lean_office colocar reembolso.ron tramitacao --nome malote --posicao 10 0 --campo 'tempo=Uniform(min: 60.0, max: 480.0)' --departamento Financeiro
utsim linha lean_office colocar reembolso.ron posto --nome pagamento --posicao 12 0 --campo 'tempo=Triangular(min: 3.0, mode: 5.0, max: 10.0)' --departamento Tesouraria
utsim linha lean_office colocar reembolso.ron conclusao --posicao 14 0 --departamento Tesouraria
utsim linha lean_office colocar reembolso.ron equipe --posicao 4 2 --departamento Financeiro
utsim linha lean_office colocar reembolso.ron equipe --nome gestores --posicao 8 2 --departamento Gerência
```

A cada comando o UTSim lista as pendências: casos sem ter para onde ir e postos de trabalho
de uma equipe que ainda não existe. Elas somem nos próximos passos. Os nomes que o comando
escolheu (`caixa`, `lote`, `aprovacao`, `concluido`, `equipe`) aparecem na saída de cada
comando.

## 3. As ligações e a rodada

As ligações dizem por onde o caso anda. Abra `reembolso.ron` num editor de texto e troque
`connections: [],` por:

```ron
connections: [
    ("pedidos", "caixa"),
    ("caixa", "lancamento"),
    ("lancamento", "lote"),
    ("lote", "aprovacao"),
    ("aprovacao", "malote"),   // primeira saída: aprova
    ("aprovacao", "caixa"),    // segunda saída: devolve
    ("malote", "pagamento"),
    ("pagamento", "concluido"),
],
```

A ordem das duas saídas da aprovação importa: a primeira segue, a segunda devolve. Depois
troque o bloco `run` para rodar 30 dias, descartar os 3 primeiros e repetir 5 vezes:

```ron
run: (stop_time: 43200.0, warmup: 4320.0, seed: 5, replications: 5),
```

O tempo é corrido, sem horário de trabalho, para o tutorial ficar curto. Para usar o horário
comercial, copie o calendário `"expediente"` de um dos [modelos prontos](modelos-prontos.md) e
aplique na entrada de pedidos e nas pessoas.

Confira e rode:

```bash
utsim check reembolso.ron
utsim linha lean_office relatorio reembolso.ron --html atual.html
```

## 4. O que o estado atual mostra

Com os números do mapa, cada caso recebe cerca de 32 minutos de trabalho (touch time) e leva
cerca de 9,4 horas do pedido ao pagamento: a razão de atividade fica perto de 6 %. O Pareto das
esperas mostra onde está o resto: a espera para completar o lote e o malote. O FPY fica perto
de 81 %, pelos erros do lançamento e pelas devoluções da aprovação.

No relatório, o mapa do fluxo de valor do processo administrativo mostra a escada do lead time, com a
espera em vermelho e o trabalho em verde, e o fluxograma de raias mostra as passagens entre
Solicitante, Financeiro, Gerência e Tesouraria.

## 5. O estado futuro

A equipe propôs três mudanças:

1. **Lista de conferência no lançamento**: o %C&A do lançamento sobe de 90 % para 98 %.
2. **Fluxo unitário**: o lote sai, e cada pedido lançado vai direto para a aprovação.
3. **Pedido pelo sistema**: o malote vira uma passagem de 5 a 15 minutos.

Copie `reembolso.ron` para `futuro.ron` e faça as três mudanças no arquivo: no `on_entry` do
`lancamento`, troque `0.9` por `0.98`; apague o objeto `lote` e troque as ligações
`("lancamento", "lote")` e `("lote", "aprovacao")` por `("lancamento", "aprovacao")`; no
`malote`, troque o tempo por `Uniform(min: 5.0, max: 15.0)`. Depois compare:

```bash
utsim linha lean_office relatorio reembolso.ron --comparar futuro.ron --html comparacao.html
```

O lead time cai de cerca de 9,4 horas para menos de 1 hora, com o mesmo touch time, e o FPY
sobe para perto de 88 % (as devoluções da aprovação continuam). O relatório mostra os dois
mapas do fluxo de valor, um embaixo do outro, e a tabela de indicadores lado a lado, cada um
com o seu intervalo de confiança.

## 6. Próximos passos

- Ligue o horário comercial e veja quanto do lead time é noite e fim de semana.
- Teste só uma mudança por vez, para saber quanto cada uma vale.
- Dê aos analistas a [multitarefa](multitarefa.md) e veja o efeito das trocas.
- Leve o estudo ao UTSim Seis Sigma com o `--json` do relatório.

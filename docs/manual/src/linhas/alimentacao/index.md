# Alimentação e food service

A sétima **linha especializada** do UTSim estuda lanchonetes, restaurantes e cozinhas de
delivery: pedidos que chegam em picos, filas com desistência, um pedido que se divide entre as
estações da cozinha e se junta de novo, mesas que giram e comida pronta que esfria esperando o
entregador. A linha usa o mesmo núcleo, o mesmo `.ron` e as mesmas replicações do resto do
programa.

A linha traz:

- uma **biblioteca** de 12 predefinições em `linhas/alimentacao/biblioteca.ron`: chegada de
  pedidos, fila de clientes, caixa, comanda, estação da cozinha, montagem do pedido, balcão de
  retirada, salão com mesas, entrega, equipe, funcionário e saída;
- um **vocabulário** (o item é o **pedido**, o processo é a **estação**, a divisão é a
  **comanda**), em `linhas/alimentacao/vocabulario.ron`;
- os **indicadores** do restaurante com intervalo de confiança: pedidos por dia e na hora de
  pico, tempo do pedido, pedidos no prazo, desistências, comida descartada, o mesmo por canal, o
  tempo de cada estação, a ocupação e o giro das mesas e a ocupação da equipe;
- três **modelos prontos**.

A linha não pede nenhuma peça nova do núcleo. A desistência na fila vem da linha
[Serviços e atendimento](../servicos/index.md), e a divisão e a montagem do pedido vêm da linha
[Logística e armazém](../logistica/index.md).

## Primeiro contato

```bash
utsim linha lista
utsim linha alimentacao predefinicoes
utsim linha alimentacao relatorio examples/lanchonete_balcao.ron --prazo 12 --html relatorio.html
utsim linha alimentacao relatorio examples/cozinha_delivery.ron --prazo 45
```

O `--prazo` é o tempo do pedido, em minutos, dentro do qual ele está no prazo (padrão 15).
`utsim run modelo.ron --linha alimentacao` mostra os indicadores depois do resultado de costume,
com o prazo padrão.

## O pedido

O pedido é um item com rótulos, sorteados na chegada:

| Rótulo | O que é |
|---|---|
| `canal` | 1 balcão, 2 drive-thru, 3 delivery; sem o rótulo, conta 1 |
| `itens` | quantas partes o pedido tem: 1 só o lanche, 2 com acompanhamento, 3 com bebida |

No restaurante com mesas, o "pedido" é o grupo de clientes: ele chega, espera mesa, come e paga.

## A comanda e a montagem

A comanda é um `Separator` que divide o pedido nas suas partes e manda a parte k para a sua
saída k:

```text
(name: "comanda", kind: Separator(mode: SplitBy(count: "itens", group: "comanda"),
    process: Const(0.0)), output: ByLabel("linha")),
(name: "montagem", kind: Queue(gather: Some((group: "comanda", count: "itens")))),
connections: [
    ("comanda", "chapa"),       // saída 1: o lanche
    ("comanda", "fritadeira"),  // saída 2: o acompanhamento
    ("comanda", "bebidas"),     // saída 3: a bebida
    ("chapa", "montagem"), ("fritadeira", "montagem"), ("bebidas", "montagem"),
],
```

A montagem segura as partes até todas chegarem. Então a primeira parte que chegou sai como o
pedido inteiro, com as outras dentro. Cada pedido conta uma vez, e o seu tempo vai da chegada do
pedido até a saída.

## Desistência e comida fria

As duas usam a paciência de uma fila (`patience`), mas a linha as conta separado:

- na **fila de clientes**, quem espera mais que a paciência **desiste** e vai embora;
- no **balcão de retirada** (predefinição `retirada`), o pedido pronto que espera mais que o
  tempo até esfriar é **descartado**.

## Indicadores

Todos com intervalo de confiança de 95 % entre replicações:

| Indicador | Como sai |
|---|---|
| Pedidos por dia | pedidos que saíram na janela de estatística ÷ dias |
| Pedidos na hora de pico | pedidos por dia que chegaram na hora mais cheia do dia |
| Tempo do pedido | da chegada do pedido até a saída: a média e o tempo de 90 % dos pedidos |
| Pedidos no prazo | pedidos com tempo até o `--prazo` ÷ pedidos |
| Desistências | quem desistiu ÷ (pedidos + desistências + descartes) |
| Pedidos descartados por dia | pedidos que esfriaram no balcão de retirada ÷ dias |
| Por canal | pedidos, tempo, prazo, desistências e descartes de cada canal |
| Estações | ocupação (lugares trabalhando ÷ lugares), parada, bloqueada, quebrada, feitas e tempo na estação |
| Salões | ocupação das mesas, giro por mesa por dia e tempo à mesa |
| Pedidos por hora do dia | pedidos por dia que chegaram em cada hora |
| Ocupação da equipe | fração do turno andando ou trabalhando |

As estações e os salões são medidos no seu turno ou, sem turno próprio, no **horário de
funcionamento**, quando a loja recebe pedidos. Assim a cozinha fechada à noite não aparece como
estação parada.

## Modelos prontos

| Modelo | O que mostra |
|---|---|
| `examples/lanchonete_balcao.ron` | pico do almoço, dois caixas com desistência, comanda entre chapa, fritadeira e bebidas, montagem |
| `examples/restaurante_mesas.ron` | grupos que esperam mesa e desistem, salão com 25 mesas, caixa com os garçons |
| `examples/cozinha_delivery.ron` | pedidos de aplicativo com dois picos, preparo, embalagem, balcão onde a comida esfria, entregadores em rota |

Resultados com 5 replicações (média):

- **Lanchonete.** Cerca de 225 pedidos por dia, 81 entre 12h e 13h. O pedido leva 6,5 min (8,2
  min para 90 % deles), mas 8,2 % dos clientes desistem na fila do caixa. Os dois caixas ficam
  62 % ocupados no horário de funcionamento, e a chapa, 51 %. Com um terceiro caixa e um
  terceiro atendente, as desistências caem para 2,8 % e saem cerca de 238 pedidos por dia.
- **Restaurante.** Cerca de 65 grupos por noite; 4,6 % desistem na porta. As 25 mesas ficam
  62 % ocupadas e giram 2,6 vezes por noite; cada grupo fica 74 min à mesa. Com 30 mesas, as
  desistências caem para cerca de 1 %, e a ocupação, para 53 %.
- **Cozinha de delivery.** Cerca de 315 pedidos por dia, com 48 entre 18h e 19h. O pedido leva
  42 min até o cliente, e 64 % chegam em até 45 min. No jantar faltam entregadores: cerca de 15
  pedidos por dia esfriam no balcão. Com 18 entregadores em rota, o descarte some, e 87 % dos
  pedidos chegam no prazo.

## Decisões em aberto

As decisões A-D1 a A-D6 da seção 12 de `docs/plano_linhas_especializadas.md` (nome da linha,
fim do tempo do pedido, entregadores, comida fria, prazo, modelos prontos) seguem com as
propostas até o dono responder.

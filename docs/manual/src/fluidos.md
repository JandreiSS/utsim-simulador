# Fluidos

Alguns processos movem líquidos ou pós em vez de peças: tanques, bombas, envase. O UTSim tem uma **rede de fluidos** que convive com os objetos discretos. Ela é um bloco só dentro do modelo, com tanques, tubos, fontes, sumidouros e conversores entre itens e fluido.

A rede é um grafo **sem ciclos**. Entre dois eventos, todas as vazões são constantes, então os níveis dos tanques variam em linha reta e o motor calcula exatamente o instante em que um tanque enche, esvazia ou um conversor completa um volume.

```ron
fluids: Some((
    name: "fluidos",
    nodes: [
        (name: "caminhao_para_fluido", kind: FromItems(object: "descarga", volume: 6.0)),
        (name: "armazenamento", kind: Tank(capacity: 100.0, initial: 20.0)),
        (name: "bomba", kind: Pipe(max_rate: 3.0)),
        (name: "processo", kind: Tank(capacity: 30.0, initial: 5.0, max_out: 2.5)),
        (name: "envase", kind: ToItems(object: "fila_garrafas", volume: 0.5, max_rate: 2.0)),
    ],
    links: [
        ("caminhao_para_fluido", "armazenamento"),
        ("armazenamento", "bomba"),
        ("bomba", "processo"),
        ("processo", "envase"),
    ],
)),
```

O modelo completo está em `examples/tanques.ron`. Nele, a unidade de tempo é a hora e a de volume, o litro.

## Campos da rede

| Campo | Padrão | Significado |
|---|---|---|
| `name` | `"fluidos"` | nome do bloco nos relatórios |
| `nodes` | obrigatório | os nós (nome, tipo e, opcionalmente, `position`) |
| `links` | obrigatório | ligações `(origem, destino)` entre nós; não podem formar ciclo |

## Tipos de nó

| Tipo | Campos (padrão) | O que faz |
|---|---|---|
| `Source` | `rate` | fornece até `rate` por unidade de tempo, sem limite de volume |
| `Tank` | `capacity`, `initial` (`0`), `max_in` (sem limite), `max_out` (sem limite) | armazena até `capacity`. `max_in` e `max_out` limitam as vazões |
| `Pipe` | `max_rate` | deixa o fluido passar, sem guardar, até `max_rate` |
| `Sink` | `max_rate` (sem limite) | absorve fluido |
| `FromItems` | `object`, `volume` | cada item que entra no objeto discreto `object` (normalmente uma `Sink`) vira `volume` de fluido, entregue de uma vez no tanque seguinte |
| `ToItems` | `object`, `volume`, `max_rate` (sem limite) | acumula o fluido que recebe e, a cada `volume` completo, cria um item no objeto `object` |

Todos os valores precisam ser maiores que 0, e o nível inicial precisa caber na capacidade. Um objeto só pode receber itens de um conversor `ToItems`.

## Como as vazões são calculadas

1. cada fonte oferece até a sua taxa;
2. um tanque com nível maior que zero oferece até `max_out`. Um tanque cheio só aceita o que consegue escoar;
3. tubos e sumidouros limitam a vazão;
4. várias saídas de um nó dividem igualmente o que ele oferece, e o que uma saída recusa volta para as outras;
5. vale a conservação: o que sai de um nó é igual ao que entra, menos a variação do nível.

## Relatório

Cada nó aparece no relatório do bloco de fluidos:

- `<nó>.nivel_medio`, `<nó>.nivel_max` e `<nó>.nivel_final` para os tanques;
- `<nó>.volume`: o volume total que passou por cada nó.

## No visualizador

No 3D (modo Simular), os tanques aparecem como cilindros translúcidos com o líquido azul na altura proporcional ao nível. Fontes, sumidouros, tubos e conversores têm formas próprias, e as ligações ficam ciano quando há vazão. Se as posições dos nós são todas zero, eles são dispostos em camadas automaticamente.

No modo Editar, a camada **Fluidos** mostra a rede com os níveis iniciais e permite montá-la: escolha o tipo de nó (Fonte, Tanque, Tubulação, Sumidouro, Itens em fluido, Fluido em itens) e clique no chão, ligue os nós com a ferramenta **Ligar** e ajuste os parâmetros no painel. Os conversores escolhem o objeto discreto numa lista. Ciclos e referências erradas aparecem na hora entre os problemas.
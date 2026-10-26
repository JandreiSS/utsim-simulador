# Conexões e regras de saída

As **conexões** dizem por onde os itens andam entre os objetos. Cada conexão é um par `(origem, destino)`:

```ron
connections: [
    ("chegada", "fila"),
    ("fila", "maquina"),
    ("maquina", "saida"),
]
```

## Como uma transferência acontece

O ciclo de todo objeto fixo é **receber, processar, liberar, transferir**.

1. **Receber.** Um objeto aceita um item quando a sua entrada está aberta (normalmente, quando tem espaço).
2. **Processar.** Cada tipo decide quanto tempo o item fica. A fila libera na hora, o processo segura durante o setup e o processamento.
3. **Liberar.** O objeto declara o item pronto para sair. Nesse instante a **regra de saída** escolhe a porta.
4. **Transferir.** A conexão anda quando os dois lados estão abertos: há um item liberado na saída e o destino aceita.

A transferência entre dois objetos fixos ligados diretamente é, por padrão, **instantânea**. Ela não leva tempo, mesmo que os objetos estejam a 100 metros um do outro. Para que o transporte leve tempo, use o `transfer` (tempo de viagem pela distância), uma [esteira](esteiras-e-porta-paletes.md) ou um [transportador](recursos-moveis.md). Veja a seção "A distância importa?" em [Recursos móveis](recursos-moveis.md).

## Portas e prioridade

As **portas de saída** de um objeto são numeradas pela ordem das conexões em que ele é a origem. A primeira conexão é a porta 1, a segunda é a porta 2, e assim por diante.

As **portas de entrada** seguem a ordem das conexões em que ele é o destino. Essa ordem é a **prioridade de puxar**: se duas filas alimentam a mesma máquina, a que foi ligada primeiro tem preferência quando a máquina fica livre. O Combiner usa as portas de entrada para distinguir o contêiner (porta 0) dos componentes (portas 1, 2...).

## Regras de saída

A regra fica no campo `output` do objeto de origem. É escolhida **uma vez**, no instante em que o item é liberado.

| Regra | O que faz |
|---|---|
| `FirstAvailable` (padrão) | qualquer porta; vence a primeira aberta, na ordem das portas |
| `RoundRobin` | uma porta de cada vez, em rodízio. O item espera aquela porta, mesmo que outra esteja livre |
| `Random` | uma porta sorteada com a mesma probabilidade |
| `Weighted([p1, p2, ...])` | porta sorteada com pesos relativos, um por porta de saída. Cada peso é maior ou igual a 0 e a soma é maior que 0 |
| `ByLabel("rótulo")` | usa o número do rótulo do item como porta (começando em 1) |
| `Script("...")` | um pequeno script devolve a porta (começando em 1). Veja [Fluxos de tokens e scripts](fluxos-e-scripts.md) |

Com qualquer regra que não seja `FirstAvailable`, o item espera pela porta escolhida mesmo que outra esteja livre. Isso é importante: com `RoundRobin`, se a porta 2 está bloqueada, o item da vez espera por ela.

### Exemplo: dois tipos de peça

```ron
(name: "pedidos", kind: Source(
    interarrival: Exp(mean: 1.0),
    labels: { "tipo": Discrete([(1.0, 0.6), (2.0, 0.4)]) },
)),
(name: "triagem", kind: Queue(), output: ByLabel("tipo")),
(name: "celula_a", kind: Processor(process: Exp(mean: 1.2))),
(name: "celula_b", kind: Processor(capacity: 2, process: Erlang(k: 3, mean: 3.0))),
```

com as conexões `("triagem", "celula_a")` primeiro e `("triagem", "celula_b")` depois. O tipo 1 vai para a porta 1 (`celula_a`) e o tipo 2 para a porta 2 (`celula_b`). O modelo completo está em `examples/roteamento_por_tipo.ron`.

Se o rótulo não existe, não é inteiro ou está fora de 1 até o número de portas, a simulação para com uma mensagem de erro dizendo qual rótulo e qual valor.

### Exemplo: divisão por pesos

```ron
(name: "triagem", kind: Queue(), output: Weighted([3.0, 1.0]))
```

Com duas conexões de saída, 75% dos itens vão para a porta 1 e 25% para a porta 2.

## O que o `utsim check` recusa

- objeto sem saída conectada (os itens ficariam presos), exceto a `Sink`;
- conexão repetida, de um objeto para ele mesmo, ou para um nome que não existe;
- conexão que sai de uma `Sink` ou chega numa `Source`;
- conexão com um recurso móvel (use `transport` ou `operators`);
- `Weighted` com número de pesos diferente do número de portas de saída;
- `Combiner` com número de entradas diferente de `components.len() + 1`.
# Fluxos de tokens e scripts

Alguns comportamentos não cabem nos objetos prontos: caminhões que disputam docas, regras de prioridade, decisões que dependem do estado. Para isso o UTSim tem duas ferramentas:

- **fluxos de tokens**: um desenho da lógica como um grafo de atividades;
- **scripts**: pequenas instruções em Rhai, colocadas em poucos pontos do modelo.

## Fluxos de tokens

Um **token** não é um item. É uma "linha de raciocínio" leve que espera, decide, segura recursos e pode carregar a referência de um item. Cada fluxo é um bloco da simulação: não aparece como caixa no 3D, mas aparece nos relatórios.

Um fluxo tem um nome, recursos compartilhados opcionais e uma lista de atividades. Cada atividade diz para qual vem em seguida (`next`).

```ron
flows: [
    (
        name: "recebimento",
        resources: [(name: "docas", capacity: 2)],
        activities: [
            (name: "chegada", kind: Source(interarrival: Exp(mean: 12.0)), next: ["espera_doca"]),
            (name: "espera_doca", kind: Acquire(resource: "docas"), next: ["descarga"]),
            (name: "descarga", kind: Delay(time: Uniform(min: 15.0, max: 25.0)), next: ["palete_1"]),
            (name: "palete_1", kind: CreateItem(into: "paletes"), next: ["libera_doca"]),
            (name: "libera_doca", kind: Release(resource: "docas"), next: ["partida"]),
            (name: "partida", kind: Sink),
        ],
    ),
],
```

O modelo completo, com três paletes por caminhão, está em `examples/fluxo_docas.ron`.

### As atividades

| Atividade | Campos (padrão) | O que faz |
|---|---|---|
| `Source` | `interarrival`, `first_at_zero` (`false`), `limit` (sem limite) | cria tokens em intervalos aleatórios |
| `Schedule` | `times`: lista de `(instante, quantidade)` | cria tokens em horários fixos |
| `OnEvent` | `block`, `event` (`Entry` ou `Exit`) | cria um token, carregando o item, a cada entrada ou saída num objeto |
| `Delay` | `time` | segura o token por um tempo sorteado |
| `Decide` | `rule` | escolhe um dos ramos de `next` |
| `Assign` | `labels`: nome para expressão | atribui rótulos ao token (`nome`) ou ao item carregado (`item.nome`) |
| `Acquire` | `resource`, `count` (`1`) | espera `count` unidades do recurso ficarem livres e as segura. Quem espera forma fila FIFO |
| `Release` | `resource`, `count` (`1`) | devolve unidades do recurso |
| `WaitFor` | `block`, `event` | espera o próximo evento no objeto; o token passa a carregar esse item |
| `CreateItem` | `into`, `port` (`0`), `labels` | cria um item dentro de um objeto, que precisa aceitá-lo |
| `Batch` | `size` | junta `size` tokens e segue com um |
| `Sink` | nenhum | encerra o token (não tem `next`) |

As regras de `Decide`:

- `Probability([p1, p2, ...])`: ramo sorteado com pesos relativos, um por ramo;
- `ByLabel("rótulo")`: o número do ramo (começando em 1) é o valor do rótulo do token;
- `Condition(["expr1", "expr2", ...])`: uma condição por ramo; vale a primeira verdadeira, e se nenhuma for, o último ramo;
- `Agent(observation: [...])`: um agente externo escolhe. Veja [Aprendizado por reforço](aprendizado-por-reforco.md).

### O que o fluxo registra

Além dos estados, cada fluxo grava no relatório:

- `<atividade>.espera`: para `Delay`, `Acquire`, `WaitFor` e `Batch`, o tempo que cada token passou ali;
- `<recurso>.ocupado`: o tempo em que cada parcela do recurso foi mantida;
- `<sink>.tempo_no_fluxo`: o tempo de vida do token até terminar naquele `Sink`.

### Como o tempo passa

As atividades `Assign`, `Decide`, `Release`, `CreateItem` e um `Acquire` com recurso livre são instantâneas. O token só para em `Delay`, `Acquire` (quando o recurso está ocupado), `WaitFor` e `Batch`. Um token que passa de 10.000 atividades sem nunca parar indica um laço de tempo zero, e a simulação termina com erro.

### Editar no visualizador

O botão **Fluxos** da barra superior abre o painel da lógica do usuário, onde se editam recursos, atividades e ligações.

## Scripts

O UTSim usa a linguagem [Rhai](https://rhai.rs), parecida com Rust e JavaScript. Os scripts aparecem em poucos pontos, e todos rodam num ambiente fechado: sem arquivos, sem relógio, sem rede, com limite de operações.

### Onde usar

| Onde | Campo | Para que serve |
|---|---|---|
| Quando um item entra num objeto | `on_entry: "..."` | atribuir rótulos, por exemplo sortear prioridade |
| Escolha da porta de saída | `output: Script("...")` | devolver o número da porta (começando em 1) |
| Atividades de fluxo | `Assign`, `Decide` com `Condition`, `CreateItem`, observação do agente | calcular valores e condições |

Exemplo (`examples/roteamento_por_script.ron`):

```ron
(name: "triagem", kind: Queue(),
    on_entry: "item.prioridade = if random() < 0.2 { 1 } else { 2 };",
    output: Script("if item.prioridade == 1 { 1 } else { 2 }"))
```

Cerca de 20% dos itens recebem prioridade 1 ao entrar e saem pela porta 1; os demais saem pela porta 2.

### O que o script enxerga

| Contexto | Variáveis |
|---|---|
| expressões de fluxo | `now` (instante atual), `token` (rótulos do token) e `item` (rótulos do item, ou `()` se o token não carrega nenhum) |
| `on_entry` | `item` (pode ser alterado e é gravado de volta), `now`, `block` (nome do objeto) |
| regra de saída `Script` | `item` (somente leitura), `outputs` (número de portas), `now`, `block`. O resultado é a porta, começando em 1 |

### Funções disponíveis

| Função | Resultado |
|---|---|
| `random()` | número uniforme entre 0 e 1 |
| `uniform(a, b)` | uniforme entre `a` e `b` |
| `exponential(media)` | exponencial com a média dada |
| `normal(media, dp)` | normal |
| `triangular(min, moda, max)` | triangular |
| `label(mapa, "nome", padrao)` | o rótulo `nome` de `item` ou `token`, ou o valor padrão se não existir |

Todos os sorteios usam o gerador do próprio objeto, então a simulação continua reprodutível.

### Dicas

- O RON exige aspas em volta do script. Se o script tem aspas, use `\"`: por exemplo, `labels: {"origem": "\"caminhao\""}` atribui o texto `caminhao`.
- Erros de sintaxe aparecem no `utsim check`. Erros durante a simulação (por exemplo, uma porta que não existe) param a rodada com uma mensagem.
- Prefira os objetos prontos e regras de saída simples. Use script só quando a regra não couber nelas.
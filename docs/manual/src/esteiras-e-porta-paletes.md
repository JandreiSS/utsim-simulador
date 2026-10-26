# Esteiras e porta-paletes

## Conveyor (Esteira)

A esteira é um transportador contínuo e fixo. Ela leva os itens de uma ponta à outra em tempo `comprimento / velocidade`.

```ron
(name: "esteira1", kind: Conveyor(length: 10.0, speed: 0.5, item_length: 1.0, accumulating: true),
    position: (7.0, 0.0, 0.0))
```

| Campo | Padrão | Significado |
|---|---|---|
| `length` | obrigatório | comprimento da esteira |
| `speed` | obrigatório | velocidade da correia |
| `item_length` | obrigatório | espaço que cada item ocupa na correia |
| `accumulating` | `true` | `true`: os itens se acumulam no fim quando a saída bloqueia. `false`: a correia inteira para |

Os três primeiros precisam ser maiores que 0, e `item_length` não pode passar de `length`.

### Como a esteira se comporta

- **Capacidade:** `C = ⌊length / item_length⌋` itens. A esteira de 10 m com 1 m por item guarda 10.
- **Tempo de trânsito:** `length / speed`. No exemplo, 10 / 0,5 = 20 unidades de tempo.
- **Vazão máxima:** `speed / item_length` itens por unidade de tempo. No exemplo, 0,5 item por unidade.
- **Entrada:** um item só entra se há espaço e se o último que entrou já andou pelo menos `item_length`.
- **Com acúmulo:** se a saída está bloqueada, o item da frente espera no fim e os de trás param encostados nele. Quando o da frente sai, o próximo percorre só o espaço que falta.
- **Sem acúmulo:** enquanto o item da frente espera no fim, a correia inteira para. Todos os itens atrasam exatamente o tempo parado.
- **Estados:** `empty`, `releasing` (movendo) e `blocked` (item da frente esperando).
- **Quebras e turnos:** a correia para e retoma do ponto onde estava.

**Use** a esteira quando o transporte entre dois postos é contínuo e o tempo de trânsito importa. É a forma de dar tempo de percurso a uma ligação direta. Para junções e desvios, use as filas e as regras de saída que já existem. O modelo `examples/linha_com_esteiras.ron` mostra uma esteira com acúmulo e outra sem.

## Rack (Porta-paletes)

Armazenagem endereçada: `bays` colunas por `levels` níveis.

```ron
(name: "porta_paletes", kind: Rack(
    bays: 6,
    levels: 4,
    dwell: Uniform(min: 20.0, max: 60.0),
    placement: FirstFree,
    bay_width: 1.2,
    level_height: 1.5,
), position: (10.5, 0.0, 0.0))
```

| Campo | Padrão | Significado |
|---|---|---|
| `bays` | obrigatório | número de colunas |
| `levels` | obrigatório | número de níveis |
| `dwell` | nenhum | tempo mínimo que cada item fica guardado |
| `placement` | `FirstFree` | como escolher a posição livre |
| `bay_width` | `1.0` | espaçamento entre colunas no 3D |
| `level_height` | `1.0` | espaçamento entre níveis no 3D |

A capacidade é `bays x levels`. O produto não pode passar de 1.000.000 e `bay_width` e `level_height` precisam ser maiores que 0.

- **Posição:** `FirstFree` preenche o nível mais baixo primeiro e, em cada nível, a coluna mais baixa. `Random` sorteia entre as posições livres.
- **Saída:** o item fica pelo menos `dwell` e depois é liberado, na ordem em que ficou pronto. A posição é liberada quando o item sai.
- **Estados:** `empty` e `releasing`.
- **Quebras e turnos:** pausam os tempos de permanência e os retomam com o tempo que faltava.
- **3D:** cada item é desenhado na posição certa da estrutura.

Quando o porta-paletes está cheio, ele bloqueia a entrada e o objeto anterior (uma fila, por exemplo) começa a acumular. Isso aparece no conteúdo da fila e no estado `blocked`.

### Dimensionando um porta-paletes

Use a fórmula de Little, `L = λ x W`. Se chega um palete a cada 90 s (λ = 1/90) e cada um fica 1200 s em média, o porta-paletes guarda em média 1200 / 90 ≈ 13 paletes. Com 18 posições, a ocupação média é de 74%. Os picos são o que enche o porta-paletes, e é por isso que simular vale mais que a conta de média. O [tutorial 04](tutoriais/04-armazem.md) faz exatamente isso.

### Paletes carregados por empilhadeiras

Um Rack costuma ser alimentado por empilhadeiras: o objeto anterior tem `transport` apontando para o despachante. O Rack também pode ter `transport`, para a retirada. Veja [Recursos móveis](recursos-moveis.md).
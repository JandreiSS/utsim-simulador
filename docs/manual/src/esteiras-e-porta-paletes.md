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

### Esteira desenhada por nós (`route`)

A esteira pode seguir um trajeto desenhado por nós, como se desenham ruas: retas, curvas e subidas. Com `route`, o trajeto manda no resultado e o campo `length` deixa de valer.

```ron
(name: "esteira_sobe", kind: Conveyor(length: 5.0, speed: 0.5, item_length: 0.6,
    route: Some((
        nodes: [
            (at: (0.0, 0.0, 0.8)),
            (at: (6.0, 0.0, 0.8), radius: Some(1.5)),
            (at: (6.0, 5.0, 2.2)),
        ],
        style: Channeled,
        drive: Powered,
    ))),
    position: (10.0, 4.0, 0.0))
```

| Campo de `route` | Padrão | Significado |
|---|---|---|
| `nodes` | obrigatório | nós do trajeto, ao menos 2. `at` é a posição relativa à da esteira, e `z` é a altura da superfície da correia. `radius` arredonda a quina num nó do meio |
| `style` | `Simple` | `Simple` (esteira simples) ou `Channeled` (esteira com canaletas laterais) |
| `drive` | `Powered` | `Powered` (motorizada) ou `Gravity` (roletes livres por gravidade) |
| `max_slope` | do tipo | inclinação máxima em graus |
| `width` | `0.6` | largura da esteira em metros |

Na animação, a correia anda só quando a esteira anda de verdade: para com uma quebra ou fora do turno e, sem acúmulo, enquanto a peça da frente espera na saída. Os tambores das pontas giram com ela, e os roletes da esteira por gravidade giram enquanto há peças rolando sobre eles.

O que a geometria muda no resultado:

- **Comprimento:** é o do trajeto em 3D, com as curvas e as subidas. Subir 1 m em 4 m de planta dá √(4² + 1²) ≈ 4,12 m.
- **Tempo de passagem:** comprimento ÷ velocidade.
- **Capacidade:** comprimento ÷ `item_length`.
- **Curva:** o arco é tangente aos dois trechos e fica no plano do piso. Se o raio não cabe, ele diminui até o ponto de tangência chegar à metade do trecho mais curto. O arco de raio `r` que vira θ encurta o caminho em `2r·tan(θ/2) − r·θ`.
- **Pontas:** a entrada é o primeiro nó e a saída é o último, cada um na sua altura. O operador pega e deixa a peça ao lado delas.
- **Área no piso:** é a faixa da largura da esteira em volta do trajeto. Quem anda contorna a esteira por fora. Onde a parte de baixo da estrutura (0,14 m abaixo da superfície) fica a 2,2 m do piso ou mais, dá para passar por baixo: ali só as pernas, sob os nós, são obstáculo.

#### Inclinação máxima

Acima do limite aparece um aviso, porque a peça escorrega ou tomba.

| Tipo | Limite padrão |
|---|---|
| Esteira simples (correia lisa) | 15° |
| Esteira com canaletas (guias laterais altas) | 25° |

#### Esteira por gravidade

Nos roletes livres, a peça desce pela inclinação, e os roletes de freio a seguram na velocidade-limite `speed`. Cada pedaço do trajeto tem a sua velocidade, em função da descida `g` (subida ÷ distância em planta, 0,03 = 3 %):

`v = speed × limitar((g − 0,015) ÷ (0,05 − 0,015), 0,1, 1)`

- Abaixo de 1,5 % de descida, a resistência ao rolamento vence e a peça só anda empurrada pelas de trás, a 10 % da velocidade-limite. Um trecho assim gera um aviso.
- Com 5 % ou mais de descida, a peça chega à velocidade-limite.
- A esteira inteira usa a velocidade que dá o mesmo tempo de passagem do trajeto: `L ÷ Σ (Lᵢ ÷ vᵢ)`.

#### Desenhar no UTSim 3D

No Editar, o grupo Mover da Biblioteca tem três ferramentas: "Esteira desenhada", "Esteira com canaletas" e "Caminho no piso" (um corredor de pedestres com rampas e curvas).

- **Desenhar:** cada clique no chão põe um nó. O nó encaixa numa porta (a ponta de uma esteira, na altura dela, ou o centro de um objeto), num nó de outro trajeto, na linha de um caminho, na direção do trecho anterior (até 6° dela, para seguir reto sem quebrar a linha, também ao continuar do fim de outra esteira), de 15 em 15° a partir do nó anterior ou na grade. Enquanto se desenha, o trecho mostra o comprimento, a inclinação e o total, em vermelho quando passa do limite.
- **Altura e curva:** PageUp e PageDown sobem e descem o próximo nó 10 cm (com Shift, 1 cm). A esteira começa a 0,8 m, e o caminho no piso. `[` e `]` mudam o raio da próxima curva.
- **Inclinação fixa:** no painel, ligue "Inclinação fixa no próximo trecho" e dê a inclinação em % (negativa desce). A altura de cada nó novo sai da distância em planta desde o anterior; com o comprimento digitado, o trecho tem exatamente esse comprimento nessa inclinação. Um clique numa porta, num nó ou num caminho usa a altura de lá.
- **Junção e desvio em T:** um nó na linha de outra esteira desenhada (longe das pontas) é uma junção em T. Se a esteira nova **acaba** ali, a outra é dividida nesse ponto e a nova entra na segunda metade (junção). Se **começa** ali, a primeira metade da outra também sai para a nova (desvio; a regra de saída dela escolhe o caminho). Um Ctrl+Z desfaz a esteira e a divisão juntas.
- **Comprimento exato:** com o cursor apontando a direção, digite o comprimento do trecho (por exemplo `3,5`) e tecle Enter: o nó vai a exatamente essa distância (em 3D, com a altura do próximo nó). Backspace apaga o que foi digitado.
- **Terminar:** Enter ou duplo clique. Backspace volta um nó, e Esc cancela. A esteira se liga ao objeto ou à esteira onde começa e onde acaba.
- **Continuar:** um clique no primeiro ou no último nó de uma esteira começa outra ali, como uma rua que segue.
- **Editar:** clique num nó para selecioná-lo (Shift soma nós do mesmo trajeto). Arraste o losango no chão para mover em X e Y (com Ctrl, só no eixo X ou só no Y, o de maior deslocamento), e o topo da haste para mudar a altura. As setas movem um passo da grade, e PageUp/PageDown sobem e descem (Shift: 1 cm). No painel ficam X, Y, altura e raio exatos, nivelar, inclinação constante entre as pontas, dividir a esteira num nó e remover nós. Shift + clique na linha acrescenta um nó.
- **Perfil:** com um trajeto selecionado, um painel embaixo da vista mostra o corte dele: altura × distância em planta, a altura de cada nó e a inclinação de cada trecho, em vermelho acima do limite. Arraste um nó no perfil para cima ou para baixo para mudar só a altura dele (de 5 em 5 cm; com Shift, de 1 em 1 cm).
- **Curvar um trecho:** com o trajeto selecionado, cada trecho mostra um losango laranja no meio. Arraste-o: nasce ali um nó com o maior raio que cabe, e o trecho vira uma curva suave.
- **Paralelo:** no painel, "Paralelo a (m)" e "Desenhar paralelo" criam outro trajeto ao lado, à esquerda do sentido (negativo: à direita), com as mesmas alturas e as curvas concêntricas (o raio de dentro diminui, o de fora aumenta). A esteira paralela é do mesmo tipo e não se liga a nada.

Tudo se desfaz com Ctrl+Z.

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
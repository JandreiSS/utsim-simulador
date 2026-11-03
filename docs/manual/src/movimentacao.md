# Movimentação e obstáculos

Pessoas, empilhadeiras e outros veículos (os [recursos móveis](recursos-moveis.md)) não passam
por dentro das máquinas nem das paredes: cada um anda pelo **menor caminho livre** no piso e
para no **ponto de acesso** do objeto que vai atender (a empilhadeira na frente do porta-paletes,
a pessoa ao lado da máquina). Este capítulo explica os modos de movimentação, os obstáculos, a
folga de cada recurso, a rede gerada e os avisos.

## Modos de movimentação

| Modo | Como anda | Precisa de |
|---|---|---|
| `Free` (livre) | menor caminho no piso desviando dos objetos, das paredes e dos obstáculos, até o ponto de acesso | nada |
| `Network` (pela rede) | pela rede de caminhos; do ponto onde está até o nó mais próximo e do último nó até o ponto de acesso, pelo caminho livre | `network` |
| `Legacy` (legado) | como no formato 1: linha reta sem rede; com rede, entra e sai pelo nó mais próximo em linha reta; vai até o centro do objeto | nada |
| `Aisles` (corredores) | só pelos corredores da classe do recurso (pessoas nos de pedestres e mistos, veículos nos de empilhadeiras e mistos), com prioridade ao pedestre nos cruzamentos; ver [Corredores e faixas de segurança](corredores.md) | `aisles` |

O modo vale para o modelo inteiro em `run` e pode ser trocado por recurso:

```ron
run: (stop_time: 28800.0, movement: Free),
objects: [
    (name: "ana", kind: Transporter(speed: 1.2), position: (2.0, -4.0, 0.0),
     visual: (asset: "operador")),
    (name: "empilhadeira_1", kind: Transporter(speed: 2.0, movement: Network, clearance: 1.0),
     position: (20.0, 10.0, 0.0)),
],
```

**Modelos novos nascem em `Free`.** No formato 2, `movement` ausente vale `Free`. **Modelos
antigos** (formato 1) são abertos com `movement: Legacy` e se comportam exatamente como antes;
ao salvar, o editor grava `format: 2` e `movement: Legacy`. Num modelo sem recursos móveis o
modo não significa nada e não é gravado. Para que um modelo antigo passe a desviar, troque
`Legacy` por `Free` (os tempos de viagem mudam: confira os resultados).

Um AGV (`agv: true`) num modelo com rede segue a rede aresta por aresta com o controle de
tráfego, em qualquer modo.

O `utsim check` mostra o modo de cada recurso:

```text
movimentação: ana: livre (menor caminho no piso, desviando dos obstáculos) (pessoa, folga 0,60 m)
movimentação: empilhadeira_1: pela rede de caminhos (ligação livre até o ponto de acesso) (empilhadeira, folga 1,00 m)
```

## Obstáculos

São obstáculos:

- a **área ocupada de cada objeto fixo**, um retângulo do tamanho que o objeto tem no 3D:
  processador e saída 1,8 × 1,8 m, chegada 1,6 × 1,6 m, juntar e separar 2 × 2 m, fila 4 × 1,4 m,
  esteira com o seu comprimento por 1,1 m (girada para o objeto da sua saída) e porta-paletes com
  os vãos (`bays × bay_width`) por 1 m de profundidade. `visual.scale` aumenta ou diminui a área e
  `visual.yaw` a gira. Recursos móveis e despachantes não são obstáculos;
- os **`obstacles`** do modelo: polígonos de paredes e colunas da [planta CAD](planta-cad.md) ou
  desenhados à mão.

```ron
obstacles: [
    (name: "pilar_3", polygon: [(4.0, -4.0), (6.0, -4.0), (6.0, -2.0), (4.0, -2.0)]),
    (name: "parede_norte", polygon: [(0.0, 20.0), (40.0, 20.0), (40.0, 20.15), (0.0, 20.15)]),
],
```

Para trazer as paredes e colunas de uma planta DXF: `utsim planta galpao.dxf --obstaculos
modelo.ron` (veja [Planta CAD](planta-cad.md)). Os obstáculos da planta se chamam `planta_1`,
`planta_2`... e são trocados a cada importação; os seus continuam.

## Folga de cada recurso

O recurso fica sempre a pelo menos a sua **folga** (`clearance`, em metros) de qualquer
obstáculo. Sem `clearance`, ela é a meia largura do recurso mais uma margem de 0,3 m:

| Papel | Meia largura | Folga padrão |
|---|---|---|
| pessoa | 0,3 m | 0,6 m |
| empilhadeira | 0,6 m | 0,9 m |
| AGV ou outro veículo | 0,5 m | 0,8 m |

O papel vem do ativo (`visual.asset` com "empilhadeira", "agv", "operador" ou "pessoa"), de
`agv: true`, de atender um porta-paletes (empilhadeira) ou de ser operador de uma máquina
(pessoa), como no 3D. Uma passagem só abre para o recurso quando é mais larga que o dobro da
folga: um corredor de 1,5 m deixa passar uma pessoa (1,2 m) e fecha para a empilhadeira (1,8 m).

## Pontos de acesso

O recurso não vai ao centro do objeto: vai ao **ponto de acesso**, que por padrão fica no lado
`-y` do objeto (a frente do porta-paletes; o lado da máquina onde a pessoa trabalha), no meio
dele, à distância da folga mais 5 cm da borda. Com o objeto girado (`visual.yaw`), o ponto gira
junto. Para escolher outro lado, use `access` com o ponto em coordenadas do objeto (metros):

```ron
(name: "torno", kind: Processor(process: Exp(mean: 60.0)), position: (10.0, 0.0, 0.0),
 access: (1.5, 0.0)),  // ao lado direito da máquina
```

## Distância e tempo

O caminho é calculado numa grade de 25 cm com os obstáculos inflados pela folga (A\* com
suavização: o caminho final só tem os cantos necessários). Ele nunca é mais curto que o menor
caminho exato e fica, em geral, a menos de 25 cm dele em cada contorno. O tempo da viagem é o
de sempre: o comprimento do caminho com o perfil de velocidade do recurso (veja
[Recursos móveis](recursos-moveis.md)).

Tudo é determinístico: o mesmo modelo dá sempre os mesmos caminhos e tempos, em qualquer ordem
dos obstáculos, com as replicações em paralelo ou uma por vez. Cada par origem-destino é
calculado uma vez e reaproveitado.

## Gerar a rede pelo caminho livre

```text
utsim rede gerar modelo.ron [--usar] [--folga 0.9] [--saida outro.ron] [--substituir]
```

O comando calcula os caminhos livres entre as posições dos recursos e os pontos de acesso dos
objetos que eles atendem e grava o resultado como `network` comum no `.ron`: nós `g1`, `g2`... e
arestas de mão dupla, que você pode revisar e editar à mão. A folga é a maior entre os recursos
(a rede serve a todos), ou a dada em `--folga`. Com `--usar`, o modelo passa para
`movement: Network`. Uma rede desenhada à mão só é trocada com `--substituir`.

A rede gerada guarda de onde veio (`generated: (fingerprint: "fnv1a64:...", clearance: 0.9)`).
Se depois você mover um objeto, mudar a planta ou a folga, o `utsim check` avisa que a rede está
velha.

## Avisos

Para recursos em `Free` ou `Network`, o `utsim check` e o `utsim run` avisam quando:

- o ponto de acesso de um objeto atendido fica dentro de um obstáculo ou a menos da folga dele;
- um recurso não alcança o ponto de acesso de um objeto, ou não tem caminho livre até nenhum
  dos objetos que atende (está cercado ou as passagens são estreitas demais); essas viagens saem
  em linha reta;
- um trecho da rede cruza a área de um objeto ou de uma parede (com recurso em `Network` ou com
  rede gerada);
- a rede foi gerada com outros obstáculos.

## Ainda não disponível

- conflitos entre recursos no modo livre: duas pessoas passam uma pela outra no tempo da
  simulação (as reservas de cruzamento existem só no modo `Aisles`);
- o desvio nas ligações diretas com `transfer` (que continuam em linha reta ou pela rede).

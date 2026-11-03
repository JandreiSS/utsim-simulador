# Corredores e faixas de segurança

Numa fábrica as pessoas andam pelos corredores de pedestres e as empilhadeiras pelos delas, com
faixas pintadas no piso e travessias zebradas onde os dois se cruzam. No UTSim você desenha esses
corredores no modelo (`aisles`); a rede de caminhos é gerada a partir das linhas centrais, cada
recurso em `movement: Aisles` só anda pelos corredores da sua classe, e nos cruzamentos o veículo
para e cede a passagem ao pedestre. Este capítulo explica o formato, a rede gerada, a regra dos
cruzamentos, os perfis de marcação, os avisos e o comando `utsim corredores`.

> **O programa não certifica conformidade.** Os perfis ajudam a seguir as normas e a prática
> usual, mas a responsabilidade é do projeto da fábrica. Confira sempre o texto vigente das
> normas (as Normas Regulamentadoras são revisadas com frequência e a numeração dos itens muda).

## Um exemplo

```ron
Model(
    format: 2,
    name: "cruzamento",
    run: (stop_time: 28800.0, movement: Aisles),
    objects: [
        (name: "doca", kind: Queue(), transport: "empilhadeira_1",
         position: (-10.0, 3.0, 0.0), access: (0.0, -3.0)),
        (name: "estoque", kind: Sink, position: (10.0, 3.0, 0.0), access: (0.0, -3.0)),
        (name: "empilhadeira_1", kind: Transporter(speed: 2.0), position: (-10.0, 0.0, 0.0),
         visual: (asset: "empilhadeira")),
        // ...
    ],
    aisles: [
        (name: "corredor_A", kind: Forklift, points: [(-10.0, 0.0), (10.0, 0.0)], width: 3.6),
        (name: "passeio_1", kind: Pedestrian, points: [(0.0, -10.0), (0.0, 10.0)], width: 1.2),
    ],
    floor_marking: (profile: Brasil, line_width: 0.10, crossing_stop: 2.0),
)
```

## Corredores (`aisles`)

| Campo | Padrão | O que é |
|---|---|---|
| `name` | (obrigatório) | nome único do corredor |
| `kind` | (obrigatório) | `Pedestrian` (só pessoas), `Forklift` (empilhadeiras, AGVs e outros veículos) ou `Mixed` (os dois) |
| `points` | (obrigatório) | linha central, ao menos 2 pontos `(x, y)` em metros |
| `width` | (obrigatório) | largura total, de borda a borda (m) |
| `two_way` | `true` | mão dupla; com `false`, só no sentido dos pontos (setas no piso) |
| `secondary` | `false` | passagem secundária de pedestres (mínimo menor, ver abaixo) |
| `profile` | o do modelo | perfil de marcação só deste corredor |

**Quem usa o quê.** Pessoas andam nos corredores de pedestres e nos mistos; veículos, nos de
empilhadeiras e nos mistos. A classe vem do papel do recurso, o mesmo do desenho 3D e da folga
(capítulo [Movimentação e obstáculos](movimentacao.md)): `visual.asset` com "operador" ou
"pessoa", ou um recurso que opera máquinas (`operators`), é pessoa; empilhadeira, AGV e os
outros são veículos.

## Marcação do piso (`floor_marking`)

| Campo | Padrão | O que é |
|---|---|---|
| `profile` | `Brasil` | `Brasil`, `Branco` ou `Custom((...))` |
| `line_width` | `0.10` | largura da faixa de delimitação (m); aviso fora de 0,05 a 0,15 |
| `crossing_stop` | `2.0` | parada obrigatória do veículo antes de um cruzamento com pedestres (s) |

**Perfis prontos.**

| Elemento | Brasil (padrão) | Branco |
|---|---|---|
| Delimitação dos corredores | faixa amarela contínua | faixa branca contínua |
| Sentido de mão única | setas amarelas a cada 10 m | setas brancas a cada 10 m |
| Travessia de pedestres | zebrado amarelo e preto, listras de 0,40 m | igual |
| Corredor de pedestres | 1,20 m (via principal); 0,90 m em passagem secundária | igual |
| Corredor de veículo, mão única | `L + 1,0 m` | igual |
| Corredor de veículo, mão dupla | `2L + 1,2 m` | igual |
| Corredor misto | exigência do veículo + a do pedestre | igual |

`L` é a maior largura entre o veículo e a carga: pela largura do papel (empilhadeira 1,20 m, AGV
e outro veículo 1,00 m) ou, se maior, pela folga dada no recurso (`2 × (clearance − 0,3)`). Com
`L = 1,2 m`, um corredor de empilhadeira de mão dupla pede `2 × 1,2 + 1,2 = 3,6 m`.

**As larguras dos corredores de veículo são prática de mercado, não norma.** O perfil Branco
segue a indicação de branco para corredores da redação original da NR-26; as fontes divergem
entre branco e amarelo, por isso os dois perfis existem.

`Custom` aceita qualquer um destes campos (os ausentes valem os do Brasil): `line_color`,
`arrow_color`, `zebra_colors` (duas cores), `stripe`, `arrow_spacing`, `pedestrian_min`,
`pedestrian_secondary_min`, `one_way_margin`, `two_way_margin`. Cores em `#RRGGBB`:

```ron
floor_marking: (profile: Custom((line_color: "#FFFFFF", two_way_margin: 1.5))),
```

## A rede gerada pelos corredores

Não é preciso desenhar a rede: ela sai das linhas centrais, sempre igual para os mesmos dados.

1. Cada trecho da linha central é cortado nas pontas, nos cruzamentos com outros corredores e
   onde a ponta de outro corredor encosta nele (junção em T, tolerância de 1 cm). Os nós
   recebem os nomes `n1`, `n2`... na ordem dos corredores e dos pontos.
2. Cada pedaço vira um trecho da rede, com o sentido do corredor. O pedaço em que a linha
   central passa por dentro de um obstáculo (parede, coluna, área de um objeto) fica fora da
   rede, com aviso, até ser corrigido.
3. Onde se encontram corredores de tipos diferentes nasce um **cruzamento** (por exemplo
   `corredor_A x passeio_1`).
4. A posição de cada recurso em `Aisles` e os pontos de acesso dos objetos que ele atende são
   ligados ao ponto mais próximo de um corredor da sua classe, por uma ligação reta que não
   cruza obstáculo (se nenhuma serve, vale a mais curta, com aviso). A ligação de uma pessoa não
   serve a veículos e vice-versa.

Cada recurso usa a sub-rede da sua classe: o menor caminho é calculado uma vez, por Dijkstra, e
a distância e o tempo são determinísticos. O tempo segue o perfil de velocidade do recurso
(velocidade, aceleração e desaceleração) sobre o comprimento do caminho. A rede dos corredores
não é gravada no `.ron`: ela é função dos corredores e dos objetos e é refeita ao abrir.

## Cruzamentos: prioridade ao pedestre

Em cada cruzamento há uma zona de conflito com a metade da maior largura dos corredores de cada
lado do nó. A regra, sem física e sempre na ordem dos eventos:

- o **veículo** para `crossing_stop` segundos (2 s no padrão) na borda da zona, mesmo sem
  ninguém; depois só entra se nenhum pedestre estiver na zona durante a sua travessia; se
  houver, espera parado (estado `Blocked`) até o último pedestre sair e confere de novo;
- o **pedestre** que chega à zona entra logo; só espera se um veículo já estiver dentro dela, e
  então entra quando ele sai.

Exemplo conferido nos testes (1 m/s, sem aceleração): a empilhadeira vai de (-10, 0) a (10, 0)
e a pessoa de (0, -10) a (0, 10), as duas saindo em t = 0, com a zona de 3,6 m em volta de
(0, 0). A pessoa passa direto (20 s, dentro da zona de 8,2 s a 11,8 s). A empilhadeira chega à
zona em 8,2 s, para até 10,2 s, espera a pessoa sair em 11,8 s, atravessa (3,6 s) e anda os
últimos 8,2 m: chega em 23,6 s. Sem a pessoa, chegaria em 22 s (20 m mais a parada).

O veículo para e arranca a cada cruzamento (o perfil de velocidade recomeça); a pessoa mantém o
perfil da viagem inteira.

## Avisos

`utsim check`, `utsim run` e `utsim corredores` avisam quando:

- um corredor é mais estreito que o mínimo do perfil para o tipo e o sentido, com o recurso
  mais largo que o usa (para veículo, a regra `L + 1,0 m` ou `2L + 1,2 m` e o `L` usado);
- a linha central de um corredor cruza um obstáculo (o trecho fica fora da rede) ou a faixa do
  corredor invade um obstáculo;
- o ponto de acesso de um objeto não tem corredor da classe do recurso que o alcance, ou a
  ligação ao corredor mais próximo cruza um obstáculo, ou o recurso não chega até ele pelos
  corredores da sua classe;
- um recurso não tem nenhum corredor da sua classe (em `Aisles`, as viagens saem em linha reta);
- a faixa de delimitação está fora de 0,05 a 0,15 m.

`movement: Aisles` sem `aisles` no modelo é erro de validação.

## O comando `utsim corredores`

```text
utsim corredores modelo.ron [--svg planta.svg]
```

Mostra o perfil, cada corredor (tipo, largura, sentido, comprimento), a rede gerada (nós,
trechos, ligações, cruzamentos), o modo de cada recurso e os avisos. Com `--svg`, grava a
planta com os objetos, os obstáculos, os corredores com as faixas nas cores do perfil, as
zebras, as setas, a rede, as ligações (tracejadas) e os cruzamentos (círculos vermelhos).

## Na interface 3D

No modo Editar da interface 3D os corredores se desenham com o mouse, e as faixas aparecem
pintadas no piso (no Editar e no Simular).

**As faixas no piso.** A cena desenha os polígonos de `Model::floor_marks()` como malhas finas
sobre o chão: a faixa de delimitação contínua de cada corredor (amarela no perfil Brasil, branca
no perfil Branco), as listras pretas e amarelas da travessia zebrada onde um corredor de
empilhadeiras cruza um de pedestres e as setas dos corredores de mão única. As cores e as larguras
vêm do perfil, como no `utsim corredores --svg`. As faixas se atualizam a cada edição.

**A ferramenta Corredor** (botão **Corredor** na barra ou a tecla `C`):

1. Clique no chão para pôr os pontos da linha central, um clique por ponto. Cada ponto se encaixa
   no vértice ou na linha de outro corredor, quando estiver a menos de 45 cm, e senão na grade.
2. **Enter**, ou um clique duplo no último ponto, termina o corredor. **Backspace** volta um
   ponto e **Esc** cancela o que está sendo desenhado.
3. O painel da direita mostra o tipo (pedestres, empilhadeiras ou misto), a largura, a mão dupla
   ou única, a passagem secundária de pedestres, o perfil só deste corredor (padrão do modelo,
   Brasil ou Branco) e o nome. Os novos corredores nascem com a largura mínima do perfil para o
   tipo (1,20 m, 3,60 m e 4,80 m). Os avisos do corredor, como a largura abaixo do mínimo, aparecem
   no próprio painel.
4. Clique numa linha para selecionar o corredor. Arraste um vértice para movê-lo; **Shift** mais
   clique na linha do corredor selecionado acrescenta um vértice; **Del** exclui o corredor.
   Inverter o sentido de um corredor de mão única também tem botão.

Cada mudança é um comando do documento (`AddAisle`, `SetAisle`, `RemoveAisle`) e se desfaz com
**Ctrl+Z**.

**Mostrar obstáculos e folgas.** A caixa da lista de objetos desenha a área ocupada de cada objeto
(vermelho), as paredes e colunas de `obstacles` (roxo) e, em volta de cada uma, o contorno da folga
que o recurso mantém (laranja). Cada objeto mostra o seu **ponto de acesso**, um losango ligado ao
centro dele: ciano quando é o padrão e rosa quando o modelo tem o `access`. Com a ferramenta
Selecionar, arraste o losango para outro lugar: o ponto é gravado em `access`, em coordenadas do
objeto e na grade de 5 cm, e se desfaz. Para voltar ao padrão, desligue "Ponto de acesso próprio" no
painel do objeto. Um recurso parado sobre o ponto (uma pessoa na máquina) é selecionado com um
clique; um arrasto a partir dali move o ponto.

**Modo de movimentação.** No painel do recurso (transportador) a seção **Movimentação** escolhe o
modo (padrão do modelo, legado, livre, pela rede ou pelos corredores) e a folga própria. No painel
do modelo (nada selecionado), **Movimentação padrão** é o `run.movement`, e **Piso e corredores**
tem o perfil da marcação, a largura da faixa e a parada no cruzamento. Detalhes em
[Movimentação e obstáculos](movimentacao.md).

**Avisos.** Os avisos dos corredores e da movimentação aparecem na lista de Problemas e Avisos do
painel (do modelo e do objeto). Clique num aviso: a cena destaca em laranja o corredor e o objeto
que ele cita.

**Simular.** As pessoas e as empilhadeiras andam pelo caminho calculado: o rastro da simulação
leva o caminho de cada viagem, pelos corredores ou livre, e a cena o reproduz como reproduz a
rede de caminhos. O modelo de exemplo do roteiro de teste está em
`godot/roteiros/g4_cruzamento.ron`.

## Para quem programa a interface

`Model::floor_marks()` devolve as faixas como polígonos convexos com cor e padrão
(`Continuous`, `Zebra`, `Arrow`): um quadrilátero por trecho de cada faixa de delimitação (por
dentro da largura, com junção em esquadria limitada), as listras da zebra sobre o corredor de
veículos, alinhadas ao de pedestres, e um triângulo por seta. `Model::aisle_net()` traz a rede,
as ligações e os cruzamentos.

## Normas de referência

As normas abaixo são citadas como referência; os números e itens devem ser conferidos no texto
oficial antes de qualquer uso em projeto.

- **NR-26 (Sinalização e Identificação de Segurança)** [conferir no texto vigente]: exige a
  sinalização de segurança nos locais de trabalho e remete às normas técnicas oficiais para as
  cores. A redação original indicava o branco para corredores de circulação e o amarelo para
  "cuidado".
- **NR-12 (Segurança no Trabalho em Máquinas e Equipamentos)** [conferir no texto vigente]: pede
  áreas de circulação demarcadas e desobstruídas e uma largura mínima para as vias principais; o
  valor de 1,20 m usado no perfil vem das fontes consultadas e não foi confirmado no texto
  oficial.
- **NR-11 (Transporte, Movimentação, Armazenagem e Manuseio de Materiais)** [conferir no texto
  vigente]: trata dos equipamentos de transporte motorizados e da armazenagem; não fixa a
  largura do corredor de empilhadeira (ela vem do equipamento e da prática).
- **ABNT NBR 7195 (Cores para segurança)** [conferir no texto vigente]: define as cores de
  segurança e os seus usos (amarelo para "cuidado", listras amarelas e pretas para riscos de
  colisão e tropeço, entre outras); é a referência das cores dos perfis.

Data da conferência das fontes: 2026-11-02 (fontes secundárias; o texto oficial ainda precisa
ser lido).

## Ainda não disponível

- áreas marcadas no piso (armazenagem, área a manter livre, equipamento de incêndio) e as regras
  `VehiclePriority`, `Fifo` e `StopBoth`;
- desenho e edição das paredes e colunas (`obstacles`) na interface 3D: elas só são mostradas
  (a planta CAD e o `.ron` as trazem);
- limite de velocidade por corredor, limite de espera do veículo no cruzamento e a métrica de
  espera por cruzamento nos resultados (o núcleo já conta as esperas por cruzamento);
- pedestres em modo livre atravessando os corredores de veículos só pelas zebras;
- reserva entre dois veículos no mesmo cruzamento (passam um pelo outro, como no modo livre);
- um AGV com `agv: true` num modelo com `network` segue o controle de tráfego da rede, não os
  corredores.

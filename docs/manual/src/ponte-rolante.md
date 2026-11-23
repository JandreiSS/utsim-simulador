# Ponte rolante

A ponte rolante (`Crane`, também `PonteRolante`) é um transportador que anda por três
eixos sobre uma área retangular: a ponte, no caminho de rolamento; o carro, na viga; e o
gancho, na vertical. Ela recebe os pedidos como qualquer recurso móvel: pelo campo
`transport` do objeto de onde a peça sai, ou por um `Dispatcher` cuja equipe tenha pontes.
Os pedidos esperam na fila da ponte, pela prioridade, e a regra do despachante escolhe
qual ponte atende.

```ron
(name: "ponte", kind: Crane(
    bridge: (speed: 0.8, acceleration: 0.2, deceleration: 0.2),   // ponte (m/s, m/s²)
    trolley: (speed: 0.4, acceleration: 0.2, deceleration: 0.2),  // carro
    hoist: (speed: 0.1, acceleration: 0.1, deceleration: 0.1),    // gancho
    hook_height: 6.0,                                // altura em que o gancho anda (m)
    attach: Triangular(min: 40.0, mode: 60.0, max: 90.0),   // engate
    detach: Triangular(min: 20.0, mode: 30.0, max: 45.0),   // desengate
    capacity_t: 10.0,                                // capacidade, em toneladas
    motion: Safe,
), position: (20.0, 9.0, 0.0), size: Some((40.0, 18.0, 8.0))),
```

O exemplo completo está em `examples/ponte_rolante.ron`: uma baia de caldeiraria em que
a ponte leva chapas de 2 a 8 t do recebimento ao corte, do corte à solda e da solda à
expedição.

## Área de cobertura

A área sai do `size` da ponte, centrada na sua posição e girada pelo `visual.yaw`:

- o comprimento (x local) é o curso da ponte, no caminho de rolamento;
- a largura (y local) é o vão, onde o carro anda;
- a altura é a altura útil do gancho. `hook_height` não pode passar dela.

Cada ponto servido é levado para dentro do retângulo: a ponte vai até o x dele e o carro
até o y. O gancho desce até a altura do objeto (o z da sua posição), nunca abaixo do piso
da ponte. Um objeto que a ponte atende diretamente (`transport: "ponte"`), ou o destino
dele, fora da área é erro de validação. Quando a ponte atende por um despachante, um
ponto fora da área vira aviso, e a ponte vai até a borda mais próxima.

## Os três eixos

Cada eixo tem `speed` e, opcionais, `acceleration` e `deceleration`. O perfil é o mesmo
dos outros recursos móveis: trapezoidal, ou triangular quando o trecho é curto demais
para chegar à velocidade. `motion` diz como os eixos se combinam:

| `motion` | Como anda |
|---|---|
| `Safe` (padrão; `Segura`) | O gancho anda em cima, a `hook_height`. A ponte e o carro andam juntos, e o tempo do trecho é o maior dos dois. Em cada ponta o gancho desce, engata ou solta e sobe, sozinho. |
| `Sequential` (`Sequencial`) | Como `Safe`, mas a ponte anda e depois o carro: os tempos se somam. |
| `Parallel` (`Paralela`) | Os três eixos juntos. O gancho vai direto da altura em que está até a do próximo ponto enquanto a ponte e o carro andam, e fica embaixo depois de engatar ou soltar. |

Uma viagem com `Safe`:

1. ir vazia até a peça;
2. descer o gancho e engatar (`attach`); a peça sai da origem no fim do engate;
3. subir;
4. andar até o destino;
5. descer e soltar (`detach`); a peça entra no destino no fim do desengate;
6. subir.

Durante o engate e a descida e subida em volta dele, o estado é `loading`; em volta do
desengate, `unloading`.

## Capacidade

O peso da peça vem do rótulo `weight_label` (padrão `"weight"`), em kg, o mesmo das
pessoas. Uma peça mais pesada que `capacity_t` é recusada: a ponte não vai buscá-la,
conta a recusa em `crane.refused`, e a peça fica parada onde está. O `utsim check` avisa
antes, quando o maior peso que as chegadas podem gerar passa da capacidade.

## Segurança

Ninguém deve ficar sob carga suspensa. A ponte mede o tempo com carga pendurada
(`crane.loaded_time`, da subida depois do engate até a peça tocar o destino).

A área sob a carga é um quadrado no piso, centrado embaixo do gancho e girado com o
caminho de rolamento, de metade do lado `load_zone` (padrão 2 m: um quadrado de 4 × 4 m).
Num modelo com pontes e pessoas (recursos móveis com `visual.asset` de pessoa), a rodada
mede, enquanto há peça pendurada, o tempo de cada pessoa dentro dessa área e quantas vezes
ela entrou. A posição é tomada a cada 0,1 unidade de tempo, a mesma amostragem dos
quase-conflitos. As chaves do relatório estão em [Métricas](#métricas). A análise de
exposição (`utsim exposicao` e o UTSim Segurança do Trabalho) traz o mesmo número por
pessoa e por ponte.

Sozinha, a medida não muda o caminho das pessoas: ela mostra onde o leiaute ou a regra de
circulação deixam alguém passar sob a carga.

### Afastar as pessoas do caminho da carga

Com `keep_clear: true` (`afastar_pessoas`), a ponte tira as pessoas do caminho da carga.
O caminho é a faixa que a carga varre no piso: o quadrado de `load_zone` arrastado do ponto
onde a peça sobe até o ponto onde ela desce.

- Quem anda pelo piso (pessoas e veículos, todos os recursos móveis do modelo) e chegaria à
  faixa com a carga ainda passando para na borda e espera a carga passar. A espera conta
  em `load_wait`, e o estado é `blocked`. Quem já está dentro da faixa sai dela.
- A ponte com a carga espera quem já está atravessando a faixa sair. A espera conta em
  `crane.people_wait`, e o estado é `blocked`. Enquanto ela espera, ninguém mais entra.

Quem está parado, trabalhando num posto embaixo do caminho, não é tirado de lá. A medida
de tempo sob a carga continua contando essas pessoas. A ponte só afasta as pessoas no
trecho em que anda com a carga. Na subida e na descida, a carga está sobre o ponto de pegar
ou de soltar, onde o operador trabalha. Com `load_zone: 0`, não há faixa, e o
`utsim check` avisa.

Quando alguém passa sob a carga de uma ponte sem `keep_clear`, a análise de exposição
(`utsim exposicao`) diz quanto tempo foi e sugere ligar a opção.

## Duas pontes no mesmo caminho

Duas pontes paralelas cujas áreas se sobrepõem andam nos mesmos trilhos: uma não passa pela
outra (anticolisão). Os ganchos das duas ficam sempre a pelo menos `bridge_gap`
(`folga_pontes`, padrão 3 m) um do outro, ao longo do caminho. Com três ou mais pontes no
mesmo caminho, vale a maior folga do grupo. O `utsim check` mostra, na linha de cada ponte,
com quais ela divide o caminho.

Antes de andar, a ponte reserva o trecho do caminho que vai percorrer. Se outra ponte está
no caminho:

- parada, sem trabalho, ou parada à espera de outra ponte, ela sai do caminho: anda pelo
  caminho de rolamento só o necessário, para o lado em que está, e conta em
  `crane.gave_way`;
- ocupada (andando, engatando, soltando), a ponte espera, no estado `blocked`, até o trecho
  ficar livre, e conta a espera em `crane.lane_wait`.

Uma ponte que precisa passar para o outro lado da outra nunca chega lá: elas não trocam de
lugar no caminho. Atribua a cada ponte os pontos do seu lado, pelo campo `transport` ou por
um `Dispatcher` de cada trecho.

Pontes em caminhos que se cruzam em ângulo geram um aviso: o UTSim não impede a colisão
entre elas.

## Operador

O campo `operator` (`operador`) diz quem opera a ponte:

- `Cabin` (`Cabine`, o padrão): da cabine. O operador não é modelado como pessoa.
- `Pendant(team: "operadores")` (`Botoeira(equipe: ...)`): uma pessoa da equipe opera a
  botoeira e acompanha a carga a pé.
- `Radio(team: "operadores")` (`Controle(equipe: ...)`): uma pessoa da equipe opera por
  controle remoto, do ponto onde encontrou a ponte.

A equipe é uma pessoa (`Transporter`) ou um `Dispatcher` só de pessoas. A cada transporte,
a ponte pede uma pessoa à equipe assim que sai para buscar a peça. A pessoa anda até o
ponto de pegar e para ao lado da carga, 1 m fora da área sob a carga (`load_zone`),
atravessada ao caminho de rolamento. A ponte vai até a peça ao mesmo tempo e só engata
quando a pessoa chega. O tempo de espera vai para `crane.operator_wait`.

Com botoeira, a pessoa anda junto com a carga até o destino, sempre à mesma distância do
gancho. Por isso ela nunca fica sob a carga. A viagem carregada nunca é mais rápida que
a pessoa: o tempo é o maior entre o dos eixos e a distância no piso dividida pela
velocidade da pessoa mais lenta da equipe. Com rádio, a pessoa fica onde encontrou a ponte
e a ponte anda na sua velocidade.

A pessoa fica ocupada até a peça ser solta e o gancho voltar para cima. Esse trabalho não
é intercalado com outros casos (multitarefa) nem interrompido por um pedido mais urgente.

## Campos

| Campo | Padrão |
|---|---|
| `bridge`, `trolley`, `hoist` (`ponte`, `carro`, `talha`) | obrigatórios: `(speed, acceleration, deceleration)`, `speed` maior que 0; as acelerações, se dadas, maiores que 0 (também `velocidade`, `aceleracao`, `desaceleracao`) |
| `hook_height` (`altura_gancho`) | obrigatória: maior que 0 e no máximo a altura do `size` |
| `attach`, `detach` (`engate`, `desengate`) | `Const(0.0)` |
| `capacity_t` (`capacidade_t`) | obrigatória, maior que 0 (toneladas) |
| `motion` (`movimento`) | `Safe` (ou `Sequential`, `Parallel`) |
| `weight_label` (`rotulo_peso`) | `"weight"` |
| `home` (`volta`) | `false`: com `true`, volta à sua posição quando fica sem pedidos |
| `load_zone` (`zona_carga`) | `2.0`: metade do lado, em metros, da área sob a carga; 0 ou mais |
| `operator` (`operador`) | `Cabin`: ou `Pendant(team: ...)`, `Radio(team: ...)` (veja [Operador](#operador)) |
| `bridge_gap` (`folga_pontes`) | `3.0`: distância, em metros, entre os ganchos de duas pontes no mesmo caminho; 0 ou mais (veja [Duas pontes no mesmo caminho](#duas-pontes-no-mesmo-caminho)) |
| `keep_clear` (`afastar_pessoas`) | `false`: com `true`, as pessoas esperam a carga passar e a ponte espera quem está atravessando (veja [Afastar as pessoas do caminho da carga](#afastar-as-pessoas-do-caminho-da-carga)) |

A ponte precisa de `size`. Ela não opera postos (`operators.team` não pode ser uma ponte)
e não tem perfil de pessoa.

No UTSim 3D, todos esses campos estão no painel de propriedades da ponte, em
**Parâmetros**: velocidade e acelerações de cada eixo, ordem dos movimentos, altura do
gancho, capacidade, rótulo do peso, engate e desengate, controle (cabine, botoeira ou
rádio, com a equipe do operador), área sob a carga, afastar as pessoas e folga entre
pontes. A linha **Caminho** diz com quais pontes ela divide o caminho. O comprimento do
caminho e o vão ficam em **Posição**, nas dimensões do objeto.

## Métricas

| Métrica | Significado |
|---|---|
| `crane.trips` | peças entregues |
| `crane.refused` | peças recusadas por passarem da capacidade |
| `crane.loaded_time` | tempo com uma peça pendurada no gancho |
| `crane.bridge_time`, `crane.trolley_time`, `crane.hoist_time` | tempo em que cada eixo andou (com `Safe`, a ponte e o carro andam ao mesmo tempo) |
| `crane.operator_wait` | com botoeira ou rádio: tempo parada no ponto de pegar, à espera da pessoa |
| `crane.lane_wait` | com outra ponte no mesmo caminho: tempo à espera de que ela saia do trecho |
| `crane.gave_way` | com outra ponte no mesmo caminho: vezes que saiu do caminho dela |
| `crane.people_wait` | com `keep_clear`: tempo à espera de que as pessoas saiam do caminho da carga |
| `load_wait` (em cada recurso móvel) | com uma ponte com `keep_clear`: tempo parado na borda do caminho da carga |
| `tempo_sob_carga`, `entradas_sob_carga` | com pessoas no modelo: tempo e entradas de pessoas sob a carga desta ponte. As mesmas chaves aparecem em cada pessoa, somando todas as pontes |

Os estados (`idle`, `travel_empty`, `travel_loaded`, `loading`, `unloading`) e a
utilização são os de um transportador.

## Rastro e 3D

A ponte grava no rastro as viagens no piso (`Travel`, a posição do gancho no plano) e a
altura do gancho (`Hoist`: de `from` a `to`, em metros na altura do mundo, durante
`duration`). No início da rodada, um `Hoist` sem duração marca o gancho em cima. Depois
vem um a cada descida ou subida. Em `Parallel`, o `Hoist` corre junto com a viagem.

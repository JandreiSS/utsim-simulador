# Logística e armazém

A terceira **linha especializada** do UTSim estuda centros de distribuição e armazéns: caminhões
que chegam com agenda, pátio e docas, armazenagem em porta-paletes, pedidos com várias linhas,
ondas de separação, consolidação e expedição. A linha usa o mesmo núcleo, o mesmo `.ron` e as
mesmas replicações do resto do programa.

A linha traz:

- uma **biblioteca** de 14 predefinições em `linhas/logistica/biblioteca.ron`: chegada de
  caminhões, pátio, doca, porta-paletes, entrada de pedidos, onda de separação, divisão em
  linhas, separação, consolidação, expedição, empilhadeiras, separadores, uma empilhadeira e um
  separador;
- um **vocabulário** (o item é a **carga**, o porta-paletes, a doca, a empilhadeira), em
  `linhas/logistica/vocabulario.ron`;
- quatro peças novas do núcleo, que servem a qualquer modelo: a divisão de um item pelo valor
  de um rótulo, a consolidação por grupo, as ondas por horário e o endereçamento pela curva
  ABC com tempo de acesso;
- os **indicadores** do armazém com intervalo de confiança: tempo de doca, espera no pátio,
  linhas separadas por hora, pedidos no prazo, ocupação do porta-paletes e das equipes;
- três **modelos prontos**.

## Primeiro contato

```bash
utsim linha lista
utsim linha logistica predefinicoes
utsim linha logistica relatorio examples/cd_recebimento_expedicao.ron --html relatorio.html
utsim linha logistica relatorio examples/cross_docking.ron --meta-doca 45
```

O `--meta-doca` é a meta do tempo de doca, em minutos (padrão 60): o caminhão está na meta
quando sai da doca até esse tempo depois de encostar. `utsim run modelo.ron --linha logistica`
mostra os indicadores depois do resultado de costume, com a meta padrão.

## Recebimento: agenda, pátio e docas

- **Agenda de caminhões**: a chegada de caminhões aceita a tabela de horários da chegada comum
  (`table` com `mode: Times` e `repeat: Cycle(1440.0)`). Cada linha da tabela pode trazer os
  seus rótulos: quantos `paletes` o caminhão traz e a `classe` ABC da carga.
- **Pátio**: os caminhões esperam uma doca livre.
- **Doca**: um divisor com `mode: SplitBy(count: "paletes", group: "caminhao")`. O caminhão
  encosta (o tempo de manobra é o tempo do divisor), vira tantos paletes quanto diz o rótulo
  `paletes`, e cada palete leva o número do caminhão no rótulo `caminhao`. A doca só recebe o
  próximo caminhão quando sai o último palete: com as empilhadeiras ocupadas, o caminhão fica
  mais tempo na doca. Para mais docas, ligue mais docas ao mesmo pátio.
- **Empilhadeiras**: a doca usa a equipe de empilhadeiras no campo `transport`; elas levam cada
  palete ao destino.

## Armazenagem: curva ABC e tempo de acesso

O porta-paletes aceita `placement: Abc(a: 0.2, b: 0.3)` e
`access: Some((per_bay: 0.2, per_level: 0.5))`:

- o tempo de acesso de uma posição é `coluna × per_bay + nível × per_level` (coluna e nível
  contados a partir de zero, da frente e do chão);
- as posições são ordenadas pelo tempo de acesso; a fração `a` mais rápida é da classe A, a
  fração `b` seguinte é da classe B, o resto é da classe C;
- o palete vai para a posição livre mais rápida da sua classe (rótulo `classe`: 1 = A, 2 = B,
  3 = C; sem o rótulo, C). Com a zona da classe cheia, vai para a zona pior seguinte que tiver
  lugar, e só então para uma melhor;
- o tempo de acesso entra na permanência do palete e aparece no relatório de qualquer modelo
  como `access_time` e `access_time.A`, `.B`, `.C`.

## Pedidos: linhas, ondas e consolidação

- **Entrada de pedidos**: cada pedido traz o rótulo `linhas` (quantas linhas tem) e o `sla`
  (prazo em minutos a partir da entrada).
- **Onda de separação**: uma área de espera com `waves: Some((cycle: 1440.0, times: [420.0,
  600.0]))`. Os pedidos esperam e, em cada horário do ciclo, todos os que esperam seguem
  juntos. A mesma peça serve para a coleta da transportadora (um horário só) ou para os
  caminhões de saída de um cross-docking.
- **Divisão em linhas**: um divisor com `mode: SplitBy(count: "linhas", group: "pedido")`. O
  pedido vira as suas linhas; cada uma leva o número do pedido (`pedido`) e o seu índice
  (`linha`, de 1 em diante).
- **Separação**: um processo atendido pela equipe de separadores; o tempo é por linha.
- **Consolidação**: uma área de espera com `gather: Some((group: "pedido", count: "linhas"))`.
  As linhas de cada pedido esperam até chegar a última, e o pedido segue inteiro (as outras
  linhas vão dentro da primeira). Só se expede o pedido completo.

## Manutenção, EPI e qualidade ficam no nó

As quebras das empilhadeiras ficam no campo `downtime` de cada uma (aba Manutenção), o EPI
exigido no campo `ppe` (aba Segurança), e a inspeção ou o refugo, se houver, no campo
`quality`. São os mesmos campos que os apps da família editam; a linha não guarda cópia
nenhuma desses dados.

## Indicadores

Contam o que terminou depois do aquecimento. Os objetos são reconhecidos pela predefinição que
os criou (`logistica/doca`, `logistica/separacao`, `logistica/expedicao`); sem predefinição, uma
doca é um divisor que divide por `paletes`, e a separação é um processo que recebe linhas.

| Indicador | Definição |
|---|---|
| Caminhões descarregados | caminhões cujo último palete saiu da doca |
| Espera no pátio | da chegada do caminhão até encostar na doca |
| Tempo de doca | de encostar até sair o último palete |
| Caminhões na meta de doca | fração com tempo de doca até a meta (`--meta-doca`) |
| Paletes recebidos | soma do rótulo `paletes` dos caminhões descarregados |
| Linhas separadas por hora | linhas que saíram da separação ÷ horas da janela; também por hora do dia |
| Pedidos expedidos por dia | pedidos completos que chegaram à expedição |
| Ciclo do pedido | da entrada do pedido até a expedição |
| Linhas por pedido | média do rótulo `linhas` dos pedidos expedidos |
| Pedidos no prazo | fração dos pedidos com `sla` expedidos dentro dele |
| Porta-paletes | ocupação média das posições |
| Tempo de acesso por classe | média do tempo de acesso das cargas de cada classe ABC |
| Ocupação das equipes | fração do turno andando, pegando, deixando ou trabalhando |
| Docas | fração do tempo com caminhão |

O relatório mostra só as partes que o modelo tem: sem caminhões, não há recebimento; sem
pedidos, não há expedição de pedidos. O `--html` grava a página com os cartões, as linhas
separadas por hora do dia, as docas, o porta-paletes, o acesso por classe e a ocupação. O
`--json` grava os indicadores de cada replicação e as linhas por hora do dia.

## Modelos prontos

| Modelo | O que mostra |
|---|---|
| `examples/cd_recebimento_expedicao.ron` | oito caminhões por dia com agenda, duas docas, duas empilhadeiras com quebras, porta-paletes com curva ABC, e pedidos com várias linhas separados por duas pessoas |
| `examples/separacao_em_ondas.ron` | pedidos de comércio eletrônico o dia todo, ondas às 7h, 10h, 13h e 16h, quatro separadores, consolidação e coleta da transportadora às 18h |
| `examples/cross_docking.ron` | dez caminhões por dia, três docas, três empilhadeiras levando os paletes direto à área de carregamento, saídas às 16h, 18h e 21h |

Experimentos típicos: espalhar a agenda dos caminhões, pôr uma terceira doca ou uma empilhadeira
a mais, mudar as frações da curva ABC, acrescentar uma onda, mudar o horário da coleta.

## O que ainda não está na linha

- **Rotas reais nos corredores** para o tempo de acesso (decisão G-D2: hoje o acesso é um tempo
  por coluna e por nível).
- **Ondas dinâmicas**, liberadas por volume em vez de horário (decisão G-D3).
- **Expedição parcial** de pedidos incompletos (decisão G-D4: hoje só sai o pedido completo).
- **Separação tirando do estoque** do porta-paletes: nos modelos prontos, armazenagem e
  separação correm em paralelo.

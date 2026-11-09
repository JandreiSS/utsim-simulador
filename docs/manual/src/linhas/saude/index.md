# Saúde e hospital

A quarta **linha especializada** do UTSim estuda serviços de saúde: pronto-socorro com
classificação de risco, ambulatório com agenda e internação em leitos. A linha usa o mesmo
núcleo, o mesmo `.ron` e as mesmas replicações do resto do programa.

A linha traz:

- uma **biblioteca** de 15 predefinições em `linhas/saude/biblioteca.ron`: chegada de
  pacientes, agenda de consultas, recepção, espera por gravidade, classificação de risco,
  consultório, sala de procedimento, exame, medicação e observação, leitos de internação, alta,
  equipe médica, enfermagem, um médico e um profissional de enfermagem;
- um **vocabulário** (o item é o **paciente**, a saída é a **alta**, a pessoa é o
  **profissional**), em `linhas/saude/vocabulario.ron`;
- uma peça nova do núcleo, que serve a qualquer modelo: **várias equipes ao mesmo tempo** no
  mesmo processo (`also`), para o médico e o enfermeiro na mesma sala;
- os **indicadores** do serviço com intervalo de confiança: espera até o médico por cor,
  atendidos no tempo-alvo, saídas sem atendimento, faltas, permanência, ocupação das equipes e
  dos leitos e pacientes presentes por hora do dia;
- três **modelos prontos**.

## Primeiro contato

```bash
utsim linha lista
utsim linha saude predefinicoes
utsim linha saude relatorio examples/pronto_socorro.ron --html relatorio.html
utsim linha saude relatorio examples/internacao_leitos.ron --ocupacao-leitos 0.9
utsim linha saude relatorio examples/pronto_socorro.ron --alvos 0,15,60,120,240
```

O `--alvos` traz os cinco tempos-alvo de espera até o médico, do vermelho ao azul, em minutos
(padrão `0,10,60,120,240`). O `--ocupacao-leitos` é a ocupação-alvo dos leitos, de 0 a 1
(padrão 0,85). `utsim run modelo.ron --linha saude` mostra os indicadores depois do resultado
de costume, com as metas padrão.

## Pacientes e classificação de risco

O paciente é um item com o rótulo `gravidade`, a cor da classificação de risco:

| `gravidade` | Cor | Tempo-alvo padrão |
|---|---|---|
| 1 | vermelho (emergência) | 0 min |
| 2 | laranja (muito urgente) | 10 min |
| 3 | amarelo (urgente) | 60 min |
| 4 | verde (pouco urgente) | 120 min |
| 5 | azul (não urgente) | 240 min |

A cor é sorteada na chegada (por exemplo
`Discrete([(1.0, 0.01), (2.0, 0.09), (3.0, 0.3), (4.0, 0.45), (5.0, 0.15)])`); a classificação
de risco é o processo da enfermagem que consome o tempo de avaliar (decisão H-D3). A **espera
por gravidade** é uma área de espera com `order: Some(Asc("gravidade"))`: chama primeiro o mais
grave e, entre iguais, quem chegou antes. Com `patience`, quem espera demais vai embora sem
atendimento (rótulo `abandonou`).

## Agenda, faltas e encaixes

A **agenda de consultas** é uma chegada com a tabela de horários (`table` com `mode: Times` e
`repeat: Cycle(1440.0)`); cada linha diz quantos pacientes estão marcados naquele minuto do dia.
O rótulo `presenca` diz se o paciente veio (1) ou faltou (2), e a agenda tem
`output: ByLabel("presenca")`: quem falta sai pela saída 2, ligada a uma alta "faltas". Os
encaixes são uma chegada comum, sem hora marcada; com uma cor mais leve que a dos marcados,
são atendidos depois deles.

## Médico e enfermagem juntos

O processo ganhou o campo `also`, uma lista de outras equipes pedidas junto com `operators`:

```ron
(name: "procedimento", kind: Processor(process: Triangular(min: 15.0, mode: 25.0, max: 45.0),
    operators: Some(OperatorUse(team: "medicos", priority: Some(10))),
    also: [OperatorUse(team: "enfermagem")]))
```

A etapa só começa quando chegam as pessoas de todas as equipes que ela usa; quem chega primeiro
espera os outros na sala. Cada equipe tem os seus `count`, `setup`, `process`, `requires` e
`priority`. Se as equipes da preparação e do processo forem diferentes, quem preparou é
liberado e as do processo são chamadas. Como quem chega primeiro fica parado esperando, vale
dar prioridade ao pedido da equipe mais disputada (no exemplo, o médico).

O mesmo campo serve fora da saúde: dois operadores de ofícios diferentes numa troca de
ferramenta, o motorista e o ajudante numa entrega. O painel de propriedades mostra o campo em
"Outras equipes ao mesmo tempo", debaixo dos operadores.

## Exames, medicação e leitos

- **Exame**: um processo com o tempo até o resultado; a parada do equipamento (o tomógrafo, o
  raio X) fica no campo `downtime` do nó, aba Manutenção.
- **Medicação e observação**: um processo com poltronas (`capacity`), em que a enfermagem só
  trabalha no preparo e na aplicação (`setup`, com `process: false` nos operadores) e o
  paciente fica em observação sem prender ninguém.
- **Leitos**: um processo com `capacity` igual ao número de leitos e o tempo de permanência
  (por exemplo `LogNormal(mean: 4320.0, sd: 2880.0)`, três dias). A espera por leito é a área
  de espera ligada antes. O paciente segura o leito só durante a permanência (decisão H-D4):
  os exames acontecem antes da internação.

## Parada, EPI e conferência ficam no nó

A parada dos equipamentos fica no campo `downtime` de cada um (aba Manutenção), o EPI de cada
profissional e da sala de procedimento no campo `ppe` (aba Segurança), e a dupla checagem da
medicação, se houver, no campo `quality` do nó. São os mesmos campos que os apps da família
editam; a linha não guarda cópia nenhuma desses dados.

## Indicadores

Contam os pacientes que saíram depois do aquecimento. Os objetos são reconhecidos pela
predefinição que os criou (`saude/triagem`, `saude/consultorio`, `saude/procedimento`,
`saude/leitos`); sem predefinição, pelo nome (`triagem` ou `classifica`, `consult`, `leito`).

| Indicador | Definição |
|---|---|
| Pacientes atendidos por dia | saíram do serviço sem desistir e sem faltar |
| Espera até o médico | do fim da classificação de risco (ou da chegada, sem ela) até entrar no primeiro consultório |
| Da chegada ao médico | da chegada até entrar no primeiro consultório |
| Atendidos no tempo-alvo da cor | fração com espera até o médico dentro do alvo da sua cor (`--alvos`) |
| Saíram sem atendimento | desistências ÷ (desistências + atendidos) |
| Faltas na agenda por dia | pacientes marcados com `presenca` 2 |
| Permanência no serviço | da chegada até a saída, dos atendidos |
| Espera por leito | permanência média nas áreas de espera que só levam a leitos |
| Permanência no leito | tempo médio no leito |
| Por cor | atendidos, espera, no alvo, saídas sem atendimento e permanência de cada cor |
| Ocupação das equipes | fração do plantão andando ou atendendo |
| Ocupação dos leitos | leitos ocupados em média ÷ leitos, com a ocupação-alvo (`--ocupacao-leitos`) |
| Pacientes presentes por hora do dia | média de pacientes no serviço em cada hora |

O relatório mostra só as partes que o modelo tem: sem consultório, não há espera até o médico;
sem agenda, não há faltas; sem leitos, não há internação. O `--html` grava a página com os
cartões, a tabela por cor, o gráfico dos pacientes presentes por hora do dia, os leitos e a
ocupação das equipes. O `--json` grava os indicadores de cada replicação e os presentes por
hora.

## Modelos prontos

| Modelo | O que mostra |
|---|---|
| `examples/pronto_socorro.ron` | chegadas mais fortes de dia e fracas de madrugada, cinco cores, duas salas de classificação, quatro consultórios, medicação, laboratório, tomógrafo com paradas, sala de procedimento com médico e enfermagem juntos e 30 leitos de internação |
| `examples/ambulatorio_agenda.ron` | agendas de clínica geral e de pediatria com 15 % de faltas, encaixes sem hora marcada, pré-consulta da enfermagem e consultórios que exigem a especialidade |
| `examples/internacao_leitos.ron` | internações do pronto-socorro a qualquer hora e cirurgias eletivas marcadas, duas unidades com permanências diferentes, admissão pela enfermagem e ocupação perto da meta de 85 % |

Experimentos típicos: tirar um médico da madrugada, mudar a mistura das cores, encurtar a
paciência, pôr um consultório a mais, mudar a fração de faltas e os encaixes, acrescentar ou
tirar leitos e ver a espera por leito.

## O que ainda não está na linha

- **Cor decidida na classificação** em vez de sorteada na chegada (decisão H-D3: precisa de um
  rótulo gravado por um processo).
- **Paciente que segura o leito** enquanto sai para exames (decisão H-D4).
- **Ativos 3D de hospital** (leito, maca, cadeira de rodas): por enquanto, mesas, áreas e
  máquinas da biblioteca comum.

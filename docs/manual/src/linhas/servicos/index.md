# Serviços e atendimento

A segunda **linha especializada** do UTSim estuda o atendimento a clientes: agências e lojas,
balcões de serviço público, recepção de clínicas, centrais de atendimento por telefone, chat e
e-mail. O cliente chega, tira uma senha, espera, às vezes desiste, e é chamado num guichê ou
atendido num canal. A linha usa o mesmo núcleo, o mesmo `.ron` e as mesmas replicações do resto
do programa, e reaproveita a multitarefa e as interrupções do [Lean Office](../lean-office/index.md).

A linha traz:

- uma **biblioteca** de 7 predefinições em `linhas/servicos/biblioteca.ron`: chegada de
  clientes, fila de espera, guichê, canal remoto, equipe de atendimento, atendente e saída;
- um **vocabulário** (o item é o **cliente**, o processo é o **guichê**, o operador é o
  **atendente**), em `linhas/servicos/vocabulario.ron`;
- a **desistência** na fila: cada cliente tem a sua paciência e vai embora quando ela acaba;
- os **indicadores** de atendimento com intervalo de confiança, por tipo de senha e por fila,
  o **nível de serviço**, o **painel de chamada** e o **dimensionamento por hora**;
- três **modelos prontos**.

## Primeiro contato

```bash
utsim linha lista
utsim linha servicos predefinicoes
utsim linha servicos relatorio examples/agencia_com_senhas.ron --html relatorio.html
utsim linha servicos relatorio examples/central_multicanal.ron --meta 1 --alvo 0.8
```

O `--meta` é a espera máxima da meta de nível de serviço, em minutos (padrão 15), e o `--alvo`
é a fração dos clientes que deve ficar dentro dela (padrão 0,8, os "80 % em até 15 minutos").
`utsim run modelo.ron --linha servicos` mostra os indicadores depois do resultado de costume, com
a meta padrão.

## Senhas, filas e desistência

- **Tipo da senha**: o rótulo numérico `tipo` do cliente, gravado na chegada: 1 normal, 2
  preferencial, 3 agendado. A fila de espera com `order: Desc("tipo")` chama primeiro o tipo
  maior; empates ficam na ordem de chegada. O número da senha (`N007`, `P012`) é a ordem de
  chegada do cliente entre os do seu tipo, e aparece no painel de chamada.
- **Desistência**: a fila de espera aceita `patience: Some(Exp(mean: 20.0))`. Cada cliente
  sorteia a paciência ao entrar na fila; se ainda estiver nela quando a paciência acaba, ele
  ganha o rótulo `abandonou = 1` e sai do modelo. A paciência corre também com a fila fora do
  horário (o cliente espera do mesmo jeito). O relatório de qualquer modelo mostra
  `abandoned.count` na fila e a estatística `abandon_wait` (quanto esperou quem desistiu).
- **Guichê**: um processo atendido por uma pessoa da equipe; `capacity` é o número de guichês
  (os clientes chamados ao mesmo tempo). Quando os atendentes saem de turno, use
  `Calendar("nome", Finish)` para que terminem o cliente que estão atendendo.

## Canais e interrupções

A predefinição **Canal remoto** monta um canal inteiro num só passo: os contatos, a fila do
canal (com paciência), o atendimento pela equipe e a saída. O campo "Contatos em atendimento ao
mesmo tempo" limita quantos contatos o canal atende de uma vez; os outros esperam na fila, onde
a paciência conta.

Para uma equipe que atende vários canais:

- a [multitarefa](../lean-office/multitarefa.md) dos atendentes ("Contatos ao mesmo tempo") deixa
  cada um tocar vários chats ou e-mails ao mesmo tempo. Só contatos de mesma prioridade se
  intercalam;
- o telefone, com prioridade maior no canal, e a equipe com **Interrupções: Full** tiram o
  atendente do chat ou do e-mail para atender a ligação; o contato deixado volta a esperar a
  equipe com o tempo que faltava (veja [Interrupções](../lean-office/interrupcoes.md)).

## Indicadores

Contam os clientes que terminaram (atendidos ou desistentes) depois do aquecimento.

| Indicador | Definição |
|---|---|
| Clientes atendidos e abandonos | contagens; % de abandono = abandonos ÷ (atendidos + abandonos) |
| Tempo médio de espera (TME) | tempo no modelo − tempo de atendimento sorteado, dos atendidos: conta a fila, a espera no guichê por um atendente e o tempo deixado de lado por uma interrupção |
| Espera do 90º percentil | 90 % dos atendidos esperaram até este tempo |
| Tempo médio de atendimento (TMA) | atendimento sorteado por cliente atendido |
| Nível de serviço | fração dos atendidos que esperaram até a meta (`--meta`) |
| Espera até desistir | tempo médio que quem desistiu esperou |
| Por tipo de senha e por fila | os mesmos números para cada tipo (`N`, `P`, `A`) e para cada fila de entrada (cada canal) |
| Ocupação dos atendentes | fração do expediente atendendo (ou trocando de canal) |

**Dimensionamento por hora.** Para cada hora do dia com chegadas, o relatório mostra as
chegadas por hora (média dos dias com chegadas nessa hora), o número de atendentes que a fila
M/M/c (fórmula de Erlang C, sem desistência, o que é conservador) pede para cumprir a meta com o
TMA do modelo, e o nível de serviço simulado de quem chegou naquela hora. Compare com a escala
do modelo: se a hora pede mais atendentes que os de turno, a simulação mostra a espera e as
desistências nessa hora. Com vários canais, o cálculo junta todos os contatos e usa o TMA médio.

**Painel de chamada.** As primeiras 40 chamadas da replicação 0 depois do aquecimento: dia e
hora, senha, guichê que chamou e quanto o cliente esperou.

O `--html` grava a página com os cartões, as tabelas por tipo e por fila, o histograma da
espera com a meta, a ocupação dos atendentes, o dimensionamento e o painel. O `--json` grava os
indicadores de cada replicação, o painel e o dimensionamento.

## Modelos prontos

| Modelo | O que mostra |
|---|---|
| `examples/agencia_com_senhas.ron` | agência das 10h às 16h, senhas normal e preferencial, pico no almoço, quatro caixas e dois de reforço no almoço (um a menos do que o dimensionamento pede no pico), desistência |
| `examples/central_multicanal.ron` | dez atendentes em telefone, chat e e-mail, multitarefa nos canais escritos e o telefone interrompendo (rode com `--meta 1`) |
| `examples/atendimento_ao_cidadao.ron` | triagem, serviços de documentos e de benefícios com equipes próprias, senha agendada antes da preferencial, meta de 30 minutos (rode com `--meta 30`) |

Experimentos típicos: mudar a escala do reforço, separar a fila preferencial, juntar as equipes
de documentos e benefícios, dar mais ou menos contatos ao mesmo tempo aos atendentes, deixar o
telefone sem interromper (`preempt: Never`).

## O que ainda não está na linha

- **Painel de chamada ao vivo** na interface Godot (decisão S-D4 do dono).
- **Dimensionamento com desistência** (Erlang A, decisão S-D5).
- **Quem desiste e volta outro dia** (decisão S-D3: hoje quem desiste sai do modelo).

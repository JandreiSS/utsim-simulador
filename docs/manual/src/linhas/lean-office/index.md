# Lean Office

O **Lean Office** (processos administrativos) é a primeira **linha especializada** do UTSim:
um pacote de domínio para estudar **processos administrativos de qualquer área** sobre o
mesmo núcleo, o mesmo formato `.ron` e as mesmas replicações do resto do programa. O nome é o
termo consagrado, mas o escritório não é o centro: o que importa é o caso que anda pelo
processo, seja o trabalho feito numa mesa, remoto ou dentro de um sistema. Exemplos de
processos que a linha cobre:

- compras e suprimentos;
- financeiro: contas a pagar e a receber, faturamento;
- RH: admissão, folha;
- comercial: do pedido ao faturamento, orçamentos;
- atendimento e pós-venda;
- jurídico e contratos;
- TI e chamados;
- planejamento e PCP administrativo;
- serviço público: protocolo, processos, licenças;
- secretaria acadêmica.

A linha traz:

- uma **biblioteca** de 12 predefinições neutras quanto ao local (entrada de pedidos, caixa
  de entrada, posto de trabalho, aprovação, verificação, tramitação, espera por informação,
  lote, reunião, equipe, pessoa e conclusão), em `linhas/lean_office/biblioteca.ron`. A caixa
  de entrada pode ser uma bandeja, uma caixa de e-mail ou a lista de trabalho de um sistema; a tramitação
  pode ser malote, e-mail ou sistema;
- um **vocabulário** que troca os termos gerais do UTSim pelos do processo administrativo (o
  que anda pelo processo é um **caso**, o tempo dele no processo é o **lead time**), em
  `linhas/lean_office/vocabulario.ron`;
- a **multitarefa** das pessoas: vários casos abertos ao mesmo tempo, com tempo perdido a cada
  troca;
- os **indicadores** do Lean Office (lead time, touch time, razão de atividade, %C&A, FPY, WIP
  de casos, SLA, ocupação das pessoas, esperas por causa) com intervalo de confiança;
- o **relatório da linha** em HTML, com o **mapa do fluxo de valor do processo
  administrativo** e o **fluxograma de raias por departamento**;
- cinco **modelos prontos** de áreas diferentes e o tutorial **Do mapa ao modelo**.

Colocar uma predefinição cria objetos comuns do UTSim: o modelo continua legível e editável
fora da linha e roda em qualquer edição do programa.

Os capítulos da linha:

- [Biblioteca](biblioteca.md): as predefinições, como colocá-las num modelo e a caixa de
  entrada por prioridade ou por prazo;
- [Multitarefa](multitarefa.md): a pessoa com vários casos abertos;
- [Indicadores e relatório](indicadores.md): as definições e o que o relatório mostra;
- [Modelos prontos](modelos-prontos.md): aprovação de compras, contas a pagar, central de
  solicitações, admissão de funcionário e pedido ao faturamento;
- [Tutorial: do mapa ao modelo](tutorial.md): um processo de cinco etapas, do mapa ao estado
  futuro;
- [Pacote da linha](pacote.md): o formato dos arquivos, para quem cria uma linha nova.

## Primeiro contato

```bash
utsim linha lista
utsim linha lean_office predefinicoes
utsim linha lean_office modelos
utsim linha lean_office relatorio examples/aprovacao_de_compras.ron --html relatorio.html
```

O terminal mostra os indicadores com a média entre replicações ± a meia-amplitude do
intervalo de 95 %; o arquivo `relatorio.html` é uma página única, sem recursos externos, que
abre em qualquer navegador e pode ser impressa em PDF.

Para comparar o estado atual com um estado futuro (uma alçada maior, uma etapa a menos,
fluxo unitário), grave o cenário em outro `.ron` e use `--comparar futuro.ron`: o relatório
mostra os dois mapas do fluxo de valor e a tabela de indicadores lado a lado.

`utsim run modelo.ron --linha lean_office` roda o modelo como sempre e, depois do resultado
de costume, mostra os indicadores da linha.

## Unidades e convenções

- **Tempo em minutos**, distâncias em metros. As pessoas andam a 72 m/min (1,2 m/s) até o
  posto de trabalho; no trabalho remoto ou em sistema, deixe o posto na posição da pessoa e
  ela não anda (como no modelo pronto Pedido ao faturamento).
- **Tempo corrido**: o lead time conta noites, fins de semana e esperas. O horário de
  trabalho entra pelos turnos (`shifts`), em geral um calendário `"expediente"` aplicado à
  entrada de pedidos e às pessoas. Os modelos prontos usam um calendário semanal de segunda a
  sexta, das 8h às 18h com almoço das 12h às 13h; a semana começa na segunda às 0h.
- **Rótulos dos casos** que os indicadores leem:

| Rótulo | Significado | Quem grava |
|---|---|---|
| `prioridade` | número, maior é mais urgente | a entrada de pedidos (campo `rotulos`) |
| `sla` | prazo em minutos a partir da entrada | a entrada de pedidos |
| `prazo` | instante limite (`instante da primeira caixa + sla`) | a primeira caixa de entrada |
| `erro` | 1 quando o caso tem um erro ainda não percebido | o posto (pelo %C&A), zerado pela verificação que percebe |
| `erros_<etapa>` | quantos erros a etapa gerou no caso | o posto |

## Campos do modelo usados pela linha

Quatro campos opcionais, que a simulação não lê (veja [Formato `.ron`](../../referencia-ron.md)):

- `line: "lean_office"` no modelo: o modelo é da linha;
- `preset: "lean_office/aprovacao"` em cada objeto: a predefinição que o criou, que dá o
  **papel** do objeto nos indicadores;
- `department: "Compras"` em cada objeto: o departamento, que vira a raia do fluxograma (não
  confundir com `areas` do modelo, que são áreas de piso, como sala de espera ou de pausa);
- `symbol: "inspecao"` em cada objeto: o símbolo do fluxograma (`inicio`, `fim`, `operacao`,
  `transporte`, `inspecao`, `espera`, `armazenagem`).

Modelos sem esses campos abrem e rodam como antes. Um objeto sem `preset` tem o papel do seu
tipo no núcleo (`Source` é entrada de pedidos, `Queue` é caixa de entrada, `Processor` é posto
de trabalho, `Sink` é conclusão), então os indicadores também valem para modelos que não são
da linha.

## O que ainda não está na linha

- **Interrupções** (telefone, chat, urgência que tira a pessoa do caso): dependem da
  preempção da pessoa que já está trabalhando, no plano de chegadas.
- **Vocabulário e biblioteca na interface** e a **vista 2D de planta** com zonas por
  departamento, para os processos com lugar físico (escritório, balcão, protocolo): depois da
  migração da interface.
- **Ativos 3D** de ambiente administrativo (mesa, armário, bandeja, balcão): primeiro os
  gerados por código e os livres CC0.
- **Modelos de projeto no UTSim Seis Sigma** ("Kaizen administrativo" e "DMAIC
  transacional").

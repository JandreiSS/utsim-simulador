# Modelos prontos

Os nove modelos prontos, de áreas diferentes (compras, financeiro, atendimento e TI, RH,
comercial, jurídico, serviço público, secretaria acadêmica, pós-venda), foram montados só com as predefinições da [biblioteca](biblioteca.md), com o
horário comercial de segunda a sexta. `utsim linha lean_office modelos` lista todos. Para
cada um:

```bash
utsim linha lean_office relatorio examples/<modelo>.ron --html relatorio.html
```

## Aprovação de compras

`examples/aprovacao_de_compras.ron`. Os pedidos chegam com valor, prioridade e prazo de 3
dias corridos. Compras cota o pedido (15 % saem com informação errada) e confere a cotação
(percebe 90 % dos erros e devolve ao solicitante, que demora a responder). A alçada depende do
valor: até 5 mil o gerente aprova, até 25 mil a diretoria, acima disso o comitê, que se reúne
às quartas das 14h às 16h. Cada aprovação pode devolver o pedido para nova cotação. Aprovado,
o pedido é emitido e concluído.

Experimentos típicos: aumentar a alçada do gerente (mudar o limite no script de `alcadas`),
fazer o comitê duas vezes por semana (mais uma janela no horário de `comite`), conferir na
própria cotação (%C&A maior), dedicar um comprador aos urgentes.

## Contas a pagar

`examples/contas_a_pagar.ron`. As notas fiscais chegam com valor, prioridade e prazo de
pagamento de 5 dias corridos. Contas a pagar lança cada nota (8 % saem com algum dado errado),
junta as notas lançadas num lote diário de 25 (ou o que houver depois de 8 horas) e confere o
lote: os erros percebidos voltam ao fornecedor, que demora a responder. A controladoria aprova
e a tesouraria paga, pelo prazo mais curto. No fechamento do mês (quinta e sexta da quarta
semana) a tesouraria não paga: o posto de pagamento usa o calendário `"fechamento"`, de quatro
semanas.

A maior espera é a do lote. Experimentos típicos: lote menor ou fluxo unitário (tirar
`lote_diario`), conferir no próprio lançamento, pagar também no fechamento (trocar o
calendário do pagamento por `"expediente"`).

## Central de solicitações

`examples/central_de_solicitacoes.ron`. As solicitações chegam com taxa por hora do dia (pico
das 8h às 10h, vale no almoço), com tipo (dúvida, problema, pedido de acesso) e prioridade
(normal, alta, urgente). O prazo depende da prioridade: 24, 8 e 4 horas corridas, gravado na
primeira caixa de entrada. O nível 1 faz a triagem e resolve as dúvidas e 60 % dos outros
casos; o resto vai ao nível 2, que atende pelo prazo mais curto. Os atendentes do nível 1
atendem vários canais ao mesmo tempo ([multitarefa](multitarefa.md): até 3 casos abertos,
cerca de 10 minutos em cada um, 2 minutos perdidos a cada troca).

Experimentos típicos: limitar a multitarefa (`max_open: 1`), mais um analista no nível 2,
resolver mais no nível 1 (mudar os 60 % do script de `roteamento`).

## Admissão de funcionário

`examples/admissao_de_funcionario.ron`. Cada admissão aprovada chega ao RH, com prazo de 10
dias corridos. O RH cadastra o candidato e pede os documentos, que o candidato demora a
mandar; a conferência devolve 10 % dos casos ao candidato. Depois o caso passa por e-mail para
a TI, que libera os acessos em lote (5 pedidos, ou o que houver depois de 2 dias), para o
financeiro, que cadastra na folha, e para o gestor, que confirma a data de início (10 % voltam
ao RH com alguma pendência).

O fluxograma de raias mostra as passagens entre RH, Candidato, TI, Financeiro e Gestor. A
espera pelos documentos e o lote da TI dominam o lead time. Experimentos típicos: pedir os
documentos junto com a proposta (espera menor), TI e financeiro em paralelo, acessos sem lote.

## Pedido ao faturamento

`examples/pedido_ao_faturamento.ron`. Processo comercial e fiscal feito no sistema: cada pessoa
fica no seu posto e não anda, como no trabalho remoto. Os pedidos dos clientes chegam com
valor, prioridade e prazo de faturamento de 2 dias corridos. O comercial cadastra o pedido
(10 % saem com algum dado errado); o financeiro analisa o crédito e, em 15 % dos casos, pede
garantia ou cadastro atualizado, que o cliente demora a mandar; o PCP confirma o estoque e a
data; o faturamento emite as notas em lote (15 pedidos, ou o que houver em 4 horas) e a
conferência fiscal manda as notas com erro para uma caixa de correção.

Experimentos típicos: faturar sem lote, conferir o cadastro na entrada do pedido (%C&A maior
no comercial), análise de crédito automática para clientes com limite aprovado.

## Revisão de contratos

`examples/revisao_de_contratos.ron`. Jurídico e contratos. As áreas da empresa pedem contratos
com valor, prioridade, complexidade e prazo de 10 dias corridos. O jurídico faz a triagem; os
contratos padrão (70 %) saem de uma minuta pronta, feita pelo assistente, e os específicos são
redigidos pelos advogados, que tocam até três contratos ao mesmo tempo
([multitarefa](multitarefa.md)). A minuta vai à contraparte, que demora a responder; a revisão
final do advogado sênior devolve as minutas com erro, a diretoria aprova e a assinatura
eletrônica fecha o caso.

A espera pela contraparte domina o lead time. Experimentos típicos: mais contratos com minuta
padrão (cláusulas pré-aprovadas), limitar a multitarefa dos advogados, alçada maior para
contratos de valor baixo sem passar pela diretoria.

## Licença de funcionamento

`examples/licenca_de_funcionamento.ron`. Serviço público: protocolo, processo e licença. Os
requerimentos chegam ao protocolo com prazo legal de 30 dias corridos. O protocolo recebe o
pedido (15 % com documento faltando); a análise documental percebe as faltas e emite uma
exigência ao requerente; as vistorias são agendadas em lote (8 por rota, ou o que houver em
5 dias úteis) e feitas por dois fiscais; o engenheiro dá o parecer e o secretário decide (90 %
deferidos e publicados no diário oficial, 10 % indeferidos, com uma conclusão para cada
desfecho).

Experimentos típicos: lista de documentos conferida no protocolo (menos exigências), rotas de
vistoria menores, publicação eletrônica no mesmo dia.

## Secretaria acadêmica

`examples/secretaria_academica.ron`. Pedidos de alunos com pico nas duas primeiras semanas do
semestre (taxa de chegada por período, num ciclo de 8 semanas; o modelo não tem aquecimento,
para o pico entrar na janela). O atendimento encaminha pelo tipo: declarações vão direto à
emissão; históricos passam pela conferência das notas, que espera o professor lançar a nota
que falta; ajustes de matrícula vão à coordenação, que defere ou pede um documento ao aluno, e
depois ao registro no sistema. Os documentos emitidos são assinados digitalmente.

Experimentos típicos: emissão automática das declarações, um atendente a mais só no pico,
prazo para os professores lançarem as notas.

## Pós-venda com interrupções

`examples/pos_venda_com_interrupcoes.ron`. Pedidos de troca e garantia chegam pelo portal; três
analistas em multitarefa analisam, pedem fotos ou nota ao cliente quando falta algo, e o
supervisor aprova. O telefone toca a cada 15 minutos, em média, e a ligação tira um analista do
caso (equipe com interrupções `Full`). Veja [Interrupções](interrupcoes.md) para a comparação
com a equipe sem interrupções.

## Reembolso de despesas (tutorial)

`examples/reembolso_atual.ron` e `examples/reembolso_futuro.ron`: o processo de cinco etapas
do [tutorial](tutorial.md), no estado atual e no futuro.

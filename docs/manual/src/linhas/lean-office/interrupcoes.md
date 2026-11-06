# Interrupções

Telefone, chat, uma urgência do gestor: no trabalho administrativo, a pessoa larga o caso que
está fazendo para atender e depois volta a ele. Na linha isso tem duas partes:

- a **equipe com interrupções**: o campo "Interrupções" da predefinição Equipe, que no arquivo
  é o `preempt` do `Dispatcher`. Com `Full`, um pedido mais urgente tira uma pessoa do caso
  que ela está trabalhando; com `Never` (o padrão), ele espera alguém ficar livre;
- a predefinição **Interrupção**: as chegadas das ligações (ou mensagens, ou urgências), o
  atendimento delas, com prioridade maior que a dos casos, e a conclusão delas.

```ron
(name: "analistas", kind: Dispatcher(team: ["analista_1", "analista_2"], preempt: Full)),
(name: "telefone", kind: Processor(capacity: 100, process: Exp(mean: 6.0),
    operators: Some(OperatorUse(team: "analistas", priority: Some(10)))), ...),
```

## O que acontece numa interrupção

Quando chega uma ligação e todas as pessoas estão ocupadas com trabalho de prioridade menor,
a equipe escolhe uma delas (a de trabalho menos prioritário; empate: a primeira da equipe) e:

1. o posto de trabalho do caso deixado para o relógio com o tempo que faltava e volta a
   esperar uma pessoa (estado "esperando a pessoa");
2. o caso volta a esperar a equipe, à frente dos casos de mesma prioridade;
3. a pessoa atende a ligação; quando termina, pega o próximo trabalho que espera a equipe, que pode ser o
   caso que ela deixou ou outro;
4. quem pegar o caso deixado continua de onde parou: o tempo que faltava, sem sortear de novo.

O trabalho sorteado no início continua sendo o touch time do caso; a interrupção aparece como
espera "na etapa, sem trabalho" e alonga o lead time. Uma pessoa com
[multitarefa](multitarefa.md) larga todos os casos abertos, atende a ligação sozinha e
depois retoma os casos. Transporte com o caso nas mãos e manutenção nunca são interrompidos.

As ligações não contam como casos nos indicadores (lead time, FPY, WIP, SLA): elas só tomam o
tempo das pessoas, que aparece na ocupação delas. No relatório de qualquer modelo, a equipe
mostra `preemptions.count`, quantas vezes alguém foi tirado do trabalho.

## Exemplo: pós-venda com interrupções

`examples/pos_venda_com_interrupcoes.ron` tem três analistas em multitarefa tratando pedidos de
troca e garantia e um telefone que toca a cada 15 minutos, em média, com ligações de 6
minutos. Com a equipe em `Full`, uma ligação fica de 10 a 20 minutos no sistema, em média
(quase só as que chegam perto do almoço ou do fim do expediente esperam mais); com `Never`, as
ligações esperam um analista ficar sem casos, o que com a multitarefa quase não acontece, e
passam horas esperando. Para comparar, copie o arquivo, troque `preempt: Full` por
`preempt: Never` e rode:

```bash
utsim linha lean_office relatorio examples/pos_venda_com_interrupcoes.ron --comparar sem_interrupcoes.ron --html comparacao.html
utsim run examples/pos_venda_com_interrupcoes.ron
```

O `utsim run` mostra o tempo das ligações (`telefone_fim`) e o número de interrupções
(`analistas`).

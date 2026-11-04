# Multitarefa

Uma pessoa pode ter vários casos abertos ao mesmo tempo, como o analista que começa uma
cotação, para no meio para responder outra e volta à primeira. No arquivo, é o campo
`multitask` da pessoa (um `Transporter`); na biblioteca, o campo "Casos abertos ao mesmo
tempo" das predefinições Pessoa e Equipe:

```ron
(name: "comprador_1", kind: Transporter(speed: 72.0,
    multitask: Some((max_open: 3, slice: Exp(mean: 15.0), switch: Const(5.0)))), ...)
```

- `max_open`: quantos casos a pessoa mantém abertos (o WIP pessoal). Com 1, ela trabalha
  exatamente como sem o campo.
- `slice`: quanto tempo ela fica num caso antes de passar ao próximo aberto.
- `switch`: o tempo perdido a cada troca (retomar o raciocínio, reabrir o sistema). A pessoa
  fica no estado "trocando de caso".

A pessoa trabalha os casos em rodízio. Enquanto ela está em outro caso, o trabalho do caso
deixado fica parado no posto (conta como espera "na etapa, sem trabalho"), então o touch time
de cada caso continua igual ao trabalho sorteado; o que cresce é o lead time. Quando um caso
termina e há outros abertos, ela paga uma troca e segue para o próximo; só fica livre para a
equipe quando fecha o último. A equipe só entrega um caso a mais a quem já está ocupado se
ninguém estiver livre.

Para a multitarefa aparecer, o posto de trabalho precisa ter mais mesas que pessoas
(`capacity`): com uma mesa por pessoa, os casos esperam na caixa de entrada e ninguém chega a
ter dois abertos. Um bom experimento é comparar o mesmo modelo com `max_open: 1` e com
`max_open: 3`: o lead time sobe com as trocas, e o relatório mostra a fração do tempo de cada
pessoa trocando de caso. No modelo pronto [Central de solicitações](modelos-prontos.md), os
atendentes do nível 1 trabalham assim.

No relatório de qualquer modelo, a pessoa com multitarefa tem `multitask.switches` (trocas),
`multitask.switching` (fração do tempo trocando) e `multitask.open` (casos abertos, média no
tempo). Os serviços de manutenção nunca entram no rodízio.

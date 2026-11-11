# Biblioteca

As 13 predefinições do Lean Office. Cada uma monta um ou mais objetos comuns do UTSim; a
coluna "Base no núcleo" diz quais, para quem quiser editar o `.ron` à mão. Nenhuma supõe um
lugar: a caixa de entrada pode ser uma bandeja, uma caixa de e-mail ou a lista de trabalho de um sistema, o
posto de trabalho pode ser uma mesa ou uma tela, e a pessoa só anda quando o posto fica longe
dela.

| Predefinição | Base no núcleo | Campos | Papel |
|---|---|---|---|
| `entrada` (Entrada de pedidos) | `Source` | intervalo, rótulos, horário | entrada |
| `caixa_entrada` (Caixa de entrada) | `Queue` com `order` | ordem (chegada, `Desc("prioridade")`, `Asc("prazo")`), limite | caixa de entrada |
| `posto` (Posto de trabalho) | `Processor` com `operators` | tempo, equipe, %C&A, mesas | trabalho |
| `aprovacao` (Aprovação) | `Processor` com `Weighted([aprova, devolve])` | tempo, aprovador, pesos, mesas (casos em análise ao mesmo tempo: um por aprovador) | aprovação |
| `verificacao` (Verificação) | `Processor` com `ByLabel("rota")` | tempo, equipe, fração percebida | verificação |
| `tramitacao` (Tramitação) | `Processor` de capacidade ilimitada, sem pessoa | tempo de passagem | tramitação |
| `espera_informacao` (Espera por informação) | `Processor` de capacidade ilimitada | tempo de resposta | espera por informação |
| `lote` (Agrupamento em lote) | `Queue` com `batch` | tamanho, espera máxima | lote |
| `reuniao` (Reunião ou comitê) | `Queue` (pauta) e `Processor` com horário | tempo por caso, horário, pesos | reunião |
| `equipe` (Equipe) | `Dispatcher` e duas pessoas | regra, horário, multitarefa, interrupções | equipe |
| `interrupcao` (Interrupção) | `Source`, `Processor` com prioridade e `Sink` | intervalo, duração, equipe, prioridade, horário | interrupção (não é caso; veja [Interrupções](interrupcoes.md)) |
| `pessoa` (Pessoa) | `Transporter` com perfil de pessoa | habilidades, horário, multitarefa | pessoa |
| `conclusao` (Conclusão) | `Sink` | | conclusão |

Na **aprovação**, na **verificação** e na **reunião**, a primeira ligação de saída é a que
segue e a segunda a que devolve: ligue nessa ordem. O posto de trabalho sorteia o erro na
chegada do caso (com probabilidade `1 − %C&A`); a verificação percebe uma fração dos erros,
zera o rótulo `erro` e manda o caso pela segunda saída.

## Colocar uma predefinição num modelo

O comando `utsim linha lean_office colocar` cria o arquivo se ele não existir e grava
`line: "lean_office"` no modelo:

```bash
utsim linha lean_office colocar processo.ron entrada --nome pedidos --campo 'intervalo=Exp(mean: 20.0)'
utsim linha lean_office colocar processo.ron caixa_entrada --posicao 2 0 --departamento Compras \
    --campo 'ordem=Desc("prioridade")'
utsim linha lean_office colocar processo.ron posto --posicao 4 0 --departamento Compras --campo ca=0.9
utsim linha lean_office colocar processo.ron equipe --posicao 4 2 --departamento Compras
utsim linha lean_office colocar processo.ron conclusao --posicao 6 0
```

Os valores dos campos são texto RON, como no arquivo do modelo. As ligações entre os objetos
continuam sendo feitas no editor ou no arquivo (`connections`); o comando avisa o que ainda
falta para o modelo ficar válido. O [tutorial](tutorial.md) monta um processo inteiro assim.

## Caixa de entrada por prioridade ou por prazo

Toda caixa de entrada aceita `order: Desc("rótulo")` (maior valor primeiro) ou
`order: Asc("rótulo")` (menor primeiro), com o rótulo numérico de cada caso. Empates e casos
sem o rótulo ficam na ordem de chegada, os sem rótulo depois dos outros. A ordem vale para a
saída da caixa; não use junto com `discipline: Lifo`. Para que a ordem tenha efeito, o posto
de trabalho seguinte deve ter poucas mesas (`capacity` igual ao número de casos trabalhados ao
mesmo tempo): os outros casos esperam na caixa de entrada, onde a ordem é aplicada.

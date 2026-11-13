# UTSim Desenvolvimento de Produto

O UTSim Desenvolvimento de Produto é um aplicativo da família UTSim com janela própria. Ele
organiza os requisitos do produto, as fases do desenvolvimento com os portões, a estrutura do
produto com as variantes e os roteiros, e gera a partir disso um modelo que o UTSim simula.
Funciona sem internet e sem servidor. Esta é a primeira versão (MVP).

Pela linha de comando, `utsim-produto` abre a janela e `utsim-produto --utsim-abrir=<pasta>`
abre direto um projeto.

## O projeto

Cada projeto é uma pasta com o arquivo `produto.json`. A pasta pode ficar no Google Drive.
Tudo o que você muda é gravado sozinho, menos de um segundo depois. O botão **Gravar
revisão**, embaixo da barra lateral, guarda uma cópia que nunca muda em
`revisoes/r0001.json`, `r0002.json` e assim por diante: use antes de cada portão.

Na abertura, escreva o seu nome (ele vai no histórico e nas decisões), depois abra ou crie um
projeto. Um projeto novo já vem com cinco fases: Ideia, Conceito, Projeto, Validação e
Lançamento.

## Requisitos

Cada requisito tem tipo (cliente, técnico, regulatório, segurança, qualidade), código
automático (`RC-001`, `RT-001`...), prioridade, estado, fonte, descrição e critério de
aceitação. Um requisito novo entra abaixo do escolhido, e o campo "Abaixo de" muda a
hierarquia.

- **Especificações** dizem o requisito em número: alvo, LIE, LSE, unidade e, se quiser, a
  característica do produto ligada.
- **Verificações** dizem como cada especificação se prova (teste, inspeção, análise,
  demonstração) e o resultado.
- **Registrar alteração** sobe a versão e guarda quem, quando e o quê.

Um requisito aprovado fica travado. Ele só muda por um **pedido de mudança**: abra o pedido,
escreva o motivo e o impacto, marque os requisitos afetados e aprove. Aprovar sobe a versão
de cada requisito marcado e volta cada um para "proposto", para ser revisto.

A tela **Rastreabilidade** mostra cada requisito com as especificações e as verificações, e
em amarelo o que falta especificar ou verificar.

A **Casa da qualidade** cruza as necessidades do cliente (com peso de 1 a 5) com as
características técnicas. Clique numa célula para trocar a relação (nenhuma, fraca, média,
forte). A última linha dá a importância de cada característica técnica.

## Fases e portões

A linha de cima mostra as fases, com a atual em destaque e o quanto de cada uma está pronto.
Cada fase tem entregas, checklist e aprovadores, que você pode mudar à vontade, inclusive
criar e tirar fases. No portão da fase atual, o app lista o que falta. Com tudo pronto, a
decisão "Seguir" passa para a próxima fase. "Segurar" registra sem mudar de fase, "Voltar"
volta uma fase e "Parar" encerra o projeto. Toda decisão fica na linha do tempo, com data,
quem e a justificativa.

## Estrutura, variantes e roteiros

Na **Estrutura**, crie o produto e os itens (montagem, peça, matéria-prima, comprado) e
monte a lista de materiais com a quantidade por unidade. O app recusa um item dentro dele
mesmo. A explosão mostra todos os níveis com a quantidade por unidade do produto.

Nas **Variantes**, cada variante parte de um produto e troca, tira ou acrescenta componentes.
A demanda diz quanto de cada variante entra na simulação.

Nos **Roteiros**, cada item tem as operações em ordem, com o recurso (o posto ou a máquina),
a capacidade, o tempo de processo e o setup como distribuição. Uma operação pode ter tempo
próprio em uma variante.

Nas **Características**, cada medida tem nominal, LIE, LSE, unidade, classe e a operação onde
nasce. O desvio é o do processo esperado; zero quer dizer um oitavo da tolerância (Cp 1,33).

## Para a simulação

A tela mostra a linha que os roteiros pedem: um recurso por linha, com o tempo médio de cada
variante. Os recursos seguem a ordem das operações, do componente mais fundo ao produto.

- **Gerar modelo** grava um `.ron` novo: uma chegada com a mistura das variantes, uma fila e
  um processador por recurso, os tempos de cada variante, os setups e as características nos
  nós, com LIE e LSE. O UTSim abre esse arquivo como qualquer modelo, e o UTSim Qualidade lê
  as características do nó.
- **Atualizar modelo** grava os tempos, as capacidades, os setups e as características num
  modelo que já existe, nos processadores com o mesmo nome do recurso. O leiaute, as ligações
  e o resto ficam como estão.

Nesta versão a linha é em série. Uma variante que não passa por um recurso tem tempo zero
nele, e uma variante que passa duas vezes pelo mesmo recurso conta só a primeira operação (o
app avisa).

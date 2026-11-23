# UTSim Produto

O UTSim Produto (desenvolvimento de produto) é um aplicativo da família UTSim com janela própria. Ele
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

Na lista, a faixa colorida diz o tipo e a pílula, o estado. Embaixo, à direita, ficam o
número de especificações e quantas verificações passaram. **Arraste** um requisito para
cima de outro para pô-lo abaixo dele; com **Shift**, ele fica logo antes, no mesmo nível; na
faixa que aparece no topo durante o arrasto, ele sobe para o topo.

## Casa da qualidade

A casa é desenhada inteira e se edita clicando nela:

- **Matriz.** Cada clique numa célula troca a relação: nenhuma, fraca (△, 1), média (○, 3),
  forte (●, 9). O botão direito limpa a célula.
- **Telhado.** Cada losango cruza duas características técnicas: ++ e + (em verde) quando
  melhorar uma ajuda a outra, − e −− (em vermelho) quando atrapalha. Os conflitos aparecem
  em vermelho embaixo da casa.
- **Sentido.** A seta acima de cada característica diz se ela deve aumentar, ficar no alvo
  ou diminuir.
- **Peso.** O número ao lado de cada necessidade (1 a 5): clique soma, botão direito
  subtrai.
- **Avaliação competitiva.** À direita, a nota de 1 a 5 do nosso produto e de cada
  concorrente em cada necessidade, com a linha de cada um. Escolha em "Avaliando" de quem é a
  nota e clique na escala; "Meta" marca a nota que queremos (o losango branco).
- **Porão.** O alvo, a importância (soma de peso × relação), a importância relativa em
  barras, a ordem (a primeira em destaque) e a dificuldade técnica (clique nos pontinhos).

Clicar numa necessidade ou numa característica abre embaixo o quadro para editar o texto, a
unidade, o alvo e a dificuldade. **Arraste** o nome de uma necessidade para cima ou para baixo,
ou o de uma característica técnica para os lados, para mudar a ordem: a linha colorida mostra onde
ela entra, e a relação, o telhado e as notas vão junto. Uma necessidade sem relação média ou forte ganha um "!"
amarelo; uma característica sem nenhuma relação fica em amarelo. "Trazer requisitos do
cliente" cria uma linha por requisito do cliente, e "Ver um exemplo" monta uma casa pronta.

## Fases e portões

A linha de cima mostra as fases em setas: as feitas em verde, com visto; a atual na cor do
app; as outras em contorno. Cada uma mostra o andamento (quanto das entregas, do checklist e
das aprovações está pronto) e, embaixo, o losango do portão com a cor da última decisão.
Arraste uma fase que ainda não começou para mudar a ordem.

Um projeto novo escolhe o modelo de fases: o padrão ou o **APQP** (planejar e definir,
projeto do produto, projeto do processo, validação do produto e do processo, retroalimentação
e melhoria), cada fase com o seu checklist. Antes da primeira decisão de portão dá para trocar
de modelo na própria tela. Cada fase tem entregas, checklist e aprovadores, que você pode
mudar à vontade, inclusive criar e tirar fases. No portão da fase atual, o app lista o que falta. Com tudo pronto, a
decisão "Seguir" passa para a próxima fase. "Segurar" registra sem mudar de fase, "Voltar"
volta uma fase e "Parar" encerra o projeto. Toda decisão fica na linha do tempo, com data,
quem e a justificativa.

O botão **Relatório do portão**, embaixo de cada fase, grava o relatório daquela fase na
pasta `relatorios` do projeto, em HTML (abre no navegador) e em PDF: objetivo e datas, o que
ainda falta no portão, as entregas, o checklist, os aprovadores com a situação da assinatura,
as decisões tomadas naquele portão e uma linha por aprovador para assinar no papel. Cada
clique grava um arquivo novo, com a data e a hora no nome.

## Estrutura, variantes e roteiros

Na **Estrutura**, crie o produto e os itens (montagem, peça, matéria-prima, comprado) e
monte a lista de materiais com a quantidade por unidade. **Arraste** um item para cima de
outro na lista, ou para a faixa "Solte aqui" do item aberto, para pô-lo como componente. O app recusa um item dentro dele
mesmo. A explosão mostra todos os níveis com a quantidade por unidade do produto.

Nas **Variantes**, cada variante parte de um produto e troca, tira ou acrescenta componentes.
A demanda diz quanto de cada variante entra na simulação.

Nos **Roteiros**, cada item tem as operações em ordem, com o recurso (o posto ou a máquina),
a capacidade, o tempo de processo e o setup como distribuição. Uma operação pode ter tempo
próprio em uma variante.

Nas **Características**, cada medida tem nominal, LIE, LSE, unidade, classe e a operação onde
nasce. O desvio é o do processo esperado; zero quer dizer um oitavo da tolerância (Cp 1,33).
A coluna **Cpk** mostra o último estudo de capacidade que voltou do UTSim Qualidade: verde de
1,33 para cima, amarelo de 1,00 a 1,33 e vermelho abaixo de 1,00. Passe o ponteiro para ver o
histórico com o Cp, o Cpk, o Pp e o Ppk.

## Para a simulação

A tela desenha a linha que os roteiros pedem, da chegada à saída, com a carga estimada de
cada recurso (tempo médio pesado pela demanda, sobre a capacidade e o intervalo entre
chegadas) e o gargalo em destaque. Abaixo, a tabela com o tempo médio de cada variante. Os recursos seguem a ordem das operações, do componente mais fundo ao produto.

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

## Com os outros apps

- **Do simulador para o produto.** O modelo gerado ou atualizado pelo app guarda de que projeto
  e revisão veio. Aberto a partir de um nó (`utsim-produto --utsim-abrir=<modelo.ron>
  --utsim-no=<nó>`, ou um pedido `--utsim-pedido` com o modelo e o nó), o app abre esse
  projeto na tela Roteiros, no item cujo roteiro passa pelo recurso do nó. Um pedido que pede
  resposta é respondido quando o app fecha: concluído, ou o que impediu.
- **Das características para a Qualidade, e de volta.** Na tela Características, o botão
  "Qualidade" de uma característica medida numa operação abre o UTSim Qualidade no
  processador dessa operação, no último modelo gerado ou atualizado, pedindo o estudo de
  capacidade dela. Enquanto a Qualidade está aberta, a coluna Cpk diz "aguardando…". Ao
  fechar a Qualidade, o Cp e o Cpk voltam para a característica (mesmo que o Desenvolvimento
  de Produto tenha sido fechado e aberto no meio). Sem modelo ainda, o botão fica apagado e
  diz o que fazer.
- **Modelo ligado.** A tela Para a simulação mostra o modelo ligado (o último gerado ou
  atualizado). Quando a estrutura, os roteiros ou as características mudam depois, ela avisa
  que o modelo ficou para trás e oferece "Atualizar o modelo ligado".
- **Datas.** Os campos de data aceitam `AAAA-MM-DD` ou `DD/MM/AAAA` e ficam com a borda
  vermelha quando a data não existe ou, no fim de uma fase, vem antes do início.

## Trabalho em equipe

Várias pessoas podem editar o mesmo projeto ao mesmo tempo, cada uma no seu computador, com
a pasta do projeto no Google Drive (ou outra pasta sincronizada):

- Cada gravação vira um arquivo novo na pasta `diario`, com as mudanças daquela pessoa,
  assinado com a chave do computador dela. Ninguém escreve por cima do arquivo de outra
  pessoa, então a sincronização nunca cria cópias em conflito.
- A cada poucos segundos o app junta as gravações de todos. Duas pessoas mexendo em coisas
  diferentes, ou em campos diferentes da mesma coisa, ficam as duas; no mesmo campo, vale a
  gravação mais recente. Dois requisitos criados ao mesmo tempo com o mesmo código ganham
  códigos diferentes sozinhos.
- A tela **Equipe › Diário** mostra quem grava neste projeto (com a chave de cada um), se
  tudo confere e a linha das gravações, com o que cada uma mudou. Uma gravação mexida fora do
  app, ou com assinatura que não confere, fica de fora e aparece em vermelho.
- A chave fica no perfil de cada pessoa (`produto_chave_assinatura.txt`, ao lado da
  configuração), nunca na pasta do projeto.

Um projeto feito numa versão anterior ganha o diário na primeira vez que é aberto. Depois
disso, a versão anterior do app não o abre mais: atualize pelo launcher.

### Assinaturas dos portões

Na tela Fases e portões, um aprovador que marcou "aprovou" e tem o mesmo nome escrito na
abertura vê o botão **Assinar**: a aprovação fica selada com a chave dele. Quem decide um
portão sela a decisão do mesmo jeito, e a linha do tempo mostra "assinada". Se a fase mudar
depois (entregas, checklist, objetivo), a assinatura continua lá, em amarelo: "mudou depois".

A assinatura revela uma alteração feita fora do app, mas não a impede, e não é assinatura
digital com valor legal.

## Atalhos e exemplo

- **Ctrl+S** grava agora; **Ctrl+Shift+S** grava uma revisão.
- **Ctrl+1** a **Ctrl+9** trocam de tela, na ordem da barra lateral.
- Nas telas vazias, "Carregar o exemplo da bicicleta" preenche o projeto com requisitos,
  estrutura com duas variantes, roteiros, características e a casa da qualidade, para conhecer
  o app sem digitar nada.

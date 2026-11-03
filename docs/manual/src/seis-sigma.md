# UTSim Seis Sigma (gerenciador de projetos Lean Seis Sigma)

O UTSim Seis Sigma é o aplicativo da família UTSim para conduzir projetos de melhoria Lean
Seis Sigma: termo de abertura, fases e tollgates, quadro de ações, riscos, reuniões,
cronograma, ganhos e as ferramentas de cada fase. É um programa de janela própria (não abre o
navegador), funciona sem internet e não precisa de servidor: a equipe trabalha numa pasta
compartilhada do Google Drive. O nome ainda é provisório.

Já funcionam a **fundação** (etapa L0 do `docs/plano_lean_seis_sigma.md`: o formato do
projeto, o trabalho em equipe pela pasta compartilhada, a visão geral, o termo de abertura e o
quadro de ações) e as **ferramentas qualitativas** (etapa L2: VOC e árvore CTQ, SIPOC,
Pareto, plano de coleta, Ishikawa, 5 porquês, matriz causa-efeito, FMEA com a tabela de ação
prioritária, planos de controle e de resposta e A3). As outras telas aparecem na navegação
com o que vão trazer e chegam nas próximas etapas.

## Instalar e abrir

O UTSim Seis Sigma é um produto próprio no launcher (veja [Launcher](launcher.md)), com o
nome "UTSim Seis Sigma". Escolha-o na lista, instale e clique em "Abrir". Ele não precisa do
editor de simulação instalado.

Pela linha de comando, `utsim-seis-sigma` abre a janela e `utsim-seis-sigma "pasta.lss"` abre
direto um projeto.

## Primeiro uso: quem é você

Na primeira vez, o programa pede o seu nome e e-mail. Eles vão em cada alteração que você
fizer, para a equipe saber quem mudou o quê e quando. Ficam guardados só no seu computador,
na sua pasta de usuário, e nunca dentro dos projetos:

- Windows: `%APPDATA%\UTSim\seis-sigma\config.json`;
- Linux: `~/.config/utsim/seis-sigma/config.json`.

Ali também ficam a lista de projetos recentes e o tema (sistema, claro ou escuro). "Trocar",
na tela inicial, corrige o nome ou o e-mail.

Não há senha: sem servidor, não há como conferir quem é quem. Qualquer pessoa com acesso à
pasta pode gravar no projeto em nome de outra. Para aprovações com valor de registro, o plano
prevê assinatura das aprovações com chave por pessoa (etapa P7), que mesmo assim não equivale
a uma assinatura digital com validade jurídica.

## Criar, abrir e importar projetos

Na tela inicial:

- **Novo projeto**: título, pasta onde criar e abordagem (DMAIC, DMADV, kaizen rápido, A3,
  PDCA ou ação direta). O programa cria a pasta `<título>.lss` com as fases padrão da
  abordagem (no DMAIC: Definir, Medir, Analisar, Melhorar e Controlar) e registra você na
  equipe. Caracteres que o Windows não aceita em nomes de pasta viram `-`.
- **Abrir uma pasta de projeto**: o caminho de uma pasta `.lss`.
- **Recentes**: um clique abre. Projetos cuja pasta sumiu aparecem desabilitados.
- **Importar um arquivo .lssz**: desembrulha o projeto numa pasta à sua escolha e o abre.

Por enquanto os caminhos são digitados (ou colados do Explorador de Arquivos); a janela de
escolha de pasta chega numa etapa seguinte.

## Trabalho em equipe pela pasta compartilhada

Crie o projeto dentro de uma pasta compartilhada do Google Drive da equipe, sincronizada pelo
Google Drive para computador. Cada pessoa abre a mesma pasta no seu computador. O programa
só lê e grava arquivos locais; quem leva as mudanças de um computador para o outro é o
próprio Drive.

- **Nada é sobrescrito.** Cada alteração vira um arquivo novo no diário do projeto
  (`ops/`). Por isso a sincronização nunca perde a alteração de ninguém, e o diário é também
  o histórico completo.
- **As mudanças dos outros chegam sozinhas.** O programa olha a pasta a cada 2 segundos e
  aplica o que chegou. O tempo total depende do Drive, que costuma levar de segundos a alguns
  minutos. Não é edição simultânea em tempo real.
- **Funciona desconectado.** Sem internet, você continua trabalhando; o Drive envia e recebe
  quando a conexão voltar, e o resultado é o mesmo em todos os computadores.
- **Presença.** No alto da janela aparecem as pessoas com o projeto aberto (passe o mouse
  para ver em que tela estão). Se alguém está editando a mesma ação ou o mesmo termo que
  você, aparece o aviso "Fulano está editando isto agora". É um aviso, não uma trava.
- **Arquivos ainda chegando.** Enquanto o Drive copia um arquivo, ele pode estar incompleto.
  O programa o deixa de lado, avisa na barra de baixo ("arquivos ainda chegando pela
  sincronização") e o lê quando ficar completo. O mesmo aviso aparece para alterações feitas
  por uma versão mais nova do programa: atualize pelo launcher.
- **Cópias em conflito do Drive** (como `x (1).json`) são reconhecidas e ignoradas, porque
  trazem uma alteração que o projeto já tem.

### Conflitos

Quando duas pessoas mudam o mesmo campo sem ver a mudança uma da outra (por exemplo, as duas
editaram o título da mesma ação enquanto o Drive ainda não tinha sincronizado), vale a
alteração mais recente pela ordem do diário, e a outra não se perde: o programa mostra "1
conflito a revisar" no alto e a tela **Conflitos** lista cada caso com os dois valores e quem
escreveu cada um.

- **Manter o atual**: o conflito sai da lista e o valor fica como está.
- **Restaurar o outro**: volta o valor substituído (como uma alteração nova) e o conflito sai
  da lista.

A regra vale também para quem digitava enquanto a mudança do colega chegava: o programa
lembra a versão que você viu quando começou a digitar, e se ela mudou nesse meio tempo o
caso vai para a lista.

## Telas do projeto

A navegação à esquerda tem a **Carteira** e as telas do projeto.

- **Visão geral**: as fases com a atual destacada, o andamento das ações (concluídas e
  atrasadas), os riscos abertos e os altos, os ganhos por estado, as reuniões, o prazo com os
  dias que faltam, os conflitos e a pasta do projeto.
- **Termo de abertura**: título, processo, área, declaração do problema, meta, linha de base,
  prazo, escopo dentro e fora e caso de negócio. Cada campo é gravado quando você sai dele.
  Datas em `DD/MM/AAAA`.
- **Fases e tollgates**: as fases da abordagem e o estado de cada uma. Entregáveis,
  checklist do tollgate e decisões dos aprovadores chegam na etapa L1.
- **Ações**: o quadro kanban, descrito abaixo.
- **Ferramentas**: os entregáveis das fases, descritos em [Ferramentas](#ferramentas).
- **Riscos**, **Reuniões**, **Cronograma**, **Ganhos** (etapa L1) e **Dados** (etapa L3)
  mostram o que vão trazer. Em **Dados** já está a exportação `.lssz`.

### Quadro de ações

As colunas são A fazer, Fazendo, Bloqueado, Verificar e Feito.

- Escreva o que fazer no campo do alto e tecle Enter (ou "Adicionar"): o cartão entra no fim
  de "A fazer".
- Arraste um cartão pela alça `≡` para outra coluna. Clique no título para abrir o cartão à
  direita.
- No cartão: os campos do 5W2H (o quê, por quê, onde, quando, quem, como, quanto), a coluna e
  as setas para mudar de coluna ou de posição. "Excluir" tira o cartão do quadro (a exclusão
  também fica no diário).
- Cartões com prazo vencido e fora de "Feito" aparecem com a data em destaque, e entram na
  lista de atrasadas da visão geral.

Cartões criados pelas ferramentas (a ação recomendada de uma linha do FMEA, um sinal do plano
de resposta) mostram a origem no painel do cartão.

Limites por coluna, raias, verificação de eficácia e a visão "minhas ações" de toda a carteira
chegam na etapa L1.

## Ferramentas

A tela **Ferramentas** reúne as ferramentas qualitativas das fases (etapa L2 do plano). Cada
ferramenta é um **entregável** de uma fase: escolha a ferramenta em "Novo entregável" e
clique em "Criar". Ela entra na fase onde costuma ser usada no DMAIC (VOC, SIPOC, Pareto e
A3 no Definir; plano de coleta no Medir; Ishikawa, 5 porquês, matriz causa-efeito e FMEA no
Analisar; planos de controle e de resposta no Controlar) e pode ser mudada de fase no alto
do entregável. Um projeto pode ter mais de um entregável da mesma ferramenta (dois Paretos,
por exemplo).

No alto de cada entregável ficam o nome, a fase, o estado (em elaboração ou concluído) e
"Excluir". Os entregáveis aparecem em linha, agrupados pela fase; o concluído leva `✔`.

Como no quadro de ações, cada célula é um campo próprio no diário: duas pessoas editando
linhas diferentes do mesmo FMEA não se atrapalham, e duas pessoas mudando a mesma célula sem
ver a mudança uma da outra geram um conflito na tela **Conflitos**. Em todas as listas, `⬆` e
`⬇` mudam a posição e `✖` exclui o item (com os itens abaixo dele, numa árvore). Números
aceitam vírgula ou ponto (`12,5`).

### VOC e árvore CTQ

- **Voz do cliente**: cada fala do cliente, nas palavras dele, com a fonte (SAC, pesquisa,
  visita) e o segmento. Na coluna "Necessidade", agrupe a fala numa necessidade.
- **Árvore CTQ**: cada necessidade (com a classe de Kano opcional: obrigatório, desempenho
  ou atrativo) se desdobra em direcionadores, e cada direcionador em CTQs mensuráveis, com
  definição operacional, unidade, LIE, LSE e meta.
- **Como ler**: ao lado de cada CTQ aparecem as pendências (sem unidade, sem nenhum limite,
  LIE maior ou igual ao LSE, meta fora dos limites); embaixo, o total de pendências da
  árvore, contando também falas sem necessidade, necessidades sem direcionador e
  direcionadores sem CTQ. "Árvore completa" quer dizer que cada CTQ tem unidade e ao menos
  um limite.

### SIPOC

Cinco colunas, da esquerda para a direita: fornecedores, entradas, processo (as etapas,
numeradas), saídas e clientes. Escreva no campo de cada coluna e tecle Enter. O programa
avisa quando o processo não tem de 5 a 7 etapas (o usual num SIPOC) e quando há coluna
vazia. O SIPOC gerado do modelo de simulação chega na etapa L4.

### Pareto

- **Contagens por categoria**: uma linha por categoria com a contagem. Com "Ponderar pelo
  custo unitário", cada contagem é multiplicada pelo custo e o Pareto passa a ser de custo.
- **Tabela de ocorrências**: cole uma ocorrência por linha (de uma planilha, por exemplo);
  linhas iguais são somadas sem diferenciar maiúsculas. Para várias de uma vez, termine a
  linha com `;quantidade` (`Rebarba;12`).
- "Juntar em Outros abaixo de x %" junta as categorias pequenas numa só, no fim.
- **Como ler**: a tabela traz o valor, o percentual e o acumulado de cada categoria, em
  ordem decrescente. As **poucas vitais** são as categorias necessárias para chegar a 80 %
  do total, contando a que cruza os 80 %. No gráfico, as barras são o percentual de cada
  categoria (as vitais na cor de destaque), a linha é o acumulado e a linha horizontal marca
  os 80 %. O acumulado sempre termina em 100 %.

### Plano de coleta de dados

Uma linha por medida: Y ou X, nome, definição operacional (o que conta e como medir, para
todos medirem igual), tipo de dado (contínuo, atributo ou contagem), unidade,
estratificação, tamanho da amostra, frequência, subgrupo, quem coleta, onde e formulário. A
coluna "Falta" mostra o que ainda impede a coleta: nome, definição operacional, unidade
(dado contínuo) ou amostra e frequência.

### Ishikawa

- Escreva o **efeito** no alto: ele vai na cabeça do peixe.
- Cada categoria dos 6M (mão de obra, máquina, método, material, medição, meio ambiente)
  abre a lista das suas causas; "nova categoria" cria uma categoria da equipe, além dos 6M.
- Cada causa tem o estado (aberta, provável, verificada ou descartada) e a evidência que o
  decidiu. "+ subcausa" desce um nível, sem limite de profundidade.
- **Como ler o diagrama**: cada espinha é uma categoria (metade em cima, metade embaixo) com
  as suas primeiras causas; as verificadas aparecem na cor de destaque, as prováveis em
  laranja e as descartadas apagadas. As subcausas ficam na lista abaixo do desenho.
- Se duas pessoas moverem causas ao mesmo tempo e uma ficar embaixo da outra (um ciclo), o
  programa mostra a primeira como causa principal e avisa; nada some.

### 5 porquês

- Cada **cadeia** começa de um problema, escrito à mão ou tirado de uma causa de um Ishikawa
  do projeto ("Nova cadeia a partir de uma causa do Ishikawa" ou o campo "Causa do
  Ishikawa" da cadeia).
- "+ por quê (novo ramo)" acrescenta um primeiro porquê; "+ por quê" ao lado de um porquê
  acrescenta o seguinte. Clicar duas vezes no mesmo porquê abre dois ramos.
- Cada porquê tem a evidência. Marque "causa raiz" no porquê que a equipe concluiu.
- **Como ler**: as causas raiz candidatas são os porquês marcados; sem marcação, o último
  porquê de cada ramo. Cinco níveis são a referência usual, não uma regra.

### Matriz causa-efeito

- **Saídas (Y)**: cada saída com o peso para o cliente, de 1 a 10.
- **Entradas (X)**: cada entrada recebe, para cada saída, a nota de correlação 0
  (nenhuma), 1 (fraca), 3 (média) ou 9 (forte). Célula em branco conta como 0.
- **Como ler**: o total de uma entrada é a soma de peso × nota sobre as saídas; a coluna
  "Ordem" dá a posição da entrada (1º é o maior total; empate fica na ordem de entrada). A
  linha "Soma das notas" mostra quanto cada saída é explicada pelas entradas listadas. O
  Pareto dos totais mostra as entradas que somam até 80 % da pontuação: são as primeiras a
  investigar no Analisar ou a levar para o FMEA.

### FMEA de processo

Cada linha é um modo de falha de uma etapa (ou função) do processo:

| Coluna | O que é |
|---|---|
| Etapa / função | onde no processo |
| Modo de falha | como a etapa deixa de cumprir a função |
| Efeito | o que o cliente (interno ou externo) sente |
| S | severidade do efeito, 1 a 10 |
| Causa | por que o modo de falha acontece |
| O | ocorrência da causa, 1 a 10 |
| Controle de prevenção | o que evita a causa |
| Controle de detecção | o que detecta a causa ou o modo antes de chegar ao cliente |
| D | detecção, 1 a 10 (10: não há como detectar) |
| RPN | S × O × D, de 1 a 1000 |
| AP | ação prioritária: alta, média ou baixa, pela tabela do projeto |
| Ação recomendada | o que fazer |
| Cartão | "Criar cartão" leva a ação ao quadro; depois, "Abrir cartão" mostra a coluna onde ela está |
| Ação tomada, S', O', D' | o que foi feito e as notas revistas depois da ação |
| RPN antes → depois, AP depois | o efeito da ação (em negrito quando melhorou) |

- **Escalas da empresa**: em "Escalas da empresa", descreva o que cada nota de S, O e D quer
  dizer na sua empresa. As descrições ficam no FMEA para toda a equipe.
- **Ação recomendada → cartão**: o cartão entra no fim de "A fazer" com a ação como título,
  o modo de falha e a causa no "por quê", e a origem "ação recomendada do FMEA" no painel do
  cartão. Uma linha tem um cartão; se o cartão for excluído, dá para criar outro.
- **Revisão**: preencha S', O' e D' depois da ação. Nota não revista vale a original. O
  RPN e a AP depois da ação aparecem ao lado dos de antes.
- **Características críticas**: linhas com S a partir de 9 (o valor pode ser mudado em
  "Característica crítica a partir de S") são marcadas com `◆`. Cada uma precisa de uma
  linha no plano de controle do projeto; o FMEA e o plano de controle avisam quantas
  faltam. Com S' preenchido, vale a severidade revista.
- **Como ler o RPN e a AP**: o RPN ordena, mas trata igual um 10 × 2 × 5 e um 2 × 10 × 5. A AP
  corrige isso dando peso maior à severidade: priorize as linhas de AP alta, depois as de
  AP média, e use o RPN só para desempatar.

#### A tabela de ação prioritária do UTSim

O programa já vem com uma tabela de AP para as 1000 combinações de S, O e D. Ela é própria do
UTSim, escrita a partir da lógica pública do método (a severidade pesa primeiro, depois a
ocorrência, depois a detecção); não é cópia da tabela de nenhum manual. As regras, por faixa
de severidade (vale a prioridade mais alta entre as regras atendidas; nenhuma: baixa):

| Severidade | Alta | Média |
|---|---|---|
| 9–10 (segurança, requisito legal) | O ≥ 4; ou O 2–3 com D ≥ 5 | O 2–3 com D ≤ 4; ou O = 1 com D ≥ 7 |
| 7–8 | O ≥ 6; ou O 4–5 com D ≥ 7 | O 4–5 com D ≤ 6; ou O 2–3 com D ≥ 5 |
| 4–6 | O ≥ 8 com D ≥ 5 | O ≥ 6 (fora a alta); ou O 4–5 com D ≥ 7 |
| 2–3 | nunca | O ≥ 8 com D ≥ 7 |
| 1 (sem efeito) | nunca | nunca |

Cada regra é da forma "S ≥ a, O ≥ b e D ≥ c". Por isso a prioridade **nunca cai quando S, O
ou D sobe**: piorar qualquer nota nunca baixa a prioridade. Os testes conferem isso nas 1000
combinações, contra todas as combinações piores. A tabela tem 334 combinações altas, 216
médias e 450 baixas.

#### Tabela da empresa

A empresa que tem direito de usar outra tabela (a de um manual que ela licenciou, por
exemplo) pode importá-la. No FMEA, escreva o caminho de um arquivo CSV, dê um nome à tabela
e clique em "Importar tabela da empresa". O arquivo tem uma linha por combinação:

```
S,O,D,AP
1,1,1,L
1,1,2,L
...
10,10,10,H
```

- Separador vírgula, ponto e vírgula ou tabulação; cabeçalho opcional; AP `H` (alta), `M`
  (média) ou `L` (baixa), maiúsculas ou minúsculas.
- As **1000 combinações** precisam estar lá, cada uma uma vez só. Se algo estiver errado, a
  tabela não é importada e a tela lista os problemas com o número da linha (coluna faltando,
  nota fora de 1 a 10, AP inválida, combinação repetida, combinações faltando).
- A tabela importada vale para **todos os FMEAs do projeto** e para toda a equipe (ela fica
  no diário, como qualquer alteração). Se ela tiver combinações em que a prioridade cai
  quando uma nota sobe, a tela avisa, mas aceita: a tabela é da empresa.
- "Salvar a tabela atual em CSV" grava a tabela em uso no caminho escrito: serve de modelo
  para montar a da empresa. "Voltar à tabela do UTSim" desfaz a importação.

### Plano de controle

Uma linha por característica: processo ou etapa, característica, tipo (produto ou processo),
especificação, método de medição, tamanho e frequência da amostra, método de controle (carta,
checklist, poka-yoke), responsável e plano de reação. Na coluna "Linha do FMEA", ligue a linha
à linha do FMEA que ela controla e marque se é característica crítica.

- **Como ler**: embaixo, o programa lista as características críticas do FMEA (S alto) que
  ainda não têm linha ligada a elas; "Criar a linha" cria a linha já ligada, com a etapa e o
  modo de falha. Quando a lista fica vazia, o plano cobre todas: é a verificação do
  tollgate do Controlar.

### Plano de resposta

Uma linha por sinal (regra de carta violada, limite de especificação, alarme): ação imediata,
contenção, quem acionar e prazo. "Sinal ocorreu: abrir cartão" cria um cartão em "A fazer"
com o sinal e a ação imediata no título, a contenção no "como", quem acionar no "onde", o
prazo (se for uma data) e a origem "sinal do plano de resposta". Cada vez que o sinal ocorre,
um cartão novo; "Abrir cartão" leva ao último.

### A3

Uma página com o projeto inteiro em sete blocos: contexto, condição atual, objetivo, análise
das causas, contramedidas, plano de ação e acompanhamento. O conteúdo é montado na hora a
partir do que o projeto já tem: problema, caso de negócio, meta, linha de base e prazo do
termo de abertura; as poucas vitais dos Paretos; os CTQs com unidade e limites; as etapas do
SIPOC; as causas verificadas dos Ishikawas; as causas raiz dos 5 porquês; as três entradas de
maior total das matrizes causa-efeito; as linhas de AP alta dos FMEAs com a ação; os cartões
do quadro com coluna, prazo e responsável; as linhas do plano de controle, os sinais do plano
de resposta e as características críticas sem controle. Cada bloco tem uma nota da equipe,
para o que não está em nenhum entregável. A impressão em PDF chega na etapa L5.


## A pasta do projeto

```
Melhoria da linha 2.lss/
  projeto.json                 identificação e formato do projeto
  ops/AAAA/MM/<id>-<pessoa>.json   uma alteração por arquivo
  instantaneos/                estado completo até certo ponto (acelera a abertura)
  dados/                       conjuntos de dados importados (etapa L3)
  anexos/<sha256>.<ext>        anexos guardados pelo conteúdo
  modelos/                     cópia do modelo UTSim ligado (etapa L4)
  presenca/<pessoa>.json       quem está com o projeto aberto
```

- Não edite nem apague arquivos dessas pastas à mão. Apagar uma alteração do diário faz o
  projeto voltar atrás nos computadores que abrirem depois.
- Os instantâneos são gravados por qualquer pessoa a cada 500 alterações; dois instantâneos
  feitos ao mesmo tempo são equivalentes.
- Fora da pasta compartilhada, o programa guarda um cache do estado do projeto
  (`%LOCALAPPDATA%\UTSim\seis-sigma\cache\` no Windows, `~/.cache/utsim/seis-sigma/` no
  Linux) para abrir mais rápido. Pode ser apagado a qualquer momento.
- Cada arquivo tem `"formato": N`. Um projeto num formato mais novo que o programa dá a
  mensagem "projeto no formato N; esta versão do UTSim Seis Sigma lê até M": atualize pelo
  launcher.

## Levar o projeto para quem não está na pasta

Em **Dados**, "Exportar .lssz" grava o projeto inteiro num arquivo só (por padrão ao lado
da pasta do projeto). O arquivo leva o diário, os instantâneos, os dados e os anexos, e não
leva a presença. Quem recebe usa "Importar um arquivo .lssz" na tela inicial. O projeto
importado é uma cópia: as mudanças feitas nele não voltam para a pasta original.

## O que vem depois

Pela ordem do plano: gestão do projeto (carteira, priorização, termo versionado, papéis e
RACI, tollgates, riscos, reuniões, cronograma com PERT/CPM, ganhos), as telas de estatística
(sobre o `utsim-quality` e o `utsim-stats`), integração com o modelo de simulação (SIPOC e
fluxograma gerados do modelo) e os relatórios em PDF (A3, FMEA, plano de controle).

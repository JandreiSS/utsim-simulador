# UTSim Seis Sigma (gerenciador de projetos Lean Seis Sigma)

O UTSim Seis Sigma é o aplicativo da família UTSim para conduzir projetos de melhoria Lean
Seis Sigma: termo de abertura, fases e tollgates, quadro de ações, riscos, reuniões,
cronograma, ganhos e as ferramentas de cada fase. É um programa de janela própria (não abre o
navegador), funciona sem internet e não precisa de servidor: a equipe trabalha numa pasta
compartilhada do Google Drive. O nome ainda é provisório.

Já funcionam a **fundação** (etapa L0 do `docs/plano_lean_seis_sigma.md`: o formato do
projeto, o trabalho em equipe pela pasta compartilhada, a visão geral e o quadro de ações), a
**gestão do projeto** (etapa L1: carteira com banco de ideias e priorização, termo de abertura
com versões, equipe e matriz RACI, fases e tollgates, riscos, reuniões e atas, cronograma com
Gantt e PERT, ganhos, encerramento e lições aprendidas) e as **ferramentas qualitativas**
(etapa L2: VOC e árvore CTQ, SIPOC, Pareto, plano de coleta, Ishikawa, 5 porquês, matriz
causa-efeito, FMEA com a tabela de ação prioritária, planos de controle e de resposta e A3).
A estatística (etapa L3) aparece na navegação com o que vai trazer.

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

Os caminhos podem ser digitados (ou colados do Explorador de Arquivos) ou escolhidos em
"Procurar…": uma janela do próprio programa lista as pastas, as unidades de disco (Windows) ou
a raiz e a pasta do usuário (Linux), e, ao importar, os arquivos `.lssz`. Pastas de projeto
aparecem com ▣; um duplo clique abre o projeto. Não depende de nenhum componente do sistema.

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
- **Termo de abertura**, **Equipe e RACI**, **Fases e tollgates**, **Ações**, **Riscos**,
  **Reuniões**, **Cronograma**, **Ganhos** e **Encerramento e lições**: descritas abaixo.
- **Ferramentas**: os entregáveis das fases, descritos em [Ferramentas](#ferramentas).
- **Dados** (etapa L3) mostra o que vai trazer; já está lá a exportação `.lssz`.

Em todas as telas, cada campo é gravado quando você sai dele, datas são `DD/MM/AAAA` e números
aceitam vírgula decimal (`1.234,5`). Tudo vai para o diário como qualquer alteração: chega aos
colegas pela pasta compartilhada, entra no histórico e, se duas pessoas mudarem o mesmo campo
ao mesmo tempo, aparece em **Conflitos**.

### Carteira: banco de ideias, priorização e seleção

A carteira é a pasta que guarda as pastas dos projetos (por exemplo `G:\Meu Drive\Melhoria`).
O banco de ideias, a triagem, a priorização e a seleção ficam numa **pasta de carteira** ao
lado dos projetos, com o mesmo diário de um projeto. Na primeira vez, a tela Carteira oferece
"Criar a carteira" com o nome da carteira, a unidade de negócio (planta, área ou diretoria) e o
programa de implantação; essa estrutura (programa, unidade, carteira, projeto) é a que o
programa do master black belt vai usar mais adiante (etapa L6). Abrir a pasta da carteira
diretamente, pela tela inicial, também leva à tela Carteira.

- **Ideias**: qualquer pessoa registra uma oportunidade (título, processo, problema observado,
  dados disponíveis, ganho grosseiro). O quadro mostra as ideias por situação: ideia, triagem,
  aprovada, em execução e suspensa; encerradas, canceladas e arquivadas aparecem contadas.
  Os botões "→" mostram só as passagens permitidas (por exemplo, de ideia para triagem; de
  triagem para aprovada ou de volta para ideia).
- **Triagem**: seis perguntas (processo novo? causa e solução conhecidas? risco baixo? problema
  local resolvido em poucos dias? uma página basta? ciclo curto e repetitivo?) sugerem a
  abordagem, com a pergunta que decidiu: DMADV para processo novo, ação direta para causa e
  solução conhecidas com risco baixo, kaizen rápido, A3, PDCA e, por padrão, DMAIC. Quem
  decide é o champion, no campo "Abordagem escolhida"; aprovar uma ideia exige a abordagem.
- **Priorização**: impacto e esforço de 1 a 5 colocam a ideia na matriz (ganho rápido,
  projeto estratégico, tarefa menor, evitar; 3 ou mais conta como alto). A pontuação ponderada
  usa os critérios da carteira (padrão: ganho financeiro 3, satisfação do cliente 2,
  segurança 2, alinhamento estratégico 2, disponibilidade de dados 1, duração 1 e risco 1,
  esses dois com "menor é melhor"), cada um em escala 1 a 5 ou 1-3-9, normalizados de 0 a 1 e
  somados com os pesos, de 0 a 100. O alinhamento estratégico vem da matriz objetivos ×
  ideia com intensidade 0, 1, 3 ou 9, ponderada pelos pesos dos objetivos. A classificação
  mostra também a fração dos critérios com nota.
- **Capacidade**: um aviso aparece quando um belt passa do limite de projetos da carteira
  (padrão 2).
- **Abrir o projeto**: numa ideia aprovada, o botão cria a pasta do projeto ao lado da carteira,
  com a abordagem, o processo e o problema copiados e o belt candidato como green belt; a
  ideia passa a "em execução" com a ligação ao projeto.
- **Projetos nesta pasta**: título, abordagem, situação, fase atual e ganhos validados de cada
  projeto ao lado, com "Abrir". "Atualizar a carteira" lê a pasta de novo.

### Termo de abertura e versões

Os campos do termo: título, processo, área, declaração do problema, Y do projeto, meta
(SMART, com unidade e fonte), linha de base, prazo, escopo dentro e fora, caso de negócio,
premissas, restrições, riscos iniciais e o início do cronograma.

- **Registrar versão** guarda uma cópia numerada do termo com o motivo.
- **Aprovar** só aparece na versão mais nova e só vale para quem tem o papel de patrocinador
  no projeto. A aprovação grava quem aprovou e quando e congela a linha de base do cronograma.
- Depois da aprovação, mudar meta, prazo ou escopo faz o termo aparecer como "precisa de nova
  aprovação", com a lista das mudanças; outras mudanças (área, premissas) só são contadas.
- **Comparar** mostra as diferenças de uma versão para a anterior e para o termo atual, campo a
  campo.

### Equipe e RACI

As pessoas do projeto (quem abriu o projeto entra sozinho; outras pessoas são adicionadas pelo
nome e e-mail) e os papéis de cada uma: patrocinador, champion, dono do processo, master black
belt, black belt, green belt, yellow belt, equipe e controladoria. Uma pessoa pode ter papéis
diferentes em cada projeto. Os papéis valem para as regras: só o patrocinador aprova o termo,
só a controladoria valida ganhos, só o dono do processo aceita a transferência.

A matriz RACI tem uma linha por entrega (termo, o tollgate de cada fase, cronograma, riscos,
ganhos, plano de controle, encerramento) e uma coluna por papel em uso. Ela nasce das
responsabilidades do plano (por exemplo, patrocinador A no termo e nos tollgates, controladoria
A nos ganhos, dono do processo A no plano de controle) e cada célula pode ser mudada; as
mudadas à mão aparecem em destaque. Um aviso lista as linhas sem exatamente um A.

### Fases e tollgates

No alto, as fases da abordagem com a situação de cada uma e do seu tollgate. Escolha a fase e
"Preparar o tollgate" cria o checklist padrão da fase (no DMAIC, o da seção 1.4 do plano) e
põe o patrocinador e o champion como aprovadores.

- **Itens manuais**: você marca e escreve a evidência.
- **Itens verificados**: o programa confere no estado do projeto, e o ✔ ou ✖ muda sozinho:
  termo completo; termo aprovado sem mudança de meta ou escopo; patrocinador e belt definidos;
  cronograma com tarefas e sem ciclo; riscos registrados, cada um com dono; nenhum risco alto
  sem dono; ganho registrado; ganhos previstos ou validados pela controladoria; toda ação da
  fase em "feito"; lição registrada; aceite do dono do processo; e "existe o entregável"
  de uma ferramenta (VOC e CTQ, SIPOC, Ishikawa, FMEA, plano de coleta, plano de controle), que
  vale quando o projeto tem, em qualquer fase, um entregável dessa ferramenta (tela Ferramentas)
  com ao menos uma linha ou nó. As linhas e nós dos entregáveis da fase entram no congelamento.
- Itens podem ser acrescentados (manuais ou com uma das verificações) e retirados.
- **Submeter ao tollgate** só funciona com todos os itens obrigatórios atendidos; abre uma
  rodada de revisão e põe a fase "em tollgate".
- **Regra de aprovação**: todos aprovam, maioria dos aprovadores, ou patrocinador obrigatório
  (padrão). A lista de aprovadores pode ser mudada; vazia, qualquer pessoa decide.
- **Decisões**: cada aprovador registra aprovar, aprovar com ressalvas (uma ressalva por linha;
  cada uma vira um cartão em "A fazer", na fase seguinte, ligado à decisão), refazer (a fase
  volta a "em andamento"), suspender (o projeto fica suspenso até "Retomar o projeto") ou
  cancelar. Quando a regra é atendida, o tollgate fica aprovado, a fase aprovada e a fase
  seguinte passa a ser a atual.
- **Congelamento**: cada decisão guarda o conteúdo dos entregáveis da fase (itens do checklist
  com marca e evidência, e os entregáveis das ferramentas ligados à fase) e o hash SHA-256
  desse conteúdo. Se algo muda depois da aprovação, o tollgate aparece como "alterado depois
  da aprovação", com as diferenças; "Submeter de novo" abre outra rodada para aprovar a versão
  atual, sem mexer nas fases.

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
- **Limites por coluna** (em "Limites de cartões em andamento"): o cabeçalho mostra
  "Fazendo (2/3)"; uma coluna cheia não recebe cartões de outra (o programa avisa na barra de
  baixo). Se duas pessoas moverem cartões ao mesmo tempo e a coluna passar do limite, o
  cabeçalho fica em destaque.
- **Origem**: o cartão mostra de onde veio (manual, ressalva de tollgate, resposta a risco, ata
  de reunião) e a que está ligado.
- **Verificação de eficácia**: com um critério escrito, o cartão só vai para "Feito" depois de
  marcada "Eficácia confirmada".

Cartões criados pelas ferramentas (a ação recomendada de uma linha do FMEA, um sinal do plano
de resposta) mostram a origem no painel do cartão.

### Riscos

Cada risco tem descrição, causa, efeito, probabilidade e impacto de 1 a 5, resposta (evitar,
mitigar, transferir, aceitar), plano de resposta, dono, data de revisão e situação. O nível é
probabilidade × impacto: até 4 baixo, de 5 a 14 médio, 15 ou mais alto. A matriz 5 × 5 conta
os riscos abertos em cada célula, colorida pela faixa, e a lista vem do maior nível para o
menor. "Criar ação da resposta" põe um cartão no quadro com o plano de resposta, o dono e a
data de revisão, ligado ao risco.

### Reuniões e atas

Cada reunião tem título, data, tipo (equipe, revisão de tollgate, carteira), presentes, pauta,
anotações e decisões.

- **Gerar pauta com os itens abertos** escreve a pauta com as ações atrasadas e as que vencem
  em 7 dias, os riscos altos, os riscos com revisão na data, os tollgates em revisão, as ações
  de atas anteriores ainda sem cartão e os conflitos a revisar. A pauta pode ser editada.
- **Itens da ata**: anotação, decisão ou ação; a ação tem responsável e prazo.
- **Criar cartões das ações da ata** põe, com um clique, um cartão por ação ainda sem cartão,
  com responsável e prazo, ligado à reunião. Rodar de novo não duplica.

### Cronograma

- **Início** (o mesmo campo do termo) e **dias úteis**: segunda a sexta por padrão; marque o
  sábado, por exemplo, se a equipe trabalha nele.
- **Feriados**: um a um, ou "Incluir os feriados nacionais do ano" (Confraternização, Sexta-feira
  da Paixão calculada pela Páscoa, Tiradentes, Dia do Trabalho, Independência, Aparecida,
  Finados, Proclamação da República, Consciência Negra a partir de 2024 e Natal). Carnaval,
  Corpus Christi e feriados locais entram à mão.
- **Tarefas**: nome, duração em dias úteis ou três pontos (otimista, mais provável, pessimista),
  "Depende de" com os números das linhas e defasagem (`2; 3+1`: depois da linha 2 e um dia útil
  depois da linha 3), responsável e percentual concluído. "Marcos dos tollgates" acrescenta um
  marco por fase, em sequência, com data-alvo opcional.
- **Cálculo**: a rede vai para o PERT/CPM do UTSim (o mesmo do comando `utsim pert`): datas mais
  cedo e mais tarde, folga, caminho crítico, duração esperada `(O + 4M + P) / 6` e desvio padrão
  pelo PERT clássico, probabilidade de cumprir o prazo do termo e a data-alvo de cada marco, e,
  em "Rodar Monte Carlo" (5000 sorteios), as datas P50, P80, P90 e P95 do término.
- **Gantt**: uma barra por tarefa (preenchida pelo percentual), em vermelho no caminho crítico,
  a linha de base fina por baixo, marcos como losangos, fins de semana sombreados e a linha de
  hoje. **Congelar linha de base** grava as datas atuais (a aprovação do termo também
  congela), e o atraso de cada tarefa é contado em dias úteis contra ela.

### Ganhos

Pela decisão G1 do plano, as definições padrão são: **hard**, redução de custo com efeito no
resultado ou aumento de receita com a mesma estrutura; **soft**, custo evitado, capacidade
liberada, ganho de tempo sem redução de quadro, melhoria de segurança ou satisfação; e o
acompanhamento é de 12 meses depois do Controlar.

- Estados: **estimado** (o belt registra, com a memória de cálculo), **previsto** (a
  controladoria valida a previsão), **realizado** (o primeiro lançamento mensal muda o estado) e
  **validado** (a controladoria confirma; todos os lançamentos ficam validados). Só a
  controladoria do projeto faz as validações e volta um estado para corrigir. Corrigir um mês
  validado volta o ganho a realizado.
- Um ganho estimado por simulação nunca passa a validado sem lançamentos reais.
- O acompanhamento mostra, mês a mês desde o início, o valor esperado, o realizado (✔ quando
  validado) e os acumulados. A tabela do alto soma os ganhos do projeto por categoria e estado.

### Encerramento e lições aprendidas

O encerramento pede: todos os tollgates aprovados, a transferência aceita pelo dono do
processo ("Aceitar a transferência", só para quem tem esse papel), ao menos uma lição
registrada e o resultado do Y contra a meta. Ganhos ao menos previstos e ações concluídas são
recomendados (o acompanhamento dos ganhos continua depois do encerramento). "Encerrar o
projeto" muda a situação para encerrada.

Cada lição tem título, o que funcionou, o que não funcionou, o que repetir e etiquetas
(processo, área, tipo de problema, causa raiz, solução, ferramentas). A **base de
conhecimento** busca nas lições deste projeto e dos outros projetos da mesma pasta: acentos e
maiúsculas não importam, todas as palavras precisam aparecer, e etiquetas, processo e título
pesam mais que o texto. Abaixo, os **projetos parecidos** (mesmo processo ou etiquetas em
comum).

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

Pela ordem do plano: as telas de estatística (sobre o `utsim-quality` e o `utsim-stats`),
integração com o modelo de simulação (SIPOC e fluxograma gerados do modelo), os relatórios em
PDF e os modelos da empresa, e o programa de implantação do master black belt (várias
carteiras, comitê de seleção, designação de belts, formação). Da gestão do projeto ficam para
depois a visão "minhas ações" de toda a carteira, raias no quadro, reuniões recorrentes e a
capacidade dos belts em horas.

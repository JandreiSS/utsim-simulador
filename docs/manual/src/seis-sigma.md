# UTSim LSS (gerenciador de projetos Lean Seis Sigma)

O UTSim LSS é o aplicativo da família UTSim para conduzir projetos de melhoria Lean
Seis Sigma: termo de abertura, fases e tollgates, quadro de ações, riscos, reuniões,
cronograma, ganhos e as ferramentas de cada fase. É um programa de janela própria (não abre o
navegador), funciona sem internet e não precisa de servidor: a equipe trabalha numa pasta
compartilhada do Google Drive. LSS vem de Lean Seis Sigma; até a versão 0.16.0 o app se
chamava UTSim Seis Sigma, e os projetos, o programa e os atalhos daquela época continuam valendo.

Já funcionam a **fundação** (etapa L0 do `docs/plano_lean_seis_sigma.md`: o formato do
projeto, o trabalho em equipe pela pasta compartilhada, a visão geral e o quadro de ações), a
**gestão do projeto** (etapa L1: carteira com banco de ideias e priorização, termo de abertura
com versões, equipe e matriz RACI, fases e tollgates, riscos, reuniões e atas, cronograma com
Gantt e PERT, ganhos, encerramento e lições aprendidas) e as **ferramentas qualitativas**
(etapa L2: VOC e árvore CTQ, SIPOC, Pareto, plano de coleta, Ishikawa, 5 porquês, matriz
causa-efeito, FMEA com a tabela de ação prioritária, planos de controle e de resposta e A3).
A estatística (etapa L3) aparece na navegação com o que vai trazer.

## Instalar e abrir

O UTSim LSS é um produto próprio no launcher (veja [Launcher](launcher.md)), com o
nome "UTSim LSS". Escolha-o na lista, instale e clique em "Abrir". Ele não precisa do
editor de simulação instalado.

Pela linha de comando, `utsim-seis-sigma` abre a janela e `utsim-seis-sigma "pasta.lss"` abre
direto um projeto.

## Primeiro uso: quem é você

Na primeira vez, o programa pede o seu nome e e-mail. Eles vão em cada alteração que você
fizer, para a equipe saber quem mudou o quê e quando. Ficam guardados só no seu computador,
na sua pasta de usuário, e nunca dentro dos projetos:

- Windows: `%APPDATA%\UTSim\seis-sigma\config.json`;
- Linux: `~/.config/utsim/seis-sigma/config.json`.

Ali também ficam a lista de projetos recentes, o tema (sistema, claro ou escuro) e o
**idioma** da interface: português do Brasil, inglês ou espanhol. A troca vale na hora, e cada
pessoa da equipe pode usar o seu idioma no mesmo projeto, porque o que se grava no diário são
os dados, não os textos das telas. Os textos que a própria equipe escreve (títulos, descrições,
atas) ficam como foram escritos. Por enquanto, números, datas e o símbolo da moeda seguem o
formato brasileiro em todos os idiomas, e alguns textos gerados pelas análises e relatórios
ainda saem em português. "Trocar", na tela inicial, corrige o nome ou o e-mail.

Não há senha: sem servidor, não há como conferir quem é quem. Qualquer pessoa com acesso à
pasta pode gravar no projeto em nome de outra. Para aprovações com valor de registro, o plano
prevê assinatura das aprovações com chave por pessoa (etapa P7), que mesmo assim não equivale
a uma assinatura digital com validade jurídica.

## Criar, abrir e importar projetos

Na tela inicial:

- **Novo projeto**: título, pasta onde criar e abordagem (DMAIC, DMADV, kaizen rápido, A3,
  PDCA ou ação direta). O programa cria a pasta `<título>.lss` com as fases padrão da
  abordagem (no DMAIC: Definir, Medir, Analisar, Melhorar e Controlar) e registra você na
  equipe. Caracteres que o Windows não aceita em nomes de pasta viram `-`. Em **Modelo de
  projeto**, os modelos das linhas especializadas do UTSim: "Kaizen administrativo (Lean
  Office)" (abordagem kaizen) e "DMAIC transacional (Lean Office)" (abordagem DMAIC), para
  melhorar processos administrativos. O modelo escolhe a abordagem e acrescenta ao checklist
  de cada tollgate os entregáveis da linha (VSM administrativo do estado atual e futuro, %C&A
  medido por etapa, fluxograma de raias por departamento, lead time e % dentro do SLA no plano
  de controle). Veja [Lean Office](linhas/lean-office/index.md).
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

Textos de várias linhas (atas, descrições, observações) são comparados linha a linha. Se as
duas pessoas mexeram em linhas diferentes do mesmo texto, as duas mudanças ficam e não há
conflito; só vira conflito quando as duas mudaram a mesma linha ou acrescentaram linhas no
mesmo ponto.

A regra vale também para quem digitava enquanto a mudança do colega chegava: o programa
lembra a versão que você viu quando começou a digitar, e se ela mudou nesse meio tempo o
caso vai para a lista.

### Assinaturas e integridade do diário

Na primeira vez que você informa nome e e-mail, o programa cria uma **chave de assinatura**
só sua, guardada neste computador, ao lado da configuração (`chave_assinatura.txt`). A tela de
identidade mostra a chave pública em grupos de quatro caracteres e o caminho do arquivo. A
chave nunca vai para a pasta do projeto: lá fica só a parte pública, gravada por você mesmo
na primeira mudança que fizer em cada projeto.

- **Cada mudança é assinada.** Ao gravar, o programa escreve também um elo da sua cadeia em
  `cadeia/<pessoa>-<chave>/`, com o resumo (SHA-256) de cada arquivo de mudança e o resumo do
  elo anterior, tudo assinado com a sua chave.
- **A conferência fica na tela Equipe e RACI**, na seção **Integridade do diário**. Ela mostra,
  por pessoa, quantas mudanças estão assinadas, quantas são de antes da chave e quantos elos
  há na cadeia. Abaixo vêm os sinais encontrados: uma mudança editada depois de assinada, um
  elo editado ou removido, dois elos com o mesmo número (uma cópia antiga da pasta restaurada
  e usada) e uma chave registrada por outra pessoa no seu nome, que é ignorada. Também
  aparecem avisos mais brandos: uma mudança assinada que ainda não chegou (o Drive pode estar
  sincronizando), uma mudança sem assinatura em nome de quem já assina (gravada por uma versão
  antiga, por outro app ou por outra pessoa) e o conteúdo apagado por uma anonimização.
- **Ao abrir um projeto** com sinais de alteração, a barra de baixo avisa. Use "Conferir de
  novo" depois que o Drive terminar de sincronizar.
- **Aprovações assinadas.** Em **Fases e tollgates**, cada decisão do histórico mostra se está
  assinada por quem decidiu, e quem decidiu vê o botão **Assinar**. Em **Ganhos**, o mesmo vale
  para a validação de um ganho, assinada por quem validou. A assinatura cobre a decisão e o
  conteúdo decidido (o resumo congelado do tollgate, ou o valor, a memória de cálculo e o
  acompanhamento do ganho). Se o conteúdo mudar depois, a tela mostra que a assinatura é de
  outra versão.
- **Dois computadores.** Cada computador cria a própria chave, e cada chave tem a sua cadeia,
  então as duas convivem sem conflito. A tela Equipe mostra quando uma pessoa passou a usar
  mais uma chave. Para usar a mesma chave nos dois, copie `chave_assinatura.txt` antes de
  abrir os projetos no segundo computador.

A conferência revela alterações, mas não as impede: quem tem acesso aos arquivos pode
editá-los, e o programa só mostra que isso aconteceu. A primeira chave que uma pessoa registra
num projeto é a que vale. Estas assinaturas não têm a validade jurídica de uma assinatura
digital certificada.

### Página das ações no gemba

No fim da tela **Equipe e RACI**, a seção **Acesso pela rede local** liga uma página só de
leitura com as ações abertas do projeto, separadas por responsável e em ordem de prazo, com a
coluna do quadro, o prazo (as atrasadas em vermelho) e o lugar. Ela serve para acompanhar o
quadro num tablet ou celular no chão de fábrica. A página se recarrega a cada 30 segundos e
ninguém edita nada por ela.

- **Só este computador**, que é o padrão: o endereço é `http://localhost:8790/` (outra porta,
  se essa estiver ocupada), e só um navegador deste computador abre a página.
- **Abrir para a rede local**: marque a opção antes de ligar. O endereço passa a ter o número
  deste computador na rede e um token (`/?t=…`), sorteado a cada vez que a página é ligada.
  Quem não tiver o endereço completo recebe uma página de acesso negado. O Windows pode pedir
  permissão no firewall na primeira vez.

Qualquer pessoa da rede com o endereço lê as ações, então desligue a página quando não
precisar mais. Ela também desliga ao fechar o projeto ou o programa.

## Telas do projeto

A navegação à esquerda tem a **Carteira** e as telas do projeto.

- **Visão geral**: as fases com a atual destacada, o andamento das ações (concluídas e
  atrasadas), os riscos abertos e os altos, os ganhos por estado, as reuniões, o prazo com os
  dias que faltam, os conflitos e a pasta do projeto.
- **Termo de abertura**, **Equipe e RACI**, **Fases e tollgates**, **Ações**, **Riscos**,
  **Reuniões**, **Cronograma**, **Ganhos** e **Encerramento e lições**: descritas abaixo.
- **Ferramentas**: os entregáveis das fases, descritos em [Ferramentas](#ferramentas).
- **Roteiro DMAIC**, **Dados e análises**, **Experimentos**, **Modelo UTSim** e **Relatórios**:
  descritas nos capítulos de mesmo nome.
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

- **Minhas ações**: no alto da tela, a lista dos cartões em aberto com você como responsável
  em todos os projetos da pasta, com o projeto, o prazo (em vermelho quando atrasado) e a
  coluna do quadro. **Abrir** leva ao projeto do cartão.
- **Ideias**: qualquer pessoa registra uma oportunidade (título, processo, problema observado,
  dados disponíveis, ganho grosseiro). Outros apps da família também mandam ideias: o **8D do
  UTSim Qualidade**, quando o problema pede um projeto Lean Seis Sigma, envia a ideia com o
  número do 8D, quem enviou e quando, e os anexos (relatório, dados). A ideia mostra a origem
  no alto, e cada anexo abre com um clique; os arquivos ficam em `anexos/` da carteira. O quadro mostra as ideias por situação: ideia, triagem,
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
Analisar; seleção de soluções, 5S, SMED e trabalho padronizado no Melhorar; planos de
controle e de resposta no Controlar; o evento kaizen e os ciclos PDCA nas fases da própria
abordagem) e pode ser mudada de fase no alto do entregável. Um projeto pode ter mais de um entregável da mesma ferramenta (dois Paretos,
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
- **Clicar numa barra** filtra as categorias e a tabela do resultado só com ela (a barra
  "Outros" mostra as categorias que junta); clicar de novo mostra todas. O tracejado
  vertical marca onde terminam as poucas vitais.

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
  as suas primeiras causas em cartões; a borda das verificadas fica na cor de destaque, a das
  prováveis em laranja e a das descartadas apagada. O selo "+n" diz quantas subcausas a causa
  tem; elas ficam na lista abaixo do desenho.
- **Editar no desenho**: clique num osso, fora dos cartões, para acrescentar uma causa
  naquela categoria (Enter grava e abre a próxima). Duplo clique num cartão edita o texto, e
  apagar o texto remove a causa. Arraste um cartão para outro osso para mudar a categoria (as
  subcausas vão junto). O botão direito abre o menu, com "Começar um 5 porquês com esta
  causa": a cadeia nasce no primeiro 5 porquês do projeto (criado se não houver), ligada à
  causa, e a tela passa para ele.
- Se duas pessoas moverem causas ao mesmo tempo e uma ficar embaixo da outra (um ciclo), o
  programa mostra a primeira como causa principal e avisa; nada some.

### 5 porquês

- Cada **cadeia** começa de um problema, escrito à mão ou tirado de uma causa de um Ishikawa
  do projeto ("Nova cadeia a partir de uma causa do Ishikawa" ou o campo "Causa do
  Ishikawa" da cadeia).
- "+ por quê (novo ramo)" acrescenta um primeiro porquê; "+ por quê" ao lado de um porquê
  acrescenta o seguinte. Clicar duas vezes no mesmo porquê abre dois ramos.
- Cada porquê tem a evidência. Marque "causa raiz" no porquê que a equipe concluiu.
- **A árvore**: cada cadeia aparece também como árvore, da esquerda para a direita, com o
  problema na raiz. O "+" à direita de um nó acrescenta um porquê abaixo dele (um ramo novo
  quando ele já tem filhos), duplo clique edita e o botão direito marca a causa raiz (borda
  grossa) ou remove o porquê e o que está abaixo.
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
- **Matriz severidade × ocorrência**: acima da grade, cada modo de falha com S e O é um ponto
  na matriz, na cor da AP da linha (alta em vermelho, média em âmbar, baixa em verde); o
  fundo das células mostra a faixa de S × O. A primeira coluna da grade traz a mesma cor.
  Clique num ponto para ir à linha (ela fica marcada); clicar de novo na mesma célula passa
  para a próxima linha dela.
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

#### Controle em operação pelo gêmeo

Embaixo do plano de controle, a seção "Controle em operação pelo gêmeo" acompanha cada
característica depois do projeto, com o dado que o gêmeo digital recebe da fábrica.

- **Painel ou gravações**: o endereço do painel do gêmeo nesta máquina
  (`http://127.0.0.1:8080`; o programa lê a rota `/api/gemeo`), um JSON salvo do painel, a
  pasta de gravações do gêmeo (a sessão mais recente) ou um arquivo `.jsonl` de observações.
  Outros computadores não são lidos.
- **Fonte no gêmeo**, por linha: `kpi:<id>` é um indicador que o painel calcula em cada
  janela (por exemplo `kpi:vazao`); `tag:<tag>` é uma medição que chega pela tag (por
  exemplo `tag:Torno1.diametro`). Só entram leituras de qualidade boa e com valor numérico.
- **LIC, LC e LSC**: os limites congelados da fase II. "Do modelo" usa o que o modelo
  validado espera do indicador na janela (a média simulada ± 3 desvios-padrão, como o
  painel informa); "Fase I da série" estima os limites da carta de valores individuais com a
  série lida, tirando os pontos fora em até três rodadas. Também podem ser digitados. Sem
  limites, a primeira leitura congela os do modelo.
- **Sinal de resposta**: o sinal do plano de resposta que a linha dispara.
- **Ganho, linha de base, R$/mês por unidade e melhor**: ligam a linha a um ganho do
  projeto. O realizado do mês é (média lida − linha de base) × valor por unidade, com o
  sinal invertido quando menor é melhor.

"Ler agora" lê o gêmeo e confere cada linha com as regras de Western Electric. Quando um
ponto posterior à última leitura quebra uma regra, o sinal de resposta abre um cartão em "A
fazer", com os limites, os pontos fora e os últimos valores no "por quê". Uma leitura
repetida não abre outro cartão; se o gêmeo recomeçou do tempo zero, os pontos contam como
novos. "Ler a cada minuto enquanto aberto" repete a leitura sozinho.

Para cada linha lida aparecem a carta com os limites e os pontos fora, os alertas do próprio
gêmeo para aquele indicador (modelo e fábrica se afastando), "Guardar como conjunto Gêmeo"
(a série vira conjunto de dados de origem Gêmeo, para as análises) e "Lançar realizado",
que grava o valor no mês escolhido do ganho, com o cálculo na observação, para a
controladoria validar.

### Plano de resposta

Uma linha por sinal (regra de carta violada, limite de especificação, alarme): ação imediata,
contenção, quem acionar e prazo. "Sinal ocorreu: abrir cartão" cria um cartão em "A fazer"
com o sinal e a ação imediata no título, a contenção no "como", quem acionar no "onde", o
prazo (se for uma data) e a origem "sinal do plano de resposta". Cada vez que o sinal ocorre,
um cartão novo; "Abrir cartão" leva ao último.

### Mapa do fluxo de valor

Um mapa por estado: o **atual** (fase Medir) e o **futuro** (fase Melhorar). Cada linha é um
processo na ordem do fluxo, com o tempo de ciclo, o setup, a disponibilidade em %, os
operadores, o estoque que espera antes dele (em unidades) e se ele agrega valor. No alto do
mapa ficam a demanda por dia, o tempo disponível por dia e a unidade de tempo.

Com a demanda e o tempo disponível, o app calcula:

- o **takt**: tempo disponível ÷ demanda, o ritmo do cliente;
- a **espera** de cada estoque: estoque × takt, o tempo que ele leva para ser consumido no
  ritmo do cliente;
- a **escada do lead time**, desenhada abaixo da tabela: a espera de cada estoque no degrau de
  baixo (laranja) e o tempo de ciclo do processo no de cima (verde quando agrega valor);
- o **lead time** (soma das esperas e dos ciclos), o tempo que agrega valor, a **eficiência do
  ciclo do processo** (tempo que agrega valor ÷ lead time) e o estoque em dias de demanda;
- os processos **acima do takt**, pelo ciclo dividido pela disponibilidade, e o mais lento.

"Criar o estado futuro a partir deste" copia o mapa atual para um entregável novo na fase
Melhorar, ligado ao atual; nele a equipe muda os processos e os estoques para o fluxo desejado,
e o mapa mostra o lead time e a eficiência antes e depois. Com o modelo UTSim ligado, "Gerar o
VSM atual como entregável" (tela Modelo UTSim) preenche o mapa atual com os processos do caminho
principal do modelo, o estoque médio antes de cada um e a disponibilidade medida numa rodada.

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


### Seleção de soluções

Escolhe entre soluções (no Melhorar) ou conceitos (no Analisar do DMADV). Os critérios têm
peso de 1 a 10. Há dois métodos, escolhidos no alto:

- **Matriz ponderada:** cada opção recebe de 1 a 5 em cada critério; o total soma peso ×
  nota e aparece também como percentual do máximo possível.
- **Matriz de Pugh:** marque uma opção como referência (o conceito atual ou o mais
  conhecido); as outras são comparadas com ela em cada critério como melhor (+), igual (S) ou
  pior (−). O total soma peso × sinal, e a tabela mostra quantos +, S e − cada opção teve.

A melhor opção aparece quando todas as células dela têm nota. Impacto e esforço (1 a 10) de
cada opção montam a grade de prioridade: fazer já, planejar, se sobrar tempo ou descartar.

### Auditoria 5S

A lista de verificação tem perguntas por senso (utilização, organização, limpeza,
padronização e disciplina). "Lista inicial" acrescenta duas perguntas por senso, para a
equipe adaptar à área; a lista é da empresa. "Nova auditoria" abre uma coluna com a data de
hoje e o auditor; cada pergunta recebe de 0 a 4. Embaixo ficam o percentual de cada senso e
o total, o senso mais fraco da última auditoria, o radar (a última auditoria cheia, a
primeira em contorno) e a evolução do total entre as auditorias. As fotos de antes e depois
vão como anexos do projeto.

### SMED (troca rápida)

Cada atividade da troca entra com o tempo em minutos e a classe de hoje: **interna** (feita
com a máquina parada) ou **externa** (feita com a máquina rodando). Na coluna "Depois" diga
o que a atividade vira com o estudo (interna, externa ou eliminada) e, se mudou, o tempo
novo e como. O tempo de setup é a soma das internas; a tela mostra antes e depois, as
atividades convertidas e a redução do setup, com o gráfico das duas situações. Para ver o
efeito na linha, mude o tempo de setup no modelo e rode o estudo de cenários.

### Trabalho padronizado

O takt vem do tempo disponível e da demanda do mesmo período (por exemplo 27 000 s e 450
peças: 60 s), ou é digitado. Cada elemento de trabalho tem o operador, os tempos manual,
automático (a máquina trabalha sozinha) e de deslocamento, em segundos, e o estoque padrão
em processo. A tela mostra:

- o conteúdo de trabalho de cada operador (manual mais deslocamento) e o total;
- o número mínimo de operadores, ⌈conteúdo total / takt⌉, e a eficiência do balanceamento
  (conteúdo total sobre operadores × o maior ciclo);
- o gráfico de balanceamento, com os operadores acima do takt em laranja;
- a tabela de combinação de cada operador: o trabalho manual em barra cheia, a máquina em
  barra clara (laranja quando ela ainda trabalha quando o operador volta, no ciclo seguinte)
  e o deslocamento em linha, com o takt em vermelho.

### Evento kaizen

Para o kaizen rápido. No alto ficam o primeiro dia e a duração (3 a 5 dias).
"Preencher preparo e pauta" traz o preparo (escopo e meta com o patrocinador, dados, equipe
liberada, logística) e a pauta, com os seis temas distribuídos pelos dias: entender, medir,
analisar, melhorar, padronizar e apresentar. Itens marcados como feitos ficam registrados.

As pendências têm responsável e prazo; "Mandar ao quadro" cria o cartão no quadro de ações
com origem kaizen, ligado à pendência. A pendência fica feita quando o cartão chega a
"Feito". O acompanhamento mostra as datas de 30, 60 e 90 dias depois do evento, as
pendências atrasadas em cada uma e o que a equipe viu em cada revisão. O tollgate de
abertura do kaizen confere que o evento existe.

### Ciclos PDCA

Cada ciclo registra a hipótese, o plano (Planejar), o que foi feito (Fazer), o que se
verificou com dados (Verificar), o resultado da hipótese (confirmada ou refutada) e o que se
padronizou ou ajustou (Agir). A tela diz em que etapa cada ciclo está e conta os ciclos
confirmados, refutados e em andamento. Um ciclo novo mostra o que o anterior padronizou,
para partir dali.

### Ferramentas da qualidade do DMADV

A casa da qualidade (QFD), as tolerâncias por Monte Carlo e o poka-yoke ligado ao FMEA são
ferramentas da qualidade: as telas delas ficam no UTSim Qualidade, e o UTSim LSS guarda o
resultado como entregável quando elas forem abertas de lá.

## Roteiro DMAIC

A tela **Roteiro DMAIC** do projeto mostra o método como um fluxograma vertical: as cinco
fases (Definir, Medir, Analisar, Melhorar e Controlar), os passos de cada uma com o que o passo
pede, e ao lado as ferramentas indicadas.

- Clique numa ferramenta para abrir a tela dela neste projeto: o entregável na tela
  Ferramentas, a análise na tela Dados e análises, ou a tela do termo, da equipe, do
  cronograma, dos riscos, das ações, dos ganhos, do modelo, dos relatórios ou do
  encerramento. As ferramentas em cinza, com "em breve", ainda não estão no app.
- Dois losangos marcam as decisões: "a causa raiz foi confirmada com dados?" volta para o
  Analisar, e "a meta foi atingida no piloto?" volta para o Melhorar.
- O verde marca o que o projeto já tem: um entregável com conteúdo, uma análise guardada ou
  um item verificado do tollgate da fase.
- O zoom muda o tamanho do desenho; as barras de rolagem levam ao resto.

## Dados e análises

A tela **Dados e análises** do projeto guarda os conjuntos de dados e as análises
estatísticas, sobre as mesmas funções do `utsim estatistica` e do `utsim qualidade`.

### Conjuntos de dados

Importe um CSV (vírgula ou ponto e vírgula, vírgula ou ponto decimal, com cabeçalho) e diga
a origem: **Real** (coleta) ou **Gêmeo** (gravações do gêmeo digital). O arquivo vai para
`dados/<sha256>.csv` e o projeto guarda o nome, as colunas, o número de linhas e o hash. Os
conjuntos **Simulados** vêm da tela Modelo UTSim. A origem aparece sempre ao lado do nome, e
uma análise sobre dado que não é Real avisa que não serve de linha de base real.

### Análises

Escolha o conjunto, a análise e as colunas:

| Análise | O que pede |
|---|---|
| Estatística descritiva, normalidade (Anderson-Darling) | uma coluna |
| Capacidade (Cp, Cpk, Pp, Ppk, PPM, nível sigma com o deslocamento de 1,5 explícito) | coluna, LIE e LSE, subgrupo (1 para valores individuais) |
| Carta de controle (I-MR, X̄-R, X̄-S, p, np, c, u) com Western Electric ou Nelson | coluna, subgrupo ou coluna dos tamanhos |
| Testes t (uma amostra, duas amostras de Welch ou variância combinada, pareado) | colunas e μ0 |
| ANOVA de um fator com Tukey | coluna dos valores e coluna dos grupos |
| Qui-quadrado de independência | duas colunas de categorias |
| Uma e duas proporções | x e n |
| Mann-Whitney e Wilcoxon pareado | duas colunas |
| MSA: Gage R&R (ANOVA ou médias e amplitudes) | peça, operador, medida e a tolerância opcional |
| MSA: estudo do tipo 1 | coluna, valor de referência e tolerância |
| MSA: viés | coluna, valor de referência e a tolerância opcional |
| MSA: linearidade | coluna da referência e coluna da medida, a variação do processo opcional |
| MSA: concordância de atributos | item, avaliador e resposta, e a coluna do padrão quando há |
| Experimento fatorial 2^k | as colunas dos fatores, a resposta e a ordem máxima das interações |
| Superfície de resposta | as colunas dos fatores (codificadas, 2 a 6) e a resposta |
| Fatorial geral (ANOVA) | as colunas dos fatores (cada valor diferente é um nível) e a resposta |
| Desejabilidade de várias respostas | as colunas dos fatores e, para cada resposta, a coluna, o objetivo (maximizar, minimizar ou chegar no alvo), os limites e a importância |
| Igualdade de variâncias (Levene com a mediana e Bartlett) | coluna dos valores e coluna dos grupos |
| Kruskal-Wallis e teste da mediana de Mood | coluna dos valores e coluna dos grupos |
| Friedman | coluna dos valores, do tratamento e do bloco (um valor de cada tratamento em cada bloco) |
| ANOVA de dois fatores (com a interação quando há repetições) | coluna dos valores e as colunas dos dois fatores |
| Regressão linear, simples ou múltipla | a resposta e os preditores |
| Tamanho de amostra e poder | uma média, duas médias ou duas proporções; a diferença e o desvio (ou as proporções), o poder e a significância |
| Carta EWMA | coluna dos valores individuais, λ (0,2), L (3), alvo e σ (opcionais) |
| Carta CUSUM tabular | coluna dos valores individuais, k (0,5), h (5), alvo e σ (opcionais) |

"Calcular" mostra os números, a conclusão e o gráfico (valores, linha central e limites,
pontos com sinal); "Guardar a análise" grava a **receita** (função, colunas e parâmetros), o
conjunto com o hash e a origem, o resultado e a versão do app. "Refazer" roda a receita de
novo sobre o mesmo arquivo e confere se os números são os mesmos. "Apagar", ao lado do
conjunto escolhido e de cada análise guardada, pede um segundo clique ("Apagar mesmo") antes
de apagar; um conjunto com análises guardadas só sai depois delas. Apagar é uma operação do
diário como as outras: o histórico continua lá.

O estudo do tipo 1 usa Cg = 0,2·T / (6·s) e Cgk = (0,1·T − |média − referência|) / (3·s),
com T a tolerância; o instrumento é capaz com Cg e Cgk ≥ 1,33.

Nas análises novas: **Levene** com a mediana serve para qualquer distribuição, e **Bartlett**
supõe dados normais e é mais sensível quando eles são. **Kruskal-Wallis** e **Mood** comparam
grupos sem supor normalidade. Mood é mais robusto a valores extremos, e Kruskal-Wallis é mais
poderoso. **Friedman** compara tratamentos medidos nos mesmos blocos, por exemplo as mesmas
peças ou os mesmos turnos. A **ANOVA de dois fatores** precisa de todas as combinações com o
mesmo número de repetições. Sem repetições, a interação vira o erro, e o gráfico mostra a média
de cada combinação. A **regressão** dá os coeficientes com os valores-p, o R², o R² ajustado, o
VIF de cada preditor (acima de 5, os preditores estão muito ligados entre si) e o gráfico dos
resíduos com ±2 erros padrão. O **tamanho de amostra** dá o menor n que atinge o poder pedido;
nas comparações de dois grupos, o n é de cada grupo.

As cartas **EWMA** e **CUSUM** vigiam desvios pequenos e persistentes da média, de 0,5 a 1,5 σ,
que a carta de Shewhart só percebe tarde. A EWMA desenha a média móvel ponderada (λ é o peso do
ponto novo) com os limites exatos, mais estreitos nos primeiros pontos. A CUSUM desenha C⁺, que
acumula os desvios para cima, e C⁻ abaixo do zero, com os limites ±H. Sem alvo e σ, os dois
vêm dos próprios dados (média e amplitude móvel média / 1,128). Para vigiar um processo,
informe os da fase I ou da especificação, senão a carta mede o processo contra ele mesmo. O
resultado mostra a **ARL**, o número médio de pontos até um sinal: sob controle é o intervalo
entre alarmes falsos, e com um desvio de 1 σ é a demora média para perceber. Na EWMA a ARL vem
de uma simulação com semente fixa; na CUSUM, da aproximação de Siegmund. Com os valores
padrão, as duas dão cerca de 500 pontos entre alarmes falsos e perto de 10 para perceber um
desvio de 1 σ.

O **viés** compara a média de medições repetidas de uma peça de referência com o valor dela
pelo teste t (viés = 0) e dá o intervalo de 95 % do viés; com a tolerância, mostra o viés em
porcentagem dela. A **linearidade** usa várias peças de referência espalhadas pela faixa de
uso, medidas algumas vezes cada: o app faz a reta do viés contra a referência. Inclinação
significativa quer dizer que o viés muda ao longo da faixa; só o intercepto significativo quer
dizer um deslocamento constante, que se corrige com um ajuste. A tabela mostra também o viés em
cada referência.

A **concordância de atributos** é para características julgadas por pessoas (bom ou ruim, tipo
de defeito). Cada linha do CSV é uma avaliação: o item, quem avaliou e a resposta, e o padrão
conhecido do item quando existe. Cada avaliador avalia todos os itens o mesmo número de vezes,
em ordem aleatória e sem saber qual é o item. O app dá a porcentagem de itens em que cada
avaliador concorda consigo mesmo, com o padrão, entre todos os avaliadores e de todos com o
padrão, e o kappa de Fleiss de todas as avaliações: 0,75 ou mais é concordância boa, de 0,40 a
0,75 é razoável e pede critério mais claro, abaixo de 0,40 é fraca.

### Checklist do Medir

Dois itens do tollgate do Medir passam a ser conferidos pelo app:

- **MSA aprovado**: há um Gage R&R guardado com %VE do GRR total ≤ 30 %, ou uma concordância
  de atributos com kappa ≥ 0,75 (quando o Y é julgado por pessoas), ou um dos dois fora do
  limite com a justificativa registrada (botão "Justificar" na lista das análises);
- **Linha de base com dado de origem Real**: há uma capacidade ou uma carta de controle
  guardada sobre um conjunto de origem Real (simulado ou do gêmeo não vale).

## Planejamento de experimentos

A tela **Experimentos** do projeto planeja um experimento de dois níveis, para descobrir quais
fatores mexem no Y e se eles agem juntos.

- **Fatores**: dê a cada um o nome, o nível baixo (−1) e o alto (+1). Os níveis podem ser
  números (temperatura 180 e 200) ou qualquer texto (fornecedor X e Y). O centro é o ponto do
  meio, usado pelos pontos centrais: para níveis numéricos o app usa a média dos dois; para
  texto, escreva-o (ou não use pontos centrais).
- **Plano**: o fatorial completo roda as 2^k combinações. Com muitos fatores, uma fração (1/2,
  1/4, 1/8, 1/16) roda só uma parte delas. O preço é que alguns efeitos ficam **confundidos**:
  o app mostra a relação de definição (por exemplo I = ABCD), a resolução e a tabela do que
  se confunde com o quê. Em resolução III um efeito principal se confunde com uma interação de
  dois fatores; em resolução IV os principais ficam limpos e as interações de dois fatores se
  confundem entre si; em resolução V ou mais os principais e as interações de dois fatores
  ficam limpos. As frações oferecidas vão de 3 a 8 fatores.
- **Plackett-Burman (triagem)**: para separar, entre muitos fatores (até 15), os poucos que
  mexem no Y. Roda 8, 12 ou 16 corridas, o menor número que cabe (11 fatores cabem em 12
  corridas, contra 16 da menor fração de resolução III). Os efeitos principais ficam livres
  entre si, mas cada interação de dois fatores se espalha, em parte, sobre vários efeitos
  principais. Por isso a triagem serve para escolher os fatores, e o estudo deles fica para
  um fatorial depois. Com mais de 8 fatores é o único plano oferecido. As colunas da matriz
  que sobram sem fator dão graus de liberdade ao erro.
- **Superfície de resposta**: depois de achar os poucos fatores que importam, para achar o
  melhor ajuste deles. O **composto central rotacionável** (2 a 6 fatores) junta o fatorial,
  dois pontos axiais por fator a ±α do centro (α = 1,414 com 2 fatores, 1,682 com 3) e os
  pontos centrais; os axiais saem da faixa do fatorial, então os níveis precisam ser números.
  O **composto de face centrada** põe os axiais nas faces (α = 1) e usa só três níveis por
  fator. O **Box-Behnken** (3 a 5 fatores) usa os pontos no meio das arestas e nunca põe
  todos os fatores no extremo ao mesmo tempo, bom quando essa combinação é cara ou
  arriscada; ele precisa de ao menos um ponto central. Na planilha, a coluna `centro` diz
  `axial` nos pontos axiais. As réplicas não se aplicam: o erro vem dos pontos centrais
  (use de 3 a 5).
- **Fatorial geral**: quando um fator tem mais de dois níveis (três fornecedores, quatro
  máquinas), escreva os níveis de cada fator separados por `;` (de 2 a 10). O plano cruza
  todos os níveis de todos os fatores e repete cada combinação pelas réplicas (até 1000
  corridas, sem pontos centrais). Nas colunas A, B… a planilha põe a posição do nível na
  lista, e nas colunas com o nome do fator, o nível escrito. A análise "Fatorial geral
  (ANOVA)" lê cada coluna como categorias e testa os efeitos principais e as interações de
  dois fatores. Sem réplicas e com dois fatores não sobra erro para os testes, então use
  réplicas.
- **Réplicas** repetem cada corrida (dão o erro puro); **pontos centrais** são corridas com
  todos os fatores no centro (dão erro sem dobrar o experimento e mostram curvatura).
- **Semente**: a ordem das corridas é sorteada a partir dela. A mesma semente dá a mesma
  ordem; rode na ordem sorteada para não confundir um fator com o que muda ao longo do dia.
- **Blocos**: quando as corridas não cabem num dia, num lote de matéria-prima ou num turno,
  divida o plano em blocos. O fatorial e a fração aceitam 2 ou 4 blocos; o app confunde os
  blocos com as interações de ordem mais alta possível e diz quais são (num 2³ em dois
  blocos, ABC). O composto central aceita 2 blocos: o fatorial com metade dos centros no
  bloco 1, os axiais com a outra metade no bloco 2, e o rotacionável passa a usar o α que
  deixa os blocos ortogonais ao modelo. O Box-Behnken e a triagem têm um bloco só. A
  planilha ganha a coluna `bloco`, e a ordem sorteada fica dentro de cada bloco: rode um
  bloco inteiro antes do seguinte. Na análise, a superfície e a desejabilidade põem um termo
  por bloco (Bl1, Bl2…) e dão o ponto ótimo para a média dos blocos; o fatorial avisa com que
  termos os blocos estão confundidos, porque um efeito grande ali pode ser só a diferença
  entre os blocos.

"Gravar a planilha de coleta" grava em `dados/` um CSV na ordem de execução, com a ordem
padrão, as colunas codificadas `A`, `B`, `C`..., os níveis reais e a coluna da resposta vazia;
a primeira linha guarda o planejamento inteiro. Preencha a resposta na planilha e importe-a em
**Dados e análises** como um conjunto de origem Real.

**No modelo UTSim.** Se o projeto tem um modelo ligado e cada fator diz o parâmetro que muda
(`objeto.campo`, com os níveis escritos como no `.ron`, por exemplo `Exp(mean: 0.5)`), "Rodar
no modelo UTSim" roda um cenário por ponto do plano, com as mesmas sementes em todos (números
aleatórios comuns) e as replicações do modelo como réplicas, num fio de trabalho. O resultado
vira um conjunto de dados de origem **Simulado**, com uma coluna por indicador. O erro dessa
análise é a variabilidade da simulação, não a da fábrica.

**Análise.** Em Dados e análises, "Experimento fatorial 2^k" acha sozinho as colunas `A`, `B`...
da planilha (ou escolha as colunas dos fatores; níveis reais são codificados pelo app), a
resposta e a ordem máxima das interações (2 por padrão). O resultado traz o efeito de cada
termo (média no alto menos média no baixo), os valores-p, o R², a curvatura pelos pontos
centrais e os termos confundidos. Numa triagem de Plackett-Burman as interações não se separam
dos efeitos principais: o app estima só os efeitos principais e avisa nas notas.

Para a planilha de uma superfície, use "Superfície de resposta": escolha as colunas `A`, `B`...
e a resposta. O app ajusta o modelo de segunda ordem (constante, efeitos principais,
interações de dois fatores e quadrados) e mostra os coeficientes com os valores-p, o R² e o R²
ajustado e o **ponto estacionário**, em unidades codificadas e, quando a planilha traz os
níveis reais, também nelas. Os autovalores dizem o que esse ponto é: todos negativos, um
máximo; todos positivos, um mínimo; sinais trocados, uma sela (a resposta sobe numa direção e
desce noutra); um perto de zero, uma crista. Se o ponto cai fora da região do experimento, o
veredito avisa que é extrapolação, e o ponto deve ser confirmado com novas corridas. Os
gráficos são o |t| de cada termo e a resposta prevista ao longo de cada fator, com os outros
no centro.

Quando o mesmo experimento mede várias respostas (rendimento e custo, por exemplo), o melhor
ponto de uma pode ser ruim para outra. "Desejabilidade de várias respostas" acha o meio-termo.
Preencha cada resposta na sua coluna da planilha. Para cada uma, escolha o objetivo e os
limites. **Maximizar** vai de inaceitável no limite inferior a ideal no superior;
**minimizar** é o contrário; **chegar no alvo** é ideal no alvo e inaceitável fora dos
limites. A **importância** (1 por padrão) pesa a resposta no total. O app ajusta um modelo de
segunda ordem por resposta e procura, dentro da região do experimento, o ponto com a maior
desejabilidade geral (a média geométrica das desejabilidades, de 0 a 1, de Derringer e
Suich). O resultado traz o melhor ponto (codificado e nas unidades reais), a resposta prevista
e a desejabilidade de cada uma, e o R² de cada modelo. Geral 0 quer dizer que nenhum ponto
atende a todos os limites: o ponto mostrado é o que fica menos fora, e vale rever os limites
ou o experimento. O erro vem das réplicas, dos pontos centrais e dos termos
deixados fora do modelo; sem nenhum grau de liberdade para o erro, os valores-p vêm do pseudo
erro padrão de Lenth. Os gráficos são o Pareto dos efeitos, os efeitos principais e um gráfico
de interação para cada interação de dois fatores do modelo (linhas que se cruzam ou se abrem
indicam interação). Como as outras análises, fica guardado como receita.

## Modelo UTSim

A tela **Modelo UTSim** do projeto usa o modelo de simulação do processo como instrumento:
mapear, medir, achar a causa e testar a solução antes de mexer na fábrica.

### Ligação ao modelo

Informe o caminho do arquivo `.ron` e escolha "copiar para a pasta do projeto" (o modelo vai
para `modelos/`, e todos da equipe usam a mesma cópia) ou deixe referenciado no lugar onde
está. A ligação guarda o caminho, o nome, o formato e o **hash** do conteúdo. Quando o arquivo
muda, a tela avisa e lista o que foi feito com a versão antiga (conjuntos simulados, estudos,
análises e o SIPOC gerado); "Atualizar a ligação para a versão atual" passa a usar a nova
versão, e os itens antigos continuam marcados como feitos com outra versão.

### Carta de processo, fluxograma e SIPOC

- A **carta de processo** do caminho principal do modelo aparece na tela, com o símbolo, o
  tempo, a distância e os recursos de cada etapa.
- "Fluxograma (página)", "Carta de processo (página)" e "SIPOC (página)" gravam a página do
  diagrama em `dados/` e a abrem no navegador.
- "Gerar o SIPOC como entregável" cria um SIPOC na fase Definir com fornecedores e entradas
  das fontes, as etapas do caminho principal e as saídas e clientes dos destinos do modelo.
  O belt edita na tela Ferramentas; o entregável fica marcado como gerado do modelo, com o
  hash da versão.

### Replicações e conjuntos simulados

"Rodar as replicações" roda o modelo (as replicações e a semente do próprio modelo) num fio de
trabalho, sem travar a janela, e mostra cada indicador com a média e a meia-largura do
intervalo de 95 %. Marque os indicadores e use "Guardar como conjunto simulado": o conjunto vai
para `dados/` com a origem **Simulado**, o hash do modelo, a semente e o número de replicações.
O rótulo acompanha o conjunto em todas as telas: dado simulado nunca é coleta e nunca é linha de
base real.

### Gargalo e gargalo móvel

Depois das replicações, a tela mostra os recursos pela utilização, o gargalo (maior
utilização) e o gargalo no turno. "Gargalo móvel" roda o modelo com o rastro e dá a fração do
tempo em que cada recurso foi o gargalo único e compartilhado. Os dois podem ser guardados
como análises do projeto, ligados à versão do modelo.

### Estudo de cenários

Cada cenário descreve a solução como mudanças de parâmetros do modelo, uma por linha, no
formato `objeto.campo = valor` (por exemplo `maquina.process = Exp(mean: 0.5)`); o primeiro é
a linha de base. Todos os cenários usam as mesmas sementes (números aleatórios comuns), e cada
indicador escolhido é comparado com a linha de base por **diferença pareada** por replicação,
com intervalo de 95 %. A decisão é:

- **significativa** quando o intervalo não contém zero;
- **relevante** quando é significativa e todo o intervalo fica pelo menos na diferença mínima
  de interesse informada pelo belt (sem mínimo, basta ser significativa).

"Guardar o estudo" grava o pacote `estudo.json` em `dados/`, com o nome e o hash do modelo, os
cenários, a semente, as replicações, a versão do app, os resultados de cada replicação e as
diferenças: qualquer pessoa reproduz o número. O projeto registra o estudo com a conclusão.

"Lançar ganho estimado", ao lado de um estudo guardado, transforma uma diferença significativa
em ganho. Informe a descrição, quanto vale uma unidade do indicador (por exemplo o lucro de uma
peça a mais) e quantas vezes por ano o horizonte simulado acontece (uma semana simulada: 52), e
diga se o aumento do indicador é melhora (vazão) ou se é a redução (lead time, fila). O modo
conservador, marcado de início, usa a ponta do intervalo mais próxima de zero em vez da média.
A conta é melhora × valor por unidade × períodos por ano. O ganho entra em **Ganhos** como
estimado, origem simulação, ligado ao estudo, com a memória de cálculo completa (estudo, hash do
arquivo e do modelo, semente, replicações, diferença com o intervalo e a conta). O app relê o
`estudo.json` e confere o hash antes de lançar: um arquivo mudado depois de guardado é recusado.
Como todo ganho por simulação, ele só passa a validado com lançamentos reais.

### Mapa do fluxo de valor e diagrama de espaguete

"Mapa do fluxo de valor (página)" e "Diagrama de espaguete (página)" rodam o modelo uma vez com
o rastro, num fio de trabalho, gravam a página em `dados/` e a abrem no navegador. O espaguete
desenha sobre a planta todos os deslocamentos de itens, pessoas e veículos; a tela mostra as
viagens e a distância de cada recurso móvel e dos itens, e guarda essas distâncias como análise
do modelo, ligada à versão dele. "Gerar o VSM atual como entregável" cria o mapa do fluxo de
valor do projeto a partir do modelo (veja [Mapa do fluxo de valor](#mapa-do-fluxo-de-valor)).

### Caminho crítico observado

Escolha o objeto de início e o de fim de um trecho do processo: o app roda o modelo e segue o
que realmente segurou cada unidade (espera em fila, trabalho, bloqueio e transportes), com a
criticidade de cada atividade e os caminhos mais frequentes. Responde "o que atrasa este
trecho".

### Abrir no UTSim

"Abrir no UTSim" abre a versão ativa do UTSim instalada pelo launcher com o modelo ligado (ou
o `utsim-view` do PATH, quando não há launcher).

## Programa de implantação (master black belt)

O programa fica acima das carteiras: **programa → unidades de negócio → carteiras →
projetos**, uma carteira por unidade (planta, área ou diretoria). É mais uma pasta com
diário, como a de um projeto, ao lado das pastas das carteiras e dos projetos na mesma pasta
compartilhada. Para criar, use "Novo programa de implantação" na tela inicial (a pasta vai
para o mesmo lugar escolhido em "pasta onde criar"). Abrir a pasta do programa leva às telas
do programa.

O programa procura as carteiras e os projetos ao lado dele, nas pastas ao lado (uma por
unidade, por exemplo `Planta 1/`) e em `carteiras/` dentro dele; os projetos de uma pasta
pertencem à carteira da mesma pasta. Ele lê só o resumo de cada projeto (situação, belts,
tollgates, ferramentas, ganhos), de novo a cada 30 segundos ou em "Ler as pastas de novo", e
mostra o instante dos dados. Projetos marcados como treino nunca entram nos indicadores.

```
Programa LSS da empresa.lss/
  projeto.json                 identificação (a entidade principal é o programa)
  ops/  presenca/              como num projeto
  mentoria/<belt>.lss/         diário de mentoria de cada belt (pasta restrita)
  carteiras/<unidade>/         opcional: carteiras e projetos de cada unidade
Planta 1/
  Carteira.lss/  Refugo na prensa.lss/  ...
```

### Estrutura e metas

- **Unidades de negócio** com champion, patrocinador, número de pessoas (para a cobertura) e a
  carteira ligada (a lista mostra as carteiras encontradas).
- **Objetivos estratégicos** com peso no programa e peso em cada unidade (vazio: o objetivo
  não é desdobrado nela); abaixo da tabela, o peso de cada objetivo na unidade, normalizado.
- **Metas anuais por unidade**: ganho hard validado, ganho soft validado, projetos
  encerrados, belts formados e belts certificados por nível, e cobertura (belts green ou acima
  por 100 pessoas). Definir de novo a mesma meta troca o valor.
- **Limites**: dias entre comitês de seleção (30), projetos simultâneos por nível (yellow 1,
  green 2, black 4, até 4 green belts em mentoria, champion 6 projetos), tempo-alvo da revisão
  de tollgate (45 min) e retenção dos dados pessoais (60 meses).

### Placar

Meta × realizado por unidade e do programa no ano. **Só ganho validado conta no realizado
oficial**: a soma dos lançamentos mensais validados pela controladoria no ano. O previsto ainda
não validado aparece numa coluna à parte e nunca entra no realizado; ganho estimado por
simulação também não. O gráfico compara meta e realizado do ganho hard por unidade.

### Funil e comitê de seleção

As ideias de todas as carteiras passam pelos mesmos estágios: **ideia** (qualquer pessoa),
**triagem** (champion da unidade), **seleção** (comitê) e **abertura** (termo do belt
designado), depois em execução e encerrado. Numa ideia em triagem, "Enviar ao comitê" a põe na
pauta; só o champion da unidade da ideia ou o MBB podem.

A reunião do comitê tem **pauta gerada**: as oportunidades em seleção pela pontuação
ponderada da carteira, a capacidade livre por nível de belt e as metas do ano ainda abertas.
Cada decisão (selecionar, adiar, devolver à triagem, recusar) exige o motivo, entra na ata da
reunião e muda a ideia na carteira dela: selecionar aprova a ideia (com o belt candidato) e
recusar a cancela. Se a carteira recusar a mudança (por exemplo, sem abordagem escolhida), a
decisão não é registrada e a mensagem diz o motivo.

### Capacidade e designação

- **Pessoas do programa**: nível de belt, unidade, área, horas por semana dedicadas ao
  programa, limite próprio de projetos, se precisa de projeto para a certificação, ano de
  formação e data de certificação (a formação e a certificação da etapa L7 vão preencher esses
  dois).
- **Importar pessoas (CSV do RH)**: um CSV com `nome` e `email` e, se houver, `unidade`
  (pelo nome), `area`, `nivel` (yellow, green, black ou master, também "verde", "GB"...) e
  `horas_semana`. A pessoa é achada pelo e-mail: quem já está tem os campos atualizados, e
  quem falta é cadastrado. Ninguém é removido (quem saiu passa pela anonimização). O app
  mostra quantas foram criadas, atualizadas e sem mudança, e cada linha com problema, com o
  número e o motivo. Importar o mesmo arquivo de novo não muda nada.
- **Ausências** com início, fim e motivo; cada pessoa registra as suas.
- **Carga no programa inteiro**: horas por semana das designações ativas mais as horas por
  semana das tarefas abertas do cronograma em que a pessoa é responsável (campo
  `horas_semana` da tarefa), somadas em **todas** as carteiras, porque uma pessoa pode estar
  em projetos de unidades diferentes. A capacidade é das próximas quatro semanas, descontadas as
  ausências. Os avisos de sobrecarga dizem o que passou do limite: projetos, horas, mentorados
  ou projetos acompanhados pelo champion.
- **Designação**: para uma oportunidade em seleção ou aprovada, os belts aparecem em ordem,
  cada um com as razões da posição. Vêm primeiro os que atendem ao nível exigido pela abordagem
  (black belt para DMADV, green para DMAIC, yellow para as abordagens curtas), têm vaga de
  projeto, horas livres e não estão ausentes na maior parte do período; entre eles, quem
  precisa de projeto para a certificação, depois a mesma unidade, a mesma área, a experiência
  nas ferramentas previstas (aplicou ou demonstrou no mapa de competências) e as horas livres.
  É uma ordenação por regras explicadas; quem decide é o MBB, e a decisão fica registrada com a
  posição que a pessoa tinha na sugestão.

### Mentoria e mapa de competências

O **diário de mentoria** de cada belt fica numa pasta própria (`mentoria/<belt>.lss`), vista
pelo app só pelo MBB e pelo próprio belt. Cada sessão tem data, duração, forma (individual,
grupo, revisão no gemba), projeto e fase, temas, ferramentas discutidas e, opcional, uma
avaliação de 1 a 4 por competência. Os **compromissos** de uma sessão vão para a lista pessoal
do belt ou viram cartão no quadro do projeto da sessão ("Virar cartão"), com prazo e o belt
como responsável. O progresso mostra sessões, horas, compromissos cumpridos e a evolução das
avaliações. Sem o consentimento registrado do belt, o diário não guarda sessões dele.

O **mapa de competências** cruza belts e ferramentas com quatro níveis: **E** estudou (módulo
concluído, etapa L7), **P** praticou (exercício correto no modo de ensino, etapa L8), **A**
aplicou (a ferramenta está num entregável concluído de fase aprovada de um projeto real, o app
preenche sozinho) e **D** demonstrou (confirmado pelo MBB em revisão ou mentoria, com a
evidência). Passe o mouse numa célula para ver as evidências. A linha do tempo do belt junta
sessões, tollgates aprovados, ferramentas novas e a certificação.

O **UTSim Ensino** também registra níveis no mapa: quando a pessoa conclui o caso final do
jogo, ele grava **E** estudou ou **P** praticou nas ferramentas do caso, com origem "UTSim
Ensino" e a nota como evidência. Para isso a pessoa usa no Ensino a mesma identidade do UTSim
Seis Sigma e precisa estar no cadastro do programa, ou seja, ter aberto o programa ao menos
uma vez. O próprio registro é a única coisa que o Ensino grava. Ele nunca dá "aplicou" (que vem
de projeto real) nem "demonstrou" (que só o MBB confirma), e a conferência do diário recusa
qualquer tentativa nesse sentido.

O instrutor também registra uma turma inteira a partir do Ensino. Depois de corrigir a turma, ele indica a pasta do programa, e o Ensino grava **P** praticou com origem "exercício" para cada belt que resolveu as missões. O belt é achado pelo e-mail da lista de alunos, comparado com o e-mail do cadastro do programa. Só o instrutor e o MBB podem fazer isso, porque o e-mail é dado pessoal, e uma pessoa anonimizada não é achada.

### Revisões de tollgate

A **fila de revisões** tem todos os tollgates do programa em revisão sem parecer do MBB na
rodada atual: projeto, unidade, belt, fase, data planejada, dias esperando e dias de atraso,
com os vencidos primeiro e em destaque. Agrupe por data planejada ou por unidade para revisar
em lote; "Gerar a ata do lote" registra a reunião com uma linha de decisão por projeto.

O **trabalho padrão** da revisão mostra os itens verificados pelo app já conferidos, as
perguntas padrão da fase (por exemplo, no Medir: "A definição operacional do Y é inequívoca?",
"O MSA cobre o sistema de medição usado na linha de base?", "A linha de base usa dado de origem
Real?"), as perguntas que a empresa acrescentou e o tempo-alvo. O **parecer técnico
estruturado** tem cinco itens (dado suficiente, ferramenta adequada, estatística correta,
conclusões suportadas, riscos), cada um ok, ressalva ou refazer, com comentário obrigatório
quando não for ok; o parecer vale o pior item. O parecer é do programa; a aprovação formal
continua no tollgate do projeto.

A **qualidade das revisões** mostra, por fase e por belt, a taxa de refazer, as ressalvas, a
mediana de dias entre o pedido e o parecer e as ressalvas recorrentes (o mesmo item sem ok em
dois pareceres do mesmo belt), que orientam a mentoria e o currículo.

### Painéis

Os dois painéis têm filtro por unidade e ano, a regra de cada indicador ao lado e o instante
dos dados.

- **Painel do MBB**: funil (contagem por estágio, conversão entre estágios, idade média),
  tempo de ciclo por fase (mediana e percentil 80 dos dias entre aprovações de tollgate, por
  abordagem, contra a meta de cada fase: a padrão é Definir 30, Medir 45, Analisar 45,
  Melhorar 60, Controlar 60 dias, e o MBB troca a de qualquer fase em "Meta de dias por fase",
  logo abaixo da tabela, para o programa todo; vazio volta à padrão), ganhos por estado e unidade com o total oficial só com o validado, belts por área
  (ativos, formados, certificados, cobertura, certificações de três anos a 90 dias do fim),
  uso das ferramentas por fase e as nunca aplicadas, tollgates vencidos por unidade e belt e
  as revisões pendentes, e a carga por nível e por pessoa.
- **Painel da liderança**: seis indicadores com meta e tendência em relação ao ano anterior
  (ganho validado, projetos encerrados, belts certificados, cobertura, oportunidades no funil e
  tollgates vencidos), sem dados pessoais de formação ou mentoria.

O painel da liderança tem também "Relatório anual (PDF)": os seis indicadores do ano com a
meta e o ano anterior, a comparação entre as unidades (tabela e gráficos do ganho validado e dos
projetos encerrados) e, depois, o detalhe do painel do MBB.

Os dois painéis têm "Exportar PDF" e "Exportar planilha (.xlsx)", com o filtro de unidade e o
ano escolhidos. O PDF traz cada indicador em tabela, com a regra dele, o instante dos dados e os
gráficos (funil, tempo de ciclo contra o alvo, ganho validado por unidade), no modelo de
relatório da empresa; a planilha tem uma aba por indicador. Os arquivos vão para a pasta
`relatorios/` do programa. O PDF do painel da liderança, como a tela, não leva dados pessoais.

### Perfis e permissões

| Perfil | Vê | Faz |
|---|---|---|
| MBB | tudo do programa | papéis, metas, unidades, designação, revisão de tollgate, competência demonstrada, decisões do comitê, anonimização, ondas, turmas, currículo, seleção dos alunos e certificação |
| Champion | sua unidade (ou todas, sem unidade no perfil) | triagem (enviar ao comitê), pauta do comitê, aprovação de tollgate |
| Patrocinador | placar | aprovação de termo, tollgate e encerramento |
| Belt | seus projetos e o próprio diário de mentoria | conduz projetos, registra a mentoria recebida |
| Controladoria | ganhos de todo o programa | valida ganhos |
| Instrutor | suas turmas, o banco de questões e as notas | sessões, presença, provas e correção, estudo e prática |
| Liderança | painel da liderança | comentar |
| Aluno | espaço de treino (etapa L8) | exercícios e provas |

Quem cria o programa é MBB desde o início; os demais perfis vêm da **lista de papéis** do
programa (tela Perfis e dados pessoais), mantida pelo MBB. Uma ação fora do perfil é recusada
com a mensagem de quem pode fazê-la (por exemplo, "ação reservada a: MBB (seu perfil: belt)").
E o app confere cada alteração do diário contra o perfil do autor **naquele instante**: uma
alteração de quem não tinha o perfil fica sem efeito e aparece na lista de "operações não
autorizadas". Um perfil dado depois não valida alterações anteriores, e um perfil retirado
deixa de valer dali em diante. Qualquer pessoa registra o próprio nome, e-mail e
consentimento.

**Pastas restritas no Drive.** A conferência do app é a primeira camada; a barreira real é o
compartilhamento das pastas, que o app não confere (não usa a API do Drive):

- compartilhe a pasta do programa com as pessoas do programa;
- cada pasta `mentoria/<belt>.lss` só com o MBB e o belt (no Drive, compartilhe a subpasta com
  essas duas pessoas e tire o acesso herdado dos demais, ou mantenha `mentoria/` fora da pasta
  compartilhada geral);
- valores financeiros sigilosos antes da validação numa pasta `financas/` só da controladoria
  e do MBB; banco de questões, gabaritos e notas nas pastas restritas da formação (veja
  Formação e certificação).

Quem edita os arquivos à mão consegue gravar uma alteração, mas o app a ignora se o autor não
tiver o perfil.

**Assinatura da lista de papéis.** Abaixo da lista, o MBB vê **Assinar a lista**. A assinatura
cobre todos os papéis do momento, com a pessoa, o perfil e a unidade. Depois dela, a tela
mostra "Lista assinada por ... sem mudanças desde então" ou, se alguém incluiu ou retirou um
papel, quais papéis estão fora da lista assinada, até um MBB assinar de novo. Assim um papel
gravado à mão nos arquivos aparece. A primeira assinatura vale quando é do criador do
programa, e as seguintes quando são de um MBB da lista assinada antes. Uma assinatura de
outra pessoa aparece como inválida.

### Dados pessoais

- **Consentimento**: cada pessoa registra o seu, com a finalidade (formação, mentoria e
  designação no programa); o MBB vê quem consentiu.
- **Retenção**: pessoas sem perfil, sem projeto em execução e sem alteração há mais que o prazo
  de retenção aparecem para anonimizar, e quem retirou o consentimento também. Nada é
  anonimizado sozinho: o MBB confirma.
- **Anonimizar** troca o nome por "Pessoa anonimizada" com um código, apaga e-mail, área,
  finalidade e motivos das ausências, e esvazia os textos do diário de mentoria da pessoa; em
  seguida reescreve o histórico desses campos nas pastas do programa e do diário (o único caso
  em que um arquivo do diário muda depois de gravado, mantendo o mesmo identificador), para que
  o valor original não sobreviva em nenhum arquivo. Os projetos guardam o nome na equipe deles
  e não são reescritos.

## Relatórios

A tela **Relatórios** do projeto faz os PDF no próprio programa, sem navegador nem nada a
instalar: termo de abertura, A3 (folha A3 deitada), relatório de tollgate de cada fase (estado,
rodada, hash congelado, checklist com as evidências, entregáveis da fase, decisões e
ressalvas), ata de cada reunião, FMEA e plano de controle de cada entregável (A3 deitada). O
certificado de cada certificação sai na tela Certificação do programa. Os arquivos vão para a
pasta `relatorios/` do projeto (ou `certificacoes/` do programa) e abrem no leitor de PDF do
computador; refazer um relatório substitui o anterior.

Os relatórios longos (A4 com três seções ou mais, como o do tollgate e o relatório anual do
programa) abrem com uma **capa** (o app, o título, o projeto ou programa, quem imprimiu e
quando) e um **sumário** com a página de cada seção. As tabelas têm o cabeçalho na cor do
modelo da empresa e linhas alternadas, e uma seção nunca começa sozinha no pé da página.

Os PDF trazem **gráficos** desenhados pelo próprio programa (linhas e barras, sem imagem):

- cada análise guardada em Dados e análises tem o seu relatório, com o conjunto e a origem, o
  hash, a conclusão, a tabela dos números e o gráfico: a carta de controle com a linha central,
  os limites e os pontos com sinal em vermelho; num experimento, o Pareto dos efeitos, os
  efeitos principais e as interações;
- o A3 mostra o gráfico de cada Pareto do projeto, com a linha dos 80 %;
- cada mapa do fluxo de valor tem o seu relatório, com os processos, os indicadores (e os do
  estado atual, num mapa futuro) e a escada do lead time em barras de espera e de processo.

- **Planilha** `.xlsx` com uma aba por tabela: ações, riscos, ganhos, cada FMEA e cada plano de
  controle, e os conjuntos de dados; números como números.
- **CSV** das mesmas tabelas, separado por ponto e vírgula.
- **Modelo da empresa**: nome da empresa, linha de cabeçalho, rodapé e cor de destaque.
  "Salvar o modelo" grava `modelos/relatorio.json` na pasta ao lado dos projetos, e todos os
  projetos dessa pasta passam a imprimir com ele; um `modelos/relatorio.json` dentro do projeto
  vale só para ele. O modelo tem versão.
- A **base de conhecimento** (lições aprendidas com busca nos projetos da pasta) e o arquivo
  `.lssz` continuam como antes.

## Formação e certificação

As telas **Formação: ondas, turmas e seleção**, **Currículo, sessões e presença**, **Provas e
correção**, **Certificação** e **Minha formação** ficam na pasta do programa. O app sugere e
confere; quem decide é o MBB.

### Ondas, turmas e escolha dos candidatos

- Uma **onda** é um ciclo do programa (um semestre, por exemplo) com metas de formação por
  nível. Cada **turma** tem nível (yellow, green ou black belt), onda, datas, local ou forma,
  vagas, instrutores e as mínimas de frequência (padrão 80 %) e de nota (padrão 70 %), escritas
  como fração (0,8 = 80 %).
- **Indicação**: pelo gestor, pelo champion ou pela própria pessoa, com justificativa, função,
  tempo na empresa, desempenho declarado e horas disponíveis. Green e black belt precisam de
  uma oportunidade candidata do funil, que é o projeto de formação.
- **Critérios de indicação** da empresa: tempo na empresa, função, nível anterior concluído,
  desempenho declarado pelo gestor, horas disponíveis e unidade com meta aberta, cada um
  eliminatório ou classificatório com peso. "Criar os critérios padrão" grava seis meses de
  empresa e, para black belt, o green belt concluído (eliminatórios), e desempenho (peso 2),
  8 horas disponíveis e unidade com meta aberta (classificatórios).
- **Aprovação do gestor imediato**: só o gestor indicado aprova ou recusa (com motivo),
  nunca a própria pessoa; o MBB pode decidir no lugar dele. Aprovar compromete as horas de aula
  e de projeto no período. A indicação só é alterada por quem a fez (ou pelo MBB), e a própria
  pessoa pode desistir.
- **Capacidade**: o app soma as horas por semana da turma e as do projeto de formação (padrão
  do nível) e confere com as horas livres da pessoa na tela Capacidade e designação (ou com as
  horas disponíveis da indicação, quando a capacidade não está cadastrada).
- **Seleção**: escolhida a turma, a tela mostra a proposta: elegíveis (aprovados pelo gestor,
  sem falhar critério eliminatório e com capacidade) até as vagas, os demais elegíveis na lista
  de espera pela pontuação dos classificatórios, e os que ficam fora com o motivo. "Confirmar a
  seleção" grava a decisão do MBB; quando alguém desiste, "Chamar o próximo da lista de espera"
  dá a vaga ao primeiro. Alunos de green e black belt sem projeto aparecem na pauta do comitê de
  seleção como pedidos de projeto de certificação.

### Equipes de formação

"Sugerir equipes" distribui os alunos da turma em grupos de exercício: tamanhos equilibrados,
no máximo N pessoas da mesma área por grupo (padrão 5 por grupo e 2 da mesma área, ajustáveis
na tela) e a menor repetição de unidade, área e função. O método é guloso com troca local e dá
a mesma distribuição para a mesma entrada; cada restrição que sobra é explicada (por exemplo,
aluno de green belt sem projeto candidato ou sem champion). O MBB usa a sugestão, move pessoas
entre os grupos e confirma.

### Currículo, instrutores e presença

- **Currículo por nível**: "Criar a estrutura padrão" grava os módulos do nível pelas fases do
  DMAIC (yellow belt 16 h, green belt 80 h, black belt 96 h), cada um com carga horária,
  objetivos, pré-requisito e as **ferramentas do app** que ensina. A empresa edita os módulos,
  as cargas e as ferramentas e anexa o próprio material. As cargas padrão são uma proposta.
- **Instrutores**: cadastro com os módulos que cada um pode dar, horas dadas no ano (contam
  para a renovação da certificação do próprio instrutor) e sessões; uma sessão dada por quem
  não é qualificado para o módulo aparece em destaque.
- **Sessões e presença**: na turma escolhida, cada sessão tem data, horas e módulos; a presença
  é presente, ausente ou justificado (a ausência justificada continua sendo ausência para a
  frequência, registrada com o motivo) e a caixa "ex." marca o exercício da sessão feito
  corretamente. Quem esteve em todas as sessões de um módulo fica com **estudou** nas
  ferramentas dele no mapa de competências; o exercício correto dá **praticou**. Um módulo dado
  antes do pré-requisito aparece como aviso.

### Provas

- **Banco de questões** (pasta restrita `formacao/questoes.lss`, com o gabarito): múltipla
  escolha, verdadeiro ou falso, resposta numérica com tolerância e **problema prático**. No
  problema prático a resposta esperada é calculada pelo app a partir dos dados de cada caderno,
  com as mesmas funções das telas de estatística (média, desvio-padrão, Cpk com sigma de curto
  prazo pela amplitude móvel, Ppk, DPMO), nunca digitada. "Questões de exemplo dos módulos"
  grava algumas questões para começar.
- **Sorteio**: a prova escolhe os módulos e quantas questões fáceis, médias e difíceis; a
  semente fica registrada e a mesma semente dá a mesma prova. A ordem das questões e os dados
  do problema prático variam por aluno, gerados pela semente.
- **Aplicação no app**: "Publicar os cadernos" grava na pasta da turma
  (`formacao/turmas/<turma>.lss`) um caderno por aluno, sem gabarito. O aluno abre **Minha
  formação**, "Fazer a prova", responde e entrega; depois de entregue, o caderno não muda. As
  respostas voltam como alterações do próprio aluno e são corrigidas na máquina do instrutor
  ("Corrigir as entregues"), que refaz o caderno pela semente e confere com o banco. Números
  valem dentro da tolerância, com os limites incluídos; a vírgula decimal é aceita.
- **Aplicação em papel**: o instrutor lança a nota de cada aluno (em %).
- **Notas** (pasta restrita `formacao/notas.lss`): nota, aprovação pela nota mínima da prova
  (padrão 70 %) e o detalhe de cada questão. O aluno vê a própria nota pela pasta da turma (só
  a nota e o resultado, sem o detalhe). Cabe **uma reaplicação** por prova ("reaplicação de"
  no sorteio); vale a melhor tentativa.
- Sem servidor, a prova no app não impede consulta. A empresa decide se ela vale para a
  certificação (caixa "vale para certificação" de cada prova, decisão MB3).
- **Análise dos itens**: depois da correção, a turma mostra cada questão com a dificuldade
  (quantos acertaram) e a discriminação (acertos dos 27 % melhores menos os dos 27 % piores,
  pelo resto da prova, e a correlação item-resto). O app aponta a questão fácil demais (mais
  de 90 % acertam), difícil demais (menos de 20 %), que discrimina pouco (abaixo de 0,20) e a
  que os mais fracos acertam mais que os mais fortes, sinal de enunciado confuso ou gabarito
  errado. Com menos de 10 provas os números aparecem, mas o app não julga.
- **Avaliação de reação**: em "Minha formação", cada aluno avalia a turma uma vez, de 1 a 5,
  em cinco afirmações (utilidade, clareza do instrutor, exercícios, carga horária e se
  recomendaria), com comentário opcional, na pasta da turma, como as respostas da prova. Na
  tela Turma a equipe vê só o resumo (média, quantos concordam e os comentários em ordem
  alfabética), a partir de três respostas, nunca a resposta de uma pessoa.

### Requisitos e registro de certificação

A tela **Certificação** confere os requisitos de um nível para uma pessoa com os dados do
programa. Tabela padrão (decisão MB4), que a empresa edita em "Editar os requisitos deste
nível":

| Requisito | Yellow | Green | Black |
|---|---|---|---|
| Formação do nível concluída com a frequência mínima | sim | sim | sim |
| Prova aprovada | sim | sim | sim |
| Projetos encerrados com todos os tollgates aprovados | participação em 1 | 1 como líder | 2 como líder |
| Ganho validado pela controladoria nos projetos liderados | não | valor da empresa | valor da empresa |
| Ferramentas em aplicou ou demonstrou | 3 | lista mínima | lista mínima |
| Mentoria dada (green belt acompanhado até o encerramento) | não | não | 1 |
| Parecer do MBB | não | sim | sim |

- Cada requisito mostra **atende**, **não atende** ou **ambíguo** com o motivo, e guarda a
  evidência: o identificador e o hash do item que o comprova (a turma com a frequência, a nota
  da prova, o projeto com os hashes dos tollgates, os ganhos validados, as ferramentas do mapa).
  Ambíguo vai para o MBB: ganho só previsto ou realizado sem validação, ou projeto com
  entregável alterado depois da aprovação do tollgate. Só o ganho validado conta; ganho
  estimado por simulação nunca conta, e projetos de treino não contam.
- O valor de ganho padrão é "algum ganho validado"; a empresa grava o seu. As listas mínimas
  padrão são, para green belt, SIPOC, plano de coleta, Ishikawa, FMEA e plano de controle, e
  para black belt também VOC/CTQ, matriz causa-efeito e plano de resposta.
- A mentoria dada vem das designações em que a pessoa é a mentora de um green belt cujo
  projeto encerrou com todos os tollgates aprovados.
- O MBB registra o **parecer**; com tudo atendido, "Emitir a certificação" grava o registro:
  número sequencial do programa, pessoa, nível, data, validade de **três anos**, os requisitos
  com as evidências e o emissor. O nível e a data de certificação da pessoa são atualizados.
- **Master black belt** e certificações anteriores ao programa são registradas à mão, com a
  observação de em que se apoiam.
- **Validade e renovação** (decisão MB5): aviso 90 dias antes do fim; vencida aparece como tal
  aqui e nos painéis. A renovação vale quando, no período da certificação, a pessoa tem
  qualquer uma das regras da empresa (padrão: 1 projeto encerrado como líder, 1 green belt
  acompanhado até o encerramento ou 16 h de formação dadas). A nova certificação tem o número
  seguinte e começa no dia seguinte ao fim da anterior. Uma certificação pode ser revogada, com
  o motivo.
- **Funil de certificação** (também no painel do MBB): indicados, aprovados pelo gestor,
  selecionados, formados, com projeto e certificados, por onda e nível, com o motivo das
  saídas.

O botão "PDF" de cada certificação do registro grava o certificado em `certificacoes/`.

**Certificados assinados.** Ao lado de cada certificação emitida aparece se ela está assinada
pelo emissor. O emissor, no computador com a sua chave, vê **Assinar**. A assinatura cobre o
número, a pessoa, o nível, as datas, os requisitos com as evidências e a observação: se algo
mudar depois, a tela mostra que a assinatura é de outra versão. O PDF de um certificado
assinado traz a linha da assinatura eletrônica, com o emissor, a data, o início da chave e o
selo, e diz onde conferi-la. Como as demais assinaturas do app, ela revela alterações, mas não
tem a validade jurídica de uma assinatura digital certificada.

### Pastas restritas da formação

```
Programa LSS.lss/
  formacao/questoes.lss/        banco de questões e gabaritos: só MBB e instrutores
  formacao/notas.lss/           notas: só MBB e instrutores
  formacao/turmas/<turma>.lss/  cadernos sem gabarito, respostas e a nota de cada aluno
```

Cada pasta tem o próprio diário, criado pelo MBB ou por um instrutor na primeira gravação. No
Drive, compartilhe `questoes.lss` e `notas.lss` só com o MBB e os instrutores, e cada pasta de
turma só com eles e os alunos da turma (quem abre os arquivos da pasta da turma vê as notas dos
colegas; o app mostra a cada aluno só a sua). O app confere quem grava: na pasta da turma cada
aluno só grava as próprias respostas, e nada depois de entregar.

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
  mensagem "projeto no formato N; esta versão do UTSim LSS lê até M": atualize pelo
  launcher.

## Levar o projeto para quem não está na pasta

Em **Dados**, "Exportar .lssz" grava o projeto inteiro num arquivo só (por padrão ao lado
da pasta do projeto). O arquivo leva o diário, os instantâneos, os dados e os anexos, e não
leva a presença. Quem recebe usa "Importar um arquivo .lssz" na tela inicial. O projeto
importado é uma cópia: as mudanças feitas nele não voltam para a pasta original.

## O que vem depois

Com as telas de estatística e o MSA, o MVP do plano está completo (L0 a L7 com P1). O modo de ensino
(projetos-exemplo, exercícios corrigidos, casos) será um app próprio baseado no UTSim, fora do
UTSim LSS; o mapa de competências já recebe dele as evidências de prática. A versão 2
trouxe o ganho estimado lançado do estudo de cenários, o MSA completo, o planejamento de
experimentos de dois níveis, o mapa do fluxo de valor e o espaguete, os gráficos nos PDF e a
exportação dos painéis. A P6 trouxe a seleção de soluções (ponderada e Pugh), a auditoria 5S, o SMED, o
trabalho padronizado com o balanceamento, o evento kaizen e os ciclos PDCA. Ficam para depois
Plackett-Burman, superfície de resposta e desejabilidade no DOE (no UTSim Estatística) e o espaguete desenhado sem modelo. A P7 trouxe
as assinaturas da equipe: a chave de cada pessoa, a cadeia assinada do diário conferida na
tela Equipe e ao abrir o projeto, as aprovações assinadas de tollgates e ganhos, e a visão
"minhas ações" de toda a carteira. Ficam para depois o conflito por parágrafo nos textos
longos e o acesso pela rede local com token. A P9 completou o programa com a lista de papéis
assinada pelo master black belt e os certificados assinados pelo emissor. Da gestão do projeto ficam para depois raias no quadro,
reuniões recorrentes e a capacidade dos belts em horas.

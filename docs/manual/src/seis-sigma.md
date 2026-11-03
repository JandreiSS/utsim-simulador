# UTSim Seis Sigma (gerenciador de projetos Lean Seis Sigma)

O UTSim Seis Sigma é o aplicativo da família UTSim para conduzir projetos de melhoria Lean
Seis Sigma: termo de abertura, fases e tollgates, quadro de ações, riscos, reuniões,
cronograma, ganhos e as ferramentas de cada fase. É um programa de janela própria (não abre o
navegador), funciona sem internet e não precisa de servidor: a equipe trabalha numa pasta
compartilhada do Google Drive. O nome ainda é provisório.

Já funcionam a **fundação** (etapa L0 do `docs/plano_lean_seis_sigma.md`: o formato do
projeto, o trabalho em equipe pela pasta compartilhada, a visão geral e o quadro de ações) e a
**gestão do projeto** (etapa L1: carteira com banco de ideias e priorização, termo de abertura
com versões, equipe e matriz RACI, fases e tollgates, riscos, reuniões e atas, cronograma com
Gantt e PERT, ganhos, encerramento e lições aprendidas). As ferramentas de cada fase
(etapas L2 e L3) aparecem na navegação com o que vão trazer.

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
- **Ferramentas** (etapas L2 e L3) e **Dados** (etapa L3) mostram o que vão trazer. Em
  **Dados** já está a exportação `.lssz`.

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
  de uma ferramenta (FMEA, SIPOC, Ishikawa, plano de coleta, plano de controle), que passa a
  valer quando a ferramenta da etapa L2 existir no projeto.
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

Pela ordem do plano: ferramentas qualitativas (VOC, SIPOC, Ishikawa, FMEA com a tabela de ação
prioritária, plano de controle, A3), estatística (sobre o `utsim-quality`), integração com o
modelo de simulação, os relatórios em PDF e os modelos da empresa, e o programa de implantação
do master black belt (várias carteiras, comitê de seleção, designação de belts, formação).
Da gestão do projeto ficam para depois a visão "minhas ações" de toda a carteira, raias no
quadro, reuniões recorrentes e a capacidade dos belts em horas.

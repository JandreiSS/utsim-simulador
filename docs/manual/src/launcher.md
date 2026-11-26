# Launcher

O UTSim Launcher instala, atualiza e abre os aplicativos da família UTSim: o editor (UTSim),
a prévia 3D e os que vierem depois. Ele baixa cada versão das páginas de downloads da família
no GitHub, confere a assinatura digital e instala na sua pasta de usuário. Não é preciso ser
administrador, nem código de acesso: todos os aplicativos estão abertos para todos.

## Instalar o launcher

1. Na página de [Releases do launcher](https://github.com/JandreiSS/utsim-launcher/releases),
   baixe o instalador (`.exe`) da versão mais nova.
2. O Windows pode avisar que o programa não é reconhecido (o launcher ainda não tem
   assinatura de código). Clique em "Mais informações" e depois em "Executar assim mesmo".
3. Siga o assistente. Ao final, o launcher abre.

Ao abrir, o launcher mostra uma animação curta (as engrenagens e o nome UTSIM) enquanto já
busca a lista de aplicativos. Qualquer tecla ou clique pula a animação; para não vê-la mais,
desmarque "Mostrar animação de abertura" no menu da engrenagem (canto superior direito). Com
os efeitos de animação desligados no Windows, ela não aparece.

## Primeiro uso

Não há nada a configurar: o launcher já sabe onde fica a lista de versões e mostra todos os
aplicativos. Quem usava um launcher antigo com código de testador não precisa fazer nada: o
código guardado continua lá, mas não é mais pedido nem mostrado.

Se o autor pedir, preencha também o campo "Avançado" com o endereço da lista de versões.

## Instalar e atualizar um aplicativo

Os aplicativos aparecem como cartões (**grade**) ou como uma tabela (**lista**): escolha nos
dois botões quadrados à direita da barra. Ao lado ficam a busca ("Buscar aplicativos") e a
ordem ("Nome (A–Z)", "Mais recentes" ou "Com atualização"). O launcher lembra a vista e a ordem.

Cada cartão mostra, no alto, a versão instalada e a nova (por exemplo `0.1.0 → 0.2.0`) e o
link "NOTAS ↗" com as novidades; embaixo, a situação:

| Marca | Significa | Botão |
|---|---|---|
| ATUALIZAÇÃO DISPONÍVEL | há uma versão mais nova para você | "Atualizar para ..." |
| INSTALADO | está em dia | "Abrir" |
| NÃO INSTALADO | você pode instalar | "Instalar" |
| SEM VERSÃO PARA ESTE SISTEMA | não há pacote para o seu sistema operacional | (nenhum) |

1. Clique em "Instalar" ou "Atualizar para ...". A barra fina sob o botão (e no rodapé) mostra
   o progresso. Se a conexão cair, ou se você clicar em "Cancelar", o download continua de
   onde parou na próxima vez. "Atualizar todos" instala, uma depois da outra, todas as
   atualizações disponíveis.
2. Quando terminar, clique em "Abrir". O launcher também cria um atalho no Menu Iniciar, na
   pasta "UTSim Launcher" (e na área de trabalho, se você marcar a opção no menu da
   engrenagem).

O botão "…" de cada aplicativo tem o resto:

- **Versões**: todas as versões publicadas do aplicativo (não há mais canais): a ativa, as instaladas ("Usar a ..." volta para uma delas) e as que você pode
  instalar, incluindo as anteriores;
- **Abrir a pasta do aplicativo**, **Remover versões antigas** e **Desinstalar...** (pede
  confirmação; os seus modelos, salvos em outras pastas, não são tocados).

O launcher guarda a versão ativa e mais uma anterior. O rodapé diz quando a lista foi
verificada ("VERIFICADO HÁ 10 MIN · VIA GITHUB") ou se ela é a guardada da última vez ("LISTA
GUARDADA · SEM CONEXÃO"); o botão redondo à esquerda da barra busca de novo.

## Quando algo dá errado

Mensagens comuns:

| Mensagem | O que fazer |
|---|---|
| "esta versão antiga só abre com código de testador" | É de antes de os aplicativos ficarem abertos; escolha uma versão mais nova |
| "o autor ainda não publicou o link deste pacote" | A versão acabou de sair; tente mais tarde |
| "este manifesto usa o formato N, mais novo..." | Peça ao autor o instalador novo do launcher |
| "assinatura inválida" ou "o pacote está corrompido ou foi alterado" | Não instale nada; avise o autor |

Para relatar um erro, use "Gerar relato de erro" no menu da engrenagem. O launcher salva um arquivo
`utsim-relato-<data>.zip` na sua pasta Downloads, com um resumo do computador, as versões
instaladas e os registros do launcher. Mande esse arquivo ao autor por
e-mail ou mensagem, contando o que você estava fazendo.

## Onde ficam os arquivos

| O quê | Pasta |
|---|---|
| O launcher | `%LOCALAPPDATA%\Programs\UTSim Launcher` |
| Aplicativos instalados | `%LOCALAPPDATA%\UTSim\launcher\produtos` |
| Registros | `%LOCALAPPDATA%\UTSim\launcher\logs` |

Para desinstalar o launcher, use "Configurações > Aplicativos > Aplicativos instalados >
UTSim Launcher". O desinstalador pergunta se deve remover também os aplicativos instalados por
ele.

## Para o autor: publicar

Desde 25/11/2026 todo aplicativo é publicado aberto para todos. O pacote continua cifrado (o
formato que os launchers antigos leem), mas a chave dele vai no manifesto, e é a assinatura do
manifesto que garante que o pacote veio do autor. Não há mais canais nem códigos de testador.

1. `scripts/publicar-<app>.sh <pasta_de_publicação>` gera o pacote e o manifesto local
   (`utsim-publicar publicar`, aberto por padrão; `--restrito` volta ao modo com código).
2. `utsim-publicar github --saida <pasta> --produto <id> --versao <versão>` sobe o pacote como
   release do repositório do aplicativo, deixa as 3 versões mais novas e envia o manifesto
   assinado para a release `manifesto` de
   [utsim-launcher](https://github.com/JandreiSS/utsim-launcher).

A chave privada fica só em `%APPDATA%\UTSim\chaves` (crie uma vez com
`utsim-publicar chaves gerar`). O **UTSim Admin** (`cargo build --release -p utsim-admin`)
continua como ferramenta opcional do autor, para ver as publicações e as chaves; os testadores
e o acesso por canal que ele mostra só valem para versões antigas.

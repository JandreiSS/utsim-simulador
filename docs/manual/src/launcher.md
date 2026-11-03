# Launcher (para testadores)

O UTSim Launcher instala, atualiza e abre os aplicativos da família UTSim: o editor (UTSim),
a prévia 3D e os que vierem depois. Ele baixa cada versão do Google Drive do autor, confere a
assinatura digital e instala na sua pasta de usuário. Não é preciso ser administrador.

## Instalar o launcher

1. Abra o link do instalador que o autor mandou e baixe
   `utsim-launcher-v<versão>-windows-x64-instalador.exe`.
2. O Windows pode avisar que o programa não é reconhecido (o launcher ainda não tem
   assinatura de código). Clique em "Mais informações" e depois em "Executar assim mesmo".
3. Siga o assistente. Ao final, o launcher abre.

Ao abrir, o launcher mostra uma animação curta (as engrenagens e o nome UTSIM) enquanto já
busca a lista de aplicativos. Qualquer tecla ou clique pula a animação; para não vê-la mais,
desmarque "Mostrar animação de abertura" no menu da engrenagem (canto superior direito). Com
os efeitos de animação desligados no Windows, ela não aparece.

## Primeiro uso: o código de acesso

O autor mandou um código só seu, como `K7QD-3M9X-TR2A-HH4P-W8ZE`. Digite-o na tela do código
e clique em "Continuar". Maiúsculas, minúsculas e espaços não importam, e o launcher avisa na
hora se houver erro de digitação.

- O código fica guardado só no seu computador, na sua pasta de usuário.
- Não repasse o código: ele identifica você, e o autor pode revogá-lo.
- Para inserir ou trocar o código, use o menu da engrenagem ("Inserir código" ou "Trocar
  código") ou o botão "Inserir código" de um aplicativo marcado "Sem acesso".

Quando o autor abre um aplicativo **para todos**, ele aparece com a marca "Aberto para todos" e
instala sem código. Se todos os aplicativos estiverem abertos, o launcher nem pede o código;
se algum pedir, a tela do código aparece uma vez ("Agora não" mostra os aplicativos assim
mesmo).

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
| SEM ACESSO | o seu código não dá acesso (ou você ainda não tem código) | "Inserir código" |
| SEM VERSÃO PARA ESTE SISTEMA | não há pacote para o seu sistema operacional | (nenhum) |

1. Clique em "Instalar" ou "Atualizar para ...". A barra fina sob o botão (e no rodapé) mostra
   o progresso. Se a conexão cair, ou se você clicar em "Cancelar", o download continua de
   onde parou na próxima vez. "Atualizar todos" instala, uma depois da outra, todas as
   atualizações disponíveis.
2. Quando terminar, clique em "Abrir". O launcher também cria um atalho no Menu Iniciar, na
   pasta "UTSim Launcher" (e na área de trabalho, se você marcar a opção no menu da
   engrenagem).

O botão "…" de cada aplicativo tem o resto:

- **Canal**: **estável** (versões já testadas) ou **testes** (versões novas, que podem ter
  problemas; as estáveis continuam aparecendo);
- **Versões**: a ativa, as instaladas ("Usar a ..." volta para uma delas) e as que você pode
  instalar, incluindo as anteriores;
- **Abrir a pasta do aplicativo**, **Remover versões antigas** e **Desinstalar...** (pede
  confirmação; os seus modelos, salvos em outras pastas, não são tocados).

O launcher guarda a versão ativa e mais uma anterior. O rodapé diz quando a lista foi
verificada ("VERIFICADO HÁ 10 MIN · VIA DRIVE") ou se ela é a guardada da última vez ("LISTA
GUARDADA · SEM CONEXÃO"); o botão redondo à esquerda da barra busca de novo.

## Quando algo dá errado

Mensagens comuns:

| Mensagem | O que fazer |
|---|---|
| "o Google Drive limitou os downloads deste arquivo" | Muita gente baixou ao mesmo tempo. Tente de novo em algumas horas |
| "este código de acesso não dá acesso a este pacote" | Confira o código ("Trocar código") ou fale com o autor |
| "o autor ainda não publicou o link deste pacote" | A versão acabou de sair; tente mais tarde |
| "este manifesto usa o formato N, mais novo..." | Peça ao autor o instalador novo do launcher |
| "assinatura inválida" ou "o pacote está corrompido ou foi alterado" | Não instale nada; avise o autor |

Para relatar um erro, use "Gerar relato de erro" no menu da engrenagem. O launcher salva um arquivo
`utsim-relato-<data>.zip` na sua pasta Downloads, com um resumo do computador, as versões
instaladas e os registros do launcher (nunca o seu código). Mande esse arquivo ao autor por
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

## Para o autor: o administrador

O autor gerencia testadores, publicações e acesso pela janela **UTSim Admin**
(`target\release\utsim-admin.exe`, gerada com `cargo build --release -p utsim-admin`). Ela
nunca vai para os testadores nem entra no instalador do launcher, e usa os mesmos arquivos da
linha de comando `utsim-publicar` (`%APPDATA%\UTSim`). Sem a chave privada ela só explica como
criá-la (`utsim-publicar chaves gerar`, uma vez).

- **Testadores**: "Adicionar" cria o código e o mostra uma vez, com o botão "Copiar"; envie-o só
  para a pessoa. "Canais" edita os canais; "Revogar" e "Reativar" valem para as próximas
  publicações.
- **Publicações**: produtos, versões e, de cada pacote, o link e quem tem acesso; "Verificar"
  confere assinatura, links e arquivos; "Refazer acesso" reembrulha as versões publicadas para
  os testadores ativos agora.
- **Acesso**: para cada produto e canal, "Com código" (padrão) ou "Aberto para todos". Aberto,
  qualquer launcher instala sem código, já nas versões publicadas. Ao voltar para "Com código",
  as versões publicadas enquanto estava aberto continuam abrindo para quem baixou a lista nesse
  período (a chave delas foi pública): só uma versão nova fica fechada de fato. O mesmo pela
  linha de comando: `utsim-publicar acesso --saida <pasta> --produto <id> --canal testes aberto`
  (ou `codigo`).
- **Chaves**: a chave pública e o id dela (a privada nunca aparece).

A pasta de publicação é escolhida no alto da janela (a primeira sugestão é
`G:\Meu Drive\UTSim - testes`) e fica lembrada.

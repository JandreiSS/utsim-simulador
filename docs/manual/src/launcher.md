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

## Primeiro uso: o código de acesso

O autor mandou um código só seu, como `K7QD-3M9X-TR2A-HH4P-W8ZE`. Digite-o na primeira tela
e clique em "Continuar". Maiúsculas, minúsculas e espaços não importam, e o launcher avisa na
hora se houver erro de digitação.

- O código fica guardado só no seu computador, na sua pasta de usuário.
- Não repasse o código: ele identifica você, e o autor pode revogá-lo.
- Para trocar de código (por exemplo, se o autor mandar outro), use "Trocar código".

Se o autor pedir, preencha também o campo "Avançado" com o endereço da lista de versões.

## Instalar e atualizar um aplicativo

1. Escolha o aplicativo na lista à esquerda.
2. Escolha o canal:
   - **estável**: versões já testadas;
   - **testes**: versões novas, que podem ter problemas (as estáveis continuam aparecendo).
3. Escolha a versão (a mais nova fica no topo) e leia as notas ao lado.
4. Clique em "Instalar" ou "Atualizar para ...". A barra mostra o progresso. Se a conexão
   cair, ou se você clicar em "Cancelar", o download continua de onde parou na próxima vez.
5. Quando terminar, clique em "Abrir". O launcher também cria um atalho no Menu Iniciar, na
   pasta "UTSim Launcher" (e na área de trabalho, se você marcar a opção).

Ao lado de cada versão aparecem marcas:

- **ativa**: a que abre pelo atalho;
- **instalada**: já está no computador; "Usar a ..." volta para ela;
- **sem acesso**: o seu código não dá acesso a essa versão (por exemplo, ela é de um canal
  que não está liberado para você);
- **sem pacote para este sistema**: não há versão para o seu sistema operacional.

O launcher guarda a versão ativa e mais uma anterior. "Remover versões antigas" apaga as
anteriores; "Desinstalar" remove o aplicativo inteiro (os seus modelos, salvos em outras
pastas, não são tocados).

## Quando algo dá errado

Mensagens comuns:

| Mensagem | O que fazer |
|---|---|
| "o Google Drive limitou os downloads deste arquivo" | Muita gente baixou ao mesmo tempo. Tente de novo em algumas horas |
| "este código de acesso não dá acesso a este pacote" | Confira o código ("Trocar código") ou fale com o autor |
| "o autor ainda não publicou o link deste pacote" | A versão acabou de sair; tente mais tarde |
| "este manifesto usa o formato N, mais novo..." | Peça ao autor o instalador novo do launcher |
| "assinatura inválida" ou "o pacote está corrompido ou foi alterado" | Não instale nada; avise o autor |

Para relatar um erro, clique em "Gerar relato de erro". O launcher salva um arquivo
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

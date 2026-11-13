# UTSim Ensino

O UTSim Ensino é o app de ensino da família UTSim. O aluno aprende engenharia de produção resolvendo problemas de uma fábrica simulada: lê a situação, faz uma previsão, decide dentro do orçamento e manda simular. A nota vem da própria simulação, com replicações e intervalo de confiança. Ninguém digita gabarito.

O app é instalado pelo launcher, como o UTSim e o UTSim Seis Sigma. Ele abre em computadores de laboratório sem placa de vídeo.

## Primeiro uso

Na primeira abertura o app pede o nome e o registro acadêmico. O registro identifica as entregas e gera a **variante** do aluno: os parâmetros de cada missão mudam um pouco de um aluno para outro, sempre os mesmos para o mesmo registro. Use o mesmo registro da lista do professor.

Em computadores compartilhados, o perfil pode ficar numa pasta escolhida (um pen drive, por exemplo). Para isso, abra o app com a variável `UTSIM_ENSINO_PERFIL` apontando para a pasta.

### Idioma

O programa fala português, inglês e espanhol. A língua se escolhe em **Ajustes**, em "Idioma", vale na hora e fica guardada no perfil. Na linha de comando, `--idioma en` (ou `es`, `pt-BR`) escolhe a língua daquela abertura. A língua muda os textos do programa (telas, botões, mensagens, ajuda da linha de comando e o modo missão do UTSim 3D). As missões aparecem na língua em que o pacote foi escrito, informada no campo `idioma` do `pacote.ron` (português quando falta), e o mapa avisa quando ela é outra. A campanha embutida está em português. Os relatórios do construtor de casos e as mensagens que vêm do núcleo também continuam em português.

## A campanha

A campanha tem atos, e cada ato tem missões em ordem. Cada missão resolvida libera a próxima. A primeira missão de um ato abre quando todas as do ato anterior estão resolvidas.

| Ato | Tema | Missões |
|---|---|---|
| 1 | Gargalo e fluxo | 1.1 A furadeira que segura a linha · 1.2 Reforçar o lugar errado · 1.3 Quanto tem no chão (lei de Little) · 1.4 O gargalo que muda |
| 2 | Filas e atendimento | 2.1 Balcão da ferramentaria · 2.2 Quem desiste vai embora · 2.3 Urgente na frente · 2.4 Mesma média, outra fila · 2.5 A central de atendimento |
| 3 | Capacidade, turnos e estoque intermediário | 3.1 A demanda dobrou · 3.2 Pulmão certo · 3.3 Corretiva ou preventiva · 3.4 Lote e setup · 3.5 A expansão da linha de eixos |
| 4 | Balanceamento de linha | 4.1 Montagem do ventilador · 4.2 O balanceamento no papel e no chão · 4.3 A demanda subiu |
| 5 | Arranjo físico e movimentação | 5.1 Oficina de usinagem · 5.2 Giro perto da doca · 5.3 Frota de transporte |
| 6 | Tempos e métodos | 6.1 Quantas máquinas por operador · 6.2 Trabalho externo · 6.3 Célula com operadores · 6.4 Cronometragem |
| 7 | Problemas integrados | 7.1 Linha com inspeção e retrabalho · 7.2 Pronto atendimento por turnos · 7.3 Reembolso no escritório · 7.4 Centro de distribuição: pickers e embalagem · 7.5 Do enunciado ao modelo |
| 8 | O caso Lean Seis Sigma | 8.1 Definir · 8.2 Medir · 8.3 Analisar · 8.4 Melhorar · 8.5 Controlar |

A aba **Glossário** reúne os conceitos da campanha, cada um com as missões que o usam e o capítulo do manual.

Nos atos 1 a 3 o aluno só decide com alavancas. Nos atos 4 e 5 ele monta peças do problema: distribui tarefas entre estações e posiciona máquinas numa grade, e o app gera o modelo. No ato 6 ele aplica as fórmulas de tempos e métodos e confere na simulação. No ato 7 ele junta os temas em problemas maiores: decide a escala de uma clínica por turnos, as mudanças de um processo de escritório e a equipe de um centro de distribuição, e por fim monta ou completa o modelo no UTSim e entrega o `.ron`. O ato 8 é o caso final, um projeto DMAIC completo na fábrica de eixos.

## Uma missão

A tela da missão tem, de cima para baixo:

1. **Enunciado**, com os conceitos da missão (o glossário explica cada um).
2. **Medições do estado atual**, em algumas missões. O botão roda a fábrica como está e mostra as medidas pedidas, com intervalo.
3. **Previsão**. Antes de simular, o aluno responde (qual é o gargalo, quanto vai dar a fila). A previsão vale pontos e fica travada depois da primeira rodada.
4. **Decisões**, as alavancas da missão, com o custo de cada uma e o orçamento.
5. **Justificativa**, perguntas objetivas sobre o porquê.
6. **Dicas**, em três níveis: uma pergunta que orienta, a ferramenta a olhar e o caminho da solução. Os níveis 2 e 3 descontam pontos.
7. **Simular**. O app roda as replicações com os mesmos números aleatórios para o estado inicial, para a tentativa e para a solução de referência, e mostra os resultados.

### Balanceamento (ato 4)

O quadro mostra uma coluna por estação, com a carga em segundos, a barra até o takt e as tarefas da estação. Arraste uma tarefa para outra estação, ou clique na tarefa e depois no título da estação. A tabela "Tarefas e precedências" lista o tempo e as tarefas que precisam vir antes de cada uma.

Uma atribuição que põe uma tarefa numa estação anterior à de uma tarefa que precisa vir primeiro, ou que passa do takt, é recusada com a razão escrita (por exemplo, "precedência: Embalar (estação 1) vem antes de Testar o funcionamento (estação 7), que precisa ser feita primeiro"). A eficiência é a soma dos tempos dividida pelo número de estações usadas vezes o takt. O mínimo teórico de estações é a soma dos tempos dividida pelo takt, arredondada para cima. Nem sempre ele é possível por causa das precedências.

As estações viram processadores do modelo, com o tempo da soma das tarefas. Na missão 4.2 os tempos variam (coeficiente de variação 0,3) e a simulação mostra a vazão real da linha balanceada.

### Arranjo físico (ato 5)

A grade mostra as células do galpão. Arraste cada máquina para uma célula, ou clique na máquina e depois na célula vazia. Soltar sobre outra máquina troca as duas de lugar, e a doca é fixa. O momento de transporte (viagens por dia vezes metros) é recalculado a cada mudança, e a matriz de-para mostra as viagens e a distância de cada par de máquinas.

No modelo, cada família de peças tem a sua cadeia de objetos na posição das máquinas, e as peças andam em linha reta entre elas. Por isso a matriz de-para da rodada (a mesma do fluxograma do UTSim) mostra exatamente as distâncias da grade, o que a verificação confere.

Para abrir uma missão direto, por exemplo numa demonstração em aula, use `utsim-ensino --missao ato4/ventilador`.

### Tempos e métodos (ato 6)

Algumas perguntas pedem um número calculado a partir dos dados do enunciado, pela receita de tempos e métodos que o UTSim usa no comando `utsim tempos`: o número de máquinas por operador `n' = (a + t)/(a + b)`, o tempo normal (média dos tempos observados vezes o ritmo), o tempo padrão (tempo normal vezes um mais as tolerâncias) e o número de ciclos a cronometrar para um erro relativo com 95 % de confiança. A missão 6.4 é só de cálculo e fica resolvida quando todas as respostas estão certas.

Na missão 6.1, o custo por peça é o custo por hora da operadora e das máquinas dividido pelas peças por hora da simulação.

### Modelagem (ato 7)

Nas missões de modelagem o aluno monta ou completa o modelo no UTSim. O botão "Gravar modelo inicial" grava o ponto de partida. Depois de montar o modelo, o aluno indica o arquivo `.ron`, clica em "Ler o modelo" e simula. A correção:

1. confere se o modelo carrega e é válido.
2. confere a estrutura pedida (um combinador na montagem, quebras, um caminho de volta para o retrabalho, uma estação com capacidade em paralelo, operadores).
3. roda o modelo do aluno e o modelo de referência e compara as medidas pedidas: cada uma passa quando o intervalo de 95 % do aluno encontra a faixa de tolerância em volta do valor de referência.

O enunciado diz os nomes de objeto que a correção procura (por exemplo `expedicao` para as peças aprovadas). Quando um nome falta, o critério diz qual. A lista de critérios mostra "OK" ou "Falta" e o que foi encontrado. O modelo vai dentro da tentativa assinada, e o professor corrige de novo na correção em lote. O modelo de referência da missão 7.1 é o gabarito do estudo de caso 1 do manual.

### O caso Lean Seis Sigma (ato 8)

A Usinagem Serra Alta refuga eixos por diâmetro fora da especificação e atrasa entregas. O aluno conduz o projeto como green belt, uma fase por missão: definir (CTQ, meta e escopo), medir (estudo de R&R do micrômetro e Ppk dos diâmetros), analisar (estratificação por máquina e turno e um experimento fatorial no acabamento), melhorar (troca preventiva da ferramenta e lote menor, testados no modelo da fábrica) e controlar (carta de valores individuais e regras de Nelson).

Os dados de cada aluno são gerados da semente dele, e o deslocamento da média aparece, em cada aluno, numa máquina e num turno sorteados. As respostas certas são calculadas dos dados do próprio aluno: a resposta de um colega não serve.

Há dois caminhos para analisar:

- **No jogo.** A seção "Dados da sua fábrica" mostra as primeiras linhas de cada conjunto e as ferramentas do jogo (estudo de R&R, médias por grupo, amplitude móvel, efeitos do fatorial).
- **No UTSim Seis Sigma.** "Levar para o UTSim Seis Sigma" cria um projeto DMAIC de treino (`.lss`), marcado como treino e fora dos indicadores do programa, com cada conjunto registrado com a origem "Simulado" e o modelo da fábrica em `modelos/`. O aluno analisa lá com as ferramentas completas e digita as respostas no jogo.

Os dois caminhos dão o mesmo número, porque o jogo usa as mesmas funções de qualidade que o Seis Sigma, sobre os mesmos valores com 4 casas do arquivo. "Gravar os dados (.csv)" grava os conjuntos numa pasta, para outra ferramenta.

### Pacote de qualidade e segurança do trabalho

O pacote `ensino/pacotes/qualidade_seguranca` fica fora da campanha e vem instalado com o programa. São seis casos de fábrica: onde inspecionar para o defeito não chegar ao cliente, quando a inspeção vira o gargalo, uma medição ruim, o cruzamento de pessoas com a empilhadeira, o posto de carga pesada (com a fadiga de cada pessoa) e o custo da qualidade. Os casos de segurança comparam cenários do mesmo modelo, com quase-conflitos simulados, entradas em zona sem EPI e fadiga. Eles não estimam a chance de um acidente real nem certificam conformidade com norma, e cada texto lembra isso ao aluno.

### Pacote de estatística e qualidade na fábrica

O pacote `ensino/pacotes/estatistica_qualidade` também vem instalado com o programa, fora da campanha. Ele faz a ponte entre os atos 1 a 7 e o caso Lean Seis Sigma do ato 8, com seis análises sobre dados da fábrica simulada:

1. Quantas replicações bastam para a meia largura que o gerente pede.
2. Se a furadeira nova vale a pena, com as duas opções rodadas com os mesmos números aleatórios e o teste t pareado.
3. Em que subgrupo a carta X̄-R do torno dá o primeiro sinal.
4. O Cpk da retífica antes e depois do ajuste.
5. Se o erro do paquímetro vem do instrumento ou dos operadores.
6. Qual torno desloca a média do comprimento.

Cada aluno recebe os seus próprios dados, e o gabarito é recalculado pelas mesmas análises do UTSim Estatística e do UTSim Qualidade.

Quando o UTSim Estatística ou o UTSim Qualidade está instalado pelo launcher, a missão mostra o botão "Analisar no UTSim Estatística" ou "Analisar no UTSim Qualidade". O botão grava os dados do aluno em `dados/<missão>/` do perfil, um `.csv` por conjunto, e abre o app com `--utsim-abrir=<arquivo>` para cada um. O UTSim Qualidade recebe também a ferramenta da missão (carta, capacidade, R&R ou estratificação) e os limites da especificação, e abre um estudo com uma ferramenta por arquivo. O aluno analisa lá e digita as respostas no jogo, que corrige pelos mesmos dados. Sem os apps, as ferramentas do jogo bastam: a média, o desvio, o intervalo, as amplitudes dos subgrupos e o estudo de R&R aparecem junto dos dados.

### Resultados

A tabela mostra cada indicador no estado inicial e na tentativa, com o intervalo de 95 %, a meta e a situação:

- **meta atingida**: o intervalo todo está do lado certo da meta.
- **dentro da tolerância**: o intervalo contém a meta e a média está dentro da faixa de tolerância da missão.
- **meta não atingida**.

Abaixo vêm a vista 2D, com uma replicação da fábrica animada (estações coloridas e com o estado escrito, filas como pilhas, peças andando nas setas), e as dicas automáticas, que explicam o que aconteceu e não descontam pontos.

Quando o UTSim está instalado pelo launcher no mesmo computador, aparece "Ver em 3D". O botão grava o modelo da tentativa (com a variante e as decisões do aluno) e a replicação animada na pasta do perfil (`3d/`) e abre o cliente 3D já mostrando a fábrica. Se o UTSim instalado ainda não tiver o cliente 3D, abre o visualizador com o modelo.

Ao lado aparece "Levar para o UTSim" (fora do modo prova). O botão grava só o modelo da tentativa, com a variante e as decisões do aluno, na pasta `modelos/` do perfil, e abre esse modelo no UTSim para editar. O aluno continua no simulador completo a partir do que decidiu na missão, e a missão não muda. O caminho do arquivo aparece na barra de mensagens.

O cliente 3D também tem um **modo missão**, com as mesmas telas do Ensino (campanha, missões, nota, turma e professor) dentro da janela do UTSim 3D. Ele usa o mesmo perfil do aluno e a mesma lógica, e por isso a nota de uma tentativa é a mesma nos dois programas. Para abrir, use `-- --utsim-ensino` na linha de comando do cliente 3D, com `--utsim-missao=<id>` para ir direto a uma missão e `--utsim-pacote=<pasta|pacote.utse>` para abrir outro pacote. O botão "← Voltar ao UTSim 3D" volta ao editor.

### Nota

A nota vai de 0 a 100 e tem quatro partes.

| Parte | Pontos | Como é calculada |
|---|---|---|
| Portão | | orçamento e outros limites. Quem passa do limite fica com a missão "não resolvida" e sem pontos de desempenho |
| Desempenho | 70 | para cada indicador, a fração do caminho entre o estado inicial e a solução de referência, limitada a 0 e 1, com a média ponderada pelos pesos da missão |
| Previsão | 20 | escolha certa, ou número dentro da tolerância do valor simulado no estado inicial |
| Justificativa | 10 | acertos nas perguntas objetivas |

Quando a missão não tem previsão ou justificativa, a nota é proporcional às partes que ela tem. Os descontos são os das dicas (0, 3 e 8 pontos pelo nível mais alto aberto) e o das rodadas acima do orçamento de rodadas, quando a missão tem um.

As estrelas são uma para a missão resolvida (portão e todas as metas), duas com desempenho de pelo menos 0,8 e três com pelo menos 0,95. Uma solução melhor que a referência ganha um selo.

Quando a referência piora um indicador para ganhar em outro (um custo que ela paga), esse indicador conta inteiro para quem fica pelo menos tão bem quanto a referência nele, e zero para quem fica pior.

### Caixa de areia

Depois de resolver uma missão, a caixa de areia abre a mesma fábrica sem meta e sem orçamento. Dois cenários ficam lado a lado e o app mostra a diferença pareada com intervalo, dizendo se é diferença significativa ou empate estatístico. Nada ali conta nota. O botão "Gravar .ron" grava o modelo do cenário A com as decisões aplicadas, para abrir no UTSim.

## Turma (aluno)

O professor envia um convite (`.utsc`). Na aba **Turma**, o aluno indica o convite e, se a turma usa uma pasta compartilhada no Google Drive, a pasta sincronizada no computador. Com a pasta, cada tentativa é gravada nela assim que termina. Sem a pasta, o botão "Gravar entrega" grava um arquivo `.utsr` com todas as tentativas, para enviar ao professor pelo ambiente da disciplina, por e-mail ou pen drive.

Cada tentativa é assinada com a chave do aluno, criada no primeiro uso, e encadeada à anterior. A entrega leva as decisões e as respostas, não a nota: o professor simula de novo. A assinatura e a cadeia são as mesmas do UTSim Seis Sigma. Elas revelam um arquivo alterado, uma tentativa que falta no meio ou um diário restaurado de uma cópia antiga, mas não impedem a edição e não valem como assinatura digital com validade jurídica.

### Programa Lean Seis Sigma da empresa

Quem está num programa Lean Seis Sigma no UTSim Seis Sigma pode ligar o Ensino a ele. Na aba Turma, o aluno indica a pasta do programa e clica em "Ligar". O app usa a identidade do UTSim Seis Sigma do mesmo computador e confere se a pessoa está no cadastro do programa. Depois disso, cada missão resolvida registra "praticou" no mapa de competências do aluno, nas ferramentas que o pacote liga aos conceitos da missão. Na campanha são estas: CTQ (voc_ctq), MSA, capacidade, DOE, carta de controle (cep) e, pelo lead time, WIP e takt, VSM. A evidência traz o pacote, a missão e a melhor nota. "Registrar as missões já resolvidas" envia as anteriores, e uma entrada que já está lá não se repete. As provas não registram nada. O nível "demonstrou" continua sendo dado só pelo master black belt.

O professor que é instrutor ou master black belt do programa também registra a turma inteira. Na aba Professor, depois da correção, a seção "Programa Lean Seis Sigma da empresa" recebe a pasta do programa, e "Registrar a turma no programa" grava "praticou", com origem "exercício", para cada aluno que resolveu missões. O aluno é ligado à pessoa do programa pelo e-mail da lista de alunos, sem diferença entre maiúsculas e minúsculas. Nomes e registros da turma não vão para o programa. A mensagem diz quantas competências foram enviadas e quais alunos ficaram de fora, seja por não haver pessoa com aquele e-mail, seja por aviso na entrega (cadeia quebrada ou assinatura inválida). Enviar de novo não repete as entradas.

## Modo professor

Em **Ajustes**, "Modo professor" liga a aba **Professor** e cria a chave do professor. O mesmo app serve para aluno e professor.

### Criar a turma

Informe a pasta da turma (local ou dentro do Google Drive para computador), o nome, o período, a finalidade dos dados e, se quiser, a lista de alunos em CSV:

```text
nome;registro;apelido;email
Ana Lima;2027001;ana;ana.lima@empresa.com
Beto Souza;2027002;beto;
```

O separador pode ser `;` ou `,`. O registro só aceita letras, números, `-` e `_`, porque vira o nome da pasta do aluno. O apelido aparece nos placares, nunca o nome. O e-mail é opcional. Ele só serve para ligar o aluno a uma pessoa de um programa Lean Seis Sigma, como explica a seção seguinte.

O app cria esta estrutura:

```text
turma.ron              manifesto da turma
alunos.csv             lista de alunos (só professor)
convite.utsc           convite para os alunos
pacotes/campanha.utse  a campanha publicada
entregas/<registro>/   uma pasta por aluno, com as tentativas assinadas
correcao/              notas exportadas (só professor)
```

No Drive, compartilhe a pasta da turma com os alunos. A pasta `entregas/<registro>` de cada aluno deve ficar compartilhada só com ele e o professor. O app não usa a API do Drive, só lê e grava arquivos na pasta sincronizada.

### Corrigir

"Corrigir todas as entregas" lê as pastas dos alunos e, se indicada, uma pasta com arquivos `.utsr`. Para cada aluno, o app confere as assinaturas e a cadeia das tentativas e simula de novo cada tentativa, com a variante do aluno. A nota registrada é a melhor ou a última tentativa, conforme a opção da turma.

O resultado é a **matriz aluno por missão**, com nota, estrelas, tentativas e dicas, a média de cada aluno e os avisos (tentativa alterada, cadeia quebrada, aluno fora da lista, versão diferente do pacote ou do núcleo). Abaixo vêm os **erros mais comuns** de cada missão: as decisões mais frequentes entre os que não resolveram e as previsões mais erradas.

"Exportar notas" grava `correcao/notas.csv` e `correcao/notas.xlsx`.

Uma turma de 60 alunos com as 14 missões e duas tentativas por missão (1.680 tentativas) foi corrigida em cerca de 13 segundos no notebook de desenvolvimento.

Abaixo da matriz, **pares para olhar de perto** lista os alunos com a mesma sequência de decisões numa missão cuja variante muda de aluno para aluno, ou com três ou mais tentativas nos mesmos instantes (até um minuto de diferença). Semelhança não é prova de cópia, e a decisão é do professor.

### Desafio da turma e equipes

Depois de uma correção, a seção "Desafio da turma e equipes" monta o placar de uma missão. Para cada aluno conta a tentativa da regra da turma (a melhor ou a última) feita até o prazo, se houver. A ordem é pela nota e depois pelo custo. Dois alunos aparecem empatados (com "=") quando diferem só no que a simulação mediu e os intervalos de todos os indicadores se sobrepõem, porque aí a diferença pode ser ruído. "Publicar o placar na pasta da turma" grava `placares/<missão>.csv` só com os apelidos, e os alunos o veem na aba Turma.

As equipes são escritas uma por linha, como `Azul: 2027001, 2027002`, e ficam guardadas em `turma.ron`. A solução de cada equipe é a tentativa que conta entre as dos seus membros. "Comparar as equipes nesta missão" roda todas as soluções na mesma fábrica, com os mesmos números aleatórios, e mostra a média de cada equipe no indicador de maior peso e a diferença pareada de cada par. Uma equipe só fica à frente de outra quando o intervalo da diferença não contém o zero.

### Prova

A seção "Prova" do painel monta uma prova a partir do pacote aberto (a campanha ou outro):

1. O professor dá um id e um título, a janela (início e fim, no horário de Brasília), o tempo máximo, as rodadas por missão e quantas missões sortear de cada ato. A semente fica vazia para ser sorteada na hora, ou recebe um número para repetir uma prova.
2. "Sortear e publicar na pasta da turma" grava `pacotes/<id>.utse` na pasta da turma e anota a prova em `turma.ron`, com o fim da janela como prazo. A montagem (pacote de origem, regras e semente) fica só no perfil do professor, fora da pasta compartilhada.
3. O aluno vê a prova na linha de pacotes do mapa, porque o app lê também a pasta `pacotes/` da turma. Todas as missões da prova abrem desde o início. O tempo máximo conta a partir da primeira missão aberta.
4. Na prova não há dicas, nota, estrelas nem caixa de areia. A tela mostra a janela, o tempo que resta e as rodadas usadas, e depois de cada rodada só os indicadores. Fora da janela, depois do tempo ou sem rodadas, o botão de simular fica bloqueado.
5. Para corrigir, o professor escolhe a prova em "A correção usa" e clica em "Corrigir todas as entregas". O app reconstrói a prova pela semente e confere que o pacote de origem é o mesmo da montagem. Só contam as rodadas dentro da janela e as primeiras de cada missão, e o que passar disso aparece nos avisos.

O pacote do aluno não leva a referência, as dicas nem as respostas fixas das previsões e da justificativa. As missões de entregar modelo (ato 7) ficam fora do sorteio, porque o gabarito delas é um modelo. Com a mesma semente e as mesmas regras, sai a mesma prova.

### Limites

Sem servidor, nada impede a consulta. O que o app faz é tornar a cópia pouco útil e visível. A variante muda os números de cada aluno. A nota não viaja, porque o professor simula de novo, e por isso editar o arquivo de entrega não muda a nota. Uma tentativa editada perde a assinatura e aparece nos avisos. Quem tem acesso aos arquivos pode tentar forjá-los: a assinatura revela a alteração, mas não a impede.

Na prova, o relógio é o do computador do aluno. O app bloqueia a rodada fora da janela, e o professor ignora as tentativas assinadas com data fora dela, mas quem adianta ou atrasa o relógio consegue enganar as duas conferências. Nada substitui a sala com o professor presente.

Nome, registro e notas são dados pessoais. A finalidade fica registrada na turma, o placar usa o apelido e a pasta de cada aluno fica compartilhada só com ele e o professor.

## Construtor de casos

O professor cria missões novas a partir de qualquer modelo do UTSim, sem escrever o arquivo da missão à mão. Na aba **Professor**, a seção "Construtor de casos" pede o modelo `.ron`, o título e a pasta do pacote. "Criar e calibrar" faz o seguinte:

1. sugere as alavancas do modelo: a capacidade de cada processador (até dois servidores a mais), o tamanho de cada fila limitada (da metade ao dobro) e o tamanho de cada equipe.
2. sugere as medidas de cada saída (lead time e itens que chegam).
3. escreve o rascunho da missão e um texto inicial.
4. calcula a referência pela busca exaustiva e põe metas a 5 % do valor dela.
5. confere a missão e grava a pasta do pacote, com o resultado em `conferencia.txt`.

Depois o professor reescreve o texto, ajusta faixas, custos, orçamento e metas nos arquivos da pasta e clica em "Conferir de novo". "Gravar .utse" empacota a pasta para publicar na turma. Os mesmos passos existem na linha de comando:

```text
utsim-ensino caso examples/linha_com_bloqueio.ron pacote_pulmao "Pulmão do torno"
utsim-ensino conferir pacote_pulmao
utsim-ensino pacote pacote_pulmao pulmao.utse
```

O conferidor avisa quando a referência e o estado inicial empatam (a missão não ensina nada), quando nenhuma combinação bate as metas e quando mais de 30 % batem (missão fácil). A busca exaustiva aceita até 400 combinações de alavancas.

## Editor de missões

O editor de missões faz o mesmo que o construtor, mas tudo na tela, sem abrir os arquivos da pasta. Ele fica na aba **Editor de missões**, que aparece no modo professor.

1. **Seu pacote.** Indique a pasta do pacote e clique em "Abrir ou criar". Uma pasta com `pacote.ron` abre o pacote que já está lá. Uma pasta vazia começa um pacote novo, gravado junto com a primeira missão. Uma pasta com outros arquivos e sem `pacote.ron` é recusada, para nada ser sobrescrito. A lista mostra as missões por ato, com "Editar", as setas para mudar a ordem (descer a última missão de um ato a leva para o ato seguinte) e "Remover", que apaga a missão e os arquivos que só ela usa.
2. **Missão nova.** Escolha o modelo `.ron` e o título e clique em "Criar o rascunho". O editor sugere as mesmas alavancas e métricas do construtor e escreve um enunciado inicial. O identificador nasce do título (`prof/<título>`) e pode ser trocado.
3. **Alavancas.** Cada linha tem uma caixa para usar ou não a alavanca, o rótulo que o aluno vê, a faixa (de, até, e o passo nas alavancas de valor) e o custo por unidade acima do valor inicial. O orçamento é opcional. Embaixo aparece quantas combinações a calibragem vai simular. O limite é o da busca exaustiva, 400.
4. **Métricas.** Para cada métrica há o rótulo, se ela é melhor quando menor ou maior, a meta, a tolerância (em % ou na unidade da métrica) e o peso na nota. Peso zero mostra a métrica ao aluno sem contar na nota.
5. **Enunciado e perguntas.** O texto em Markdown que o aluno lê, com a prévia desenhada do mesmo jeito que na tela do aluno. Embaixo ficam:
   - **Conceitos do glossário**: marque os termos que a missão ensina ou usa. O pacote leva um `glossario.ron` com os termos marcados, copiados do glossário da campanha, e mantém os termos próprios que já tinha.
   - **Medições antes da previsão**: valores do estado inicial que o aluno pode ver antes de responder, cada um com a métrica do modelo, a unidade e as casas.
   - **Perguntas de previsão**, respondidas antes de rodar. A de escolha tem a resposta fixa ou a opção de maior ou menor valor no estado inicial. A de valor compara a resposta com a métrica do modelo, dentro da tolerância. A de cálculo usa uma das receitas de medida do trabalho (máquinas por operador, tempo normal, tempo padrão e número de ciclos).
   - **Perguntas de justificativa**, objetivas, sobre o porquê, com a opção certa marcada.
   - **Dicas**, até três níveis: a pergunta que guia, a ferramenta e o caminho.

   A métrica de uma medição ou de uma pergunta se escolhe numa lista com as métricas que o modelo produz, o custo das decisões e o número de cada alavanca em uso.

   **Dados da missão.** Uma missão pode trazer conjuntos de dados com um efeito plantado, gerados para cada aluno a partir da semente dele. O professor escolhe o tipo (tempos de processo de uma distribuição, estudo de R&R, grupos com uma média deslocada, série com deslocamento ou tendência, fatorial de dois níveis, comparação pareada, subgrupos para a carta X̄-R ou registros da simulação) e ajusta os números no formulário. Os registros da simulação vêm do próprio modelo da missão: o professor marca as máquinas que entram na coleta, escolhe a máquina com a causa plantada e o tempo de processo que ela passa a ter, e diz quanto tempo a coleta dura. Cada aluno recebe o tempo de cada peça em cada máquina, rodado com a semente dele, e pode ter a causa numa máquina diferente da do colega quando "Sortear a máquina por aluno" está marcado. As perguntas que servem a esse conjunto pedem a máquina mais lenta ou a menos estável. Cada conjunto tem um nome, a ferramenta do UTSim Qualidade que o abre e, se for o caso, os limites de especificação. A amostra mostra as primeiras linhas e o efeito plantado em palavras. As perguntas sobre dados aparecem junto das perguntas de previsão, e o editor só oferece as que servem para o tipo do conjunto. "Conferir com os dados" calcula a resposta com os dados de exemplo e testa 30 alunos fictícios: se algum recebe dados sem resposta (um efeito pequeno demais para o ruído, por exemplo), o editor avisa. Nas perguntas que pedem um grupo ou uma máquina, ele avisa também quando os dados de algum aluno apontam outro grupo que não o do efeito plantado. "Analisar em" diz para qual app, UTSim Estatística ou UTSim Qualidade, o aluno leva os dados. Uma missão só de perguntas não tem alavancas nem calibragem.
6. **Calibrar.** Simula todas as combinações em segundo plano. Mostra quantas cabem no orçamento e nas restrições, quantas batem todas as metas e uma tabela com a situação inicial, a referência e a meta sugerida de cada métrica (5 % de folga em torno da referência). Os avisos do conferidor aparecem embaixo. "Usar a referência e as metas sugeridas" leva os valores para a missão. As metas são arredondadas para o lado da folga, então a referência sempre as bate. Se nenhuma combinação cabe no orçamento, a calibragem avisa e não sugere nada.
7. **Gravar no pacote.** Confere os campos e grava a missão na pasta. Uma missão aberta da lista volta para o lugar dela, mesmo com o identificador trocado. Uma missão nova com o identificador de outra é recusada. Para mudar o ato de uma missão gravada, use as setas da lista. "Conferir o pacote" faz a mesma conferência que o app do aluno faz ao abrir o pacote, e "Gravar o .utse" empacota a pasta para publicar na turma.

Ao abrir de novo uma missão gravada, as alavancas e as métricas que ela não usa voltam como sugestões desmarcadas. Os campos que o editor ainda não mostra (as rodadas, por exemplo) são mantidos como estavam. **Missões de peças.** Em "O aluno decide", escolha "Tarefas nas estações" para um balanceamento de linha ou "Máquinas na grade" para um arranjo. As alavancas saem da missão, e a medida da peça entra marcada com peso: a eficiência do balanceamento ou o momento de transporte (viagens vezes distância).

No balanceamento, o modelo precisa de processadores chamados `est1`, `est2` e assim por diante, um por estação. O tempo de cada estação vem das tarefas que o aluno põe nela. O takt começa no intervalo entre chegadas do modelo. Escreva as tarefas com nome e tempo, marque em "Depois de" as que precisam vir antes e dê a estação de cada uma no estado inicial. A variação dos tempos (CV) deixa a simulação com tempos aleatórios em volta da carga de cada estação. Embaixo, a tela mostra a soma dos tempos, o mínimo teórico de estações e a eficiência do estado inicial.

No arranjo, cada máquina corresponde aos objetos do modelo com o nome dela antes de `__` (`serra__eixos_1` é a serra), e o arranjo do aluno muda a posição desses objetos. As máquinas das chegadas e das saídas (a doca) começam fixas. Cada chegada do modelo vira um fluxo, com o roteiro das conexões e um volume proporcional às chegadas. Ajuste o tamanho da grade, a célula inicial de cada máquina, os volumes e os roteiros. A grade inicial aparece desenhada, com o seu momento de transporte.

"Conferir as peças" procura o ótimo pela busca exata e avisa quando ele não existe (uma tarefa maior que o takt, uma grade grande demais) ou quando o estado inicial já é o ótimo. A calibração acha a referência pela mesma busca e sugere a meta da medida.

**Missões de modelagem.** Marque "Missão de modelagem" para que o aluno construa o modelo em vez de mexer em alavancas. O modelo aberto no editor vira o gabarito, e as alavancas e as métricas saem da missão. Em "Modelo inicial do aluno", carregue o ponto de partida dele, um modelo mais simples ou incompleto. Enquanto o modelo inicial for o próprio gabarito, o editor avisa. Os critérios de correção vêm sugeridos pelo gabarito: a quantidade de objetos de cada tipo, as quebras, o retrabalho, os postos em paralelo, os operadores e as medidas (produção, lead time e utilização), cada medida com a sua tolerância. Ajuste os rótulos, que o aluno lê no relatório, e os pesos. Remova o que não interessa e acrescente outros critérios pelo tipo. "Sugerir de novo pelo gabarito" refaz a lista. "Conferir a modelagem" corrige o gabarito e o modelo inicial: o gabarito precisa passar em todos os critérios, e o modelo inicial não pode passar em todos, senão a missão não ensina nada. Use nas medidas os nomes de objetos que o enunciado pede, para que o modelo do aluno possa ser comparado.

## Dados com efeito plantado

O módulo `dados` do núcleo gera conjuntos de dados didáticos com semente, com a resposta conhecida guardada junto: tempos de processo de uma distribuição, um estudo de R&R com os desvios de peça, operador e repetibilidade, grupos com a média de um deles deslocada, uma série com deslocamento ou tendência e um fatorial de dois níveis com efeitos e interação. A mesma receita dá os mesmos dados em qualquer computador. A verificação confere que as análises que o aluno usa recuperam o efeito plantado: o ajuste de distribuições, o estudo de R&R por ANOVA, a ANOVA de um fator, as regras de Nelson e os efeitos do fatorial.

## Pacotes de missões

As missões vêm em pacotes (`pacote.ron` com as missões, os modelos, os textos e o glossário, numa pasta ou num `.utse`). Cada missão (`missao.ron`) aponta um modelo do UTSim e lista as alavancas (ou um `balanceamento`, com as tarefas, o takt e as estações, ou um `arranjo`, com a grade, as máquinas e os fluxos), cada uma traduzida em chaves `<objeto>.<campo>` do modelo, as métricas com meta e tolerância, as previsões, as perguntas e as dicas. O app confere tudo ao carregar: as chaves existem no modelo, as faixas são possíveis, o orçamento permite uma solução e as métricas saem da simulação. Um pacote com erro é recusado com a razão de cada problema.

O pacote diz do que precisa em `requer`. `nucleo >= 0.1` pede uma versão mínima do núcleo de simulação (também valem `>`, `=`, `<=` e `<`). `linha:lean_office` pede uma linha especializada ou um pacote transversal do UTSim (`lean_office`, `servicos`, `logistica`, `qualidade`, `seguranca`). Um requisito que esta versão não atende não impede nada. O pacote abre, o mapa mostra um aviso com o que falta e a razão, a lista de pacotes marca o pacote com ⚠, e `utsim-ensino conferir` escreve o aviso no relatório. O caminho é atualizar o UTSim Ensino pelo launcher.

Os pacotes que vêm com o programa (a pasta `pacotes`, instalada pelo launcher ao lado de `bin`) aparecem sozinhos numa linha no topo do mapa, junto com a campanha, e também em "Outros pacotes de missões", no fim do mapa. Um clique troca de pacote. Para jogar um pacote que o professor passou, o aluno digita a pasta ou o `.utse` em "Outros pacotes de missões" e clica em "Abrir pacote". O mapa passa a mostrar as missões desse pacote, e o app volta a abri-lo nas próximas vezes até o aluno clicar em "Voltar à campanha". O progresso fica guardado por pacote, então a campanha continua de onde parou. As tentativas entram no mesmo diário assinado, e o professor corrige as entregas com o mesmo pacote aberto no modo professor. Na linha de comando, `utsim-ensino --pacote <pasta|pacote.utse>` abre o pacote direto.

Nas missões de peças, o conferidor compara a referência guardada com o ótimo de uma busca exata (o menor número de estações dentro do takt, e o menor momento de transporte). A campanha embutida passa por uma conferência automática a cada versão (`cargo test -p utsim-ensino-core --test conteudo`): a busca exaustiva confirma que a referência guardada é a melhor combinação, que ela difere do estado inicial com significância, que alguma combinação bate as metas e que o estado inicial não bate. Os números da referência ficam guardados e um teste acusa qualquer mudança do núcleo que os altere.

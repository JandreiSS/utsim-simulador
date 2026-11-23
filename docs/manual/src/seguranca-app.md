# UTSim Segurança do Trabalho

> **Aviso legal.** O UTSim apoia o estudo, o ensino e a organização das informações. Ele não substitui o profissional legalmente habilitado, não emite laudo e não certifica conformidade com nenhuma Norma Regulamentadora. O cumprimento da legislação é responsabilidade do usuário e da empresa. Esse aviso fica fixo no rodapé de todas as telas do aplicativo e abre todo relatório.

O UTSim Segurança do Trabalho é um aplicativo da família UTSim com janela própria. Ele organiza o inventário de perigos e riscos por posto de trabalho, o plano de ação, a matriz de risco e a exposição que a simulação de um modelo produz. Funciona sem internet e sem servidor, com a equipe numa pasta compartilhada pelo Drive. Esta é a primeira versão, ainda sem acabamento visual.

Pela linha de comando, `utsim-seguranca` abre a janela.

As normas aparecem aqui só pelo número e pelo tema, para situar o assunto. O programa não reproduz texto de norma. Os temas mais ligados a este aplicativo são o gerenciamento de riscos ocupacionais (NR-1), os equipamentos de proteção individual (NR-6), a movimentação de materiais (NR-11), a segurança em máquinas (NR-12) e a ergonomia (NR-17).

## Pasta de unidade e postos

A pasta de unidade e a tela Postos são as mesmas do [UTSim Qualidade](qualidade-app.md), e os dois aplicativos podem abrir a mesma pasta. Veja lá como criar a unidade, cadastrar os postos e importá-los de um modelo.

## Inventário de riscos por posto

A tela Inventário de riscos segue cinco etapas, inspiradas na lógica do gerenciamento de riscos ocupacionais. Escolha o posto e percorra as etapas:

1. **Identificar o posto.** Mostra a área, as atividades, as pessoas expostas e os EPI exigidos.
2. **Levantar perigos.** Cada perigo tem grupo (físico, químico, biológico, ergonômico, acidente e mecânico), descrição, fonte, circunstância e possíveis lesões.
3. **Avaliar riscos.** Informe os controles existentes, a probabilidade e a severidade. O nível sai da matriz e aparece com a cor dele. Passe o mouse sobre o nível para ver a decisão que a empresa definiu para ele.
4. **Planejar ações.** Cada ação segue a hierarquia de controle, na ordem eliminar, substituir, controle de engenharia, controle administrativo e EPI. Informe o responsável, o prazo, a situação e se a ação foi eficaz.
5. **Acompanhar.** Registre reavaliações com data, probabilidade, severidade e nota. O nível atual é o da última reavaliação. Depois de registrar, o formulário fica com os valores gravados e o botão desabilita, então um clique a mais não repete a reavaliação. Mude um campo para registrar outra. Cada reavaliação do histórico tem "Editar", que a traz ao formulário para corrigir, e "Remover", que pede confirmação.

No alto da tela ficam os alertas em vermelho: os perigos acima do nível mais baixo que ainda não têm ação e as ações com prazo vencido.

As edições ficam num rascunho até você usar "Gravar", para que cada tecla digitada não vire uma alteração na pasta compartilhada.

Quando o posto está ligado a um nó do modelo, os perigos gravados no próprio nó (aba Segurança do nó, no UTSim ou na tela Modelo) também aparecem no inventário, marcados "no nó do modelo". Eles entram nos alertas e no relatório, e as ações e as reavaliações deles ficam na unidade, como as dos outros perigos. Para mudar a descrição, o grupo, os controles, a probabilidade ou a severidade, edite no nó. O inventário relê o modelo quando o arquivo muda.

Na etapa 2, o botão "Editar no nó" de um perigo do nó abre a tela Modelo já nesse nó. Um perigo da unidade, num posto ligado a um nó, tem o botão "Levar para o nó": ele grava o perigo na aba Segurança do nó e o tira da unidade, e as ações e as reavaliações dele passam a valer para o perigo no nó. Para isso o app toma a trava do modelo por um instante. Se outra pessoa ou outro app edita o modelo, nada muda e a barra de estado diz quem edita. Se a tela Modelo tem mudança por gravar no mesmo arquivo, grave ou desfaça antes.

## Matriz

A matriz padrão é uma grade própria de 5 por 5, com níveis baixo, médio, alto e crítico. Na tela Matriz, clique numa célula para trocar o nível dela e edite os nomes, as cores e as decisões dos níveis. É possível importar a matriz da sua empresa por CSV (de 3 por 3 até 5 por 5) e exportá-la. Uma matriz em que mais probabilidade ou mais severidade baixa o nível é recusada, e o programa mostra o motivo. "Gravar na unidade" faz a matriz valer para toda a unidade.

## Exposição

Escolha um modelo e use "Rodar". A simulação roda em segundo plano e mostra, com média e intervalo de confiança, o tempo das pessoas em zonas de risco, as entradas sem o EPI pedido e os quase-conflitos entre pessoas e veículos. Tudo é simulado, serve para comparar layouts e não prevê acidentes.

Num modelo com ponte rolante e pessoas, a tela mostra também o tempo e as entradas de cada pessoa sob a carga suspensa de cada ponte (a área quadrada embaixo do gancho enquanto há peça pendurada, campo `load_zone` da ponte). A meta é zero: qualquer tempo sob a carga aparece em vermelho, entra no relatório HTML e vira um perigo sugerido para o posto da ponte ou da pessoa.

"Sugerir perigos" propõe um perigo para cada zona em que alguém de um posto esteve e para cada pessoa que ficou sob a carga suspensa de uma ponte. Marque os que fizerem sentido e grave. Eles entram no inventário sem avaliação, e a probabilidade e a severidade ficam por sua conta. "Gravar HTML de exposição" escreve o relatório na pasta `relatorios/`.

## Ergonomia

No alto da tela aparece o aviso da NR-17: a nota é uma pontuação própria para estudo, ensino e comparação, e não substitui a análise ergonômica do profissional habilitado. Escolha um modelo (pelo caminho ou pela lista dos modelos dos postos), a unidade de tempo (s, min ou h) e use "Rodar estudo". O estudo de tempos roda em segundo plano.

Para cada elemento de trabalho de cada operador, marque a postura do tronco, dos braços, do pescoço e das pernas, a carga em kg, a pega e a vibração. A nota de 0 a 10 e a faixa aparecem ao vivo, calculadas com o método da unidade. Elementos sem marcação contam como neutros. A repetição e a duração vêm dos ciclos medidos no estudo.

- **Gravar marcações** guarda as marcações na pasta de unidade, e só nesse momento. Ao rodar o mesmo modelo de novo, elas voltam.
- **Importar método (CSV)** troca os pesos e as faixas pelos da sua empresa e grava o método na unidade. Se o CSV for recusado, o motivo aparece na tela.
- **Rodar e comparar** estuda um segundo modelo e mostra a nota e o tempo padrão antes e depois, por posto.
- **Gravar HTML de ergonomia** escreve `relatorios/ergonomia.html`.

## Relatório

Escolha todos os postos ou só alguns e use "Gravar HTML do inventário". O arquivo vai para a pasta `relatorios/` da unidade e pode ser impresso pelo navegador. O relatório não é o documento legal do programa de gerenciamento de riscos. O PDF virá em uma etapa futura.

## Dados pessoais e pasta restrita

A unidade guarda as pessoas só por código (`P-1a2b3c`). Nome, setor e anotações de saúde ficam numa pasta restrita, separada da unidade, por causa da LGPD. O melhor é criar essa pasta numa pasta do Drive compartilhada só com quem pode ver os dados. O caminho dela fica gravado em `unidade.json`, e quem abre a unidade sem acesso à pasta restrita simplesmente não vê os nomes. Além do compartilhamento do Drive, o arquivo `acesso.json` da pasta restrita lista quem pode gravar nela.

Anonimizar uma pessoa reescreve o registro só com o código e apaga da pasta restrita os arquivos de operação antigos que tinham o nome e os dados de saúde. Os relatórios da unidade usam sempre o código, e um teste confere que nenhum nome da pasta restrita aparece nos arquivos da unidade.

## Checklist de máquinas e cadastro de EPI

O checklist próprio do UTSim tem itens escritos pelo projeto em sete temas: proteções fixas e móveis, intertravamento, parada de emergência, comandos, sinalização, manutenção e bloqueio, e capacitação. A NR-12 (segurança no trabalho em máquinas e equipamentos) aparece só por número e tema. O checklist não reproduz a norma e não substitui a apreciação de riscos do profissional habilitado. A empresa pode usar o roteiro do seu responsável técnico, importado por CSV (`tema;item;severidade`).

Cada item recebe conforme, não conforme ou não se aplica, com observação, foto e ação. Um item não conforme vira perigo no inventário do posto, com a origem do checklist e a severidade sugerida pelo item, uma vez só, mesmo que a inspeção seja gravada de novo.

O cadastro de EPI guarda o tipo, o uso, o número do certificado de aprovação digitado pelo usuário, a validade e o intervalo de troca (NR-6, por número e tema). O app aponta o EPI exigido num posto que não está no cadastro e o certificado vencido.

## Incidentes e indicadores

O registro de quase-acidentes e incidentes guarda data, posto, zona, tipo, descrição, causas, ações, o perigo do inventário e as pessoas, estas só pelo código da pasta restrita. Um registro com nome no lugar do código é recusado.

Os indicadores usam as horas-homem de exposição de cada mês e uma base configurável (um milhão de horas por padrão):

- taxa de frequência (incidentes com lesão, com e sem afastamento) e taxa de frequência com afastamento;
- taxa de gravidade (dias perdidos e dias debitados);
- dias sem acidente;
- razão entre quase-acidentes e incidentes com lesão;
- Pareto de causas e de postos;
- contagem por zona do layout;
- carta de controle `u` das lesões por mês.

A tabela de dias debitados de norma técnica não é embutida: a empresa digita, em cada caso, os dias que debita pela tabela que usa.

## Telas de checklist, EPI e incidentes

A tela Checklist mostra o aviso sobre a NR-12 e o modelo em uso, que pode ser o checklist próprio do UTSim ou um roteiro da empresa lido de um CSV e guardado na unidade. Escolha o posto, informe a máquina e use "Nova inspeção". Cada item recebe conforme, não conforme ou não se aplica, com observação e ação. A inspeção só vai para a unidade em "Gravar inspeção". Depois de gravada, "Levar não conformidades ao inventário" cria os perigos do posto, uma vez só por item. Uma inspeção gravada por engano sai da lista com "Remover".

Em todas as telas do app, remover pede confirmação: o primeiro clique troca o botão por "Confirmar" (em vermelho) e "Cancelar", e só o segundo apaga. Vale para inspeções, EPI, incidentes, perigos e ações do inventário.

A tela EPI cadastra o tipo, o uso, o número do certificado digitado, a validade e o intervalo de troca. Abaixo do cadastro ficam as pendências: o equipamento exigido num posto que não está no cadastro e o certificado vencido.

A tela Incidentes tem quatro abas:

- **Registro** lista e grava quase-acidentes e incidentes. As pessoas envolvidas são escolhidas pelo código, entre as da pasta restrita. O nome só aparece ao lado do código quando a pasta restrita abre para você. Um registro com problema, como um nome no lugar do código, é recusado com a mensagem do motivo. "Remover", ao lado de "Editar", tira um registro feito por engano. As pessoas da pasta restrita continuam lá.
- **Horas-homem** guarda as horas de exposição de cada mês.
- **Indicadores** calcula as taxas do período escolhido, o Pareto de causas e de postos, os eventos por zona e a carta `u` das lesões por mês. Esta aba nunca mostra nomes.
- **Pessoas (restrita)** cria a pasta restrita, cadastra pessoas e anonimiza. Cadastrar e anonimizar só aparecem para quem pode gravar na pasta restrita, e a anonimização pede uma segunda confirmação porque apaga o nome para sempre.

## Ferramentas e estudos

A aba **Ferramentas** é a mesma do UTSim Qualidade (veja o capítulo do UTSim Qualidade), com as ferramentas de análise de causa e de dados: folha de verificação, estratificação, Pareto, histograma, dispersão, Ishikawa, 5 porquês, FMEA, A3 e poka-yoke (os dispositivos à prova de erro que também protegem as pessoas, com as verificações). Os dados são digitados ou importados de CSV e xlsx, e cada ferramenta tem relatório próprio em PDF e HTML, além do consolidado do estudo.

## Abrir pelo UTSim

No UTSim 3D, "Abrir no UTSim Segurança" nas abas do nó abre a tela Modelo com o nó escolhido, mesmo sem unidade aberta. O UTSim grava o que estiver pendente e entrega a edição. Se outra pessoa ou outro programa estiver editando, o modelo abre só para leitura, com o aviso de quem edita. Na linha de comando: `utsim-seguranca --utsim-abrir=<modelo.ron> --utsim-no=<nó>`. Arquivos de dados (CSV ou xlsx) também abrem, com `--utsim-ferramenta=<chave>`, como no UTSim Qualidade. Pedidos de outros apps (`--utsim-pedido=<arquivo>`) também são atendidos e respondidos ao fechar, como lá.

## Modelo (mão dupla com o UTSim)

A aba "Modelo" é a tela de entrada quando a unidade tem modelo ligado a algum posto. Ela abre o mesmo arquivo `.ron` que o UTSim usa: escolha um dos modelos dos postos ou digite o caminho. A lista à esquerda mostra os objetos do modelo, com filtro por nome, o que cada um tem (qualidade, inspeção, retrabalho, EPI, perigos, quebras) e um número-chave do resultado simulado. Ao escolher um objeto, os resultados da simulação daquele nó aparecem primeiro, marcados como "Simulado", e "Simular de novo" refaz a simulação. Abaixo ficam as abas de campos: a aba Segurança mostra os campos do próprio nó (EPI exigido, regra de operação, perigos do posto e marcações de ergonomia) e a aba Manutenção mostra os dados de quebras só para leitura. Num nó com quebras, a aba Manutenção começa pela manutenção simulada, marcada como "Simulado": disponibilidade, OEE, falhas e preventivas por replicação, MTBF e MTTR simulados, tempo parado em corretiva e em preventiva, espera média pelo técnico e custo, na média de até 5 replicações e com os tempos na unidade do modelo. Cada campo editado vale na hora, com erro de faixa ao lado do campo, e há desfazer e refazer (Ctrl+Z e Ctrl+Y).

Nada vai para o arquivo antes de "Gravar no modelo" (ou Ctrl+S). A gravação troca o arquivo inteiro de uma vez, e o UTSim enxerga a mudança. O que se edita no UTSim aparece aqui: quando o arquivo muda no disco e você não tem mudança sem gravar, a aba recarrega sozinha. Se tiver, ela avisa e deixa você escolher entre recarregar e manter.

Só uma pessoa edita o modelo por vez. Ao abrir, a aba grava uma trava ao lado do arquivo e a renova a cada minuto. Se o UTSim, o UTSim Qualidade ou outra pessoa já edita, a aba abre só para leitura e avisa "Em edição por ... no ... desde ...". O botão "Tentar editar" tenta de novo, e a trava de um programa que travou vence sozinha em cinco minutos.

A regra de operação de um processo diz quem pode operar o posto. Ela traz as habilidades exigidas de todo operador, a escolha de exigir o EPI do posto e o das zonas de risco que contêm o posto e os tempos para a equipe entrar e sair. Com o EPI exigido, a aba mostra a lista somada que o operador precisa vestir. O EPI que cada pessoa veste é o EPI do nó dela. Deixe um tempo vazio para não ter tempo de entrar ou de sair. A mesma regra aparece na aba Segurança do nó no UTSim 3D.

À direita ficam os resultados da simulação do mesmo modelo, por nó: para cada pessoa que passa pelas zonas do nó, o tempo em zona, as entradas sem EPI e os quase-conflitos. Eles são sempre marcados como "Simulado".

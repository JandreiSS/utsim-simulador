# UTSim Qualidade

O UTSim Qualidade é um aplicativo da família UTSim com janela própria. Ele organiza os postos de trabalho de uma unidade (fábrica ou laboratório) e mostra os indicadores de qualidade que a simulação de um modelo produz. Funciona sem internet e sem servidor, e a equipe trabalha numa pasta compartilhada pelo Drive. Esta é a primeira versão, ainda sem acabamento visual.

Pela linha de comando, `utsim-qualidade` abre a janela.

## A pasta de unidade

A unidade é uma pasta que a equipe compartilha. Cada alteração vira um pequeno arquivo novo dentro dela, que ninguém edita depois. Assim duas pessoas podem trabalhar ao mesmo tempo sem perder alterações. A mesma pasta serve ao UTSim Segurança do Trabalho: os postos que você cria aqui aparecem lá.

Na tela inicial, escreva o seu nome (o autor de cada alteração), o caminho da pasta e use "Abrir unidade". Para começar uma unidade nova, preencha também o nome dela e use "Criar unidade". O programa lembra o seu nome e a última pasta num arquivo de configuração no seu computador.

## Postos

A tela Postos lista os postos da unidade. Cada posto tem nome, área, descrição, atividades, pessoas expostas por turno, equipamentos de proteção, o modelo `.ron` e o objeto do modelo que ele representa.

- **Novo posto** e **Gravar** criam ou alteram um posto. **Remover** apaga o posto da unidade.
- **Importar postos do modelo** lê um arquivo `.ron` e sugere um posto para cada processador, combinador ou separador que ainda não tem posto. Marque os que quiser e use "Criar os postos marcados".
- **Atualizar** lê as alterações que outras pessoas gravaram. O programa também atualiza sozinho a cada cinco segundos.

## Simulação de qualidade

Escolha o modelo (pelo caminho ou pela lista dos modelos já ligados aos postos), o número de replicações e use "Rodar". A simulação roda em segundo plano e a janela continua respondendo. O rótulo "Origem: Simulado" fica sempre visível, porque tudo o que aparece aqui vem do modelo e não de medições da fábrica.

O resultado traz:

- **Resumo** com a média e o intervalo de confiança de 95 % de cada indicador: concluídos, entregues, refugados, rendimento de primeira passagem, rendimento acumulado, percentual de refugo e de retrabalho, escape de defeitos ao cliente, defeitos por unidade, DPMO e eficácia da inspeção.
- **Pareto** dos defeitos por tipo e por posto.
- **Capacidade** das características medidas no modelo (Cp, Cpk, Pp e Ppk).

Os botões "Gravar relatório HTML" e "Gravar CSV das medições" escrevem os arquivos na pasta `relatorios/` da unidade.

## Custo da qualidade

Escolha o modelo e o número de replicações e use "Rodar". O cálculo roda em segundo plano. Os custos do modelo aparecem com o rótulo "Origem: Simulado". A tabela mostra, para as quatro categorias (prevenção, avaliação, falha interna e falha externa), a média, o intervalo de confiança de 95 %, os lançamentos digitados e o total. Abaixo ficam os custos por posto e por defeito.

Os **lançamentos digitados** (treinamento, calibração, garantia, devoluções) têm categoria, descrição, valor e posto, e ficam gravados na unidade. Eles entram no total na próxima vez que você rodar.

"Rodar e comparar" calcula um segundo modelo com os mesmos números aleatórios e mostra a diferença entre os totais e se ela é significativa. "Gravar HTML do custo da qualidade" escreve `relatorios/custo_qualidade.html`.

## Planos

A tela Planos guarda, por posto, o plano de controle (característica, limites, método de medição, frequência, carta e a reação ao sinal) e o plano de inspeção (o que, como, quando e quem). Cada linha pode ser incluída, editada e removida. O botão "Sugerir do modelo" lê um modelo `.ron` e propõe uma linha de controle para cada característica com limites e uma linha de inspeção para cada estação de inspeção, ligadas ao posto pelo objeto. As sugestões só vão para a unidade quando você clica em "Gravar as sugestões".

Ao editar uma linha de inspeção aparece o painel de amostragem por atributos. Você pode projetar o plano pelos quatro números (p1, alfa, p2 e beta, simples ou duplo) ou digitar n e c. A tela mostra a probabilidade de aceitação em p1 e em p2, o AOQL para o lote informado e a curva característica de operação. "Usar este plano na linha" leva o plano para a linha, e "Gravar HTML da amostragem" escreve `relatorios/plano_amostragem.html`. Planos de uma tabela que a empresa tenha o direito de usar entram por CSV (`nome;n;c` ou `nome;n1;c1;r1;n2;c2`).

## 8D

A tela 8D lista os 8D da unidade e abre um para edição. Há uma seção para cada uma das oito disciplinas, com o texto que explica o que ela pede, e a disciplina seguinte aparece em destaque. A contenção atrasada em relação a hoje é avisada em vermelho. Na causa raiz, cada causa tem a categoria dos 6M e a marca de confirmada por dado, e sob cada causa ficam os porquês. As ações têm verificação por dado (carta ou estudo no modelo) e a prevenção registra a atualização da FMEA e do plano de controle. O 8D só encerra quando D1 a D7 estão completas. Nada vai para a unidade antes de "Gravar".

Quando a causa não aparece, o botão "Enviar para a carteira do Seis Sigma" pede a pasta da carteira, o nome e o e-mail, e cria uma ideia na carteira com o 8D anexado. O número da ideia fica guardado no 8D depois de você gravar. O 8D precisa estar gravado antes do envio.

## CEP

A tela CEP importa o CSV de uma característica (valores individuais ou subgrupos), calcula a carta de fase I com as regras de Western Electric e mostra os sinais. "Congelar limites" guarda os limites na unidade. A fase II aplica esses limites a dados novos, e cada subgrupo com sinal abre um cartão de reação com a reação escrita na linha do plano de controle escolhida. A origem do dado (Real ou Simulado) fica sempre visível.

Para comparar com o simulado, informe o modelo e a característica. O modelo roda em segundo plano e a tela mostra as médias, o teste da diferença, a razão dos desvios e a curva EWMA dos resíduos, que mostra a deriva do processo real em relação ao modelo.

## Limites desta versão

O PDF do relatório e as ferramentas de projeto como a FMEA e o A3 ainda não estão neste aplicativo. Os projetos de melhoria continuam no UTSim Seis Sigma.

## Modelo (mão dupla com o UTSim)

A aba "Modelo" é a tela de entrada quando a unidade tem modelo ligado a algum posto. Ela abre o mesmo arquivo `.ron` que o UTSim usa: escolha um dos modelos dos postos ou digite o caminho. A lista à esquerda mostra os objetos do modelo, com filtro por nome, o que cada um tem (qualidade, inspeção, retrabalho, EPI, perigos, quebras) e um número-chave do resultado simulado. Ao escolher um objeto, os resultados da simulação daquele nó aparecem primeiro, marcados como "Simulado", e "Simular de novo" refaz a simulação. Abaixo ficam as abas de campos: a aba Qualidade mostra os campos do próprio nó (defeitos, características medidas, inspeção, retrabalho e custos) e a aba Manutenção mostra os dados de quebras só para leitura. Cada campo editado vale na hora, com erro de faixa ao lado do campo, e há desfazer e refazer (Ctrl+Z e Ctrl+Y).

Nada vai para o arquivo antes de "Gravar no modelo" (ou Ctrl+S). A gravação troca o arquivo inteiro de uma vez, e o UTSim enxerga a mudança. O que se edita no UTSim aparece aqui: quando o arquivo muda no disco e você não tem mudança sem gravar, a aba recarrega sozinha. Se tiver, ela avisa e deixa você escolher entre recarregar e manter.

Só uma pessoa edita o modelo por vez. Ao abrir, a aba grava uma trava ao lado do arquivo e a renova a cada minuto. Se o UTSim, o UTSim Segurança do Trabalho ou outra pessoa já edita, a aba abre só para leitura e avisa "Em edição por ... no ... desde ...". O botão "Tentar editar" tenta de novo, e a trava de um programa que travou vence sozinha em cinco minutos.

À direita ficam os resultados da simulação do mesmo modelo, por nó: defeitos gerados, rendimento da inspeção e, para o modelo inteiro, refugo, retrabalho e escape. Eles são sempre marcados como "Simulado".

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

## Limites desta versão

O PDF do relatório, o plano de controle e as ferramentas de projeto (FMEA, A3, 8D) ainda não estão neste aplicativo. Os projetos de melhoria continuam no UTSim Seis Sigma.

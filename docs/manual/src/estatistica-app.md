# UTSim Estatística

O UTSim Estatística é o aplicativo de análise estatística da família UTSim, com janela própria. Ele trabalha com planilhas de dados digitados, importados de CSV ou xlsx, ou vindos da simulação, faz as análises mais usadas no dia a dia da engenharia e grava relatórios em HTML e PDF. Funciona sem internet e sem servidor. Esta versão cobre as etapas E1 e E2 de `docs/plano_estatistica.md`.

Pela linha de comando, `utsim-estatistica` abre a janela. Com `utsim-estatistica --utsim-abrir=<arquivo.csv ou .xlsx>` (a convenção da família, usada por exemplo pelo "Analisar no UTSim Estatística" das missões do UTSim Ensino), o app já abre numa planilha nova com o arquivo: na última unidade usada, onde a planilha é gravada; sem unidade conhecida, a planilha fica aberta e a barra pede para abrir ou criar uma unidade antes de analisar.

## A pasta de unidade

O projeto fica na mesma pasta de unidade do UTSim Qualidade e do UTSim Segurança do Trabalho. Na tela Unidade, escreva o seu nome, o caminho da pasta e use "Abrir unidade" (ou preencha o nome e use "Criar unidade"). Dentro da unidade o Estatística usa:

- `estatistica/planilhas/`: uma planilha por arquivo;
- o diário da unidade: o histórico das análises, que outras pessoas da equipe também veem;
- `relatorios/`: os relatórios em HTML e PDF;
- `estatistica/para_qualidade/`: os CSV mandados ao UTSim Qualidade.

## Planilha

Cada coluna tem um nome e um tipo: número, texto ou data. Clique no nome para selecionar a coluna e use o botão direito para renomear, ordenar ou remover. O tipo muda na caixa embaixo do nome. Numa coluna de números, `1,5` e `1.5` valem o mesmo; uma célula vazia é um valor ausente. Células que não combinam com o tipo ficam em vermelho e não entram nas contas.

"Gravar" guarda a planilha na unidade. Rodar uma análise grava a planilha antes, porque o histórico guarda o hash (SHA-256) dos dados usados.

### Importar

Em Arquivo › Importar CSV ou xlsx, escreva o caminho do arquivo. No CSV, o separador `;` usa vírgula decimal e o `,` usa ponto. No xlsx, escolha a aba quando houver mais de uma; o valor gravado das fórmulas é o que vale. A primeira linha com texto vira o cabeçalho, e o tipo de cada coluna é deduzido. A planilha importada guarda o nome do arquivo e o hash dos bytes como origem.

### Exportar

Arquivo › Exportar planilha em xlsx grava `estatistica/exportados/<nome>.xlsx` na unidade. A aba "Dados" tem o cabeçalho e as células (números como números); a aba "Origem" tem a origem dos dados, o hash e a data.

### Operações de dados (menu Dados)

- **Empilhar:** junta várias colunas numa só e cria uma coluna de rótulos com o nome da coluna de origem.
- **Desempilhar:** separa uma coluna de valores em uma coluna por rótulo de outra coluna.
- **Codificar:** cria uma coluna nova trocando valores (um código por linha, `antigo = novo`).
- **Subconjunto:** cria uma planilha nova só com as linhas que atendem a uma condição (`=`, `≠`, `<`, `≤`, `>`, `≥`, contém).
- **Ordenar:** reordena as linhas pela coluna escolhida.
- **Calculadora:** cria ou substitui uma coluna com uma expressão. Use os nomes das colunas (entre aspas simples quando têm espaço), `+ - * / ^`, `abs`, `raiz`, `ln`, `log10`, `exp`, `arred(x; casas)` e, sobre a coluna inteira, `media`, `desvio`, `soma`, `min`, `max`, `n` e `mediana`. Exemplo: `(x - media(x)) / desvio(x)` padroniza `x`.

## Análises (menu Estatística)

| Grupo | Análises |
|---|---|
| Estatística básica | descritiva, normalidade (Anderson-Darling), t de uma amostra, t de duas amostras (Welch ou variância combinada), t pareado, uma proporção, duas proporções, igualdade de variâncias (Levene e Bartlett), equivalência (TOST) |
| ANOVA | um fator com as comparações de Tukey, dois fatores |
| Não paramétricos | Mann-Whitney, Wilcoxon pareado, Kruskal-Wallis, mediana de Mood, Friedman |
| Tabelas | qui-quadrado de independência, com as contagens observadas e esperadas |
| Regressão e correlação | regressão linear (coeficientes, VIF, resíduos), correlação de Pearson ou Spearman com valores-p |
| Poder e tamanho de amostra | uma média, duas médias e duas proporções, com a curva de poder |

Cada análise abre um formulário com as colunas e os parâmetros. Os testes são bilaterais, com 95 % de confiança. Os grupos (ANOVA, Kruskal-Wallis, Mood, variâncias) vêm de uma coluna de rótulos ao lado da coluna de valores; se os grupos estiverem em colunas separadas, use Dados › Empilhar antes.

O resultado mostra a conclusão, os números, as tabelas e os gráficos. "Refazer com outras escolhas" reabre o formulário.

### Equivalência (TOST)

O TOST responde se duas médias são iguais na prática, isto é, se a diferença fica entre um limite inferior e um superior que você escolhe pelo processo (a menor diferença que importa). Ele faz dois testes t unilaterais. A conclusão é "equivalentes" quando os dois rejeitam, o que é o mesmo que o intervalo de confiança de 1 − 2α (90 % com α = 0,05) caber inteiro nos limites. As comparações são duas amostras (Welch), pareada ou uma amostra contra μ₀. Um teste t sem significância não prova igualdade; o TOST prova.

## Dados da simulação (menu Simulação)

"Abrir replicações ou experimento do UTSim" roda um arquivo `.ron` do UTSim pela mesma biblioteca do simulador e abre o resultado como planilha:

- **um modelo:** uma linha por replicação, com a coluna `replicacao` e uma coluna por indicador do relatório (`objeto.medida.estatística`, por exemplo `fila.content.mean`). O número de replicações pode ser trocado na hora; vazio usa o do modelo;
- **um experimento** (o arquivo de cenários do UTSim): uma linha por cenário e replicação, com as colunas `cenario`, uma por fator alterado (`base` quando o cenário não muda aquele fator) e `replicacao`;
- **as medições de qualidade de um modelo:** escolha "Medições de qualidade" para ter uma linha por característica medida em cada peça (`replicacao`, `item`, `objeto`, `caracteristica`, `instante`, `valor`), a mesma fonte do UTSim Qualidade. O modelo precisa de características no campo `quality` dos nós.

A planilha fica com a origem "Simulado", o caminho do modelo, as replicações, a semente e o SHA-256 do arquivo do modelo. Rodar o mesmo modelo de novo dá os mesmos números. A simulação roda em segundo plano; a janela segue respondendo.

### Comparar cenários

Escolha a resposta, a coluna dos cenários e a coluna das replicações. Os cenários de um experimento do UTSim usam as mesmas sementes em cada replicação (números aleatórios comuns), por isso a comparação é pareada: a tabela mostra a média de cada cenário, a diferença para o primeiro cenário com intervalo de 95 % e, com três cenários ou mais, todos os pares com a correção de Bonferroni.

### Precisão e replicações necessárias

Escolha um ou mais indicadores (uma linha por replicação) e a meia-largura desejada do intervalo, relativa à média (em %) ou absoluta. A análise mostra a meia-largura de agora, quantas replicações atingem a desejada e quantas faltam, com a curva da meia-largura esperada por número de replicações. Com vários indicadores vale o maior número.

### Ajuste de distribuição

Escolha a coluna de dados (por exemplo, tempos de ciclo medidos) e o critério (AIC, BIC, Kolmogorov-Smirnov ou Anderson-Darling). O ajuste é o mesmo do UTSim: todas as famílias candidatas com parâmetros, AIC, BIC, valores-p, a situação de cada uma (escolhida, aceita, rejeitada) e a justificativa. O histograma mostra a curva esperada da distribuição escolhida. Quando nenhuma família é aceita, a proposta é a distribuição empírica.

### Gravar no modelo

O ajuste de distribuição e a precisão propõem uma mudança no modelo: a distribuição para um campo de um nó (por exemplo `maquina.process`) ou o número de replicações (`run.replications`). Nada muda sozinho:

1. no resultado, "Abrir modelo…" (ou Simulação › Abrir modelo para gravar). O caminho já vem preenchido quando a planilha é simulada;
2. o Estatística toma a trava do modelo: enquanto ele estiver aberto aqui, o UTSim e os outros apps só leem. Se outra pessoa ou outro app já edita, o modelo abre só para leitura e a tela diz quem edita e desde quando ("Tentar editar" tenta de novo);
3. "Aplicar ao modelo…" escolhe o campo e aplica. Cada aplicação é um passo de desfazer na tela Modelo, que mostra os passos, os campos de distribuição e as replicações;
4. "Gravar no modelo" grava o arquivo inteiro de uma vez. "Fechar" solta a trava; o que não foi gravado se perde.

## Gráficos

Histograma, boxplot, gráfico de probabilidade normal e dispersão (com a reta de mínimos quadrados). As análises também trazem os gráficos que cabem a elas: boxplot por grupo na ANOVA, resíduos na regressão, curva de poder no tamanho de amostra.

## Histórico

A tela Histórico lista as análises da unidade, de todas as pessoas. "Rodar de novo" relê a planilha e recalcula. Se os dados mudaram desde a análise original, aparece um aviso. "Tirar" remove a análise do histórico.

## Relatórios

- **Desta análise:** no resultado, "Relatório HTML e PDF" grava os dois arquivos em `relatorios/`.
- **Consolidado:** marque análises no histórico e use "Relatório consolidado". Cada análise roda de novo com os dados de agora. O relatório tem capa, sumário, uma tabela de resumo e uma seção por análise.

Todo relatório mostra a origem dos dados (manual, importado com o hash, simulado) e o autor.

## Capacidade, cartas de controle, MSA e Pareto

Essas ferramentas ficam no UTSim Qualidade, a tela única da família para elas. O menu Qualidade do Estatística grava as colunas escolhidas num CSV em `estatistica/para_qualidade/` e abre o UTSim Qualidade, se ele estiver instalado pelo launcher. Para o Pareto, escolha a coluna das categorias (defeito, posto) e, se quiser, uma coluna de números para somar no lugar de contar: o CSV sai já contado, `categoria;contagem`, da maior para a menor. Por enquanto o resultado não volta sozinho ao Estatística. Isso chega com o pedido entre apps, numa próxima versão.

## O que vem depois

O plano (`docs/plano_estatistica.md`) prevê, nas próximas etapas: o pedido entre apps (o Qualidade e o UTSim pedindo análises e ajustes, e os dados indo junto ao Qualidade), exportar xlsx, experimentos planejados rodando o simulador, GLM, regressão logística, confiabilidade com censura, assistente guiado e séries temporais.

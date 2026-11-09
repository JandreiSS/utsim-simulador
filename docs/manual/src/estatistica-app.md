# UTSim Estatística

O UTSim Estatística é o aplicativo de análise estatística da família UTSim, com janela própria. Ele trabalha com planilhas de dados digitados, importados de CSV ou xlsx, ou vindos da simulação, faz as análises mais usadas no dia a dia da engenharia e grava relatórios em HTML e PDF. Funciona sem internet e sem servidor. Esta é a primeira versão (etapa E1 de `docs/plano_estatistica.md`).

Pela linha de comando, `utsim-estatistica` abre a janela.

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
| Estatística básica | descritiva, normalidade (Anderson-Darling), t de uma amostra, t de duas amostras (Welch ou variância combinada), t pareado, uma proporção, duas proporções, igualdade de variâncias (Levene e Bartlett) |
| ANOVA | um fator com as comparações de Tukey, dois fatores |
| Não paramétricos | Mann-Whitney, Wilcoxon pareado, Kruskal-Wallis, mediana de Mood, Friedman |
| Tabelas | qui-quadrado de independência, com as contagens observadas e esperadas |
| Regressão e correlação | regressão linear (coeficientes, VIF, resíduos), correlação de Pearson ou Spearman com valores-p |
| Poder e tamanho de amostra | uma média, duas médias e duas proporções, com a curva de poder |

Cada análise abre um formulário com as colunas e os parâmetros. Os testes são bilaterais, com 95 % de confiança. Os grupos (ANOVA, Kruskal-Wallis, Mood, variâncias) vêm de uma coluna de rótulos ao lado da coluna de valores; se os grupos estiverem em colunas separadas, use Dados › Empilhar antes.

O resultado mostra a conclusão, os números, as tabelas e os gráficos. "Refazer com outras escolhas" reabre o formulário.

## Gráficos

Histograma, boxplot, gráfico de probabilidade normal e dispersão (com a reta de mínimos quadrados). As análises também trazem os gráficos que cabem a elas: boxplot por grupo na ANOVA, resíduos na regressão, curva de poder no tamanho de amostra.

## Histórico

A tela Histórico lista as análises da unidade, de todas as pessoas. "Rodar de novo" relê a planilha e recalcula. Se os dados mudaram desde a análise original, aparece um aviso. "Tirar" remove a análise do histórico.

## Relatórios

- **Desta análise:** no resultado, "Relatório HTML e PDF" grava os dois arquivos em `relatorios/`.
- **Consolidado:** marque análises no histórico e use "Relatório consolidado". Cada análise roda de novo com os dados de agora. O relatório tem capa, sumário, uma tabela de resumo e uma seção por análise.

Todo relatório mostra a origem dos dados (manual, importado com o hash, simulado) e o autor.

## Capacidade, cartas de controle e MSA

Essas ferramentas ficam no UTSim Qualidade, a tela única da família para elas. O menu Qualidade do Estatística grava as colunas escolhidas num CSV em `estatistica/para_qualidade/` e abre o UTSim Qualidade, se ele estiver instalado pelo launcher. Por enquanto o resultado não volta sozinho ao Estatística. Isso chega com o pedido entre apps (etapa E2 do plano).

## O que vem depois

O plano (`docs/plano_estatistica.md`) prevê, nas próximas etapas: saídas da simulação como planilha, ajuste de distribuição gravado no nó do modelo, equivalência (TOST), experimentos completos rodando o simulador, GLM, regressão logística, confiabilidade com censura, assistente guiado e séries temporais.

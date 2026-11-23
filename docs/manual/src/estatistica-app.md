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

A planilha é uma grade de células sempre pronta para digitar, como numa planilha comum:

- clique numa célula e digite; Enter grava e desce, Tab grava e vai para a direita (com Shift, sobe ou volta), Esc desiste. F2 ou duplo clique editam o que já está na célula;
- as setas andam; com Shift, ou arrastando o mouse, a seleção cresce. Clique no cabeçalho para pegar a coluna inteira, no número para pegar a linha, no canto para pegar tudo (ou Ctrl+A);
- Ctrl+C copia a seleção com tabulações (cola direto no Excel); Ctrl+V cola o que veio do Excel a partir da célula ativa, e um valor só colado numa área preenche a área toda; Ctrl+X recorta; Delete apaga;
- escrever ou colar além do fim cria as linhas e colunas que faltam;
- Ctrl+Z desfaz e Ctrl+Y (ou Ctrl+Shift+Z) refaz, também as operações do menu Dados. Os botões da barra fazem o mesmo;
- duplo clique no cabeçalho renomeia a variável;
- o botão direito abre copiar, recortar, colar, apagar, inserir e remover linhas e colunas, renomear, o tipo da coluna e ordenar.

Cada coluna tem um nome e um tipo: número, texto ou data. O tipo é deduzido dos valores enquanto você digita; para escolher, use o botão direito › Tipo da coluna, e o cabeçalho mostra "(fixo)". "Automático" volta a deduzir. Numa coluna de números, `1,5` e `1.5` valem o mesmo. O separador de milhar segue a língua: em português `1.234` e `1.234,5` são mil duzentos e trinta e quatro (e quatro e meio); em inglês, `1,234` e `1,234.5`. Uma célula vazia é um valor ausente. Células que não combinam com o tipo ficam em vermelho e não entram nas contas. O rodapé mostra a célula ativa, o tamanho da seleção, a contagem, a soma e a média dos números selecionados.

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
| Estatística básica | descritiva, normalidade (Anderson-Darling), t de uma amostra, t de duas amostras (Welch ou variância combinada), t pareado, uma proporção, duas proporções, igualdade de variâncias (Levene e Bartlett), equivalência (TOST), intervalo de tolerância, transformação de Johnson |
| ANOVA | um fator com as comparações de Tukey, dois fatores |
| Não paramétricos | Mann-Whitney, Wilcoxon pareado, Kruskal-Wallis, mediana de Mood, Friedman |
| Tabelas | qui-quadrado de independência, com as contagens observadas e esperadas |
| Regressão e correlação | regressão linear (coeficientes, VIF, resíduos), regressão logística binária, correlação de Pearson ou Spearman com valores-p |
| Poder e tamanho de amostra | uma média, duas médias e duas proporções, com a curva de poder |

Cada análise abre um formulário com as colunas e os parâmetros. Os testes são bilaterais, com 95 % de confiança. Os grupos (ANOVA, Kruskal-Wallis, Mood, variâncias) vêm de uma coluna de rótulos ao lado da coluna de valores; se os grupos estiverem em colunas separadas, use Dados › Empilhar antes.

O resultado mostra a conclusão, os números, as tabelas e os gráficos. "Refazer com outras escolhas" reabre o formulário.

### Equivalência (TOST)

O TOST responde se duas médias são iguais na prática, isto é, se a diferença fica entre um limite inferior e um superior que você escolhe pelo processo (a menor diferença que importa). Ele faz dois testes t unilaterais. A conclusão é "equivalentes" quando os dois rejeitam, o que é o mesmo que o intervalo de confiança de 1 − 2α (90 % com α = 0,05) caber inteiro nos limites. As comparações são duas amostras (Welch), pareada ou uma amostra contra μ₀. Um teste t sem significância não prova igualdade; o TOST prova.

### Intervalo de tolerância

O intervalo de confiança fala da média; o de tolerância fala das peças. Ele dá os limites que guardam ao menos uma proporção da população (a cobertura, por exemplo 95 %) com a confiança escolhida (por exemplo 95 %). Pode ter os dois lados ou só um ("só o inferior" para uma resistência mínima, "só o superior" para um tempo máximo).

- **Normal:** `x̄ ± k·s`, com o fator `k` exato para o tamanho da amostra. Serve quando os dados são normais; se o Anderson-Darling rejeita a normalidade (p < 0,05), o resultado avisa.
- **Não paramétrico:** usa os próprios valores ordenados e vale para qualquer distribuição contínua. Pede mais dados: os extremos da amostra só chegam a 90 % de cobertura com 95 % de confiança a partir de 46 valores (dois lados) ou 29 (um lado). Com menos, o resultado mostra a confiança atingida e quantos valores faltam.

### Transformação de Johnson

Quando os dados não são normais e a conta pede a normal (capacidade, intervalo de tolerância normal, cartas), a transformação de Johnson leva os valores à normal. São três famílias: SU, sem limites; SB, limitada dos dois lados; e SL, a lognormal. O app escolhe a família e os parâmetros pelos quantis da amostra, testa cada candidata pelo Anderson-Darling e fica com a de maior p. O resultado mostra a fórmula com γ, η, ε e λ, o p antes e depois, os valores transformados e os gráficos de probabilidade normal antes e depois. Com p abaixo de 0,10 mesmo depois de transformar, use um método que não suponha a normal, como o intervalo de tolerância não paramétrico. São precisos ao menos 8 valores.

Os valores transformados podem ir para a planilha: no resultado, **Gravar as colunas** cria a coluna "<nome> Johnson", pronta para outras análises.

### Regressão logística binária

Quando a resposta tem só dois valores (peça boa ou com defeito, entregue no prazo ou atrasada), a logística dá a probabilidade do evento em função dos preditores. O evento é o maior dos dois valores: 1 contra 0, "sim" contra "não". Para trocar, codifique a coluna (menu Dados › Codificar).

Os preditores podem ser números ou texto. Um preditor de texto, como o turno, entra com o primeiro nível em ordem como referência, e cada outro nível ganha um coeficiente. A razão de chances diz quantas vezes as chances do evento crescem por unidade do preditor (ou de um nível contra a referência); acima de 1 aumenta o risco, abaixo de 1 diminui.

O resultado mostra:
- o teste G do modelo inteiro;
- o teste de Wald de cada coeficiente;
- o R² de McFadden e o AIC;
- o Hosmer-Lemeshow (um p baixo diz que a curva não acompanha os dados);
- a tabela de classificação com o corte escolhido (0,5 de partida).

**Gravar as colunas** leva a probabilidade ajustada de cada linha para a planilha. Se um preditor separa por completo os eventos dos não eventos, a logística não tem estimativa finita e o app explica. Com menos de 10 casos da classe menor por termo, as estimativas podem ser instáveis, e o resultado avisa.

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

## Experimentos planejados (menu DOE)

**DOE > Criar plano** monta o experimento como uma planilha nova:

- **Plackett-Burman:** para triagem de muitos fatores (até 23) com poucas corridas (8, 12, 16, 20 ou 24). Estima só os efeitos principais.
- **Composto central:** o fatorial completo, os pontos axiais e os centros, para o modelo de segunda ordem. Os pontos axiais podem ser rotacionáveis, na face do cubo (três níveis por fator) ou em dois blocos ortogonais (o fatorial num dia e os axiais em outro, por exemplo).
- **Box-Behnken:** de 3 a 5 fatores, três níveis, sem corridas nos cantos do cubo.

A planilha traz a ordem padrão, a ordem de execução (sorteada pela semente, dentro de cada bloco), o bloco, o tipo do ponto e os fatores codificados: −1 e +1 nos níveis baixo e alto, 0 no centro. Rode as corridas nessa ordem, preencha a coluna Resposta (ou acrescente outras colunas de resposta) e grave.

Para receber os fatores nas unidades do processo, preencha "Nível baixo (−1) no processo" e "Nível alto (+1) no processo", um valor por fator, separados por ponto e vírgula (por exemplo `150; 1,5` e `200; 2,5` para temperatura e pressão). Um fator sem níveis fica codificado. A planilha traz então 150 e 200 nos pontos do fatorial, 175 no centro e, nos axiais, o centro ± α vezes a meia-amplitude. Ela guarda os níveis de cada coluna (mesmo se a coluna for renomeada), e as análises abaixo codificam esses fatores sozinhas.

- **Analisar superfície de resposta:** ajusta o modelo de segunda ordem (ou de primeira, com ou sem interações) aos fatores codificados. Mostra os coeficientes com valor-p, o R² e os resíduos, e acha o ponto estacionário, dizendo se ele é um máximo, um mínimo ou uma sela. Avisa quando o ponto fica fora da região do experimento.
- **Otimizar respostas (desejabilidade):** para uma ou duas respostas, escolha maximizar, minimizar ou chegar a um alvo, com os limites aceitáveis. O app procura dentro da região do experimento o ponto de maior desejabilidade geral e mostra as respostas previstas nele.

**DOE > Rodar o plano no modelo** roda o plano aberto no simulador. Escolha o modelo do UTSim (`.ron`) e as replicações de cada corrida. Para cada fator, diga o campo do modelo que ele muda e o valor em RON, com `{x}` no lugar do nível:

| Fator | Campo do modelo | Valor (RON) | Inteiro |
|---|---|---|---|
| Tempo | `maquina.process` | `Exp(mean: {x})` | não |
| Servidores | `maquina.capacity` | `{x}` | sim |

"Inteiro" arredonda o nível, para campos que só aceitam números inteiros (num composto central, os pontos axiais caem entre os níveis). Os campos usam as mesmas chaves dos cenários do UTSim. Cada linha do plano vira uma corrida, e todas usam as mesmas sementes. O resultado abre como planilha nova, "Simulado", com as colunas do plano, a coluna `replicacao` e uma coluna por indicador do relatório. Analise essa planilha pelo menu DOE, escolhendo o indicador como resposta.

Em **Otimizar respostas** sobre essa planilha, o melhor ponto aparece em "Gravar no modelo do UTSim", como valores dos campos. Abra o modelo (o app toma a trava) e clique "Aplicar ao modelo": todos os campos mudam num só passo de desfazer, e nada vai ao disco até "Gravar".

Quando a planilha tem os níveis no processo, o ponto estacionário e o melhor ponto aparecem nas duas escalas: a coluna "nível no processo" da tabela e o fim da conclusão. Numa planilha sem níveis, volte às unidades do processo com: valor = centro + nível codificado × (alto − baixo)/2.

## Histórico

A tela Histórico lista as análises da unidade, de todas as pessoas. "Rodar de novo" relê a planilha e recalcula. Se os dados mudaram desde a análise original, aparece um aviso. "Tirar" remove a análise do histórico. "Renomear" (ou um duplo clique no nome) dá outro nome à análise: Enter grava, Esc cancela. Se a planilha de uma análise foi apagada, "Rodar de novo" fica desligado. Apagar uma planilha na tela inicial pede confirmação e diz quantas análises do histórico foram feitas sobre ela.

## Relatórios

- **Desta análise:** no resultado, "Relatório HTML e PDF" grava os dois arquivos em `relatorios/`.
- **Consolidado:** marque análises no histórico e use "Relatório consolidado". Cada análise roda de novo com os dados de agora. O relatório tem capa, sumário, uma tabela de resumo e uma seção por análise.

Todo relatório mostra a origem dos dados (manual, importado com o hash, simulado) e o autor.

## Capacidade, cartas de controle, MSA e Pareto

Essas ferramentas ficam no UTSim Qualidade, a tela única da família para elas. O menu Qualidade do Estatística grava as colunas escolhidas num CSV em `estatistica/para_qualidade/` e abre o UTSim Qualidade, se ele estiver instalado pelo launcher, já com um estudo da ferramenta sobre esse CSV. Na capacidade e na carta, escolha a coluna das medições na ordem da coleta (na carta, uma segunda coluna pode ser o subgrupo). No MSA, as colunas são achadas pelo nome: peça, operador e medida. Para o Pareto, escolha a coluna das categorias (defeito, posto) e, se quiser, uma coluna de números para somar no lugar de contar: o CSV sai já contado, `categoria;contagem`, da maior para a menor. O pedido fica gravado em `estatistica/pedidos/`, numa pasta de cada autor. Quando você fecha o Qualidade, os indicadores do estudo (Cpk, Ppk, regras da carta, % do sistema de medição, conforme a ferramenta), os achados e a conclusão voltam ao Estatística numa janela "Resposta do UTSim Qualidade". Se o Estatística já estiver fechado, a resposta aparece na próxima vez que você abrir a unidade. Cada pessoa vê só as respostas dos pedidos dela, e as já lidas vão para `lidos/`.

### Pedido de outro app

O UTSim e o Seis Sigma podem pedir um experimento ao Estatística (pedido `planejar_experimento`). O pedido traz o modelo, os fatores com o campo e os níveis baixo e alto, os indicadores de resposta e, se quiser, o número máximo de corridas e as replicações. O Estatística abre na unidade do pedido (ou na última usada) com o plano pronto: o composto central de face centrada quando cabe no orçamento, senão o Box-Behnken e, para muitos fatores, o Plackett-Burman. A janela "Rodar o plano no modelo" já vem preenchida; confira os campos e rode. Analise, aplique o melhor ponto e grave como de costume. Ao fechar o Estatística, a resposta volta para quem pediu com o plano, a rodada, o melhor ponto aplicado e o que foi gravado no modelo. O modelo só muda se você aplicar e gravar.

## O que vem depois

O plano (`docs/plano_estatistica.md`) prevê, nas próximas etapas: os pedidos `analisar` e `ajustar_distribuicao` de outros apps, GLM, stepwise, confiabilidade com censura, assistente guiado e séries temporais.

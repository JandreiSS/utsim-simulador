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
5. **Acompanhar.** Registre reavaliações com data, probabilidade, severidade e nota. O nível atual é o da última reavaliação.

No alto da tela ficam os alertas em vermelho: os perigos acima do nível mais baixo que ainda não têm ação e as ações com prazo vencido.

As edições ficam num rascunho até você usar "Gravar", para que cada tecla digitada não vire uma alteração na pasta compartilhada.

## Matriz

A matriz padrão é uma grade própria de 5 por 5, com níveis baixo, médio, alto e crítico. Na tela Matriz, clique numa célula para trocar o nível dela e edite os nomes, as cores e as decisões dos níveis. É possível importar a matriz da sua empresa por CSV (de 3 por 3 até 5 por 5) e exportá-la. Uma matriz em que mais probabilidade ou mais severidade baixa o nível é recusada, e o programa mostra o motivo. "Gravar na unidade" faz a matriz valer para toda a unidade.

## Exposição

Escolha um modelo e use "Rodar". A simulação roda em segundo plano e mostra, com média e intervalo de confiança, o tempo das pessoas em zonas de risco, as entradas sem o EPI pedido e os quase-conflitos entre pessoas e veículos. Tudo é simulado, serve para comparar layouts e não prevê acidentes.

"Sugerir perigos" propõe um perigo para cada zona em que alguém de um posto esteve. Marque os que fizerem sentido e grave. Eles entram no inventário sem avaliação, e a probabilidade e a severidade ficam por sua conta. "Gravar HTML de exposição" escreve o relatório na pasta `relatorios/`.

## Ergonomia

No alto da tela aparece o aviso da NR-17: a nota é uma pontuação própria para estudo, ensino e comparação, e não substitui a análise ergonômica do profissional habilitado. Escolha um modelo (pelo caminho ou pela lista dos modelos dos postos), a unidade de tempo (s, min ou h) e use "Rodar estudo". O estudo de tempos roda em segundo plano.

Para cada elemento de trabalho de cada operador, marque a postura do tronco, dos braços, do pescoço e das pernas, a carga em kg, a pega e a vibração. A nota de 0 a 10 e a faixa aparecem ao vivo, calculadas com o método da unidade. Elementos sem marcação contam como neutros. A repetição e a duração vêm dos ciclos medidos no estudo.

- **Gravar marcações** guarda as marcações na pasta de unidade, e só nesse momento. Ao rodar o mesmo modelo de novo, elas voltam.
- **Importar método (CSV)** troca os pesos e as faixas pelos da sua empresa e grava o método na unidade. Se o CSV for recusado, o motivo aparece na tela.
- **Rodar e comparar** estuda um segundo modelo e mostra a nota e o tempo padrão antes e depois, por posto.
- **Gravar HTML de ergonomia** escreve `relatorios/ergonomia.html`.

## Relatório

Escolha todos os postos ou só alguns e use "Gravar HTML do inventário". O arquivo vai para a pasta `relatorios/` da unidade e pode ser impresso pelo navegador. O relatório não é o documento legal do programa de gerenciamento de riscos. O PDF virá em uma etapa futura.

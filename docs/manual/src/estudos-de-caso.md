# Estudos de caso

Depois dos [tutoriais](tutoriais/index.md), o projeto traz três estudos de caso no estilo do que um engenheiro de produção levaria ao programa. Eles ficam na pasta `docs/estudos_de_caso/` do projeto. Cada um é um **enunciado** completo (contexto da empresa, dados, regras do processo e perguntas de decisão com critérios de aceitação), e não diz como montar o modelo: essa parte é com você.

| Caso | Tema | Dificuldade |
|---|---|---|
| 1. Linha de montagem com inspeção e retrabalho | linha de 4 estações com montagem de kit, quebras, dois turnos, inspeção com 12% de defeito e retrabalho; quantos inspetores e onde pôr um buffer | média |
| 2. Centro de distribuição com corredor estreito | docas, esteira, porta-paletes, empilhadeiras e pickers; quantas pessoas e máquinas, e se vale mudar a embalagem de lugar | alta |
| 3. Pronto atendimento com turnos e prioridade | chegadas que variam com a hora, triagem, urgentes na frente e a escala de pessoal de menor custo | média a alta |

Cada pasta tem `enunciado.md` (o que você recebe), `dados/` (arquivos de dados brutos para ajustar distribuições com o [`utsim fit`](ajuste-de-distribuicoes.md)) e `gabarito/` (o modelo de referência, os experimentos e a resolução comentada, para conferir depois de resolver).

Sugestão de uso:

1. Leia o enunciado inteiro e liste o que o modelo precisa representar.
2. Ajuste as distribuições dos arquivos de dados.
3. Monte o modelo no editor, valide com `utsim check` e confira os números básicos (taxas, utilização, lei de Little) antes de comparar alternativas.
4. Responda às perguntas com [experimentos e otimização](experimentos-e-otimizacao.md), sempre com intervalos de confiança.
5. Só então abra o `gabarito/` e compare.

Se alguma coisa do enunciado não couber nos objetos do programa, anote: o que você precisou aproximar é uma informação valiosa.

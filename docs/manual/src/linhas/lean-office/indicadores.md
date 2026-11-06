# Indicadores e relatório

## Indicadores

Os indicadores contam os casos concluídos depois do aquecimento; o WIP é a média no tempo da
mesma janela. Cada valor vem com o intervalo de 95 % entre replicações.

| Indicador | Definição |
|---|---|
| Lead time (L/T) | da chegada do caso à conclusão |
| Tempo de processamento (P/T) | trabalho sorteado para o caso nas etapas (postos, aprovações, verificações, reuniões); tramitação e espera por informação não são trabalho |
| Touch time | parte do trabalho feita por pessoas (postos com equipe) |
| Razão de atividade | touch time médio ÷ lead time médio |
| %C&A por etapa | 1 − erros gerados pela etapa (rótulo `erros_<etapa>`) ÷ visitas à etapa |
| %C&A acumulado (rolled) | produto do %C&A das etapas |
| FPY | fração dos casos concluídos sem nenhuma volta e sem erro pendente |
| WIP de casos | casos em aberto no modelo, média no tempo (e casos parados por objeto) |
| % dentro do SLA | casos com rótulo `sla` concluídos em até `sla` minutos |
| Casos concluídos por dia | concluídos ÷ dias corridos da janela |
| Voltas de retrabalho | maior número de visitas a uma mesma etapa − 1, por caso |
| Ocupação das pessoas | fração do tempo trabalhando no caso, trocando de caso ([multitarefa](multitarefa.md)), andando, livre e fora do expediente; ocupação no expediente e por hora do dia |
| Esperas por causa | tempo médio por caso na caixa de entrada, na etapa sem trabalho (pessoa ocupada, fora do expediente, bloqueio), esperando informação de fora, em tramitação, esperando a reunião, esperando completar o lote |

Vale sempre **lead time = tempo de processamento + soma das esperas**: o relatório usa isso
para o Pareto das esperas.

Os indicadores vêm de métricas por caso que o núcleo guarda só quando a linha pede: visitas,
permanência, trabalho e touch time por etapa. Com elas desligadas (todo `utsim run` sem
`--linha`), os resultados do modelo são idênticos.

## Relatório da linha

O `--html` de `utsim linha lean_office relatorio` grava uma página com:

- cartões com lead time, touch time, razão de atividade, FPY, SLA e WIP;
- a tabela de indicadores com as definições;
- o gráfico do WIP de casos no tempo, o histograma do lead time com a linha do SLA e o
  Pareto das esperas por causa;
- as tabelas das etapas (P/T e touch por visita, %C&A), dos casos parados por objeto e da
  ocupação das pessoas, também por hora do dia;
- a distribuição das voltas de retrabalho;
- o **mapa do fluxo de valor do processo administrativo** do caminho principal: cada etapa com P/T, touch,
  L/T da etapa, %C&A, pessoas, horário e departamento; as caixas de entrada com os casos
  parados e a espera; a escada do lead time (espera em vermelho, trabalho em verde) e o rodapé
  com lead time, touch time, razão de atividade e %C&A acumulado. Com `--comparar`, o mapa do
  estado futuro;
- o **fluxograma de raias por departamento**, com as raias do campo `department` e as
  passagens entre departamentos contadas.

O `--json` grava os indicadores de cada replicação, para planilhas e para o UTSim Seis Sigma.

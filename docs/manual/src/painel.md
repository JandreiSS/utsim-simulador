# Painel para gestores e engenheiros

O comando `utsim painel` roda as replicações de um modelo e serve, no próprio computador, uma
página com os indicadores (KPIs), os intervalos de confiança e os semáforos. Não há serviço
externo, conta, nuvem nem internet: a página, o estilo, os gráficos (SVG desenhados em
JavaScript puro, sem biblioteca) e os dados vêm do próprio programa.

```text
utsim painel modelo.ron [--porta 8080] [--config x.painel.ron] [-n 30] [--abrir]
utsim painel modelo.ron --exportar pasta/
```

- Sem `--exportar`, o servidor escuta só em `127.0.0.1` (esta máquina) e recusa pedidos cujo
  endereço não seja o local. Abra `http://127.0.0.1:8080/` no navegador (ou use `--abrir`).
  Enquanto as replicações terminam, a página se atualiza: os intervalos encolhem a cada
  replicação, e o botão "Cancelar" interrompe o estudo mantendo as replicações prontas.
- Com `--exportar`, roda tudo, grava a pasta e sai, sem servidor. A pasta tem `index.html`
  (página única com os dados dentro, abre com duplo clique, sem rede), `kpis.csv`,
  `objetos.csv`, `replicacoes.csv`, `painel.json` e `painel.xlsx`.
- O painel liga as amostras de série (120 por replicação) e os histogramas (40 classes) quando
  o modelo não os define, para ter os gráficos e os percentis. Os resultados das replicações
  são os mesmos de `utsim run`.

## Visão do gestor

Uma tela para ler em segundos: a frase-resumo, os cartões e o Pareto de perdas de tempo.

- **Cartão**: valor com a precisão da estimativa (`118 ± 3`, o intervalo de confiança da
  média), a faixa esperada numa rodada (percentis 5 a 95 entre as replicações), um minigráfico
  com o valor de cada replicação e, havendo meta, o semáforo.
- **Semáforo pela probabilidade**: a probabilidade de cumprir a meta é a fração das
  replicações que a cumprem, com o intervalo de Wilson. Verde a partir de 90 %, amarelo a
  partir de 50 %, vermelho abaixo (configurável). Com menos de duas replicações o semáforo fica
  "indefinido". Quando o intervalo de confiança da média contém a meta, o cartão avisa
  "indefinido" e diz quantas replicações a mais resolveriam.
- **Pareto**: tempo médio por rodada em que cada recurso ficou bloqueado, esperando operador
  ou transporte, parado ou em falha.

## Visão do engenheiro

| Página | Pergunta |
|---|---|
| Objetos | tabela ordenável e filtrável (CSV e `.xlsx`); barras de ocupação com intervalo; estados por objeto. Clique abre o objeto: medidas, estados, Gantt, conteúdo no tempo com banda, histogramas |
| Distribuições | gráfico "erro e risco" de cada KPI: pontos (replicações), caixa P25-P75, bigode P5-P95, média com intervalo de confiança; histograma do tempo no sistema |
| Séries | WIP no tempo, média com banda do intervalo de confiança e faixa P5-P95 |
| Gantt | estados de todos os objetos na primeira replicação |
| Estatística | tabela completa dos KPIs (n, intervalos, replicações para ±2 %, probabilidade e Wilson), lei de Little, fórmula de cada número |

## KPIs e fórmulas

`T` é o tempo observado depois do aquecimento. Cada KPI é calculado por replicação e resumido
entre replicações (média, intervalo de t de Student, intervalo de uma rodada
`média ± t·s·√(1+1/n)`, percentis P5, P25, P50, P75 e P95).

| id | Fórmula | Unidade |
|---|---|---|
| `vazao` | chegadas às Sinks / `T` (por hora conforme `time_unit`) | itens/h |
| `lead_time` | média de `time_in_system` das Sinks, ponderada pelos itens | tempo |
| `lead_time_p90` | percentil 90 do tempo no sistema, pelos histogramas das Sinks | tempo |
| `wip` | soma do conteúdo médio ponderado no tempo de filas, estações, esteiras, estantes, montagens e separadores | itens |
| `utilizacao_gargalo` | fração do tempo trabalhando do recurso mais ocupado | % |
| `utilizacao_gargalo_turno` | o mesmo sobre o tempo em turno (fora do turno sai da conta) | % |
| `utilizacao_media` | média da fração trabalhando entre os recursos | % |
| `oee:<objeto>` | disponibilidade × desempenho × qualidade (ver abaixo) | % |
| `otd` | itens no prazo / itens com prazo | % |
| `perdas` | perdidos / (perdidos + entrados) das fontes com `on_blocked: Drop` | % |
| `nivel_servico` | fração dos itens com tempo no sistema `<= lead_time_limit` | % |

Um KPI cujos dados o modelo não tem **não aparece**: o painel não inventa número.

- **OEE** só existe para os objetos listados em `oee` no arquivo do painel (o ciclo ideal não
  é campo do modelo). Disponibilidade `= 1 - tempo em falha / tempo planejado` (tempo fora de
  turno não conta como planejado); desempenho `= ciclo ideal × saídas / tempo operando`;
  qualidade `= 1 - chegadas nas Sinks de refugo / chegadas em todas as Sinks` (sem
  `scrap_sinks`, 100 %, e o cartão avisa). Os três fatores aparecem no cartão.
- **Atendimento de prazo** segue a convenção do rótulo `prazo` (instante absoluto de entrega):
  o painel lê das Sinks as métricas próprias `prazo.no_prazo` e `prazo.total`. Enquanto o
  núcleo não as registra, o cartão não aparece.
- **Lei de Little** (página Estatística): confere `WIP ≈ vazão × lead time`. Itens fora das
  filas e estações contadas (transportadores) podem fazer a lei "não fechar".

## Arquivo do painel (`<modelo>.painel.ron`)

Opcional e separado do modelo; sem ele o painel se monta sozinho. `--config` escolhe outro
arquivo. Nomes de campo desconhecidos são erro (pega erros de digitação).

```ron
Dashboard(
    title: Some("Linha 1"),
    time_unit: Seconds,                  // Seconds, Minutes, Hours ou Unit (modelos abstratos)
    level: 0.95,                         // confiança dos intervalos
    thresholds: (green: 0.9, yellow: 0.5),
    cards: ["vazao", "lead_time_p90", "wip", "utilizacao_gargalo"],   // ordem do gestor
    targets: {"vazao": AtLeast(120.0), "lead_time_p90": AtMost(900.0)},
    lead_time_limit: Some(600.0),        // liga `nivel_servico`
    oee: [(object: "M2", ideal_cycle: 40.0, scrap_sinks: ["refugo"])],
)
```

Metas e cartões que não casam com nenhum KPI viram avisos na página. KPIs em `%` usam a
escala 0 a 100 nas metas (`AtMost(80.0)` é 80 %).

## Impressão, PDF e planilha

- **PDF**: "Imprimir / PDF" no topo abre a impressão do navegador, que tem folha de estilo para
  A4 (cartões em grade, gráficos vetoriais, sem menus); escolha "Salvar como PDF". Imprime a
  página aberta.
- **CSV**: tabela de objetos e KPIs. **`.xlsx`**: abas Resumo, KPIs, Objetos, Replicações e
  Série WIP (compilação com a feature `xlsx`, ligada no instalador padrão). **JSON**: todas as
  rotas da API.
- Cada gráfico tem um `title` com os valores ao passar o mouse; as cores seguem os estados da
  cena 3D, com tema claro e escuro.

## API

Somente leitura, JSON, em `127.0.0.1`: `/api/resumo`, `/api/kpis`, `/api/objetos`,
`/api/objeto/{nome}`, `/api/series`, `/api/distribuicoes`, `/api/pareto`, `/api/gantt`,
`/api/estados`, `/api/config`, `/api/progresso` e `/api/export/{kpis.csv,objetos.csv,painel.json,painel.xlsx}`.
O único `POST` é `/api/cancelar`, que exige o cabeçalho `X-UTSim`.

Ainda não há: gêmeo digital ao vivo, alertas, modo quiosque, acesso pela rede local com token,
Sankey e comparação de cenários (ver `docs/plano_gemeo_digital_e_painel.md`).

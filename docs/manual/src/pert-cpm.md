# PERT/CPM de uma parte do processo

Depois de montar a fábrica inteira, você pode escolher um trecho e pedir o PERT/CPM dele: a
rede de atividades, o início e o fim mais cedo e mais tarde de cada atividade, as folgas
total e livre, os caminhos críticos, a distribuição do tempo total e a probabilidade de
cumprir um prazo. Por enquanto o cálculo está na linha de comando (`utsim pert`); a janela
na interface vem depois.

## Escolhendo o trecho

Há duas formas:

- **Objetos:** `--objetos corte,solda,pintura` usa exatamente esses objetos;
- **De A até B:** `--de corte --ate embalagem` usa todos os objetos que estão em algum
  caminho de `corte` até `embalagem` pelas ligações, com a mesma ideia de uma medição
  `spans`.

```bash
utsim pert examples/montagem_paletes.ron --de paletes --ate saida_caixas --prazo 10
utsim pert examples/centro_de_distribuicao.ron --de chegada_caminhoes --ate expedicao \
    --modo montecarlo --prazo 4000 --html pert.html --csv pert/
```

## Como a rede é montada

Cada objeto vira uma **atividade** (atividade no nó) e cada ligação entre dois objetos do
trecho vira uma precedência fim-início.

| Objeto | Duração da atividade |
|---|---|
| `Processor` | `setup` (se houver) + `process`; a capacidade não muda a duração de um item |
| `Combiner` | `setup` + `process`; é uma **junção E**: começa quando a última entrada chega |
| `Separator` | `process`; é uma **divisão E**: as saídas seguem em paralelo |
| `Conveyor` | `length / speed`, o tempo mínimo de percurso |
| `Rack` | `dwell` (0 sem ele) |
| `Queue` | 0 no cálculo clássico; a espera aparece no modo simulado |
| `Source`, `Sink` | 0 (marcos) |
| ligação com `transfer` | atividade própria, `delay + distância / speed` |
| ligação com `transport` | atividade própria: viagem vazia + carga + viagem carregada + descarga, com a distância da rede de caminhos |

O relatório sempre lista as **hipóteses** e os **avisos**. Os principais:

- **Saídas OU.** Um objeto com várias saídas que não é `Separator` manda cada item para
  **uma** delas. O PERT clássico não representa isso. Com `Weighted`, a rede fica com o
  ramo mais provável; com as outras regras, os ramos são tratados como servidores paralelos
  equivalentes e a rede usa o primeiro. Os objetos que só estavam nos ramos descartados
  saem da rede.
- **Retrabalho.** Uma ligação de volta forma um ciclo, que o CPM não aceita. A rede é
  cortada nessa ligação e as atividades do laço recebem a duração multiplicada pelo número
  esperado de passagens, `1 / (1 - p)`, com `p` vindo do `Weighted` de quem manda de volta.
- **Recursos compartilhados.** O cálculo clássico e o Monte Carlo ignoram filas, operadores,
  transportadores ocupados, quebras e turnos. Só o modo simulado mede essas esperas, por isso
  não compare os números dos modos sem levar isso em conta.
- **Partes desconectadas.** Uma seleção com partes sem ligação entre si vira caminhos
  paralelos do início ao fim da rede.

## Os três modos

`--modo simulado` (o padrão) roda o modelo inteiro, com as replicações dele (`-n` muda o
número), e segue cada unidade que sai do trecho de volta pelo que de fato a segurou:

- em cada objeto, o tempo é dividido em **espera** (fila, componentes faltando, operador,
  quebra, turno), **trabalho** (preparação e processo) e **bloqueio** (item pronto que não
  pode sair, inclusive esperando a empilhadeira chegar e carregar);
- numa montagem, a entrada crítica é a que chegou por último;
- o que sai de um `Separator` continua pelo item que entrou nele.

O tempo total de uma unidade vai da entrada no primeiro objeto do trecho à saída do último.
O resultado traz o tempo total médio com o intervalo de 95% entre replicações, os percentis,
a fração de unidades dentro do prazo, o **índice de criticidade observado** de cada
atividade (fração das unidades cujo caminho passa por ela) e os caminhos mais frequentes.
É o modo para a pergunta "o que atrasa esta parte da fábrica".

`--modo classico` é o PERT de livro: estimativas `(a, m, b)` tiradas das distribuições do
modelo (`Const` dá `a = m = b`, `Triangular` dá os três, `Uniform` dá `m` no meio, as outras
usam os quantis de 1% e 99% e a moda), o tempo esperado do caminho crítico, o desvio padrão
e `P(T ≤ prazo) = Φ((prazo - μ) / σ)`. Por padrão a média e a variância de cada atividade
são as exatas da distribuição; `--momentos classicos` usa `te = (a + 4m + b) / 6` e
`σ² = ((b - a) / 6)²`. Quando há caminhos quase críticos em paralelo, o relatório avisa que
esse cálculo subestima o tempo total.

`--modo montecarlo` sorteia as durações (`--sorteios`, 10 000 por padrão) e faz um CPM por
sorteio: média e percentis do tempo total, `P(T ≤ prazo)` com intervalo, o índice de
criticidade de cada atividade e de cada caminho e a **sensibilidade** (correlação de postos
entre a duração da atividade e o tempo total, e a crucialidade, que é essa correlação vezes
a criticidade). A mesma semente dá o mesmo resultado em qualquer computador e com qualquer
número de núcleos.

Em todos os modos o relatório termina com a tabela do CPM sobre as durações médias (ES, EF,
LS, LF, folgas) e os caminhos críticos.

## Arquivos

- `--html arquivo.html`: relatório autocontido com o diagrama da rede (caminho crítico em
  destaque, seta mais grossa quanto maior a criticidade), o Gantt, o histograma do tempo
  total com a linha do prazo, o tornado da sensibilidade e a tabela.
- `--csv pasta/`: `atividades.csv` (estimativas, datas, folgas e criticidades),
  `caminhos.csv`, `tempo_total.csv`, `diagrama.csv` (posição de cada caixa, para desenhar a
  rede em outra ferramenta) e, no Monte Carlo, `sensibilidade.csv`.

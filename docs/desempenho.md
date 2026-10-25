# Desempenho e robustez

Medições da fase 11: vazão do núcleo, da gravação do replay, da cena 3D e do fluxo JSONL em modelos sintéticos de 10, 100 e 500 objetos com 100 mil itens, o que foi otimizado, as metas (RNF) que os testes cobram e a robustez do leitor de modelos.

## Máquina e método

| Item | Valor |
|---|---|
| CPU | AMD Ryzen 9 5900XT, 16 núcleos / 32 threads |
| Memória | 32 GB |
| Sistema | Windows 11 Pro |
| Compilador | rustc 1.99, perfil `release` (`cargo bench`), uma replicação, uma thread |

Os números variam alguns por cento entre rodadas (a máquina é compartilhada); cada etapa é a mediana de até três execuções (uma só quando passa de 1 s).

Modelos: `utsim gerar-modelo --objetos N --tipo linha|rede|armazem --itens K` (código em `utsim_model::synthetic`). Todos têm carga de cerca de 0,7 por servidor e a fonte cria exatamente `K` itens.

| Tipo | Forma |
|---|---|
| linha | fonte, pares fila e processador em série, sorvedouro |
| rede | fonte com saída `RoundRobin` para ramos paralelos fila e processador, sorvedouro comum |
| armazém | fonte, fila, porta-paletes (6 x 4 posições) em paralelo, expedição |

Reproduzir:

```
cargo bench -p utsim-scene --bench desempenho                       # tabelas deste documento
cargo bench -p utsim-scene --bench desempenho -- --itens 20000 --objetos 10,100
cargo test --release -p utsim-scene --test desempenho -- --ignored --nocapture   # metas (RNF)
utsim gerar-modelo --objetos 500 --tipo rede --itens 100000 --saida grande.ron
```

## Resultados (100 mil itens)

Colunas: eventos/s e itens/s são do núcleo (`run_replications`); "replay" grava o rastro completo; "Scene::build" indexa o rastro; "1 quadro" é o custo de `Scene::frame`; "JSONL" escreve 300 quadros (mais o cabeçalho) no fluxo de `utsim cena`.

### Depois das otimizações

| tipo | objetos | eventos | núcleo (s) | eventos/s | itens/s | replay (s) | registros | Scene::build (s) | 1 quadro (ms) | JSONL 300 quadros (s) | JSONL (MB) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| linha | 10 | 500 mil | 0,109 | 4,58 M | 917 mil | 0,187 | 3,3 M | 0,416 | 0,003 | 0,002 | 0,8 |
| linha | 100 | 5 M | 1,312 | 3,81 M | 76 mil | 2,607 | 34,8 M | 2,408 | 0,034 | 0,055 | 8,8 |
| linha | 500 | 25 M | 8,859 | 2,82 M | 11,3 mil | 14,067 | 174,8 M | 10,686 | 0,463 | 0,361 | 44,5 |
| rede | 10 | 200 mil | 0,038 | 5,31 M | 2,65 M | 0,073 | 1,2 M | 0,275 | 0,002 | 0,001 | 0,7 |
| rede | 100 | 200 mil | 0,043 | 4,68 M | 2,34 M | 0,072 | 1,2 M | 0,149 | 0,019 | 0,021 | 6,7 |
| rede | 500 | 200 mil | 0,051 | 3,90 M | 1,95 M | 0,081 | 1,2 M | 0,120 | 0,174 | 0,100 | 33,4 |
| armazém | 10 | 200 mil | 0,045 | 4,41 M | 2,20 M | 0,074 | 1,2 M | 0,158 | 0,016 | 0,017 | 5,4 |
| armazém | 100 | 200 mil | 0,062 | 3,23 M | 1,62 M | 0,094 | 1,2 M | 0,129 | 0,237 | 0,171 | 72,0 |
| armazém | 500 | 200 mil | 0,111 | 1,80 M | 902 mil | 0,140 | 1,2 M | 0,119 | 2,638 | 1,097 | 338,8 |

### Antes (mesma máquina, mesmo modelo, primeira versão da fase 11)

| tipo | objetos | núcleo (s) | eventos/s | itens/s | replay (s) | Scene::build (s) | 1 quadro (ms) | JSONL 300 quadros (s) |
|---|---|---|---|---|---|---|---|---|
| linha | 10 | 0,119 | 4,21 M | 842 mil | 0,198 | 0,241 | 0,003 | 0,002 |
| linha | 100 | 1,596 | 3,13 M | 63 mil | 2,688 | 2,399 | 0,036 | 0,060 |
| linha | 500 | 10,089 | 2,48 M | 9,9 mil | 15,425 | 33,928 | 0,522 | 0,478 |
| rede | 10 | 0,065 | 3,08 M | 1,54 M | 0,114 | 0,174 | 0,002 | 0,002 |
| rede | 100 | 0,068 | 2,94 M | 1,47 M | 0,097 | 0,159 | 0,037 | 0,032 |
| rede | 500 | 0,084 | 2,38 M | 1,19 M | 0,132 | 0,162 | 0,138 | 0,108 |
| armazém | 10 | 0,073 | 2,76 M | 1,38 M | 0,111 | 0,196 | 0,036 | 0,029 |
| armazém | 100 | 0,110 | 1,81 M | 906 mil | 0,178 | 0,387 | 0,482 | 0,307 |
| armazém | 500 | 0,199 | 1,01 M | 503 mil | 0,269 | 0,283 | 7,223 | 3,126 |

Leitura:

- O núcleo ganha de 10 % a 45 % (mais nos modelos pequenos e nos de armazém, onde o custo fixo por evento pesa mais).
- O custo cresce linearmente com o número de eventos: a linha de 500 objetos faz 5 vezes os eventos da de 100 e leva 6,7 vezes o tempo (efeito de cache, não de algoritmo).
- `Scene::build` e o replay são limitados por memória: a linha de 500 objetos grava 175 milhões de registros (cerca de 10 GB). Para modelos assim, grave só um trecho (`utsim replay`/`cena --ate`, ou reduza `stop_time`).
- O quadro de 500 racks (8,8 mil poses) cai de 7,2 ms para 2,6 ms; o resto do custo é a montagem de `items_at`.
- O fluxo JSONL escreve de 100 a 300 MB/s; o tamanho (cerca de 240 bytes por pose) é do protocolo `utsim-scene/1`.

## Pontos quentes e correções

Sem criador de perfil externo: a análise foi feita lendo o código, cronometrando as fases por dentro de `Timeline::build` (temporário) e comparando variantes. Nenhuma correção muda resultados: os testes de instantâneo (`golden`) e os de teoria de filas (`queueing_theory`) seguem passando sem alteração.

| Onde | Problema | Correção |
|---|---|---|
| `Simulation::run_until` | um `HashSet::remove` (SipHash) a cada evento retirado da fila, mesmo sem nenhum cancelamento | só consulta o conjunto quando há cancelamentos pendentes |
| `Ctx::record` | alocava uma `String` para a chave a cada chamada (o sorvedouro chama uma vez por item) | busca antes e só aloca na primeira vez |
| `StateTracker` | um `BTreeMap` atualizado a cada mudança de estado | vetor curto ordenado por estado (a saída continua ordenada igual) |
| `Timeline::build` | `HashMap` com SipHash para achar o item de cada registro; o mesmo em `slots`, consultado por quadro | `FxMap` (hash multiplicativo) para chaves inteiras |
| `Timeline::build` | mediana global por ordenação completa de todos os intervalos (35 milhões), calculada mesmo com trânsito fixo; dois vetores novos por item | seleção em tempo linear, só no modo automático; sem vetores por item |

Medidas que ficaram de fora por exigirem mudar tipos públicos: reduzir o tamanho de `TraceRecord` (56 bytes, por causa do `Vec` de `Travel`) e `ItemStep` (usa `usize`).

## Metas (RNF)

Mantidas como testes `#[ignore]` em `crates/utsim-scene/tests/desempenho.rs` (rode em `release`). As metas ficam em cerca de um terço do medido, para passar em máquinas mais lentas e falhar só diante de uma regressão real.

| Código proposto | Meta | Medido aqui |
|---|---|---|
| RNF-P1 | o núcleo processa pelo menos 1 milhão de eventos por segundo (10 a 100 objetos) | 3,2 a 4,6 milhões |
| RNF-P2 | 100 mil itens por uma linha de 100 objetos em menos de 10 s | 1,3 s |
| RNF-P3 | gravar o replay custa no máximo 3 vezes a execução; `Scene::build` indexa pelo menos 2 milhões de registros por segundo | 1,4 vezes; 15 a 20 milhões |
| RNF-P4 | um quadro de uma cena de 500 objetos em menos de 16 ms (60 quadros/s) e fluxo JSONL de pelo menos 30 MB/s | 2,6 ms; 100 a 290 MB/s |

Dois testes comuns (sem `#[ignore]`) protegem a escala em qualquer perfil: 100 mil itens no núcleo em menos de 60 s mesmo em depuração, e gravar, indexar e transmitir 10 mil itens em menos de 60 s. Um algoritmo quadrático estouraria os dois com folga.

## Robustez do leitor de modelos

`crates/utsim-model/tests/robustez.rs` (proptest) passa textos aleatórios, exemplos mutilados (truncados, com caracteres apagados, números trocados ou trocados por valores extremos, nomes vazios ou repetidos, campos desconhecidos, lixo inserido) por `Model::parse`, `validate`, uma execução curta e `replay`. O resultado deve ser sucesso ou um erro em português; pânico, travamento (mais de 60 s) ou mensagem fora do padrão reprovam. `crates/utsim-fit/tests/robustez.rs` faz o mesmo com CSV aleatório e tabelas degeneradas no leitor de dados e no ajuste. Para diagnosticar uma interrupção sem mensagem (falta de memória), defina `UTSIM_FUZZ_TRACE=arquivo` e o último texto testado fica gravado.

Achados corrigidos (cada um virou caso fixo em `known_findings_are_errors`):

| Entrada | Efeito antes | Agora |
|---|---|---|
| fonte com `Exp(mean: 1e-300)` | execução sem fim | `validate` recusa: mais de 1e9 chegadas esperadas |
| `Rack(bays: 4294967296, levels: 4294967296)` | estouro aritmético ou alocação gigante | `validate` recusa: máximo de 1 milhão de posições |
| `Split(99999999999)` | bilhões de cópias de item | `validate` recusa: máximo de 1 milhão |
| `operators.count: 99999999999` | alocação gigante | `validate` recusa: máximo de 1 milhão |
| fluxo `Source` com `Exp(mean: 0.0)` sem `limit` | memória esgotada (aborto do processo) | `validate` recusa; os tempos dos fluxos agora também são validados (`Delay`, `Source`) |
| tubulação de fluido com `max_rate: 99999999999` | execução sem fim em um único instante | o núcleo interrompe com erro depois de 2 milhões de eventos no mesmo instante |

## Página web: motor em um Web Worker

A simulação, a validação, o ajuste e o replay rodam em um Web Worker (`web/worker.js`); a página (`web/app.js`) só desenha. Veja `docs/arquitetura.md` para o contexto geral.

- Protocolo: a página envia `{id, op, args}` (`init`, `examples`, `validate`, `run`, `fit`, `replay`) e recebe `{id, ok, result}` ou `{id, ok: false, error}`. Cada chamada vira uma promessa.
- Modo servido (`web/`): `new Worker("worker.js")`; o worker carrega `dist/utsim.js` e `dist/utsim_bg.wasm` ao lado.
- Modo autocontido (`dist/utsim.html`): `scripts/web.sh` grava o código de cola do wasm-bindgen e o `worker.js` em elementos `<script type="text/plain">`; a página monta um `Blob` com os dois, cria o worker a partir dele e passa os bytes do wasm (decodificados do base64) na mensagem `init`.
- Cancelar: o botão "Cancelar" aparece enquanto há uma tarefa. Ele encerra o worker (`terminate`), rejeita as chamadas pendentes e já inicia um worker novo. Não há progresso fino, então a barra de status mostra o tempo decorrido.
- `node web/smoke.mjs` exercita o protocolo do worker (com um `self` simulado) além do wasm direto.

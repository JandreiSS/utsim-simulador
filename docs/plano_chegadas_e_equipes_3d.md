# Plano: chegadas e controle de equipes no modelo e no 3D

Plano técnico, sem implementação, de dois pedidos do dono: (1) ver como os simuladores 3D de
eventos discretos maduros representam as **chegadas** (fontes, horários, sequências, tipos de
item, lotes) e como elas aparecem na cena; (2) ver como esses simuladores fazem o **controle de
equipes** (um despachante que recebe pedidos de tarefa e os distribui a operadores e
empilhadeiras) e como as equipes são ligadas e mostradas no 3D. Depois, comparar com o que o
UTSim já tem e propor melhorias concretas no modelo e no editor 3D, com prioridade, estimativa,
modelo de IA e testes. Os requisitos sugeridos começam em RF-259 e ficam só aqui até o dono
aprovar.

A pesquisa usou material público (documentação e vídeos de tutorial) de simuladores 3D de
mercado. Nada aqui copia texto, nomes de campos ou telas deles: o documento descreve a prática
comum de forma genérica.

Documentos relacionados, que este plano não repete:

- `docs/plano_analises_e_dinamica.md`: motorista na empilhadeira, desvio de obstáculos,
  reservas de trecho;
- `docs/plano_godot.md` e, quando chegar ao `main`, `docs/plano_interface_rust.md`: a
  interface em volta da vista 3D está migrando para Rust/egui;
- `docs/backlog_v02.md`: regras de despacho de produção, equipes de manutenção, calendário com
  feriados, tabelas globais importadas de planilha (itens que este plano detalha em parte);
- `docs/manual/src/objetos.md`, `recursos-moveis.md`, `pessoas.md`, `quebras-e-turnos.md`.

**Restrição de trabalho.** Outro agente mexe agora em `crates/utsim-godot` e `godot/`. As
tarefas deste plano que tocam nesses lugares ficam para depois dessa entrega e devem ser
combinadas com ela; as tarefas de núcleo (`utsim-core`, `utsim-objects`, `utsim-model`,
`utsim-scene`, `utsim-edit`) podem andar antes.

---

## 0. Ponto de partida

### 0.1 Chegadas no UTSim hoje

| Recurso | Onde | Situação |
|---|---|---|
| Tempo entre chegadas por distribuição (`interarrival`), primeira em zero, limite (`max_arrivals`) | `ObjectKind::Source`, `crates/utsim-objects/src/source.rs` | existe |
| Rótulos sorteados na criação (`labels: { nome: Dist }`) | `Source.labels` | existe; só números |
| Chegadas por período de um ciclo (`schedule`: `Default`, `Interarrival`, `Rate`, `Closed`, rótulos do período) | `ArrivalSchedule`, `PeriodArrivals` | existe, com validação e chaves de experimento |
| Item que não sai na criação: espera ou perde (`on_blocked: Wait / Drop`) | `OnBlocked` | existe |
| Chegadas em instantes fixos com quantidade (`Schedule { times: [(t, n)] }`) | `utsim-flow`, atividade `Schedule` | existe só nos fluxos, não na `Source` |
| Valores reais em qualquer campo de distribuição (`data: [..]`, ajuste automático) | `crates/utsim-model/src/dados.rs` | existe |
| Lote na fila (`Queue.batch`: tamanho, espera máxima, `flush`) | `crates/utsim-objects/src/queue.rs` | existe; não há lote na chegada |
| Roteamento por rótulo (`OutputRule::ByLabel`, `Weighted`, `Script`) | `crates/utsim-core/src/block.rs` | existe |
| Fonte no 3D: forma própria, porta de saída verde no lado `+x` quando selecionada ou sob o mouse, sem porta de entrada | `godot/scripts/editor.gd` (`NO_INPUT`, `port_out`) | existe |
| Cor por estado (`generating`, `blocked`, `off_shift`) | `crates/utsim-scene/src/style.rs` (`state_color`) | existe |

O que falta: **sequência de chegadas** (tabela ordenada de linhas com instante ou intervalo,
quantidade, tipo e rótulos), **lote na chegada** (várias peças por chegada), **tipos de item**
com nome, cor e modelo 3D, **rótulos de texto**, importar a tabela de chegadas de planilha,
**editor visual do horário** e, no 3D, **prévia dos itens** que a fonte cria e do que está
preso nela.

### 0.2 Equipes no UTSim hoje

| Recurso | Onde | Situação |
|---|---|---|
| Despachante com equipe (`Dispatcher { team, rule }`) e regras `FirstAvailable`, `RoundRobin`, `Nearest` | `crates/utsim-objects/src/dispatcher.rs` | existe |
| Fila de tarefas por prioridade e FIFO entre iguais; membro que fica livre pega a cabeça da fila; carga combinada até a capacidade | idem | existe |
| Prioridade na tarefa (`TaskRequest.priority`) | `crates/utsim-core/src/task.rs` | existe no núcleo, mas **todos os pedidos usam 0**: o modelo não expõe |
| Tarefas `Transport` e `Operate` (andar até a estação, ficar em `utilize` até `MSG_RELEASE`) | `TaskKind`, `transporter.rs` | existe |
| Operadores numa máquina (`OperatorUse { team, count, setup, process, requires }`) | `crates/utsim-objects/src/processor.rs` | existe |
| Habilidades (`person.skills`, `requires`) | `crates/utsim-core/src/person.rs` | existe |
| Volta para casa quando ocioso (`home: true`, volta à posição do próprio bloco; tarefa nova interrompe a volta) | `Transporter.home` | existe; a casa é a posição do bloco |
| Turnos e fim de turno (`Finish`) respeitados pelo despachante | `quebras-e-turnos.md` | existe |
| Técnico de manutenção como recurso | `crates/utsim-core/src/engine/maint.rs` | existe |
| Estados `travel_empty`, `travel_loaded`, `loading`, `unloading`, `utilize`, `resting` | `State` | existe, com cor |
| Pessoa andando e carregando (animação, mãos) | `godot/scripts/pessoa.gd`, `utsim-scene` | existe |
| Despachante no 3D: marcador pequeno (`Shape::Flag`), sem porta | `utsim-scene/src/style.rs`, `editor.gd` | existe; **sem linhas para a equipe nem para quem pede** |
| Ligações de fonte para transportador desenhadas como "lógicas" (cor mais apagada) | `editor.gd` (`LINK_LOGICAL`), `utsim-godot/src/document.rs` | existe para `transport` de fluxo |

O que falta: ver e editar no 3D quem é da equipe e quem pede a quem; prioridade de tarefa no
modelo; regras de escolha além das três; preempção; casa num ponto próprio e locais de pausa;
um membro em mais de uma equipe; sequências de tarefa compostas; e o estado da equipe ao vivo
(fila de tarefas, quem está fazendo o quê).

---

## 1. Prática comum nos simuladores 3D de mercado

### 1.1 Chegadas

- **A fonte é um objeto no piso**, com forma própria, nome e uma ou mais portas de saída. A
  ligação de saída é uma linha com seta até o próximo objeto. Muitas ferramentas mostram as
  portas como marcas coloridas nos lados do objeto e numeram as saídas.
- **Quatro modos de chegada** na mesma fonte, escolhidos numa lista:
  1. tempo entre chegadas por distribuição (o modo básico);
  2. **horário de chegadas**: tabela de linhas com instante (ou hora do dia), quantidade e
     rótulos; pode repetir em ciclo (diário, semanal);
  3. **sequência de chegadas**: lista ordenada de linhas com tipo, quantidade e rótulos, que a
     fonte percorre em ordem com um tempo entre linhas por distribuição; repete ou para no fim;
  4. **taxa por período**: a taxa muda ao longo do dia (curva de demanda), com o processo de
     Poisson não homogêneo por trás.
- **Lote na chegada**: cada chegada cria N itens de uma vez (N fixo ou por distribuição), e
  os itens saem juntos ou um a um.
- **Tipo do item na criação**: o tipo define cor, forma ou modelo 3D e rótulos padrão; os
  rótulos podem ser número ou texto. O tipo é a chave para roteamento e para os relatórios
  por tipo.
- **Gatilhos na criação**: ao criar, a fonte pode atribuir rótulos por expressão (prazo =
  agora + 8 h, sequência = contador) e chamar uma rotina do usuário.
- **Tabelas externas**: horário e sequência vêm de planilha, com importação e ligação ao
  arquivo, e editor de tabela dentro do programa (linhas, colunas, colar da planilha).
- **No 3D**: os itens criados aparecem saindo da porta; quando a saída bloqueia, eles se
  acumulam visíveis junto da fonte; um contador mostra quantos foram criados. A fonte fica
  com a cor do estado (gerando, bloqueada, fora do horário).

### 1.2 Controle de equipes

- **Despachante como objeto da cena** (gerente de equipe), em geral um ícone pequeno que pode
  ficar em qualquer lugar do piso. Ele não move itens: só recebe pedidos e distribui.
- **Ligações de centro**: além das portas de entrada e saída dos itens, cada objeto tem uma
  ligação "de centro" ou "de referência", desenhada com outra cor ou tracejada. Ligar o
  despachante a cada operador ou empilhadeira forma a equipe; ligar uma máquina ou fila ao
  despachante diz a quem ela pede. Na prática, a pessoa arrasta com uma tecla modificadora
  segurada, e as linhas ficam visíveis no editor e somem (ou apagam) na execução.
- **Despachante de despachantes**: um despachante pode ter outros despachantes como membros,
  o que forma hierarquias (equipe do turno A e do turno B atrás de um despachante geral).
- **Regras de escolha**: primeiro livre, rodízio, mais próximo, menos ocupado (menor
  utilização acumulada), há mais tempo ocioso, aleatório, por habilidade e por regra do
  usuário. Do lado da tarefa ("puxar"): o membro que fica livre escolhe na fila por
  prioridade, ordem de chegada, a mais próxima dele ou por regra.
- **Prioridade e preempção**: cada pedido tem prioridade e um modo de preempção: não
  interromper; interromper só se o membro estiver indo vazio ou voltando para casa;
  interromper qualquer tarefa e retomá-la depois (ou descartá-la e pedir de novo).
- **Sequência de tarefa**: o pedido não é um verbo só, é uma lista de passos que o membro
  executa: ir até, carregar, ir até, descarregar, trabalhar por um tempo, aguardar um sinal,
  pausar, voltar. Os pedidos padrão de "transportar" e "operar" são sequências prontas, e o
  usuário pode montar outras (por exemplo, buscar uma ferramenta antes de operar).
- **Recursos compartilhados**: o mesmo operador atende várias máquinas e até várias equipes;
  um pedido pode exigir mais de um membro ao mesmo tempo (dois operadores para um setup).
- **Casa e pausa**: cada membro tem uma posição de descanso (um ponto do piso ou um objeto,
  como uma sala de pausa). Quando ocioso, volta para lá ou fica onde parou. Pausas de turno
  levam o membro a um local próprio.
- **No 3D**: o operador anda pelos caminhos, carrega o item nas mãos ou nos garfos, fica
  parado na estação durante o trabalho; cores de estado no próprio recurso ou num pequeno
  indicador sobre ele; linhas da tarefa atual (do membro até o destino) opcionais;
  despachante mostra quantos pedidos estão na fila.

---

## 2. Comparação e lacunas

| Prática comum | UTSim hoje | Lacuna | Prioridade |
|---|---|---|---|
| Tempo entre chegadas por distribuição | sim | nenhuma | |
| Taxa por período, ciclo diário ou semanal | `schedule` com `Rate`, `Interarrival`, `Closed` | editor visual da grade | média |
| Horário com instante e quantidade | só no `utsim-flow` | levar para a `Source` | alta |
| Sequência ordenada (tipo, quantidade, rótulos) | não | novo | alta |
| Lote por chegada | não (só lote na fila) | novo | alta |
| Tipo de item com cor e modelo | não (rótulos numéricos; cor só por objeto) | novo | alta |
| Rótulo de texto e por expressão na criação | só número sorteado | novo, com cuidado no formato | média |
| Tabela importada de planilha | `data` só para distribuições | estender para tabelas de chegada | média |
| Itens visíveis acumulados na fonte, contador | não | 3D | média |
| Despachante como objeto | sim (marcador) | sem ligações visíveis | alta |
| Ligações de centro (equipe e solicitantes) | só nos campos `team`, `transport`, `operators.team` | desenhar e editar no 3D | alta |
| Prioridade de pedido | núcleo tem; modelo não expõe | expor por objeto e por rótulo | alta |
| Regras: menos ocupado, há mais tempo ocioso, aleatório | não | novo | média |
| Regra do lado da tarefa (mais próxima do membro livre) | não (cabeça da fila) | novo | média |
| Preempção | só interrompe a volta para casa | níveis de preempção | média |
| Sequência de tarefa composta | só `Transport` e `Operate` | passos novos limitados | baixa |
| Despachante de despachantes, membro em várias equipes | não ("um transportador só pode estar na equipe de um despachante") | novo | baixa |
| Casa num ponto próprio, local de pausa | casa = posição do bloco | novo | média |
| Estado da equipe ao vivo (fila, tarefa atual) | cor por estado do membro | indicador e linhas de tarefa | média |

---

## 3. Propostas para as chegadas

### C1. Horário e sequência de chegadas na `Source`

Um campo novo e opcional `table` na `Source`, com dois modos. Ausente, nada muda.

```ron
(name: "pedidos", kind: Source(
    interarrival: Exp(mean: 5.0),            // usado só no modo Sequence, entre linhas
    table: Some((
        mode: Times,                          // Times | Sequence
        repeat: Cycle(1440.0),                // Once | Cycle(duração) | Loop
        rows: [
            (at: 480.0,  count: 12, item_type: "A", labels: { "prioridade": 2.0 }),
            (at: 600.0,  count: 5,  item_type: "B"),
            (at: 840.0,  count: 8,  item_type: "A", labels: { "prazo": 1080.0 }),
        ],
    )),
))
```

- **`Times`** (horário): cada linha cria `count` itens no instante `at` do ciclo. Com
  `Cycle(d)`, repete a cada `d`; com `Once`, para no fim (estado `idle`). Equivale à atividade
  `Schedule` dos fluxos, que passa a usar o mesmo código.
- **`Sequence`** (sequência): as linhas não têm instante; a fonte percorre a lista em ordem e
  sorteia `interarrival` entre uma linha e a próxima. `Loop` volta ao início. Serve para
  ordens de produção, mix de pedidos e roteiros de caminhões.
- `count` aceita número fixo ou `Dist` (inteiro arredondado, mínimo 0).
- Convive com `schedule`: se ambos existem, é erro de validação com mensagem clara (um ou
  outro).
- Itens criados no mesmo instante saem na ordem da tabela; com saída bloqueada, valem as
  regras de `on_blocked`.
- Chaves de experimento: `pedidos.table.rows[n].count`, `.at`, `.labels.<nome>`.
- Validação: `at` em ordem não decrescente e dentro do ciclo; `count >= 0`; tipo existente
  (C3).

**Estimativa** 14–20 h (núcleo, validação, migração, testes, manual). **Modelo** Opus no
formato e na semântica; Sonnet nos testes e no manual.

### C2. Lote por chegada

Campo opcional `batch: Option<Dist>` na `Source` (também por linha da tabela pelo `count`):
cada chegada cria `batch` itens no mesmo instante. Com `release: Together | OneByOne(Dist)`,
os itens saem juntos (o próximo objeto decide se cabem) ou espaçados. A métrica
`entered.count` passa a contar itens, e uma nova `arrivals.count` conta as chegadas.

**Estimativa** 5–8 h. **Modelo** Sonnet.

### C3. Tipos de item

Uma lista opcional `item_types` no nível do modelo:

```ron
item_types: [
    (name: "A", color: "#3b82f6", asset: Some("caixa_pequena"), labels: { "peso": 8.0 }),
    (name: "B", color: "#f59e0b", asset: Some("palete"), labels: { "peso": 400.0 }),
],
```

- A `Source` ganha `item_type: Option<String>` (fixo) ou `item_type_mix: [(nome, peso)]`
  (sorteado), além das linhas da tabela (C1).
- O tipo é guardado no item como rótulo numérico reservado (`type`, índice a partir de 1), o
  que mantém `OutputRule::ByLabel("type")` e os scripts funcionando sem mudança no núcleo.
- Os relatórios ganham quebra por tipo: tempo no sistema e vazão na `Sink`, já que o rótulo
  viaja com o item.
- O 3D usa a cor e o modelo do tipo para o item (hoje o item usa a forma padrão).

**Estimativa** 10–14 h (formato, relatório por tipo, cena). **Modelo** Opus no formato;
Sonnet no relatório e na cena.

### C4. Rótulos de texto e por expressão

Duas extensões, separadas por risco:

- **Texto constante por linha ou tipo** (`"cliente": "Loja 12"`): o item guarda uma tabela de
  textos ao lado dos números. É útil para relatórios e para a vista 3D (etiqueta), não para
  cálculo. Risco pequeno.
- **Expressão na criação** (`"prazo": "now + 480"`, `"seq": "count"`): reaproveita o
  avaliador de expressões do `utsim-flow` (`Assign`). Risco médio: precisa do mesmo
  determinismo e de erros com o nome do campo.

**Estimativa** 8–12 h. **Modelo** Opus.

### C5. Tabela de chegadas de planilha

A lista `data` (hoje só para distribuições) passa a aceitar tabelas: `(table:
"pedidos.table", source: "dados/pedidos.csv", columns: { at: "hora", count: "qtd",
item_type: "tipo" })`. A tabela é lida ao carregar o modelo, guardada no `.ron` como cópia
(mesma lógica de hash e aviso de `dados.rs`) e pode voltar a ser lida. Horas do dia em
`hh:mm` são convertidas pela unidade de tempo do modelo. É o mesmo pedido da linha "tabelas
globais importadas de planilha" do backlog, limitado às chegadas.

**Estimativa** 8–12 h. **Modelo** Sonnet.

### C6. Editores (na interface Rust/egui), feito

Os editores de tabela e de horário são parte da interface em volta da vista 3D e devem ser
feitos já na interface Rust/egui, seguindo `docs/plano_interface_rust.md` quando ele chegar.

- **Tabela de chegadas**: grade editável (instante, quantidade, tipo, rótulos), colar da
  planilha, ordenar, validar linha a linha com a mensagem do núcleo, total de itens por ciclo
  e por tipo.
- **Horário por período** (`schedule`): faixa horizontal do ciclo com os períodos como
  blocos arrastáveis, cor por modo (padrão, taxa, distribuição, fechado), taxa em itens por
  hora e um gráfico de barras do número esperado de chegadas por período
  (`expected_in_period` já existe).
- **Prévia**: botão "ver as primeiras chegadas" que roda só a fonte (sem o resto do modelo)
  por um ciclo e mostra o histograma de chegadas por hora e por tipo.

**Estimativa** 16–24 h. **Modelo** Sonnet.

### C7. A fonte no 3D

Fica para depois da entrega do outro agente em `godot/` e `crates/utsim-godot`.

- **Portas sempre visíveis no modo Editar** para fontes e saídas (hoje só no selecionado ou
  sob o mouse), com o número da porta quando houver mais de uma saída; opção para esconder.
- **Prévia do item**: um item pequeno do tipo (cor e modelo de C3) sobre a fonte; com mistura
  de tipos, um anel com as fatias das proporções.
- **Itens acumulados**: quando a saída bloqueia com `on_blocked: Wait`, o item preso aparece
  na boca da porta; os perdidos (`Drop`) piscam e somem.
- **Selo do modo**: ícone pequeno (relógio para horário, lista para sequência, curva para taxa
  por período) e o contador de itens criados durante a execução.
- **Lote**: os itens de um lote saem juntos, lado a lado.

**Estimativa** 10–14 h. **Modelo** Sonnet.

---

## 4. Propostas para as equipes

### E1. Ligações de equipe e de pedido visíveis e editáveis no 3D

É a mudança que o dono mais vai sentir, porque deixa a equipe visível como nos simuladores de
mercado.

- **Desenho.** Dois tipos novos de linha no editor, além das ligações de itens:
  - **equipe** (despachante → membro): tracejada, cor própria (por exemplo lilás), do centro
    do despachante ao centro do membro;
  - **pedido** (solicitante → despachante ou transportador): pontilhada, outra cor, de cada
    objeto que tem `transport` ou `operators.team` apontando para a equipe.
  As linhas de equipe e pedido aparecem no modo Editar; na execução ficam apagadas, e só as
  do objeto selecionado se acendem. Uma opção na barra liga ou desliga cada tipo.
- **Edição pelo modo Ligar.** Com a ferramenta Ligar e uma tecla modificadora (proposta: `Alt`
  segurado, ou uma ferramenta própria "Equipe", tecla E), arrastar:
  - do despachante a um transportador: acrescenta o membro em `team` (de novo: remove);
  - de um objeto de fluxo ao despachante ou transportador: define `transport` daquele objeto;
  - de um processador ao despachante, com a tecla e um clique direito: define
    `operators.team` (abre a caixa de `count`, `setup`, `process` e `requires`).
  Sem a tecla, Ligar continua ligando portas de itens como hoje. Tudo passa pelo documento,
  com validação e desfazer.
- **Sem formato novo.** As linhas são derivadas dos campos que já existem (`team`,
  `transport`, `operators.team`), então os modelos atuais aparecem com as linhas sem
  migração. Do lado do núcleo, `utsim-edit` ganha as operações `team_add`, `team_remove`,
  `set_transport`, `set_operators`; `utsim-scene` exporta as linhas com o tipo.

**Estimativa** 14–20 h (operações em `utsim-edit` e `utsim-scene`: 5–7 h, podem ir antes;
editor 3D: 9–13 h, depois da entrega do outro agente). **Modelo** Opus nas operações e na
interação; Sonnet no desenho.

### E2. Prioridade de pedido no modelo

O núcleo já ordena a fila por prioridade; falta dizer qual é.

- `transport_priority: Option<i32>` em `ObjectDef` (prioridade dos transportes que saem dele);
- `priority: Option<i32>` em `OperatorUse`;
- `priority_label: Option<String>` nos dois: a prioridade vem de um rótulo do item (um pedido
  urgente criado com `"prioridade": 9` passa na frente).

Sem os campos, prioridade 0 como hoje: resultados idênticos. **Estimativa** 5–8 h. **Modelo**
Sonnet (especificação clara).

### E3. Regras de escolha do membro e da tarefa

Novas variantes de `DispatchRule`, todas determinísticas (empate para o primeiro da equipe):

| Regra | Escolhe | Dado que o despachante precisa |
|---|---|---|
| `LeastBusy` | o membro livre com menor tempo ocupado acumulado | tempo ocupado por membro, que ele soma pelos pedidos e avisos de livre |
| `LongestIdle` | o membro livre há mais tempo | instante do último aviso de livre |
| `Random` | um membro livre ao acaso | fluxo de números aleatórios próprio do despachante |

E um campo novo `pick: TaskPick` para quando um membro fica livre e há fila:
`Priority` (hoje: prioridade, depois ordem de chegada), `Nearest` (a tarefa mais próxima do
membro dentro da maior prioridade) e `NearestAny` (a mais próxima, ignorando prioridade, com
aviso de risco de inanição no manual).

**Estimativa** 8–12 h. **Modelo** Opus (regras e testes de igualdade com o comportamento
atual).

### E4. Preempção

Campo `preempt: Preempt` no despachante, aplicado quando chega um pedido de prioridade maior
que a de todos os membros ocupados e nenhum está livre:

- `Never` (padrão, como hoje);
- `Empty`: interrompe um membro que está indo vazio buscar uma tarefa de prioridade menor ou
  voltando para casa; a tarefa interrompida volta para a frente da fila;
- `Full`: também interrompe um `Operate` em andamento de prioridade menor; a estação volta a
  `waiting_operator`, o tempo restante é guardado e retomado quando outro membro chegar
  (mesma lógica de pausa que as quebras já usam).

Transporte com item carregado nunca é interrompido (o item não fica no chão). `Full` é o mais
arriscado: precisa de protocolo novo entre despachante, membro e estação. Proposta: entregar
`Empty` primeiro e `Full` depois, se o dono quiser.

**Estimativa** `Empty` 8–12 h, `Full` 14–20 h. **Modelo** Opus.

### E5. Casa, área de espera e local de pausa

- `home_at: Option<[f64; 3]>` ou `home_object: Option<String>` no `Transporter`: a casa deixa
  de ser só a posição do próprio bloco. Um objeto pode ser uma "área de espera" (um
  `Queue` sem fluxo ou um marcador novo `Area`, decisão do dono).
- `break_at` no calendário da pessoa: na janela de pausa, o membro termina a tarefa (ou para,
  conforme `on_end`), anda até o local e volta depois. Hoje ele fica onde está.
- No 3D, um marcador de casa (círculo no piso com o nome do membro) e a linha tracejada do
  membro até a casa no modo Editar.

**Estimativa** 8–12 h (núcleo e cena). **Modelo** Sonnet, com revisão Opus do protocolo de
pausa.

### E6. Hierarquia e membros compartilhados

- Despachante como membro de outro despachante: o de cima repassa o pedido ao de baixo pela
  regra; o de baixo responde como um membro livre quando algum dos seus fica livre.
- Um transportador em mais de uma equipe: o membro passa a ter uma lista de despachantes e
  avisa todos ao ficar livre; o primeiro pedido que chega o ocupa e os outros são avisados.
  Muda a regra "um transportador só pode estar na equipe de um despachante".

**Estimativa** 12–18 h. **Modelo** Opus. Prioridade baixa: só se um estudo de caso pedir.

### E7. Sequências de tarefa compostas

Um passo além de `Transport` e `Operate`: uma lista curta de passos que um objeto pede à
equipe, com os verbos `TravelTo(objeto)`, `Load`, `Unload`, `Work(Dist)`, `WaitSignal`,
`Return`. As tarefas padrão viram sequências prontas por dentro, sem mudar o resultado.
Casos de uso: buscar ferramenta antes do setup, inspeção itinerante, ronda de
abastecimento. Boa parte disso já se faz com `utsim-flow` (`Acquire`, `Delay`, `Release`);
por isso a prioridade é baixa e a primeira entrega pode ser só um exemplo de fluxo que faz
uma ronda.

**Estimativa** 20–30 h (completo) ou 3–4 h (exemplo com fluxo). **Modelo** Opus.

### E8. Equipe ao vivo no 3D e no painel

- Sobre o despachante: número de pedidos na fila e uma barra com membros livres e ocupados.
- Sobre cada membro: anel de cor do estado (as cores de `state_color`) e, ao selecionar, a
  linha até o destino da tarefa atual. A cena ao vivo (`utsim-scene/1`) ganha o campo
  `task_target` no estado do membro.
- Resultados por equipe: utilização média e por membro, tempo médio de espera do pedido
  (do pedido à chegada do membro), pedidos atendidos por membro (equilíbrio da carga),
  preempções. O despachante passa a registrar `request_wait` como tally.

**Estimativa** 10–14 h (métricas e cena: 4–6 h antes; 3D: 6–8 h depois). **Modelo** Sonnet.

---

## 5. Formato, compatibilidade e testes

### Formato

- Todos os campos novos são opcionais e omitidos ao gravar quando ausentes. Os modelos atuais
  carregam e rodam com o **mesmo resultado byte a byte**; os snapshots golden de
  `scripts/verificar` não mudam.
- `item_types` e `table` são as únicas mudanças que justificam subir o número do formato se o
  dono quiser marcar a versão; como são aditivas, a migração é vazia.
- Avisos: `table` e `schedule` juntos é erro; `priority_label` que nenhuma fonte cria vira
  aviso; equipe sem solicitante vira aviso ("despachante sem quem peça").

### Testes

| O quê | Como |
|---|---|
| Horário `Times` | contagem exata por ciclo e por tipo; instantes exatos no rastro; `Once` para no fim; repetição com `Cycle` |
| Sequência | ordem dos tipos igual à da tabela; `Loop`; média do intervalo entre linhas dentro do IC |
| Lote | itens por chegada; `arrivals.count` × `entered.count`; `OneByOne` com espaçamento |
| Tipos | rótulo `type` correto; `ByLabel("type")` roteia; relatório por tipo soma ao total |
| Igualdade com o atual | modelos de `examples/` com e sem os campos novos em valor padrão dão o mesmo relatório |
| Prioridade | dois solicitantes com prioridades diferentes: o de maior é sempre atendido antes quando ambos esperam |
| Regras novas | casos pequenos com resposta calculada à mão; `Random` reprodutível com a mesma semente |
| Preempção `Empty` | membro indo vazio é desviado; a tarefa interrompida é feita depois; nenhum item se perde (conservação) |
| Casa e pausa | posição final do membro ocioso; tempo de deslocamento para a pausa no rastro |
| Editor | operações de `utsim-edit` com desfazer e refazer; teste de documento no `utsim-godot` (depois) |
| Visual | captura comparada da cena de exemplo com linhas de equipe (quando houver os testes de regressão visual do backlog) |

Exemplos novos em `examples/`: `chegadas_por_tabela.ron` (pedidos por hora e tipos),
`equipe_com_prioridade.ron` (duas máquinas, uma urgente, regra `LeastBusy`, preempção
`Empty`) e um tutorial curto no manual.

---

## 6. Prioridades e estimativas

| # | Tarefa | Horas | Modelo | Depende de | Pode começar |
|---|---|---|---|---|---|
| 1 | E2 Prioridade de pedido no modelo | 5–8 | Sonnet | nada | já |
| 2 | C1 Horário e sequência na `Source` | 14–20 | Opus | nada | já |
| 3 | C3 Tipos de item (formato, relatório, cena) | 10–14 | Opus + Sonnet | nada | já |
| 4 | C2 Lote por chegada | 5–8 | Sonnet | C1 (mesmo arquivo) | depois de 2 |
| 5 | E1a Operações de equipe em `utsim-edit` e linhas em `utsim-scene` | 5–7 | Opus | nada | já |
| 6 | E3 Regras `LeastBusy`, `LongestIdle`, `Random` e `pick` | 8–12 | Opus | E2 | depois de 1 |
| 7 | E8a Métricas de equipe e `task_target` na cena | 4–6 | Sonnet | nada | já |
| 8 | E1b Ligações de equipe e pedido no editor 3D | 9–13 | Opus | 5, entrega do outro agente | **feito** (interface em Rust, passo 5b) |
| 9 | C7 Fonte no 3D (portas, prévia, acumulados, selo) | 10–14 | Sonnet | 3, entrega do outro agente | **feito em parte** (interface em Rust, passo 5b: portas sempre visíveis, prévia na cor dos itens, selo do modo e contador de criados; faltam o anel das proporções de tipos, os itens presos e piscando e o lote lado a lado) |
| 10 | C6 Editores de tabela e horário (egui) | 16–24 | Sonnet | 2, plano da interface Rust | feito (passo 5b da interface Rust) |
| 11 | E5 Casa, área de espera e pausa | 8–12 | Sonnet + Opus | nada | já |
| 12 | E4 Preempção `Empty` | 8–12 | Opus | E2 | depois de 1 |
| 13 | C5 Tabela de chegadas de planilha | 8–12 | Sonnet | 2 | depois de 2 |
| 14 | E8b Equipe ao vivo no 3D | 6–8 | Sonnet | 7, 8 | **feito** (interface em Rust, passo 5b) |
| 15 | C4 Rótulos de texto e por expressão | 8–12 | Opus | 2 | depois de 2 |
| 16 | E4 Preempção `Full` | 14–20 | Opus | 12 | se o dono quiser |
| 17 | E6 Hierarquia e membros compartilhados | 12–18 | Opus | 6 | se um caso pedir |
| 18 | E7 Sequências de tarefa compostas | 20–30 (ou 3–4 com fluxo) | Opus | 12 | se um caso pedir |
| 19 | Manual, exemplos e tutorial | 6–8 | Sonnet | cada entrega | junto |
| 20 | Conferir links e listas do manual | 1 | Haiku | 19 | junto |

**Primeira leva sugerida** (núcleo, sem tocar em `godot/` nem `crates/utsim-godot`): 1, 2,
3, 5, 7 e 11 em worktrees separadas (arquivos diferentes: `dispatcher.rs` e `processor.rs`
em 1; `source.rs` em 2; `lib.rs` do modelo e `utsim-scene` em 3, combinar com 2 pela ordem;
`utsim-edit` em 5). Total 46–67 h. **Segunda leva** (3D e editores): 8, 9, 10, 14, depois que
a interface Rust e o outro agente entregarem. Total do plano sem os opcionais (16, 17, 18):
131–191 h.

---

## 7. Requisitos sugeridos

| Id | Requisito |
|---|---|
| RF-259 | A `Source` aceita uma tabela de chegadas em modo horário (instante, quantidade, tipo, rótulos) ou sequência (ordem, quantidade, tipo, rótulos), com repetição única, cíclica ou em laço |
| RF-260 | A `Source` cria um lote de itens por chegada, com tamanho fixo ou sorteado |
| RF-261 | O modelo declara tipos de item com nome, cor, modelo 3D e rótulos padrão; os relatórios mostram tempo no sistema e vazão por tipo |
| RF-262 | A tabela de chegadas pode ser importada de CSV, com hash e aviso de mudança como nos dados reais |
| RF-263 | Os pedidos de transporte e de operador têm prioridade definida no objeto ou num rótulo do item |
| RF-264 | O despachante oferece as regras menos ocupado, há mais tempo ocioso e aleatório, e a escolha da tarefa pela mais próxima do membro livre |
| RF-265 | O despachante pode interromper um membro que vai vazio ou volta para casa para atender um pedido de prioridade maior |
| RF-266 | Cada membro tem uma casa num ponto ou objeto próprio e um local de pausa |
| RF-267 | O editor 3D desenha as ligações de equipe e de pedido e permite criá-las e removê-las arrastando com a ferramenta Ligar |
| RF-268 | Os resultados mostram por equipe a utilização por membro, a espera média dos pedidos e o número de pedidos por membro |

---

## 8. Decisões do dono

**Decididas em 2026-11-03:** o dono aceitou todas as propostas de D1 a D10.

1. **D1 Tabela na `Source` ou objeto novo.** Proposta: campo `table` na própria `Source`
   (um objeto só, como na prática comum). Alternativa: um objeto `ArrivalTable` separado.
2. **D2 Tipos de item no modelo.** Proposta: `item_types` no nível do modelo, com o tipo
   guardado como rótulo numérico reservado `type`. Confirmar o nome do rótulo.
3. **D3 Rótulos de texto e expressão.** Fazer agora (C4) ou esperar um caso de uso.
4. **D4 Tecla da ligação de equipe.** `Alt` segurado com a ferramenta Ligar, ou ferramenta
   própria "Equipe" (tecla E). A proposta é a ferramenta própria, por ser mais fácil de
   descobrir, com `Alt` como atalho.
5. **D5 Linhas na execução.** Apagadas com destaque só no selecionado (proposta), sempre
   visíveis ou escondidas.
6. **D6 Preempção.** Entregar só `Empty` agora (proposta) ou também `Full`.
7. **D7 Membro em várias equipes e hierarquia (E6).** Fazer só quando um estudo de caso pedir
   (proposta) ou já.
8. **D8 Área de espera.** Usar um objeto existente sem fluxo ou criar um marcador `Area`
   novo (proposta, porque também serve para salas de pausa e zonas de escritório do plano das
   linhas especializadas).
9. **D9 Sequências de tarefa (E7).** Só exemplo com fluxo agora (proposta) ou o recurso
   completo.
10. **D10 Ordem com a interface Rust.** Confirmar que os editores de tabela e de horário (C6)
    são feitos direto na interface egui, sem passar pelo Godot.

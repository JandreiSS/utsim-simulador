# Primeiros passos

## Teste rápido na linha de comando

Antes de abrir qualquer janela, confirme que o motor funciona:

```bash
utsim run examples/mm1.ron
```

O resultado é uma tabela com uma linha por objeto. Veja o capítulo [Resultados e estatística](resultados.md) para aprender a lê-la.

## Abrir o visualizador

O visualizador chama-se `utsim-view`. Ele tem dois modos: **Editar** e **Simular**.

```bash
# abre um modelo existente (o arquivo é criado ao salvar, se não existir)
utsim-view examples/montagem_paletes.ron

# começa do zero
utsim-view --novo

# abre uma rodada gravada, somente para assistir
utsim replay examples/mm1.ron --out replay.json
utsim-view replay.json
```

Se você compilou com o cargo, troque `utsim-view` por `cargo run --release -p utsim-viewer --`.

Um modelo sem posições (tudo em 0, 0, 0) ganha um layout automático ao abrir. O layout vira um passo que dá para desfazer e vai para o arquivo quando você salva.

## A barra superior

| Controle | O que faz |
|---|---|
| **Editar** | volta ao modo de edição |
| **▶ Simular** | roda o motor e passa para o modo Simular |
| **até t =** | até quando simular, no modo Editar. Padrão: o menor entre `run.stop_time` e 2000 |
| **Novo**, **Salvar**, **Salvar como...**, **Desfazer**, **Refazer** | arquivo e histórico (`Ctrl+N`, `Ctrl+S`, `Ctrl+Shift+S`, `Ctrl+Z`, `Ctrl+Y`) |
| **Enquadrar (F)** | ajusta a câmera para mostrar o modelo inteiro |
| **Fluxos** | abre o painel da lógica do usuário ([Fluxos de tokens](fluxos-e-scripts.md)) |
| **Visual: simples / realista** | troca as formas simples pelos modelos 3D da biblioteca ([3D realista](visual-3d.md)) |

### A barra superior do UTSim 3D (prévia)

Na nova interface 3D (`utsim-3d`), a barra superior é organizada em grupos, da esquerda para a direita:

| Grupo | Controles |
|---|---|
| Arquivo | **Novo** (`Ctrl+N`), **Abrir…** (`Ctrl+O`, modelos `.ron` e replays `.json`), **Salvar** (`Ctrl+S`), **Salvar como…** (`Ctrl+Shift+S`) |
| Editar | **Desfazer** (`Ctrl+Z`), **Refazer** (`Ctrl+Y`) e o menu **Organizar** (alinhar, distribuir, copiar, recortar, colar, duplicar, excluir, selecionar tudo) |
| Modos | **Editar \| Simular** (`Tab` alterna) e, no Editar, as ferramentas **Selecionar \| Ligar \| Rede \| Corredor** (`Esc`, `L`, `N`, `C`) |
| Vista | **Grade** (encaixe, `G`) com o passo ao lado e **Atalhos** (`F1`) |
| Análise | **Resultados**, **PERT/CPM** (da seleção; com dois objetos, de um até o outro) e, na seta ao lado, as variantes do PERT/CPM e os **Fluxos de tokens** |

O nome do modelo fica à direita (com `*` quando há mudanças não salvas; o arquivo aparece ao parar o mouse sobre ele). No Simular, os grupos que só servem para editar saem da barra. Numa janela estreita, os grupos que não cabem vão para o menu **Mais**, no fim da barra, em vez de serem cortados. A roda do mouse sobre a barra (e sobre qualquer painel) nunca aproxima a câmera, e os atalhos de teclado valem sempre, exceto enquanto você digita num campo.

### Os painéis do UTSim 3D (prévia)

O painel da esquerda muda com o modo.

- **No Editar**, em cima fica a **Biblioteca**, com os objetos por categoria e o ícone de cada um. Clique numa entrada (ela fica marcada) e depois clique no chão para colocar; ou arraste a entrada até o chão e solte onde o objeto deve ficar. Soltar sobre um painel cancela a colocação. Embaixo fica a lista **Objetos**, com a caixa de busca (por nome ou tipo; o `×` limpa a busca). Um clique seleciona e um duplo clique enquadra o objeto. Objetos com problema aparecem em vermelho e com aviso em amarelo (o número aparece ao parar o mouse sobre eles). No pé do painel fica a caixa **Mostrar obstáculos e folgas**.
- **No Simular**, em cima ficam as **Replicações** (digite o número e tecle Enter, ou use `−` e `+`) e o botão **Simular de novo**. Logo abaixo vêm a linha de situação e a lista de **Blocos**: um clique foca o bloco na órbita e é o bloco que as câmeras Seguir, Primeira pessoa e Do recurso acompanham. Depois vêm o menu **Câmera**, os botões **Fluxos** e **Gravar replay** e as caixas **Rótulos** e **Sombras**. No pé fica o resumo da rodada, com o gargalo em laranja e as saídas de cada bloco. A tabela completa fica em **Resultados**.

A **barra de status**, no pé da janela, mostra o que acabou de acontecer. Quando o texto não cabe, ele termina em "…" e aparece inteiro ao parar o mouse sobre ele. Enquanto uma rodada ou análise corre, a barra mostra o progresso e o botão **Cancelar**. No Simular, a **barra de reprodução** fica logo acima dela, sobre a vista: **◀ evento**, **Reproduzir** (ou **Pausar**, também com `Espaço`), **evento ▶**, a velocidade, a linha do tempo (clique ou arraste para ir a um instante) e o tempo atual sobre o fim.

No painel de propriedades, um rótulo que não cabe na coluna quebra em duas linhas, e a coluna de rótulos cresce com o painel. Um rótulo nunca é cortado sem que o texto inteiro apareça ao parar o mouse sobre ele.

### Ligar pelos nós (UTSim 3D)

No Editar, cada objeto com fluxo tem dois **nós** pequenos nos lados: o de **saída** (verde, à direita do objeto) e o de **entrada** (azul, à esquerda). A chegada só tem o nó de saída e a saída só o de entrada; transportadores e despachantes não têm nós, porque se ligam pela ferramenta **Equipe**. O nó sob o mouse cresce e ganha uma borda branca.

| Ação | Como |
|---|---|
| Ligar | arraste do nó de saída de um objeto até o outro objeto ou até o nó de entrada dele. Uma seta acompanha o mouse enquanto você arrasta |
| Ligar ao contrário | arraste do nó de entrada até o objeto de origem ou até o nó de saída dele |
| Selecionar uma ligação | clique na seta, ou clique num nó: ele seleciona a ligação que sai (ou chega) por ali. Com várias, cada clique passa à seguinte |
| Remover a ligação selecionada | clique no **×** vermelho no meio da seta, ou tecle `Delete` |
| Desfazer e refazer | `Ctrl+Z` e `Ctrl+Y`, um passo por ligação |

**Selecionar** é a ferramenta padrão: ela está ativa ao abrir um modelo, depois de colocar um objeto da Biblioteca e depois de uma ligação, e `Esc` em qualquer outra ferramenta volta a ela, sem perder a seleção. Com ela, um clique seleciona (um objeto pequeno na tela também é pego clicando perto do centro dele), arrastar um objeto move a seleção, `Shift` + clique soma à seleção, e arrastar no chão vazio desenha uma caixa que seleciona o que estiver dentro. Transportadores e despachantes têm um terceiro nó, **laranja**, que liga quem opera cada máquina; veja [Como ligar um operador a uma máquina](recursos-moveis.md#como-ligar-um-operador-a-uma-máquina-e-a-empilhadeira-ao-que-ela-transporta). Na Biblioteca, a fila se chama **Espera**, e FIFO, LIFO ou prioridade se escolhem no painel, em **Política de acesso** (veja [Objetos](objetos.md#queue-fila)).

O objeto selecionado é marcado só nos quatro cantos (em vermelho, os objetos com problema), sem moldura em volta, para não cobrir os nós. Numa ligação curta, o × fica ao lado da seta. A ferramenta **Ligar** (`L`) continua valendo: arraste de um objeto ao outro, ou clique na origem e depois no destino. O arquivo `.ron` grava as ligações como sempre, na lista `connections`.

A simulação nunca é conduzida pela janela. O botão **▶ Simular** roda o motor em outra tarefa e a janela apenas lê o resultado gravado. Por isso o que você vê é exatamente o que o `utsim run` calcula.

## Modo Editar

No modo Editar você monta e altera o modelo.

| Ação | Como |
|---|---|
| Adicionar um objeto | escolha o tipo na **Paleta** (à esquerda) e clique no chão. A grade é de 1 unidade |
| Selecionar e mover | clique no objeto; arraste-o no chão. Um arrasto vale um passo de desfazer |
| Girar | `R` gira a seleção 90° (anti-horário visto de cima), `Shift+R` −90°, `Ctrl+R` 15° e `Ctrl+Shift+R` −15°; ou arraste a alça de giro (veja abaixo) |
| Ligar dois objetos | com um objeto selecionado, tecle `C` ou clique em **Ligar** e clique no destino |
| Remover uma ligação | clique na seta, ou no `x` da lista de ligações do painel |
| Escolher entre objetos sobrepostos | clique de novo no mesmo ponto: cada clique passa ao objeto seguinte da pilha, e uma dica ao lado do cursor lista os objetos e marca o selecionado |
| Selecionar pela lista | aba **Objetos** do painel da esquerda (veja abaixo) |
| Excluir um objeto | tecla `Delete` |
| Desfazer e refazer | `Ctrl+Z` e `Ctrl+Y` |
| Salvar | `Ctrl+S` (pede um nome se não houver arquivo) |
| Salvar com outro nome | `Ctrl+Shift+S` ou o botão **Salvar como...**: digite o caminho do arquivo (relativo à pasta do programa ou completo; sem extensão vira `.ron`). Se o arquivo já existe, o diálogo avisa que será sobrescrito |
| Cancelar a ferramenta | `Esc` |

A paleta oferece: Chegada, Fila, Processo, Montagem, Desmontagem, Saída, Esteira, Porta-paletes, Transportador e Despachante. Os nomes em inglês do arquivo são Source, Queue, Processor, Combiner, Separator, Sink, Conveyor, Rack, Transporter e Dispatcher.

O painel **Propriedades** (à direita) mostra os campos do objeto selecionado, a lista de ligações e os **Problemas** do modelo. Ao trocar de objeto o painel volta ao topo, onde ficam o nome e a **Posição**: os campos **X**, **Y** e **Z** (em metros) e a **Rotação** (em graus, nos objetos fixos) aceitam números digitados e cada alteração é um passo de desfazer. As listas suspensas mostram todas as opções de uma vez; as muito longas (equipes, transportes, nós da rede) têm uma caixa de busca, e as famílias de distribuição vêm em grupos. Os campos de distribuição abrem no formulário (família e parâmetros, cada um com o seu rótulo), em todos os objetos. O botão `{ }` de um campo troca só aquele campo para o código (texto RON) e de volta, até outro objeto ser escolhido. Em **⚙ Configurações** (na ponta direita da barra de baixo), "Campos de distribuição abrem em" escolhe **Formulário** ou **Código** para todos os campos; a escolha fica gravada neste computador (`%APPDATA%\UTSim\utsim3d.json`), não no modelo. Na mesma janela, **Idioma** escolhe a língua do UTSim 3D (português, inglês, espanhol, francês ou italiano), também gravada ali; sem escolha, vale a do sistema. Por ora só a janela Configurações e os painéis ao vivo mudam de língua; o resto das telas segue em português. Em "editar como texto" (distribuições), um texto que não é válido continua na caixa, em vermelho, com o motivo logo abaixo; o modelo só muda quando o texto passa a valer, e "descartar texto" volta ao valor do modelo. Sem nenhum objeto selecionado, ele mostra as configurações da rodada (tempo de parada, aquecimento, semente e replicações).

### Girar objetos

Para organizar a fábrica, cada objeto fixo (chegada, espera, processo, montagem, desmontagem, saída, esteira, porta-paletes) gira em volta do próprio centro. `R` gira a seleção 90° no sentido anti-horário visto de cima e `Shift+R` gira 90° no sentido horário; `Ctrl+R` e `Ctrl+Shift+R` fazem o mesmo em passos de 15°. Com vários objetos selecionados, cada um gira em volta do próprio centro, e o giro de todos é um passo só de desfazer. Com um único objeto fixo selecionado aparece a **alça de giro**, um botão roxo com uma seta curva atrás dele: arraste-a em volta do objeto e ele acompanha em passos de 15° (com `Shift`, 90°); soltar fecha o giro num passo de desfazer. O valor exato vai na **Rotação** do painel, logo abaixo de X, Y e Z, e aceita qualquer número de graus.

Tudo gira junto: o modelo 3D, os nós de entrada e saída (que continuam nos lados do próprio objeto), as setas das ligações, os itens parados sobre ele, o lugar onde o operador trabalha e a área que os recursos móveis contornam. Ele fica salvo no arquivo como `visual.yaw` (em graus); um objeto sem giro não grava o campo. A esteira sem `visual.yaw` aponta para o objeto que ela alimenta; a Rotação mostra essa direção, e girá-la grava o valor (também 0°). **Restaurar padrão**, na aparência, não desfaz a rotação. Os recursos móveis (transportadores e despachantes) não giram assim, porque viram para onde andam; neles o campo **Giro do desenho**, na aparência, só corrige para onde o modelo 3D olha.

Os problemas aparecem enquanto você monta o modelo, e os objetos com problema ficam com contorno vermelho. O modelo só roda quando não sobra nenhum. As mensagens são as mesmas do comando `utsim check`. Abaixo deles, em amarelo, ficam os **Avisos**: coisas que não impedem a simulação mas provavelmente não são o que você quis (uma ligação longa e instantânea, um objeto longe da rede de caminhos, um transportador que não atende ninguém).

### A aba Objetos e a ordem das ligações

Na camada Objetos, o painel da esquerda tem duas abas: **Biblioteca** (o que colocar) e **Objetos** (o que já existe). A aba Objetos lista todos os objetos do modelo, com uma caixa de busca por nome e um filtro por tipo. Clique seleciona, `Shift` + clique soma à seleção e **duplo clique enquadra o objeto** na vista. Objetos com problema aparecem em vermelho. É o caminho para selecionar objetos que estão no mesmo ponto ou escondidos atrás de outros.

A lista **Ligações** do painel de um objeto mostra as saídas na ordem das portas (porta 1, porta 2, ...) e as entradas na ordem de prioridade (a primeira tem preferência quando o objeto puxa um item). Os botões **Subir** e **Descer** trocam uma ligação com a vizinha da mesma lista, e `Ctrl+Z` desfaz. A ordem é a mesma que as regras de saída (pesos, por rótulo, rodízio) usam; com a regra por pesos, o painel mostra o destino ao lado de cada peso. Os pesos acompanham o número da porta, não o destino: ao trocar duas portas, confira os pesos.

### Camadas: Objetos, Rede e Fluidos

No alto do painel da esquerda há três botões. **Objetos** é a biblioteca descrita acima. **Rede** edita a rede de caminhos dos recursos móveis e **Fluidos** edita a rede de fluidos. Cada camada tem as suas ferramentas, a sua seleção e o seu painel de propriedades; `Del`, `Esc`, `Ctrl+Z` e `Ctrl+S` valem em todas.

| Camada Rede | Como |
|---|---|
| Colocar um nó | ferramenta **Nó** e clique no chão (encaixa na grade) |
| Desenhar arestas | ferramenta **Aresta**: clique num nó e depois em outro. Clicar no chão cria um nó já ligado, então dá para desenhar um caminho inteiro de uma vez. `Esc` termina |
| Mão única ou dupla | a caixa "Arestas novas de mão dupla"; numa aresta selecionada, o campo **Sentido** |
| Limitar AGVs | numa aresta selecionada, "Limitar veículos (AGV)" e a capacidade |
| Mover, renomear, excluir | arraste o nó; o painel tem nome e posição; `Del` exclui o nó com as suas arestas, ou a aresta selecionada |

O painel mostra o comprimento da aresta (calculado pelas posições dos nós). Uma linha tracejada liga cada objeto atendido por um recurso móvel ao nó por onde ele entra na rede, o mais próximo.

| Camada Fluidos | Como |
|---|---|
| Colocar um nó | escolha Fonte, Tanque, Tubulação, Sumidouro, Itens em fluido ou Fluido em itens e clique no chão |
| Ligar | ferramenta **Ligar**: clique no nó de origem e depois no de destino (a vazão segue a seta) |
| Parâmetros | vazões, capacidade, nível inicial, entrada e saída máximas e, nos conversores, o objeto discreto e o volume por item |

Os tanques aparecem no nível inicial. A validação é imediata: um ciclo, dois conversores "Fluido em itens" no mesmo objeto ou um objeto inexistente viram problemas.

## Modo Simular

Depois de **▶ Simular**, o visualizador toca a rodada gravada.

| Ação | Como |
|---|---|
| Reproduzir e pausar | `Espaço` |
| Velocidade | `+` e `-` |
| Evento anterior e seguinte | `←` e `→` |
| Estatísticas de um objeto | clique nele |
| Enquadrar | `Home` (no Editar, também `F`) |
| Andar pela fábrica em primeira pessoa | `F` no Simular (`Esc` sai) |

Itens, operadores e veículos aparecem em movimento. As estatísticas finais da replicação (janela depois do aquecimento) ficam no painel de cada objeto.

## A câmera

| Ação | Como |
|---|---|
| Deslocar a vista | botão esquerdo arrastando no vazio |
| Girar | botão direito arrastando |
| Aproximar e afastar | roda do mouse, em direção ao ponto do chão sob o cursor (na órbita) |
| Enquadrar tudo | tecla `F` ou botão **Enquadrar** |
| Trocar o modo da câmera | menu **Câmera** da barra, ou as teclas `1` (órbita), `2` (seguir), `3` (primeira pessoa) e `4` (do recurso) |

Os outros modos servem para ver a planta de dentro: **Seguir** acompanha um empilhador ou AGV (selecione-o e use **Seguir este recurso**), **Primeira pessoa** deixa você andar pela planta a 1,6 m do chão com `W`, `A`, `S`, `D` (`Shift` corre, botão direito olha em volta) e **Do recurso** põe a câmera no assento do motorista. `Esc` volta à órbita. Detalhes em [3D realista e aparência](visual-3d.md). `F3` mostra o tempo de quadro.

## Posições e unidades

A posição de um objeto no arquivo é `(x, y, z)`, com `z` na vertical. No 3D, `x` vai para a direita e `y` vai para o fundo da cena. As posições são em metros quando você usa velocidades em metros por unidade de tempo.

As posições do layout importam para a simulação quando há recursos móveis: as distâncias percorridas por operadores e empilhadeiras vêm delas. Veja [Recursos móveis](recursos-moveis.md).

## Verificar sem abrir janela

```bash
utsim-view examples/mm1.ron --headless-check
```

O comando carrega o modelo, valida, simula e imprime um resumo.

## E agora?

Faça o [tutorial 01](tutoriais/01-um-caixa.md): uma fila e um caixa. É o menor modelo útil e já mostra tudo o que você precisa para ler resultados.
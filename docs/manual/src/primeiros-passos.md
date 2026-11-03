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

A simulação nunca é conduzida pela janela. O botão **▶ Simular** roda o motor em outra tarefa e a janela apenas lê o resultado gravado. Por isso o que você vê é exatamente o que o `utsim run` calcula.

## Modo Editar

No modo Editar você monta e altera o modelo.

| Ação | Como |
|---|---|
| Adicionar um objeto | escolha o tipo na **Paleta** (à esquerda) e clique no chão. A grade é de 1 unidade |
| Selecionar e mover | clique no objeto; arraste-o no chão. Um arrasto vale um passo de desfazer |
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

O painel **Propriedades** (à direita) mostra os campos do objeto selecionado, a lista de ligações e os **Problemas** do modelo. Ao trocar de objeto o painel volta ao topo, onde ficam o nome e a **Posição**: os campos **X**, **Y** e **Z** (em metros) e o **Giro** (em graus, só na aparência) aceitam números digitados e cada alteração é um passo de desfazer. As listas suspensas mostram todas as opções de uma vez; as muito longas (equipes, transportes, nós da rede) têm uma caixa de busca, e as famílias de distribuição vêm em grupos. Em "editar como texto" (distribuições), um texto que não é válido continua na caixa, em vermelho, com o motivo logo abaixo; o modelo só muda quando o texto passa a valer, e "descartar texto" volta ao valor do modelo. Sem nenhum objeto selecionado, ele mostra as configurações da rodada (tempo de parada, aquecimento, semente e replicações).

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
| Enquadrar | `F` |

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
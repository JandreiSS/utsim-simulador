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
| **Novo**, **Salvar**, **Desfazer**, **Refazer** | arquivo e histórico (`Ctrl+N`, `Ctrl+S`, `Ctrl+Z`, `Ctrl+Y`) |
| **Enquadrar (F)** | ajusta a câmera para mostrar o modelo inteiro |
| **Fluxos** | abre o painel da lógica do usuário ([Fluxos de tokens](fluxos-e-scripts.md)) |
| **Visual: simples / realista** | troca as formas simples pelos modelos 3D da biblioteca ([3D realista](visual-3d.md)) |

A simulação nunca é conduzida pela janela. O botão **▶ Simular** roda o motor em outra tarefa e a janela apenas lê o resultado gravado. Por isso o que você vê é exatamente o que o `utsim run` calcula.

## Modo Editar

No modo Editar você monta e altera o modelo.

| Ação | Como |
|---|---|
| Adicionar um objeto | escolha o tipo na **Paleta** (à esquerda) e clique no chão. A grade é de 1 unidade |
| Selecionar e mover | clique no objeto; arraste-o no chão. Um arrasto vale um passo de desfazer |
| Ligar dois objetos | com um objeto selecionado, tecle `C` ou clique em **Ligar** e clique no destino |
| Remover uma ligação | clique na seta, ou no `✕` da lista de ligações do painel |
| Excluir um objeto | tecla `Delete` |
| Desfazer e refazer | `Ctrl+Z` e `Ctrl+Y` |
| Salvar | `Ctrl+S` (pede um nome se não houver arquivo) |
| Cancelar a ferramenta | `Esc` |

A paleta oferece: Chegada, Fila, Processo, Montagem, Desmontagem, Saída, Esteira, Porta-paletes, Transportador e Despachante. Os nomes em inglês do arquivo são Source, Queue, Processor, Combiner, Separator, Sink, Conveyor, Rack, Transporter e Dispatcher.

O painel **Propriedades** (à direita) mostra os campos do objeto selecionado, a lista de ligações e os **Problemas** do modelo. Sem nenhum objeto selecionado, ele mostra as configurações da rodada (tempo de parada, aquecimento, semente e replicações).

Os problemas aparecem enquanto você monta o modelo, e os objetos com problema ficam com contorno vermelho. O modelo só roda quando não sobra nenhum. As mensagens são as mesmas do comando `utsim check`.

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
| Aproximar e afastar | roda do mouse |
| Enquadrar tudo | tecla `F` ou botão **Enquadrar** |

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
# Relatório do piloto de usabilidade do UTSim

Papel: engenheiro de produção que nunca usou o UTSim e conhece o programa só pelo manual. Operação do
editor 3D exclusivamente por roteiros (`utsim-view --roteiro`), olhando as capturas como um usuário
olha a tela. CLI usada onde o manual a recomenda (`utsim fit`, `utsim run`, `utsim experiment`,
`utsim otimizar`). Nenhum arquivo de modelo foi escrito ou editado à mão: os três modelos foram
montados e salvos pelo editor. Arquivos de experimento e de otimização (que não são modelos) foram
salvos pelo editor (caso 1) ou escritos à mão para a CLI (casos 2 e 3), como o manual descreve.

Entregas:

- `trabalho/01-linha-de-montagem/respostas.md`: caso 1 completo (3 perguntas, validação, IC).
- `trabalho/02-centro-de-distribuicao/respostas.md`: caso 2 completo (3 perguntas).
- `trabalho/03-clinica-com-turnos/respostas.md`: caso 3 completo (4 perguntas), com aproximações.
- Capturas em `trabalho/0X-*/cap/rNN/`, roteiros em `trabalho/0X-*/roteiros/`.

Cada achado tem uma etiqueta: **[modelo/simulação]** vale para qualquer interface (motor, formato,
CLI, manual, relatórios); **[interface atual]** é específico do editor 3D atual (egui/Bevy) e pode
sumir ou mudar quando a interface for refeita em outro motor.

## 1. Resumo

### O que funcionou bem

- O motor é rápido: 20 replicações de 11 dias do caso 1 em 0,25 s; 576 pontos x 10 replicações do
  caso 3 em 7 s. Isso deixa a busca em grade exaustiva viável e muda o jeito de trabalhar.
- O `utsim fit` é excelente: diagnóstico i.i.d., 20 famílias, regra de escolha explicada, linha
  pronta e HTML com gráficos. A explicação "por que" de cada candidata é o melhor do produto.
- A validação do modelo fechou em todos os casos (12% de reprovação, disponibilidade da solda, lei de
  Little no porta-paletes, 220 pacientes por dia, 6 pedidos por palete).
- Quebras, turnos, Combiner, Separator "Dividir em cópias", esteira, porta-paletes com permanência
  mínima, AGV com aresta de capacidade 1 (corredor estreito) e operadores com despachante funcionaram
  como o manual descreve.
- A aba Cenários com a diferença pareada (Δ com ≈) e a aba Otimização (grade, custos, restrições,
  "equivalente") respondem exatamente o tipo de pergunta dos casos. "Salvar experimento" gera um
  arquivo que a CLI roda e dá o mesmo resultado.
- As mensagens de problema no painel ("Combiner precisa de 2 entradas...") são claras e em português.
- O truque "Base (nó da rede)", que move o transportador para o nó, salvou o caso 1.

### O que bloqueou ou atrapalhou mais

1. **Não há campo de posição** nem lista de objetos: objetos sobrepostos ou muito próximos não podem
   ser selecionados (4 inspetores na mesma mesa; filas a 1 m no CD). Tive de mudar o layout do
   cliente ou usar truques.
2. **Chegadas com taxa variável no tempo não existem**: o caso 3 exigiu 12 fontes com janelas de
   turno, uma montagem frágil (errei uma janela e só a validação mostrou).
3. **Troca de turno pausa o atendimento até o dia seguinte**: não há "terminar em hora extra" nem
   capacidade variável por turno; o caso 3 ficou com uma distorção conhecida.
4. **A janela Resultados aponta um gargalo errado** (estação de montagem com ocupação de 347%).
5. **Turnos digitados objeto por objeto**: 11 objetos x 3 janelas no caso 1, 18 objetos no caso 3;
   não há calendário global nem copiar propriedades.
6. **O `utsim fit` usa por padrão a coluna de numeração** dos CSV dos clientes e devolve uma resposta
   absurda (Uniform(1, 250), `Discrete([(1.0,119.0)])`) sem um aviso claro.
7. **Métricas que o usuário precisa e não existem prontas**: utilização dentro do turno, utilização
   sem o tempo bloqueado, tempo de ciclo entre dois pontos do fluxo, fração de perdidos.
8. No editor, o ✓ e o ✗ da otimização viram quadrados vazios: não dá para ver quais pontos são
   viáveis.

## 2. Registro de achados

Formato: ID, categoria, severidade, etiqueta; onde; esperado; aconteceu; captura; sugestão.

### Modelagem, motor, relatórios e CLI

**A01. Ocupação de 347% e gargalo errado no Combiner.** Erro do programa, alta, [modelo/simulação].
Onde: janela Resultados, aba Resumo e gráfico Ocupação (caso 1). Esperado: ocupação da estação 1 entre
0 e 100% (uma máquina). Aconteceu: "Gargalo: estacao1 (ocupação 347,3% ± 5,6)"; o conteúdo do
Combiner conta as placas que esperam dentro dele e é dividido pela capacidade 1. O gargalo apontado é
falso (a estação 1 não é o gargalo). Captura: `01-linha-de-montagem/cap/r14/03_resumo.png`,
`05_ocupacao.png`. Sugestão: calcular a ocupação do Combiner pelo contêiner em processo (ou pelos
estados processing/setup) e nunca passar de 100%.

**A02. `utsim fit` usa a coluna de numeração por padrão.** Atrito/confusão, alta, [modelo/simulação].
Onde: `utsim fit tempos_calibracao.csv` e `utsim fit chegadas_caminhoes.csv --timestamps`. Esperado:
ajustar a coluna de tempos (ou recusar). Aconteceu: ajusta "medida" (1 a 250) e recomenda
Uniform(1, 250); com `--timestamps`, devolve `Discrete([(1.0,119.0)])` com CV 0. O manual avisa, mas
o resultado sai sem alerta forte. Sugestão: detectar coluna que é 1, 2, 3... e pular para a próxima
numérica, ou parar com "a coluna X parece numeração; use --coluna 2".

**A03. Não há chegada com taxa variável no tempo.** Lacuna de funcionalidade, alta, [modelo/simulação].
Onde: caso 3 (chegadas por hora). Esperado: uma Source com tabela de taxa por hora (Poisson não
estacionário). Aconteceu: precisei de 12 fontes, cada uma com 1 ou 2 janelas de turno. Sugestão:
`interarrival` com "taxa por período" (tabela) ou Source com `rate_schedule`.

**A04. Troca de turno pausa o item até o próximo turno do mesmo objeto.** Lacuna de funcionalidade,
alta, [modelo/simulação]. Onde: caso 3, Processos por turno. Esperado: opção "terminar o item em
andamento antes de sair de turno" (comum em serviços e fábricas). Aconteceu: pacientes em consulta às
12h ficam pausados até 07h do dia seguinte (permanência de 61 a 68 min contra 13 min de consulta).
Sugestão: `shifts.on_end: Finish | Pause` e, melhor ainda, capacidade por turno (6 médicos de 07h às
12h, 5 de 12h às 17h) num só objeto.

**A05. Operador que sai de turno no meio do trabalho: comportamento não documentado.** Manual
incompleto, média, [modelo/simulação]. Onde: `recursos-moveis.md`, `quebras-e-turnos.md`. Esperado:
dizer se o operador termina a fase, abandona ou pausa. Aconteceu: só diz que os recursos móveis "têm
esses estados". Sugestão: uma seção com esse caso.

**A06. A prioridade só existe pela ordem das ligações.** Lacuna/confusão, média, [modelo/simulação].
Onde: casos 1 (fila de retorno) e 3 (urgentes). Esperado: prioridade explícita (campo de prioridade
na ligação, ou fila com disciplina por rótulo). Aconteceu: tive de criar as ligações na ordem certa
desde o início; um Processo de capacidade maior que o número de operadores ainda puxa pacientes da
fila antes de haver médico, o que quebraria a prioridade. O manual cita "fila do despachante por
prioridade" sem dizer como definir a prioridade. Sugestão: `Queue(discipline: Priority("rótulo"))` e
documentar a prioridade de serviços do despachante.

**A07. Não há utilização "dentro do turno" nem "sem bloqueio".** Lacuna de funcionalidade, média,
[modelo/simulação]. Onde: inspetores (caso 1) e empilhadeiras (caso 2). Esperado: utilização sobre o
tempo disponível. Aconteceu: a "ocupação" do editor (47,3% para o inspetor 1) inclui o tempo fora de
turno no denominador; calculei (utilize + travel_empty) / (1 − off_shift) e 1 − idle − blocked à mão.
Sugestão: métricas `available_utilization` e `busy_excluding_blocked`, e colunas na tabela.

**A08. Não há tempo entre dois pontos do fluxo.** Lacuna de funcionalidade, média,
[modelo/simulação]. Onde: caso 2, "tempo de ciclo do pedido" (da fila de pedidos à expedição).
Aconteceu: o `time_in_system` da saída conta desde a criação do caminhão (inclui 1 a 3 h no
porta-paletes); estimei somando permanências. Sugestão: marcar um instante num rótulo ("carimbo") em
qualquer objeto e medir na saída o tempo desde o carimbo.

**A09. Fração de perdidos não é usável.** Lacuna/manual, média, [modelo/simulação]. Onde: caso 3.
Aconteceu: `lost` aparece só na linha da fonte que perdeu alguém (como "lost 0,7000 ± 0,35", sem dizer
se é contagem ou média), e o manual diz que `.lost.*` não serve como objetivo. Com 12 fontes, somar
perdidos é manual. Sugestão: `source.lost.count`, `source.generated.count` e uma métrica de modelo
"perdidos/gerados".

**A10. Dispatcher com estado "blocked 82,8%".** Confusão, baixa, [modelo/simulação]. Onde: `utsim run`
do caso 2, linha `equipe_pickers`. Esperado: algo como "com serviços na fila". Aconteceu: "blocked"
sugere travamento. Sugestão: renomear para `jobs_waiting`.

**A11. Custos da otimização contam a quantidade toda, não o extra.** Atrito, baixa,
[modelo/simulação]. Onde: Otimização, caso 1. Aconteceu: objetivo 460 inclui as 2 posições que já
existem em cada buffer; o custo das extras (R$ 160) tive de subtrair. Sugestão: "custo a partir de"
(valor base) por variável.

**A12. Δ ± 0,0000 para pontos que diferem só no custo.** Confusão, baixa, [modelo/simulação]. Onde:
tabela da otimização (editor e CLI). A diferença pareada do objetivo é uma conta determinística e
aparece como "significativa" em laranja. Sugestão: mostrar Δ de custo sem IC, e o Δ pareado das
métricas das restrições (que é o que interessa).

**A13. Estados em inglês no meio de textos em português.** Confusão, baixa, [modelo/simulação]. Onde:
painel de estatísticas no modo Simular ("fração em idle", "waiting_operator") e linhas de estados do
`utsim run`. A tabela do editor traduz ("ocioso", "em falha"), o resto não. Captura:
`01-linha-de-montagem/cap/r19/03_stats.png`. Sugestão: traduzir em todos os lugares, com o nome técnico
entre parênteses.

**A14. Mensagem de erro do modo texto em inglês e técnica.** Confusão, média, [modelo/simulação]. Onde:
"editar como texto" de uma distribuição. Aconteceu: "Unexpected variant named `uniform` in enum `Dist`,
expected one of ..." (o nome é sensível a maiúsculas). Captura:
`01-linha-de-montagem/cap/r09/01_chegada_bases.png`. Sugestão: aceitar maiúsculas/minúsculas e
mensagem em português ("distribuição desconhecida 'uniform'; você quis dizer Uniform?").

**A15. Sugestão de "famílias discretas" para tempos com uma casa decimal.** Confusão, baixa,
[modelo/simulação]. Onde: `utsim fit` da calibração e das chegadas de caminhões ("considere candidatas
discretas"; Binomial negativa "aceita" para tempo entre chegadas). Sugestão: só sugerir discretas se a
variável for contagem; para tempo arredondado, avisar só do arredondamento.

**A16. O modelo se chama "novo" e não há onde mudar o nome.** Atrito, baixa, [modelo/simulação] e
[interface atual]. Onde: título dos relatórios ("novo: 20 replicação(ões)"), experimento "novo".
Sugestão: campo "Nome do modelo" nas configurações da rodada.

**A17. Turno padrão com período 1.440 (minutos) e "fora de turno: −0,0%".** Atrito/erro, baixa,
[modelo/simulação]. Onde: seção Turnos ao ser ligada. O padrão supõe minutos (o caso 1 é em segundos)
e o resumo mostra "−0,0%" (zero negativo). Captura: `01-linha-de-montagem/cap/r07/04_solda.png`.
Sugestão: unidade de tempo do modelo explícita e usada nos padrões.

**A18. Sem unidades nos campos.** Confusão, média, [modelo/simulação]. Onde: "Tempo de simulação",
"Aquecimento", "Velocidade máxima 1,20", "período". Esperado: "s", "min", "m/s". Aconteceu: tudo sem
unidade; no caso 3 (minutos) a velocidade padrão do operador "1,2" seria 1,2 m/min sem aviso.
Sugestão: unidade de tempo e de comprimento do modelo nas configurações, exibida em cada campo.

**A19. Não há calendário global.** Lacuna de funcionalidade, média, [modelo/simulação]. Onde: casos 1 e
3. Aconteceu: o mesmo calendário digitado em 11 e 18 objetos (88 campos no caso 1). Sugestão:
calendários nomeados no modelo (`calendars`) que os objetos referenciam.

### Editor 3D (interface atual)

**B01. Não há campo de posição no painel do objeto.** Lacuna, alta, [interface atual]. Onde: painel
Propriedades. Esperado: x, y digitáveis (os casos dão coordenadas). Aconteceu: só clique e arrasto com
grade; arrastos no roteiro terminam a ±1 m do alvo de forma não determinística. Captura:
`01-linha-de-montagem/cap/r07/02_fila.png`. Sugestão: campos x, y, z (e giro) no painel.

**B02. Objetos sobrepostos não podem ser selecionados.** Bloqueio, alta, [interface atual]. Onde: 4
inspetores no mesmo ponto (caso 1); fila de picking, quebra e fila de pedidos a 1 m (caso 2). Clicar no
rótulo da cena não seleciona; clicar no bloco seleciona sempre o de cima. Captura:
`01-linha-de-montagem/cap/r12/01_rotulo.png`, `02-centro-de-distribuicao/cap/r03/00_zoom.png`. Contorno:
"Base (nó da rede)" no caso 1; afastar os objetos no caso 2 (mudança de layout registrada). Sugestão:
lista de objetos (árvore) com seleção por nome, e clique no rótulo seleciona.

**B03. O painel mantém a rolagem ao trocar de objeto.** Atrito, média, [interface atual]. Onde:
Propriedades. Aconteceu: depois de rolar até "Turnos" num objeto, o próximo abre rolado e o "Nome" fica
fora da tela. Captura: `01-linha-de-montagem/cap/r04/00_saida.png`. Sugestão: voltar ao topo ao trocar
de seleção.

**B04. ✓ e ✗ viram quadrados na otimização.** Erro do programa, alta, [interface atual]. Onde: aba
Otimização (restrições do melhor ponto e coluna de viabilidade). Aconteceu: os dois aparecem como "□";
não dá para saber quais pontos são viáveis (a CLI mostra ✓ e ✗ certos). Captura:
`01-linha-de-montagem/cap/r18/03_resultado.png`, `05_rolado.png`. Sugestão: fonte com esses glifos ou
texto "viável"/"inviável" com cor.

**B05. Tabelas de cenários largas são cortadas.** Atrito, média, [interface atual]. Onde: aba Cenários
com 6 cenários (caso 1, varredura do buffer A). As colunas a partir do 5º cenário somem, sem rolagem
horizontal. Captura: `01-linha-de-montagem/cap/r17/01_comparacao.png`. Sugestão: rolagem horizontal,
tabela transposta (cenários em linhas) ou exportar CSV da comparação.

**B06. A janela Resultados corta o fim da aba Resumo.** Atrito, baixa, [interface atual]. Onde: botões
"Exportar CSV/HTML" e "Métricas próprias" (time_in_system) ficam abaixo da área visível, sem indicação
de rolagem. Captura: `01-linha-de-montagem/cap/r14/03_resumo.png`.

**B07. Texto inválido no "editar como texto" é descartado em silêncio.** Erro/atrito, média,
[interface atual]. Onde: distribuição em modo texto. Aconteceu: com texto inválido, o modelo continua
com o valor antigo (Const(1.0)), a mensagem aparece só no painel e não entra em "Problemas"; ao trocar
de objeto, a edição se perde e o modelo salvo fica com Const(1.0) sem aviso. Captura:
`01-linha-de-montagem/cap/r09/04_estacao1_setup.png`. Sugestão: manter o texto, marcar o objeto em
vermelho e listar em Problemas.

**B08. A ordem das ligações (portas e prioridade) não é mostrada nem editável.** Lacuna, média,
[interface atual]. Onde: lista "Ligações" do painel e "Pesos: saída 1, saída 2 (na ordem das
ligações)". Esperado: ver "saída 1 = expedicao" e poder reordenar. Captura:
`01-linha-de-montagem/cap/r12/03_pesos.png`. Sugestão: nome do destino ao lado de cada peso, e setas
para subir/descer ligações.

**B09. Não há "Salvar como".** Atrito, baixa, [interface atual]. Onde: barra superior. Ao abrir um
arquivo, Ctrl+S grava por cima, sem diálogo; para versionar tive de copiar arquivos. No roteiro, o
texto que eu esperava digitar no diálogo virou atalhos de teclado ("C" = Ligar etc.).
Sugestão: "Salvar como" (Ctrl+Shift+S).

**B10. Abrir um caminho inexistente cria um modelo vazio em silêncio.** Atrito, baixa, [interface
atual]. `utsim-view modelo_v2.ron` (que não existia) abriu vazio com o título do arquivo. Sugestão:
avisar "arquivo não existe; será criado ao salvar".

**B11. Navegação da câmera limitada.** Atrito, média, [interface atual]. A roda aproxima no centro da
tela, não no cursor; o zoom de afastamento é fraco (Rolar −10 quase não muda) e o chão desenhado tem
cerca de 40 x 40 m, então layouts de 48 m e 105 m (casos 1 e 2) começam fora da tela. "Focar" só
funciona em Simular. Captura: `01-linha-de-montagem/cap/r02/01_zoom3.png`,
`02-centro-de-distribuicao/cap/r03/00_zoom.png`. Sugestão: zoom no cursor, "enquadrar seleção" em
Editar, chão que acompanha o modelo.

**B12. Clique sob um painel é ignorado sem aviso.** Atrito, baixa, [interface atual]. Colocar um
objeto num ponto do mundo que cai sob o painel de Propriedades não faz nada. Captura:
`01-linha-de-montagem/cap/r01/03_processo.png`.

**B13. Listas suspensas que rolam escondem opções.** Confusão, média, [interface atual]. Onde: equipe
de operadores e transporte de saída (caso 2: "equipe_pickers" fica escondido no fim da lista);
distribuições (Pearson V e "Deslocada" só aparecem rolando; no caso 2 achei que não existiam).
Captura: `02-centro-de-distribuicao/cap/r06/e_lista_equipe.png`,
`03-clinica-com-turnos/cap/r03/03_dist_rolada.png`. Sugestão: lista com busca, ou listar despachantes
primeiro; mostrar indicador de rolagem.

**B14. Rolar dentro de uma lista suspensa rola o painel depois que ela fecha.** Erro, baixa, [interface
atual]. Caso 3: ao escolher Pearson V, o painel ficou deslocado e o rótulo seguinte saiu da tela, de
forma intermitente (3 tentativas). Captura: `03-clinica-com-turnos/cap/r04/m_pv1.png`.

**B15. Digitar letras fora de um campo dispara atalhos.** Atrito, baixa, [interface atual]. Quando o
foco ainda não chegou ao campo "Nome", "chegada_bases" virou "C" = Ligar. Sugestão: atalhos de uma
letra só com o ponteiro sobre a cena.

**B16. Rótulos de cena se sobrepõem e desviam o clique.** Atrito, baixa, [interface atual]. Nomes iguais
na cena e no painel/lista (ex.: "equipe_pickers", "Processo") disputam o clique; um clique que eu
pretendia na lista caiu no rótulo da cena e desmarcou o objeto. Sugestão: rótulos da cena não
clicáveis ou em camada inferior às listas.

**B17. "Média = 1" desatualizada no painel de distribuições.** Confusão, baixa, [interface atual].
Ao trocar a família, a linha "média = ..." às vezes continua com o valor anterior até o próximo
quadro. Pequeno, mas confunde ao conferir parâmetros.

**B18. A Rodada no modo Simular mostra "estatísticas finais (janela após o aquecimento)" com a
animação até t = 2.000 e aquecimento de 86.400.** Confusão, baixa, [interface atual]. Captura:
`01-linha-de-montagem/cap/r19/03_stats.png`. E "replicação 0" (contagem começando em zero).

### Modo roteirizado (ferramenta do piloto)

**C01. `Digitar` não digita maiúsculas.** Erro, média (para o piloto). "Uniform" virou "uniform"
(com ou sem `Segurar("Shift")`). Impediu o modo texto de distribuições; usei os menus. Captura:
`01-linha-de-montagem/cap/r10/01_shift.png`.

**C02. `Arrastar` termina a ±1 m do alvo, variando entre execuções.** Atrito, média. Mesmo partindo da
posição exata do objeto. Impediu empilhar os inspetores por arrasto.

**C03. `Focar` só em Simular; não há condição "rótulo visível" nem "valor do campo".** Atrito, baixa.
Para conferir um valor eu precisava ler o `.ron` salvo; uma condição `Campo("solda", "process",
"Triangular(...)")` ajudaria.

## 3. Tempo e esforço por caso

| Caso | Execuções de roteiro | Para montar o modelo | Para analisar | Situação |
|---|---|---|---|---|
| 1. Linha de montagem | cerca de 38 | cerca de 28 (colocar 7, ligar 2, propriedades 6, turnos/inspetores 7, rede 1, explorações 5) | 6 (Resumo, Cenários x3, Otimização, Simular) | completo, 3 perguntas com IC |
| 2. Centro de distribuição | cerca de 17 | cerca de 16 (colocar 3, ligar 6 por causa da sobreposição, propriedades 5, rede 1, exploração 1) | 1 (Cenários) + CLI | completo, 3 perguntas com IC |
| 3. Pronto atendimento | 9 | 8 (colocar 1, ligar 1, exploração 1, propriedades 4, correção da janela 1) | 1 (variante +15%) + CLI | completo, com aproximações registradas |

O caso 1 custou mais porque foi quando aprendi o editor: cerca de metade das execuções foi para
descobrir rótulos e posições de campos, a rolagem do painel e o problema dos objetos empilhados. Nos
casos 2 e 3 o maior custo foi a seleção de objetos próximos e as listas suspensas. Os roteiros de
propriedades chegaram a 1.300 a 1.400 passos (turnos e operadores são a maior parte).

Preferência entre editor e CLI: para **montar** o modelo, o editor; para **analisar**, uso os dois.
A aba Cenários é ótima para 2 a 4 cenários (vejo o Δ pareado na hora) e o "Salvar experimento" me
dá um arquivo reproduzível. Para varreduras com mais de 4 cenários e otimizações com 6 variáveis,
preferi `utsim experiment` e `utsim otimizar`: a tabela inteira cabe, o ✓/✗ aparece certo e dá para
gravar `--json`. Falta no editor um "Salvar problema de otimização" (só existe para experimentos).

## 4. As 10 melhorias de maior impacto para um usuário novo

1. Campos de posição (x, y) no painel e lista de objetos com seleção por nome (B01, B02).
2. Corrigir a ocupação do Combiner e o gargalo apontado (A01).
3. Chegadas com taxa por período (tabela de taxa por hora) (A03).
4. Calendários de turno nomeados e reutilizáveis, com opção "terminar o item ao sair de turno" e
   capacidade por turno (A04, A19).
5. `utsim fit` detectar coluna de numeração e escolher a coluna de dados (A02).
6. Métricas prontas: utilização dentro do turno, utilização sem bloqueio, tempo entre dois pontos,
   fração de perdidos (A07, A08, A09).
7. Viável/inviável legível na otimização do editor e "Salvar problema de otimização" (B04).
8. Prioridade explícita (fila por prioridade de rótulo) e ordem das ligações visível e editável
   (A06, B08).
9. Unidades de tempo e comprimento do modelo, exibidas em todos os campos e usadas nos padrões
   (A17, A18).
10. Tabela de cenários com rolagem horizontal ou transposta, e listas suspensas com busca (B05, B13).

## 5. Números suspeitos

**S1. Ocupação de 347% e gargalo "estacao1" (caso 1).** Ver A01. A ocupação real da estação 1 (pela
fração processing + setup) é de cerca de 51% do tempo total. Qualquer usuário novo tomaria a estação
1 como gargalo.

**S2. Utilização dos inspetores muito desigual (75,7%, 67,3%, 53,6%) e o 4º inspetor nunca usado
(caso 1).** Não é erro: o despachante "Primeiro livre" sempre escolhe o primeiro, e com 3 bancadas
só 3 inspeções acontecem ao mesmo tempo. Mas o relatório não explica, e o Δ "+0,0000 ± 0,0000" entre 3
e 4 inspetores parece defeito à primeira vista. Também: com 2 inspetores, a caminhada cai de 4,1% para
0,4% do tempo (eles quase não voltam à mesa), o que é coerente mas surpreende.

**S3. Caminhões por 40 h: 87,1 ± 3,2 contra 83,1 esperados (caso 2).** O IC de 30 replicações não
cobre o valor teórico. Com 300 replicações deu 83,17 ± 1,03, então é acaso. Fica o registro de que 30
replicações podem enganar a validação.

**S4. Fila da embalagem cresce com mais pickers (caso 2, embalagem a 20 m).** Com 7 e 8 pickers, a
espera na fila da embalagem sobe para 110 e 149 s (pico de 55 a 72 caixas), embora as 5 mesas fiquem
ociosas 36% do tempo e a ocupação média seja de 56%. Pode ser real (entregas em rajadas de até 3
caixas e chegada de paletes em lotes de 10), mas a magnitude merece uma verificação do motor (por
exemplo, se a espera na fila conta desde a reserva do lugar pelo transportador e não desde a entrega).

**S5. Porta-paletes com pico de 91 de 96 posições, média de 42 (caso 2).** Plausível com caminhões de
10 paletes e chegadas exponenciais, mas o pico é estável entre cenários (91,2 ± 2,3) e merece atenção
do cliente: o porta-paletes está perto de encher.

**S6. Permanência na consulta de 61 a 68 min com consulta média de 13 min (caso 3).** Efeito da pausa
na troca de turno (A04): pacientes atendidos às 12h, 17h e 21h ficam pausados até o dia seguinte. Não
afeta as esperas nas filas, mas o `time_in_system` (93 a 130 min) não deve ser lido como tempo do
paciente na clínica.

**S7. `lost 0,7000 ± 0,3455` só numa das 12 fontes (caso 3, +15%).** Não fica claro se é contagem por
replicação; e as outras fontes não mostram 0, simplesmente omitem a métrica.

**S8. Diferença no número de urgentes entre cenários com a mesma semente (caso 3, 901,6 contra 897,0).**
Mudar a capacidade da triagem muda a sequência dos sorteios de "Pesos" na saída. Não é erro, mas
contraria a ideia do manual de que mexer num objeto não muda os sorteios dos outros: aqui muda o do
próprio objeto e reduz um pouco o ganho dos números aleatórios comuns.

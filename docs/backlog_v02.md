# Backlog da versão 0.2 e modelo de IA por tarefa

Lista do que pode ser integrado ou melhorado depois da v0.1, com o modelo indicado para cada
tarefa e a leva em que ela pode rodar. O plano dos cinco recursos pedidos em 2026-11-02 (PERT/CPM,
estatística avançada, motorista, desvio de objetos e planta CAD) fica em
`docs/plano_analises_e_dinamica.md`. A interface Godot segue `docs/plano_godot.md`.

## Critério de escolha do modelo

| Modelo | Quando usar |
|---|---|
| Opus | Projeto de arquitetura, algoritmos novos, estatística que precisa estar correta, mudanças que cruzam várias crates, decisões de formato do modelo |
| Sonnet | Implementação com especificação clara, interface, empacotamento, documentação técnica, testes a partir de um plano |
| Haiku | Tarefas mecânicas: extrair textos para tradução, renomear, ajustar CI, formatar, conferir links e listas |

Regra de paralelismo: duas tarefas só rodam ao mesmo tempo se não mexem nos mesmos arquivos.
Cada uma roda numa cópia isolada do repositório (worktree) e entrega um branch para revisão.

## Em andamento (leva 0)

| Tarefa | Modelo | Situação |
|---|---|---|
| G2 Godot: edição do fluxo | Opus | feita |
| G5 Godot: empacotamento e instalador | Sonnet | em andamento |
| Plano: PERT/CPM, estatística, motorista, desvio, CAD | Opus | em andamento |

## Leva 1 (depois da G2)

| Tarefa | Modelo | Depende de | Arquivos principais |
|---|---|---|---|
| G3 Godot: rede, fluidos, resultados e gráficos (**feita**) | Opus | G2 | `godot/`, `crates/utsim-godot` |
| Crate `utsim-pert` (rede de atividades, CPM, PERT, caminho crítico estocástico) | Opus | plano | crate nova |
| Estatística: aquecimento (MSER-5), médias em lotes, replicações necessárias | Opus | plano | `utsim-model/src/analysis.rs` ou crate nova |
| Leitura de DXF, camadas e calibração de escala (núcleo, sem interface) | Sonnet | plano | crate nova |
| Capacidade por hora do dia (resto da lacuna 1 dos estudos de caso) | Sonnet | nenhuma | `utsim-objects`, `utsim-model` |
| Gatilho de push da CI no repositório recriado | Haiku | nenhuma | `.github/workflows/` |

## Leva 2

| Tarefa | Modelo | Depende de |
|---|---|---|
| Interface PERT/CPM no Godot (seleção, diagrama de rede, Gantt, histograma): **feita na G3**, falta o tornado | Sonnet | `utsim-pert`, G3 |
| Botão "Fluxograma" na interface Godot (G3 ou a seguinte): chama `utsim-diagram` pela ponte, mostra o SVG num painel, exporta SVG, HTML, Mermaid e CSV e acende no desenho a etapa selecionada no 3D | Sonnet | `utsim-diagram`, G3 |
| Estudo de tempos e métodos na interface (Godot ou egui, depois da migração da interface para Rust): botão "Tempos" que chama `utsim_diagram::tempos` pela ponte, mostra o gráfico do ciclo e as tabelas, escolhe operador, intervalo e ciclo, e acende no 3D o ator do elemento selecionado | Sonnet | `utsim-diagram`, interface |
| Estudo de tempos com várias replicações (ciclos de todas as replicações juntos, IC dos tempos por elemento), atendimento sincronizado com máquinas diferentes e operador que antecipa a caminhada (sair para a próxima máquina antes do chamado) | Opus | `utsim-diagram`, `utsim-objects` |
| Campo opcional `symbol` em `ObjectDef` (proposta): guarda no `.ron` o símbolo do fluxograma de cada objeto, em vez de depender do nome ou da opção `--simbolo`. Muda o formato do modelo: decisão do dono | Opus | `utsim-diagram` |
| Painel de estatística avançada no Godot: P1 **feito na G3**, falta o P2 (Gantt de estados, WIP com banda, ANOVA, mapa de calor) | Sonnet | estatística, G3 |
| Planta CAD como fundo no piso, com encaixe nas linhas | Sonnet | DXF, G2 |
| Obstáculos estáticos e roteamento automático (malha + A*) | Opus | plano, DXF |
| Reservas determinísticas de trecho e cruzamento entre recursos móveis | Opus | roteamento |
| Motorista na empilhadeira (pose sentada, recurso opcional) | Sonnet | plano, G2 |
| G4 Godot: realidade virtual e aumentada (OpenXR) | Opus | G3 |

## Simulação e modelagem

| Tarefa | Modelo |
|---|---|
| Regras de despacho (prioridade, menor tempo, data de entrega) e setup dependente da sequência (matriz de setup) | Opus |
| Produção puxada: kanban, CONWIP, supermercado | Opus |
| Equipes de manutenção para as quebras (MTBF/MTTR com técnico como recurso) | Sonnet |
| Custos por estado, por item e por recurso, com relatório de custo | Sonnet |
| Energia e emissões por estado de cada objeto | Sonnet |
| Calendário com feriados e exceções | Sonnet |
| Submodelos: um grupo de objetos vira um bloco reutilizável com portas | Opus |
| Tabelas globais importadas de planilha (roteiros, mix de produtos, chegadas programadas) | Sonnet |
| Chegadas por tabela (horário e sequência), lote por chegada, tipos de item; prioridade, regras novas, preempção e casa das equipes; ligações de equipe no 3D: plano em `docs/plano_chegadas_e_equipes_3d.md` | Opus (projeto) e Sonnet |
| Linhas especializadas (pacotes de domínio), começando pelo Lean Office: plano em `docs/plano_linhas_especializadas.md` | Opus (projeto) e Sonnet |

## Análise e decisão

| Tarefa | Modelo |
|---|---|
| Otimização com vários objetivos (fronteira de Pareto) no `utsim-opt` | Opus |
| Planejamento de experimentos (fatorial, hipercubo latino) e metamodelo | Opus |
| Comparação de cenários lado a lado na 3D (duas vistas sincronizadas) | Sonnet |
| Relatório exportável em PDF e planilha, com gráficos | Sonnet |
| Mapa de calor de deslocamentos e congestionamento no piso | Sonnet |

## Dados e integração

| Tarefa | Modelo |
|---|---|
| Biblioteca Python (PyO3) para rodar modelos e ler resultados em notebooks | Opus |
| Conectores MQTT e banco SQL, além do OPC UA que já existe | Sonnet |
| Gêmeo digital: modelo alimentado por dados ao vivo e comparação com o real | Opus |
| Protocolo próprio para placas (ESP32, Arduino, Raspberry Pi e placas do dono) enviarem dados direto ao UTSim: conversar o desenho com o dono antes (transporte, formato, autenticação, biblioteca para as placas) | Opus |

## Interface, ativos 3D e conteúdo

| Tarefa | Modelo |
|---|---|
| Gravar vídeo da simulação pelo próprio programa (MP4, com câmeras roteirizadas) | Sonnet |
| Tutorial interativo dentro do programa | Sonnet |
| Tradução da interface e do manual (inglês, espanhol): extrair e organizar textos | Haiku |
| Tradução da interface e do manual: revisão técnica dos termos | Sonnet |
| Ambiente de fábrica (galpão, piso, docas, iluminação) e pessoas com uniformes e EPI, via Tripo 3D e Blender | Sonnet |
| Linha de produção de ativos: escala, eixos, níveis de detalhe e conferência automática dos `.glb` | Sonnet |
| Guia passo a passo item a item (LaTeX, regras de escrita do autor) | Sonnet |
| Vídeos tutoriais por exemplo (roteiro, gravação e narração neural) | Sonnet |

## Ensino

| Tarefa | Modelo |
|---|---|
| Modo sala de aula: exercícios com enunciado, dados e gabarito; correção automática do modelo do aluno pelas métricas, com tolerância | Opus (projeto) e Sonnet (interface) |
| Pacotes de exercícios a partir dos estudos de caso e dos tutoriais | Sonnet |
| **Jogo de resolução de problemas da indústria pela simulação** (ideia do dono, 2026-11-03, marcada como muito importante): o aluno resolve problemas reais com o que o núcleo simula, sem precisar usar o simulador no começo; trilha do básico da engenharia de produção (gargalo, filas, balanceamento, arranjo físico, capacidade, tempos e métodos) a problemas complexos, terminando num caso que exige Lean Seis Sigma (DMAIC no app UTSim Seis Sigma). Decisão do dono (2026-11-03): o ensino é um **app novo, próprio, baseado no UTSim**, fora do UTSim Seis Sigma; o antigo L8 do plano do Seis Sigma (seção 8.6: projetos-exemplo, dados com efeitos plantados, exercícios corrigidos, casos a partir de modelos) passa para esse app, que também deve abrigar o jogo (confirmar com o dono ao planejar). Reaproveita as linhas especializadas, o estudo de tempos e o modo sala de aula | Opus (projeto) e Sonnet |

## Engenharia de qualidade e segurança do trabalho

| Tarefa | Modelo |
|---|---|
| **Frente futura de engenharia de qualidade e de segurança do trabalho, integrada aos outros apps** (ideia do dono, 2026-11-06; ainda não planejada, decidir com o dono se é app próprio ou módulos dentro dos apps existentes). Qualidade: CEP e capacidade ligados aos dados simulados do UTSim, plano de controle, inspeção e amostragem, custo da qualidade, refugo e retrabalho no modelo, ligação com o FMEA, a MSA e o DMAIC do UTSim Seis Sigma. Segurança: análise de riscos e perigos por posto, ergonomia ligada ao estudo de tempos (`utsim tempos`), EPI por posto no 3D, interação entre pessoas e empilhadeiras e zonas de risco no arranjo físico, registro de incidentes e indicadores. Integração: mesmo modelo e dados do UTSim, projetos do Seis Sigma, casos para o app de ensino e o jogo, linhas especializadas e o futuro protocolo das placas como fonte de dados | Opus (plano) |

## Qualidade e distribuição

| Tarefa | Modelo |
|---|---|
| Launcher com códigos de acesso, canais e atualizações pelo Drive | Opus (projeto) e Sonnet (implementação) |
| Testes de regressão visual do cliente Godot (capturas comparadas) | Sonnet |
| Desempenho com modelos grandes (10 mil objetos, perfis do kernel e da ponte) | Opus |
| Fuzzing do leitor de modelos `.ron` e da migração de formato | Sonnet |
| Avisar campos desconhecidos no `.ron` (hoje um `shifts:` dentro de `Processor(...)` é ignorado em silêncio) | Sonnet |
| Relatório de erros opcional, sem custo de servidor | Sonnet |
| Reduzir o tamanho do pacote Linux | Haiku |

# Piloto simulado (antes da v0.1)

Sem casos reais disponíveis, o piloto teve três partes:

1. **Estudos de caso** (`docs/estudos_de_caso/`): três problemas realistas com gabarito.
2. **Validação** (`docs/validacao.md`): 11 casos com resposta teórica, 122 métricas, todas
   dentro do intervalo de confiança.
3. **Usuário iniciante** (este diretório): um engenheiro que nunca viu o UTSim recebeu só
   os enunciados, os dados, o manual e o programa, e montou os modelos pela interface.
   As respostas dele bateram com os gabaritos.

## Lista de acabamento para a v0.1 (prioridade)

Marcação: [M] vale para qualquer interface (modelo e simulação); [I] é da interface atual.

| # | Achado | Tipo | Prioridade |
|---|---|---|---|
| 1 | Gargalo errado no painel de resultados: a montagem aparece com 347% de ocupação, porque conta os componentes esperando dentro dela | M | alta |
| 2 | Não há campo de posição nem lista de objetos para selecionar; objetos sobrepostos não podem ser escolhidos | I | alta |
| 3 | Chegadas que mudam por horário exigem várias fontes (lacuna 1 dos estudos de caso) | M | alta |
| 4 | `utsim fit` usa a coluna de numeração por padrão e devolve resultado sem sentido, sem aviso | M | alta |
| 5 | Turnos: trabalho em andamento pausa no fim do turno (sem opção de terminar); não há calendário global de turnos | M | média |
| 6 | Métricas que faltam: ocupação dentro do turno, ocupação sem bloqueio, tempo entre dois pontos do fluxo, fração de perdas | M | média |
| 7 | Aba Otimização: os sinais de viável e inviável aparecem como caixas vazias | I | média |
| 8 | Tabela de cenários corta depois de 4; listas suspensas escondem opções; painel não volta ao topo ao trocar de objeto; texto inválido descartado em silêncio; ordem das ligações não aparece; falta "Salvar como"; o zoom vai ao centro, não ao cursor | I | média |
| 9 | Números a investigar: fila da embalagem cresce com mais separadores enquanto as mesas ficam 36% ociosas | M | média |

Detalhes, capturas e respostas: `relatorio_usuario_iniciante.md` e `respostas_*.md`.

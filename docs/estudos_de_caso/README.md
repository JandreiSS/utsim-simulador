# Estudos de caso

Três casos que um engenheiro de produção ou de operações traria ao UTSim. Eles são maiores que os [tutoriais](../manual/src/tutoriais/index.md) e foram escritos para o piloto simulado do programa: cada enunciado traz o contexto da empresa, os dados, as regras do processo e perguntas de decisão com critérios de aceitação, sem dizer **como** montar o modelo. Quem resolve recebe só o `enunciado.md`, a pasta `dados/` e o manual.

## Índice

| Caso | Dificuldade | Domínio | O que exercita |
|---|---|---|---|
| [1. Linha de montagem com inspeção e retrabalho](01-linha-de-montagem/enunciado.md) | média | fábrica | montagem de kit (`Combiner`), quebras e dois turnos, inspeção com operadores que andam, retrabalho em laço, prioridade por ordem de conexão, ajuste de quatro conjuntos de dados, experimento (inspetores e buffer) e otimização com restrições |
| [2. Centro de distribuição com corredor estreito](02-centro-de-distribuicao/enunciado.md) | alta | logística | docas, esteira com acúmulo, porta-paletes seletivo, empilhadeiras em rede de caminhos com pista única, `Dispatcher` como equipe de operadores e de transporte, carrinho de 3 caixas, `transfer` por distância, ajuste com `--timestamps`, experimento de frota, equipe e layout |
| [3. Pronto atendimento com turnos e prioridade](03-clinica-com-turnos/enunciado.md) | média a alta | serviço | chegadas por hora, triagem, rótulo e prioridade, sala de espera que perde pacientes, equipes por turno, ajuste de uma mistura, otimização do custo da escala |

Cada pasta tem:

- `enunciado.md`: o que se entrega ao analista;
- `dados/*.csv`: dados brutos gerados com semente fixa, para o analista ajustar distribuições;
- `gabarito/modelo.ron`: o modelo de referência, verificado com `utsim check` e `utsim run`;
- `gabarito/experimento.ron` e/ou `gabarito/otimizacao.ron`: os estudos que respondem às perguntas;
- `gabarito/resolucao.md`: as respostas esperadas, com intervalos de confiança, o raciocínio e as funcionalidades exercitadas;
- `gabarito/gerar_dados.py`: gera os CSVs (as distribuições de origem só aparecem aqui).

O teste `crates/utsim-model/tests/estudos_de_caso.rs` carrega e roda os três modelos de referência e confere os números principais com tolerâncias largas.

## Ordem sugerida

1. **Caso 1** primeiro: é o mais parecido com os tutoriais (linha com gargalo), com estações, quebras e uma otimização.
2. **Caso 3** em seguida: muda o tipo de problema (pessoas, filas, turnos) e mostra os limites do modelo para chegadas que variam com a hora.
3. **Caso 2** por último: o maior em número de objetos e o único que depende de distâncias reais e de uma rede de caminhos.

## Lacunas encontradas no UTSim

Construir os modelos de referência mostrou coisas que um engenheiro de produção esperaria encontrar e que o UTSim ainda não faz. Em cada uma, o gabarito usa a aproximação mais próxima e registra a diferença. Estas são as descobertas mais valiosas do piloto.

| # | Lacuna | Aproximação usada | Caso | Gravidade |
|---|---|---|---|---|
| 1 | **Taxa de chegada e capacidade que variam com a hora.** Não há tabela horária de chegadas nem capacidade por faixa de horário. | Uma `Source` por faixa de taxa constante, ligada só dentro da sua faixa por `shifts` (exato quando o tempo entre chegadas é `Exp`, porque o relógio pausado não tem memória). A capacidade que varia por turno vira um `Processor` por turno, cada um com seu `capacity`. A clínica usou 6 fontes e 6 grupos de profissionais, e o modelo fica difícil de ler e de editar. | 3 | alta |
| 2 | **O fim de turno pausa o atendimento em andamento em vez de terminá-lo.** Uma consulta às 11h59 fica congelada no médico até o turno voltar, no dia seguinte. | Aceito no gabarito. O congelamento só atrapalha medidas de tempo de passagem (`time_in_system` sai com 94 min em vez de uns 30); as esperas em fila não são afetadas, mas a capacidade do início do turno seguinte cai uns 2,4%. Seria útil uma opção "terminar o item corrente antes de sair do turno". | 3 | alta |
| 3 | **Não há métricas por janela de tempo, por turno ou percentis.** Só médias (com intervalo) e o máximo do conteúdo. Não dá para dizer "95% dos pacientes esperam menos de 20 min" nem "a espera do turno S2 ≤ 15 min", nem usar isso como restrição na otimização. | As metas do gabarito são médias do dia. A restrição por turno não existe. O histograma (`--bins`) mostra a distribuição da permanência, mas não entra como restrição. | 3 e 2 | média |
| 4 | **Fila só FIFO ou LIFO e tempo de serviço independente do rótulo.** Não há `Queue` com prioridade e um `Processor` não muda o tempo de processo conforme o tipo do item (urgente demora mais). | Prioridade: duas filas ligadas ao processo por ordem de conexão, com `ByLabel` antes (funciona, não é intuitivo e não preempta). Tempo por tipo: uma distribuição única para a mistura. | 3 | média |
| 5 | **O MTBF conta no relógio, inclusive fora de turno.** Quem tem o tempo entre falhas em horas de operação precisa saber disso. Para `Exp` é equivalente, para `Weibull` e outras não. | O enunciado já diz que as falhas foram registradas em tempo corrido. | 1 | baixa |
| 6 | **Variável de escolha (`Choice`) não aceita custo, e o tamanho de uma equipe só varia como lista de nomes.** Para "quantos inspetores/pickers/empilhadeiras" é preciso criar todos os membros e trocar o texto `["a", "b"]` do `team`. Dá para otimizar, mas sem custo por pessoa no objetivo, e o resultado diz "3 e 4 são equivalentes". | Custo por posição de buffer (numérico) entra no objetivo; a equipe fica como `Choice` sem custo. No caso 2 não houve otimização, porque a decisão principal era de equipe. | 1 e 2 | média |
| 7 | **Experimentos e otimização não alteram a rede de caminhos nem criam objetos.** Só mexem em campos de objetos existentes. Mudar o layout (mover a embalagem) funciona, mas a rede continua a mesma. | O gabarito mantém um nó de rede na posição antiga e muda só as posições dos objetos; o picker ainda anda o caminho certo pelo nó mais próximo. | 2 | média |
| 8 | **O relatório de otimização em grade imprime todos os pontos.** Com 14.112 pontos, a saída tem 14 mil linhas; não há limite para a tabela. | Para espaços grandes usa-se um domínio menor em grade (como no gabarito) ou um algoritmo genético ou de recozimento. No caso da clínica, o genético com orçamento de 450 pontos **não achou** o ótimo (R$ 16.240 contra R$ 15.950). | 3 | baixa |

Outras observações do piloto, que não são lacunas:

- **O `utsim fit` é rigoroso.** A primeira amostra do tempo de inspeção (300 observações de uma Gama verdadeira) teve todas as famílias rejeitadas, com p = 0,003 (o piso do bootstrap), e o programa recomendou a distribuição empírica; três das quatro primeiras amostras do tempo de triagem (Triangular verdadeira) rejeitaram a família verdadeira. Os CSVs usados nos casos vêm de sementes em que o ajuste escolhe uma família (veja `gerar_dados.py`). Quem aplicar o mesmo procedimento a dados reais pode esbarrar nisso.
- **O refinamento da otimização trabalha.** Na linha de montagem, um ponto de 4 posições extras parecia viável com 10 replicações e virou inviável com 30. O relatório deixa isso claro.
- **O arquivo de dados com coluna de numeração** exige `--coluna 2`; o manual avisa, mas é um tropeço fácil.
- **As chegadas por hora** pedem 6 fontes (e as escalas pedem 6 grupos de profissionais). Seria útil poder copiar e colar objetos com seus turnos no editor.
- **Variância das rodadas.** O caso 2 tem rajadas (10 paletes por caminhão, 6 caixas por palete), o que dá intervalos de confiança largos para esperas em fila (±30% da média) mesmo com 30 replicações de 40 horas. O gabarito usa a diferença pareada com as mesmas sementes para decidir.

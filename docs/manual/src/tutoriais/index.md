# Visão geral dos tutoriais

Os seis tutoriais são exercícios guiados. Cada um tem um modelo pronto na pasta `examples/` do projeto, com os comandos para rodar e os **resultados esperados**. A ordem vai do mais simples ao mais completo.

| # | Tutorial | Você aprende | Arquivos |
|---|---|---|---|
| 01 | [Uma fila e um caixa](01-um-caixa.md) | ler o relatório; comparar com a fórmula da fila M/M/1; lei de Little | `tutorial_01_caixa.ron` |
| 02 | [Linha com gargalo](02-gargalo.md) | achar o gargalo pela utilização; testar o reforço de uma máquina | `tutorial_02_linha.ron`, `experimento_tutorial_02.ron` |
| 03 | [Operadores que andam](03-operadores.md) | ver que a distância entre as máquinas muda o resultado | `tutorial_03_operadores.ron`, `experimento_tutorial_03.ron` |
| 04 | [Empilhadeiras e porta-paletes](04-armazem.md) | dimensionar a frota e o porta-paletes de um armazém | `tutorial_04_armazem.ron`, `experimento_tutorial_04.ron` |
| 05 | [Ajuste de distribuição](05-ajuste.md) | transformar dados reais em distribuições para o modelo | `tutorial_05_ajuste.ron`, `experimento_tutorial_05.ron`, `dados/tutorial_05_*.csv` |
| 06 | [Experimento e otimização](06-experimento-e-otimizacao.md) | escolher o número de atendentes com `utsim experiment` e `utsim otimizar` | `tutorial_06_atendimento.ron`, `experimento_tutorial_06.ron`, `otimizacao_tutorial_06.ron` |

## Como seguir

1. Tenha o `utsim` instalado (veja a [Introdução](../introducao.md)). Os comandos abaixo supõem que você está na pasta raiz do projeto, onde fica `examples/`.
2. Valide o modelo com `utsim check` e rode com `utsim run`.
3. Para ver em 3D, abra o modelo no visualizador: `utsim-view examples/tutorial_01_caixa.ron`.
4. Faça os exercícios finais de cada tutorial. É assim que se aprende.

## Sobre os números

Os resultados dos tutoriais foram obtidos com as sementes dos arquivos e **se repetem exatamente** na mesma versão do programa. Se você mudar a semente, o número de replicações ou o tempo de simulação, os números mudam um pouco. O que deve permanecer é a conclusão e a ordem de grandeza, dentro dos intervalos de confiança (`±`).

Um teste automático do projeto (`crates/utsim-model/tests/tutoriais.rs`) roda todos os modelos dos tutoriais e confere que os resultados principais continuam dentro das faixas citadas aqui.

## Unidades

- Tutoriais 01, 02, 05 e 06: o tempo é em **minutos**.
- Tutoriais 03 e 04: o tempo é em **segundos** e as distâncias em **metros**.
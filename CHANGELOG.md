# Changelog

As mudanças deste repositório ficam registradas aqui. O histórico de antes da separação
(2026-11-25) está no CHANGELOG.md da família.

## [Não lançado]

### Novo
- Aba **Ambiental** do nó (família 0.2.0): liga o bloco ambiental e edita a etapa e a potência em cada grupo de estado (produzindo, preparando, esperando, parada, fora de turno); o mesmo dado que o UTSim Ambiental edita. Depois de uma rodada, a aba mostra a energia do nó, a parte sem valor agregado e a energia por peça. "Abrir no UTSim Ambiental" abre o app no nó, como os de Qualidade e Segurança.
- Camada **Ambiental** da reprodução no UTSim 3D (fase AM13 do UTSim Ambiental), na chave "Ambiental" do painel Simular quando o modelo tem bloco ambiental: brilho no piso na cor da energia de cada máquina, coluna translúcida com a potência no instante, contorno nas peças que vão para o refugo com o cartão da mochila ao clicar, e o medidor da fábrica no canto (potência no instante, pico da curva, janela de pico da tarifa). A conta é a do UTSim Ambiental (`utsim-ambiental-core`, `camada3d`): a potência mostrada é a da curva de carga do app no mesmo instante. Exemplo `examples/linha_verde.ron`.

### Alterado
- Usa a família pela tag v0.2.1 (métricas ambientais por nó no relatório).
- A ponte do Godot usa o núcleo do UTSim Ambiental (`utsim-ambiental-core`, pelo commit no Cargo.toml).
- Repositório próprio, separado do monorepo da família UTSim; usa a família pela tag v0.1.0.

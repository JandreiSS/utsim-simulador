# Zonas de risco, EPI e exposição

O modelo pode guardar zonas de risco no piso e o EPI exigido em cada posto. O comando `utsim exposicao` mede, sobre o rastro de cada replicação, quanto tempo cada pessoa passa em cada zona, quantas vezes entra sem o EPI pedido e quantas vezes chega perto demais de uma empilhadeira. O exemplo pronto é `examples/cruzamento_empilhadeira.ron`.

> O UTSim apoia o estudo, o ensino e a organização das informações. Ele não substitui o profissional legalmente habilitado, não emite laudo e não certifica conformidade com nenhuma Norma Regulamentadora. O cumprimento da legislação é responsabilidade do usuário e da empresa.

## Zonas e EPI no modelo

```ron
zones: [
    (name: "corredor_empilhadeira", kind: Vehicle, level: Some(4), ppe: ["colete"],
     polygon: [(-12.0, 4.5), (12.0, 4.5), (12.0, 7.5), (-12.0, 7.5)]),
],
```

Tipos de zona: `Vehicle` (circulação de veículos), `Machine` (alcance de máquina), `SuspendedLoad` (carga suspensa), `Noise` (ruído), `Heat` (calor) e `Other`. O `level` é o nível da matriz de risco e dá a cor no 3D.

No objeto, `ppe: ["oculos", "luvas"]` diz o EPI do posto. A pessoa usa o EPI dela (o `ppe` do próprio transportador) mais o de todos os postos que atende como operador ou transportadora. Os nomes que o 3D sabe vestir são `capacete`, `oculos`, `protetor_auricular`, `luvas`, `botina`, `colete`, `mascara` e `avental`.

## O que é medido

| Medida | Como sai |
|---|---|
| Tempo em zona | tempo de cada pessoa dentro de cada polígono, na janela de estatística |
| Entradas | quantas vezes a pessoa entrou na zona |
| Entradas sem EPI | entradas de pessoa que não usa todo o EPI pedido pela zona |
| Quase-conflito simulado | cada vez que a distância entre uma pessoa e um veículo fica abaixo do limiar (1,5 m por padrão) |
| Mapa de calor | tempo de pessoas por célula do piso e células com quase-conflito |
| Conferência do layout | posto dentro de zona de veículo, corredor misto sem parada antes dos cruzamentos |

A pessoa e o veículo andam em velocidade constante em cada viagem, sem aceleração, e ficam parados onde terminaram até a próxima viagem.

```bash
utsim exposicao examples/cruzamento_empilhadeira.ron --limiar 1.5 --html exposicao.html
```

Quase-conflitos simulados servem para comparar layouts e regras de circulação do mesmo modelo. Eles não estimam a probabilidade de um acidente real. A NR-11 (transporte e movimentação de materiais) e a NR-6 (EPI) são citadas só como tema, sem conferência automática de conformidade.

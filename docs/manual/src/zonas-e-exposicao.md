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

## No 3D

Na aba Simular do cliente 3D, o painel da esquerda tem as chaves **Segurança** e **Calor**. Ligadas, elas mostram a camada de segurança e qualidade da replicação 0, calculada na ponte em Rust com as mesmas contas deste capítulo. Nenhum dado novo é preciso no modelo.

- **Zonas no piso.** Cada zona vira um polígono semitransparente com contorno e nome. A cor segue o `level`: verde no nível 1, passando por verde-amarelado, amarelo e laranja, até vermelho no nível 5. Sem `level`, a zona sai azul.
- **EPI na pessoa.** A pessoa veste as peças do EPI que ela usa naquele instante, que são o `ppe` dela mais o do posto que está atendendo. Ao chegar a outro posto, as peças trocam. As peças são geradas por código, uma para cada nome da lista acima.
- **Pessoa sem o EPI pedido.** Quem está dentro de uma zona sem todo o EPI que a zona pede ganha um anel vermelho no chão e um aviso com o nome da zona e do que falta.
- **Quase-conflitos.** Cada quase-conflito simulado ganha um marcador no local, que cresce enquanto ele acontece. A lista fica no painel e cada linha salta a reprodução até dois segundos antes do instante. Valem os avisos do parágrafo acima: são simulados e servem para comparar layouts.
- **Qualidade.** O item que termina com `defeitos` maior que zero ganha uma marca vermelha enquanto está na cena, e os postos com `inspection` ganham uma lupa girando sobre eles.
- **Calor.** A chave **Calor** põe no piso uma textura com a presença das pessoas e as células de quase-conflito, do transparente ao vermelho.

Limites desta primeira versão: a posição da pessoa para a conta das zonas segue o trajeto da exposição (velocidade constante em cada viagem), e o item é marcado como defeituoso por inteiro, desde que aparece, pelo rótulo que tem no fim da replicação.

## Desenhar e editar zonas no editor 3D

No Editar, a ferramenta **Zona** (botão na barra, tecla `Z`) desenha e muda as zonas de risco sem escrever o `.ron`:

1. Clique no chão para pôr os cantos da zona; os pontos encaixam na grade (`G` liga e desliga o encaixe).
2. Feche a zona clicando de novo no primeiro ponto, com `Enter` ou com um duplo clique. São pelo menos 3 cantos; `Backspace` volta o último ponto e `Esc` cancela.
3. A zona nova se chama `zona_1`, `zona_2`... e começa como tráfego de veículos, nível 3, sem EPI. No painel da direita mude o **nome**, o **tipo** (tráfego de veículos, alcance de máquina, carga suspensa, ruído, calor ou outro), o **nível de risco** (1 a 5, que dá a cor) e marque o **EPI exigido para entrar**.
4. Clique numa zona para editá-la: arraste o meio para mover a zona inteira e arraste um canto para mudar a forma. `Del` exclui a zona selecionada.

Cada mudança é um passo de desfazer (`Ctrl+Z`, `Ctrl+Y`) e vai para a lista `zones` do modelo ao salvar. No Editar as zonas aparecem no piso com uma cor translúcida pelo nível, a mesma da camada de segurança do Simular (verde no 1, verde-amarelo, amarelo, laranja, vermelho no 5; azul sem nível), a borda e o nome.


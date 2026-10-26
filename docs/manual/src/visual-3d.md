# 3D realista e aparência

A simulação não depende do 3D. O 3D realista é só **apresentação**: o mesmo resultado numérico, desenhado com modelos mais bonitos. Trocar a aparência de um objeto nunca muda os números do relatório.

No visualizador, a chave **Visual: simples / realista** da barra superior alterna entre formas simples (caixas e cilindros coloridos) e os modelos 3D articulados da biblioteca.

## O campo `visual`

Cada objeto aceita um campo opcional `visual`. Todos os subcampos são opcionais; o que faltar usa o padrão do tipo do objeto.

```ron
(name: "empilhadeira_1", kind: Transporter(speed: 2.0),
    visual: (asset: "empilhadeira", scale: 1.0, yaw: 0.0, color: "#ff8800"))
```

| Campo | Significado |
|---|---|
| `asset` | nome de um modelo da biblioteca, ou o caminho de um arquivo `.glb` relativo ao arquivo do modelo |
| `scale` | escala uniforme (1,0 é o tamanho do próprio modelo, em metros) |
| `yaw` | giro em torno do eixo vertical, em graus, anti-horário visto de cima |
| `color` | cor da pintura principal, em `#rrggbb` |

## Biblioteca de modelos

Os modelos da biblioteca são gerados pelo próprio UTSim, estão em `assets/modelos/` e acompanham o programa. São leves (de 60 a 1.300 triângulos), em medidas reais.

| Nome | O que é | Apelidos aceitos |
|---|---|---|
| `empilhadeira` | empilhadeira com torre, garfos e rodas | `forklift` |
| `paleteira` | paleteira manual | |
| `agv` | veículo autoguiado com luz de estado | |
| `palete` | palete | |
| `caixa` | caixa de papelão | |
| `porta_paletes` | estrutura de porta-paletes | `rack` |
| `esteira` | esteira de rolos | `conveyor` |
| `tanque` | tanque com pernas | `tank` |
| `maquina` | máquina com porta e torre de sinalização | `machine` |
| `bancada` | bancada de trabalho | |
| `operador_simples` | pessoa estilizada | `operador`, `pessoa`, `person` |
| `area` | faixa pintada no chão | |

### Escolha automática

Sem `visual.asset`, o visualizador escolhe pelo papel do objeto:

| Objeto | Modelo padrão |
|---|---|
| Transportador que carrega paletes de um porta-paletes | `empilhadeira` |
| Transportador de uma rede com `agv: true` | `agv` |
| Transportador que trabalha como operador numa máquina | `operador_simples` |
| Outro veículo | `paleteira` numa cena com paletes, senão `agv` |
| `Rack` | `porta_paletes`, nas medidas do objeto |
| `Conveyor` | `esteira`, no comprimento do objeto |
| `Processor` | `maquina` |
| `Combiner` | `bancada` |
| `Queue` | `area`, uma faixa pintada no chão |
| demais | forma simples |

Os itens aparecem como um palete com caixas numa cena com paletes (porta-paletes ou empilhadeiras), e como uma caixa nas demais. Veículos e pessoas param na borda das máquinas, e a empilhadeira que opera um porta-paletes vai ao corredor em frente a ele.

## Ativos de terceiros

A pasta `assets/terceiros/` traz 41 modelos `.glb` prontos, todos com licença **CC0** (sem exigências) ou **CC-BY 3.0** (exige crédito). Veja `assets/terceiros/CREDITOS.md`.

| Grupo | Origem e licença | Conteúdo |
|---|---|---|
| Pessoa | Quaternius, Universal Animation Library 1 e 2 (CC0) | boneco com esqueleto humanoide e 43 animações (parado, andar, correr, empurrar, dirigir, sentar, pegar, operar) |
| Pessoa leve | Kenney, Mini Characters (CC0) | quatro pessoas estilizadas com 32 animações |
| Máquinas e esteiras | Kenney, Factory Kit e Conveyor Kit (CC0) | máquinas, braços robóticos, ponte rolante, caixas, esteiras em módulos, funis |
| Armazenagem | Quaternius, Kenney, MilkAndBanana via Poly Pizza (CC0) | palete, barril, caixote, carrinho, estantes, bancada |
| Empilhadeira e paleteira | KolosStudios via Poly Pizza (**CC-BY 3.0**) | em escala real, mas de malha única (garfos e rodas não se movem separados) |

> **Crédito obrigatório (CC-BY 3.0):** "Forklift" e "Pallet Truck", por KolosStudios, licença CC-BY 3.0, via Poly Pizza. Ao redistribuir o UTSim com esses modelos, mantenha esta linha.

Para usar um deles, aponte o `asset` para o arquivo:

```ron
visual: (asset: "../assets/terceiros/kenney_factory_kit/machine.glb", scale: 1.0)
```

O arquivo `assets/terceiros/catalogo.ron` registra, de cada modelo, as medidas reais em metros, o eixo da frente, a escala sugerida (`escala_real`), o giro necessário (`giro_y_graus`), os nomes das animações e dos nós úteis (por exemplo, a mão onde prender o item carregado).

## Seus próprios modelos 3D (Blender)

Qualquer modelo seu pode substituir os da biblioteca. O fluxo é: modelar ou ajustar no **Blender**, exportar como `.glb`, apontar o `visual.asset` para o arquivo.

### Requisitos do arquivo

- **Formato:** `.glb` (glTF binário), com as **texturas embutidas**. Arquivos `.gltf` com texturas em arquivos separados não são aceitos.
- **Escala real, em metros.** Um modelo de empilhadeira deve medir uns 2 a 3,5 m de comprimento. Se o seu sai grande ou pequeno demais, ajuste `visual.scale` (ou a escala no Blender).
- **Eixo vertical:** Y para cima, que é o padrão do glTF.
- **Frente do modelo:** os modelos do UTSim olham para **+X** do glTF. Muitos modelos de terceiros olham para **+Z**; para esses, gire com `visual.yaw` (comece com `90`) e confira no visualizador.
- **Origem:** coloque a origem do modelo no centro, na altura do chão.

### Passo a passo no Blender

1. **Unidades:** em *Scene Properties > Units*, deixe *Unit System* em *Metric* e *Unit Scale* em 1,0. Assim, 1 unidade do Blender é 1 metro.
2. **Escala:** selecione o modelo e aplique a escala e a rotação (`Ctrl+A > Rotation & Scale`), para que a escala do objeto seja 1.
3. **Frente:** no Blender, o eixo Z é o vertical. Gire o modelo de modo que a frente aponte para **+X** do Blender. O exportador converte: o +X do Blender vira o +X do glTF.
4. **Origem:** posicione a origem (*Object > Set Origin*) no centro da base do modelo.
5. **Exportar:** *File > Export > glTF 2.0 (.glb/.gltf)*. Escolha o formato *glTF Binary (.glb)*. Em *Transform*, deixe marcado *+Y Up*. Em *Mesh*, aplique os modificadores. Marque *Materials: Export* e deixe as texturas embutidas.
6. **Usar:** copie o `.glb` para uma pasta do projeto, de preferência ao lado do `.ron`, e escreva `visual: (asset: "meu_modelo.glb")`.

Se o objeto aparecer virado, ajuste `yaw` (90, -90 ou 180). Se aparecer pequeno ou grande, ajuste `scale`.

### Partes móveis

Os modelos da biblioteca têm nós com nome (`garfos`, `carro`, `roda_de`, `luz`...), e o visualizador gira ou desloca esses nós conforme o movimento (garfos subindo, rodas girando, luz de estado). Para um modelo seu ter partes móveis, **divida a malha em objetos separados**, dê a cada parte um nome e coloque a origem de cada uma na articulação. Os nomes esperados de cada modelo estão em `docs/plano_3d_realista.md`.

Para pessoas animadas, os dois arquivos do Quaternius têm o mesmo esqueleto e os mesmos nomes de ossos, então as animações de um servem ao boneco do outro. Os da Kenney usam outro esqueleto.

### Cor

`visual.color` troca a cor do material chamado `pintura`. Para o seu modelo aceitar a troca de cor, dê o nome `pintura` ao material da parte que deve mudar. O material `luz` é a luz de estado, que o visualizador recolore.

## Limites atuais

- O corredor da empilhadeira num porta-paletes é sempre o lado `-y` dele.
- Os tanques da rede de fluidos ainda usam o cilindro translúcido, não o modelo `tanque`.
- No navegador, `visual.asset` com caminho de arquivo cai no modelo padrão. Só os nomes da biblioteca funcionam por lá.
- Não há sombras nem níveis de detalhe.

## Cena 3D para outros motores

O comando `utsim cena` grava onde cada objeto, item e recurso móvel está, e o que está fazendo, ao longo de uma rodada. É o mesmo conteúdo que o visualizador desenha, num formato que clientes em outros motores (Unreal, Unity, realidade virtual) podem ler.

```bash
utsim cena examples/agvs.ron --fps 10 --ate 600 --saida agvs.jsonl
```

O formato é JSON em linhas (protocolo `utsim-scene/1`): um cabeçalho com o layout, e depois um quadro por linha, em metros, com o eixo Z para cima. Opções: `--rep N` (replicação), `--fps` (quadros por unidade de tempo simulado), `--ate T` (último instante) e `--saida` (sem ela, escreve na saída padrão).
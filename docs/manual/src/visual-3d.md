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
| `operador_simples` | pessoa estilizada, feita de blocos | |
| `area` | faixa pintada no chão | |

### Escolha automática

Sem `visual.asset`, o visualizador escolhe pelo papel do objeto:

| Objeto | Modelo padrão |
|---|---|
| Transportador que serve um porta-paletes (ele, ou o despachante dele, é o `transport` de um `Rack` ou de um objeto ligado a um) | `empilhadeira` |
| Transportador com `agv: true` | `agv` |
| Transportador que é operador de uma máquina (`operators.team`), ou com `asset` `"operador"`, `"pessoa"` ou `"person"` | pessoa animada (veja abaixo); no navegador, `operador_simples` |
| Outro veículo | `paleteira` numa cena com paletes, senão `agv` |
| `Rack` | `porta_paletes`, nas medidas do objeto |
| `Conveyor` | `esteira`, no comprimento do objeto |
| `Processor` | `maquina` |
| `Combiner` | `bancada` |
| `Queue` | `area`, uma faixa pintada no chão |
| demais | forma simples |

Os itens aparecem como um palete com caixas numa cena com paletes (porta-paletes ou empilhadeiras), e como uma caixa nas demais. Veículos e pessoas param na borda das máquinas, e a empilhadeira que opera um porta-paletes vai ao corredor em frente a ele.

## Movimento

- **Pessoas animadas.** As pessoas usam o boneco CC0 da Universal Animation Library, com roupa azul de trabalho, capacete amarelo e colete laranja com faixas. Cada situação tem a sua animação: parada, andando, correndo (acima de 2 m/s), andando com a caixa nas mãos, de pé segurando a caixa, pegando ou entregando o item e trabalhando no painel da máquina. A passada acompanha a velocidade, então o pé não desliza, e a troca de uma animação para outra leva um quarto de segundo. A pose depende só do instante: saltar para um tempo mostra a mesma pose que chegar lá reproduzindo. O item carregado fica entre as mãos. Para usar a pessoa de blocos, escreva `visual: (asset: "operador_simples")`.
- **Carga nos garfos.** Ao guardar um palete, a empilhadeira para no corredor de frente para a célula, os garfos sobem com o palete, ele entra na célula quando os garfos chegam ao nível (no meio da descarga) e os garfos descem vazios. Ao retirar, é o inverso. Em filas, esteiras e máquinas o item também muda de lugar no meio da carga ou descarga.
- **Curvas suaves.** Nas quinas do caminho, veículos e pessoas fazem uma curva em arco de meio metro antes e depois do vértice, girando aos poucos, em vez de virar de uma vez. Os trechos retos e os instantes de chegada não mudam.
- **Sombras.** Com o visual realista, a caixa **Sombras** ao lado da chave liga as sombras do sol sobre o piso e os modelos. Vem ligada no desktop e desligada no navegador, onde pesam mais.

O exemplo `examples/pessoas_carregando.ron` mostra pessoas levando caixas a pé, uma delas correndo de volta.

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

## Câmeras

Em **Simular** (e a primeira pessoa também em Editar), o menu **Câmera** da barra troca o modo de olhar a planta. As teclas `1` a `4` fazem o mesmo, e a câmera desliza do ponto de vista antigo para o novo.

| Modo | Tecla | O que faz |
|---|---|---|
| Órbita | `1` | o de sempre: arrastar desloca, botão direito gira, roda aproxima |
| Seguir | `2` | acompanha um recurso móvel por trás e por cima; o botão direito gira em volta dele e a roda muda a distância (ou use o controle "distância" do quadro no canto da vista) |
| Primeira pessoa | `3` | anda pela planta a 1,6 m do chão |
| Do recurso | `4` | vai no assento do motorista (empilhadeira), no ponto de vista de uma pessoa ou sobre o AGV; o botão direito olha em volta |

- **Seguir um recurso:** clique num recurso móvel (ou escolha na lista de blocos) e use **Seguir este recurso** ou **Ver do recurso** no painel. Sem seleção, as teclas `2` e `4` escolhem o primeiro recurso móvel. Clicar em outro recurso enquanto segue passa a seguir esse. `Esc` sai do modo, e **Enquadrar (F)** volta à órbita e enquadra o modelo.
- **Primeira pessoa:** `W`, `A`, `S`, `D` andam, `Shift` corre, a roda do mouse muda a velocidade (0,5 a 30 m/s; o controle "velocidade" do quadro no canto também). Segure o botão direito para olhar em volta, ou ligue **Travar o mouse** (tecla `L`) para olhar só movendo o mouse; `Esc` solta o mouse e, de novo, sai do modo. O chão é plano (a câmera fica sempre a 1,6 m) e os blocos são obstáculos: você desliza ao longo deles em vez de atravessá-los. Marcas no chão (áreas pintadas) podem ser pisadas. Em Editar, o mouse não edita enquanto você anda. No navegador não há trava do mouse; o botão direito olha em volta.
- Cada modo guarda o que você ajustou (distância, velocidade, para onde olhava).
- Seguir e Do recurso só existem em Simular, e só para recursos móveis (empilhadeiras, AGVs e pessoas).

## Modelos grandes (níveis de detalhe)

Numa planta com muitos objetos, o visualizador troca o que está longe por algo mais barato: o modelo realista vira uma caixa a mais de 45 m, o item realista vira um cubo a mais de 30 m e some a mais de 140 m, e só os 150 rótulos mais próximos (a até 90 m) são escritos. Isso só vale para modelos com pelo menos 100 blocos realistas (ou 400 itens desenhados); uma planta pequena é sempre vista inteira.

`F3` mostra o tempo de quadro, a contagem de objetos em cada nível e os controles dos limiares; `F4` liga e desliga o recurso. Os números medidos estão em `docs/desempenho.md`.

## Limites atuais

- O corredor da empilhadeira num porta-paletes é sempre o lado `-y` dele.
- Os tanques da rede de fluidos ainda usam o cilindro translúcido, não o modelo `tanque`.
- No navegador, `visual.asset` com caminho de arquivo cai no modelo padrão. Só os nomes da biblioteca funcionam por lá, e as pessoas aparecem como `operador_simples`.
- Pessoas e veículos que esperam no mesmo objeto podem aparecer um dentro do outro, porque a simulação os põe no mesmo ponto.
- Não há animação de empurrar, dirigir ou sentar, e a pessoa com a caixa anda com o mesmo passo em qualquer velocidade (não há animação de correr carregando).

## Cena 3D para outros motores

O comando `utsim cena` grava onde cada objeto, item e recurso móvel está, e o que está fazendo, ao longo de uma rodada. É o mesmo conteúdo que o visualizador desenha, num formato que clientes em outros motores (Unreal, Unity, realidade virtual) podem ler.

```bash
utsim cena examples/agvs.ron --fps 10 --ate 600 --saida agvs.jsonl
```

O formato é JSON em linhas (protocolo `utsim-scene/1`): um cabeçalho com o layout, e depois um quadro por linha, em metros, com o eixo Z para cima. Opções: `--rep N` (replicação), `--fps` (quadros por unidade de tempo simulado), `--ate T` (último instante) e `--saida` (sem ela, escreve na saída padrão).

### Ao vivo

Durante uma emulação, a mesma cena pode ser servida por socket, no mesmo relógio do servidor OPC UA, para um cliente de realidade virtual acompanhar a planta enquanto o CLP a comanda:

```bash
utsim emular examples/tutorial_02_linha.ron --cena-porta 7878
```

Sem OPC UA, para desenvolver um cliente, `utsim cena examples/agvs.ron --servir 7878 --velocidade 5` roda a rodada no ritmo do relógio. Em ambos, `--cena-fps` (padrão 20) define os quadros por segundo de relógio e `--cena-endereco` o endereço de escuta (padrão `127.0.0.1`).

- A porta atende TCP (um JSON por linha, como no arquivo) e WebSocket (um JSON por mensagem), ao mesmo tempo e para vários clientes. Cada um recebe o cabeçalho ao conectar e depois os quadros.
- Um cliente lento perde quadros, mas não atrasa a simulação nem os outros.
- O cliente pode mandar `{"tipo":"comando","variavel":"torno.Parar","valor":true}` para parar (ou retomar, com `false`) um objeto, ou `{"tipo":"comando","variavel":"Velocidade","valor":5}` para mudar a velocidade. O efeito aparece nos quadros seguintes (`state` igual a `stopped`). Mensagens do servidor têm `"tipo"` (`aviso`, `fim`); cabeçalho e quadros não, e leitores devem ignorar tipos que não conhecem.
- Exemplos: `cargo run -p utsim-scene --features server --example cliente_cena -- 127.0.0.1:7878 --parar torno` (TCP, imprime um resumo por segundo) e a página `web/cena_ao_vivo.html` (WebSocket, vista de topo; clique num objeto para pará-lo).
- **Segurança:** não há autenticação nem criptografia. Quem alcança a porta vê a planta e pode parar objetos. Use só em rede local confiável.

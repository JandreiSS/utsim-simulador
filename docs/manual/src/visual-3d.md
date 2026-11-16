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
| `mesa` | mesa de trabalho com monitor e teclado (processos administrativos) | `desk`, `mesa_computador` |
| `armario` | armário de pastas com duas portas | `armario_pastas` |
| `bandeja` | bandeja de entrada de dois andares | `bandeja_entrada` |
| `pilha_papeis` | pilha de pastas e papéis: o estoque de casos de uma caixa de entrada | `papeis`, `pilha_casos` |
| `divisoria` | divisória entre mesas | |

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

Uma fila (`Queue`) com `visual: (asset: "esteira")` é desenhada como uma esteira de rolos de 3 m: os itens ficam em cima dos rolos, em fila ao longo da esteira, o mais antigo na ponta de saída; com a primeira linha cheia começa uma segunda camada por cima, e o que não cabe nas duas fica só contado no rótulo (`fila (27) +9`).

Os itens aparecem como um palete com caixas numa cena com paletes (porta-paletes ou empilhadeiras), e como uma caixa nas demais. Veículos e pessoas param na borda das máquinas, e a empilhadeira que opera um porta-paletes vai ao corredor em frente a ele.

### Elementos do Blender (UTSim 3D)

O UTSim 3D procura primeiro os elementos de `assets/modelos3d/`, feitos no Blender, e usa a biblioteca como reserva. A máquina padrão é desenhada como o `cnc` e a bancada como o `posto`. A empilhadeira e o AGV também vêm de lá, no mesmo tamanho. Os itens que circulam continuam as caixas e os paletes da biblioteca. O porta-paletes, a esteira e a faixa da fila continuam da biblioteca, porque as medidas deles vêm do modelo. Para outro elemento, escreva o nome dele em `visual.asset`: `cnc`, `prensa`, `injetora`, `posto`, `robo`, `celula`, `estante`, ou a versão `_medio` de cada um, com mais detalhe. Esses nomes também aparecem na lista **Ativo** do painel, no grupo "Elementos 3D". Numa fila, `espera` desenha a mesa de espera: as peças ficam no tampo, a mais antiga do lado da saída, seis por camada e duas camadas; o que não cabe aparece só no rótulo (+N). Em `agv_medio` e `empilhadeira_medio` o veículo continua o da biblioteca, só com o desenho mais detalhado; `esteira_medio` e `porta_paletes_medio` ficam com a esteira e o porta-paletes nas medidas do modelo.

A ponte rolante (`Crane`) é desenhada com a `ponte_rolante`, esticada até o `size` dela sem deformar as vigas nem as rodas: os trilhos crescem ao longo do caminho de rolamento, as vigas no vão e as colunas na altura. A ponte anda ao longo do caminho, o carro ao longo das vigas e o gancho sobe e desce, seguindo as viagens da simulação. O gancho sobe e desce pela altura do rastro (o evento `Hoist`); num rastro antigo, sem ela, desce a 1,2 m do chão para engatar e para soltar. A peça fica pendurada nele durante o transporte, e enquanto ela está suspensa o chão mostra, embaixo do gancho, o quadrado sob a carga (`load_zone`, o mesmo que a segurança usa para contar as pessoas sob carga suspensa), em amarelo translúcido.

As máquinas se mexem conforme o estado delas na simulação:

| Estado do posto | O que aparece |
|---|---|
| entrando (`entering`) | a porta abre durante o tempo de entrar |
| processando | um ciclo da máquina por peça, com a porta ainda aberta |
| em setup | o ajuste da máquina |
| saindo (`leaving`) | a porta fecha durante o tempo de sair |
| quebrado | a tampa de manutenção abre |
| à espera do reparo, em manutenção | a tampa fica aberta |

Cada elemento traz essa tabela no próprio catálogo (`estados`), então um posto simples ou uma célula mostram só o que têm. Nos outros estados a máquina fica em repouso.

Uma máquina que ninguém opera (sem `operators`) faz um ciclo automático por peça: a porta abre para a peça entrar, fecha enquanto ela é trabalhada e abre de novo para ela sair. A peça aparece onde o elemento a segura: na entrada, no lugar de trabalho e na saída da máquina; na garra do robô, que a leva da entrada à saída; na mesa do AGV, que sobe ao carregar e desce ao descarregar; e no garfo da empilhadeira, que sobe até o vão do porta-paletes.

Os tempos de entrar e sair são os da regra de operação do nó (aba Segurança). O operador fica no lugar de operar do elemento, olhando para a máquina. Enquanto o posto está entrando, ele anda da porta até esse lugar, e na saída faz o caminho de volta.

As pessoas usam os corpos `pessoa_homem` e `pessoa_mulher`, com as mesmas animações do boneco. O corpo, os tons de pele e de cabelo e a altura são escolhidos pelo nome da pessoa, então ficam iguais em toda reprodução. Ela veste os EPIs da lista `ppe` do próprio nó e os dos postos que ela opera, os mesmos que a análise de exposição considera (nomes como `respirador`, `luva` e `bota` também valem), e a camisa pega a cor de `visual.color`. Para escolher o corpo, escreva `visual: (asset: "pessoa_mulher")` ou `"pessoa_homem"`.

## Movimento

- **Pessoas animadas.** As pessoas usam o boneco CC0 da Universal Animation Library, com roupa azul de trabalho, capacete amarelo e colete laranja com faixas. Cada situação tem a sua animação: parada, andando, correndo (acima de 2 m/s), andando com a caixa nas mãos, de pé segurando a caixa, pegando ou entregando o item e trabalhando no painel da máquina. A passada acompanha a velocidade, então o pé não desliza, e a troca de uma animação para outra leva um quarto de segundo. A pose depende só do instante: saltar para um tempo mostra a mesma pose que chegar lá reproduzindo. O item carregado fica entre as mãos. Para usar a pessoa de blocos, escreva `visual: (asset: "operador_simples")`.
- **Motorista na empilhadeira.** Cada empilhadeira com assento (a da biblioteca e `empilhadeira_kolosstudios`) leva o mesmo boneco, sentado, com capacete e colete: com as mãos no volante e a animação de dirigir enquanto a empilhadeira anda, carrega ou descarrega, e sentado em repouso quando está parada, esperando ou bloqueada. Ele gira junto com o veículo e acompanha a escala e o giro do `visual`. É só aparência: a simulação não lê o motorista (o motorista como recurso, que anda até a empilhadeira, fica para depois). No modo **Editar**, o painel do veículo, na seção de aparência, tem a caixa **Mostrar motorista** (ligada por padrão); ela vale só na tela e não é gravada no modelo. A câmera **Do recurso** fica na cabeça do motorista e o esconde dessa vista. A paleteira, o AGV e as pessoas não têm motorista. O ponto do quadril no assento, e o tamanho do boneco, vêm do catálogo (`motorista` em `assets/terceiros/catalogo.ron`) ou, para a empilhadeira da biblioteca, do próprio programa; o assento do modelo `empilhadeira_kolosstudios` é pequeno e o motorista fica menor (80%), com o volante um pouco longe do banco.
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
| Planta (2D) | | só na interface Godot: vista de cima, sem perspectiva, como uma planta baixa; arrastar com o botão direito desloca, a roda aproxima, e a vista nunca gira. Voltar à Órbita devolve o mesmo ângulo de antes |
| Do recurso | `4` | vai nos olhos do motorista (empilhadeira; o motorista fica escondido), no ponto de vista de uma pessoa ou sobre o AGV; o botão direito olha em volta |

- **Seguir um recurso:** clique num recurso móvel (ou escolha na lista de blocos) e use **Seguir este recurso** ou **Ver do recurso** no painel. Sem seleção, as teclas `2` e `4` escolhem o primeiro recurso móvel. Clicar em outro recurso enquanto segue passa a seguir esse. `Esc` sai do modo, e **Enquadrar (F)** volta à órbita e enquadra o modelo.
- **Primeira pessoa:** `W`, `A`, `S`, `D` andam, `Shift` corre, a roda do mouse muda a velocidade (0,5 a 30 m/s; o controle "velocidade" do quadro no canto também). Segure o botão direito para olhar em volta, ou ligue **Travar o mouse** (tecla `L`) para olhar só movendo o mouse; `Esc` solta o mouse e, de novo, sai do modo. O chão é plano (a câmera fica sempre a 1,6 m) e os blocos são obstáculos: você desliza ao longo deles em vez de atravessá-los. Marcas no chão (áreas pintadas) podem ser pisadas. Em Editar, o mouse não edita enquanto você anda. No navegador não há trava do mouse; o botão direito olha em volta.
- Cada modo guarda o que você ajustou (distância, velocidade, para onde olhava).
- **Andar (UTSim 3D, no Simular):** a tecla `F` põe você dentro da fábrica, na altura dos olhos (1,7 m), de frente para o ponto que a órbita olhava. `W`, `A`, `S` e `D` andam e o mouse olha, com o cursor preso. `Shift` corre, `E` e `Q` (ou `Espaço` e `C`) sobem e passam a voar, e `V` alterna entre voar e voltar ao chão. A roda muda a velocidade, de 0,5 a 40 m/s. O clique na mira (o `+` no meio da vista) abre o painel ao vivo do que está à frente. `Esc` sai e devolve a câmera de antes, e `Home` sai e enquadra o modelo. Andando, você atravessa os objetos: não há colisão.
- **Cor do chão (UTSim 3D):** no painel do modelo (nada selecionado), em Piso e corredores, ligue "Cor do chão" e escolha a cor. Ela fica gravada no modelo (`floor_marking.floor_color`), e a grade muda de tom para continuar visível.
- Seguir e Do recurso só existem em Simular, e só para recursos móveis (empilhadeiras, AGVs e pessoas).

## Modelos grandes (níveis de detalhe)

Numa planta com muitos objetos, o visualizador troca o que está longe por algo mais barato: o modelo realista vira uma caixa a mais de 45 m, o item realista vira um cubo a mais de 30 m e some a mais de 140 m, e só os 150 rótulos mais próximos (a até 90 m) são escritos. Isso só vale para modelos com pelo menos 100 blocos realistas (ou 400 itens desenhados); uma planta pequena é sempre vista inteira.

`F3` mostra o tempo de quadro, a contagem de objetos em cada nível e os controles dos limiares; `F4` liga e desliga o recurso. Os números medidos estão em `docs/desempenho.md`.

## Limites atuais

- O corredor da empilhadeira num porta-paletes é sempre o lado `-y` dele.
- Os tanques da rede de fluidos ainda usam o cilindro translúcido, não o modelo `tanque`.
- No navegador, `visual.asset` com caminho de arquivo cai no modelo padrão. Só os nomes da biblioteca funcionam por lá, e as pessoas aparecem como `operador_simples`.
- Pessoas e veículos que esperam no mesmo objeto podem aparecer um dentro do outro, porque a simulação os põe no mesmo ponto. Quem trabalha junto no mesmo posto (operadores por peça) já fica lado a lado, a 0,7 m, na frente da máquina. Os que não cabem na largura dela ficam nas laterais, um de cada lado, da frente para o fundo, virados para a máquina.
- Não há animação de empurrar, dirigir ou sentar, e a pessoa com a caixa anda com o mesmo passo em qualquer velocidade (não há animação de correr carregando).

## Cena 3D para outros motores

O comando `utsim cena` grava onde cada objeto, item e recurso móvel está, e o que está fazendo, ao longo de uma rodada. É o mesmo conteúdo que o visualizador desenha, num formato que o cliente Godot (3D, realidade virtual e aumentada) pode ler.

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

# Plano: 3D realista e realidade virtual

Base: a pesquisa em `docs/pesquisa_3d_animacao.md` (simuladores comerciais, simuladores comerciais, simuladores livres) e a cena
independente de motor (`utsim-scene`, `docs/arquitetura.md` seção 4p).

## Ideia central

A simulação não muda. O 3D realista é **apresentação**: cada motor lê a mesma cena
(`utsim-scene/1`), com pose, papel e movimento semântico de cada entidade, e decide
como desenhar. Por isso dá para ter, ao mesmo tempo:

- o visualizador Bevy (desktop e navegador), com modelos glTF e animações;
- um cliente Unreal com visual de alta qualidade e realidade virtual pelo OpenXR;
- no futuro, realidade virtual no próprio Bevy, quando o suporte a OpenXR acompanhar a
  versão do Bevy que usamos.

```
modelo .ron ─► simulação ─► rastro ─► utsim-scene ─┬─► Bevy (desktop / web)
                                                   ├─► utsim cena (arquivo .jsonl) ─► Unreal (VR)
                                                   └─► socket ao vivo (emulação) ─► Unreal / VR
```

## Como os objetos 3D serão feitos

| Tipo | Como | Por quê |
|---|---|---|
| Empilhadeira, paleteira, AGV, porta-paletes, esteira, palete, caixa, tanque | **Gerados por código** (gerador de `.glb` em Rust, peças articuladas como nós separados: chassi, torre, carro dos garfos, garfos, rodas) | Licença GPL sem dúvida; medidas reais e paramétricas (altura dos níveis do porta-paletes, comprimento da esteira); o motor só move os nós (sem esqueleto) |
| Pessoas (operadores) | **Personagens CC0 já animados** (Quaternius Universal Animation Library 2: parado, andando, andando carregando) | Animação de esqueleto é cara de fazer à mão; CC0 permite redistribuir junto com o UTSim |
| Cenário decorativo (máquinas específicas, paredes, docas) | `.glb` do usuário no campo `visual.asset`; opcionalmente gerados com IA | Não precisa articular; cada fábrica é diferente |

**Sobre o ArtCraft:** serve para gerar peças **estáticas** de cenário a partir de imagem
(imagem → malha 3D), por exemplo uma máquina específica da sua fábrica. Não serve para os
recursos móveis: a malha gerada vem numa peça só, sem garfos ou rodas separados e sem
esqueleto. As gerações usam créditos ou chaves de API de terceiros, e a licença do
resultado depende do provedor. Por isso esses arquivos ficam fora do repositório (pasta
local do modelo) até a licença ser conferida.

## Como cada movimento vira animação

| Entidade | Movimento (`Motion`) | O que o motor faz |
|---|---|---|
| Pessoa | `Moving{speed}` | ciclo de caminhada, velocidade do clipe proporcional à velocidade (pés sem deslizar) |
| Pessoa | carregando (`lift` > 0, item com `parent`) | clipe "andar carregando", item preso às mãos |
| Pessoa | `Working` | clipe de trabalho na bancada |
| Empilhadeira | `Moving` | rodas giram pela distância; direção pela tangente do caminho |
| Empilhadeira | `Loading/Unloading{progress}` | aproximação final (offset travel) e garfos sobem até `lift` (altura do nível do porta-paletes) |
| AGV | `Moving` / `Blocked` | segue a rede; luz de estado (verde, laranja) |
| Tanque | `fill` | altura do líquido |

## Preparo para realidade virtual (já feito no contrato)

1. Escala real em metros, eixo Z para cima, convenções fixas no protocolo.
2. Todo o cálculo de posição é puro (`utsim-scene`), sem depender da janela.
3. A saída tem um ponto único (`SceneSink`): arquivo, socket ou runtime de RV.
4. O protocolo é versionado (`utsim-scene/1`).

Ainda falta, nas fases abaixo:
- câmera em primeira pessoa na altura dos olhos;
- painéis de informação dentro do mundo 3D, em vez de só na interface 2D;
- meta de 90 quadros por segundo, com instâncias e níveis de detalhe.

## Fases propostas (estimativa em horas Opus 5.5)

| Fase | Conteúdo | Estimativa |
|---|---|---|
| A3D-1 | Cena independente de motor (`utsim-scene`), `utsim cena`, visualizador usando a cena (**em andamento**) | 3–4 h |
| A3D-2 | Gerador de `.glb` articulados (empilhadeira, paleteira, AGV, porta-paletes, esteira, palete, caixa, tanque) e carga de glTF no Bevy pelo `visual.asset` | 4–6 h |
| A3D-3 | Pessoas animadas (CC0), máquina de estados visual, itens presos às mãos e garfos, garfos subindo até o nível, rodas, giro suave nas curvas | 5–8 h |
| A3D-4 | Câmeras: seguir um recurso, primeira pessoa (andar pela planta); instâncias e níveis de detalhe para modelos grandes | 3–4 h |
| A3D-5 | Cliente Unreal + realidade virtual (OpenXR), lendo `utsim-scene/1` de arquivo e depois ao vivo | 6–10 h (sessão separada) |
| A3D-6 | Fluxo de cena ao vivo (socket) a partir da emulação, para RV em tempo real conectada ao CLP | 2–3 h |

## Pedidos prontos para outras sessões

### Sessão 1: cliente Unreal com realidade virtual (rodar depois que A3D-1 estiver integrada)

> Crie, numa pasta **fora** do repositório do UTSim (por exemplo
> `C:\Users\Jandrei\Documents\Codes\UTSimUnreal`), um projeto Unreal Engine 5 (C++),
> a partir do template de realidade virtual (OpenXR), chamado `UTSimUnreal`. Ele
> reproduz em 3D uma rodada do UTSim, lendo o fluxo de cena `utsim-scene/1` descrito em
> `C:\Users\Jandrei\Documents\Codes\UTSim\crates\utsim-scene\src\lib.rs` e em
> `docs/arquitetura.md`, seção 4p. Gere um arquivo de exemplo com
> `utsim cena examples/agvs.ron --fps 20 --saida agvs.jsonl`.
> 1. Um plugin C++ `UTSimLink` lê o `.jsonl`:
>    - a primeira linha é o `Header`;
>    - cada linha seguinte é um `Frame`;
>    - converte metros com Z para cima em centímetros do Unreal: `(100x, -100y, 100z)`, e
>      `yaw` em radianos anti-horário em graus do Unreal com o sinal invertido.
> 2. Um ator `AUTSimPlayer` cria um ator por entidade da cena estática e, a cada frame
>    do jogo, interpola entre os dois quadros que cercam o tempo de reprodução.
>    Mapeamento:
>    - `MobileRole::Person` → personagem com Animation Blueprint (parado, andando,
>      carregando, trabalhando; velocidade vinda de `Motion::Moving`);
>    - `Forklift` → empilhadeira com componente do garfo na altura `lift`;
>    - `Agv` e `Vehicle` → veículos;
>    - itens → caixas presas ao `parent`;
>    - tanques → cilindro com nível `fill`.
>    Use primitivas e o Mannequin do Unreal por enquanto. Os `.glb` do UTSim podem ser
>    importados depois.
> 3. Controles: reproduzir, pausar e velocidade, com um widget 3D na mão (RV) e teclado
>    no desktop. Na RV, teleporte e movimento livre. O jogador começa na altura dos olhos,
>    a 160 cm.
> 4. Deixe preparado, sem implementar, um modo ao vivo: um socket TCP que recebe as mesmas
>    linhas JSON.
>
> Teste no desktop (sem óculos) com `agvs.jsonl`. O projeto Unreal fica fora do
> repositório GPL, porque o conteúdo da Epic segue a licença do Unreal. A comunicação é
> só pelo protocolo.

### Sessão 2: gerador de modelos articulados (A3D-2; pode rodar depois que A3D-1 estiver integrada)

> No repositório UTSim (`C:\Users\Jandrei\Documents\Codes\UTSim`, branch `feat/fase-1`),
> crie a crate `crates/utsim-assets` (binário `utsim-assets`), que gera arquivos `.glb`
> por código, sem programas externos, para a biblioteca embutida de aparências:
> - `empilhadeira`: chassi, torre, carro dos garfos e garfos como nós separados, com nomes
>   fixos para o motor mover;
> - `paleteira`, `agv`, `palete`, `caixa`, `porta_paletes` (parametrizado por baias e
>   níveis), `esteira` (por comprimento) e `tanque`.
>
> Use medidas reais em metros, Z para cima no modelo, e exporte em Y-up, como o glTF
> exige. Documente os nomes dos nós. No visualizador (`crates/utsim-viewer`), carregue o
> `.glb` quando o objeto tiver `visual.asset` ou pelo papel padrão
> (`MobileRole`/tipo do objeto), e mova os nós a partir de `utsim_scene::Pose`: o garfo
> pela `lift` e as rodas pela distância. Teste com os roteiros do visualizador
> (capturas), em `examples/armazem.ron` e `examples/agvs.ron`.

## Pendências que dependem de você

- Autorizar o download dos personagens CC0 da Quaternius (Universal Animation Library 2),
  para versioná-los em `assets/terceiros/` com a licença.
- Instalar o Unreal Engine 5 pelo Epic Launcher, se quiser o cliente Unreal. Hoje há só
  as pastas de configuração e o launcher, sem o motor instalado.

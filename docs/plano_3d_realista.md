# Plano: 3D realista e realidade virtual

Base: a cena
independente de motor (`utsim-scene`, `docs/arquitetura.md` seção 4p).

## Ideia central

A simulação não muda. O 3D realista é **apresentação**: cada motor lê a mesma cena
(`utsim-scene/1`), com pose, papel e movimento semântico de cada entidade, e decide
como desenhar. Por isso dá para ter, ao mesmo tempo:

- o visualizador Bevy (desktop e navegador), com modelos glTF e animações;
- o cliente Godot, com visual de alta qualidade, realidade virtual e realidade aumentada
  pelo OpenXR nativo do Godot.

```
modelo .ron ─► simulação ─► rastro ─► utsim-scene ─┬─► Bevy (desktop / web)
                                                   ├─► utsim cena (arquivo .jsonl) ─► Godot (3D, RV, RA)
                                                   └─► socket ao vivo (emulação) ─► Godot ao vivo
```

## Como os objetos 3D serão feitos

| Tipo | Como | Por quê |
|---|---|---|
| Empilhadeira, paleteira, AGV, porta-paletes, esteira, palete, caixa, tanque | **Gerados por código** (gerador de `.glb` em Rust, peças articuladas como nós separados: chassi, torre, carro dos garfos, garfos, rodas) | Sem dúvida de licença (código nosso); medidas reais e paramétricas (altura dos níveis do porta-paletes, comprimento da esteira); o motor só move os nós (sem esqueleto) |
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
| A3D-2 | Gerador de `.glb` articulados (empilhadeira, paleteira, AGV, porta-paletes, esteira, palete, caixa, tanque) e carga de glTF no Bevy pelo `visual.asset` (**feito**, ver abaixo) | 4–6 h |
| A3D-3 | Pessoas animadas (CC0), máquina de estados visual, itens presos às mãos e garfos, garfos subindo até o nível, rodas, giro suave nas curvas, sombras (**feito**, ver abaixo) | 5–8 h |
| A3D-4 | Câmeras: seguir um recurso, primeira pessoa (andar pela planta), do recurso; níveis de detalhe para modelos grandes (**feito**: `camera_modes.rs`, `lod.rs`, `docs/arquitetura.md` 4p e `docs/desempenho.md`; os itens já compartilham malha e material, então não foram precisas instâncias) | 3–4 h |
| A3D-5 | Cliente Godot: 3D, realidade virtual e aumentada (OpenXR), lendo `utsim-scene/1` de arquivo e ao vivo | ver o plano Godot |
| A3D-6 | Fluxo de cena ao vivo (socket) a partir da emulação, para RV em tempo real conectada ao CLP (**feito**: `--cena-porta`, `utsim cena --servir`, TCP e WebSocket, comandos de volta) | 2–3 h |

## Biblioteca de modelos (A3D-2, feita)

A crate `crates/utsim-assets` gera os modelos por código e grava `.glb` (glTF binário):

- só malhas montadas em Rust (caixas, cilindros, esferas), sem programa externo nem download;
- medidas reais em metros, cores lisas PBR, de 60 a 1.300 triângulos por modelo;
- modelados com Z para cima e `+x` para a frente, gravados em Y para cima: `(x, y, z)` vira `(x, z, -y)`;
- os modelos olham para `+X` do glTF, a convenção de direção do UTSim. Um motor que espera
  `+Z` para a frente gira o modelo -90° em torno da vertical;
- cada peça móvel é um nó com nome e pivô na articulação, então o motor só muda a
  transformação do nó, sem esqueleto;
- o material `pintura` é a pintura principal (trocada por `visual.color`) e `luz` é a luz de
  estado que o motor recolore.

`cargo run -p utsim-assets` regrava os padrões em `assets/modelos/` (versionados, cerca de
320 KB no total). `utsim-assets --listar` mostra nós, triângulos e medidas, e
`--porta-paletes BAIAS,NIVEIS,LARGURA,ALTURA` ou `--esteira COMPRIMENTO` geram variantes. Um
teste regera os arquivos e compara byte a byte (os números são arredondados numa grade de
2^-14 m e o JSON é escrito numa ordem fixa, então o resultado é igual em qualquer sistema).
Outro teste lê cada `.glb` com a crate `gltf` e confere os nós e as medidas.

| Modelo | Medidas (m, x × y × z) | Nós (pivô) |
|---|---|---|
| `empilhadeira` | 3,43 × 1,10 × 2,25 (2,30 até a face dos garfos) | `chassi`; `cabine` (banco e proteção); `luz` (giroflex); `torre` (base da torre); `carro` (filho de `torre`; a altura da origem é a altura do topo dos garfos, 0,10 em repouso); `garfos` (filho de `carro`); `roda_de`, `roda_dd`, `roda_te`, `roda_td` (eixo; altura do pivô = raio) |
| `paleteira` | 1,74 × 0,54 × 1,25 | `chassi`; `garfos` (topo a 0,085); `timao` (articulação na bomba); `roda_direcao`, `roda_e`, `roda_d` (eixo) |
| `agv` | 1,13 × 0,81 × 0,35 | `chassi` (para-choque amarelo na frente, escuro atrás); `luz` (faixa de luz de estado); `roda_e`, `roda_d` (eixo) |
| `palete` | 1,20 × 1,00 × 0,144 | só a raiz (estático) |
| `caixa` | 0,40 × 0,30 × 0,30 | só a raiz (material `papelao`) |
| `porta_paletes` | baias × largura da baia, profundidade, níveis × altura do nível + viga | `estrutura` (montantes azuis com travamento, vigas laranja; o nível k apoia a carga em k × altura + viga) |
| `esteira` | comprimento × largura × altura dos rolos | `estrutura`, `rolos` |
| `tanque` | 2r × 2r × (pernas + altura + teto) | `estrutura`, `casco` (base do casco) |
| `maquina` | 1,80 × 1,52 × 2,20 | `chassi`; `porta`; `luz` (torre de sinalização) |
| `bancada` | 1,50 × 0,75 × 1,50 | `estrutura` |
| `operador_simples` | 0,30 × 0,64 × 1,76 | `tronco` (quadril); `cabeca` (pescoço, filho de `tronco`); `braco_e`, `braco_d` (ombros, filhos de `tronco`); `perna_e`, `perna_d` (quadril) |
| `area` | comprimento × largura × 0,008 | `piso` |

Braços, pernas e rodas giram em torno do eixo lateral (`Z` do glTF): ângulo positivo leva um
membro para a frente, e uma roda que anda para a frente gira `-distância / raio`.

**No visualizador** ("Visual: simples / realista" na barra; começa em realista):
- os `.glb` da biblioteca são gerados em memória pela própria `utsim-assets` (os mesmos bytes
  dos arquivos versionados) e registrados na fonte `embedded://` do Bevy. Assim o desktop e o
  navegador carregam do mesmo jeito, sem acesso a arquivo nem HTTP. Um `.glb` do usuário em
  `visual.asset` é lido do disco, relativo ao arquivo do modelo (só no desktop);
- escolha padrão: empilhadeira, AGV e pessoa pelo `MobileRole`; veículo genérico vira
  paleteira numa cena com paletes (porta-paletes ou empilhadeiras) e AGV nas demais;
  porta-paletes pelo `RackParams`, esteira pelo comprimento, `Processor` vira máquina,
  `Combiner` vira bancada, `Queue` vira área pintada no chão. Os demais tipos ficam com a
  forma simples. Itens: palete com oito caixas numa cena com paletes, senão uma caixa;
- os nós seguem a `Pose`: `carro` (e `garfos`) na altura `lift`, rodas pela distância
  percorrida, luz do AGV pelo movimento (verde andando, laranja bloqueado, vermelho parado,
  azul carregando ou descarregando, branco ocioso), luz das máquinas e da empilhadeira pela
  cor do estado, operador com passo de 0,7 m sem deslizar o pé, braços para a frente
  carregando, parado quando ocioso e um movimento curto trabalhando;
- itens carregados vão nos garfos (na altura `lift`), nas mãos ou no deque do AGV;
- como a rede leva o veículo até o centro do bloco, o visual ajusta a posição: a empilhadeira
  que carrega ou descarrega num porta-paletes vai para o corredor na frente da célula, de
  frente para ela; veículos e pessoas param na borda das máquinas; o operador trabalha de pé
  na frente da máquina.

Roteiros de conferência: `visual_realista_armazem.ron` (com
`examples/centro_de_distribuicao.ron`), `visual_realista_agvs.ron` (com `examples/agvs.ron`)
e `visual_realista_operadores.ron` (com `examples/celula_com_operadores.ron`).

Lacunas conhecidas:
- o corredor da empilhadeira é sempre o lado `-y` do porta-paletes;
- o tanque existe na biblioteca, mas os tanques da rede de fluidos ainda usam o cilindro
  transparente;
- `.glb` do usuário com texturas externas (`.gltf` com arquivos ao lado) não é aceito, e no
  navegador `visual.asset` com arquivo cai no modelo padrão.

## Movimento realista (A3D-3, feito)

- **Pessoas animadas por esqueleto.** As pessoas usam o boneco do `pessoa_operador`, com os
  clipes dos dois arquivos do Quaternius (mesmo esqueleto, retarget pelo nome dos ossos,
  conferido ao montar o grafo de animação). Clipes: `Idle_Loop` parado, bloqueado ou parado
  por quebra; `Walk_Loop` andando, `Jog_Fwd_Loop` acima de 2 m/s; `Walk_Carry_Loop` andando
  carregando; parado carregando, as pernas de `Idle_Loop` com os braços de
  `Walk_Carry_Loop` (máscaras de ossos); `PickUp_Table` na coleta e na entrega, no ritmo do
  `progress`; `Interact` trabalhando. Transição de 0,25 s. O tempo de cada clipe sai da cena:
  os de andar pela distância percorrida (passada medida no clipe pela velocidade do pé de
  apoio: 1,31 m por volta no andar e no andar carregando, 5,44 m na corrida, na escala do
  arquivo), a coleta pelo `progress`, os outros pelo relógio simulado. Saltar no tempo dá a
  mesma pose. O item fica entre as mãos. Roupa de trabalho por tingimento dos dois
  materiais, capacete e colete presos aos ossos `Head` e `spine_02`. Sem os arquivos (navegador)
  ou se falharem, volta o `operador_simples`.
- **Carga saindo dos garfos na hora certa.** Na cena (`utsim-scene`), o item troca de mãos no
  meio da janela de carga ou descarga, quando os garfos estão no nível: descarregando, sobe
  nos garfos, entra na célula lá em cima e os garfos descem vazios; coletando, o inverso. Vale
  também para filas, esteiras e máquinas. A cena põe a carga da empilhadeira nos garfos e a
  própria empilhadeira no corredor de frente para a célula (antes só o visualizador fazia
  isso), então o fluxo `utsim-scene/1` também mostra a troca sem salto.
- **Giro suave.** Função pura `turning::sample`: arco tangente nos últimos e primeiros 0,5 m em
  volta de cada vértice, inclusive entre trechos percorridos sem parar; posição e direção
  contínuas, pedaços retos inalterados.
- **Sombras.** Sol com sombras em cascata dimensionadas pela cena, piso que recebe sombras,
  luz ambiente mais suave com sombras, caixa "Sombras" ao lado da chave "Visual" (desligada
  por padrão no navegador). No `centro_de_distribuicao.ron`, build de release: 548 quadros por
  segundo sem sombras e 448 com elas (com os níveis de detalhe do A3D-4).

Roteiros de conferência: `a3d3_pessoas.ron` (com `examples/celula_com_operadores.ron`),
`a3d3_pessoas_carregando.ron` (com o novo `examples/pessoas_carregando.ron`, em que pessoas
levam caixas) e `a3d3_empilhadeira.ron` (com `examples/centro_de_distribuicao.ron`). O passo
`MedirQuadros(segundos)` dos roteiros grava a taxa de quadros no `log.txt`.

Lacunas do A3D-3:
- não há clipe de correr carregando: acima de 2 m/s com carga, o andar carregando acelera;
- pessoas paradas no mesmo objeto podem se sobrepor (o rastro as põe no centro do bloco);
- o capacete e o colete são formas simples presas a ossos, não roupa com pele;
- no modo ao vivo, a quina entre dois trechos só se arredonda quando o segundo trecho chega.

## Ativos de terceiros

A pasta `assets/terceiros/` guarda 41 modelos `.glb` prontos (cerca de 18 MB), só com licença
CC0 ou CC-BY 3.0, para trocar ou complementar os modelos procedurais. Créditos e licenças
estão em `assets/terceiros/CREDITOS.md` e no `LICENCA.txt` de cada pacote; as medidas
reais, os eixos, a escala sugerida, os clipes e os nós úteis de cada arquivo estão em
`assets/terceiros/catalogo.ron`. O teste `crates/utsim-assets/tests/terceiros.rs` abre cada
arquivo do catálogo e confere existência, clipes, nós, medidas e licença.

O que veio:

| Grupo | Origem (licença) | Conteúdo |
|---|---|---|
| Pessoa | Quaternius, Universal Animation Library 1 e 2, versão Standard (CC0) | Boneco com esqueleto humanoide de 65 ossos e 43 clipes em cada arquivo. Mapa para o `Motion`: `Idle` = `Idle_Loop`; `Moving` = `Walk_Loop` (ou `Jog_Fwd_Loop`, `Sprint_Loop`); carregando = `Walk_Carry_Loop` (no arquivo 2); `Working` = `Interact` ou `Fixing_Kneeling`; coleta = `PickUp_Table`; empurrar = `Push_Loop`; dirigindo = `Driving_Loop`; sentado = `Sitting_Idle_Loop`. Os dois arquivos têm o mesmo esqueleto e os mesmos nomes de ossos, então os clipes de um servem ao boneco do outro. |
| Pessoa leve | Kenney, Mini Characters (CC0) | Quatro pessoas estilizadas de 7 ossos e 32 clipes (`idle`, `walk`, `sprint`, `pick-up`, `holding-both`, `interact-right`, `drive`, `sit`). Para carregar, toque `walk` e, por cima, a camada de braços `holding-both` (ela só mexe nos braços). Medem 0,7 m, então use a escala 2,6. |
| Máquinas e esteiras | Kenney, Factory Kit e Conveyor Kit (CC0) | Máquinas, braços robóticos com nós `element-a`... e `claw-a/b`, ponte rolante, caixas, esteiras em módulos de 1 m, funis, pistão e alavanca com clipes `toggle`. Estáticos, mas com nós para articular. |
| Armazenagem | Quaternius, Kenney, MilkAndBanana via Poly Pizza (CC0) | Palete, barril, caixote, carrinho, estantes e bancada. |
| Empilhadeira e paleteira | KolosStudios via Poly Pizza (**CC-BY 3.0**, crédito obrigatório) | Modelos em escala de metros, mas de malha única: garfos e rodas não são nós separados. |

Convenções: todos os arquivos são glTF padrão, Y para cima. Pessoas olham para `+Z` (conferido
pela posição do pé e da ponta do pé no esqueleto do Quaternius); nos outros pacotes `+Z` é só
a convenção, marcada como `frente_verificada: false` no catálogo. Os modelos do UTSim olham
para `+X`, então o catálogo traz `giro_y_graus` (90). A escala vem de `escala_real`
(`tamanho_real_m = tamanho_m * escala_real`), e `centro_m` mostra os arquivos cuja origem fica
longe do centro (empilhadeira e paleteira), que o motor precisa recentrar.

Lacunas conhecidas, sem item CC0 encontrado: empilhadeira ou paleteira com garfos separados
(as duas obtidas são CC-BY e de malha única), AGV, carrinho de mão industrial de duas rodas,
torno, prensa e fresadora CNC detalhadas (use as máquinas do Kenney como substitutas). Também
ficou de fora o pacote Universal Base Characters (CC0): vem em `.gltf` com texturas separadas
de vários MB e sem `.glb`, e o boneco do próprio Universal Animation Library já cobre o uso.
Rejeitados por licença ou origem: Mixamo, Sketchfab com licença padrão e modelos gerados por
IA sem termos claros (por exemplo, os da Meshy).

Como personalizar:

1. **Trocar um modelo**: copie o seu `.glb` para uma pasta do projeto e aponte `visual.asset`
   do objeto para ele. O arquivo precisa ser `.glb` (texturas embutidas), com a malha em
   metros e Y para cima.
2. **Acrescentar**: ponha o `.glb` em `assets/terceiros/<pacote>/`, acrescente um
   `LICENCA.txt` com a licença e o endereço, uma linha em `CREDITOS.md` e uma entrada em
   `catalogo.ron`. O teste `terceiros` recusa arquivos sem catálogo, sem licença ou com
   licença que não seja CC0 ou CC-BY, e o total precisa ficar abaixo de 40 MB.
3. **Editar no Blender**: importe o `.glb`, ajuste forma, cores ou esqueleto e exporte de novo
   como glTF binário (`.glb`) com "Y para cima". Para separar garfos ou rodas de uma malha
   única, divida a malha em objetos e dê a cada um um nome e um pivô na articulação, como nos
   modelos de `assets/modelos`. Para trocar o clipe usado por um movimento, edite o
   `mapa_movimento` em `catalogo.ron`.
4. **Reaproveitar clipes**: um clipe só funciona em um esqueleto com os mesmos nomes de ossos.
   Os dois arquivos do Quaternius compartilham o esqueleto; os do Kenney usam outro.

## Pedidos prontos para outras sessões

### Sessão 2: gerador de modelos articulados (A3D-2; pode rodar depois que A3D-1 estiver integrada)

> No repositório UTSim (branch `feat/fase-1`),
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

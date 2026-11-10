# Elementos 3D gerados no Blender

Os elementos da fábrica (máquinas, posto, robô, célula, esteira, veículos, estantes e
cargas) são gerados por scripts Python no Blender, sem janela, e versionados em
`assets/modelos3d/`. Os modelos em Rust de `assets/modelos/` (`utsim-assets`) continuam
valendo como reserva e para os tamanhos por parâmetro.

## Como gerar

```bash
bash scripts/blender.sh            # todos
bash scripts/blender.sh --so cnc   # só alguns
```

O script acha o Blender (`BLENDER`, `PATH` ou a pasta padrão do Windows) e roda
`blender -b --factory-startup -P tools/blender/gerar.py -- --saida assets/modelos3d`.
Para conferir o visual, `tools/blender/vitrine.py` monta uma pequena linha com pessoas
e renderiza uma imagem.

Cada execução grava exatamente os mesmos bytes (mesma versão do Blender). Os testes de
`crates/utsim-assets/tests/modelos3d.rs` conferem os arquivos versionados sem o Blender.
Com `UTSIM_BLENDER=1`, o `scripts/verificar.sh` gera tudo de novo e falha se algo mudar.

## Arquivos

- `<nome>.glb`: detalhe baixo, o padrão (até 64 KB e 1500 triângulos cada).
- `<nome>_medio.glb`: arestas chanfradas e cilindros mais redondos.
- `catalogo.json`: para cada arquivo, o tamanho, os nós, os pontos (posição e giro), os
  clipes (em laço ou não, e os nós que movem) e as medidas que a ponte usa (`dados`).

Sem compressão Draco ou meshopt: o `GLTFDocument` do Godot não as lê. A leveza vem do
nível de detalhe baixo e de malhas sem textura (cerca de 1,4 MB para os 28 arquivos).

## Convenções

As convenções são as mesmas do `utsim-assets`:

- metros, Z para cima, `+x` é a frente e `+y` a esquerda;
- a origem fica no piso, no centro da base;
- o glTF fica com Y para cima;
- a raiz tem o nome do elemento;
- cada parte móvel é um nó com o pivô na junta;
- o material `pintura` recebe a cor do nó;
- o material `luz` é a luz de estado que a interface recolore.

Nas máquinas e no posto, o lado de trabalho fica em `-y`.

### Pontos (nós vazios)

A posição do ponto é o lugar. O `+x` local do ponto é para onde se olha ou para onde a
peça anda.

| Ponto | Para que serve |
|---|---|
| `ponto_operador` | onde a pessoa fica para operar (olha para a máquina) |
| `ponto_porta` | por onde a pessoa chega (na célula, fora do portão) |
| `ponto_peca` | onde a peça fica durante o processo |
| `ponto_entrada`, `ponto_saida` | onde a peça chega e de onde sai (ligam um elemento ao próximo) |
| `ponto_garra` | ponta dos dedos do robô (a base da peça presa) |
| `ponto_carga` | garfo da empilhadeira, mesa do AGV |
| `ponto_vao_<b>_<n>` | vão `b`, nível `n` do porta-paletes e da estante (`+x` aponta para o corredor) |

### Clipes

Cada clipe dura 1 s (24 quadros). A interface posiciona o clipe pela **fase**, de 0 a 1,
do estado que a simulação manda. Não há evento solto: a fase sai do intervalo do estado
no trace (`(t - início) / (fim - início)`). Os clipes em laço repetem a cada unidade de
fase. Assim o 3D mostra o que a simulação está fazendo em qualquer instante, mesmo ao
pular no tempo.

| Elemento | Clipe | Laço | Gatilho na simulação |
|---|---|---|---|
| cnc, prensa, injetora | `ciclo` | sim | processando |
| | `setup` | sim | em setup |
| | `porta_abrir` | não | operador entrando (0 → 1) e saindo (1 → 0) |
| | `quebra` | não | quebrado (abre a tampa de manutenção) |
| posto | `ciclo` | sim | processando |
| robo | `pegar`, `levar`, `soltar` | não | carregar, transportar e descarregar a peça |
| | `ciclo` | sim | os três seguidos, para um robô sempre ocupado |
| celula | `porta_abrir` | não | pessoa entrando ou saindo da célula |
| esteira | `rolar` | sim | fase = distância / (2π · `raio_rolo`) |
| empilhadeira | `garfo_subir` | não | altura do garfo / `garfo_curso` |
| agv | `mesa_subir` | não | carregando ou descarregando |

Parado, livre e bloqueado aparecem pela cor da `luz`, como hoje.

### Interações

A peça passa de um elemento a outro pelos pontos:

- da esteira, sai em `ponto_saida`;
- na garra, vai presa em `ponto_garra` entre `pegar` e `soltar`;
- na máquina, entra em `ponto_entrada` e fica em `ponto_peca`.

O palete vai no `ponto_carga` do garfo e fica no `ponto_vao_*` do porta-paletes. O
operador anda até `ponto_porta`, abre a porta, vai a `ponto_operador`, opera e sai pelo
mesmo caminho. Fazer a peça, o palete e a pessoa seguirem esses pontos no 3D é da ponte
e da interface (próximas etapas).

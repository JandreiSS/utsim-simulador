# Manufatura discreta

A quinta **linha especializada** do UTSim estuda fábricas de peças: ordens de produção que
passam por máquinas, com mistura de produtos, troca de produto, quebras, operadores, inspeção e
produção puxada por cartões. A linha usa o mesmo núcleo, o mesmo `.ron` e as mesmas
replicações do resto do programa.

A linha traz:

- uma **biblioteca** de 10 predefinições em `linhas/manufatura/biblioteca.ron`: liberação de
  ordens, fila de ordens com a regra de despacho, máquina, máquina com troca de produto,
  montagem, supermercado, laço de cartões (kanban e CONWIP), expedição, equipe de operadores e
  um operador;
- um **vocabulário** (o item é a **ordem**, a saída é a **expedição**, a pessoa é o
  **operador**), em `linhas/manufatura/vocabulario.ron`;
- uma peça nova do núcleo, que serve a qualquer modelo: a **troca de produto** (`changeover`),
  a preparação que só acontece quando o produto muda, com o tempo de cada troca numa matriz;
- os **indicadores** da fábrica com intervalo de confiança: ordens por dia, tempo de
  atravessamento, material em processo, ordens no prazo, atraso, OEE de cada máquina com as
  três perdas, trocas por dia e ocupação dos operadores;
- três **modelos prontos**.

Inspeção, retrabalho e refugo vêm do pacote [Qualidade](../pacotes-transversais.md): o modelo
pede `packs: ["qualidade"]` e usa as predefinições de lá.

## Primeiro contato

```bash
utsim linha lista
utsim linha manufatura predefinicoes
utsim linha manufatura relatorio examples/usinagem_troca.ron --html relatorio.html
utsim linha manufatura relatorio examples/linha_conwip.ron --oee-alvo 0.75
```

O `--oee-alvo` é o OEE que cada máquina deveria alcançar, de 0 a 1 (padrão 0,85).
`utsim run modelo.ron --linha manufatura` mostra os indicadores depois do resultado de
costume, com a meta padrão.

## Ordens, produtos e prazo

A ordem de produção é um item com rótulos:

| Rótulo | O que é |
|---|---|
| `produto` | o produto da ordem (1, 2, 3...), sorteado na liberação, por exemplo `Discrete([(1.0, 0.5), (2.0, 0.3), (3.0, 0.2)])` |
| `prazo` | minutos que a ordem tem, desde a liberação, para sair pela expedição |
| `vence` | o instante da data de entrega, gravado pela fila de ordens na primeira passagem (`vence = now + prazo`) |
| `cartao` | 1 nos cartões do laço de cartões; os indicadores não os contam como ordens |

## Regras de despacho

A fila de ordens escolhe a próxima ordem pelo campo `order`:

| `order` | Regra |
|---|---|
| `None` | ordem de chegada |
| `Some(Asc("vence"))` | data de entrega mais próxima primeiro |
| `Some(Asc("produto"))` | junta os produtos iguais para trocar menos (o produto de número maior pode esperar muito) |
| `Some(Desc("prioridade"))` | prioridade maior primeiro |

## Troca de produto

O processo ganhou o campo `changeover`. Com ele, a preparação (`setup`) só acontece quando o
produto da peça que chega é diferente do produto da peça anterior:

```ron
(name: "torno", kind: Processor(setup: Some(Const(6.0)), process: Triangular(min: 6.0, mode: 8.0, max: 11.0),
    changeover: Some((label: "produto", matrix: [
        (from: 1.0, to: 2.0, time: Triangular(min: 2.0, mode: 3.0, max: 5.0)),
        (from: 2.0, to: 1.0, time: Triangular(min: 3.0, mode: 4.0, max: 6.0)),
    ]))))
```

- `label` é o rótulo do produto.
- `matrix` dá o tempo de cada troca, do produto `from` para o produto `to`.
- Uma troca sem linha na matriz usa o `setup`, que é obrigatório com `changeover`.
- A máquina começa pronta para o primeiro produto que chega, sem troca (decisão M-D2).
- Com `capacity` maior que 1, vale o último produto que começou na máquina (decisão M-D3).
- Uma peça sem o rótulo não conta como troca.

O relatório do modelo ganha a contagem `changeovers` da máquina. Os modelos sem `changeover`
continuam como antes: preparação em toda peça. O painel de propriedades do UTSim 3D mostra o
campo em "Troca de produto", logo abaixo da preparação.

## Kanban e CONWIP

O laço de cartões não precisa de peça nova no núcleo: os cartões são itens que circulam. A
predefinição "Laço de cartões" cria cinco objetos:

| Objeto | Papel |
|---|---|
| `conwip` | fila de ordens antes do laço; ligue a liberação a ela |
| `conwip_cartoes` | cria os cartões no início (`max_arrivals` é o número de cartões) |
| `conwip_quadro` | o quadro onde os cartões esperam |
| `conwip_liberar` | junta a ordem (primeira entrada) com um cartão (segunda entrada) e solta a ordem com o cartão dentro; ligue a saída à primeira etapa |
| `conwip_devolver` | tira o cartão da ordem no fim; o cartão volta ao quadro (saída 1) e a ordem segue (saída 2, ligue à expedição) |

A ordem leva o cartão dentro dela, por isso as máquinas leem os rótulos da ordem (o produto, a
data de entrega). Sem cartão no quadro, a ordem espera fora do laço. Um laço que cobre a linha
toda é um CONWIP; um laço entre duas etapas é um kanban. O supermercado (uma fila com
`capacity`) é o jeito mais simples de limitar o estoque entre duas etapas.

## Quebra, EPI e inspeção ficam no nó

A quebra de cada máquina fica no campo `downtime` dela (aba Manutenção), o EPI de cada máquina
e de cada operador no campo `ppe` (aba Segurança), os defeitos no campo `quality` do processo
que os gera e a inspeção no campo `inspection` da estação. São os mesmos campos que os apps da
família editam; a linha não guarda cópia nenhuma desses dados.

## Indicadores

Contam as ordens que saíram depois do aquecimento. Uma ordem que sai por uma saída com
`sink_role: Some(Scrap)` é refugo. As máquinas são os processos e as montagens com tempo de
processo, menos as estações de inspeção, o retrabalho e o laço de cartões.

| Indicador | Definição |
|---|---|
| Ordens por dia | ordens boas que saíram, por dia |
| Tempo de atravessamento | da liberação até a saída, das ordens boas |
| Material em processo | média de ordens e peças no sistema (sem os cartões) |
| Ordens no prazo | fração das ordens com `vence` ou `prazo` que saíram até a data de entrega |
| Atraso médio das atrasadas | quanto passaram da data de entrega, só as atrasadas |
| Refugo | refugo ÷ (refugo + ordens boas) |
| Por produto | ordens, atravessamento e no prazo de cada produto |
| OEE | disponibilidade × desempenho × qualidade, por máquina |
| Disponibilidade | tempo sem quebra e sem troca ÷ tempo de turno |
| Desempenho | tempo de ciclo médio (média do `process` ÷ `capacity`) × peças ÷ tempo rodando; perde com falta de peça e com bloqueio (decisão M-D4) |
| Qualidade | ordens boas ÷ todas, do modelo inteiro (decisão M-D5) |
| Em troca | fração do tempo de turno em troca de produto |
| Trocas por dia | trocas de produto da máquina, por dia (só com `changeover`) |
| Ocupação dos operadores | fração do turno andando ou operando |

O `--html` grava a página com os cartões, a tabela de OEE por máquina, a tabela por produto e
a ocupação dos operadores. O `--json` grava os indicadores de cada replicação.

## Modelos prontos

| Modelo | O que mostra |
|---|---|
| `examples/usinagem_troca.ron` | três produtos, torno e fresa com matriz de troca, fila pela data de entrega, quebras e OEE; o experimento de juntar produtos na fila do torno |
| `examples/linha_conwip.ron` | carteira sempre cheia, célula em U com gargalo e 6 cartões; o efeito do número de cartões na vazão, no atravessamento e no prazo |
| `examples/oficina_ordens.ron` | roteiro diferente por produto, operadores com duas habilidades cada, defeitos no torno e na fresa, inspeção com retrabalho e refugo |

Experimentos típicos: trocar a regra de despacho, encurtar as trocas da matriz (SMED), mudar o
número de cartões, tirar uma habilidade de um operador, mudar a mistura dos produtos ou a
probabilidade de defeito.

## O que ainda não está na linha

- **Regra de despacho que segue o produto da máquina** (escolher primeiro a ordem do mesmo
  produto que a máquina acabou de fazer): hoje `Asc("produto")` junta pelo número do produto.
- **Troca por posto** quando a máquina tem capacidade maior que 1 (decisão M-D3).
- **Tempo de ciclo ideal próprio** para o OEE, diferente da média do `process` (decisão M-D4).
- **Ativos 3D de fábrica** (torno, fresa, quadro de cartões): por enquanto, máquinas, mesas e
  áreas da biblioteca comum.

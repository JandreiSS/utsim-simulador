# Pacotes transversais: Qualidade e Segurança

Uma linha, como o Lean Office, é o domínio inteiro do modelo (campo `line`). Um pacote transversal
não é uma linha. Ele traz predefinições que servem a qualquer linha, e o modelo as soma pelo campo
`packs`:

```ron
Model(
    name: "Meu modelo",
    line: Some("lean_office"),
    packs: ["qualidade", "seguranca"],
    ...
)
```

Os pacotes não mudam o que a simulação faz. As predefinições criam objetos comuns, marcados com
`preset: "qualidade/<id>"` ou `"seguranca/<id>"`, então o `.ron` continua legível e roda sem o
pacote instalado. O campo `packs` só diz à interface quais bibliotecas mostrar.

`utsim linha lista` mostra os pacotes, e os transversais aparecem marcados. Colocar uma
predefinição de um pacote transversal acrescenta o nome dele em `packs` e não mexe em `line`:

```bash
utsim linha qualidade colocar modelo.ron estacao_inspecao --nome inspecao --campo 'amostragem=5'
```

## Qualidade

Formato dos blocos em `docs/formato_qualidade_seguranca.md`. Predefinições:

| Predefinição | O que cria |
|---|---|
| `estacao_inspecao` (Estação de inspeção) | Processo com `inspection` e saída `ByLabel("inspecao")`. Ligue uma saída por porta: aprovado, retrabalho e refugo |
| `inspecao_medicao` (Inspeção por medição) | Inspeção que mede uma característica com o erro do sistema de medição |
| `retrabalho` (Retrabalho) | Processo com `rework`: conta a passagem e corrige defeitos |
| `refugo` (Refugo) | Destino final com `sink_role: Scrap` |
| `cliente` (Cliente) | Destino final com `sink_role: Customer` |
| `processo_defeito` (Processo com defeito) | Processo com `quality`: um tipo de defeito com probabilidade por item |

Modelo pronto: `examples/inspecao_e_retrabalho.ron`.

## Segurança

Predefinições de posto com o EPI exigido (campo `ppe`):

| Predefinição | EPI |
|---|---|
| `posto_epi` (Posto com EPI) | A lista do campo `epi` (padrão óculos e luvas) |
| `posto_ruidoso` (Posto ruidoso) | Óculos, luvas e protetor auricular |
| `posto_solda` (Posto de solda) | Máscara, luvas, avental e óculos |
| `doca_carga` (Doca de carga) | Capacete, botina e colete |

Uma predefinição só cria objetos, e a zona de risco é do modelo (campo `zones`). Para colocar uma:

```bash
utsim linha seguranca zona modelo.ron corredor --tipo veiculo --nivel 4 --epi colete \
    --poligono -12 4.5 12 4.5 12 7.5 -12 7.5
```

No editor, o comando é `AddZone`. Modelo pronto: `examples/cruzamento_empilhadeira.ron`, que
`utsim exposicao` analisa.

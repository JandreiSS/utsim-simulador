# Dados reais e ajuste automático

Todo campo de tempo do modelo (processamento, setup, intervalo entre chegadas, tempo entre quebras, reparo, carga e descarga dos transportadores, permanência no porta-paletes, valores de rótulos, períodos de chegada, atividades de fluxos) aceita dois jeitos de informar a distribuição:

1. **Distribuição já calculada**, como sempre: `process: Gamma(shape: 3.5, scale: 1.46)`.
2. **Valores reais**: você informa os tempos observados e o UTSim escolhe sozinho o melhor modelo de distribuição, explica a escolha e gera um relatório.

O segundo jeito usa a mesma análise do [`utsim fit`](ajuste-de-distribuicoes.md), mas sem copiar parâmetros à mão: o modelo aponta para os dados, e o ajuste é refeito quando os dados mudam.

## Como escrever no `.ron`

O campo continua com uma distribuição qualquer (ela é substituída pelo ajuste). A ligação com os dados fica na lista `data` do modelo, uma entrada por campo:

```ron
Model(
    name: "Atendimento",
    run: (stop_time: 50000.0),
    objects: [
        (name: "chegada",    kind: Source(interarrival: Exp(mean: 1.0))),
        (name: "atendentes", kind: Processor(capacity: 3, process: Const(1.0), setup: None)),
        (name: "saida",      kind: Sink),
    ],
    connections: [("chegada", "atendentes"), ("atendentes", "saida")],
    data: [
        // Instantes de chegada anotados num arquivo: ajusta os intervalos entre eles.
        (field: "chegada.interarrival", source: "dados/chegadas.csv",
         column: "instante_min", timestamps: true),
        // Tempos de atendimento: a melhor família pelo AIC.
        (field: "atendentes.process", source: "dados/atendimento.csv",
         choose: Best, criterion: Aic),
        // Poucos valores, escritos no próprio modelo: distribuição empírica.
        (field: "atendentes.setup", values: [0.2, 0.5, 0.3, 0.4, 0.2, 0.6],
         choose: Empirical),
    ],
)
```

O exemplo completo está em `examples/dados_reais.ron`.

| Campo da entrada | Padrão | Significado |
|---|---|---|
| `field` | obrigatório | o campo de distribuição, como `objeto.campo` (veja abaixo) |
| `source` | nenhum | arquivo CSV/TXT com os valores, relativo à pasta do modelo |
| `column` | automática | coluna do arquivo: nome do cabeçalho ou posição a partir de 1. Sem ela, vale a mesma escolha automática do `utsim fit`, que pula colunas de numeração |
| `timestamps` | `false` | a coluna traz instantes crescentes; ajusta os intervalos entre eles |
| `values` | `[]` | os próprios valores, para conjuntos pequenos. Use `source` ou `values`, não os dois |
| `choose` | `Best` | `Best` (a melhor família aceita), `Family("Lognormal")` (essa família, mesmo que os testes a rejeitem, com aviso) ou `Empirical` (os próprios valores) |
| `criterion` | `Aic` | o que ordena as famílias aceitas: `Aic`, `Bic`, `Ks` (menor distância de Kolmogorov-Smirnov) ou `Ad` (menor estatística de Anderson-Darling, que pesa mais as caudas) |
| `alpha` | `0.05` | nível de significância dos testes de aderência |
| `bootstrap` | `300` | réplicas do bootstrap paramétrico dos p-valores |
| `seed` | `1` | semente do bootstrap |
| `fitted` | nenhum | o registro do último ajuste, escrito pelo UTSim (não precisa escrever à mão) |

Os nomes de família em `Family(...)` podem ser escritos em português ou em inglês, com ou sem acento: `"Gama"` ou `"Gamma"`, `"Lognormal"` ou `"LogNormal"`, `"Exponencial"`, `"Weibull"`, `"Normal"`, `"Uniforme"`, `"Triangular"`, `"Beta"`, `"Erlang"`, `"Log-logística"`, `"Pearson V"`, `"Pearson VI"`, as deslocadas (`"Weibull deslocada"`...) e as discretas (`"Poisson"`, `"Geométrica"`, `"Binomial"`, `"Binomial negativa"`).

### Como nomear o campo

O campo é escrito como nas chaves de cenário: o nome do objeto, um ponto e o nome do campo no `.ron`. Campos dentro de campos levam mais pontos.

| Exemplo | Campo |
|---|---|
| `"torno.process"` | tempo de processamento |
| `"torno.setup"` | setup (pode estar vazio, `setup: None`) |
| `"torno.downtime.mtbf"`, `"torno.downtime.mttr"` | tempo entre quebras e tempo de reparo (a quebra precisa existir) |
| `"chegada.interarrival"` | intervalo entre chegadas |
| `"chegada.labels.peso"` | valor de um rótulo |
| `"empilhadeira.load_time"`, `"empilhadeira.unload_time"` | carga e descarga |
| `"porta_paletes.dwell"` | permanência mínima no porta-paletes |
| `"flows.nome_do_fluxo...."` | durações das atividades de um fluxo de tokens |

Para ver a lista de todos os campos de distribuição de um modelo, rode `utsim dados modelo.ron` num modelo sem `data`: o comando lista os campos que aceitam dados.

## O que acontece ao abrir o modelo

1. Os valores são lidos (do arquivo ou da lista `values`).
2. O UTSim olha o **suporte** dos dados: se são inteiros, se há zeros ou negativos e qual o intervalo. Famílias discretas só entram com dados inteiros; famílias que exigem valores positivos ficam de fora se houver zero ou negativo, e o relatório diz por quê; as famílias limitadas (uniforme, triangular, beta) usam o intervalo observado.
3. Todas as famílias que se aplicam são ajustadas e testadas (Kolmogorov-Smirnov, Anderson-Darling e qui-quadrado), e ordenadas pelo critério.
4. A escolha segue `choose`. Com `Best`, vence a primeira família aceita. Se **nenhuma família passar** nos testes, o campo usa a **distribuição empírica** e o UTSim avisa.
5. A distribuição escolhida vai para o campo, e a entrada ganha o registro `fitted`: família, distribuição, hash dos dados, número de valores e as opções usadas.

Tudo é **determinístico**: os mesmos dados com as mesmas opções dão sempre a mesma escolha, em qualquer máquina.

### Registro, reprodutibilidade e dados que mudam

Quando o modelo é salvo (pelo editor ou com `utsim dados modelo.ron --gravar`), o registro `fitted` vai junto:

```ron
fitted: (
    family: "Gama",
    dist: Gamma(shape: 3.5062574525060994, scale: 1.4587491276044857),
    hash: "fnv1a64:97625f46d8b77ed3",
    n: 80,
    settings: "auto1 Best Aic alpha=0.05 bootstrap=300 seed=1",
),
```

Na próxima abertura:

- **dados e opções iguais** ao registro: a distribuição registrada é usada sem refazer o ajuste (abre na hora);
- **dados diferentes** (um valor, a ordem ou a quantidade mudou): o campo é reajustado e aparece o aviso `os dados mudaram desde o último ajuste`, com o hash antigo e o novo e a família de antes e a de agora;
- **opções diferentes** (outro `criterion`, por exemplo): o campo é reajustado, sem aviso;
- **arquivo que não pode ser lido** (apagado, ou o modelo aberto em outra máquina): é usado o último ajuste registrado, com aviso. Sem registro, é erro.

O hash considera só os números: trocar `1,5` por `1.50` no arquivo não muda nada.

Como o campo do objeto guarda sempre a última distribuição ajustada, o modelo continua legível por qualquer ferramenta do UTSim, mesmo as que não conhecem `data`.

## Avisos sobre os dados

O ajuste só faz sentido com dados independentes e de um mesmo processo. Os avisos aparecem no `utsim check`, no `utsim run`, no editor e no relatório, com o campo na frente (`atendentes.process (dados): ...`):

- **poucos pontos**: abaixo de 30 valores os testes têm pouca força; abaixo de 10, nenhum teste distingue as famílias;
- **tendência** (Mann-Kendall) e **dependência entre valores consecutivos** (sequências e Ljung-Box): o processo pode estar mudando ao longo da coleta;
- **mistura**: a densidade dos dados tem dois ou mais picos separados, como acontece quando produtos, turnos ou operadores diferentes estão no mesmo arquivo; separe os grupos;
- **valores atípicos** fora de 1,5·IQR em quantidade, e **valores extremos** além de 3·IQR dos quartis: confira se não são erros de anotação;
- **empates por arredondamento**: muitos valores repetidos indicam dados arredondados, o que deixa os p-valores menos confiáveis;
- **nenhuma família aceita**: o campo usa a empírica, que nunca gera valores fora do intervalo observado;
- **família fixada rejeitada**: com `Family(...)`, a família escolhida foi rejeitada pelos testes.

## Relatórios

```bash
utsim dados examples/dados_reais.ron --relatorio relatorios_dados
```

O comando mostra, para cada campo, de onde vieram os dados, o que aconteceu (ajustado agora, registro reaproveitado, dados mudaram) e a distribuição em RON. Com `--relatorio pasta` ele grava:

- `index.html`: um índice com todos os campos, a distribuição escolhida e o número de avisos, com link para cada relatório;
- `<campo>.html`, um por campo: a escolha e a justificativa em texto simples, os avisos, o suporte e o hash dos dados, a tabela das candidatas na ordem do critério (parâmetros, AIC, BIC, p-valores e situação), a estatística descritiva, o diagnóstico, o histograma com as densidades das melhores candidatas e os gráficos Q-Q, P-P e da função de distribuição acumulada;
- `<campo>.csv`, um por campo: a tabela das candidatas, para planilhas.

As páginas não dependem de internet: abrem em qualquer navegador, também sem rede.

`--gravar` grava no modelo o registro de cada ajuste (o arquivo é reescrito no formato do editor; os comentários do começo do arquivo são mantidos).

O mesmo relatório sai do `utsim fit` para um arquivo avulso:

```bash
utsim fit dados.csv --coluna tempo --relatorio tempo.html --criterio ks
```

grava `tempo.html` e `tempo.csv` com a escolha automática pelo critério pedido.

## Na interface (em preparação)

O núcleo já tem o que a interface 3D vai usar: o comando de edição `SetFieldSource` troca um campo entre **Distribuição** e **Dados** como um passo de desfazer (Ctrl+Z); em **Dados** o ajuste é feito na hora e, se os valores não puderem ser lidos ou ajustados, nada muda. Voltar para **Distribuição** mantém no campo a distribuição que você informar (por exemplo, a última ajustada). Renomear o objeto atualiza as entradas de `data`; apagar o objeto remove as dele. O botão no painel de propriedades chega numa próxima versão da interface; até lá, escreva a lista `data` no `.ron` e use `utsim dados`.

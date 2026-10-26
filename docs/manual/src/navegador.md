# Página no navegador

O UTSim também roda no navegador, sem instalar nada. O motor de simulação é compilado para WebAssembly e roda no seu próprio computador. **Nenhum dado seu é enviado a um servidor.**

## Como obter a página

A página vem no pacote `utsim-<versão>-web` de cada versão. Para gerar a partir do código:

```bash
bash scripts/web.sh          # ou scripts/web.ps1 no Windows
python3 -m http.server -d web
```

Abra `http://localhost:8000/`. Para pular a compilação do visualizador 3D (a primeira vez leva uns 10 minutos), use `bash scripts/web.sh --sem-3d` (no Windows, `-Sem3d`).

O arquivo `web/dist/utsim.html` é **autocontido**: abre com um clique, sem servidor, mas sem a aba 3D.

## As três abas

### Simular

1. Escolha um exemplo na lista, ou **cole um modelo `.ron`** na caixa de texto. Os modelos dos tutoriais podem ser colados da pasta `examples/`.
2. Ajuste o número de **replicações**.
3. **Validar** confere o modelo e mostra os problemas, com as mesmas mensagens do `utsim check`.
4. **Simular** roda e mostra uma tabela com as métricas de cada objeto: média ± meia-amplitude do intervalo de confiança de 95%.

### Ajustar distribuição

É o `utsim fit` no navegador. Escolha um arquivo CSV ou TXT, ou cole os dados na caixa. Opcionalmente informe a coluna, marque "a coluna são instantes (usar as diferenças)" e escolha o número de réplicas do bootstrap. O relatório, com tabela, explicações e gráficos, aparece abaixo. Veja [Ajuste de distribuições](ajuste-de-distribuicoes.md).

### 3D

O botão **Gerar visualização 3D** roda o modelo e abre o visualizador na própria página. O campo de tempo máximo é opcional (padrão: o menor entre o tempo do modelo e 2000). O 3D no navegador é **somente leitura**: mostra a rodada, mas não edita o modelo.

Requisitos:

- um navegador com **WebGL2** (todos os atuais);
- a página servida por HTTP (`python3 -m http.server -d web`), pois o visualizador não vem no `utsim.html` autocontido;
- a primeira vez que o visualizador é construído leva uns 10 minutos.

## Página que não congela

A simulação roda num trabalhador em segundo plano (*Web Worker*), então a página continua respondendo. Um botão **Cancelar** aparece enquanto algo está rodando e interrompe a execução.

## Diferenças em relação ao programa

- O navegador não grava arquivos. Os resultados aparecem na página.
- As replicações rodam uma depois da outra (o programa nativo usa vários núcleos). Os **resultados são idênticos**, mas o programa nativo é mais rápido.
- `visual.asset` com arquivo `.glb` não é suportado no 3D do navegador.
- O editor 3D completo (`utsim-view`) é um programa à parte; o navegador só reproduz.
# Emulação OPC UA

A emulação roda o modelo **em tempo real** e o expõe por um servidor **OPC UA**. Um CLP, um supervisório (SCADA) ou qualquer cliente OPC UA lê o estado dos objetos e comanda paradas como se fosse a planta. É o **comissionamento virtual**: testar a lógica de controle contra um modelo, antes de a máquina existir ou sem arriscar a planta.

```bash
utsim emular examples/linha_com_bloqueio.ron --velocidade 10 --porta 4840 --gravar comandos.ron
# Servidor OPC UA em opc.tcp://127.0.0.1:4840/
```

## Opções

| Opção | Padrão | Significado |
|---|---|---|
| `--porta` | `4840` | porta TCP do servidor. Com `0`, o programa escolhe uma livre e a mostra |
| `--velocidade` | `1.0` | fator de velocidade. `1.0` é tempo real; `10.0`, dez vezes mais rápido. Uma unidade de tempo do modelo vale 1 segundo de relógio na velocidade 1.0 |
| `--ate` | `run.stop_time` | tempo simulado em que a emulação termina |
| `--gravar arquivo.ron` | nenhum | grava os comandos recebidos, com o instante simulado de cada um |
| `--reproduzir arquivo.ron` | nenhum | reaplica os comandos nos mesmos instantes simulados, sem OPC UA e sem esperar o relógio. Produz o mesmo relatório |

`Ctrl-C` encerra a emulação, grava o log de comandos (se pedido) e imprime o relatório. A emulação usa a replicação 0 do modelo.

## Espaço de endereços

O servidor usa o namespace `urn:utsim:emulacao` e nós de texto. O ponto de acesso não tem segurança e aceita acesso anônimo, então **use só em rede confiável**.

| Nó (`UTSim/<modelo>/...`) | Tipo | Acesso | Significado |
|---|---|---|---|
| `TempoSimulado` | Double | leitura | tempo simulado atual |
| `Velocidade` | Double | leitura e escrita | fator de velocidade (maior que 0) |
| `<objeto>/Conteudo` | UInt32 | leitura | itens dentro do objeto |
| `<objeto>/Estado` | String | leitura | estado atual (`idle`, `processing`, `stopped`...) |
| `<objeto>/Entradas` | UInt64 | leitura | itens que entraram |
| `<objeto>/Saidas` | UInt64 | leitura | itens que saíram |
| `<objeto>/Parar` | Boolean | leitura e escrita | `true` para o objeto; `false` o retoma |

Os valores são atualizados a cada 50 ms. As escritas são aplicadas no tick seguinte, no tempo simulado corrente. A simulação continua num só fio de execução e determinística para a mesma sequência de comandos.

## Passo a passo com um cliente

1. Rode `utsim emular examples/linha_com_bloqueio.ron`.
2. Num cliente genérico (UaExpert, `opcua-commander`), conecte-se a `opc.tcp://127.0.0.1:4840/`.
3. Expanda `Objects > UTSim > <modelo>`.
4. Assine `torno/Conteudo` e `torno/Estado`.
5. Escreva `true` em `torno/Parar`. O estado vira `stopped` e a fila anterior passa a crescer.
6. Escreva `false` para retomar.

## Repetir uma sessão

Rode com `--gravar` e o programa guarda cada comando com o instante simulado. Depois, `--reproduzir` reaplica tudo, mais rápido que o tempo real, e dá o mesmo resultado. Serve para depurar uma sessão que deu problema e para fazer testes de regressão da lógica de controle.

## Parar um objeto por comando

A parada por `Parar` usa o mesmo mecanismo das quebras e dos turnos: o objeto fecha entrada e saída, os tempos pausam, e o estado mostrado é `stopped`. O objeto só volta quando nenhum motivo de parada está ativo.

## Cena 3D ao vivo junto

Com `--cena-porta 7878`, a emulação também serve a cena 3D (protocolo `utsim-scene/1`, por TCP e WebSocket) no mesmo relógio do servidor OPC UA. Um cliente de realidade virtual vê o mesmo instante que o CLP. Os clientes de cena podem mandar os mesmos comandos de `Parar` e `Velocidade`, que entram no log de `--gravar`. Opções: `--cena-endereco` (padrão `127.0.0.1`) e `--cena-fps` (padrão 20). Sem autenticação: use só em rede confiável. Detalhes em [Visual 3D](visual-3d.md#ao-vivo).

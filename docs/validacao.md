# Validação contra resultados conhecidos

Este documento lista os casos em que o UTSim é comparado com valores obtidos de forma
independente do simulador: fórmulas fechadas, cadeias de Markov resolvidas no próprio teste
e contas feitas à mão. Os casos aqui complementam os de `crates/utsim-model/tests/`
(`queueing_theory.rs`, `validation.rs`, `finite_capacity.rs` e os testes determinísticos de
transporte, esteira, turnos e fluidos), que cobrem M/M/1, M/M/c, redes de Jackson em série,
M/M/1/K, M/M/∞, a lei de Little e os instantâneos (golden).

Código: `crates/utsim-model/tests/validacao_piloto.rs`.

## Como reproduzir

```
cargo test --release -p utsim-model --test validacao_piloto -- --nocapture --test-threads=1
bash scripts/validacao_tabela.sh        # imprime a tabela de resultados abaixo
```

As sementes são fixas, então os números abaixo se repetem exatamente. A suíte inteira leva
menos de 2 s em release (e poucos segundos em debug).

## Critério de aceitação

Cada métrica estocástica é estimada com 20 replicações independentes (12 no caso 8, que
precisa do rastro) após um aquecimento. O teste calcula a média e a meia largura `h` do IC95%
(t de Student) e aceita quando

```
|média - esperado| <= 1.4 · h + piso · |esperado|
```

- O fator 1.4 transforma o IC95% em algo próximo de um IC99% (a razão entre os quantis
  t de 0.995 e 0.975 fica entre 1.35 e 1.45 para 9 a 29 graus de liberdade). Com dezenas de
  métricas comparadas, um IC95% puro reprovaria um caso correto em cerca de 5% das
  métricas.
- O piso (0.2% a 1%, conforme o caso) cobre o viés residual de aquecimento finito e de
  contagem nas bordas da janela. Ele é pequeno diante das meias larguras dos casos que
  mais precisam dele.
- Valores determinísticos (esteira, fluidos, turnos exatos, rota fixa) são comparados com
  tolerância absoluta de 1e-9 (ou de uma unidade de contagem, onde a borda da janela pode
  contar um item a mais).

Cada teste coleta todas as métricas do caso, imprime uma linha `VALIDACAO|caso|métrica|...`
para cada uma e só então falha, de modo que um erro mostra o quadro completo.

## Casos, fórmulas e modelagem

### 1. Série com buffers finitos e bloqueio (cadeia de Markov exata)

Linha saturada de máquinas exponenciais: fonte sem espera, `Processor` com `Exp`, filas com
`capacity` entre elas, sumidouro no fim. O bloqueio é "após o serviço": a máquina termina,
fica no estado `blocked` segurando a peça e só a solta quando há lugar. O teste monta a
CTMC (estado = situação de cada máquina e conteúdo de cada buffer), resolve `πQ = 0`,
`Σπ = 1` por eliminação de Gauss com pivô parcial e obtém vazão, fração processando e
bloqueada de cada máquina e conteúdo médio de cada buffer. O próprio solucionador é
verificado contra valores conhecidos (duas máquinas iguais sem buffer: vazão 2/3 μ; buffer 1:
3/4 μ; buffer grande: tende a min μ).

- 1a. duas máquinas μ = 1; 1, sem buffer.
- 1b. duas máquinas μ = 1; 0.8, buffer 3.
- 1c. três máquinas μ = 1; 0.8; 1.25, buffers 1 e 2.

### 2. Retrabalho (rede de Jackson com realimentação)

Uma estação recebe chegadas de taxa λ e devolve cada peça para a mesma fila com
probabilidade `p` (`output: Weighted([p, 1-p])`). Razão de visitas `1/(1-p)`, taxa efetiva
`λ/(1-p)`, `ρ = λ s/(1-p)`. O número no sistema (fila mais servidor) é o de uma M/M/1:
`L = ρ/(1-ρ)`, `Lq = ρ²/(1-ρ)` e, por Little, `W = L/λ` (tempo total, somando todas as
passagens).

- 2a. λ = 0.3, p = 0.25, s = 1.5 (ρ = 0.6).
- 2b. λ = 0.2, p = 0.5, s = 1.8 (ρ = 0.72).

### 3. M/G/1 (Pollaczek-Khinchine)

`Wq = λ E[S²] / (2 (1-ρ))` com `E[S²] = E[S]² (1 + cs²)`; aqui λ = 1, E[S] = 0.8,
ρ = 0.8, então `Wq = 1.6 (1 + cs²)`. Serviços: determinístico (cs² = 0), Erlang-2 (1/2),
Erlang-4 (1/4), Uniforme(0.4; 1.2) (1/12) e Gama de forma 1/2 (cs² = 2). Também se confere
`Lq = λ Wq`, `W = Wq + E[S]` e ρ.

### 4. Máquina com quebras

Máquina saturada com `downtime` (mtbf e mttr). Disponibilidade `A = MTBF/(MTBF + MTTR)`,
independente das distribuições (relógio de falha contado no tempo corrido, trabalho
retomado após o reparo). Fração processando = A, fração em falha = 1 - A e vazão efetiva
`A/E[S]`. Três configurações: Exp/Exp com A = 0.8; Gama/Uniforme com processo fixo (testa a
insensibilidade); Exp/Exp com A = 0.5 e processo Erlang-3.

### 5. Turnos

Período 40 com janelas fora de turno `[10, 16)` e `[30, 34)`: 25% do tempo parada.

- 5a. Vazão saturada com `Exp(0.5)`: `(1 - 0.25)/0.5 = 1.5`. Janela de medição de 100 ciclos
  inteiros (aquecimento 40, fim em 4040), então a fração fora de turno é exatamente 0.25.
- 5b. Serviço fixo 0.7 em 10 ciclos: uptime total 300, `floor(300/0.7) = 428` itens.

### 6. Transportador

Perfil trapezoidal de um trecho `d` com velocidade máxima `v`, aceleração `a` e
desaceleração `δ`: `t = d/v + (v/2)(1/a + 1/δ)` quando `d >= (v²/2)(1/a + 1/δ)`; caso
contrário o perfil é triangular, `t = sqrt(2 d (1/a + 1/δ))`. Com `v = 2`, `a = 1`, `δ = 0.5`,
`d = 12`: cada trecho leva 9. Carga 1.5 e descarga 1: ciclo `R = 1.5 + 9 + 1 + 9 = 20.5`.

- 6a. Rota fixa, um item a cada 40, empilhadeira de casa na origem: fração de cada estado
  (`loading`, `travel_loaded`, `unloading`, `travel_empty`) e utilização `R/40 = 0.5125`.
- 6b. Chegadas Poisson λ = 0.03 com a empilhadeira começando e terminando cada viagem no
  destino: o transportador é um servidor M/D/1 com serviço R. `ρ = λR`,
  `W = λR²/(2(1-ρ)) + R`.

### 7. Esteira

Comprimento 30, velocidade 1.5, passo por item 0.6 (`g = s/v = 0.4`).

- 7a/7b. Trânsito livre (espaçamento sempre acima de `g`): `L/v = 20` exato, mínimo, médio e
  máximo, nos dois modos (acumuladora ou não).
- 7c/7d. Fonte saturada: vazão `v/s = 2.5`, ou 2500 saídas em 1000 unidades de tempo.
- 7e/7f. Esteira alimentando uma máquina `Exp(m = 2)`. Quando a peça da frente sai, a seguinte
  está um passo atrás e leva `g` para chegar à ponta; se o serviço acabar antes, a máquina
  espera. O ciclo é `E[max(S, g)] = g + m e^(-g/m) = 2.03746`; vazão `1/ciclo` e utilização
  `m/ciclo = 0.9816`. (Um primeiro palpite, vazão `1/m` e utilização 1, estava errado: a
  esteira cinemática de fato priva a máquina por `g` quando o serviço é mais curto que um
  passo. O simulador estava certo e a conta foi corrigida.)

### 8. Transferência por distância (`transfer`)

Fonte Poisson λ = 0.2 em `(0,0,0)`, fila e máquina `Exp(3)` em `(18,24,0)`; `transfer:
(speed: 2, delay: 1.5)`. Atraso `1.5 + 30/2 = 16.5`. O atraso fixo preserva o processo de
Poisson, então a fila é M/M/1 com ρ = 0.6: `W = 16.5 + 1/(μ - λ) = 24`. Itens em trânsito
`λ · 16.5 = 3.3` (integral sobre o rastro de `Transit`/`Moved`), `L = λW = 4.8` e a razão
`L/(λ_medido W_medido)` deve ser 1 (Little com os itens a caminho).

### 9. Combinador

Contêineres sempre disponíveis, lote de componentes.

- 9a. Componentes Poisson λ = 1, lote de 4, montagem fixa de 2: vazão de lotes `λ/n = 0.25`
  e fração processando `0.25 · 2 = 0.5`.
- 9b. Tudo saturado, lote de 3, preparo 0.5 mais `Exp(2)`: ciclo 2.5, vazão 0.4, frações
  0.2 em preparo e 0.8 processando.

### 10. Fluidos (exato até o evento)

Níveis lidos com paradas imediatamente antes, no instante e depois de cada evento, mais
médias de nível e volumes (tolerância 1e-9).

- 10a. Fonte 3, ralo 1, tanque de capacidade 12 com 2 de início: enche em `t = 5`
  (nível `2 + 2t`); depois a fonte é estrangulada para 1.
- 10b. Tanque com 8, fonte 1, tubo 2.5: esvazia em `t = 16/3`; nível médio em `t = 10` é
  `(8 · 16/3 - 0.75 · (16/3)²)/10`; volume no ralo 18 (conservação).
- 10c. Dois tanques em cascata: `t1` (6, saída máx. 2) esvazia em `t = 3`; `t2` (cap. 4,
  saída máx. 1) sobe até 3 e esvazia em `t = 6`.

### 11. Perda de Erlang (caso extra)

Sistema de perda M/G/c/c (`on_blocked: Drop`, sem fila) com c = 3 e a = 2.4. A probabilidade
de perda é a fórmula B de Erlang, `B = 0.2684`, e o número médio de servidores ocupados é
`a (1 - B)`. A fórmula vale para qualquer distribuição de serviço (insensibilidade), então
o teste usa serviço exponencial e determinístico.

## Resultados

Gerados por `scripts/validacao_tabela.sh` (sementes fixas).

| Caso | Métrica | Esperado | Simulado ± IC95% | Resultado |
|---|---|---|---|---|
| 1a. Série 2 máquinas, sem buffer (μ=1; 1) | vazão | 0.6667 | 0.6654 ± 0.0014 | PASS |
| 1a. Série 2 máquinas, sem buffer (μ=1; 1) | m1 processando | 0.6667 | 0.6668 ± 0.0014 | PASS |
| 1a. Série 2 máquinas, sem buffer (μ=1; 1) | m1 bloqueada | 0.3333 | 0.3332 ± 0.0014 | PASS |
| 1a. Série 2 máquinas, sem buffer (μ=1; 1) | m2 processando | 0.6667 | 0.6672 ± 0.0013 | PASS |
| 1b. Série 2 máquinas, buffer 3 (μ=1; 0.8) | vazão | 0.7289 | 0.7284 ± 0.0016 | PASS |
| 1b. Série 2 máquinas, buffer 3 (μ=1; 0.8) | m1 processando | 0.7289 | 0.7279 ± 0.0023 | PASS |
| 1b. Série 2 máquinas, buffer 3 (μ=1; 0.8) | m1 bloqueada | 0.2711 | 0.2721 ± 0.0023 | PASS |
| 1b. Série 2 máquinas, buffer 3 (μ=1; 0.8) | m2 processando | 0.9112 | 0.9120 ± 0.0017 | PASS |
| 1b. Série 2 máquinas, buffer 3 (μ=1; 0.8) | conteúdo médio de q1 | 1.9494 | 1.9557 ± 0.0101 | PASS |
| 1c. Série 3 máquinas, buffers 1 e 2 (μ=1; 0.8; 1.25) | vazão | 0.6490 | 0.6490 ± 0.0017 | PASS |
| 1c. Série 3 máquinas, buffers 1 e 2 (μ=1; 0.8; 1.25) | m1 processando | 0.6490 | 0.6496 ± 0.0016 | PASS |
| 1c. Série 3 máquinas, buffers 1 e 2 (μ=1; 0.8; 1.25) | m1 bloqueada | 0.3510 | 0.3504 ± 0.0016 | PASS |
| 1c. Série 3 máquinas, buffers 1 e 2 (μ=1; 0.8; 1.25) | m2 processando | 0.8112 | 0.8105 ± 0.0018 | PASS |
| 1c. Série 3 máquinas, buffers 1 e 2 (μ=1; 0.8; 1.25) | m2 bloqueada | 0.0323 | 0.0321 ± 0.0007 | PASS |
| 1c. Série 3 máquinas, buffers 1 e 2 (μ=1; 0.8; 1.25) | m3 processando | 0.5192 | 0.5187 ± 0.0025 | PASS |
| 1c. Série 3 máquinas, buffers 1 e 2 (μ=1; 0.8; 1.25) | conteúdo médio de q1 | 0.6307 | 0.6292 ± 0.0021 | PASS |
| 1c. Série 3 máquinas, buffers 1 e 2 (μ=1; 0.8; 1.25) | conteúdo médio de q2 | 0.3510 | 0.3495 ± 0.0036 | PASS |
| 2a. Retrabalho p=0.25 (λ=0.3, s=1.5, ρ=0.6) | razão de visitas (saídas de p / saídas do sistema) | 1.3333 | 1.3330 ± 0.0026 | PASS |
| 2a. Retrabalho p=0.25 (λ=0.3, s=1.5, ρ=0.6) | utilização ρ | 0.6000 | 0.5987 ± 0.0032 | PASS |
| 2a. Retrabalho p=0.25 (λ=0.3, s=1.5, ρ=0.6) | número médio no sistema L | 1.5000 | 1.4895 ± 0.0238 | PASS |
| 2a. Retrabalho p=0.25 (λ=0.3, s=1.5, ρ=0.6) | fila Lq = ρ²/(1-ρ) | 0.9000 | 0.8908 ± 0.0215 | PASS |
| 2a. Retrabalho p=0.25 (λ=0.3, s=1.5, ρ=0.6) | tempo no sistema W = L/λ | 5.0000 | 4.9683 ± 0.0725 | PASS |
| 2b. Retrabalho p=0.5 (λ=0.2, s=1.8, ρ=0.72) | razão de visitas (saídas de p / saídas do sistema) | 2.0000 | 1.9955 ± 0.0032 | PASS |
| 2b. Retrabalho p=0.5 (λ=0.2, s=1.8, ρ=0.72) | utilização ρ | 0.7200 | 0.7159 ± 0.0029 | PASS |
| 2b. Retrabalho p=0.5 (λ=0.2, s=1.8, ρ=0.72) | número médio no sistema L | 2.5714 | 2.5436 ± 0.0472 | PASS |
| 2b. Retrabalho p=0.5 (λ=0.2, s=1.8, ρ=0.72) | fila Lq = ρ²/(1-ρ) | 1.8514 | 1.8277 ± 0.0449 | PASS |
| 2b. Retrabalho p=0.5 (λ=0.2, s=1.8, ρ=0.72) | tempo no sistema W = L/λ | 12.8571 | 12.7307 ± 0.2140 | PASS |
| 3a. M/D/1 (cs²=0) | ρ | 0.8000 | 0.8001 ± 0.0016 | PASS |
| 3a. M/D/1 (cs²=0) | Wq | 1.6000 | 1.6191 ± 0.0248 | PASS |
| 3a. M/D/1 (cs²=0) | Lq = λ Wq | 1.6000 | 1.6193 ± 0.0265 | PASS |
| 3a. M/D/1 (cs²=0) | W = Wq + E[S] | 2.4000 | 2.4191 ± 0.0248 | PASS |
| 3b. M/E2/1 (cs²=1/2) | ρ | 0.8000 | 0.8013 ± 0.0018 | PASS |
| 3b. M/E2/1 (cs²=1/2) | Wq | 2.4000 | 2.4113 ± 0.0347 | PASS |
| 3b. M/E2/1 (cs²=1/2) | Lq = λ Wq | 2.4000 | 2.4116 ± 0.0368 | PASS |
| 3b. M/E2/1 (cs²=1/2) | W = Wq + E[S] | 3.2000 | 3.2126 ± 0.0350 | PASS |
| 3c. M/E4/1 (cs²=1/4) | ρ | 0.8000 | 0.7996 ± 0.0023 | PASS |
| 3c. M/E4/1 (cs²=1/4) | Wq | 2.0000 | 1.9984 ± 0.0300 | PASS |
| 3c. M/E4/1 (cs²=1/4) | Lq = λ Wq | 2.0000 | 1.9971 ± 0.0334 | PASS |
| 3c. M/E4/1 (cs²=1/4) | W = Wq + E[S] | 2.8000 | 2.7986 ± 0.0303 | PASS |
| 3d. M/U/1 (cs²=1/12) | ρ | 0.8000 | 0.7994 ± 0.0012 | PASS |
| 3d. M/U/1 (cs²=1/12) | Wq | 1.7333 | 1.7243 ± 0.0272 | PASS |
| 3d. M/U/1 (cs²=1/12) | Lq = λ Wq | 1.7333 | 1.7223 ± 0.0287 | PASS |
| 3d. M/U/1 (cs²=1/12) | W = Wq + E[S] | 2.5333 | 2.5247 ± 0.0274 | PASS |
| 3e. M/G/1 Gama (cs²=2) | ρ | 0.8000 | 0.7995 ± 0.0015 | PASS |
| 3e. M/G/1 Gama (cs²=2) | Wq | 4.8000 | 4.7859 ± 0.0645 | PASS |
| 3e. M/G/1 Gama (cs²=2) | Lq = λ Wq | 4.8000 | 4.7880 ± 0.0655 | PASS |
| 3e. M/G/1 Gama (cs²=2) | W = Wq + E[S] | 5.6000 | 5.5851 ± 0.0655 | PASS |
| 4a. Quebras Exp/Exp (MTBF 40, MTTR 10) | disponibilidade (fração processando) | 0.8000 | 0.8009 ± 0.0036 | PASS |
| 4a. Quebras Exp/Exp (MTBF 40, MTTR 10) | fração em falha | 0.2000 | 0.1991 ± 0.0036 | PASS |
| 4a. Quebras Exp/Exp (MTBF 40, MTTR 10) | vazão efetiva A/E[S] | 0.8000 | 0.8010 ± 0.0045 | PASS |
| 4b. Quebras Gama/Uniforme (MTBF 40, MTTR 10), processo fixo | disponibilidade (fração processando) | 0.8000 | 0.8004 ± 0.0015 | PASS |
| 4b. Quebras Gama/Uniforme (MTBF 40, MTTR 10), processo fixo | fração em falha | 0.2000 | 0.1996 ± 0.0015 | PASS |
| 4b. Quebras Gama/Uniforme (MTBF 40, MTTR 10), processo fixo | vazão efetiva A/E[S] | 1.6000 | 1.6008 ± 0.0031 | PASS |
| 4c. Quebras Exp/Exp (MTBF 10, MTTR 10), processo Erlang-3 | disponibilidade (fração processando) | 0.5000 | 0.4998 ± 0.0031 | PASS |
| 4c. Quebras Exp/Exp (MTBF 10, MTTR 10), processo Erlang-3 | fração em falha | 0.5000 | 0.5002 ± 0.0031 | PASS |
| 4c. Quebras Exp/Exp (MTBF 10, MTTR 10), processo Erlang-3 | vazão efetiva A/E[S] | 0.8333 | 0.8328 ± 0.0054 | PASS |
| 5a. Turnos: vazão saturada (25% fora de turno, serviço Exp 0.5) | fração fora de turno | 0.2500 | 0.2500 (exato) | PASS |
| 5a. Turnos: vazão saturada (25% fora de turno, serviço Exp 0.5) | fração processando | 0.7500 | 0.7500 (exato) | PASS |
| 5a. Turnos: vazão saturada (25% fora de turno, serviço Exp 0.5) | vazão | 1.5000 | 1.4951 ± 0.0091 | PASS |
| 5b. Turnos: contagem exata (serviço fixo 0.7, 10 ciclos) | itens produzidos | 428.0000 | 428.0000 (exato) | PASS |
| 5b. Turnos: contagem exata (serviço fixo 0.7, 10 ciclos) | fração fora de turno | 0.2500 | 0.2500 (exato) | PASS |
| 6a. Transportador: rota fixa, perfil trapezoidal (ciclo 20.5) | tempo de um trecho | 9.0000 | 9.0000 (exato) | PASS |
| 6a. Transportador: rota fixa, perfil trapezoidal (ciclo 20.5) | itens entregues | 100.0000 | 100.0000 (exato) | PASS |
| 6a. Transportador: rota fixa, perfil trapezoidal (ciclo 20.5) | loading | 0.0375 | 0.0375 (exato) | PASS |
| 6a. Transportador: rota fixa, perfil trapezoidal (ciclo 20.5) | travel_loaded | 0.2250 | 0.2250 (exato) | PASS |
| 6a. Transportador: rota fixa, perfil trapezoidal (ciclo 20.5) | unloading | 0.0250 | 0.0250 (exato) | PASS |
| 6a. Transportador: rota fixa, perfil trapezoidal (ciclo 20.5) | travel_empty | 0.2250 | 0.2250 (exato) | PASS |
| 6a. Transportador: rota fixa, perfil trapezoidal (ciclo 20.5) | utilização = ciclo/intervalo | 0.5125 | 0.5125 (exato) | PASS |
| 6b. Transportador: carga Poisson = servidor M/D/1 (R = 20.5) | utilização ρ = λR | 0.6150 | 0.6160 ± 0.0025 | PASS |
| 6b. Transportador: carga Poisson = servidor M/D/1 (R = 20.5) | tempo no sistema W = Wq + R | 36.8734 | 36.7879 ± 0.3460 | PASS |
| 7a. Esteira acumuladora: trânsito livre = L/v | tempo médio de trânsito | 20.0000 | 20.0000 (exato) | PASS |
| 7a. Esteira acumuladora: trânsito livre = L/v | trânsito mínimo | 20.0000 | 20.0000 (exato) | PASS |
| 7a. Esteira acumuladora: trânsito livre = L/v | trânsito máximo | 20.0000 | 20.0000 (exato) | PASS |
| 7b. Esteira não acumuladora: trânsito livre = L/v | tempo médio de trânsito | 20.0000 | 20.0000 (exato) | PASS |
| 7b. Esteira não acumuladora: trânsito livre = L/v | trânsito mínimo | 20.0000 | 20.0000 (exato) | PASS |
| 7b. Esteira não acumuladora: trânsito livre = L/v | trânsito máximo | 20.0000 | 20.0000 (exato) | PASS |
| 7c. Esteira acumuladora saturada: vazão = v/s | saídas em 1000 unidades | 2500.0000 | 2500.0000 (exato) | PASS |
| 7d. Esteira não acumuladora saturada: vazão = v/s | saídas em 1000 unidades | 2500.0000 | 2500.0000 (exato) | PASS |
| 7e. Esteira acumuladora + máquina Exp(2): ciclo E[max(S, s/v)] | vazão = 1/(g + m e^(-g/m)) | 0.4908 | 0.4901 ± 0.0016 | PASS |
| 7e. Esteira acumuladora + máquina Exp(2): ciclo E[max(S, s/v)] | utilização da máquina = m/ciclo | 0.9816 | 0.9818 ± 0.0002 | PASS |
| 7f. Esteira não acumuladora + máquina Exp(2): ciclo E[max(S, s/v)] | vazão = 1/(g + m e^(-g/m)) | 0.4908 | 0.4901 ± 0.0016 | PASS |
| 7f. Esteira não acumuladora + máquina Exp(2): ciclo E[max(S, s/v)] | utilização da máquina = m/ciclo | 0.9816 | 0.9818 ± 0.0002 | PASS |
| 8. Transferência por distância: atraso + d/v, Little com itens em trânsito | tempo no sistema W = 16.5 + 7.5 | 24.0000 | 24.0480 ± 0.3210 | PASS |
| 8. Transferência por distância: atraso + d/v, Little com itens em trânsito | itens em trânsito = λ·16.5 | 3.3000 | 3.2965 ± 0.0293 | PASS |
| 8. Transferência por distância: atraso + d/v, Little com itens em trânsito | L total (blocos + trânsito) | 4.8000 | 4.8049 ± 0.0940 | PASS |
| 8. Transferência por distância: atraso + d/v, Little com itens em trânsito | Little: L total / (λ_medido · W_medido) | 1.0000 | 0.9999 ± 0.0003 | PASS |
| 9a. Combinador: lote de 4 componentes Poisson λ=1 | lotes por unidade de tempo = λ/n | 0.2500 | 0.2502 ± 0.0006 | PASS |
| 9a. Combinador: lote de 4 componentes Poisson λ=1 | fração processando = (λ/n)·2 | 0.5000 | 0.5004 ± 0.0011 | PASS |
| 9b. Combinador saturado: ciclo = preparo 0.5 + Exp(2) | lotes por unidade de tempo = 1/2.5 | 0.4000 | 0.4008 ± 0.0009 | PASS |
| 9b. Combinador saturado: ciclo = preparo 0.5 + Exp(2) | fração em preparo = 0.5/2.5 | 0.2000 | 0.2004 ± 0.0004 | PASS |
| 9b. Combinador saturado: ciclo = preparo 0.5 + Exp(2) | fração processando = 2/2.5 | 0.8000 | 0.7995 ± 0.0004 | PASS |
| 10a. Tanque enchendo: nível exato antes e depois do evento (t=5) | nível em t=4.9 | 11.8000 | 11.8000 (exato) | PASS |
| 10a. Tanque enchendo: nível exato antes e depois do evento (t=5) | nível em t=5 | 12.0000 | 12.0000 (exato) | PASS |
| 10a. Tanque enchendo: nível exato antes e depois do evento (t=5) | nível em t=5.1 | 12.0000 | 12.0000 (exato) | PASS |
| 10a. Tanque enchendo: nível exato antes e depois do evento (t=5) | nível em t=9 | 12.0000 | 12.0000 (exato) | PASS |
| 10a. Tanque enchendo: nível exato antes e depois do evento (t=5) | nível médio em t=9 | 9.2222 | 9.2222 (exato) | PASS |
| 10a. Tanque enchendo: nível exato antes e depois do evento (t=5) | volume da fonte em t=9 | 19.0000 | 19.0000 (exato) | PASS |
| 10a. Tanque enchendo: nível exato antes e depois do evento (t=5) | volume no ralo em t=9 | 9.0000 | 9.0000 (exato) | PASS |
| 10b. Tanque esvaziando: nível exato antes e depois do evento (t=16/3) | nível em t=5 | 0.5000 | 0.5000 (exato) | PASS |
| 10b. Tanque esvaziando: nível exato antes e depois do evento (t=16/3) | nível em t=5.5 | 0.0000 | 0.0000 (exato) | PASS |
| 10b. Tanque esvaziando: nível exato antes e depois do evento (t=16/3) | nível em t=10 | 0.0000 | 0.0000 (exato) | PASS |
| 10b. Tanque esvaziando: nível exato antes e depois do evento (t=16/3) | nível médio em t=10 | 2.1333 | 2.1333 (exato) | PASS |
| 10b. Tanque esvaziando: nível exato antes e depois do evento (t=16/3) | volume no ralo | 18.0000 | 18.0000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | t1 em t=2.9 | 0.2000 | 0.2000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | t2 em t=2.9 | 2.9000 | 2.9000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | t1 em t=3 | 0.0000 | 0.0000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | t2 em t=3 | 3.0000 | 3.0000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | t1 em t=3.1 | 0.0000 | 0.0000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | t2 em t=3.1 | 2.9000 | 2.9000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | t1 em t=5.9 | 0.0000 | 0.0000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | t2 em t=5.9 | 0.1000 | 0.1000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | t1 em t=6 | 0.0000 | 0.0000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | t2 em t=6 | 0.0000 | 0.0000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | t1 em t=7 | 0.0000 | 0.0000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | t2 em t=7 | 0.0000 | 0.0000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | volume total no ralo | 6.0000 | 6.0000 (exato) | PASS |
| 10c. Tanques em cascata: eventos em t=3 e t=6 | nível médio de t2 | 0.9000 | 0.9000 (exato) | PASS |
| 11a. Perda de Erlang M/M/3/3 (a=2.4) | probabilidade de perda (Erlang B) | 0.2684 | 0.2686 ± 0.0012 | PASS |
| 11a. Perda de Erlang M/M/3/3 (a=2.4) | servidores ocupados em média a(1-B) | 1.7558 | 1.7539 ± 0.0051 | PASS |
| 11b. Perda de Erlang M/D/3/3 (a=2.4) | probabilidade de perda (Erlang B) | 0.2684 | 0.2684 ± 0.0010 | PASS |
| 11b. Perda de Erlang M/D/3/3 (a=2.4) | servidores ocupados em média a(1-B) | 1.7558 | 1.7525 ± 0.0025 | PASS |

## Divergências encontradas

Nenhum caso reprovou. A única discrepância observada durante a construção foi do próprio
valor esperado (caso 7e/7f, explicada acima) e não do simulador.

Observações de modelagem que não são defeitos, mas que o usuário precisa saber:

- Uma fonte "sempre pronta" (saturada) não pode usar `interarrival: Const(0.0)`: a validação
  rejeita médias que levariam a mais de 1e9 chegadas. Os casos saturados usam
  `Const(1e-4)` com `on_blocked: Wait` (o padrão): a fonte só cria a próxima peça depois que
  a anterior sai, então o intervalo de 1e-4 fica escondido enquanto a máquina trabalha. O
  viés introduzido é da ordem de 1e-4 em relação ao ciclo e fica dentro do piso de tolerância.
- Bloqueio no UTSim é "após o serviço" (a peça fica na máquina no estado `blocked`).
  Comparações com resultados de teoria de filas com bloqueio devem usar esse mecanismo
  (como na cadeia de Markov do caso 1) e não o bloqueio "antes do serviço".
- A esteira cinemática carrega o espaçamento entre peças: depois de um bloqueio, a peça
  seguinte precisa percorrer um passo `s/v` até a ponta. Modelos de "esteira como fila
  instantânea" subestimam o ciclo de uma máquina rápida alimentada por esteira.

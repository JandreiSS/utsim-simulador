# Aprendizado por reforço

Em alguns problemas, a regra de decisão não é conhecida: para qual linha mandar o próximo cliente, qual pedido atender primeiro. Em vez de escrever a regra, você pode deixar um **agente** aprendê-la por tentativa e erro, usando o modelo como ambiente.

## Como funciona

O modelo **pausa e pergunta** ao agente. No fluxo de tokens, a atividade `Decide` com a regra `Agent` para a simulação, entrega uma observação (uma lista de números) e espera a escolha de um ramo.

```ron
flows: [
    (
        name: "triagem",
        activities: [
            (name: "chegada", kind: Source(interarrival: Exp(mean: 0.5)), next: ["escolhe_linha"]),
            (name: "escolhe_linha", kind: Decide(rule: Agent(observation: ["now"])),
                next: ["cria_rapida", "cria_lenta"]),
            (name: "cria_rapida", kind: CreateItem(into: "fila_rapida"), next: ["fim"]),
            (name: "cria_lenta",  kind: CreateItem(into: "fila_lenta"),  next: ["fim"]),
            (name: "fim", kind: Sink),
        ],
    ),
],
```

A decisão leva tempo zero de simulação. A ação `0` leva ao primeiro ramo de `next`, a ação `1` ao segundo, e assim por diante. Para o agente mandar um item para um lado, o fluxo costuma **criar** o item depois da escolha (`CreateItem` em cada ramo). O modelo completo é `examples/roteamento_rl.ron`: uma linha rápida com uma máquina e uma linha lenta com duas máquinas.

## Rodar sem agente

Um modelo com `Decide` do tipo `Agent` precisa de um agente. Sem ele, o `utsim run`, o replay e o 3D usam a decisão padrão de `run.default_decision`:

```ron
run: (stop_time: 200.0, seed: 11, replications: 5, default_decision: Some(0)),
```

Com `default_decision: Some(0)`, a resposta é sempre a opção 0 (ou a última, se houver menos opções). Sem esse campo, `utsim check` valida o modelo mas `utsim run` recusa a rodada.

## O ambiente (crate `utsim-env`)

A crate `utsim-env` transforma o modelo num ambiente no estilo Gym:

```rust
use utsim_env::{Env, EnvConfig, RewardTerm};

let modelo = utsim_model::Model::load("examples/roteamento_rl.ron")?;
let config = EnvConfig {
    // recompensa = -(área sob o número de itens na fila rápida) entre decisões
    reward: vec![RewardTerm::integral("fila_rapida.content.mean", -1.0)],
    observe_blocks: vec!["fila_rapida+maquina_rapida".into(), "fila_lenta+maquina_lenta".into()],
    episode_length: Some(100.0),
    ..EnvConfig::default()
};
let mut env = Env::new(modelo, config)?;
let mut obs = env.reset(1)?;              // roda até a primeira decisão
while obs.options > 0 {
    let r = env.step(0)?;                 // responde e roda até a próxima
    obs = r.observation;
    if r.done { break; }
}
```

| Campo de `EnvConfig` | Significado |
|---|---|
| `reward` | termos da recompensa: Σ peso x variação da métrica entre decisões |
| `decision_cost` | custo constante subtraído em cada decisão |
| `episode_length` | duração do episódio (substitui `run.stop_time`) |
| `observe_blocks` | objetos cujo conteúdo entra na observação. `"a+b"` soma os conteúdos |
| `observe_metrics` | métricas ao vivo (`objeto.métrica`) que entram na observação |

- **Observação:** o vetor do `Decide`, depois o conteúdo dos objetos de `observe_blocks` e as métricas de `observe_metrics`.
- **Recompensa:** Σ peso x Δ métrica entre decisões, menos `decision_cost`. Com `RewardTerm::integral`, a métrica de média no tempo é multiplicada pelo tempo decorrido, o que dá a área acumulada. Com peso -1 sobre o conteúdo dos objetos, o retorno do episódio é o oposto da soma dos tempos no sistema (pela lei de Little). Maximizar o retorno é minimizar o atraso total.
- **Episódio:** `reset(semente)` roda até a primeira decisão. `step(ação)` responde e roda até a próxima ou até o fim. O aquecimento é zerado e `episode_length` substitui o tempo de parada.

## Linha de base: Q-learning

`utsim_env::qlearning` traz um Q-learning tabular (faixas por dimensão, ε-guloso, taxa de aprendizado que decai com as visitas). No exemplo, 1000 episódios de treino bastam para vencer a política aleatória e chegar perto da heurística "menor atraso esperado". Para ver os números:

```bash
cargo test -p utsim-env --test learning -- --nocapture
```

## Python

A crate `crates/utsim-py` expõe `utsim.Env` em Python (listas e dicionários simples, sem numpy). Ela exige um ambiente Python e é compilada à parte:

```bash
cd crates/utsim-py
maturin develop --release
cd ../..
python crates/utsim-py/python/exemplo_gym.py
```

```python
import utsim
env = utsim.Env("examples/roteamento_rl.ron",
                reward=[("fila_rapida.content.mean", -1.0, True)],
                observe_blocks=["fila_rapida+maquina_rapida"], episode_length=100.0)
obs, info = env.reset(seed=1)
obs, reward, done, info = env.step(0)
```

`reward` é uma lista de tuplas `(métrica, peso, ponderar_pelo_tempo)`.
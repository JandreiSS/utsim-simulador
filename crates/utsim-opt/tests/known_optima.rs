//! Validação do otimizador contra ótimos conhecidos da teoria das filas.
//!
//! Três problemas cujo ótimo sai de fórmulas fechadas, todos com a busca exaustiva
//! (`Algorithm::Grid`):
//!
//! 1. Dimensionamento de servidores M/M/c (Erlang C): λ = 1, a = 2,5, c ∈ {3..8},
//!    objetivo c + 5·Lq. O ótimo c* é o mínimo da fórmula.
//! 2. Dimensionamento de buffer M/M/1/K com perda (`on_blocked: Drop`): objetivo
//!    K + 50·λ·P_K, com K = capacidade da fila + 1 e P_K = (1-ρ)ρ^K/(1-ρ^(K+1)).
//! 3. Restrição: mesmo problema 1 com Wq ≤ 0,1. Isso torna o ótimo irrestrito
//!    (c = 5, Wq ≈ 0,130) inviável, e o melhor viável é o mais barato entre c ≥ 6.
//!
//! Como a comparação é estatística, o teste aceita que o melhor ponto seja c* **ou**
//! um ponto equivalente a ele (teste pareado com 95%), desde que c* esteja nesse
//! conjunto. Também exige que c* nunca seja significativamente pior que o melhor.
//! Por fim, a diferença pareada de um ponto consigo mesmo deve ser exatamente zero.
//!
//! Observação sobre o problema 2: as métricas do relatório não trazem a contagem de
//! perdas (a `Source` registra `lost` como tally, cuja média é sempre 1). Por isso a
//! perda é medida pela vazão aceita, `chegada.output / T`, com T = stop − warmup,
//! que é uma constante do cenário: λ·P_K = λ − vazão. A função objetivo medida fica
//! então deslocada por uma constante, (c_slot + c_perda·λ) = 51, que não muda o
//! ranking. O teste soma essa constante antes de comparar com a teoria.

use utsim_core::stats::Summary;
use utsim_model::Model;
use utsim_opt::{
    Evaluation, OptProblem, OptResult, Point, compare, evaluate, optimize, paired_difference,
};

fn model(text: &str) -> Model {
    let m = Model::parse(text).expect("model parses");
    m.validate().expect("model is valid");
    m
}

/// Probabilidade de espera Erlang C para c servidores e carga a = λ/μ (exige a < c).
fn erlang_c(c: u32, a: f64) -> f64 {
    let rho = a / f64::from(c);
    let mut term = 1.0; // a^0/0!
    let mut sum = 0.0;
    for k in 0..c {
        sum += term;
        term *= a / f64::from(k + 1);
    }
    // Após o laço, `term` vale a^c/c!.
    let tail = term / (1.0 - rho);
    tail / (sum + tail)
}

/// Lq do M/M/c: P_w · ρ/(1-ρ), com ρ = a/c.
fn lq_mmc(c: u32, a: f64) -> f64 {
    let rho = a / f64::from(c);
    erlang_c(c, a) * rho / (1.0 - rho)
}

/// P_K do M/M/1/K com ρ = λ/μ.
fn mm1k_pk(rho: f64, k: i32) -> f64 {
    (1.0 - rho) * rho.powi(k) / (1.0 - rho.powi(k + 1))
}

fn index_of(r: &OptResult, key: &str, value: &str) -> usize {
    r.evaluations
        .iter()
        .position(|e| e.point[key] == value)
        .unwrap_or_else(|| panic!("ponto {key}={value} não avaliado"))
}

/// Verifica a regra de equivalência: c* é o melhor ou está no conjunto equivalente
/// ao melhor, e c* não é significativamente pior que o melhor.
fn assert_optimum_or_equivalent(r: &OptResult, star: usize, label: &str) {
    let best = r.best.expect("há ponto viável");
    let mut set = vec![best];
    set.extend(r.equivalent.iter().copied());
    println!(
        "{label}: melhor = índice {best}, conjunto equivalente = {:?}",
        set
    );
    assert!(
        set.contains(&star),
        "{label}: c* (índice {star}) não é o melhor nem estatisticamente equivalente a ele (melhor: índice {best})"
    );
    if star != best {
        let d = paired_difference(&r.evaluations[star], &r.evaluations[best]);
        println!(
            "{label}: diferença pareada c* - melhor = {:.4} ± {:.4}",
            d.mean, d.half_width
        );
        assert!(
            d.mean - d.half_width <= 0.0,
            "{label}: c* é significativamente pior que o melhor ({:.4} ± {:.4})",
            d.mean,
            d.half_width
        );
    }
}

fn describe_eval(e: &Evaluation) -> String {
    let s: &Summary = &e.objective;
    format!("{:.4} ± {:.4}", s.mean, s.half_width)
}

// ---------------------------------------------------------------------------
// 1. Dimensionamento de servidores M/M/c
// ---------------------------------------------------------------------------

const STAFFING_MODEL: &str = r#"Model(
    name: "M/M/c staffing",
    run: (stop_time: 10000.0, warmup: 500.0, seed: 7),
    objects: [
        (name: "chegada", kind: Source(interarrival: Exp(mean: 1.0))),
        (name: "fila", kind: Queue()),
        (name: "maquina", kind: Processor(capacity: 3, process: Exp(mean: 2.5))),
        (name: "saida", kind: Sink),
    ],
    connections: [("chegada", "fila"), ("fila", "maquina"), ("maquina", "saida")],
)"#;

const CS: f64 = 1.0; // custo por servidor
const CW: f64 = 5.0; // custo por item na fila (Lq)

fn staffing_problem(constraints: &str) -> OptProblem {
    OptProblem::parse(&format!(
        r#"OptProblem(
            name: "staffing",
            model: "staffing.ron",
            replications: 8,
            variables: [(key: "maquina.capacity", domain: Integer(min: 3, max: 8))],
            objective: (
                terms: [("fila.content.mean", {CW})],
                costs: [("maquina.capacity", {CS})],
            ),
            constraints: [{constraints}],
        )"#
    ))
    .expect("problem parses")
}

#[test]
fn staffing_optimum_matches_erlang_c() {
    let base = model(STAFFING_MODEL);
    let problem = staffing_problem("");
    let r = optimize(&problem, &base).expect("optimize runs");

    let a = 2.5;
    let theory: Vec<(u32, f64)> = (3..=8)
        .map(|c| (c, CS * f64::from(c) + CW * lq_mmc(c, a)))
        .collect();
    let (c_star, _) =
        theory.iter().copied().fold(
            (0, f64::INFINITY),
            |acc, (c, v)| if v < acc.1 { (c, v) } else { acc },
        );
    println!("M/M/c (a = 2,5): objetivo teórico = c + 5·Lq (Erlang C)");
    for (c, v) in &theory {
        let i = index_of(&r, "maquina.capacity", &c.to_string());
        println!(
            "  c = {c}: objetivo teórico {v:.4}, medido {} (Lq teórico {:.4})",
            describe_eval(&r.evaluations[i]),
            lq_mmc(*c, a),
        );
    }
    let best = r.best.expect("há ponto viável");
    println!(
        "c* teórico = {c_star}; melhor medido = c = {}",
        r.evaluations[best].point["maquina.capacity"]
    );
    for line in &r.explanation {
        println!("  {line}");
    }
    let star = index_of(&r, "maquina.capacity", &c_star.to_string());
    assert_optimum_or_equivalent(&r, star, "M/M/c");
}

#[test]
fn staffing_constraint_makes_optimum_infeasible() {
    // Wq ≤ 0,1. Teórico: Wq(c) = Lq(c)/λ: c=3 → 3,51; c=4 → 0,533; c=5 → 0,130 (viola);
    // c=6 → 0,034; c=7 e 8 também atendem. O melhor viável é o de menor custo: c = 6.
    let base = model(STAFFING_MODEL);
    let problem = staffing_problem("(metric: \"fila.staytime.mean\", max: 0.1)");
    let r = optimize(&problem, &base).expect("optimize runs");

    let a = 2.5;
    println!("M/M/c com restrição Wq ≤ 0,1 (teórico: Wq = Lq/λ)");
    let mut theory_feasible = Vec::new();
    for c in 3..=8u32 {
        let wq = lq_mmc(c, a);
        let i = index_of(&r, "maquina.capacity", &c.to_string());
        let e = &r.evaluations[i];
        let (_, wq_s, _) = &e.constraints[0];
        println!(
            "  c = {c}: Wq teórico {wq:.4}, Wq medido {:.4} ± {:.4}, viável medido: {}, objetivo medido {}",
            wq_s.mean,
            wq_s.half_width,
            e.feasible,
            describe_eval(e)
        );
        if wq <= 0.1 {
            theory_feasible.push(c);
        }
        // Ponto inviável nunca é o melhor.
        if !e.feasible {
            assert_ne!(
                Some(i),
                r.best,
                "c = {c} é inviável e foi escolhido como melhor"
            );
        }
        assert_eq!(
            e.feasible,
            wq <= 0.1,
            "c = {c}: viabilidade medida difere da teórica (Wq teórico {wq:.4})"
        );
    }
    let c_star = theory_feasible[0];
    println!("melhor viável teórico: c = {c_star}");
    let star = index_of(&r, "maquina.capacity", &c_star.to_string());
    assert_optimum_or_equivalent(&r, star, "M/M/c com restrição");
    for line in &r.explanation {
        println!("  {line}");
    }
}

#[test]
fn common_random_numbers_identical_points_pair_to_zero() {
    let base = model(STAFFING_MODEL);
    let problem = staffing_problem("");
    let point: Point = [("maquina.capacity".to_string(), "5".to_string())].into();
    let a = evaluate(&base, &problem, &point).expect("evaluate");
    let b = evaluate(&base, &problem, &point).expect("evaluate");
    assert_eq!(
        a.per_replication, b.per_replication,
        "mesma semente, mesmo resultado"
    );
    let d = paired_difference(&a, &b);
    println!(
        "CRN: diferença pareada de um ponto consigo mesmo = {} ± {}",
        d.mean, d.half_width
    );
    assert_eq!(d.mean, 0.0);
    assert_eq!(d.half_width, 0.0);
    // Ponto da grade: compare() também deve reconhecer o empate como equivalente.
    let r = compare(&problem, vec![a, b]);
    assert_eq!(
        r.equivalent,
        vec![1],
        "dois pontos idênticos são equivalentes"
    );
}

// ---------------------------------------------------------------------------
// 2. Dimensionamento de buffer M/M/1/K com perda
// ---------------------------------------------------------------------------

const LAMBDA: f64 = 1.0;
const MU_MEAN: f64 = 0.8; // μ = 1,25, ρ = 0,8
const C_SLOT: f64 = 1.0; // custo por posição de sistema
const C_LOSS: f64 = 50.0; // custo por unidade de perda (por unidade de tempo, por λ)
const STOP: f64 = 20000.0;
const WARMUP: f64 = 1000.0;

#[test]
fn mm1k_buffer_optimum_matches_closed_form() {
    let m = model(&format!(
        r#"Model(
            name: "M/M/1/K buffer",
            run: (stop_time: {STOP}, warmup: {WARMUP}, seed: 41),
            objects: [
                (name: "chegada", kind: Source(interarrival: Exp(mean: 1.0), on_blocked: Drop)),
                (name: "fila", kind: Queue(capacity: Some(1))),
                (name: "maquina", kind: Processor(process: Exp(mean: {MU_MEAN}))),
                (name: "saida", kind: Sink),
            ],
            connections: [("chegada", "fila"), ("fila", "maquina"), ("maquina", "saida")],
        )"#
    ));
    // Vazão aceita medida = chegada.output / T; λ·P_K = λ − vazão.
    let t_meas = STOP - WARMUP;
    let problem = OptProblem::parse(&format!(
        r#"OptProblem(
            name: "buffer",
            model: "buffer.ron",
            replications: 8,
            variables: [(key: "fila.capacity", domain: Integer(min: 1, max: 8))],
            objective: (
                terms: [("chegada.output", {})],
                costs: [("fila.capacity", {C_SLOT})],
            ),
        )"#,
        -C_LOSS / t_meas
    ))
    .expect("problem parses");
    let r = optimize(&problem, &m).expect("optimize runs");

    let rho = LAMBDA / (1.0 / MU_MEAN);
    // Objetivo medido = c_slot·(K-1) − c_perda·(vazão) = [c_slot·K + c_perda·λ·P_K] − (c_slot + c_perda·λ).
    let offset = C_SLOT + C_LOSS * LAMBDA;
    let theory: Vec<(i32, f64, f64)> = (2..=9)
        .map(|k| {
            let pk = mm1k_pk(rho, k);
            (k, pk, f64::from(k) * C_SLOT + C_LOSS * LAMBDA * pk)
        })
        .collect();
    let k_star = theory
        .iter()
        .copied()
        .fold(
            (0, f64::INFINITY),
            |acc, (k, _, v)| if v < acc.1 { (k, v) } else { acc },
        )
        .0;
    println!("M/M/1/K (ρ = 0,8, c_slot = 1, c_perda = 50): objetivo teórico = K + 50·λ·P_K");
    for (k, pk, v) in &theory {
        let i = index_of(&r, "fila.capacity", &(k - 1).to_string());
        let e = &r.evaluations[i];
        let measured_corrected = e.objective.mean + offset;
        // obj = (K-1) - c_perda·vazão  =>  vazão = (K-1 - obj)/c_perda.
        let throughput = (f64::from(k - 1) - e.objective.mean) / C_LOSS;
        println!(
            "  K = {k}: P_K teórico {pk:.5}, objetivo teórico {v:.4}, medido {measured_corrected:.4} ± {:.4} (vazão medida {throughput:.4} vs λ(1-P_K) {:.4})",
            e.objective.half_width,
            LAMBDA * (1.0 - pk),
        );
    }
    println!("K* teórico = {k_star}");
    let star = index_of(&r, "fila.capacity", &(k_star - 1).to_string());
    assert_optimum_or_equivalent(&r, star, "M/M/1/K");
    for line in &r.explanation {
        println!("  {line}");
    }
}

use chr_direct_conditional::{
    engine::{Event, PreparedRuleset},
    equality::EqualityProbe,
};
use chr_syntax::{Answer, Query, Rule, Var, c, eq, or, t, v};
fn values(p: EqualityProbe) -> [u64; 6] {
    [
        p.walker_ticks,
        p.support_ticks,
        p.binding_probes,
        p.empty,
        p.full,
        p.partial,
    ]
}
fn main() {
    let rules = vec![Rule::simplify(
        "emit-or-recur",
        [c("stream", [v(0)])],
        or(
            eq(v(0), t("pair", [v(1), v(1)])),
            c("stream", [v(0)]).into(),
        ),
    )];
    let p = PreparedRuleset::new(rules).unwrap();
    let mut e = p
        .start(Query {
            constraints: vec![c("stream", [v(10)])],
            outputs: vec![("answer".into(), Var(10))],
        })
        .unwrap();
    let wanted = Answer {
        outputs: vec![("answer".into(), t("pair", [v(99), v(99)]))],
        residual: vec![],
    };
    let mut answers = 0;
    let mut stages = [0u64; 7];
    let mut counts = [[0u64; 6]; 7];
    for calls in 1..=6_000_000 {
        let stage = e.allocation_stage();
        stages[stage] += 1;
        let before = values(e.store().probe());
        let event = e.tick();
        let after = values(e.store().probe());
        for i in 0..6 {
            counts[stage][i] += after[i] - before[i];
        }
        match event {
            Event::Answer(a) => {
                assert!(chr_observe::equivalent(
                    &a,
                    &wanted,
                    &mut Default::default()
                ));
                answers += 1;
                if [1, 8, 32, 64, 128].contains(&answers) {
                    println!(
                        "{{\"answers\":{answers},\"calls\":{calls},\"stages\":{stages:?},\"counts\":{counts:?},\"supports\":{},\"variables\":{}}}",
                        e.supports().node_count(),
                        e.store().variable_count()
                    );
                }
                if answers == 128 {
                    return;
                }
            }
            Event::Exhausted => panic!("continuing source exhausted"),
            Event::Progress => (),
        }
    }
    panic!("service cutoff after {answers}");
}

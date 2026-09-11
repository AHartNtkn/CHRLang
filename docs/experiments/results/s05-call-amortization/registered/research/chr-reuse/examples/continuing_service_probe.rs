#[allow(dead_code, unused_imports)]
mod gate {
    include!("../tests/consumer_pressure.rs");
    pub fn run() {
        let rules = vec![Rule::simplify(
            "emit-or-recur",
            [c("stream", [v(0)])],
            or(
                eq(v(0), t("pair", [v(1), v(1)])),
                c("stream", [v(0)]).into(),
            ),
        )];
        let schema = Schema {
            family: "aliases",
            resource: false,
            fail_tail: false,
            work: 0,
            payload: 0,
        };
        let p = Owner::new("conditional", schema, rules);
        let mut run = p.start(Query {
            constraints: vec![c("stream", [v(10)])],
            outputs: vec![("answer".into(), Var(10))],
        });
        let mut answers = 0;
        for calls in 1..=20_000_000 {
            match run.tick() {
                Event::Answer(a) => {
                    oracle::same_raw(
                        vec![a],
                        vec![Answer {
                            outputs: vec![("answer".into(), t("pair", [v(99), v(99)]))],
                            residual: vec![],
                        }],
                    );
                    answers += 1;
                    if [1, 8, 32, 64, 128, 256, 512].contains(&answers) {
                        println!("{{\"answers\":{answers},\"calls\":{calls}}}");
                    }
                    if answers == 512 {
                        return;
                    }
                }
                Event::Done => panic!("continuing source exhausted"),
                Event::Progress => (),
            }
        }
        panic!("probe cutoff after {answers} answers");
    }
}
fn main() {
    gate::run();
}

#[allow(dead_code)]
#[path = "../tests/runtime_support/mod.rs"]
mod scalar;
#[path = "../../chr-compiled/examples/support/resource_fusion_source.rs"]
mod source;
use chr_syntax::{Term, Var};
use std::collections::BTreeMap;
fn value(
    s: &mut chr_relational::contextual::Store,
    vars: &mut BTreeMap<Var, chr_relational::Value>,
    t: &Term,
) -> chr_relational::Value {
    match t {
        Term::Var(v) => *vars.entry(*v).or_insert_with(|| s.unknown()),
        Term::App(n, xs) => {
            let xs = xs.iter().map(|x| value(s, vars, x)).collect::<Vec<_>>();
            s.constructor(n, &xs)
        }
    }
}
fn main() {
    assert!(std::hint::black_box(cfg!(feature = "metrics")));
    for n in [0usize, 1, 3] {
        let rules = source::rules("plain");
        let q = source::query("plain", n, 0);
        let program = chr_compiled::resource_fusion::Program::infer(&rules).unwrap();
        let fused = program.lower(&q).unwrap();
        let mut store = chr_relational::contextual::Store::default();
        let mut vars = BTreeMap::new();
        for c in &q.constraints {
            let xs = c
                .args
                .iter()
                .map(|x| value(&mut store, &mut vars, x))
                .collect::<Vec<_>>();
            store.post(&c.name, &xs);
        }
        assert_eq!(store.matches(&rules[0].kept, &rules[0].removed).len(), 0);
        assert_eq!(
            store.matches(&rules[1].kept, &rules[1].removed).len(),
            n * n
        );
        let tuples = store.matches(&fused[0].kept, &fused[0].removed).len();
        assert_eq!(tuples, n * n * 2 * n * (2 * n).saturating_sub(1));
        let expected = scalar::run(&rules, &q, 200000);
        for (name, r) in [("original", rules.as_slice()), ("fused", fused)] {
            let mut e = chr_direct_conditional::engine::PreparedRuleset::new(r.to_vec())
                .unwrap()
                .start(q.clone())
                .unwrap();
            let mut answers = vec![];
            let mut exhausted = false;
            for _ in 0..200000 {
                match e.tick() {
                    chr_direct_conditional::engine::Event::Answer(a) => answers.push(a),
                    chr_direct_conditional::engine::Event::Exhausted => {
                        exhausted = true;
                        break;
                    }
                    _ => (),
                }
            }
            assert!(exhausted);
            scalar::same_raw(answers, expected.clone());
            println!(
                "n={n} form={name} initial_contextual_fused_tuples={tuples} conditional_discovered={}",
                e.stats().discovered_tuples
            );
        }
    }
}

use chr_family::{Mode, Request, Solver, Value};
fn a() -> Value {
    Value::app("a", [])
}
fn b() -> Value {
    Value::app("b", [])
}
fn run(request: Request) -> Vec<chr_syntax::Answer> {
    let mut results = vec![];
    for mode in [Mode::Eager, Mode::Named] {
        let mut s = Solver::new(request.clone(), mode).unwrap();
        s.advance(10000);
        assert!(s.exhausted());
        let answers = s.observe().unwrap();
        results.push(answers);
    }
    assert_eq!(results[0].len(), results[1].len());
    for e in &results[0] {
        assert!(
            results[1]
                .iter()
                .any(|a| chr_observe::equivalent(a, e, &mut Default::default()))
        );
    }
    results.pop().unwrap()
}
#[test]
fn repeated_labels_correlate_and_distinct_labels_are_independent() {
    for second in [0, 1] {
        let answers = run(Request {
            labels: second + 1,
            equations: vec![
                (Value::var(0), Value::choice(0, a(), b())),
                (Value::var(1), Value::choice(second, a(), b())),
            ],
            outputs: vec![0, 1],
        });
        assert_eq!(answers.len(), if second == 0 { 2 } else { 4 });
        if second == 0 {
            for a in answers {
                assert_eq!(a.outputs[0].1, a.outputs[1].1);
            }
        }
    }
}
#[test]
fn occurs_failure_restricts_only_its_own_alternative() {
    let x = Value::var(0);
    let answers = run(Request {
        labels: 1,
        equations: vec![(x.clone(), Value::choice(0, a(), Value::app("s", [x])))],
        outputs: vec![0],
    });
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0].outputs[0].1, chr_syntax::atom("a"));
}
#[test]
fn binding_an_acyclic_opaque_family_does_not_force_distribution() {
    let request = Request {
        labels: 1,
        equations: vec![(Value::var(0), Value::app("f", [Value::choice(0, a(), b())]))],
        outputs: vec![0],
    };
    let mut s = Solver::new(request, Mode::Named).unwrap();
    s.advance(100);
    assert!(s.exhausted());
    assert_eq!(s.stats().splits, 0);
    assert_eq!(s.observe().unwrap().len(), 2);
}

#[derive(Clone)]
enum Spec {
    V(u64),
    A(&'static str, Vec<Spec>),
    C(u32, Box<Spec>, Box<Spec>),
}
impl Spec {
    fn value(&self) -> Value {
        match self {
            Self::V(v) => Value::var(*v),
            Self::A(n, args) => Value::app(n, args.iter().map(Self::value)),
            Self::C(d, a, b) => Value::choice(*d, a.value(), b.value()),
        }
    }
    fn scalar(&self, bits: u64) -> chr_syntax::Term {
        match self {
            Self::V(v) => chr_syntax::v(*v),
            Self::A(n, args) => {
                chr_syntax::t(n, args.iter().map(|a| a.scalar(bits)).collect::<Vec<_>>())
            }
            Self::C(d, a, b) => {
                if bits & (1 << d) == 0 {
                    a.scalar(bits)
                } else {
                    b.scalar(bits)
                }
            }
        }
    }
}
#[test]
fn generated_requests_agree_per_assignment_with_independent_reference_unification() {
    use Spec::{A, C, V};
    use chr_syntax::{Goal, Query, Rule, Var, c};
    let pool = vec![
        V(0),
        V(1),
        V(2),
        A("a", vec![]),
        A("b", vec![]),
        A("s", vec![V(0)]),
        A("s", vec![V(1)]),
        C(0, Box::new(V(1)), Box::new(A("a", vec![]))),
        C(0, Box::new(A("b", vec![])), Box::new(V(0))),
        C(1, Box::new(V(2)), Box::new(A("s", vec![V(0)]))),
        A("f", vec![V(1), V(2)]),
        C(1, Box::new(A("a", vec![])), Box::new(A("b", vec![]))),
    ];
    for (i, a) in pool.iter().enumerate() {
        for (j, b) in pool.iter().enumerate() {
            let request = Request {
                labels: 2,
                equations: vec![(Value::var(0), a.value()), (Value::var(1), b.value())],
                outputs: vec![0, 1, 2],
            };
            let mut expected = vec![];
            for bits in 0..4 {
                let rule = Rule::simplify(
                    "request",
                    [c(
                        "run",
                        [chr_syntax::v(0), chr_syntax::v(1), chr_syntax::v(2)],
                    )],
                    Goal::And(vec![
                        Goal::Unify(chr_syntax::v(0), a.scalar(bits)),
                        Goal::Unify(chr_syntax::v(1), b.scalar(bits)),
                    ]),
                );
                let query = Query {
                    constraints: vec![c(
                        "run",
                        [chr_syntax::v(0), chr_syntax::v(1), chr_syntax::v(2)],
                    )],
                    outputs: (0..3).map(|v| (format!("out{v}"), Var(v))).collect(),
                };
                let mut reference = chr_reference::Search::new(vec![rule], query).unwrap();
                let batch = reference.advance(100);
                assert!(batch.exhausted);
                if let Some(answer) = batch.answers.into_iter().next() {
                    expected.push((bits, answer));
                }
            }
            for mode in [Mode::Eager, Mode::Named] {
                let mut s = Solver::new(request.clone(), mode).unwrap();
                s.advance(10000);
                assert!(s.exhausted(), "{i} {j} {mode:?}");
                let actual = s.projected_answers().unwrap();
                assert_eq!(actual.len(), expected.len(), "{i} {j} {mode:?}");
                for (bits, wanted) in &expected {
                    let got = actual
                        .iter()
                        .find(|(b, _)| b == bits)
                        .unwrap_or_else(|| panic!("missing assignment {bits}: {i} {j} {mode:?}"));
                    assert!(
                        chr_observe::equivalent(&got.1, wanted, &mut Default::default()),
                        "{i} {j} {bits} {mode:?}: {:?} expected {wanted:?}",
                        got.1
                    );
                }
            }
        }
    }
}
#[test]
fn observation_cannot_certify_an_unfinished_request() {
    let mut s = Solver::new(
        Request {
            labels: 0,
            equations: vec![(Value::var(0), a())],
            outputs: vec![0],
        },
        Mode::Named,
    )
    .unwrap();
    assert!(s.observe().is_err());
    s.advance(1);
    assert!(s.observe().is_err());
    s.advance(1);
    assert!(s.observe().is_ok());
}

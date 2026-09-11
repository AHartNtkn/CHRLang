//! Experimental source data and hand-derived observations, independent of engine algorithms.
use chr_syntax::{
    Answer, Constraint, Goal, Guard, Query, Rule, Term, Var, and, atom, c, eq, or, t, v,
};

#[derive(Clone, Debug)]
pub struct Case {
    pub id: String,
    pub rules: Vec<Rule>,
    pub query: Query,
    pub expected: Vec<Answer>,
    pub exhausted: bool,
    pub raw_answers: u64,
    pub budget: usize,
    pub answer_limit: Option<usize>,
}
pub fn answer(values: Vec<Term>, mut residual: Vec<Constraint>) -> Answer {
    residual.sort();
    Answer {
        outputs: values
            .into_iter()
            .enumerate()
            .map(|(i, x)| (format!("out{i}"), x))
            .collect(),
        residual,
    }
}
pub fn query(constraints: Vec<Constraint>, outputs: &[u64]) -> Query {
    Query {
        constraints,
        outputs: outputs
            .iter()
            .enumerate()
            .map(|(i, x)| (format!("out{i}"), Var(*x)))
            .collect(),
    }
}
fn case(
    id: &str,
    rules: Vec<Rule>,
    input: Vec<Constraint>,
    outputs: &[u64],
    expected: Vec<Answer>,
    raw: u64,
) -> Case {
    Case {
        id: id.into(),
        rules,
        query: query(input, outputs),
        expected,
        exhausted: true,
        raw_answers: raw,
        budget: 10_000,
        answer_limit: None,
    }
}
fn start(body: Goal) -> Vec<Rule> {
    vec![Rule::simplify("start", [c("start", [v(0), v(1)])], body)]
}
fn equation_case(id: &str, body: Goal, expected: Vec<Answer>, raw: u64) -> Case {
    case(
        id,
        start(body),
        vec![c("start", [v(0), v(1)])],
        &[0, 1],
        expected,
        raw,
    )
}
fn pick() -> Rule {
    Rule::simplify(
        "pick",
        [c("pick", [v(0)])],
        or(eq(v(0), atom("a")), eq(v(0), atom("b"))),
    )
}

pub fn semantic_cases() -> Vec<Case> {
    let mut cases = vec![
        equation_case("U01-clash", eq(atom("a"), atom("b")), vec![], 0),
        equation_case(
            "U02-self",
            eq(v(0), v(0)),
            vec![answer(vec![v(0), v(1)], vec![])],
            1,
        ),
        equation_case(
            "U03-alias",
            and([eq(v(0), t("f", [v(1)])), eq(v(1), atom("a"))]),
            vec![answer(vec![t("f", [atom("a")]), atom("a")], vec![])],
            1,
        ),
        equation_case(
            "U04-occurs",
            and([eq(v(0), t("f", [v(1)])), eq(v(1), v(0))]),
            vec![],
            0,
        ),
        equation_case(
            "U05-conditional-occurs",
            or(eq(v(0), t("f", [v(0)])), eq(v(0), atom("a"))),
            vec![answer(vec![atom("a"), v(0)], vec![])],
            1,
        ),
        equation_case(
            "U06-union-cycle",
            or(eq(v(0), t("f", [v(1)])), eq(v(1), t("f", [v(0)]))),
            vec![
                answer(vec![t("f", [v(0)]), v(0)], vec![]),
                answer(vec![v(0), t("f", [v(0)])], vec![]),
            ],
            2,
        ),
        equation_case(
            "C01-correlation",
            and([or(eq(v(0), atom("a")), eq(v(0), atom("b"))), eq(v(1), v(0))]),
            vec![
                answer(vec![atom("a"), atom("a")], vec![]),
                answer(vec![atom("b"), atom("b")], vec![]),
            ],
            2,
        ),
        case(
            "C02-independent",
            vec![pick()],
            vec![c("pick", [v(0)]), c("pick", [v(1)])],
            &[0, 1],
            vec![
                answer(vec![atom("a"), atom("a")], vec![]),
                answer(vec![atom("a"), atom("b")], vec![]),
                answer(vec![atom("b"), atom("a")], vec![]),
                answer(vec![atom("b"), atom("b")], vec![]),
            ],
            4,
        ),
        equation_case(
            "C03-inactive-label",
            or(
                eq(v(0), atom("a")),
                or(eq(v(0), atom("b")), eq(v(0), atom("c"))),
            ),
            vec![
                answer(vec![atom("a"), v(0)], vec![]),
                answer(vec![atom("b"), v(0)], vec![]),
                answer(vec![atom("c"), v(0)], vec![]),
            ],
            3,
        ),
        case(
            "H01-distinct-resources",
            vec![Rule::simplify(
                "pair",
                [c("p", [v(0)]), c("p", [v(1)])],
                c("hit", []).into(),
            )],
            vec![c("p", [atom("a")])],
            &[],
            vec![answer(vec![], vec![c("p", [atom("a")])])],
            1,
        ),
        case(
            "H02-repeated-head-alias",
            vec![
                Rule::simplify(
                    "same",
                    [c("p", [v(0)]), c("p", [v(0)])],
                    c("hit", []).into(),
                ),
                Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
            ],
            vec![c("p", [v(0)]), c("p", [v(1)]), c("bind", [v(0), v(1)])],
            &[0, 1],
            vec![answer(vec![v(0), v(0)], vec![c("hit", [])])],
            1,
        ),
        case(
            "H03-constructor-wakeup",
            vec![
                Rule::simplify("known", [c("p", [atom("a")])], c("hit", []).into()),
                Rule::simplify("bind", [c("bind", [v(0)])], eq(v(0), atom("a"))),
            ],
            vec![c("p", [v(0)]), c("bind", [v(0)])],
            &[0],
            vec![answer(vec![atom("a")], vec![c("hit", [])])],
            1,
        ),
        case(
            "H04-inherited-history",
            vec![
                Rule::propagate("emit", [c("p", [])], c("hit", []).into()),
                pick(),
            ],
            vec![c("p", []), c("pick", [v(0)])],
            &[0],
            vec![
                answer(vec![atom("a")], vec![c("p", []), c("hit", [])]),
                answer(vec![atom("b")], vec![c("p", []), c("hit", [])]),
            ],
            2,
        ),
        case(
            "H05-conditional-history",
            vec![
                Rule::propagate("emit", [c("p", [atom("a")])], c("hit", []).into()),
                pick(),
            ],
            vec![c("p", [v(0)]), c("pick", [v(0)])],
            &[0],
            vec![
                answer(vec![atom("a")], vec![c("p", [atom("a")]), c("hit", [])]),
                answer(vec![atom("b")], vec![c("p", [atom("b")])]),
            ],
            2,
        ),
        case(
            "H06-conditional-consumption",
            vec![
                Rule::simplify("consume", [c("p", []), c("q", [])], c("hit", []).into()),
                Rule::simplify(
                    "choose",
                    [c("choose", [])],
                    or(c("q", []).into(), c("r", []).into()),
                ),
            ],
            vec![c("p", []), c("choose", [])],
            &[],
            vec![
                answer(vec![], vec![c("hit", [])]),
                answer(vec![], vec![c("p", []), c("r", [])]),
            ],
            2,
        ),
        case(
            "V01-fresh",
            vec![Rule::simplify("fresh", [c("p", [])], c("q", [v(8)]).into())],
            vec![c("p", []), c("p", [])],
            &[],
            vec![answer(vec![], vec![c("q", [v(0)]), c("q", [v(1)])])],
            1,
        ),
        equation_case(
            "A01-disconnected-failure",
            and([eq(v(0), atom("a")), Goal::Fail]),
            vec![],
            0,
        ),
        case(
            "A02-pending-failure",
            vec![
                Rule::simplify(
                    "start",
                    [c("start", [v(0)])],
                    and([eq(v(0), atom("a")), c("bad", []).into()]),
                ),
                Rule::simplify("bad", [c("bad", [])], Goal::Fail),
            ],
            vec![c("start", [v(0)])],
            &[0],
            vec![],
            0,
        ),
        equation_case(
            "A03-raw-duplicates",
            or(eq(v(0), atom("a")), eq(v(0), atom("a"))),
            vec![answer(vec![atom("a"), v(0)], vec![])],
            2,
        ),
        case(
            "A04-alpha-residual",
            vec![Rule::simplify(
                "start",
                [c("start", [])],
                or(c("p", [v(0)]).into(), c("p", [v(1)]).into()),
            )],
            vec![c("start", [])],
            &[],
            vec![answer(vec![], vec![c("p", [v(0)])])],
            2,
        ),
        case(
            "M01-committed",
            vec![
                Rule::simplify("first", [c("p", [v(0)])], eq(v(0), atom("a"))),
                Rule::simplify("second", [c("p", [v(0)])], eq(v(0), atom("b"))),
            ],
            vec![c("p", [v(0)])],
            &[0],
            vec![answer(vec![atom("a")], vec![])],
            1,
        ),
        case(
            "M02-nonbinding",
            vec![Rule::simplify("known", [c("p", [atom("a")])], Goal::True)],
            vec![c("p", [v(0)])],
            &[0],
            vec![answer(vec![v(0)], vec![c("p", [v(0)])])],
            1,
        ),
    ];
    // Constructor dispatch is committed; the two recursive alternatives are explicit OR.
    let recursive = vec![
        Rule::simplify(
            "build",
            [c("build", [t("s", [v(4)]), v(1)])],
            or(
                and([eq(v(1), t("a", [v(3)])), c("build", [v(4), v(3)]).into()]),
                and([eq(v(1), t("b", [v(3)])), c("build", [v(4), v(3)]).into()]),
            ),
        ),
        Rule::simplify(
            "end",
            [c("build", [atom("z"), v(0)])],
            eq(v(0), atom("nil")),
        ),
    ];
    let mut expected = vec![];
    for x in ["a", "b"] {
        for y in ["a", "b"] {
            for z in ["a", "b"] {
                expected.push(answer(vec![t(x, [t(y, [t(z, [atom("nil")])])])], vec![]));
            }
        }
    }
    cases.push(case(
        "C04-recursive-fresh",
        recursive,
        vec![c("build", [chr_programs::unary(3), v(0)])],
        &[0],
        expected,
        8,
    ));
    let mut guard = Rule::simplify("guard", [c("p", [v(0), v(1)])], c("hit", []).into());
    guard.guards = vec![Guard::Equal(v(0), v(1))];
    cases.push(case(
        "G01-suspended",
        vec![guard.clone()],
        vec![c("p", [v(0), v(1)])],
        &[0, 1],
        vec![answer(vec![v(0), v(1)], vec![c("p", [v(0), v(1)])])],
        1,
    ));
    cases.push(case(
        "G02-recheck",
        vec![
            guard,
            Rule::simplify("bind", [c("bind", [v(0), v(1)])], eq(v(0), v(1))),
        ],
        vec![c("p", [v(0), v(1)]), c("bind", [v(0), v(1)])],
        &[0, 1],
        vec![answer(vec![v(0), v(0)], vec![c("hit", [])])],
        1,
    ));
    let mut fair = case(
        "S01-finite-sibling",
        vec![
            Rule::simplify(
                "start",
                [c("start", [v(0)])],
                or(c("loop", []).into(), eq(v(0), atom("done"))),
            ),
            Rule::simplify("loop", [c("loop", [])], c("loop", []).into()),
        ],
        vec![c("start", [v(0)])],
        &[0],
        vec![answer(vec![atom("done")], vec![])],
        1,
    );
    fair.exhausted = false;
    fair.budget = 200;
    cases.push(fair);
    cases.sort_by(|a, b| a.id.cmp(&b.id));
    cases
}

/// Natural query, with choice rules ordered before carry expansion in the reference policy.
pub fn carry_case(k: usize, w: usize, noise: usize) -> Case {
    let tags = (0..k).map(|i| v(i as u64)).collect::<Vec<_>>();
    let tag = t("tag", tags.clone());
    let mut input = vec![c("carry", [tag, chr_programs::unary(w)])];
    input.extend(tags.into_iter().map(|x| c("pick", [x])));
    let noise_constraints = (0..noise)
        .map(|i| c("unrelated", [chr_programs::unary(i)]))
        .collect::<Vec<_>>();
    input.extend(noise_constraints.clone());
    let mut expected = vec![];
    for bits in 0..(1usize << k) {
        let values = (0..k)
            .map(|i| atom(if bits & (1 << i) == 0 { "a" } else { "b" }))
            .collect::<Vec<_>>();
        let mut residual = noise_constraints.clone();
        residual.push(c("carry", [t("tag", values.clone()), atom("z")]));
        expected.push(answer(values, residual));
    }
    let outputs = (0..k as u64).collect::<Vec<_>>();
    case(
        &format!("carry-k{k}-w{w}-n{noise}"),
        vec![
            pick(),
            Rule::simplify(
                "carry",
                [c("carry", [v(0), t("s", [v(1)])])],
                c("carry", [v(0), v(1)]).into(),
            ),
        ],
        input,
        &outputs,
        expected,
        1u64 << k,
    )
}

pub fn application_cases() -> Vec<Case> {
    use chr_programs::unary as n;
    let mut cases = vec![
        case(
            "app-add-forward",
            chr_programs::arithmetic(),
            vec![c("add", [n(2), n(3), v(0)])],
            &[0],
            vec![answer(vec![n(5)], vec![])],
            1,
        ),
        case(
            "app-add-left",
            chr_programs::arithmetic(),
            vec![c("add", [v(0), n(3), n(5)])],
            &[0],
            vec![answer(vec![n(2)], vec![])],
            1,
        ),
        case(
            "app-add-right",
            chr_programs::arithmetic(),
            vec![c("add", [n(2), v(0), n(5)])],
            &[0],
            vec![answer(vec![n(3)], vec![])],
            1,
        ),
        case(
            "app-add-decompose",
            chr_programs::arithmetic(),
            vec![c("add", [v(0), v(1), n(3)])],
            &[0, 1],
            (0..=3)
                .map(|x| answer(vec![n(x), n(3 - x)], vec![]))
                .collect(),
            4,
        ),
        case(
            "app-add-repeated",
            chr_programs::arithmetic(),
            vec![c("add", [v(0), v(0), n(4)])],
            &[0],
            vec![answer(vec![n(2)], vec![])],
            1,
        ),
        case(
            "app-add-failure",
            chr_programs::arithmetic(),
            vec![c("add", [n(3), v(0), n(2)])],
            &[0],
            vec![],
            0,
        ),
        case(
            "app-sub",
            chr_programs::arithmetic(),
            vec![c("sub", [n(5), n(2), v(0)])],
            &[0],
            vec![answer(vec![n(3)], vec![])],
            1,
        ),
    ];
    let a = |x, y| t("a", [x, y]);
    let identity = a(a(atom("s"), atom("k")), atom("k"));
    let d = a(a(atom("s"), atom("s")), a(atom("s"), atom("k")));
    let x = t("c", [atom("x")]);
    let y = t("c", [atom("y")]);
    for (id, program, result) in [
        ("app-sk-identity", a(identity.clone(), x.clone()), x.clone()),
        (
            "app-sk-duplication",
            a(a(d.clone(), x.clone()), y.clone()),
            a(a(x.clone(), y.clone()), y.clone()),
        ),
    ] {
        let mut app = case(
            id,
            chr_programs::sk(),
            vec![c("eval", [t("p", [program, atom("nil")]), v(0)])],
            &[0],
            vec![answer(vec![result], vec![])],
            1,
        );
        app.budget = 20_000;
        cases.push(app);
    }
    let wrapped = a(a(atom("k"), d.clone()), v(0));
    cases.push(case(
        "app-sk-ignored-hole",
        chr_programs::sk(),
        vec![
            c("no_c", [wrapped.clone()]),
            c(
                "eval",
                [
                    t("p", [a(a(wrapped, x.clone()), y.clone()), atom("nil")]),
                    v(1),
                ],
            ),
        ],
        &[0, 1],
        vec![answer(
            vec![v(0), a(a(x, y.clone()), y)],
            vec![c("no_c", [v(0)])],
        )],
        1,
    ));
    cases.push(case(
        "app-infer-identity",
        chr_programs::typing(),
        vec![c("infer", [identity, v(0)])],
        &[0],
        vec![answer(vec![t("fun", [v(0), v(0)])], vec![])],
        1,
    ));
    cases.push(case(
        "app-lambda-identity",
        chr_programs::lambda(),
        vec![c(
            "step",
            [
                t("app", [t("lam", [atom("x"), atom("x")]), atom("a")]),
                v(0),
            ],
        )],
        &[0],
        vec![answer(vec![atom("a")], vec![])],
        1,
    ));
    cases.push(case(
        "app-lambda-norm-residual",
        chr_programs::lambda(),
        vec![c("norm", [atom("x")])],
        &[],
        vec![answer(vec![], vec![c("norm", [atom("x")])])],
        1,
    ));
    let mut synth = case(
        "app-type-synthesis-prefix",
        chr_programs::typing(),
        vec![c(
            "infer",
            [
                v(0),
                t("fun", [atom("u"), t("fun", [atom("v"), atom("u")])]),
            ],
        )],
        &[0],
        vec![answer(vec![atom("k")], vec![])],
        1,
    );
    synth.exhausted = false;
    synth.answer_limit = Some(1);
    synth.budget = 1000;
    cases.push(synth);
    cases.sort_by(|a, b| a.id.cmp(&b.id));
    cases
}

pub fn registry() -> Vec<Case> {
    let mut cases = semantic_cases();
    cases.extend(application_cases());
    for k in [0, 1, 2, 3] {
        for w in [0, 1, 4] {
            for noise in [0, 3] {
                cases.push(carry_case(k, w, noise));
            }
        }
    }
    cases.sort_by(|a, b| a.id.cmp(&b.id));
    cases
}

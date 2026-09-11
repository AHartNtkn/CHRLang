use chr_syntax::{and, atom, c, eq, or, t, v, Answer, Query, Rule, Term, Var};
#[derive(Clone, Copy)]
pub struct Schema {
    pub build: bool,
    pub depth: usize,
    pub tail: usize,
    pub payload: usize,
    pub distinct: bool,
}
fn nat(n: usize) -> Term {
    (0..n).fold(atom("z"), |x, _| t("s", [x]))
}
fn is_nat(mut term: &Term) -> bool {
    loop {
        match term {
            Term::App(n, a) if n == "s" && a.len() == 1 => term = &a[0],
            Term::App(n, a) => return n == "z" && a.is_empty(),
            _ => return false,
        }
    }
}
fn ground(t: &Term) -> bool {
    match t {
        Term::Var(_) => false,
        Term::App(_, a) => a.iter().all(ground),
    }
}
impl Schema {
    pub fn rules(self) -> Vec<Rule> {
        vec![
            Rule::simplify(
                "work-step",
                [c("work", [t("s", [v(0)]), v(1), v(2)])],
                if self.build {
                    and([
                        c("work", [v(0), v(1), v(3)]).into(),
                        eq(v(2), t("wrap", [v(3)])),
                    ])
                } else {
                    c("work", [v(0), v(1), v(2)]).into()
                },
            ),
            Rule::simplify(
                "work-base",
                [c("work", [atom("z"), v(0), v(1)])],
                or(
                    eq(v(1), t("pair", [v(0), v(99), v(99)])),
                    eq(v(1), t("pair", [v(0), v(99), v(99)])),
                ),
            ),
            Rule::simplify(
                "tail-step",
                [c("tail", [t("s", [v(0)]), v(1), v(2)])],
                c("tail", [v(0), v(1), v(2)]).into(),
            ),
            Rule::simplify(
                "tail-base",
                [c("tail", [atom("z"), v(0), v(1)])],
                eq(v(1), t("seen", [v(0), v(0)])),
            ),
        ]
    }
    pub fn query(self, index: usize) -> Query {
        let arg = (0..self.payload).fold(
            atom(&format!("seed-{}", if self.distinct { index } else { 0 })),
            |x, _| t("data", [x]),
        );
        Query {
            constraints: vec![
                c("work", [nat(self.depth), arg, v(8)]),
                c("tail", [nat(self.tail), v(8), v(9)]),
                c("marker", [atom(&format!("caller-{index}"))]),
            ],
            outputs: vec![("result".into(), Var(8)), ("later".into(), Var(9))],
        }
    }
}
pub enum Prepared {
    Call(chr_reuse::calls::Caller),
    Reuse(chr_reuse::continuations::Prepared),
    Compiled(chr_compiled::PreparedRuleset),
    Lowered(Schema),
}
pub enum Running {
    Call(Box<chr_reuse::calls::CallerRun>),
    Reuse(Box<chr_reuse::continuations::Search>),
    Compiled(Box<chr_compiled::SearchEngine>),
    Lowered {
        arg: Term,
        marker: chr_syntax::Constraint,
        build: usize,
        remaining: usize,
    },
}
pub enum Event {
    Progress,
    Answer(Answer),
    Done,
}
impl Prepared {
    pub fn new(mode: &str, schema: Schema) -> Self {
        let rules = schema.rules();
        match mode {
            "call-direct" | "call-memo" => {
                Self::Call(chr_reuse::calls::Caller::new(rules, 2, mode == "call-memo").unwrap())
            }
            "direct" | "whole" => Self::Reuse(
                chr_reuse::continuations::Prepared::new(
                    rules,
                    if mode == "direct" {
                        chr_reuse::continuations::Mode::Direct
                    } else {
                        chr_reuse::continuations::Mode::CompactLive
                    },
                )
                .unwrap(),
            ),
            "scan" | "sealed" => {
                let p = chr_compiled::PreparedRuleset::new(rules, None).unwrap();
                Self::Compiled(if mode == "sealed" {
                    p.specialize_inferred()
                } else {
                    p
                })
            }
            "lowered" => {
                assert_eq!(rules, schema.rules());
                Self::Lowered(schema)
            }
            _ => panic!("mode"),
        }
    }
    pub fn start(&self, query: Query) -> Running {
        match self {
            Self::Call(p) => Running::Call(Box::new(p.start(query, 2_000_000, 2_000_000))),
            Self::Reuse(p) => Running::Reuse(Box::new(p.start(query).unwrap())),
            Self::Compiled(p) => Running::Compiled(Box::new(
                p.start_search(
                    query,
                    chr_compiled::Policy::Global,
                    chr_compiled::Access::Scan,
                )
                .unwrap(),
            )),
            Self::Lowered(schema) => {
                assert_eq!(
                    query.outputs,
                    vec![("result".into(), Var(8)), ("later".into(), Var(9))]
                );
                assert_eq!(query.constraints.len(), 3);
                let (work, tail, marker) = (
                    &query.constraints[0],
                    &query.constraints[1],
                    &query.constraints[2],
                );
                assert_eq!(work.name, "work");
                assert_eq!(work.args.len(), 3);
                assert!(is_nat(&work.args[0]) && ground(&work.args[1]));
                assert_eq!(work.args[2], v(8));
                assert_eq!(tail.name, "tail");
                assert_eq!(tail.args.len(), 3);
                assert!(is_nat(&tail.args[0]));
                assert_eq!(tail.args[1..], [v(8), v(9)]);
                assert_eq!(marker.name, "marker");
                assert!(marker.args.iter().all(ground));
                let mut depth = 0;
                let mut x = &work.args[0];
                while let Term::App(name, args) = x {
                    if name != "s" {
                        break;
                    }
                    depth += 1;
                    x = &args[0];
                }
                let mut facts = query.constraints.into_iter();
                let mut work = facts.next().unwrap();
                let _tail = facts.next().unwrap();
                let marker = facts.next().unwrap();
                let arg = work.args.swap_remove(1);
                Running::Lowered {
                    arg,
                    marker,
                    build: if schema.build { depth } else { 0 },
                    remaining: 2,
                }
            }
        }
    }
    pub fn tick(&mut self, run: &mut Running) -> Event {
        match (self, run) {
            (Self::Call(p), Running::Call(r)) => match p.step(r).unwrap() {
                chr_reuse::calls::CallerEvent::Progress => Event::Progress,
                chr_reuse::calls::CallerEvent::Answer(a) => Event::Answer(a),
                chr_reuse::calls::CallerEvent::Done => Event::Done,
            },
            (_, Running::Reuse(r)) => {
                let mut b = r.advance(1);
                if let Some(a) = b.answers.pop() {
                    Event::Answer(a)
                } else if b.exhausted {
                    Event::Done
                } else {
                    Event::Progress
                }
            }
            (_, Running::Compiled(r)) => match r.tick() {
                chr_compiled::SearchEvent::Complete(mut b) => {
                    Event::Answer(b.engine.observe().unwrap())
                }
                chr_compiled::SearchEvent::Exhausted => Event::Done,
                _ => Event::Progress,
            },
            (
                _,
                Running::Lowered {
                    arg,
                    marker,
                    build,
                    remaining,
                },
            ) => {
                if *remaining == 0 {
                    return Event::Done;
                }
                *remaining -= 1;
                let result =
                    (0..*build).fold(t("pair", [arg.clone(), v(0), v(0)]), |x, _| t("wrap", [x]));
                Event::Answer(Answer {
                    outputs: vec![
                        ("result".into(), result.clone()),
                        ("later".into(), t("seen", [result.clone(), result])),
                    ],
                    residual: vec![marker.clone()],
                })
            }
            _ => panic!("owner mismatch"),
        }
    }
}

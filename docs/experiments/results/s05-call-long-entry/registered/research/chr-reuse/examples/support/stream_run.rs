use super::source::Schema;
use chr_syntax::{Answer, Query, Rule, Term, atom, t, v};
pub const MODES: [&str; 7] = [
    "direct",
    "compact-live",
    "scan",
    "sealed",
    "dependencies",
    "templates",
    "lowered",
];
pub enum Event {
    Progress,
    Answer(Answer),
    Done,
}
pub enum Prepared {
    Reuse(chr_reuse::continuations::Prepared),
    Compiled(chr_compiled::PreparedRuleset),
    Graph(chr_direct_choice::demand::Prepared),
    Lowered(Schema),
}
pub enum Running {
    Reuse(Box<chr_reuse::continuations::Search>),
    Compiled(Box<chr_compiled::SearchEngine>),
    Graph(Box<chr_direct_choice::demand::Run>),
    Lowered {
        schema: Schema,
        index: usize,
        count: usize,
    },
}
impl Prepared {
    pub fn new(mode: &str, schema: Schema, rules: Vec<Rule>) -> Self {
        match mode {
            "direct" | "compact-live" => Self::Reuse(
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
            #[cfg(feature = "carrier-contraction")]
            "carriers" => Self::Compiled(
                chr_compiled::PreparedRuleset::new(rules, None)
                    .unwrap()
                    .specialize_inferred()
                    .contract_carriers_inferred()
                    .unwrap(),
            ),
            "templates-zero" => Self::Graph(
                chr_direct_choice::demand::Prepared::with_reuse(
                    rules,
                    chr_direct_choice::demand::Reuse::MatchDependencies,
                )
                .unwrap()
                .with_template_follow_limit(0),
            ),
            "scan" | "sealed" => {
                let p = chr_compiled::PreparedRuleset::new(rules, None).unwrap();
                Self::Compiled(if mode == "sealed" {
                    p.specialize_inferred()
                } else {
                    p
                })
            }
            "dependencies" | "templates" => {
                let p = chr_direct_choice::demand::Prepared::with_reuse(
                    rules,
                    chr_direct_choice::demand::Reuse::MatchDependencies,
                )
                .unwrap();
                Self::Graph(if mode == "templates" {
                    p.with_derivation_templates()
                } else {
                    p
                })
            }
            "lowered" => {
                assert_eq!(rules, schema.rules(), "exact-source rules required");
                Self::Lowered(schema)
            }
            _ => panic!("unknown mode"),
        }
    }
    pub fn start(&self, query: Query) -> Running {
        match self {
            Self::Reuse(p) => Running::Reuse(Box::new(p.start(query).unwrap())),
            Self::Compiled(p) => Running::Compiled(Box::new(
                p.start_search(
                    query,
                    chr_compiled::Policy::Global,
                    chr_compiled::Access::Scan,
                )
                .unwrap(),
            )),
            Self::Graph(p) => Running::Graph(Box::new(p.start(query).unwrap())),
            Self::Lowered(schema) => {
                let head = query
                    .constraints
                    .iter()
                    .find(|c| c.name == "stream")
                    .expect("stream query");
                let mut term = &head.args[0];
                let mut n = 0;
                while let Term::App(name, args) = term {
                    if name != "s" || args.len() != 1 {
                        break;
                    }
                    n += 1;
                    term = &args[0];
                }
                assert_eq!(term, &atom("z"));
                assert!(
                    query == schema.query(n, false) || query == schema.query(n, true),
                    "exact-source query required"
                );
                Running::Lowered {
                    schema: *schema,
                    index: 0,
                    count: n + usize::from(!schema.fail_tail),
                }
            }
        }
    }
}
impl Running {
    pub fn tick(&mut self) -> Event {
        match self {
            Self::Reuse(run) => {
                let mut batch = run.advance(1);
                if let Some(a) = batch.answers.pop() {
                    Event::Answer(a)
                } else if batch.exhausted {
                    Event::Done
                } else {
                    Event::Progress
                }
            }
            Self::Compiled(run) => match run.tick() {
                chr_compiled::SearchEvent::Complete(mut b) => {
                    Event::Answer(b.engine.observe().unwrap())
                }
                chr_compiled::SearchEvent::Exhausted => Event::Done,
                _ => Event::Progress,
            },
            Self::Graph(run) => match run.tick() {
                chr_direct_choice::demand::Event::Answer(a) => Event::Answer(a),
                chr_direct_choice::demand::Event::Exhausted => Event::Done,
                chr_direct_choice::demand::Event::Progress => Event::Progress,
            },
            Self::Lowered {
                schema,
                index,
                count,
            } => {
                if index == count {
                    return Event::Done;
                }
                let seed = if schema.family == "aliases" {
                    v(42)
                } else {
                    (0..if schema.family == "distinct" {
                        *index
                    } else {
                        0
                    })
                        .fold(atom("root"), |a, _| t("next", [a]))
                };
                let payload = (0..schema.payload).fold(atom("end"), |a, _| t("data", [a]));
                let value = t("item", [seed.clone(), seed, payload]);
                *index += 1;
                Event::Answer(Answer {
                    outputs: vec![("result".into(), value.clone()), ("again".into(), value)],
                    residual: vec![],
                })
            }
        }
    }
}

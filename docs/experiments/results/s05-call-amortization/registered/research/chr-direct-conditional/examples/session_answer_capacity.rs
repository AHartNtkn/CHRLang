//! Diagnose retained capacity in exact exported answers.
#![allow(dead_code)]
#[cfg(feature = "alloc-meter")]
#[path = "../../chr-compiled/experiments/meter.rs"]
mod meter;
#[allow(dead_code)]
#[path = "../tests/runtime_support/mod.rs"]
mod oracle;
#[allow(dead_code)]
#[path = "support/post_continuation_source.rs"]
mod source;
#[path = "support/static_posts.rs"]
mod static_posts;
#[path = "support/value_choices.rs"]
mod value_choices;
use chr_direct_choice::demand::{Event, Prepared as Demand, Reuse, Run};
use chr_syntax::{Answer, Query, Rule};
use std::time::Instant;

struct Reading {
    ns: u128,
    #[cfg(feature = "alloc-meter")]
    memory: meter::Reading,
}
fn measure<T>(f: impl FnOnce() -> T) -> (T, Reading) {
    #[cfg(feature = "alloc-meter")]
    let start = meter::begin();
    let clock = Instant::now();
    let value = f();
    let ns = clock.elapsed().as_nanos();
    #[cfg(feature = "alloc-meter")]
    let memory = meter::end(start);
    (
        value,
        Reading {
            ns,
            #[cfg(feature = "alloc-meter")]
            memory,
        },
    )
}
impl Reading {
    fn json(&self) -> String {
        #[cfg(feature = "alloc-meter")]
        let memory = self.memory.json();
        #[cfg(not(feature = "alloc-meter"))]
        let memory = "null";
        format!("{{\"ns\":{},\"memory\":{memory}}}", self.ns)
    }
}
enum Prepared {
    Demand(Box<Demand>),
    Conditional(Box<chr_direct_conditional::engine::PreparedRuleset>),
    Compiled(
        Box<chr_compiled::PreparedRuleset>,
        chr_compiled::Access,
        chr_compiled::Policy,
    ),
}
enum Running {
    Demand(Box<Run>),
    Conditional(Box<chr_direct_conditional::engine::Engine>),
    Compiled(Box<chr_compiled::SearchEngine>),
}
impl Prepared {
    fn new(
        mode: &str,
        rules: Vec<Rule>,
        kind: &str,
        choices: usize,
        history: bool,
        initialize: bool,
    ) -> Self {
        if mode == "conditional" {
            return Self::Conditional(Box::new(
                chr_direct_conditional::engine::PreparedRuleset::new(rules).unwrap(),
            ));
        }
        if [
            "scan",
            "indexed",
            "sealed-scan",
            "sealed-indexed",
            "active-scan",
            "active-indexed",
            "native-scan",
            "native-indexed",
            "active-native-scan",
            "active-native-indexed",
        ]
        .contains(&mode)
        {
            Self::Compiled(
                Box::new({
                    let code = if mode.contains("native") {
                        let id = source::FAMILIES.iter().position(|f| *f == kind).unwrap() * 6
                            + [0, 1, 3].iter().position(|k| *k == choices).unwrap() * 2
                            + usize::from(history);
                        Some(if initialize {
                            chr_compiled::access_initialized_continuation_bundled(id)
                        } else {
                            chr_compiled::access_continuation_bundled(id)
                        })
                    } else {
                        None
                    };
                    let p = chr_compiled::PreparedRuleset::new(rules, code).unwrap();
                    if mode.starts_with("sealed-") {
                        p.specialize_inferred()
                    } else {
                        p
                    }
                }),
                if mode.ends_with("scan") {
                    chr_compiled::Access::Scan
                } else {
                    chr_compiled::Access::Indexed
                },
                if mode.starts_with("active-") {
                    chr_compiled::Policy::Active
                } else {
                    chr_compiled::Policy::Global
                },
            )
        } else {
            let base = mode.strip_suffix("-template").unwrap_or(mode);
            let reuse = match base.strip_suffix("-miss").unwrap_or(base) {
                "current" => Reuse::CurrentContext,
                "birth" => Reuse::StaticBirth,
                "dependencies" => Reuse::MatchDependencies,
                _ => panic!("unknown mode"),
            };
            let mut p = Demand::with_reuse(rules, reuse).unwrap();
            if base.ends_with("-miss") {
                p = p.with_miss_reuse();
            }
            if mode.ends_with("-template") {
                p = p.with_derivation_templates();
            }
            Self::Demand(Box::new(p))
        }
    }
    fn start(&self, q: Query) -> Running {
        match self {
            Self::Demand(p) => Running::Demand(Box::new(p.start(q).unwrap())),
            Self::Conditional(p) => Running::Conditional(Box::new(p.start(q).unwrap())),
            Self::Compiled(p, access, policy) => {
                Running::Compiled(Box::new(p.start_search(q, *policy, *access).unwrap()))
            }
        }
    }
}
impl Running {
    fn collect(
        &mut self,
        cancel: bool,
        first_clock: bool,
    ) -> (Vec<Answer>, bool, usize, Option<u128>) {
        let start = first_clock.then(Instant::now);
        let mut first = None;
        let mut answers = vec![];
        for tick in 1..=2_000_000 {
            match self {
                Self::Conditional(r) => match r.tick() {
                    chr_direct_conditional::engine::Event::Answer(a) => {
                        answers.push(a);
                        if first.is_none() {
                            first = start.map(|clock| clock.elapsed().as_nanos());
                        }
                    }
                    chr_direct_conditional::engine::Event::Exhausted => {
                        return (answers, true, tick, first);
                    }
                    _ => (),
                },
                Self::Demand(r) => match r.tick() {
                    Event::Answer(a) => {
                        answers.push(a);
                        if first.is_none() {
                            first = start.map(|clock| clock.elapsed().as_nanos());
                        }
                    }
                    Event::Exhausted => return (answers, true, tick, first),
                    Event::Progress => (),
                },
                Self::Compiled(r) => match r.tick() {
                    chr_compiled::SearchEvent::Complete(mut b) => {
                        answers.push(b.engine.observe().unwrap());
                        if first.is_none() {
                            first = start.map(|clock| clock.elapsed().as_nanos());
                        }
                    }
                    chr_compiled::SearchEvent::Exhausted => return (answers, true, tick, first),
                    _ => (),
                },
            }
            if cancel {
                return (answers, false, tick, first);
            }
        }
        panic!("service cutoff")
    }
}
fn validate(actual: &[Answer], expected: &[Answer], complete: bool) {
    if complete {
        oracle::same_raw(actual.to_vec(), expected.to_vec());
    } else {
        let mut remaining = expected.to_vec();
        for answer in actual {
            let i = remaining
                .iter()
                .position(|candidate| {
                    chr_observe::equivalent(candidate, answer, &mut Default::default())
                })
                .expect("invalid canceled output");
            remaining.swap_remove(i);
        }
    }
}

fn spare_term(t: &chr_syntax::Term) -> usize {
    match t {
        chr_syntax::Term::Var(_) => 0,
        chr_syntax::Term::App(name, args) => {
            name.capacity() - name.len()
                + (args.capacity() - args.len()) * std::mem::size_of::<chr_syntax::Term>()
                + args.iter().map(spare_term).sum::<usize>()
        }
    }
}
fn spare(a: &Answer) -> usize {
    (a.outputs.capacity() - a.outputs.len()) * std::mem::size_of::<(String, chr_syntax::Term)>()
        + (a.residual.capacity() - a.residual.len()) * std::mem::size_of::<chr_syntax::Constraint>()
        + a.outputs
            .iter()
            .map(|(name, t)| name.capacity() - name.len() + spare_term(t))
            .sum::<usize>()
        + a.residual
            .iter()
            .map(|c| {
                c.name.capacity() - c.name.len()
                    + (c.args.capacity() - c.args.len()) * std::mem::size_of::<chr_syntax::Term>()
                    + c.args.iter().map(spare_term).sum::<usize>()
            })
            .sum::<usize>()
}
fn probe(mode: &str, family: &str) -> usize {
    let rules = source::rules(family, 3, true);
    let count = chr_compiled::resource_count::Program::infer(&rules).unwrap();
    let (rules, init) = static_posts::Prepared::infer(&rules).unwrap().into_parts();
    let rules = if mode == "birth-miss" {
        value_choices::lower(&rules)
    } else {
        rules
    };
    let p = Prepared::new(mode, rules, family, 3, true, true);
    let q = init.initialize(&count.lower(&source::query(family, 3, 4, 0, false)).unwrap());
    let mut run = p.start(q);
    let (answers, complete, _, _) = run.collect(false, false);
    assert!(complete);
    validate(&answers, &source::expected(family, 3, 0, true), true);
    answers.iter().map(spare).sum()
}
fn main() {
    for family in ["common", "independent"] {
        for mode in ["birth-miss", "conditional", "native-scan"] {
            println!(
                "{family}/{mode}: {} spare answer bytes",
                probe(mode, family)
            );
        }
    }
}
#[test]
fn common_export_fits_the_exact_owned_answer_budget() {
    // The shared owned syntax can hold this complete source with no spare bytes.
    // Check actual exported consumer objects, after independent answer validation.
    assert_eq!(probe("birth-miss", "common"), 0);
}

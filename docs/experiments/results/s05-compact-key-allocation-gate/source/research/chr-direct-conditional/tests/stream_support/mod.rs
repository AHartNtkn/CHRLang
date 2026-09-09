//! Reusable source fixtures and closed-form, allocation-free answer checking.
//! No backend, evaluator, or timing dependencies belong in this module.
use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, and, atom, c, or, t, v};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    Ground,
    Aliases,
}
impl Family {
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "ground" => Some(Self::Ground),
            "aliases" => Some(Self::Aliases),
            _ => None,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Ground => "ground",
            Self::Aliases => "aliases",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extent {
    Unbounded,
    Finite(usize),
}
pub struct Fixture {
    pub family: Family,
    pub rules: Vec<Rule>,
    pub query: Query,
    /// Exact number of successful raw answers, including identical duplicates.
    pub finite_answers: Option<usize>,
}
fn success(family: Family) -> Goal {
    match family {
        Family::Ground => c("result", [atom("a")]).into(),
        // The local is fresh per application and shared across both residuals.
        Family::Aliases => and(vec![
            c("result", [v(1)]).into(),
            c("pair", [v(1), v(1)]).into(),
        ]),
    }
}
pub fn fixture(family: Family, extent: Extent) -> Fixture {
    let (rules, initial, finite_answers) = match extent {
        Extent::Unbounded => (
            vec![Rule::simplify(
                "emit_or_recur",
                [c("loop", [])],
                or(success(family), c("loop", []).into()),
            )],
            c("loop", []),
            None,
        ),
        Extent::Finite(n) => {
            let numeral = (0..n).fold(atom("z"), |rest, _| t("s", [rest]));
            (
                vec![
                    Rule::simplify("stop", [c("loop", [atom("z")])], Goal::Fail),
                    Rule::simplify(
                        "emit_or_recur",
                        [c("loop", [t("s", [v(0)])])],
                        or(success(family), c("loop", [v(0)]).into()),
                    ),
                ],
                c("loop", [numeral]),
                Some(n),
            )
        }
    };
    Fixture {
        family,
        rules,
        query: Query {
            constraints: vec![initial],
            outputs: vec![],
        },
        finite_answers,
    }
}
/// Checks the complete answer without allocation, normalization, or an evaluator.
/// Names and free-variable identities are inspected directly; residual order is
/// immaterial, but multiplicity and joint aliases must match exactly.
pub fn valid_answer(family: Family, answer: &Answer) -> bool {
    if !answer.outputs.is_empty() {
        return false;
    }
    match family {
        Family::Ground => {
            let [result] = answer.residual.as_slice() else {
                return false;
            };
            let [Term::App(name, args)] = result.args.as_slice() else {
                return false;
            };
            result.name == "result" && name == "a" && args.is_empty()
        }
        Family::Aliases => {
            if answer.residual.len() != 2 {
                return false;
            }
            let mut result: Option<Var> = None;
            let mut pair: Option<Var> = None;
            for constraint in &answer.residual {
                match (constraint.name.as_str(), constraint.args.as_slice()) {
                    ("result", [Term::Var(x)]) if result.is_none() => result = Some(*x),
                    ("pair", [Term::Var(x), Term::Var(y)]) if pair.is_none() && x == y => {
                        pair = Some(*x)
                    }
                    _ => return false,
                }
            }
            result.is_some() && result == pair
        }
    }
}

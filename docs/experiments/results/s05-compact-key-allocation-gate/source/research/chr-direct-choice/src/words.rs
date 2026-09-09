//! Checked direct lowering of a closed word-generation source schema.
//! No choice graph, source matcher or per-history CHR state is used.
use chr_syntax::{Answer, Goal, Query, Rule, Term, Var, and, atom, c, eq, or, t, v};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug)]
pub enum Alphabet {
    Binary,
    Duplicate,
    Nested,
}
pub fn source(alphabet: Alphabet, reject: bool) -> Vec<Rule> {
    let assign = |name: &str| eq(v(1), t("cons", [atom(name), v(2)]));
    let choice = match alphabet {
        Alphabet::Binary => or(assign("a"), assign("b")),
        Alphabet::Duplicate => or(assign("a"), assign("a")),
        Alphabet::Nested => or(assign("a"), or(assign("b"), assign("c"))),
    };
    let mut rules = vec![
        Rule::propagate("watch", [c("token", [v(0)])], c("seen", [v(0)]).into()),
        Rule::simplify(
            "base",
            [c("build", [atom("z"), v(0)])],
            eq(v(0), atom("nil")),
        ),
        Rule::simplify(
            "step",
            [c("build", [t("s", [v(0)]), v(1)])],
            and([choice, c("build", [v(0), v(2)]).into()]),
        ),
        Rule::simplify(
            "use",
            [c("task", [v(0), v(1)]), c("token", [v(2)])],
            c("out", [v(0), v(0), v(1), v(1), v(2), v(2)]).into(),
        ),
    ];
    if reject {
        rules.push(Rule::simplify(
            "reject",
            [c(
                "out",
                [t("cons", [atom("b"), v(0)]), v(1), v(2), v(3), v(4), v(5)],
            )],
            Goal::Fail,
        ));
    }
    rules
}

pub struct Prepared {
    alphabet: Alphabet,
    reject: bool,
}
pub struct Words {
    letters: &'static [&'static str],
    first_letters: Vec<&'static str>,
    digits: Vec<usize>,
    left_depth: usize,
    independent: bool,
    handle: Var,
    finished: bool,
}
impl Prepared {
    pub fn new(rules: &[Rule]) -> Result<Self, String> {
        for alphabet in [Alphabet::Binary, Alphabet::Duplicate, Alphabet::Nested] {
            for reject in [false, true] {
                if rules == source(alphabet, reject) {
                    return Ok(Self { alphabet, reject });
                }
            }
        }
        Err("unsupported source: requires the exact checked word schema".into())
    }
    pub fn start(&self, query: Query) -> Result<Words, String> {
        if !query.outputs.is_empty() {
            return Err("selected outputs are outside this certificate".into());
        }
        let mut builds = BTreeMap::new();
        let mut task = None;
        let mut token = None;
        for constraint in query.constraints {
            match (constraint.name.as_str(), constraint.args.as_slice()) {
                ("build", [depth, Term::Var(target)]) => {
                    let depth =
                        depth_value(depth).ok_or("build depth must be a closed unary natural")?;
                    if builds.insert(*target, depth).is_some() {
                        return Err("duplicate build target".into());
                    }
                }
                ("task", [Term::Var(x), Term::Var(y)]) => {
                    if task.replace((*x, *y)).is_some() {
                        return Err("duplicate task".into());
                    }
                }
                ("token", [Term::Var(handle)]) => {
                    if token.replace(*handle).is_some() {
                        return Err("duplicate token".into());
                    }
                }
                _ => return Err("query is outside the checked word schema".into()),
            }
        }
        let (x, y) = task.ok_or("missing task")?;
        let handle = token.ok_or("missing token")?;
        if handle == x || handle == y {
            return Err("token handle must be independent of word outputs".into());
        }
        let left_depth = *builds.get(&x).ok_or("missing left build")?;
        let right_depth = *builds.get(&y).ok_or("missing right build")?;
        let independent = x != y;
        if builds.len() != if independent { 2 } else { 1 } {
            return Err("unreferenced build".into());
        }
        let depth = left_depth
            .checked_add(if independent { right_depth } else { 0 })
            .ok_or("depth overflow")?;
        let letters: &[&str] = match self.alphabet {
            Alphabet::Binary => &["a", "b"],
            Alphabet::Duplicate => &["a", "a"],
            Alphabet::Nested => &["a", "b", "c"],
        };
        let first_letters = letters
            .iter()
            .copied()
            .filter(|letter| !(self.reject && left_depth > 0 && *letter == "b"))
            .collect();
        Ok(Words {
            letters,
            first_letters,
            digits: vec![0; depth],
            left_depth,
            independent,
            handle,
            finished: false,
        })
    }
}
fn depth_value(mut term: &Term) -> Option<usize> {
    let mut depth = 0usize;
    loop {
        match term {
            Term::App(name, args) if name == "z" && args.is_empty() => return Some(depth),
            Term::App(name, args) if name == "s" && args.len() == 1 => {
                depth = depth.checked_add(1)?;
                term = &args[0];
            }
            _ => return None,
        }
    }
}
impl Words {
    fn word(&self, start: usize, end: usize) -> Term {
        let mut value = atom("nil");
        for index in (start..end).rev() {
            let letter = if index == 0 && self.left_depth > 0 {
                self.first_letters[self.digits[index]]
            } else {
                self.letters[self.digits[index]]
            };
            value = t("cons", [atom(letter), value]);
        }
        value
    }
    fn advance(&mut self) {
        for index in (0..self.digits.len()).rev() {
            self.digits[index] += 1;
            let base = if index == 0 && self.left_depth > 0 {
                self.first_letters.len()
            } else {
                self.letters.len()
            };
            if self.digits[index] < base {
                return;
            }
            self.digits[index] = 0;
        }
        self.finished = true;
    }
}
impl Iterator for Words {
    type Item = Answer;
    fn next(&mut self) -> Option<Answer> {
        if self.finished {
            return None;
        }
        let x = self.word(0, self.left_depth);
        let y = if self.independent {
            self.word(self.left_depth, self.digits.len())
        } else {
            x.clone()
        };
        let h = Term::Var(self.handle);
        let answer = Answer {
            outputs: vec![],
            residual: vec![
                c("seen", [h.clone()]),
                c("out", [x.clone(), x, y.clone(), y, h.clone(), h]),
            ],
        };
        self.advance();
        Some(answer)
    }
}

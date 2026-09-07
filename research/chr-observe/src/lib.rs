//! Exact experimental answer comparison, independent of reference normalization.
//!
//! Outputs fix a partial variable bijection. Residual occurrences are matched as
//! a multiset under extensions of that same bijection; failed matches roll back.
use chr_syntax::{Answer, Constraint, Term, Var};

#[derive(Default, Debug)]
pub struct Stats {
    pub term_pairs: u64,
    pub occurrence_scans: u64,
    pub occurrence_candidates: u64,
    pub backtracks: u64,
}
#[derive(Clone, Default)]
struct Bijection(Vec<(Var, Var)>);
impl Bijection {
    fn terms(&mut self, left: &Term, right: &Term, stats: &mut Stats) -> bool {
        stats.term_pairs += 1;
        match (left, right) {
            (Term::Var(a), Term::Var(b)) => {
                if let Some((_, mapped)) = self.0.iter().find(|(x, _)| x == a) {
                    return mapped == b;
                }
                if self.0.iter().any(|(_, y)| y == b) {
                    return false;
                }
                self.0.push((*a, *b));
                true
            }
            (Term::App(a, x), Term::App(b, y)) => {
                a == b
                    && x.len() == y.len()
                    && x.iter().zip(y).all(|(x, y)| self.terms(x, y, stats))
            }
            _ => false,
        }
    }
}
pub fn equivalent(left: &Answer, right: &Answer, stats: &mut Stats) -> bool {
    if left.outputs.len() != right.outputs.len() || left.residual.len() != right.residual.len() {
        return false;
    }
    let mut mapping = Bijection::default();
    for ((name, a), (other, b)) in left.outputs.iter().zip(&right.outputs) {
        if name != other || !mapping.terms(a, b, stats) {
            return false;
        }
    }
    match_occurrences(
        &left.residual,
        &right.residual,
        &mut vec![false; right.residual.len()],
        mapping,
        stats,
    )
}
fn match_occurrences(
    left: &[Constraint],
    right: &[Constraint],
    used: &mut [bool],
    mapping: Bijection,
    stats: &mut Stats,
) -> bool {
    let Some((first, rest)) = left.split_first() else {
        return true;
    };
    for (i, candidate) in right.iter().enumerate() {
        stats.occurrence_scans += 1;
        if used[i] || first.name != candidate.name || first.args.len() != candidate.args.len() {
            continue;
        }
        stats.occurrence_candidates += 1;
        let mut extended = mapping.clone();
        if first
            .args
            .iter()
            .zip(&candidate.args)
            .all(|(a, b)| extended.terms(a, b, stats))
        {
            used[i] = true;
            if match_occurrences(rest, right, used, extended, stats) {
                return true;
            }
            used[i] = false;
        }
        stats.backtracks += 1;
    }
    false
}

/// Keeps one actual answer per known alpha-equivalence class; never merges continuations.
#[derive(Default)]
pub struct AnswerSet {
    answers: Vec<Answer>,
    pub stats: Stats,
}
impl AnswerSet {
    pub fn insert(&mut self, answer: Answer) -> bool {
        if self
            .answers
            .iter()
            .any(|old| equivalent(old, &answer, &mut self.stats))
        {
            return false;
        }
        self.answers.push(answer);
        true
    }
    pub fn into_answers(self) -> Vec<Answer> {
        self.answers
    }
}

pub mod graph;

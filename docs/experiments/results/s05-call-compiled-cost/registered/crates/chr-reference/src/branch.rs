use crate::{Stats, instantiate, matching, unify::Substitution};
use chr_syntax::{Answer, Constraint, Goal, Query, Rule, Term};
use std::collections::{BTreeSet, VecDeque};

#[derive(Clone)]
pub(crate) struct Occurrence {
    pub id: u64,
    pub constraint: Constraint,
}
pub(crate) type Token = (usize, Vec<u64>);

/// All mutable state is owned. Clone only occurs at an explicit source OR.
#[derive(Clone)]
pub(crate) struct Branch {
    pub pending: VecDeque<Goal>,
    pub store: Vec<Occurrence>,
    pub substitution: Substitution,
    pub history: BTreeSet<Token>,
    pub next_var: u64,
    next_occurrence: u64,
    outputs: Vec<(String, Term)>,
}
pub(crate) enum Step {
    Continue,
    Split(Goal, Goal),
    Failed,
    Answer(Answer),
}

impl Branch {
    pub fn new(query: Query) -> Self {
        let mut bindings = instantiate::Bindings::new();
        let mut next_var = 0;
        // Dense internal renaming keeps even very large external IDs innocuous.
        let pending = query
            .constraints
            .iter()
            .map(|c| Goal::Constraint(instantiate::constraint(c, &mut bindings, &mut next_var)))
            .collect();
        let outputs = query
            .outputs
            .into_iter()
            .map(|(name, var)| {
                (
                    name,
                    instantiate::term(&Term::Var(var), &mut bindings, &mut next_var),
                )
            })
            .collect();
        Self {
            pending,
            store: vec![],
            substitution: Substitution::default(),
            history: BTreeSet::new(),
            next_var,
            next_occurrence: 0,
            outputs,
        }
    }

    pub fn step(&mut self, rules: &[Rule], stats: &mut Stats) -> Step {
        if let Some(goal) = self.pending.pop_front() {
            match goal {
                Goal::Constraint(constraint) => {
                    self.store.push(Occurrence {
                        id: self.next_occurrence,
                        constraint,
                    });
                    self.next_occurrence += 1;
                    stats.introductions += 1;
                }
                Goal::Unify(a, b) => {
                    stats.equations += 1;
                    if !self
                        .substitution
                        .unify(&a, &b, &mut stats.unification_pairs)
                    {
                        return Step::Failed;
                    }
                }
                Goal::And(goals) => {
                    // Flatten at the current FIFO position. Conjunction has no source order guarantee.
                    for goal in goals.into_iter().rev() {
                        self.pending.push_front(goal);
                    }
                }
                Goal::Or(a, b) => return Step::Split(*a, *b),
                Goal::True => {}
                Goal::Fail => return Step::Failed,
            }
            return Step::Continue;
        }
        if let Some(application) = matching::find(self, rules, stats) {
            self.store.retain(|o| !application.removed.contains(&o.id));
            self.history.insert(application.token);
            self.next_var = application.next_var;
            self.pending.push_back(application.body);
            stats.rule_applications += 1;
            Step::Continue
        } else {
            // An exhaustive scan found no enabled unused tuple; residuals may be nonground.
            Step::Answer(Answer {
                outputs: self
                    .outputs
                    .iter()
                    .map(|(n, t)| (n.clone(), self.substitution.resolve(t)))
                    .collect(),
                residual: self
                    .store
                    .iter()
                    .map(|o| Constraint {
                        name: o.constraint.name.clone(),
                        args: o
                            .constraint
                            .args
                            .iter()
                            .map(|t| self.substitution.resolve(t))
                            .collect(),
                    })
                    .collect(),
            })
        }
    }
}

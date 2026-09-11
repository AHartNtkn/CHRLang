//! Nonbinding matching over opaque values and demanded conditional structure.
use crate::equality::{Binding, Change, DemandJob, DemandStatus, Store, Term, TermView};
use crate::support::{Arena, Job, Operation, Status, Support};
use std::collections::BTreeSet;
use std::sync::Arc;
#[derive(Clone, Debug)]
pub enum Pattern {
    Slot(usize),
    Constructor { name: String, args: Vec<Pattern> },
}
pub(crate) enum Node {
    Slot(usize),
    Constructor { name: String, args: Vec<usize> },
}
#[derive(Clone)]
pub struct Plan {
    nodes: Arc<Vec<Node>>,
    roots: Arc<Vec<usize>>,
    slots: usize,
}
impl Plan {
    pub fn new(patterns: Vec<Pattern>, slots: usize) -> Result<Self, String> {
        fn lower(pattern: Pattern, nodes: &mut Vec<Node>, slots: usize) -> Result<usize, String> {
            let node = match pattern {
                Pattern::Slot(slot) => {
                    if slot >= slots {
                        return Err("pattern slot is outside prepared frame".into());
                    }
                    Node::Slot(slot)
                }
                Pattern::Constructor { name, args } => Node::Constructor {
                    name,
                    args: args
                        .into_iter()
                        .map(|p| lower(p, nodes, slots))
                        .collect::<Result<_, _>>()?,
                },
            };
            let id = nodes.len();
            nodes.push(node);
            Ok(id)
        }
        let mut nodes = vec![];
        let roots = patterns
            .into_iter()
            .map(|p| lower(p, &mut nodes, slots))
            .collect::<Result<_, _>>()?;
        Ok(Self {
            nodes: Arc::new(nodes),
            roots: Arc::new(roots),
            slots,
        })
    }
    pub(crate) fn roots(&self) -> &[usize] {
        &self.roots
    }
    pub(crate) fn node(&self, id: usize) -> &Node {
        &self.nodes[id]
    }
    pub fn start(
        &self,
        store: &Store,
        region: Support,
        terms: Vec<Term>,
    ) -> Result<MatchJob, String> {
        if terms.len() != self.roots.len() {
            return Err("term arity differs from prepared pattern arity".into());
        }
        for term in &terms {
            store.inspect(*term);
        }
        Ok(self.begin(
            store,
            region,
            Arc::clone(&self.roots),
            terms,
            vec![None; self.slots],
            true,
        ))
    }
    pub(crate) fn start_bound(
        &self,
        root: usize,
        store: &Store,
        region: Support,
        term: Term,
        slots: Vec<Option<Term>>,
    ) -> MatchJob {
        self.begin(
            store,
            region,
            Arc::new(vec![root]),
            vec![term],
            slots,
            false,
        )
    }
    fn begin(
        &self,
        store: &Store,
        region: Support,
        roots: Arc<Vec<usize>>,
        terms: Vec<Term>,
        slots: Vec<Option<Term>>,
        bind_missing: bool,
    ) -> MatchJob {
        MatchJob {
            plan: self.clone(),
            roots,
            terms: Arc::new(terms),
            identity: store.identity(),
            version: store.version(),
            original: region,
            current: Some(Frame {
                support: region,
                slots,
                tasks: vec![Task::Roots(0)],
            }),
            alternatives: vec![],
            wait: None,
            dependencies: vec![],
            seen: BTreeSet::new(),
            bind_missing,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matched {
    pub support: Support,
    pub slots: Vec<Option<Term>>,
}
pub enum MatchStatus {
    Pending,
    Match(Matched),
    Done,
    Stale,
}
#[derive(Clone)]
enum Task {
    Roots(usize),
    Pattern {
        pattern: usize,
        term: Term,
    },
    Children {
        pattern: usize,
        term: Term,
        index: usize,
    },
    Resolve {
        pattern: usize,
        variable: usize,
        index: usize,
    },
}
#[derive(Clone)]
struct Frame {
    support: Support,
    slots: Vec<Option<Term>>,
    tasks: Vec<Task>,
}
enum Wait {
    #[cfg(feature = "equality-binding-coverage")]
    Coverage {
        job: Job,
        pattern: usize,
        variable: usize,
    },
    Overlap {
        job: Job,
        pattern: usize,
        variable: usize,
        index: usize,
        binding: Binding,
    },
    Difference {
        job: Job,
        pattern: usize,
        variable: usize,
        index: usize,
        binding: Binding,
        overlap: Support,
    },
    Demand {
        job: Box<DemandJob>,
        result: Option<Support>,
        dependency: usize,
    },
}
pub struct MatchJob {
    plan: Plan,
    roots: Arc<Vec<usize>>,
    terms: Arc<Vec<Term>>,
    identity: u64,
    version: u64,
    original: Support,
    current: Option<Frame>,
    alternatives: Vec<Frame>,
    wait: Option<Wait>,
    dependencies: Vec<Change>,
    seen: BTreeSet<usize>,
    bind_missing: bool,
}
impl MatchJob {
    pub fn dependencies(&self) -> &[Change] {
        &self.dependencies
    }
    fn touch(&mut self, variable: usize) {
        if self.seen.insert(variable) {
            self.dependencies.push(Change {
                variable,
                support: self.original,
            });
        }
    }
    /// One support/demand step or pattern/guarded-edge action. Branch frame copies
    /// are bounded by the prepared rule's slots and pattern depth, never by worlds.
    pub fn tick(&mut self, store: &Store, arena: &mut Arena) -> MatchStatus {
        if self.identity != store.identity() || self.version != store.version() {
            return MatchStatus::Stale;
        }
        if self.current.is_none() {
            self.current = self.alternatives.pop();
            return if self.current.is_some() {
                MatchStatus::Pending
            } else {
                MatchStatus::Done
            };
        }
        if let Some(wait) = self.wait.take() {
            match wait {
                #[cfg(feature = "equality-binding-coverage")]
                Wait::Coverage {
                    mut job,
                    pattern,
                    variable,
                } => match job.tick(arena) {
                    Status::Pending => {
                        self.wait = Some(Wait::Coverage {
                            job,
                            pattern,
                            variable,
                        })
                    }
                    Status::Complete(overlap) => {
                        if overlap == Support::FALSE {
                            self.current = None;
                        } else {
                            self.current.as_mut().unwrap().tasks.push(Task::Resolve {
                                pattern,
                                variable,
                                index: 0,
                            });
                        }
                    }
                },
                Wait::Overlap {
                    mut job,
                    pattern,
                    variable,
                    index,
                    binding,
                } => match job.tick(arena) {
                    Status::Pending => {
                        self.wait = Some(Wait::Overlap {
                            job,
                            pattern,
                            variable,
                            index,
                            binding,
                        })
                    }
                    Status::Complete(overlap) => {
                        self.wait = Some(Wait::Difference {
                            job: arena.job(Operation::Difference(
                                self.current.as_ref().unwrap().support,
                                binding.support,
                            )),
                            pattern,
                            variable,
                            index,
                            binding,
                            overlap,
                        })
                    }
                },
                Wait::Difference {
                    mut job,
                    pattern,
                    variable,
                    index,
                    binding,
                    overlap,
                } => match job.tick(arena) {
                    Status::Pending => {
                        self.wait = Some(Wait::Difference {
                            job,
                            pattern,
                            variable,
                            index,
                            binding,
                            overlap,
                        })
                    }
                    Status::Complete(remainder) => {
                        let frame = self.current.as_mut().unwrap();
                        if overlap != Support::FALSE {
                            if remainder != Support::FALSE {
                                let mut other = frame.clone();
                                other.support = remainder;
                                other.tasks.push(Task::Resolve {
                                    pattern,
                                    variable,
                                    index: index + 1,
                                });
                                self.alternatives.push(other);
                            }
                            frame.support = overlap;
                            frame.tasks.push(Task::Pattern {
                                pattern,
                                term: binding.term,
                            });
                        } else {
                            frame.support = remainder;
                            frame.tasks.push(Task::Resolve {
                                pattern,
                                variable,
                                index: index + 1,
                            });
                        }
                    }
                },
                Wait::Demand {
                    mut job,
                    mut result,
                    mut dependency,
                } => {
                    if result.is_none() {
                        match job.tick(store, arena) {
                            DemandStatus::Pending => (),
                            DemandStatus::Complete { entailed } => result = Some(entailed),
                            DemandStatus::Stale => return MatchStatus::Stale,
                        }
                    } else if let Some(change) = job.dependencies().get(dependency) {
                        self.touch(change.variable);
                        dependency += 1;
                    } else if let Some(entailed) = result {
                        self.current.as_mut().unwrap().support = entailed;
                        return MatchStatus::Pending;
                    }
                    self.wait = Some(Wait::Demand {
                        job,
                        result,
                        dependency,
                    });
                }
            }
            return MatchStatus::Pending;
        }
        let frame = self.current.as_mut().unwrap();
        if frame.support == Support::FALSE {
            self.current = None;
            return MatchStatus::Pending;
        }
        let Some(task) = frame.tasks.pop() else {
            let frame = self.current.take().unwrap();
            return MatchStatus::Match(Matched {
                support: frame.support,
                slots: frame.slots,
            });
        };
        match task {
            Task::Roots(index) => {
                if let Some(root) = self.roots.get(index) {
                    frame.tasks.push(Task::Roots(index + 1));
                    frame.tasks.push(Task::Pattern {
                        pattern: *root,
                        term: self.terms[index],
                    });
                }
            }
            Task::Pattern { pattern, term } => match self.plan.node(pattern) {
                Node::Slot(slot) => {
                    if let Some(previous) = frame.slots[*slot] {
                        self.wait = Some(Wait::Demand {
                            job: Box::new(store.entails(frame.support, previous, term)),
                            result: None,
                            dependency: 0,
                        });
                    } else if self.bind_missing {
                        frame.slots[*slot] = Some(term);
                    } else {
                        frame.support = Support::FALSE;
                    }
                }
                Node::Constructor { name, args } => match store.inspect(term) {
                    TermView::Variable(variable) => {
                        #[cfg(feature = "equality-binding-coverage")]
                        {
                            let region = frame.support;
                            self.touch(variable);
                            self.wait = Some(Wait::Coverage {
                                job: arena
                                    .job(Operation::And(region, store.binding_coverage(variable))),
                                pattern,
                                variable,
                            });
                        }
                        #[cfg(not(feature = "equality-binding-coverage"))]
                        frame.tasks.push(Task::Resolve {
                            pattern,
                            variable,
                            index: 0,
                        });
                    }
                    TermView::Constructor {
                        name: actual,
                        args: children,
                    } => {
                        if name == actual && args.len() == children.len() {
                            frame.tasks.push(Task::Children {
                                pattern,
                                term,
                                index: 0,
                            });
                        } else {
                            frame.support = Support::FALSE;
                        }
                    }
                },
            },
            Task::Children {
                pattern,
                term,
                index,
            } => {
                let Node::Constructor { args, .. } = self.plan.node(pattern) else {
                    unreachable!()
                };
                let TermView::Constructor { args: children, .. } = store.inspect(term) else {
                    unreachable!()
                };
                if let Some(pattern_child) = args.get(index) {
                    frame.tasks.push(Task::Children {
                        pattern,
                        term,
                        index: index + 1,
                    });
                    frame.tasks.push(Task::Pattern {
                        pattern: *pattern_child,
                        term: children[index],
                    });
                }
            }
            Task::Resolve {
                pattern,
                variable,
                index,
            } => {
                let region = frame.support;
                self.touch(variable);
                if let Some(binding) = store.bindings(variable).get(index).copied() {
                    self.wait = Some(Wait::Overlap {
                        job: arena.job(Operation::And(region, binding.support)),
                        pattern,
                        variable,
                        index,
                        binding,
                    });
                } else {
                    self.current = None;
                }
            }
        }
        MatchStatus::Pending
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::equality::UnifyStatus;
    fn bind(store: &mut Store, arena: &mut Arena, region: Support, a: Term, b: Term) {
        let mut job = store.unify(region, a, b);
        loop {
            match job.tick(store, arena) {
                UnifyStatus::Complete { .. } => break,
                UnifyStatus::Pending => (),
                UnifyStatus::Stale => panic!(),
            }
        }
    }
    #[test]
    fn first_slot_keeps_conditional_value_opaque_and_repetition_demands_equality() {
        let mut arena = Arena::new();
        let (_, birth) = arena.fresh_variable();
        let mut store = Store::new();
        let x = store.fresh_variable();
        let a = store.constructor("a", vec![]);
        bind(&mut store, &mut arena, birth, x, a);
        let plan = Plan::new(vec![Pattern::Slot(0)], 1).unwrap();
        let mut job = plan.start(&store, Support::TRUE, vec![x]).unwrap();
        let mut matches = vec![];
        for _ in 0..1000 {
            match job.tick(&store, &mut arena) {
                MatchStatus::Match(m) => matches.push(m),
                MatchStatus::Done => break,
                MatchStatus::Pending => (),
                MatchStatus::Stale => panic!(),
            }
        }
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].support, Support::TRUE);
        assert_eq!(matches[0].slots, vec![Some(x)]);
        assert!(job.dependencies().is_empty());
        let plan = Plan::new(vec![Pattern::Slot(0), Pattern::Slot(0)], 1).unwrap();
        let mut job = plan.start(&store, Support::TRUE, vec![x, a]).unwrap();
        for _ in 0..1000 {
            match job.tick(&store, &mut arena) {
                MatchStatus::Match(m) => {
                    assert_eq!(m.support, birth);
                    return;
                }
                MatchStatus::Done => panic!("expected supported equality"),
                MatchStatus::Pending => (),
                MatchStatus::Stale => panic!(),
            }
        }
        panic!("matching did not finish");
    }
}

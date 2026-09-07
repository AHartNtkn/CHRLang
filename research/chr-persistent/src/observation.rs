//! Completion snapshots borrow their arena through their originating machine.
//! Retaining the machine retains its full arena; snapshot capture and export are
//! explicit operations. Neither observation operation changes source counters.
pub use crate::terms::Term as Handle;
use crate::terms::{Arena, Bindings};
use chr_observe::graph::{AnswerView, Stats, TermView};
use std::rc::Rc;
#[derive(Default, Debug, Clone, Copy)]
pub struct CaptureStats {
    pub snapshots: u64,
    pub residual_occurrences: u64,
    pub store_visits: u64,
}
pub struct CompletedAnswer {
    pub(crate) owner: Rc<()>,
    pub(crate) outputs: Rc<[(String, Handle)]>,
    pub(crate) bindings: Bindings,
    pub(crate) residual: Vec<(usize, Rc<[Handle]>)>,
}
pub struct View<'a> {
    pub(crate) arena: &'a Arena,
    pub(crate) answer: &'a CompletedAnswer,
}
impl AnswerView for View<'_> {
    type Handle = Handle;
    type Variable = u64;
    fn output_count(&self) -> usize {
        self.answer.outputs.len()
    }
    fn output(&self, i: usize) -> (&str, Handle) {
        let (n, t) = &self.answer.outputs[i];
        (n, *t)
    }
    fn residual_count(&self) -> usize {
        self.answer.residual.len()
    }
    fn residual_name(&self, i: usize) -> &str {
        &self.arena.predicates[self.answer.residual[i].0].0
    }
    fn residual_arity(&self, i: usize) -> usize {
        self.answer.residual[i].1.len()
    }
    fn residual_arg(&self, i: usize, j: usize) -> Handle {
        self.answer.residual[i].1[j]
    }
    fn resolve(&self, mut h: Handle, stats: &mut Stats) -> TermView<'_, Handle, u64> {
        while let Handle::Var(v) = h {
            stats.dereferences += 1;
            let mut storage = crate::Storage::default();
            let next = self.answer.bindings.get(&v, &mut storage);
            stats.binding_visits += storage.visits;
            match next {
                Some(t) => h = t,
                None => break,
            }
        }
        match h {
            Handle::Var(v) => TermView::Variable(v),
            Handle::Node(n) => {
                let n = &self.arena.nodes[n];
                TermView::Constructor(&n.name, &n.args)
            }
        }
    }
}
impl View<'_> {
    fn term(&self, h: Handle, stats: &mut Stats) -> chr_syntax::Term {
        match self.resolve(h, stats) {
            TermView::Variable(v) => chr_syntax::Term::Var(chr_syntax::Var(v)),
            TermView::Constructor(n, args) => chr_syntax::Term::App(
                n.to_owned(),
                args.iter().map(|&h| self.term(h, stats)).collect(),
            ),
        }
    }
    pub(crate) fn export(&self, stats: &mut Stats) -> chr_syntax::Answer {
        chr_syntax::Answer {
            outputs: (0..self.output_count())
                .map(|i| {
                    let (n, h) = self.output(i);
                    (n.to_owned(), self.term(h, stats))
                })
                .collect(),
            residual: (0..self.residual_count())
                .map(|i| chr_syntax::Constraint {
                    name: self.residual_name(i).to_owned(),
                    args: (0..self.residual_arity(i))
                        .map(|j| self.term(self.residual_arg(i, j), stats))
                        .collect(),
                })
                .collect(),
        }
    }
}

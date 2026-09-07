//! Exact borrowed observation. Storage sharing is not part of answer identity.
#[derive(Default, Debug, Clone, Copy)]
pub struct Stats {
    pub term_pairs: u64,
    pub occurrence_scans: u64,
    pub occurrence_candidates: u64,
    pub backtracks: u64,
    pub dereferences: u64,
    pub binding_visits: u64,
}
pub enum TermView<'a, H, V> {
    Variable(V),
    Constructor(&'a str, &'a [H]),
}
/// Handles are interpreted only by their originating view. Variables returned by
/// `resolve` identify unbound holes within that view. Occurrences retain multiplicity.
pub trait AnswerView {
    type Handle: Copy;
    type Variable: Copy + Eq;
    fn output_count(&self) -> usize;
    fn output(&self, index: usize) -> (&str, Self::Handle);
    fn residual_count(&self) -> usize;
    fn residual_name(&self, index: usize) -> &str;
    fn residual_arity(&self, index: usize) -> usize;
    fn residual_arg(&self, occurrence: usize, argument: usize) -> Self::Handle;
    fn resolve(
        &self,
        handle: Self::Handle,
        stats: &mut Stats,
    ) -> TermView<'_, Self::Handle, Self::Variable>;
}
fn terms<L: AnswerView, R: AnswerView>(
    left: &L,
    right: &R,
    a: L::Handle,
    b: R::Handle,
    mapping: &mut Vec<(L::Variable, R::Variable)>,
    stats: &mut Stats,
) -> bool {
    stats.term_pairs += 1;
    match (left.resolve(a, stats), right.resolve(b, stats)) {
        (TermView::Variable(a), TermView::Variable(b)) => {
            if let Some((_, mapped)) = mapping.iter().find(|(x, _)| *x == a) {
                return *mapped == b;
            }
            if mapping.iter().any(|(_, y)| *y == b) {
                return false;
            }
            mapping.push((a, b));
            true
        }
        (TermView::Constructor(a, x), TermView::Constructor(b, y)) => {
            a == b
                && x.len() == y.len()
                && x.iter()
                    .zip(y)
                    .all(|(&x, &y)| terms(left, right, x, y, mapping, stats))
        }
        _ => false,
    }
}
fn occurrences<L: AnswerView, R: AnswerView>(
    left: &L,
    right: &R,
    index: usize,
    used: &mut [bool],
    mapping: &mut Vec<(L::Variable, R::Variable)>,
    stats: &mut Stats,
) -> bool {
    if index == left.residual_count() {
        return true;
    }
    for i in 0..right.residual_count() {
        stats.occurrence_scans += 1;
        if used[i]
            || left.residual_name(index) != right.residual_name(i)
            || left.residual_arity(index) != right.residual_arity(i)
        {
            continue;
        }
        stats.occurrence_candidates += 1;
        let checkpoint = mapping.len();
        if (0..left.residual_arity(index)).all(|j| {
            terms(
                left,
                right,
                left.residual_arg(index, j),
                right.residual_arg(i, j),
                mapping,
                stats,
            )
        }) {
            used[i] = true;
            if occurrences(left, right, index + 1, used, mapping, stats) {
                return true;
            }
            used[i] = false;
        }
        mapping.truncate(checkpoint);
        stats.backtracks += 1;
    }
    false
}
/// Joint alpha-equivalence of ordered named outputs and a residual multiset.
/// No pair memoization or storage-identity shortcuts are used.
pub fn equivalent<L: AnswerView, R: AnswerView>(left: &L, right: &R, stats: &mut Stats) -> bool {
    if left.output_count() != right.output_count()
        || left.residual_count() != right.residual_count()
    {
        return false;
    }
    let mut mapping = vec![];
    for i in 0..left.output_count() {
        let (name, a) = left.output(i);
        let (other, b) = right.output(i);
        if name != other || !terms(left, right, a, b, &mut mapping, stats) {
            return false;
        }
    }
    occurrences(
        left,
        right,
        0,
        &mut vec![false; right.residual_count()],
        &mut mapping,
        stats,
    )
}

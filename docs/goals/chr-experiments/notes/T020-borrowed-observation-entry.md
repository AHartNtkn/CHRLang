# A6 implementation entry: observation before materialization

Read-only source review identifies an independent implementation path. A3 and A1
results are not prerequisites. The current persistent state exports before its
consumer receives an answer; changing only the consumer cannot avoid that cost.

Add a generic borrowed answer-view comparator to chr-observe, independent of
chr-persistent. Views expose named outputs, distinct residual occurrences, and
resolved variable/constructor observations through opaque handles. Maintain one
joint variable bijection with rollback during residual multiset matching. Compare
constructor/predicate names and arities across views. Do not assume equal handles
have equal denotation or require equal constructor sharing topology. Initial
comparison has no node-pair memoization; that separate optimization needs
mapping-sensitive correctness evidence.

Refactor the persistent internal completion event to precede export. The existing
eager consumers still export at the same event and retain their public behavior
as controls. A graph-completion path shares the same transition implementation,
returning an immutable snapshot: named roots, completed bindings and residual
predicate/root arrays. It need not retain pending source work or propagation
history. Snapshot construction and map traversal are charged. Borrow the machine's
arena together with the snapshot when observing; indices are stable, bindings are
snapshot-local. Further source execution requires ending that borrow first.
Observation counters must not change source-work counters.

The independent gate combines194 targeted comparisons and4,096 exhaustive small
graph comparisons from the earlier entry plan. A synthetic DAG view and a brute
force complete-variable-bijection oracle must not call candidate comparison or
persistent export. Real completion integration must additionally test:

- Same arena handle with different branch bindings; retained snapshots remain
  unchanged after another branch advances.
- Joint output/residual aliases, residual multiplicity, and rollback after a
  rejected residual candidate.
- Equal denotation with different physical constructor sharing.
- Identical raw completions and fully exported answers on finite source cases.

A small snapshot may retain a large binding structure; its machine retains the
arena. The initial comparator can still traverse exponentially many unfolded
occurrences even though it avoids constructing them. These are explicit lifecycle
and traversal questions. Later three-way costs compare current eager observation,
eager comparison before cloning, and graph comparison before export, including
snapshot capture, comparison, export, retention and destruction. No language or
production architecture decision is implied.

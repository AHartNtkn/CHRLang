# A6: exact borrowed graph observation gate

Status: semantic implementation authorized; no registered matrix has run.

Question: can full answer alpha-equivalence and residual multiset comparison read
shared graph roots under answer-local bindings without materializing owned trees?
This does not change answer identity, permit partial answers, or adopt a production
representation. It does not depend on A1 or A3.

The first candidate uses a generic borrowed AnswerView, separate from persistent
execution. It traverses constructors, maintains one injective variable mapping
across named outputs and residual occurrences, and rolls back rejected residual
matches. No numeric-handle identity shortcut or node-pair memoization is allowed
in this first contrast. Different physical sharing must be unobservable, while
variable aliases remain observable.

## Independent synthetic gate

`research/chr-observe/tests/graph_oracle.rs` owns a test-only DAG adapter and
independent oracle. Fixtures begin as plain finite syntax, imported with/without
constructor interning and with/without one-link variable aliases. The oracle
works only on original plain fixtures: enumerate every complete bijection of
reachable free variables (at most four), substitute it, then compare ordered named
outputs and sorted residual multisets. It must not call candidate comparison,
persistent export or the current materialized comparator.

Twelve pair templates in order: ground equality, ground mismatch, repeated-output
renaming, output alias conflict, joint output/residual agreement, joint conflict,
residual permutation needing rollback, multiplicity/alias conflict, four-cycle
versus two cycles, constructor argument-position conflict, output-name conflict,
and depth-six repeated nonground structure with renaming. Manual expected booleans
are checked against the exhaustive oracle too. Each template crosses four physical
sharing combinations (left/right interned or expanded), two alias representations,
and two comparison orientations:192 cases. Two further same-arena/same-root cases
use different bindings yielding equal then unequal denotations, for194 total.

Also compare all64 directed loop-free three-vertex graph fixtures with all64
others:4,096 cases. Every fixture contains one node occurrence per vertex plus
edge occurrences selected from six ordered edges. Expected equivalence is checked
both by whole-fixture variable bijections and independent adjacency permutation.
The DAG layout varies deterministically with the input bitmask; semantic outcome
must not depend on representation.

The root runner must freeze fixture/oracle, candidate and relevant syntax code,
registration and compiler configuration before the registered test binaries run.
Record every case's expected/actual result and observation counters; run all
three groups in fresh processes, then exact replay. Per group bound30seconds,
1GiB address space. Compilation precedes this semantic gate and has no runtime
interpretation. Hand-sized implementation tests may run during development;
generated matrix execution waits for the recorded source freeze.

## Real completion obligations

Separate integration checks must reach actual persistent source completion before
export: branch-local bindings on common roots, retained completion re-observation
after advancing a sibling, joint aliases/residual multiplicity, and eager/graph
raw-completion/full-answer correspondence. Snapshot construction and observation
have separate counters from source transitions. The graph snapshot may retain
bindings and arena ownership; those costs are explicit later measurements.
Passing a synthetic adapter gate alone does not discharge integration.

A later cost protocol compares current eager delivery, competent eager comparison
before cloning, and graph comparison before export. It includes preparation,
retention, dedup, output materialization and destruction. Avoided tree construction
does not imply avoided traversal or lower total memory. Any mismatch is minimized
and repaired before affected comparative costs; bounds do not close A6.


## V2 representation-matched comparator control

Before cost runs, read-only review identified that eager and graph comparators
use different residual mapping strategies. The borrowed view interface now exposes
constructor name, normalized handle and arity plus child access, allowing an
allocation-free view of ordinary owned trees to use the same comparator. No
child-reference arrays are constructed. This interface change needs renewed gates.

Repeat each of the192 template and4,096 graph pair cases for four view pairs:
DAG/DAG, tree/DAG, DAG/tree, tree/tree, in that order. Add the same two common-root
binding cases. The three group counts are768,2,16,384:17,154 comparisons per pass.
Expected results come from the unchanged independent fixture oracle; graph/tree
adapter agreement alone is not sufficient. Record and exactly replay all result
and counter rows in six fresh children at the same bounds. V1 inputs/evidence
remain available. No comparative cost result preceded this change.

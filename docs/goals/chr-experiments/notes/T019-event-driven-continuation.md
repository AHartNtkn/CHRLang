# A3 continuation: distinguish rediscovery from retained-join cost

Read-only analysis of the sizing result identifies an event-driven next contrast.
Keep current rows/tests/dependencies, but subscribe each live prefix to its next
head's predicate pool. New prefixes inspect compatible existing occurrences; new
occurrences notify every compatible head-position subscription. A successful
extension emits a prefix event. Unchanged cached extensions need no new work.

Binding changes invalidate tests through all inspected alias dependencies and
invalidate successful descendants. Requeue invalidated tests whose parents remain
live; regenerated parents discover their suffixes. Consumption subtracts support
and cancels queued tests with unavailable parents/occurrences. Root predicate
changes need explicit pool/subscription updates because some compatible tests did
not exist before binding. Recheck pending task validity before publishing rows.

Deduplicate pending extension tasks, unioning supported contexts from simultaneous
prefix/occurrence events. Discovery order must not silently choose a different
source rule: complete candidates still use the registered selector and guard/
history validation. Drain discovery before certifying no match. The current
serialized update contract suffices; concurrency is a separate question.

This avoids revisiting known tests, but initial rejected pairs can still be
quadratic. Join-key indexes, seed-ordered/TREAT-like recomputation and lazy first
witness selection test different explanations. If retained intermediates dominate
memory, eliminating rediscovery may not rescue that representation.

Per-context task execution also remains distinct from sharing newly introduced
matching work. A later supported-task contrast groups by parent row, occurrence
and actual matching-input class, computing one result per equivalent input class
while retaining support-local dependencies. Charge resolution/grouping costs and
distinguish inherited rows from newly shared matching calls. Different output tags
with post-choice right arrivals already provide a discriminating fixture.

Before another run, register cutoff checkpoints plus queue admissions, duplicate
admissions, stale cancellations and pending peaks. Retain all eight sizing workloads
and censored evidence; compare the event-driven mode at the same bounds first.
Add simultaneous prefix/occurrence arrival, root-pool movement and consumed queued
parent semantic tests. Source accounting, real allocation/retention, and alternative
join architectures remain open. A6 and A1 progress does not depend on this contrast.

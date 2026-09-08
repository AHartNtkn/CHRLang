# R03 supported equality component

Supported finite-tree equality and nonbinding equality demand now pass independent projection checks. Failure and binding effects stay within their active supports, including conditional cycles. T035 remains active: these results do not establish source matching, consuming execution, publication or a complete conditional runtime.

## Behavior and necessary state

The component stores stable variables, immutable interned constructors and disjoint guarded bindings. A unification job resolves guarded alternatives, decomposes constructors and checks occurs paths using explicit resumable continuations. Each path intersects its support with the guarded binding it follows. Visited terms retain the support already explored, so later visits inspect only additional support. Constructor-field and binding-list traversal advance incrementally; Boolean operations use the existing resumable support jobs.

The store owns cumulative failed support and binding change records. An interrupted operation may have committed a sound prefix; those effects and notifications remain observable. Its caller must preserve the unfinished obligation, restart the original equation, or cancel the query. Dropping a job is not successful completion. The store does not itself retain that continuation.

Nonbinding demand returns the support on which two terms are already equal. It never binds unknown variables to enable equality. Demand records touched variables, including followed aliases and free endpoints, under its original request support. This is conservative dependency information: finer support attribution is not claimed. A later binding of the same value on additional support generates another change record.

Unfinished jobs reject intervening store mutations using a global version. Completed jobs return their historical result; refreshing a query requires a new job. This component boundary prevents stale writes but is not the eventual support-local scheduling protocol. The runtime must ensure that disjoint activity cannot continually restart finite-support work before claiming finite-sibling progress.

## Independent evidence

The root-owned oracle uses owned source trees, a scalar substitution map, its own occurs check and integer truth masks. It neither invokes candidate equality to predict results nor derives expected answers from candidate bindings. Successful projections compare joint most-general X/Y structure after alpha normalization; satisfiability alone would be too weak.

- 20,736 two-equation sequences range over `X`, `Y`, `a`, `b`, `f(X)`, `f(Y)` and four supports: false, true, a birth and its complement. Both worlds are checked, giving 41,472 projected failure/result comparisons.
- 5,184 equality-demand queries produce 10,368 world checks against equality of independently resolved trees. Store versions and notification counts remain unchanged by these demands.
- Focused checks cover alias dependencies intersecting later changes, same-value support expansion, a three-edge cycle failing exactly on the intersection of two births, and a multi-field clash confined to its active alternative.
- Interruption checks preserve committed notifications, reject a stale writer, and pause a deep occurs traversal before another binding makes its continuation stale. A fresh retry correctly detects the resulting cycle.

The complementary-edge cases distinguish a physically cyclic guarded graph from a cycle in a live interpretation. `X=f(Y)` under one alternative and an opposing alias under the other must not be globally rejected. The independent matrix and longer-cycle witness check that distinction directly.

All eight independent equality tests pass, alongside the deep-traversal unit test and existing components. All twenty package tests pass with counters disabled; default package/workspace tests, Clippy and formatting also pass. [Validation receipts](R03-conditional-equality/) preserve initial missing-API RED, independent GREEN and workspace/counter-free checks. An independent implementation review found no blocking component defect and prompted the partially traversed stale-job witness. No comparative measurements were run.

## Remaining responsibility gates

Next integrate structural head demand, occurrence identity, propagation history and atomic supported body work. The [protocol](R03-conditional-protocol.md) still requires direct activation, source-order preservation and support-local invalidation/completion. Global version rejection alone cannot satisfy that progress requirement. Exactly-once raw answer publication and joint nonground output/residual export remain unimplemented.

This component adds guarded-binding lists, support traversal, dependency records and explicit continuation state. Their retention and processing costs must be charged in the eventual complete-path comparison. The current gate establishes semantic behavior, not that conditional equality is faster or simpler than the dedicated or integrated alternatives.

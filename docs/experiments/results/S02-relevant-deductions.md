# Equality deductions can be reused after unrelated state changes

The new contextual mode reuses a deduction across different equality states while preserving each caller's bindings and consumable resources. Four source confirmations pass against independent scalar and compiled controls. This establishes a broader reuse mechanism, not an efficiency advantage.

## The distinction from existing evidence

[Exact-state deduction reuse](S02-shared-deduction-gate.md) already shares newly computed equality, and its [lifecycle comparison](S02-deduction-confirmation.md) already includes changed bindings and strong compiled/lowered controls. Repeating that experiment would not address the remaining validity boundary: different equality-state identities miss even when the equation reads the same information.

The candidate records the ordered canonical equation inputs and the roots/descriptors reached transitively through their constructor children. A hit applies only the merge's parent/descriptor changes and generated child equations. It does not install the cached caller's entire maps. Pending work, unrelated bindings, resource occurrences and source history remain local.

This is a specific relevance key for one equality operation. It is not arbitrary continuation equivalence, a new CHR-expressed union-find, or a strategic port rewrite. Those organizations keep their distinct evidence obligations.

## Why the key and replay are credible

The ordinary step reads canonical input roots, their descriptors and forward constructor reachability to test occurs cycles and constructor compatibility. The key records the root and descriptors of every reachable handle, including descendant root changes. Immutable arena nodes are shared; growing the arena does not change existing descriptors.

For equal keys, the ordinary step makes the same failure decision or the same local update: redirect the second root to the first, combine their descriptors and enqueue the same child equations. Replaying that update preserves maps outside the affected roots. This argument is scoped to the current equality operation; introducing additional reads or writes requires revisiting the key.

A deliberate input-only-key mutation fails an independent witness. One caller aliases a constructor's child to the other equation input, producing an occurs failure; another caller does not and must succeed. Omitting descendant reads wrongly replays the failed result into the valid caller. The [mutation receipt](s02-relevant-deductions/read-set-mutation.log) preserves the failure.

## What the gate checks

| Obligation | Observed result |
|---|---|
| Reuse across distinct maps | Ordered and persistent stores bind disjoint caller variables to different constants before the same constructor equation. A diagnostic assertion confirms a reuse hit; their exported caller bindings remain different. |
| Preserve pending work and claims | Different queued bindings survive replay. Two distinct open/ticket tuples remain available in the sibling after one caller consumes its tuple. Parent state is unchanged. |
| Reject incompatible reuse | Changed relevant bindings clash, occurs cycles fail, and failed siblings do not poison a valid caller. The descendant-state witness distinguishes cached failure from a valid merge. |
| Complete source correspondence | 48 configurations: single/shared/distinct/changed-binding families × depths 0/4/16 × token absent/present × query order. Relevant-key, exact-state and uncached contextual execution agree with independent scalar and compiled Scan/Indexed complete raw answers. |
| Actual source reuse | With tokens and original order, the changed-binding family records sampled relevant-key hits of 3, 14 and 50 at depths 0, 4 and 16. The shared family has the same observations; single and distinct families record zero. |
| Repeated qualification | Two diagnostic and two counters-off executions pass all four new tests. Existing contextual store/source regressions pass in both builds; local choice/guard regressions also pass. All ten processes finish within their bounds. |

The hit sampler reads the live frontier's shared arena after service. Its maximum is a lower bound on total hits because the arena is no longer available after final exhaustion. It proves the mechanism operates; it is not a count of all saved operations. The rows for other modes do not instrument their own cache-hit mechanisms.

The initial store witness used the same constant handle for its supposedly unrelated binding and the tested equation. That changed a reachable representative, so the candidate correctly missed. Disjoint constants provide the intended positive witness. This exposes conservative key sensitivity to representative identity; it does not justify assuming all semantically unrelated bindings will hit.

A separate assertion initially counted matching derivations as distinct physical claims. Merged constructor descriptors can give multiple derivations for the same occurrence tuple. The corrected assertion checks distinct occurrence IDs and actual consumption. Complete raw source observations remain independently checked; no answer multiplicity is relaxed.

## Costs and limitations

The candidate constructs a transitive key before every lookup and retains copied key descriptions plus replay data. It still applies caller-map updates on a hit. Exact-state reuse instead retains successor maps and can adopt them directly. These are different lookup, mutation and retention obligations; a hit is not automatically cheaper than recomputation.

The table has a 4,096-entry cap, which bounds entries rather than bytes. This gate does not qualify cap saturation, long-lived consumers, query cancellation, memory restoration or compilation/lifecycle costs for the new mode. Ordered and persistent store behavior is checked, but the complete-source candidate currently starts with ordered maps. A persistent source-cost contrast needs its own qualification.

The reference interpreter and independent scalar code are unchanged. The optional `deduction-work` feature owns hit diagnostics; counters are absent in the counters-off build. Scoped Clippy passes with warnings denied. There are no comparative timing or allocation measurements in this gate.

## Next decision and evidence

The [full breadth review](S02-relevant-deductions-breadth-review.md) selects bounded key/replay ownership and lifecycle qualification next. A credible cost comparison must include recomputation, exact-state reuse, compiled execution and applicable source elimination, plus single-use and distinct-input overhead controls. No new architecture is selected.

[Registration](../registrations/S02-relevant-deductions.md) · [source tests](../../../research/chr-relational/tests/relevant_deductions.rs) · [store](../../../research/chr-relational/src/contextual.rs) · [source executor](../../../research/chr-relational/src/contextual_execute.rs) · [freeze](s02-relevant-deductions/confirmation/freeze.json) · [ten outcomes](s02-relevant-deductions/confirmation/results.json) · [sampled reuse rows](s02-relevant-deductions/confirmation/reuse.csv) · [audit](s02-relevant-deductions/confirmation/audit.json) · [Clippy](s02-relevant-deductions/clippy-qualified.log).

Final executions use 60-second wall/CPU, 1-GiB address-space and 200,000 source-turn bounds. Both diagnostic row sets agree. The receipt parser was corrected to extract the first row after the test-name prefix; source executions passed before that parser correction. Earlier driver bytes remain recoverable from the freeze. The corrected driver refuses to overwrite the receipt directory.

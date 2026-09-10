# The local handle graph now executes explicit alternatives

The experimental search owner preserves the registered source answers, branch-local effects and body order over the existing local graph-scanning mechanism. A finite sibling progresses beside recurring work, and query cancellation releases prepared owners. This establishes a broader candidate for comparison; it does not establish that copying its graphs is economical.

## What changed

The [search owner](../../../research/chr-relational/tests/support/local_search.rs) retains a queue of owned branches. Each branch contains the existing local handle graph, live occurrences, propagation history, a body-variable environment and a stack of unfinished goals. At an explicit `or`, it copies that state and gives each child one alternative while retaining the remainder of the body in both.

Both children can refer to the same immutable prepared rules. Their mutable graphs, handle targets, occurrences and histories are separately owned. This is copied search state, not shared execution across branches. Body-local variables reuse their environment within each branch, so repeated aliases remain connected without allowing a failed sibling to bind the other sibling's values.

The owner services one goal or source-selection turn at a time and enqueues the right child before the left, matching the independent scalar control's policy. Another rule cannot fire within that branch until its pending body is finished. The existing matcher, primitive body operations, equality repair and answer extraction perform the actual computation. Matching and equality can still take size-dependent work; the test does not prove constant-time service or arbitrary latency bounds.

The prepared search entry admits nonempty, guard-free rules with alternatives. The deterministic prepared entry retains its original admission and implementation as the measured control. Retained tuple/prefix caches are not enabled in the search experiment. Clone support and a module import are the only changes to that existing deterministic implementation; its earlier source is preserved as a snapshot.

## Independent source and ownership evidence

| Check per confirming execution | Scope | Result |
|---|---|---|
| Source matrix | 48 configurations varying token count, duplicate alternatives, aliased outputs, hidden-cycle failure and branch order | Complete answers match the independent scalar order and raw multiplicity; compiled Scan/Indexed and contextual controls agree on complete answer multisets |
| Body competition | A chosen body posts a newer request, then enables an older request with one token remaining | Both alternatives return the older consumer as winner |
| Continuing service | A finite answer beside an indefinitely recurring sibling, under two query-variable namespaces | Expected answer within 500 turns; 32 later turns remain nonterminal |
| Cancellation and preparation reuse | Cancel each continuing search, then reuse preparation for a changed empty query | Prepared owner count returns to one; complete owned answers remain valid after their search is disposed |

The matrix establishes a fresh aliased pair before choice. Each branch posts a consuming request, keeps permission, observes token identities through propagation, and can subsequently fail through a hidden cycle. Equal-valued token occurrences and duplicate alternatives retain their distinct source multiplicity. Aliasing the requested output with the shared pair introduces additional real failure cases rather than merely renaming independent outputs.

**The body boundary is necessary for the tested source order.** A trigger consumes one token and posts a newer ready request. A later equation in the same chosen body enables an older suspended request. The search owner finishes that body before choosing another rule, so the older request consumes the remaining token. Exposing an unfinished body as an ordinary continuation constraint would not establish this behavior; the [earlier body study](S02-integrated-bodies.md) provides the corresponding contrary transformation.

Four confirming executions pass, two with local counters on and two off. All three new tests pass in each. Another 18 tests across the existing multihead/cache, contextual-store and contextual-source executables pass. Scoped Clippy and formatting checks pass. Independent scalar and reference code are unchanged.

The finite source checks require exhaustion within 200,000 turns per path. Confirming executables have 60-second wall/CPU and 1 GiB address-space bounds. The scalar control checks exact delivered order; the other controls check complete raw multisets because their service quanta differ. These are explicit comparison contracts, not an assumption that all engines expose the same order.

## Architectural consequence and limits

The source-driven local graph can now be investigated beyond deterministic bodies while preserving effects and fresh-variable relationships. Its existing deterministic timing evidence cannot be extrapolated to this search owner: a fork copies graph slots, occurrence maps, histories, environments and unfinished goals. The cost of those copies, including retained inert slots, must be charged alongside any matching/equality savings.

Prepared-owner release is narrower than full memory accounting. This entry measures neither requested allocations nor RSS, retained consumer bytes, sustained query streams or compilation. It also does not qualify retained-join search, positive guards, arbitrary scheduling contracts or distributed claims. No complete-architecture recommendation follows.

Next register positive equality guards for this local source path. They can determine whether existing guarded source comparisons are admissible and whether newly available constructor/equality information wakes consuming work without binding unknowns during a guard check. This is a concrete prerequisite to comparing broader complete paths, not a reason to repeat the deterministic timing matrix.

The strongest ready alternative is joint structural theory/source integration, whose new components have independent semantic entries but lack a combined interface and lifecycle comparison. The guard gate comes first because it closes a specific source-admission gap in the now choice-capable candidate. Reconsider the alternatives at that gate or an obstruction; demand-driven choices and conditional ownership remain required. This is package one after the normal/neutral breadth review. The research goal remains active.

## Evidence

[Prospective registration](../registrations/S02-local-choice-entry.md), [frozen source and binaries](s02-local-choice-entry/freeze.json), [raw receipts](s02-local-choice-entry/), [audit summary](s02-local-choice-entry/audit.json), [runner](../../../research/chr-relational/experiments/local_choice_entry.py), and [prior deterministic source snapshot](s02-local-choice-entry/before/local_multihead.rs). The audit verifies all 18 registered source inputs, five binary identities, four confirming executions and three regression receipts. The source and ownership assertions supply correctness evidence; hashes and passing process status alone do not.

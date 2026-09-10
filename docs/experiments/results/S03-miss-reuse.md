# Reusing misses within a turn avoids substantial repeated discovery

Within-turn miss reuse reduces the size-128 delayed-miss witness from 6,432,964 to 50,697 force entries while preserving complete answers and service ticks. Successful cases receive no hits and still pay cache lookups. This establishes avoidable work and an overhead tradeoff, not a speedup or an architectural winner.

## The design and its validity boundary

The experimental option retains an unsuccessful call probe only until the next `tick`. It caches the unresolved output returned when no clause matches; successful results and failures retain their existing treatment. Every tick clears the miss table before servicing source work. The default path does not enable the option.

The source audit identifies why this boundary is useful. Ordinary force/match traversal reads existing graph state. A source application may post or consume resources, redirect producer handles or add results, but then returns `Progress`, ending the tick. A local pull-tab returns `Split`, also ending the tick. Selecting a child context therefore starts with an empty table. External support reclamation can occur between ticks; the next tick clears the table before using the changed graph.

Recursive re-entry is treated conservatively. Re-entering a call marks every active probe as dependent on a temporary recursive unknown. Those probes cannot enter the miss table. This does not assert that every such probe is intrinsically unreusable; it avoids needing a recursive fixed-point validity analysis for this candidate.

The implementation adds a per-run map, a recursion marker stack and cache lookup/clearing operations. It does not add persistent miss explanations, caller projections or a general dependency invalidation system. That is a bounded design choice supported by the current turn protocol, not evidence that broader reuse is unnecessary.

## Source and adversarial checks

Seven resource-dependency tests pass in both the diagnostic and metrics-off builds. The paired source helper runs no-miss reuse and within-turn reuse under all three existing successful-result policies. It preserves alias identity, hidden constructor failure, nested claims, delayed producers, recursive waiting and selected alternatives. Added cases check late body posts and cancellation followed by reuse of the same preparation; owned results remain valid after the run and preparation are dropped.

A deliberate mutation retained misses across turns. The late-post witness then lost the independently expected successful observation. The [mutation receipt](s03-miss-reuse/stale-miss-mutation.log) verifies that the test detects this invalidation error. Restoring the turn boundary passes.

The existing 31-test suspended-source suite passes in both builds, and the demand/pull-tab diagnostic regressions pass. Those regressions exercise their existing configurations; they are not evidence that every combination of miss reuse, derivation templates and local lifting has been tested. The candidate's paired checks are the resource suite and registered work matrix. Reference and independent scalar implementations are unchanged.

## Causal work comparison

The [registration](../registrations/S03-miss-reuse.md) replays 30 configurations, three reuse policies and two changed queries with the option off and on, twice: 720 demand sessions. Each setting's 180 rows repeats exactly. All complete observations agree with independent scalar and compiled Scan/Indexed controls. The off rows exactly reproduce the preceding work screen, including force, match, validation and tick columns.

| Size-128 source, original order | Force entries: off → on | Match entries: off → on | Lookups / hits / inserts with reuse |
|---|---:|---:|---:|
| Plain independent calls | 647 → 647 | 129 → 129 | 388 / 0 / 0 |
| Known-key success | 1,159 → 1,159 | 513 → 513 | 388 / 0 / 0 |
| Known-key miss | 100,619 → 66,949 | 50,567 → 33,537 | 389 / 130 / 256 |
| Delayed-key success | 660,327 → 660,327 | 339,169 → 339,169 | 6,756 / 0 / 0 |
| Delayed-key miss | 6,432,964 → 50,697 | 3,253,118 → 25,409 | 644 / 383 / 128 |

The table uses CurrentContext and the first query value. The full CSVs preserve every policy, query value, size and order. Reversed-order delayed misses fall from 6,358,665 to 33,545 force entries. Across all 180 matched sessions, force counts fall in 72, remain unchanged in 108 and never rise. Answer counts and service ticks remain identical in every pair.

A hit avoids repeated execution inside a force request; lookup counts are reported separately. They must not be added to force/match counts as equal-cost units. In particular, unchanged force counts with additional lookups are a real adverse condition that future timing and allocation runs must include.

## What this resolves and what remains

Much of the unsuccessful dependency discovery in this screen is avoidable without a general cross-turn cache. The simple candidate is now credible enough for subsequent ownership and lifecycle comparison. This does not determine whether its time saved repays map allocation, clearing and lookup cost, or whether another access organization avoids the work more economically.

The successful original-order chain still performs 660,327 force entries; this option does not solve that case. Constructor-heavy validation, combined execution strategies, sustained consumers and broader contextual dependencies remain separate questions. No comparative timing, RSS or allocation advantage has been established. The candidate remains explicitly enabled; it is not an adopted production policy.

## Next selection

This completes package three after the joint-ownership breadth review. Proceed to T072's contextual equality and local consuming-execution source comparison as package four, then review full breadth. That investigation can change representation boundaries and shared deduction, while more tuning of this cache cannot answer it. The strongest ready alternative remains the dependency ownership/lifecycle qualification, now with a less wasteful miss control; reconsider it at the contextual gate or an obstruction.

T071 retains required allocation, ordinary-allocator timing, cancellation/lifetime costs, successful-chain attribution and broader source obligations. Joint symbolic costs and union/inclusion also remain required. No task ordering supplies negative evidence for those designs, and the architecture goal remains active.

## Receipts and reproduction

- [Prepared/run implementation](../../../research/chr-direct-choice/src/demand.rs), [paired resource checks](../../../research/chr-direct-conditional/tests/resource_dependencies.rs), [work harness](../../../research/chr-direct-conditional/tests/dependency_work.rs), and [confirmation driver](../../../research/chr-direct-conditional/experiments/miss_reuse_confirmation.py).
- [Source/binary freeze](s03-miss-reuse/confirmation/freeze.json) and [ten process outcomes](s03-miss-reuse/confirmation/results.json).
- [Off rows](s03-miss-reuse/confirmation/rows-off.csv), [on rows](s03-miss-reuse/confirmation/rows-on.csv), and [lookup/hit/insertion rows](s03-miss-reuse/confirmation/misses-on.csv).
- [Source checks](s03-miss-reuse/source-invalidation.log), [adversarial mutation](s03-miss-reuse/stale-miss-mutation.log), and [Clippy with warnings denied](s03-miss-reuse/clippy.log).

All final executables run with 60-second wall/CPU and 1-GiB address-space bounds; finite work paths have 2,000,000 service turns. There are no final cutoffs. Sources and binaries remain frozen, and earlier modified source versions are preserved in `before/`. The driver refuses to overwrite its receipt directory.

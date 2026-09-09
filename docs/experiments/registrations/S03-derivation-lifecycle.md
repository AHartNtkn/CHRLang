# Registered fresh-derivation lifecycle sizing

The [fresh-derivation gate](../results/S03-fresh-derivation-source.md) establishes a reusable residual plan with fresh instantiation, bounded construction and explicit scheduling limits. This package asks whether that mechanism repays key construction, specialization, instantiation and retention. It is selected before more local-lifting precision or reusable prefix artifacts because its multi-rule reuse costs are not yet measured.

## Sources, hypotheses and controls

Five families use the same recursive constructor-input countdown. The base returns `item(key, accumulator, fresh_unknown)`; the choice family returns either `item` or `other`, independently for each application. `single` has one call, `repeat` four equal-input calls, `distinct` four same-sized distinct keys, `choice` four equal-input calls with independent choices, and `grow` one call whose accumulator doubles at every step. Query keys and insertion order alternate across changed queries. Resource variants add one finishing consumption and token per call, ensuring source outcomes agree independently of the contested-order counterexample.

H1: repeated substantive derivations may repay a query-local residual-plan cache. H2: short or distinct requests may expose key/construction overhead. H3: owned tree templates may suffer on duplicating accumulators despite the construction bound. H4: a specialized source control may avoid work retained by both graph strategies. Favorable and adverse outcomes remain scoped to these sources and bounds.

Compare six modes: ordinary MatchDependencies demand (`dependencies`), the same executor with fresh derivation templates (`templates`), compiled scanning, compiled indexing, inferred compiled specialization (`sealed`), and an exact-schema source control (`lowered`). The exact control validates the entire source and query before lazy complete-answer generation, including fresh unknown distinctions, independent choice multiplicity and complete resource consumption. It is not a general compiler. The independent scalar evaluator validates both its enumeration and every executor. Source/query near misses must be rejected by the exact control.

The new runner's correctness gate covers all modes/families, resource presence/absence, depths 0/1/8 and both insertion orders, plus cancellation followed by reuse. No timing comparison is made on sources with different committed outcomes.

## Accounting and bounds

Use `chr-derivation-cost MODE FAMILY DEPTH QUERIES RESOURCE REVERSE_FIRST [CANCEL_TICKS]`. Prepared rules are reused across queries; derivation caches are query-owned and are rebuilt for changed queries. Template construction/hits are included in execution, not assumed free or reported as independently timed compiler phases. The primary sum includes preparation, setup, execution with complete observation, engine/answer disposal and prepared disposal. Record source/input construction separately and examine the broader sum too. Record first-answer latency from execution start. Native Rust compilation is outside this boundary.

Primary runs use ordinary allocation and all engine/kernel/observer/traversal counters disabled. Requested-allocation diagnostics use a separate binary; they are not RSS measurements. Verify complete answers outside primary intervals and require exact restoration after query and prepared disposal. Each process is pinned to the first available CPU and bounded at 60 seconds wall/CPU and 1 GiB address space. Candidate and scalar bounds are 2,000,000 steps per query. Retain and investigate failed/cutoff receipts; do not call them losses.

## Exploratory sizing

For each non-growing family use depths 0/8/32; for `grow` use 0/8/12 (reused queries alternate n/n+1, within the depth-13 output bound). Cross all six modes with five families, their three depths, one/eight queries, resources absent/present and both starting orders: **720 exploratory single timings**, randomized with seed 7111. They determine confirmation design, not gain/loss claims.

Allocation diagnostics cover each family's short/cold endpoint (depth 0, one query) and long/reused endpoint (depth 32 or grow depth 12, eight queries), with all modes/resources/orders, twice: **480 processes**. Allocation records must replay exactly. Cancellation uses the choice family at depth 8, two queries, resources present and forward order; cancel after zero/one ticks on the first query and complete the second, across ordinary/meter binaries: **24 processes**. Run the meter self-check first. Freeze source and binary hashes before comparative runs.

After sizing, investigate consequential defects or uncertainty and register confirmation repetitions, randomization and thresholds prospectively. Compare its value with broader integration, unknown-input derivations and reusable lowering artifacts. This package cannot settle a general architecture or language policy.

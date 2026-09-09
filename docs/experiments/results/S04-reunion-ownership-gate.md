# Reunion has reusable preparation and complete heap-ownership evidence

The checked reunion plan now survives changing queries without retaining completed query state. Its allocation tradeoff is conditional: substantial private work can save traffic, while small queries and payloads expose overhead. The [paired identity-transport attribution](S04-reunion-identity-attribution.md) corrects a consequential copying cost; no ordinary timing conclusion follows.

The initial complete [registered gate](../registrations/S04-reunion-ownership-gate.md) has **480 successful processes and 240 exact allocation replays**. The paired repair repeats the same matrix with another 480 successful processes and 240 exact replays. All **384 unchanged-control process pairs** match across revisions. Complete raw answers agree with the independent evaluator on every warm/preflight and measured query.

## Preparation belongs to the rules; query state belongs to the query

`PreparedPhase` checks the local-prefix/owner property once and retains the rules. Starting each changed query validates variable ownership, computes its identity range and creates private states. Tests mutate the caller's original rule vector, run four changed queries, reject cross-owner aliases, and continue using the same prepared plan successfully. Query execution cannot change the checked source.

Work fields and increments are conditional on `replay-diagnostic`. They are absent in the allocation binaries. The ordinary semantic suite remains active without those fields, while diagnostic tests retain the 142-versus-198 and 148-versus-204 source-transition witnesses. After the identity repair, all **20 ordinary and 21 diagnostic package tests**, strict Clippy and formatting pass. No reference-interpreter or independent-evaluator code changed.

The measured phases include prepared construction; query-input construction; setup; execution through owned answers; engine, answer and input release; a separate first-answer cancellation lifecycle; and final prepared release. Each query and cancellation returns exactly to the prepared-owner live baseline. Final prepared release returns to the original baseline. Both repetitions reproduce every allocation reading, and live bytes are continuous between measured phases.

The permanent-factoring control retains a source vector but its existing query entry repeats analysis and source preparation at setup. Those costs are charged, not assumed intrinsic to every possible factoring implementation. It publishes unique answers; the measured sources have distinct complete raw answers, and full oracle equality checks that condition. The broader source tests still preserve duplicate alternatives, but this cost comparison does not cover duplicate publication contracts.

## Corrected allocation results remain conditional

These are four-owner, four-query results after the identity repair. Depth is the base private countdown depth; changed queries alternate that depth and depth+1. Traffic includes preparation, input creation and complete-query disposal, excluding the separately reported cancellation trial. Peaks are incremental live requested heap, not RSS.

| Source | Copy traffic MiB | Reunion traffic MiB | Indexed traffic MiB | Copy peak KiB | Reunion peak KiB | Indexed peak KiB |
|---|---:|---:|---:|---:|---:|---:|
| Plain, depth0 | 0.832 | 0.597 | 1.736 | 33.3 | 37.3 | 197.4 |
| Plain, depth48 | 35.586 | 9.935 | 17.451 | 81.1 | 50.8 | 520.3 |
| Payload, depth0 | 8.262 | 8.958 | 10.382 | 1,911.3 | 1,919.1 | 1,847.3 |
| Payload, depth48 | 43.016 | 18.296 | 26.989 | 1,920.5 | 1,928.3 | 1,869.5 |

**Saving traffic does not necessarily lower peak memory.** Across the 48 reunion configurations, requested traffic is below Copy in 38 and above it in ten; peak growth is below Copy in 14 and above it in 34. These are coverage counts, not workload weights or an architectural score. Short two-owner one-query plain execution requests 40,898 bytes under reunion versus 40,048 under Copy; the corresponding payload case requests 476,546 versus 365,936 bytes.

**The retained-answer boundary matters.** On the four-owner depth0 payload query, the first query's live growth immediately after execution is 1,963,138 bytes. Releasing the engine leaves 1,845,514 bytes, releasing answers leaves 120,810, and releasing the input leaves the 11,370-byte prepared owner. The consumer's retained answers dominate this endpoint. These are observed release boundaries; they do not identify every internal cache owner or establish sustained-stream behavior.

**Permanent factoring finds a useful independent region in the payload source.** Its analysis keeps the joined computation coupled but separates the ground payloads, producing two regions. Plain/equal/late sources produce one region. A preflight assertion that expected one region on every source stopped the second process; one prior reunion sample had completed. Both receipts, source snapshots and original binaries are preserved. The corrected matrix starts afresh and charges the actual two-region path. No source was changed to suppress that decomposition.

## What remains before a timing claim

The [four-package review](S04-reunion-ownership-breadth-review.md) selected the identity-copy attribution because it could change the adverse payload result at low implementation cost. The attribution is T077's fifth package. It supports the repair and leaves clear contrary cases; further allocation tuning is not automatically next.

Qualify stronger execution/elimination controls before registering ordinary timing. The current Indexed path uses generic prepared execution. Existing inferred specialization and Scan access must be considered. The current [carrier checker](../../../research/chr-compiled/src/carriers.rs) requires exactly two rule uses per predicate and variable slots in non-control arguments; these sources have multiple owner-specific walk rules and literal owner keys. Its inability to accept them is a checker limitation, not evidence that their ground countdown work is unavoidable.

This distinction can change the architectural interpretation: reunion avoids repeating private work, while source analysis may eliminate that work. Investigate the applicable existing specialization and the direct-elimination opportunity on these same complete sources. Preserve unknown-input, resource, observation and eligibility limits rather than treating the finite measured fragment as a general compiler. Register timing only once this stronger comparison is concrete.

T072 integrated dependency repair remains the strongest separate alternative. The next control qualification is bounded and can invalidate an apparent sharing advantage before a large timing matrix; after that boundary reassess integration rather than opening another unbounded reunion refinement cycle. Repeated dynamic reunion, broader ownership inference, adaptive splitting, sustained lifetime, language tradeoffs and whole architectures remain required.

## Evidence and scope

[Initial allocation audit](s04-reunion-ownership/audit.json), [initial summary](s04-reunion-ownership/summary.json), [initial freeze](s04-reunion-ownership/freeze.json), [paired audit](s04-reunion-identity/audit.json), [corrected summary](s04-reunion-identity/summary.json), [paired freeze](s04-reunion-identity/freeze.json), [ordinary tests](s04-reunion-identity/tests.log) and [diagnostic tests](s04-reunion-identity/tests-diagnostic.log) retain the result. Individual JSON receipts contain all phase readings and commands. Baseline source snapshots make the paired attribution reproducible even though the current implementation includes the repair.

The meter records requested allocations, not RSS, and collects no elapsed timings in this gate. Source-rule construction, oracle work, validation, process startup and compilation are outside measured phases. First-answer cancellation follows the complete queries over the same prepared owner; final owner release is included in the complete-query sum. Consumers retain each query's answers until its release phase. No timing, compilation-amortization or universal architecture ranking is established.

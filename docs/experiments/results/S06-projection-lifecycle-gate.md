# Projection avoids hidden search, but output can dominate its representation

A scope-based ordering policy now preserves the independently enumerated answers and avoids the connected star's large intermediate factor. Owned output expansion also passes reuse, cancellation and disposal checks. The next cost comparison is credible for finite weighted answers and expanded multisets; it cannot yet claim equivalent ordered CHR execution.

The important limit is visible in the ownership evidence: dense output is fully materialized before the first tuple is delivered. Compact hidden counts do not eliminate required output work. Neither the assignment counts nor the allocation observations establish a runtime advantage.

## What the ordering experiment establishes

The greedy policy estimates each possible elimination from the union of its current factor scopes and distinct domain sizes. It chooses the smallest estimated Cartesian product, with deterministic width/coordinate tie-breaking. It uses the input, not future answers or workload labels. Subsequent comparisons must charge constructing the policy and compare its choices with fixed orders and competent alternatives.

On the eight-coordinate connected star, it selects hidden leaves before the center. That order requires **28 elimination visits and a largest factor of four entries**, versus the entry's center-first control with 504 visits and 128 entries. Both preserve the same visible correlation and multiplicity. These visits involve different scopes of work, so their ratio is not a speedup.

Across 256 generated problems, eight visible masks and both set/count semantics, greedy, ascending and descending orders pass **12,288 complete weighted-map comparisons per build**. Greedy results additionally pass 4,096 expanded-multiset checks and 18,432 later-restriction comparisons per build. The oracle directly enumerates original domain-choice tuples and tests relation membership; it does not use elimination. These small generated problems validate denotation, not the heuristic's quality on larger graphs.

## Expansion preserves multiplicity, not derivation order

The new iterator owns a complete weighted answer map and produces independently owned tuples. It checks the total output limit before expansion; arithmetic overflow, an output-bound error and exhaustion are distinct. A 128-coordinate witness distinguishes individually representable weights from an overflowing total. Dropping an iterator cancels remaining expansion without visiting every repeated answer.

Grouping loses order information. Two Boolean choices, with the first hidden, have visible sequence `0,1,0,1` under direct lexicographic choice traversal. Grouped expansion gives `0,0,1,1`. Their multisets agree, but their sequences differ. This is a finite traversal counterexample, not a new claim about every permitted CHR scheduler. It establishes that the weighted map alone does not reconstruct that order.

First expanded output also waits for the entire weighted map. Consequently a lifecycle comparison must charge weighted materialization before first observation. Streaming enumeration is an important competing endpoint, particularly for cancellation and dense output. General CHR source eligibility, effectful hidden work, caller-shared identities and arbitrary structural paths remain separate obligations; the current declaration check does not infer privacy.

## Ownership is accounted for through changing queries and cancellation

The diagnostic uses the existing requested-allocation meter. Three families cross immediate release, a four-answer window and retained-all consumers with exhaustion or cancellation after eight outputs. Four changing queries reuse one projection. Each of the 18 configurations runs in two independent processes, with exactly matching diagnostic records.

Before the fourth query's expansion, the source, elimination order and prepared projection are disposed. The iterator remains usable. Once the iterator is exhausted or cancelled, every remaining measured live byte belongs to the explicitly retained consumer buffers. Disposing those buffers returns live heap exactly to the initial baseline in all 36 qualified processes.

The table shows the full-exhaustion, immediate-release configuration for each family. Bytes are requested heap sizes, not RSS. These are observations of one implementation, not comparisons against other solvers.

| Family | Prepared projection after source/order disposal | Pending fourth-query outputs after producer disposal | Ordering allocation requests | Allocation requests across the complete four-query lifecycle |
|---|---:|---:|---:|---:|
| 12 independent Boolean coordinates, one visible | 6,666 B | 466 B | 12,672 B | 60,688 B |
| Eight-coordinate connected star, two visible | 1,868 B | 472 B | 8,336 B | 37,284 B |
| Ten visible Boolean coordinates, dense output | 5,848 B | 91,984 B | 2,824 B | 316,124 B |

The dense case retains 1,024 distinct tuples before expansion. The independent case retains only two weighted tuples, but must produce 4,096 owned observations for its unrestricted query. Expansion can move unique tuples out of the map; repeated tuples require independent owned values. This explains why small pending state and total output work are separate questions.

Retaining every delivered answer across the four queries leaves 405,504 B, 13,062 B and 129,024 B of consumer buffers respectively. A four-answer window leaves 100 B, 104 B and 136 B. These bytes include vector capacity, not just tuple payloads. They survive producer disposal by design and disappear on consumer disposal. This bounded conservation check does not establish sustainable memory for ongoing sources.

The current projection also retains deduplicated domains for eliminated coordinates and clones its domain collection for each weighted observation. The diagnostic charges these owners. Whether compacting them changes the architectural cost conclusion requires attribution against a credible competitor; the present evidence does not establish that they are harmless or decisive.

## What runs next, and why

Keep T076 active and register a bounded lifecycle pilot for explicitly equivalent finite endpoints. Compare greedy and fixed elimination orders with streaming backtracking enumeration, an independence-recognizing control for separable inputs, and existing structural solving where its meaning applies. Vary hidden work, connected intermediate width, dense visible output and changing-query reuse independently. Separate weighted-answer requests from expanded-multiset requests; preserve first/full observation, cancellation and disposal costs. Source-ordered CHR use needs additional correspondence evidence before it enters that claim.

Adaptive search ownership/costs remain the strongest ready distinct alternative. Projection receives the next cost entry because this qualification now supplies concrete favorable/adverse representation costs and valid endpoints; measurement could decide whether eliminating hidden work repays those obligations. Do not add further projection refinements merely because the code is familiar. Reconsider adaptation, native local ownership, conditional equality/lifetime and broader reuse at the pilot or a consequential obstruction.

This is package two after the descriptor breadth review. Review all directions by package four, counting any separate attribution package. No timing matrix ran here. Broader theories, language policy, complete architectures and the research goal remain unresolved.

## Evidence and validation

The [prospective registration](../registrations/S06-projection-lifecycle-gate.md), [independent tests](../../../research/chr-structural/tests/projection_lifecycle.rs), [ownership probe](../../../research/chr-structural/examples/projection_ownership.rs), [bounded driver](../../../research/chr-structural/experiments/projection_lifecycle_gate.py), [audit](../../../research/chr-structural/experiments/audit_projection_lifecycle.py) and [raw evidence](s06-projection-lifecycle-gate/) identify the exact sources, commands and bounds.

The final [qualified audit](s06-projection-lifecycle-gate/qualified/audit.json) checks six successful bounded correctness processes: seven new lifecycle tests, ten existing projection tests and sixteen finite-path tests in each feature configuration. It also checks all 36 owner diagnostics and their exact pairs. Every process has 60-second wall/CPU and 1 GiB address-space limits. Clippy passes for default tests and the metrics-off allocation probe/tests; formatting passes. The reference interpreter is unchanged.

The initial receipts and source snapshots remain available alongside the qualified run. The audit checks their frozen hashes using the preserved snapshots and original binaries. The earlier projection entry's production source is also preserved and hash-checked against its manifest. Reported numbers above come from the qualified run.

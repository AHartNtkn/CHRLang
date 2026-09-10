# Capacity solving lowers measured lifecycle cost in its closed source fragment

Capacity solving meets the registered gain criterion in 138 of 144 comparisons against four ordinary execution controls. Six zero-request comparisons remain unresolved; none is classified as a loss. These are bounded lifecycle results for the admitted consuming-capacity source, not a general solver or architecture ranking.

## The comparison includes the work around solving

The [prospective registration](../registrations/S06-capacity-cost-pilot.md) fixes all 180 previously qualified configurations: five execution modes, requests 0/2/4, empty/tight/spare supplies, choice weights one/two and reuse one/four. Each batch constructs the source, prepares it once and executes changing independent/aliased queries. Every complete answer retains chosen outputs, done occurrences, spare tokens and inert noise.

Capacity is compared separately with generic Scan, generic Indexed, inferred-specialized Scan and inferred-specialized Indexed. The primary interval sums source construction, inference/preparation, query construction, joint setup/execution/complete observation/engine disposal and all recorded input/prepared/consumer/source disposal. This charges successful answer production as well as failure rejection.

The pilot runs 360 excluded warmups and 2,520 measured primary processes. Seven paired blocks on each CPU randomize scenario order and the five mode orders. All 2,880 processes complete with independent mathematical, scalar-source and compiled-source answer checks outside measured phases. The launcher also verifies analytical counts. Source and binary hashes match the [lifecycle qualification](S06-capacity-lifecycle-entry.md).

A descriptive practical gain requires median paired ratio at most 0.90 and all seven pairs faster on each CPU; loss uses the reverse direction and a 1.10 threshold. This criterion is fixed before execution. It establishes neither inferential significance nor unseen-program prediction. Counts describe registered contrasts, including some equivalent empty-query shapes; they are not workload weights.

| Control | Gains | Unresolved | Losses |
|---|---:|---:|---:|
| Generic Scan | 33 | 3 | 0 |
| Generic Indexed | 35 | 1 | 0 |
| Specialized Scan | 35 | 1 | 0 |
| Specialized Indexed | 35 | 1 | 0 |

## Gains are not confined to immediate failure

Against specialized Scan, two requests with tight supply and one query have median paired ratios 0.497 and 0.499 on CPUs 0 and 1. Both modes produce the same two complete answers. Four requests, tight supply and four changed queries have ratios 0.276 and 0.240, including alternating successful independent queries and infeasible aliased queries.

The output-heavy case—four requests, spare supply, duplicate choice weight two and four queries—has ratios 0.133 and 0.141 against specialized Scan. It produces raw answer counts [256, 32, 256, 32]. Capacity still enumerates required successful assignments and duplicates their weighted outputs; it does not discard repeated source derivations to obtain the gain.

Representative pooled lifecycle medians and joint execution medians, in milliseconds:

| Requests / supply / weight / reuse | Capacity total | Specialized Scan total | Capacity joint execution | Specialized Scan joint execution |
|---|---:|---:|---:|---:|
| 2 / tight / 1 / 1 | 0.0477 | 0.0957 | 0.0229 | 0.0512 |
| 4 / tight / 1 / 4 | 0.1150 | 0.4719 | 0.0812 | 0.4197 |
| 4 / spare / 2 / 4 | 1.0612 | 7.7792 | 0.8301 | 7.2255 |
| 4 / empty / 1 / 4 | 0.0393 | 0.3160 | 0.0133 | 0.2704 |

The phase records locate consequential savings in the joint execution interval, with lower preparation cost as well in these witnesses. For tight four-request reuse, preparation medians are 9.77 µs capacity and 27.52 µs specialized Scan. Thus this pilot does not reveal a preparation penalty that explains away the gains. Separate phase medians need not sum, and the joint interval does not isolate propagation from answer construction.

The six unresolved contrasts all have zero requests and one query. Their median ratios favor capacity, but at least one pair fails the unanimity criterion. These tiny cases do not establish equal cost or a loss. Further repetition merely to change that label has less architectural value than a distinct reuse comparison; no deployment threshold or automatic policy is being selected from them.

## Every registered contrast

Entries show pooled capacity/control median lifecycle ratio, followed by G (gain) or U (unresolved). Classification uses per-CPU paired ratios as defined above, not the pooled display ratio. Exact pairs, extrema and phases are in the analysis artifact.

| Requests | Supply | Weight | Reuse | Scan | Indexed | Specialized Scan | Specialized Indexed |
|---|---|---:|---:|---|---|---|---|
| 0 | empty | 1 | 1 | 0.600 U | 0.541 G | 0.543 U | 0.549 G |
| 0 | empty | 1 | 4 | 0.541 G | 0.524 G | 0.512 G | 0.474 G |
| 0 | empty | 2 | 1 | 0.489 G | 0.568 G | 0.567 G | 0.571 U |
| 0 | empty | 2 | 4 | 0.505 G | 0.530 G | 0.492 G | 0.508 G |
| 0 | tight | 1 | 1 | 0.697 U | 0.676 U | 0.639 G | 0.591 G |
| 0 | tight | 1 | 4 | 0.587 G | 0.519 G | 0.528 G | 0.499 G |
| 0 | tight | 2 | 1 | 0.580 U | 0.531 G | 0.524 G | 0.523 G |
| 0 | tight | 2 | 4 | 0.503 G | 0.493 G | 0.431 G | 0.459 G |
| 0 | spare | 1 | 1 | 0.629 G | 0.625 G | 0.605 G | 0.563 G |
| 0 | spare | 1 | 4 | 0.548 G | 0.551 G | 0.509 G | 0.500 G |
| 0 | spare | 2 | 1 | 0.616 G | 0.556 G | 0.541 G | 0.562 G |
| 0 | spare | 2 | 4 | 0.532 G | 0.493 G | 0.500 G | 0.495 G |
| 2 | empty | 1 | 1 | 0.376 G | 0.324 G | 0.353 G | 0.315 G |
| 2 | empty | 1 | 4 | 0.270 G | 0.235 G | 0.298 G | 0.235 G |
| 2 | empty | 2 | 1 | 0.214 G | 0.165 G | 0.224 G | 0.173 G |
| 2 | empty | 2 | 4 | 0.108 G | 0.087 G | 0.114 G | 0.089 G |
| 2 | tight | 1 | 1 | 0.540 G | 0.452 G | 0.498 G | 0.443 G |
| 2 | tight | 1 | 4 | 0.341 G | 0.276 G | 0.289 G | 0.283 G |
| 2 | tight | 2 | 1 | 0.290 G | 0.241 G | 0.293 G | 0.236 G |
| 2 | tight | 2 | 4 | 0.162 G | 0.126 G | 0.157 G | 0.134 G |
| 2 | spare | 1 | 1 | 0.524 G | 0.442 G | 0.546 G | 0.411 G |
| 2 | spare | 1 | 4 | 0.395 G | 0.315 G | 0.421 G | 0.337 G |
| 2 | spare | 2 | 1 | 0.320 G | 0.256 G | 0.325 G | 0.261 G |
| 2 | spare | 2 | 4 | 0.233 G | 0.186 G | 0.235 G | 0.187 G |
| 4 | empty | 1 | 1 | 0.198 G | 0.161 G | 0.217 G | 0.170 G |
| 4 | empty | 1 | 4 | 0.111 G | 0.087 G | 0.124 G | 0.092 G |
| 4 | empty | 2 | 1 | 0.020 G | 0.015 G | 0.021 G | 0.015 G |
| 4 | empty | 2 | 4 | 0.012 G | 0.009 G | 0.013 G | 0.010 G |
| 4 | tight | 1 | 1 | 0.296 G | 0.235 G | 0.311 G | 0.243 G |
| 4 | tight | 1 | 4 | 0.219 G | 0.177 G | 0.244 G | 0.190 G |
| 4 | tight | 2 | 1 | 0.059 G | 0.046 G | 0.062 G | 0.047 G |
| 4 | tight | 2 | 4 | 0.050 G | 0.039 G | 0.052 G | 0.041 G |
| 4 | spare | 1 | 1 | 0.407 G | 0.331 G | 0.389 G | 0.334 G |
| 4 | spare | 1 | 4 | 0.320 G | 0.258 G | 0.335 G | 0.270 G |
| 4 | spare | 2 | 1 | 0.147 G | 0.109 G | 0.155 G | 0.113 G |
| 4 | spare | 2 | 4 | 0.128 G | 0.104 G | 0.136 G | 0.104 G |

## What this establishes and what it does not

The measured candidate replaces occurrence matching and consuming-rule execution with source admission, grouped weighted domains, capacity constraints and bounded extraction. It uses ordinary allocation and compiles branch/prune diagnostics out. Visited states remain an operational limit. Its supported language is still a closed finite producer/consumer/failure-sink fragment, with grouped aliases, ground token keys and preserved inert residuals.

The result supports carrying direct resource reasoning into coherent architecture comparisons. It does not establish that all CHR resources can be expressed this way, that capacity inference is always cheap, or that a general language should require these restrictions. Competing owners, replenishment, partial token information, surrounding live effects and missing failure sinks remain separate admission and language questions.

Complete outputs remain retained through prepared disposal and validation before consumer disposal. Outer reserved bookkeeping, independent validation and external process/publication costs are outside the interval sum. Whole-process time includes oracle work and cannot be used as engine cost. Clock calls and validation affect the allocator/cache context shared by candidates. This is not an incremental-cancellation, first-publication, sustained-retention, native-compilation or whole-architecture comparison. Allocation evidence remains the separate qualification, not diagnostic timing mixed into this pilot.

## Four-package breadth review and next investigation

The capacity sequence now has a semantic entry, source-derived solver, lifecycle qualification and cost pilot. It has answered a bounded question: direct source-derived resource constraints can preserve the stated semantics and reduce recorded full lifecycle cost relative to all four tested ordinary organizations. Wider eligibility and lifetime remain required; the initial question no longer needs another small timing matrix to become decision-relevant.

Select T075 for renaming-aware reuse in the finite-phase comparison. First inspect the existing [call table and transport](S05-call-transport-gate.md), [checked phase entry](S05-call-entry-gate.md) and [caller cost pilot](S05-caller-cost-pilot.md). Renaming-aware call reuse already exists and has measured favorable/adverse regimes; do not reconstruct it as an untested mechanism. The specific missing contrast is applying a credible reusable-result control to the multiple initial private goals and changed finite domains used by the learning study, with complete caller behavior and answer remapping.

The current call table admits one initial private-family occurrence and requires single-head private rules. The finite-learning queries begin with several private domain/entry occurrences, and the capacity source adds actual two-head resource consumption. Determine which boundary can be soundly adapted or generalized, and where an explicit restriction remains necessary. Reuse existing key, fresh-identity transport and lifecycle evidence wherever applicable. A cache that recognizes only identical debug strings is insufficient for the renamed-query contrast.

The strongest alternative is extending capacity inference to a phase inside surrounding effects. It could widen the gain's language relevance but needs new source-order, shared-resource and publication correspondence. The reuse investigation can challenge the already measured learning recommendation and offers existing implementations to inspect. Select its bounded applicability/semantic gate first; review broader capacity phases, compilation and sustained lifetime at that gate or an obstruction before a larger cache campaign. T073 remains unfinished, and the architecture goal remains active.

## Evidence

[Manifest, exact order and frozen hashes](s06-capacity-cost-pilot/manifest.json), [full process records](s06-capacity-cost-pilot/runs.jsonl), [all contrasts and attribution](s06-capacity-cost-pilot/analysis.json), [validation hashes](s06-capacity-cost-pilot/validation.json), [run log](s06-capacity-cost-pilot/run.log).

The [runner and analyzer](../../../research/chr-hvm/resource_capacity/cost_pilot.py) refuses to overwrite observations. Recheck the existing evidence with `python research/chr-hvm/resource_capacity/cost_pilot.py --analyze`. Per-process limits are 60 wall seconds, 20 CPU seconds and 1 GiB; the launcher is bounded to 1,000 seconds. No source or execution engine changed during the pilot.

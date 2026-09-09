# Shared deductions: useful reuse, stronger complete controls

**Sharing compatible deductions saves work, but compiled execution remains faster on every substantive configuration tested.** Persistent maps reduce cache retention costs. The exact-source control wins every comparison against persistent shared deductions, making source-derived elimination the stronger next investigation.

This is a bounded execution result. It does not reject integration, establish a general compiler, or include native compilation costs.

## What was compared

The [prospective registration](../registrations/S02-deduction-confirmation.md) fixes four source families: one branch; four branches repeating the same equality; four distinct equalities; and repeated equality after different context bindings. Each optionally consumes a branch-local token. Depths zero and 32, one/four changing queries, and both insertion orders give 64 source configurations. Preparation is reused; equality caches belong to individual queries.

Nine modes retain ordered/persistent equality maps with caching both off and on, the relational executor, compiled scanning/indexing, inferred specialization with indexing, and exact-schema source elimination. The last control validates the entire source and query shape. It does not derive an optimization for arbitrary programs.

All 5,220 processes succeeded: 4,032 ordinary timings, 1,152 separate allocation runs and 36 cancellation runs. Complete answers were checked against the independent scalar evaluator outside the primary interval. All 576 allocation pairs replay exactly; query and prepared requested-live bytes return to their starting values. Source and binary hashes agree with the preceding validated build and current worktree. See the [audit](s02-deduction-confirmation/audit.json), [freeze](s02-deduction-confirmation/freeze.json) and [complete results](s02-deduction-confirmation/summary.json).

## Registered comparisons

A gain means the upper pointwise 95% bootstrap bound on the primary time ratio is below 0.90; a loss means the lower bound exceeds 1.10. Seven paired blocks and 10,000 resamples follow the registration. Unresolved means this experiment does not distinguish a practical gain or loss; it does not mean equivalence. These counts describe the chosen cases, without workload weights or a population-wide ranking.

| Numerator / denominator | Gains | Losses | Unresolved |
|---|---:|---:|---:|
| shared/contextual | 7 | 25 | 32 |
| persistent-shared/persistent | 8 | 12 | 44 |
| persistent-shared/shared | 27 | 0 | 37 |
| persistent/contextual | 0 | 29 | 35 |
| persistent-shared/contextual | 9 | 27 | 28 |
| shared/relational | 56 | 0 | 8 |
| persistent-shared/relational | 63 | 0 | 1 |
| contextual/scan | 17 | 32 | 15 |
| shared/scan | 13 | 33 | 18 |
| persistent-shared/scan | 12 | 32 | 20 |
| persistent-shared/sealed | 17 | 32 | 15 |
| lowered/persistent-shared | 64 | 0 | 0 |
| lowered/scan | 64 | 0 | 0 |

`contextual` uses ordered maps without caching; `shared` adds caching. `persistent` changes the maps without caching; `persistent-shared` combines both. `sealed` is inferred specialization with indexed access.

## Where the result changes

**Compatible reuse helps; different work does not repay the same machinery.** At depth 32, persistent shared deductions gain against uncached ordered maps in all eight shared-family configurations and lose in the other 24. Ordered caching gains in seven of eight shared cases, with one unresolved, and loses in the other 24. Persistent maps gain over ordered cached maps in 27 cases overall with no classified losses; uncached persistence has 29 losses and no gains. The representation and reuse policy must be evaluated together.

**Compiled controls retain a substantive advantage after the representation repair.** Persistent shared deductions lose to scanning and specialization in all 32 depth-32 cases. At depth zero they gain in 12 scanning comparisons and 17 specialization comparisons, with the rest unresolved. Shallow setup savings are a real contrary regime; they do not establish broad superiority.

**The relational comparison alone would give an incomplete architecture recommendation.** Persistent shared deductions beat it in 63 configurations, yet exact-schema elimination beats persistent shared deductions and scanning in all 64. Further precision on relational overlap is unlikely to change the bounded selection while that stronger control applies.

Representative median primary milliseconds: depth 32, four queries, token present, forward starting order. Each total includes preparation, setup, complete execution/observation and engine, answer and prepared disposal.

| Family | Ordered, no cache | Ordered, cache | Persistent, no cache | Persistent, cache | Relational | Scan | Indexed | Specialized | Exact source |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| single | 0.467 | 0.822 | 0.612 | 0.581 | 1.084 | 0.141 | 0.176 | 0.180 | 0.032 |
| shared | 1.441 | 0.878 | 1.712 | 0.817 | 3.595 | 0.299 | 0.324 | 0.332 | 0.032 |
| distinct | 1.448 | 3.196 | 2.089 | 2.346 | 5.206 | 0.587 | 0.632 | 0.599 | 0.087 |
| changed | 1.437 | 3.268 | 1.791 | 2.205 | 3.917 | 0.298 | 0.312 | 0.336 | 0.030 |

## Allocation, observation and lifecycle

The compatible shared example reduces requested allocation from 1.835 MB with uncached ordered maps to 0.802 MB with persistent caching. Scanning requests 0.380 MB and exact elimination 0.037 MB. Peak requested growth is respectively 56.8, 64.0, 75.3 and 8.3 kB: lower traffic does not necessarily mean lower peak retention. These are allocator-requested bytes, not RSS.

In that example, persistent caching spends a median 0.752 ms executing and observing, versus 0.187 ms for scanning and 0.004 ms for exact elimination. Median first-answer times are 0.163, 0.030 and 0.0002 ms. Source/input-inclusive medians are 0.825, 0.309 and 0.040 ms. The advantage therefore is not explained merely by moving preparation outside the reported primary total. These phase medians describe costs; no separate phase significance test was registered.

The single, distinct and changed families request 0.724, 2.915 and 2.786 MB with persistent caching, versus 0.508, 2.011 and 1.845 MB with uncached ordered maps. Exact state keys miss across irrelevant context bindings; general relevance keys could change that cost, but need their own correctness and lifetime evidence. The preceding [map attribution](S02-equality-map-attribution.md) isolates snapshot representation; this confirmation compares complete controls and does not attribute every cross-engine difference to caching.

Ordinary timing disables engine, kernel and observer counters and uses the ordinary allocator. Meter timing is not used for speed claims. Disposal and early cancellation pass, but these short lifetimes do not establish sustained retention, eviction or consumer behavior. Native compilation remains outside this comparison.

## Disposition

Carry compatible reuse and the adverse distinct/shallow regimes into later architecture comparisons. Keep exact-state memoization distinct from union-find through CHR, generalized validity and strategic port rewrites. Their mechanisms remain unresolved.

The [four-package breadth review](S02-deduction-breadth-review.md) selects reusable source-derived lowering next. T072 remains unfinished. Neither this result nor the selection closes the research goal.

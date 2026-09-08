# Repeated large equations can repay cache maintenance

Dependency caching saves about 25–35% of runtime for common equations with 256–1,024 constructor rows and 64 requests in this experiment. Distinct large requests instead take about 2.75 times the ordinary runtime. This establishes a useful regime for reuse, while leaving validity, lifetime and broader architectural choices open.

## What this comparison answers

The earlier [specialized-control comparison](S05-specialized-control.md) found much less runtime benefit at depth64 than its large work savings suggested. This experiment independently increases equation depth and requests within a query to test whether recognition and replay can repay their overhead. It uses the existing ordinary scalar executor, dependency cache, direct choice graph and inferred-specialized compiled executor; it adds no production baseline.

The [prospective registration](../registrations/S05-operation-crossover.md) fixes 22 sources and four executors: 88 cells, five timing repetitions and two allocation repetitions. All 616 processes completed within 60 seconds and 1 GiB per process. Complete source observations passed independent validation, all 88 allocation cells replayed exactly, and disposal restored measured live allocations. Work diagnostics ran twice separately and matched the registered common-operation predictions.

## Where reuse pays

Each entry below is the median paired dependency-cache/ordinary runtime ratio, followed by the observed five-pair range. Below 1 favors caching. Bold entries meet the registered practical criterion: at least 20% median improvement and every pair below 1. These are bounded pilot observations, not confidence intervals.

### Common success before discrimination

| Constructor rows | 4 requests | 16 requests | 64 requests |
|---|---|---|---|
| 64 | 1.090 [1.060, 1.192] | 0.933 [0.860, 0.981] | 0.931 [0.912, 0.990] |
| 256 | 1.045 [0.950, 1.062] | 0.885 [0.873, 0.904] | **0.746 [0.723, 0.856]** |
| 1024 | 1.037 [0.993, 1.080] | 0.802 [0.734, 0.820] | **0.647 [0.330, 0.941]** |

### Common clash before discrimination

| Constructor rows | 4 requests | 16 requests | 64 requests |
|---|---|---|---|
| 64 | 1.099 [0.973, 1.207] | 0.897 [0.828, 0.939] | 0.821 [0.760, 0.910] |
| 256 | 1.070 [1.005, 1.249] | 0.956 [0.931, 1.100] | **0.690 [0.533, 0.753]** |
| 1024 | 1.092 [1.024, 2.509] | 0.948 [0.896, 0.971] | **0.691 [0.639, 0.731]** |

Four requests do not establish a practical benefit at any tested depth. At 1,024 rows and 16 successful requests, the median ratio is 0.802: close to, but outside, the registered threshold. Refining that exact boundary would not change the present conclusion that sufficiently repeated common operations can pay and unique requests can lose.

The after-discrimination success case at 1,024 rows/64 requests improves by about 44% (ratio 0.562, range 0.550–0.578). The corresponding clash median improves by 26%, but one pair loses (range 0.685–1.361); it does **not** meet the practical criterion. Its precise runtime benefit remains uncertain. Distinct requests lose about 40% at depth64 and 176% at depth1,024, with every pair unfavorable.

## Work savings and complete-path costs

At depth1,024 with 64 common successful requests, ordinary execution visits 131,454 equation pairs; dependency reuse visits 2,064. Both execute the same 190 source equations. The cache records 177 hits, including 63 substantive repeated operations and 114 choice equations. The smaller runtime improvement therefore measures the remaining recognition, source execution, setup, observation and lifetime costs as well as saved equation traversal.

The table shows complete runtime lifecycle medians for the largest common-before-discrimination and distinct-request sources. Requested traffic is cumulative requested heap allocation; peak is the measured live increment. Neither is RSS. Ratios above are paired within repetitions and need not equal ratios of these separate medians.

| Source, 1,024 rows / 64 requests | Executor | Runtime ms | Requested MiB | Peak KiB |
|---|---|---:|---:|---:|
| Common success | ordinary | 2.845 | 10.479 | 866.4 |
| Common success | dependencies | 1.856 | 2.763 | 866.4 |
| Common success | graph | 91.384 | 158.081 | 526.0 |
| Common success | specialized | 29.919 | 65.869 | 37426.3 |
| Common clash | ordinary | 1.246 | 9.384 | 866.4 |
| Common clash | dependencies | 0.860 | 1.655 | 866.4 |
| Common clash | graph | 0.781 | 1.527 | 526.0 |
| Common clash | specialized | 20.589 | 57.583 | 36890.1 |
| Distinct success | ordinary | 2.723 | 10.475 | 922.1 |
| Distinct success | dependencies | 7.489 | 19.046 | 922.1 |
| Distinct success | graph | 621.039 | 1693.871 | 968.5 |
| Distinct success | specialized | 26.138 | 59.277 | 37512.0 |

The direct graph retains a meaningful contrary result: common clash before discrimination uses less traffic and peak heap than the dependency cache. Its paired runtime ratio is 0.899 [0.840, 0.947], consistently lower but short of the 20% practical threshold. Common success has a substantially slower graph runtime despite lower peak heap. Reuse therefore does not dominate sharing across all efficiency dimensions.

The inferred-specialized control does not overturn the ordinary or cached scalar results on these sources. This is not evidence that compilation intrinsically loses: source selection and branch arena ownership differ from the query-owned shared arena. Other storage and compiled organizations remain unresolved. After-discrimination graph costs are particularly large (roughly 561–622 ms at the largest size); these results describe this executor, not all direct graphs.

## Complexity and limits of the finding

The dependency cache adds premise collection, validity checks, binding-delta replay, entry indexing and eviction to the ordinary source executor and unifier. It does not replace those services. Exact-context reuse instead retains binding roots; this matrix omits it because the preceding changed-choice sources had zero hits. That omission does not resolve exact-context reuse on other sources.

Capacity is 32 entries, and the constructor arena belongs to a whole query. Evicting an entry does not reclaim that arena. Distinct sources contain 64 substantive keys, so they also impose replacement pressure; the loss cannot be attributed solely to either request uniqueness or capacity. Dependency width, invalidation patterns, useful revisits after eviction and sustained ownership require separate contrasts.

Each cell prepares rules and measures one query, including setup, execution with observation, engine/answer disposal and prepared disposal. A separate run measures first answer or exhaustion and cancellation; it is not added to primary runtime. Zero-answer cases have no first-answer latency. Two warmup queries precede measured preparation. Native compilation, process startup, source generation and oracle work are excluded, so this is not complete deployment-lifecycle superiority. Rule count also grows with request count; the curve includes preparation and source-selection effects, not just unification.

The semantic gate covers more behavior than these cost sources. The cost evidence itself concerns finite choices and single-head consuming continuations, with an independently checked answer set. It supplies no workload weights or universal cache policy.

## Disposition and next comparison

T067's bounded stable-reuse trial is complete: correctness, useful runtime opportunity, adverse overhead and alternative-control costs now have receipts. S05 remains open for invalidation, eviction/lifetime and generalized continuation tables. The noisy after-discrimination clash result is explicitly unresolved; neither its best nor worst observed pair changes the established common-success benefit or distinct-request loss.

The next investigation is [demand-driven matching subscriptions](S01-subscription-entry.md). Existing retained left/right pairs already materialize part of a source join; the new question is whether active demands should control what gets retained and updated. That contrast could eliminate unnecessary maintenance or repeated discovery in ordinary multihead execution. The entry compares its decision value and dependencies with reusable parallel workers and corrected replay policies. Those alternatives remain required in the [experimental sequence](../sequence.md).

## Reproducible evidence

- [Registration](../registrations/S05-operation-crossover.md), [source/binary freeze](s05-operation-crossover/freeze.json), [randomized order](s05-operation-crossover/order.json).
- [Audit](s05-operation-crossover/audit.json), [all cell summaries and paired ratios](s05-operation-crossover/summary.json), [phase data](s05-operation-crossover/phases.csv), and per-process receipts in the same directory.
- [Runner](../../../research/chr-reuse/experiments/operation_crossover.py), [source generator](../../../research/chr-reuse/experiments/crossover_source.rs), [independent receipt audit](../../../research/chr-reuse/experiments/summarize_crossover.py).
- Separate build, feature, formatting, Clippy, semantic gate and work-check receipts accompany the timing and allocation receipts. No comparative run uses the work-counter build.

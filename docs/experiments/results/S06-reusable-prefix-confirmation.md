# Reusable preparation has confirmed gains and losses

**Reusable source-derived preparation wins all long-prefix, repeated-query comparisons against the five tested alternatives.** Short queries often favor ordinary execution. Retention reduces repeated setup, but its preparation and memory costs prevent a universal policy.

This [registered confirmation](../registrations/S06-reusable-prefix-confirmation.md) completes all 3,036 processes: 2,352 ordinary timings, 672 allocation runs and 12 cancellation runs. Independent complete answers, all 336 exact allocation replays, and query/prepared disposal checks pass. The [audit](s06-reusable-prefix-confirmation/audit.json) verifies source and binary identity against the validated sizing implementation.

## Where preparation pays

Seven families cover one/four/sixteen plain calls, one/four independent choices, and one/four contradictory calls. Source depths are 1/32; query counts are 1/8; consuming suffixes are absent/present. Reusable preparation retains one or both known alternating predicate signatures. Query keys change. All six modes use Global source order and scanning.

All 14 depth-32/eight-query configurations have a practical gain for reusable preparation against ordinary execution, inferred specialization, per-query lowering, specialized per-query lowering and rebuilding the parameterized artifact. That includes failures and wide entries. The benefit survives charging initial preparation and final disposal.

At depth one, reusable preparation loses to ordinary execution and specialization in 12 of 14 one-query configurations and nine of 14 eight-query configurations. The remaining short cases are unresolved. Repeating a cheap query does not by itself justify a larger prepared representation.

At depth 32 with one query, reusable preparation gains against ordinary execution in 13 cases and specialization in ten. Its comparison with per-query lowering is unresolved in all 14. Those cells do not establish which lowering variant is cheaper for a one-shot long query.

## Full registered comparison

Gain: upper pointwise 95% bootstrap bound on the time ratio is below 0.90. Loss: lower bound exceeds 1.10. Unresolved: neither condition holds. Seven paired blocks and 10,000 resamples follow the fixed registration; no outliers were excluded. Counts describe this suite, without workload weights or a population-wide ranking.

| Numerator / denominator | Gains | Losses | Unresolved |
|---|---:|---:|---:|
| reusable/original | 27 | 21 | 8 |
| reusable/sealed | 24 | 21 | 11 |
| reusable/lowered | 18 | 5 | 33 |
| reusable/lowered-sealed | 18 | 3 | 35 |
| reusable/rebuilt | 22 | 0 | 34 |
| rebuilt/lowered | 0 | 4 | 52 |

`original` is ordinary compiled execution; `sealed` adds inferred specialization. `lowered` rebuilds query-specific target rules; `lowered-sealed` also specializes them. `reusable` retains parameterized target preparation; `rebuilt` prepares that parameterized target again for each query. Rebuilt versus query-specific lowering has no confirmed gains and four losses: parameterization alone is not the demonstrated advantage.

Representative median primary milliseconds, depth32/eight queries with consuming suffixes:

| Family | Ordinary | Specialized | Per-query lowering | Specialized lowering | Reusable | Rebuilt |
|---|---:|---:|---:|---:|---:|---:|
| plain1 | 0.821 | 0.544 | 0.304 | 0.344 | 0.175 | 0.301 |
| plain4 | 2.553 | 1.705 | 0.669 | 0.639 | 0.341 | 0.710 |
| plain16 | 10.817 | 6.439 | 1.971 | 2.011 | 0.885 | 1.918 |
| choice1 | 0.892 | 0.647 | 0.332 | 0.338 | 0.204 | 0.317 |
| choice4 | 6.442 | 4.273 | 1.801 | 1.813 | 1.319 | 1.646 |
| fail1 | 0.774 | 0.545 | 0.322 | 0.334 | 0.164 | 0.293 |
| fail4 | 2.309 | 1.463 | 0.668 | 0.614 | 0.271 | 0.587 |

For plain16 in that table, the registered reusable/per-query-lowering geometric ratio is 0.505, with interval [0.419, 0.676]. This ratio uses paired samples and is not the ratio of the two marginal medians. All configurations and intervals are in the [full summary](s06-reusable-prefix-confirmation/summary.json).

## What the complete cost contains

Primary time includes preparation, setup, complete execution/observation, engine and answer disposal, and prepared disposal. Ordinary allocator timing disables counters. Separate meter runs count requested heap bytes, not RSS. Native compilation is excluded.

For the long repeated plain16 case, reusable preparation rises from 0.047 to 0.477 ms compared with query-specific lowering. Total query setup falls from 1.555 to 0.096 ms, while execution/observation falls from 0.365 to 0.302 ms. Source/input-inclusive medians are 2.037 and 0.949 ms. Avoided setup, rather than an omitted preparation phase, accounts for most of the difference. Individual phase medians need not sum to the median total.

Primary requested allocation falls from 4.450 MB to 1.745 MB in that case, while peak requested growth rises from 88.7 to 140.2 kB. First-answer medians are 0.042 and 0.038 ms. These separate endpoints prevent a lower total from being mistaken for lower peak retention or a proportional improvement in first observation.

Choice4 retains substantial execution and observation: median 1.015 ms under per-query lowering and 0.924 ms with reusable preparation. Its sixteen alternatives still have to be delivered. The artifact eliminates repeated preparation; it does not eliminate all source alternatives or output work.

The short one-shot plain16 case shows the opposite cost balance. Reusable preparation takes 0.153 ms total versus 0.094 ms specialized ordinary execution. Primary requested bytes are 176,025 versus 58,743. Its earlier first observation cannot compensate for the larger preparation under the registered total-cost endpoint.

## Limits and next investigation

The known signature pair is part of the measured interface. Unexpected signatures, source changes, adaptive artifact retention and sustained consumers are not tested by prebuilding those two artifacts. Rebuilt mode is a deliberate no-retention control; it does not prove that signature changes necessarily require repeated construction.

The independent source gate and these measurements cover private acyclic prefixes. They do not establish source-general elimination of the equality or graph workloads in other stages. This comparison contains no exact-schema direct-answer control, so it ranks these six execution paths rather than claiming the best achievable code for this source family.

Remaining one-shot lowering uncertainty matters if selecting that particular variant becomes necessary. It cannot overturn the confirmed opposing short-ordinary and long-repeated regimes. There is no justified workload distribution or universal policy to tune here. More precision on the same cases is therefore lower priority than testing whether recursive/effectful source analysis changes the work an architecture needs.

The [next-package selection](S06-reusable-prefix-next.md) chooses that broader source gate and retains unplanned signatures and lifetime as required work. This is the third package since the latest breadth review; the next completed package triggers the four-package breadth review. T073 and the architecture goal remain active.

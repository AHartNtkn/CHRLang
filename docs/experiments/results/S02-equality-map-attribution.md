# Persistent maps help retained deductions and hurt ordinary updates

**Whole-map copying was a consequential cost of retaining equality transitions.** Persistent maps improve the cached path in 28 configurations, with no classified losses. The same change hurts uncached execution in 28 configurations. Cheap snapshots and cheap ordinary updates remain different requirements.

## What was compared

The [registered experiment](../registrations/S02-equality-map-attribution.md) crosses ordered versus persistent equality maps with transition reuse enabled versus disabled. All four modes use the same source evaluator, constructor arena, equality-state keys, occurrence ownership, matching, cache cap and observation contract. Live occurrences retain their existing map representation.

The persistent variant uses the repository's tested ordered-map kernel. Its updates share unchanged tree paths instead of copying an entire retained map. This adds path-node allocation, traversal, hashing and value copies; persistence is not free when the uncached ordered map can mutate in place.

The experiment completed **2,464 processes**: 1,792 main timings, 512 main allocation diagnostics, 16 cancellation checks, 112 calibration timings and 32 calibration allocation diagnostics. Allocation replay and disposal checks pass. Complete answers match the independent scalar evaluator. Generated equality/fork tests now run under both storage representations and both reuse settings; kernel snapshot tests and scoped Clippy pass. Reference behavior is unchanged.

## Confirmed representation tradeoffs

Each of 64 source configurations has seven same-block timings. Practical classifications use the registered pointwise 95% bootstrap interval and 10% threshold. Counts describe these cells, not workload weights or simultaneous population-wide evidence.

| Numerator / denominator | Gains | Losses | Unresolved |
|---|---:|---:|---:|
| Persistent cached / ordered cached | 28 | 0 | 36 |
| Persistent uncached / ordered uncached | 0 | 28 | 36 |
| Ordered cached / ordered uncached | 8 | 26 | 30 |
| Persistent cached / persistent uncached | 8 | 8 | 48 |

All 24 substantive single/distinct/changed configurations gain when cached maps become persistent. Compatible repeated work has four gains and four unresolved representation comparisons. All shallow representation comparisons are unresolved. The uncached representation has seven substantive losses and one unresolved cell in each family.

Within either representation, compatible substantive repeated work has eight practical cache gains. On persistent maps, distinct and changed-binding work still have eight cache losses between them. The benefit requires actual reuse; inexpensive snapshots do not eliminate validity checks, lookup or retained state.

## Complete costs and memory

The following medians include preparation, setup, execution with complete observation and engine/answer/prepared disposal. Each case runs four changing queries at depth32/33, with resources present and forward starting order. Values are milliseconds.

| Family | Ordered uncached | Ordered cached | Persistent uncached | Persistent cached |
|---|---:|---:|---:|---:|
| Single | 0.385 | 0.824 | 0.609 | 0.594 |
| Compatible repeated | 1.377 | 0.883 | 1.706 | 0.785 |
| Distinct pairs | 1.521 | 2.974 | 2.241 | 2.298 |
| Changed bindings | 1.485 | 2.952 | 1.798 | 2.115 |

Classifications use paired ratios, not ratios of the marginal medians above. Including source/input construction retains these representative orderings. Most differences are in execution with observation, where equality updates and cache operations occur. These measurements do not independently time map operations.

For cached execution, persistent maps reduce both requested allocation and peak requested growth in these substantive cases:

| Family | Ordered → persistent requested MB | Ordered → persistent peak growth kB |
|---|---:|---:|
| Single | 1.026 → 0.724 | 154 → 60 |
| Compatible repeated | 1.105 → 0.802 | 158 → 64 |
| Distinct pairs | 4.079 → 2.915 | 610 → 249 |
| Changed bindings | 4.004 → 2.786 | 613 → 232 |

Without caching, persistent allocation traffic rises: the compatible repeated case requests 2.605 MB versus 1.835 MB for ordered maps, despite a somewhat lower peak. Persistent cached execution also still retains more than uncached execution in the single/distinct/changed cases. The remaining retention belongs in the architectural comparison rather than being hidden by the improvement.

## Calibration and limits

The eight ordered-mode calibration configurations have no classified practical after/before shift. Their paired geometric means range approximately 0.976–1.089. Some intervals still permit material changes, so the calibration is not proof of equivalence. Allocation differences are small and exact: 64–320 extra requested bytes for uncached cases and 2,208–9,152 for cached cases, reflecting the additional map representation fields.

The four-mode contrasts above use the same current binary and are the primary evidence. Earlier compiled/direct-control sizing retains its own binary and exploratory status; its timings are not combined into a new statistical ranking. Confirmation against those controls must use the current source and binaries.

Counter-free ordinary timing and separate allocation builds both check the persistent kernel's metric ownership flags. Requested allocation is not RSS. Native compilation is excluded. No new observation or language restriction is adopted.

## Disposition and next investigation

Retain the persistent cached variant as a credible competitor, alongside the cheaper ordered uncached control. The paired result supports investigating the snapshot representation rather than treating the initial cache overhead as intrinsic. It does not select a universal map policy or justify automatic routing between the variants.

Next confirm complete costs against corrected relational execution, compiled scanning/indexing, inferred specialization and the exact-source control. That comparison must retain both ordered and persistent modes: replacing every map with the variant favorable to caching would weaken the uncached competitor. The runner's `sealed` control uses inferred specialization with Global+Indexed access; it is not the scanning variant of specialization from another experiment.

This is the third package in the current T072 cycle, after the new-deduction gate and lifecycle sizing. One complete-control confirmation is justified because the representation materially changed the candidate's costs and the sources/harness are already ready. After it, perform the four-package breadth review before another map or cache refinement. T073's reusable general lowering and distinct integrated rewrite organizations remain strong alternatives.

Broad validity keys, sustained retention/eviction, union-find through CHR, strategic port rewrites and complete architecture composition remain unresolved. The goal remains active.

[All configurations and calibration intervals](s02-equality-map-attribution/summary.json) · [Source/binary freeze](s02-equality-map-attribution/freeze.json) · [Tests](s02-equality-map-attribution/tests.log) · [Kernel snapshot checks](s02-equality-map-attribution/kernel.log) · [Scoped Clippy](s02-equality-map-attribution/clippy.log)

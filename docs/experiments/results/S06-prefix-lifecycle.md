# Prefix elimination can repay transformation costs; short chains remain adverse

**Source-derived elimination earns total-cost gains on long private chains, even against sealed execution.** Repeated eligibility analysis initially exaggerated some cold costs; a memoized pass reduces that overhead substantially. The corrected short-chain comparison still contains losses. This supports a conditional optimization opportunity, not universal lowering.

## Complete costs and controls

Four paths execute the same source: ordinary Global scanning, sealed specialization, prefix lowering into ordinary Global scanning, and prefix lowering into sealed execution. The sources vary chain length1/8/32, one/four calls, deterministic/choice terminal equations, consuming endpoints absent/present and one/eight changed queries. Choice with four calls retains all 16 raw alternatives, including residual unmatched consuming heads.

Preparation includes source analysis or target preparation. Lowered query setup includes expansion, freshening, emitted-program preparation, optional sealing, engine creation and temporary artifact disposal. Original preparation is reused. Execution with complete observation, engine/answer disposal and final preparation disposal all count. Source/input construction and first-answer latency are separate. The harness Rust build is not a measured native compiler cost.

The [sizing registration](../registrations/S06-prefix-lifecycle-sizing.md) produced 192 timing, 64 allocation and eight cancellation/reuse processes. The [confirmation](../registrations/S06-prefix-lifecycle-confirmation.md) ran 192 warmups, 1,344 measured ordinary processes and 384 allocation processes. All completed answers passed independent scalar comparison; allocation replays agreed exactly and query/prepared disposal restored baselines. No cutoff occurred.

Primary timings use counters disabled and the ordinary allocator. Allocation instrumentation is separate and reports requested heap bytes, not RSS. Seven paired same-block ratios with a median difference of at least 10% and consistent direction determine practical gains/losses; otherwise a cell is unresolved. These are descriptive criteria, not confidence intervals or workload weights.

## What the initial confirmation found

Original prefix lowering had eight practical gains, thirteen losses and 27 unresolved comparisons against sealed execution across 48 source configurations. The long-chain, eight-query consuming cases gained, including one-call cases. Some long-chain cold cases lost despite fewer source applications.

Phase accounting made the cold loss consequential to investigate. In the plain single-call length32 case, source analysis consumed much more time than sealed preparation. The implementation repeatedly revisited overlapping dependency prefixes and built successive candidate maps. That is avoidable compiler work, not a necessary cost of the transformation.

## Preparation correction and affected rechecks

The [registered attribution](../registrations/S06-prefix-analysis-attribution.md) replaces repeated DFS with memoized dependency bounds. Each candidate's dependencies are visited once; the largest closed acyclic prefix is still selected. Freshening, expansion, entry generation and target execution remain unchanged.

An independent bounded graph checker agrees on 125 dependency graphs covering calls, foreign residuals and cycles. The semantic, priority-counterexample, alias and execution-trace gates also pass in default and counter-free builds. Prefix selection is now computed before cloning the selected definitions, rather than cloning each candidate prefix.

The paired repair experiment ran 384 ordinary processes, including warmups, and 32 allocation processes at length32. A further registered short-chain recheck ran 256 ordinary and 32 allocation processes at length1. Every answer and repeated allocation check passed.

For the cold deterministic single-call consuming case at length32, median preparation fell from 155 to 53 microseconds in the repair comparison. Total lifecycle median fell from 0.215 to 0.123 milliseconds. Its after/before paired ratio is 0.587; after/sealed is 1.013 with a range crossing one. The original cold-loss conclusion therefore does not apply to the corrected compiler.

The corrected length32 comparison has twelve practical gains and four unresolved cells against sealed execution. Its consuming eight-query cases show:

| Calls and terminal | Corrected lowering lifecycle median | Sealed median | Paired ratio |
|---|---:|---:|---:|
| One, deterministic | 0.277 ms | 0.612 ms | 0.449 |
| Four, deterministic | 0.672 ms | 1.576 ms | 0.424 |
| One, binary choice | 0.356 ms | 0.621 ms | 0.584 |
| Four, binary choice | 1.715 ms | 4.244 ms | 0.409 |

Ratios are medians of pairs, not ratios of marginal medians. At length1 the corrected recheck has six practical losses, ten unresolved cells and no practical gains. For example, eight-query deterministic four-call consumption has ratio 1.625 and meets the loss criterion. Length8 and the lowered-plus-sealed path retain only the initial compiler's full-matrix evidence; they were not remeasured after the analysis change.

## Memory and necessary machinery

Lowering does not uniformly reduce memory. In the corrected length32 deterministic single-call consuming case over eight queries, requested allocation traffic is 550,307 bytes versus746,942 for sealed execution, while measured peak requested growth is 65,018 versus50,696. Code/source representation and analysis can retain more memory even while runtime work falls.

The pass removes private occurrence creation and matching, but keeps unification, choices, resource selection and complete observation. It adds source analysis, capture-free expansion and an entry rule. The current API also repeats target preparation for changed queries. A reusable parameterized artifact could change that tradeoff and remains untested; the present short losses are not impossibility results.

## Disposition and next decision

Retain the optimization opportunity for sufficiently substantive eligible prefixes. Do not require all programs to have private leading prefixes, assume that short queries should be lowered, or infer a global routing policy from these sources. The existing source-order restriction remains supported by its consuming-priority counterexample.

The prepared-artifact question could improve short/repeated cases, but further local tuning would leave actual local pull-tab execution and other distinct organizations untested. The [breadth review](S06-first-breadth-review.md) therefore returns to [local pull-tab source execution](S03-local-pulltab-entry.md), while keeping query-artifact reuse, effectful recursion and broader eligibility open under T073/S06. This changes order, not evidence status.

## Evidence

[Original confirmation analysis](s06-prefix-confirmation/analysis.json), [original freeze](s06-prefix-confirmation/freeze.json), [sizing analysis](s06-prefix-sizing/analysis.json), [repair analysis](s06-prefix-analysis/analysis.json), [short recheck](s06-prefix-analysis/short/analysis.json), and [both compiler/binary identities](s06-prefix-analysis/freeze.json). Raw process records are adjacent.

[Runner](../../../research/chr-compiled/examples/prefix_cost.rs), [sources](../../../research/chr-compiled/examples/support/prefix_source.rs), [compiler](../../../research/chr-compiled/src/pure_prefix.rs), [semantic and graph tests](../../../research/chr-compiled/tests/pure_prefix.rs), [counter-free validation](s06-prefix-analysis/counter-free.log), and [Clippy](s06-prefix-analysis/clippy.log). Eight compiler tests and the runner gate pass; strict scoped Clippy passes. Independent reference behavior was unchanged.

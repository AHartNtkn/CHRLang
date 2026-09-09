# Confirm reusable source-derived preparation costs

The corrected sizing matrix identifies preparation/reuse tradeoffs and noisy individual timings. Confirm total-cost differences with all six controls, including contrary short, wide and failed queries. This is the third package since the latest breadth review; it cannot settle broader lowering or architecture selection.

H1: retained artifacts can reduce total repeated-query costs compared with per-query lowering. H2: ordinary/specialized execution can remain stronger for short or one-shot work. H3: the retention benefit must be distinguished from parameterized execution itself. Native compilation, unplanned signatures and sustained lifetimes are outside this bounded comparison.

## Fixed configurations and controls

Cross seven families (plain1, plain4, plain16, choice1, choice4, fail1, fail4), source depths 1/32, query counts 1/8 and consuming suffix absent/present: **56 source configurations**. Use original, sealed, lowered, lowered-sealed, reusable and rebuilt modes, all with Global+Scan. The exact sources, complete-answer checks, two alternating query signatures and phase ownership are unchanged from the corrected sizing registration.

Reuse the corrected ordinary and allocation binaries under `/tmp/chr-reusable-prefix-depth-8cedb32d/`. Verify current source/manifests and binary hashes against that freeze; freeze the new driver, analysis and registration before timing. One query prepares one reusable signature; eight queries prepare both known alternating signatures. Arguments change and each query owns its execution state. Rebuilt mode intentionally retains no target artifact between queries.

Run seven ordinary repetitions, shuffling source configurations and then modes within each block with seed 7411: **2,352 timings**. Run all 336 cells twice with the meter: **672 allocation runs**, requiring exact per-phase replay. Repeat six-mode choice4/depth8/two-query/resource cancellation at one tick, both allocators: **12 cancellation runs**. Total **3,036**, plus meter self-check. No warmup or post-hoc outlier exclusion.

## Controls, endpoints and interpretation

Each process uses the first available CPU, 60 seconds wall/CPU, 1 GiB address space and two-million-step scalar/source limits. All complete answers are independently checked outside primary intervals. All query/prepared requested-live bytes must return to their starting values. Ordinary timing disables counters and uses the ordinary allocator; separate meter runs measure requested heap allocations, not RSS. Retain and investigate failures and cutoffs.

Primary time includes preparation, setup, complete execution/observation and engine/answer/prepared disposal. Source/input-inclusive time, first observation, phases, requested allocation and peak growth are separate reported endpoints. Native compilation is excluded.

In this exact order compare reusable/original, reusable/sealed, reusable/lowered, reusable/lowered-sealed, reusable/rebuilt, rebuilt/lowered. For each configuration compute the geometric mean of seven same-block primary ratios and a pointwise 95% percentile bootstrap interval from 10,000 resamples, seed `7412 + 6 * configuration_index + contrast_index`, sorted bounds at indices 249 and 9749. Upper bound below 0.90 is a practical numerator gain; lower bound above 1.10 is a practical loss; otherwise unresolved. These descriptive counts are not simultaneous or population-wide evidence and imply no workload weights.

Carry favorable and adverse results forward. Investigate consequential uncertainty or implementation costs if they could change the bounded decision. Do not continue local precision automatically: after confirmation, compare broader recursive/effectful lowering and unplanned-signature lifetime against the strongest ready integration or observation investigation. All remain required regardless of priority.

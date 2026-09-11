# Mixed-source full lifecycle pilot

T078 compares actual original/counted paths after the mixed-source work gate. Counting removes common work otherwise shared by conditional execution, so complete time and ownership can change which architecture is credible. This takes priority over another local repair because the intended mechanisms and independent observations now qualify.

## Hypotheses and comparisons

H1: counting's avoided traversal repays source inference, query rewriting and preparation on substantive sources; depth-zero cases expose overhead. H2: conditional or demand execution remains competitive after the explicit controls also receive counting. H3: immediate/window/all consumers and changed queries expose different traffic and retention despite identical complete answers. Do not combine these into a workload-weighted score.

Use 14 modes: birth, birth-miss, birth-miss-template, conditional, scan, indexed, active-scan, active-indexed, sealed-scan, sealed-indexed, native-scan, native-indexed, active-native-scan, active-native-indexed. Native here means actual generated access execution in the Rust binary. Global and Active are distinct policies; the gate validates their same complete observations on these sources. The generated source is checked against runtime source preparation.

Eight source shapes: common choices0/depth0, common choices0/depth16, and each of common/independent/early with choices3 and depth0/16. Cross history false/true, arrival original/reversed, immediate/window/all consumers, original/counted query execution. This gives 96 scenarios and 2,688 mode/count cells. Each session reuses preparation for four alternating query identities.

## Exact runs and bounds

One excluded ordinary warmup and five ordinary primary repetitions per cell, plus two separately built allocation repetitions: 21,504 processes. Add one ordinary and one metered cancellation process per cell: 5,376 processes. Total 26,880 lifecycle processes. Cancellation stops the first and third queries after one service operation; the other queries complete with the same preparation. Register five-sample medians, ranges and paired ratios as exploratory only; a subsequent targeted confirmation must be registered separately if uncertainty affects selection.

Use deterministic shuffle seed 781041. Shuffle scenario order within each block and all mode/count configurations within each scenario. Pair comparisons by scenario and repetition. Exclude no successful observations as outliers. Preserve failures and stop the campaign for diagnosis. No automatic retries. Pin child processes to logical CPU0, bound each to 60 seconds wall/CPU and 1 GiB address space, and cap the campaign at 20 minutes. Bound complete service at 2,000,000 operations. Build and qualify before freezing; keep ordinary and metered artifacts separate.

## Full accounting and independent checks

Measure 36 disjoint intervals: source construction, counting inference, static-propagation inference, value-choice preparation, engine preparation and temporary source disposal; for each of four queries, input construction, counting rewrite, static initialization, setup, execution with owned observation, engine disposal and consumer actions; finally prepared and consumer disposal. Empty optional passes retain identical clock boundaries. Pipeline preparation owns all required source artifacts until disposal. Preserve all phases; count source transformation copies and disposal rather than subtracting them.

Independent scalar and analytical complete answers are constructed before measured intervals. Validate returned alternatives, multiplicity, unused fuel and fresh aliases outside intervals, including after preparation disposal. Immediate/window/all retention is across completed queries; it is not streaming within a query. Final requested live heap must return to the pre-source baseline. Allocation repeats must match in every phase after baseline normalization. Allocation is requested heap, not RSS. Kernel/engine/work counters and allocation profiling are off in primary timing; allocation uses a separate meter build.

Run ordinary empty-interval calibration with 100,000 readings. Mark a total as signal-limited if below 100 times the median interval overhead times 36. Timings are complete measured phase sums, not process latency or isolated first-answer latency. Native compilation, process startup, validation, measurement buffers and sustained lifetime remain outside this experiment. Do not claim complete architectural lifecycle superiority from this pilot.

At the result, compare further confirmation/causal investigation with broader integration, recognition, restoration and sustained ownership. The goal remains active.

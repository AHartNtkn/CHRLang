# Primary lifecycle sizing for demand execution with nonground posts

Measure the missing complete-time comparison for the qualified passive-post capability. The [ownership](../results/S03-post-ownership.md) and [copy attribution](../results/S03-candidate-copy.md) establish allocation costs and limited sensitivity bounds, not speed. The full prefix portfolio review selects T071; compact runtime confirmation is the strongest ready distinct alternative. This is the first package after that review.

## Existing implementation and eligibility

Reuse the exact ordinary binary in `s03-post-ownership/freeze.json`, verifying source archive and binary hashes. Its build artifacts show no metrics, kernel-metrics, work-diagnostics, candidate-profile or allocation-meter features. Current compiled core, demand templates and post-source definitions match the frozen sources. Current runner/demand changes add only opt-in candidate-profiling hooks, absent in the reused binary. This measures that qualified implementation/toolchain, not a fresh rebuild or compilation economics.

Execution and owned observation are inseparable in the existing runner. Disable first-answer clocks with `DEPENDENCY_FIRST_CLOCK=off` for every process; report combined execution/observation and do not claim separately measured delivery latency. The existing clock-check command has been verified before registration; its preliminary calibration is not part of the primary analysis.

## Hypotheses and exact matrix

H1: Successful/miss/template reuse can lower complete time despite higher requested heap. H2: Allocation-dominant copies may or may not dominate elapsed time; a zero-copy byte bound cannot decide the runtime comparison. H3: Arrival order and retained consumers can change which executor is useful. Keep these conditional and do not invent workload weights.

Six source families: `post-input`, `post-output`, `post-forward`, `post-miss`, `post-duplicate`, `post-template`. Sizes8/32; original/reversed arrival; immediate/window/all consumers. Six modes: `birth`, `birth-miss`, `birth-miss-template`, `scan`, `indexed`, `sealed` (the existing inferred-specialized explicit control). Four changing a/b queries share preparation per process. This gives432 complete lifecycle cells.

Use five fixed exploratory timing repetitions per cell:2,160 primary processes. Create360 matched scenario/repetition blocks with seed71310, randomizing all blocks and mode order within each. Run one excluded warmup process per cell in a separately randomized order with seed71311 before the primary blocks. Warmup is process-level; primary processes still include their own source construction, preparation and first use. Do not call them warmed persistent engines.

Before those runs, execute five pinned100,000-interval clock calibrations and36 cancellation/reuse checks: each family/mode at size8, original arrival, window consumer, alternating cancellation. These are gates, not comparative timings. Every process independently checks full raw answers against scalar execution and analytical post expectations, including retained answers after prepared disposal. Cancellation gates must match the earlier complete/partial endpoints exactly.

## Boundaries, freeze and resource bounds

Count source construction, preparation, query input, setup, combined execution/owned observation, engine disposal, consumer handling, prepared disposal and final consumer disposal. All four queries are complete in primary cells. Independent validation, harness vectors, process startup/transport and report serialization are outside totals. Native compilation, RSS and sustained streams remain unmeasured. Existing allocation/peak records apply to the same frozen endpoint and implementation; do not run another allocation matrix merely to repeat them.

Freeze registration, driver/auditor text and hashes, exact cells, warmup/primary order, parent source archive/freeze/build-artifact hashes and binary hash before comparative runs. Record platform and permitted affinity. Require CPU0 still permitted and pin all children there. Each process has60-second wall/CPU and1-GiB virtual-memory bounds; the binary retains2,000,000 service ticks. The campaign has30 minutes. Preserve and diagnose cutoffs or failures, without treating partial work as completed cost. No concurrent assistant-initiated build or benchmark.

## Analysis and interpretation

Reconstruct every phase, full endpoint, source parameters and paired block from raw data. Require exact endpoint/tick agreement with the parent for each complete cell and each cancellation gate. Report medians and full ranges for complete totals and phases, plus paired demand/control ratios for all three demand policies against all three explicit controls. Do not select only the best control after seeing the data.

Use the practical10% difference to identify consequential follow-up, not as a significance test on five observations. If any total in a cell falls below100×maximum median empty-clock cost×its24 measured phases, mark affected comparisons instrumentation-sensitive. Keep every sample. Report the relevant unchanged requested traffic and peak heap next to timing; neither quantity alone determines efficiency.

At a favorable or contrary result, compare phase costs with the existing copy-attribution bounds. Do not infer time attribution from bytes. If a plausible avoidable cost could change the comparison, select a bounded causal repair/profile with measured replacement overhead. If a source capability is unsupported, scope that limitation instead of weakening the oracle. At this package's result or obstruction compare confirmation/repair with compact confirmation, integration and call-reuse costs. Writable heads, dynamic choices, broader discovery and sustained lifetime remain separate required investigations; this pilot cannot close demand or the goal.

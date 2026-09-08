# S01 low-yield subscription follow-up — prospective registration

## Question and controls

Can retaining one useful join repay repeated discovery through many unsuccessful continuations? The [initial pilot](../results/S01-subscription-lifecycle.md) establishes maintenance losses and savings over eager retention, but its selective search is cheap and dense search produces substantial output. This follow-up supplies the missing favorable mechanism before drawing a broader subscription conclusion.

The indexed control now estimates left-first, right-first and endpoint-pair access using current index degrees. Endpoint-pair access probes the exact middle index. Planning scans and key/probe costs are included. All policies preserve the same source observations; no rule, occurrence, matching or consumption contract changes. Include eager, subscribed and compiled Global/Indexed controls. No eligible region exists for the current single-head specializer.

## Source and sizing

Use [subscription_low_yield.rs](../../../research/chr-compiled/experiments/subscription_low_yield.rs). For N in {4,8,16}, create N selected left endpoints and N selected right endpoints. Middle rows connect every selected left to N absent right endpoints and N absent left endpoints to every selected right, plus one real bridge. There are 2N²+1 middle occurrences and exactly one useful data triple. Two query seeds retain distinct inert query markers.

Use R in {2,16,64} pulses in three families: `low-stable` holds one demand for all pulses; `low-reopen` opens/pulses/retires it each round; `low-churn` removes and reinserts the real middle bridge before each pulse while keeping the demand live. Every query yields exactly R receipts and retains all data rows. These cases distinguish repetition, demand lifetime and maintenance while keeping output count and data volume matched.

The largest N16/R64 configuration of each family passes complete-source sizing in all four executors within 60 seconds and 1 GiB. Those gate runs collect no comparative timings. Final timing and allocation binaries must repeat full gates across the matrix after source freeze.

## Work predictions and gate

At each request, both endpoint sets contain N rows; each one-sided middle traversal includes dead continuations. The new control selects endpoint-pair access: N² exact middle probes and 2N planning-row visits. Indexed execution repeats that per pulse. Subscriptions discover once for low-stable and low-churn, and R times for low-reopen. Churn inserts add two subscribed maintenance probes per round; this is separate from discovery. Eager execution uses no request discovery plan.

Final retained triple count is one for eager and zero for indexed/subscribed. Indexed constructs/invalidates none. Eager constructs one triple except churn, which constructs 1+R, and invalidates constructed minus one. Subscribed constructs one for stable, R for reopen, and 1+R for churn; all are invalidated by final retirement. Work diagnostics must replay exactly in two independent runs and satisfy these counts.

Retain the independent Cartesian-product update gate, complete source tests, alias/resource faults, changed-query/cancellation checks and original selective-endpoint test. A probe count alone does not prove semantic correspondence.

## Matrix and attribution controls

The primary follow-up has 27 low-yield sources × four executors = 108 cells. Carry the initial `selective`, `dense` and `consuming` N4/R8 sources through all four revised executors (12 cells). Also run the frozen initial indexed binary on those three identical sources (three cells). This supplies contemporaneous paired evidence for the access-plan change on earlier regimes; do not infer improvement from cross-run median comparisons.

Total: **123 cells and 861 comparative processes**, with five timing and two allocation repetitions. All cells reuse preparation for two changed queries. Use randomized blocks with seed20260916. The old indexed controls use their original frozen timing/allocation binaries and source commit; the new binaries have a separate freeze and output directory. Preserve both.

## Measurement and interpretation

Use the initial pilot's phase boundaries: preparation; query cloning/setup, execution, observation, engine/answer disposal for two queries; prepared disposal. The primary total includes all these phases. Record cold latency through first complete observation. Separately advance at most 32 executor service units before disposal; these units differ across engines and do not support equal-work cancellation rankings.

Two warmup queries precede measured preparation. Timing uses counter-free release binaries and the ordinary allocator. Work and requested-heap diagnostics use separate builds/runs. Require independent full answers, exact allocation replay, query/cancellation/prepared live-heap restoration, source/binary hashes, toolchain, CPU affinity, commands and terminal receipts. Requested bytes and peak live heap are not RSS. Native compilation, startup, source generation and oracle validation are excluded.

Run serially on the lowest available CPU. Per runtime process: 60 seconds wall, 1 GiB address space, two million executor service steps. Build commands: 120 seconds. Stop on any cutoff for diagnosis before changing bounds or sources. Completed receipts are preserved; do not restart a live process.

Report all paired ratios and ranges. A practical runtime difference requires at least 20% median difference from one and every pair on the same side. Five repetitions are bounded pilot evidence, not confidence intervals. No pooled weights or universal policy.

A stable low-yield benefit would establish an economic opportunity for demand retention against competent rediscovery; retain reopening, churn, setup, memory and prior consumption countercases. A loss requires phase/work attribution before bounding the mechanism. A consequential regression on the earlier sources requires investigation before treating the access plan as generally preferable. In either case, broader active-demand populations and intermediate stages remain unresolved. Reassess further refinement against reusable parallel workers and corrected restoration at the next checkpoint.

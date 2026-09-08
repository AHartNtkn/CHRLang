# R08 regional lifecycle pilot

Prospective registration under T043. [Readiness](../results/R08-regional-readiness.md) establishes diagnostic separation, protocol ownership and the specialized serial finite gate. No new timing is used to select these configurations.

## Decision and competing explanations

Determine whether permanent certified regions earn complete cold-lifecycle savings with two workers, including construction, transport, unique full observation, join and release. E16 supplies a coarse-service sizing signal, not a repeated ranking. The competing explanations are useful concurrent source work; service/worker overhead overwhelming that work; owner-side publication dominating; or a stronger serial execution organization erasing the apparent benefit.

Predict two-worker benefit against same-quantum Inline/one-worker for balanced carry if capacity pays. Predict poorer fine-quantum and one-region results, with imbalance and product/duplicate controls exposing limited parallel work or owner costs. Compare inferred-specialized whole-source execution independently: it may win through lower source overhead or lose by repeating work across explicit alternatives. These mechanisms must not be conflated with transport speedup.

The combined addition/type-inference fixture supplies a concrete certified application witness. It does not establish prevalence. No workload weights, universal ranking, connected-region generalization, native compilation amortization, warm worker pools or stronger serial factorized executor claim follows.

This is preferred to static-lowering eligibility now because a bounded gated interface and current stronger serial control can resolve the coarse-region question without a new executor. If gains are confined to synthetic carry or disappear against specialized serial, record that boundary and reconsider static eligibility; do not enlarge work until workers win.

## Frozen source and cells

Use the existing `examples/support/region_cost_cases.rs` fixtures unchanged. Eight finite cases: `one-zero`, `one-work`, `two-work`, `owner-product`, `asym-first`, `asym-last`, `duplicate-eight`, `mixed-add-infer`.

- Every finite case: Inline, Threads1 and Threads2 at Q64/K4: 24 cells.
- One-work and two-work additionally: those three modes at Q1/K4: 6 cells.
- Every finite case: Specialized whole-source, inferred specialization, Global policy, Indexed access, Q0/K0: 8 cells.
- Stream-prefix and refute-loop: Inline, Threads1 and Threads2 at Q1/K4 and Q8/K4: 12 cells.

Total 50 cells. Regional Q is a bound on atomic source steps per request, not bounded wall time. K bounds outstanding requests. Specialized is a complete alternative, not a matched-microstep control. It must charge full deduplication, raw multiplicity and residual observation. Prefix/refutation do not receive a specialized timing cell: their question is accepted regional progress and cancellation under matched scheduling.

## Measurements and validity

Fresh processes use `region_lifecycle` (ordinary allocator) or `region_lifecycle_memory` (requested-heap meter). Primary uses `--no-default-features --features lifecycle`; work uses default metrics plus lifecycle; allocation uses counter-free lifecycle with the metered example. Build in separate target directories and assert factor/persistent/kernel/observer/compiled diagnostic availability in every row. Do not infer feature configuration from command text alone.

Primary lifecycle is contiguous source clone/construction through search, shutdown and engine/prepared disposal, plus separately timed output disposal after validation. No diagnostic snapshot interrupts live workers in primary runs. Regional construction combines preparation/query setup/worker creation; report that boundary explicitly. Specialized includes rules preparation and query setup. First service starts at construction and ends at the first unique full answer. Native compilation, process startup and source fixture generation are outside this interval. Validation duration is reported separately, and its effect on output-release cache state remains a limitation.

Validate full finite residual/output membership, exact unique coverage, raw multiplicity and exhaustion against independently constructed expectations outside the live lifecycle. Validate prefix membership, uniqueness and registered endpoint separately. Preserve cutoff status; it is not exhaustion. Work snapshots occur only after shutdown in diagnostic runs; actual work includes unaccepted replies. At matched Q, accepted source work and ordered answer contracts are already gated. Physical speculative work can vary with timing. Allocation runs report the final requested-live delta after query, workers and outputs are disposed. A separate standard-library-only blocking-channel probe establishes 48 bytes retained by the receiving thread until that thread exits, with no additional retention on a repeated receive. On this toolchain, Inline/Specialized should restore baseline; threaded runs may restore baseline or retain those 48 bytes depending on whether receiving blocks. Report every delta and investigate any other value; do not prewarm or suppress retention. The retained infrastructure is charged as live requested heap; owner-thread/process teardown is outside the query interval. Thus this is cold query lifecycle evidence, not complete process-lifetime superiority. Requested bytes are not RSS.

## Order, resources and interpretation

One warmup, five primary repetitions, one allocation and one work run per cell: 400 processes. Seed 43044 independently shuffles the warmup block, each primary repetition block and each diagnostic block. Freeze the full order before execution. Run serially with no concurrent builds or experiments.

Pin each child and its inherited threads to CPUs 0,2,4, verified allowed and on distinct package/core identities before launch. Record topology, actual allowed affinity, CPU description and accessible cgroup limits. The owner and workers share this set; individual threads are not assigned dedicated cores. System contention, turbo and migration within the set remain limitations. Inspect ancestor cgroup limits and abort if an exposed quota permits fewer than three CPUs. The current accessible ancestor reports `max 100000`. Abort before measurements if this verified capacity is unavailable; do not substitute a single CPU silently.

Bounds: 30 seconds and 1 GiB address space per process; 20 minutes for the batch after builds. Regional owner turns use each fixture's declared budget; Specialized permits 20 million ticks. Record all timeouts, cutoffs, nonzero exits and unattempted cells. A semantic/configuration error stops the batch for investigation. Resource failures remain explicit and do not become favorable completion timings.

Report all five primary observations, medians and min/max ranges, with phases, first service, allocation and work separately. Separated observed ranges support a bounded ordering; overlapping ranges remain unresolved with five repetitions. Incomplete repetition groups cannot support a complete comparison. Do not pool warmup/diagnostic timings or infer a crossover outside measured points. Compare Threads2 with same-Q Inline and Threads1 for transport/capacity, and compare each complete alternative with Specialized separately. Register any consequential rerun or changed configuration before execution.

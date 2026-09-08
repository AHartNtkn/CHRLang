# S09 cold/reuse worker timing pilot

This prospective pilot tests whether reusable workers repay coordination under the same persistent source service and certified product coordinator. The source-correctness and allocation gates are recorded in [regional correspondence](../results/S09-regional-source-gate.md), [concurrent accounting](../results/S09-concurrent-meter.md) and [native sizing](../results/S09-native-sizing.md).

## Hypotheses and decisions

- **H1: granularity.** Larger balanced regional work and a coarser source quantum can produce a practical whole-runtime wall-time gain over inline execution. Contrary outcome: coordination, setup and ownership still outweigh useful parallel service.
- **H2: reuse.** Sixteen changed queries per prepared pool amortize creation sufficiently to change some cold-versus-inline comparisons. Contrary outcome: per-query costs dominate even after preparation is reused.
- **H3: adverse controls.** Skewed work and one tiny region weaken or reverse a worker gain. A result only on balanced sources supports a bounded parallel organization, not general adoption.
- **H4: total resources.** A wall-time gain may require more process CPU. Report that tradeoff, not a universal efficiency winner.

## Frozen matrix and execution

Source families: balanced at base depths64/256; skewed at base depths64/256; tiny at depth0. Balanced/skewed have four independent countdown predicates, each ending in two distinct answers, yielding sixteen complete products. Tiny has one region and two products. Skew multiplies depth by region index plus one. Within a session, base depth increases by query-index modulo three. Controls use identical source, certification, service and product policies.

Cross five source cells with query counts1/16, source quanta16/128, and inline or 1/2/4 workers: **80 configurations**. The coordinator outstanding window is4. Run one discarded process warmup per configuration, then seven fresh-process repetitions of every configuration: **640 total processes, 560 measured**. Randomize configuration order within every block using Python `random.Random(20260908)`. Warmup warms the executable/system; every measured process creates a fresh runtime. Reuse occurs within the sixteen-query processes.

Use the existing counter-free, ordinary-allocator release `worker_cost` executable. Build with `cargo build -p chr-factors --release --no-default-features --example worker_cost`; freeze its hash, relevant source files, Cargo files, Rust toolchain and current hardware/affinity before the first warmup. Do not pin individual worker threads or change host scheduling. Record observed affinity and quota information; missing quota data is not evidence of unlimited resources.

Each process is limited to60 seconds and1 GiB address space; each query is additionally bounded at10,000,000 coordinator advances. Any cutoff/assertion invalidates that cell's ranking and triggers diagnosis. Save all outputs and unsuccessful runs. Resume completed receipts only against the same freeze; never restart a live run.

## Endpoints and validation

Primary endpoint: summed runtime lifecycle wall time, including source construction/preparation, query construction/certificate/setup, joint execution and observation, close/drain, query/answer disposal, shutdown and runtime disposal. Full source execution and product observation cannot currently be isolated; report their joint interval.

Secondary endpoints: prepare/setup/close/disposal attribution; first complete observation relative to query execution start; per-query wall total and mean of queries1–15 in reused sessions; process CPU usage. CPU includes startup, independent answer validation and output serialization, so it has broader scope than the summed wall intervals. Neither endpoint includes compilation of an independent user program. Expected-answer construction and harness storage are outside primary wall timing. No memory ranking follows from this timing-only matrix; allocation diagnostics remain separate evidence and can motivate a matched follow-up.

Every process validates all complete output tuples, residuals and raw multiplicity against the hand-derived products outside primary wall intervals. It must return exactly the declared query count and consistent nonnegative phase totals. Assert counters disabled in the executable. Retain the reference interpreter unchanged.

## Interpretation fixed before runs

For each source/reuse/quantum cell, pair each worker's observation with inline from the same repetition block. Report all seven ratios, their median and extrema, plus raw phase ranges. A **practical wall benefit** requires median worker/inline lifecycle ratio <=0.90 and every paired ratio <1. A **practical wall loss** requires median >=1.10 and every paired ratio >1. Otherwise classify the cell as unresolved at this pilot's precision. This is a practical screening rule, not a confidence interval. Apply the same descriptive rule separately to CPU and first-observation endpoints; do not combine endpoints into a score.

Also compare multiple workers with one worker to separate usable parallel capacity from beating the inline control. Examine query-count and quantum contrasts directly, without selecting a best policy from these data and claiming held-out confirmation. Warm-query means exclude query0 and must remain separate from full lifecycle totals.

If plausible remaining variance spans a consequential architecture choice, perform a prospectively registered follow-up. Attribute favorable or adverse phases to unavoidable obligations, repairable costs or explicit coordinator policies. Tiny or skewed losses cannot reject balanced parallelism; a balanced win cannot settle connected work, output-heavy streams or another representation. Completing this matrix does not complete T069's broader evidence obligations or the architecture goal.

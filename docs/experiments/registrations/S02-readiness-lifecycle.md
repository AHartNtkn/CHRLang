# Readiness lifecycle sizing

## Decision and hypotheses

This exploratory pilot determines whether matcher-relevant settlement's qualified early-failure saving survives preparation, query admission, answer production and disposal, and whether bounded draining materially repairs successful-source costs. It compares existing schedules in the relational engine and the existing generic compiled Global/Indexed engine as an independent execution competitor. No new baseline or language default is introduced. All completed observations must match the independent scalar evaluator.

H1: separate output plus early source failure can favor selective readiness over full settlement after lifecycle accounting. H2: successful output and shared matcher/output dependencies expose readiness maintenance costs; batching can repair repeated advances without removing those costs. H3: preparation reuse changes amortization, but does not imply a fixed workload distribution. H4: generic compiled execution can outweigh the integration benefit on these sources. Opposing outcomes remain separate regimes.

## Exact configurations

Use the qualified `tests/support/readiness_source.rs` separated source with the earlier guard's prefix set to impossible (`possible=false`). Cross base depth 4/64, shared dependency false/true, outcomes success/fail/clash/cancel, and 1/16 queries per preparation. Query i has depth `base + i % 2` and `1 + i % 2` token occurrences; the same prepared rules handle every query. Cancellation uses the success source and drops the query engine immediately after admission, without execution. It measures admission cancellation, not equal amounts of engine-internal progress.

Five configurations: full equality settlement before every source advance; selective matcher settlement without output batching; bounded draining with deduction budget 8; bounded draining with budget 256; existing generic compiled execution using Global rule policy and Indexed access, without generated code. All use the same exact sources. Source construction and independent scalar validation occur before measured phases; source-specific optimization is not added for this pilot. No generated-compilation cost claim follows.

160 cells: 2 depths × 2 dependency patterns × 4 outcomes × 2 reuse counts × 5 configurations. Run five ordinary allocator release processes and two requested-allocation release processes per cell: 1,120 total samples. First perform entry validation at depth 4, both dependency patterns, all outcomes and all configurations, with 2 changing queries. These entry observations qualify the harness, not comparative timing evidence. A separate allocator self-check must pass.

## Measurement boundaries

Measure rule preparation; each query's setup; execution through complete first observation (or failure/exhaustion); engine disposal; preparation disposal; retained consumer-answer disposal. These fixtures yield at most one answer, so first and complete observation coincide. Relational answer production occurs inside advance; report execution and observation jointly for every engine. Never label this as separately isolated observation cost. Keep all answers through preparation disposal, then validate complete outputs/residuals outside intervals and dispose of the consumers in a measured phase. Requested live bytes must return exactly to the pre-preparation baseline in every allocation process.

The phase sum is the primary session lifecycle endpoint. Retained answer memory and peak requested bytes are measured from the phase windows; input fixtures, oracle results and preallocated receipt slots are resident before the baseline. These are requested heap sizes, not RSS. Primary timing uses ordinary allocation with engine/kernel diagnostic features off; allocation process times are not primary timing. The harness maintains its mandatory 100,000-advance cutoff; no per-engine diagnostic counters are enabled. Rule source construction, external compilation and process startup are outside the session endpoint and cannot support complete architectural lifecycle superiority.

## Ordering, bounds and interpretation

Build separate ordinary and alloc-meter release artifacts, freeze sources, registration, toolchain, build commands and binary hashes after entry validation. Each process is CPU-pinned to an available registered CPU, limited to 60 seconds CPU/wall and 1 GiB address space. Run serially. Use seeded shuffled complete blocks (seed 7204), five ordinary blocks then two allocation blocks. Before samples, one unrecorded ordinary run per configuration at depth 4, separate success, one query warms code/filesystem startup; each sample is a fresh process. No within-process workload warmup is concealed in its lifecycle.

Require exact allocation phase/advance repeats after subtracting each process's resident live baseline. Preserve every raw exit and cutoff. Inspect any mismatch or bound before interpreting a cell. A bound does not reject a design.

Report medians and observed ranges of paired total lifecycle ratios. Use a provisional 10% practical difference only when all five paired ratios are on the same side of 1 and median total duration exceeds 20 times a 1,000-sample empty-phase clock-floor estimate. Otherwise label the comparison unresolved for timing and diagnose whether repetition, batching of complete sessions or a source extension could change the decision. This is sizing, not confirmatory statistical inference. Investigate consequential avoidable costs before freezing a follow-up comparison. No averaging across outcome classes or invented workload weights.

After the pilot, conduct the required full portfolio review (package four since the last review), comparing the value of further readiness optimization, partner plans, graph memo/lifecycle costs and every other active unresolved direction. Keep T072 and the overall goal unfinished unless their actual requirements are met.

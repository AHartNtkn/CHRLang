# R03 conditional lifecycle pilot registration

Prospective registration before comparative invocations. T035 selects total conditional execution cost after its independent runtime gate; further explicit-search queue tuning is the strongest ready alternative, but the existing scheduling diagnosis already resolves its main causal issue. This comparison can decide whether direct supported work earns its discovery, completion and retention machinery.

## Hypotheses and controls

H1: direct conditional execution may amortize opaque work across explicit choice histories enough to offset support bookkeeping. H2: ordinary no-OR and immediate structural discrimination may expose overhead without useful shared work. H3: output enumeration/retention can dominate even when physical source applications decrease. H4: reusable lowered plans can change the cold/repeated-query contrast; both implementations must actually share immutable lowered plans across starts.

Compare `conditional` with `explicit`: the latter is the generic, global-rule-order, indexed CHR execution control, with explicit FIFO source search. It uses `code: None`; this is not a generated-dispatch comparison or a universal best-engine claim. Both consume exactly the same source rules and queries. The runtime finite source gate and runner's independent scalar oracle check complete answers and raw multiplicity. Language semantics remain nonbinding heads/guards, finite trees, multiset resources and explicit OR. No inferred workload weights.

## Exact matrix

The executable workload definitions are `research/chr-direct-conditional/experiments/main.rs`.

- `plain`, base sizes 16 and 64: unary countdown through an opaque four-variable payload, no OR, one nonground full answer.
- `shared`, 16 and 64: four independent binary atom choices followed by the same opaque countdown; 16 complete raw answers.
- `discriminate`, 16 and 64: four independent choices, then one of 16 fully structural gate rules before countdown; 16 answers. This is an adverse whole-workload contrast, not a single-operation ablation.
- `output`, depths 4 and 6: recursive binary list construction; 2^depth full answers.

Each backend/family/base-size runs with one query or four changing queries through one prepared owner. Query i has size base+(i mod 2); all answers remain retained until the batch's engines and prepared handle are disposed. Cold refers to a fresh native process and one preparation/query, not a cold CPU/filesystem cache. Rule/query syntax construction is harness input generation and is excluded; the source clone consumed by preparation is included identically.

There are 32 cells. Five primary repetitions per cell, one untallied warmup process per cell, and one allocation plus one work-diagnostic process per cell: 256 total process invocations. Primary matrix order uses Python Random(35036), warmups first, then shuffled repetitions; diagnostics follow in shuffled order. These are exploratory pilot repetitions, not a confirmatory confidence claim. No favorable samples or cutoffs may be dropped.

## Measurement boundaries

Native release binaries use separate target directories. Primary: `--no-default-features --features experiment`, ordinary allocator, conditional and compiled/kernel counters disabled. Allocation: `--no-default-features --features alloc-meter`, requested-heap meter only. Work: `--features experiment`, counters enabled, ordinary allocator. Exact build commands, source hashes, binary hashes, toolchain/host/CPU affinity and process outcomes are retained by the driver.

Measure rule preparation, each query setup, combined execution/full observation through exhaustion, first-answer latency from service start, each engine disposal, and final prepared-owner disposal. The conditional service interleaves execution and export, so separate execution-versus-observation attribution is not available here. The explicit path also includes terminal branch observation/disposal in service. All output vectors are retained equally through the batch. Backend lifecycle per query is (preparation + all setups/services/engine disposals + prepared disposal)/query count.

Only after every backend interval completes, independently construct expected answers and compare full observations. Measure answer disposal after validation separately. Also report lifecycle including this disposal, but flag its warmed-by-validation boundary; a material result depending on that phase requires a cleaner follow-up before an architectural claim. Do not describe backend time alone as complete application lifecycle. Oracle work, report formatting and driver startup are not native backend intervals. No RSS inference: the process later runs an oracle and its process high-water would include that work.

Allocation diagnostics report requested bytes/calls, phase live start/end and peak live. These are not RSS. Retained source/earlier answers/harness baselines must be distinguished from phase growth; cross-phase maxima must not be added. Work diagnostics report physical applications, physical OR operations and candidates (conditional discovered tuples versus explicit candidate visits, explicitly different mechanisms). Diagnostic timings do not enter primary timing medians.

## Bounds and interpretation

Each process has 30 seconds wall time, 1 GiB address-space limit and 20 million service steps/query; oracle has 2 million scalar steps. Driver stops after 30 minutes total, preserving completed results and every failure. Run serially, pinned to the first available CPU in the driver's inherited affinity. No live comparative job may be restarted on an observation timeout. Meter self-check must pass before allocation cells.

Report all cell medians/ranges and per-query lifecycle, first answer, preparation, disposal, answer count, requested heap and work. A ratio is descriptive within this matched workload/configuration only. Savings must survive charging all recorded backend phases and checking answer-disposal sensitivity. If source sharing occurs without total savings, inspect support/discovery/observation/retention costs before proposing another implementation. If no-OR dominates the cost difference, avoid generalizing shared-work results to ordinary computation. If ranges overlap materially or sample variability can reverse a conclusion, freeze a targeted follow-up with more repetitions. If ordinary or discrimination overhead is severe, assess whether necessary machinery or a removable implementation issue causes it; do not tune merely to win. Any semantic failure invalidates the affected comparison until its root cause is fixed and source freeze renewed.

Preparation reuse has an explicit ownership test. Native compilation cost, broad application generalization, long-lived reclamation, generated dispatch and parallelism remain unresolved. This pilot cannot establish complete architectural lifecycle superiority or an overall language winner.

# S04 restoration lifecycle pilot — prospective comparison

This pilot asks when preserving state through copying, reversible changes or replay earns its costs under fair source search. The source/restoration gate already passes; the new workload gate must pass before timing. No S04 comparative measurements precede this registration.

## Hypotheses and decisions

- **H1:** Large mostly unchanged state can make copying expensive enough for trails or replay to win complete lifecycle cost. Compare small/retained, then mutation as counterpressure.
- **H2:** Root replay can lose through repeated prefixes; periodic checkpoints can recover enough work to change that result. Compare root replay with checkpoint intervals 1, 4 and 16, including depth and work sweeps.
- **H3:** Broad frontiers and dense mutation can make restoring between retained paths costlier than preserving states. Compare balanced/spine search at the same maximum depth, and retained/mutation at the same initial state size.
- **H4:** Failure placement changes the cost of ownership policies. Early/late rejection must preserve the same successful observations and raw multiplicity.
- **H5:** A within-representation advantage may disappear against the existing indexed engine. Include ordinary and COW indexed execution and preparation reuse; no new production baseline or universal default follows.

These are competing possibilities. A loss is not an intrinsic architectural cost until the responsible implementation work has been inspected. The strongest ready alternative is S05 stable-identity equation/failure reuse; this bounded pilot comes first because complete restoration candidates now exist and the result can change ownership across multiple search regimes.

## Exact sources and configurations

The executable source generator is `research/chr-restoration/examples/lifecycle.rs`. Each query begins with N identified cells holding distinct unknowns. A binary choice edits the first M cells, performs W recursive work steps and continues with one fewer depth level. An edit consumes its old cell, binds its unknown to the branch atom and posts a fresh cell. The result retains the full choice path and query label. All cells and the first original unknown are observed.

The work chain is a deliberately simple source computation, potentially optimizable by a stronger compiler. It varies interpreter work between choices; it is not evidence that such work is intrinsically necessary in the final architecture.

| Family | Initial cells N | Choice depth D | Edited cells M | Work W | Shape |
|---|---:|---:|---:|---:|---|
| linear | 64 | 0 | 0 | 16 | No choice; one work chain |
| small | 8 | 3 | 0 | 2 | Balanced binary tree |
| retained | 128 | 3 | 0 | 2 | Balanced binary tree |
| mutation | 128 | 3 | 32 | 2 | Balanced binary tree |
| work | 8 | 3 | 0 | 32 | Balanced binary tree |
| deep | 8 | 6 | 0 | 2 | Balanced binary tree |
| spine | 8 | 6 | 0 | 2 | Right child terminates; left child recurses |
| early | 64 | 4 | 8 | 16 | Right child fails before edits/work |
| late | 64 | 4 | 8 | 16 | Right child fails after edits/work |

Balanced/spine varies branching shape at fixed maximum depth; small/deep varies depth with balanced shape. Frontier width is an outcome of shape and scheduling, not an independently fixed number. This pilot does not identify every independent width/depth effect.

Compare eight configurations: Copy, Trail, root Replay, Checkpoint1, Checkpoint4, Checkpoint16, existing Global Indexed, and existing Global Indexed with arena COW. Checkpoints are saved after a non-fork progress step at or beyond their interval. Use exactly 1 and 8 complete changing queries per prepared ruleset; query labels and initial variable identities change by query index. The COW feature is built separately; only the indexed control uses that build in the matrix.

There are 9 × 8 × 2 = **144 cells**. Two warmup queries precede each measured preparation. Run **five ordinary-allocator timing processes and two allocation-diagnostic processes per cell**, 1,008 measured processes total. This is a bounded pilot, not a held-out confirmation. Shuffle cells within each repetition using seed 20260908. Record the exact order. Use one available CPU selected before the run and record affinity; no concurrent benchmark processes.

## Correctness and accounting

Gate all nine families at query indices 0 and 7 against the independent owned-syntax evaluator, in all seven ordinary configurations. COW uses the same gate in its build. Check analytical raw counts: one for linear/early/late, D+1 for spine, 2^D otherwise. Every measured query validates its complete raw observations against the independent evaluator outside measured intervals.

Measure ruleset preparation, each query's setup, joint execution/observation through exhaustion, engine disposal, answer disposal and prepared-rule disposal. Report first observation separately as elapsed execution time to the first complete answer; add setup/preparation explicitly when discussing cold latency. Do not add first latency to the lifecycle sum.

After the complete-query batch, measure an additional query through its first complete answer, verify that answer against the independent expected set, then cancel and dispose it. Report this cancellation scenario separately from the complete-query batch. Prepared-rule disposal occurs after this additional use and counts once in the batch lifecycle sum; report that placement. Consumers retain all answers until each query ends, then release them. Other output lifetimes remain S08 work.

Primary builds disable engine/kernel metrics and use the ordinary allocator. Allocation builds enable only the existing requested-allocation meter, in separate processes. No timing ranking uses allocation-build times. Report requested traffic and live/peak requested heap separately; do not call them RSS. Check that measured runtime ownership returns to the pre-preparation live allocation level after full disposal. No engine work-count ranking is part of this pilot.

Independent expected answers, fixture construction, input generation, validation and JSON reporting are outside measured phases. Their presence affects the process allocator and is disclosed. The result/phase vectors have their capacity allocated before preparation. Execution includes collecting answer vectors and one first-answer clock read. Compilation, process startup and source generation are excluded; do not claim complete architectural lifecycle superiority including native compilation. Record build commands, toolchain, source hashes and binary hashes.

## Bounds and interpretation

Each process has a 60-second wall limit and 1 GiB address-space limit. Each finite engine must exhaust within 100,000 scheduler advances. Root replay includes prefix work inside an advance; this is not a constant-time quantum. Gate processes use the same limits. Builds have a 120-second process bound and are not measurements. Record every cutoff, failure and raw output; a cutoff is not a completed timing.

Use the sum of the named complete-query phases plus preparation/disposal as the primary endpoint. Summarize medians and per-repetition paired ratios, with min/max paired ratios across the five timing repetitions. A **20% difference** is the initial practical threshold. If paired ratios straddle that boundary, report uncertainty; add prospectively specified repetitions only when precision could change the bounded decision. For small effects without a consequential decision, retain the uncertainty rather than claiming equivalence.

Allocation repetitions must match all phase readings exactly; investigate disagreement. Report a memory/time tradeoff when neither organization dominates both. Do not aggregate families using invented weights. An opposing regime establishes a conditional result, not automatic routing. Profile or instrument a consequential loss before attributing it to necessary complexity. Further checkpoint policies, sustained retention, adaptive splitting and temporary separation/reunion remain required beyond this initial matrix.

## Gate process partition after the allocation gate cutoff

Before any matrix process ran, the combined 126-comparison allocation gate reached its 60-second process limit without a semantic failure report. Preserve that raw cutoff. Execute the unchanged cases one family per process, still at 60 seconds and 1 GiB, to distinguish aggregate gate duration from an individual workload limit. No case, executor, answer check or engine bound is weakened. Investigate any remaining family cutoff before proceeding.

## Root-replay cutoff diagnosis and bounded extension

The family-level allocation gate also reached 60 seconds on mutation. Isolating configurations identifies root replay: all six other ordinary configurations pass that family in their separate allocation-gate processes. A separate work-only build observes 2,404 source steps for Copy/Trail and 2,418 / 6,023 / 20,423 for checkpoint intervals 1 / 4 / 16. A deterministic-prefix projection gives 799,623 root-replay steps for the same complete mutation query. The projection is checked against actual root-replay counts on all eight other workload families. Keep this projected count distinct from an observed completed mutation replay.

Before any matrix timings, extend mutation gate processes to **180 seconds**, and root-replay mutation matrix processes to **600 seconds**. Other matrix cells retain 60 seconds; address-space and engine bounds are unchanged. This charges the expensive candidate rather than shrinking the workload. Quantifying its remaining retention/time tradeoff against checkpointed replay is the reason for the bounded extension; the work counts do not establish a timing ratio. If the extended process still cuts off, retain and investigate it before further repetition.

COW builds gate the indexed configuration used by their matrix cells. Ordinary builds gate all seven ordinary configurations. No unmeasured restoration configuration in a COW build is required to repeat the expensive root-replay gate. The matrix still has all 144 cells and 1,008 processes. The work-only feature must be absent from timing and allocation binaries; the runner refuses timing samples from that feature.

# Size reusable source-derived prefix lifecycle costs

This is exploratory sizing after the reusable-prefix correctness gate. Its purpose is to find consequential preparation, query and retention costs before registering confirmation. No inferential ranking will be made from one timing per cell.

## Questions and controls

H1: retaining source-derived target preparation can repay its initial cost across changing query arguments. H2: short prefixes, one-shot queries, wider entries or quick contradictions can favor ordinary or query-specialized execution. H3: rebuilding a parameterized artifact on each query can separate parameterization overhead from retained-preparation benefit.

Six modes: original compiled Global+Scan; inferred specialization with Scan; per-query lowering with ordinary or specialized target execution; reusable artifacts; and rebuilt parameterized artifacts. All share the same compiled engine and independent scalar oracle. Reusable mode prepares the first ordered predicate signature for one query, or both alternating signatures for multiple queries, during preparation. Those signatures are known by this workload interface. Rebuilt mode deliberately retains no target artifact between queries. Neither policy proves behavior for unseen signatures or adaptive retention.

Seven families: plain1/plain4/plain16 have one/four/sixteen independent private-chain calls; choice1/choice4 branch into f/g at each call; fail1/fail4 supply a clashing ground output to the ordinary private chain. Include consuming suffix absence/presence. Plain/choice complete answers contain all outputs and residuals; failure families have zero answers. Queries alternate changed keys and forward/reverse occurrence order.

Cross depths 1/8/32, query counts 1/8, resource false/true, seven families and six modes: **504 ordinary timing processes** in shuffled order, seed 7401. Run every cell twice with the allocation meter: **1,008 allocation processes**, requiring exact per-phase allocation replay. All six modes additionally run choice4/depth8/two queries/resources with cancellation at one tick on the first query, under both allocators: **12 cancellation processes**. Total **1,524**, plus allocator self-check.

## Lifecycle and bounds

Use separate release builds with default features disabled: ordinary allocator, then alloc-meter enabled. Assert engine/kernel/observer counters disabled. Freeze source, binaries, toolchain, driver and analysis before comparisons. Each process is pinned to the first available CPU, limited to 60 seconds wall/CPU and 1 GiB address space; each source/scalar execution has a two-million-step bound. Preserve failed/cutoff receipts and investigate before continuing.

Primary total: preparation, query setup, complete execution/observation, engine and answer disposal, prepared disposal. Report source/input-inclusive totals separately, plus first answer, phase costs, requested allocation and peak requested growth. Charge signature construction and all retained-artifact construction in preparation. Rebuilt artifact construction/destruction belongs to setup; target rules retained by a live search belong to its lifecycle. Ruleset/source/query disposal must restore allocator live bytes. The meter reports requested heap bytes, not RSS. Native compilation is excluded.

Independent scalar answers are computed before primary intervals and checked afterward. Each query's actual input must equal its prevalidated fixture. Cancellation answers must be a valid prefix, and the next query must complete after interruption. Timing is ordinary allocator only; meter elapsed times are not speed evidence.

## Interpretation and next boundary

Describe all cells without significance labels or aggregated workload weights. Examine reusable versus both per-query lowering and ordinary/specialized execution, then rebuilt versus reusable to locate retained-preparation costs. Investigate consequential overhead before attributing it to the architecture. Wide entries and failed queries are adverse controls, not a comprehensive application distribution.

Choose exact repetitions and practical criteria prospectively for confirmation based on the decision still unresolved after sizing. Broader recursive/effectful lowering, source changes, unplanned signature changes, richer argument structures and sustained artifact lifetime remain required. This is the second package since the latest breadth review, not closure of T073 or S06.

## Reporting correction before repeated sizing

The initial matrix completed all processes and exact allocation replay, but the per-query depth field recorded the alternating key seed. Source depth is fixed throughout a process. Correct that metadata field, rebuild both binaries and repeat the same 1,524-process matrix with the same ordering and bounds in `s06-reusable-prefix-sizing-corrected/`. Retain the initial receipt in `s06-reusable-prefix-sizing/`. The corrected run is the primary sizing receipt; do not combine single timings into an unregistered confirmatory comparison.

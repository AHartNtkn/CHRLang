# R02 integrated cost pilot

Measure whether the integrated value/constructor representation earns its equality, congruence, access and activation machinery in complete ordinary computations. This is an exploratory pilot after the independent semantic gate. It does not choose a universal architecture or impose a new language contract.

## Predictions, controls and selection

The integrated candidate can propagate class/constructor information directly to source access, avoiding dedicated binding dereference and repeated structural key normalization. Conversely, union-find incidence, congruence repair, activation and retention of value/occurrence records can cost more than dedicated terms and bindings. High-degree alias updates with few useful firings expose maintenance cost; repeated deeper constructors expose possible normalization savings. Cheap recursive build is an adverse control with little selective matching to save.

Compare `integrated`, `generic-indexed`, `generated-indexed`, `generated-scan`, and `generated-global-scan`. Dedicated controls use the corrected anchor-aware R01 engine. The first three distinguish integrated interpretation, dedicated generic execution and dedicated native generation on indexed paths. Scanned controls keep indexing and activation from receiving automatic preference on adverse workloads. Distinct permitted application orders are allowed; complete analytic answers govern the comparison. Native compilation is excluded, so generated variants are an execution control, not a complete cold-compilation recommendation.

The gate and implementation are selected over another R01 access-maintenance refinement because they can change the representation shared by equality, matching and source effects. R04 inference-matched solving remains independently eligible; this pilot tests a broader ordinary consuming path rather than a selected finite logical region. Continue into another refinement only if it could materially change the architecture comparison.

## Exact workload registry

Programs and analytic observations are in `research/chr-integrated/experiments/workloads.rs`. Join rules consume `open(K,f(Y)), ticket(K)` to produce `result(K,Y)`, propagate `seen(K,Y)`, and consume bind occurrences to post equations.

- `independent`: n distinct flat keys, n independently bound variables, all matches useful.
- `fanout`: n distinct flat keys sharing one bound variable, all matches useful.
- `repair`: n distinct flat keys sharing one variable, bound to g(...), so no open/ticket join succeeds.
- `build`: recursive list construction with one output and no residuals.

The first three vary n in {8,64} and constructor depth in {1,8}, independently: 12 workload cells. Build uses n in {8,64}, depth argument 1: two cells. Total 14 workloads × five variants = 70 cells. Every process prepares once, then runs four genuinely separate queries at sizes n,n+1,n,n+1. Flat key names have fixed width. Residual multiplicity, full constructor contents and selected outputs are validated against analytic formulas outside measured intervals. Small instances additionally pass independent source replay, terminal enumeration and integrated index/consistency audit.

Five primary timing processes, one work-count process, and one allocation process per cell: 490 processes and 1,960 requested complete answers. Shuffle all jobs with Python `random.Random(20260910)` and save the exact order. No discarded warmup; the process is the replication unit. No adaptive expansion under this registration.

## Build, timing and resource controls

Use installed rustc and ordinary Cargo release defaults. Three package-isolated targets, `cargo build --release -p chr-integrated --target-dir /tmp/chr-r02-{time,work,memory}` with respectively `--no-default-features --features experiment`, `--features experiment`, and `--no-default-features --features alloc-meter`. Copy binaries to `/tmp/chr-r02-bins/{time,work,memory}`. Freeze all relevant source/build/generated files and binary hashes before runs; record resolved dependency features and compiler/environment. Both candidate and dedicated counters must be off for primary timing and memory. Work counts use both on. Allocation meter runs use requested heap bytes, not RSS.

Each process runs `chr-integrated-cost MODE FAMILY SIZE DEPTH 4 VARIANT`. Start from owned rule/query ASTs. Measure preparation; setup/lowering; bounded execution; actual first full observation; engine disposal; answer disposal; prepared disposal. Integrated APIs borrow source syntax, so dispose the owned source rules inside preparation and the owned query inside setup; dedicated APIs consume those inputs. Measure fixture/oracle work separately as harness cost. Query lifecycle is request-through-engine-drop plus answer disposal; four-query lifecycle adds preparation and its disposal. No trace recording, exhaustive audit, second observation or correctness-oracle work belongs in the primary intervals.

Generated preparation includes code/source validation. Bundled Rust compilation is not isolated per ruleset and has no architectural amortization claim. Preparation and queries do not share mutable values, histories or index state. Allocation baselines include prepared state and preallocated report storage; after each answer disposal require stable live requested bytes across all four queries, with preparation disposal separately reported. This establishes bounded no-growth, not zero one-time retention.

Bound each query to 1,000,000 engine steps. Child limits: minimum allowed CPU affinity, 2 GiB address space, 20 CPU seconds, 30 wall seconds, no core dump. Batch bound: 15 minutes. Run children serially and do not build/run other experiments concurrently. Stop and preserve output on child failure or validation/feature/retention disagreement. Preserve timeouts and cutoffs; they are incomplete results, not answers or source refutations.

Require allocator self-check and empty-clock calibration before runs. Interpret no phase direction below 100× median empty-clock cost. Do not subtract fixed overhead. Report per-cell five-process median and range, complete lifecycle and execution; differences below 10% or overlapping ranges are inconclusive timing directions. Work counts are implementation-specific, not an interchangeable scalar cost; allocation requests can substantiate physical differences. Do not average workloads into an architecture score.

## Interpretation boundary

A surviving integrated advantage justifies testing the responsible regime and obligations, not OR/search or general parallel claims. A loss requires distinguishing necessary representation machinery from an evident control defect. The premeasurement review already requires competent anchor-informed access, immediate constructor interning and local merge-cycle checks; no result should be explained away by silently changing the candidate after freeze. Register a separate follow-up if a consequential confound is found.

Full language guard design, native compilation amortization, long-running reclamation, explicit search and parallel service remain open architectural questions. Their dispositions depend on the measured mechanisms, not on whether this pilot finishes.

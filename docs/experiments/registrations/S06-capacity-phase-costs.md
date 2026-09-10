# Counter-free resource-phase composition cost pilot

This pilot tests whether avoiding source execution repays ordered continuation construction and ordinary caller transport. The ownership gate establishes lower requested allocation on selective queries, higher allocation on complete output-heavy queries, and lower live-heap peaks outside zero-request cases. Runtime may agree or disagree with those metrics. This is package three after the matched-reuse breadth review; compare adaptive reunion and the other distinct directions at its boundary.

Use the frozen primary `capacity_phase_lifecycle` binary qualified by S06-capacity-phase-ownership. No code or compiler configuration changes. It uses the ordinary allocator with engine and capacity diagnostics disabled. Compare phase plus specialized scanned caller against whole-source generic Scan/Indexed and inferred-specialized Scan/Indexed.

Six scenarios `(requests, supply, weight, reuse, stop)`:

1. `(0, empty, 1, 1, complete)` — preparation/empty-work overhead.
2. `(2, tight, 1, 1, complete)` — small successful work and allocation overhead.
3. `(4, tight, 1, 4, complete)` — selective successful and aliased-failing queries.
4. `(4, empty, 1, 4, complete)` — early capacity rejection.
5. `(4, spare, 2, 4, complete)` — output-heavy complete answers, an allocation-adverse regime.
6. `(4, spare, 2, 4, first)` — stop query zero after one valid owned answer, then complete three changed queries.

Cross each with both consuming head orders (`need`, `token`) and both callers (`take`, `later`):24 scenarios and120 configurations. Preserve the runner's query identities, alternating alias pattern, reversed token order and shared outside unknown. The first-valid-answer endpoint does not require the same answer order. Phase solving remains eager and uninterruptible within its registered bounds.

Run each configuration on logical CPUs0 and1, serially pinned; these are repeat placements on this host, not independent hardware samples. Use seed20260923 to shuffle scenario/CPU jobs and mode order within each job. One warmup block and seven measured paired blocks yield240 warmups and1680 measured processes,1920 total. Freeze the exact order in the manifest before running. Do not select cells or extend repetitions after viewing results under this registration.

The primary endpoint is the sum of disjoint source-build, preparation, query-build, solve/transport/setup/execute/observe/drop, input-drop, prepared-drop, consumer-drop and source-drop intervals. Independent complete-answer checks run outside these intervals. Include all owned answers through prepared disposal. Compilation, process startup, independent validation, outer preallocated bookkeeping and serialization of the measurement record are outside this endpoint. Record per-phase times for attribution; do not add nested or separately measured allocation intervals. This endpoint is not process latency or isolated first-answer latency. Different numbers of small interval clocks can affect tiny cases; do not make sub-resolution phase claims.

For each scenario/control/CPU, compute seven paired phase/control total-time ratios. A practical gain requires median <=0.90 and all seven ratios <1; a loss requires median >=1.10 and all seven >1. A reported direction must satisfy the same criterion on both CPUs; otherwise it is unresolved. This is a descriptive bounded pilot criterion, not a statistical confidence claim. Report all contrasts, ranges and raw pairs; no workload weighting or universal winner.

Each process has60 seconds wall/CPU and1GiB address space. The full launcher has600 seconds wall time. Capacity limits remain100,000 states and4096 answers; ordinary service permits200,000 ticks per continuation, not equal aggregate work. Preserve failed processes and cutoffs. Require every row to pass the ownership runner's independent complete-answer checks and the parent's mathematical count and phase/configuration checks. Freeze the ownership manifest, raw/audit receipts, source/binary hashes, launcher and this registration; verify unchanged hashes after runs.

A gain must carry the measured composition into broader source/lifetime comparisons. A loss requires attribution and consideration of a consequential avoidable cost, including eager continuation materialization and repeated caller setup. For overlap, investigate variance or resolution only if it can change a bounded decision. At the boundary compare further resource-phase work against adaptive search, local claims, richer theories, sustained lifetime and broader reuse; no more than four packages before a full breadth review.

Run `PYTHONDONTWRITEBYTECODE=1 python research/chr-hvm/resource_capacity/phase_costs.py`; reproduce analysis with `--analyze`.

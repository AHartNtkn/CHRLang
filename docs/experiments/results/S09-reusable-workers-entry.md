# Next parallel question: can reusable workers repay coordination?

Investigate a worker pool that survives changing CHR queries. Existing cold-worker results do not establish whether parallel execution pays after pool and source preparation costs are amortized. The decision can affect state ownership, input sharing, cancellation and the usefulness of source-derived independent regions.

## Current evidence and required change

The [first-cycle feasibility check](S00-first-cycle-feasibility.md) verified that two regional workers can enter service concurrently. Inspection of `research/chr-factors/src/parallel_regions.rs` shows query-specific searches created inside worker threads, with service requests carrying region, request ID and quantum. That implementation is a possible starting point, not evidence for reusable-query performance.

A reusable pool needs explicit begin/end-query ownership, worker-local prepared source, generation-safe request/reply handling, cancellation that drains or rejects old work, and complete shutdown. A fresh query must not inherit bindings, resource occurrences, history, buffered answers or exhaustion from a previous query. Keep infrastructure failure distinct from logical refutation.

## Gate and prospective comparison

Start with independently certified regions and the same source observations as the current serial execution. Check changed queries, changing region counts, cancellation while replies are pending, stale replies, worker failure, finite completion and duplicate/product observations. Verify actual overlap on the current hardware without treating overlap as speedup evidence.

Compare inline execution, one persistent worker and multiple persistent workers over compatible source and state representations. Include cold startup and reused pools, balanced substantial work, tiny jobs, skewed regions and output-product pressure. Separate rule preparation, pool creation, query transfer/setup, useful work, coordination, first/full observation, cancellation/end-query and shutdown. Avoid imposing an owned transport or duplicated representation on one side solely to fit the other executor's interface.

Hardware availability and allocation-meter suitability must be inspected before registering worker counts and resource bounds. A thread-local allocation meter cannot establish total process heap traffic without a validated cross-thread ownership/accounting design. Source, work and observation correctness remain independent of timing.

## Why this precedes another local refinement

The [subscription low-yield comparison](S01-subscription-low-yield.md) establishes both a practical retention opportunity and adverse lifetime cases. Refining its exact crossover would not change that bounded conclusion. Larger active-demand populations, other intermediate stages and generated multihead access remain required; they are not rejected.

The strongest ready architectural alternative is broader generated multihead access: it could remove generic selection on sources the existing single-head specializer cannot admit. Corrected replay/checkpoint policies also remain ready. Reusable workers are selected next because their persistent ownership boundary has not yet been priced, while the concurrency service and independent-region gate already provide a concrete implementation path. A warm-worker benefit could change the cost of parallel organization; a controlled loss could bound it despite the earlier cold-start confound.

The effort estimate is an implementation judgment: the pool requires a new query-lifecycle protocol and adversarial isolation gate before a bounded cost matrix; broader generated access requires extending selection/lowering and charging compilation and artifacts across a wider source surface. Reassess after the lifecycle gate reveals actual costs. The order preserves coverage rather than converting estimated effort into negative evidence.

T069 owns the reusable-pool/source gate and prospective lifecycle comparison. Connected-work parallelism, general source partitioning, whole architectures and the research goal remain open.

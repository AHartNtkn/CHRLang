# Different sources expose different execution costs

**Cycle traversal dominates deep equality allocation; candidate discovery dominates broad full-settlement allocation; readiness adds a separate large cost under bounded settlement.** The next experiment will test whether cycle traversal needs to construct owned descriptor copies.

The [registered diagnostic](../registrations/S02-execution-attribution.md) completes144 processes across36 source/representation/schedule configurations. Requested bytes, allocation/free counts, root-relative live/peak ownership, advances and complete answers match exactly between profile/control builds and repetitions. All72 profiled runs have exact exclusive allocation sums. Cancellation has zero inner scopes; compiled execution has only the outer scope.

## Three distinct responsibilities

Representative vector-bucket, two-query results:

| Source and schedule | Responsibility | Requested bytes | Share of execution allocation | Diagnostic wall share, two repeats |
|---|---|---:|---:|---:|
| Shared successful equality, depth64, full | Cycle traversal |1,899,160|92.62%|78.46–78.71%|
| Broad reversed source, width128, full | Candidate discovery |7,919,460|83.56%|51.85–52.48%|
| Broad reversed source, width128, bounded | Readiness traversal |7,719,256|44.89%|63.19–63.81%|

The bounded broad source still incurs the same7,919,460 discovery bytes as full settlement. Readiness is additional work, not a replacement for that discovery. In the deep successful source, there are274 cycle traversals but only14 candidate-discovery scopes. The broad full source has261 discovery scopes,506 incidence repairs and1,012 cycle traversals. Scope counts are exact work boundaries, not comparable units of CPU cost.

The favorable early-failure control remains important: bounded execution of the separate-equality depth64 case requests138,430bytes versus2,046,133 for full settlement. Its readiness traversal requests only1,136bytes. A policy that removes readiness indiscriminately would discard a demonstrated useful regime.

## What the clocks can establish

Diagnostic wall clocks and allocation callbacks are inside the measured execution and add overhead. Their shares support investigation of the responsibilities above; they do not establish ordinary speedups or exact CPU fractions. Non-cancelled profile/control execution-time ratios range0.526–1.670, with median1.008. That variation prevents treating these paired durations as a clean overhead correction.

The allocation/work conclusions do not depend on those clocks: every scope's counts and requested bytes repeat exactly, and their exclusive sums equal measured execution/observation allocation. Setup, consumer disposal and validation stay outside the attribution root. Primary lifecycle timing remains the earlier ordinary-allocator experiment.

## The next intervention

`Store::reaches` requests owned descriptors for each visited constructor value. The descriptor API constructs names and child vectors, although reachability needs only child identities. Its traversal also allocates a visited set and work stack. The current scope measures their combined cost; it does not yet rank those subcomponents.

Test borrowing constructor children during cycle traversal while preserving exactly the same reachability question, traversal order and visited-set behavior. This isolates descriptor ownership before considering memoization or a different cycle algorithm. Keep finite-tree failure, indirect cycles, constructor coalescence, forked ownership, early failure and broad consumption as correctness controls. No change to equality or language semantics is implied.

The [full portfolio review](S02-execution-portfolio-review.md) compares this step with discovery invalidation, readiness dependency work, graph memo attribution and solving. T072 remains active and the review count resets; the research goal remains unfinished.

[Raw evidence and source/binary freeze](s02-execution-attribution/) · [Per-run attribution](s02-execution-attribution/analysis.json) · [Runnable auditor](../../../research/chr-relational/experiments/execution_attribution.py)

Source/store/library regressions pass with execution profiling and vectors. The execution runner and existing admission/deduction profilers pass Clippy. Fixed callback storage is reused; ordinary builds contain none of these new execution scopes.

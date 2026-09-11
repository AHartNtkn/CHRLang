# Borrowed traversal removes a large temporary allocation cost

**Borrowing constructor children sharply reduces deep cycle-check allocation without changing traversal work. The complete compiled control remains faster in the new pilot.** The next investigation is discovery invalidation, whose measured cost borrowing leaves untouched.

The [registered intervention](../registrations/S02-borrowed-cycles.md) changes only descriptor ownership inside cycle traversal. Constructor children are read directly from relation rows; target checks, stack order, incidence order and the visited set are preserved. Both incidence representations pass source/store/library tests, including new deep indirect-cycle and shared-child DAG cases with independent expected answers and fork isolation.

## What borrowing removes

The allocation matrix completes144 processes with exact repeated profile/control agreement and complete ownership restoration. All scope counts and all non-cycle exclusive allocation records match the prior owned traversal. The12 compiled configurations are unchanged.

| Relational configurations | Lower requested bytes | Equal | Higher |
|---|---:|---:|---:|
| Complete lifecycle |20|4|0|
| Peak live bytes above root |11|13|0|

The four equal requested-byte cases cancel before execution. Large temporary allocation savings need not reduce peak residency much.

For vector buckets, full settlement, shared successful equality at depth64, two changing queries:

| Quantity | Owned descriptors | Borrowed children |
|---|---:|---:|
| Cycle traversal requested bytes |1,899,160|174,128|
| Complete lifecycle requested bytes |2,260,375|535,343|
| Peak live bytes above root |92,453|92,252|
| Cycle traversal scopes |274|274|

The1,725,032-byte lifecycle saving lies entirely inside cycle traversal. Borrowing removes descriptor copies rather than avoiding reachability checks. In the broad reversed width128 case, full lifecycle allocation falls only from10,131,396 to9,881,101bytes; discovery remains dominant.

## Complete cost against the stronger competitor

A separate ordinary-allocator pilot passes72 complete-source entry processes and300 timing processes, following60 excluded warmups. Each process prepares rules, runs two changing queries, validates complete scalar/analytical answers outside measured phases and releases all owned results. Counters and allocation diagnostics are off.

Compiled execution meets the registered favorable screen in all12 comparisons against borrowed/vector full settlement and all12 against borrowed/vector bounded settlement. For the reversed width128 broad case, compiled/full-relational paired lifecycle ratios range0.253–0.376, with median0.340. For shared successful depth64 equality, the corresponding range is0.061–0.237, median0.171.

These comparisons establish the current pilot's remaining gap. They do **not** establish borrowing's speedup over owned traversal: the owned and borrowed timing campaigns were separate, not interleaved pairs. A direct speed claim would need the registered follow-up specified in the experiment. No representation default is selected here. Input construction and independent validation remain outside the existing lifecycle endpoint; compilation is not isolated.

## Why discovery is next

The preceding execution attribution measured7,919,460 discovery bytes in the broad full-settlement witness. That scope is exactly unchanged by borrowing. The executor currently clears every candidate cache after an equality step, including rules whose heads may not discriminate on term shape or equality.

Test an inferred cache-validity condition for rules whose matches depend only on occurrence identity: flat heads with distinct variables and no guards are a candidate, not yet a proven certificate. Cached bindings must remain valid through representative changes and body execution. Repeated-variable heads, constructors, guards that become true, competing consumption, new occurrences and propagation history are necessary adverse controls. General rules must retain correct invalidation; this is an optimization hypothesis, not a language restriction.

This has more immediate decision value than another cycle-only refinement: it targets an unchanged major responsibility in a different source regime. Readiness traversal, broader integrated representation, graph memo attribution and direct solving remain required; reconsider them at the cache-validity gate. T072 remains active, package count is one since the portfolio review, and the research goal remains unfinished.

[Allocation comparison](s02-borrowed-cycles-allocation/borrowed-comparison.json) · [Allocation evidence](s02-borrowed-cycles-allocation/) · [Ordinary timing evidence](s02-borrowed-cycles-timing/) · [Paired timing screens](s02-borrowed-cycles-timing/analysis.json)

Both auditors pass; measured sources and binaries are frozen. Source/store/library regressions and Clippy pass with the new traversal. The separate graph and native prototypes receive no changes or priority from this result.

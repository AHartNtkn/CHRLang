# Adaptive ownership is qualified; fewer checks do not select the best policy

Adaptive search now has a credible eager control without adaptive schedule storage, and every tested owner is released through exhaustion or cancellation. Backoff reduces allocation on some sources, but copying or fixed schedules can still request less. The next experiment must compare complete time as well as memory across those controls.

The qualification passes **32 primary and 34 diagnostic regression tests**, including 36 eager/scheduled lockstep source cases per build and the existing adaptive semantic/progress checks. It also passes **576 ownership processes**, covering 288 configurations twice. This establishes finite ownership and credible controls, not a timing advantage or a complete architecture choice.

## Eager checking no longer pays for adaptive state

Repeated execution uses one generic algorithm with statically selected schedule storage. The eager schedule has no fields and always attempts independence. Scheduled execution retains EveryBoundary, fixed skips and failed-check backoff. Existing eager callers now use the stateless engine; there is no second executor implementation to drift semantically.

On this target, the eager engine occupies 88 bytes and the scheduled engine 120 bytes. With diagnostic fields enabled they occupy 152 and 184 bytes. The schedule contributes 32 bytes; the eager schedule itself occupies zero. These are engine-value sizes, not heap-allocation savings, RSS or a complexity score.

Across plain/history/late sources, rounds 0/1/3, depths 0/4 and two query identities, eager and scheduled-EveryBoundary execute identical progress, answer and exhaustion events. Complete answers also agree with the independent scalar evaluator. Their owner traces have identical allocation requests, frees and live-heap changes. Thus the forthcoming timing comparison can distinguish policy bookkeeping from a different eager algorithm.

## Backoff helps some allocation costs, but stronger controls matter

The table reports requested allocation bytes across four complete changing queries, with three additional rounds, private depth four and immediate consumer release. These are deterministic allocation observations, not time measurements.

| Control | Plain | Inherited history | Late links |
|---|---:|---:|---:|
| Copy | 5,254,017 | 62,405,399 | 4,724,961 |
| Initial-phase reunion | 5,209,929 | 54,035,151 | 4,680,873 |
| Stateless eager | 4,677,619 | 44,544,441 | 6,566,547 |
| Scheduled EveryBoundary | 4,677,619 | 44,544,441 | 6,566,547 |
| Fixed skip 1 | 5,365,413 | 44,681,739 | 6,091,849 |
| Fixed skip 8 | 5,764,631 | 51,854,173 | 5,268,289 |
| Backoff cap 1 | 4,627,459 | 44,544,441 | 5,914,005 |
| Backoff cap 8 | 4,602,379 | 44,544,441 | 5,630,511 |

Late-link backoff reduces the eager path's allocation, but neither cap beats copying or initial-phase reunion. Fixed skip 8 requests less than either backoff cap there. Comparing only backoff with eager would therefore overstate the case for adaptation.

Inherited-history backoff has exactly the eager allocation profile at both tested depths. The existing work gate explains the limit: independence checks succeed, so failure-triggered backoff does not skip them. It cannot recognize successful but economically unhelpful separation. Its state and decisions still need timing even when its heap requests match eager execution.

The smaller private-work cases supply further contrary evidence. At depth zero, plain eager requests 2,788,331 bytes versus copying's 2,500,729; at depth four, eager requests less than copying. No one source family or depth is assigned a workload weight, and allocation alone does not establish the runtime crossover.

Cancellation also changes the comparison. For late links at depth four, cancelling after the first answer per query requests 6,409,909 bytes with eager, 5,222,275 with backoff cap 8 and 4,630,611 with copying. For history at that depth, fixed skip 1 requests 24,279,097 bytes versus eager/backoff's 34,600,537. Each cancellation delivers a valid answer and the same preparation then serves a fresh query. These are first-permitted-answer endpoints, not a claim that all schedules publish an identical source-order prefix.

## Producer and consumer ownership separate cleanly

Each cell reuses preparation across four changing query identities and either exhausts every query or cancels after its first answer. Consumers immediately release answers, retain a window of four, or retain all delivered answers. Complete preflight answers and their raw multiplicities are independently checked; measured answers are checked by borrowing them between allocation phases, so the validator does not retain discarded outputs inside the owner account.

After every engine and input are disposed, preparation has exactly one strong owner. With immediate release, live heap returns to the prepared baseline after every query, including cancelled queries. Retained answers still validate after preparation is disposed. Disposing the consumer then returns measured live heap exactly to its initial baseline in every process.

For late links at depth four, immediate release leaves no additional consumer heap, the four-answer window leaves 6,456 bytes, and retaining all 64 delivered answers leaves 98,592 bytes. These values agree across the controls. The history family retains 2,433,312 additional bytes when all 64 answers are kept. Required output retention is distinct from engine history.

Source rules, independent expected answers, and fixed bookkeeping storage remain outside this diagnostic's owner baseline. Reported consumer bytes are additional retained heap relative to those owners, not the complete transitive size of every referenced object. Transient semantic validation runs between measured phases and is excluded from requested-byte totals. This gate does not measure source loading, RSS, sustained streams or total architectural lifecycle superiority; the next cost registration must state and charge its source/preparation boundary explicitly.

## Next measure costs without selecting a policy in advance

Keep T077 active for a prospectively registered counter-free lifecycle pilot. Carry Copy, initial reunion, stateless eager, scheduled EveryBoundary, fixed schedules and both backoff caps into it. Use both tested private-work depths, changing-query reuse, complete output and cancellation; include first-answer costs and sustained ownership where they could reverse the result. Separate diagnostic work/allocation runs from ordinary-allocator timing.

The strongest distinct alternatives remain incremental projection output, native local ownership, conditional equality/lifetime repair and broader caller reuse. Adaptive timing gets the next entry because this gate now exposes favorable and adverse allocation regimes with correct controls; actual time could reverse their ordering. More policy tuning without that comparison would not resolve the decision.

The eager control gate and the ownership matrix are two packages after the projection breadth review. Reconsider alternatives at the timing entry or any consequential obstruction, and complete another full breadth review by package four. Delayed choice splitting, checkpoint/replay policies, successful-but-unprofitable separation and broader source inference remain separate required questions. The research goal remains active.

## Evidence and validation

The [registration](../registrations/S04-adaptive-ownership.md), [eager tests](../../../research/chr-restoration/tests/eager_schedule.rs), [owner probe](../../../research/chr-restoration/examples/adaptive_ownership.rs), [bounded driver](../../../research/chr-restoration/experiments/adaptive_ownership.py), [audit](../../../research/chr-restoration/experiments/audit_adaptive_ownership.py) and [raw evidence](s04-adaptive-ownership/) record the implementation, matrix and outcomes. Existing [adaptive source evidence](S04-adaptive-entry.md) remains applicable to the scheduled policies.

Twelve bounded test executables pass, totaling 32 primary and 34 diagnostic tests. The initial owner preflight and all 576 matrix processes pass under 60-second wall/CPU and 1 GiB address-space bounds; each query has a 200,000-call service bound. All 288 allocation pairs agree exactly. Primary allocation-probe/test and diagnostic-test Clippy pass, as does formatting. The independent scalar oracle and reference interpreter are unchanged.

The [audit result](s04-adaptive-ownership/audit.json) checks frozen sources and binaries, regression coverage, exact repeats, owner conservation and eager/scheduled relative heap traces. Absolute process baselines differ by four bytes because the mode names have different lengths; the [audit note](s04-adaptive-ownership/audit-baseline-note.md) records the correction and preserves the initial verifier. Allocation requests and frees are not adjusted. Complete raw preflight answers and phase receipts are losslessly compressed, with compressed and original-content hashes in [the receipt inventory](s04-adaptive-ownership/compressed-receipts.json).

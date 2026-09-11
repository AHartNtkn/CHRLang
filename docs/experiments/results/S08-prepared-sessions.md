# Preparation reuse helps both complete execution paths

Reusing preparation has a confirmed practical benefit for both demand and generated execution in the128-query sessions. Demand is directionally faster than the matched generated control, but a10% advantage does not reproduce consistently. Immediate-release consumers leave stable engine-owned requested heap in every tested configuration.

The experiment also exposed a repairable output cost. Demand exported456 unnecessary bytes per answer on the common source. Reserving known field sizes eliminates that excess; retained common-source answers now occupy the same space in all three principal engines. This is an implementation repair, not evidence that graph semantics need more output storage.

## What the longer sessions actually do

Each session runs1,16 or128 queries through the existing mixed source. Query i has a fresh identity, depth4+i%3 and alternating arrival order. Queries exercise choice, consuming continuation, fresh aliased output and static propagation; the independent source retains branch-dependent work after counting. A scalar interpreter validates every original query before measurement. Measured answers are independently checked outside the recorded intervals, including after producer disposal.

The comparison either prepares once or prepares again for every query. Seven configurations include demand, conditional, generated Global/Active scanning and inferred scanning; conditional and generated GlobalScan include both initialization settings. Consumers release immediately, keep a two-query window or retain all answers. Cancellation abandons even-indexed queries and completes subsequent queries with the selected preparation policy.

The original and repaired pilots each complete3,024 processes. Each has252 exact allocation pairs, separate ordinary timing and RSS runs, and cancellation/disposal checks. The repaired pilot also has42 exact one-query reuse/rebuild allocation controls: when there is nothing to amortize, their ownership costs agree.

## Confirmed timing result

Two separately registered confirmations each run1,560 processes:24 excluded warmups and64 paired repetitions of24 configurations. These confirm128-query immediate/all consumers for demand and generated GlobalScan, with both generated initialization settings.

All12 reuse/rebuild contrasts exceed the registered10% practical-gain threshold in both confirmations. The table gives the quiet replication's paired-median interval for initialized paths; smaller ratios mean reuse is faster. Every preparation, query, execution/observation and disposal phase counts.

| Source / consumer | Demand reuse / rebuilding | Generated reuse / rebuilding |
|---|---:|---:|
| Common / immediate | 0.642–0.700 | 0.680–0.747 |
| Common / all retained | 0.688–0.743 | 0.687–0.760 |
| Independent / immediate | 0.603–0.667 | 0.633–0.697 |
| Independent / all retained | 0.645–0.695 | 0.670–0.721 |

Demand also exceeds10% against the uninitialized generated control in all four cases in both confirmations. Giving generated execution the same initialization opportunity narrows that result:

| Reused demand / reused initialized generated | First confirmation | Quiet replication |
|---|---:|---:|
| Common / immediate | 0.867–0.944 | 0.870–0.970 |
| Common / all retained | 0.839–0.933 | 0.841–0.913 |
| Independent / immediate | 0.817–0.916 | 0.877–0.947 |
| Independent / all retained | 0.824–0.903 | 0.808–0.865 |

All intervals favor demand directionally. Only the last replication establishes10%; its first confirmation does not, so the practical magnitude remains unresolved. Their intervals overlap. Phase attribution finds nearly unchanged median demand execution (12.93 versus12.97ms), while generated execution shifts from15.10 to15.40ms. This is a threshold-sensitive difference, not contradictory answer or ownership behavior. No observation is excluded or pooled into a more favorable result.

A brief auxiliary capacity-probe build overlapped the first confirmation. The entire confirmation was repeated without concurrent builds or experimental jobs; both runs remain in the evidence. Registered20-contrast order-statistic intervals give a combined union-bound error of0.02470 across both runs, conditional on independent, stable paired-ratio distributions. The signal floor is0.538ms and no contrast is signal-limited. These assumptions are not proof that environmental noise is independent.

## What owns the memory

After every immediate-release query, live requested heap is constant in all84 allocation configurations, including both preparation policies. After final preparation and consumer disposal, task-owned live heap returns to its root baseline in every metered run. The retained window stays bounded over these tested trajectories; keeping every answer makes consumer storage grow.

For128 queries with reused, initialized preparation (fixed, preallocated harness storage is outside these requested-heap deltas):

| Source / path | Immediate-release peak bytes | Consumer bytes when retaining all |
|---|---:|---:|
| Common / demand | 37,410 | 811,152 |
| Common / conditional | 30,721 | 811,152 |
| Common / generated | 63,067 | 811,152 |
| Independent / demand | 66,974 | 989,328 |
| Independent / conditional | 70,782 | 989,328 |
| Independent / generated | 93,863 | 891,024 |

The capacity probe explains the independent-source difference too: demand and conditional residual vectors retain768 spare bytes across each eight-answer query; generated export retains none. This is variable-length packing overhead, not semantically necessary data. The next consumer comparison carries direct export versus explicit compaction, charging resizing and immediate-release costs rather than treating smaller storage as free.

RSS is measured separately with stack-backed smaps_rollup reads and pre-touched fixed harness buffers. For reused initialized128-query immediate consumers, final RSS remains436–592KiB above the warmed root in the selected paths, even though all task-owned live heap has been released. Retaining all answers raises the corresponding resident deltas to1,596–2,036KiB. Those are process totals, including allocator residency and code/harness pages; they cannot be substituted for engine-owned bytes. The exact requested-heap trajectory establishes ownership restoration directly.

## The repair and its validation

The failing capacity test observes3,648 spare bytes in eight demand answers where the common source requires none. Export now reserves output count, resource arity and residual-call arity plus its output slot before filling vectors. The test passes, complete-source and demand tests pass, and Clippy passes. The reference interpreter is unchanged.

The entire pilot is rerun after the repair. All216 non-demand allocation controls reproduce exactly. Demand improves in36 cells; at128 retained queries the common source falls from1,278,096 to811,152 consumer bytes. The independent source falls from1,456,272 to989,328. No engine, source or consumer was omitted to obtain these results.

## Architectural consequence and next experiment

Both complete paths should reuse preparation where the source is unchanged. The tested reuse retains source artifacts, not completed derivations or active query graphs. It therefore answers preparation ownership but cannot settle a continuing producer's growing history. The source/engine/query/consumer owners are explicit, and export capacity is separable from logical obligations.

The [portfolio review](S08-prepared-sessions-review.md) selects continuing active-state ownership and reclamation next, including the previously identified conditional binding/support cost and variable-length export packing. Broader integration, coarser recognition, compilation and held-out challenges remain required. Native compilation, process startup and validation/harness setup are outside the summed execution endpoint here; no complete architectural lifecycle superiority is claimed.

## Reproduce or inspect

- [Pilot registration](../registrations/S08-prepared-sessions.md), [repair registration](../registrations/S08-prepared-sessions-repair.md), [confirmation](../registrations/S08-prepared-sessions-confirm.md), [quiet replication](../registrations/S08-prepared-sessions-replication.md).
- [Original pilot audit](s08-prepared-sessions/audit.json), [repaired audit](s08-prepared-sessions-repair/audit.json), [exact controls and repair effects](s08-prepared-sessions-repair/parent-comparison.json).
- [First confirmation](s08-prepared-sessions-confirm/audit.json), [quiet replication](s08-prepared-sessions-replication/audit.json), [threshold attribution](s08-prepared-sessions-replication/threshold-attribution.json).

Each results directory retains raw compressed receipts, frozen sources or their frozen parent reference, binary hashes, jobs and campaign completion. The two pilots and two confirmations total9,168 lifecycle processes, in addition to72 qualification sessions and the capacity/test diagnostics. Full observations, first-answer readings, all phase costs and RSS trajectories remain available for individual cases.

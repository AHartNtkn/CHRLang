# R02 repair and matcher maintenance: initial result

All 392 registered processes completed, validating 1,568 full answers across 56 cells. Every counted integrated batch/nested query exercised descriptor repair and source execution with pending deductions. Output-only matcher maintenance accounts for some requested heap, but does not explain the large nested-constructor cost. The nested result exposed a consequential congruence-work defect, so representation disposition awaits its separately registered correction.

[Registration](../registrations/R02-repair-maintenance-pilot.md), [source/binary freeze](R02-repair-pilot/freeze.json), [raw processes](R02-repair-pilot/raw.jsonl), [summary distributions](R02-repair-pilot/summary.tsv), and [execution order](R02-repair-pilot/order.json) preserve the complete pilot. The runner and analysis are `research/chr-integrated/experiments/repair_pilot.py` and `repair_analyze.py`. The historical source is 27e055b with exactly matching current runner, workload and build inputs; its library hash was verified against the commit before builds.

## Complete query lifecycle

Milliseconds below include one preparation, four queries at n,n+1,n,n+1, each first full observation and query/answer disposal, then prepared disposal. Native compilation remains excluded. Five-process medians are exploratory; ranges and all phases are in the summary.

| n=64 family/depth | Current integrated | Dedicated generated global scan | Integrated requested peak bytes | Dedicated requested peak bytes |
|---|---:|---:|---:|---:|
| batch / 1 | 1.773 | 1.655 | 277,862 | 100,995 |
| batch / 8 | 2.212 | 2.101 | 348,104 | 168,118 |
| nested / 1 | 3.421 | 1.750 | 337,917 | 116,696 |
| nested / 8 | 3.913 | 2.313 | 368,800 | 183,819 |

Batch lifecycle differences are under the registered 10% threshold or have overlapping ranges: no timing direction. Integrated beats dedicated indexed execution for batch, demonstrating why the scanned control matters. Nested has a large adverse direction against the global scan: size-64 ranges are disjoint at both depths. Allocation counts alone would mislead: nested integrated makes fewer execution allocation requests while retaining substantially more requested heap.

## Maintenance attribution

Head eligibility uses exact predicate name and arity. Every occurrence remains in observation storage; constructor child dependencies still carry equality and finite-tree obligations. Matcher predicate/argument memberships and occurrence wake-up incidence exist only for signatures in prepared rule heads. Independent tests exercise duplicate residuals, nested bindings, joint aliases, same-name/different-arity predicates and live consuming/propagating heads.

At n64, requested peak falls from 297,642 to 277,862 bytes for batch depth1; 357,697 to 337,917 for nested depth1; 259,924 to 238,432 for independent; and 217,302 to 196,618 for fanout. Build and low-yield repair peaks are unchanged. All corresponding integrated work counts are identical before/after. Fanout lifecycle improves 1.406→1.248 ms with disjoint ranges and more than 10%; most other timing comparisons are inconclusive. The eligibility lookup also has a cost: unchanged peak does not promise zero timing overhead.

Retain this narrow maintenance responsibility: it conserves storage while preserving complete observations. It supplies no general integrated speed claim and leaves a substantial heap gap to the strongest dedicated controls.

## Consequential defect investigation

At n8, nested performs 34 equality steps; at n64, 2,053. Batch performs 9 and 65 respectively. Both depths give identical counted work. Nested n64 has 74 descriptor repairs, 129 source applications and 128 applications with pending deductions; batch has 64 repairs, 129 applications and 126 with pending deductions.

Inspection finds `repair_step` enumerates every constructor peer at the repaired canonical signature, scheduling an owner equation for every pair before previous equations necessarily receive service. Constructor congruence needs transitive connection of owners, not a complete pair graph. This is a specific repairable mechanism in the candidate, not evidence that integrated equality intrinsically needs quadratic equality obligations. Investigate a single-witness obligation while checking pending signature movement, finite-tree failure and complete source replay. Preserve this pilot unchanged and preregister a follow-up before timing that treatment. This follow-up outranks immediate R04 work because it directly bears on the large observed representation loss; it is not a general invitation to tune the candidate.

## Validation and limits

Fourteen independent semantic tests and two runner tests pass in current and historical configurations; current additionally passes two maintenance tests. Both instrumentation configurations pass. Counter-free allocation-build Clippy and formatting pass. A separate read-only review found no blocker in source correspondence or maintenance ownership.

All three actual counter flags match their registered mode. Requested-heap meter self-checks pass, all four answer-disposal baselines are stable in each memory process, and empty-clock medians are recorded (current 22 ns). No timeouts or cutoffs occurred. Ordinary allocation and disabled counters govern primary timing; work/memory are separate builds. Retained bytes are requested heap, not RSS. Source traces and exhaustive audits are outside timing. No workload weighting, universal winner, OR/search claim, or isolated native compilation claim follows.

# Miss reuse earns its cost on delayed probes; access and arrival still change the winner

Within-turn miss reuse shows a replicated practical gain in four delayed-miss scenarios under every tested successful-result policy. Demand execution still has both gains and losses against Indexed execution. Reversing arrival order changes the large delayed-hit comparison from a demand loss to a gain.

The confirmation completes 5,376 primary processes and 224 warmups on two pinned CPUs. Every process preserves independently checked complete answers. These findings price the measured four-query lifecycle; they do not select a universal executor, settle general graph execution, or complete the research goal.

## What the confirmation establishes

Fourteen source/consumer scenarios compare eight modes: CurrentContext, StaticBirth and MatchDependencies, each with miss reuse off/on, plus Scan and Indexed. Each scenario has 24 paired blocks on each CPU. Mode positions are balanced exactly, all raw results are retained, and no timing outlier is excluded. The run finishes in about 697 seconds, within its registered bounds.

The primary endpoint sums 24 measured phases: source construction, preparation, four changing queries with input/setup and owned observation, engine disposal and consumer/preparation disposal. Independent oracle work, validation, fixed harness buffers, process startup, parsing and native compilation are outside that endpoint. First-owned-answer latency is secondary and is not added to session time.

A practical gain requires every paired session ratio to be below 0.90 on both CPUs; a loss requires every ratio above 1.10 on both. Anything else is unresolved practical separation. This is the prospectively chosen observed-repeat criterion, not a nominal confidence interval or a population guarantee. The 10% band is a reporting convention, not a workload weight or language policy.

## The cache saves real elapsed time

Each successful-result policy has four gains and ten unresolved comparisons against the same policy without miss reuse. There are no separated losses in that causal comparison. All four gains occur in delayed-miss scenarios: size 8 original arrival, size 128 original arrival, size 128 reversed arrival, and size 128 original arrival with retained-all output. Known-miss allocation savings do not automatically become a confirmed timing gain.

For size 128, original arrival and immediate release, CurrentContext's CPU-0 median measured session falls from 1,128.94 ms to 9.50 ms with miss reuse. CPU 2 falls from 1,269.14 ms to 10.63 ms. Every paired ratio supports the registered gain. “Miss” here means an unsuccessful matching/forcing probe; the complete query still returns its correct answer, including any residual constraints.

The required mechanism is specific. A run-owned map retains misses within a source turn; the next turn clears it. Probes depending on temporary recursive unknowns cannot supply reusable misses. The earlier [validity experiment](S03-miss-reuse.md) tests late changes and rejects stale cross-turn reuse. This result prices those lookup, clearing and validity duties. It does not establish a general cross-query cache or justify its retention policy.

## A stronger access control reverses apparent advantages

Selected CPU-0 median measured session times illustrate the source dependence. All rows use size 128 and immediate release; CPU-2 results support the stated practical classifications.

| Source | Demand with CurrentContext miss reuse | Scan | Indexed | Registered consequence |
|---|---:|---:|---:|---|
| Known hit, original arrival | 0.874 ms | 1.637 ms | 2.554 ms | Gain against Indexed; Scan separation unresolved |
| Delayed hit, original arrival | 120.340 ms | 332.235 ms | 18.413 ms | Gain against Scan; loss against Indexed |
| Delayed hit, reversed arrival | 0.876 ms | 1.754 ms | 2.850 ms | Gain against both controls |
| Delayed miss, original arrival | 9.495 ms | 8.576 ms | 1.705 ms | Loss against Indexed; Scan separation unresolved |

The original delayed-hit row is an especially useful control. Demand is substantially faster than Scan but substantially slower than Indexed. Comparing only with scanning would misidentify an access/discovery difference as an executor-wide advantage. Conversely, the reversed source has a real demand gain against Indexed; the Indexed result is not universal either.

The allocation evidence adds a separate cost dimension. Demand has higher measured peak heap than Scan in every matched ownership scenario, including its confirmed timing gains. A faster session can therefore require more live memory. Requested traffic, peak heap and elapsed time must remain separate; none supplies an implicit weight for the others.

## All predeclared comparisons remain visible

Counts below refer to the 14 experimental scenarios, not frequencies of application use.

| Demand mode | Against Scan: gain / loss / unresolved | Against Indexed: gain / loss / unresolved |
|---|---:|---:|
| CurrentContext | 2 / 5 / 7 | 4 / 6 / 4 |
| CurrentContext + miss reuse | 2 / 0 / 12 | 5 / 4 / 5 |
| StaticBirth | 6 / 5 / 3 | 6 / 6 / 2 |
| StaticBirth + miss reuse | 2 / 0 / 12 | 5 / 4 / 5 |
| MatchDependencies | 2 / 5 / 7 | 4 / 6 / 4 |
| MatchDependencies + miss reuse | 2 / 0 / 12 | 3 / 4 / 7 |

Different separation counts do not establish that one successful-result policy is faster than another. The registered comparisons pair miss reuse with its own control and demand modes with the two compiled controls. They do not directly select among the three successful-result policies, and these deterministic sources do not exercise their general cross-choice benefits.

First observation is available in every complete query. For the original size-128 delayed-miss case on CPU 0, median first-owned-answer latency is 2.294 ms for CurrentContext with miss reuse and 0.311 ms for Indexed. Including measured source/preparation/input/setup for that first query gives 2.419 and 0.465 ms. These are first-query observations in this harness, not cold process-start latencies or evidence about fairness between continuing alternatives.

## The unresolved comparisons need a different diagnostic, not selective exclusions

Pinning does not remove all large timing excursions. The most extreme current/plain-size-8 session takes 3,081,771 ns on CPU 0, about 53 times its matching cell median. Its fourth query setup accounts for 3,013,451 ns; that phase's median is 2,747 ns. Another Scan setup takes 6,210,702 ns against a matching median near 98,738 ns. Source endpoints and service counts are unchanged.

The [excursion analysis](s03-dependency-timing/excursion-analysis.json) locates the eight largest within-cell session excursions in individual setup or execution phases. The data contain elapsed time, not phase CPU time or scheduler traces. They therefore cannot determine whether these excursions arise from allocation/runtime behavior, external preemption or another system effect. They remain included in all classifications.

The concrete follow-up is paired CPU/elapsed attribution on the affected source/phase cases, with its own overhead qualification. It could resolve comparisons whose typical ratios differ substantially but whose observed ranges cross the practical boundary. More repetitions under the same extreme-range rule are not, by themselves, a remedy: additional samples can reveal additional excursions. These classifications remain unresolved, not equivalent or rejected.

## Architectural consequence and next investigation

The bounded result is now useful: reusing unsuccessful probes can repay its validity machinery by a large margin, yet a competent access plan and arrival order can still determine which executor is economical. This comparison does not isolate demand organization from access policy. Repeated successful discovery, broader resource indexing, dynamic choices, nonground body posts, writable heads and sustained ownership remain required T071 investigations.

Proceed to T076 reusable symbolic formulas and union/inclusion, as selected at the [breadth review](S03-dependency-clock-breadth-review.md). That investigation could replace evaluation rather than refine recognition of the same work. Resolving additional timing classifications here cannot overturn the observed coexistence of favorable and adverse demand regimes, though it can change their boundaries; preserve the targeted CPU/elapsed follow-up rather than treating it as resolved.

T072's cheaper relevant-read validation also remains required. Complete-architecture and held-out comparisons must eventually combine these results with source eligibility, necessary responsibilities and broader language tradeoffs. No cache default, mandatory restriction or universal winner is adopted. The goal remains active.

## Evidence and reproduction

[Prospective registration](../registrations/S03-dependency-timing.md), [timing driver](../../../research/chr-direct-conditional/experiments/dependency_timing.py), [exact scenarios](s03-dependency-timing/scenarios.json), [frozen order](s03-dependency-timing/order.json), [source/binary freeze](s03-dependency-timing/freeze.json), [all raw runs](s03-dependency-timing/runs/), [per-process records](s03-dependency-timing/results.jsonl), [driver audit](s03-dependency-timing/audit.json) and [all 210 audited contrasts](s03-dependency-timing/review-audit.json).

Run `python research/chr-direct-conditional/experiments/dependency_timing_audit.py` to check every raw result, ownership endpoint signature, balanced position, phase total, first-observation field and predeclared classification. Full independent answer checks occur in the frozen runner before each process succeeds. The [clock qualification](S03-dependency-clock.md) and [ownership comparison](S03-dependency-ownership.md) supply the preceding measurement evidence. The driver refuses to overwrite its results.

# R02 transitive congruence: correction and architecture disposition

One durable congruence connection per repaired constructor preserves complete source behavior and substantially reduces redundant equality work. It does not establish a general timing advantage for integrated execution. Complete setup and disposal costs matter, and the strongest dedicated access policy changes with workload size.

The [224-process follow-up](../registrations/R02-congruence-witness-pilot.md) validated 896 full answers. Its larger-case crossover justified a separately [registered access-control comparison](../registrations/R02-large-access-control.md): 70 processes and 280 full answers. Together with the [initial repair pilot](R02-repair-maintenance-pilot.md), T029 records 686 processes and 2,744 complete answers, with no timeout, cutoff, feature discrepancy, answer mismatch or query-retention growth.

## Correctness and mechanism

Every constructor bucket is connected through established or queued owner equalities. Inserting one descriptor therefore requires one edge to a current valid peer, not an edge to every peer. Equality only coarsens child identities; key movement cannot invalidate a queued owner equation. Owners and these obligations survive occurrence consumption. Finite-tree checks remain mandatory when an equation is serviced.

A new independent two-layer constructor stress first failed at n8 with 62 equality steps against the linear service bound. The correction passes at n8,32,64, including source firings while repairs and equations remain pending, terminal congruence audit, exact live resources, history and full joint observation. Independent review found no counterexample to the pending-obligation connectivity argument.

| Nested size, depth1 | Equality steps before | Equality steps after | Four-query lifecycle before, ms | After, ms | Peak requested heap before | After |
|---|---:|---:|---:|---:|---:|---:|
| 8 | 34 | 16 | 0.318 | 0.293 | 42,708 | 42,452 |
| 64 | 2,053 | 128 | 3.379 | 2.045 | 337,917 | 268,742 |
| 256 | 32,805 | 512 | 34.153 | 11.269 | 3,102,175 | 1,077,117 |

The n8 timing is inconclusive; n64/n256 before/after ranges are disjoint with substantial reductions. Batch equality work remains n+1; independent/fanout/repair/build timing directions are inconclusive. The targeted fix concerns real serviced obligations and allocations, not a counter definition.

## Complete paths and competing access policies

At n64, corrected nested integration still loses to dedicated generated global scan: depth1 2.045 vs1.714 ms; depth8 2.701 vs2.234 ms. Ranges are disjoint. Execution alone would conceal the boundary cost: at depth1 the mean-per-query execution medians are 296.7 vs315.0 microseconds, while setup is159.9 vs66.9 and engine disposal23.5 vs11.5 microseconds. These are component observations, not independent architecture rankings. Complete lifecycle includes one preparation, four queries, first full observations and all disposal; native compilation remains excluded.

At n256 integration appeared to beat global scan, requiring the indexed control at that same size. Fresh five-process comparisons give:

| Family, n256 depth1 | Integrated ms | Generated indexed ms | Generic indexed ms | Generated global scan ms | Integrated peak requested heap | Generated indexed peak |
|---|---:|---:|---:|---:|---:|---:|
| batch | 8.336 | 8.866 | 8.814 | 17.447 | 1,484,225 | 547,531 |
| nested | 11.134 | 9.383 | 9.333 | 17.311 | 1,077,117 | 610,080 |

Batch's integrated/indexed difference is below10% and ranges overlap: inconclusive. Nested integration is about19% slower than the generated indexed control, with disjoint ranges. Active scanning is slower than these controls in both cases. A scan crossover alone would have overclaimed the result. Higher requested heap persists even when integrated execution uses fewer allocation calls.

## Disposition and next investigation

Retain the integrated candidate as a bounded alternative: its source effects genuinely overlap pending equality and its fanout regime has a modest measured benefit. Its necessary machinery includes class ownership, descriptors and parent incidence, constructor congruence obligations, matching indexes, source activation, occurrence resources, propagation history, finite-tree checks and complete publication. Head-only matcher maintenance and transitive congruence obligations are sound reductions in avoidable work; they do not eliminate that machinery.

Dedicated execution remains the strongest measured choice on the nested cases once its competent access policy is included. Batch is inconclusive at larger sizes and worse at the small case; integrated heap remains higher. Do not choose a universal architecture without workload weights the user has not supplied. Do not generalize this ordinary positive-guard experiment to OR, nonmonotone guards, parallelism or long-lived within-query reclamation.

Select a **compiled explicit-search semantic gate** next, rather than another R02 refinement. R04's finite fragment and independent oracle are ready, but current compiled execution rejects source OR. A credible native branch control can distinguish eliminating source execution from merely improving its backend, and can also ground R03's conditional/recomputation comparison. The strongest alternative is implementing native maintaining-arc-consistency versus support-CNF learning directly. That can select a finite backend but cannot yet compare avoiding ordinary rule execution. The missing compiled-search control therefore has broader immediate decision value. No source semantic adoption decision is needed for that bounded gate. Measure copying/persistence alternatives only after their obligations and complete answers are checked; a control's availability gives no automatic priority to tuning it.

R02 is bounded at these tested ordinary regimes, not universally closed. Reopen it for a concrete surviving search/guard/representation interaction or a demonstrated defect that could change the recommendation. Native compilation amortization, direct conditional activation, solver inference/learning, search lifetimes and parallel coordination remain open in the coverage map.

## Reproduction and verification

[Congruence source/binary freeze](R02-congruence-pilot/freeze.json), [source snapshot](R02-congruence-pilot/source.tar.gz), [raw runs](R02-congruence-pilot/raw.jsonl), [summary](R02-congruence-pilot/summary.tsv); [large-access freeze](R02-large-access-pilot/freeze.json), [raw runs](R02-large-access-pilot/raw.jsonl), [summary](R02-large-access-pilot/summary.tsv). Scripts are `congruence_pilot.py`, `congruence_analyze.py`, `large_access_pilot.py`, `large_access_analyze.py` under `research/chr-integrated/experiments`. Build scripts, exact orders, environment, feature trees and calibration logs accompany each main freeze. Historical input reconstructs from27e055b plus the initial repair source snapshot; identical runner/workload/build hashes were checked.

Workspace all-target tests and Clippy pass. Counter-free experiment tests and allocator-build Clippy pass; formatting passes. Fifteen independent semantic cases, two maintenance cases and two native runner cases cover the current package when experiment features are enabled. Primary timings use an ordinary allocator with all three counter flags disabled; requested allocation diagnostics and work counts are separate. Empty-clock medians and stable post-answer requested live bytes pass. No exhaustive audit or analytic oracle work occurs in primary timing intervals. Process medians/ranges are exploratory; no aggregate score, RSS claim or native compilation break-even is reported.

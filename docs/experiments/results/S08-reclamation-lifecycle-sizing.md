# Reclamation reduces retention, but does not settle total efficiency

Selective metadata reclamation reduces the dependency graph's peak requested memory in every tested configuration. Complete-cost samples show mixed timing effects and substantial remaining costs. The next investigation returns to distinct integrated execution organizations rather than extending this local reclamation cycle automatically.

**These timings are exploratory.** Each cell has one ordinary-allocator sample. No practical gain/loss classification, sustainable growth law or whole-architecture ranking follows. The [breadth review](S08-lifetime-breadth-review.md) preserves the remaining lifetime and call-reuse obligations.

## What ran and what was charged

**All 4,818 registered processes completed successfully.** The 144 source/consumer configurations each have eleven controls, two allocation repetitions and one ordinary sample. Another 66 processes cancel the first query after four answers and complete the second. The [independent audit](s08-reclamation-lifecycle-sizing/audit.json) verifies 1,584 exact allocation replays, all command parameters, randomized ordinary order, complete observations/counts, source/binary hashes and primary-cost arithmetic. No samples were excluded.

**The comparison includes preparation, both changed queries and every owner disposal.** Primary time sums preparation, setup, service/observation, consumer disposal, engine disposal and prepared disposal. Source/input-inclusive time is also recorded. Graph maintenance runs inside service/observation, including scans and vector shrinking. No extra collection occurs at exhaustion. A separate per-maintenance timer was not added: repeated tiny timer calls would perturb this sizing comparison; the current evidence does not isolate maintenance duration from its effects on subsequent work.

**Ordinary builds omit heap snapshots and all diagnostic counters.** Separate meter builds record requested heap allocation, not RSS. Fixed harness slots are preallocated equally across consumers outside owner baselines. Complete independent replay finishes before measured owners exist. The runner gate covers all eleven controls, three consumers and changed queries, including assertions that the configured reclamation actually runs. Native compilation is excluded.

## Memory findings

**Reclamation reduces peak retention more consistently than allocation traffic.** Immediate and every-16-answer reclamation both lower the dependency graph's peak requested growth in all 144 configurations. Template-graph peaks fall in 118 and 120 configurations respectively. These are exact measured allocation comparisons, not timing classifications.

**Frequent shrinking can add allocation traffic.** Immediate dependency reclamation lowers primary requested bytes in none of the 144 configurations; periodic reclamation lowers them in 72. For templates, immediate reclamation lowers traffic in nine cases and periodic reclamation in 81. Returning space during a live query and minimizing total allocation are different objectives.

**Consumer release consistently separates answer storage from engine state.** All 1,056 matched engine/source/query groups reach identical engine-owned bytes after consumer release under immediate-release, window-four and retain-all policies. Every query and prepared disposal restores its live baseline. The [full snapshots](s08-reclamation-lifecycle-sizing/summary.json) retain intermediate delivery points, not only final memory.

## Representative complete costs

**The repeated-value stream shows both the memory benefit and the remaining cost.** This configuration has depth 64, work four between answers, payload eight, token consumption and immediate consumer release. Timings are single ordinary samples for preparation plus two changed queries. Post-release bytes describe the first query after 65 answers, with its engine still alive.

| Engine / maintenance | Primary time, ms | Primary requested bytes | Peak requested growth | Engine bytes after consumer release |
|---|---:|---:|---:|---:|
| Direct | 1.095 | 1,773,074 | 33,878 | 17,818 |
| Compact live-history reuse | 1.533 | 3,244,989 | 605,006 | 572,289 |
| Specialized Scan | 2.366 | 4,281,983 | 402,186 | 44,096 |
| Dependencies / retain | 51.252 | 8,441,069 | 879,662 | 851,405 |
| Dependencies / every answer | 50.575 | 8,442,189 | 527,144 | 435,981 |
| Dependencies / every 16 answers | 51.397 | 8,425,837 | 651,768 | 497,405 |
| Templates / retain | 4.941 | 1,895,785 | 371,091 | 354,882 |
| Templates / every answer | 3.083 | 1,896,929 | 290,825 | 249,602 |
| Templates / every 16 answers | 3.512 | 1,881,793 | 305,688 | 256,266 |
| Checked lazy generator | 0.111 | 189,528 | 8,860 | 0 |

**Other source relationships retain contrary results.** On distinct values with the same remaining parameters, template timings are 6.205 ms retaining metadata, 6.613 ms reclaiming after every answer, and 4.758 ms reclaiming every 16 answers. On aliases they are approximately 51.7, 51.8 and 53.3 ms. These observations identify possible regimes for future confirmation; they do not prove the timing differences are reliable.

**Metadata removal leaves other owners intact by design.** Graph nodes, obligation slots, birth contexts, resources and potentially useful results remain. Their reachability and identity requirements are separate from the proved incompatibility of reclaimed supports. The memory benefit cannot be extrapolated into a complete graph collector, and the remaining cost cannot be declared intrinsic to graphs.

## Architectural implications and limits

**Retaining all metadata was a consequential implementation choice.** Reclamation substantially reduces measured graph retention before engine disposal while preserving the tested service behavior. It also introduces a scan-and-shrink operation and a maintenance cadence. A production choice must account for that machinery, useful-cache retention, memory trajectories and latency rather than selecting the smallest final heap number.

**The strongest exact-source control still avoids most of this machinery.** Its checking, query setup and lazy owned-answer construction are charged, but it remains hand-derived. The source-derived compilation question is how much of that elimination can be inferred for broader programs and at what preparation/language cost. This pilot does not answer it.

**The source-policy counterexample remains outside equivalent-source timing claims.** The [reclamation gate](S08-metadata-reclamation-gate.md) records a contested token for which graph service and fixed scalar order choose different winners. This cost matrix uses independently equivalent stream sources. It does not adopt a language policy or repair the broader fixed-policy correspondence question by assumption.

The [four-package breadth review](S08-lifetime-breadth-review.md) selects T072's distinct integrated organizations next. T074 remains unfinished for sustained regimes, confirmation, broader reclamation, failing-stream costs, compact outputs and publication. The architecture goal remains active.

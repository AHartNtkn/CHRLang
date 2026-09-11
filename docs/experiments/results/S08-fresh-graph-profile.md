# Graph costs split by execution mode

**A generic memo-storage replacement is not the next supported intervention.** With the qualified seeking algorithm, context inclusion still consumes 58–63% of sampled user cycles in dependency execution and consuming templates. Pure templates instead spend 18.6% in finite validation and 8.3% in set-insertion routines, with only 4.4% in context inclusion. The useful boundary depends on what the graph executes.

This refresh completed **144 lifecycle qualification processes, 48 exact allocation pairs and 1,200 separately profiled complete lifecycles**. All eight profiles exceed the registered 500-sample gate, with 1,289–10,959 samples and zero lost samples. Current engine code was built and frozen; no engine behavior changed in this package.

## What was compared

The [registration](../registrations/S08-fresh-graph-profile.md) uses existing dependency, template and independent direct execution. Lookup and seeking both retain completed-traversal reuse. Pure/consuming sources and immediate/retained-all consumers run two changing queries per preparation. Continuing sources deliver 128 answers per query through a capacity-four queue and cancel with work pending; flat sources exhaust one alternative per query. Full answers and consumer ownership are checked by the existing source/lifecycle gates.

Ordinary qualification and metered runs are separate. Eight pinned `perf` invocations profile continuing dependency/template execution with lookup/seeking and pure/consuming sources; dependency invocations run 100 complete processes and template invocations run 200. Sampled fractions cover the complete child processes and their execution environment. They are diagnostic attribution, not exclusive heap fractions or a fresh ordinary timing comparison.

## Where sampled cycles go

Columns are disjoint self-symbol percentages from the retained reports. Context sums lookup/seeking and its inclusion wrapper. Set insertion includes the visible set-valued B-tree insertion symbols; it does not establish which owner allocated every node. These symbols use the report's 0.5% display threshold, so smaller individual routines are not included in these sums.

| Context / graph / source | Context inclusion | Finite validation visit | Force | Set insertion |
|---|---:|---:|---:|---:|
| lookup / dependencies / pure | 76.72% | 7.41% | 4.51% | 1.61% |
| lookup / dependencies / consuming | 80.50% | 5.47% | 4.58% | 0.82% |
| lookup / templates / pure | 16.07% | 21.31% | 9.54% | 8.11% |
| lookup / templates / consuming | 79.69% | 4.30% | 3.86% | 1.53% |
| seeking / dependencies / pure | 57.76% | 13.87% | 9.35% | 2.52% |
| seeking / dependencies / consuming | 61.95% | 10.84% | 8.94% | 2.19% |
| seeking / templates / pure | 4.40% | 18.60% | 9.04% | 8.34% |
| seeking / templates / consuming | 62.58% | 8.61% | 6.33% | 3.27% |

The distinction is material. Replacing finite-validation scratch or its completed set may help pure templates, but those functions do not explain the dominant context cost in the consuming template profile. Conversely, another inclusion-only change would miss much of the pure-template cost. Prior [context selection experiments](S08-context-selection.md) also show that density alone does not select a safe economical algorithm: early conflict and sparse support remain required adverse cases.

Source inspection identifies actual inclusion callers: selecting compatible result edges during force and normalization, validating finite completed results, filtering output obligations, and checking resource birth/consumption. The [annotated seeking assembly](s08-fresh-graph-profile/seeking-context-assembly.txt) resolves the sampled symbol to the ordered-map context operation. It does not assign the caller's contribution; that is the next measurement boundary.

## Requested ownership remains substantial

All 24 matched lookup/seeking ownership comparisons have identical requested totals, peaks and per-phase requested bytes. The predicate changes which work executes, but neither variant changes requested ownership on these fixtures.

Representative continuing sessions: two changing queries, 128 answers each, immediate release.

| Execution | Requested bytes | Peak above resident inputs | Requested bytes in production |
|---|---:|---:|---:|
| dependencies, pure | 9,689,899 | 682,842 | 99.92% |
| dependencies, consuming | 14,067,558 | 1,068,036 | 99.91% |
| direct, pure | 766,053 | 50,788 | 99.26% |
| direct, consuming | 1,308,354 | 53,079 | 99.35% |
| templates, pure | 6,464,623 | 979,336 | 99.88% |
| templates, consuming | 8,519,230 | 1,357,268 | 99.85% |

Production contains expansion, graph observation and other nested work; these totals cannot identify a memo allocation fraction. Every metered lifecycle restores ownership after disposal, and both repeats agree exactly after normalizing resident inputs. Peak requested heap is not RSS. The direct control prevents treating a graph-internal improvement as an architecture choice.

## Next: distinguish repeated validity work from lookup cost

Keep T074 active. Register caller-specific inclusion work and allocation attribution: number of checks, examined support/context entries, success versus early rejection, and costs of result selection, finite validation, output filtering and resource liveness. Reuse existing profiling/meter infrastructure where it fits; do not introduce a storage representation before measuring its responsibility.

The gate must include repeated and unique checks, sparse/dense supports, early/late rejection, shared versus independent consumed resources, changing contexts, and cancellation. The present successful streaming lifecycle cannot alone distinguish all those cases. Add end-to-end sources for the missing contrasts and validate raw results, priority and ownership before comparative execution. Pair instrumented and ordinary builds; reconcile exclusive requested bytes with complete lifecycle phases, including finite-path/completed-set ownership where separable. If caller attribution establishes substantial redundant validity work, qualify a context-validity reuse intervention against recomputation. If it instead isolates lookup or scratch costs, test that specific representation change with the same adverse sources.

This is a measurement-driven refinement of the [portfolio review](S02-direct-readiness-review.md), not rejection of memoization or a new priority for graph execution. Broader integration, solving, generated execution, sustained ownership, parallelism and language tradeoffs remain required. Package count one; full review within three more packages, earlier if the cross-architecture decision changes. The goal remains active.

Reproduce with `python3 research/chr-reuse/experiments/fresh_graph_profile.py --audit`. [Analysis](s08-fresh-graph-profile/analysis.json), [responsibility summary](s08-fresh-graph-profile/responsibilities.json), [source/binary freeze](s08-fresh-graph-profile/freeze.json) and raw profiler/ownership receipts are retained together.

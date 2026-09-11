# Long prepared sessions now have trustworthy ownership accounting

**All seven modes pass 8,192-query sessions.** The completed qualification covers
1,470 processes and 431,424 query sessions, including full consumption and
first-answer cancellation. It also repairs a validation bottleneck without changing
any measured application allocation in the 1,431 preserved comparison receipts.
Longer sessions are now available for observed compiler amortization.

## What changed

The shared runner sizes query counts and phase records from the actual workload.
It retains three explicit input policies: repeated depths with changing variable
namespaces; a four-depth-pair cycle with changing namespaces; and growing-distinct
depths. The cycle represents a bounded reusable working set. Growing inputs remain
a separate adverse case.

The first version successfully completed every smaller case and three long cases,
but each long process spent roughly 51–53 seconds qualifying execution. It repeatedly
ran the independent oracle on alpha-equivalent queries. That run was deliberately
interrupted to repair validation; every completed receipt is preserved.

The validator now caches expected answers by depth pair within one fixed ruleset.
Each distinct class is checked by Direct and the independent scalar evaluator.
Every delivered answer is still checked with the existing exact alpha-equivalence
comparison, preserving multiplicity, aliases and residual facts. Growing-distinct
queries receive separate oracle evaluations. A 200-comparison executable check
covers all families and policies, fresh outputs and distant namespaces.

The repeated 8,192-query trace qualifier fell from 53.23 to 2.08 seconds; the cycle
fell from 51.11 to 2.24 seconds. Direct's repeated case fell from 51.98 to 5.08
seconds. These are **validation-process observations**, not engine speed
comparisons. Application requested bytes, peak ownership, consumer bytes and
query-end trajectories agree exactly with all 1,431 corresponding terminal
pre-amendment receipts.

## Retention depends on the query working set

With outputs dropped, the ordinary depth-128 trace case retains:

| Session policy | Retained bytes after the working set is established |
|---|---:|
| Repeated queries, complete | 19,860 |
| Four-query cycle, complete | 62,345 |
| Repeated queries, first-answer cancellation | 62,211 |
| Four-query cycle, first-answer cancellation | 233,117 |

All four stabilize within the 64-query checks. The complete repeated and cycle
cases stay at the same levels through 8,192 queries. The values include prepared
state and retained traces after query-engine/input disposal, with no consumer
outputs retained. Cancellation keeps more unfinished work available for reuse.

Growing-distinct complete queries behave differently: across the four families,
query-end trace ownership starts around 10–13 KB and reaches 105–109 KB by query
sixteen. Completed-key retention therefore remains a real cost. A finite working
set plateau does not settle eviction or regeneration for changing workloads.

## Long-session heap tradeoffs

The following full-consumption sessions use family zero, depth 128, 8,192 queries
and immediate output disposal. Requested GB is cumulative requested heap allocation
(decimal); peak KiB is maximum application-owned live heap divided by 1,024.

| Mode | Requested GB, repeat / cycle | Peak KiB, repeat / cycle |
|---|---:|---:|
| Trace | 1.056 / 1.068 | 147.9 / 191.5 |
| Direct | 4.859 / 4.963 | 69.2 / 71.1 |
| Scan | 2.827 / 2.877 | 98.4 / 101.2 |
| Indexed | 4.045 / 4.122 | 102.7 / 104.9 |
| Inferred | 3.367 / 3.428 | 86.0 / 87.3 |
| Generated | 3.014 / 3.067 | 102.7 / 105.1 |
| Generated plus inferred | 2.847 / 2.896 | 85.9 / 87.1 |

Traces request the fewest bytes here while retaining the largest peak. Scan also
requests fewer bytes than ordinary generated execution, reinforcing the need for
strong generic controls. These heap results establish no runtime or compiler-cost
winner. Requested allocation is not RSS; oracle expectations and measurement
records are outside application heap roots, while all application state and
consumer capacity are included.

## Validation and next experiment

The 490 configurations cover all seven modes and four families. They include
448 sixty-four-query cases across two depths, retention, cancellation and repeated
or cycling inputs; 28 growing-distinct sixteen-query cases; and fourteen
8,192-query scaling cases. Each has two exact allocation repetitions and one
ordinary source-validation repetition. All counts, oracle-class counts, ownership
continuity, consumer equivalence and final root restoration pass. Generated source
gates and both ordinary/metered cache tests and scoped Clippy checks pass.

The verbose receipts are losslessly compressed with hashes of both original and
compressed bytes. The successful pre-amendment receipts remain independently
inspectable; none is presented as a failed candidate or a completed original matrix.

**Next: register replicated user-program compilation and longer paired sessions
against Direct, scan and inferred controls as well as generated execution.** Include
favorable and adverse cases from the preceding timing diagnosis, and keep source
emission/build/artifact disposal separate from shared dependency construction.
Use observed sessions to test amortization rather than declaring a crossover from
four-query medians. Bounded retention, broader observers and sparse projection
remain required alternatives. This amended qualification is package two after the
last portfolio review; T075 and the research goal remain active.

[Initial registration](../registrations/S05-call-long-entry.md) ·
[Validation amendment](../registrations/S05-call-long-cached-oracle.md) ·
[Preserved initial receipts](s05-call-long-entry/) ·
[Completed qualification](s05-call-long-cached-oracle/) ·
[Exact comparison and trajectories](s05-call-long-cached-oracle/diagnosis.json).

Reproduce with `call_long_entry.py audit` and `call_long_analysis.py audit` in
`research/chr-reuse/experiments/`.

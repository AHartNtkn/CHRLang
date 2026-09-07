# E15 corrected service/session cost comparison

The corrected timing batch favors direct equality over both Python net services
throughout this matrix. Maintaining controller counts lowers logical action totals,
but does not establish a runtime improvement over terminal scanning. Duplicate-state
grouping and asynchronous latency isolation retain their distinct benefits; neither
shares an equality across different source states.

All 810 timing sessions and 270 separate allocation sessions pass the
[combined audit](E15-cost-audit-v2.json): 3,240 query observations, the complete
135-configuration matrix, exact work agreement across batches, and unchanged frozen
source/harness/oracle hashes. The [summary](E15-cost-summary-v2.json) retains timing
ranges, work and both allocation samples. Raw [timing](E15-cost-time-v2.jsonl) and
[allocation](E15-cost-memory-v2.jsonl) observations remain reproducible. No session
hit a resource ceiling; the longest child took 514.44 seconds against its 1,800-second
bound. This cost comparison is complete; the broader directions remain open.

## Protocol and validity

Code revision `049f53f`, [v2 manifest](E15-cost-manifest-v2.json), and
[registration](../registrations/E15.md) fix the corrected protocol. Each fresh process
prepares a ruleset/backend once and runs three identical queries with fresh search
state. Retained serialized outputs and scalar measurement metadata remain between
queries. There is no cross-query answer cache or varied notebook workload.

The 135 configurations cover 25 workloads under ungrouped async search, with three
equality services: direct, scan-net and counted-net. Five workloads additionally
exercise FIFO and round/async grouping controls. Every configuration has one warm-up
before all five measured untraced sessions. Cold totals include source decoding/rule
preparation, backend construction, initialization, evaluation, output serialization
and release. The third-query total reuses preparation only. First-answer latency ends
at trusted publication and does not include later serialization/release.

Search release precedes independent answer validation, and validator garbage is
collected outside measured phases before the next query. The [diagnostic protocol](E15-cost-diagnostic.md)
did not isolate that boundary; its timing observations are not pooled with v2. The
behavioral regression checks both release and validation-garbage isolation. At the
measurement revision, all 28 scheduling and 22 net tests passed and an independent
read-only review found no remaining blocking attribution defect.

Release includes dropping search and an explicit full Python collection. That
collection traverses other live process objects too; it is a measured cleanup policy,
not a count of engine-owned garbage alone. Imports, initial request/oracle preparation,
validation and measurement bookkeeping are outside component totals; whole child
time is retained separately. Traced execution times will not be performance evidence.

## Complete-query runtime

Cold totals below are milliseconds, median [minimum, maximum] over five untraced
fresh-process sessions, async without grouping.

| Workload | Direct | Scan net | Counted net |
| --- | ---: | ---: | ---: |
| opaque-64 | 6.20 [5.56, 6.74] | 222.20 [198.94, 227.66] | 229.56 [228.58, 240.73] |
| duplicates-6 | 8.97 [8.48, 11.59] | 52.43 [50.58, 54.15] | 53.03 [47.82, 54.84] |
| SK identity | 11.59 [10.94, 11.90] | 1,211.14 [1,096.86, 1,228.39] | 1,157.09 [1,130.99, 1,254.50] |
| SK duplication evaluation | 34.52 [30.82, 34.84] | 16,086.44 [15,503.10, 16,612.95] | 16,260.79 [15,638.98, 16,777.40] |
| SK ignored hole | 47.12 [45.41, 51.15] | 26,324.70 [25,423.89, 27,697.24] | 27,178.94 [25,764.21, 27,909.84] |

For all 45 matched workload/policy pairs, direct's cold range is below both net
ranges. Counted versus scan has 41 overlapping cold ranges and four separated in
scan's favor: lambda identity async ungrouped, SK identity FIFO, lag-16 async ungrouped,
and opaque-64 async ungrouped. There is no separated counted cold-total benefit.
The same counts hold for three-query amortized totals. Overlap is inconclusive,
not an equality claim; these finite ranges are not universal speed guarantees.

Preparation amortization cannot remove the dominant net execution cost. SK ignored
hole prepares either net backend in roughly 2 ms, while its third query still takes
26,031.54 [25,607.30, 27,248.51] ms for scan and
27,357.30 [26,893.19, 27,691.55] ms for count. Direct's third query takes
47.88 [45.88, 50.90] ms. The [status gate](E15-status.md) establishes roughly 21%
fewer total actions on the larger SK cases; these timings show why action reduction
alone cannot select the runtime service.

## Scheduling and grouping retain different benefits

On duplicates-6, async grouping lowers cold totals to 1.60 [1.47, 1.74] ms for direct,
4.34 [3.83, 4.37] ms for scan, and 4.29 [4.16, 5.00] ms for count. Each grouped range
is below its ungrouped range in the table. Round grouping also separates favorably
for all three services. Exact lineage preserves the raw alternatives while avoiding
repeated work for identical states.

On lag-64 without grouping, cold first-answer latency changes from round to async:
1.24 [1.14, 1.29] to 0.90 [0.81, 0.94] ms for direct;
28.71 [26.44, 31.30] to 3.63 [3.33, 3.91] ms for scan;
30.65 [27.48, 31.97] to 3.67 [3.38, 3.90] ms for count.
Each latency range separates, while the corresponding complete-query ranges overlap.
This supports isolation of a small sibling from a large finite service, not a general
fairness theorem or faster total search. The recursive-prefix probe establishes a
finite witnessed answer, not exhaustion or a universal latency bound.

For opaque-64, grouping avoids no source jobs: the distinct output bindings prevent
whole-state merging. Grouped/ungrouped cold ranges overlap within both round and
async for all three services. Neither work counts nor timings establish the desired
post-choice sharing across these different states.

## Storage and remaining investigation

The table gives the maximum traced allocation outside validation, in KiB, for async
ungrouped sessions. Both repetitions agree to the displayed precision except where
a range is shown. This peak includes preparation and measurement records, not just
live source state.

| Workload | Direct | Scan net | Counted net |
| --- | ---: | ---: | ---: |
| opaque-64 | 210.00 | 1,046.26 | 1,056.66 |
| duplicates-6 | 158.62 | 1,052.42 | 1,103.00 |
| SK identity | 89.19 | 3,785.90 | 3,835.06 |
| SK duplication evaluation | 113.64–113.65 | 20,076.42 | 20,208.98 |
| SK ignored hole | 127.95 | 27,748.40 | 27,811.98 |

Direct has the lower peak range in all 45 matched pairs. Counted peak is higher
than scan in 17 pairs; the other 28 overlap. No counted peak reduction is established.
Async duplicate grouping lowers peaks to 33.38 KiB for direct and 720.62 KiB for
either net service; round grouping reaches the same peaks. Net preparation limits
the resulting whole-session peak even when repeated source work is avoided.

After the third ignored-hole query releases search, traced live allocation is
38.90 KiB for direct, 229.44 KiB for scan and 238.77 KiB for count. This includes
prepared data, retained outputs and measurement records. It shows that most of the
large measured peak is transient; three identical queries cannot establish long-session
storage stability. The full phase records distinguish release from subsequent validation.

Tracemalloc starts after imports and initial request/oracle preparation and measures
Python allocations, not total process memory. Ignored-hole process RSS high-water
marks are 15,872 KiB for direct, 83,916–85,612 KiB for scan and 79,452–80,052 KiB for
count. The lower counted RSS here coexists with higher traced peak; these different
metrics and two observations do not establish a general physical-memory advantage.

This result argues against assuming that this encoded Python equality service is a
performance improvement. It does not compare a native compiled net engine, routed
stores, compact IDs, graph-slot reclamation or a different bridge. In particular,
the current bridge copies/scans service counters after each one-action advance;
source inspection identifies that administrative work, but no timing ablation has
isolated its contribution.

Feasible follow-ups remain explicit: operation projection and active sharing across
different states, representation-matched direct controls, service-boundary costs,
storage/reclamation, broader applications and independent parallel execution. More
repetitions of this matrix alone would not test those mechanisms. The overall
experimental goal remains active.

Reproduce source fixtures with `cargo run -q -p chr-symbolic-fixtures --example export_cases`.
Run `python research/chr-scheduling/run_net_measurements.py FIXTURES time` and then its
separate `memory` batch under the registered bounds. Run
`python research/chr-scheduling/summarize_net_measurements.py TIME MEMORY FIXTURES DIAGNOSTIC_TIME` to validate the
matrix and produce its summary. The manifest fixes the exact fixture and code hashes.

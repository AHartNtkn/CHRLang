# E15 diagnostic cost batch: validation before collection

These observations use a protocol in which oracle validation precedes timed full
collection. The independent review found that recursive validator closures leave
cyclic garbage, so release-inclusive totals include some validation work. They do
not isolate the intended search-release cost. Preserve these data as diagnostic;
the corrected comparison is registered in E15 and must be rerun.

The timing batch has 810 complete sessions and passes 2,430 bidirectional answer
checks. The allocation batch was deliberately interrupted for protocol repair after
138 completed sessions; its running child has no completed observation. This is
neither a source-program failure nor a resource-limit exclusion.

The following numerical account describes that diagnostic protocol. Its timing
claims are not confirmation under the corrected release-before-validation protocol.

## What was compared

The three services are the ordinary finite-tree unifier, a net unifier that scans
allocated slots for unfinished controllers, and the same net unifier with a maintained
controller count. Net graph construction, reduction, complete readback and caller
publication remain present in both net variants. The reference interpreter is independent.

All 25 prior cost workloads run with async scheduling and no whole-state grouping.
Five workloads additionally run under FIFO, round without/with grouping, and async
with grouping. These cover large-operation latency, identical alternatives, different
outputs with common subsequent equations, a recursive sibling, and SK evaluation.
The [registration](../registrations/E15.md) fixes the cases, five timed repetitions,
two allocation repetitions, ordering, bounds and interpretation criteria.

Each process prepares one immutable ruleset/backend and runs the same query three
times with fresh search and observer state. Serialized outputs are retained; search
state is released after every query. This measures preparation reuse, not cross-query
answer caching or a varied notebook session.

A cold-query total is the sum of source/rule preparation, backend preparation, query
initialization, evaluation, serialization and release. Oracle decoding, validation,
measurement bookkeeping and interpreter imports/startup are outside that sum; whole
child duration is also recorded. Allocation-instrumented times are not timing evidence. Timed release also includes validator garbage under this diagnostic protocol.
These boundaries differ from the earlier E09 cost receipt, so their timings are not pooled.

## Complete timing evidence

Direct cold-query ranges are below both net ranges in all 45 matched workload/policy
pairs. Scan/count cold-query ranges overlap in all 45 pairs. Their three-query amortized
ranges overlap in 43 pairs; scan is lower in `lag-64` with round grouping and `opaque-16`
without grouping. No counted-status range is lower. Overlap leaves small effects
unresolved; it does not prove equal performance.

The following are async, without whole-state grouping. Values are median [minimum,
maximum] milliseconds from five untraced processes, including cold preparation and release.

| Query | Direct | Scan net | Counted net |
| --- | ---: | ---: | ---: |
| opaque-64 | 6.11 [5.53, 6.84] | 219.56 [206.56, 224.26] | 228.27 [209.24, 235.27] |
| SK identity | 10.58 [10.35, 11.49] | 1,202.72 [1,096.51, 1,248.36] | 1,143.66 [1,141.61, 1,237.95] |
| SK duplication | 33.94 [30.54, 36.89] | 16,262.36 [15,921.78, 16,619.89] | 16,692.82 [15,881.79, 16,794.69] |
| SK ignored hole | 47.13 [44.93, 49.47] | 27,161.39 [26,512.96, 27,864.27] | 26,733.28 [25,518.49, 27,942.78] |

Preparation reuse does not remove the larger net application cost. For the ignored-hole
query, median preparation is about 1.9 ms for either net service, while third-query
cost is 26,729 ms for scan and 27,308 ms for count. Direct third-query cost is 47.37 ms.
The recurring service work dominates this application; amortizing compilation cannot
account for an orders-of-magnitude improvement here.

Identical-alternative grouping has a separated timing benefit with every service.
On `duplicates-6`, async cold totals fall from 8.49 [7.83, 9.26] to 1.63 [1.48, 1.80] ms
for direct, from 50.65 [45.74, 53.71] to 4.03 [3.90, 5.85] ms for scan, and from
52.24 [46.93, 53.44] to 4.41 [4.06, 4.88] ms for count. This is exact whole-state reuse,
not reuse across different answers.

Async delivers the finite sibling earlier in `lag-64`. Cold first-answer medians
[range] change from round to async as follows: direct 1.16 [1.14, 1.28] to
0.86 [0.81, 0.91] ms; scan 28.74 [26.36, 31.50] to 3.39 [3.12, 3.60] ms; count
30.82 [28.37, 32.10] to 3.45 [3.38, 3.66] ms. These are latency results for this
finite probe, not a general fairness theorem or a total-throughput ranking.

Different output states still prevent reuse in `opaque-64`. Grouped and ungrouped
timing ranges overlap for all three services; no source jobs are avoided. The earlier
work-count evidence and these timings therefore do not supply the desired sharing
of operations across heterogeneous contexts.

## Interpretation and next discriminating work

Lower action counts did not establish a runtime improvement for counted status.
Action categories have different host costs, and the complete measured service includes
codec, mutation, scheduling and observation overhead. This comparison does not isolate
their individual elapsed-time contributions. It does establish that the prior roughly
21% action reduction cannot be reported as a measured end-to-end speedup.

These Python results do not rank native interaction-net implementations or different
encodings. The current whole-substitution boundary repeats conversion and encoded
store work; projected operations, routed stores and other representations remain open.
Additional repetitions might distinguish a small scan/count effect, but would not answer
the larger open question of sharing computation across different source states.

The next proposed experiment distinguishes projected direct equality, completed-result
memoization and sharing an operation already in progress. It retains caller-local
substitutions and separates projection, representation and service reuse. The
[protocol analysis](../../goals/chr-experiments/notes/T015-inflight-equality.md) records
controls, counterexamples, lifetime rules, primary sources and applicability limits.
Its semantic gate is not implemented yet. No source contract or production architecture
is adopted by this receipt, and the broader research goal remains active.

## Reproduction and audit

The original harness is committed at `ca57007`; the [original manifest](E15-cost-manifest.json)
records its hashes and fixture. Raw files are [810 timing sessions](E15-cost-time.jsonl)
and [138 completed allocation sessions](E15-cost-memory.jsonl). Their semantic/action
observations remain diagnostic controls for the corrected run. The interrupted child
was deliberately stopped for protocol repair and produced no completed row.

Current measure_net.py implements the corrected protocol; reproducing these diagnostic
phase boundaries requires the original committed harness. The revised E15 registration
uses distinct v2 output names and requires all configurations again. Do not pool the
protocols or interpret the interrupted suffix as a measured failure/exclusion.

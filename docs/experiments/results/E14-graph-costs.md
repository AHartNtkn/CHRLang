# A6 repeated observation costs

Graph observation has a bounded runtime benefit on repeated unary answers and a
bounded cost on distinct DAG8 answers under the registered criterion. Most ranges
overlap. Allocation savings repeat exactly, but do not establish a general runtime
or peak-memory benefit. Keep eager observation as a credible alternative.

[Registration](../registrations/E14-graph-costs.md), [raw runs](E14-graph-costs-v1.jsonl),
[manifest](E14-graph-costs-v1-manifest.json), [audit and all comparisons](E14-graph-costs-v1-audit.json),
[frozen inputs](E14-graph-costs-v1-inputs.tar.gz).

All768children pass:128warm-ups,448timed and192separate allocation runs. The
16workloads and four policies are shuffled within complete blocks using the
registered seed. All source/observation checks pass; allocation profiles repeat
exactly across three samples and match all64pilot profiles. No child reaches a
bound; maximum wall diagnostic is0.0127s.

## Same-comparator contrast

This comparison holds the rollback comparator algorithm fixed. Ratios below are
the median of seven same-block graph/eager-tree cold-time ratios. “Separated” means
non-overlapping observed ranges, not a population confidence interval. An overlapping
range remains inconclusive even when its median favors one policy.

| Workload | Median paired ratio | Observed ranges |
|---|---:|---|
| DAG4 repeat |0.781|overlap|
| DAG4 distinct |1.534|overlap|
| DAG8 repeat |0.623|overlap|
| DAG8 distinct |1.650|graph higher, separated|
| Unary16 repeat |0.787|graph lower, separated|
| Unary16 distinct |1.308|overlap|
| Unary64 repeat |0.581|graph lower, separated|
| Unary64 distinct |1.310|overlap|
| Symmetric4 repeat |1.006|overlap|
| Symmetric4 distinct |1.048|overlap|
| Symmetric8 repeat |1.065|overlap|
| Symmetric8 distinct |1.024|overlap|
| Retention64 repeat |1.066|overlap|
| Retention64 distinct |1.119|overlap|
| Retention1024 repeat |0.888|overlap|
| Retention1024 distinct |0.887|overlap|

Cold-plus-disposal gives the same classification. All16first-answer comparisons
in this contrast overlap. Lower allocation traffic on repeated DAG8 does not
satisfy the registered runtime separation criterion. Distinct DAG8 avoids no
exports and adds graph resolution/snapshot work; this is a concrete adverse result.
The current comparator still traverses logically repeated substructure, so
mapping-sensitive reuse is a relevant next mechanism, not an established fix.

## Other controls and contrary evidence

Avoiding eager answer clones has five lower separated cold-time ranges, eleven
overlapping and none higher. Changing only eager comparator strategy has sixteen
overlapping cold ranges, despite the verified allocation differences. Comparing
graph observation against legacy eager comparison gives two lower, twelve overlapping
and two higher ranges: distinct unary64 is the additional higher case. Thus a
whole-policy benefit must not be attributed solely to export avoidance.

All64first-answer comparisons across the four contrasts overlap. A first answer
cannot benefit from suppressing earlier duplicate exports. There is no supported
first-answer speed claim from these runs.

The [pilot allocation table](E14-graph-cost-pilot.md) reproduces exactly. Graph
traffic falls on repeated streams and rises on all eight distinct streams.
Symmetric repeated cases can have slightly higher graph peaks despite lower
traffic. The tiny-answer cases retain a common source arena in every policy;
they do not settle cross-query observer lifetime costs.

Independent read-only review recomputed all256comparison classifications and
paired ratios, verified all64allocation profiles against the pilot, checked
validation-separated lifecycle arithmetic and all55frozen source/archive hashes.
No material blocker was found. Paired samples belong to the same randomized block;
they are not simultaneous runs. These short workloads still expose host variance.

## Bounded recommendation and remaining questions

Do not replace eager observation unconditionally. Retaining graph snapshots is a
viable experimental option where duplicate export is substantial; clone avoidance
is a distinct improvement available to eager observation. Confirm application
benefits before selecting a delivery policy.

A6 remains open for larger duplication/size sweeps to distinguish timing noise
from scaling, comparison reuse that preserves variable mappings, partial streaming,
cross-query retention with competent lifetime controls, and application workloads.
Those are feasible implementation/measurement questions. None requires waiting
for constructor relationalization or the direct named-choice graph entry.
The next execution slot investigates that newly clarified graph proposal; A6's
remaining questions retain their evidence and concrete follow-ups.

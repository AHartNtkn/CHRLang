# A6 observation lifecycle pilot

All128 pilot children pass full observations, source/comparator accounting and
ownership checks. Graph observation reduces allocation traffic on the repeated
streams and increases it on all eight distinct streams. The matched comparator
control shows why export avoidance and comparator improvements need separate
attribution. Single timing observations do not support a speed ranking.

[Registration](../registrations/E14-graph-cost-pilot.md), [raw records](E14-graph-cost-pilot-v1.jsonl),
[manifest](E14-graph-cost-pilot-v1-manifest.json), [audit](E14-graph-cost-pilot-v1-audit-v2.json),
[frozen inputs](E14-graph-cost-pilot-v1-inputs.tar.gz).

## Allocation findings

Requested bytes include source construction, engine preparation/execution,
observation and disposal, excluding independent validation. These two controls
use the same rollback comparator: eager trees export every raw answer; graph
comparison exports recognized answers only. Each stream has32raw completions.

| Workload | Eager trees, matched comparator | Graph comparison |
|---|---:|---:|
| DAG4 repeat |105,558|54,322|
| DAG4 distinct |167,768|171,128|
| DAG8 repeat |879,446|84,210|
| DAG8 distinct |939,960|943,320|
| Unary16 repeat |80,701|51,630|
| Unary16 distinct |108,079|111,439|
| Unary64 repeat |181,901|79,918|
| Unary64 distinct |215,983|219,343|
| Symmetric4 repeat |147,042|128,426|
| Symmetric4 distinct |177,220|184,420|
| Symmetric8 repeat |271,130|238,594|
| Symmetric8 distinct |297,284|308,324|
| Retention64 repeat |80,917|76,057|
| Retention64 distinct |114,999|118,359|
| Retention1024 repeat |588,785|583,925|
| Retention1024 distinct |595,971|599,331|

DAG8 repeat reduces allocation calls from17,315 to1,382. However lower allocation
traffic is not automatically lower peak memory: symmetric8 repeat has100,290bytes
peak above baseline for eager trees and100,670for graph comparison. The graph
index releases1,834bytes on disposal; ordinary delivered answers remain retained.
Distinct workloads pay snapshot/index overhead without avoiding delivery exports.

The four controls expose another mechanism. On symmetric8 repeat, legacy eager
comparison requests441,514bytes; the same eager export policy using rollback
comparison requests271,130; graph comparison requests238,594. Much of this
allocation reduction therefore comes from comparator strategy, before avoiding
exports. The graph/tree rollback controls have identical first four comparator
work counters. The two legacy eager controls likewise agree on comparison work,
while clone ownership changes allocation.

## Measurement scope

The driver measures source construction/preparation, first delivery, exhausted
delivery, and separate index/engine/output disposal. Expected answers exist outside
the cold interval. Outputs survive index and engine disposal and are then validated.
Validation temporaries leave no retained memory; after output disposal all measured
ownership returns to the baseline. Meter times are not runtime evidence.

Traffic is computed from prevalidation allocation readings plus the separate
output-drop interval. Post-validation cumulative readings include oracle allocation
and are not pure lifecycle traffic or peak. The maximum child wall diagnostic is
0.00961s; none reaches a resource bound. Program and observation work counters agree
between System and Meter. Allocation measurements record actual constructor/string/
vector costs that resolution counters alone omit.

All policies retain the source arena during these single-query runs. Tiny answers
with a consumed payload therefore measure common arena ownership and extra observer
metadata; they do not isolate graph-specific cross-query arena retention. Repeated
symmetric streams have two recognized classes, not one. Distinct tags may reject
before residual matching. Both qualifications are registered.

The initial auditor's prefix selector included `first_ns` among deterministic
snapshot fields. The corrected auditor excludes timing fields, as required by the
registration. The frozen original auditor and all53inputs are verified in the
archive; the current auditor's hash is in the receipt. No workload, measurement or
semantic criterion changed. Independent read-only review checked the lifecycle
arithmetic, key allocation figures and all distinct-stream increases, finding no
measurement invalidity.

## Next work

Randomized repeated timings and separate repeated allocation runs can now assess
which observed cost differences persist. Keep all16workloads and all four controls.
Cross-query retention, partial streaming, larger graphs, mapping-sensitive traversal
memoization and application workloads remain separate feasible investigations.
A6 is open; this pilot does not select a production observer or close alternatives.

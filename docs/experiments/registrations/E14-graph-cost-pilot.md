# A6 observation lifecycle pilot

Status: v1 pilot registered before execution. The exact fixture, lifecycle and
185-column schema below are frozen. V2 semantic gates pass. This pilot sizes a later repeated cost comparison; single timings do not rank
architectures.

## Distinguishing hypotheses and controls

Graph comparison before export may avoid allocating duplicate answer trees but
still traverse their logical expansions. Retained completion bindings and an arena
can cost more than retaining small materialized answers. A competent eager observer
may recover much of the benefit merely by avoiding unnecessary clones. Compare:

1. `eager_clone`: same FIFO Machine execution, ordinary export at every completion,
   then clone into AnswerSet before deciding duplicate status, matching existing
   delivery behavior. Recognized original answers remain available to the caller.
2. `eager_compare`: ordinary export at every completion; compare against delivered
   answers and keep the new answer only when distinct, without an additional clone.
3. `eager_graph_compare`: ordinary export at every completion, with delivered
   answers as the seen store, using an allocation-free tree view and the exact same
   rollback comparator as the graph policy. This separates the comparator strategy
   from the export/representation policy.
4. `graph_compare`: capture roots and immutable bindings; compare against recognized
   snapshots; export only recognized new answers for normal caller delivery. Dispose
   duplicate snapshots. Retain unique snapshots for subsequent exact comparisons.

Identical source rules, FIFO order and full residual observations are local
controls. No production semantics are selected. Every output is an ordinary full
answer, and exact known equivalence is the only dedup criterion.

## Proposed finite workloads

Sixteen workloads, each32raw alternatives from an explicit balanced source OR:
DAGdepth4/8, unary length16/64, symmetric residuals4/8vertices, and tiny answers
with64/1024unreachable distinct nullary payload nodes. Each has repeated and distinct
variants. DAG equations Xi=pair(Xi+1,Xi+1) execute before the split and terminate
in a free variable; unary structure also remains finite. Distinct tags follow the
primary payload in output order. Tiny workloads consume their payload before
splitting, retaining only a small answer while the machine arena keeps the payload.

Repeated symmetric cases alternate one n-cycle and two n/2-cycles,16of each:
two recognized classes. Other repeated families have one recognized class.
Distinct variants have32classes. Distinct tags can short-circuit residual
comparison in the symmetric case; report that rather than claiming it tests a
hard residual comparison. The exact source producer and analytic expected answers
must be frozen before pilot execution.

## Required lifecycle measurement

Use separate System and allocation-meter binaries, never compare meter times as
runtime results. One fresh child per workload/control/instrumentation:128children.
All64System cells precede64meter cells, in frozen generator then control order.
Per child30seconds/1GiB plus a declared finite source-step limit. No concurrent
builds, tests, profiles or unrelated experiment workloads during the timed batch.

Generate expected observations outside measurements. Include actual program/query
construction and Machine preparation in cold measurements. Record preparation,
first fully delivered answer and exhausted delivery; preserve outputs throughout
subsequent dedup-state and engine disposal. Measure dedup-state, machine and output
destruction separately. Perform full observation validation outside timed work,
and make any validation gap explicit. Do not subtract gaps where engine work can
continue; these drivers are synchronous and complete before disposal.

Nonallocating counter snapshots separate source counts, historical eager-export
contribution, capture, exact comparison and delivery export resolution. Ground
constructor/string/vector materialization is not counted by resolution counters;
allocation traffic, live/peak bytes and lifecycle time must capture it. Report
baseline live memory and absolute readings so input/oracle ownership is visible.
Do not infer a byte peak from retained-record counts. Record host/runtime/compiler,
source/binary hashes and actual commands before the first child; use exclusive
versioned records and preserve failures.

Validation must compare full delivered observations, raw/recognized counts,
exhaustion, source counters excluding attributable eager export, and actual graph
export count (recognized only). All three eager modes export32times. Graph capture
occurs32times; retained snapshots equal recognized classes at exhaustion. Require
System/meter semantic and deterministic work agreement. Cost repetition and
hash/host variability assessment need a subsequent registered protocol.

The tiny-payload single-query probe retains the live machine's arena in all four
policies. It therefore cannot isolate additional arena retention caused by graph
observation alone. It can expose snapshot/binding/index overhead and absolute
lifecycle ownership. Keeping an observer across completed machines or queries
needs a separate lifetime comparison, including the option to retain already
materialized delivery answers. Do not attribute common source arena retention to
only the graph observer.


The pre-run comparator review exposed a confound between legacy eager mapping
copies and graph rollback. The fourth control addresses it before any cost data:
`eager_graph_compare` versus `graph_compare` holds the comparator algorithm fixed.
Legacy eager controls remain useful to isolate clone ownership and mapping strategy.
Require identical first four rollback-comparator counters between tree and graph
representations on the same workload, alongside equal source observations. Different
resolution/binding counters are expected from their representations.


## Frozen execution and field schema

Generator order: dag4 repeat/distinct, dag8 repeat/distinct, unary16
repeat/distinct, unary64 repeat/distinct, symmetric4 repeat/distinct, symmetric8
repeat/distinct, retention64 repeat/distinct, retention1024 repeat/distinct.
For each, policies are eager_clone, eager_compare, eager_graph_compare,
graph_compare. Execute this64-cell order with System, then with Meter:128children.
The producer and analytic oracle are exactly `graph_cost_cases.rs`; only its
`build_source` runs inside cold time. DAG leaf is free; unary terminates in z.
Source step budget100,000 for every cell, fully exhausted with raw32.

TSV has185 columns: five identity/validation fields; seven nanosecond intervals
(preparation, first delivery, cold exhausted delivery, index drop, engine drop,
validation, output drop); eight allocation readings of four fields each
(calls/requested/live/peak at baseline, prepared, first, exhausted, index dropped,
engine dropped, immediately before output drop, output dropped); and three fixed
47-field source/observation snapshots at prepared/first/exhausted. Each snapshot
has19 source fields, three eager-export fields, three capture fields, four legacy
comparison fields, six rollback-comparison fields, six graph-export resolution
fields and six raw/export/delivery/index/frontier/queue scalars. `index_keys` means
separately owned keys: zero for both delivered-tree comparison controls.

Only prevalidation readings describe uninterrupted lifecycle allocation traffic.
Compute allocation traffic as engine_dropped minus baseline plus the separate
output_dropped minus output_drop_before interval. Cumulative calls/requested/peak
after validation include oracle temporaries and must not be reported as pure
lifecycle cost. Use engine_dropped peak for the prevalidation peak; record the
absolute baseline. Assert oracle temporaries leave no live memory and all measured
source/index/output ownership returns to baseline after output disposal. Expected
answers and command arguments remain baseline-owned until after measurement.

The cold interval includes prepared/first numeric diagnostic snapshots. Once source
execution ends there is no asynchronous work. Destruction intervals exclude numeric
snapshot/validation gaps; report their sum with cold only as an explicitly defined
synchronous lifecycle measure. No warm session/pool is implied.

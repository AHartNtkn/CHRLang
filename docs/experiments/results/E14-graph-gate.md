# A6 exact observation directly over graphs

The borrowed graph comparator passes4,290 independent comparisons plus exact
replay. It also preserves all64 registry completion contracts through actual
persistent execution. This establishes a bounded semantic entry for comparing
before tree export. It does not establish a performance or memory benefit.

## Independent comparison gate

[Registration](../registrations/E14-graph-gate.md), [raw groups](E14-graph-gate-v1.jsonl),
[manifest](E14-graph-gate-v1-manifest.json), [audit](E14-graph-gate-v1-audit.json),
[frozen inputs](E14-graph-gate-v1-inputs.tar.gz).

There are192 template/layout/alias/orientation cases, two common-root cases, and
4,096 directed-graph pairs. All4,290 comparisons agree with independent expected
results;393 are positive and3,897 negative. Six isolated processes cover the
three groups and replay; all result and observation counters replay exactly.
The largest child takes0.1575s under the30s diagnostic bound.

The oracle enumerates complete variable bijections over original plain fixtures
and compares ordered named outputs and sorted residual multisets. The directed
graph cases also check adjacency under every vertex permutation. Neither oracle
calls the candidate comparator or persistent exporter. A synthetic DAG adapter
varies constructor interning and variable alias links. Tests cover common handles
under different bindings, different physical sharing with equal denotation, joint
output/residual aliases, multiplicity, rollback and connected residual structure.

The comparator uses generic borrowed views rather than a dependency on persistent
execution. It has no handle-equality shortcut or node-pair memoization. Its
constructor traversal can therefore still unfold repeated substructure logically
many times; avoiding allocated trees does not prove avoiding traversal.

## Actual source completion gate

[Registration](../registrations/E14-graph-source-gate.md), [raw cases](E14-graph-source-gate-v1.jsonl),
[manifest](E14-graph-source-gate-v1-manifest.json), [audit](E14-graph-source-gate-v1-audit.json),
[frozen inputs](E14-graph-source-gate-v1-inputs.tar.gz).

All64 E00 cases pass in128 isolated children including replay:147 raw completions,
145 recognized observations,62 exhausted cases and the two expected unfinished
prefixes. Paired eager/borrowed machines agree on every event, complete raw answer,
duplicate decision and full expected observation. Retained snapshots re-export
unchanged after later source execution. The driver applies recognized-answer
limits rather than raw-completion limits.

Completion snapshots retain named roots, immutable bindings and residual root
arrays. They borrow their originating machine's arena for comparison/export,
without retaining pending source work or propagation history. A private owner
token prevents applying a snapshot to another machine's indices. Existing eager
consumers use the same internal source transition and preserve their public
completion behavior.

Source counters agree after subtracting explicitly recorded eager-export
contributions to dereferences/storage visits. Capture, graph comparison and
validation export resolution are separately counted. Constructor/string/vector
materialization is not counted by those export counters; a ground export can
record zero resolution work. Lifecycle allocation/time measurements must cover
that work. This gate deliberately exports every
raw completion for equality checking, including duplicates; its validation work
is not a measurement of a graph delivery policy that exports only new answers.

## Remaining investigations

A6 remains open. A three-control lifecycle comparison should test current eager
delivery, competent eager comparison before cloning, and graph comparison before
export. It must include snapshot construction, retained binding/arena memory,
dedup comparison, actual user output materialization and destruction. Duplicate
streams with deep DAGs may avoid substantial allocation; distinct streams and
small answers retaining a large arena may expose overhead. Those are hypotheses,
not established results. Mapping-sensitive memoization and reclamation are
separate feasible follow-ups; neither is required before this first cost contrast.


Workspace all-target tests, all-target Clippy with warnings denied, and formatting
checks pass. Independent read-only review found no material semantic blocker and
identified the export-counter scope qualification above.

## Current v2 gate: shared comparator for trees and graphs

The child-access interface now supports an allocation-free view of ordinary trees,
allowing export policy to be compared using the same rollback algorithm. V2 adds
tree/graph, graph/tree and tree/tree checks to every template and directed-graph
case:17,154 comparisons per pass,1,569positive and15,585negative. All observations
and counters replay exactly. The64-case source gate also passes128children with
identical deterministic records to v1. [Comparator audit](E14-graph-gate-v2-audit.json)
and [source audit](E14-graph-source-gate-v2-audit.json) accompany versioned raw,
manifest and frozen-input files. Workspace tests, Clippy and formatting pass.
The [four-control pilot](E14-graph-cost-pilot.md) now tests the resulting cost contrast.

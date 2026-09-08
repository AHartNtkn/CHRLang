# Fork-owner allocation evidence (T051)

Arena ownership earns one bounded copy-on-write comparison. It accounts for
36.6% of inclusive split-requested bytes in the large mostly-failing case, and
all observed post-fork segments preserve its nodes and predicate dictionaries.
This identifies avoidable copying in the measured execution path; it does not
establish a lifecycle speedup or make arena sharing a production recommendation.

## Evidence and scope

The [prospective registration](../registrations/R03-fork-owner.md) and validated
instrumentation were committed at `19b7270` before execution. All 72 processes
completed across 36 cells, with 2,280 independently validated raw answers. Every
cell's two fresh processes agree exactly in non-time counts, requested traffic
and live gauges. All baselines restore; no missing jobs, cutoffs, malformed rows,
accounting errors or frozen-source/binary changes occur. Maximum process duration
was 0.895 seconds and maximum query service was 266,828 ticks, within the registered
30-second/1-GiB process and 20-million-tick query bounds.

[Raw runs and manifests](r03-fork-owner/), [audit](r03-fork-owner/audit.json),
[complete summary](r03-fork-owner/summary.json) and
[implementation gates](r03-fork-owner-gate/) retain the evidence. The independent
review checked disjoint owner windows, unchanged outer peaks, both-child segment
reset and failed-work accounting. The root ran the workspace suite and ordinary
counter-free smoke, and challenged the analyzer with incorrect owner traffic,
missing continuations, shared-handle allocations, answer loss and malformed rows.

Requested heap traffic is allocator demand, not RSS. Durations are diagnostic
accounting only; there is no timing comparison to T050 or another architecture.
The closed-occurs correction is present in this freeze.

## Attribution

At n512/a64/q1 mostly-fail, inclusive split service requests 40,956,482 bytes.
The arena contributes 15,006,159 bytes, or 36.64%; its node vector and owned node
contents contribute 5,325,327, and its intern table and keys contribute 9,581,103.
Closedness and predicate dictionaries account for the rest of the arena total.
The complete measured lifecycle requests 44,899,225 bytes, so the current arena
clone traffic is 33.42% of that total. This fraction is not an achievable timing
improvement and does not charge any proposed sharing mechanism.

Other owners remain consequential: the index contributes 6,300,000 bytes,
watchers 5,985,504, dependencies 5,368,608, occurrence keys 4,078,368, occurrence
store 3,511,872 and pools 640,080. Outputs contribute 8,316. The explicit split
remainder is 57,575 bytes. Shared rules, regions, dispatch and binding handles
request zero bytes as expected. Owner sums plus remainder reconcile exactly to
inclusive split service in every sample.

Cold-query owner fractions follow; all reuse samples are retained in the summary.
Size zero still contains source alternative syntax and is not an empty-arena case.

| n | alternatives | outcome | arena requested bytes | inclusive split bytes | arena share |
|---:|---:|---|---:|---:|---:|
| 0 | 1 | mostly-fail | 0 | 0 | — |
| 0 | 1 | all-success | 0 | 0 | — |
| 0 | 8 | mostly-fail | 14,826 | 26,453 | 56.05% |
| 0 | 8 | all-success | 14,826 | 43,253 | 34.28% |
| 0 | 64 | mostly-fail | 828,450 | 917,525 | 90.29% |
| 0 | 64 | all-success | 828,450 | 934,325 | 88.67% |
| 64 | 1 | mostly-fail | 0 | 0 | — |
| 64 | 1 | all-success | 0 | 0 | — |
| 64 | 8 | mostly-fail | 212,247 | 573,202 | 37.03% |
| 64 | 8 | all-success | 212,247 | 590,002 | 35.97% |
| 64 | 64 | mostly-fail | 1,910,223 | 5,143,250 | 37.14% |
| 64 | 64 | all-success | 1,910,223 | 5,316,850 | 35.93% |
| 512 | 1 | mostly-fail | 0 | 0 | — |
| 512 | 1 | all-success | 0 | 0 | — |
| 512 | 8 | mostly-fail | 1,667,351 | 4,552,450 | 36.63% |
| 512 | 8 | all-success | 1,667,351 | 4,569,250 | 36.49% |
| 512 | 64 | mostly-fail | 15,006,159 | 40,956,482 | 36.64% |
| 512 | 64 | all-success | 15,006,159 | 41,130,082 | 36.48% |

## Continuations and candidate boundary

Across both repetitions, all 8,400 post-fork segments are mutation-free for arena
nodes and predicate dictionaries. There are no post-fork node misses, local hits
or predicate insertions. Actual inherited hits remain charged and counted.
No-choice controls have zero owner events and zero post-fork segment activity.

For n512/a64/q1 mostly-fail, the segment endpoints are 62 subsequent splits,
63 failures and one completion; all preserve the 1,025 inherited nodes. The
all-success control has 62 subsequent splits and 64 completions, also all
mutation-free. It performs 131,583 inherited hits versus 2,181 in mostly-fail.
Thus the evidence is stronger than a high hit fraction: both descendants can
reach their endpoints without detaching arena ownership. The diagnostic tests
separately prove that actual misses, local hits and predicate insertion are
recognized and sibling mutations remain independent.

The selected next comparison is lookup-before-detachment arena copy-on-write.
It can preserve stable IDs, immutable node contents and branch-local variable
bindings while sharing the five arena containers. Every insertion must detach
before mutation, and ordinary lookup must not detach. This adds reference-count
ownership and a mutation boundary; it does not require a trail, scheduler change,
new source semantics or shared mutable equality state.

The current fixture has no post-fork insertion, so it cannot measure the adverse
detachment cost. The next gate and registration must include immediate post-fork
construction with distinct sibling terms, full observation and disposal, alongside
these read-preserving cases and no-choice preparation controls. Compare current
ordinary-allocator counter-free complete paths; keep work/allocation diagnostics
separate. Reuse and no-choice costs must include the indirection/refcount overhead.

Whole-state copy-on-write is not selected: the occurrence store, indexes and
scheduling state mutate during ordinary continuation work. Persistent collections
or replay could avoid portions of the remaining cost but carry wider ownership
and operational obligations. One narrow arena comparison now has higher expected
decision value than either those expansions or the ready pure-carrier contraction
gate: it targets measured traffic and has a concrete nonmutation witness. If the
complete lifecycle benefit is absent, retain that contrary result and reassess
carrier contraction rather than expanding storage machinery by default.

T051 is complete with this causal selection. The broader architecture goal remains
active; this evidence neither ranks all search organizations nor closes the other
question dispositions.

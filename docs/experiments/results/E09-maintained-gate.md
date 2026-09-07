# A3 maintained matching: bounded source gate

Selective maintained prefixes preserve the tested source behavior and avoid repeated
matching in the small gate. The same retained representation with full invalidation
adds work against competent prefix recomputation. This supports a larger resource
investigation, not a runtime or architecture recommendation.

## Evidence

[V2 registration](../registrations/E09-maintained-gate.md),
[raw records](E09-maintained-gate-v2.jsonl), [pre-run manifest](E09-maintained-gate-v2-manifest.json),
[audit](E09-maintained-gate-v2-audit.json), and [frozen inputs](E09-maintained-gate-v2-inputs.tar.gz).
All80 isolated children pass: eight supported-store cells,24 integrated source
cells, eight additional hash-seed cells, followed by exact replay. The store gate
checks88 context projections per pass. The source gate checks every transition
against the independent tree control and full ground answers against an analytic
formula. Modes and quanta agree on source effects, source steps and full answers.
The largest child takes1.958s under the30s bound; this includes oracle work.
All40 scheduling unit tests also pass, including12 maintained-entry tests.

The candidate retains successful and failed extension tests, dependency indexes,
prefix rows and context masks. Complete selection still checks ordinary guards,
propagation history and live distinct occurrences. Regressions exercise suspended
alias chains, middle-head insertion, support-local consumption, distinct bindings,
fork isolation, successor invalidation, and safe rejection of abandoned updates.
The finite source fixtures exercise kept and consuming joins through a source
command producer, explicit choices and equality awakenings.

## Work observations

N4/R1, Q8, hash seed0. Totals include source, matching and administrative labels;
labels do not have equal runtime cost.

| Join | Alternatives | Prefix actions | Full invalidation actions | Selective actions | Full / selective match calls |
|---|---:|---:|---:|---:|---:|
| Keep |1|50,420|92,649|23,988|2,800 /275|
| Keep |2|98,147|182,458|45,117|5,509 /459|
| Consume |1|34,217|69,530|20,473|2,038 /233|
| Consume |2|65,741|136,220|38,087|3,985 /375|

Maintained modes peak at131 retained tests per context,262 across two contexts.
They retain90 tests per completed keep context and12 per consume context.
These are record counts, not heap measurements. Context completion currently
retains its snapshot/index metadata until the matcher is dropped.

Global row payloads can acquire additional support, and forks inherit tests.
New extension computations remain per context. Thus two distinct alternatives
still repeat much of the source and matching work; shared payload counts do not
establish sharing of those computations. Predicate pools are rebuilt after changes,
and discovery still visits retained prefixes and occurrence candidates.

## Replay diagnosis

[V1 raw evidence](E09-maintained-gate-v1.jsonl) passes every semantic check but
fails its exact-counter replay requirement. The [diagnostic](E09-maintained-gate-v1-replay-diagnostic.json)
records four pairs differing only in two invalidation-visit counters. Hash-dependent
set iteration controls whether an affected descendant is queued directly and again
through its ancestor. Both orders erase the same test closure; extra queued entries
are charged no-op visits. [V1 frozen inputs](E09-maintained-gate-v1-inputs.tar.gz)
preserve that run's implementation and protocol.

V2 keeps the algorithm, fixes seed0 for the main grid, and adds seeds1/2 for the
sensitive cells. Exact per-seed replay passes. Across seeds, only the identified
two administrative counters can differ; every other recorded deterministic result
agrees. A deduplicated work queue is a possible optimization, not required to
interpret these small differences. Future cost comparisons need hash-seed
sensitivity assessment when that nuisance approaches their uncertainty.

## Disposition and next work

A3 remains open. The [registered sizing experiment](../registrations/E09-maintained-sizing.md)
expands store size, rounds and alternatives while retaining adverse consuming
cases and all resource failures. Complete lifecycle memory/time, lazy witness
search, seed-ordered recomputation, join-key maintenance, multiway joins and sharing
new tests across contexts remain feasible. The [matching source assessment](A3-matching-directions.md)
explains why these are distinct architectural alternatives. This finite gate does
not establish general fairness, concurrent update correctness, cancellation recovery,
unbounded support, or production language scheduling.

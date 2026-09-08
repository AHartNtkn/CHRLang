# R03 arena ownership lifecycle registration (T052)

This registration freezes one comparison after the diagnostic allocation result
at `148c6dc`. Runs begin only after the source gate, runner and analyzer are
validated and committed. This is an experimental ownership choice, not a language
restriction or production selection.

## Hypotheses and decision value

Sharing the arena should avoid copied nodes, intern keys and dictionaries when
continuations perform inherited lookups. Contrary explanations are that lookup
indirection/reference counting dominates small or unique-owner cases, or that
immediate post-fork insertion pays essentially the same copying plus ownership
bookkeeping. Other mutable engine owners remain unchanged. A whole lifecycle
comparison can resolve whether avoiding the measured arena traffic is useful;
a copy-byte fraction alone cannot.

The prior diagnostic run identifies 36.6% arena split traffic in the large case,
and actual mutation-free continuations. One owner and one mutation boundary give
this test greater expected decision value now than the ready carrier-contraction
gate or wider persistent-storage/replay redesign. No further storage expansion is
implied by implementing the candidate.

## Source workloads and controls

`read` uses the independent state-preservation fixture: build n live payloads,
a right-recursive choice spine with a leaves, key equations selecting all leaves
or only the last, complete successful cleanup, duplicate aliased residuals and
joint outputs. No-choice a1 measures ownership overhead without splitting.

`insert` retains that payload and cleanup, but each selected leaf posts a
candidate whose fired source rule constructs `stamp(new(key,H,H))` before its
wanted-key equation. The term is created after the fork, including in failed
leaves; successful answers expose its exact key and alias structure. Complete
all-success output keys are exactly 0..a-1 once each; mostly-fail output key is 0.
This charges detachment even when insertion precedes failure. A separate semantic
gate covers divergent siblings, predicate insertion and re-fork after insertion.
Preparation cannot pre-intern the source-body construction. a1 measures insertion
with unique ownership. No workload weights are inferred.

## Exact configurations and manifest

For each ownership configuration, run n0/512 × a1/64 × mostly-fail/all-success ×
q1/4 × read/insert: 32 cells. Query i uses n+i%2 while reusing prepared rules.
Control builds own ArenaData directly. Candidate builds add `arena-cow`, storing
one Rc<ArenaData>, doing immutable lookup before controlled insertion detaches.
Bindings, matching, activation, source scheduling and observation are identical.

For each of the 64 configuration/cell combinations run one warmup, five primary,
one allocation and one work process: 512 fresh processes. Python Random seed 52052
shuffles the full configuration/cell batch separately within each mode, with modes
warmup, primary, allocation, work in that order. No conditional repetition or
resizing. Freeze source, registration, binary hashes, commands, build status,
environment and full prospective manifest before execution; verify hashes after.

Build six locked offline release artifacts in separate target directories:

| mode | control features | candidate features | defaults |
|---|---|---|---|
| primary | experiment | experiment,arena-cow | disabled |
| allocation | alloc-meter | alloc-meter,arena-cow | disabled |
| work | experiment | experiment,arena-cow | enabled |

Primary uses the ordinary allocator, with engine/kernel/observer counters disabled
and no fork instrumentation. Allocation and source-work diagnostics are separate
builds/runs. The allocator measures requested heap, not RSS. Each process uses the
minimum inherited available CPU, 30-second wall timeout and 1-GiB address-space
limit. Each query permits 20 million ticks. Builds each have 180 seconds; execution
has 1200 seconds after builds. Retain all failures, cutoffs, malformed rows and
missing jobs. Run allocator self-check before the matrix.

## Endpoints and acceptance

Measure preparation, query construction, setup, execution, first complete
observation, engine disposal, answer disposal and prepared disposal. Primary
comparison is complete lifecycle divided by query count, excluding only the
separately measured validation interval. Full independent validation happens after
retaining answers and disposing the engine. Insertion validation allocates a
temporary key-membership vector outside measured intervals; answer disposal follows
validation and therefore uses validation-warmed data in both configurations.
Candidate lookup, detach, mutation and
reference-count disposal are charged in their actual intervals.

Require exact full answer multiplicity/aliases/residuals, expected keys, failure
counts and exhaustion. Expected source applications are n+1+a+(n+1)*successes
for read and n+1+2a+(n+1)*successes for insert. Complete source-work records must
agree between ownership configurations. Diagnostic baselines restore exactly;
event-class traffic reconciles to service. Primary diagnostic fields are absent
or null as specified by the runner. Audit commands against the manifest, not only
configuration labels.

Report all 32 paired cells, five-repetition medians and full ranges, memory traffic,
peak requested heap, phase costs and first observation. Label a timing advantage
resolved only when the five-run ranges separate; overlapping ranges remain
unresolved. This is a conservative within-session screen, not a confidence
interval. Do not compare timings to the earlier diagnostic freeze.

If arena sharing resolves a lifecycle advantage in large read cases, retain its
bounded applicability alongside contrary no-choice, insertion or small cases.
If all relevant benefits overlap or lose, stop arena refinement and reassess the
carrier-contraction gate. Mixed outcomes require an evidence-backed applicability
statement, not an invented aggregate winner. A crossover follow-up needs a concrete
architectural decision it could change. Compilation costs are not credibly
isolated here; do not claim complete architectural lifecycle superiority.

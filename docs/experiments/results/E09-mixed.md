# E09 mixed scans, binding and admission batches

All 12 hand-derived source cases agree with the independent E11 tree control.
All 324 scheduler/quantum/batch configurations exhaust, preserve complete answers
and raw multiplicity, and replay byte-for-byte. Represented source-step counts
match the independent control in every configuration. The largest run consumes
101,973 actions, below the registered 5,000,000-action limit.

The cases exercise initially disabled constructor heads, pure guards enabled by
branch-local equations, a nonground guarded residual, and propagation history
across two identical explicit choices. They check joint output/residual aliases
and one propagated occurrence per lineage. This service rescans state; it does
not implement an incremental wakeup index. Candidate code imports no interpreter;
the harness uses the independent control and full-residual alpha checker.

## Findings at size 12, quantum 8

In the scan-lag case, async without grouping and batch 8 returns the small `a`
answer at action 790 and the large residual `b` answer at 11,449. Round without
grouping returns `b` at 11,384 and `a` at 11,421. Both execute 35 source jobs.
The round barrier holds the small branch's successor equation until the large
scan finishes; completed quiescent observations can still publish during that
round. This is a service-specific latency distinction, not a source answer-order
requirement.

There is contrary evidence to a blanket first-answer recommendation. In the
guard-binding case, round without grouping/batch 8 publishes the nonground `b`
answer at 42,118 and `a` at 57,352. Async publishes `a` at 53,679 and `b` at 57,354.
Round returns *an* answer earlier; async returns the particular `a` answer earlier.
The compared endpoint must therefore be stated. Both answers are trustworthy under
the same source contract, including the retained mark/output alias in `b`.

Batch size changes sharing eligibility. In the propagation-history case, identity
grouping with batch 1 cannot combine the two ready entries: async takes 53,188
actions and 24 source jobs. With batch 8, it takes 35,180 actions and 20 source
jobs representing the same 24 transitions. Both preserve two raw alternatives,
one unique observation, and exactly one `seen(a)` residual per lineage. Round
shows the same grouped result. Larger batches enable this reuse but do not supply
a universal latency or storage improvement.

## Important control limitation and next action

The deliberately unsuccessful three-head rule has predicate `q`; the stores
contain other predicates. This selector nevertheless enumerates ordered occurrence
tuples before testing heads. A predicate filter can eliminate that search. The
measured delay therefore demonstrates isolation from the current expensive finite
service, not the inherent cost of multihead CHR or a robust advantage against an
indexed selector.

The next necessary control is a yielding predicate-filtered selector preserving
the same first enabled occurrence tuple, compared on these cases and on workloads
whose head predicates do match but whose joins or guards reject. Charge pool/index
construction and binding-sensitive maintenance. Check transitions independently,
including alias/root handling and propagation history, before comparing policies.
This can distinguish avoidable selection work from residual scheduling effects.
It is feasible work and prevents closing the scan question now.

Heterogeneous support subdivision, operation sharing across distinct states,
longer synthesis, cached-key costs and net/second-service integration also remain
open. No production scheduler or language contract is selected by these probes.

Evidence: [raw results](E09-mixed.jsonl), [replay](E09-mixed-replay.jsonl),
[source and registration hashes](E09-mixed-manifest.json).

```sh
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/run_mixed.py
```

The harness validates all hand-derived cases against the independent control before
starting policy runs. Counts include selector, observer and grouping work; there
are no new wall-time or byte claims in this experiment.

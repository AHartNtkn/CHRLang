# E09 resumable exact observation gate

The yielding observer agrees with the independent recursive alpha-equivalence
checker on 4,096 generated observation pairs and 661 pairs of E00 expected/reference
answers, at each of quanta 1, 8 and 64. All 14,271 configurations pass and replay
exactly. There are 188 equivalent generated pairs and 145 equivalent E00 pairs.
Three focused tests additionally exercise joint aliases, residual multiplicity,
backtracking, late mismatch and rejection of unfinished observations.

`research/chr-scheduling/observer.py` explicitly traverses both term trees under a
joint variable bijection and searches residual permutations. Each candidate owns
its variable mappings, so failed matches cannot contaminate later alternatives.
The resumable stack preserves exact full-residual comparison. It neither projects
away residual constraints nor treats possible equivalence as proven equivalence.
Only the validation harness imports the independent checker.

Per quantum, generated pairs consume 32,748 actions; E00 pairs consume 5,083.
Actions charge term traversal, search, map/set copying and final publication.
They are repeatable work counts, not wall-time measurements or a complexity bound.
Residual permutation search remains potentially expensive, and Python container
allocation remains outside any hard real-time guarantee.

Reproduce after exporting E00 fixtures as in the source gate:

```sh
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-scheduling -v
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/check_observer.py < /tmp/chr-e09-cases.jsonl
```

Raw records: [gate](E09-observer-gate.jsonl) and
[replay](E09-observer-gate-replay.jsonl).

This gate enables resumable comparisons; it does not yet implement final-answer
queue service, deduplication indexing or admission backpressure. Policy runs must
keep observer input snapshots stable, charge comparisons and retained answers,
and distinguish evaluator progress from final-output latency. Better exact keys,
indexing and symmetric residual workloads remain useful E14 follow-ups; no closure
or policy selection follows from this correctness gate.

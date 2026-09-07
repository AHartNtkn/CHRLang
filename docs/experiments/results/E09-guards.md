# E09 certified early pure guards

All 732 configurations pass and replay exactly: 192 E00 source-transition checks,
324 mixed cases, 54 matching-predicate cases and 162 no-pruning controls. Source
work is invariant across quanta. The preceding 570-row prefix matrix also replays
exactly. Twenty-one tests pass, including later-head deferral, fresh guard-local
variables and a nonground stored variable that the guard must not bind.

A guard may reject a prefix only when every rule variable it reads has a value in
the prefix's pattern-binding map. That value may itself contain unbound source
variables. The guard uses the sealed substitution and nonbinding equality; later
head matching cannot change these fixed values. Unavailable rule variables cause
deferral, not failure or fresh allocation. Ordinary full-tuple guard evaluation
remains authoritative and preserves fresh-local allocation and body scope.
This is a certificate for the current total pure equality guards, not a claim about
arbitrary effectful or partial host procedures. No source semantics changes.

## Measured work

Size 12, quantum 8, ungrouped async:

| Workload | Prefix actions | Early-guard actions |
|---|---:|---:|
| Distinct q values, rejecting equality guard | 40,922 | 4,256 |
| Acyclic edge join, no guards | 6,147 | 6,147 |
| Successful guard ready after second head | 725 | 819 |
| Successful guard requiring third head | 725 | 779 |
| Fresh-local reflexive successful guard | 716 | 730 |

The rejecting case performs 276 dependency visits and 132 early guard evaluations.
It avoids constructing the rejected third-head suffixes; only the start/finish
rules produce complete accepted head tuples. All complete observations and raw
lineages remain unchanged.

The successful controls expose the cost of repeated readiness and guard evaluation.
For the early-ready successful guard there are 20 dependency visits and eight early
evaluations; the full guard is still evaluated normally. The fresh-local guard has
12 dependency visits and zero early evaluations: it correctly waits for the normal
fresh-variable scope. All three controls perform exactly floor(N/3) simplifications
and retain N mod 3 q occurrences, verified against hand expectations and the
independent interpreter.

These are charged action counts, not a timing or byte comparison. Static dependency
schedules and per-prefix guard-result retention could avoid repeated checks; their
preparation, storage and no-pruning overhead remain unmeasured. The observed benefit
supports early rejection when its prerequisites are certified, not blindly moving
all guards or assuming that readiness requires ground terms.

## Portfolio next step

The scan, predicate, prefix and guarded-prefix services now have independent
transition evidence and opposed work probes. Further selector/index/trail refinements
remain open, but do not block composition with a distinct execution service.
Proceed to E15's second-service composition using the existing finite interaction-net
unifier behind a sealed source boundary. Test complete projected states, failure
privacy and scheduler/observer behavior, charging encoding, decoding and source
selection alongside net work. This challenges conclusions drawn from the direct
service without adopting a production backend. Heterogeneous operation sharing and
the other E15 interactions remain separate required investigations.

Evidence: [raw results](E09-guards.jsonl), [replay](E09-guards-replay.jsonl),
[source and registration hashes](E09-guards-manifest.json).

```sh
cargo run -q -p chr-symbolic-fixtures --example export_cases > /tmp/chr-e09-cases.jsonl
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-scheduling -v
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/run_guards.py < /tmp/chr-e09-cases.jsonl
```

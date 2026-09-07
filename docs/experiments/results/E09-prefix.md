# E09 distinct-prefix matching

All 570 registered configurations pass and replay exactly: 192 E00 source-transition
checks, 324 mixed scheduling cases and 54 matching-predicate cases. Source work is
invariant across service quanta. The preceding 624-row scan/predicate matrix also
replays exactly after the integration. Nineteen local tests pass, including failed
prefix rollback and continued search after propagation-history exclusion.

The new selector extends one head at a time in original occurrence order. It
rejects reused occurrences and inconsistent nonbinding matches before generating
the remaining suffix. Each child owns its copied pattern-binding map and used-ID
set. Complete prefixes enter the same history/guard/body logic. Guards still run
only at the complete-tuple boundary. No source contract or reference code changes.

## Work consequences

Ungrouped async, batch 8, size 12, quantum 8:

| Workload | Full scan | Predicate product | Distinct prefix |
|---|---:|---:|---:|
| Incompatible predicates | 11,449 | 934 | 949 |
| Matching predicates, rejecting guard | 48,574 | 54,583 | 40,922 |
| Acyclic edge join | 43,065 | 48,994 | 6,147 |

All variants return the same complete observations and execute the same represented
source derivations. This table counts all instrumented actions, including prefix
copies and scheduler dispatch; it is not a wall-time or memory comparison.

For the acyclic join, predicate-product matching uses 12,446 match and 22,116
resolve actions. Prefix matching uses 1,055 and 2,148 respectively, while charging
616 prefix-copy, 290 candidate and 323 prefix-visit actions. Only two complete
matched tuples remain, for the start/finish rules; no complete cyclic edge match
exists. Thus early join rejection accounts for the reduction, not an omitted
administrative category or changed source selection.

For the rejecting guard, all three q heads individually match. Prefix matching
still generates 1,322 complete matched tuples including start/finish, and spends
8,316 copy actions plus 3,216 prefix visits. It improves over rebuilding all head
matches for each product tuple, but cannot avoid the guard-rejected suffixes while
guards remain at the full-tuple boundary. The incompatible-predicate case gains
nothing from prefix traversal and pays a small overhead over simple filtering.

## Remaining question

Test early pure-guard evaluation only when its rule variables are already bound by
the prefix. Later nonbinding head matching cannot change those bindings. Guards
with unavailable variables, including fresh rule-local variables, must remain
undecided until their ordinary boundary. Validate rollback and first-enabled-tuple
preservation independently and include guard-success/no-pruning controls, charging
dependency checks and repeated evaluation. This is an execution optimization
question, not permission for guards to unify or produce effects.

Further feasible work remains on binding-map trails versus copies, maintained
indexes and wakeups, measured selector runtime/storage, heterogeneous operation
sharing, longer synthesis and net/second-service composition. These results do not
close those directions or select a production architecture. More acyclic-path sizes
alone are unlikely to alter the narrower conclusion that failed prefixes should
not repeatedly generate inconsistent suffixes.

Evidence: [raw results](E09-prefix.jsonl), [replay](E09-prefix-replay.jsonl),
[source and registration hashes](E09-prefix-manifest.json).

```sh
cargo run -q -p chr-symbolic-fixtures --example export_cases > /tmp/chr-e09-cases.jsonl
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-scheduling -v
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/run_prefix.py < /tmp/chr-e09-cases.jsonl
```

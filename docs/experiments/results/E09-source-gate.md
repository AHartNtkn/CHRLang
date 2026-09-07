# E09 resumable source adapter gate

The independent direct-tree adapter passes all 64 E00 cases at service quanta
1, 8 and 64: 192 configurations and byte-identical replay. Each quantum covers
2,771 source transitions and 140,980 instrumented actions. The largest case is
SK ignored-hole evaluation (47,066 actions), followed by SK duplication (32,479).
These are correctness/work counts, not comparative timings.

`research/chr-scheduling/source.py` implements private resumable source steps:
nonbinding head matching, finite-tree unification with occurs checking, pure
equality guards, ordered occurrence selection, propagation history, rule-local
freshening, explicit binary OR and quiescent full-residual answers. State snapshots
are immutable and shared across forks. This experimental fixed selector is the
control's policy, not an adopted language requirement. The service is direct-tree
unification; interaction-net integration remains separate.

The checker compares every continued and forked state with the independently
implemented E11 tree interpreter, including pending goals, substitutions, stored
occurrence IDs, fresh counters and history. Answers additionally agree with the
hand expectations and Rust reference, including raw multiplicity, exact alpha
comparison of full residuals, exhaustion and registered finite prefixes. Candidate
code imports neither interpreter. Only the validation harness imports the control.
Three focused tests cover delayed matching after binding, transactional occurs
failure and yielding during 240 unsuccessful multihead tuple selections.

Reproduce:

```sh
cargo run -q -p chr-symbolic-fixtures --example export_cases > /tmp/chr-e09-cases.jsonl
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-scheduling -v
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/check_source.py < /tmp/chr-e09-cases.jsonl
```

Raw records: [gate](E09-source-gate.jsonl) and [replay](E09-source-gate-replay.jsonl).

The adapter accounts for traversal and copying work with resumable actions.
Host tuple/list allocation, hashing and permutation advancement can still perform
variable-size work; the action budget is not a wall-time bound. Query initialization
is outside the source-step gate. The independent exact observer is still atomic
validation code. Comparative scheduling must account for admission, grouping,
observer service and storage separately, and cannot infer scheduler fairness from
this gate alone. Next: resumable observation and explicit policy/support queues,
then the registered policy comparison and separate net-service integration.

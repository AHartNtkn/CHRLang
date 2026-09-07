# E15 policies over direct and net equality

All 980 configurations pass and replay exactly: 620 exhausted E00 semantic/application
configurations and 360 controlled configurations, including recursive finite prefixes.
The largest run consumes 7,101,679 actions, within the preregistered 20,000,000-action
bound. The injected-service tests cover branch-private occurs failure and a finite
answer alongside recursion. No backend fallback occurs.

A prepared NetEquality service is now supplied explicitly to Search and each source
job. Its codec and compiled rules are shared read-only; each request's graph and
substitution result are private. All net build/reduce/status/readback and codec
phases remain inside yielded source work. Rule selection stays fixed to scan for
both services. This is a hybrid composition, not a net-compiled full CHR engine.

## Same policy questions, distinct service work

At quantum 8, batch 8:

| Workload and service | Policy/grouping | Total actions | First answer action | Source jobs |
|---|---|---:|---:|---:|
| Large equation, direct | round off | 1,187 | 1,172 | 11 |
| Large equation, direct | async off | 1,193 | 622 | 11 |
| Large equation, net | round off | 7,050 | 7,035 | 11 |
| Large equation, net | async off | 7,056 | 935 | 11 |
| 6 duplicate choices, direct | async off | 5,110 | 4,102 | 382 |
| 6 duplicate choices, direct | async on | 535 | 535 | 22 |
| 6 duplicate choices, net | async off | 15,350 | 14,342 | 382 |
| 6 duplicate choices, net | async on | 695 | 695 | 22 |
| Opaque depth 64, net | async off | 52,034 | 51,963 | 16 |
| Opaque depth 64, net | async on | 52,077 | 52,006 | 16 |

The large-equation probe returns `a` first in these runs. Async can publish it
while unrelated net work is unfinished; the round barrier postpones its next source
operation. Total action counts within each service are nearly equal, so this is
latency isolation rather than less completed source work. Units are instrumented
service actions, not equal-duration CPU operations or native performance ratios.

Grouping preserves 64 raw duplicate alternatives and one exact unique observation.
It performs 22 shared source jobs representing 382 source transitions under both
services. In this workload the net service adds 160 actions to the shared run and
64 times that amount to the ungrouped run. The larger operation cost amplifies the
benefit of actual reuse. Different output bindings in the opaque workload still
prevent whole-state grouping from sharing otherwise independent operations.

## Representation and prefix accounting

Every completed matched configuration has the same source-job count under direct
and net equality. Eighteen comparisons differ only among recursive runs stopped
at their first answer: net completion permits additional recursive source jobs to
be admitted before that stopping event. These are different finite prefixes of
ongoing work, not lost answers or demonstrated changes in state-grouping eligibility.
Raw answers and the requested finite observation are verified separately.

This does not prove that alias representative/table ordering can never affect exact
state keys. Deliberately reconvergent alias paths remain a useful separate test.
The controlled inputs here are reconstructed from rule records for both services;
physical syntax sharing differs from the literal-object E09 generator runs. Their
identity-check action counts should not be pooled across preparations. The two
services in this matrix receive the same prepared rule representation.

## Disposition

The finite-service scheduling mechanism and duplicate-support sharing survive
composition with a distinct graph equality service. The source/net boundary also
exposes status scans, codecs and output materialization as substantial work, as
recorded in the source gate. No engine or source-language restriction is selected.

Next measure compilation/preparation, runtime and storage for this composition,
with a credible controller-status maintenance control for the dominant scan cost.
Heterogeneous operation sharing remains especially relevant because whole-state
keys avoid no source work in the intended completed application cases. Other E15
combinations, native choice correspondence, routed stores, maintained source indexes,
longer synthesis and final direction audits remain open. More duplicate-only
repetitions are unlikely to resolve those questions.

Evidence: [raw results](E15-policies.jsonl), [replay](E15-policies-replay.jsonl),
[source and registration hashes](E15-policies-manifest.json).

```sh
cargo run -q -p chr-symbolic-fixtures --example export_cases > /tmp/e15-cases.jsonl
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/run_net_policies.py < /tmp/e15-cases.jsonl
```

Preparation is outside this work-count matrix and must be included in subsequent
time/storage measurements. No wall-time or byte conclusion follows from these rows.

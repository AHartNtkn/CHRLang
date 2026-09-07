# E09 grouping-key traversal ablation

All 750 registered configurations pass and replay exactly: 162 comparison
microcases, 216 integrated controlled workloads and 372 exhausted E00 semantic
configurations. The existing 180-row reverse-traversal baseline also replays
byte-for-byte. Thirteen source, observer, scheduler and comparison tests pass.

The three comparison modes preserve exactly the same complete-state equality
contract. Reverse and forward differ only in nested tuple traversal order. Identity
uses forward traversal and adds a charged pointer-equality check on immutable
objects. Reference code remains independent and unchanged.

## What explains the overhead?

At depth 64 and quantum 8, comparison actions are:

| Inputs | Reverse | Forward | Identity + forward |
|---|---:|---:|---:|
| Equal, independently rebuilt | 3,151 | 3,151 | 4,723 |
| Equal, physically shared | 3,151 | 3,151 | 19 |
| First mismatch, rebuilt | 3,160 | 28 | 36 |
| Last mismatch, rebuilt | 28 | 3,160 | 4,736 |
| Last mismatch, shared common goals | 28 | 3,160 | 44 |

Moving the mismatch reverses which traversal wins. The earlier opaque-work
penalty was substantially an unfavorable comparison order, not a fundamental
cost of exact-state grouping. Physical sharing makes identity shortcuts effective;
rebuilt equal inputs pay extra identity checks without avoiding their structure.
Action types have different host costs, so these counts do not establish that a
pointer check makes wall time worse on rebuilt inputs.

In the integrated depth-64 opaque workload at quantum 8:

| Policy | Reverse total | Forward total | Identity total | Source jobs in every mode |
|---|---:|---:|---:|---:|
| round | 10,304 | 6,781 | 6,817 | 16 |
| async | 10,224 | 6,701 | 6,712 | 16 |

The ungrouped totals from the preceding control are 6,659 and 6,669 respectively.
Forward traversal removes most of the extra work. No mode shares the independent
operations across different output bindings: all retain 5,827 source actions.
Changing comparison order cannot change this complete-state eligibility boundary.

On the six-choice duplicate workload, both grouped policies improve from 829 total
actions to 517 with identity checks; source jobs remain 22, representing 382 source
transitions and 64 raw alternatives. This is cheaper recognition of already shared
states, not additional source work sharing.

There are no source-job-count changes between modes in any matched integrated or
E00 conformance configuration. Thus the measured changes here do not conceal an
asynchronous batching change in executed source work. Such changes remain possible
on other arrival patterns.

## Bounded conclusions and next investigations

A fixed traversal order has no generally favorable mismatch position. These
opposed controls sufficiently establish that narrow conclusion; more depths alone
are unlikely to change it. The practical selection depends on actual state shape,
physical sharing and host costs. It is premature to choose a production key scheme.

Useful remaining work includes measured preparation/runtime/storage costs and
batch-size sensitivity; cached summaries or discriminating metadata with their
construction/retention costs; heterogeneous operation sharing; mixed scans and
wake-ups; longer synthesis and net-service composition. Those questions are not
closed by these comparison counts. Next measure the integrated cost of the explicit
variants, including query setup and output retention, before deciding whether any
key refinement merits further complexity.

Evidence: [raw results](E09-keys.jsonl), [replay](E09-keys-replay.jsonl),
[source/registration hashes and host](E09-keys-manifest.json).

```sh
cargo run -q -p chr-symbolic-fixtures --example export_cases > /tmp/chr-e09-cases.jsonl
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-scheduling -v
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/run_keys.py < /tmp/chr-e09-cases.jsonl
```

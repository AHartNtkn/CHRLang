# E12 fixed-ID continuation table

Exact state reconvergence avoids repeated source transitions on duplicate-choice
workloads. This implementation retains substantial state and does not help the
measured intended applications under its conservative key. These are results for
fixed IDs, full history retention and individual FIFO lineage jobs; broader state
reuse remains open.

Code `c1e72f7`; release build. The [first batch](E12-continuations-v1.jsonl) and
[replay](E12-continuations-replay.jsonl) each contain all 208 registered runs:
64 E00 cases plus 40 duplicate-choice workloads, in Direct and ExactIds modes.
All pass; every non-time field repeats exactly. Paired logical steps, raw answers,
failures, unique answers and maximum frontier also agree. Both modes use the same
persistent scalar transition service and exact full-answer observer. Reference
implementation code is not imported by the candidate.

| Case | Executed transitions: direct / table | Actual rule applications: direct / table | Requested bytes: direct / table | Peak live requested bytes: direct / table |
| --- | ---: | ---: | ---: | ---: |
| Duplicate K8 W64 N0 | 34,053 / 162 | 16,639 / 72 | 20,275,171 / 1,963,645 | 1,536,122 / 953,166 |
| Duplicate K8 W64 N16 | 34,069 / 178 | 16,639 / 72 | 36,900,916 / 7,745,990 | 1,667,185 / 2,485,534 |
| Distinct carry K3 W4 N3 | 107 / 107 | 39 / 39 | 85,112 / 343,378 | 32,818 / 251,454 |
| SK duplication evaluation | 431 / 431 | 19 / 19 | 404,302 / 2,584,102 | 174,133 / 2,139,520 |

The K8 W64 N16 case keeps all 34,069 FIFO lineage jobs. It forwards 33,891 through
cached edges, preserving 256 raw derivations and one deduplicated answer. Its
final retained requested storage is 2,468,043 bytes with the table versus 87,270
bytes without it. Reduced allocation traffic therefore does not imply lower
retained memory. With N0 the peak also decreases; unrelated store content changes
the tradeoff.

Executed work decreases in 33 of 104 workloads: the 32 duplicate grid cases with
K>0 and E00's raw-duplicate case. The tested arithmetic, SK/lambda and type cases
show no executed-step reduction. This does not establish absence of useful
reconvergence under alpha-equivalent or call-level keys. Distinct unresolved
choices also remain distinct full states, even when later rules consume their
values opaquely; this table cannot replace choice-conditioned work sharing.

Runtime rows include initialization, input cloning, key projection/lookup,
transition execution, edge forwarding, answer construction and deduplication.
Input workload generation and expected-answer checks are outside the measured
interval. Allocation counters include map/key and observer costs, but requested
live bytes are not RSS. Two fresh observations per configuration are exploratory
runtime evidence, not a precise timing confidence interval; operation and storage
counts provide the main attribution. Raw rows retain both elapsed measurements.

## Correctness evidence and remaining work

All 64 registry cases match full registered answers, raw multiplicity, exhaustion
and finite-prefix limits in both modes. All 40 duplicate workloads are also
checked against the independent reference. A targeted nested-choice test verifies
that a shared continuation creates later children for each incoming derivation.
Key tests distinguish history with the same visible store, pending failure,
aliases, occurrence order and multiplicity; behavioral tests show distinct next
events and distinguish an alias-induced failure from the independent case.

The key resolves terms but retains next-allocation counters, exact free-variable
and occurrence IDs, every history token, pending order and full outputs/store.
It is conservative. Retaining all keys/edges and old tokens is a measured storage
limitation. Feasible follow-ups are alpha-renaming, dead-token relevance,
reclamation/cache bounds, grouping equivalent lineage jobs, and exact call tables
with caller filtering. Each changes a distinct cost or equivalence obligation and
needs its own control. Operation-table cost measurements and proof-producing
failure reuse are independent E12 investigations still in progress.

Reproduce with `cargo build --release -p chr-reuse --example continuation_probe`,
then `python research/chr-reuse/run_continuations.py OUTPUT --seed 1201` and a
second output with seed 1202. Workspace tests, Clippy and formatting pass.

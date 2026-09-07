# E15 maintained controller-status control

Counted status passes all 1,296 reference boundary configurations and 64 composed
source cases with exact replay. The original 1,296-row scan boundary control also
replays exactly. Every reference request uses fewer total charged actions with the
maintained count. This is a bounded work result, not a universal graph-cost or
wall-time claim.

## Same terminal predicate, different accounting

The maintained value is the number of live nodes whose tag is neither DATA nor Out.
Public allocation classifies the added node. A rewrite applies its prepared delta:
controllers created on the RHS minus controllers consumed on the LHS. RHS allocation
uses the internal allocator, so it does not also apply public-allocation updates.
Wiring changes no tags. Induction over these two mutation paths preserves the same
count a full scan would compute after each completed primitive.

Metadata is compiled once during prepared-service construction. Requests charge one
maintenance action per public allocation and per rewrite, then one terminal count
check. The service still builds and reduces the complete graph, validates status,
reads both outputs, decodes them and preserves the original substitution. Counted
status neither omits validation nor publishes on a nonzero count. Focused tests
compare the count to an independent scan after every primitive and check rejection
of a stuck controller before publication.

## Effect on composed source work

| Case | Scan total | Counted total | Maintenance actions | Final checks |
|---|---:|---:|---:|---:|
| SK duplication | 3,868,948 | 3,065,837 | 1,016,246 | 139 |
| SK ignored hole | 6,300,045 | 4,973,205 | 1,660,672 | 166 |

The scan mode had 1,819,496 and 2,987,678 status-scan actions respectively. The
maintained mode pays the displayed updates and checks instead, yielding roughly
21% lower overall action totals. Encoding, reduction and readback counts and complete
source observations remain unchanged. The net service still performs much more
representation and graph work than the direct service; this refinement does not
establish competitive native-engine performance.

The work unit combines a bounded tag classification/update or a precomputed delta
update. Python lookup/allocation overhead and metadata preparation are not quantified
by treating these actions as equal-duration operations. Subsequent runtime/storage
measurement must use both modes and include preparation. Node slots and their
storage are not reclaimed by this change.

## Disposition

This is a credible stronger validation control for the dominant terminal scan cost.
Its work benefit is established on the registered requests and source cases. Generic
graphs or different allocation/rewrite ratios need not have the same tradeoff.
Typed data-only construction could avoid some public-allocation classifications;
that is a further preparation/interface optimization, not assumed here.

Next compare complete policy runtime/storage with direct, scan-net and counted-net
services, including cold compilation/output and repeated queries sharing prepared
rules and services. Heterogeneous operation sharing,
routed stores, slot reclamation, native choice correspondence and other E15 pairings
remain independently open. Further status-only size sweeps are unlikely to answer
the project's larger representation and sharing questions.

Evidence: [boundary](E15-count-bridge.jsonl), [boundary replay](E15-count-bridge-replay.jsonl),
[source](E15-count-source.jsonl), [source replay](E15-count-source-replay.jsonl),
[source hashes and host](E15-status-manifest.json).

```sh
cargo run -q -p chr-cases --example net_unification_cases > /tmp/e15-requests.jsonl
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/check_net_bridge.py count < /tmp/e15-requests.jsonl
cargo run -q -p chr-symbolic-fixtures --example export_cases > /tmp/e15-cases.jsonl
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/check_net_source.py count < /tmp/e15-cases.jsonl
```

# E09: yielding throughout a unification request

The scheduler can now service construction, reduction and readback separately.
All 288 workload/quantum configurations pass and replay exactly; the 19-test net
suite passes. This supplies a whole-request service boundary for E09, not a
completed scheduler-policy comparison or a general source-search fairness proof.

## Reproduction

```
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-nets -v
PYTHONDONTWRITEBYTECODE=1 python research/chr-nets/check_service.py docs/experiments/results/E09-service-gate.jsonl
PYTHONDONTWRITEBYTECODE=1 python research/chr-nets/check_service.py docs/experiments/results/E09-service-replay.jsonl
```

The 96 existing independently expected unification workloads run at quanta 1,8,64.
Every completed result preserves the original table, private failure and projected
MGU. Phase counts agree across quanta; resume counts differ as expected. Every
field replays exactly. The largest request uses 27,781 actions, below the registered
250,000 bound. No result is observable during partial construction or readback.

Separate interleaving tests give a small identity and a large depth-128 request
one action each per turn. The small request finishes while the large request is
still constructing its graph, and in a second test while it is still reading its
result. This directly exercises the phases an evaluator-only quantum would miss.
It does not assert a wall-clock latency bound.

## What each action charges

UnificationJob takes an immutable encoded table/equation list and an already
compiled, shared read-only rule system. Admission creates fixed-size private
control state. Build actions create header nodes/wires or one fixed-arity data
node; the private graph is not reduced while partially wired. Reduce actions
perform one actual net interaction. Scan actions check one stored node slot for
an unresolved controller. Read actions visit or assemble one fixed-arity output
node. One publication action makes the two complete outputs available.

No recursive whole-tree builder or decoder runs between those action boundaries.
The reader uses an explicit stack. Input/source encoding and shared rule
compilation remain setup responsibilities that must be charged separately by the
scheduler harness; the API does not quietly perform them per request. Host
allocation, dictionary resizing and garbage collection remain finite runtime work,
not hard real-time primitives.

For N=16,D=16:

| Request | Build | Reduce | Slot checks | Read | Publish | Total actions |
|---|---:|---:|---:|---:|---:|---:|
| Identity with padding | 1,799 | 2,611 | 8,795 | 7,142 | 1 | 20,348 |
| Alias-chain binding | 448 | 7,326 | 15,447 | 1,586 | 1 | 24,808 |
| Indirect occurs failure | 469 | 8,765 | 17,870 | 676 | 1 | 27,781 |

These are explicit service actions, not equally costly instructions. Charging only
active-pair reduction would omit most actions on the padded identity request.
Slot checks are also a concrete optimization opportunity: maintaining an exact
live-controller count could replace a full metadata scan. That requires validating
the maintained invariant and charging its update cost; a policy comparison should
not infer an unavoidable graph-runtime penalty from the current scan alone.

## Correctness and liveness scope

The work generator follows the same data construction order and reduction rules
as the validated E06 service. Yielding changes no graph operation. Partial graphs
remain private; scan rejects unresolved controllers; full readback precedes
publication. The frozen request has finite acyclic input data, and the finite-tree
unifier terminates under that premise. Build/scan/read traverse finite structures
without admitting new source work, so every such request needs finitely many
actions. FIFO service with positive quanta can therefore eventually finish an
admitted request, assuming finite predecessors and available resources.

This argument does not cover a source rule that recursively evaluates itself
inside one service, a nonterminating host guard, mutable input snapshots, or an
observer whose comparison set grows while it runs. It does not establish fair
source selection, symbolic support ownership, wake-up correctness or answer
canonicalization. Those remain explicit E09 obligations.

## Next discriminating work

Implement the per-alternative, sealed-round and asynchronous policies with an
explicit source-state owner and a charged selector/observer boundary. Keep a
completed support independent of an unfinished sibling. Use the existing complete
CHR controls for source trace/answer checking rather than treating a toy service
script as a general CHR interpreter. The independent E11 tree witness checker was
inspected as an available semantic control; it remains unchanged.

Compare grouping and quantum costs, challenge barriers with a huge finite service,
and include source recursion, matching scans, wake-ups and duplicate observation.
E06's general encoded/native compiler and richer protocol work remain queued and
open; this ready scheduling investigation does not depend on their completion.

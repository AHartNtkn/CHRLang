# An asynchronous scheduler over disjoint supports

Symbolic scheduling can release one completed support group without waiting for unrelated groups. The construction keeps at most one source operation in progress per alternative, represented by disjoint conditions rather than mandatory per-leaf tickets. It requires finite resumable operations on sealed snapshots.

This original construction strengthens the sealed-round control in T015-symbolic-scheduling.md. It trades concurrency within an individual alternative for a simpler source-state consistency argument. The language remains unordered CHR; the runtime chooses one allowed committed policy.

## Ownership of a projected source state

Partition Live among control entries. An entry is Ready(E,S), Busy(E,S,j), Certifying(E,S,j), or Complete(E,observation), where E is an exact support, S an immutable projected-family snapshot and j a resumable job. Failed contexts are outside Live. Distinct entries have disjoint supports. Equal entries may share representation, but alternative identity remains in the separate ledger.

Only the job owning E may commit a source update on E. Other alternatives may update their own parts of physically shared structures. Their updates must preserve projection outside their own supports. This is the conditional-update invariant, not a machine-wide mutex or a source-order requirement.

A Ready job selects the next source transition under the fixed policy using finite matching/guard/history work. An unknown head demand is a non-enabled match, not an invitation to block the host worker. Pending equations or other producer goals remain part of source policy selection. Once no source step is available, the job certifies quiescence on its unchanged snapshot.

## Job and publication rules

- Admit each finite request at the tail of a FIFO service queue. Give every action a finite graph/worklist budget; unfinished runnable jobs return to the tail.
- Seal a request's support and input snapshot at admission. New source work cannot enlarge what it must finish. Administrative subdivision partitions that original finite support and retains the corresponding immutable inputs.
- A selected source operation can publish on E1 as soon as its complete scalar transition is ready throughout E1. Remove E1 from its busy support and enqueue Ready(E1,S1) for the next transition. The remaining E2 stays with the unfinished operation. This is never partial publication of one scalar unification.
- Source OR replaces its owned parents with children and creates Ready entries for them. Pull back other conditional structures consistently. Newly introduced source goals receive fresh service requests; recursive source calls do not remain inside an old job indefinitely.
- Representation-only support merging is optional. Merge only finite already-admitted compatible requests with an immutable combined input; do not continually absorb new requests into an accepted batch. A simplest correct implementation never merges admitted jobs, but still shares operations over their initial symbolic supports.
- Every service request either completes using its finite snapshot or returns a suspended administrative continuation with an independently admitted finite producer. Waiting releases the worker. There is no circular acquisition of ownership for another support while holding one; work requiring overlapping branch state belongs to that branch's one selected source operation.

At initialization, one Ready entry may cover the whole query family. A choice-independent next operation need not split it. If matching or equality distinguishes contexts, administrative subdivision creates several entries. In the worst case these become as numerous as the alternatives; the invariant permits compression but does not guarantee it.

## Why jobs make progress

A finite immutable snapshot has finitely many alternatives and finite projected data. For each alternative, the selector and one finite source transition terminate under the assumed guard/built-in procedures. The symbolic service algorithm must be a finite simulation of those operations: exact condition operations, bounded graph traversals, finite worklists and no recursive source normalization. Hence each admitted request has a finite administrative computation, possibly with a finite tree of subrequests.

At every finite admission instant a queued job has finitely many predecessors. FIFO finite service eventually reaches it. Repeating this argument gives each finite administrative subrequest enough service to finish. Newly created source requests enter behind work already queued. An indefinitely recursive alternative therefore generates infinitely many separate finite requests, not one request that prevents a sibling's finite request from completing.

Fix a successful explicit alternative with a finite selected source derivation. Its first request completes by the preceding argument. Its successor is admitted finitely and also completes; induction over the derivation reaches certification and answer extraction. Partial publication avoids waiting for the rest of an already-running request after that alternative's full step is ready. Progress still assumes available memory and a draining output consumer.

## Safety and certificate stability

The branch-state owner prevents source interference while its operation runs. Thus quiescence checking does not restart because of another branch's updates, and a selected match cannot be invalidated by a competing consumer within the same branch. Private unification commits all its bindings together. The transition-by-transition projection proof is the one in T016-composed-machine-argument.md; disjoint support ownership supplies the stability premise that sealed rounds previously supplied.

A global version counter would not implement this invariant: it could invalidate a stable support whenever any alternative changes. Versions and invalidation must respect the projected branch state. An implementation can choose immutable conditional snapshots and disjoint commits to make that relationship explicit.

## Boundaries and costs

The construction answers the feasibility question for asynchronous symbolic fairness under the stated finite-service assumptions. It does not prove efficient exact condition operations, bounded response time, or a compact representation for every family. A single finite selector can be expensive; it must yield, and it may need to subdivide before it can expose a small completed result.

It also does not permit simultaneous source operations within the same alternative. Such parallelism could be added with the commuting-Apply certificate or a transaction scheme, but is independent of the owner's central goal of sharing across alternatives. The source semantics does not promise physical parallel execution of every enabled rule.

The key empirical quantities are how often supports fragment, the cost of preserving snapshots and ownership, administrative visits before the first partial result, and work saved by initially shared requests. These can be measured against per-leaf tickets and sealed rounds once implemented. No further owner taste decision is required to retain all three scheduler designs for comparison.

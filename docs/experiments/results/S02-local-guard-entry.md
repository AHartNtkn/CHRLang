# Equality guards preserve the tested local graph search behavior

The local graph search now checks positive equality guards without binding unknowns. Complete answers agree with independent controls across the registered cases. This closes a source-admission gap; branch copying and sustained ownership still need comparative measurement.

## What the experiment establishes

A guard asks whether its terms are already equal. It must neither make them equal nor claim resources when it fails. The experimental matcher now checks every guard at each complete candidate tuple, before recording history or consuming occurrences. A failed guard allows it to try later tuples. Scanning after a completed body can discover an earlier request enabled by new equality information.

The comparator reads captured graph handles and constructor syntax. Guard-local unknowns have their own identity: a repeated local unknown equals itself, but cannot equal a query unknown merely because their numeric variable IDs coincide. It creates no graph nodes and installs no equations. Its recursive traversal and the graph equality helper can still incur size-dependent work and temporary bookkeeping.

The existing deterministic entry remains guard-free. This experiment extends the prepared search entry, whose branches copy mutable graphs and share immutable rules. It does not add subscriptions or retained joins.

| Evidence per confirming execution | What it tests | Result |
|---|---|---|
| 64 source configurations | Four guard forms; unresolved, aliased, equal-ground and conflicting inputs; optional alternatives; reversed order | Complete ordered answers match the independent scalar evaluator; compiled Scan/Indexed and contextual controls match complete raw multisets |
| Failed first tuple | A later matching request satisfies the guard | The later request consumes the resource |
| Unknowns and fresh scope | Failed guards, repeated guard-local variables, colliding numeric IDs and conjunctive guards | Unknowns remain unbound; fresh aliases remain distinct; every conjunct is required |
| Body completion and failure | A body posts a newer request before enabling an older one; another branch fails through a cycle | Only the successful branch publishes an answer, with the older consumer winning |

The matrix crosses four terms: `X=Y`, `f(X)=f(Y)`, `X=f(Y)` and `pair(X,X)=pair(Y,Y)`. Each source initially suspends a guarded request before its setup rule. This exercises late enablement and branch-local information, rather than only guards known at query setup.

Four confirming executions pass: two with local counters enabled and two disabled. Each passes all four tests. Another 21 tests pass across the local-choice, deterministic multihead/cache, contextual and contextual-source executables. The missing-admission failing run, subsequent passing run and successful scoped Clippy run are preserved. No reference or scalar implementation was changed.

Finite paths must exhaust within 200,000 turns. Each confirming executable has 60-second wall/CPU and 1 GiB address-space bounds. Exact delivered order is compared with the scalar policy; complete answer multisets are compared with the other policies. Prepared ownership returns to the caller alone after each finite local search. That assertion does not measure all retained memory.

## What follows from this result

The candidate can enter guarded-choice lifecycle comparisons. Its earlier deterministic graph-scan gains cannot answer whether copying branch state repays those gains, especially with wide frontiers, late failure or retained outputs. This result supplies no timing, allocation, RSS or complete-architecture superiority claim.

**Next qualify ownership and lifecycle endpoints for guarded choices under T072.** Compare the local search with the existing compiled Scan/Indexed and contextual searches. Reuse prepared rules across changing queries; retain and release outputs independently; cancel an ongoing sibling. Account separately for preparation, query state, branch state and consumer answers before registering costs. The [execution sequence](../next-cycle.md#start-here-the-next-experiments) specifies the next two packages and their decision gates.

Joint structural theory/source integration is the strongest ready alternative. It could reveal interactions absent from the separate theory entries, but requires a combined source contract and oracle before costs. Ownership qualification comes first because it directly tests whether the newly admitted graph candidate is fit for a complete cost comparison, using existing controls and a qualified source contract. Demand-driven choice expansion could avoid copying, and conditional publication could alter retained-state costs; both remain required alternatives. Reconsider them at the ownership gate or an obstruction, rather than presuming another local optimization.

This is package two after the normal/neutral breadth review. At most two further packages precede a full review of every remaining mechanism. Finishing this gate does not finish integrated execution, the experimental sequence or the research goal.

## Evidence

[Registration](../registrations/S02-local-guard-entry.md), [tests](../../../research/chr-relational/tests/local_guards.rs), [runner](../../../research/chr-relational/experiments/local_guard_entry.py), [frozen inputs and binaries](s02-local-guard-entry/freeze.json), [audit](s02-local-guard-entry/audit.json) and [raw receipts](s02-local-guard-entry/). The audit checks 19 recorded source inputs, six binary identities, four confirming executions and four regression receipts. Both prior source snapshots match commit `31a34014b`; they preserve the preceding choice experiment's inputs. Assertions about answers and ownership establish the semantic result; artifact hashes establish provenance.

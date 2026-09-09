# Direct handle repair is a distinct local execution mechanism

The local prototype merges nodes by rewriting registered handles and waking attached applications. It passes the independent constructor and consuming-source checks without a parent forest or relational match tuples. It makes the tradeoff concrete: later uses follow one handle, while merging pays for repair and activation.

## What differs from the CHR-expressed implementation

The [local prototype](../../../research/chr-relational/tests/support/local_ports.rs) stores a descriptor and a list of registered handles at each live node. Handles point directly to nodes. A merge moves the smaller handle set into the larger set and rewrites those targets; retired nodes carry no forwarding parent. Queued equations and constructor children use the same repairable handles, so they remain valid after a merge.

Same-constructor merges enqueue child equations. A graph traversal checks whether either node already reaches the other before merging; constructor name or arity disagreement fails. This uses local reachability rather than materializing the CHR encoding's transitive ancestry relation. Neither strategy has a cost result yet.

Consuming requests register watchers on their input nodes. A node becoming `f(child)` activates its attached requests, and a consuming application binds its result by posting an equation. An ordered ready set selects requests by their source registration order. Tokens are individually numbered and consumed once. The prototype drains equality before selecting another source application; this is an explicit policy, not a language choice.

The [CHR encoding](S02-chr-constructors-gate.md) instead follows parent constraints, rewrites exposed descriptor columns through source rules and discovers applications through CHR match tuples. The existing relational Store also has a Rust parent/rank table and relational index repair. A shared ability to represent equality does not make these operation and ownership models equivalent.

## Independent evidence and contrary controls

The [combined source tests](../../../research/chr-relational/tests/chr_constructors.rs) now run the local prototype on all 2,160 three-node equation configurations already checked against independent recursive substitution. The CHR Scan/Indexed comparisons remain in the same gate. Complete observations preserve aliases and residual multiplicity; a failed or unfinished computation cannot pass as an empty successful answer.

| Question | Evidence | Bounded finding |
|---|---|---|
| Does local repair preserve finite-tree equations? | All 2,160 unary configurations agree with independent substitution. Additional binary `pair(X,X)` cases compare success and a child clash independently; different arities fail. | Direct handle repair implements the tested constructor/equality semantics. It is not evidence for a general source compiler. |
| Can equality and consumption alternate? | Two consuming requests suspend before information arrives. Supplying `f(f(a))` permits zero, one or two firings according to token availability, matching complete ordinary-source answers. | Newly available information wakes computation, and a consuming result can enable another consuming request. |
| Is activation local? | One selected input acquires a descriptor with 0, 8 or 64 unrelated suspended requests present. | Exactly one additional request inspection occurs in each case. Unrelated requests remain suspended in the result. |
| Does the adverse broad merge pay for repair? | Join 1, 8 or 64 input aliases, then supply a constructor and service their consumers. | Before descriptor supply, handle repair counts are respectively 0, 7 and 63. Supply activates every consumer; all outputs and individual consumed token IDs validate. |
| Are fresh values and failed forks isolated? | Clone a suspended run; allocate fresh nodes separately in a successful consuming branch and a branch with an unobserved constructor cycle. | The failed branch cannot publish, the successful branch consumes its own token, and the original run retains its unknowns and token. Cloning is owned copying, not graph-sharing evidence. |
| Which contested consumer wins? | Reverse two ready requests competing for one token. | Local registration order matches the ordinary source outcomes in both orders. The CHR encoding's descriptor-order discrepancy remains explicitly asserted in the same test. |

Request selection is only validated for the implemented `take(f(X), Y)` rule. Nested patterns, multiple rules, propagation histories and general competing joins need their own scheduling and source-correspondence arguments. Agreement on this example does not prove universal policy preservation.

## An avoidable registration cost was corrected

The first local implementation woke all prior watchers when registering another request on an already-known input. A 64-request witness recorded 2,080 request inspections. The [failing check](s02-local-ports-gate/registration-defect.log) exposed this quadratic preparation behavior.

Registration now inspects and schedules only the new request. The same witness records 64 registration inspections, then validates all 64 complete outputs and consumption. This repair concerns discovery bookkeeping, not an intrinsic cost of local node integration. Diagnostic counts include registration as well as subsequent activation; the selective-control assertion measures the activation delta.

## Validation and limits

All 40 relational-package tests pass. Strict scoped Clippy passes for the source-gate target. [Receipts](s02-local-ports-gate/) preserve the initial unimplemented failure, registration defect, final package tests and lint result. Commands: `cargo test -p chr-relational` and `cargo clippy -p chr-relational --test chr_constructors --no-deps -- -D warnings`. Reference-interpreter code and unrelated research are unchanged.

This is a diagnostic prototype with counters enabled. No timing, allocator or RSS measurement has run. Prepared-rule compilation is absent: the consuming rule is implemented directly, while constructor descriptors accept arbitrary names and arities. Any cost comparison must include an equally specialized direct control or a corresponding general source-derived implementation; generic CHR overhead cannot be charged to the representation difference alone.

The prototype retains handle slots, retired node slots, requests and watcher entries until query disposal. Small-to-large merging bounds handle relocation under this implementation but does not establish a complete complexity bound: reachability checks, watcher visits, ready-set operations, output traversal and copying remain separate costs. Sustained reclamation and long-lived artifacts remain required.

## Next investigation and selection

This is the third package after the lifetime breadth review: forest operations, CHR-expressed constructor/consumption, then local handle rewriting. T072 remains active. The next source comparison must address broader rule derivation, nested matching and source effect/history ownership, with equivalent specialization and a declared scheduling contract. It must identify which useful work disappears relative to the strongest compiled and exact-source controls before a broad cost matrix.

Call-level reuse remains the strongest alternative investigation. It could supply repeated-work savings through recognition and replay instead of shared state. Compare its readiness and implementation effort at the next package boundary; the fourth completed package triggers the required breadth review. A timing matrix over the current hard-coded consuming rule is not automatically the most valuable next step.

Neither local correctness nor the selective activation count selects an architecture. General source correspondence, total efficiency, lifetime, necessary complexity, language tradeoffs, complete architectures and held-out challenges remain unresolved.

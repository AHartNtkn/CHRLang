# Direct choice graphs can preserve contextual resource ownership in the kernel

The new kernel passes independent checks for choice correlation, dynamic multiplicity and atomic consumption within selected alternatives. It keeps constructor children shared until demanded and can consume a resource in one alternative while preserving it for a late consumer in another. **This is a representation feasibility result, not yet evidence that a direct graph executes CHR correctly or efficiently.**

The [prospective registration](../registrations/S03-direct-graph-kernel.md) defines the experiment. The implementation is in [chr-direct-choice](../../../research/chr-direct-choice/src/lib.rs), separate from the reference interpreter and existing candidate executors.

## What the tests establish

| Question | Evidence | Result |
|---|---|---|
| Can partial contexts be combined and subtracted correctly? | All 81 sparse contexts over four labels, all 6,561 pairs and all 16 Boolean assignments; expected truth values computed separately | Intersection and subtraction match the truth tables; subtraction regions never overlap |
| Does opaque access avoid choosing child values? | Expose a constructor containing repeated and independent choice nodes | One constructor view retains the original child node identities |
| Do repeated values retain correlation and aliases? | Observe a constructor containing X twice, independent Y, and one unknown twice | Exactly the four expected joint values; repeated X agrees and the unknown identity survives |
| Are inactive and equal-valued alternatives handled differently? | A versus nested B/C; equal arms; choices absent from observed roots | Three histories for the nested choice; six after an independent equal-arm birth; off-output choices still count |
| Can consumers conflict without globally consuming a shared resource? | Six consumption requests checked against eight independent per-assignment resource sets | Atomic multihead consumption, overlap, duplicate-head rejection and late arrivals match the scalar oracle |
| Do graph demand and resources work together? | An A-headed consumer takes task/token only where a shared value is A; a late consumer requests both everywhere | The late consumer receives B only. A later birth under the A application produces two local descendants, neither able to consume the parent again |
| Are dynamic birth dependencies enforced? | Attempt to introduce a birth under a conditional label without its causal ancestor | Rejected with the expected invariant failure |

The four initial tests failed on unimplemented kernel operations before implementation and then passed. Two further tests exercise combined behavior and invalid birth construction. All six tests pass on the final source and its replay; Clippy with warnings denied and formatting checks pass. Commands, exit codes and source hashes are in [validation.json](s03-graph-kernel/validation.json); [test output](s03-graph-kernel/tests.log) records the final gate. Each process had a 60-second bound; no cutoff occurred. Test durations are not architectural timing evidence.

## What this changes in the investigation

A direct value graph need not copy its constructor graph into complete branch states to preserve resource-sensitive alternatives. This implementation instead maintains sparse choice contexts and subtracts successful consumption regions from occurrence availability. Distinct source occurrences retain separate identities even when their values agree. A constructor demand carries its context into resource consumption, which is the connection that a pure value generator would not establish.

That benefit introduces explicit obligations: dynamic-birth activation, region algebra, occurrence ownership and complete raw-history reconstruction. It does not establish that this machinery costs less than Conditional support tracking or ordinary execution. Current regions are disjoint lists of conjunctions; repeated overlapping updates may fragment them, and there is no region merging. This is an adverse case for the eventual cost comparison, not a reason to assume a different region representation wins.

## What remains before source or cost claims

The kernel has no source-rule evaluator, binding/equality service, propagation history, failure scheduler or completion detector. Its immutable term arena is acyclic by construction; that does not discharge finite-tree equality or cycle rejection. Observation recursively materializes terms and enumerates active histories only when explicitly requested; the caller must establish that all relevant source obligations have finished. It currently has no bounded observation service, cancellation or reclamation policy. Arena IDs are local to their owner.

T062 remains active. The next implementation must connect source matching and equality to this demand/resource boundary and pass the [compound source gate](../registrations/S03-dynamic-resource-source-gate.md), including disconnected failure and finite-sibling progress. A graphless direct control must distinguish source-lowering savings from graph sharing. No comparative cost registration should treat this kernel as a completed executor.

This continuation follows the [first-cycle selection](S00-first-cycle-feasibility.md): direct graph organization still offers a larger unresolved architectural contrast than expanding the output-heavy S01 grid. Completing source correspondence is necessary to test that contrast. After that gate, reassess it against the integrated and direct-compilation alternatives before extending graph-specific tuning.

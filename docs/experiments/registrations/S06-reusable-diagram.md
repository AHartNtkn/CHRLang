# Reusable finite formulas: denotation gate

T076 tests whether a compiled representation can operate on whole sets of possibilities instead of rebuilding a solver per observation. This package establishes exact finite denotation; it makes no timing claim.

## Decision and alternatives

Use a reduced ordered multi-valued decision diagram: each node selects a declared variable and has one edge per atomic name; equal children collapse. Conjunction, union and existential projection operate on shared nodes. Existing prepared name solving, finite projection, symbolic rebuilding and explicit early-rejection enumeration remain cost controls for subsequent trials. This is an experimental representation, not a selected language architecture.

This can change which work a reusable formula performs. The strongest ready alternative is cheaper relevant-read validation (T072); its attribution already locates a cost but still evaluates the same deductions. The diagram gate is selected under the recorded breadth review because it tests direct set operations, including currently untested union. Expected effort is one bounded implementation and denotation package; review source correspondence next, and portfolio breadth after at most four packages.

## Exact contract

A formula denotes a set of assignments to n variables over one finite alphabet of k distinct atoms. Equality/disequality constrain two coordinates. Logical conjunction and union are set intersection and union. Existential projection forgets selected coordinates: its denotation contains every complete assignment whose retained coordinates extend to a satisfying input assignment. It is therefore cylindrified over forgotten coordinates. Inclusion is ordinary set inclusion over the same declared universe. Neither union nor projection preserves raw CHR answer multiplicity; no consuming-source replacement is claimed here.

The arena owns all nodes and operation memo tables are local to an operation. A node handle is used only with its owning arena. Preparation and operations have explicit finite work limits and must report exhaustion; exhaustion never means false or an empty set. An exhausted operation releases its partial result; successful operations can retain intermediate nodes, which must be charged at disposal in later accounting. No global cache or fresh caller identity allocator is introduced.

## Prospective correctness matrix and controls

Independently enumerate full assignments, evaluate equality/disequality with integer comparisons, and compare every assignment with diagram membership. Use three variables, alphabet sizes 1, 2 and 3, all 8 disequality-edge subsets, with/without equality between coordinates 0 and 1. For every formula test all 8 existential masks and changing assignment queries. Compare all formula pairs for union, intersection and inclusion. Include empty and universal sets, redundant constraints, contradictions, equal branches, unknown coordinates/values, and explicit bound exhaustion. Repeat the deterministic gate in debug and release builds. Register any expansion before executing it.

Resource bounds: 60 seconds wall time per test executable, 1 GiB address space, and 1,000,000 diagram work units per ordinary operation. Small intentionally exhausted bounds are correctness witnesses. No benchmark matrix or statistical repetitions apply to this semantic gate. Preserve logs, exact sources and toolchain hashes before confirmation.

## Interpretation and next boundary

Agreement establishes these finite logical operations only. A failure requires a causal repair and rerun. Compact nodes demonstrate representation, not total efficiency: compilation may still be expensive, diagram order may be adverse, and output enumeration remains necessary when requested. Next establish correspondence for matched finite-name sources, then compare preparation, query setup, membership/full output, retention and disposal against existing controls. Structured unbounded terms, raw alternatives, source effects, identity lifetime and sustained costs remain independent required work.

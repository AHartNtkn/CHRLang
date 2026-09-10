# Symbolic answers preserve fresh holes and hidden constraints across callers

The joint logical interface can return unresolved term templates while preserving caller imports, repeated holes and hidden existential constraints. Four final confirming runs agree with independent membership checks. This supports a symbolic answer contract; it does not establish hidden-variable elimination, canonical answer sets or raw CHR equivalence.

## The representation and its meaning

A symbolic answer contains output templates, declared caller imports and a joint conjunction of name, structural and exclusion requirements. Variables absent from outputs and imports are existentially hidden. The conjunction remains attached to the answer: its hidden restrictions are not discarded merely because those variables are unobserved.

Instantiation maps every import to the supplied caller identity. Two imports may intentionally map to one caller variable, imposing the corresponding equality. Every other variable receives a fresh identity, used consistently across its output and formula occurrences. Fixed constructor and atom names remain unchanged. Transport concerns logic-variable identities, not alpha-equivalence of object-language binders.

The caller supplies occupied identities and a fresh-ID cursor. Allocation avoids occupied and imported identities, and commits its updated state only after the complete instance succeeds. Missing or extra import mappings and identity exhaustion return errors without changing allocator state. Independent instantiations therefore share only declared imports when they use the same allocator correctly.

The consistency endpoint rejects direct binding or capture of hidden existential identities. It checks reachable output bindings for finite-tree cycles, then invokes the existing joint feasibility service for the logical requirements. Caller equality remains external. The endpoint does not maintain an incremental equality store or produce canonical residual formulas.

## Independent evidence

| Check per final execution | Scope | Result |
|---|---|---|
| Symbolic membership | 12,096 combinations of requirements, exclusion graphs, import modes, finite/unbounded names and ground caller assignments | Agreement with independent ground evaluation and hidden-name enumeration |
| Output identity | Returned X/Y and `pair(P,P)` with an unconstrained fresh P | Repeated holes share identity; fresh locals avoid caller identities and other instances |
| Intentional caller aliasing | X and Y imported as the same caller variable | Shared identity and logical consequences are preserved |
| Late bindings | Hidden H must differ from visible X/Y over two names | Partial bindings remain feasible; two distinct assigned visible names make the conjunction false |
| Invalid requests | Missing/extra mappings, identity exhaustion, hidden capture and output cycles | Explicit errors; failed transport leaves allocator state unchanged |
| Finite decoder comparison | A returned nonground pair supplied as a finite ground-domain value | Existing finite decoder rejects it; symbolic output retains its unresolved holes |

The matrix crosses four name subsets, eight exclusion graphs, three structural templates, three requirements, three import modes and finite/unbounded name interpretations. Ground visible assignments use two atoms and a lambda witness. Incompatible assignments to intentionally aliased imports are excluded because they are not distinct caller states. The independent oracle supplies a third fresh name for the hidden variable when the unbounded case needs it.

Four final confirming executions pass, two with structural metrics enabled and two disabled. Each passes all three tests and 12,096 membership comparisons. Another 65 existing tests pass across 15 executables, including the zero-test library target. Earlier pre-lint confirmations and recoverable artifacts are preserved separately; the final claims use the final source and binaries. Scoped Clippy and formatting pass. No executable reaches its 60-second wall/CPU or 1 GiB address-space limit.

An additional test exposed a real interface gap before final confirmation. A cyclic binding through output-only P was invisible to logical requirements that never mentioned P. The endpoint now checks reachable output bindings as well as the conjunction. This validates the output's finite-tree obligation without forcing an unconstrained hole into the normal-form grammar. The failing test and passing correction are retained.

## What this changes architecturally

Finite ground domains are not required merely to preserve unresolved output aliases. An exact symbolic conjunction can retain those aliases and delay its logical feasibility work until caller bindings arrive. For the qualified theories, consistent renaming preserves the formula's denotation; the independent cases exercise both fresh renaming and intentional import aliasing.

The interface still carries consequential work. It retains hidden formulas, reconstructs identities, copies term templates and rechecks constraints. The fresh allocator retains an occupied-ID set in this prototype. None of those costs has been measured here, and no sustainable-memory claim follows from successful transport.

Symbolic representation and finite elimination now expose different responsibilities. Finite elimination materializes coordinate relations and projected ground sets. Symbolic answers retain existential formulas and can describe infinitely many fillings. Comparing the cost of returning one formula with enumerating every ground answer would compare different observations. Raw CHR residual occurrences and their observer/consumer effects remain a separate contract, as established by the [joint source entry](S06-joint-entry.md).

Overlapping symbolic alternatives remain unresolved. This entry handles one conjunction at a time; it neither decides general inclusion/equivalence between formulas nor constructs a canonical union. Hidden-variable elimination, broader source closure and the lifetime of reused identities also remain required investigations.

## Next experiment

Register lifecycle qualification for matched finite-name observations through symbolic conjunctions, finite projection and independent explicit enumeration. Use the same logical formula and caller restrictions; charge preparation, fresh transport, membership or complete finite output as requested, retained answers and disposal. Separate membership queries from complete-answer enumeration. Exercise selective and output-heavy cases, changed callers and immediate versus retained consumer ownership before timing.

The strongest ready alternative is symbolic-union/inclusion analysis. It could change observation complexity across multiple alternatives, but the single-conjunction paths already admit a meaningful matched comparison. Lifecycle qualification comes first because it can expose whether retaining formulas and fresh-identity state merely shifts the cost of avoiding ground materialization. It cannot settle the union question, which stays open.

Reconsider union analysis, local branch-copy attribution and demand-driven choices at that gate. This is package three after the guarded-choice breadth review; one further package triggers a full review. No efficiency winner, language adoption or complete architecture selection follows. The research goal remains active.

## Evidence

[Registration](../registrations/S06-symbolic-transport.md), [symbolic interface](../../../research/chr-structural/src/symbolic.rs), [independent tests](../../../research/chr-structural/tests/symbolic_transport.rs), [runner](../../../research/chr-structural/experiments/symbolic_transport.py), [final audit](s06-symbolic-transport/audit.json), [frozen inputs and binaries](s06-symbolic-transport/freeze.json), [earlier receipts](s06-symbolic-transport/pre-lint/) and [raw evidence](s06-symbolic-transport/). Forty inputs are frozen for final confirmation. Existing solvers and the reference implementation are unchanged; prior library and pre-lint source snapshots preserve earlier evidence.

# Shared constructor storage preserves separate equality and claims

**The contextual store shares immutable constructor storage while keeping sibling bindings, consumption and failure isolated.** It agrees with an independent owned-substitution evaluator on 1,296 ordered equation-pair configurations, including complete exports and repeated-variable occurrence matches. This is a store correctness result; a complete source executor and cost comparison remain necessary.

## The mechanism exercised

An append-only arena owns unknown identities and constructor nodes. Forked stores share that arena. Each context has its own equality-parent map, merged-class descriptors, pending deductions, failure state and live occurrence map. Map snapshots are shared until a context mutates them. Occurrence records retain distinct identities even when their values are equal.

Matching reads constructors directly through the current context's classes and scans its live occurrences. It does not serialize a solved term tree into a relational view before matching. The same open constructor and token occurrences can therefore be available under different sibling bindings. A successful atomic claim changes only its context's live map. A subsequent contradiction invalidates that context without affecting its sibling or parent.

This differs from cloning the entire relational store: constructor storage is physically shared, and matching reads context-local equality information directly. It remains a serial ownership design. It does not implement distributed claims, strategic port rewriting, or cross-context reuse of deductions performed after a fork.

## Evidence and adverse cases

The focused witness begins with `open(f(X))` and two distinct `ticket(X)` occurrences. Forked contexts bind X to a and b respectively. Each has two valid candidate claims. Claiming one pair prevents reusing its consumed open occurrence in that context, while the sibling and parent retain their own candidates. Contradicting the first sibling leaves the second valid.

Additional checks cover branch-local posting, direct finite-tree cycles, access to an outer merged constructor while child equalities remain pending, rejection of partial exports, distinct kept/removed identities, stale claims and preservation of kept occurrences. The physical arena-sharing check accompanies behavioral checks; sharing a pointer alone would not establish ownership correctness.

The generated gate uses eight terms: X, Y, a, b, f(X), f(Y), f(a), g(X). All 36 unordered equation pairs are followed by all 36 pairs in a fork of the first result. Each successful context's jointly normalized exports and complete repeated-variable match set must agree with independent recursive substitution. Invalid finite-tree equations must fail without exporting answers. The oracle has no arena, parent classes, context maps or matching indexes.

Three contextual tests pass, alongside the existing 14 relational tests. Strict Clippy for the changed crate passes; existing integrated build-script dead-code warnings remain in the receipt. Reference implementations were unchanged. The [registration](../registrations/S02-contextual-store-gate.md), [tests](../../../research/chr-relational/tests/contextual.rs), [gate log](s02-contextual-store/gate.log), [full tests](s02-contextual-store/full-tests.log), and [Clippy](s02-contextual-store/clippy.log) provide the commands and checks.

## Costs and responsibilities still exposed

This implementation introduces context-map ownership and indirect class lookup in exchange for sharing constructor storage. A first mutation can copy an inherited map. Matching scans occurrences, recursively follows descriptors, and constructs candidate bindings. The arena retains nodes from failed or discarded alternatives until the final owning store is released. These choices can be expensive; no allocation or timing claim is made here.

Stable identities allow sharing; they do not prove that two source applications can reuse a resource effect or a newly computed equality consequence. Source history, fresh body variables, choice birth, guarded application, exact residual publication and fair service still require an executor. The current low-level Value and Match interfaces also assume their caller uses identities and claims from the appropriate arena/context; they are not a checked language boundary.

## Next decision-bearing work

Integrate this store into a complete source path with explicit pending effects and fair alternatives, then compare source answers with the independent scalar evaluator and cloned relational execution. The first sources must include a common prefix followed by different consuming alternatives, later failure, kept heads, repeated variables, fresh body outputs and a finite sibling beside ongoing work. Establish those obligations before timing.

Only that complete path can establish whether the shared constructor representation repays context lookup, map copying, discovery and retention. The strongest comparison is the corrected relational executor, which already supports useful partial-equality interleaving. T072 remains active; this gate does not settle S02's other integrated organizations or select an architecture.

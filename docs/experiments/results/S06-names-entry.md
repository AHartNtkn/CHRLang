# Names need a domain and an observation contract

The names entry preserves finite logical solutions, but that does not make its compact formulas interchangeable with CHR answers. The literal program retains `var` occurrences and accepts more constructor shapes than a declared atomic-name domain. Those differences are observable and must remain explicit architectural choices.

## What “names” means in the recorded proposal

The proposal in [the solver analysis](../../goals/chr-sharing/notes/T015-solver-certificates.md) concerns represented names: constants used as lambda variable names. The [projection analysis](../../goals/chr-sharing/notes/T016-exact-solver-projection.md) proposes either an injective finite-signature encoding or a separate name theory. Neither proposal establishes nominal abstraction or alpha-equivalence of lambda binders.

The current authoritative [literal translation](../../../crates/chr-programs/src/lib.rs) rejects `var(app(X,Y))` and `var(lam(X,Y))`. Constructor arity matters. `var(app)` and `var(app(X))` remain residual, as does `var(other(X))`. A domain that admits only atomic names changes the latter two cases. This makes the domain boundary more precise than describing rejection by constructor spelling alone.

Fixed names and logical variables have different roles. `lam(x,x)` and `lam(y,y)` are distinct observed terms under the current contract. Fresh logical variables can be renamed when transporting an answer, but two such variables may later receive the same name. Their initially different identities do not establish disequality. The existing literal-substitution tests remain unchanged and pass; this entry introduces no capture-avoidance or binder quotient.

## The implemented comparison

The experimental [name formula](../../../research/chr-structural/src/names.rs) represents a conjunction of root restrictions. Known admissible roots discharge a logical obligation; a prohibited root makes the formula inconsistent; unknown roots remain restricted variables. Repeated restrictions can share one formula entry. Refinement rechecks restrictions after caller bindings, and transport requires a complete injective mapping of the restricted variables.

Two explicit contracts are compared. **Literal roots** prohibits only `app/2` and `lam/2`. **Atomic names** admits nullary constructors and prohibits structured values. Neither interprets an unbound logical variable as a completed name value. The caller still owns finite-tree equality, freshness relative to its other state, and bindings; this small formula component is not a complete equality solver or a CHR executor.

An independent oracle substitutes ground values throughout each tree and evaluates the selected domain. The candidate instead compiles root restrictions first and refines only the remaining variables. The unchanged reference separately executes the full literal lambda program. Its expected answers are checked for complete failure or the exact residual multiset, including repeated obligations and partial aliases.

| Evidence per names-test execution | Cases | What it establishes |
|---|---:|---|
| Two domain interpretations against independent ground denotation | 17,934 | Compiling and refining restrictions preserves the tested assignments |
| Literal source on ground obligations | 8,967 | Exact residual multiplicity or failure agrees with the source expectation |
| Literal source on partial obligations | 183 | Unknown restrictions and aliases remain present |
| Explicit finite-alphabet assignments | 1,224 | A compact restriction still represents many distinct assignments when enumeration is requested |

The four confirming executions—two default and two metrics-off—pass. Four earlier executions also pass; their test source is preserved in a snapshot after a Clippy correction to slice construction. The frozen matrix contains 13 term templates, all ordered lists of zero to two obligations, and seven ground values for each of two variables. These are finite checks, not a proof about arbitrary source programs.

A separate source extension passes 24 delayed-binding queries across duplicates and rule order, plus a late alias-chain failure. A `var(X)` occurrence remains available to reject a later `app/2` or `lam/2` binding. The structural and lambda regression run passes 53 tests across 11 executables, one of which contains no tests. The four-test names executable is counted separately. Final Clippy and formatting checks pass. Reference code and existing independent oracles are unchanged.

## What cannot be optimized away

**A true logical restriction can remain an observable CHR occurrence.** With `var(x)`, a host propagation rule can produce `seen(x)`. Replacing that source occurrence with the logical formula `true` would lose the effect. Duplicate restrictions likewise cannot disappear from raw answers merely because conjunction is idempotent. Formula projection needs a declared private logical interface or an independently established source boundary.

**A compact answer is not an enumeration speedup.** Six unknown names over a three-name alphabet have 729 complete assignments. The formula retains six restrictions, but an endpoint requesting all assignments still has 729 outputs. The finite enumeration tests make that distinction explicit; they do not invent source alternatives. With an empty alphabet, a nonempty set of name variables has no assignment. The candidate's atomic-name domain is open-ended; finite-alphabet enumeration is a separate test restriction, not an implemented finite-domain solver.

**Constructor rejection alone offers little work to eliminate on a known root.** The candidate performs a root check, and the literal source has two corresponding rejection patterns. This observation does not rank their implementations or rule out a benefit when combined with equality, projection or other theories. It does make a name-only timing matrix less discriminating than testing disequality, where an unknown restriction can later contradict a binding and the available alphabet affects satisfiability.

## Consequence and next experiment

The entry establishes a usable, independently checked name-restriction component and concrete boundaries for integrating it. It does not justify adopting a name domain, changing lambda equality, compressing source residuals or selecting a solver architecture. Finite-signature name encoding, general source certification, ownership, combined-theory solving and complete costs remain required.

Next investigate delayed name disequality under T076. Compare explicit finite assignments with a symbolic constraint representation, testing aliases, contradictions after binding, hidden variables and caller transport. Distinguish finite alphabets from an unbounded supply of represented atoms. This can change whether equality classes and retained exclusions avoid useful search, whereas further root-check timing would mainly measure a small local implementation. Integrated execution and continuing conditional ownership remain ready alternatives at that next gate or an obstruction.

This is the first theory-entry package after the conditional breadth review; the additional source wake-up qualification counts as a second validation package toward the four-package review boundary. No research-completion claim follows.

## Reproducible evidence

The prospective [entry registration](../registrations/S06-names-entry.md) and [source wake-up extension](../registrations/S06-names-source-wakeup.md) define the matrices and limits. [Raw runs](s06-names-entry/), [audit](s06-names-entry/audit.json), [final frozen inputs](s06-names-entry/final-freeze.json) and the [runner](../../../research/chr-structural/experiments/names_entry.py) preserve source and binary identities, assertions and process outcomes. Confirming test processes use 60-second wall/CPU and 1 GiB address-space bounds; each reference query must exhaust within 1,000 steps. Test durations support no comparative timing or lifecycle claim.

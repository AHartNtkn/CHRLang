# The structural theories compose for feasibility, but raw source effects remain

Atomic-name, name-disequality and normal/neutral requirements agree with independent feasibility checks when they share caller variable identities. Their overlap is simpler than a general combined solver: every atomic name is also normal and neutral. This supports an explicit logical interface, not transparent replacement of CHR residual occurrences.

## The joint interface and its responsibilities

The experimental endpoint accepts name obligations, structural requirements, name exclusions and a caller-supplied finite-tree substitution. It answers whether some filling satisfies the conjunction. It reads the substitution without modifying it; the caller still owns consistent equality, matching and consuming execution.

The endpoint expands reachable caller bindings once per supplied term, detects reachable cycles, and passes normalized requirements to the existing components. It maps remaining name variables to common exclusion identities. Reflexive substitutions are harmless; unrelated caller cycles are outside the requested terms and are not inspected. This is a feasibility entry, not an incremental equality store, prepared solver, projected answer enumerator or performance optimization.

The optional finite alphabet restricts explicit name obligations and exclusion endpoints. It does not restrict every otherwise unconstrained structural leaf. Normal/neutral semantics retain the existing treatment of lambda binder interiors. These are declared experimental semantics; they introduce neither nominal binding nor alpha-equivalence.

## Why the components can agree without another structural search

After expansion, the structural summary assigns each unresolved variable a strongest positive requirement: domain, normal or neutral. Every atomic name satisfies all three. A variable also constrained as a name can therefore use its name-solver assignment; each remaining structural variable can use an arbitrary atom. Repeated occurrences receive the same value.

Consequently, once the structural summary and atomic-name checks succeed, feasibility depends on the name equality/exclusion problem. Finite alphabets may require an actual search for compatible names. An unbounded alphabet permits fresh distinct names for unresolved exclusion classes. This is an analytical extension argument under the stated theory definitions, supported by the bounded independent tests below. It is not a claim about general combined theories, richer binders or exact projected outputs.

Fixed constructor information still matters. Binding a name-required variable to a lambda makes the conjunction false, even though that lambda may satisfy a normal requirement. Aliasing two excluded variables also makes it false. Sharing caller identities is therefore necessary; independently checking formulas under inconsistent variable mappings would not implement this interface.

## Independent evidence

| Check per confirming execution | Scope | Result |
|---|---|---|
| Joint partial feasibility | 3,840 combinations of structural templates, requirements, name subsets, exclusions, caller snapshots and alphabets | Agreement with independent ground semantics and enumeration |
| Binding and dependency witnesses | All six orders of three bindings, alias conflicts, repeated holes, reflexive/reachable/unrelated cycles | Expected outcomes; caller maps remain unchanged |
| Closed ground source existence | 640 complete literal CHR queries with required atomic companions | Agreement with the logical endpoint and independent evaluator |
| Raw behavior witnesses | Surviving name/exclusion occurrences, kept-head observation and consuming host effects | Logical truth does not preserve those raw effects by itself |

The independent enumerator uses five atoms and three structured witnesses, preserving all three variable identities. Its domain is sufficient to supply or refute witnesses for the registered templates; it is not a general bounded-enumeration completeness theorem. The source matrix includes a redex that must fail, not only satisfiable name terms.

Four confirming executions pass, two with structural metrics enabled and two disabled. Each passes all three tests, including 3,840 feasibility comparisons and 640 source queries. Another 59 existing tests pass across 13 executables, including the zero-test library target. Scoped Clippy and formatting pass. All runs stay within 60-second wall/CPU, 1 GiB address-space and 2,000-turn source bounds. Reference and existing component implementations are unchanged.

## The language boundary is observable

A hidden name constrained to differ from two distinct visible names has no value over a two-name alphabet. It does have a value with unbounded fresh names. Projecting away that hidden name cannot simply discard its finite-domain restrictions. A future exact-output interface must preserve that dependency.

Ground `var(a)`, `norm(a)` and `neq(a,b)` form a logically true conjunction under the tested contract. Literal CHR execution still retains `var` and `neq` occurrences. A kept-head observer can produce `seen(a)`, and a consumer with a permit can produce `taken(a)`. Replacing the conjunction by a logical success answer cannot reproduce those effects.

The 640-query agreement therefore has a specific scope: ground, closed companion-equipped sources observed only for answer existence. It establishes neither raw-answer equivalence nor a general closure certificate. Inference, checked declarations and mandatory restrictions remain separate language alternatives; this experiment adopts none of them.

## Next investigation and priority

Next register an executable closed logical boundary with exact projected answers, including hidden finite-name dependencies and returned aliases. Compare it with explicit enumeration of the same declared logical contract, and preserve the literal CHR observer/consumer witnesses as exclusions from transparent replacement. Eligibility must be explicit before claiming that the solver can eliminate source execution. Selective, output-heavy and changing-query cost comparisons follow a qualified answer/ownership boundary.

The strongest ready alternative remains local branch-copy/service attribution and a demand-driven or representation contrast. That could change the local/contextual crossover, but the cost pilot already keeps both candidates viable. Exact joint output and source eligibility come first because this entry identifies a concrete unsolved dependency: finite hidden names affect visible answers, while raw host effects forbid unrestricted replacement. A runtime matrix for the feasibility wrapper would not answer that architectural question.

Reconsider branch-copy attribution, demand-driven choices and conditional lifetime at the exact-output/source gate or an obstruction. This is package one after the guarded-choice breadth review; three further packages trigger another full review. No solver-cost advantage or complete architecture selection follows. The research goal remains active.

## Evidence

[Registration](../registrations/S06-joint-entry.md), [joint endpoint](../../../research/chr-structural/src/joint.rs), [independent and source tests](../../../research/chr-structural/tests/joint_theories.rs), [runner](../../../research/chr-structural/experiments/joint_entry.py), [audit](s06-joint-entry/audit.json), [frozen inputs and binaries](s06-joint-entry/freeze.json) and [raw receipts](s06-joint-entry/). The registration and 36 input hashes are frozen before confirmation. The failing composition test and subsequent passing runs are preserved. The prior library input is snapshotted to retain earlier experiment provenance.

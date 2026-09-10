# Structural normality is stronger than literal normalization rules

The structural checker preserves the tested normal/neutral denotations and shared-hole requirements. It also rejects some ground terms that the literal CHR rules leave residual until a companion `var` occurrence arrives. A complete logical recognizer therefore needs an explicit source boundary; it cannot silently replace those rules.

## The represented property

The declared tree domain contains atomic names, `app/2` and `lam/2`. A neutral term is a name or an application of a neutral term to a normal term. A normal term is neutral or a lambda whose body is normal. The binder position has only the domain requirement, matching the [recorded interpretation](../../goals/chr-sharing/notes/T015-solver-certificates.md). Binder validity, capture avoidance and alpha-equivalence are separate questions.

The [partial-tree compiler](../../../research/chr-structural/src/normal_forms.rs) propagates these three requirements through known constructors and retains requirements on holes. Their inclusions are `Neutral ⊆ Normal ⊆ Domain`, so repeated uses of one hole must retain the strongest requirement. Refinement expands finite caller bindings and checks the resulting constraints; it neither generates constructor assignments nor owns the caller's equality store.

For example, normality of `app(H,H)` requires H to be neutral. A lambda can be normal but cannot fill H here. Treating the two occurrences of H independently, or retaining only a normality restriction, would accept an invalid filling. A known `app(lam(...),...)` can instead be rejected while its children remain unknown.

## Three independently checked views

The ground oracle first checks the declared domain, then checks for beta-redexes through application children and lambda bodies. Binder interiors are excluded from the normality check itself, as the predicate requires. Neutrality additionally excludes a lambda root. This oracle does not use the candidate's requirement propagation.

The existing regular-tree automaton supplies a second representation control, with explicit grammar states for the three languages. The source control separately runs the unchanged reference and the full literal lambda program. Its expected residuals come from the five literal structural equations under a fixed set of companion names, not from assuming the complete recognizer is source-equivalent.

| Evidence per confirming execution | Cases | What is compared |
|---|---:|---|
| Ground memberships | 2,142 per representation | 714 trees, three requirements; candidate and regular automaton versus the independent oracle |
| Partial-tree assignments | 164,400 | 548 templates, 100 assignments to two holes and three requirements |
| Complete literal source queries | 2,856 | 714 ground trees crossed with no companions, `var(a)`, `var(b)`, or both |

All four confirming executions pass: two default and two metrics-off. Named checks cover shared requirements, late aliases, cyclic binding rejection, unknown children, unsupported constructors and partial source residuals. Another 55 structural regression tests pass across 12 executables, one of which has no tests. Clippy and formatting checks pass. No reference or existing oracle implementation changed.

## The observable source difference

Consider `norm(app(a,app(lam(b,b),a)))`. The structural interpretation is false because the right argument contains a beta-redex. Without `var(a)`, however, the literal source leaves the whole `norm` constraint residual: its outer application rule lacks the companion occurrence needed to inspect that argument. Adding `var(a)` exposes the failure.

The source matrix checks this distinction generally. It compares exact residual multisets or failure, including the companion occurrences that survive rule execution. It does not classify every residual `norm` as a successfully recognized normal form. Similarly, `norm(X)` with `var(X)` leaves `var(X)`, preserving the restriction on future bindings and the source occurrence.

The declared domain is another boundary. An `other/1` term is outside this structural theory, while its literal `norm` occurrence can remain residual. Domain certification and ownership of these predicates are therefore prerequisites for using the complete recognizer to prune source execution. They cannot be inferred from the predicate names alone.

## Architectural consequence

These constraints can represent restrictions on partially specified programs without enumerating their fillings. Repeated holes require shared variable information in addition to regular structure. That supports carrying the component into a combined logical solver experiment; it does not establish that a standalone regular automaton can encode arbitrary equality or disequality correlations.

The entry does not measure compilation, conjunction with name exclusions, exact existential summaries, source-boundary checking, retained ownership or complete output costs. The finite checks also do not prove arbitrary source correctness. Recognizing a restriction is a different endpoint from enumerating all satisfying programs or reproducing all CHR derivations. No speed, memory or architectural winner follows.

The [four-package breadth review](S06-normal-breadth-review.md) now selects explicit-choice ownership for the source-driven local graph path. Further structural-theory integration remains required and is its strongest ready alternative. The architecture research goal remains active.

## Evidence

[Prospective registration](../registrations/S06-normal-entry.md), [frozen inputs and binaries](s06-normal-entry/freeze.json), [raw receipts](s06-normal-entry/), [audit summary](s06-normal-entry/audit.json), and [runner](../../../research/chr-structural/experiments/normal_entry.py). Confirming executables use 60-second wall/CPU and 1 GiB address-space limits; reference queries must exhaust within 2,000 steps. Test durations support no comparative cost claim. The [prior module snapshot](s06-normal-entry/before/structural-lib.rs) preserves the preceding disequality entry's import input and matches its frozen hash.

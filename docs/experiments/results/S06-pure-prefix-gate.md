# Source-derived inlining removes private calls while preserving resource order

**The compiler now derives a finite inlining transformation from source structure, without comparing the program with a known schema.** Eligible examples preserve complete answers under renaming, changed constants, fresh variables and competing resource consumers. An execution trace reduces a three-call private chain plus consumption from four source applications to two, including the generated entry rule.

This is a correctness and work-elimination result. It does not establish lower total cost or a general direct compiler. Required equations, choices and resource execution remain, and code generation can increase artifact size.

## Existing implementations and the distinct question

The audited [sealed-region specialization](../../../research/chr-compiled/src/regions.rs) compiles ordered single-occurrence matching but retains source execution. [Carrier contraction](../../../research/chr-compiled/src/carriers.rs) skips a recognized pure decreasing step while preserving arbitration at its actual tail. The [recursive compiler](../../../research/chr-compiled/src/recursive.rs) removes the occurrence store for one certified deterministic, ground-controlled recursive call. [Finite relation compilation](../../../research/chr-finite/src/model.rs) solves a separately bounded finite relation. Native generated access compiles rule operations rather than proving their execution unnecessary.

The new pass instead removes an acyclic set of private source calls containing equations and explicit finite choices, including calls whose variables later participate in ordinary consuming rules. It emits CHR with those calls expanded. The residual source still uses an ordinary executor; this pass does not claim to replace its resource selector or unifier.

## Inferred conditions

The selected predicates form a leading source-rule prefix. Each has exactly one source head use: one removed head with distinct variable parameters, no kept heads and no guards. Its body contains only equations, true/fail, finite choices, conjunctions and calls within the selected prefix. The call graph must be acyclic. Predicate names, constructor names, variable numbers and query layouts are read from the source rather than fixed by the experiment.

The compiler finds the largest closed acyclic prefix satisfying those checks. It freshens nonparameter variables for each expansion, substitutes actual arguments and expands calls inside remaining bodies. For initial queries it emits a finite entry rule whose parameters carry the query variables, preserving their relationship to outputs. Remaining resource rules retain relative order. Expansion has a 100,000-processed-goal-node bound; compilation failure is explicit.

**The source-order condition is substantive.** Consider a high rule consuming `p(a), token`, a lower rule consuming `p(X), token`, and a still-lower rule binding X to a. Under the original order, the lower consuming rule wins before the binding. Moving the binding first makes the high rule win. The executable counterexample produces low versus high residuals. The compiler rejects this arrangement instead of treating equation purity as sufficient to move it.

An external head observing a private occurrence also blocks elimination. Constructor-dependent heads, guards and recursion fail the prefix condition unless some separate earlier prefix is eligible. These are sufficient implemented checks, not necessary language restrictions: a rejected source may admit a stronger certificate. Artifacts are validated for source-order/Global execution; no Active-policy guarantee is made.

## Why the transformation is plausible—and what is tested

A private head in the selected prefix always matches its own occurrence, cannot compete with another rule for that occurrence, and performs no resource effects. Its finite expansion produces logical equations and explicit alternatives. Under source-order selection, pending private calls are serviced before remaining resource applications. Inlining moves that finite pure work into pending bodies without permitting resource rules to observe or claim the private occurrences.

This is the reasoning behind the sufficient conditions, not a formal compiler proof. Capture-free substitution, interaction with body boundaries, raw alternative multiplicity and the stated scheduling contract must still be validated. The gate includes eight combinations of predicate renaming, variable renaming and constructor constants; four shared-token cases varying aliases and query order; distinct fresh results from two calls; a traced chain; and rejection/counterexample cases.

The renamed and resource-coupled finite examples agree with the independent owned source evaluator before and after transformation. Compiled Global scanning and indexing agree on those cases too. Two calls sharing an input variable retain two successful alternatives where independent calls retain four; fresh variables from separate applications remain distinct while repeated uses within one result remain aliased.

A separate source leaves ordinary recursion in place and has one finite sibling using an inlined binding call. Both compiled access modes publish the expected finite answer within 500 advances and remain live for another 32 advances. This is a bounded service witness, not a universal advancement-cost bound.

The trace test proves actual source-application elimination. Original execution fires p, q, r and use. The emitted program fires its entry rule and use. Both publish the same full answer. Thus the pass is more than changed labels or an eligibility report, while the trace also makes the added entry cost visible.

## Architectural and language consequence

Exact-schema recognition is not required to remove all such private source execution. The transformation works on source-derived predicates, arguments, nested calls and finite choices, with ordinary consuming behavior preserved under its conditions. It supplies a new point between generic prepared execution and a complete direct solver.

The restriction to a pure, private, acyclic leading prefix is substantial. It does not address effectful recursion, selective heads whose eligibility depends on query groundness, intervening observers, or contextual contraction across competing resource effects. No declaration currently buys information the inference lacks within this fragment, so this package introduces no new declaration or mandatory language restriction.

Next compare compilation plus changed-query execution and artifact disposal against the same original sources. Vary chain/branch expansion, number of calls, external ordinary work and query reuse. Charge entry overhead, repeated lowering, freshening and code growth; compare with existing sealed/generated execution where eligible. A single-call or cold query is necessary counterpressure to the traced chain. Register costs prospectively after bounded sizing and revisit selection at that boundary. T073 and broader S06 remain open.

## Evidence

[Registration](../registrations/S06-pure-prefix-gate.md), [compiler pass](../../../research/chr-compiled/src/pure_prefix.rs), [behavioral and trace tests](../../../research/chr-compiled/tests/pure_prefix.rs), [default gate](s06-pure-prefix/gate.log), [counter-free gate](s06-pure-prefix/counter-free.log), and [Clippy](s06-pure-prefix/clippy.log). All seven tests pass in both configurations; strict scoped Clippy passes. The independent evaluator and existing runtime behavior were unchanged.

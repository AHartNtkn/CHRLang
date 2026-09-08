# A relational head plan can select constructors before source occurrences

The new head-plan prototype finds the same candidate occurrences and variable bindings as an independent exhaustive matcher on 1,664 small configurations. It can begin with a selective inner constructor and use that result to find source occurrences. **This establishes the proposed matching mechanism; equality maintenance and complete source execution are still required.**

The [registration](../registrations/S02-relational-head-gate.md) defines this first gate. The implementation is in [chr-relational](../../../research/chr-relational/src/lib.rs), with [independent tests](../../../research/chr-relational/tests/head_plan.rs). It depends only on owned syntax data and does not call the reference interpreter or an existing executor.

## The boundary being tested

The existing integrated control already uses constructor descriptors and value identities during matching. It does not export solved terms into another store before matching. Its remaining path selects an occurrence, recursively walks its source pattern, then checks the other occurrence heads. A partially specified constructor such as `f(a,X)` cannot use that control's complete-value lookup until X is known.

The new plan compiles constructor patterns and source heads into atoms sharing logical slots. Constructor atoms relate parent identity, symbol and child identities. Source atoms relate occurrence identity, predicate and argument identities. Each join step chooses the smallest available relation or bound-column bucket. It then checks repeated slots and distinct occurrence identities. Candidate results retain kept and consumed head positions in source order.

In the selective witness, twenty `open(f(letter,unknown))` occurrences contain only one inner `a`. The query begins with the one-row `a` constructor relation, then reaches the matching `f` parent and `open` occurrence. The resulting binding points to the correct unknown. This differs from merely flattening a recursive pattern walk into instructions that must still start from an occurrence. It is not yet evidence that the join planner is faster.

## Evidence and interpretation

The main grid covers eight values, both arrival orders, five head patterns and all their kept/removed splits: 1,664 configurations. Values include unknowns, different atoms, constructors over unknowns, repeated aliases and distinct unknown fields. The independent matcher enumerates occurrence tuples and matches owned term trees without using the candidate's plan or relation indexes. Tests compare the entire tuple-to-variable-binding map, not only counts.

Additional witnesses establish that equal-valued source occurrences remain distinct, one occurrence cannot fill two heads, and duplicate constructor proofs do not multiply source applications. Fresh equality snapshots distinguish unknown structure from established constructors and independent unknowns from actual aliases. Separate left/right views demonstrate that an alternative containing `f(a)` and another containing `f(b)` have different eligibility for an `f(a)` head.

The initial three tests failed on unimplemented operations before implementation. All four final test functions, replay, Clippy with warnings denied and formatting pass within the 60-second process bounds. [Commands and hashes](s02-head-plan/validation.json), [tests](s02-head-plan/tests.log), [replay](s02-head-plan/replay.log). No timing, work-saving or architecture ranking is claimed.

## Responsibilities this gate does not supply

**A relation view is a tested input contract, not an equality solver.** Its value identities must represent the established finite-tree equality state. The prototype does not yet maintain congruence or repair keys after equality changes. Rebuilding fresh views in tests checks snapshot meaning only; it does not validate incremental repair cost or behavior.

**Context-local equality remains a runtime obligation.** Joining an `f(a)` fact from one alternative with an occurrence from an incompatible alternative is unsound. Separate views are a correctness control, not a selected efficient architecture. A shared implementation must attach compatible contexts to facts/joins or otherwise preserve that separation. A global unqualified class merge is insufficient.

**The view cannot justify a global equality-completion barrier.** The existing integrated control can use established facts while unrelated deductions remain pending. The new runtime must preserve that useful interleaving and cannot treat all outstanding repair as a prerequisite for every source match. It must also prevent publication after a pending contradiction, even if some positive joins have already fired.

**Candidate matches are not committed applications.** Guards, fresh body variables, consuming claims, propagation history, source goals, failure and answer publication are not executed by this package yet. The future source owner must revalidate live resource identities when a candidate is committed. Equality-only aliases must remain valid; constructor cycles and clashes must fail their interpretations independently of which result roots are observed.

The current materialized `View` is suitable for this semantic gate. A runtime must own these facts directly or charge any view construction and duplicated indexes. Adding a shadow relation export beside the existing matcher would not establish that a representation boundary had been eliminated.

## Next comparison

T063 remains active. Next, connect relational plans to an actual equality/resource owner and validate complete source effects, including equality enabling a selective constructor join while unrelated deduction work remains pending. Contrast sparse useful updates with broad alias repair and few useful matches. The read-only plan is not a substitute for that work.

Flat relational joins are selected for the first S02 contrast because they directly test the visible matching boundary and have a precise independent oracle. Richer local incidence rewrites and contextual equality representations remain distinct alternatives. The existing integrated engine is a control; its representations and queue interfaces are not mandatory requirements for the new candidate. S01 retained matching and S06 direct relation compilation remain open, with any shared analysis justified by actual semantic equivalence rather than similar terminology.

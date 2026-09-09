# S03: identity requirements for reusable source derivations

This is a semantic countermodel experiment, not a timing comparison or an implementation of memoized pull-tabbing. It decides whether a reusable application may return the same concrete result graph for equal inputs, and whether result reuse can bypass occurrence claims.

Before execution, fix three source witnesses:

1. `make(k,R) <=> R=box(Z)` with fresh body-local Z. Compare one request used twice with two independent requests on the same ground key. Two calls require distinct unknowns; sharing a concrete result would introduce an unjustified alias even though the rule is deterministic.
2. `make(k,R) <=> R=a \/ R=b`. Compare one call used twice with two independent calls. Expected raw answer counts are 2 and 4. Sharing a result choice across calls would correlate independent births.
3. `ask(k) \\ token(k) <=> out(k)` with two distinct equal-valued asks and one or two distinct equal-valued tokens. Expected output counts are 1 and 2. Reuse of the body result cannot justify another consumption; equal-valued tokens cannot be collapsed.

For all six inputs, compare complete answers from the independent scalar source oracle, compiled global scanned/indexed execution, Conditional execution and the direct named-choice graph against hand-derived answers. Use exact joint alpha-equivalence and raw answer multiplicity. Explicitly check that the incorrect alias/correlation/effect predictions disagree with the expected answers. These checks reject concrete sharing rules; they do not claim an implemented cache has been mutation-tested.

The existing harness caps the scalar oracle at 200,000 operations and each engine at 2,000,000 ticks. Run the focused test filter with a 60-second process timeout, once with default features and once with default metrics disabled. No comparative timings, allocation ranking, RSS claim or reference changes. Existing source/progress gates remain required for the eventual candidate; this test does not repeat their coverage.

Interpretation: if the complete controls agree, use the countermodels to distinguish reusable derivation templates from reusable concrete event results. If a control disagrees, diagnose that disagreement before selecting an implementation. A concrete-result sharing counterexample does not reject pull-tabbing or effect-aware derivation reuse. The next implementation must state which identity it retains, what it instantiates freshly, and which context justifies every effect.

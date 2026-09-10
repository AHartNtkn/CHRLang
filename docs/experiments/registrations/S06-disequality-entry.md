# Delayed atomic-name disequality and existential hidden names

T076, third package since the conditional breadth review. The names entry supplies the explicit atomic-name contract and literal source residual controls. This experiment implements equality classes plus retained disequality edges for represented atomic names. It does not implement arbitrary-term disequality or nominal abstraction.

## Denotation and comparison

Variables range over atomic names. Equalities identify values; disequalities require different eventual values. A distinct variable identity is not proof of inequality. Fixed atoms retain their spelling. Given assignments to visible variables, ask whether assignments to hidden variables exist. Compare a compiled equality-class/exclusion graph with an independent oracle that enumerates complete assignments and checks the original equations directly.

Finite domain: an explicitly supplied alphabet bounds all variables and named operands. Use backtracking over unresolved classes with early exclusion checks. Unbounded domain: unresolved classes can receive distinct fresh atoms outside the finite visible/fixed set. Check visible equality and exclusions without enumerating hidden values. This argument does not apply to finite alphabets. The candidate returns feasibility for a supplied visible assignment; it does not claim to enumerate all answers or serialize a projected formula.

## Frozen matrices

1. All64 undirected graphs on four declared variables, alphabets of sizes0,1,2,3, all16 visible subsets, and every assignment from that alphabet to the visible subset. Expected22,656 projection-membership comparisons. Enumerate complete assignments independently once per graph/domain and project them for the oracle. Include isolated declared variables: an empty domain cannot assign even an unconstrained hidden variable.
2. The same64 graphs under an unbounded domain, all16 visible subsets with visible values a or b. Expected5,184 comparisons. The independent oracle enumerates four variables over a,b plus four fresh atoms; four fresh values suffice for the four variables. Retain the mathematical extension argument separately from this finite check.
3. One equality and one disequality, each over the25 ordered endpoint pairs from X0,X1,X2,a,b:625 original formulas. For all8 assignments of three variables to a,b, compare compilation/refinement with direct equation evaluation:5,000 checks. Run the unchanged reference on the same ground-completed source queries and verify failure or exact neq residual. Use explicit body equations after posting neq to exercise delayed aliases and bindings. Also test unknown neq(X,Y), neq(X,X), and later caller bindings directly.
4. Explicit adverse witnesses: a hidden variable unequal to both visible values over two atoms imposes equality of the visible values; under an unbounded supply it imposes no such restriction. A triangle is impossible over two names and possible over three/unbounded names. Include fixed-atom contradictions, duplicate exclusions, reflexive equality, late alias collapse, and renamed caller variables with fixed names unchanged.

Record logical assignment attempts on the triangle and complete graph witnesses versus the oracle's complete-assignment count. These are feasibility endpoints; no timing, memory, or complete-enumeration superiority claim. A successful early extension need not visit all assignments, so do not interpret it as producing all answers.

Tests precede implementation. Preserve failing import/semantic receipts, then freeze source, registration and binary identities before two default and two metrics-off confirming executions. Bound each executable at60-second wall/CPU and1GiB address space; each reference query must exhaust within2,000 steps. Existing names and structural tests remain controls. Investigate consequential mismatches without weakening the denotation.

At the gate compare further disequality/source integration with normal/neutral entry (next scheduled), integrated execution and conditional ownership. A changed logical interface does not permit altering raw CHR residuals. Full lifetime/cost comparisons, general finite-tree disequality, source closure and finite-signature name encoding remain separate required work. One further package triggers the full breadth review.

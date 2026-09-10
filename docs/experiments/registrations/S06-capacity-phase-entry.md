# Capacity results at an ordinary caller boundary

T073 tests whether the closed consuming-capacity solver can support an initial rule-priority phase inside a larger ordinary source. This is a semantic investigation, not comparative timing.

First challenge direct resumption from the solver's exported complete answers. Closed answer comparison treats residuals as multisets; ordinary rule competition may observe occurrence order. Use reversed variable identities versus producer priority, and reversed unused token order, with an ordinary caller that consumes the first eligible occurrence. Compare the complete original source against the naive resumed caller through independent scalar and compiled execution. Record a mismatch as a limitation of that boundary, not a defect in closed-answer semantics.

If direct resumption fails, derive the required ordered residual and interface bindings from the admitted source and input rather than weakening the caller contract. A valid phase must retain inert and spare occurrences in their original order, produce done occurrences in source execution order, preserve all weighted derivations, and transport every private binding needed by named outputs or the caller. Do not infer a general continuation from normalized answers.

Qualification matrix: three domain profiles ([1,2], [2,1], [3,3]); token supply [1,1], [2,2], [0,0]; aliases independent/shared; variable identity order ascending/descending; weights1/2; producer query order original/reversed; ordinary caller consumes a done occurrence, consumes a spare token, observes shared variables, writes a shared variable, or creates later private work. Each full outcome must agree with independent scalar and compiled controls. Include original output labels, an unrelated shared unknown and private variables absent from query outputs. Compare a competing token consumer before versus after the proposed prefix; before-prefix interference must not be moved across the boundary.

Limits:120 seconds per test process;200,000 ordinary source steps per finite query; existing capacity state/output and input-size bounds. Cancellation or ongoing work is not complete failure. A source gate for finite full answers does not qualify incremental publication or interruption during capacity solving. Register further lifecycle costs only after this semantic boundary is credible.

Selection: this can widen direct resource elimination beyond its closed source fragment. Adaptive reunion is the strongest ready alternative and needs policy/progress qualification. Review it, local resource claims, richer theories and sustained lifetime at this gate or an obstruction. This is the first package after the matched-reuse breadth review.

## Extension after the token-first counterexample

The initial 720-case matrix used need-first consumption. Source review identified admitted token-first consumption, and an independent caller counterexample failed the initial ordered continuation. Before the expanded run, add both consumer head orders and both original/reversed token orders to every matrix configuration:2,880 cases per build. Preserve the failing receipt and repair the order derivation rather than excluding token-first sources. This is a correctness extension within this source-boundary investigation, not a timing selection.

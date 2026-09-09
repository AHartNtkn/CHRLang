# Probe observable boundaries of recursive contraction

Before selecting a source-derived recursive implementation, test which source events may be contracted. These are semantic probes within the next T073 gate, not performance trials or a general compiler.

Compare independent scalar source execution and ordinary compiled Global execution with both Scan and Indexed access. Retain raw complete answers and residual occurrences. Bound finite probes at 10,000 service ticks.

1. Ground recursive calls with different depths: a base case posts a claim, and claims compete for one token. Compare ordinary recursion with eagerly replacing each known recursive input by its base constructor in query order. Test whether completion order changes the winning claim. This explicitly tests one proposed contraction, not every recursive compiler.
2. Replace the base effect with a pure equation feeding later consuming rules. Across bounded pairs of depths and initial occurrence orders, compare the same eager contraction against the original. A successful finite probe is evidence for its conditions, not a general soundness theorem.
3. Supply a recursive input containing an unknown tail. Without a later binding, preserve the suspended residual and unknown output. With a lower-priority binding, require resumed recursion and a complete consuming result. Ground-input-only preprocessing cannot stand in for this behavior.

Use distinct source predicate names and variable identities where useful to separate the semantic condition from a workload spelling. Preserve the independent evaluator. Record exact answers that discriminate a wrong contraction. Select the runtime/compiler boundary from these results; do not register performance comparisons until a source-derived candidate passes the complete gate.

The unknown-input probe also compares ordinary late binding with making that same input constructor available at query entry. Competing consumers distinguish schedules even when final output bindings agree. This extension is semantic investigation only; no performance selection uses it.

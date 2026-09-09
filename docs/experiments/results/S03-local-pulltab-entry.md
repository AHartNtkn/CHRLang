# Next package: actual local pull-tab source execution

**Establish a source executor in which local graph rewrites move a named choice through an application context.** This must differ operationally from the suspended context-cache evaluator already measured. The [breadth review](S06-first-breadth-review.md) selects this gate ahead of another lowering refinement.

Start by inspecting the direct graph and demand representations and their existing choice-identity counterexamples. Specify the graph operation that replaces an application using a choice with that same choice applied to two corresponding application contexts. Track which nodes and source occurrences are copied, shared or context-owned. Preserve correlation when the same named choice is used twice, and independence for distinct dynamic choice births.

The first source witnesses need actual constructor demand, opaque use before demand, two uses of one choice, independent calls, consumed resources, off-output failure and a finite sibling beside continuing work. A pure expression graph alone does not discharge the source-execution obligation. An application result cannot be reused as a fresh derivation when its labels or resource claims must be independent.

Compare complete answers independently and retain the explicit source policy. A compiler/source checker may bound the first fragment, but its rejections are checker boundaries until justified semantically. Count structural rewrites only after correctness; register full lifecycle costs separately. Existing exact-schema lowerings remain controls where they apply, not a reason to exclude source cases where this mechanism matters.

T071 resumes ownership of this package. Its fresh cross-application templates, broader aliases and sustainable lifetime remain open beyond the first gate. T072 and T073 keep their remaining obligations. No architecture or goal closure follows from completing this package.

## First registered correctness gate

The first implementation lifts a named choice at a directly demanded call argument through that call. Aliases and already computed producer results may lead to the argument's choice. The result is the same named choice over two calls with the corresponding argument substituted. Other argument identities and the logical output identity remain shared; each copied call receives an obligation supported by its choice arm. No choice birth is introduced by the rewrite.

A structural test must observe the rewritten graph, copied arguments and conditional obligations. Complete-source tests compare answers with the independent scalar evaluator and existing compiled/direct-graph controls. Include correlation, independent births, consumption, residual multiplicity, off-output failure and finite service. Nested constructor demand and choice splitting required by resource settlement remain outside this first rewrite; they retain the existing service operation and cannot support a claim about general pull-tabbing.

This is a correctness experiment, with no comparative timing claim. Existing source tests retain their 100,000-tick collection bound; finite-service witnesses retain their own bounds. A failure requires distinguishing an implementation defect from a source-policy difference. After the gate, review the strongest ready alternative before extending the mechanism or registering cost measurements.

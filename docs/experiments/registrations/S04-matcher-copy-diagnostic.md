# S04: test the cost of copying environments for incompatible partners

Test whether copies made before known-key rejection materially drive the mutation workload's cost. The operation is visible in the source; its allocation and timing contribution are not yet measured. This diagnostic follows the frozen lifecycle pilot and does not change that running experiment.

## A prediction derived from the source

Use the exact registered mutation query at index 0: 128 cells, the first 32 edited, three binary choice levels, and two work steps between choices. Each edit moves its cell occurrence to the end of the occurrence order by consuming it and posting a fresh cell.

At the first choice level, the next selected cell is always the earliest cell occurrence. At the second and third levels, the 96 untouched cells precede every edited cell. The source matcher clones its entry environment before rejecting each of those wrong-key partners.

Therefore copying or trailing should make **(4 + 8) × 32 × 96 = 36,864 environment copies for rejected partners** during one complete query. First-head attempts have an empty entry environment and do not contribute to this count. Count rejection of an entry-bound variable distinctly from any other mismatch.

The entry environment for the cell head contains four terms: the cell key, remaining edit list, branch atom and continuation. With r edits remaining after the current one and choice level l, these terms contain 2r + l + 9 source term nodes. The mean r over each 32-edit sequence is 15.5. Thus the wrong-partner copies should clone **4 × 32 × 96 × 42 + 8 × 32 × 96 × 43 = 1,572,864 term nodes**. This is a logical node count, not a heap-byte estimate.

Validate those predictions with a separate diagnostic build. Instrument calls immediately around the entry-environment clone and distinguish accepted/rejected candidates. Do not instrument primary timing builds. Compare complete answers with the independent source evaluator and verify that source-step counts are unchanged. A disagreement requires correcting the prediction or identifying an execution difference before interpreting costs.

## Candidate correction and correctness gate

A candidate correction may check values fixed by previous heads before cloning the environment. The full nonbinding matcher remains the authority for acceptance. The precheck must not invent bindings for unknown rule variables, change tuple order, or alter which source application runs.

Prove the precheck's one-sided obligation: every candidate accepted by full matching under the entry environment passes the precheck. Then test repeated variables, unknown inputs, constructor paths, aliases and partial failures, including counterexamples to treating an absent entry binding as a mismatch. Run the existing restoration/source and lifecycle-source gates against the independent oracle and existing controls.

Count avoided copies separately from candidate visits. This correction may avoid copying without reducing the number of visited partners; do not describe it as indexing.

## Contrary controls and interpretation

Include a source in which all candidate partners agree with the entry bindings, both for small values and large equal structures. In that regime an extra precheck may repeat traversal while saving no copies. A selective-rejection win alone cannot justify an unconditional policy.

If the diagnostic establishes consequential avoidable allocation, register a paired lifecycle comparison with the preserved original binaries and corrected source. Fix exact configurations, repetitions, resource bounds and practical thresholds before those new timings. Keep source work, ruleset reuse, cancellation and full observations matched; retain preparation and disposal costs.

Do not rank restoration architectures from this diagnostic. It determines whether their common matcher is a material confound and whether a small correction should precede a final bounded interpretation. S05 stable-identity operation/failure reuse remains the strongest ready alternative; if the copy cost is small or the contrary checking cost cancels its benefit, proceed to that alternative rather than extending local matcher tuning indefinitely.

## Adverse alias traversal for the paired cost registration

The implemented root precheck follows query-variable binding chains. Include all-compatible keys reached through a long alias chain in the forthcoming cost comparison: extra root traversal can cost work even when constructor children are not inspected. This requirement is recorded before comparative correction timings. Correctness gates alone do not establish that the precheck pays in this regime.

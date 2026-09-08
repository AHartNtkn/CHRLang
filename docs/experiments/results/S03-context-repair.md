# Context repair removes a material prototype cost, but preserves the main tradeoff

The context-allocation repair reduces the graph's early-discrimination lifecycle cost by about 30% and requested bytes by about 39%. It preserves all tested semantics and is retained. **The graph still loses to Global scan on this case, while retaining its opaque-work advantage; more graph tuning is no longer the selected next investigation.**

The [prospective paired experiment](../registrations/S03-context-repair.md) completed all 252 processes: five primary and two allocation blocks over 36 cells. Each block includes the original frozen graph, the repaired graph and fresh controls. The [runner is unchanged](../../../research/chr-direct-conditional/examples/s03_lifecycle.rs). [Freeze](s03-context-repair/freeze.json), [order](s03-context-repair/order.json), [raw results](s03-context-repair/raw.jsonl), [summary](s03-context-repair/summary.tsv), [paired ratios](s03-context-repair/ratios.json).

## What changed and what it cost

[Context intersection and subtraction](../../../research/chr-direct-choice/src/lib.rs) now test compatibility without building a temporary context. Successful intersection clones and extends the map once; subtraction no longer constructs an intersection solely to ask whether the regions overlap. The context representation, partition semantics, rule scheduling and answer publication are unchanged. No cache or additional retained index was introduced.

| Family | Original graph lifecycle, ms | Repaired graph lifecycle, ms | Paired repaired/original ratio | Requested MiB, original → repaired |
|---|---:|---:|---:|---:|
| Binary words | 2.172 | 2.071 | 0.953 | 8.071 → 7.920 |
| Nested words | 2.735 | 2.619 | 0.966 | 10.112 → 9.856 |
| Duplicate words | 4.330 | 4.151 | 0.979 | 15.999 → 15.691 |
| No-choice work | 0.318 | 0.330 | 1.039 | 1.081 → 1.081 |
| Opaque common work | 1.763 | 1.698 | 0.963 | 7.007 → 6.873 |
| Early discrimination | 58.219 | 40.441 | 0.695 | 199.956 → 122.836 |

Lifecycle columns are medians for one preparation and sixteen changing queries with disposal. Ratios are medians of paired block ratios, which need not equal ratios of the displayed medians. Discrimination is the only repair effect meeting the preregistered practical criterion: all five ratios agree in direction and the median exceeds a 20% improvement. The small movement elsewhere remains inconclusive, including no-choice work; it is not claimed as a regression or a gain.

Discrimination allocation calls fall from 2,437,141 to 1,715,125. Peak requested live heap is unchanged in every family. The repair removes transient work rather than changing retained ownership. Allocation fingerprints replay exactly, and every measured query returns requested live heap to its setup baseline after disposal. These are requested-heap observations, not RSS or ongoing-query reclamation guarantees.

## What the fresh controls now say

On early discrimination the repaired graph has paired median lifecycle ratios 4.317 versus Global scan and 1.864 versus Global index. Its ratio to Conditional is 1.013 and is inconclusive. Thus the previous clear graph loss to Conditional depended materially on avoidable prototype overhead; the larger loss to the compiled controls survives this repair.

Opaque common work continues to favor the graph: paired ratios are 0.523 versus Global scan and 0.495 versus Conditional, with all five pairs agreeing. No-choice work still favors Global scan. Word generation continues to favor the checked graphless compiler, with graph/compiler paired ratios ranging from 12.4 to 21.2. The compiler's source eligibility and the graph's bulk first-answer behavior are unchanged by this repair.

These conclusions use fresh controls and the original binary rerun in the same blocks. They do not splice the first pilot's timings into a new ratio. Native user-program compilation is still outside the measured lifecycle, and neither pilot supplies a universal workload weighting or architecture winner.

## Validation and interpretation

All twelve graph tests pass, including the exhaustive context truth table and independent finite-tree equation catalogue. All fourteen source-gate tests pass with default and disabled metrics. Both repaired release binaries pass full runner/oracle validation; the allocation self-check passes. Clippy and formatting pass. The original binary hashes and unchanged runner are checked before the paired runs. No semantic mismatch, process timeout, service cutoff or memory-bound failure occurred. [Validation commands](s03-context-repair/validation.json), [time validation](s03-context-repair/time-validate.log), [allocation validation](s03-context-repair/memory-validate.log).

The allocation change supports the profile's diagnosis: substantial discrimination cost was unnecessary intersection construction and repeated copying. It does not explain every remaining cost. The implementation still has repeated contextual discovery and bulk materialization; adopting new region representations or matching plans would be a distinct experiment, not another trivial correction.

## Next architectural investigation

The bounded T062 trial is complete: a direct graph candidate, independent semantic gates, credible compiled and graphless controls, full lifecycle pilot, consequential diagnosis and paired repair are recorded. **S03 as a whole remains open**, including other direct graph/derivation mechanisms, broader workloads, contextual reuse, observation and lifetime.

S02's distinct relational execution alternative is selected next. The current integrated control already matches constructor descriptors directly and exports terms only for observation; pretending to remove a solved-store export boundary there would test nothing new. The remaining visible boundary is its recursive source-pattern walker followed by separate candidate-tuple checking. The new investigation must assess compiling constructor patterns and source heads into one relational join plan over value identities, including equality-driven eligibility and consuming occurrence ownership. Context-local equality must be addressed explicitly rather than assuming a global class merge remains valid under choices.

The strongest ready alternative is S01's selective/consuming lifecycle extension. It can change the choice between recomputation and retained matching, but its current implementation still concerns a fixed six-rule lowering. S02 can change the representation and execution boundary itself and has an existing integrated control plus independent gates. That broader unresolved contrast now has greater expected decision value than another graph-specific refinement. This is a prioritization judgment, not a negative finding about S01 or a prediction that relational execution wins. S06 broader compilation remains independent and may share source analysis only where the semantic obligations actually coincide.

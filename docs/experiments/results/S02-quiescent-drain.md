# Bounded draining repairs repeated source advances

**The engine now finishes unrelated equality in bounded batches once no source rule can fire.** At depth 64, successful output takes 9 advances instead of 72 with a sufficiently large budget. Early failure keeps its reduction from 67 deductions to 4.

The implementation and two frozen confirmations establish this repair. They do not yet determine whether selective settlement repays the cost of discovering which equality matters.

## What changed

The [matcher-settlement experiment](S02-matcher-settlement.md) showed how to preserve fixed consumer priority while letting unrelated output equality remain pending. Its successful cases repeatedly advanced the source engine to make one background deduction, even after matching had found no application.

The new explicit `advance_ready` operation belongs to the existing relational engine. It settles matcher-visible equality at source-application boundaries. If no rule can fire, it drains unrelated equality before reconsidering matching. Source bodies still execute through the existing machinery; draining never jumps ahead of pending body effects.

Each call takes a nonzero deduction budget. Readiness settlement and subsequent draining share that budget. A state with work remaining goes to the back of the frontier. This bounds serviced equations per call and permits cancellation between calls; it does not bound the time spent on one deduction, relevance analysis or matching.

Preparation owns read metadata inferred from its complete rule set. Every query engine reuses that same metadata; the public advance operation takes only a budget, so a caller cannot supply another program's read set. Ordinary eager preparation does not infer readiness metadata. The fixtures do not declare their own eligibility.

## What the measurements say

These are engine-advance and equality-deduction counts, not elapsed timings. The previous selective controller needed 72 advances for the separate successful depth-64 source. Full settlement needed 9.

| Depth-64 source | Budget 1: advances | Budget 8: advances | Budget 256: advances | Deductions at every budget |
|---|---:|---:|---:|---:|
| Separate output; success | 72 | 16 | 9 | 68 |
| Separate output; source failure | 8 | 8 | 8 | 4 |
| Separate output; late contradiction | 72 | 16 | 9 | 68 |

**Batching repairs the successful-source advance count without giving up early failure.** At depths 4, 16 and 64, separate successful and late-clash cases take `8 + ceil(depth / budget)` advances. Early source failure always takes 8 advances and 4 deductions. The same relationships hold for possible/impossible earlier guards and one/two tokens.

**A late contradiction still exposes the cost of speculation.** Full settlement performs 67 deductions and takes 7 advances at depth 64; the repaired selective path performs 68 and takes at least 9 in these cases. Fewer repeated advances do not eliminate that genuine difference in when failure is discovered.

**Shared dependencies still require readiness work.** When the long equation is read by the matcher, total deductions match full settlement. A small budget can add yields while preserving the selected consumer. Choosing a budget trades responsiveness against repeated checking; no default is selected from these fixtures.

## Why the repair is safe in the tested cases

The controller can drain without rediscovering source applications only after settling every matcher-visible component and finding no application. Remaining unrelated, consistent equality cannot then enable one. A contradiction discards the interpretation instead of publishing it. Pending source bodies invalidate that premise, so they continue to receive their ordinary turns.

Each confirmation checks 351 executions against the independent scalar evaluator: the existing 117 non-cancellation source configurations at budgets 1, 8 and 256. This includes constructor patterns, repeated variables, positive guards, newly posted requests, priority conflicts and shared/disjoint resources. Another 216 detailed work rows observe the 72 separated/shared configurations at those budgets; these are the same sources, not extra source diversity.

Nine cancellation cases stop after actual consumption while output equality remains pending, then successfully reuse the same preparation. Three choice cases deliver the short branch before the long branch, preserve complete answers and check the deduction budget on every call. The preceding 120 readiness, 24 priority and 12 early-failure observations also reproduce.

Two mutations check that the tests enforce the intended responsibilities. Unbounded draining fails the per-call budget assertion. Draining across pending effects fails the early-failure deduction assertion. Both fail through observed behavior, and exact source bytes are restored before freezing. All 14 library tests pass in both confirmations; scoped Clippy and the existing source/store/head-plan regressions pass.

## The next decision

**Keep the repaired selective schedule in the cost comparison.** It retains a real early-failure benefit, preserves the tested fixed priority and removes an avoidable successful-source scheduling cost. It still performs relevance analysis and can discover contradictions later than full settlement.

The next T072 package must measure complete lifecycle costs for the existing full-settlement controller, selective settlement and bounded draining on these qualified sources. Charge read preparation, changing-query setup, execution, observation, cancellation and disposal. Use ordinary release timing separately from work and requested-allocation diagnostics. Include preparation reuse and both early failure and successful/clashing output; do not weight those outcomes to manufacture an overall winner.

Partner ordering remains the strongest independent matching alternative, and graph memo/lifecycle work remains ready. A bounded lifecycle pilot is still the more immediate discriminator because the source mechanism now passes correctness checks and its known repeated-advance defect is repaired. Its result must trigger the next full portfolio review: this is package three since the [last review](S08-traversal-portfolio-review.md). The goal and T072 remain active.

## Evidence

- [Registration](../registrations/S02-quiescent-drain.md)
- [Source/binary freeze](s02-quiescent-drain/freeze.json), [source archive](s02-quiescent-drain/sources.zip)
- [First confirmation](s02-quiescent-drain/confirmation-0.json), [second confirmation](s02-quiescent-drain/confirmation-1.json), [rows](s02-quiescent-drain/audit.json)
- [Mutation receipts](s02-quiescent-drain/mutation-audit.json), [Clippy](s02-quiescent-drain/clippy.log), [regressions](s02-quiescent-drain/regression.log)
- [Runner](../../../research/chr-relational/experiments/quiescent_drain.py), [receipt auditor](../../../research/chr-relational/experiments/audit_quiescent_drain.py), [mutation runner](../../../research/chr-relational/experiments/check_quiescent_drain.py)

Run `python3 research/chr-relational/experiments/audit_quiescent_drain.py` to verify archived inputs, exact repetitions, work formulas and mutation receipts. Both confirmations run serially with a 60-second timeout/CPU limit and 1 GiB address-space limit. No architectural timing inference uses test duration.

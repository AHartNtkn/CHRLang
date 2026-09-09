# Mixed sources separate discovery costs from answer ownership

Contextual execution uses less peak requested memory than scanning on the sampled substantive mixed sources, but allocates more over the lifecycle. Restarting demand increases history costs. A separate defect in conditional answer allocation is now repaired; the remaining execution overhead still requires a favorable sharing witness before architectural judgment.

## Sources and independent correctness

The [source registration](../registrations/S10-broad-mixed-gate.md) and [source fixture](../../../research/chr-direct-conditional/examples/support/broad_mixed_source.rs) combine keyed consuming joins, a kept permit, propagation history, equality guards, constructor matching, explicit choices and fresh aliases shared between outputs and residuals. Seven variations add independent versus shared unknowns, delayed guard activation, early versus late failure, 16 history items per input, or depth-16 returned structures.

For counts 0/1/3 and two changed queries, independently constructed expectations enumerate assignments, reject failed or conflicting shared choices, and retain every unconsumed resource and alias. The scalar evaluator and seven candidate configurations agree on all 42 configurations in both builds. Prepared source is reused; query keys, variables and occurrence order change. [Gate](../../../research/chr-direct-conditional/tests/broad_mixed.rs), [default log](s10-broad-mixed-gate/default.log), [feature-off log](s10-broad-mixed-gate/off.log).

These are richer composition probes, not held-out sources or an application distribution. Shared aliases test correlation; they do not by themselves establish substantial common work being shared after a choice.

## Complete measured lifecycle

The [registered pilot](../registrations/S10-broad-mixed-lifecycle.md) completes 4116 processes: 1176 allocation runs with 588 exact pairs, then 2940 ordinary-allocator timing runs. Seven modes, seven families, three counts, two reuse levels and two consumers are covered. Complete answers, phase order, query ownership, retained-answer ownership, cancellation and prepared disposal all pass. [Audit](s10-broad-mixed-lifecycle/audit.log), [all cells](s10-broad-mixed-lifecycle/summary.csv), [source/binary freeze](s10-broad-mixed-lifecycle/freeze.json), [toolchain](s10-broad-mixed-lifecycle/toolchain.txt).

The table gives three active inputs, four changing queries per preparation, query-level answer release and one cancellation probe. Figures are requested allocation bytes. Peak is requested live memory above the measurement baseline; neither is RSS.

| Family | Scan traffic | Eager contextual traffic | Demand contextual traffic | Scan peak | Eager contextual peak |
|---|---:|---:|---:|---:|---:|
| Independent choices | 776529 | 1434387 | 1710319 | 81893 | 48095 |
| Shared unknowns | 428937 | 683683 | 784431 | 40364 | 23943 |
| Delayed guards | 995745 | 2063403 | 2733255 | 89765 | 48295 |
| Early failure | 273237 | 387373 | 449547 | 24608 | 19381 |
| Late failure | 358319 | 638784 | 731852 | 35459 | 23182 |
| History | 2804281 | 6480447 | 12257755 | 292871 | 150704 |
| Deep answers | 1475089 | 1844355 | 2120287 | 176105 | 84434 |

The mixed history/guard obligations reverse the earlier fused source's demand advantage. This is evidence against a universal restarting-demand policy, not against avoiding eager tuple materialization. Indexed, inferred-specialized and persistent shared-equality controls are retained in the complete matrix. Their tradeoffs must be assessed per source; no pooled weighting is assigned.

Timing is exploratory. For example, independent-choice median phase sums are 996640 ns for Scan, 880156 for eager contextual and 978623 for demand; deeper returned structures have medians 1494762, 991169 and 1428011 respectively. Five samples and their ranges in the cell table do not support formal practical-win classifications. All conditional timings in this first matrix describe the implementation before the export repair below.

## Consumer retention uncovered an avoidable export cost

Holding deep answers across four queries initially retained 166400 bytes in six controls but 465920 in conditional execution. An independent [capacity attribution](../registrations/S10-answer-capacity-attribution.md) recursively counts the returned vectors and strings. It reproduces both measured totals exactly. Per query, conditional answers contain 61056 spare constructor-field bytes and 13824 spare bytes in other vectors; semantic observations remain equivalent. [Diagnostic source](../../../research/chr-direct-conditional/examples/answer_capacity.rs), [before](s10-broad-mixed-lifecycle/answer-capacity.log).

Incremental conditional export appended fields to empty vectors even when arity was known. The [registered repair](../registrations/S10-known-arity-export-repair.md) reserves the exact output count, constructor arity and constraint arity when each owner is created. Traversal, conditional selection and publication remain incremental. The final residual count depends on active support, so its ordinary growth policy remains; there is no extra bulk compaction pass.

A fresh 168-process allocation attribution covers all 84 conditional cells, with 84 exact pairs. Traffic changes only in answer-delivery phases; all owner baselines and complete observations pass. Deep retained answers now occupy 171008 bytes across four queries. The remaining 4608 excess bytes are spare residual-vector capacity. [After capacity](s10-export-repair/capacity.log), [allocation audit](s10-export-repair/allocation-audit.log), [all repaired cells](s10-export-allocation/summary.csv).

| Conditional family, three inputs/four queries | Original traffic | Repaired traffic | Repaired query-release peak |
|---|---:|---:|---:|
| Independent choices | 3513563 | 3430619 | 61788 |
| Shared unknowns | 3182075 | 3161339 | 46316 |
| Delayed guards | 5225283 | 5142339 | 64712 |
| Early failure | 1873461 | 1863093 | 34384 |
| Late failure | 3261492 | 3251124 | 43864 |
| History | 17126779 | 16601467 | 180606 |
| Deep answers | 4843915 | 4539787 | 100191 |

The correction materially changes retained memory, but does not explain most conditional allocation traffic. Do not use the original answer-capacity difference as evidence that conditional execution inherently needs that memory. No post-repair comparative timing was run, so there is no measured speed claim for the repair.

The runtime, streaming, library and broader-source suites pass 23 tests in each feature configuration, including finite siblings beside ongoing work and consumer-owned answers. Strict Clippy passes. [Default](s10-export-repair/default.log), [feature-off](s10-export-repair/off.log), [Clippy](s10-export-repair/clippy.log). The baseline's engine source is recoverable at commit `0bd36f86f`; the [auditor](../../../research/chr-direct-conditional/experiments/audit_broad_mixed.py) checks its original hash when the current repaired source differs.

## Remaining measurement boundaries

Preparation, changing-query inputs/setup, first and remaining delivery, cancellation, engine disposal, prepared disposal and retained-answer disposal are charged. Execution and observation are coupled in these APIs and reported together. Answers retained past prepared disposal are revalidated. Native compilation, startup, source AST construction, expected/scalar checks and preallocated measurement bookkeeping remain outside the measured intervals. Finite four-query retention is not sustained-stream evidence. No source lowering is applied in this pilot.

The [runner](../../../research/chr-direct-conditional/examples/broad_mixed_cost.rs), [matrix driver](../../../research/chr-direct-conditional/experiments/broad_mixed_lifecycle.py) and [repair checker](../../../research/chr-direct-conditional/experiments/export_repair_check.py) retain exact execution and ownership evidence. There were no matrix cutoffs.

## Next: give conditional sharing a substantive opportunity

T078 next qualifies common work performed after a choice, with consuming effects, branch-local guards/failure and full residual observations. Vary the amount of common work independently of choice count, plus independent-work and early-failure controls. Demonstrate that the conditional representation actually performs less repeated source work; merely labeling a family shared is insufficient. Reuse existing credible source cases where they provide that mechanism.

The strongest ready alternative is selective conditional discovery or resumable contextual discovery. Both remain consequential. The next favorable witness takes priority because these mixed sources expose overhead but do not yet determine whether substantial shared post-choice execution can repay it. That missing opportunity must be resolved before using local discovery costs to judge the organization. At that gate, reconsider implementation defects and total lifecycle measurement. Compilation, sustained lifetime, language tradeoffs, all other unresolved directions and held-out challenges remain required; the research remains active.

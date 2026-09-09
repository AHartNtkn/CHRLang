# Conditional execution shares real work, but allocation can outweigh it

Conditional execution performs a common consuming traversal once across eight source alternatives, while explicit paths repeat it eight times. That does not yet repay its lifecycle allocation on the larger measured sources. The next comparison must account for avoidable discovery and source-derived elimination before judging the architecture.

## The sharing mechanism actually operates

The [registered gate](../registrations/S10-post-choice-work-gate.md) places substantial work after source choices. A common continuation consumes fuel along a depth-d traversal, retains a permit with propagation history, and publishes a fresh unknown shared between its result and residual. Choice count and traversal depth vary independently.

The independent-work control places distinct traversals inside the choice tree and varies their depth with the chosen path. The early-failure control rejects one first-choice arm before traversal. These are different source tasks with separate independently constructed observations; their elapsed times would not be interchangeable work comparisons.

At three choices and depth 16, scalar complete traces contain 128 consuming step applications for common work, 140 for independent work, and 64 after early failure. Direct conditional application traces contain 16, 140 and 16 respectively. All complete outputs, residual fuel, permits, history marks and fresh aliases agree. This proves that the common case exercises useful sharing, while the independent control prevents a source label from standing in for the mechanism.

The [source fixture](../../../research/chr-direct-conditional/examples/support/post_choice_source.rs) and [gate](../../../research/chr-direct-conditional/tests/post_choice_work.rs) check 54 configurations: three families, choices 0/1/3, depth 0/4/16 and two changed queries. The scalar and seven candidate configurations agree in default and feature-off builds. Tracing is explicitly enabled for the conditional work diagnostic in both builds; these are not counter-free timing samples. [Default](s10-post-choice-gate/default.log), [feature-off](s10-post-choice-gate/off.log), [strict Clippy](s10-post-choice-gate/clippy.log).

## Complete requested-allocation comparison

The [allocation registration](../registrations/S10-post-choice-allocation.md) extends depth to 64 and compares choices 0/3 with reuse 1/4. Seven execution configurations across 336 cells complete 672 isolated processes, with 336 exact phase replays. Every measured and warm complete answer agrees with independent expectations, every finite run exhausts within 500000 calls, and query, cancellation and prepared owners restore. [Audit](s10-post-choice-allocation/audit.log), [all cells](s10-post-choice-allocation/summary.csv), [freeze](s10-post-choice-allocation/freeze.json).

The table shows three choices and four changing queries per preparation. Bytes include preparation, inputs, execution plus owned-answer delivery, one cancellation probe and all disposal. They measure requested allocation traffic, not RSS.

| Family/depth | Scan | Specialized Scan | Contextual eager | Contextual demand | Conditional |
|---|---:|---:|---:|---:|---:|
| Common / 4 | 562978 | 562714 | 926322 | 916282 | 696719 |
| Common / 16 | 1240946 | 1228906 | 4456874 | 2474098 | 2546543 |
| Common / 64 | 4516834 | 4516570 | 49518090 | 8811730 | 25020535 |
| Independent / 64 | 4308312 | 4303632 | 54304267 | 8866643 | 210860544 |
| Early failure / 64 | 2448288 | 2449144 | 24841749 | 4489565 | 30233630 |

Indexed and persistent shared-contextual controls remain in the full table. No mode is assigned an invented workload weight. Conditional execution avoids repeated source steps in common and early-failure cases, yet its larger allocation obligations can outweigh that saving. The independent source additionally carries choice-tree preparation and different residual fuel; compare organizations within a row, not absolute totals across different source tasks.

At common depth 16, conditional peak requested memory is 54964 bytes above baseline versus Scan's 93976. At depth 64, those peaks become 436320 and 224008. Demand contextual uses 59980 bytes at depth 64 while allocating more traffic than Scan. Even within this family, traffic and retention are distinct tradeoffs.

No ordinary-allocator timing matrix ran. Instrumented clock readings in raw files are diagnostic and do not establish performance rankings. Native compilation, process startup, source AST creation, scalar/expected-answer validation and measurement bookkeeping remain excluded. Preparation here means runtime rule preparation. Answers are released after each complete query; sustained lifetime is still unresolved.

The [runner](../../../research/chr-direct-conditional/examples/post_choice_cost.rs), [driver](../../../research/chr-direct-conditional/experiments/post_choice_allocation.py) and [auditor](../../../research/chr-direct-conditional/experiments/audit_post_choice_allocation.py) preserve the evidence. The frozen test-source copy records its pre-lint annotation form; the current annotation only marks unused functions in the imported scalar helper. The measured execution code is unchanged.

## Why source-derived elimination comes next

The private traversal consumes a predictable amount of fuel and exposes no intermediate result on these sources. That suggests a stronger control: derive a resource-count transformation from the source, with checked ground depth, sufficient fuel, required permit and absence of competing readers/writers. This is currently an analytical opportunity, not an implemented or validated optimization.

T078 next qualifies that source-derived control. Preserve source choice multiplicity, fresh variables, failed alternatives and complete residual resources. Include insufficient fuel, unknown depth, absent permits, competing rules and queries outside the certificate. Ordinary execution must remain available outside eligibility, with certification and preparation charged in a subsequent comparison. Do not implement an answer generator that merely recognizes benchmark names or calls the independent oracle.

The strongest ready alternative is conditional selective discovery. That remains consequential: a consuming step can discover many resource tuples even when only one application is needed, so the current allocation loss is not an intrinsic lower bound. Source-derived elimination takes priority because it could remove the repeated work being shared and thereby change the value of both conditional and conventional discovery improvements. Reconsider those repairs after the source gate. This selection does not resolve them or the other architectural, language, compilation, lifetime and held-out obligations. The goal remains active.

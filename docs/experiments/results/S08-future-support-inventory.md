# Completed histories retain conditions that no future task can use

The graph retains cached-result conditions and consumption claims that conflict with every pending task, even while useful work remains. They are a concrete target for metadata reclamation. This does not establish that their referenced nodes or identity-bearing slots can be freed.

**Next validate reclaiming incompatible result entries and consumption claims.** Their readers check the stored condition against the active context; permanently conflicting entries cannot satisfy that check. Preserve compatible entries and their order, graph nodes, choice labels, obligation indices and scheduling. Measure reclamation cost only after independent continuation and resource tests pass.

## What “cannot be used again” means here

**Task contexts only acquire additional choice assignments.** Current `Run::tick` either requeues the same context, completes/fails it, or clones it into children at an unassigned choice. No interface reinstates a completed task or drops its assignments. All producers of a split request require an unassigned label. A condition conflicting with a pending task therefore conflicts with every descendant of that task.

**The inventory is conservative when an assignment is missing.** A condition counts as potentially usable if it agrees with at least one pending task on their shared labels. The missing labels might be supplied later. Such compatibility does not prove the condition will actually be reached; other semantic constraints may prevent that. With no pending tasks, no condition can be selected by further service of this run.

**The diagnostic reads state without changing execution.** `Run::future_supports` counts total and incompatible result supports, obligations, choice births and consumption claims. It does not run during ordinary service or reclaim memory. A witness verifies that four completed answers leave both pending work and incompatible conditions, that repeated inventories agree without changing graph counts, and that all stored conditions become unusable at exhaustion. The compatibility predicate agrees with exhaustive extension checks for all 729 pairs of partial assignments over three labels.

## Measured inventory

**Two diagnostic runs produce identical trajectories for all 96 queries.** The [registration](../registrations/S08-future-support-inventory.md) covers repeated, distinct and alias streams; depth 16/64 and changed-query depth n+1; resource and terminal-failure variants; and dependency/template graphs. Complete raw answers agree with both independent scalar execution and the analytical formula. The [receipts and audit](s08-future-support-inventory/audit.json) retain 528 snapshots, source/compiler/binary freezes and exact replay.

**Unusable result conditions occur before exhaustion in 284 of 412 snapshots with pending tasks.** This count includes source and template variants; it is a diagnostic frequency, not an application weighting. The remaining snapshots are not evidence that all retained graph state is necessary.

**The repeated resource stream shows the distinction clearly.** At depth 64, work four and payload eight, the dependency graph has the following counts immediately after delivery. “Incompatible” means that every pending task conflicts with the entry's condition.

| Answers delivered | Pending tasks | Results: total / incompatible | Obligations: total / incompatible | Claims: total / incompatible |
|---|---:|---:|---:|---:|
| 1 | 4 | 16 / 6 | 20 / 5 | 1 / 1 |
| 4 | 4 | 37 / 24 | 38 / 20 | 4 / 4 |
| 16 | 4 | 121 / 96 | 110 / 80 | 16 / 16 |
| 64 | 1 | 454 / 384 | 391 / 320 | 64 / 64 |

**Templates reduce some retained metadata but retain incompatible entries too.** At answer 64, the template graph has one pending task, 76 results with 69 incompatible, 12 obligations with five incompatible, and 65 claims with 64 incompatible. Its final branch has already consumed the token but has not yet delivered its answer. The compatible claim must remain: discarding all historical claims would risk making an already-consumed resource appear live.

**Choice births illustrate why compatibility is not a garbage collector.** Both graphs retain 64 births at answer 64, none classified incompatible. The future task's context can agree with every ancestral birth while already assigning those labels. Birth maps also participate in match-dependency validity and label lookup. Their representation requires a separate argument; the current inventory neither frees nor renumbers them.

## Which reclamation is justified to test

**Result entries and consumption claims have a local removal argument.** Production result readers in forcing, match-dependency discovery, argument lifting and residual observation select entries only when their supports are subsets of the current context. Resource liveness likewise reads a consumption claim only when its condition holds. The monotonicity argument makes an incompatible entry unselectable by every future task. Filtering those entries while preserving the remaining order cannot remove a selectable result or claim. This is an analytical argument to validate experimentally, not an implemented collector or a measured saving.

**Obligation slots and graph nodes need a different analysis.** Call nodes contain obligation-origin indices, and queued tasks retain obligation cursors and round boundaries. Compacting that vector could change service or refer to the wrong origin. A result's target node may also be shared by another result, argument or output. Its condition being unusable does not prove the target unreachable. The proposed first reclamation must leave these identities intact.

**No byte or speed saving follows from these counts alone.** The [owner probe](S08-stream-ownership-gate.md) measured retained heap; this diagnostic measures conditions. Map sizes, vector capacities, shared targets and the cost of scanning pending tasks must be charged in a future allocation/timing comparison. Retaining a useful cache can still beat repeatedly recognizing and pruning it.

## Next package and breadth

**A bounded metadata-reclamation gate now has higher decision value than timing the unchanged owners.** It targets a proved unusable category inside a measured retaining owner. Compare retaining everything with selective removal at delivery boundaries; preserve aliases, raw multiplicity, failure, overlapping resources, unrelated pending tasks and fresh derivations. Include cases with compatible historical claims, such as the template snapshot above. Validate continuation behavior and every source observation before measuring allocation or time.

**This is the second package in the current lifetime cycle.** The source/owner gate came first; this support inventory comes second. A reclamation gate and its first bounded cost comparison would reach the governing four-package breadth review. At that boundary compare broader call-level reuse and distinct integrated S02 execution before any further local refinement. Their required investigations remain unfinished.

All 22 graph-package tests, the three stream tests with default and counter-free features, and scoped strict Clippy pass. T074 remains active. The full architecture choice, sustained limits, broader reclamation and the research goal remain unresolved.

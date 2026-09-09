# Direct normalization substantially reduces growing-output cost

**Reading graph children directly and sizing the owned answer vector reduces the growing-output example's requested traffic by about half.** Complete answers remain unchanged. The effect is large enough to require a fresh comparison against compiled execution before interpreting the graph's earlier loss.

The [registered attribution](../registrations/S08-normal-copy-attribution.md) changes only normalization. It clones the name required by the owned answer, reads child IDs individually and reserves the final vector's known arity. The measured effect includes both avoiding a temporary child-ID vector and sizing the output vector directly; it does not isolate those two contributions.

Constructor stability makes this traversal sound: the only node replacement is `bind`, used only by `producer` after checking that its old node is Unknown. Recursive forcing can append cells but cannot replace the parent constructor. Integer IDs survive arena growth; each child borrow ends before recursion. Traversal order and short-circuit signals remain unchanged.

## Evidence

All 2,896 registered processes succeed, with 320 exact allocation replays and complete query/prepared ownership restoration. Requested allocation falls in all 160 mode/source comparisons, exclusively during execution/observation. Preparation, setup and disposal requested allocations are unchanged. Demand has seven practical timing gains, zero losses and 73 unresolved comparisons; templates have nine gains, zero losses and 71 unresolved comparisons. Unresolved is not equivalence.

Representative substantive depth, eight changed queries, resources present and forward starting order:

| Family | Mode | Primary ms, before → after | Requested bytes, before → after | Paired classification |
|---|---|---:|---:|---|
| single | dependencies | 0.533 → 0.486 | 601,387 → 600,811 | unresolved |
| single | templates | 0.209 → 0.217 | 276,395 → 275,819 | unresolved |
| repeat | dependencies | 1.657 → 1.874 | 2,344,531 → 2,342,227 | unresolved |
| repeat | templates | 0.437 → 0.410 | 763,051 → 760,747 | unresolved |
| distinct | dependencies | 1.713 → 1.730 | 2,344,531 → 2,342,227 | unresolved |
| distinct | templates | 0.641 → 0.599 | 1,040,443 → 1,038,139 | unresolved |
| choice | dependencies | 14.210 → 14.027 | 12,048,543 → 12,011,679 | unresolved |
| choice | templates | 1.670 → 1.645 | 2,533,035 → 2,496,171 | unresolved |
| grow | dependencies | 9.494 → 7.250 | 10,832,743 → 5,328,039 | gain |
| grow | templates | 8.810 → 7.152 | 10,822,479 → 5,317,775 | gain |

For growing templates, the paired interval is [0.788, 0.820]; for ordinary demand it is [0.736, 0.771]. Small and nongrowing cases largely remain uncertain, including intervals allowing slowdowns. Marginal medians do not replace paired inference.

Primary time includes preparation, setup, complete execution/observation and all engine, answer and prepared disposal. Source/input construction and first-observation timings remain in raw receipts. Meter timings are excluded from speed claims; requested bytes are not RSS and native compilation is excluded. No workload weights or whole-architecture ranking follow from this attribution.

## Validation and next decision

All 150 semantic tests and scoped strict Clippy pass, including source agreement, fresh aliases, residual multiplicity, off-output failure, finite service and cancellation. [Source/binary provenance](s08-normal-copy-attribution/freeze.json), [paired results](s08-normal-copy-attribution/summary.json) and the audit preserve evidence. The independent reference implementation is unchanged.

An initial launch stopped before meter self-check because copied baseline files lacked executable permissions. Restoring executable bits changed no binary contents. The [setup receipt](s08-normal-copy-setup-failure/README.md) remains separate from the successful registered matrix; no timing or allocation observations were excluded.

Keep direct normalization and proceed to the [fresh complete-control confirmation](../registrations/S08-observation-control-confirmation.md). Further small-copy refinements have lower decision value than discovering whether the repaired graph now changes the comparison with Scan, specialization and exact-source elimination. Sustained consumers, compact output contracts, reclamation and broader architecture mechanisms remain required. The subsequent control confirmation and breadth review select T075; broader T074 observation/lifetime remains pending and the research goal stays active.

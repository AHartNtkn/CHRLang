# R01: credible access and preparation before cost comparison

Status: [implemented and independently checked](../results/R01-access-lifetime-entry.md).

Select a bounded access/lifetime extension to the [validated semantic entry](../results/R01-semantic-entry.md). The architectural question remains whether compilation, activation or representation determines total efficiency. Predicate-only selective joins and implicit preparation lifetimes cannot yet distinguish that question credibly.

## Selection against alternatives

The strongest ready alternative is [R02 consuming class/index integration](../results/R02-consuming-integration-entry.md). It can change a representation boundary, but its dedicated control also needs competent binding-aware access. A small common control extension now has higher immediate value than comparing integration against global partner scanning. R02 remains free to use a different access organization; neither this code nor an R01 performance win is a prerequisite.

A full [R04 solver comparison](../results/R04-solving-comparison-selection.md) currently selects among backends for a checked finite relation. It has lower expected architectural value than this missing execution control or R02. Another shared-service or observation refinement has no newly identified decision advantage.

## Bounded implementation contract

Add comparable bound-argument lookup to generic and generated execution, retaining predicate scanning as an explicit access control. Do not constrain source modes. A known-ground lookup may exclude incompatible entries, but an unknown argument must remain available for later activation once a justified equation changes it. Keys must represent entailed values, not raw node identities that differ only through binding. Keep source occurrence IDs and ordered propagation tuples separate from key equality.

Binding-aware maintenance must handle initially unknown arguments, aliases, nested constructors, multiple equal-valued occurrences and consumption. Preserve the same ordered candidate traversal after filtering, so indexed/scanned traces can be compared within the same policy. Account for key derivation, repair, lookup, bucket materialization and retained entries. Retaining the scan mode is a deliberate experimental ablation, not a production fallback.

Provide flat-key chain queries so store size can vary without increasing key depth. Keep structured keys and delayed grounding as separate semantic and later cost contrasts. Add adverse cases with many equal keys and many repairs yielding little useful work; do not equate a selective-key improvement with a universal index win.

Separate prepared-ruleset ownership from per-query state. Both generic and generated modes must reuse legitimate static preparation across runtime queries. Retain explicit cold construction and final destruction boundaries. Make detailed diagnostics opt-in for cost use while preserving the existing trace/audit correctness checks when enabled. Observation and engine teardown remain separately measurable; do not invent streaming semantics for the current no-OR state-inspection API.

## Predictions and validation

If indexing eliminates most partner scans in both policies, a large scan-only activation advantage would have overstated activation's independent value. If activation still avoids revisiting many eligible buckets, its benefit survives the competent lookup control. If key repair or preparation dominates low-selectivity and cold queries, cheap scanning remains a viable architecture choice in those regimes. These are predictions, not measurements.

Before comparative runs, validate full analytic observations and independent terminal/application checks with scan/indexed access, generic/generated execution and both policies. Include different queries reusing one prepared ruleset and verify their bindings, histories and output aliases remain independent. Registration of exact cost workloads, source freeze, measurement boundaries and interpretation still follows implementation review. No timing matrix belongs to this implementation entry; a cost registration and source freeze are required first.

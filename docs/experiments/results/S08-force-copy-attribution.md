# Exposed graph values were copied without an ownership need

**Avoiding the copy in `force` reduces allocation in every tested case and improves total time in some cases.** The cost was an implementation choice, not a requirement of graph execution. Normalization still performs a separate copy, so the output-cost investigation remains open.

The [prospective registration](../registrations/S08-force-copy-attribution.md) isolates one change: inspect an exposed constructor or unknown by reference and return its existing ID. Previously `force` cloned the constructor name and child vector only to discard the clone. Calls, choices, failure, counters and scheduling follow the same paths; normalization and owned answer construction are unchanged.

## What the measurements establish

All 2,896 registered processes completed: 2,240 ordinary timings, 640 allocation runs and 16 cancellation runs. All 320 allocation cells replay exactly. Complete independent source answers agree; query and prepared ownership checks pass. Both modes have eight practical gains, zero losses and 72 unresolved comparisons under the registered pointwise criterion. Unresolved means insufficient evidence for the specified practical difference, not equivalence.

Requested allocation decreases in all 160 mode/source comparisons. Every decrease occurs in execution/observation; preparation, setup and disposal requested allocations are identical. That isolates the measured traffic reduction to the path changed. The complete output contract, including tree ownership, remains intact.

The table shows substantive depth, eight changed queries, resources present and forward starting order. Timings are marginal medians; classifications use seven paired ratios and need not agree with ratios of medians.

| Family | Mode | Primary milliseconds, before → after | Requested bytes, before → after | Paired classification |
|---|---|---:|---:|---|
| single | dependencies | 0.573 → 0.502 | 614,331 → 601,387 | unresolved |
| single | templates | 0.205 → 0.210 | 277,515 → 276,395 | unresolved |
| repeat | dependencies | 2.108 → 1.803 | 2,395,635 → 2,344,531 | unresolved |
| repeat | templates | 0.444 → 0.447 | 766,859 → 763,051 | unresolved |
| distinct | dependencies | 1.872 → 1.733 | 2,395,635 → 2,344,531 | unresolved |
| distinct | templates | 0.724 → 0.710 | 1,044,251 → 1,040,443 | unresolved |
| choice | dependencies | 15.328 → 14.209 | 12,603,427 → 12,048,543 | unresolved |
| choice | templates | 1.906 → 1.762 | 2,654,315 → 2,533,035 | unresolved |
| grow | dependencies | 11.676 → 9.658 | 12,017,783 → 10,832,743 | unresolved |
| grow | templates | 12.980 → 8.870 | 12,003,055 → 10,822,479 | gain |

In the growing-output template example, the paired after/before ratio is 0.717 with pointwise interval [0.662, 0.795]. Ordinary demand on the same configuration is unresolved: ratio 0.885, interval [0.779, 1.078]. The favorable template result does not establish a general advantage over demand or compiled execution.

## Scope and next action

Primary time includes preparation, query setup, execution with complete observation, engine/answer disposal and prepared disposal. Source/input construction, first observation and all individual allocation measurements remain in the raw receipts. Requested heap bytes are not RSS. Native compilation is excluded. These are selected source configurations without workload weights or population-wide claims.

The counter-free source suite passes 150 tests, including fresh unknowns, shared choices, residual aliases and multiplicity, off-output failure, and finite service. Scoped strict Clippy passes. [Frozen sources and binaries](s08-force-copy-attribution/freeze.json), [raw results and analysis](s08-force-copy-attribution/summary.json), and the audit retain provenance. The independent reference implementation is unchanged.

Keep the allocation-free exposed-value return. It has the same result and requires less transient ownership; no registered comparison shows a practical loss. This does not resolve the remaining output cost. `normal` still clones a constructor's child vector before allocating its owned output vector. Inspecting child IDs without that temporary vector requires checking that recursive forcing cannot mutate the parent constructor. That is the next concrete T074 attribution, followed by a fresh comparison with Scan, specialization and applicable exact-source controls.

This continuation is justified by the measured allocation effect and a second distinct copy in the same losing output path. Generalized reuse, broader integrated execution and compilation remain strong alternatives at the next selection review. Compact outputs, sustained consumers, reclamation and whole-architecture comparisons remain required. The goal is active.

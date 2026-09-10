# Graph scanning keeps its timing advantage on smaller queries

Integrated graph scanning passes the practical-gain test against conventional Scan in 52 of the 60 additional size, reuse and CPU comparisons. Eight remain unresolved; none qualifies as a loss. Retaining tuples or partial joins is consistently slower than graph scanning at widths 4 and 16.

The result supports carrying graph scanning into a broader architecture comparison. It does not select an architecture: memory costs, unsupported language features and independent compilation costs remain consequential.

## What changed when the workloads got smaller

The [registered extension](../registrations/S02-multihead-scale.md) uses the same frozen executable, seven engines and six source families as the [earlier lifecycle study](S02-multihead-lifecycle.md). It fills the five remaining width/query-count combinations on the same two CPU placements. Each comparison uses 21 randomized blocks; 8,820 processes completed in 27.4 seconds of driver wall time.

| Width | Queries per prepared ruleset | Graph scan gains against Scan | Unresolved | Losses |
|---|---:|---:|---:|---:|
| 4 | 1 | 11 | 1 | 0 |
| 4 | 4 | 10 | 2 | 0 |
| 16 | 1 | 12 | 0 | 0 |
| 16 | 4 | 9 | 3 | 0 |
| 64 | 1 | 10 | 2 | 0 |

Each row contains six families on two CPUs. Gains require a paired median ratio below 0.9, established by the registered sign test with Holm correction across all 960 directional tests. Favorable medians alone do not qualify. These tests remain conditional on representative blocks despite possible thermal and frequency dependence.

**Preparation does not erase the graph advantage in the tested small-query regime.** For CPU 0 sparse width 4 with one query, graph scan's median total is 13.22 microseconds versus Scan's 26.21. Their median preparation costs are 0.86 and 5.73 microseconds, respectively; graph scan also spends less in setup and execution. This is evidence about these prepared organizations, not a lower bound on conventional compilation.

## Retaining matches needs a more selective justification

Both retained strategies lose against local graph scan in all 48 width-4 and width-16 comparisons. Across the entire extension, full tuples have 54 losses and six unresolved comparisons; partial joins have 56 losses and four unresolved comparisons. Neither has a qualified gain against that control.

Larger nested sources still show how retention can help. At width 64 with one query on CPU 0, partial joins reduce median execution from 163.81 to 91.16 microseconds, but increase setup from 24.94 to 83.20 and engine disposal from 10.57 to 21.40. Total medians are 224.34 for graph scan and 219.06 for partial joins; the registered comparison is unresolved. Phase medians are separate statistics and need not sum to the median total.

On CPU 8 the same partial-join case has a favorable paired median ratio of 0.814, but still does not pass the multiplicity-corrected test. The earlier four-query study's qualified nested gain remains evidence within its own scope. This extension neither pools samples with that study nor turns an unresolved single-query result into equivalence.

Dense setup remains the adverse mechanism. At width 16 with one query on CPU 0, partial joins spend a median 255.90 microseconds in setup within a 299.69-microsecond total, compared with graph scan's 28.22-microsecond total. This agrees with the previously inspected eager retention of successful prefixes and their environments. Demand-sensitive retention, different head orders and compact ownership remain untested alternatives to this organization.

## The other controls prevent a premature policy choice

Against conventional Scan, full tuples have seven gains, 25 losses and 28 unresolved comparisons; partial joins have six gains, 24 losses and 30 unresolved comparisons. Beating conventional scanning in a few cells therefore does not establish that retention is needed: the simpler graph scan is a stronger control here.

Indexed execution has 34 losses and 26 unresolved comparisons against Scan, with no qualified gains. Specialized indexed execution has 37 losses and 23 unresolved comparisons. These findings apply to the present access organization and source families; they do not settle broader discrimination or partner-order plans.

Scan specialization has two gains and two losses, with 56 unresolved comparisons. Both gains occur in broad updates at width 4 with four queries, one on each CPU. The losses occur on CPU 8 at width 64 with one query, for sparse and broad updates. This mixed result argues for retaining preparation and reuse in subsequent specialization comparisons.

## Memory and measurement limits still matter

This extension performs no new allocation measurement. The unchanged source and binary preserve the relevance of the earlier allocation/ownership gate, whose independent audit passes again. That evidence includes graph scan's higher requested traffic and peak storage in the nested four-query example, despite its timing advantage over Scan. Requested allocations are not RSS, and no workload weights justify collapsing time and memory into one score.

Every process checks complete answers independently outside the measured intervals. Primary totals include source construction, preparation, changing-query input/setup/execution/observation and disposal. They exclude process startup, validation, inter-phase harness bookkeeping and native compilation. Preflight warms the execution path. Cancellation is recorded separately at unequal adapter progress. Broader guards, search, sustained consumers and complete language support remain outside this comparison.

The audit reconstructs all 1,260 randomized blocks and 8,820 commands, validates output metadata and phase structure, checks the absence of allocation diagnostics, verifies frozen binary/driver/registration/auditor hashes, and recomputes all 480 paired comparisons and 960 corrected tests. No engine or reference implementation changed.

## Next decision: qualify native identities rather than refine this timing matrix

Select T080's native identity, equality and propagation correspondence next. Its [61 reference-checked obligations](S03-native-identity-obligations.md) distinguish replacing an occurrence from binding its value, ordered propagation histories, fresh unknowns and legal effect order. The current native compiler admits ground consuming programs only. A native representation that satisfies those obligations could change whether a direct graph architecture is viable for the broader language.

| Ready investigation | What could change | Reason for its current priority |
|---|---|---|
| Native identities, equality and history | Feasibility and required responsibilities of a serious alternative architecture | Selected: independent obligations exist, but no native representation satisfies them yet |
| Demand-sensitive joins and compact ownership | Whether retention can avoid the measured dense setup cost while preserving nested benefits | Required; the current graph scan already supplies a competitive path without these caches |
| Broader guards, search and coherent integrated execution | Whether graph scan's bounded gain survives full semantic responsibilities | Required; the present result supplies a useful candidate, not complete language qualification |
| Richer solving and reuse | Whether source derivation or repeated futures can eliminate more work than matching improvements | Required; their existing favorable/adverse results remain bounded |

The eight unresolved graph-scan comparisons have paired median ratios between 0.658 and 0.848. They cannot be called gains, but their uncertainty does not currently change the decision to carry graph scan forward while qualifying a distinct architecture. More samples would refine the boundary of an already established bounded advantage. Native identity correspondence has greater potential to change which architectures can enter the complete comparison. This is a priority judgment, not experimental resolution of those eight cells.

Reconsider integrated language support and coherent architecture qualification after the native identity source gate, or immediately after a consequential native obstruction. T072 remains unfinished. The research goal remains active.

## Evidence

[Raw runs](s02-multihead-scale/runs.jsonl), [registered order](s02-multihead-scale/order.json), [audit and comparisons](s02-multihead-scale/audit.json), [frozen inputs](s02-multihead-scale/freeze.json), [environment](s02-multihead-scale/environment.json) and [completion receipt](s02-multihead-scale/complete.json) retain the experiment. The [driver](../../../research/chr-relational/experiments/multihead_scale.py) and [independent auditor](../../../research/chr-relational/experiments/audit_multihead_scale.py) reproduce execution and analysis.

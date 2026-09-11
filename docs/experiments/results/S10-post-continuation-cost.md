# Counting makes demand competitive in the complete mixed paths

Counting repays its full measured cost on the substantive sources and makes demand competitive with strong explicit execution. The selected counted demand paths are faster than generated Global scanning, but their confirmed intervals do not establish a 10% gain. Counting has a confirmed overhead loss on the zero-work demand source.

## Complete costs change the interpretation of shared work

The [source/work gate](S10-post-continuation.md) showed conditional execution sharing 128 explicit traversal steps as 16 common steps. Counting eliminates those steps in both paths. This pilot measures what remains, including the preparation and rewriting needed to obtain that saving.

The table uses three choices, depth 16, history present, original arrival order and four changing queries with all answers retained. Times are pilot medians of the complete 36-phase sum, not isolated execution or confirmation medians.

| Source and path | Original time | Counted time | Original requested bytes | Counted requested bytes | Counted peak excess |
|---|---:|---:|---:|---:|---:|
| Common — generated Scan | 1.107 ms | 0.492 ms | 1,484,859 | 590,592 | 82,013 |
| Common — demand with miss reuse | 2.939 ms | 0.475 ms | 2,700,743 | 491,436 | 76,184 |
| Common — conditional | 2.369 ms | 0.709 ms | 2,801,030 | 632,895 | 50,091 |
| Independent — generated Scan | 1.452 ms | 0.791 ms | 1,733,785 | 816,349 | 121,733 |
| Independent — demand with miss reuse | 3.146 ms | 0.627 ms | 2,778,651 | 691,715 | 114,811 |
| Independent — conditional | 21.483 ms | 4.061 ms | 23,255,941 | 3,611,881 | 94,628 |
| Early failure — generated Scan | 0.644 ms | 0.344 ms | 802,501 | 339,010 | 44,936 |
| Early failure — demand with miss reuse | 1.451 ms | 0.338 ms | 1,493,630 | 299,803 | 45,833 |
| Early failure — conditional | 3.186 ms | 0.620 ms | 3,179,373 | 619,198 | 37,210 |

Demand's counted path includes static-propagation initialization and value-choice preparation. The other paths execute the original propagation rule. These are actual complete paths returning the same observations; the comparison does not isolate demand's executor from its compiler passes. That distinction determines the next control below.

Conditional execution retains the lowest counted peak in these examples while spending more time and traffic. Sharing, requested traffic, live memory and elapsed time therefore remain distinct decision dimensions. No workload weights or universal winner are inferred.

## Targeted confirmation preserves gains and resolves overhead

The [registered confirmation](../registrations/S10-post-continuation-confirmation.md) adds 64 paired repetitions for 39 contrasts using the unchanged ordinary binary. It includes all three demand policies against generated Global Scan, generated Active Scan and inferred Scan; counting versus original execution for three representative paths; and a zero-work overhead control.

| Confirmed comparison | Common | Independent | Early failure |
|---|---:|---:|---:|
| Counted/original demand with miss reuse | 0.150–0.168 | 0.202–0.216 | 0.190–0.212 |
| Counted/original generated Scan | 0.443–0.485 | 0.511–0.563 | 0.490–0.540 |
| Counted/original conditional | 0.276–0.301 | 0.185–0.197 | 0.231–0.251 |
| Counted demand with miss reuse / counted generated Scan | 0.877–0.940 | 0.845–0.927 | 0.822–0.963 |

All nine counting comparisons establish a practical gain under the registered 10% criterion. The last row establishes a directional benefit, with uncertain 10% magnitude. Birth without miss reuse also has a directional benefit against generated Scan in all three cases; the template policy is within ±10% in all three. Several comparisons against generated Active and inferred scanning establish practical gains. [Every interval and raw ratio](s10-post-continuation-cost/confirmation/audit.json).

Counting loses on the zero-choice, zero-depth demand case: its counted/original interval is 1.196–1.400. The pilot requests 93,058 bytes versus 86,533 and takes a median 126.7 µs versus 97.3 µs. Conditional and generated Scan overhead magnitudes remain uncertain; they are not classified as practical losses from their medians alone.

Across the selected contrasts there are 21 practical gains, one practical loss, four results within ±10%, and 13 unresolved practical magnitudes. These counts have no workload weighting. All are above the registered signal floor. The intervals are the 19th through 46th ordered ratios of 64; under independent samples from a stable distribution, their simultaneous error bound is at most 0.02408. Those statistical assumptions are not established by the run. First/last-half medians and all samples are retained. Selection from the pilot makes this confirmation, not held-out validation.

## What the phases establish

Counting's large saving occurs in execution and observation. On common demand with miss reuse, their grouped pilot median falls from 2.749 ms to 0.328 ms, while preparation rises from 53.7 µs to 77.9 µs. The avoided work repays that added preparation on this source. This does not attribute every instruction or allocated byte to the counting transformation. Group medians need not sum to the median total. [Derived phase summaries](s10-post-continuation-cost/phase-summary.json).

The runner separately charges source construction, counting inference, static-propagation inference, value-choice preparation, engine preparation and temporary source disposal. Each query charges input, counting rewrite, static initialization, setup, execution with owned observation, engine disposal and consumer action. Prepared and consumer disposal close the session. Immediate/window/all consumers retain across completed queries; this is not within-query streaming.

## Evidence and validation

The [pilot registration](../registrations/S10-post-continuation-cost.md) fixes 96 scenarios and 2,688 cells across 14 modes and original/counted execution. All 26,880 processes complete: 2,688 excluded warmups, 13,440 primary timings, 5,376 allocation runs and 5,376 ordinary/metered cancellation runs. Every allocation pair repeats exactly in all phases; all measured owners restore. The campaign takes 112.82 seconds under its registered resource bounds.

The confirmation completes 5,070 processes, including 78 excluded warmups. Both stages validate full scalar and analytical observations outside measured intervals, including multiplicity, residual fuel, fresh aliases and answers retained after preparation disposal. Actual generated Global/Active controls pass 144 additional source comparisons. Strict Clippy and ordinary/meter smoke checks pass. Primary engine/kernel/work counters are disabled; allocation diagnostics use a separate build. Requested bytes and peak are not RSS.

[Runner](../../../research/chr-direct-conditional/examples/post_continuation_cost.rs), [pilot freeze and source archive](s10-post-continuation-cost/freeze.json), [pilot audit](s10-post-continuation-cost/audit.json), [all cells](s10-post-continuation-cost/summary.csv), [confirmation freeze](s10-post-continuation-cost/confirmation/freeze.json). Native compilation, process startup, validation, preallocated measurement buffers and sustained lifetime are outside the measured endpoint. This result does not establish complete architectural lifecycle superiority.

## Give the explicit controls the same preparation opportunity next

Keep T078 active and implement static-propagation initialization for the explicit and conditional paths, including correctly generated transformed source. Compare original versus initialized source independently before measuring the changed full paths. Charge inference, query initialization, preparation and disposal in each case. Preserve history multiplicity, adverse zero-work cases and complete observations; do not replace the source task to make the control faster.

This control is more consequential than more unchanged repetitions. The current directional differences against generated Global Scan are small enough that avoiding one interpreted propagation application per query could change their interpretation. A result from the whole demand pipeline cannot establish which part deserves architectural credit. Match that compiler opportunity before using the comparison to select execution organization.

Sustained ownership is the strongest ready alternative: conditional's lower peak and demand's finite complete gains make long-running retention relevant. The preparation control takes one bounded package first because it can change which competitors should enter that study. Broader integrated execution, coarser recognition, restoration and the other questions remain required. This pilot and confirmation bring the count to three packages after the full review; review the entire portfolio after the next package. The research goal remains active.

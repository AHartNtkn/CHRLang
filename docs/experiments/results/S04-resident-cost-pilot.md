# Restoration costs remain conditional after sole-branch reuse

Corrected replay is inexpensive on the deterministic source, but reconstructing competing branches remains costly. Copying and Indexed execution show opposing time and peak-memory advantages. These results support credible controls for the next investigation, not a universal restoration policy or architecture.

The [registered pilot](../registrations/S04-resident-cost-pilot.md) completes **1,344 processes across 192 configurations**: twelve sources, eight modes, one/eight changed queries, five ordinary timing repetitions and two separate allocation repetitions. All complete-answer checks pass. All 192 allocation pairs reproduce exactly, measured phases preserve live-byte continuity, and final disposal restores the initial live baseline. Four independently gated binaries, package tests, strict Clippy and formatting pass.

## Reconstruction still dominates branching replay

The following figures are eight-query lifecycle medians in milliseconds, with the full five-sample range. They include prepared-owner construction, query setup, execution through owned answers, query/answer release and prepared-owner disposal. They exclude source construction, validation and compilation. Timings are exploratory; no formal gain/loss classification was registered.

| Source | Copy | Root replay | Checkpoint 1 | Indexed |
|---|---:|---:|---:|---:|
| linear | 0.695 [0.634, 0.814] | 0.648 [0.600, 0.702] | 0.681 [0.618, 0.761] | 0.925 [0.858, 1.080] |
| retained | 4.071 [3.927, 4.828] | 55.071 [50.747, 56.776] | 6.957 [6.604, 7.539] | 6.235 [6.047, 7.073] |
| mutation | 52.801 [51.360, 53.517] | 16,141.355 [15,773.095, 16,264.523] | 115.469 [108.003, 118.450] | 21.273 [20.674, 24.851] |
| work | 17.745 [16.997, 19.319] | 2,586.759 [2,480.899, 2,786.149] | 19.531 [18.916, 20.523] | 8.591 [8.103, 8.903] |
| deep | 17.173 [14.988, 18.613] | 683.111 [670.619, 689.956] | 18.621 [17.911, 19.076] | 16.638 [15.482, 18.364] |
| early | 4.783 [4.710, 5.031] | 4.836 [4.679, 6.909] | 4.715 [4.644, 5.035] | 3.567 [3.472, 3.905] |

**The deterministic case no longer requires repeated reconstruction.** The preceding independent [work gate](S04-resident-replay-source-gate.md) establishes 100 actual steps for 100 continuing calls, versus the former 5,050. Current linear timings overlap among restoration modes. This pilot compares current modes; it is not a historical paired timing attribution for that correction.

**Increasing the checkpoint interval trades snapshots for repeated execution, with an unfavorable balance on the inspected branching sources.** On mutation, eight-query medians are 115.469 ms at interval1, 188.286 ms at interval4, 473.264 ms at interval16 and 16,141.355 ms for root replay. The corresponding requested traffic is 158.6, 239.5, 571.3 and 19,133.7 MiB. On work-between-choices, interval1 takes 19.531 ms versus 46.140 and 166.029 ms at intervals4/16. The separate work gate counts 1,018, 8,376 and 139,320 actual source steps for interval1, interval16 and root replay on that source; Copy performs 1,004.

**Execution, rather than preparation, explains the large replay costs.** In mutation root replay, median execution/observation is 16,140.609 ms out of a 16,141.355 ms total. Preparation is about 23 microseconds. One-query root replay is also expensive at 2,057.868 ms. Reusing preparation eight times cannot remove reconstruction repeated within every query.

The implementation reconstructs a saved recipe at every FIFO service switch; the sole-state correction applies only when competitors are absent. Trail similarly restores branch versions through undo/redo and retains changes needed by competing branches. Mutation Trail takes 475.272 ms and peaks around 4,125 KiB. These are costs of these representations and switching policies. They do not prove that all trailing, batching or alternative progress contracts have the same costs.

## Lower peak memory can accompany much more work

Requested allocation traffic and incremental peak live requested bytes measure different obligations. The table uses the same eight-query lifecycle. Each cell is **traffic in MiB / peak growth in KiB**; neither quantity is RSS.

| Source | Copy | Root replay | Checkpoint 1 | Indexed |
|---|---:|---:|---:|---:|
| retained | 3.12 / 184.7 | 39.30 / 187.8 | 6.18 / 186.6 | 11.11 / 1,070.7 |
| mutation | 56.40 / 201.8 | 19,133.74 / 216.6 | 158.61 / 208.3 | 25.53 / 1,207.2 |
| work | 20.90 / 31.5 | 2,944.33 / 28.2 | 23.53 / 32.1 | 8.34 / 238.3 |
| deep | 21.04 / 141.5 | 970.07 / 140.0 | 25.39 / 137.3 | 19.78 / 1,241.8 |

**Indexed's mutation advantage has a memory cost.** It takes about 21.3 ms versus Copy's 52.8 ms and requests less traffic, but its peak growth is about 1,207 KiB versus 202 KiB. Indexed-COW reduces that peak to about 1,097 KiB and traffic to 24.66 MiB; its 21.323 ms median overlaps ordinary Indexed. On retained-store sources, Copy is faster and uses less traffic and peak memory than either Indexed control. No workload weighting is assumed between these regimes.

**Root replay's small peak is not free.** On work-between-choices it reduces peak growth from Copy's 31.5 to 28.2 KiB while increasing traffic from 20.90 to 2,944.33 MiB and median time from 17.745 to 2,586.759 ms. This source does not justify that exchange merely because one memory number improves. Other state sizes and memory limits remain an explicit sensitivity question.

## First answers and cancellation do not rescue mutation replay

Mutation's first answer over a fresh measured prepared owner takes a median 2,005.678 ms under root replay, versus 6.288 ms for Copy and 2.340 ms for Indexed. The separately measured first-answer cancellation lifecycle takes 1,998.196, 6.384 and 2.552 ms respectively. Reconstructing work happens before that answer is available, so early cancellation cannot avoid it on this source.

All cancellation trials use the same prepared owner after the complete-query trials; prepared-owner disposal follows cancellation. The primary full-query sum excludes cancellation phases but includes that final owner release. Source/query construction, oracle work and answer validation occur outside measured intervals and can affect caches or allocator state. Two validated warmups precede measured preparation: “first” does not mean a cold process. Compilation remains unmeasured, and output consumers retain answers until each query ends. Sustained streams and alternative consumer lifetimes are separate required comparisons.

## Next test actual separation and reunion

Select a source-correct temporary separation/reunion gate next under T077. Its strongest ready alternative is T072 integrated dependency repair and full consuming-source costs. Both can change architecture: reunion can avoid preserving one coupled search state during independent work; integration can remove repeated discovery and service boundaries.

Reunion comes first because the current restoration controls now have source, ordinary-time and allocation evidence, while actual reconnection remains unimplemented in this comparison. Establish a bounded occurrence-level witness that performs substantive independent work, then reconnects through a late binding or shared consumer. Compare it with ordinary execution and permanent factoring; include same-predicate independent occurrences and frequent-reunion near misses. Preserve choice correlation, raw multiplicity, fresh identities, consumption and declared progress. A correctness gate is the next boundary, followed by a cost registration only if the distinctive mechanism is exercised.

Do not spend another package narrowing the current replay medians: their uncertainty cannot plausibly reverse the large reconstruction consequence on these sources. That judgment does not settle larger read-heavy states, alternative switching policies, adaptive splitting or sustained lifetime. Reassess reunion against integrated work at its source gate. If source contracts obstruct the proposed mechanism, investigate that obstruction rather than substituting permanent independence and calling reunion complete.

This is T077's second bounded package after resident replay correctness. The broader S04 direction, integrated execution, complete architectures and held-out challenges remain open.

## Reproduce and inspect

[Registration](../registrations/S04-resident-cost-pilot.md), [audit](s04-resident-cost/audit.json), [all endpoints and ranges](s04-resident-cost/summary.json), [individual phase rows](s04-resident-cost/phases.csv), [run order](s04-resident-cost/order.json) and [source/binary freeze](s04-resident-cost/freeze.json) accompany individual command/stdout/stderr receipts in the same directory. Four feature-tree receipts verify that engine, kernel and replay counters are absent. Allocation runs use a separate allocator build and their elapsed times are excluded from timing claims.

Run `python3 research/chr-restoration/experiments/resident_cost.py build`, then `run`, then `python3 research/chr-restoration/experiments/summarize_resident_cost.py`. The build command refuses to overwrite an existing freeze; the run command verifies and reuses already successful exact receipts. Audit uses the current frozen source contents and binaries, so later source changes require checking out this evidence revision before reproduction.

# Specialization helps, but the choice/check advantage survives these compiled controls

On substantive choice/check sources, inferred specialization improves on Scan without removing the explicit choice tree. Conditional execution retains a large advantage on the measured choice/check sources, while the low-sharing streams favor explicit execution. Reusing prefix artifacts saves repeated setup but does not make prefix lowering competitive on these sources.

## The complete comparison

The [source gate](S10-arrival-controls.md) qualified specialization, fresh prefix lowering and reusable prefix artifacts independently. This pilot charges their full measured lifecycle against Scan, resumable contextual execution and two conditional support orders. Prefix controls are excluded only from streams that their certificate rejects; the other five controls execute those streams.

The same-signature case runs four queries at fixed depth with alternating work4/0 and payload8/0. The changing-signature case runs depths n..n+3 with fixed work4/payload8 and alternating input order. These are separate regimes; their absolute costs cannot be attributed solely to cache hits because the query work also differs. Comparisons between engines within a cell use the same queries.

Artifact preparation is lazy and measured in setup. One repeated signature retains one artifact; four changed signatures retain four. A separate measured phase releases artifacts before consumer and prepared-rule disposal. No artifact construction is treated as free preparation.

## Results

All1,344 comparative processes complete. All192 allocation pairs replay exactly. The audit verifies frozen sources/binaries, registered configurations and endpoints, phase continuity,432 consumer-independent engine groups,32 semantic-output groups and final owner restoration.

Four changing queries, immediate answer release, successful sources:

| Source | Control | Requested bytes | Peak live growth | Lifecycle median ms [min, max] |
|---|---|---:|---:|---:|
| oldest-first | Scan | 83,458,572 | 23,889,472 | 81.86 [72.36, 94.09] |
| oldest-first | Inferred specialization | 66,698,711 | 21,533,211 | 53.95 [51.57, 62.68] |
| oldest-first | Fresh prefix | 85,097,664 | 22,975,962 | 83.84 [72.17, 90.44] |
| oldest-first | Prepared prefix | 87,271,300 | 23,009,600 | 76.85 [64.21, 90.50] |
| oldest-first | Resumable contextual | 150,436,380 | 8,283,952 | 63.89 [58.03, 83.73] |
| oldest-first | Conditional ascending/direct | 2,256,704 | 84,509 | 4.96 [4.54, 5.41] |
| oldest-first | Conditional reverse/general | 1,751,024 | 68,205 | 3.84 [3.39, 4.21] |
| aliases | Scan | 9,940,433 | 352,897 | 7.56 [6.94, 8.33] |
| aliases | Inferred specialization | 8,550,347 | 383,367 | 5.83 [5.10, 8.57] |
| aliases | Resumable contextual | 10,397,489 | 36,047 | 4.06 [3.61, 4.64] |
| aliases | Conditional ascending/direct | 199,081,941 | 2,829,354 | 391.05 [374.17, 454.22] |
| aliases | Conditional reverse/general | 171,774,165 | 201,755 | 352.74 [272.44, 367.26] |

**The stronger controls preserve the two-regime conclusion.** Specialization lowers oldest-first changing-query traffic from83.46MB to66.70MB and its exploratory median from81.86ms to53.95ms. Conditional reverse/general requests1.75MB with a3.84ms median. On the alias stream, explicit controls remain much cheaper and faster; specialization requests8.55MB versus171.77MB for reverse conditional execution. These are matched source/lifecycle comparisons, not an overall ranking.

**Stable signatures help setup, but not total prefix traffic.** On four same-signature oldest-first queries, fresh prefix lowering requests17,184,528 bytes and prepared prefix17,492,743. Their medians are12.34ms [10.95,17.07] and13.42ms [11.06,21.17], respectively. Overlap precludes a timing policy from these five samples. The prepared artifact retains12,180 bytes; changing signatures retain four artifacts totaling45,060 bytes before measured release.

Across completed-query cells, specialization lowers traffic against Scan in24 cases and raises it in four. Prepared prefix raises traffic against fresh prefix in all14 completed-query cases. Cancellation cells remain separate from those counts. Tiny, newest-first, failure, retained-answer and cancellation outcomes remain in the [complete summary](s10-arrival-lifecycle/summary.json).


## Why reusable preparation does not win here

On the same-signature oldest-first case, fresh lowering requests44,053/42,220/44,053/42,220 setup bytes over the four queries. Prepared reuse requests58,537/9,897/14,370/9,897. Thus reuse does save setup after its first query. However, its four execution phases request more bytes than fresh lowering:5,375,184/3,310,691/5,375,184/3,310,691 versus5,239,300/3,252,545/5,239,300/3,252,545.

This separates the measured phases: a setup benefit is outweighed by execution traffic, with retained artifacts adding a distinct lifetime cost. Code inspection identifies a relevant representation difference: reusable shapes parameterize all constraint arguments, while per-query lowering embeds known term structure and passes only query variables into its entry rule. The phase difference is measured; the exact contribution of that parameterization has not been isolated by this pilot.

Even eliminating all setup traffic would leave the prefix controls far above the conditional execution traffic in the substantive arrival cases. A narrower setup optimization cannot explain away that gap. A different explicit state representation, native execution or source-derived solving can still change the comparison; those are broader investigations.

## Validation and limits

The [registration](../registrations/S10-arrival-lifecycle.md) fixes192 cells,384 allocation runs and960 ordinary timing runs. The [evidence directory](s10-arrival-lifecycle/) contains commands, frozen sources/binaries, randomized orders, raw outputs and the independent audit. [Sizing](s10-arrival-lifecycle-sizing/) checks artifact ownership and cancellation before the comparative matrix.

Each process validates complete analytical and independent scalar answers with separate owners before measurement. Counter-free ordinary-allocator timing and allocation diagnostics use separate builds. Source/input construction, preparation, setup, execution/observation and all query/artifact/consumer/prepared disposal count. First observation is a subinterval and is never added twice. Compilation, process startup, fixed harness buffers and correctness replay are excluded. No native compilation or cold-cache claim follows.

Arrival cancellation stops the first query after100 engine calls; stream cancellation stops it after four answers. These test safe disposal at different progress points, not equal completed work or a cross-engine cancellation-speed ranking. Later queries complete and reuse the intended artifacts. Actual retained output storage includes the previously attributed conditional residual-vector spare capacity; the audit accounts for it only when comparing semantic payload owners, not when reporting costs.

Timing medians and ranges are exploratory. No workload weights, universal order or architecture winner is inferred. Direct solving that removes the alternatives, native compilation, broader restored-state controls and long-lived unbounded signature populations remain unresolved.

## Next investigation and breadth

This is the second package after the order-lifecycle breadth review, following stronger-control qualification. It answers the immediate objection that the earlier favorable conditional result lacked the already-available specialization and prefix controls. Refining prefix setup further cannot eliminate the dominant execution traffic identified here.

**Return to T079 effect/ownership certification with an executable beneficiary.** Its existing kept-head, equality-activation and independent-consumption witnesses give it a concrete starting point. Identify an additional runtime responsibility that a sound property can remove beyond current serial accounting and unary dispatch; test inference, checked declarations and required admission through that operation. Do not claim a benefit from a checker that changes no execution.

The strongest alternative is direct source-derived solving of choice/check work. It could remove the explicit alternatives entirely, but its eligibility must distinguish private logical deductions from externally observed or consuming effects. Effect/ownership analysis can inform that boundary as well as other execution organizations. This is a priority judgment: it neither rejects direct solving nor makes language restrictions mandatory. Compare a concrete elimination proposal against the effect beneficiary at the next gate, and pursue whichever has stronger decision value relative to its actual implementation cost.

T078 remains unfinished for direct solving, native generation and coherent complete architecture comparisons; T074 remains unfinished for broader support and lifetime questions. No research closure follows from these two source regimes.

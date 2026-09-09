# Join pruning has bounded gains, but does not close the whole-path cost gap

Prefix traversal reduces allocation in some incompatible and correlated sources, while dense and mixed sources expose overhead. Conventional Scan and contextual execution remain substantially cheaper in several complete-path comparisons. The result supports carrying distinct controls forward, not selecting one join organization universally.

The [registered primary pilot](../registrations/S10-join-lifecycle.md) completes 2,940 processes and 420 exact allocation pairs. A [registered head-list attribution](../registrations/S10-join-prefix-attribution.md) adds 168 processes and 84 exact pairs. All complete observations, phase accounting and disposal checks pass. Revised prefix timing is not measured.

## Complete costs preserve the contrary cases

The table gives total requested heap traffic in decimal MB at n=3, four changed queries per preparation, and per-query answer release. For suffix families, n=3 means eight suffix occurrences; mixed families use their original n=3. Costs include preparation, input, delivery, disposal and three cancellation probes. Prefix values include the head-list correction described below.

| Source | Filtered conditional | Direct tuple check | Revised prefix | Scan | Contextual resumable |
|---|---:|---:|---:|---:|---:|
| Incompatible prefix | 0.280 | 0.297 | 0.278 | 0.142 | 0.087 |
| Correlated suffix | 1.955 | 1.936 | 1.886 | 0.287 | 0.292 |
| Independent suffix | 4.204 | 4.385 | 4.235 | 0.502 | 0.513 |
| Dense compatible suffix | 1.009 | 1.043 | 1.040 | 0.170 | 0.161 |
| Mixed independent | 3.473 | 3.594 | 3.534 | 0.887 | 2.394 |
| Mixed delayed binding | 5.186 | 5.307 | 5.247 | 1.103 | 3.516 |
| Mixed early failure | 1.913 | 1.907 | 1.877 | 0.367 | 0.661 |

**Pruning helps within conditional execution in some regimes.** Revised prefix traffic for correlated suffixes is 69,584 bytes below filtered discovery and 50,602 below direct checking. Incompatible-prefix traffic is only 1,160 bytes below filtered discovery. Those gains include preparation and cancellation, not only successful joins.

**Avoiding candidate work does not guarantee lower total cost.** Revised prefix remains above filtered discovery on independent suffixes: the exact totals are 4,235,185 versus 4,204,185 bytes, a 31,000-byte increase. Dense compatible suffixes also favor filtered discovery. The conditional controls retain pool construction, support operations, candidate/dependency machinery and exact branch observation; early pruning does not remove those responsibilities.

**Peak demand can disagree with traffic.** On mixed independent queries, revised prefix peak growth is 65,628 bytes versus Scan's 81,893, despite about four times Scan's traffic. On the incompatible-prefix source it is 18,159 versus Scan's 16,610 and contextual resumable's 9,106. No weighted total or universal winner follows from these different quantities.

## One measured implementation cost was unnecessary

The initial prefix path rebuilt the prepared head-reference vector on every cursor service call. Active traversal already owns its occurrence pools, so it does not need those head references. The correction dispatches active cursors before head-list construction; initial filtering and pool construction remain charged.

All 22 existing source/work signatures remain identical. Across the 84 revised cells, changed memory readings occur only during first delivery and cancellation service. Preparation, input, setup and disposal readings are unchanged. Revised traffic is no greater in any cell. In the substantive mixed-independent example it falls from 3,576,142 to 3,534,158 bytes; correlated suffix traffic falls from 1,891,878 to 1,885,558. This correction does not reverse the broader comparisons above.

The primary source snapshot is preserved and checked against its original hash. Primary timing describes that original implementation. The attribution measures allocation only; it supplies no revised speed claim.

## Ownership and cancellation were checked directly

All queries agree with independent complete expectations and the scalar evaluator outside measured intervals. Measured answers preserve multiplicity, residual constraints and aliases. Held answers are checked again after preparation is dropped.

Cancellation occurs after 1, 64 and 256 service calls. In the primary matrix's first allocation repeats, diagnostics find live direct-check state in four snapshots and live prefix cursors in eight; retained pools occur in every conditional build. These diagnostics repeat exactly across allocation and timing processes, and across original/revised prefix allocation. Cancellation actually exercises retained discovery ownership, rather than merely an empty search.

Each query and cancellation restores preparation plus held answers. Dropping preparation leaves exactly consumer-owned answers; releasing those answers restores the original measured baseline. For retained independent-suffix answers, the conditional path holds 54,320 requested bytes versus 44,336 in Scan/contextual answers, despite equivalent observations. Equal answers need not have identical owned allocation layouts; this distinction belongs in the broader output-lifetime comparison.

The revised prefix peak with four retained independent-suffix query results is 80,620 bytes; Scan is 78,997 and contextual resumable 70,057. Bounded retained consumers therefore change the comparison. This is not yet a sustained-window or long-stream result.

## Timing and validation limits

Five ordinary-allocator timing samples per primary cell remain exploratory. For correlated suffixes, original prefix median is 1.600 ms (range 1.059–2.319), filtered conditional 1.599 ms (1.143–2.859), Scan 0.421 ms (0.283–0.506), and contextual resumable 0.252 ms (0.168–0.287). The conditional ranges do not support a fine speed ranking. No clock subtraction or practical-win classification is used.

[Primary receipts](s10-join-lifecycle/) and [revised attribution receipts](s10-join-prefix-attribution/) contain 3,108 process records, summaries, source/binary hashes, exact execution orders and ownership audits. Both auditors pass. The corrected prefix path passes all 125 crate tests, 37 selected counter-free tests and scoped strict Clippy; filtered and direct runner configurations also pass Clippy. Rust formatting and Python syntax checks pass.

Requested heap allocation is not RSS. Execution and observation are coupled in delivery. Original source construction, process startup, native compilation and validation are excluded. No compilation-inclusive superiority is claimed.

## Next decision

The [breadth review](S10-join-breadth-review.md) selects remaining language-property comparisons under T079. Join mechanisms now have direct source and lifecycle evidence with contrary controls; another local join refinement has lower immediate decision value than testing whether stronger source properties remove responsibilities and what expressiveness they cost. Broader join variants, equality relevance, sustained lifetime and coherent/held-out architecture comparisons remain required. T078 and the full goal are unfinished.

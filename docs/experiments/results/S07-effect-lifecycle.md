# The effect certificate saves bookkeeping; declarations do not buy a different runtime

Certifying immutable bindings reduces allocation in the useful read-heavy cases, but the saving is modest in these complete workloads. Inferred, checked and required admissions have identical allocation and ownership results on accepted sources. The experiment supports a bounded optimization opportunity, not a mandatory language restriction or general speed policy.

## Complete costs

The [source gate](S07-binding-effects-gate.md) proved the extra runtime operation could disappear beyond unary dispatch and serial accounting. This [registered pilot](../registrations/S07-effect-lifecycle.md) now charges checking, source/input setup, complete execution/observation, changed-query reuse and disposal. Each displayed case uses four changing queries, immediate answer release, no final failure, and starts at64 requests.

| Source | Control | Requested bytes | Peak live growth | Lifecycle median ms [min, max] |
|---|---|---:|---:|---:|
| blocked | Conditional, feature off | 522,795 | 28,727 | 1.008 [0.816, 1.333] |
| blocked | Conditional, feature on | 522,795 | 28,727 | 1.404 [0.882, 1.757] |
| blocked | Conditional, inference | 504,123 | 27,822 | 0.769 [0.750, 1.285] |
| blocked | Scan | 168,417 | 23,498 | 0.183 [0.119, 0.239] |
| blocked | Inferred specialization | 170,990 | 23,795 | 0.143 [0.135, 0.267] |
| rewrite | Conditional, feature off | 3,738,843 | 251,595 | 8.620 [6.796, 10.235] |
| rewrite | Conditional, feature on | 3,738,843 | 251,595 | 7.989 [7.491, 9.494] |
| rewrite | Conditional, inference | 3,720,171 | 246,915 | 7.659 [6.725, 9.031] |
| rewrite | Scan | 3,732,393 | 186,812 | 8.545 [7.271, 9.728] |
| rewrite | Inferred specialization | 3,884,022 | 168,943 | 10.669 [9.619, 14.019] |
| writer | Conditional, feature off | 2,008,826 | 35,074 | 7.765 [7.732, 10.372] |
| writer | Conditional, feature on | 2,008,826 | 35,074 | 8.161 [7.130, 9.865] |
| writer | Conditional, inference | 2,008,826 | 35,074 | 8.252 [7.462, 8.899] |
| writer | Scan | 352,013 | 23,105 | 0.419 [0.360, 0.585] |
| writer | Inferred specialization | 209,314 | 18,970 | 0.230 [0.226, 0.426] |

**The bookkeeping saving is real but source-dependent in importance.** Inference saves18,672 requested bytes across four blocked queries and the same amount across four rewriting queries. The former is3.6% of the ordinary total; the latter is0.5% because constructor rewriting adds substantial work without adding the same blocked-read dependencies. Neither percentage supplies a workload weight.

Across completed-query cases, inference lowers traffic eight times and leaves it unchanged seven times, including the actual writer and tiny cases. No completed case increases requested bytes. Feature-off and feature-on ordinary controls have identical allocation totals in all18 lifecycle configurations, including cancellation. This does not prove zero instruction or timing overhead from enabling the feature.

**The conventional controls remain consequential.** On blocked matching and the actual writer, Scan/specialization have substantially lower total allocation and timing medians than the conditional paths. On substantive constructor rewriting the timing ranges overlap, and allocation/peak costs differ. Removing this one index does not establish a conditional architecture winner.

Five timing repetitions yield exploratory medians and ranges. Feature-on/off and admission variants have noisy timing differences despite equal allocation. Optional optimization speed adoption therefore remains open; these samples do not justify a mandatory policy or a precise speedup claim.

## Inference, declarations and expressiveness

All12 accepted-source configurations have identical normalized phase allocation and owner trajectories for inference, checked declaration and required declaration. Only the external root allocation baseline and timing fields are excluded from that equality check. The declaration supplies no additional runtime shortcut beyond the same source-verified property.

The writer is admitted by inference with the ordinary dependency machinery. Checked and required no-binding declarations reject it. Admission-only runs measure the actual rejection, separately from execution: accepted ordinary/inferred preparation requests4,234 total bytes including source construction; rejection requests4,288 bytes, with the extra54 bytes belonging to the returned error string. Every accepted or rejected owner is disposed. Rejection timings are not query-performance measurements.

The writer changes an initially free requested output variable to `a`, which appears in both output names and the residual tag. Under the present engine contract, a source that cannot change bindings cannot produce that same output binding from the unchanged query. Returning a residual fact instead would change the observation contract. Thus a mandatory no-binding restriction has a concrete expressive cost; it is not merely another implementation switch.

This differs from checker conservatism. Reflexive equations and unreachable writers can be binding-free in practice but remain outside the current certificate, as the [source gate](S07-binding-effects-gate.md) demonstrates. Those rejections do not establish an inherent language limitation. Broader effect analysis and regional eligibility remain unresolved.

## Owners and correctness

All833 comparative processes complete:238 allocation runs and595 ordinary timing runs. All119 allocation pairs replay exactly. The independent audit verifies114 lifecycle cells, five admission cells, registered ordering and metadata, complete/cancelled endpoints, phase continuity,266 consumer-independent engine groups and36 output-owner groups within their physical representations. Every query, consumer, prepared owner and rejection object releases to its expected baseline.

All lifecycle processes first compare complete analytical and independent scalar answers with the engine using separate prepared owners. Queries vary depth and input order/tag values. The rewriting source builds constructor structure and consumes a finish token; the writer must reactivate existing requests. Cancellation stops query0 after100 engine calls, then later queries complete. That tests disposal under incomplete work, not equal completed work or a cross-engine cancellation-speed ranking.

**Output capacity explains the cross-engine retained-byte difference.** Across four retained answers, conditional execution owns11,808 extra bytes on blocked/writer sources and11,616 on rewriting. The [layout probe](../../../research/chr-direct-conditional/examples/effect_layout.rs) observes conditional residual capacity128 versus exact capacities65–68 or66–69 in Scan, with48-byte slots. Those spare slots account for the differences exactly. Reported allocation and peaks include the actual storage; no adjustment makes a cost disappear.

The [evidence directory](s07-effect-lifecycle/) includes raw runs, commands, source/binary freezes, the complete summary, layout output and audit. The [sizing receipts](s07-effect-lifecycle-sizing/) validate the new source/lifetime paths before the matrix. A source snapshot matches the exact measured hash; formatting that snapshot reproduces the current source byte-for-byte. The reference interpreter and engine implementation are unchanged by this measurement package. Scoped feature-on/off Clippy and formatting checks pass.

Ordinary timing disables counters and the allocation meter. Requested heap bytes are not RSS. Native compilation, process startup, fixed harness buffers and independent validation are excluded; timings describe warmed within-process lifecycles. First observation is a contained interval, never added to total time twice.

## Four-package breadth review: investigate direct source-derived solving next

The four packages since the order-lifecycle review are stronger-control qualification, its complete lifecycle comparison, the binding-effect certificate, and this admission/lifecycle pilot. The first two preserve the conditional choice/check advantage against existing compilation; the latter two demonstrate a smaller bookkeeping benefit and a concrete language boundary.

**Select T073 direct source-derived solving of private finite choice/check work.** The source, independent oracle, adverse arrival order, failure and consuming finish controls already exist. A genuine solver must derive the relation from the supplied rules and query, preserve complete aliases/residuals/multiplicity, and account for ownership and externally visible effects. It cannot be a generator of the hand-written expected answer or silently reinterpret consuming CHR as set logic. The first gate must distinguish an eligible private region from a linked writer/consumer that makes early elimination unsound.

The strongest ready alternative is finer effect analysis or more precise certificate timing. That could improve eligibility or settle a small optional optimization; it would not by itself remove the explicit alternatives or the dominant conditional support work measured earlier. Direct solving can change which work either architecture needs to perform. Its implementation is less ready, but the potential architectural consequence justifies an independent source/semantic entry now. This is a priority judgment, not a rejection of better effect analysis.

Native graph execution, connected parallelism, generalized reuse and broader language properties remain required in the [cycle](../next-cycle.md). The direct-solving gate must compare its concrete next implementation cost with those alternatives and with effect refinement. T079 remains unfinished for precision, broader beneficiaries and optional adoption; no mandatory restriction is selected. The research goal remains active.

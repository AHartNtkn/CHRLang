# Source simplification beats the reusable solver paths here, but does not beat competent enumeration generally

Constraint-based simplification has a registered timing gain against finite projection and symbolic rebuilding in all 17 admitted scenarios. Against explicit early rejection it has two losses, thirteen unresolved comparisons and two comparisons with insufficient clock signal. The strong explicit control prevents an inference that reusable symbolic machinery is necessary on these sources.

The confirmation completes 4,896 primary processes and 204 warmups on two pinned physical cores. Every process validates its complete answers against independent enumeration after the measured work. No outlier is excluded. This is a bounded retained-output lifecycle result, not a universal architecture choice.

## What the measurements include

Each preparation serves one, sixteen or 128 changing requests where qualified. The endpoint includes preparation, request construction, symbolic transport when applicable, complete observation, producer disposal and final consumer disposal. Answers remain owned until validation outside the two measured intervals. All six paths use the same source and observation contract.

Source/oracle fixtures, process startup, parsing, native user-program compilation and fixed harness capacity are excluded. Consumer timing is retained-all only. First-value latency, streaming, cancellation and immediate/window timing remain outside this confirmation. The preceding allocation comparison covers those finite retention policies separately and must not be mistaken for their runtime comparison.

A gain requires every paired ratio below 0.90 on both CPUs; a loss requires every ratio above 1.10 on both. A compared cell falling below 4,200 ns makes the contrast insufficient-signal regardless of ratios. Otherwise ranges that cross the practical boundary are unresolved. These are predeclared observed-repeat criteria, not confidence intervals or application weights.

## Every registered comparison remains visible

| Candidate / control | Gain | Loss | Unresolved | Insufficient clock signal |
|---|---:|---:|---:|---:|
| Simplifier / diagram | 4 | 0 | 13 | 0 |
| Simplifier / prepared names | 9 | 0 | 8 | 0 |
| Simplifier / finite projection | 17 | 0 | 0 | 0 |
| Simplifier / symbolic rebuilding | 17 | 0 | 0 | 0 |
| Simplifier / explicit enumeration | 0 | 2 | 13 | 2 |
| Diagram / prepared names | 5 | 3 | 9 | 0 |
| Diagram / finite projection | 16 | 0 | 1 | 0 |
| Diagram / symbolic rebuilding | 14 | 0 | 3 | 0 |
| Diagram / explicit enumeration | 0 | 7 | 8 | 2 |

Counts refer to the selected 17 scenarios. They cannot rank application-wide usefulness or select between controls that were not directly contrasted.

## Lower allocation need not mean lower elapsed time

For the three-variable, three-name clique with 128 membership queries, CPU-0 median session times are 18.02 microseconds for simplification and 9.97 microseconds for explicit enumeration. CPU 2 gives 19.94 and 10.42 microseconds. Every paired ratio supports the registered simplifier loss. The overlapping-alternatives membership case also has a separated loss on both CPUs.

These are cheap existential requests. Explicit execution rejects partial conflicts and stops at a witness; it does not enumerate the entire hidden space merely because the independent oracle does. Preparation and validating a reduced relation still cost time. The result does not attribute the difference to one particular phase, because the primary clock deliberately measures a coarse lifecycle.

A useful opposite comparison appears in the six-variable hidden-star source with 128 full-output queries. CPU-0 medians are 95.67 microseconds for simplification, 185.92 for diagrams, 265.56 for prepared names, 381.20 for finite projection and 2,306.34 for symbolic rebuilding. The simplifier has registered gains against those four controls on both CPUs.

Explicit enumeration takes a CPU-0 median 117.75 microseconds in that case, but its comparison with simplification remains unresolved under the registered all-pairs rule. The CPU-0 maximum paired ratio reaches 1.091, so the lower typical simplifier time is not reported as a confirmed gain. This retains an apparent favorable regime as a question about the boundary, rather than turning a median into a verdict.

The allocation evidence adds another dimension: simplification often lowers traffic while explicit enumeration has the smaller peak heap. Timing, traffic and peak memory remain separate consequences. No workload weights combine them into a universal winner.

## What remains uncertain

Two explicit cells become too short in primary execution: the six-variable one-query clique/full-output case and the three-variable sixteen-query overlap/membership case. The predeclared rule marks their paired comparisons insufficient-signal even though their median ratios look large. Seven additional scenarios were already excluded by qualification. No claim about their speed follows.

Large excursions also remain in the data. One projected free/six-variable/sixteen-query session on CPU 0 takes 2,125,364 ns against a cell median of 70,900 ns, about thirty times larger. The largest simplifier excursions exceed five times their cell medians. All frozen inputs and complete endpoint metadata agree, and the independent output assertions pass.

These observations localize uncertainty to repeated executions of the same source, but coarse elapsed clocks cannot distinguish graph/runtime behavior, allocator effects and preemption. No CPU-time or scheduler evidence was collected here. Targeted CPU/elapsed attribution remains the concrete follow-up where it could change a decision; no outlier is relabeled or discarded. The broad separated gains and explicit-control losses already survive those excursions.

## Architectural consequence and handoff

On this eligible finite logical-set corpus, source reasoning can repay its cost relative to general reusable solver paths. Yet cheap explicit execution remains competitive and is conclusively faster in two admitted comparisons. The evidence supports keeping both source elimination and competent explicit execution in architectural comparisons; it does not establish a need for a general symbolic runtime on these sources.

All selected source relations simplify under the qualified rules. Arbitrary hidden cores, useful symbolic union/projection beyond these reductions, ordering, broader structural theories and host-visible effects remain required investigations. Nothing here rejects those mechanisms or selects a mandatory language restriction.

The bounded lifecycle comparison selected by the [breadth review](S06-graph-simplification-breadth-review.md) is now complete. Move active work to T077 search restoration: its existing source and ownership evidence make consequential adaptive timing uncertainty and checkpoint/replay costs the strongest ready distinct investigation. Further precision here cannot remove the observed coexistence of strong solver-path gains and explicit-control losses, although it may clarify individual boundaries. Preserve those targeted follow-ups under T076; the overall research goal remains active.

## Evidence

[Prospective registration](../registrations/S06-formula-timing.md), [pinned driver](../../../research/chr-structural/experiments/formula_timing.py), [independent auditor](../../../research/chr-structural/experiments/formula_timing_audit.py), [exact scenarios](s06-formula-timing/scenarios.json), [balanced schedule](s06-formula-timing/order.json), [source/binary freeze](s06-formula-timing/freeze.json), [all raw runs](s06-formula-timing/runs/), [per-process results](s06-formula-timing/results.jsonl) and [all 153 audited contrasts and largest excursions](s06-formula-timing/review-audit.json). [Clock qualification](S06-formula-clock.md) and [matched ownership](S06-simplified-ownership.md) supply the preceding evidence.

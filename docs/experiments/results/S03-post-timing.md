# Demand can be faster despite more allocation; large misses remain expensive

The primary timing pilot finds favorable demand regimes that allocation measurements alone did not reveal. Large unsuccessful matches remain much slower than the specialized explicit control. Actual generated access and Active scheduling are still missing on these sources, so favorable demand results need those controls before confirmation or architectural selection.

**Next, qualify the existing generated/access mechanisms on the same post sources and carry passing controls into lifecycle comparison.** This addresses a competitor-credibility gap. The research remains active; writable heads, dynamic choices and sustained ownership remain separate unanswered directions.

## Complete timing now supplements ownership evidence

The [registration](../registrations/S03-post-timing.md) covers six post families, sizes eight/32, both arrival orders, three consumer lifetimes and six execution modes. Four changing a/b queries reuse prepared rules. Five randomized paired repetitions give 2,160 primary processes; another 432 excluded warmups and 36 cancellation/reuse checks also pass. All primary and gate endpoints match the frozen ownership study exactly, including service ticks, complete answers and cancelled prefixes.

This uses the existing ordinary-allocator binary, not a new baseline. Its build artifacts show no engine/kernel/work/profile/allocation diagnostics. The current compiled core, demand templates and post sources match the frozen definitions; current runner/demand differences are opt-in profiling hooks absent from this build. The [qualification record](s03-post-timing/qualification.json) documents that applicability. Costs apply to the recorded binary/toolchain, not a fresh build or user-program compilation.

Source construction, preparation, inputs, setup, execution with owned observation, engine disposal, consumer handling and preparation/final-consumer disposal count in totals. First-answer clocks are disabled; execution and owned observation remain a combined phase. Startup, independent validation, harness vectors and process transport are outside. The [audit](s03-post-timing/audit.json) reconstructs all 432 cells, 360 matched blocks and 648 demand/control comparisons.

## Higher requested heap does not imply slower execution

For size32, original arrival, four queries and retained-all answers, the following results use demand with within-turn miss reuse. Memory comes from the matched frozen ownership matrix; it is not newly measured here.

| Source and executor | Median complete time | Requested bytes | Peak heap above baseline |
|---|---:|---:|---:|
| Output post — Demand | 0.970 ms | 1,614,631 | 93,381 |
| Output post — Scan | 1.297 ms | 1,259,424 | 50,638 |
| Output post — Indexed | 1.501 ms | 1,239,360 | 96,306 |
| Output post — Specialized | 1.491 ms | 1,176,128 | 94,322 |
| Unsuccessful match — Demand | 6.107 ms | 12,259,743 | 149,602 |
| Unsuccessful match — Specialized | 1.103 ms | 998,792 | 144,362 |

**The output-post advantage appears against all three explicit controls in every sampled pair.** Demand/control ratios range 0.648–0.844 against Scan, 0.605–0.727 against Indexed and 0.565–0.753 against specialization. Yet demand requests more bytes than every control and peaks above Scan. This is a time–memory tradeoff, not evidence that heap traffic predicts speed.

**The unsuccessful-match loss is large and survives arrival reversal.** Original-arrival demand/specialized ratios have median6.014 and range5.392–7.772. Reversed arrival has median5.389 and range4.707–6.853. Its allocation loss and runtime loss point in the same direction for this implementation, but do not establish an intrinsic limitation of demand or of a different discovery algorithm.

All ratios are paired observations; the ratio of separate medians need not equal the median paired ratio. Five samples are exploratory sizing, not confirmation. No universal winner follows from these examples.

## Arrival and reuse policy matter

**Arrival order changes demand's costs substantially on these sources.** For size32 output posts with retained-all answers, demand's median falls from0.970 ms in original order to0.476 ms in reverse order. Specialized execution stays near1.49–1.57 ms. On duplicate resources, demand changes from2.813 ms to0.648 ms, while specialization stays near2.28–2.31 ms. These paired source orders supply both favorable and adverse regimes; they are not workload frequencies.

**Adding another reuse mechanism does not automatically improve the result.** On original-order size32 repeated-template input, miss-plus-template demand is slower than Scan in every sampled pair (ratios1.504–1.697), while its Indexed comparison straddles equality. A demonstrated template hit does not establish enough saved work to repay construction, lookup, replay and observation.

Across the72 complete source/lifetime scenarios, median ratios fall below0.90 against every existing explicit control in52 scenarios for birth reuse,54 for birth-plus-miss and50 for birth-plus-miss-plus-template. Requiring every observed pair below0.90 lowers those counts to45,40 and37. These counts are descriptive and unweighted; they do not provide significance or justify selecting a different policy per scenario without paying for that selection.

Five pinned empty-clock calibrations have median costs12,12,12,12,13ns. All432 cells clear the registered31,200-ns total threshold. Individual ratios still contain large excursions; all samples and phases are retained. Passing the clock threshold does not eliminate noise or establish statistical confidence.

## The miss defect still deserves causal investigation

**Combined execution/observation dominates the large miss case.** For size32 original-arrival demand with miss reuse, that phase has a5.789-ms median. Setup is about0.178 ms and all other complete phases are smaller. The earlier [copy attribution](S03-candidate-copy.md) identifies9.931 MB of copied arguments/environments/occurrences within12.260 MB total requests, but those bytes do not attribute elapsed time.

An optimistic bound setting the whole combined execution/observation phase to zero leaves a0.312-ms median remainder, below specialization's1.103-ms complete median. This is deliberately stronger than any realistic copy repair and charges no replacement work. It shows why the previous allocation bound cannot rule out a runtime reversal. It does not show that copies consume enough time, that a repair reaches the bound, or that the rest of the executor becomes free.

A consequential follow-up must either attribute actual time to copying/matching or implement sound delayed/borrowed match state and measure its complete replacement cost. A different discovery algorithm that avoids candidate visits is a separate alternative. Preserve partial bindings, aliases, source competition and claims; do not infer a gain from fewer allocations alone.

## Strengthen the explicit competitor before confirming demand gains

**The existing “sealed” mode is inferred specialization, not general native generation.** Its source checker admits a leading removed head with no kept heads and at most one distinct nullary partner. The post source's `take` rule has a non-nullary `token(Key,Value)` partner, so that rule is outside this specialization. Actual generated access can execute its general shape; its costs have not yet been measured here.

The [structural-prefix study](S01-generated-prefix.md) establishes that generated access can eliminate frame/pool/template bookkeeping, and activation can reduce discovery, while also exposing a policy-dependent answer counterexample. It does not predict the post result. These post sources need their own complete-answer and work gate under actual generated callbacks/access and Global/Active, with both arrival orders, changed bindings, misses and duplicate resources. A policy result that changes answers must remain separate.

Use the same post source definition and existing general emitters, independent scalar/analytical answers, preparation reuse, cancellation and retained outputs. If controls qualify, extend the existing lifecycle runner and register their costs in the bounded follow-through. Do not introduce another production baseline or treat an ineligible specialization mode as a substitute. The measured Scan/Indexed/specialized results remain valid in their scope.

## Next selection and remaining scope

**The stronger-control gate takes precedence over more timing precision now.** It could eliminate a currently favorable demand comparison by changing execution of the same source, at modest implementation cost using existing emitters and checks. Compact runtime confirmation is the strongest ready distinct alternative, but narrowing uncertainty around a possibly weaker post control would be less useful than testing the missing control first.

At the control/cost result or obstruction, compare further demand confirmation or miss repair with compact confirmation, integration and call-reuse costs. This is package one after the full prefix portfolio review. Broad discovery, writable resource heads, dynamic choices, sustained consumers and complete-path composition remain required. Passive body-post support is not evidence for those capabilities, and this pilot does not choose language restrictions.

## Validation

All primary, warmup and cancellation processes complete. Full scalar answers, analytical post expectations, retained answers and cancelled prefixes validate inside the unchanged runner. The independent audit verifies frozen source/binary/build features, exact source parameters, randomized jobs, phase boundaries, parent endpoint agreement and matched historical allocation records. The earlier ownership and copy-attribution audits pass again. Driver/auditor syntax and durable question/task links are checked. No runtime engine or independent reference implementation is changed in this package.

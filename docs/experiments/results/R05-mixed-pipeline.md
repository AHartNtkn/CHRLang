# Mixed phases: work placement changes the execution choice

All 224 registered processes complete, all 28 cells have their planned repetitions, and 8,960 full raw answers validate. All 28 allocation processes return to their baseline after measured disposal. Every diagnostic application count matches the prospective prediction. Source/binary freezes pass. The result supports a placement-dependent applicability boundary between these current engines, not a weighted winner or automatic hybrid execution.

[Registration](../registrations/R05-mixed-pipeline.md), [semantic gate](R05-mixed-pipeline-gate.md), [raw outcomes](r05-mixed-pipeline/runs.jsonl), [manifest](r05-mixed-pipeline/metadata.json), [audit](r05-mixed-pipeline/audit.json), [all cell summaries](r05-mixed-pipeline/summary.tsv), [work predictions](r05-mixed-pipeline/work-predictions.json).

## Lifecycle and physical work

Cold-query medians include preparation, setup, execution through all 16 complete observations, validation-warmed engine/output disposal and prepared disposal. Primary timing disables engine/kernel counters and uses the ordinary allocator. The two engines execute identical registered source. Work counts below come from separate diagnostic processes.

| Pre depth | Post depth | Conditional ms | Specialized explicit ms | Conditional applications | Explicit applications |
|---:|---:|---:|---:|---:|---:|
|0|0|1.223|0.570|34|49|
|0|16|3.566|2.251|290|305|
|8|8|2.415|1.719|170|305|
|16|0|1.361|1.288|50|305|
|0|64|10.835|10.117|1058|1073|
|32|32|6.383|7.839|578|1073|
|64|0|1.786|6.389|98|1073|

The 16/0 cell has overlapping observed timing ranges and no resolved ranking, both cold and reuse. All other cold pairs have separated ranges. The four-query prepared-reuse results show the same qualitative ordering; actual inputs alternate between the base depths and both depths incremented by one. The 64/0 conditional reuse outlier is retained in the summary. First-service summaries refer to query zero, not an average of all reused queries.

The mechanism is specific. Conditional execution shares the pre-countdown, but the sixteen gate applications create sixteen post occurrences. Although their `post(depth,U)` arguments are identical, later work executes per occurrence: measured conditional applications are `pre+16*post+34`. Specialized explicit executes `1+16*(pre+post+3)`, with the initial start prefix counted once. No valid histories or aliases are combined by output deduplication. Identical call syntax alone does not establish common occurrence ownership.

The large pre-only case requests about 1.508 MB per cold conditional query versus 5.301 MB explicit. In the large post-only case, conditional requests 9.201 MB versus 6.059 MB. Conditional peak requested live heap is lower in every registered cell, but its allocation traffic and lifecycle can be higher. For example, at 0/0 its peak is 95,075 bytes above baseline versus 224,132, despite slower completion and greater allocation traffic. Peak, traffic and elapsed time answer different questions; none supplies an overall scalar preference.

Moving work also changes head arity, term lifetime, occurrence preparation and source-rule positions. Application counts explain the sharing boundary, not every nanosecond of the lifecycle difference. The finite constructed countdowns could be eliminated by a stronger correspondence-checked lowering; this comparison does not establish that such execution is necessary or that either candidate beats all valid compilation strategies.

## Bounded architecture disposition

Substantial opaque common work can earn conditional bookkeeping even when followed by discrimination. Substantial work after discrimination does not automatically regain sharing in this organization, and the measured lifecycle may favor specialized explicit. Small/pre-only work can remain inconclusive despite a large application-count reduction. These conclusions extend the homogeneous-regime evidence with a real source transition and a current same-session control.

This supplies the selected applicability distinction. Another placement sweep, automatic reunion or an inferred execution-switching policy is not selected. Complete residual aliases, exact tuple coverage and exhaustion remain the observation contract. Query setup/preparation reuse and per-query output disposal are charged; native compilation, RSS and unrestricted language applicability are not established. Other workload distributions are not inferred from these controlled cases.

## Next selection

T043 selects a bounded counter-free coarse-region parallel readiness gate, followed by a newly registered repeated pilot if the boundary remains credible. The decision is whether two retained, certified independent regions provide a complete cold-lifecycle benefit after worker creation, transport, owner product observation and shutdown. E16 contains both a favorable exploratory granularity signal and contrary overhead/imbalance/publication controls, but lacks a repeated counter-free conclusion.

The entry must distinguish operational ownership/admission state from optional diagnostics, preserve source work and answers with diagnostics disabled, review serial executor competence and define the certified-region applicability boundary. Then select matched-quantum serial/one-worker/two-worker controls with balanced capacity, one-region overhead, imbalance, owner-heavy publication and cancellation evidence. A parallel result cannot be generalized to connected source regions or presented as superiority over a stronger serial lowering. No existing matrix is automatically queued.

Actual static lowering/compilation is the strongest broader alternative. R04 already establishes favorable finite lowering in its exact fragment; collapsing this particular pipeline would add a favorable simplification without resolving general eligibility. A meaningful new compilation experiment needs a reusable input family, charged transformation/code generation/toolchain/loading, correspondence and amortization. That remains valuable, but requires more new semantic/implementation scope than auditing the existing regional worker interface. If the parallel entry exposes substantial protocol redesign or no meaningful deployment boundary, reconsider compilation eligibility rather than automatically expanding worker machinery.

## Validation

Three independent semantic tests and the reusable-runner lifecycle test pass in default and counter-free experiment builds. Strict Clippy passes both modes and the allocation runner; formatting and the isolated meter self-check pass. Root and independent review checked full-answer validation, shared preparation, retired-prefix work aggregation, measurement ownership, partial-outcome handling and the interpretation limits. Runtime implementation and reference execution remain independent of these fixtures.

# Fusion's execution savings must pay for inference and certification

Resource fusion lowers execution allocation in the inspected cases, but inference and per-query certification can outweigh those savings. All 1350 registered processes complete with independent answers and restored owners. Five-sample timing remains exploratory; the next investigation moves to coherent execution-path composition.

## Complete lifecycle evidence

The [registered pilot](../registrations/S06-resource-fusion-lifecycle.md) compares original/fused sources under generic Scan, specialized Scan and generic Indexed. Five families cover deterministic work, distinct choices, duplicate choices, shared payload aliases and spare resources. Producer firings 0/1/4 and preparation reuse 1/4 give 180 cells.

All 90 allocation preflights pass. The 360 allocation processes produce 180 exactly matching phase-reading pairs; this ownership gate passes before the 900 ordinary-allocator timing processes begin. Each measured query is validated against complete independent scalar answers outside its phases. All query, cancellation, compiled-preparation and inference owners return to their registered requested-live baselines.

Inference and first-query certification occur before emitted-rule preparation. Each later changed query receives its own certificate and reuses that preparation. Input, setup, execution, owned observation and each disposal owner are measured separately. The first observation of each completed branch happens after all completed branch states have been collected; peak memory therefore describes an explicit finite batch consumer, not streaming delivery. Cancellation is separate from full-query totals.

## Allocation findings

The table gives total requested bytes for generic Scan, including inference, certification, preparation and complete query disposal. The two sides execute identical source queries.

| Family | Firings per query | Queries per preparation | Original | Fused |
|---|---:|---:|---:|---:|
| Plain | 0 | 1 | 11794 | 15420 |
| Plain | 1 | 1 | 17857 | 22247 |
| Plain | 4 | 4 | 117423 | 118948 |
| Distinct choices | 4 | 4 | 912647 | 881271 |
| Duplicate choices | 4 | 4 | 894015 | 862639 |
| Shared payload choices | 4 | 4 | 302383 | 296287 |
| Spare resources | 4 | 4 | 132527 | 134324 |

The deterministic four-query case exposes the accounting tradeoff directly. Execution allocation falls from 53524 to 45556 bytes. Fused inference costs 5157 bytes and four certifications cost 5780 bytes, partly offset by smaller emitted preparation. Total requested traffic is slightly higher after fusion. Zero- and single-firing cases expose larger preparation penalties relative to useful work.

Choice-heavy queries amortize more of that work. For distinct choices at four firings and four queries, execution allocation falls from 748912 to 706784 bytes; total traffic falls by 31376 bytes after inference/certification. Peak requested growth is slightly higher, 137869 versus 137108 bytes. Fewer allocations do not imply less retained state.

Access and specialization remain separate dimensions. At the displayed distinct-choice size, Indexed requests 1056871 bytes originally and 995255 after fusion; both exceed the generic Scan totals. Specialized Scan requests 915007 and 883083. All 180 cells and phase breakdowns are in the [allocation audit](s06-resource-fusion-lifecycle/allocation-audit.json); no workload weights select among them.

## Timing is still uncertain

Five ordinary samples per cell show possible gains and adverse preparation regimes, with substantial variation. For generic Scan and four queries of four firings, plain-source medians are 168µs original and 158µs fused, with ranges 163–235µs and 95–174µs. Distinct-choice medians are 778µs and 722µs, with ranges 604–990µs and 455–1053µs. Shared-payload medians go the other way, 235µs versus 286µs, but their ranges 219–418µs and 233–437µs overlap.

The cold zero-firing single-query case has medians 13.3µs original and 19.6µs fused, with ranges 10.3–14.5µs and 19.0–20.9µs. That is a pilot observation consistent with the measured preparation penalty, not a population or architecture claim. No samples are excluded and no confirmatory speed classification is made.

The CPU diagnostic has measurable clock-window overhead. A [registered 5000-sample empty-closure probe](../registrations/S06-resource-clock-probe.md) yields median wall 29ns and CPU 515ns using the same call order. CPU intervals enclose the wall-clock calls. Consequently, zero runs flagged by the wall-greater-than-CPU threshold do not prove an undistorted clock or explain away variability. The [probe](s06-resource-fusion-lifecycle/clock-audit.json) is diagnostic evidence; its values are not subtracted from workload measurements.

These overlapping timing regimes remain unresolved where they could affect use of fusion. Even a stronger favorable timing result within this certificate would not choose a complete execution architecture: both query eligibility and ordinary behavior outside the certificate still need ownership and composition. Investigating that interaction has higher immediate decision value than widening this component matrix.

## Reproducibility and limits

The [timing audit](s06-resource-fusion-lifecycle/timing-audit.json), [allocation audit](s06-resource-fusion-lifecycle/allocation-audit.json), [source/binary freeze](s06-resource-fusion-lifecycle/freeze.json), prospective orders and individual receipts retain the evidence. Both release binaries disable engine and kernel work metrics. Allocation uses a separate diagnostic allocator; primary timing uses the ordinary allocator. Scoped Clippy passes for both builds and all six resource-fusion source tests pass counter-free.

Requested heap traffic is not RSS. Process startup, source/expected-answer generation and compilation are outside the measured lifecycle. Compiler inference here is source analysis, not credibly isolated native compilation cost. The batch observation policy does not establish streaming or sustained lifetime behavior. Reference-interpreter code is unchanged.

The [S10 entry decision](S10-composition-entry.md) selects coherent execution paths and mixed-mechanism source qualification next. T073 retains broader resource derivations, eligibility, timing and lifecycle work. The architecture goal remains active.

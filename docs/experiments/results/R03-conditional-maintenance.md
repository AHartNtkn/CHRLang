# R03 conditional maintenance diagnosis

Repeated deferral of consumed tuples caused the predicted quadratic maintenance growth. Intersecting dirty work with current occurrence liveness restores linear candidate selection in the countdown witness and materially changes the lifecycle comparison. Ordinary computation still favors explicit execution in these cells; opaque sharing has a larger conditional advantage. Immediate discrimination at size64 is now inconclusive rather than a large conditional penalty.

## Causal evidence and correction

The source audit predicted n(n+1)/2+3n+3 selections with step before base. Metrics measured exactly 187 at n16 and 2275 at n64. Reversing mutually exclusive rule priority preserved the complete answer and gave 53 and 197 selections. Discovered tuples remained 34 and 130. After the correction, both orders select exactly 34 and 130, with zero countdown deferrals. The [before](r03-conditional-maintenance/diagnostic-before.log) and [after](r03-conditional-maintenance/diagnostic-after.log) receipts retain the failing linear-work assertion and passing result; the instrumented pre-correction source is preserved as evidence.

A consumed lower-priority tuple had reached scope deferral before resource authorization could reject its liveness. The runtime now resumably intersects the dirty support with every selected occurrence's live support before current-round clipping and deferral, including after body service. Stable IDs and monotone shrinking liveness prove that an excluded region cannot become eligible later. This is a necessary-condition filter, not a positive eligibility certificate; matching, guards and source authorization remain required. Partial-support tests require both complete answers and genuine bounded deferral, so rejecting only globally dead tuples is insufficient.

Metrics increments are disabled in primary builds. Full runtime resource, source-order, multiple-frame, raw-multiplicity and finite-sibling checks pass in both feature modes. No language contract changed.

## Paired lifecycle evidence

The [prospective follow-up](../registrations/R03-conditional-maintenance.md) ran the corrected candidate, verified frozen baseline binaries and the generic global indexed explicit control in the same shuffled session. All 384 processes completed: 960 queries and 21360 raw answers validated. Primary timing comprises 240 ordinary-allocator/counter-free processes; warmups and allocation/work diagnostics are separate. Exact [metadata](r03-conditional-maintenance/metadata.json), [raw results](r03-conditional-maintenance/runs.jsonl), [summaries](r03-conditional-maintenance/summary.json), build logs and source checks accompany the result. Frozen baseline binary hashes were verified before and after execution.

Selected cold-query medians, in milliseconds per query including preparation, setup, complete execution/observation, engine disposal and prepared disposal:

| Family | Size | Corrected conditional | Explicit | Conditional / explicit |
|---|---:|---:|---:|---:|
| No OR |16|0.155|0.080|1.94|
| No OR |64|0.453|0.380|1.19|
| Opaque sharing |16|0.377|0.999|0.38|
| Opaque sharing |64|0.720|6.323|0.11|
| Immediate discrimination |16|3.702|2.568|1.44|
| Immediate discrimination |64|11.338|12.041|0.94|
| Recursive output |4|0.219|0.240|0.91|
| Recursive output |6|0.639|0.798|0.80|

At no-OR64, corrected lifecycle is0.273 of the paired baseline cold cost and0.231 in four-query reuse. Requested allocation falls from3.284MB to0.569MB cold, while source applications stay 65. The correction removes substantial avoidable work; it does not remove all ordinary overhead. Corrected no-OR ratios across sizes/reuse are1.09–1.94, with separated observed ranges.

Opaque sharing is2.65–10.23 times faster than explicit execution across the registered cells. At64 it performs 69 versus 1055 applications and requests0.709MB versus4.323MB cold. Discrimination64 requests9.25MB instead of baseline53.93MB and runs at0.321 of baseline time cold. Its corrected/explicit timing ranges overlap: 11.108–12.462ms versus11.692–12.365ms. The four-query cell also overlaps. No preference is resolved there. At discrimination16, explicit remains faster with separated ranges.

Output4 ranges overlap. Output6 favors conditional versus explicit in this session, but baseline/corrected conditional ranges overlap, so this run does not establish a material output-family speedup from liveness filtering. Every sample, including the explicit output6 reuse outlier, remains in the evidence. Additional repetitions to resolve the small discrimination/output rankings are lower priority than the selected eligibility question; those narrow rankings do not currently change the architectural disposition.

Answer disposal after independent validation remains separately recorded and does not reverse the separated comparisons. Its cache-warmed boundary, combined execution/observation timing, excluded source-syntax generation, requested-heap rather than RSS accounting, generic rather than generated control, and unisolated native compilation cost remain explicit limits. No weighted or universal winner is inferred.

## Disposition and next selected question

T036 is complete. The conditional design remains useful where substantial opaque work can be shared; necessary support/resource/publication machinery and its costs remain real. Maintenance defects must not be treated as architectural lower bounds, and application-count savings must not be treated as lifecycle savings.

Select T037, an R04/R05 region-eligibility and correspondence gate. Native finite-region lowering already has strong bounded cost evidence, but its applicability to nonground relations and surrounding CHR is unresolved. The new question is whether inferred or checked declared regions can safely eliminate arbitration and resource machinery, and what restrictions or reformulations are required. This can change the architecture more than another conditional tuning pass or larger solver sweep.

The [selection note](../../goals/chr-experiments/notes/T037-region-eligibility.md) gives accepted/adverse witnesses and a shared-variable counterexample: uniquely defined predicate expansion can change which earlier consumer fires. Predicate uniqueness alone is not an eligibility proof. Start with correspondence and explicit accepted/excluded programs; any later cost comparison needs its own registration. Language adoption remains an owner decision, and the broader goal remains active.

All 46 package tests pass default/off; the additional matched runner gate passes counter-free. Workspace tests, strict Clippy, allocation-runner Clippy and formatting pass. Root audited liveness scope ownership; independent source diagnosis and next-selection review accompany the implementation evidence.

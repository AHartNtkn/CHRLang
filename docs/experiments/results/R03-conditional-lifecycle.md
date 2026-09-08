# R03 conditional lifecycle pilot

Direct conditional execution earns its bookkeeping cost on the registered opaque shared-work cases, but loses substantially on ordinary computation and immediate discrimination. The output-growth case has a small advantage at depth six and overlapping timing ranges at depth four. These are bounded comparisons against generic global indexed explicit search, not a universal architecture ranking.

All 256 registered workload processes completed: 640 queries and 14,240 raw answers passed independent full-observation checks. This includes warmups and separate diagnostic runs. Primary timing uses 160 counter-free, ordinary-allocator processes; allocation and work diagnostics each use 32 separate processes. The [prospective registration](../registrations/R03-conditional-lifecycle.md), [raw process records](r03-conditional-lifecycle/runs.jsonl), [source/build/host metadata](r03-conditional-lifecycle/metadata.json), [summary](r03-conditional-lifecycle/summary.tsv) and validation logs preserve the evidence.

## Measured contrast

The table reports median backend lifecycle milliseconds per query for cold single-query cells. Lifecycle charges shared rule preparation, query setup, complete execution/observation, engine disposal and final prepared disposal. Answers remain retained until backend disposal; their later disposal after independent validation is recorded separately. Four-query batches alternate base and base+1, and give the same direction in every family.

| Workload | Size | Conditional ms | Explicit ms | Conditional / explicit |
|---|---:|---:|---:|---:|
| No OR | 16 | 0.277 | 0.093 | 2.98 |
| No OR | 64 | 1.751 | 0.396 | 4.42 |
| Opaque shared work | 16 | 0.452 | 1.012 | 0.45 |
| Opaque shared work | 64 | 2.004 | 6.058 | 0.33 |
| Immediate discrimination | 16 | 6.318 | 2.640 | 2.39 |
| Immediate discrimination | 64 | 35.040 | 11.962 | 2.93 |
| Recursive output | 4 | 0.238 | 0.222 | 1.07 |
| Recursive output | 6 | 0.704 | 0.774 | 0.91 |

Across cold and reuse cells, conditional opaque work is 2.22–3.32 times faster. Its no-OR cost is 2.98–4.81 times higher, and discrimination cost is 2.38–2.93 times higher. Depth-four output ranges overlap: the pilot does not resolve a timing preference there. Depth-six conditional ranges are below explicit ranges, but the modest 9–11% median advantage is narrower than the opaque-work result and is not broad evidence for output-heavy programs. Answer-disposal sensitivity does not reverse any separated comparison; its validation-warmed boundary remains a limitation.

Both prepared owners now retain immutable lowered rules across query starts. The conditional owner shares source, head/guard plans and variable maps through one Arc, while occurrences, history, body state and versions remain query-local. An ownership test checks the actual shared allocation, independent query behavior and query survival after the external prepared handle is dropped. The measured reuse contrast therefore uses real preparation reuse.

## Work and allocation explain different parts

At shared size 64, conditional execution performs 69 physical applications versus 1,055 explicit applications, with four physical birth operations versus 15 explicit forks. Requested allocation is 3.43 MB versus 4.32 MB. Shared work is sufficient to pay for this candidate's overhead in that regime.

At discrimination size 64, physical applications are close: 1,060 versus 1,071. Conditional requested allocation is 53.93 MB versus 7.07 MB, while peak requested live heap is approximately 567 KB versus 501 KB. At no-OR size 64, both perform 65 applications, but requested allocation is 3.28 MB versus 0.267 MB; requested live peaks are approximately 52 KB versus 30 KB. Allocation traffic and transient bookkeeping, not only retained state size, therefore require investigation before attributing the adverse result to unavoidable architectural complexity.

For depth-six output, seven conditional applications replace 127 explicit applications, yet total cost improves only modestly. Requested allocation is 0.398 MB versus 1.062 MB. Enumeration and full output recovery remain real work even when source execution is shared. This pilot does not isolate their time from conditional source service.

Candidate counts have different definitions: conditional records discovered tuples, explicit records examined candidates. They are retained as mechanism diagnostics, not directly comparable units. All allocation figures describe requested heap, not RSS; oracle execution occurs after backend intervals and prevents treating whole-process high-water memory as backend RSS.

## Interpretation and next investigation

The result supports conditional execution as a plausible regime-dependent organization. It does not support using this implementation unconditionally for ordinary CHR or selecting it because its source application count is lower. No application distribution or workload weights are assumed. Required machinery still includes supported equality/resources, birth histories, dependency activation, completion certificates and joint export.

The next selected investigation is the adverse no-OR/discrimination bookkeeping growth. In the current round protocol, lower-priority candidates can be deferred after a higher-priority application consumes their support. A read-only source audit derives n(n+1)/2+3n+3 candidate selections for the no-OR countdown: 187 at n16 and 2,275 at n64, despite only 2(n+1) discovered tuples. These are analytical predictions, not measured counters. Intersecting dirty scope with every selected occurrence's live support before deferral is sound because occurrence identity is stable and liveness only shrinks; supported partial liveness must remain intact. T036 will measure that recurrence and the correction, with reversed mutually exclusive rule priority as a discriminant. This is more valuable than another broad workload matrix now: it distinguishes removable maintenance overhead from necessary conditional cost and can change the current ordinary-computation conclusion. Any revised runtime needs its independent source/progress gate and a prospectively frozen comparison; the present measurements remain evidence for this exact source revision.

T035 is complete: full runtime semantics, actual reusable preparation, prospective registration and a validated pilot are established. T036 owns the bounded maintenance diagnosis. The overall goal remains active. Generated dispatch, native compilation cost, wider application generalization, query-internal reclamation, parallel ownership and remaining language tradeoffs still require the decision-map audit.

Workspace tests and strict Clippy pass. The 44 package tests plus the independent matched runner gate pass with counters disabled, and allocation-runner Clippy and formatting pass. Independent reviews covered preparation ownership and runner measurement boundaries; primary and diagnostic source hashes were unchanged during execution. No failing or timed-out cell was excluded.

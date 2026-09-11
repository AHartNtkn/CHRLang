# Probe lifecycle costs: allocation tradeoffs, no established timing gain

**Saved candidate work has not yet established a complete-lifecycle speedup over the strengthened indexed control.** The probe reduces requested allocation on selective sources, adds it on duplicate and broad sources, and increases retained preparation and peak ownership. Closely interleaved timing and a separate thread-CPU diagnostic do not meet the registered gain criterion.

## What the cost includes

Each preparation serves two changing queries. The endpoint includes constructing/preparing the source, constructing and setting up both queries, execution, complete observation, producer disposal, prepared disposal and immediate or retained-answer disposal. Cancellation stops the first query early and checks a complete second query using the same preparation. Complete answers are independently validated outside the measured intervals.

The pilot covers four families, widths 16/128, Global/Active, immediate/retained answers and normal/cancel-first sessions. Probe/indexed, ordinary indexed and ordinary Scan give 192 cells. Five ordinary-allocator timing runs and two separate meter runs give 1,344 processes. Every allocation pair matches exactly and final requested live ownership returns to its root. This measures requested allocations, not RSS. Compilation and process startup are outside the endpoint.

## Allocation results are clear

Compared with the existing indexed control:

| Family | Requested allocation over 16 scenarios | Peak ownership change |
|---|---|---:|
| Selective | Lower in all 16 | +256 bytes |
| Neutral | Lower in 8, higher in 8 | +232 bytes |
| Duplicate projection | Higher in all 16 | +256 bytes |
| Broad keyed bucket | Higher in all 16 | +256 bytes |

At width 128 with Global, immediate release and normal completion, selective requests fall from1,022,268 to989,684 bytes. Duplicate requests rise from746,826 to785,698 bytes. Broad requests rise from945,022 to945,686 bytes, entirely accounted for by the added preparation requests in that case.

For the four-head source, preparation retains 2,564 bytes instead of 2,308 and requests 5,256 instead of 4,592. The added links are a small fixed owner; temporary probing has a source-dependent traffic cost. Exact retained-answer and cancellation checks show these owners are released, rather than merely disappearing from a final peak estimate.

## Timing needed further investigation

The pilot establishes no qualified speedup over indexed execution. Its two qualified slowdowns are scoped observations, not a general rejection. Against Scan, 15 scenarios favor the indexed probe and 31 favor Scan, with 18 unresolved; these comparisons combine access and probe differences and cannot attribute an advantage to probing alone.

A separately registered confirmation runs 50 complete lifecycles per process, with 20 alternating paired blocks over nine endpoints: all four families and both policies at width 128, plus selective/Global at width 16. This gives 18,000 complete sessions. Another 36 meter processes reproduce 18 exact historical allocation pairs. All nine timing comparisons remain unresolved.

| Confirmation endpoint | Probe/control median wall-time ratio | Pair range |
|---|---:|---:|
| Selective, Global,128 | 0.939 | 0.900–1.426 |
| Selective, Active,128 | 0.956 | 0.763–1.453 |
| Duplicate, Global,128 | 1.096 | 0.656–1.535 |
| Broad, Global,128 | 1.007 | 0.684–1.344 |

A further 18,000-session diagnostic records thread CPU and wall time separately. Only 4 of 360 batches have wall/CPU above 1.2; median wall/CPU is 1.005, maximum 1.749. CPU comparisons remain variable and none meet the same 10%/all-pairs-same-side criterion. This diagnostic does not establish descheduling as the main cause of the uncertainty. It is separate evidence, not a replacement for the primary wall series. No threshold or policy was tuned to obtain a winner.

## Architectural consequence

The probe's candidate reduction is real, but much of the complete cost remains elsewhere. In the selective width 128 Global control, the pilot phase medians are approximately 294 microseconds for query construction/setup and 399 microseconds for execution, against a median complete total of 828 microseconds. Phase medians need not sum to the median total. Execution also includes initial occurrence/index work, so its cost is not synonymous with match discovery.

The next investigation should use the **existing prepared-query API** to test repeated stable table structure against rebuilding it. This is a different way of paying admission costs, not a new production baseline. The source must preserve the same arrival order and complete chosen tuples in both paths. Selective and neutral tables are stable across the two current queries; broad/duplicate right tables change, so a reusable prefix must not silently retain those changed facts. That distinction supplies an adverse control.

More timing repetitions of the current endpoint could resolve a small effect, but first changing a demonstrated large responsibility has greater architectural value relative to its cost. Allocation and timing evidence remain available for any later policy boundary. No general probe adoption, universal winner, language restriction or completed research goal follows.

## Evidence and next review

The [full portfolio review](S01-probe-lifecycle-review.md) selects prepared-query source/ownership qualification under T082 and resets the four-package count. Other discovery, integration, solving, reuse, parallel and language questions remain required.

All three frozen archives, job lists and raw results are audited. Scoped Clippy passes for the ordinary, allocation and dual-clock configurations. Retained answers remain readable after producer/preparation disposal. The reference interpreter is unchanged.

Registrations: [pilot](../registrations/S01-probe-lifecycle.md), [confirmation](../registrations/S01-probe-lifecycle-confirmation.md), [clock diagnosis](../registrations/S01-probe-clock-diagnosis.md). Analyses: [pilot](s01-probe-lifecycle/analysis.json), [confirmation](s01-probe-lifecycle-confirmation/analysis.json), [clock diagnosis](s01-probe-clock-diagnosis/analysis.json). Auditors: `audit_probe_lifecycle.py`, `audit_probe_lifecycle_confirmation.py`, `audit_probe_clock.py` under `research/chr-compiled/experiments/`.

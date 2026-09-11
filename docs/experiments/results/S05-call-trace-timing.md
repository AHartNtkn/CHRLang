# Call reuse has scoped speed gains and a retained-memory cost

**Trace reuse qualifies faster than Direct in 11 of 144 scenarios, all with four
repeated queries. It qualifies slower in 22; 111 remain unresolved.** The qualified
gains carry approximately twice Direct's peak ownership. Trace reuse is much more
competitive against the two transition-table controls.

The [registered timing](../registrations/S05-call-trace-timing.md) uses the exact
ordinary binary from the ownership gate: 576 configurations in ten shuffled
blocks, **5,760 processes**. No other builds ran alongside timing. The ownership
evidence was audited first, including exact allocation repetitions and consumer
conservation. Every timing process checks complete independent preflight answers
and its measured ordered output or prefix. No process failed or was excluded.

## Complete phase-sum results

Each row compares trace/control across 144 scenarios. Ratios below one favor trace.

| Control | Qualified gains | Qualified losses | Unresolved | Median ratio range |
|---|---:|---:|---:|---:|
| Direct | 11 | 22 | 111 | 0.599–1.826 |
| Stride-one transition reuse | 142 | 0 | 2 | 0.013–0.508 |
| Stride-sixteen transition reuse | 82 | 0 | 62 | 0.180–1.017 |

All 432 comparisons clear the 2,400 ns clock floor. Qualification additionally
requires all ten paired ratios on the same side of one and at least a 10% median
difference. Many comparisons remain unresolved because they fail those criteria;
clock qualification alone is not sufficient. Their contrary observations remain
in the data and are not attributed to an unverified machine cause.

Against Direct, query reuse separates the outcomes:

| Query policy, 48 scenarios each | Gains / losses / unresolved | Median ratio range |
|---|---:|---:|
| One query | 0 / 11 / 37 | 1.014–1.826 |
| Four repeated-shape queries | 11 / 0 / 37 | 0.599–1.309 |
| Four distinct-depth queries | 0 / 11 / 37 | 1.012–1.742 |

The eleven gains occur at depths 32 or 128, across all four source modes. Their
trace/Direct peak-byte ratios range from 1.957 to 2.136. This is a measured
speed–memory tradeoff, not an unqualified improvement. The complete phase sum
charges source construction, preparation, inputs, setup, observation, consumption,
query disposal, prepared disposal and consumer disposal. Compilation and process
startup are not measured by this engine-lifecycle comparison.

## Consequential diagnosis

Setting all trace disposal times to zero while keeping Direct fully charged leaves
47 of 48 single-query median ratios above one, and 47 of 48 distinct-query ratios
above one. This analytical bound does not model other effects of eviction and is
not an achieved optimization. It shows that disposal time alone does not explain
most adverse medians.

The previous ownership witness distinguishes completed-key retention from
unfinished machines. Bounded retention remains worth testing against future
revisits, but it should not be presented as a guaranteed time repair. Admission,
key construction, duplicated caller/callee state and import/export remain charged
work in the current implementation.

## Decision

Keep the compressed-call mechanism as a candidate for repeated substantive calls.
Its gains over transition tables do not select it over stronger recomputation.
The [portfolio review](S05-call-trace-timing-review.md) therefore selects a matched
compiled-execution control for these same sources before more reuse tuning.
The existing `PreparedRuleset` interface can prepare arbitrary source rules and
provides indexed execution; source order and cancellation still need a direct gate.
No advantage for that control is assumed.

The audit verifies all 5,760 terminal sessions, 432 comparisons, phase sums and
frozen timing inputs. [Complete results and disposal bound](s05-call-trace-timing/analysis.json).
The research goal remains active.

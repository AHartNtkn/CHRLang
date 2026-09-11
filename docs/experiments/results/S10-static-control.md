# Matching initialization preserves conditional tradeoffs, not a universal winner

Giving explicit and conditional execution the same static initialization does not establish a 10% timing gain for that pass. Demand remains competitive, but one previously directional comparison is now uncertain. Initialization often reduces allocation traffic while increasing peak requested memory, making longer-lived ownership the next consequential question.

## What the matched control changes

All compared sources, answers and counting behavior are unchanged. Explicit and conditional execution now receive the source-derived propagation initialization already used by demand. Generated controls execute code generated from the transformed rules. Inference, source copies, query initialization, preparation and disposal are charged.

The independent gate checks 216 source configurations. Every original/counting combination passes through ten explicit configurations and conditional execution, with scalar checks of original and transformed sources. That is 5,400 complete executions, including independent scalar runs. Returned answers remain valid after preparations are dropped. The [registration](../registrations/S10-static-control.md), [source gate](../../../research/chr-direct-conditional/tests/initialized_continuation.rs) and [runner](../../../research/chr-direct-conditional/examples/static_continuation_cost.rs) specify the paths.

## Initialization saves some traffic but adds ownership

The table shows counted sources with three choices, depth16, history present, original arrival and four changed queries retaining all answers. Times are medians of64 complete lifecycle measurements.

| Path/source | Initialization off | Initialization on | Requested bytes off → on | Peak excess off → on |
|---|---:|---:|---:|---:|
| Generated Scan / common | 0.505 ms | 0.503 ms | 590,592 → 542,847 | 82,013 → 87,235 |
| Generated Scan / independent | 0.719 ms | 0.720 ms | 816,349 → 780,175 | 121,733 → 124,790 |
| Generated Scan / early failure | 0.338 ms | 0.332 ms | 339,010 → 322,751 | 44,936 → 50,317 |
| Conditional / common | 0.688 ms | 0.653 ms | 632,895 → 576,850 | 50,091 → 54,901 |
| Conditional / independent | 3.829 ms | 3.445 ms | 3,611,881 → 3,400,335 | 94,628 → 104,025 |
| Conditional / early failure | 0.628 ms | 0.576 ms | 619,198 → 551,887 | 37,210 → 41,347 |

Across528 history-bearing allocation comparisons, traffic falls in336 and rises in192. Peak rises in all528. Without the propagation rule, initialization has no source work to eliminate and raises both traffic and peak in all528 comparisons. These are counts within this matrix, not workload weights.

The pass retains transformed source information and clones query inputs; its allocation and retention costs are real implementation responsibilities, not proven necessities of static analysis. The next ownership study must inspect and repair unnecessary retained owners before treating them as architectural costs.

## Demand remains credible against the matched controls

The registered intervals below compare the already-initialized demand path with initialized generated Global scanning. Each interval estimates the median of64 paired complete-time ratios.

| Demand policy | Common | Independent | Early failure |
|---|---:|---:|---:|
| Birth | 0.869–0.949 | 0.867–0.944 | 0.889–0.996 |
| Birth with miss reuse | 0.894–1.003 | 0.853–0.914 | 0.856–0.990 |
| Birth with miss/template reuse | 0.975–1.048 | 0.921–0.992 | 0.984–1.076 |

Birth is directionally faster in all three selected cases; miss reuse is directionally faster in two. The common miss-reuse interval crosses one, so the matched comparison does not preserve every directional conclusion from the preceding study. None of these nine intervals establishes a10% gain against generated Global scanning. Template-policy differences are within the practical band. Comparisons against initialized generated Active and inferred scanning include seven practical gains. [All71 intervals and samples](s10-static-control/diagnosed-audit.json).

For initialization itself,24of44 intervals are within±10% and20 have unresolved practical magnitude. Thirteen show directional gains and four directional losses, but none establishes a10% gain or loss. The71-interval simultaneous error bound is at most0.04384 under independent samples from stable distributions. Those are statistical assumptions, not guarantees of this host. First/last-half medians and all ratios are retained. No contrast is signal-limited under the registered50,400ns floor.

## The allocation audit exposed an address-dependent cost

The original cross-binary audit fails on21 template configurations. All2,400 allocation pairs within the new binary match exactly, while1,323of1,344 comparisons with the preceding binary match phase-for-phase. The21 differences affect40 execution/observation phases, in one or two192-byte allocation/deallocation increments; phase live memory and peaks are unchanged.

The template ground-value cache uses a tree keyed by heap addresses. An instrumented copy of the frozen source records actual cache insertions and independently replays their order. All36 cache groups reproduce the measured11,520bytes exactly. Changing only the replay order to sorted insertion positions requests11,904bytes; two groups differ by192bytes. Tree shape can therefore change allocation counts with address order even when the source and completed answers agree.

The [post-run diagnostic amendment](../registrations/S10-static-control-diagnosis.md) corrects the audit's cross-binary assumption. The [diagnosed audit](s10-static-control/diagnosed-audit.json) still requires exact within-binary pairs, complete observations and owner restoration, and records every parent difference. It only admits the observed template execution differences with unchanged live/peak readings and the reproduced allocation/deallocation unit. The original failed audit remains preserved. This is not a claim that arbitrary future discrepancies are explained.

No timing samples, contrasts or thresholds changed. Initialized/uninitialized comparisons use the same new binary; the parent allocation discrepancies are not silently counted as exact. [Diagnostic generator](../../../research/chr-direct-conditional/experiments/pointer_cache_diagnostic.py), [diagnostic output](s10-static-control/pointer-diagnostic.log), [all parent differences](s10-static-control/bridge-differences.json).

## Validation and interpretation

All16,100 lifecycle processes complete:100 excluded warmups,6,400 primary timings,4,800 allocation runs and4,800 ordinary/metered cancellation runs. The36 disjoint phases include all preparation passes, four query lifecycles, consumers and disposal. Independent scalar and analytical observations are checked outside measured intervals. Generated-source qualification, ordinary/meter smoke runs, strict Clippy and feature checks pass. [Frozen sources and binaries](s10-static-control/freeze.json), [campaign](s10-static-control/campaign.json), [diagnosed audit log](s10-static-control/diagnosed-audit.log).

Timing covers four selected scenarios with all answers retained; the other92 scenarios supply allocation/cancellation evidence. Requested heap is not RSS. Native compilation, process startup, validation, preallocated measurement buffers and sustained service are outside the endpoint. The reference interpreter is unchanged.

The [full portfolio review](S10-static-control-review.md) selects sustained ownership and preparation reuse under T074. The research goal remains active.

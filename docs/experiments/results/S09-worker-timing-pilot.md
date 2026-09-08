# Reused workers can reduce wall time, with higher CPU cost

Four reusable workers establish practical lifecycle wall-time gains in five registered cells, including both cold and reused balanced work. They use more process CPU in every registered cell and lose strongly on tiny queries. These results support a bounded latency/CPU tradeoff; they do not select parallel execution as a universal architecture.

## What the pilot establishes

The [prospective registration](../registrations/S09-worker-timing-pilot.md) fixes eighty configurations: five balanced/skewed/tiny source cells, one or sixteen changed queries per runtime, source quanta16/128, and inline or1/2/4 workers. All modes use the same persistent source service, conservative certificate, request-order product coordinator and window4. Eight randomized blocks comprise eighty discarded warmups and **560 measured processes**. All640 processes complete, and every query passes exact complete-product and raw-multiplicity checks.

A practical benefit requires a median paired worker/inline ratio at most0.90 with all seven paired ratios below1; loss requires a median at least1.10 with all seven above1. Other cells remain unresolved at this pilot's precision. This is the registered screening rule, not a confidence interval. Cell counts describe this experiment, not a workload distribution.

| Execution | Practical lifecycle benefits | Practical lifecycle losses | Unresolved lifecycle comparisons | Practical process-CPU losses |
|---|---:|---:|---:|---:|
| One worker | 0 | 14 | 6 | 14 |
| Two workers | 1 | 6 | 13 | 19 |
| Four workers | 5 | 6 | 9 | 20 |

Each row compares twenty source/reuse/quantum cells with inline. No worker mode establishes a practical process-CPU benefit. Four workers also establish eleven first-observation benefits and four losses; five remain unresolved. That latency endpoint starts at execution entry for query0 and excludes source preparation and query setup.

## The clearest contrasts

Ratios below are medians of paired repetition-block ratios. Less than1 means less measured resource than inline.

| Four-worker case | Lifecycle ratio | Process CPU ratio | Registered wall result |
|---|---:|---:|---|
| Balanced depth64, one query, quantum128 | 1.269 | 1.645 | Loss |
| Balanced depth64, sixteen queries, quantum128 | 0.622 | 1.446 | Benefit |
| Balanced depth256, one query, quantum128 | 0.618 | 1.431 | Benefit |
| Balanced depth256, sixteen queries, quantum16 | 0.750 | 2.066 | Benefit |
| Balanced depth256, sixteen queries, quantum128 | 0.707 | 1.871 | Benefit |
| Skewed depth256, sixteen queries, quantum128 | 0.844 | 1.696 | Benefit |
| Tiny, sixteen queries, quantum128 | 3.773 | 1.971 | Loss |

**Reuse changes a real lifecycle comparison.** At balanced depth64/quantum128, the cold four-worker path loses even though its execution/observation phase is faster. Worker preparation has a median of0.246ms and shutdown/disposal of the runtime adds about0.060ms. Over sixteen queries, the lifecycle ratio becomes0.622; the mean warm-query wall ratio, excluding query0, is0.600. This supplies the warm-pool evidence that cold-worker measurements could not establish.

**Preparation is not the only cost or benefit.** At balanced depth256 with sixteen queries and quantum128, four-worker median lifecycle is14.507ms. Setup accounts for4.932ms and joint execution/observation for8.071ms. Their paired ratios are0.952 and0.553 respectively. The source/product phase benefits while query construction and certification remain substantial. These phase medians are separate statistics and need not sum exactly to the lifecycle median.

**Coarser service is a consequential policy.** At balanced depth64 with sixteen queries, quantum16 has a favorable median lifecycle ratio0.841 but does not pass the practical-benefit rule; quantum128 does. This motivates controlling service quantum in any further comparison. It does not validate an automatic quantum-selection policy or establish how a coarser quantum affects continuing-source fairness and cancellation latency.

**Skew is a limitation, not an automatic rejection.** Most skew comparisons remain unresolved, but the largest reused skew case at quantum128 passes the benefit rule. At skewed depth64 with sixteen queries/quantum128, the median ratio is0.882 and the seven ratios span approximately0.578–1.009. That evidence cannot support a robust default choice for the smaller skewed regime. Its uncertainty does not erase the distinct balanced benefits or the measured larger skew benefit.

## Total efficiency and necessary machinery

Process CPU is a separate resource endpoint: it includes process startup, independent validation and reporting. Summed lifecycle wall intervals exclude validation and reporting. A lower wall ratio with a higher CPU ratio is a measured tradeoff between elapsed time and processor use, not proof of lower total computational cost.

The parallel organization adds worker-local preparation, channels, query generations, cancellation/drain, request ownership, ordered reply admission and joined shutdown. Its product/coordinator policies are shared with inline, making this a controlled comparison of worker organization within that runtime. The existing independent-region certificate remains conservative; the experiment does not test connected-work ownership or a different representation exposing different parallel units.

The [allocation diagnostics](S09-concurrent-meter.md) establish complete disposal and explain variable cancellation work. They are not a matched allocation matrix for every source size and reuse count in this timing pilot. Memory tradeoffs at those exact cells still need a separate measurement before a broader total-efficiency disposition. Query certification also clones rule syntax and builds region vectors while execution uses full prepared source; that explicit implementation cost is not intrinsic to all parallel architectures.

## Evidence and limits

The [raw receipts](s09-worker-timing/) include every warmup and measured process, process CPU, declared configuration and complete phase output. The [analysis](s09-worker-timing/analyze.py) verifies source hashes, configuration identity and phase arithmetic, then recomputes every registered paired ratio. The [summary](s09-worker-timing/summary.json) retains all ratios, phase ranges and comparisons against one worker. The [freeze](s09-worker-timing/freeze.json) identifies source, binary and Rust toolchain.

No run reaches the60-second or1GiB address-space bound. The [hardware receipt](s09-worker-timing/hardware.json) records16 available logical CPUs and inherited affinity0–15. The queried cgroup quota file is unavailable; no unlimited-resource claim follows. No individual worker affinity or scheduling policy was imposed.

These are recursive deterministic regions ending in explicit binary alternatives. They validate one useful/adverse workload contrast, not broad CHR programs, large output streams, large rulesets or independent user-program compilation. Neither a new production baseline nor a language restriction is selected.

## Disposition and next work

**The claim that reusable workers can repay coordination now has direct positive evidence within stated regimes.** Tiny-query losses and higher CPU use are equally part of that result. General parallel rejection based on cold owned-request measurements is unsupported. Universal parallel adoption is also unsupported.

The [matched allocation follow-up](S09-worker-allocation.md) is complete and records higher worker traffic/peaks plus a ready carrier-contraction control. T069 remains active for the existing lowered serial path comparison. Repeating every overlapping timing cell would not change the existence of the demonstrated latency/CPU tradeoff. Target an unresolved cell only when it would change a concrete eligibility or architectural choice; do not manufacture a global ranking by adding workloads or averaging cells.

At this breadth checkpoint, broader generated multihead access remains the strongest ready alternative and is next in the [ordered cycle](../remaining-investigations.md#execution-order). Completing matched allocation accounting is a short continuation of the current comparison: it can reveal a retained-memory cost that changes its total-efficiency disposition. After that bounded decision, another round of worker tuning must justify itself against the distinct generated-access investigation. Connected-work parallelism, output-heavy lifetime, coherent architecture comparisons and held-out challenges remain required elsewhere in the sequence.

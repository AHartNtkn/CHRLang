# Current mixed-work sharing evidence

The current comparison supports conditional execution for the registered pre-heavy `(64,0)` workload, with separated lifecycle ranges for both query counts. Specialized has separated lower ranges in eleven of the fourteen matched placements/reuse pairs. Balanced `(32,32)` with four queries overlaps. This narrows the practical sharing recommendation: physical work savings are real, but are insufficient to select conditional execution for mixed work generally.

The [prospective registration](../registrations/R05-current-mixed.md) and [entry gate](R05-current-mixed-entry.md) precede execution at `96fef65`. All 224 processes complete, all 28 cells have their full repetition sets, and all 8,960 answers pass the independent full-structure checker. There are no errors, cutoffs, missing jobs or application-prediction disagreements. All 28 allocation runs restore the baseline after disposal. Source and binary hashes remain unchanged, and metadata records seed 48048. [Raw process records and summaries](r05-current-mixed/metadata.json) include commands, flags, phases and work counts; full answer objects were validated outside timing rather than serialized to disk.

## Complete measured lifecycle

Milliseconds per query, median [minimum, maximum] of five primary processes. Preparation and final prepared disposal are amortized over the query count. Each later query alternates both depths by one. Combined service includes full observation and completed explicit branch disposal; engine and retained-output disposal are also charged. The allocation-free validation warms later disposal intervals. These are backend lifecycle measurements, not whole-process or isolated compiler costs.

| Pre | Post | Queries | Conditional ms | Specialized ms | Separated lower range |
|---|---|---|---|---|---|
| 0 | 0 | 1 | 1.235 [1.149, 1.281] | 0.569 [0.541, 0.605] | Specialized |
| 0 | 0 | 4 | 1.209 [1.166, 1.232] | 0.481 [0.468, 0.495] | Specialized |
| 0 | 16 | 1 | 3.527 [3.340, 3.684] | 1.993 [1.842, 2.055] | Specialized |
| 0 | 16 | 4 | 3.716 [3.547, 3.827] | 1.873 [1.839, 1.957] | Specialized |
| 8 | 8 | 1 | 2.415 [2.381, 2.612] | 1.415 [1.307, 1.433] | Specialized |
| 8 | 8 | 4 | 2.520 [2.434, 2.759] | 1.364 [1.316, 1.439] | Specialized |
| 16 | 0 | 1 | 1.377 [1.273, 1.478] | 0.956 [0.923, 1.040] | Specialized |
| 16 | 0 | 4 | 1.338 [1.309, 1.363] | 0.894 [0.863, 0.940] | Specialized |
| 0 | 64 | 1 | 11.195 [10.658, 11.764] | 6.244 [6.150, 6.336] | Specialized |
| 0 | 64 | 4 | 11.240 [11.008, 11.582] | 6.448 [6.316, 6.515] | Specialized |
| 32 | 32 | 1 | 6.474 [6.255, 7.272] | 4.002 [3.888, 4.295] | Specialized |
| 32 | 32 | 4 | 6.519 [6.295, 8.233] | 4.116 [4.008, 6.833] | Unresolved |
| 64 | 0 | 1 | 1.720 [1.669, 1.867] | 2.132 [2.108, 2.203] | Conditional |
| 64 | 0 | 4 | 1.777 [1.734, 1.818] | 2.028 [1.939, 2.078] | Conditional |

## Attribution, memory and limits

Work builds exactly reproduce `pre + 16*post + 34` Conditional applications and `1 + 16*(pre+post+3)` Specialized applications, including changed queries. At cold `(32,32)`, Conditional performs 578 applications versus 1,073 yet has higher lifecycle. At `(64,0)`, it performs 98 versus 1,073 and has lower lifecycle. The result distinguishes physical sharing from the machinery needed to realize it; application count is not elapsed cost.

At `(64,0)` cold, median execution/observation is 1.626 ms Conditional versus 2.019 ms Specialized; preparation is 0.045 versus 0.069 ms and disposal 0.017 versus 0.008 ms. Thus this advantage survives preparation and complete disposal. At cold `(32,32)` the corresponding service times are 6.349 versus 3.915 ms. No new causal intervention is required to interpret these outcomes: current complete-path timing and independent counts directly answer the registered applicability question. Earlier ratios are not multiplied into this result.

Conditional requests fewer heap bytes only in the pre-only 16 and 64 placements (both query counts); in the other placements its traffic is higher. Its peak requested heap above baseline is lower in every cell. For cold `(64,0)`, requested traffic is 1,507,849 versus 3,541,320 bytes and peak is 123,891 versus 428,540 bytes. For cold `(32,32)`, traffic is 5,335,713 versus 3,688,184 and peak is 258,571 versus 307,216. Retention and traffic therefore support different comparisons. Requested heap is not RSS, and successful finite disposal does not resolve ongoing-history retention.

Five repetitions leave four-query balanced ordering unresolved; retain that observation without extra repetitions chosen to obtain separation. No universal crossover, workload weights or application distribution follows. The finite source could admit stronger contextual lowering; these results do not prove that its countdown work is necessary. Explicit cloning and conditional historical storage are chosen implementations, not architectural lower bounds.

## Disposition and next work

T048 is complete. Keep explicit execution as the general baseline and conditional sharing as a bounded option for enough opaque work before discrimination, with contrary post-heavy, small-work and ongoing-lifetime evidence. This does not authorize automatic routing, reunion or a language restriction.

T049 selects an evidence-sufficiency and final-decision audit, rather than another timing sweep. The current control gap is now answered; the strongest ready experimental alternative is contextual recursive eligibility. That could broaden direct lowering but requires new interference/progress correspondence. The audit must decide whether that or any other unresolved direction could materially change the bounded recommendation relative to its cost, rather than treating an unselected extension as resolved. It must identify exact owner adoption choices and any genuine missing evidence, and select further investigation if needed. Neither this pilot nor the audit task definition completes the goal.

# Demand posts retain a conditional advantage against stronger execution controls

Demand execution remains a credible architecture candidate on the tested passive posts. Its favorable cases survive generated matching and both activation policies, while large unsuccessful searches remain substantially slower. These are bounded time–memory tradeoffs, not a complete architecture selection.

The [registered confirmation](../registrations/S03-post-confirmation.md) completes 55,296 primary processes: 64 repetitions of all 864 cells. All 864 excluded warmups also pass. Complete answers, endpoint metadata and service counts agree with the qualified parent. No process cuts off and no cell falls below the registered clock-signal floor.

## What the stronger comparison establishes

The matrix retains six post families, sizes 8/32, both arrival orders and immediate/window/all consumers. Each process reuses preparation for four changing queries. Three demand policies face nine explicit controls: generic Scan/Indexed, inferred specialization, Active Scan/Indexed, generated Scan/Indexed and Active generated Scan/Indexed. These are qualified controls for the same sources and answers.

The table counts scenarios whose simultaneous median-ratio interval establishes at least a 10% advantage or disadvantage against **every one of the nine controls**. Each row covers the same 72 scenarios; counts are descriptive coverage, not workload weights.

| Demand policy | Faster than every control | Slower than every control | Mixed or unresolved comparisons |
|---|---:|---:|---:|
| Birth-based demand | 46 | 6 | 20 |
| Demand with miss reuse | 42 | 6 | 24 |
| Demand with miss and template reuse | 40 | 6 | 26 |

All six across-the-board losses are size-32 unsuccessful posts: both arrival orders and all three consumer lifetimes. More caching is not uniformly beneficial. Across all 1,944 individual contrasts, 1,414 establish a practical advantage, 256 a disadvantage, 51 an interval within ±10%, and 223 leave the practical magnitude unresolved. The [full audit](s03-post-confirmation/audit.json) retains every comparison.

## Representative gains and liabilities

These witnesses follow up the preceding pilot's consequential cases. Each uses demand with miss reuse, size 32, four queries and retained-all consumers. Ratios divide demand time by the named explicit control; below one favors demand. Median times and paired median ratios are different summaries and need not divide exactly.

| Source and arrival | Explicit control | Demand / control median time (ms) | Interval for paired median ratio | Registered conclusion |
|---|---|---:|---:|---|
| Output posts, reversed | Active generated Scan | 0.464 / 1.083 | 0.409–0.442 | Demand faster |
| Duplicate posts, reversed | Active generated Scan | 0.706 / 1.718 | 0.392–0.445 | Demand faster |
| Output posts, original | Generated Scan | 1.092 / 1.275 | 0.804–0.929 | Practical magnitude unresolved |
| Forwarding posts, original | Generated Indexed | 1.884 / 2.296 | 0.771–0.865 | Demand faster |
| Duplicate posts, original | Generated Indexed | 3.242 / 2.454 | 1.310–1.460 | Demand slower |
| Unsuccessful posts, original | Generated Indexed | 6.889 / 1.216 | 5.374–6.115 | Demand slower |

**A speed benefit need not save memory.** On reversed output posts, demand requests 876,971 bytes and reaches 93,381 bytes of excess live heap; Active generated Scan requests 860,112 and peaks at 69,986. On reversed duplicate posts, demand requests less traffic, 1,166,674 versus 1,230,330 bytes, but peaks higher, 128,152 versus 99,420. The allocation readings come from the unchanged, independently audited [parent cost matrix](S03-post-control-cost.md); this confirmation does not rerun or reinterpret them as RSS.

**The unsuccessful-search liability remains material after full accounting.** On the original large miss, demand requests 12,259,743 bytes versus 878,200 for Generated Indexed. Its median combined execution/observation phase is 6.529 ms versus 0.929 ms. Preparation, inputs, setup and disposal are also charged in the complete totals above. This localizes the largest elapsed phase, but does not attribute its time among candidate copying, discovery and validation.

**Copy avoidance remains an experiment, not an established remedy.** The [copy-attribution study](S03-candidate-copy.md) bounds requested allocation from three copying scopes. It does not measure a replacement and cannot prove a runtime limit. The confirmed favorable cases justify retaining demand as a serious competitor; the adverse cases justify a causal discovery/copy comparison. Neither selects the present implementation universally.

## Uncertainty, retained samples and measurement scope

The registration fixes 64 repetitions and all 1,944 contrasts before running. Within each randomized matched block, every mode executes once. The interval uses sorted paired ratios 16 and 49; the exact binomial calculation plus Bonferroni correction bounds family error below 5% under independent repetitions from a stable distribution. This is conditional evidence for this machine and run series, not a guarantee across machines or programs.

Individual ratios sometimes vary widely. For the large original miss, the full range is 1.291–18.100 even though the median interval is much narrower. Every sample remains included. A separate [post hoc temporal check](s03-post-confirmation/temporal-check.json) splits each contrast into its first and last 32 chronological blocks: no confirmed faster/slower contrast has a half-median crossing its practical threshold. The reversed-output half-medians are 0.424 and 0.423. Maximum absolute lag-one log-ratio correlation across contrasts is 0.399. These descriptive checks expose no such simple reversal; they cannot establish independence or eliminate host interference, and they do not change the registered inference.

The exact frozen ordinary binary has engine/kernel diagnostics and allocation metering disabled. Empty-clock medians are 14/13/12/16/12 ns, giving a 38,400 ns floor across 24 intervals; every cell clears it. Execution and owned observation are combined. Source/preparation/input/setup, engine disposal, consumer operations, prepared disposal and final output disposal are charged separately. First-answer latency, native compilation and artifacts, startup, oracle validation and recording are outside the total. Complete architectural lifecycle superiority is therefore still unmeasured.

## Evidence and next decision

The [freeze](s03-post-confirmation/freeze.json) preserves source text/hashes, the parent source/build/audit identities, exact binary and randomized orders before comparisons. Compressed [primary receipts](s03-post-confirmation/runs.jsonl.gz) and [warmups](s03-post-confirmation/warmup.jsonl.gz) retain every command, stdout, stderr and status. The [campaign](s03-post-confirmation/campaign.json) completes in 278.3 seconds. The [auditor](../../../research/chr-direct-conditional/experiments/audit_post_confirmation.py) reconstructs every endpoint, phase total and paired interval; the [temporal script](../../../research/chr-direct-conditional/experiments/post_confirmation_temporal.py) reproduces the separate descriptive check. The parent allocation audit reproduces exactly.

The [full portfolio review](S03-post-portfolio-review.md) selects compact union's runtime confirmation against direct single traversal next. That is one bounded, already qualified comparison capable of changing a different complete-path choice. Demand copy/discovery repair, writable heads, dynamic choices, sustained lifetime, integration and substantive call reuse retain their own required experiments. No direction is rejected or considered answered merely because another is scheduled first.

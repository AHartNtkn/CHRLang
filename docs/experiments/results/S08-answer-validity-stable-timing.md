# Answer caching has narrow gains; template execution needs a matched control

**Within-answer validity reuse qualifies as faster in two of 64 dependency cases and none of 64 template cases.** It requests more memory in every graph case. Keep its result conditional; these measurements do not support enabling it everywhere.

The same campaign identifies a different architectural question: cached templates beat Direct in five small pure-source cases. Two of those also qualify without the answer cache. That supports investigating template/source simplification separately from graph ownership, rather than attributing the benefit to this cache.

## Validated comparison

The [fresh registration](../registrations/S08-answer-validity-stable-timing.md) uses stable expression IDs in both candidates and controls. All 384 metered entries match the repaired source/ownership receipts exactly **before warmups or timing**. The campaign then completes 3,200 ordinary processes and 16,000 fresh complete sessions, plus 320 excluded warmup processes. All 640 registered contrasts and complete source answers pass the audit.

Each process averages five sessions; each session prepares fresh rules, executes two changing queries, retains answers through producer/preparation disposal, validates complete raw answers and disposes consumer ownership. There are ten paired blocks with adjacent rotating variants. Source construction, independent validation and process startup remain outside this endpoint; compilation is not isolated. No engine, validity or allocation counters run in primary timing.

A qualified gain requires median paired ratio at most 0.90 and every block below 1; a loss requires median at least 1.10 and every block above 1. Otherwise the comparison is unresolved, not equivalent. No samples are dropped and no workload weights are assigned.

## Results that change a decision

| Comparison | Qualified candidate faster | Qualified candidate slower | Unresolved | Median ratio range |
|---|---:|---:|---:|---:|
| Cached / uncached dependency | 2 | 0 | 62 | 0.880–1.011 |
| Cached / uncached template | 0 | 0 | 64 | 0.924–1.044 |
| Direct / cached dependency | 59 | 0 | 5 | 0.119–0.627 |
| Direct / cached template | 24 | 5 | 35 | 0.365–1.139 |

The two cache gains use lookup context checking and pure depth 32 sources: distinct calls with successful tail/forward insertion (ratio 0.880, all-block range 0.520–0.964), and repeated calls with failed tail/reverse insertion (0.887, range 0.860–0.940). The changed expression-ID ownership gate retains the cost tradeoff: all 128 graph configurations request more bytes with caching;64 peak higher and 64 have equal peaks.

Direct is the faster qualified competitor in most dependency cases, but five individual comparisons remain unresolved. In the most extreme contrary block—seeking, repeated, successful consuming source, depth 8, reversed insertion—Direct's five sessions are 338,305,288,288 and 5,306µs, versus 416,336,343,293 and 307µs for cached dependency execution. That block's mean ratio is 3.847. The median-session ratio would be 0.908, but substituting that endpoint would not satisfy the registration. The observation remains part of the unresolved result; no machine-level cause is inferred.

## The template result is separate from the cache result

All five qualified cached-template advantages over Direct are distinct-call, pure, depth 8 sources:

| Context | Tail | Insertion | Direct / cached template |
|---|---|---|---:|
| Lookup | Success | Reverse | 1.121 |
| Lookup | Failure | Reverse | 1.139 |
| Seeking | Success | Forward | 1.119 |
| Seeking | Success | Reverse | 1.129 |
| Seeking | Failure | Reverse | 1.125 |

The last comparison and seeking/success/forward also qualify for uncached templates. All corresponding cache-on/off comparisons remain unresolved. Thus neither the answer cache nor general graph sharing is established as the cause of the template advantage.

For seeking/distinct/pure/depth 8/success/forward, median process-mean phase times are:

| Two-query session phase | Uncached template | Cached template | Direct |
|---|---:|---:|---:|
| Preparation | 8.45µs | 7.78µs | 2.41µs |
| Setup | 15.61µs | 16.27µs | 12.40µs |
| Execution/observation | 105.96µs | 112.14µs | 136.18µs |
| Producer disposal | 5.62µs | 5.57µs | 2.73µs |
| Prepared disposal | 0.82µs | 0.84µs | 0.89µs |
| Consumer disposal | 7.31µs | 7.99µs | 7.72µs |

These phase medians explain the shape; their sum is not the registered paired statistic. Template construction happens partly during execution, so the table does not isolate derivation cost. It motivates a matched source-simplification comparison, not a causal attribution to a particular function.

## Next investigation

The [portfolio review](S08-answer-validity-timing-review.md) selects T073: qualify a shared source/template simplification comparison in existing explicit and graph execution paths. First inspect what current template derivation follows or contracts and which transformations the explicit controls already perform. Reuse an existing transformation where applicable. Establish exact resource, choice, alias, output and progress correspondence before cost measurement; add adversarial sources when needed.

This could distinguish useful source simplification from graph-specific ownership and validity costs. It is more consequential now than trying to refine a cache with two narrow gains and extra allocation everywhere. T074 remains pending broader validity, sustained lifetime and cancellation work. No complete architecture or language restriction is selected. The research goal remains active.

Reproduce with `python 3 research/chr-reuse/experiments/answer_validity_timing.py --stable-ids --audit`. [All 640 contrasts](s08-answer-validity-stable-timing/analysis.json), frozen sources/binaries, exact ownership entries and raw sessions are retained together. The earlier failed timing campaign contributes no results to this comparison.

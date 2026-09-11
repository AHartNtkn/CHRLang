# Reuse repays union construction on overlap, while peak memory stays higher

The measured allocation crossover lies between 12 and 13 full-output queries for the specified overlapping source. At 64 queries, union requests about one-third fewer bytes than deduplicated explicit execution and has lower sampled runtime in every repeat. Its peak memory remains higher under all three consumer policies.

This is a conditional result for a finite logical-set operation. It does not select a complete CHR architecture, establish a universal union advantage, or resolve the cost of a simpler duplicate-free explicit traversal.

## The prediction survives an independent extension

The [main registration](../registrations/S06-union-reuse.md) selected 14, 15 and 64 queries before running. At 15 queries, the measured totals exactly match the preceding pilot's prediction: 1,282,728 bytes for union and 1,304,768 for deduplicated explicit execution.

Union was already ahead at 14 queries. A separately [registered allocation-only bracket](../registrations/S06-union-reuse-bracket.md) then tested 12 and 13; those choices were made after observing the main extension. Both predictions match exactly, under immediate, windowed and retained-all consumers:

| Full-output queries | Union requested bytes | Deduplicated explicit bytes | Union consequence |
|---|---:|---:|---|
| 12 | 1,221,112 | 1,205,336 | 15,776 more bytes |
| 13 | 1,268,368 | 1,289,512 | 21,144 fewer bytes |
| 15 | 1,282,728 | 1,304,768 | 22,040 fewer bytes |
| 64 | 3,801,864 | 5,644,096 | 1,842,232 fewer bytes |

The boundary belongs to this caller sequence, source and implementation. Query counts include unrestricted, two fixed-name, aliasing and contradictory callers in a five-query cycle. It is not a general “13 queries makes union worthwhile” rule. No timing was added to the 12/13 bracket.

## Runtime evidence improves, but remains exploratory

At 64 full-output queries with retained-all consumers, sampled median session time is 5.977 ms for union, 11.512 ms for explicit execution, 11.982 ms for deduplicated explicit execution and 11.636 ms for the reduced disjunction. Union's matched-repeat ratio spans 0.369–0.581 against deduplicated explicit execution and 0.482–0.587 against explicit execution.

Those five-repeat ranges give a credible favorable sizing result against the tested controls. They are not confidence intervals or held-out confirmation. At 15 queries the union/deduplicated ratio spans 0.698–0.879; the smaller traffic advantage should not be conflated with the different observed time ratio.

The full extension retains adverse sources and membership requests. Fifty-four of 120 timing contrasts contain an insufficient-signal cell under the registered clock rule. No sample is excluded. All raw ratios remain in the audit; these results do not supply workload weights or a universal ranking.

## Peak memory does not cross with traffic

At 64 overlap/full-output queries:

| Consumer policy | Union peak excess | Deduplicated explicit peak excess |
|---|---:|---:|
| Immediate release | 171,104 bytes | 135,808 bytes |
| Two-answer window | 262,688 bytes | 225,768 bytes |
| Retain all | 3,296,120 bytes | 3,105,344 bytes |

**Most of the retained-all gap belongs to the output containers.** After preparation is disposed, union's held answers still own 3,270,800 bytes versus 3,104,192 for deduplicated explicit execution: a 166,608-byte difference. Prepared state before query execution owns 25,256 versus 1,088 bytes. Final consumer disposal restores each process baseline exactly.

This rules out attributing the entire peak gap to the union graph. Every mode currently returns a `BTreeSet` of complete assignment vectors; insertion order can change its occupancy and retained allocation. The measurement establishes which owner retains the bytes, not that the difference is intrinsic to union or exact logical output. A representation that emits unique ordered rows directly could change it.

## Adverse controls still matter at longer reuse

For 64 full-output queries and retained-all consumers:

| Source | Union requested bytes | Deduplicated explicit bytes | Union peak | Deduplicated explicit peak |
|---|---:|---:|---:|---:|
| Overlap | 3,801,864 | 5,644,096 | 3,296,120 | 3,105,344 |
| Disjoint | 693,088 | 603,416 | 616,280 | 594,344 |
| Redundant | 1,116,440 | 625,432 | 627,256 | 619,288 |
| Single | 667,992 | 623,304 | 644,536 | 619,120 |

Across the main extension's 72 matched allocation scenarios, union requests fewer bytes than deduplicated explicit execution in nine overlap/full cases and more in 63. Its peak is higher in all 72. The analogous simplified-disjunction comparison has nine traffic improvements and 63 adverse cases; its peak is lower only in one case. These counts summarize coverage, not application frequencies.

Redundant source alternatives are cheaply recognized by the explicit deduplication control. Membership already allocates nothing during direct checks. More reuse cannot manufacture execution-allocation savings where there are none. Repeated identical diagram preparation and the name-solver adapter's caller transport remain implementation choices, not limits on their abstract architectures.

## The consequential challenge to the favorable result

The tested explicit output executor traverses each branch separately and inserts results into a set to merge overlaps. A simpler alternative is to traverse assignments once, reject a partial assignment only when every alternative is impossible, and emit each accepted complete assignment once. This can share discovery and avoid duplicate output without compiling a decision graph.

That alternative could reverse the favorable runtime/traffic result and address the retained-output gap. The diagram already emits unique coordinate-ordered rows; forcing it to build a set can also hide a useful property. The next comparison must distinguish branch discovery from output collection, preserve the same exact logical set, and charge conversion when an output contract actually requires it. Neither a set container nor source-alternative multiplicity may be silently imposed as the language's output contract.

## Verification, scope and next work

The main extension completes 1,584 processes: 864 allocation and 720 ordinary timing runs. The bracket adds 72 allocation processes, for 1,656 comparative processes in total. All complete-answer checks, first-result stopping/preparation reuse and final ownership checks pass. All 432 main and 36 bracket allocation cells repeat exactly. Only accepted query counts changed in the runner; engine algorithms and the reference remain unchanged.

The [main audit](s06-union-reuse/audit.json) and [bracket audit](s06-union-reuse/bracket/audit.json) verify source archives, binaries, phase sequences, exact repeats and predictions. Their [raw receipts](s06-union-reuse/), [main driver](../../../research/chr-structural/experiments/union_reuse.py) and [bracket driver](../../../research/chr-structural/experiments/union_reuse_bracket.py) preserve the prospective split. Strict Clippy and the preceding lifecycle/union evidence audits pass.

Lifecycle exclusions remain those of the [pilot](S06-union-lifecycle.md): execution/output are fused, first-row latency and native compilation are unmeasured, and oracle fixtures plus fixed harness containers sit outside totals. Logical assignments do not by themselves preserve CHR branch counts, residual occurrences or consuming effects.

The [full portfolio review](S06-union-reuse-review.md) selects the duplicate-free explicit/output contrast before further timing confirmation. It conservatively counts the separate bracket result as the fourth package boundary. Structural-prefix generated/access controls and unfinished demand/integration costs remain concrete alternatives and required work. The research goal remains active.

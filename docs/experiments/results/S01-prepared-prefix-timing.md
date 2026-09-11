# Prepared-query timing supports a conditional time–memory tradeoff

**Reusing query structure has qualified complete-time gains in several tested cases. Arena sharing has no independently qualified timing gain here.** Its measured allocation benefit remains real, but does not by itself choose the faster representation.

The [registered comparison](../registrations/S01-prepared-prefix-timing.md) completes1,920 measured processes containing96,000 fully prepared, executed, observed and disposed sessions. Another96 warmup processes are excluded. Every session validates complete answers outside measured intervals. The ordinary allocator is used, with engine/kernel/observer counters, allocation metering and CPU-clock instrumentation off.

## What meets the timing criterion

Each row below covers24 matched scenario/partner combinations. A gain requires at least10% median improvement and every one of20 paired blocks favoring the candidate. An unresolved result does not establish equivalence.

| Candidate versus reference | Qualified gains | Unresolved |
|---|---:|---:|
| Ordinary-arena reuse versus ordinary-arena fresh |6|18|
| Shared-arena reuse versus ordinary-arena fresh |8|16|
| Shared-arena reuse versus ordinary-arena reuse |0|24|

None of these three contrasts qualifies as a loss. The complete archive reports all144 registered pairwise comparisons, including comparisons in the opposite architectural direction.

For selective joins at width128, Global scheduling, ordinary partner execution and immediate answer disposal:

| Execution | Median complete two-query session |
|---|---:|
| Ordinary-arena fresh |0.798 ms|
| Ordinary-arena reuse |0.615 ms|
| Shared-arena fresh |0.807 ms|
| Shared-arena reuse |0.538 ms|

Shared reuse versus ordinary fresh qualifies: median paired ratio0.677, range0.491–0.922. Ordinary reuse versus ordinary fresh does not: median0.744, range0.537–1.186. These are paired-block criteria, so comparing the displayed medians alone would overstate the evidence.

The memory tradeoff remains consequential. In the separately metered same-source witness, shared reuse requests894,778bytes and peaks at492,046bytes above root; ordinary fresh requests1,025,080 and peaks at321,014. The faster qualified configuration therefore retains substantially more memory. Compilation is not isolated by this endpoint.

## Changed inputs, cancellation and uncertainty

Changed-right-table families remain in the matrix. Shared reuse versus fresh qualifies for duplicate/Active without probing, but the other changed-table contrasts in this comparison remain unresolved. The cancellation scenarios establish no qualified fresh/reuse timing gain. Earlier allocation evidence already shows that paying preparation upfront can make cancellation more expensive.

The additional benefit of sharing alone is uncertain in all24 comparisons. A post-hoc batch diagnostic tested whether isolated slow sessions explain the reversals:233of1,920 processes have a slowest session exceeding twice their median, and the largest ratio is12.78. Replacing process means with session medians for diagnosis reduces the count of adverse sharing pairs from147to133, but all24 contrasts still cross equality. Thus isolated slow sessions do not fully explain the uncertainty. This diagnostic neither replaces the registered endpoint nor identifies an environmental cause.

Further precision could refine a conditional policy. It currently has less decision value than investigating the separate integrated-admission gap: this package already establishes that prepared reuse can change total cost, while its peak premium and changing-term costs remain. No universal policy is selected from these component results.

## Disposition

Carry fresh execution, prepared reuse and the conditional arena-sharing alternative into coherent architecture comparisons. Preserve their preparation, cancellation, memory and term-novelty obligations. The [full portfolio review](S01-prepared-prefix-review.md) selects integrated admission under T072 next and accounts for all57 question groups. T082 remains unfinished; the research goal remains active.

[Raw evidence and freeze](s01-prepared-prefix-timing/) · [All paired comparisons](s01-prepared-prefix-timing/analysis.json) · [Uncertainty diagnostic](s01-prepared-prefix-timing/batch-diagnostic.json) · [Runnable auditor](../../../research/chr-compiled/experiments/prepared_prefix_timing.py)

The current auditor additionally checks the frozen job order, binary/archive hashes, warmup counts and the post-hoc diagnostic. The source archive preserves the execution driver before those additional audit checks.

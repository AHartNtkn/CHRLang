# Equality controls change some whole-path costs, not the overall architecture choice

Avoiding redundant invalidation removes most of the measured reset-history penalty. Source elimination handles constructor reflexivity more effectively, while runtime detection helps only when equality endpoints already share a canonical root. Mixed-source penalties remain, so these results do not select resumable or eager execution universally.

The [primary registration](../registrations/S10-equality-lifecycle.md) completed 5,376 processes and 768 exact allocation pairs. A consequential dynamic-equality result prompted a [registered alias follow-up](../registrations/S10-equality-alias-lifecycle.md): another 896 processes and 128 exact pairs. Both matrices pass complete-answer and ownership audits. Timing uses five ordinary-allocator samples per cell and remains exploratory.

## The costly reset policy was avoidable

The table reports total requested heap allocation, in decimal MB, for n=3, four changed queries sharing preparation, and answers released after each query. It includes source analysis, preparation, input, execution/delivery, disposal and a cancellation probe. Each number replays exactly in two processes. “Precise” means same-root detection; “eliminated” means syntactically identical body equations are replaced before preparation.

| Source and mode | Original, conservative | Original, precise | Eliminated, precise |
|---|---:|---:|---:|
| Reset history, eager | 7.403 | 0.434 | 0.435 |
| Reset history, resumable | 4.741 | 0.428 | 0.428 |
| Reset history, demand | 4.662 | 4.662 | 4.662 |
| Reset history, Scan | 0.628 | 0.628 | 0.622 |
| Constructor reflexivity, eager | 7.795 | 7.649 | 0.435 |
| Constructor reflexivity, resumable | 5.160 | 4.986 | 0.428 |
| Constructor reflexivity, demand | 5.079 | 5.079 | 4.662 |
| Constructor reflexivity, Scan | 0.693 | 0.693 | 0.622 |

**The reset penalty largely came from invalidation policy.** In the first measured reset query, resumable first-delivery traffic drops from 1,157,073 to 78,801 bytes with identical 1,273-byte preparation and 12,469-byte setup traffic. The original 4.74 MB whole-path result therefore cannot be treated as a necessary cost of resumable matching.

**Source elimination avoids work before runtime detection can help.** Fresh outer constructor nodes have different roots even for `box(X)=box(X)`. Their merge still invalidates matching under the precise control. Eliminating the equation avoids that construction and merge; its 643-byte source-analysis cost is included. Restarting demand retains its repeated search cost because it does not retain matching progress.

## Semantic equality and shared identity are different controls

The primary dynamic source initially binds each unknown to an atom, then equates it with another occurrence of that same atom. Both values export identically, but the occurrences have distinct canonical nodes. Precise invalidation changes no allocation readings in this source's execution path. At the substantive query-release cell, eager traffic remains 15.939 MB and resumable traffic 8.582 MB.

The follow-up passes the *same pair of unknown variables* to both the alias constraint and the later propagation item. The first equation merges their roots; later equations now satisfy the runtime check. Its expected answers preserve independence between pairs and aliases within each pair.

| Actual shared aliases, n=3/reuse=4/query release | Conservative traffic | Precise traffic | Precise peak bytes |
|---|---:|---:|---:|
| Scan | 1.180 MB | 1.180 MB | 59,015 |
| Eager | 15.662 MB | 5.808 MB | 59,125 |
| Demand | 8.180 MB | 8.180 MB | 55,669 |
| Resumable | 8.304 MB | 1.145 MB | 56,045 |

**The follow-up confirms a narrower mechanism than general redundant-equality detection.** Syntactic elimination leaves these nonidentical equations intact and adds 935 bytes of analysis traffic. Direct store tests independently demonstrate that equal ground exports can have distinct roots, while repeated shared-alias equations preserve their root. Both source variants agree with independent complete expectations and the scalar evaluator.

**Eager and resumable execution can still differ substantially.** This source performs 48 meaningful alias merges before propagation, repeatedly invalidating discovery. Preserving later redundant work does not remove the eager candidate-materialization cost during that earlier phase. The earlier one-alias work witness was not a prediction that every larger dynamic source would give these paths equal lifecycle costs.

## Contrary cases and ownership costs remain

Mixed sources retain the earlier penalties. For n=3/reuse=4/query release, precise original-source traffic is 1.436 MB eager, 1.766 MB resumable and 0.777 MB Scan on independent choices. On delayed binding it is 2.065, 2.628 and 0.996 MB respectively. Same-root detection changes none of these traffic totals. Source elimination adds 3,816 or 4,016 bytes where it cannot simplify the source.

Source analysis is an actual cost even when there is no useful query work. At n=0/reuse=1/query release, resumable delayed-source traffic rises from 12,435 to 16,451 bytes with elimination; peak grows from 7,113 to 8,987 bytes. The transformed input is released inside preparation once the engine owns its prepared representation. The reported peak therefore includes necessary overlap during preparation, not an unnecessarily retained input copy.

Retained answers change peak comparisons without changing discovery. For actual aliases with four retained query results, every mode retains 48,220 answer bytes. Precise peaks become 95,180 bytes for Scan, 102,154 for eager and 92,210 for resumable. Each query and cancellation restores preparation plus retained answers; preparation disposal leaves those answers valid; releasing them restores the initial baseline. These are bounded consumers, not a sustained-lifetime result.

## Timing, validation and interpretation

Exploratory timing agrees with some large traffic changes but does not establish a general speed ranking. Reset-history resumable median falls from 1.373 ms (range 1.163–1.895) to 0.387 ms (0.292–0.405). Actual-alias resumable falls from 2.808 ms (2.214–3.686) to 0.896 ms (0.634–0.910). These comparisons use the substantive query-release cells above.

Unchanged Scan controls expose substantial timing variation: in the actual-alias cell, conservative and precise builds have identical allocation readings but medians of 1.485 and 2.622 ms, with overlapping ranges. Five samples cannot resolve the cause or support small performance claims. No practical-win classification, overhead subtraction or workload-weighted winner follows.

The runners validate independent raw expectations and the scalar outside measured intervals, check every measured answer, and revalidate retained answers after prepared disposal. Primary timing has ordinary allocation and engine/kernel/work counters disabled; allocation builds self-check requested-heap metering. Auditors verify every phase, exact replay, execution-order coverage, owner restoration and frozen input hash. Five focused representation/broad/composition tests and strict Clippy for ordinary and allocation examples pass. Explicit Rust formatting and Python syntax checks pass.

[Primary raw results and summary](s10-equality-lifecycle/) and [alias results and summary](s10-equality-alias-lifecycle/) contain all 6,272 process receipts. Reproduce audits with `audit_equality_lifecycle.py` and `audit_equality_alias_lifecycle.py` in `research/chr-direct-conditional/experiments`. Build configuration and exclusions are in the registrations. Requested allocation is not RSS; execution and observation remain coupled in delivery; native compilation and process startup are excluded. No compilation-inclusive superiority is claimed.

## Next decision

Carry both equality controls into future coherent-path comparisons. They correct a consequential attribution while leaving adverse mixed sources and meaningful canonical-change costs visible. Neither requires a language restriction; neither establishes that general dependency-sensitive invalidation is unnecessary.

Select [support-aware conditional joining](S10-support-join-entry.md) next. Unlike another refinement of equality detection, it tests a still-unmeasured way to avoid incompatible candidate combinations before matching. Broader invalidation remains required, but the present large reset defect now has credible runtime and source controls. T078, language/lifetime comparisons, complete-architecture comparisons and held-out challenges remain unfinished.

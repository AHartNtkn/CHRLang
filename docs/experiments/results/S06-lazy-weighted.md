# Lazy output makes cancellation competitive in longer unequal-name sessions

Lazy projected output beats enumeration in all sixteen unequal-name cases at 256 queries, including first-record cancellation. It reduces requested allocation against eager projection in every scenario and never raises peak ownership. Equality stars and cliques still establish no gains over enumeration.

The result supports separating preparation from demand-driven output. It does not yet establish how weighted logical results should be delivered to an ordinary consuming caller; that is the next experiment.

## What changed

The iterator borrows prepared factors and domain slices, computes the next visible assignment when requested, and returns an owned weighted tuple. It avoids constructing the complete query-result map before the first item. The connected wrapper decodes only delivered tuples, preserving output aliases. Eager output remains a measured control.

Restriction errors and assignment bounds are checked before output. Count overflow also retains its upfront behavior: a factor-weight upper bound usually proves safety; otherwise actual products are checked before the iterator is returned. Tests cover a later overflowing tuple after an individually valid first tuple, valid restricted subsets, incompatible restrictions and annihilating zero factors. The safety check is included in query setup costs.

## Complete lifecycle results

The [registration](../registrations/S06-lazy-weighted.md) covers 192 scenarios: equality stars, equality cliques and unequal-name stars; six coordinates; duplicate choices; aliases; 4/16/64/256 changing queries; retained or immediate consumers; complete output or cancellation. Each of four executors runs twice with the allocation meter and five times with ordinary allocation and counters disabled.

| Sparse lazy compared with | Gains | Losses | Uncertain |
|---|---:|---:|---:|
| Sparse eager output | 57 | 0 | 135 |
| Cartesian lazy output | 40 | 3 | 149 |
| Enumeration | 29 | 110 | 53 |

All 576 comparisons pass clock qualification. A gain requires at least a 10% paired median improvement and all five repetitions faster; losses use the symmetric rule.

The gains over enumeration all occur on unequal-name relations:

| Queries | Full output: gain / loss / uncertain | Cancellation: gain / loss / uncertain |
|---|---:|---:|
| 4 | 0 / 6 / 2 | 0 / 8 / 0 |
| 16 | 3 / 0 / 5 | 0 / 6 / 2 |
| 64 | 6 / 0 / 2 | 4 / 0 / 4 |
| 256 | 8 / 0 / 0 | 8 / 0 / 0 |

This changes the earlier cancellation result. The measured benefit depends on session length and relation structure; it does not justify extrapolating a universal crossover or assigning workload weights.

## Output demand changes the cost substantially

For duplicate choices, plain outputs, 256 unequal-name queries and immediate consumption, median complete runtime is:

| Executor | Complete output | First-record cancellation |
|---|---:|---:|
| Sparse eager | 591.65 µs | 576.14 µs |
| Sparse lazy | 493.80 µs | 190.48 µs |
| Cartesian lazy | 486.77 µs | 187.69 µs |
| Enumeration | 1,099.07 µs | 396.62 µs |

Eager projection does almost the same work under cancellation because setup computes the whole result. With lazy output, cancellation avoids computing and decoding subsequent tuples. In this example, median query setup falls from 408.75 to 34.77 µs; obtaining the first records moves from 7.92 to 45.49 µs. The complete session still improves substantially after that work is counted.

Sparse and Cartesian lazy results are close in this example. The useful improvement is demand-driven output from a prepared relation; it is not evidence that sparse elimination is always necessary.

## Memory improves against eager output, but is not uniformly best

Sparse lazy requests fewer bytes than sparse eager in all 192 scenarios: ratios range from 0.314 to 0.969. Peak ownership falls in 62 scenarios and stays equal in 130. Relative to enumeration, requested allocation remains higher throughout, while peak ownership is lower in sixteen scenarios and higher in 176.

In the cancellation example above, requested allocation falls from 712,968 bytes for eager projection to 235,272 for lazy projection. Enumeration requests 208,700. Peak ownership stays 18,934 bytes for both projected versions, versus 12,616 for enumeration. Avoiding output work need not lower a peak established elsewhere in the lifecycle.

Every allocation repetition agrees, consumer ownership matches across executors and final ownership returns to its starting value. Retained tuples are checked after producer disposal. Measurements describe requested heap allocation rather than RSS.

## Correctness and the next decision

The finite tests check lazy output in order against independent assignment answers across 2,048 generated problem/visibility combinations, both elimination traversals and set/counted semantics. Connected tests check weighted order, aliases, restrictions, first-item cancellation and repeated traversal across the existing 128 source configurations. Projection and connected tests pass with counters enabled and disabled; the fixture test and scoped Clippy pass.

The cost matrix completes 1,536 allocation and 3,840 ordinary processes. [Allocation](S06-lazy-allocation.json) and [runtime samples](S06-lazy-cost.json) contain the numerical results. Reproduce with `python3 research/chr-structural/experiments/sparse_cost.py lazy` after building the existing runner targets.

The [portfolio decision](S06-lazy-weighted-review.md) selects delivery into a consuming caller next. Weighted tuple order, complete raw source multiplicity and observable rule competition are different obligations. Test their interaction directly before treating these gains as evidence for a coherent complete architecture. The research goal remains active.

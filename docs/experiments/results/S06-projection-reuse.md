# Longer sessions repay projection for full unequal-name output

Sparse projection beats enumeration in all eight full-output unequal-name cases at 256 queries. It still loses on equality stars and cliques, and cancellation remains adverse. The gains carry more requested allocation and higher peak ownership.

The next experiment should make projected output lazy. The current path constructs every weighted tuple before returning the first; that work persists when the consumer immediately cancels.

## The crossover was measured, not extrapolated

The runner now accepts longer sessions, sizes its recorder accordingly and cycles unrestricted, `a`, `b`, unrestricted queries. It prepares rules once, checks every actual weighted answer, and charges query, consumer and disposal costs. Independent expected results are computed outside measurement for each restriction pattern.

The [registration](../registrations/S06-projection-reuse.md) specifies six coordinates, four session lengths, ordinary/duplicate choices, plain/aliased outputs, immediate/retained consumption and complete/first-record cancellation. Each cell has two allocation runs and five randomized ordinary runs.

| Unequal-name session length | Full output: gains / losses / uncertain versus enumeration | Cancellation: gains / losses / uncertain |
|---|---:|---:|
| 4 queries | 0 / 6 / 2 | 0 / 8 / 0 |
| 16 queries | 0 / 0 / 8 | 0 / 8 / 0 |
| 64 queries | 6 / 0 / 2 | 0 / 8 / 0 |
| 256 queries | 8 / 0 / 0 | 0 / 2 / 6 |

These verdicts require at least a 10% paired median gain and every repetition faster, with the symmetric rule for losses. They establish gains at sampled session lengths, not an exact crossover between them.

For duplicate domains, plain outputs, immediate consumption and full exhaustion, complete median times are:

| Queries | Sparse projection | Cartesian projection | Enumeration |
|---|---:|---:|---:|
| 4 | 64.00 µs | 70.59 µs | 35.43 µs |
| 16 | 106.47 µs | 85.95 µs | 101.35 µs |
| 64 | 139.28 µs | 199.20 µs | 357.19 µs |
| 256 | 588.45 µs | 610.55 µs | 1,400.61 µs |

The advantage is evidence for prepared projection, rather than for sparse traversal alone. At 256 queries, all eight full-output unequal-name comparisons between sparse and Cartesian projection remain uncertain. Earlier traversal improvements do not automatically become a distinct long-session advantage.

## The adverse controls remain adverse

Neither equality stars nor equality cliques gains against enumeration at any tested session length. Of their combined 128 comparisons, 118 qualify as losses and ten remain uncertain. Longer reuse does not rescue those sources in this range.

Across all families, sparse versus enumeration produces 14 gains, 150 losses and 28 uncertain results. Sparse versus Cartesian projection produces 39 gains, two losses and 151 uncertain results. One enumeration comparison fails the clock qualification and remains uncertain; all other comparisons exceed the measurement floor.

Allocation is a real cost of the fourteen runtime gains. Requested bytes are 1.81–2.35 times enumeration's, and peak ownership is 1.002–1.82 times enumeration's. The peak difference becomes small when retained answers dominate ownership; it is larger with immediate consumption.

In the 256-query example above, sparse projection requests 712,968 bytes and peaks at 18,934 bytes; enumeration requests 359,228 bytes and peaks at 12,616 bytes. These are requested heap measurements, not RSS. All measured ownership is released at final disposal.

## First output identifies the next avoidable work

For the same 256-query input with first-record cancellation, sparse projection takes 593.77 µs versus enumeration's 393.36 µs. Sparse query setup accounts for a median 428.00 µs and first observation for 7.87 µs. Enumeration spends 23.63 µs in setup and 296.45 µs obtaining first records.

The implementation explains that split: `weighted_answers` materializes a map of every weighted tuple during projected query setup. The caller then turns that map into an iterator. Cancelling after its first item cannot recover the work already done. Enumeration instead traverses until the requested record is available.

Test a lazy weighted iterator using the existing factors, dictionaries and restriction mapping. Check complete tuple order, weights, aliases, cancellation and held results before timing it. Preserve error behavior deliberately, including count overflow and query bounds. This could change the output-policy tradeoff measured here; another preparation optimization would leave this eager output work in place.

Broader caller observers and selective trace retention remain ready alternatives. T076 stays active for this output investigation, with a portfolio review due after the next package. No source regime or output policy is assigned a workload weight, and no general architecture is selected.

## Results

The experiment completes 1,152 allocation and 2,880 ordinary processes, covering 342,720 matrix queries. Repeated allocation and consumer ownership agree; every actual weighted output matches the independent expectation, including retained answers checked after producer disposal. The runner fixture test and scoped Clippy pass.

[Allocation results](S06-projection-reuse-allocation.json) and [runtime samples and comparisons](S06-projection-reuse.json) retain the numerical evidence. Reproduce with `python3 research/chr-structural/experiments/sparse_cost.py long` after building the existing runner targets. This is package three since the portfolio review. The research goal remains active.

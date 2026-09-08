# Worker wall-time gains also consume more heap resources

All240 matched allocation processes restore their runtime baseline. Worker modes request more allocation bytes and have higher median incremental peaks than inline in every matched cell. Together with the timing pilot, this establishes a bounded wall-time/CPU/heap tradeoff within the measured persistent execution path.

## Matched evidence

The [prospective registration](../registrations/S09-worker-allocation.md) fixes the timing pilot's eighty configurations, three fresh allocation processes each. The [allocation executable](../../../research/chr-factors/examples/worker_allocation.rs) uses the same source generator, Runtime, Session, source quantum, query changes and complete expected products. Every run validates full answers and raw multiplicity, then drops queries, answers, workers and runtime state. No run reaches the60-second or1GiB address-space bound.

Allocation calls, cumulative requested bytes and free counts replay exactly in all eighty cells. Concurrent peak observations are retained individually; ratios below use the median of the three incremental peaks. Incremental peak means peak requested live bytes above the measurement's starting baseline, not RSS or process memory reserved by the allocator.

| Worker count | Requested-byte ratio range versus inline | Median incremental-peak ratio range versus inline |
|---|---:|---:|
| 1 | 1.001–1.397 | 1.009–1.368 |
| 2 | 1.002–1.599 | 1.024–1.715 |
| 4 | 1.004–2.002 | 1.051–2.407 |

These are ranges across configurations, not workload weights or uncertainty intervals. They do not imply equal importance for every source cell.

## The tradeoff in representative four-worker cells

| Source, query count, quantum | Wall ratio from timing pilot | Requested-byte ratio | Incremental-peak ratio |
|---|---:|---:|---:|
| Balanced depth64, sixteen queries, quantum128 | 0.622 | 1.042 | 1.375 |
| Balanced depth256, one query, quantum128 | 0.618 | 1.022 | 1.095 |
| Balanced depth256, sixteen queries, quantum128 | 0.707 | 1.011 | 1.110 |
| Skewed depth256, sixteen queries, quantum128 | 0.844 | 1.004 | 1.057 |
| Tiny, sixteen queries, quantum128 | 3.773 | 1.259 | 2.407 |

The recurring balanced depth64 case therefore trades lower elapsed time for higher CPU use and a materially larger live working peak. At the larger skewed case, requested-allocation overhead is proportionally small, but the timing pilot still reports higher CPU use. No aggregate efficiency winner follows without a resource preference.

## Measurement scope

The uninterrupted meter window covers source construction through runtime disposal. Expected-answer construction, argument parsing and reporting infrastructure are outside it. The known process-local channel context is initialized before the window in every mode; its48-byte retained owner is documented separately. Thus the matrix compares runtime ownership, not identical cold process-startup intervals. Exact answer checking uses in-place sorting/comparison without allocating on success. Instrumented wall times are not used.

The [raw receipts and summary](s09-worker-allocation/) retain all observations and [source/binary hashes](s09-worker-allocation/hashes.json). The [driver](../../../research/chr-factors/experiments/worker_allocation_pilot.py) reproduces the configuration matrix and descriptive ratios. The earlier timing receipt is now explicitly pinned to source commit `92498da`, preserving its applicability while experimental feature declarations evolve.

## A consequential stronger-control question remains

The source used here has pure decreasing countdown steps. The existing compiler's carrier-contraction analysis accepts all four predicates, and the [lowering gate](s09-worker-allocation/lowering-gate.json) validates24 complete-product cases across region counts1/2/4, depths0/16/64/256, and balanced/skewed inputs. This is direct eligibility and correctness evidence, not a measured serial speedup.

That control can remove intermediate execution rather than parallelize it. The matched persistent inline/worker result remains valid, but it cannot establish a parallel advantage over the strongest available serial execution. T069 therefore remains active for a bounded comparison with the already-implemented lowered path. This is more consequential than resolving a marginal worker timing cell: it can change whether the demonstrated worker opportunity survives an available optimization.

Broader generated multihead access remains the next distinct investigation in the ordered sequence. Its comparison is not discharged by a single-head countdown certificate. Connected-work parallelism, large output products, large prepared rulesets, other representations and coherent architectures remain open; the current memory matrix does not close them.

# S04 timing checkpoint: state policies have opposing cost regimes

> This records an earlier gate or checkpoint. The [complete lifecycle report](S04-lifecycle-pilot.md) and [matcher correction gate](S04-matcher-copy-gate.md) give the subsequent evidence.

All 720 registered timing processes have completed with correct full observations. Copying is faster than the existing indexed control on several small or read-heavy workloads, while indexed execution is substantially faster on mutation and work-heavy cases. Allocation diagnostics are still running, so these are timing findings, not total-efficiency recommendations.

The [registration](../registrations/S04-lifecycle-pilot.md) fixes the configurations, endpoints and five repetitions per cell. The [machine-readable timing checkpoint](s04-lifecycle/timing-checkpoint.json) contains all 144 cells, including one-query batches, first observation and cancellation. No allocation-build timings contribute here.

## Complete lifecycle for eight changing queries

Values are median milliseconds, including preparation, setup, joint execution/observation and disposal. Each query validates its full raw answers outside timing. Native compilation, process startup and fixture construction are excluded. Consumers keep all answers until each query ends.

| Workload | Copy | Trail | Checkpoint 1 | Indexed | Indexed COW |
|---|---:|---:|---:|---:|---:|
| No-choice linear work | 0.699 | 0.738 | 0.960 | 0.906 | 0.887 |
| Small balanced tree | 1.158 | 2.997 | 1.402 | 1.779 | 1.707 |
| Large retained store | 4.011 | 6.699 | 6.769 | 6.680 | 6.224 |
| Mutation | 483.422 | 877.781 | 544.733 | 21.183 | 20.560 |
| More work between choices | 18.087 | 79.395 | 19.235 | 8.399 | 7.938 |
| Deeper balanced tree | 16.516 | 38.628 | 18.610 | 17.744 | 16.369 |
| Narrower spine | 1.292 | 2.249 | 1.494 | 1.737 | 1.593 |
| Early rejection | 12.187 | 11.904 | 15.007 | 3.722 | 3.253 |
| Late rejection | 19.130 | 39.281 | 25.563 | 6.259 | 5.980 |

Unpaired medians in this table need not have the same ratio as the per-repetition paired comparisons below. Interpretation uses the registered paired ratios and their observed ranges.

## What the timings support

**Retained state and mutation give opposite comparisons.** On the retained-store case, Copy/Indexed has median paired ratio 0.612, ranging from 0.561 to 0.679. On mutation it is 23.034, ranging from 21.954 to 24.651. These differences are well beyond the pilot's 20% practical threshold. Neither result selects one whole architecture across workloads.

**The tested trail policy does not earn its switching cost in most of these families.** For eight-query batches it is consistently more than 20% slower than Copy on small, retained, mutation, work, deep, spine and late rejection. Linear work has a smaller difference; early rejection ranges across both directions and remains uncertain. This concerns the implemented common-ancestor undo/redo policy with one source step per FIFO service, not all mutable restoration or scheduling policies.

**Frequent checkpoints prevent most of root replay's large timing cost.** On mutation, root replay has a median 112.948 seconds for the eight-query batch; Checkpoint 1 takes 0.545 seconds. Copy takes 0.483 seconds and indexed execution 0.021 seconds. The matched work diagnostic explains repeated prefixes, while the remaining gap against indexed execution requires investigation of the common matcher. Larger checkpoint intervals may still have retention benefits; allocation results are needed before judging that tradeoff.

**Some smaller differences remain unresolved.** In particular, Copy versus Indexed on the deep case has paired ratios from 0.837 to 1.041. Early-rejection Trail/Copy ranges from 0.782 to 1.359. These ranges do not establish equivalence or a practical winner. Additional precision is justified only if the final time/memory decision depends on it.

## What happens next

The [measurement review](S04-measurement-review.md) identifies a concrete representation difference: the restoration matcher copies environments before rejecting partners that disagree with previously bound values. The [registered matcher diagnostic](../registrations/S04-matcher-copy-diagnostic.md) predicts the number and size of those avoidable copies and requires an all-compatible adverse control. The large mutation timing gap makes that diagnostic material before attributing the loss to restoration.

The current cell-count sweep is not a comprehensive favorable test of large immutable payloads, alternative service quanta, working-state reuse or richer checkpoint placement. Those remain S04 obligations. S05 stable-identity operation/failure reuse remains the strongest ready alternative after the initial restoration comparison and any necessary attribution correction.

The frozen allocation matrix continues in the recorded session. Final phase/allocation replay audit and the combined time/memory report remain outstanding. T066 and the broader research goal remain active.

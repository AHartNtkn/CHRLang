# S04 sizing: root replay repeats substantial source work

> This records an earlier gate or checkpoint. The [complete lifecycle report](S04-lifecycle-pilot.md) and [matcher correction gate](S04-matcher-copy-gate.md) give the subsequent evidence.

Root replay exceeds the allocation gate's initial 60-second limit on the mutation workload. The other configurations pass that gate. A separate work-count investigation identifies repeated source prefixes as a consequential cost, so the pilot now gives this case a larger explicit limit rather than shrinking its workload.

This is sizing and diagnosis. It is not a comparative timing result, and the cutoff does not reject checkpointed replay or establish an architectural winner.

## What reached the limit

The first combined allocation gate reached its 60-second wall limit. Splitting the same tests by family located the issue in mutation; splitting that family by executor isolated root replay. Copying, trailing, all three checkpoint intervals and existing indexed execution complete their two-query allocation gates. The isolated root-replay process reaches 60 seconds without a final gate result.

The [combined cutoff](s04-lifecycle/gate-meter.json), [family cutoff](s04-lifecycle/gate-meter-mutation.json) and [isolated root cutoff](s04-lifecycle/isolate-mutation-replay.json) are retained. These gates include independent evaluation and full-answer checking. Their process durations must not be used as primary engine timing ratios. The ordinary-allocator gate passes the full mutation comparison.

## Source work explains why checkpointing matters

The diagnostic build counts actual calls to the shared source-step operation. It also executes a copying traversal that records each interpretation's prefix length. For deterministic source steps under the same FIFO order, summing each repeated prefix plus its next step projects root replay's work without executing those prefixes again.

That projection exactly matches actual root-replay counts on all eight other workload families. Projected answers are checked against the independent owned-syntax evaluator, and copying/trailing counts match the traversal's service count. The large mutation root count remains a projection, not a completed measured execution count.

| Family | Copy / Trail, observed | Root replay | Checkpoint 1, observed | Checkpoint 4, observed | Checkpoint 16, observed |
|---|---:|---:|---:|---:|---:|
| No-choice linear work | 37 | 703 observed | 37 | 91 | 287 |
| Small balanced tree | 164 | 3,943 observed | 178 | 423 | 1,383 |
| Large retained store | 164 | 3,943 observed | 178 | 423 | 1,383 |
| Mutation | 2,404 | **799,623 projected** | 2,418 | 6,023 | 20,423 |
| More work between choices | 1,004 | 140,323 observed | 1,018 | 2,523 | 8,427 |
| Deeper balanced tree | 1,452 | 77,943 observed | 1,578 | 3,759 | 12,151 |
| Narrower spine | 141 | 5,007 observed | 153 | 358 | 1,167 |
| Early rejection | 323 | 51,526 observed | 331 | 811 | 2,743 |
| Late rejection | 627 | 100,166 observed | 635 | 1,571 | 5,295 |

Root replay projects roughly 333 times the source work of copying on mutation. That is not a 333-times timing estimate: source steps differ in cost, and copying/checkpoints/trails incur different ownership work. Checkpoint 1 almost eliminates repeated source steps, but its retained state and reconstruction allocations still need measurement.

The no-choice row also shows the policy's scope: root replay reconstructs even when there is no sibling to switch to. Retaining the current working state across consecutive service, changing the service quantum, and choosing checkpoint positions are possible stronger policies. This pilot compares the implemented policies explicitly; it cannot attribute all their work to a necessary property of replay.

## How the experiment proceeds

The [prospective pilot registration](../registrations/S04-lifecycle-pilot.md) now allows 180 seconds for mutation gates and 600 seconds for root-replay mutation matrix processes. All other matrix cells retain 60 seconds. Memory, source-service and correctness limits remain unchanged. The extension preserves the exact workload and permits the low-retention alternative's time/memory tradeoff to be measured.

The matrix still compares all eight configurations across nine families and one/eight changing queries. Cancellation is a separate measured scenario. Primary timing and allocation diagnostics use separate binaries; the work-count feature is absent from both. The runner refuses timing samples from a diagnostic build.

The next decision requires the lifecycle results: whether avoided state copying pays for restoration or replay, and whether a benefit survives comparison with existing indexed execution. S05 stable-identity reuse remains the strongest ready alternative after this bounded comparison. Broader checkpoint policies, sustainable lifetime and temporary separation/reunion remain unresolved.

## Evidence

The [work diagnostic receipt](s04-lifecycle/work-diagnostic.json) contains 53 complete-answer-validated rows, the command and source hashes. Eight actual root counts match the projection; the mutation projection is checked through its full copying traversal and independent answers. The instrumentation is feature-gated in `research/chr-restoration`; no reference-interpreter implementation is changed.

## Execution checkpoint

All 36 final family/build gates pass, including root replay's allocation mutation gate under the extended bound. The four binaries are frozen in [the source/binary manifest](s04-lifecycle/freeze.json); strict all-feature Clippy, release regression tests and formatting pass. The registered matrix has started. At this checkpoint 243 process receipts are complete; final paired timing and allocation conclusions await the full audit. The long-running unified exec session is recorded in T066's state and must be polled before any restart.

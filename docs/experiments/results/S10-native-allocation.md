# Native heap requests and virtual mappings now have separate measurements

The native diagnostic preserves all qualified outputs and releases every tracked heap allocation and mapping at final disposal. Its large mapping reservations are confirmed, but they are not physical-memory measurements. Python host allocations, libc-internal allocations and OS residency remain outside this result.

## What the diagnostic measures

A separately built copy of the primary harness wraps direct malloc, calloc, realloc, free, strdup, mmap and munmap calls in the compiled runtime, wire encoder and harness. The primary timing executable is unchanged. Requested bytes exclude the wrapper's aligned allocation header; calls internal to libc, including its stream buffers, are not intercepted.

The meter reports cumulative requested heap traffic, current live requested bytes and the live peak. Mapping traffic, live mapping size and its peak are separate fields. Reallocation counts the new requested size as traffic and changes live bytes by the size difference. Failed operations do not change live totals. These are diagnostic measurements; their elapsed times cannot be used as primary timing evidence.

Checkpoints cover runtime initialization, preparation, source-text release, every query reset, preparation disposal and final consumer disposal. The calloc wrapper uses real calloc instead of explicitly touching the full payload. Metadata still changes allocation size and can perturb layout, which is another reason to keep this build separate from physical-memory and timing comparisons.

## Qualification and ownership evidence

All 26 qualified sessions run twice, producing 52 processes and 958 query endpoints. Original service budgets and immediate-versus-retained consumer choices are preserved. Wire bytes match the frozen qualified observations exactly, and query service counts, pending/unsupported status and dynamic graph-word counts are unchanged. Paired memory signatures are identical.

Runtime initialization has 268,435,476 live requested heap bytes: two 128-MiB tables plus twenty bytes for five initial symbol names. The main 32-GiB heap mapping remains live after every query reset. The evaluator stack raises peak live mappings to 64 GiB in every session and is unmapped at query reset. Reusing the main heap cursor is therefore distinct from releasing its reservation.

Preparation disposal leaves zero tracked mapping bytes. It leaves between 592 and 28,416 requested heap bytes for retained consumer ownership, depending on the session. Publication and consumer disposal then return both tracked heap and mapping totals to zero. Exact wire checks include the answers published after preparation disposal, verifying that those bytes remain valid independently of the engine.

| Quantity across the 26 session configurations | Observed range |
|---|---:|
| Cumulative direct heap requests | 272,593,565–280,892,606 bytes |
| Peak live requested heap | 268,454,928–268,498,503 bytes |
| Peak live virtual mappings | 68,719,476,736 bytes (64 GiB) |
| Cumulative mapping requests across query reuse | 515,396,075,520–1,065,151,889,408 bytes |
| Tracked heap and mappings after final consumer disposal | Zero |

Cumulative mapping traffic can exceed a terabyte while simultaneous reservations remain 64 GiB. It records repeated reservation operations, not simultaneous memory use or bytes touched. Likewise, the large calloc requests do not imply equal physical residency.

## Checks that make the accounting credible

The allocator self-check verifies zero initialization, string duplication, growing and shrinking realloc, integer-overflow rejection, real allocation failure under a temporary address-space bound with the original buffer preserved, successful mapping/release, failed mapping and failed unmapping. Both ordinary and undefined-behavior-sanitized self-checks pass with identical final counters. The diagnostic runtime builds with strict warnings.

The first replay exposed a gate mismatch: the qualified native corpus mixes immediate and retained publication, so its output ordering depends on the recorded keep flag. Replaying those original flags preserves exact bytes and checks both ownership paths. Initialization also includes the five source-visible symbol allocations in addition to the two large tables; the gate checks their full total. These are accounting/control corrections, not changed source semantics.

The [audit](s10-native-allocation/audit.json) freezes the meter, build inputs, binary and raw runs. The qualified primary binary's hash is checked before building the diagnostic copy. No reference-interpreter or native language implementation changes are part of this package.

## Architectural consequence and next comparison

The native path's fixed table requests and large reservations are now measurable obligations of this implementation. They cannot yet be interpreted as a time or RSS disadvantage, or as necessary costs of native graph execution. A different runtime allocation policy remains a distinct consequential possibility if these obligations affect the comparison.

T078 next qualifies Python host allocation scope and process residency, keeping those diagnostics separate from primary timing. This is more decision-relevant now than another learning-only pilot: physical residency can materially change the interpretation of the large native requests, and host ownership is part of the coherent path the next pilot must charge. The native direct-call meter does not fill either gap.

At that boundary, choose between the bounded mixed-source cost pilot and T073's substantive learning cost pilot using the recorded decision-value comparison. Compilation, source capability, streaming/backpressure, sustained lifetime and local/parallel ownership remain required investigations. This native memory gate does not complete the research goal.

## Evidence

- [Registration](../registrations/S10-native-allocation.md)
- [Build and self-check code](../../../research/chr-hvm/native_allocation/), [strict build commands](s10-native-allocation/build.json)
- [Ordinary self-check](s10-native-allocation/check.log), [UBSan self-check](s10-native-allocation/check-ubsan.log)
- [Raw 52 processes](s10-native-allocation/runs.jsonl), [gate log](s10-native-allocation/gate.log), [audit and hashes](s10-native-allocation/audit.json)

Reproduce from the repository root with `python3 research/chr-hvm/native_allocation/build.py` and `python3 research/chr-hvm/native_allocation/gate.py`. The diagnostic allows the qualified 96-GiB virtual-address limit; each replay is bounded at thirty wall seconds and twenty CPU seconds.

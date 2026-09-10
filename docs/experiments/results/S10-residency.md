# Native reservations greatly exceed sampled residency; query reset retains pages

Native RSS samples range from 1,876 to 98,200 KiB across the qualified corpus, despite 64-GiB peak virtual reservations. Preparation disposal releases the large resident heap; query reset retains it for reuse. Host tracing also exposed an emitter reference cycle, now corrected with unchanged emitted programs and complete outputs.

## Native residency is a separate measurement

A diagnostic native build uses ordinary allocation and reads its own Linux `smaps_rollup` and `status` at explicit lifecycle boundaries. It does not use the requested-allocation wrapper. RSS, anonymous pages, private-dirty pages and virtual size are recorded separately. The monitor itself has a small process footprint; these samples are not a continuous peak and their timings are not primary evidence.

The calibration reserves 16 MiB, touches every page, and unmaps it. Recorded RSS goes from 1,552 KiB after reservation to 17,936 KiB after touching, then returns to 1,552 KiB after release. This checks that the probe distinguishes reserving address space from making pages resident.

All 26 native sessions run twice. Every one of the 958 endpoints has identical wire bytes and unchanged native work signatures against the qualified evidence. Original service budgets and immediate/retained consumer flags are preserved. The primary timing binary remains unchanged.

The largest sampled case completes 385,420 native service calls and uses 12,323,994 dynamic graph words. Its memory checkpoints show the distinction clearly:

| Boundary | RSS (KiB) | Virtual size (KiB) |
|---|---:|---:|
| Service end | 98,200 | 67,374,284 |
| Query reset | 98,196 | 33,819,852 |
| Preparation disposal | 1,868 | 3,268 |
| Consumer disposal | 1,868 | 3,268 |

The dynamic graph words occupy about 94 MiB, consistent with most of this case's resident increase. This is an inference from the graph-use and residency measurements, not an exact page attribution. The two large calloc table requests do not make their full capacity resident on this corpus.

Query reset unmaps the evaluator stack and resets the heap cursor. It does not release the main heap's touched pages. That is a reuse/retention tradeoff to carry into sustained-lifetime comparisons. Final process RSS also need not be zero: the executable, libraries and allocator state remain after engine-owned storage is disposed. Across the native runs, RSS after preparation disposal is 1,812–1,868 KiB.

## Python host allocation tracing has a narrower scope than RSS

A separate diagnostic executes the authoritative host source with tracing at its existing phase boundaries. Python tracing starts after imports. Its current/peak figures include checkpoint bookkeeping and do not cover all C/libc allocations; host RSS includes the interpreter and tracing machinery. The primary host source does not enable this instrumentation.

The final traced host matrix contains 52 processes and 958 exact external owned-wire endpoints. All native work signatures and temporary cleanup checks pass. Traced phase peaks range from 114,797 to 296,097 bytes; sampled host RSS ranges from 18,124 to 18,304 KiB at the per-process maximum phase. These are diagnostic-host figures, not an estimate of untraced host RSS or cumulative allocation traffic.

After the host returns, 23,486–26,860 traced bytes remain. Leading retained tracebacks include temporary-file support, regex/compiler caches, parsing allocations and diagnostic bookkeeping. This result does not assert zero Python memory or prove that every interpreter allocation was returned to the operating system. Host and native maxima must not be added as though they were measured simultaneously.

## A concrete host-lifetime defect was corrected

The initial host trace retained generated-program strings after function return. With cyclic GC disabled, a direct witness found nine local emitter functions still alive; collecting cycles released 35 objects and returned that count to zero. Local recursive visitors captured their own function cells and indirectly retained emitter data.

The three private recursive traversals now receive their recursive visitor explicitly instead of capturing themselves. The public compiler interface and emitted language are unchanged. The same GC-disabled witness now finds zero local emitter functions after return and zero objects for the subsequent collection. All 26 source groups produce exactly the same generated programs and dictionaries as the frozen prior emitter.

Before the correction, traced phase peaks were 131,849–351,427 bytes and post-return traced allocation was 26,306–82,394 bytes. Those ranges are diagnostic attribution, not paired timing results. The complete native/host replays and an additional untraced primary-host replay verify the corrected source emission and ownership path. Remaining interpreter/tracer retention is reported separately rather than hidden by a forced collection in the measured host.

## What this changes in the architecture investigation

Large requested allocations and reservations cannot stand in for physical residency. The native implementation's substantial query heap can nevertheless remain resident between queries, even when its allocation cursor resets. A sustained workload with occasional large queries may therefore have a different memory tradeoff from the short lifecycle gate. This remains an explicit lifetime investigation.

T078 next registers one bounded mixed-source cost pilot using the qualified ordinary Rust and combined native paths. Select complete-query batches for completion-time comparisons; engine-specific service cutoffs are not equivalent amounts of source work. Keep unsupported source capabilities visible, validate full outputs outside intervals, and distinguish internal timing from external owned-wire publication and parent-observed elapsed.

The strongest ready alternative is T073's substantive learning cost pilot. The coherent-path pilot now has source, observation, primary-clock and separately scoped memory evidence; it can test broader interactions than another learning-only source extension. That makes one bounded pilot the next priority. Reconsider T073 and the distinct-mechanism sequence after its result or a consequential obstruction, rather than extending runner refinement indefinitely.

This is not a complete architecture choice. Compilation, continuous residency peaks, sustained consumers, streaming/backpressure, richer source support and parallel/local ownership retain their investigations. No universal winner or workload weighting follows.

## Evidence

- [Prospective registration and lifetime-attribution amendment](../registrations/S10-residency.md)
- [Native build](s10-residency/build.json), [mapping calibration](s10-residency/check.log), [native raw runs](s10-residency/runs.jsonl), [native audit/hashes](s10-residency/audit.json)
- [Final host raw traces](s10-residency/host.jsonl), [host audit/hashes](s10-residency/host-audit.json), [initial host traces](s10-residency/host-before.jsonl)
- [Failing lifetime witness](s10-residency/compiler-red.log), [corrected witness and exact emission checks](s10-residency/compiler-green.log), [frozen prior emitter](s10-residency/compiler-before.py)
- [Untraced primary-host replay and audit](s10-residency/primary-recheck.log)
- [Reproduction scripts](../../../research/chr-hvm/residency/)

The goal remains active.

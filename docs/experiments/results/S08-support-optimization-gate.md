# Two Boolean optimizations preserve source results and separate their benefits

Direct cheap-root handling avoids temporary continuation and memo structures without changing engine service counts. A256-entry completed-result cache reduces service calls on the depth64 stream by38.1%. These are correctness and work results; their allocation, retention and timing tradeoffs still need a lifecycle comparison.

## Independent mechanisms

The `support-identities` feature asks the existing Boolean identity function for a direct root result. When it succeeds, the job contains no continuation frames or memo entries and returns the result on its first service call. It introduces no retained lookup table. It uses the same identities as the ordinary apply algorithm.

The separate `support-result-cache` feature stores completed substantive root results in their originating arena. Keys include the Boolean operation and canonical operands; cheap roots do not enter the cache. The default bound is256 entries with insertion-order eviction. An evicted operation is recomputed normally. Arena and job identities prevent servicing a cached job against another arena. Only complete results are inserted, so dropping an unfinished job does not publish a result.

Append-only canonical nodes give a cached operation stable meaning within its arena. New nodes do not change existing nodes. This premise does not authorize reuse after arbitrary reclamation, handle reassignment or across query arenas. Both experimental features remain off by default; cache capacity and eviction policy are controls for investigation, not selected language or architecture policies.

## Correctness evidence

Independent truth tables construct all16 Boolean functions of two variables. Not, And, Or and Difference are checked against bitwise expected results at capacities0/1/4, with immediate repetition and forced eviction. Additional tests cover interrupted jobs, interleaved equal jobs, new nodes, separate arenas and rejection of foreign-job service. A direct-root test verifies both its Boolean result and absence of continuation/memo storage.

Complete source tests run the cache alone, identities alone and both combined, with counters disabled and the existing serial/unary/equality controls. They preserve raw answers, joint aliases, multiplicity, resource effects and the tested progress obligations. The final combined suite and feature-off library checks pass; [validation logs](s08-support-optimization-gate/validation/) give the commands' results. Scoped strict Clippy and package formatting pass.

## Bounded source work

The [prospective gate](../registrations/S08-support-optimization-gate.md) specifies four release builds and inferred alias streams at depths0/64: eight runs. Each run independently checks complete source answers. Traces record apply jobs and serviced frames; they are diagnostic builds, with no timing or allocation measurement.

| Depth64 policy | Engine service calls | Boolean frames | Jobs returned directly |
|---|---:|---:|---:|
| Ordinary | 1,257,853 | 1,004,853 | 0 |
| Identities only | 1,257,853 | 997,937 | 6,916 |
| Cache only | 779,212 | 518,645 | 7,576 |
| Combined | 779,212 | 511,729 | 14,492 |

All policies create21,048 Boolean jobs and deliver the same complete answers. Direct identity jobs still occupy one public service call, explaining their unchanged engine count. Cache hits replace multi-frame work with a direct result. The bounded cache realizes7,576 substantive hits on this source, fewer than the8,544 repeated roots from the unbounded opportunity inventory.

At depth zero, all policies use739 engine service calls and185 jobs. The cache has no substantive hit; identities handle all185 jobs directly. This is a real overhead-dominated source for the next cost comparison, not evidence that the cache is free when unused.

The [raw gate records and summary](s08-support-optimization-gate/) preserve builds, sources and trace results. Ordinary-policy service counts match the prior repetition diagnostic at both depths. No output or cache behavior is inferred from timings.

## What could change the architectural decision next

The next package must charge preparation, changing-query setup, complete observation, cache/engine retention, consumer windows, cancellation and disposal. Compare the four policies independently before treating their combination as worthwhile. Use cold and repeated queries, sparse/unique operations, changed supports, aliases and distinct outputs, with strong Scan/resumable complete-path controls. Separate counter-free ordinary-allocator timing from allocation/work diagnostics and register exact repetitions and limits before running.

The cache adds a map, eviction queue, owner identity and retention bound. The identity path is simpler. A lower service count cannot decide whether that extra state repays its lookup, insertion and disposal costs, or closes the much larger complete-path allocation gap. A single capacity does not establish the best policy; capacity exploration must be justified by a consequential result.

At this gate, a bounded lifecycle comparison remains more discriminating than a new non-overlap/effect certificate because both mechanisms now have independent semantic evidence and a credible contrary tiny case. Reconsider that ordering at the cost result. Broader ownership certificates remain required, alongside reclamation, publication, residual output representation, compilation, coherent architecture comparisons and held-out challenges. This is the first package after the repetition breadth review; the goal remains active.

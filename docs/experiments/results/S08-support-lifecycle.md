# Boolean reuse reduces cost but leaves a large complete-path gap

The bounded Boolean policies reduce allocation on the tested streams, while Scan and resumable execution remain substantially cheaper on the substantive alias witness. The cache also adds retention and management responsibilities. This pilot supports a stronger conditional control, not selection of a complete architecture.

The [registration](../registrations/S08-support-lifecycle.md) defines138 cells: four Boolean policies in inferred conditional execution, plus Scan and resumable controls. Primary sources vary aliases/distinct outputs, tiny/depth64 work, preparation reuse1/4 and immediate/retained consumers. Additional cells cover cross-query four-answer windows, explicit terminal failure and cancellation followed by complete queries. The cache retains at most256 substantive completed roots per query arena.

Each cell has two allocation processes and five counter-free ordinary-allocator timing processes. Complete raw answers are independently checked against the scalar evaluator and analytical source expectations before measurements. Timed phases include source construction, preparation, each input/setup, execution with full observation and consumer eviction, each engine disposal, final consumer release and prepared disposal. First observation is recorded inside execution and never double-counted. Fixed harness buffers, validation, process startup and native compilation are excluded.

## Completed evidence

All966 processes complete:276 allocation runs and690 ordinary timing runs. The [independent audit](s08-support-lifecycle/audit.json) verifies138 exact allocation pairs, every measured phase transition, registered orders, frozen source/binary hashes and owner restoration. All168 comparable engine-owner groups are independent of consumer retention; all68 output-owner groups have equal retained bytes across the six controls. Strict scoped Clippy passes for both meter and ordinary builds, and package formatting passes.

Requested allocation falls in all23 matched configurations for identities alone and for the combination. Cache alone lowers traffic in19 and increases it in four tiny one-query cases. Both cache policies increase peak requested memory in19 configurations and leave it equal in four; identities alone lowers peaks in11 and leaves12 equal. These are unweighted cell counts, not an application score.

For aliases at depth64, four changing queries and immediate answer release:

| Complete path | Requested bytes | Peak growth, bytes | Median total time, ms | Full five-sample range, ms |
|---|---:|---:|---:|---:|
| Conditional, ordinary Boolean jobs | 370,332,261 | 2,810,282 | 681.01 | 512.17–710.26 |
| Conditional, identities | 360,599,125 | 2,810,218 | 634.15 | 599.57–705.73 |
| Conditional, cache | 208,815,077 | 2,829,418 | 359.23 | 347.95–402.64 |
| Conditional, combined | 199,081,941 | 2,829,354 | 409.17 | 353.66–427.80 |
| Scan | 9,940,433 | 352,897 | 7.18 | 6.92–8.53 |
| Resumable contextual | 10,397,489 | 36,047 | 4.41 | 4.09–4.77 |

The cache has substantially lower observed total times than ordinary Boolean jobs in this witness, but leaves a large gap to the other complete paths. Combining identities and caching lowers traffic further while its median time is higher than cache alone here; their ranges overlap. This does not justify assuming the combination is always faster.

The tiny depth-zero one-query alias case makes the contrary cost visible. Ordinary conditional requests116,239 bytes; identities52,599; cache116,271; combined52,631. Cache alone adds32 bytes with no substantive reuse. Timing ranges overlap broadly: ordinary89–208 microseconds, identities111–177, cache94–196 and combined121–161. Reduced allocation alone does not prove a tiny-case speed improvement.

[All pointwise summaries](s08-support-lifecycle/summary.json) preserve distinct outputs, failure, cancellation and consumer policies. Cancellation results are separate configurations; their partial work is not added to completed-query costs to invent a workload. Every cancelled first query is followed by the registered later complete queries, and final consumer/prepared disposal restores the owner baseline.

## What remains unanswered

The stream supplies tiny work, changing contexts, separate query arenas, failure and cancellation. It does not supply a broad substantive workload with mostly unique Boolean operations. That contrary regime remains necessary before adopting a reuse policy. Five timing samples give exploratory medians and ranges; no formal gain/loss classification or workload-weighted ranking is made.

Cheap identities require no retained result table. The cache introduces a map, insertion-order queue, ownership identity and capacity policy. Its saving must be weighed with live memory and disposal costs. Cross-query preparation reuse does not warm the Boolean cache: each query owns a fresh arena. Within-query repeated operations provide its hits.

Further capacity tuning alone is not selected next. The repetition gate already found421,337 frames in first substantive evaluations at depth64, so avoiding later root evaluations cannot eliminate all significant work. The current fixed ascending variable order is a separate representation choice: nested conditions may reconstruct chains rather than share them. A bounded variable-order comparison can test a structural explanation for both first-evaluation work and retained support nodes. This is a hypothesis to test, not an assertion that reversing order wins.

The next gate should compare the current order with a credible alternative under identical Boolean meaning and complete CHR observations, keeping direct identities and bounded cache as independent controls. Check dynamic births, contexts, consuming resources, failure and ongoing observations before cost measurements. Do not conflate physical node order with source scheduling or language semantics.

This representation question has greater immediate decision value than more precise timing of the present gap or another cache capacity. Stronger non-overlap/effect certification remains a required alternative and returns at that gate. Broad sparse-reuse costs, sustained reclamation/publication, residual-output layouts, native compilation, coherent architecture comparisons and held-out challenges remain unfinished. The research goal stays active.

# Single explicit traversal removes union's allocation advantage

A direct traversal of the disjunction obtains the same output-allocation savings as union without constructing a decision graph. Across all 96 matched scenarios, union and direct single traversal have identical execution allocation and retained output, while union has higher total traffic and peak. Union retains a sampled runtime advantage on repeated overlap, so the remaining choice is a conditional time–preparation tradeoff.

The [registered factorial comparison](../registrations/S06-unique-output.md) separates traversal from collection. It completes 768 allocation and 640 ordinary timing processes. All 384 allocation cells repeat exactly, preserve complete independent answers and restore owned heap after disposal. No complete CHR architecture is selected by this logical-set result.

## The simpler competitor changes the earlier inference

The earlier explicit control traversed every source alternative separately. The new `unique` mode traverses coordinate assignments once and rejects a prefix only when every alternative already has a violated constraint. A still-viable prefix may eventually fail; accepting it provisionally does not assert a solution. Complete assignments are emitted once, in coordinate order.

This avoids repeated discovery/output across overlapping alternatives without a prepared diagram, memo table or dynamic union operation. Preparation normalizes and deduplicates source branches exactly as the existing deduplicated control does. The candidate is an explicit execution strategy for the same qualified finite relation, not another production baseline.

For width-eight overlap, 64 full-output queries and retained-all consumers:

| Traversal / collector | Requested bytes | Peak excess | Output retained after preparation disposal | Sampled median time |
|---|---:|---:|---:|---:|
| Union / set | 3,801,864 | 3,296,120 | 3,270,800 | 5.963 ms |
| Single explicit / set | 3,277,712 | 3,271,952 | 3,270,800 | 6.866 ms |
| Branchwise deduplicated / set | 5,644,096 | 3,105,344 | 3,104,192 | 11.154 ms |
| Union / ordered rows | 5,080,856 | 2,583,352 | 2,557,984 | 3.101 ms |
| Single explicit / ordered rows | 4,556,704 | 2,559,184 | 2,557,984 | 3.503 ms |
| Branchwise deduplicated / ordered rows | 9,006,624 | 2,657,280 | 2,557,984 | 10.818 ms |

**The execution-allocation benefit belongs to avoiding duplicate discovery/output, not exclusively to a union graph.** Under the set collector, union and single explicit traversal each request 3,274,896 bytes during execution/output. Under ordered collection each requests 4,553,888. Union's preparation requests 525,240 bytes versus 1,088 for single traversal, leaving a 524,152-byte total difference in either collector.

**More repetitions of this query cycle cannot amortize that traffic difference away under these implementations.** Both traversals allocate one assignment buffer and emit the same unique ordered rows into the same collector; membership checks allocate nothing in either. The measured per-query allocation equality supports that source-level explanation across every tested configuration. A different representation, compiler or request regime remains a separate question; the earlier traffic crossover applies to the branchwise control, not to this stronger one.

**Union still avoids some runtime checking.** On the repeated overlap case, union/single-explicit time ratios range from 0.867 to 0.973 with sets and 0.776 to 0.906 with ordered rows. These are favorable sizing observations, not confirmation that a chosen practical threshold holds or a proof of general superiority. Union pays preparation to follow compiled decisions; direct traversal repeatedly checks whether alternatives remain viable.

## Output representation changes the tradeoff independently

Ordered collection reduces retained bytes from 3,270,800 to 2,557,984 for both unique traversals. This accounts for a substantial memory reduction without changing the final logical assignment set. It also removes the earlier retained-output difference between graph and explicit paths: both now emit in the same order and produce the same physical result shape.

The ordered collector appends rows directly when traversal guarantees uniqueness and order. Branchwise modes append, sort and deduplicate. All outer vectors shrink to their final length; growth, sorting, duplicate-row disposal and resizing are charged. A vector implementation returning unused capacity is not silently treated as necessary output ownership.

Lower retained memory coexists with greater requested traffic here. Vector growth and final resizing increase allocation requests even when collection runs faster and retains less. This is direct evidence against using allocation traffic as a substitute for total efficiency. It also prevents treating the preceding set-container peak difference as an intrinsic property of union.

Both collectors represent the same final set. The ordered collector additionally supplies canonical final order; branchwise collection cannot publish that order before finishing sorting/deduplication. The cancellation check concerns an internal visitor's first valid result, not first delivery from every collector. No streaming-latency or language-interface decision follows.

## Contrary regimes remain in the comparison

At 64 full-output queries with ordered rows:

| Source | Union requested bytes / peak | Single explicit requested bytes / peak | Union / single sampled median time |
|---|---|---|---|
| Overlap | 5,080,856 / 2,583,352 | 4,556,704 / 2,559,184 | 3.101 / 3.503 ms |
| Disjoint | 833,800 / 483,200 | 755,840 / 477,072 | 0.566 / 0.487 ms |
| Redundant | 1,464,944 / 501,280 | 973,936 / 493,312 | 1.060 / 0.573 ms |
| Single | 1,016,496 / 518,560 | 971,808 / 493,144 | 0.551 / 0.535 ms |

These medians describe sizing, not confirmed directional findings. One-query cases and membership remain in the full audit. Membership has identical allocation phase traces under both collector labels, as registered: the output is a boolean and neither collection path is used. No source-frequency weighting combines these regimes into a winner.

## Complexity and language consequences

Single traversal owns normalized branch constraints, a recursion buffer and the chosen collector. Union additionally constructs canonical decision nodes and temporary import/application maps, then retains its graph and intern table. Some of that construction/retention can be improved; it is not all a necessary runtime obligation. Conversely, direct traversal's repeated viability checks are real work even when they allocate nothing.

The comparison supports keeping a simple disjunction traversal as a credible competitor. It does not establish that its present prefix checks are optimal, or that compilation cannot repay itself at larger or differently structured relations. Additional indexing or incremental viability state would introduce its own preparation and ownership costs and needs a decision-relevant contrast before implementation.

Both candidates still operate on complete finite logical assignments. They do not preserve raw successful-branch counts, residual CHR occurrences or consuming effects merely by returning the same set. The previous independent source correspondence and those limitations remain applicable. Hidden projection, richer structures and whole-architecture composition remain required.

## Validation and handoff

The missing-mode test first failed, then passed with exact unique ordered output and first-result stopping across 40 source/caller configurations. The collector test first failed for the absent operation, then passed across 160 configurations, including retained values after producer disposal. Both tests pass in default and metrics-off builds. Every matrix process independently checks its full session, stopping/reuse and owned-heap restoration; the 32 smoke cases, strict Clippy and historical evidence audits also pass.

[Frozen sources and binaries](s06-unique-output/freeze.json), [raw receipts](s06-unique-output/), the [driver](../../../research/chr-structural/experiments/unique_output.py) and [audit](s06-unique-output/audit.json) preserve all cells. The auditor verifies exact repeats, collector-negative controls and identical union/single-traversal execution traffic and retained output. Five primary repetitions remain exploratory; the registered clock-signal rule and all samples remain visible. Execution/observation are fused; startup, native compilation, oracle fixtures and fixed recording containers are excluded.

**Next investigate the generated/access control for structural prefixes under T081.** The strongest distinct alternative has demonstrated saved structural work but has not faced actual generated execution or a different activation/partner plan. Those controls could change whether retained matching deserves its missing cost comparison. The present compact comparison now has a materially stronger explicit competitor and exposes a conditional runtime/preparation tradeoff; further local precision is less valuable immediately than testing that distinct architectural contrast.

T076 remains required for consequential runtime confirmation, broader relations and theories, output/service policies and complete-path integration. Reconsider those at the structural-prefix control result or obstruction, especially when choosing a coherent architecture would depend on the remaining time difference. This is one package since the full reuse portfolio review; switching tasks supplies no evidence against compact solving. The goal remains active.

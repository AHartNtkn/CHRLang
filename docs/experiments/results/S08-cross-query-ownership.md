# Cross-query retention separates equal output costs from unequal engine costs

All four engines retain equal answer bytes in this eight-query stream gate, while their execution-state costs differ substantially. Every query and prepared owner releases independently of retained answers. This is requested-allocation evidence; no ordinary-allocator timing comparison ran.

The [registration](../registrations/S08-cross-query-ownership.md) specifies 96 complete cells and 12 cancellation cells, each repeated twice. Sources produce aliased or increasingly distinct outputs, with a consuming completion rule and successful or explicitly failing tails. Every process uses depths n through n+7, alternating input order. Consumers release immediately, retain the latest four answers across query boundaries, or retain all answers. Cancellation interrupts query zero after four answers; seven subsequent queries complete.

Every query first runs outside measured ownership against both the independent scalar evaluator and an analytical answer formula. Its full raw answers, joint aliases and multiplicity must agree. Validation also retains answers across engine and prepared disposal and checks them again. The measured replay retains only the selected consumer's answers. Fixed queue and checkpoint capacities are preallocated equally across consumers and excluded from the owner baseline.

Conditional execution and inferred unary dispatch both use the scheduler-derived serial-accounting improvement and precise equality invalidation. Scan and resumable contextual execution are separate complete controls. Prefix joins are not enabled. These are current implementations of distinct organizations, not a claim that every organization has exhausted its optimization possibilities.

## Measured owners and traffic

All 216 registered processes complete, with 108 exact replays and 1,728 measured query lifecycles. The [independent audit](s08-cross-query-ownership/audit.json) checks frozen source/binary hashes, randomized order, command parameters, expected snapshot and answer counts, consumer limits and final owner restoration. All 288 engine-owner groups have identical disposal deltas across consumer policies. All 216 output-owner groups have equal retained bytes across engines. [Raw records and summaries](s08-cross-query-ownership/) preserve the complete matrix. Strict scoped runner Clippy and package formatting pass.

For the alias stream starting at depth 64, successful tails produce 548 complete answers over eight queries. Every engine retains 706,372 requested bytes for retain-all, 5,156 for the final four-answer window, and zero for immediate release after engine disposal. The window carries answers across query boundaries; it is not cleared between queries.

The same source with immediate release exposes different execution costs:

| Engine | Eighth exhausted engine, bytes before disposal | Eight-query requested traffic, bytes | Peak growth over root baseline, bytes |
|---|---:|---:|---:|
| Conditional | 2,972,057 | 805,020,310 | 2,993,463 |
| Inferred unary conditional | 2,937,215 | 803,738,829 | 2,959,770 |
| Scan | 22,016 | 20,690,205 | 363,137 |
| Resumable contextual | 1,792 | 21,402,975 | 36,307 |

Traffic includes preparation, query construction/setup, service/observation, consumer eviction and disposal. Peak growth includes prepared state and the maximum measured owner population; it is not the final engine size. The exhausted-engine column is the measured before/after disposal difference for query eight. It does not identify which internal structure owns each byte or how much could safely be released earlier.

Successful and failing tails, distinct outputs and first-query cancellation all satisfy the same ownership checks. These cases establish separate correctness and lifetime boundaries, not one combined application-weighted score. The [summary](s08-cross-query-ownership/summary.json) retains each case individually.

## Interpretation and next investigation

Output-only results do not settle residual-output layouts. This stream has no residual constraints in its completed answers. The earlier unary/propagation examples include residual stores and showed different retained output footprints. Their difference remains an explicit separate observation question.

Engine disposal returning memory establishes query-lifetime release; it does not establish economical retention during a query. Conditional support, equality, occurrence and discovery owners remain candidates for attribution. A large retained owner is not necessarily dead state, and clearing it without validating future roots would not be a sound reclamation experiment.

The next bounded question is why the conditional path allocates and retains more on the same stream despite equal owned answers. Identify responsible operations and future-live owners before registering timing or implementing collection. Compare unchanged source work with the Scan/resumable controls; any reclamation needs continued answers, alias, consumption, failure and cancellation checks. This attribution could alter the credibility of a complete architectural comparison, so it takes priority over a new non-overlap certificate at this boundary. T079 remains unfinished and returns at the next attribution result.

Longer streams, continuous single-query operation, residual-output representation, backpressure, ordinary timing, broader reclamation, native compilation and held-out complete architectures remain required. Eight changing queries are a bounded sustained-consumer experiment, not an asymptotic or universal architecture result.

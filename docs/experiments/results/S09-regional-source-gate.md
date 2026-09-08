# Reusable workers now produce complete certified query answers

The regional coordinator passes independent complete-answer checks with the same source service running inline or on reusable workers. It publishes finite products beside continuing work and supports a fresh query after cancellation. This establishes a source-correct comparison path; it supplies no parallel speedup or lifecycle-cost result.

## What was compared

The experimental runtime prepares persistent source execution once per inline runtime or worker. Each changed query uses the existing predicate-and-variable independence certificate. A session owns regional answer caches and incremental Cartesian-product jobs, preserving global output order, joint aliases, full residual multisets and raw completion multiplicity.

Inline and worker modes call the same `Search::service`. Service requests are bounded by a source quantum and an outstanding-request window. The coordinator admits replies in request order and alternates source admission with one product step. Regional answers are freshened separately before combination. Product jobs enumerate combinations incrementally rather than constructing the whole product first.

These are explicit comparison policies. Ordered admission can wait behind an earlier expensive request. Inline prefetch performs its window's work serially before admission. Neither policy is established as the best scheduler. One source step may itself be expensive, so a finite quantum is not a wall-time service bound.

## Independent evidence

The [source tests](../../../research/chr-factors/tests/reusable_regions.rs) compare complete finite observations and raw counts against the independent owned-syntax executor. Expected unique answers use pairwise exact equivalence rather than the coordinator's answer-set container. The reference interpreter is unchanged.

| Check | Scope and observed result |
|---|---|
| Changed-query product gate | Inline and 1/2/4 workers, source quanta 1/7, windows 1/4. Empty queries, independent choices, shared variables, independent unknown residuals, refutation and free output variables pass. |
| Finite source registry | Every registry case marked exhausted passes complete-product correspondence in inline and two-worker modes. This extends the directed products to the registry's source semantics. |
| Duplicate choices and cancellation | Raw multiplicity remains distinct from unique products. Dropping sessions after different service prefixes permits correct subsequent queries. |
| Continuing region | Across all four execution modes, both quanta and both windows, the two hand-derived finite products publish within 256 coordinator steps while another source branch continues. Exhaustion remains false and total raw count remains unknown. Explicit closure then permits a correct fresh query. |
| Ordinary counter-free executable | Forty complete query checks, each followed by a cancelled query, pass without test-only worker hooks or destruction counters. |
| Deliberate faults | Reusing variable identities across factors, suppressing all products and truncating product enumeration each cause runtime assertion failures. All three source mutations are restored; release tests and the ordinary executable then pass again. |

The [receipt directory](s09-regional-source/) contains commands, exit status, output, cutoff status, exact fault changes and source hashes. The [validator](../../../research/chr-factors/experiments/validate_reusable_regions.py) completed release/default and counter-free tests, the ordinary executable, package tests, Clippy and four further counter-free repeats. Each process has a 60-second timeout; none reached it. Fault success requires a runtime assertion failure, not a compile error or timeout. These repetitions check correctness and lifecycle stability, not statistical performance.

## Ownership and measurement consequences

**Query setup still recomputes the conservative certificate.** It clones rule syntax and constructs regional rule vectors; execution then uses the full prepared source rather than those regional subsets. This is compatible across inline and worker controls but potentially avoidable preparation and scanning work. Measure certificate/setup separately and investigate it if it obscures the warm-worker decision. Do not describe that cost as intrinsic to parallel execution.

**Explicit close and final disposal are different endpoints.** Closing a session cancels/drains workers and releases their searches, outstanding replies and product jobs. Regional answer caches and the global seen-answer set remain owned by the session until it is dropped. Runtime shutdown joins workers and releases prepared execution, while retained rule syntax lasts until runtime drop. Lifecycle measurement must include both drops and externally retained answer disposal.

**The product coordinator preserves global completion information.** It reports raw totals only after all regional sources exhaust, except that a certified empty factor proves global refutation. The test with continuing work deliberately does not compare an unfinished execution with a completed scalar timing.

## Next investigation and decision scope

T069 remains active. Validate process-wide allocation accounting across thread allocation/free, synchronized phase boundaries and query cancellation; then register balanced, skewed, tiny and output-heavy source families before comparative timings. Include worker CPU time, wall time, cold preparation, changing queries, observation and complete shutdown. Requested allocation bytes remain distinct from RSS.

The strongest ready architectural alternative remains generated multihead access, as recorded in the [ordered cycle](../remaining-investigations.md#execution-order). Completing source correspondence makes the worker comparison feasible; it does not rank it against that alternative or resolve connected-work parallelism, language tradeoffs or the architecture goal.

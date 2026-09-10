# Bounded consumer demand works across the admitted stream controls

The stream controls preserve complete answers when a full delivery queue pauses producer service and the consumer later resumes. They also preserve retained answers across engine disposal and preparation reuse. This qualifies a common pull-driven service endpoint; it does not show that engine memory stays bounded during sustained execution.

The finite gate checks **576 sessions and18,432 queries per build** across eight controls, two query-size trajectories, three output families, two terminal behaviors, three retention policies and two delivery schedules. The seven general controls additionally pass continued emission through64 demanded answers. Both builds pass all four tests and scoped Clippy.

## What the audit found before extending the tests

The [existing cross-query gate](S08-cross-query-ownership.md) already establishes eight-query ownership with immediate release, a four-answer window and retain-all consumers. Its queries grow from depth n through n+7. That evidence separates engine and consumer bytes, but it cannot by itself establish a memory plateau for stationary work or behavior under paused consumer demand.

The existing stream implementations supply direct continuation execution, compact live-state reuse, scanned and inferred-specialized compiled execution, dependency and derivation-template graphs, and an exact-source lazy generator. Conditional execution supplies an eighth finite control. The new gate uses those implementations and their source generator; it introduces no new reference evaluator or baseline engine.

## The service contract

A producer advances only when the delivery queue has free capacity. A full queue rejects further pump requests without calling the producer. The consumer can then remove answers and permit service to resume. This is a harness demand contract; it does not change source rule priority or require background execution during a consumer pause.

The eager consumer has queue capacity one and drains one answer. The burst consumer fills a queue of four, issues eight paused pump requests, then drains up to three answers. Exhaustion permits draining a partially filled queue. Every full-queue probe verifies that the producer service-call count stays unchanged, and every enqueue verifies the capacity bound.

Consumer retention is separate from delivery buffering. Delivered answers are released immediately, kept in a four-answer window across query boundaries, or all retained. The queue bound limits pending exported answers only. Engine histories, saved alternatives, prepared data and consumer-retained answers may still grow; no byte or RSS measurement ran here.

## Finite queries and continuing sources

The finite source includes consuming completion, delayed work and repeated, increasingly structured or jointly aliased outputs. Successful and explicitly failing tails have different mathematical raw counts. One prepared source serves32 queries with alternating input order. Stable-size queries alternate depths four/five; growing-source controls use depths four through35. The latter intentionally require increasing work and output.

Every generated query first agrees with the independent scalar evaluator and the source's mathematical complete-answer formula. Each candidate then delivers the same full raw multiset, preserving aliases and multiplicity. Retained owned answers are compared exactly with their already validated snapshots after each query engine and after preparation disposal. Verification retains separate observations; those are test owners, not evidence about measured consumer memory. A future allocator runner must place validation outside its accounting interval.

The continuing emitter is a different source: each branch either emits an answer containing a fresh shared pair of unknowns or recurs. Seven general controls run both queue schedules and all three retention policies:42 sessions per build. Demand advances through prefixes1,8,32 and64; each complete answer matches an independent formula, the source does not report exhaustion, and retained outputs remain valid after cancellation and preparation disposal. This is finite-prefix evidence, not a theorem about all infinite executions.

A separate source places one finite answer beside a spinning sibling. All seven general controls deliver that answer, remain paused while the queue is full, resume the ongoing branch, and permit cancellation followed by a new query on the same preparation. Both failing alternatives exhaust without an answer. Five controls also pass the residual-only version described below.

## Two findings that matter for later measurements

**A nominally slow consumer need not create backpressure.** The first schedule drained three answers every eight producer opportunities. The queue never filled in some sessions, so that run did not exercise the intended condition. The final schedule waits for observable fullness or exhaustion before draining; the gate now requires actual blocked attempts. These are logical service checks, not a simulation or measurement of wall-clock consumer latency.

**The dependency/template graph prototype has a source admission restriction.** It rejects the residual-only ongoing witness, whose primary head has no explicit output argument, with `head needs distinct variable output`. The admission check occurs before selecting template reuse, so both variants share this boundary. Direct, compact-live, scanned, specialized and conditional execution pass that original witness.

An additional output-passing witness makes the shared comparison possible without treating the restriction as resolved. It stores the fresh aliased pair in a named output, and all seven general controls pass it. This is a capability distinction of the current prototype, not an impossibility result for graph execution or evidence that the language should require output-passing rules.

The exact-source lazy generator also has an explicit boundary: its rules and queries must equal the finite stream schema accepted by its checker. It participates in all finite sessions but does not represent either ongoing source. The materialized resource-phase API likewise has no qualified interruptible delivery endpoint. These exclusions remain visible rather than becoming implicit language restrictions.

## Next measure the retaining owners over longer service

Select a bounded sustained-ownership experiment next under T074. The new gate provides independently checked finite and continuing endpoints, including a forced pause; requested allocation and live-memory trajectories can now distinguish exported buffering from retained engine history. Start with stable-size changed queries and continuing-emission prefixes, with the growing-source cases as separate controls.

Account for preparation, live query/frontier/history, pending delivery, consumer windows and final disposal. Compare existing graph reclamation with retention where its validity contract applies. Include repeated and distinct work, cancellation and reused preparation; validate complete finite answers or the independently specified continuing prefixes outside the measured ownership interval. Freeze lengths, modes, repetitions and limits prospectively. Requested heap traffic, live heap and RSS must remain separate, and no timing claim can come from allocation runs.

Adaptive separation's ownership/cost comparison is the strongest alternative: it now has a checked policy and opposing work effects. Sustained ownership receives this next package because it can alter the economics of several admitted organizations at once, using the just-qualified endpoint. Local claims, richer theories, broader integration/reuse, general source eligibility and coherent architectures remain required. This is package one after the [adaptive breadth review](S04-adaptive-breadth-review.md); reconsider at the ownership gate or an obstruction, with another full breadth review by package four. The research goal remains active.

## Evidence

- [Registration and entry corrections](../registrations/S08-consumer-pressure-entry.md), [source/service tests](../../../research/chr-reuse/tests/consumer_pressure.rs).
- [Final metrics-off checks](s08-consumer-pressure-entry/final-primary.log), [final default checks](s08-consumer-pressure-entry/final-default.log), [primary Clippy](s08-consumer-pressure-entry/clippy-primary.log), [default Clippy](s08-consumer-pressure-entry/clippy-default.log).
- [Audit and final source hashes](s08-consumer-pressure-entry/audit.json). The raw directory also preserves the initial schedule/admission failures and intermediate successful runs. Final hashes record the validated gate, not a pre-run performance manifest.

No final test reached its240-second command bound, finite service bound or logical pump bound. No backend implementation or independent reference code changed. Performance, physical memory, stronger service guarantees and broader language coverage remain outside this gate's claims.

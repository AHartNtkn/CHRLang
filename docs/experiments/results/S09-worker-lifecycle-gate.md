# Workers now survive changing source queries with explicit ownership barriers

A reusable pool now keeps prepared CHR source inside its workers while replacing query-owned state between runs. The lifecycle gate detects stale replies, cross-pool identities and retained query searches. Full regional partitioning, combined-answer handling and performance measurement remain next.

## What is implemented

Each worker prepares the same source once using the existing `PreparedMachine`. A query starts fresh machines, continuation queues, observation sets and raw-completion counts for its assigned regions. The original source handoff is released after worker-local preparation. No `Rc` crosses a thread boundary; query terms and returned answers use owned syntax.

Begin-query assigns a monotonically increasing generation and a fresh cancellation flag. The owner accepts no new query until end-query has completed. Query handles also retain a private pool identity, so coincident generation numbers in different pools cannot authorize service. Request IDs remain monotonic across queries.

End-query signals cancellation, drains pending service replies without adding their answers to any logical result, and waits for every worker to release its searches. Only then can the next query begin. Returned answers already owned by a caller may outlive that barrier; worker query roots may not. Test-only destruction counters directly check search-owner release after the acknowledgement.

Source service checks cancellation between atomic machine steps. A full source step is not a wall-time bound. The request window is bounded, and the reply channel has room for all outstanding replies plus lifecycle acknowledgements. Shutdown closes inputs, drains replies, checks outstanding reservations and joins every worker. Worker errors poison the pool and remain distinct from logical refutation.

## Independent correctness and adverse checks

The tests vary one, two and four workers, change region counts between zero and four, reuse identical and changed queries, and compare complete regional answers with the independent owned-syntax evaluator. The ordinary release executable performs 30 independent source-query comparisons while reusing workers; it also ends queries with pending requests.

The gate separately checks raw duplicate multiplicity and unique answers. Repeating an identical query does not inherit its previous observation set or completion count. A logically refuted query yields no answer, then a fresh query succeeds. Invalid output names are rejected before beginning a query; invalid prepared source shuts down initialization cleanly.

A coordinated test holds two workers at their service boundary, signals cancellation, releases them and checks zero source completions and no answers. Both report cancellation, and the next query starts normally. This verifies concurrent service entry and cancellation isolation; it is not a speedup measurement or an interrupt-latency guarantee.

A stale reply with an otherwise current reservation is rejected before consuming that reservation. Foreign pool and previous-query handles cannot submit work. Worker panic returns an infrastructure error, prevents further reuse and still permits cleanup. Shutdown with a full request window releases all query searches without admitting canceled results.

Three deliberate faults fail release assertions: retaining searches after end-query, accepting the wrong reply generation and accepting another pool's identity. The restored implementation passes again. Release tests run with metrics enabled and disabled; eight repeated counter-free runs check the bounded concurrency scenarios. Counter-free Clippy and an ordinary release executable without test hooks or destruction counters also pass.

## What this gate does not establish

The current pool receives already-separated source queries. It does not certify that regions are independent, assemble their answer products, preserve whole-query publication policy or define how outstanding regional replies enter those products. Passing regional source checks is therefore not complete parallel-query correspondence.

The next integration must preserve the existing independent-region certificate, empty/single-region behavior, aliases, raw and unique product observations, finite completion, cancellation with buffered products, and changed-query reuse. Inline and threaded controls should use compatible source and state representations. The `PreparedMachine` service used here has its own explicit continuation queue and observation set; comparisons with other schedulers cannot be presented as transport-only effects.

The pool fixes source for its lifetime. Each worker holds a prepared source copy and query-local state; source/program changes require a new pool in this experiment. The cost comparison must charge those preparation copies, input transfer, reply ownership, barriers and shutdown. This is an investigated organization, not a language restriction or proof that all parallel runtimes need these copies.

## Measurement feasibility

The current affinity exposes 16 logical CPUs grouped into 12 physical cores by the available topology files. No readable `cpu.max` value was found at the inspected cgroup path; that absence does not prove unrestricted CPU quota. The [hardware receipt](s09-worker-lifecycle/hardware.json) preserves the inventory. Actual scaling and contention remain unmeasured.

The existing requested-allocation meter uses process-global atomics, rather than thread-local counters. It is a possible cross-thread meter, but begin/end snapshots, phase barriers, cross-thread frees and allocation replay still need direct validation. Do not assume that a worker acknowledgement makes channel bookkeeping quiescent, or equate requested heap with RSS. Record CPU consumption as well as elapsed time when comparing different worker counts.

## Disposition and receipts

T069 remains active. Integrate the regional certificate/product owner and a compatible inline control, validate complete query observations and lifetime, then prospectively register cold versus reused pool costs. Broader generated access and corrected restoration remain ready alternatives at the next selection review. No parallel efficiency conclusion follows from this lifecycle gate.

- [Pool implementation](../../../research/chr-factors/experiments/reusable_workers.rs), [adversarial tests](../../../research/chr-factors/tests/reusable_workers.rs), [ordinary release source gate](../../../research/chr-factors/examples/reusable_workers_gate.rs).
- [Validation runner](../../../research/chr-factors/experiments/validate_reusable_workers.py), [restored release checks](s09-worker-lifecycle/restored.json), [ordinary executable](s09-worker-lifecycle/restored-executable.json), [source hashes](s09-worker-lifecycle/source-hashes.json), and individual fault/repetition receipts in the same directory.
- The reference interpreter, existing regional executor and persistent machine implementation are unchanged.

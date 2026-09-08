# S01: direct and retained discovery now execute the source fragment

**Both checked lowerings preserve the registered six-rule query outcomes. Retention maintains incident pairs correctly, but does not avoid the work of returning every requested pair.** This is semantic and work-count evidence; no lifecycle timing or heap comparison has run for these implementations.

The [implementation](../../../research/chr-compiled/src/update_join.rs) has two modes. Direct indexes rows by current key and enumerates their Cartesian product on each request. Retained additionally stores matching pairs and reverse incidence by row identity. Replacement changes only incident pairs; a binding changes the key memberships of its dependent rows. Both complete each request before processing the next driver instruction and emit at most one receipt per request-service tick.

## What was actually compiled

These are hand-written checked lowerings of the exact [registered source](../registrations/S01-update-discovery-gate.md), not general source-derived join compilers. Preparation accepts only that complete six-rule AST, including priority and bodies. The canonical source syntax is shared with the fixture; the independent row-list oracle and source replay algorithms remain separate. Source mutations that change priority or omit receipts are rejected before execution.

Query admission accepts left/right rows, exactly one closed driver script, atom-or-variable keys, arbitrary payload terms and selected output roots. Binding instructions must assign distinct variables to ground atoms. Extra source predicates, duplicate output names, malformed scripts and broader equations are unsupported queries, not logical failures. These limits are prototype admission constraints, not language restrictions. The dedicated controls still accept the ordinary source directly.

A replacement with no matching row is quiescent with the unresolved driver residual, just as the source control is; it does not silently acknowledge or fail. Prepared plans can serve another query after cancellation. Observation before completion is rejected, while unused output variables remain present in completed answers.

The separate single-rule consuming-partner witness remains covered by the ordinary controls. The exact six-rule lowering does not accept that alternative source yet. Thus the complete S01 gate is still open; there is no claim that the candidate supports arbitrary consuming joins.

## Correctness argument and checks

A row's identity is independent of its value. Equal-valued replacement obtains a fresh increasing identity; duplicate rows remain separate. The index stores each live row under its resolved key. In Retained mode, a pair is present exactly when its two live rows have equal keys; each pair also belongs to both reverse-incidence sets.

Insertion visits the opposite key bucket and installs the corresponding pairs. Retirement uses only the row's incidence set and updates the opposite sets. Binding first detaches all rows whose keys depend on that variable, changes the binding, and reattaches them under the new key. Detaching all affected rows before attachment prevents half-updated key comparisons. Payloads retain their logical handles and resolve under final bindings, preserving late aliases in existing receipts.

While a request is active, the driver cannot mutate rows. An ordered pair cursor therefore enumerates every eligible source tuple once, with no positive match made by binding variables. Finishing that enumeration corresponds to exhausting propagation before acknowledgement. Each request starts a new cursor, so the same row pair can fire in later requests. No per-query global table of all historical propagation tokens is necessary for this serial fragment. This argument depends on the closed driver and priority certificate; it does not justify removing history in general CHR.

The existing Global Scan/Indexed source controls and independent source replay remain the semantic comparison. Direct and Retained were added to the full registered grid and adverse checks, including driver-first/last input, zero requests, missing keys, replacement, late binding, duplicate rows and joint unknown aliases. Other gates cover source rejection, missing replacement, bounded resume, cancellation/reuse and unused outputs. Counter-free builds assert zero diagnostics and the same answers.

Nine gate tests pass in default and disabled-counter builds. The complete compiled-package suite passes 81 tests. Focused strict Clippy passes in both configurations. Two fresh default test processes reproduce the work diagnostic lines exactly. [Default](s01-lowered-gate/default.log), [replay](s01-lowered-gate/replay.log), [counter-free](s01-lowered-gate/off.log), [package](s01-lowered-gate/package.log), [commands and source hashes](s01-lowered-gate/validation.json).

## Work evidence

Counts refer to this implementation, not equal-duration operations. They include initial retained-pair construction. Pair visits are separate from pair construction and invalidation.

| Case | Stored pairs constructed | Pairs invalidated | Pairs visited for receipts | Receipts |
|---|---:|---:|---:|---:|
| Direct: 8×8 same-key rows, 4 requests | 0 | 0 | 256 direct cursor pairs | 256 |
| Retained: same source/query | 64 | 0 | 256 retained cursor pairs | 256 |
| Retained: 8 distinct keys, one equal-valued replacement, two requests for its key | 9 | 1 | 2 | 2 |
| Retained: 8×8 rows sharing an unknown key, ground request before binding and another afterward | 128 | 64 | 64 | 64 |

A separate dense replacement assertion requires 72 total constructed pairs, eight invalidations and 64 pairs remaining. This verifies incident maintenance rather than full-cache reconstruction. The broad binding case deliberately exposes counterpressure: the initial 64 pairs on the shared unknown key do not serve the first ground-key request, then must be invalidated and rebuilt when the key becomes ground.

The static dense source returns every compatible pair. Direct discovery is already proportional to its required receipts; retained discovery does not eliminate that output work. Stored-pair visits and direct cursor operations have different physical implementations, so these counts do not establish equal elapsed cost. Preparation, heap retention and disposal must still be charged before deciding whether retention pays.

## Remaining work and selection

S01 has executable direct and retained candidates for the six-rule source, but no comparative native measurement yet. General compilation, the alternative consuming source, finer service bounds and a genuinely selective multihead/update contrast remain open. Query setup and row updates perform size-dependent finite work within a call; one-receipt ticks do not establish a hard latency bound on those operations.

The next S01 registration must compare complete lifecycles on both favorable reuse and adverse mutation/no-request cases, and distinguish receipt cost from discovery. A selective source extension is warranted before treating this all-pairs family as evidence about maintained joins generally. The strongest ready alternative is the first-cycle graph/integration/compiler/worker feasibility assessment: its untested boundaries could alter the architecture more than another key-only size sweep. Record that assessment before expanding the cost grid; existing prototypes receive no priority from having passed this gate.

The preceding goal turn was progress: it established the source-policy incompatibility and corrected independent-checker work. This turn adds real candidate execution and incident-maintenance evidence. Neither turn resolves the architecture goal or S01 as a whole.

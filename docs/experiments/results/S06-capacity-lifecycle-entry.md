# Capacity solving has qualified lifecycle and ownership measurements

The capacity solver and four ordinary execution controls now preserve the same complete answers across changing queries and preparation disposal. All 180 configurations pass in one primary and two diagnostic runs each. This qualifies a cost comparison; it does not establish a timing advantage.

## What is measured together

The runner compares capacity solving with generic Scan, generic Indexed, inferred-specialized Scan and inferred-specialized Indexed. Each prepares the same finite consuming source once, then reuses its preparation. Source construction, inference/preparation, query construction, joint setup/execution/complete observation/engine disposal, input disposal and prepared/consumer/source disposal have explicit intervals. Internal setup and solving remain a joint interval where the APIs do not provide a credible finer boundary.

The [registered matrix](../registrations/S06-capacity-lifecycle-entry.md) varies requests 0/2/4, empty/tight/spare supplies, choice weights one/two and reuse one/four. Changing queries alternate independent and fully aliased producer variables with renamed identities. All answers retain the exact done/token residual multiplicity, unused output variable and inert noise. Complete outputs stay owned across queries and survive preparation disposal.

The primary solver compiles branch/prune diagnostic updates out through a constant parameter. Visited states remain operational because they enforce the search limit. The paired semantic test confirms identical answers and state-limit behavior with diagnostics enabled and disabled. Primary branch/prune fields remain zero; the separate diagnostic build reports them and requested heap allocation. Existing engine/kernel metrics remain disabled in both measurement builds.

## Validation

The final frozen gate completes 540 release processes: 180 primary and 360 diagnostic. Independent scalar source execution, ordinary compiled execution and the mathematical capacity relation validate complete answers outside the measured intervals. The launcher additionally checks analytical answer counts, matching primary/diagnostic outcomes and operational-state counts.

Both diagnostic repetitions have exactly equal allocation/work records after excluding elapsed time. Adjacent phases join at the same requested-live count, and final task-owned heap equals the starting baseline. Outer phase/output/count/work vectors are reserved before that baseline and remain explicitly excluded. Validation's temporary allocations restore their own starting live count. The allocation meter self-check also passes.

All six resource-capacity tests pass in default and metrics-off modes, including the 1,536-case source matrix, capacity pruning, source boundaries, explicit limits, reuse after errors and the new diagnostics pairing. Scoped primary and diagnostic Clippy pass. An initial full gate is preserved alongside the final gate; final builds and sources are identified by the current manifest and receipt.

[Semantic default](s06-capacity-lifecycle-entry/default-tests.log), [semantic metrics off](s06-capacity-lifecycle-entry/off-tests.log), [RED](s06-capacity-lifecycle-entry/red.log), [GREEN](s06-capacity-lifecycle-entry/green.log), [primary Clippy](s06-capacity-lifecycle-entry/primary-clippy.log), [diagnostic Clippy](s06-capacity-lifecycle-entry/diagnostic-clippy.log), [meter check](s06-capacity-lifecycle-entry/meter-check.log), [final gate](s06-capacity-lifecycle-entry/gate.log).

## Allocation opportunity, not a timing conclusion

For four requests, tight supply, weight one and four changed queries, all modes produce answer counts [6, 0, 6, 0]. The independent queries admit six assignments; aliased queries cannot fit their indivisible demand into the split supply. Total requested bytes include all recorded lifecycle owners and phases:

| Organization | Requested bytes |
|---|---:|
| Capacity | 130,606 |
| Generic Scan | 512,198 |
| Generic Indexed | 674,806 |
| Specialized Scan | 463,404 |
| Specialized Indexed | 626,012 |

The capacity diagnostic visits 19 states and 18 branches for each independent query and rejects each aliased query in one state without a branch. This is one registered witness, not a workload aggregate or a universal allocation ordering. Requested bytes are neither RSS nor continuous resident memory peaks. Timing must use the ordinary-allocation build and its full lifecycle endpoint.

## What the runner does not establish

This is a synchronous completion endpoint. Solver state/output-limit errors are independently tested, but the runner does not compare incremental cancellation, first external publication, ongoing work or streaming. Complete answers remain retained until batch disposal; sustained consumer retention is a separate experiment. Outer bookkeeping and independent oracle work are excluded from interval sums. Whole process elapsed includes that oracle work and cannot be presented as engine cost.

The admitted source remains the closed finite producer/consumer/failure-sink fragment from the [solver report](S06-resource-capacity-solver.md). The current finite single-head solver cannot represent its two-head private consumer. Resource fusion and traversal-count transformations answer different source conditions and remain controls where applicable; this gate supplies no broader admission or architecture claim. No reference-interpreter or existing execution-engine behavior changes.

## Next decision

Register a bounded ordinary-allocation lifecycle pilot across the five qualified modes. Use cold and reused preparation, infeasible and output-heavy sources, and include the inference/setup costs that can reverse the apparent benefit. Freeze sizes, repetition/order, resource bounds and interpretation before comparative runs. Any expanded size must first preserve complete outcomes under independent checks; source cutoffs are not losses.

Renaming-aware call reuse remains the strongest distinct alternative, because it could change earlier learning conclusions, but it still needs sound answer remapping and ownership. Capacity solving now has both a cold-query pruning witness and qualified competing lifecycle paths. One bounded pilot can determine whether that elimination repays its machinery with lower remaining entry cost. Select that pilot; review call reuse and broader compilation after it and any consequential attribution, before widening the solver fragment or repeating near-parity comparisons. Broader lifetime, language and coherent-architecture studies remain required. The goal remains active.

## Reproduction

Build the [capacity_lifecycle example](../../../research/chr-direct-conditional/examples/capacity_lifecycle.rs) in release mode with `--no-default-features --features experiment` into `target/s06-capacity-lifecycle-entry/primary`, and with `--no-default-features --features alloc-meter,capacity-diagnostics` into the corresponding diagnostic directory. The [qualification launcher](../../../research/chr-hvm/resource_capacity/lifecycle_gate.py) refuses to overwrite existing runs. [Manifest](s06-capacity-lifecycle-entry/manifest.json), [raw runs](s06-capacity-lifecycle-entry/runs.jsonl), and [audit receipt](s06-capacity-lifecycle-entry/audit.json) preserve configurations, phases, outcomes and source/binary hashes. Per-process bounds are 60 wall seconds, 20 CPU seconds and 1 GiB; the launcher bound is 600 seconds.

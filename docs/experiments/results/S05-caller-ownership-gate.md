# Complete caller resumption now has explicit lifetime ownership

The call-reuse path now imports replay equations directly into the persistent machine and executes the remaining caller program. Its ownership probe exactly replays all 16 configurations and restores every disposal baseline. Memoized interface results stay with the prepared caller; completed answers stay valid independently of caller execution state.

## The assembled execution path

`PreparedMachine::start_replaying` imports replay equations, caller constraints and outputs through one variable scope. The equations run before resumed source work. This preserves interface aliases and fresh-result identities without synthetic binding predicates or a generated caller program.

The [prepared Caller](../../../research/chr-reuse/src/calls.rs) owns the private call table, full caller program and source-admission data. Starting a query stores its input. The first execution step checks the private phase, computes or retrieves interface answers, and creates a machine for each resumed alternative. Subsequent steps return actual caller answers, including later failures and additional calls under the full source program.

A `CallerRun` owns the active machines and cursors. Each run is checked against its creating prepared caller. Complete answers are owned syntax. Dropping a run cancels its remaining work; dropping the prepared caller releases its private cache and programs. A service limit remains a terminal error on repeated stepping, never a subsequent `Done` event.

This is a batched private expansion followed by incremental caller service. It does not provide asynchronous interruption inside the first expansion step. The call service bound handles that case as an error; cancellation before expansion, after expansion and after a returned answer is separately exercised. This batching remains a latency and retention cost to measure.

## Prospective ownership evidence

The [registration](../registrations/S05-caller-ownership-gate.md) fixed Direct/Memo, recursion depths 0/8, four cancellation/exhaustion points and two identical-call queries per prepared caller. The counter-free allocation-meter release binary ran twice under a 60-second and 1 GiB address-space bound. All 16 rows, representing 32 queries per process, replay exactly.

Immutable source inputs, independent expected answers and the fixed answer-vector buffer are outside the owner baseline. The probe captures live requested heap bytes after preparation, caller disposal, consumer release and prepared disposal. Comparator validation runs between snapshots; allocation traffic and peaks are not reported. These bytes are not RSS.

| Depth-8 exhausted case | Direct calls | Memoized calls |
|---|---:|---:|
| Prepared-owner growth above baseline | 6,958 bytes | 6,958 bytes |
| Growth remaining after a query and its answers are released | 6,958 bytes | 8,843 bytes |
| Two returned answers above that retained-owner level | 610 bytes | 610 bytes |
| Growth after prepared disposal | 0 bytes | 0 bytes |

The 1,885-byte difference is retained private-cache storage in this case. It is not a leak: it survives caller cancellation/release with the prepared owner and disappears when that owner is disposed. The second identical query adds no further retained memory. The Direct control returns to its prepared baseline after every query. All cancellation configurations restore the final owner baseline.

Returned answers are independently validated after run disposal in the probe. The semantic gate also validates answers after both run and prepared caller disposal. Nothing in these results establishes that indefinite cache retention is efficient on distinct queries or long streams.

## Semantic and implementation validation

The persistent and reuse packages pass 92 tests together. All 16 call tests pass with diagnostic counters disabled. Strict scoped Clippy passes for both libraries, the call tests and the ownership example. The new replay test first failed when interface equations were ignored, then passed with shared-scope import.

The complete-caller tests compare all outputs and residuals with independent ordinary source evaluation across Direct/Memo, three depths, successful and conflicting caller updates, cancellation and reuse. They verify that a canceled caller cannot poison prepared rules or cached interface results. A separate test checks that private and caller limits remain errors on repeated polling.

[Receipts](s05-caller-ownership-gate/) include the registration's two raw runs, audit, source/binary hashes, source patch and base commit, toolchain, build and validation logs. The [replay script](../../../research/chr-reuse/experiments/call_ownership.py) enforces the resource bounds and exact owner checks. Reference-interpreter source is unchanged.

## Next decision

The assembled path is ready for a prospectively registered lifecycle pilot that includes the full resumed caller. Compare Direct and Memo calls with ordinary and compact whole-state execution, credible compiled access/specialization, and applicable source-derived or exact-source elimination. Retain adverse cases with short or distinct calls, low reuse and substantial caller work. Changed caller state must be real source work or observable residuals, not merely a different benchmark label.

Measure preparation, query setup, private expansion plus replay, caller execution/observation, cancellation and disposal. The first-observation endpoint starts before private expansion. Work counts and requested allocation remain separate from counter-free ordinary-allocator timing. Compilation costs still require their own accounting before a compilation-inclusive architectural claim.

This is the third package since the integrated breadth review. The next cost package triggers the four-package selection review, including precise S02 dependency repair, structural solving and restoration/reconnection. T075 remains active; broader call boundaries, effectful reuse, streaming, eviction, full architectures and the overall goal remain unfinished.

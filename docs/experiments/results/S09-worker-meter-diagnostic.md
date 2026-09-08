# Worker allocation accounting works; process and pool lifetimes matter

The process-global meter accounts exactly for cross-thread allocation, transfer, resizing and disposal in the isolated checks. The reusable runtime restores its requested-heap baseline after disposal once the standard library's persistent channel context is accounted separately. This supports bounded allocation diagnostics, not a parallel performance conclusion.

## Findings that affect the pilot

| Finding | Evidence | Measurement consequence |
|---|---|---|
| Cross-thread ownership is accounted correctly | Two workers each allocate 1,024 bytes for parent disposal, free parent-allocated 2,048-byte buffers, and grow/shrink buffers from 16 to 64 to 8 bytes. Five isolated processes pass exact requested bytes, calls and live restoration. | The existing meter need not be replaced with a thread-local meter. It measures requested heap demand across these ownership transfers. |
| A blocking channel initializes 48 bytes of persistent context | The first worker lifetime retains 48 bytes; the next seven restore their baseline. Calling `thread::current()` does not account for it. A separate empty-channel blocking operation retains exactly 48 bytes, after which all runtime cycles restore baseline. Installed standard-library source caches an `Arc<Inner>` in a thread-local channel context. | Charge process-level channel initialization separately from query and pool disposal. Do not classify these bytes as retained CHR query state or silently discard them from cold cost. |
| First request-map use retains 192 bytes until pool disposal | First queries use 45 allocation calls and 4,840 requested execution/observation bytes; later queries use 44 and 4,648. This holds for 1/2/4 workers. An isolated `BTreeMap<u64, usize>` insert/remove reproduces the 192-byte retained node, subsequent reuse allocates nothing, and dropping the map releases it. The pool inserts/removes requests in that map. | Keep first-query and reused-query diagnostics separate. This is reusable bookkeeping, not unexplained measurement variance. |
| Query close is not complete disposal | The phase receipts distinguish close, session drop, answer drop, shutdown and runtime drop. | Include the complete ownership chain in total lifecycle accounting. A worker acknowledgement alone is not the final heap endpoint. |

The attribution to the channel context uses local installed Rust source at `library/std/src/sync/mpmc/context.rs`: `Context::with` retains a context in thread-local storage, and `Context::new` allocates its `Arc<Inner>`. The [toolchain receipt](s09-worker-meter-channel/toolchain.json) identifies that file and hash. The isolated operation reproduces the persistent allocation without a CHR runtime. This explains the observed owner; it does not establish a portable 48-byte ABI requirement.

## What ran

The [prospective diagnostic](../registrations/S09-worker-meter-diagnostic.md) specifies the initial checks and interpretation before the lifecycle matrix. The [initial receipts](s09-worker-meter/) expose the first baseline mismatch. The [thread-handle attribution control](s09-worker-meter-context/) records that merely obtaining the current thread handle does not resolve it. Both retain the exact executable source used in those screens.

The [completed channel-context diagnostic](s09-worker-meter-channel/) contains thirty isolated executable processes: five cross-thread checks, five request-map checks, and five lifecycle processes for each of inline and 1/2/4 workers. Each lifecycle process runs eight pool/runtime lifetimes with four queries apiece: **160 complete runtime lifetimes and 640 complete queries**. All restore the post-channel-initialization baseline after full disposal. The executable independently checks the hand-derived complete answer and raw count one outside measured intervals.

Phase traffic is stable on this deterministic witness after separating first and subsequent queries. Concurrent peaks may vary with allocation order; they are not required to replay exactly. Counters use atomics, but multi-counter readings are not a transactional snapshot of arbitrary concurrent allocator activity. This result therefore validates the tested synchronized usage, not arbitrary phase boundaries.

The final executable passes counter-free release compilation and Clippy. The [runner](../../../research/chr-factors/experiments/validate_worker_meter.py) applies a 60-second process timeout and 1 GiB address-space limit. No completed diagnostic process reaches either bound. The [source and binary hashes](s09-worker-meter-channel/source-binary-hashes.json) identify the measured artifact. No production engine, reference interpreter or shared allocation-meter source changed.

## Remaining work before comparative timing

This allocation witness has one independent source region. Multiple workers exist, but only one executes source work. It cannot validate concurrent regional allocation peaks, cancellation while several services are pending, or out-of-order reply bookkeeping. Those checks remain necessary for the intended balanced/skewed parallel sources.

Next, exercise multiple active regions, cancellation and changed query sizes under the same meter. Compare whole-lifecycle traffic and phase-local traffic separately; if acknowledgements do not establish stable boundaries, use a joint interval or validate stronger quiescence. Include process CPU time as well as ordinary-allocator wall timing in the subsequent prospective cost registration. Requested allocation bytes remain distinct from RSS.

T069 and S09 remain open. The diagnostic makes the next parallel comparison more trustworthy; it does not rank parallel organization, settle connected-work parallelism or select an architecture.

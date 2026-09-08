# S09 worker allocation diagnostic: prospective checks

This diagnostic establishes which requested-heap measurements can support the reusable-worker pilot. It makes no timing comparison. The process-global allocator in `research/chr-compiled/experiments/meter.rs` is unchanged.

Before running the lifecycle matrix, fix these checks and interpretation:

- Run five isolated processes each for inline and 1/2/4 workers. Each process performs eight complete runtime lifetimes, each with four queries over reused preparation. The source simplifies `p(X)` to `X=a`; every query checks that complete answer and raw count one outside allocation intervals.
- Record preparation, query setup, execution with observation, explicit close, session drop, external answer drop, worker shutdown and runtime drop. Buffer readings in preallocated storage and emit them after measured lifetimes. Query construction belongs to setup. Timing is not an endpoint.
- Run five additional isolated cross-thread processes. Two prepared, idle threads allocate 1,024 bytes each, transfer ownership for parent disposal, dispose parent-allocated 2,048-byte buffers, and grow/shrink buffers from 16 to 64 to 8 bytes. Atomic command/acknowledgement phases and precreated storage isolate these allocator operations from channel bookkeeping. Exact calls, requested bytes and live restoration must match. Concurrent peak order may vary.
- Each executable process has a 60-second timeout and 1 GiB address-space limit. A cutoff or assertion failure is a diagnostic failure to investigate, never a speed or memory ranking.
- Compare allocation calls, requested bytes, frees and live deltas across repeated phases. Stable phase traffic supports that boundary on this witness only. Variation around worker acknowledgements means phase-local traffic cannot require exact replay without a stronger quiescence protocol; use a joint interval or implement and validate that protocol before claiming isolated costs.
- After each full runtime disposal, live requested bytes must return to the pre-preparation baseline. A difference requires identifying the retaining owner, including runtime infrastructure. Do not call an allocation difference a semantic leak without that attribution.
- Cumulative requested traffic and simultaneous peak are different quantities. Peak variation under concurrent schedules is not failure of traffic accounting. No requested-heap metric is RSS.

The hand-controlled cross-thread check has already passed one exploratory run. The lifecycle matrix and its boundary interpretation are registered here before their first runs. This diagnostic does not register comparative performance hypotheses or close T069.

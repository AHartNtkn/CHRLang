# Changed-query prepared sessions: pilot

## Question and hypotheses

Does preparation reuse change the complete-path comparison as sessions grow, and does engine-owned requested heap remain bounded under releasing consumers? Reuse should save fixed preparation traffic and time; a contrary is that preparation is too small to matter or longer-lived state grows. Retaining all answers should incur consumer-owned growth; a two-query window should bound it. RSS need not track requested live heap because the allocator may retain pages.

Use the existing mixed sources and corrected detached ownership. Every query has seed i, depth 4+i%3, and reversed arrival when i is odd. Independently execute each original source through the scalar interpreter before measurement and compare complete expected observations. Keep no scalar answers in the measured session. Validate measured outputs against those analytical observations outside the timing intervals and after producer disposal. Counted queries remain the same source contract; the scalar checks use original queries.

## Exact configurations

Families common and independent, choices3, history enabled; query counts1,16,128; consumers immediate, window2 and all. Compare one preparation per session versus one per query. Seven paths: birth-miss with initialization; conditional and native-scan each with initialization off/on; active-native-scan and sealed-scan with initialization. All use the counting pass. This is252 cells.

Five ordinary-allocator primary repetitions per cell, one excluded warmup, two allocation-meter repetitions, two separate ordinary RSS repetitions, and one ordinary plus one metered cancellation run per cell:3,024 processes. Cancel even-indexed queries after the first service step, finish odd-indexed queries, and dispose every owner. Noncancelled execution records first answer and full completion. RSS is sampled at root, preparation, after each consumer action, final preparation disposal and final consumer disposal using a stack-backed smaps_rollup read. Preallocate and touch harness storage before root; retain identical capacities for reuse/rebuild within each query count.

Freeze source, runner, binary and job hashes before comparison. Shuffle seed740913 within stage/rep; CPU0;60-second wall/CPU and1-GiB address-space per process;2,000,000 service calls per query;20-minute campaign. Any failure or cutoff is retained and investigated. No silent exclusions. Allocation repeats must exactly match every phase, and all task-owned live heap must return to root. Distinguish engine/consumer ownership from total resident pages and bounded validation-harness allocator effects.

## Analysis and next decision

This is a pilot, not statistical confirmation. Report five-sample median paired reuse/rebuild ratios and ranges; do not call their sign a confirmed gain. Sum disjoint preparation, query input/lowering/setup, execution with exact observation, engine disposal, consumer ownership changes and final disposal. First-answer latency includes preparation only for the first query under reuse and every query under rebuilding; retain both execution-first and lifecycle-to-first measures. Charge all phases before claiming an amortization benefit.

Compare requested traffic, preparation owners, released-consumer live trajectories, retained consumer bytes after preparation disposal and maximum phase peak. RSS readings are process totals; report within-process changes from warmed root and across two independent repetitions, without interpreting retained pages as live engine objects. Inspect consequential growth/crossovers before selecting confirmation. Native program compilation and process/oracle/harness overhead are not in the summed execution endpoint; this comparison makes no compilation or cold-process superiority claim.

At the pilot result, select a bounded confirmation where uncertainty could change the architecture comparison, or the separate continuing-production/reclamation contrast or broader integration/coarser recognition. Revisit the strongest ready alternatives explicitly. Finite changed queries do not answer surviving active-state reclamation.

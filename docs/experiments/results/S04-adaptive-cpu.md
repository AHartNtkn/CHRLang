# Adaptive restoration: CPU variation remains consequential

This comparison does not establish an adaptive-policy advantage. Backoff-1 loses to Copy in one first-answer scenario; the other 65 registered contrasts remain unresolved. CPU time varies almost as much as elapsed time, so excluding elapsed interruptions would not explain away the uncertainty.

**Next price checkpointed replay against competent restoration controls.** That changes how much work is preserved or reconstructed. Repeatedly timing unchanged adaptive policies has less immediate decision value. Broader adaptive policies, independence, reconnection and sustained ownership remain unanswered.

## What was compared

The [registration](../registrations/S04-adaptive-cpu.md) fixes eight modes and six scenarios before measurement. Modes are Copy, initial reunion, eager repeated separation, scheduled EveryBoundary, fixed skips 1 and 8, and failed-check backoff caps 1 and 8. The sources cover plain, history-sensitive and late work, depths 0/4, one/four changing queries, exhaustion and first-answer cancellation. All observed answers are retained for independent checking after producer disposal.

The campaign completed **3,072 comparison processes, 192 warmups and five clock controls**. Serial children were pinned to CPUs 0 and 2, verified as distinct physical cores. Sixteen paired blocks per core rotate each mode through every position twice; the adjacent ordinary/CPU build order is randomized. No process reached the 60-second, 1-GiB or 200,000-service-call limit. The workload campaign took 126.2 seconds.

Primary measurements use ordinary allocation and elapsed clocks without engine work counters. A separate build adds Linux thread-CPU clocks. It preserves the source, engine policies and existing operational service bound. Source construction, preparation, input construction, setup, first observation, remaining execution/observation and all disposal phases are timed separately. Execution and observation within service remain combined; the lifecycle total is a sum of these intervals, not gross process time. Oracle work, preflight, report construction and process startup are outside that total. Compilation is not priced.

## Findings and their limits

| Question | Observation | Architectural consequence |
|---|---|---|
| Does adaptive backoff earn a clear timing gain? | No gains under the registered separation rule; 65 of 66 contrasts unresolved. | No adaptive policy is selected. Unresolved is not equivalent performance. |
| Is there a clear adverse case? | Late work, depth 0, four queries, cancellation after each first answer: backoff-1/Copy median lifecycle ratios are 1.438 on CPU 0 and 1.429 on CPU 2. Every paired ratio exceeds 1.10. | Copy is the stronger measured control for this bounded contrast. The smallest ratio is 1.101, close to the rule's boundary; retain that qualification. |
| Can time off the executing thread explain the spread? | Median within-cell max/min is 1.588 for CPU time and 1.587 for elapsed time. | Active CPU variation remains. These clocks do not isolate frequency, caches, allocator behavior or lower-level work. |
| Are there elapsed interruptions? | 14 of 1,536 diagnostic service samples have registered material elapsed excess. No sample meets the stricter elapsed-only excursion rule. | Some samples contain substantial additional elapsed time, but the large excursions also have elevated CPU time. Preserve them all. |
| Does adding CPU clocks visibly perturb the aggregate? | Instrumented/ordinary cell-median service ratios range from 0.951 to 1.026, median 0.995. | This describes this interleaved sample. It does not prove that instrumentation has no effect. CPU results remain diagnostic. |

A gain requires **every** paired lifecycle ratio below 0.90 on both cores; a loss requires every ratio above 1.10 on both. These are prospectively chosen observed-repeat criteria, not population confidence intervals. The stringent rule leaves uncertainty visible; it does not establish an absence of useful differences.

All 96 CPU diagnostic cells clear the registered clock floor. The largest empty-pair medians are 181 ns for CPU and 13 ns for elapsed time. Every service aggregate exceeds 100 times its interval count times the corresponding floor. Tiny preparation/disposal intervals are not independently ranked. CPU brackets surround elapsed brackets, so small negative elapsed-minus-CPU differences are expected clock overhead, not negative waiting time.

## The largest excursions contain two effects

**Run 1141 combines increased CPU time with about 3 ms of additional elapsed time.** Scheduled execution on the shallow plain source takes 3.845 ms of service elapsed time versus 0.848 ms of CPU time. Those are 7.714 and 1.702 times their respective cell medians. The first-observation phase accounts for 2.998 ms of elapsed excess.

**Run 2782 also combines both effects.** Backoff-8 on history/depth4/first-answer cancellation takes 14.233 ms elapsed and 11.235 ms CPU, respectively 2.000 and 1.578 times the medians. Its first-observation phase accounts for 2.998 ms of elapsed excess. Run 1267, by contrast, is roughly 1.96 times its elapsed median and 1.97 times its CPU median, without positive aggregate elapsed excess.

The [phase reconstruction](s04-adaptive-cpu/excursion-phases.json) is post-registration descriptive attribution. It introduces no sample exclusion or new policy decision rule. Similar approximately 3-ms gaps do not identify an operating-system cause.

## Validation and reproducibility

The [auditor](../../../research/chr-restoration/experiments/audit_adaptive_cpu.py) reconstructs every result from raw process output, checks frozen source/binary/schedule hashes, validates phase sums and CPU pairing, and verifies balanced positions and stable answer/service counts. The runner checks complete independent oracle answers outside timed intervals and checks actual retained answers after all producers are disposed. Cancellation validates the delivered answer and endpoint; it does not pretend the cancelled search exhausted.

The engine, repeated-source fixture and reunion implementation match the prior pilot's recorded hashes. Review of the runner confirms static engine dispatch, independent preflights and separate diagnostic clocks. Fresh package tests pass with and without `cpu-clock`: 32 tests in each build. Strict Clippy passes for the CPU-clock example. Test builds use a separate target directory and do not replace frozen measurement binaries.

Reproduce the audit with `python research/chr-restoration/experiments/audit_adaptive_cpu.py`. The [freeze](s04-adaptive-cpu/freeze.json), [schedule](s04-adaptive-cpu/order.json), [raw results](s04-adaptive-cpu/results.jsonl), [audit](s04-adaptive-cpu/review-audit.json), [clock controls](s04-adaptive-cpu/clocks.json) and [campaign receipt](s04-adaptive-cpu/audit.json) preserve inputs and observations. The freeze identifies the measured sources and binary hashes; it is not an archived operating-system image. The driver refuses to overwrite a frozen run.

## Next decision: price preservation versus reconstruction

The [resident replay source gate](S04-resident-replay-source-gate.md) already demonstrates much less reconstruction with frequent checkpoints. It does not measure whether snapshot creation and retained ownership repay those savings. Compare Copy, undo, resident replay, checkpoint intervals 1/4/16 and applicable persistent/COW controls on read-heavy, mutation-heavy, work-between-choice and deep-choice sources, with changing-query reuse and cancellation.

Current `lifecycle.rs` can supply sources and controls, but its measured answers are validated before engine disposal, and source/query construction is outside its lifecycle intervals. Qualify the complete ownership endpoint and those measurement boundaries before registering comparative costs. Do not silently reuse its totals as the broader lifecycle measurement.

T072 relevant-read validation is the strongest distinct alternative: it could avoid owned-key reconstruction but requires a new sound dependency mechanism. Restoration's existing independent source correspondence and controls make the next bounded ownership/cost comparison less expensive to reach. Reconsider integration at that result boundary, with demand capability and compact solving retained by the [sequence](../next-cycle.md). This result resolves neither T077 nor the architecture goal.

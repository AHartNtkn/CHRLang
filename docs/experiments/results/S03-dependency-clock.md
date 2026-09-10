# Demand clocks preserve answers and allocation; confirmation needs controlled repetition

The first-owned-answer clock preserves the qualified ownership behavior and complete source endpoints. Execution intervals exceed the registered timer-scale threshold. Unpinned session timings nevertheless vary too much to support speed conclusions from five samples, so the next comparison uses pinned CPUs and balanced repetitions.

## The observation being timed

The runner samples the first timestamp after materializing an answer into its consumer-owned batch. It starts that clock at execution entry, after preparation and query setup. Compiled execution includes `observe()` before the timestamp; demand execution includes receiving its owned answer. The first latency is reported separately and is not added to execution time.

Clock selection is explicit through `DEPENDENCY_FIRST_CLOCK=on/off`, defaulting to off. No clock check is added to progress-only scheduler turns. The off path performs no inner clock reads. The on path reads at collection entry and first answer publication. Independent answer validation still occurs outside measured intervals.

## Correctness and allocation qualification

Eighty cells cover all eight modes, five source families, sizes 8/128, original arrival and immediate release. Each runs twice with the meter for clock-on and clock-off, and once with the ordinary allocator for each clock mode: 480 processes. Every endpoint matches the prior ownership result. Both allocation repetitions and both clock modes reproduce every prior allocation/deallocation count, requested byte total and baseline-relative live/peak value exactly.

Another 32 ordinary processes cover known-hit and delayed-miss cancellation at size 8, all eight modes, retained windows and retained-all answers. Canceled queries without an answer have a null first timestamp; completed queries have one no later than the enclosing execution/observation interval. They preserve the prior endpoint signatures. Retained answers remain independently checked after producer/preparation disposal.

The five-repetition sizing sample adds 200 ordinary processes across 40 cells: five families, sizes 8/128 and current/current-miss/Scan/Indexed modes. All complete answers still match independently. Five separate clock calibrations and one allocator self-check bring the registered qualification/sizing total to 718 processes. No cutoff or allocation disagreement occurs. The two subsequent pinned clock checks test affinity feasibility separately.

## Timer scale and observed variability

| Quantity | Observed result |
|---|---:|
| Empty intervals per calibration | 100,000 |
| Median empty interval across five processes | 12–21 ns |
| 99th-percentile empty interval | 22–23 ns |
| Largest empty-interval outlier | 125,330 ns |
| Smallest execution/observation interval in sizing | 3,611 ns |
| Smallest complete measured session | 61,175 ns |
| Smallest individual phase | 25 ns |
| Median within-cell session maximum/minimum ratio | 1.49 |
| Largest within-cell session maximum/minimum ratio | 2.49 |

The smallest execution is about 172 times the largest median empty interval, exceeding the preregistered factor of 100. This establishes adequate scale for the intended execution comparison; it is not a bound on interruption noise or proof that instrumentation has zero cost.

Individual tiny phases can be almost entirely timer overhead. They remain charged in the complete session total, but isolated nanosecond disposal rankings would not be credible. The timer's long outlier and session variation also rule out treating the five unpinned samples as a confirmation or subtracting a universal timer constant from each result.

CPU affinity checks succeed on logical CPUs 0 and 2, which expose distinct physical core IDs. Their pinned empty-interval medians are both 12 ns, with 99th percentiles of 17 and 15 ns. This verifies feasibility, not that pinning eliminates all variance or that results generalize to other hardware.

## Selection at the fourth-package review

The [breadth review](S03-dependency-clock-breadth-review.md) selects the [registered lifecycle confirmation](../registrations/S03-dependency-timing.md): two pinned CPUs, balanced mode positions and repeated complete sessions. The primary endpoint includes the measured source/preparation, four changing queries, owned observations and disposal. It still excludes independent validation, process startup, parsing and native compilation; no all-lifecycle architectural superiority can follow.

T076 reusable symbolic solving remains the next distinct investigation. Finishing the now-qualified time comparison is justified because it can determine whether large work/allocation savings actually pay against competent access controls. Further refinement or precision must be reconsidered against T076 and T072's relevant-key repair at that result or a concrete obstruction. The goal remains active.

## Evidence

[Registration](../registrations/S03-dependency-clock.md), [runner](../../../research/chr-direct-conditional/examples/dependency_ownership.rs), [qualification driver](../../../research/chr-direct-conditional/experiments/dependency_clock.py), [frozen inputs and binaries](s03-dependency-clock/freeze.json), [raw runs](s03-dependency-clock/runs/), [qualification](s03-dependency-clock/qualification.json), [exploratory sizing](s03-dependency-clock/sizing.json), [independent audit](s03-dependency-clock/review-audit.json), [affinity checks](s03-dependency-clock/affinity-check.json) and [Clippy](s03-dependency-clock/clippy.log).

Run `python research/chr-direct-conditional/experiments/dependency_clock_audit.py` to recheck the raw results and ownership equivalence. Historical ownership auditing uses its preserved frozen runner; this experiment supplies the separate current-run comparison.

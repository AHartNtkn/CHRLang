# Clock calibration makes small phase timings unsuitable for primary comparisons

Many short lifecycle phases are close enough to measurement overhead that their isolated timings cannot support architecture decisions. Per-answer clock instrumentation also warrants a paired check before whole-service confirmation. Keep these phase records diagnostic and qualify primary timing without repeated serialization clocks.

## The measured clock costs

The registered calibration runs 42 independent processes: seven repetitions for each of three clock families on each of two allowed CPUs. All 420,000 empty intervals, baseline-loop checks, process order and reported CPU checks pass. Rust uses `Instant`, native C uses `CLOCK_MONOTONIC`, and Python uses `perf_counter_ns`, matching the current runners.

| Clock family | Empty-interval median, both CPUs | Median extra bulk cost per pair, CPU 0 / CPU 1 | Registered screening threshold |
|---|---:|---:|---:|
| Rust | 12 ns | 24.1 / 24.5 ns | 5.370 µs |
| Native C | 12 ns | 24.3 / 24.1 ns | 3.129 µs |
| Python | 30 ns | 87.3 / 88.3 ns | 9.749 µs |

The bulk estimate compares loops with and without clock pairs. It includes bookkeeping and effects on loop optimization; it is not a universal clock constant. The threshold is 100 times the largest observed per-process p99 empty interval or nonnegative bulk extra cost per pair. This is the prospectively specified conservative 1% screening heuristic, not an error bound or confidence interval. The highest Rust bulk sample increases its threshold relative to native C despite similar medians.

## Which existing observations are sensitive

Applying that screen to the existing qualification records flags 1,368 of 2,286 Rust query setups and 2,252 query disposals. All 2,286 Rust observations containing answers flag for accumulated serialization time relative to their answer count. Source preparation does not flag in any of the 267 admitted Rust sessions.

For native execution, 167 of 479 query setups and 431 observer setups flag. Every pending-drop and export interval flags, as do 407 of 413 observations containing answers when serialization is scaled by answer count. Tiny cleanup intervals can correctly do very little work; being flagged does not demonstrate a defect in the underlying operation.

Python emission, artifact writing, query encoding, native invocation and consumer assembly do not flag in the 26 combined sessions. Input and consumer release always flag. These counts describe measurement sensitivity in the existing corpus, not comparative gains or losses.

## Why whole-service instrumentation needs a check

An explicitly exploratory follow-up estimates clock density by multiplying the calibration scale by answer count plus two boundary pairs, then dividing by recorded service time. It exceeds the 1% screen in 997 of 2,286 Rust observations and 50 of 479 native observations. This estimate is not measured overhead: it uses a separate process calibration, ignores context-dependent costs and can exceed service time on a zero-work cancellation.

The result is enough to require a paired instrumentation check before confirmation. Primary service timing should measure execution plus serialization jointly, with only the first-publication and whole-service boundaries. Separately instrument serialization for attribution. Preserve complete bytes, service budgets, cancellation and ownership; compare diagnostic and primary builds on the same sources to establish the effect of changing clock density. Do not subtract an estimated clock cost from measured engine time.

## Decision and scope

The current runners remain qualified for semantics, ownership and diagnostic phase accounting. Their fine-grained timing is not qualified as a primary architectural comparison. T078 next removes repeated per-answer clock calls from the primary path, validates exact outputs against the diagnostic path, and measures that instrumentation effect before cost registration. Separate allocation diagnostics remain required.

This changes the measurement plan, not the preferred language architecture. At full qualification, reconsider one bounded mixed-source pilot against broader source analysis as required by the [sequence](../next-cycle.md). Compilation, external streaming, language breadth, local ownership and every other consequential mapped direction remain open.

## Reproduction and interpretation

The [registration](../registrations/S10-clock-calibration.md) fixes CPU 0/1, seven blocks, seed 20260910, 1,000 warmup pairs, 10,000 empty samples and 100,000 bulk iterations per process. Bounds are ten seconds wall, five CPU seconds and 256 MiB address space. The diagnostic whole-service density calculation is post-calibration exploratory attribution, not a preregistered confirmatory result.

[The probes and analysis](../../../research/chr-hvm/clock_calibration/) are reproduced with `run.py` followed by `analyze.py`. [Raw evidence](s10-clock-calibration/) includes every sample, compiler commands/versions, source and binary hashes, screening counts and the exploratory formula. Probes use ordinary allocation and do not execute an engine. No runtime, allocator, workload weight or architecture ranking is inferred from their differences.

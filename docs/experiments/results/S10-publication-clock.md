# Primary service timing no longer clocks every answer

Primary Rust and native timing now includes serialization within whole service and clocks only first publication. Separate diagnostic builds retain per-answer serialization clocks. Complete answers and work remain unchanged; the bounded pilot establishes no practical speed difference between these clock configurations.

## Correctness and ownership evidence

Both configurations pass 338 Rust corpus processes: 4,572 admitted query checks in total, with 1,174 explicit exclusions. Another 130 queries replay substantial cancellation and retained-consumer preparation reuse. All complete bytes, service calls, exhaustion flags and wire capacities match the archived qualified runner.

Both native configurations replay all 26 prepared sessions, totaling 958 query checks. Answer bytes, service calls, pending/unsupported status and dynamic graph-word counts match. First-observation timestamps exist exactly when an answer is emitted and remain inside service time. The native engine source and answer serializer are unchanged; the harness changes only clock placement and diagnostic reporting.

Primary serialization and derived compute-only fields are `null`, meaning unmeasured. They are not zero-cost claims. Diagnostic fields retain their nested arithmetic. Named lifecycle totals still count service once, including observation and serialization.

Rust selects the diagnostic configuration through the `serialization-clock` feature; `--no-default-features` is primary. Native builds use `SERIALIZATION_CLOCK=0/1` over the same harness. Compile-time conditions prevent primary per-answer clock pairs; both paths retain the first-observation clock. Engine/kernel metric settings remain off and allocation remains ordinary.

Scoped strict Rust Clippy passes in both configurations, formatting passes, and native builds pass the existing `-Wall` warning policy with warnings treated as errors. The executable RED check demonstrates that the prior runner reports measured serialization where primary requires an unmeasured field. Prior runner and gate source snapshots preserve that evidence.

## The instrumentation pilot

Selection used finite answer counts, not observed timings. Each Rust mode received its highest-answer-count source and first single-answer source when available; native received corresponding high-count and single-answer sources. The qualified mode11 finite-phase corpus contains only four-answer complete cases. Its missing single-answer control was recorded before the pilot, and its high-count witness remains included. This is a corpus limitation, not an impossibility claim about the solver.

The resulting 27 configurations run five primary and five diagnostic repetitions on each of CPU0/CPU1: 540 pilot processes plus 108 excluded warmups. Every output is independently rechecked. All 54 configuration/CPU comparisons are unresolved under the registered screen of disjoint observed ranges and a 10% median difference. No practical speedup or slowdown is established.

This finding does not prove clock overhead is zero. It bounds the evidence from this pilot, which includes compiled-code layout and measurement variability. Primary uses fewer intrusive clock calls because that is the intended measurement boundary, not because this sample proves an engine faster. More precision about this difference would not change that choice, so another clock-tuning matrix is not selected.

## Breadth review and next investigation

Five successive packages have qualified Rust lifecycle, host preparation, combined host execution, clocks and the primary/diagnostic split. The measurement path now has credible semantic controls. Allocation diagnostics and a coherent mixed-source cost pilot remain unfinished under T078.

The strongest ready alternative is compatible-query learning within source-derived finite solving, under T073. The [finite-phase gate](S06-finite-phase-gate.md) supplies a checked publication boundary and independent source/caller controls; the [lifecycle evidence](S06-finite-lifecycle.md) supplies selective and adverse regimes. Learned information might avoid repeated solving across related queries rather than merely make the current executions cheaper. That could change what work a complete architecture needs.

**Select that distinct source-analysis investigation next.** Establish a validity gate for reusing learned failures across compatible queries, with changed domains, aliases and assumptions that invalidate the learning. Demonstrate actual avoided solver work. Compare recomputation, retained complete results and learned information separately; preserve raw multiplicity and the full consuming caller. A cached answer is not evidence that learning a reusable contradiction works.

This gate can proceed with existing correctness controls and does not require the remaining native allocation instrumentation. It has greater decision value now than further clock precision, and it prevents measurement preparation from indefinitely displacing a distinct execution mechanism. Review T078 allocation qualification and the mixed-source pilot at that gate or a consequential obstruction. Neither T078 nor the broader native/source-analysis directions are resolved by this change of order.

## Reproduction and limits

The [registration](../registrations/S10-publication-clock.md) records the two configurations, full corpus, cancellation schedules, pilot selection amendment, repetitions, affinity, bounds and interpretation. [Scripts](../../../research/chr-hvm/publication_clock/) build, qualify, select, run and audit the experiment. Run `build.py`, `gate.py`, `pilot.py`, then `analyze.py`.

[Raw evidence](s10-publication-clock/) includes both corpus replays, retained cancellation, exact selected sources, all warmup/pilot observations, diagnostic analysis and source/binary hashes. Allocation, RSS, compilation costs and external streaming are not measured here. The combined host runner remains a diagnostic batch control; a subsequent whole-path pilot must explicitly select the qualified primary native binary and recheck that composition. No architecture ranking or research completion follows.

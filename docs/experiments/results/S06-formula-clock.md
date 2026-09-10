# Coarse retained-output timing qualifies seventeen matched scenarios

The timing path now disables assignment-attempt diagnostics at compile time and measures two coarse lifecycle intervals. Seventeen scenarios clear the registered signal threshold for all six controls. Seven other scenarios remain too short for a matched primary comparison; their allocation evidence remains valid.

## Why the timing interval changed

Fine-grained clocks can dominate no-op disposal or simple membership calls. The primary candidate therefore uses a preparation-through-producer-disposal interval and a final consumer-disposal interval. It retains every output, validates complete answers between those intervals, and excludes fine phase clocks and phase-report storage from the timed work.

This requires retained-all consumers. Immediate/window disposal cannot use this particular validation arrangement, so the coarse build rejects those modes. It measures complete atomic query results, not streaming first-value latency. Source/oracle fixtures, process startup, parsing, native compilation and fixed harness capacity are outside the endpoint. Those limits prevent a claim about complete architectural lifecycle superiority.

## Counter and ownership checks

The finite-name solver's assignment-attempt accumulator was unconditional. It is now compiled only with the existing metrics feature; the recursive argument is absent when metrics are disabled. Default and metrics-off tests preserve satisfiability, with positive diagnostic counts only in the enabled build. Operational bounds, parity/component arithmetic and freshness allocation remain because they affect execution or correctness.

The coarse clock configuration rejects compilation with metrics, allocation instrumentation or fine phase clocks enabled. The negative-build receipt confirms the metrics restriction. The timed binary uses the ordinary allocator. Existing source, graph, joint-theory and symbolic-transport tests provide semantic regression checks.

Qualification runs 144 cells across six modes, four families, two variable counts and three query counts. Each has two meter/off-clock runs, two meter/fine-clock runs, one ordinary/off-clock run and five coarse ordinary timing runs: 1,440 workload processes. Every diagnostic phase reproduces the preceding ownership allocation after normalizing its initial baseline. Complete retained answers validate in every coarse run.

## Signal and uncertainty

Five independent processes sample 100,000 empty intervals each. Their medians are 14, 21, 12, 21 and 20 ns. The prospective signal requirement is 100 times two intervals times the largest median, giving 4,200 ns per session. No overhead constant is subtracted and no outlier is excluded.

Of 144 cells, 137 clear that threshold at their minimum observation. The seven short cells are explicit-enumeration membership cases: all four three-variable one-query families, and the free/star/clique sixteen-query cases. Requiring every competitor to qualify leaves 17 of 24 scenarios. All eight 128-query scenarios qualify.

Timing still varies. The median within-cell maximum/minimum ratio is 1.49; the largest is 6.36. Qualification does not establish relative speed. The separately [registered primary comparison](../registrations/S06-formula-timing.md) uses paired balanced blocks on two pinned physical cores and rechecks the signal threshold on primary observations.

This completes the final selected qualification package. The [primary result](S06-formula-timing.md) records the bounded timing conclusion and the T077 handoff. Short operations, other consumer policies, streaming and broader source eligibility remain unanswered.

## Evidence

[Qualification registration](../registrations/S06-formula-clock.md), [runner](../../../research/chr-structural/experiments/formula_clock.py), [independent auditor](../../../research/chr-structural/experiments/formula_clock_audit.py), [frozen inputs/binaries](s06-formula-clock/freeze.json), [all raw runs](s06-formula-clock/runs/), [sizing](s06-formula-clock/sizing.json), [audited qualification](s06-formula-clock/review-audit.json), [metrics-off tests](s06-formula-clock/metrics-off.log), [metrics-on tests](s06-formula-clock/metrics-on.log), [negative metrics build](s06-formula-clock/negative-metrics.log) and [regressions](s06-formula-clock/regressions.log).

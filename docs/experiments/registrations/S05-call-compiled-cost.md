# Call traces against matched compiled execution

Registered after the source-order repair and before comparative measurements.
The question is whether the call-trace gains against Direct survive stronger
existing execution, and what requested heap ownership each requires.

Use the existing call_trace_ownership runner: trace, Direct, compiled Global
scan, Global indexed, and indexed with inferred specialization. The four source
families, depths 4/32/128, one query / four repeated / four distinct queries,
retained outputs 0/all, and complete / first-answer cancellation yield 720 cells.
The source-order gate covers both branch orientations; this cost pilot retains
its existing forward orientation. Prepare once per process and reuse across
changing query variables. Validate complete answers outside measured intervals
against Direct and the independent scalar evaluator, including cancellation
prefixes and output validity after preparation disposal.

First run two allocation-meter repetitions per cell, requiring exact requested
bytes, peak ownership, query-end ownership and consumer bytes across repeats.
Require equal consumer bytes and counts across all five configurations, continuous
phase ownership, and restoration of the initial heap root. Requested heap bytes
are not RSS. Freeze separate release binaries with default features disabled:
ordinary allocator for primary timing; alloc-meter only for heap diagnostics.
Do not overwrite earlier campaign binaries. No builds during measurements.

Then run ten randomized complete blocks of all 720 cells with seed 607096.
Record three existing 10,000-sample clock checks before primary runs. Each process
has 1 GiB address-space, 30 CPU seconds and 45 wall seconds. Preserve failures;
any validation failure stops the campaign for repair, with no excluded samples.
Total bound is 1,440 heap and 7,200 timing processes (plus three clock checks).

Primary cost sums source construction, preparation, source disposal, query input,
setup, service plus observation, output consumption, query-engine and input
disposal, preparation disposal and consumer disposal. Semantic validation is
outside intervals. Report each phase and the complete sum; first observation is
the first service_observe interval. Process startup and Rust compilation are not
in that sum, so this does not decide whole compiler lifecycle superiority.

Compare trace with each control by scenario, without workload weights. A time gain
requires median paired ratio <=0.9 and all ten ratios <1; loss requires median
>=1.1 and all ten >1. Otherwise report unresolved magnitude/direction. Both median
costs must exceed 100 times the largest clock-check p99. Report all heap ratios
and coupling to timing. No universal winner. If compiled controls erase the
scoped Direct gains, prioritize the resulting execution/reuse tradeoff over
further tuning of a weak comparison. If gains survive, use measured retained
state to choose bounded retention versus broader caller interleaving next.

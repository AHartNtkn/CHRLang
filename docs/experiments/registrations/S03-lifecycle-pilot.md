# S03 first lifecycle pilot: prospective comparison

## Decisions and hypotheses

Compare complete prepared execution organizations now that source gates pass. H1: on opaque common work, the direct graph or Conditional may avoid repeated execution sufficiently to repay context maintenance. H2: early discrimination and no-choice work may expose overhead that sharing cannot repay. H3: on word generation, checked graphless lowering may account for much of the benefit that otherwise appears attributable to choice graphs. H4: preparation, answer materialization or disposal may reverse an execution-only contrast.

This is a bounded pilot, not confirmatory architecture selection. No workload weights or universal winner will be inferred. The strongest ready alternative is S01's selective/consuming comparison; the present pilot is selected because it can now test a previously unavailable complete graph organization with competent source controls. Reassess after this pilot before further graph tuning.

## Exact cells and queries

Use six families. Word sources are binary a/b, nested a/(b/c), and duplicate a/a, without first-b rejection. Each query has one build, one correlated task and a distinct unknown token inserted first; no selected outputs. The four-query depth cycles are binary `[0,2,4,2]`, nested `[0,1,3,1]`, duplicate `[0,2,5,2]`. Compare `global-scan`, `global-index`, `active-index`, `conditional`, `graph`, and `words` (checked graphless lowering) on these three families: 18 cells. Active is admitted only here, under its verified token-first premise.

The common-work families are `plain`, `shared`, and `discriminate` from the registered common-work source gate. Each uses depth cycle `[0,1,8,32]`. Compare `global-scan`, `global-index`, `conditional`, and `graph`: 12 cells. The plain family carries four unknowns; the others have sixteen complete four-field alternatives. The discrimination source selects a ground pack before the same work loop. No direct countdown compiler is inferred from the word compiler; the cost of such a compiler remains an S06 question.

There are 30 cells. Each process performs one unrecorded warmup cycle on a fresh prepared owner, disposes it, then measures a fresh prepared owner reused for four cycles (16 changing queries). No query state is reused. Complete expected answers are independently generated and checked outside timed intervals, preserving raw multiplicity and joint aliases. Reject any incorrect or unfinished run. Confirm each new maximum word depth against the independent source oracle before timing it.

## Lifecycle and instrumentation

The common starting input is an available source AST. Charge the actual preparation API, including AST copying when that API needs ownership. Source parsing and fixture construction are outside this comparison. Record preparation separately, then query setup (including the required owned query copy), execution plus complete answer materialization, engine disposal, answer disposal, and prepared-owner disposal. Record elapsed time to the first materialized answer from the start of query execution as an overlapping latency endpoint; never add it to total time.

Direct graph and Conditional service calls materialize answers internally. Therefore execution and observation are a joint interval for all engines. Compiled terminal-branch observation and terminal-branch disposal occur inside that interval; remaining search-engine disposal is separate. State these ownership differences when interpreting phases. Primary lifecycle cost is the sum of preparation, all query setup/execution/disposal intervals, and prepared disposal. Validation and receipt formatting are excluded. Consumer ownership retains all answers until validation, then disposes them before the next query.

Use a release example runner at `research/chr-direct-conditional/examples/s03_lifecycle.rs`, with all primary engine/kernel counters disabled and the ordinary allocator. Build it in an isolated time target with `--no-default-features`. Use a separate isolated target with `--no-default-features --features alloc-meter` for requested-heap diagnostics, using the existing allocation meter and its standalone self-check. Do not treat allocation-instrumented durations as primary timings. Record requested cumulative bytes, allocation calls, live endpoints and peak for each phase. They are not RSS. Do not report unavailable cross-engine operation counts as zeros.

Native generation and compilation of a new user ruleset are not isolated by this bundled build. No total cold architectural or compilation break-even claim follows. The word compiler's runtime schema check is charged as preparation; it is not a measurement of native code generation.

## Repetitions, freeze and bounds

Run five primary process repetitions per cell and two allocation process repetitions per cell: 150 time processes and 60 allocation processes. Within each repetition block shuffle all 30 cells with Python `random.Random(30301)` for time, `random.Random(30302)` for allocation, advancing the generator across blocks. Save the exact order before execution. Pin processes to the lowest available CPU in the measured affinity set and record that set; if pinning is unavailable, record the condition before runs and keep it identical across cells.

Before comparative execution freeze the runner, sources, dependencies, compiler version, resolved Cargo features, binary hashes, exact commands and machine information. Record clock/empty-wrapper calibration; do not subtract a guessed overhead. If a lifecycle interval is under twenty times the summed calibrated wrapper overhead for its measured phases, classify that timing as below useful resolution and prospectively increase a paired query-cycle count before confirmation.

Bound every measurement process at 60 seconds and 1 GiB address space. Bound each query at 2,000,000 service calls and 10,000 answers. A cutoff is not an exhausted search or a completed timing sample. Inspect its responsible phase and progress, then register a paired size/bound revision or a justified analytical limit. Do not silently skip the failing cell. Compilation has a separate recorded process handle and is not a query timing sample.

## Interpretation and follow-up

Publish every raw record and per-cell ranges/medians. Pair by repetition block. A provisional direction requires all five lifecycle ratios on the same side of equality and a median difference of at least 20%; otherwise call it inconclusive. This operational pilot criterion is not a statistical confidence claim. Report phase reversals, first-answer tradeoffs and allocation disagreements even when total time has a clear direction.

A graph loss triggers diagnosis of avoidable scan/materialization or retention costs before broad rejection. A graph gain must survive the relevant lowering control and be tied to work placement, not merely fewer demanded outputs. Inconclusive differences matter only where their plausible resolution would change a bounded choice; use sensitivity or more evidence accordingly. Do not expand the matrix reflexively. At the checkpoint compare the value of any repair with the strongest ready S01/S02/S06 alternative. S03's other graph/derivation mechanisms and S08 lifetime obligations remain unresolved by this pilot.

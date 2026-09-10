# Learning has an ordinary-allocator measurement path

Primary and diagnostic learning builds preserve the same complete outcomes and cancellation/reuse behavior. The primary build disables learning diagnostic updates at compile time and uses the ordinary allocator. Comparative learning timings have not run.

## What is qualified

One source runner now supports primary and diagnostic release builds. Learning probe, exclusion and installation updates sit behind a constant generic parameter, false in the primary build and true only when `learning-diagnostics` is enabled. A direct paired semantic test verifies zero primary diagnostic counts, active diagnostic counts, identical source work and the same retained regions. The runner rejects enabled compiled, observation or conditional engine metrics.

Operational source-step, partition and term budgets remain active because they determine limits and service behavior. They are not presented as removable diagnostic overhead. The allocation meter is compiled only into the separate diagnostic build; primary allocation readings and baseline are `null`, rather than zero. The reference interpreter is unchanged.

The runner records source construction, finite and caller preparation, learner construction, input construction, query setup, finite service, session disposal, caller transport/execution/owned observation/disposal, input cleanup, and disposal of every retained engine and consumer owner. Finite service and session disposal now have separate intervals. Caller transport, execution, observation and local disposal remain a joint phase.

First-answer timing ends when an `Answer` is owned by the internal consumer and starts before query construction. It includes query construction and finite service. It is not external publication latency: JSON is written after execution and validation, and does not serialize the complete answers. This endpoint cannot silently substitute for the common Rust/native answer-wire endpoint.

## Independent qualification

The complete-query matrix has 216 configurations: three policies, four accepted pair relations, duplicate weights one/two, capacities zero/one/four and follow-up counts one/four/sixteen. Another 36 configurations cancel the first wider follow-up after one finite progress event and then continue using the same preparation and learner. These cover all policies, accepted relations and capacities with weight one and four follow-ups.

Each configuration runs once in the primary release build and twice in the diagnostic release build: 756 processes total. All completed queries agree with independent scalar and compiled raw observations; analytical enumeration checks cardinality and multiplicity. First-owned-answer presence agrees with the independently expected nonempty answers. Primary and diagnostic retained-region counts agree.

Cancellation publishes no complete finite result, preserves the retained count and diagnostic installation count, and is followed by correctly answered queries. Checking installation count also detects a replacement at full capacity. Both diagnostic repetitions have identical allocation phase signatures. Every diagnostic run restores its task-owned live requested-byte baseline after all owners are disposed. The meter self-check passes.

Both feature builds pass 34 relevant tests. Strict scoped Clippy passes for primary and diagnostic runner configurations, and formatting passes. The [audit](s06-learning-primary/audit.json) records exact binary and source hashes. Builds use `--release --no-default-features`, with `experiment` for primary and `alloc-meter,learning-diagnostics` for diagnostics.

The allocation replay script now takes explicit `--binary` and `--output` paths and ignores timing fields when checking allocation repeatability. A separate [432-process replay](s06-learning-primary/ownership-script-check.log) validates that command against the diagnostic binary.

## Limits of the evidence

The elapsed values in the qualification logs are not comparative evidence. These runs were selected for correctness and ownership, without randomized timing blocks or a frozen practical timing threshold. Many sources are small enough for clock overhead to matter. A cost pilot must register sizing, repetitions, ordering, thresholds and cutoff interpretation prospectively.

Reserved record and outer-consumer buffers, validation and JSON formatting are outside the named intervals. Complete inner answers and their disposal are charged. Named interval sums are not complete process time. Compilation, external publication, common-prefix depth scaling and sustained consumer behavior remain outside this gate. Cancellation here occurs during the private finite phase; caller-stage cancellation is separate work.

## Selection at the qualification boundary

Resume T078's host/native allocation qualification and bounded mixed-source coherent-path comparison. This is a prioritization judgment, not a new experimental result. The strongest competing next task is a learning cost pilot with common-prefix depth and reuse variation. Its primary path is now qualified, but its current source matrix has little substantive common work; extending it would require another source/measurement package before a credible architectural timing conclusion.

T078 can instead test whether the existing coherent paths retain their advantages across matching, equality, choices and complete observation. Its qualified source and answer gates make that a broader consequential comparison, provided host/native ownership and the primary binary composition are checked. The next T078 package must identify and qualify those missing boundaries; it must not restart general runner development without an invalidating accounting defect.

T073 remains unfinished. Reconsider its registered cost pilot at T078's allocation qualification boundary or a consequential obstruction, and again after the bounded mixed-source pilot. Direct resource derivations, general conflict learning and independent native compilation retain their distinct obligations. Neither this gate nor that pilot completes the research goal.

## Evidence

- [Prospective qualification registration](../registrations/S06-learning-primary.md)
- [756 raw processes](s06-learning-primary/runs.jsonl), [audit and hashes](s06-learning-primary/audit.json), [process gate](../../../research/chr-hvm/learned_regions/primary_gate.py)
- [Metrics-off semantic tests](s06-learning-primary/gate-off.log), [default semantic tests](s06-learning-primary/gate-default.log)
- [Primary Clippy](s06-learning-primary/clippy-primary.log), [diagnostic Clippy](s06-learning-primary/clippy-diagnostic.log), [meter check](s06-learning-primary/meter-check.log), [toolchain](s06-learning-primary/toolchain.log)
- [Primary release build](s06-learning-primary/build-primary-release.log), [diagnostic release build](s06-learning-primary/build-diagnostic-release.log)

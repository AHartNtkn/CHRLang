# Interrupted queries and artifact lifetime gate

Add an optional cancellation service budget to the existing artifact runner.
When present, interrupt even-numbered queries after the budget and complete each
following changed query with the same preparation. A cancellation produces no
answer; report whether execution had already exhausted, without treating a partial
query as validated source completion. Measure setup, serviced work and engine
disposal, and check requested live-byte restoration in diagnostic builds.

Use payload's seven modes and subscription's ten modes, sizes 0 and 16, cancellation
budgets 0 and 3, four alternating interrupted/completed queries, two repetitions,
and ordinary/metered builds: 272 processes, 544 interrupted lifetimes and 544
independently checked complete queries. Zero-budget interruptions must be pending;
record any completion before the three-step endpoint. Equal service budgets do not
mean equal work across engines and supply no cross-engine cancellation ranking.
Require exact diagnostic phase replay, query/prepared live-byte restoration and
complete following-query agreement with the independent source oracle.

Compile generic, payload and subscription source artifacts in a dedicated target
for each build. Once all processes using them have exited, measure filesystem
unlink of their source and executable files, separately from generation/compilation
and runtime sums. Keep hashes and source-generation commands in receipts. Verify
these task-owned paths are absent afterward. This is host filesystem syscall cost,
not durable-storage synchronization or a promise of immediate kernel cache release.
The shared runtime remains reusable and is not charged to per-ruleset disposal.

Limits: each build/compile 60 seconds and 4 GiB address space; each runtime process
60 seconds and 1 GiB; completed queries two million source steps. One ordinary pair
and diagnostic pair per cell are correctness/accounting evidence, not comparative
timing. Investigate any leak, resumed-query disagreement or cutoff. After this gate,
register the full cost comparison rather than another unrelated executor refinement.

# T070 lifecycle sizing before comparative confirmation

Use the validated ordinary-link and ThinLTO artifacts to establish feasible query
sizes, reuse counts and phase magnitudes. This is one exploratory repetition per
cell. It does not establish speedups, crossover thresholds or architecture rankings.

Run chain, private payload, subscription, dispatch16 and dispatch64. For each, use
base sizes 8 and 32 and query counts 1 and 16, alternating the existing changed-query
inputs. Run generic, prepared-plan, prepared-plan plus inferred specialization,
native generated repair, native generic repair, native prepared repair and native
plus inferred specialization, under both link configurations. This gives 280
processes and 2,380 independently checked queries if all cells finish. In dispatch
sources, size selects the first matching rule modulo rule count; it is not store
size. The chain and subscription sizes count edges or joined paths, while payload
size counts opaque payload variables. These axes must not be pooled as equal work.

Reuse the fixed bitcode runtime from the link-optimization gate. Emit dispatch64
without queries, and compile it once in each configuration with the same flags.
Verify existing binary and shared-library hashes first; record new source hashes,
commands, compilation time and binary sizes. Execute the matrix in a seeded shuffled
order (seed 7001). Record process wall time and child CPU separately from the sum
of runtime intervals. Runtime timing has ordinary allocation and all counters off.
Every answer must match the independent scalar source executor; require truthful
exhaustion and exact phase totals. Verify both first and changing queries.

Limit each compilation to 60 seconds and 4 GiB address space, each process to
60 seconds and 1 GiB, each query to 2 million source steps. Preserve every cutoff
or failure receipt. Continue independent cells after recording a cutoff; inspect
its responsible phase before proposing confirmation sizes. No completed-cost
interpretation is allowed for an unfinished cell.

Report the largest phase magnitudes and any resource/validation failure. Do not
rank the single timing observations. Use this screen to select a prospectively
registered pilot after allocation and source-equivalent retained controls are
available. The current generic/native modes share the same matching source, but
these seven modes alone do not answer whether retaining matches is preferable.
The pilot must also address cancellation and artifact lifetime; the current runner
accounts for completed query and prepared-state disposal, not those endpoints.

Sizing gets priority over a new architecture implementation because it checks the
cost comparison's feasible region using already validated executables. A cutoff
that reflects harness/oracle work must not be attributed to engine cost. Direct
pull-tabbing and derivation reuse remain required and return to selection after
the first complete T070 cost contrast.

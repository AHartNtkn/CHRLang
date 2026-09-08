# The specialized control does not change the bounded ordering

Inferred specialization does not overturn the scalar advantage on these six sources. Dependency caching still offers allocation savings without a registered practical runtime improvement, and distinct requests incur a clear loss. Direct graph execution retains a favorable common-failure case with lower peak memory.

The [registered follow-up](../registrations/S05-specialized-control.md) completes **336 processes**, with exact allocation replay in all 48 cells. Every source-head predicate passes inferred eligibility, and all configurations pass complete independent and analytical answers.

## Comparable lifecycle results

Eight-query medians include prepared construction, query setup, complete execution/observation and disposal. The graph is direct choice-graph execution. Specialized is the inferred single-head compiled path, including its preparation cost.

| Source | Ordinary scalar, ms | Dependency cache, ms | Specialized, ms | Graph, ms |
|---|---:|---:|---:|---:|
| Before success | 1.276 | 1.217 | 6.476 | 13.518 |
| After success | 1.232 | 1.149 | 5.157 | 32.381 |
| Before clash | 0.633 | 0.607 | 3.337 | 0.537 |
| After clash | 0.802 | 0.808 | 3.940 | 27.063 |
| Trivial | 0.640 | 0.686 | 2.238 | 14.830 |
| Distinct requests | 1.341 | 2.081 | 5.376 | 32.356 |

**Specialization remains slower than ordinary scalar in every registered cell.** Eight-query paired Specialized/Ordinary medians range from 3.497 to 5.161, with all pairs above one. This supports the included scalar organization as a necessary competing control. It does not imply that interpretation intrinsically beats compilation: state ownership, source selection, terms and branch organization differ. Arena-COW, native compilation and other compiled organizations are not resolved by this follow-up.

**Dependency reuse still has a modest runtime effect on repeated work.** Before-success and after-success ratios against ordinary scalar are 0.949 [0.913, 0.960] and 0.955 [0.920, 0.991]. Both show a smaller effect than the registered 20% practical threshold. Before-clash crosses one; after-clash is near one. In contrast, distinct requests give 1.559 [1.465, 1.592], corroborating the first pilot's adverse result.

**The graph's common-failure advantage remains a tradeoff.** Before-clash Graph/Ordinary is 0.843 [0.778, 0.849], below one throughout but short of the registered 20% median threshold. Graph peak is 53.9 KiB, versus ordinary 73.8 KiB and dependency cache 79.0 KiB. Graph requests 1.154 MiB cumulatively, compared with dependency cache's 1.058 MiB: lower peak and lower allocation traffic choose different configurations. For one-query before-clash, Graph/Ordinary ranges from 0.755 to 1.489, so that timing remains uncertain. After discrimination the graph incurs much larger time and traffic. No workload weighting combines these regimes.

## Why fewer equation pairs did not yield a large lifecycle gain

The initial pilot's phase decomposition puts before-clash scalar setup at about 0.310 ms out of 0.624 ms for eight queries. Dependency caching primarily changes execution, from roughly 0.258 to 0.235 ms; it does not eliminate arena construction and input handling. Before-success execution changes from about 0.820 to 0.757 ms, while setup and observation/disposal responsibilities remain. These are phase descriptions, not a profiler attribution of every instruction.

The distinct-request overhead appears chiefly in execution: about 0.884 ms ordinary versus 1.566 ms with dependencies in the initial pilot. The recorded work shows that only choice equations hit there; all sixteen substantive requests still require unification and dependency construction. This supports the proposed mechanism explanation without claiming a measured instruction-level breakdown.

No timing ratio in this report mixes the original and follow-up freezes. Each cited ratio comes from its own randomized blocks. Allocation quantities agree across repeated diagnostics within each matrix.

## Next decision-relevant investigation

The current favorable test uses 64 constructor rows and sixteen requests per query. It establishes a real work/traffic benefit, but it cannot settle whether larger operations or more within-query reuse repay cache validity in runtime. Register an operation-size/request-frequency crossover next, including distinct-request pressure and the relevant ordinary/graph/compiled controls. Establish an actual favorable runtime regime or a measured sensitivity bound; do not infer that the cache never pays from this one size.

This is selected ahead of another compiled-storage refinement because it resolves the central S05 question of whether substantial operation reuse can substitute economically for direct sharing. A profile or additional storage control remains necessary before attributing the scalar/specialized gap to one responsibility. Binding-invalidation patterns, cache capacity/eviction, query lifetime and generalized tables remain open. No global cache policy or whole architecture is selected.

## Evidence

[Audit](s05-specialized-control/audit.json), [all endpoints and paired ratios](s05-specialized-control/summary.json), [phase table](s05-specialized-control/phases.csv), [freeze](s05-specialized-control/freeze.json) and raw process receipts preserve the comparison. Ordinary-allocator timing is counter-free; heap diagnostics run separately and do not measure RSS. Failure has zero answers and no first-answer latency. Native compilation, fixture generation and oracle validation are excluded from measured runtime phases. The [initial seven-configuration pilot](S05-lifecycle-pilot.md) supplies additional Direct, Exact and Conditional controls within its own freeze.

T067 remains active for the crossover and consequential attribution. Completing these matrices does not close S05 or the architecture goal.

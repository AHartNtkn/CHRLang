# Resumable matching helps stable history but still loses ground under invalidation

Resumable matching reduces stable-history and dense-propagation allocation to about the eager control's level, far below restarting demand. Mixed sources expose retained-state and invalidation costs. Sharing immutable head syntax with the prepared source reduces part of that overhead, but does not make resumable matching uniformly cheaper than eager execution.

The [registered lifecycle pilot](../registrations/S10-resumable-lifecycle.md) completed 4200 processes: 1200 allocation runs with 600 exact replay pairs, then 3000 ordinary-allocator timings. A separately [registered ownership attribution](../registrations/S10-resumable-head-ownership.md) adds 720 allocation processes and 360 exact pairs. Full observations and every query, consumer, cancellation and prepared-owner check pass in both. [Primary audit](s10-resumable-lifecycle/audit.log), [primary cells](s10-resumable-lifecycle/summary.csv), [ownership audit](s10-resumable-head-ownership/audit.log), [revised allocation cells](s10-resumable-head-ownership/summary.csv).

## Complete allocation after the ownership correction

The table uses n=3, four changed queries and immediate answer release. Stable/reset history contains 48 items; dense contains six identical occurrences and requires thirty ordered pair observations. Other rows are complete mixed sources with guards, bindings, consumption, branching and fresh outputs. Traffic includes preparation, inputs, execution/delivery, one-service cancellation and disposal. Peak is requested live heap above the initial baseline, not RSS.

| Source | Scan traffic | Eager traffic | Demand traffic | Revised resumable traffic | Eager peak | Revised resumable peak |
|---|---:|---:|---:|---:|---:|---:|
| Stable history | 605896 | 415389 | 4642717 | 408629 | 38915 | 35723 |
| Equality-reset history | 627784 | 7403197 | 4661693 | 4740693 | 43499 | 35979 |
| Dense duplicates | 315307 | 266114 | 2893030 | 264214 | 17136 | 15240 |
| Independent mixed branches | 776529 | 1435659 | 1711591 | 1765799 | 48287 | 62639 |
| Delayed binding | 995745 | 2064675 | 2734527 | 2628351 | 48487 | 62839 |
| Mixed history and branching | 2804281 | 6481719 | 12259027 | 7574675 | 150896 | 143982 |
| Deep outputs | 1475089 | 1845627 | 2121559 | 2175767 | 84626 | 85386 |

Stable history supports the intended mechanism: saved progress avoids repeated history probing with less peak than eager materialization. Dense duplicates preserve all thirty observations; the reduction is not deduplication of equal-valued occurrences. The [source/work gate](S10-resumable-contextual-gate.md) independently measures the reduced offers.

The mixed rows prevent extending that conclusion to the whole executor. Resumable independent execution retains more peak memory and allocates more than eager. Delayed binding improves over restarting demand but still allocates more than eager and Scan. Mixed history improves over demand while remaining more allocation-intensive than eager, with a lower peak. These are conditional tradeoffs, not a weighted ranking.

Filtered conditional execution remains a stronger control than its earlier unfiltered version, but is still costly here: 9632782 requested bytes on stable history, 5290365 on dense duplicates and 3428743 on independent mixed branches. These measurements do not resolve support-aware joining or all conditional organizations.

## The ownership attribution changes implementation responsibility

The initial cursor copied immutable head patterns into each continuation. Its position in the per-rule cursor array already identifies the immutable prepared rule. The revised cursor retains only counts, positions and environments; the executor supplies the same prepared head slices when resuming. No borrowed syntax is stored in the cursor, and the engine retains the prepared source while searches live.

This change is tested across all 360 eager/demand/resumable allocation cells. Every eager and demand memory reading remains exactly equal to its primary-pilot counterpart, not merely similar in total. Revised cursor observations, order, invalidation and owner checks pass. Baseline source snapshots are preserved and verified against the original hashes, so primary timings remain attributable to their actual implementation.

For independent mixed branches, resumable traffic falls from 1905181 to 1765799 bytes and peak from 69023 to 62639. Delayed-binding traffic falls from 2804021 to 2628351. Stable-history traffic changes only from 409209 to 408629. Immutable-head duplication therefore explained part of the mixed-source penalty, but the corrected cursor still has environment, branch-cloning and invalidation costs. The data does not justify attributing the entire initial penalty to necessary architecture.

[Baseline source snapshots](s10-resumable-lifecycle/source-snapshots.json), [primary freeze](s10-resumable-lifecycle/freeze.json), [revised freeze](s10-resumable-head-ownership/freeze.json), [revised source](../../../research/chr-relational/src/contextual.rs), [executor](../../../research/chr-relational/src/contextual_execute.rs).

## Equality invalidation is consequential

Adding a redundant self-equality after every history application increases revised resumable traffic from 408629 to 4740693 bytes. Eager rises from 415389 to 7403197, while restarting demand stays near its already high cost. The source/work gate shows all three offering 152 candidates for sixteen items in this regime.

The current boundary invalidates on any processed equality step, even when roots and logical information do not change. This is a conservative implementation policy, not an inherent requirement of contextual matching. A compiler could also eliminate syntactically trivial equalities before execution. Those are distinct stronger controls: runtime change detection can cover dynamic redundancy; source elimination can avoid the operation entirely. Neither saving should be assumed before its semantic and complete-cost comparison.

## Consumer ownership and time limits

Retained answers remain independently usable after prepared-source disposal. For four deep-output queries, Scan, eager and both cursor versions retain exactly 166400 consumer-owned bytes. Revised resumable peak is 210186 bytes, versus eager 209426 and Scan 299881. For stable history, all retain 40844 answer bytes; revised resumable peak is 66356 versus eager 74903 and Scan 74079. These are short bounded-retention results, not sustained-lifetime conclusions.

Primary timings are exploratory medians/ranges from five ordinary-allocator processes per cell. Before the ownership correction, stable history has resumable median 362952 ns (230349–369527), eager 394090 (281332–449364) and demand 967745 (957242–1518475). Independent mixed branches have resumable median 1330496 (1005136–1704943), eager 1012131 (657655–1123453) and Scan 768964 (645610–889959). No formal practical-win classifications or workload-weighted averages are assigned.

The revised cursor has allocation evidence only. Do not transfer the primary time samples to it or infer a revised timing ranking. Both stages charge actual retained state and disposal, but exclude native compilation, process startup, original source AST construction, validation and preallocated bookkeeping. Execution and observation remain coupled in delivery. [Toolchain](s10-resumable-lifecycle/toolchain.txt), [hardware](s10-resumable-lifecycle/hardware.txt).

## Validation and next investigation

Strict Clippy passes for both primary runners and the revised allocation runner. The revised code passes sixteen relational tests in diagnostic and counter-free builds, including exact work counts, and the broad mixed/progress gates with independent complete observations. [Primary allocation Clippy](s10-resumable-validation/clippy-meter.log), [ordinary Clippy](s10-resumable-validation/clippy-time.log), [revised Clippy](s10-resumable-validation/head-clippy.log), [counter-free semantic checks](s10-resumable-validation/head-ownership-tests.log), [diagnostic checks](s10-resumable-validation/head-work-tests.log), [composition](s10-resumable-validation/head-composition.log).

Recent discovery packages now provide source/work gates and full-cost pilots for filtering and resumption. The breadth review selects [equality invalidation versus source elimination](S10-equality-invalidation-entry.md) as one bounded stronger-control investigation. The large reset penalty can reverse the traffic comparison with Scan, so it is more consequential than another small cursor-layout refinement. The strongest ready alternative, support-aware conditional joining, remains required and is reconsidered after this control gate.

No universal discovery strategy or architecture is selected. Broader language properties, sustained lifetime, connected parallel work and held-out coherent-architecture challenges remain required. T078 and the research goal remain active.

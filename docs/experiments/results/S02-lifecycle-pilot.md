# Relational joins help selective constructor access, but broad discovery is expensive

The relational candidate beats the existing integrated engine on immediate selective constructor matches. It loses to the generic dedicated controls on every tested family, and broad matching exposes expensive repeated candidate enumeration. These results justify investigating that concrete cost before judging relational integration as an architecture.

All 140 registered processes completed with correct answers: 100 ordinary-allocator timing runs and 40 separate allocation runs. The 20 cells' allocation diagnostics replay exactly. There were no process or service cutoffs.

## Measured lifecycle comparison

Each process prepares a ruleset once and runs 16 changing queries: sizes 4, 16, 64, 16 repeated four times. Times include preparation, query setup, execution with observation, engine and answer disposal, and prepared disposal. Native compilation, process startup, fixture construction and independent validation are outside these intervals.

| Source family | Relational, ms | Integrated, ms | Dedicated indexed, ms | Dedicated scanned, ms |
|---|---:|---:|---:|---:|
| Flat joins | 3.840 | 0.825 | 0.888 | 0.578 |
| Immediate selective constructors | 1.734 | 2.874 | 0.715 | 0.450 |
| Immediate dense constructors | 5.671 | 1.142 | 1.127 | 0.731 |
| Delayed selective constructors | 4.922 | 3.462 | 1.871 | 0.919 |
| Delayed dense constructors | 9.216 | 1.805 | 2.310 | 1.179 |

Entries are medians of five process repetitions, not weighted application scores. [Full component and size results](s02-lifecycle/summary.csv) include every cell. The dedicated controls use generic execution with Global selection; generated execution and other source-selection policies are not ranked by this pilot.

**Immediate selectivity produces a stable advantage over the integrated control.** The paired relational/integrated lifecycle ratio is 0.601, with all five ratios between 0.588 and 0.614. Relational setup costs more, but execution with observation costs less. This is evidence that direct constructor-first access can repay its representation cost on this source; it is not evidence against dedicated equality execution, whose controls remain faster here.

**Broad matching is a consequential loss.** Paired relational/integrated ratios are 4.730 for flat joins, 4.768 for immediate dense joins and 5.107 for delayed dense joins. All repetitions favor the control. Relational/dedicated ratios also exceed the registered practical threshold in every family. On these sizes the extra dedicated index itself does not repay its maintenance relative to scanning.

**Delayed selectivity remains uncertain against the integrated control.** Its paired median ratio is 1.421, but the range is 0.817–2.604. It fails the prospectively required same-direction condition. Do not treat its median as an established loss. Several other cells also show timing outliers; their large directional differences survive those observations. Further precision is needed if delayed selectivity becomes the deciding contrast.

## Allocation and the responsible mechanism

The relational candidate requests 8.490 MiB over the immediate dense lifecycle, compared with 1.806 MiB for the integrated control. On immediate selectivity the ordering reverses: 2.035 versus 7.704 MiB. Requested allocation traffic is not RSS or peak engine-owned memory. Per-phase live baselines include harness data; the receipt preserves them without presenting them as isolated engine peaks.

The dense relational execution-plus-observation median is 4.690 ms of its 5.671 ms lifecycle. Source inspection identifies a specific repeated operation: the head plan materializes every matching tuple, while source execution consumes only the first applicable one before requesting another complete list. For N distinct successful pairs with no competing rules, this constructs N+(N−1)+…+1 candidate matches: 2,080 for N=64. That is an analytical count of the current path, not a measured general work counter or an inherent requirement of relational execution.

A diagnostic profile of 100 whole dense relational processes records 794 samples. HeadPlan::join alone accounts for 14.09% of sampled self time, with additional table search, allocation and result-building work in its callers. This profile includes warmup, validation and harness work, so it is attribution evidence rather than a phase timing estimate. The [profile report](s02-lifecycle/profile-report.log), [command](s02-lifecycle/profile-command.json) and compressed recording accompany the result.

## Evidence and next decision

The [prospective registration](../registrations/S02-lifecycle-pilot.md) fixes hypotheses, workloads, repetitions, bounds and interpretation. The [freeze](s02-lifecycle/freeze.json) records source and binary hashes, toolchain, affinity and successful semantic/meter gates. [Raw process records](s02-lifecycle/raw.jsonl), [paired ratios](s02-lifecycle/summary.json) and the [analysis script](../scripts/s02_summarize.py) support the table. Before measurement, 60 engine configurations and 15 independent scalar configurations matched the mathematical answers. Tests, strict Clippy and formatting pass.

The immediate next question is whether preserving or lazily discovering eligible candidates can avoid full rediscovery without introducing greater invalidation or scheduling costs. This overlaps S01's retained-discovery obligation. Any implementation must preserve rule priority, guards, fresh arrivals, equality updates and distinct consuming ownership; caching a stale application list is not sufficient. Register a bounded paired comparison against these frozen binaries before changing that path.

This investigation is selected ahead of another large matrix because an observed, localized cost could change the measured ranking. S06 direct solving remains the strongest independent architectural alternative and must be reassessed after this bounded correction. The pilot does not justify more indefinite relational tuning, rejection of shared contextual equality, or selection of a universal engine. T063 and the broader research remain active.

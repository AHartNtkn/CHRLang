# R01 native pilot: access and activation dominate this comparison

Access and activation change measured cost much more than generated versus generic execution in this pilot. Indexing helps moderate selective chains but adds cost to recursive construction and low-yield active repairs. These results support choosing access plans by available information and workload, not enabling every index universally.

All 640 registered processes completed, validating 2,560 full answers. The two counted repetitions agree exactly in every cell. Allocation diagnostics show equal live requested bytes after each of four query/answer disposals. This is bounded retention evidence, not a proof of zero one-time retention or an RSS measurement.

## Evidence and boundaries

The [prospective registry](../registrations/R01-native-pilot.md), [source/binary freeze](R01-native-pilot/freeze.json), [environment and resolved features](R01-native-pilot/environment.json), [raw results](R01-native-pilot/raw.jsonl), [execution order](R01-native-pilot/order.json) and [all-cell summary](R01-native-pilot/summary.tsv) preserve the comparison. `research/chr-compiled/experiments/analyze.py` regenerates the summary and checks completeness, counted repeat agreement and bounded retention.

Each process prepares once and runs n,n+1,n,n+1. Timings below include preparation, query setup, source execution, first full observation, engine and answer disposal, and preparation disposal. They exclude fixture/oracle work, recorded separately as harness time, and native compilation. Bundled native compilation does not establish the cost of compiling an independent user ruleset. No cold architectural superiority or compilation break-even follows.

The native runner passes default and counter-free semantic checks. Workspace tests and Clippy pass; focused allocation-feature Clippy and formatting pass. Independent read-only review checked the runner and metric ownership; its [receipt](R01-native-pilot/review.md) states reclamation limits. Shared boxes code retains its own enabled accounting constant. Changed-variable reporting is semantic and remains enabled in all configurations. The empty clock median is approximately 23 ns; small preparation/disposal differences remain noisy accounting, without overhead subtraction.

## Measured regimes

Five-process median four-query lifecycle, in milliseconds, for generated execution at base size 64:

| Family | Global scan | Global indexed | Active scan | Active indexed |
|---|---:|---:|---:|---:|
| Recursive build | 0.500 | 1.412 | 0.718 | 1.391 |
| Flat chain | 118.285 | 10.338 | 7.401 | 6.462 |
| Delayed chain | 143.695 | 12.740 | 8.784 | 7.115 |
| Equal-key collision | 1.093 | 1.185 | 0.611 | 0.659 |
| Low-yield binding repair | 1.513 | 1.191 | 1.001 | 1.236 |

For flat chains, indexing reduces global candidate visits from 460,069 to 14,949 at n=64. Activation plus indexing reaches a lower lifecycle cost, even though its 16,838 candidate visits exceed global indexed visits: candidate count alone does not measure all necessary work. Delayed chains similarly show large lookup and activation effects.

For recursive build, active indexing performs 2,210 key visits and 11,863 execution allocation requests, while active scanning uses 3,216 allocations. No indexed bucket is read on that active path: the single active occurrence already determines the candidate. Global scan is substantially cheaper here. This points to avoiding unnecessary access maintenance, not a restriction on recursive source terms.

For low-yield repairs, active scanning and indexing both visit 192 candidates at n=64, but execution allocations rise from 3,438 to 4,570 with indexing. A keyed structure has no discovery advantage when the active occurrence already identifies every attempted head. Global scanning, however, revisits 4,224 candidates and can benefit from indexing. The choice depends on the combined execution organization.

Small workloads reverse several moderate-size directions. At base size 8, generated active chain scanning costs 0.346 ms versus 0.403 ms indexed; active delayed scanning costs 0.496 versus 0.556 ms indexed. Small collision favors global scan (0.091 ms) over active scan (0.114 ms), whereas size 64 favors activation. Do not extrapolate a single crossover or assign workload weights from these two sizes.

Generation is not yet the consequential cost uncertainty. At n=64, generic active indexed chain takes 6.994 ms versus generated 6.462 ms; delayed takes 7.450 versus 7.115 ms. These median differences are below the registered 10% interpretation screen. Other cells likewise require their distributions, not a universal dispatch claim. The work runs confirm matched candidate counts and allocation counts for generic/generated paths, but do not imply that all generated compilation pays for itself. Preparation also includes source identity checking on generated paths.

## Consequential control limitation and next selection

The active indexed chain still grows from 318 candidate visits at n=8 to 16,838 at n=64. Source inspection explains a material avoidable path: matching visits heads in source order, and only substitutes the active occurrence when it reaches that head. When `edge` or `job` is the second head, its known key is not available while enumerating the earlier `reach` head. An index exists but cannot use the anchor's information yet.

This is a control limitation, not evidence that binding-aware activation inherently requires these scans. The next selected investigation is to make the active occurrence's established pattern information available before partner enumeration while preserving source occurrence tuples and application order. Generated and generic paths need equivalent treatment. Unknown terms must remain nonbinding; incompatible anchored patterns must fail without changing query bindings; repeated occurrences, history and cutoff/resume must remain correct.

This bounded repair now has greater information value than beginning R02's class/constructor implementation: it can substantially change the dedicated indexed control using information already present, with existing independent semantic checks. R02 remains independent and its equality/consumption organization must not inherit this matcher. Once this control uncertainty is resolved, reassess R02 against selective index construction and isolated compilation costs. More repetitions of the current source-order matcher would not answer that question.

Required machinery remains visible: generated selectors and body functions; generic prepared templates; dispatch tables, occurrence queues, binding dependencies, indexes/reverse keys and propagation histories. Current evidence favors eliminating useless maintained access and using established anchor information before introducing more machinery. It does not settle integrated representation, search storage, direct solving, parallelism, language restrictions or the full architecture. The goal remains active.

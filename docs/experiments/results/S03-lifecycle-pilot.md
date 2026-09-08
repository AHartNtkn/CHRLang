# Direct graphs help opaque work; lowering dominates the tested word source

The direct graph is faster than the general controls on opaque common work, but loses after early discrimination and on word generation. The checked graphless word compiler is substantially faster on its admitted source. **These results support different choices for different work placement and eligibility; they do not select a universal architecture.**

All 210 registered processes completed with correct full answers and no cutoff. This is the first comparative lifecycle pilot in the renewed sequence. It follows the [prospective registration](../registrations/S03-lifecycle-pilot.md), including five primary repetitions and two separate allocation repetitions per cell. The [runner](../../../research/chr-direct-conditional/examples/s03_lifecycle.rs), [freeze](s03-lifecycle/freeze.json), [saved order](s03-lifecycle/order.json), [raw records](s03-lifecycle/raw.jsonl), [summary](s03-lifecycle/summary.tsv) and [paired ratios](s03-lifecycle/ratios.json) support the results below.

## Complete lifecycle results

Times are median milliseconds for one preparation, sixteen changing queries and disposal. Each family uses its registered depth cycle, so compare engines within a row; rows are not workload weights.

| Family | Global scan | Global index | Active index | Conditional | Direct graph | Graphless words |
|---|---:|---:|---:|---:|---:|---:|
| Binary words | 1.268 | 1.641 | 1.071 | 2.382 | 2.268 | 0.119 |
| Nested words | 1.324 | 2.096 | 1.361 | 2.844 | 2.877 | 0.208 |
| Duplicate words | 1.886 | 3.258 | 1.631 | 4.341 | 4.727 | 0.272 |
| No-choice work | 0.263 | 0.364 | — | 1.073 | 0.413 | — |
| Opaque common work | 3.427 | 6.616 | — | 4.045 | 2.163 | — |
| Early discrimination | 9.804 | 23.501 | — | 41.727 | 62.884 | — |

**Opaque work gives the graph a real favorable case.** Paired graph/Global-scan lifecycle ratios have median 0.618; graph/Conditional ratios have median 0.515. Every primary pair agrees in direction. The same loop behind early discrimination instead gives ratios 6.334 and 1.521 respectively, again consistently in the opposite direction. These meet the preregistered pilot criterion; they are not statistical confidence claims.

**The word result cannot justify a graph recommendation.** The graph takes a paired median 14.4–19.4 times the graphless compiler's lifecycle cost across the three families. It also loses to the admitted ordinary compiled controls. Graph-versus-Conditional word differences do not meet the registered practical criterion and remain inconclusive. The direct compiler accepts an exact closed source schema; these savings do not establish a general compiler or its eligibility precision.

**Ordinary overhead matters.** With no choices, the graph is about 1.62 times Global scan's lifecycle cost by paired median, while beating Conditional. Its difference from Global index is inconclusive under the registered 20% criterion. The measured controls do not support assuming indexed access is always preferable to scanning.

Preparation and disposal are included in these conclusions. No clear graph-versus-control execution direction reverses when these phases are added in this pilot, although disposal narrows some differences. Parsing, source-fixture construction and independently compiling native code are outside the starting boundary. The result is prepared-AST lifecycle evidence, not total cold architectural superiority.

## Answer latency and allocation tell different parts of the story

The graph's bulk observation delays the first word answer until almost all answer materialization is finished. At binary depth four, median first-answer latency is about 387 microseconds for the graph, versus 90 for Conditional and 134 for Global scan. The graphless iterator returns its first answer in about 0.79 microseconds. These per-query latency observations overlap execution time and are not added to lifecycle totals.

Allocation traffic strengthens the diagnosis of the discrimination loss. Across its sixteen queries and preparation/disposal, the graph requests approximately 200.0 MiB in 2.44 million allocation calls, versus 10.9 MiB for Global scan. In opaque work it requests 7.0 MiB versus Global scan's 4.3 MiB, despite being faster. Fewer requested bytes alone therefore do not predict elapsed cost.

The graph has lower peak requested live heap in these measured cells: for discrimination, about 144 KiB versus Global scan's 326 KiB. These peaks include resident source/expectation fixtures and measurement records, so they are process-requested heap gauges, not pure engine sizes or RSS. Per-query setup-to-answer-disposal checks return to their initial requested-live baseline in every allocation process. That proves disposal of the measured finite query owners, not sustainable reclamation during an ongoing query.

All allocation fingerprints repeat exactly across their two runs. The ordinary-allocator clock calibration has median 12 ns; every primary lifecycle exceeds the registered 15,840 ns resolution floor for 66 phase measurements. No fixed overhead was subtracted.

## The discrimination loss has a concrete repair candidate

An exploratory CPU profile of the frozen graph binary records 71 samples, with none lost. It includes warmup and validation and is not another primary timing sample. About 54% of samples lie under the engine's context-subtraction path; context-map subtree cloning accounts for about 21% self samples. The sample is small and optimized frames are partly merged, so these percentages locate a bottleneck rather than apportioning exact architectural cost. [Profile metadata](s03-lifecycle/profile.json), [call paths](s03-lifecycle/profile-callers.log), [self samples](s03-lifecycle/profile-self.log), [compressed raw profile](s03-lifecycle/profile.data.gz).

| Observed hotspot | Relevant source responsibility | Candidate action |
|---|---|---|
| Context-map cloning and intersection | Construct partial-choice intersections | Check compatibility before allocation and construct the result once |
| Context subtraction under service/publication | Exclude busy, failed or already published regions | Avoid constructing an intersection merely to ask whether it exists |
| Term-vector cloning through rule-head collection | Rebuild head lists during matching | Retain prepared head metadata if the first repair leaves this material |
| Allocation/free and map destruction | Temporary regions and maps | Reduce temporary ownership after identifying the responsible operation |
| Pattern/head traversal | Scan candidates in contexts | Reassess discovery organization against the existing compiled controls |

The first two actions target visible avoidable work in [Context](../../../research/chr-direct-choice/src/lib.rs): subtraction currently constructs an intersection only to test its existence, while intersection repeatedly copies a growing map through `select`. They preserve the current region model and can be checked against the existing exhaustive truth-table oracle. Their effect is not yet measured. The other actions are possibilities, not an authorized queue of automatic tuning tasks.

## Decision and remaining scope

A bounded context-operation repair is selected next, ahead of a new S02 integration implementation or S01 selective/update expansion. It addresses a profiled cost with small semantic scope and can establish how much of the graph's adverse result belongs to this prototype's temporary allocations. This has greater immediate decision value than attributing the loss to direct graphs generally. Rerun matched controls after the repair; retain the present frozen results as the original pilot.

Word lowering and first-answer behavior remain distinct findings even if context maintenance improves. The word compiler removes a matching loop on a tightly checked source; the graph's bulk observer introduces a delivery tradeoff. Neither is settled by faster region intersection.

The independent S02/S06 designs, other graph/derivation mechanisms, selective matching, language tradeoffs, long-lived reclamation, parallelism, coherent architecture comparison and held-out challenges remain unresolved. T062 and the research goal remain active. This pilot is evidence for the decision map, not research closure.

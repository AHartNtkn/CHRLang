# Corrected local pull-tabbing has no confirmed practical gain in this pilot

The local rewrite does not establish a practical lifecycle advantage over ordinary demand with the same dependency handling. Four of 80 configurations show a repeatable loss; the other 76 remain unresolved under the registered criterion. Ordinary demand itself has useful cold-query cases, while compiled specialization and source-derived lowering are stronger on the long identity-chain sources.

These are bounded findings about the implemented direct-argument operation. They do not reject broader pull-tabbing, fresh derivation reuse or graph architectures.

## What the comparison includes

The [prospective registration](../registrations/S03-pulltab-lifecycle.md) fixes five families, depths 0/32, one/eight changed queries, resource presence/absence and both starting insertion orders. Seven modes compare ordinary StaticBirth demand, dependency-aware demand, dependency-supported lifting, compiled scanning/indexing, inferred compiled specialization and source-derived pure-prefix lowering followed by that specialization. The latter two are labeled `sealed` and `prefix` in the raw data.

The source carries a changing query tag into every output. Optional finishing calls consume one token per consumer. Hand-built complete answers and the independent scalar evaluator agree with every control; tests cover both insertion orders and restarting after cancellation. The depth/tag/order variation prevents a runner from reusing stale query answers. This is a finite, acyclic source family, not a restriction on the intended language.

Each total includes preparation, query setup, execution with complete observation, engine disposal, answer disposal and prepared-state disposal. Prepared source rules are reused across queries. Prefix lowering pays for source analysis and each query's transformation and specialized target preparation. Source/input generation is recorded separately and included in a sensitivity check. Rust compiler costs are not isolated, so the results do not establish compilation-inclusive superiority.

The pilot ran 560 exploratory timings, 560 warmups, 3,920 confirmatory timings, 560 separate allocation diagnostics and 28 cancellation processes: **5,628 measurement processes**, plus the meter self-check. All completed within the registered bounds; allocation replays match exactly, and query/prepared ownership returns to its starting live count. No comparative timing uses diagnostic counters or the allocation meter.

## Confirmed and unresolved comparisons

For each source configuration, a practical gain requires a median paired ratio at most 0.90 and all seven repetitions favoring the candidate. A loss requires a median at least 1.10 and all seven repetitions favoring the control. Other results remain unresolved; the criterion is not a claim of statistical equivalence or a universal performance probability.

| Candidate / control | Gain | Loss | Unresolved |
|---|---:|---:|---:|
| Local lifting / dependency-aware demand | 0 | 4 | 76 |
| Dependency-aware / StaticBirth demand | 26 | 0 | 54 |
| Dependency-aware demand / inferred compiled specialization | 37 | 38 | 5 |
| Prefix lowering / inferred compiled specialization | 28 | 12 | 40 |
| Prefix lowering / dependency-aware demand | 40 | 39 | 1 |

Including source/input construction leaves these classifications unchanged except for one prefix-versus-specialization cell, which moves from unresolved to gain. This boundary sensitivity does not change the main conclusions.

The four local-lifting losses occur at depth 0 with forward insertion: one consumer with resources, and four independent consumers without resources, each at one and eight queries. The paired median losses are about 11–22%. No opaque or nested case is evidence of a successful direct-argument lift: those families deliberately exercise overhead or a mechanism boundary.

Some unresolved cells have favorable medians. For example, four independent consumers at depth 0, one resource-consuming query and forward order give a lifting/demand median ratio of 0.862, but the seven ratios range from 0.747 to 1.111. That is not a verified gain or a loss. Additional precision could change this narrow result; it remains an explicit follow-up rather than negative architectural evidence.

## Where total cost and memory point in different directions

Four independent consumers, depth 32, eight changed queries, resources present and forward starting order give these medians and exact allocation diagnostics:

| Organization | Lifecycle time | Requested bytes in primary phases | Peak growth across measured phases |
|---|---:|---:|---:|
| StaticBirth demand | 39.803 ms | 12,287,841 | 164,508 bytes |
| Dependency-aware demand | 30.835 ms | 9,699,601 | 149,336 bytes |
| Local lifting | 31.033 ms | 9,724,789 | 156,512 bytes |
| Inferred compiled specialization | 12.201 ms | 20,814,379 | 378,823 bytes |
| Prefix lowering plus specialization | 4.793 ms | 11,225,027 | 290,075 bytes |

The dependency-aware representation uses less requested memory here while taking more time. The meter reports requested allocations, not RSS. Peak growth includes measured source/input, preparation and query phases relative to the initialized fixture baseline; it excludes validation/reporting allocations. Diagnostics cover the short/cold and long/reused extremes, not every intermediate allocation configuration.

Phase attribution locates the long-case runtime difference in execution/observation. Dependency-aware demand spends a median 29.343 ms there and 1.371 ms in query setup; compiled specialization spends 11.932 ms and 0.224 ms respectively. Prefix lowering spends 3.889 ms in execution/observation and 0.861 ms in setup, including per-query lowering and target preparation. Source and input generation do not account for the difference. First-answer latency from execution start in that configuration is about 2.886 ms for dependency-aware demand, 1.459 ms for compiled specialization and 0.486 ms for prefix lowering; setup/preparation are additional phases.

This source contains long chains of identity-producing calls. The dependency and lifting repairs eliminate redundant expansions, but the graph still services obligations and follows indirect results; the prefix compiler can eliminate those source calls. The result identifies work a stronger competitor avoids. It does not show that all remaining graph traversal is unavoidable, or that a different graph organization could not perform comparable elimination.

Cold sources supply contrary evidence. With one consumer, depth 0, one resource-consuming query and forward order, dependency-aware demand takes about 0.027 ms, compiled specialization 0.056 ms and prefix lowering 0.067 ms. Prefix preparation and transformation do not repay themselves there. No workload weighting combines the cold and long regimes into a winner.

## Necessary machinery and language scope

Dependency-aware demand needs a checked source fragment, conditional call results, dependency support inspection, source obligations and contextual resource claims. Local lifting adds copied calls, administrative result edges and conditional obligations. On this pilot it provides no confirmed practical gain that justifies selecting those extra owners over ordinary dependency-aware execution; the unresolved cases limit that statement.

The compiled controls have broader source execution machinery, plus inferred specialization and, for the prefix mode, an eligibility analysis and capture-free transformation. Their costs are charged. The demonstrated prefix elimination depends on a finite private source prefix; it supplies no evidence for unrestricted effectful recursion or arbitrary contextual lowering. Comparing the responsibilities exposes tradeoffs but does not assign a numeric complexity score.

## Evidence, validation and remaining work

The [sizing freeze](s03-pulltab-sizing/freeze.json), [confirmation freeze](s03-pulltab-confirmation/freeze.json) and [confirmation manifest](s03-pulltab-confirmation/manifest.json) locate every receipt and frozen source. The [machine-readable analysis](s03-pulltab-lifecycle-summary.json) retains all 400 primary comparison cells, phase medians, ratio ranges and allocation extrema. Reproduce it with `python3 research/chr-direct-conditional/experiments/pulltab_analysis.py`.

The new runner's source gate checks 420 complete control/source instances plus cancellation/reuse checks. Clippy passes for the counter-free runner. Meter self-check, exact phase-allocation replay and full ownership restoration pass. Source correctness checks run outside primary timing; the reference evaluator is unchanged.

The [post-pilot selection](S03-pulltab-postpilot-review.md) selects a fresh-derivation mechanism gate next. Cold lifting precision, stronger traversal/obligation handling, sustained retention and broader local rewrites remain open. Existing local results cannot discharge those mechanisms, other sequence stages or the architecture goal.

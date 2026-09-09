# Demand execution has opposing cost regimes; copying obscured their size

**Demand execution retains a useful opaque-work advantage, but it does not win this source family outright.** Reading immutable obligation records instead of copying them removes most of the discriminating case's allocation traffic. Scanned execution still wins that case, and source-specific lowering is cheaper across all six rechecked configurations.

The result supports keeping demand execution as an architectural candidate for common work carried through choices. It also strengthens the case for investigating how source analysis eliminates execution, rather than choosing between general executors alone. This experiment does not establish that a language needs several runtimes.

## What the comparison actually exercises

The sources recursively construct boxes around a payload. Plain sources have no choices. Opaque sources carry three choices through construction before observing them. Discriminating sources inspect the choices before construction. Each query observes the result twice, preserving correlation. The consuming variant then claims a token through a source rule. Reused preparation serves alternating depths and insertion orders.

Seven paths run the same sources: demand execution with static birth-context reuse, its current-context control, direct choice graphs, Conditional execution, compiled global scanning, compiled global indexing, and a direct lowering. The lowering checks an exact source schema and exact query eligibility, then directly constructs complete answers. It is a source-specific control, not a general compiler or the correctness oracle.

The demand implementation suspends source calls and caches results under valid contexts. It is not a classic local pull-tab implementation. The source checker restricts matching, recursion shape and resource interactions; unrestricted CHR, generalized resource aliases and cross-application templates are outside this result. The earlier [source](S03-suspended-source.md), [residual](S03-demand-residual.md), and [resource](S03-demand-resource.md) gates define those boundaries and test failure and finite service independently.

## Registered measurements and validation

The [sizing registration](../registrations/S03-demand-lifecycle-sizing.md) led to 147 ordinary timing processes, 84 allocation processes and 14 cancellation processes. Sizing found no cutoff. The [confirmation registration](../registrations/S03-demand-lifecycle-confirmation.md) then fixed the same 147 configurations: seven paths, three families, depths 0/8/32, one/eight queries with consumption, plus depth32/eight queries without consumption. Confirmation ran 147 warmups, 1,029 measured ordinary processes and 294 allocation processes. Every completed query passed full independent scalar-answer comparison; repeated allocation readings matched exactly phase by phase.

Primary timing uses counter-free release binaries and the ordinary allocator. It sums preparation, query setup, execution with complete observation, engine disposal, answer disposal and prepared disposal. Source/input construction and first-answer latency are recorded separately. Execution and observation share an interval because the engines publish answers from their service API. Independent oracle fixtures and validation are outside the timed phases; host wall time includes them. Cancellation after one service tick is explicitly unfinished and is followed by a complete query using the same preparation.

Requested allocation traffic and peak live requested bytes come from separate instrumented processes. They are not RSS. Query and prepared disposal must restore the allocation baseline. These short runs do not establish sustainable memory under ongoing consumers. Compilation is not credibly isolated in these shared harness builds, so there is no compilation-inclusive architectural claim.

Confirmation uses seven same-block ratios, a 10% median practical threshold, and consistent direction in every pair. These are descriptive decision criteria, not confidence intervals. Some short cases remain unresolved by that rule. No workload weights are assigned.

## A consequential implementation cost, then a fresh comparison

The original confirmed consuming discriminating case requested 57.4 MB and took a median 20.1 ms across eight queries. Publication copied the entire obligation list twice, including inactive sibling contexts; service also copied individual records. Those records are append-only with immutable activation contexts.

The [registered intervention](../registrations/S03-obligation-borrow-attribution.md) replaces those copies with indexed reads. The initial loop extent stays fixed. Expansion still yields before publication, and the service round retains its fixed endpoint, so new obligations cannot escape completion checks or starve older obligations. Cache validity, source effects and observations are unchanged. Default source/service gates, the 80-case work matrix, counter-free source gates and runner tests pass after the change.

The paired before/after experiment ran 96 ordinary processes including warmups and 12 allocation processes. With consumption, discriminating execution fell from 20.53 ms to 8.12 ms: median paired ratio 0.395, range 0.331–0.410. Requested bytes fell from 57,386,471 to 7,290,855. Without consumption the ratio was 0.378. Smaller effects on plain and opaque sources were mixed under the practical criterion; all six allocation reductions repeated exactly.

A separately registered 240-process competitor recheck followed. The table reports repaired demand and frozen controls at depth32/eight changed queries with consumption. Times are lifecycle medians in milliseconds. Paired ratios, which determine dispositions, are in the linked analysis.

| Source placement | Demand | Direct graph | Scanned | Indexed | Direct lowering |
|---|---:|---:|---:|---:|---:|
| Plain | 0.593 | 1.236 | 0.658 | 1.757 | 0.043 |
| Opaque choices | 1.596 | 3.115 | 4.615 | 14.231 | 0.271 |
| Discriminate first | 7.758 | 34.659 | 5.042 | 14.712 | 0.315 |

Demand beats direct graphs and indexing in all six rechecked configurations. Against scanning, opaque consumption has ratio 0.342 (0.329–0.447), while discrimination has ratio 1.531 (1.456–1.867). The plain consuming comparison is unresolved: its range crosses one. Direct lowering wins all six configurations. The repair therefore reverses the earlier indexed-relative loss, but preserves opposing scanned-relative regimes.

Lower allocation traffic does not imply lower retained memory. For discriminating consumption, repaired demand's measured peak requested growth is 422,824 bytes, versus scanning's 196,646 bytes. Opaque demand requests about 1.90 MB versus scanning's 5.62 MB, but its peak still exceeds the direct graph's. These are measured host-baseline-relative peaks, not memory requirements proved for the mechanism.

## Architectural consequence and next investigation

**Keep the common-work benefit; narrow the adverse conclusion.** Static birth-context reuse avoids repeated opaque construction. The large original discriminating cost was substantially repairable. The remaining scanned-relative loss is a result for this implementation and source contract, not proof that all demand or pull-tab organizations must lose.

**Direct lowering is the strongest control on these sources.** Its much smaller lifecycle cost survives preparation and changed queries, but its exact schema checker does not establish broad source eligibility. Broader recursive/contextual lowering remains required under S06. Generalizing the compiler and measuring compilation costs are necessary before claiming a general architectural replacement.

**Advance the selection review to contextual equality and local consuming rewrites.** Another local optimization might shift the demand/scanning boundary, but these sources already support opposite regimes and a stronger direct control. Further precision on this matrix cannot make demand the preferred implementation of the tested schema while that control applies. A distinct integration experiment can instead establish whether source matching and consumption benefit from exposing partial constructor/equality information—work this matrix does not exercise. It has more decision value now than further tuning these known sources.

T071 remains unresolved for actual local pull-tab operations, fresh derivation templates across applications, broader aliases and sustained retention. Those obligations remain scheduled under S03; they are not discharged by advancing to S02. The next package should establish a contextual/local rewrite source witness and its independent effect/progress contract before any timing. Whole architectures, language restrictions and held-out evidence remain necessary before selection.

## Reproducible evidence

- [Sizing analysis](s03-demand-sizing/analysis.json), [source/binary freeze](s03-demand-sizing/freeze.json), and adjacent raw processes.
- [Original confirmation analysis](s03-demand-confirmation/analysis.json), [freeze](s03-demand-confirmation/freeze.json), and adjacent raw processes. Its full matrix applies to the original binary; only the six selected configurations were rechecked after repair.
- [Paired repair analysis](s03-demand-borrow/analysis.json), [both binary identities](s03-demand-borrow/freeze.json), [competitor recheck](s03-demand-borrow/recheck/analysis.json), and adjacent raw processes.
- [Lifecycle runner](../../../research/chr-direct-conditional/experiments/demand_cost.rs), [source and lowering](../../../research/chr-direct-conditional/experiments/demand_source.rs), and orchestration/analysis scripts in the same directory. Recorded commands use frozen binaries; rebuilding requires the recorded feature configurations and source identities.
- [Default semantic/work validation](s03-demand-borrow/semantic-default.log), [counter-free runner/source validation](s03-demand-borrow/semantic-off.log), and [Clippy](s03-demand-borrow/clippy.log).

# Saved-read lookup has a quadratic near-miss regime

Recorded-read validation examines every incompatible saved context for the same inputs in the constructed near-miss source. With 32 alternatives and constructor depth eight, it performs 4,464 candidate checks without a hit. Repeated compatible contexts instead produce 279 checks and 279 hits.

**Both regimes now have independent complete-source checks.** The next comparison must price their full costs together. Candidate counts alone neither reject read validation nor justify another index.

## The source distinguishes useful reuse from failed recognition

Each branch binds two leaves, equates the same two constructor chains, publishes a distinct tag and optionally consumes a token. Unique branches use different leaf names. Repeated branches use the same name, allowing relevant deductions to be reused. Mixed branches make odd alternatives incompatible, so the complete result must preserve successful branches and reject the others.

The [registration](../registrations/S02-read-near-miss.md) fixes 1/8/32 alternatives, depths 1/8, the three binding patterns, optional consumption and both query arrival orders: **72 configurations**. Six contextual modes and compiled Scan/Indexed execute each source. Analytically specified answers first agree with the independent scalar evaluator; all eight modes then agree on complete answers, including tags, bindings and residuals. Results remain valid after engines and prepared owners are disposed.

One counter-free and two diagnostic confirmations pass within the 120-second wall/CPU, 1-GiB and 200,000-service-call limits. Each diagnostic confirmation records 432 contextual rows, which reproduce exactly. The two compiled controls supply correctness evidence; this experiment does not price their execution.

## Exact counts establish the adverse mechanism

| Alternatives, depth eight | Unique: candidate checks / hits | Repeated: candidate checks / hits |
|---|---:|---:|
| 1 | 0 / 0 | 0 / 0 |
| 8 | 252 / 0 | 63 / 63 |
| 32 | 4,464 / 0 | 279 / 279 |

For these sources, each of the `depth + 1` equality input pairs sees the earlier incompatible contexts. Total candidate checks are `(depth + 1) × n × (n − 1) / 2`. The exact assertion holds across all unique-source configurations and both map representations. A separate operation witness checks every individual insertion: the next target merge examines exactly `i` incompatible entries after `i` previous contexts, for 41 cases across sizes 1/8/32.

Compatible repetition checks one saved candidate for each reusable equality: `(depth + 1) × (n − 1)`. At size32/depth8 those 279 successful candidates require 2,790 individual read validations. Unique and mixed cases reject candidates on their first changed read, so their candidate and read counts are equal. Ordered and persistent variants have identical work counts here; that does not make their descriptor allocation or timing equal.

The candidate uses the existing key tree to select an input-pair range, then validates entries within that range. The quadratic count is a property of this search on accumulating incompatible contexts, not a theorem that all sound relevant-state recognition requires quadratic work. Its bounded 4,096-entry cache is not saturated by these cases.

## Counter ownership now includes the final step

A diagnostic getter that reads only the current frontier loses access to counters when the last search state is released. During development it reported 4,433 rather than 4,464 candidate checks on the size32/depth8 unique source. Exact source formulas exposed the incomplete observation boundary.

The diagnostic engine now retains only a small shared counter object containing hits, candidate checks and read checks. It does not retain the source store or cached deductions. This object and every increment are confined to `deduction-work`; the counter-free representation and lookup algorithm are unchanged. Counters remain readable after exhaustion, and the complete-source assertions now require exact totals.

The earlier changed-source reuse gate consequently reports **276 hits per relevant policy**, rather than the frontier-based observation of 266. Its complete answers and positive-reuse conclusion still hold. The [ownership comparison](S02-read-ownership.md) did not enable these counters, so this correction changes no recorded allocation result. Earlier observations using the frontier getter must be treated as potentially incomplete work counts, not full-session totals.

An isolated mutation disables candidate counting without changing lookup. The exact operation witness fails, confirming that it observes the claimed work. Four relevant/contextual regression groups total 20 tests; strict scoped Clippy also passes. The independent [audit](s02-read-near-miss/review-audit.json) verifies frozen hashes, bounds, repeat equality, the exact formulas and all source coordinates.

## Breadth review and the next bounded comparison

This is the fourth package after the adaptive result when counting archive applicability and recorded-read qualification separately, followed by ownership and this near-miss gate. The portfolio review retains all unanswered directions and selects **one bounded recognition-cost package**, including the near-miss source and the already measured favorable controls. It must account for lookup, miss construction, replay and ownership before primary timing is interpreted. No new lookup index is selected from these counts.

| Direction | Required next evidence and selection consequence |
|---|---|
| Integrated equality | Price the qualified recognition regimes; distinct flat, CHR-expressed and local-rewrite organizations remain independent trials. |
| Demand execution | Nonground posts, writable heads and dynamic choices remain the strongest distinct next implementation; reconsider at the recognition-cost result or a concrete obstruction. |
| Compact solving | Connected cores beyond current reductions and richer union/projection remain required; finite-formula results do not settle them. |
| Restoration | Carry the audited checkpoint costs; larger retained states, different switching policies and sustained consumers require different sources. |
| Calls and failure reuse | Substantive caller relevance, fresh outputs, invalidation and eviction remain unpriced beyond their gates. Equality-operation reuse cannot discharge them. |
| Discovery and generation | Intermediate joins, broad rule populations and identical prepared/generated plans remain required; they need their own source/control comparison. |
| Source elimination | Retain lowering as a strong control where eligible; broader consuming derivations and recursive/effectful boundaries remain open. |
| Choice and graph organizations | Pull-tabbing, fresh derivation reuse, compressed supports and local ownership retain distinct obligations. An equality cache cannot substitute for them. |
| Sustained ownership and output | Immediate/window/all consumers, exact observation and publication remain required; finite release does not establish sustainable engine memory. |
| Native and parallel execution | Warm independent workers and connected resource claims need separate qualified comparisons, including useful work surviving serial lowering. |
| Language properties | Carry explicit source eligibility and unrestricted boundaries with each beneficiary; this lookup changes no language semantics or mandatory restrictions. |
| Complete architectures | Use the resulting conditional costs in coherent paths with simpler competitors. Do not add component timings or assume that a subsystem gain survives composition. |
| Held-out challenges and closure | Freeze any selected policy before new sources; retain every unresolved entry in the 57-question map. No closure claim follows. |

The cost package has a concrete chance to change a decision: ordered validation already beats recomputation on traffic in a substantive case, but higher peaks and now-demonstrated failed recognition may reverse the total tradeoff. Its sources, oracle and controls are ready. Extending demand capability can change a broader architecture question, but requires a new source capability before comparable costs are available. That justifies this bounded comparison first, not indefinite refinement. Reassess at its result boundary, including any attribution or clock qualification needed to reach it.

## Reproduce and inspect

[Prospective registration](../registrations/S02-read-near-miss.md), [source freeze](s02-read-near-miss/source-freeze.json), [bounded confirmations](s02-read-near-miss/gate.json), [diagnostic rows](s02-read-near-miss/bounded-diagnostic-0.log), [counter-free result](s02-read-near-miss/bounded-primary-0.log), [mutation result](s02-read-near-miss/mutation.log), [regressions](s02-read-near-miss/regressions.log), [prior-source counter check](s02-read-near-miss/prior-source-counters.log) and [Clippy](s02-read-near-miss/clippy.log) retain the evidence.

Run `python research/chr-relational/experiments/audit_read_near.py` for the recorded-result audit. The `read_near_gate.py` and `read_near_mutation.py` scripts in the same directory reproduce bounded confirmations and the isolated counter mutation. No comparative timing or allocation ranking is registered by this gate. T072 and the research goal remain active.

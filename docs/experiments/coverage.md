# Architectural questions, evidence and current selection

Future work is selected by its ability to change an architectural or language-design decision under the [sequence](sequence.md). A completed experimental package is evidence for its stated claim; it does not establish that its entire mechanism is resolved or that its follow-ups deserve execution.

**Current selection:** [R04 lifecycle evidence](results/R04-lifecycle-pilot.md) validates 720 processes and 1,800 queries. Native finite lowering is cheaper in every registered cold/reuse cell; active source search has large scheduling/frontier costs. T033 selects a separate diagnostic investigation of failure service and branch retention before a storage or conditional-execution comparison. Native compilation amortization and broader language/lifetime/parallel questions remain open.

## Question map

“Screen in R00” means evaluate the candidate and its decision-critical uncertainty before selecting an implementation. “Conditional follow-up” means there is a plausible question but current evidence does not justify putting its experiment ahead of the architectural comparison. Neither means empirically rejected or sufficiently investigated for final closure.

| Question / decision | Current evidence and its limit | Next discriminating investigation | Dependency and disposition |
|---|---|---|---|
| Q1. Generic, generated or locally triggered execution? | E04 shows separate dispatch/cache effects; E09 retention still revisits cached tests; E13 tests single-entry unfolding | R01: generated and interpreted paths with matched indexes, then activation/access contrasts on no-OR and multihead updates | [Four-cell semantic entry](results/R01-semantic-entry.md) passes; prepared generic and native cursor paths are real. [Bound-key access/lifetime control](results/R01-access-lifetime-entry.md) passes; [Counter-free pilot](results/R01-native-pilot.md) passes; selective chains benefit from indexing while active build/repair pay unproductive maintenance. [Anchor-informed partner enumeration](results/R01-anchor-pilot.md) now cuts active indexed chain discovery from 16,838 to 772 visits at n=64. The dedicated control is usable for R02; selective maintenance and compilation costs remain open. No graph-engine dependency. |
| Q2. Dedicated term/equality representation or one substrate for data and rules? | E18 proves finite monotone feasibility; E06/E15 are service comparisons; E12 shows representation-sensitive costs | R02: complete application/equality path, with consumption and wake-up before a general CHR claim | [Consuming entry](results/R02-consuming-integration-entry.md) and [partial-equality serialization analysis](results/R02-partial-equality-serialization.md) specify the path, control and publication obligations. [Integrated source gate](results/R02-integrated-semantic-gate.md) passes independent replay. [First cost pilot](results/R02-integrated-cost-pilot.md) shows moderate fanout benefit with higher heap and contrary scan controls; [Repair/maintenance and transitive congruence](results/R02-congruence-witness-pilot.md) are measured: avoidable work corrected, nested complete lifecycle still favors dedicated execution; broader search/guard interactions remain open. A solved-unifier API is not required. |
| Q3. Which explicit-search storage organization pays? | E01 controls copying/persistence; other results expose support, key and export costs | R03: branch persistence/copy/trail/replay versus a coherent supported/local alternative on cheap/deep/wide and low/high-reuse regimes | Screen in R00. Implement only contrasts whose storage/switching predictions matter; no requirement to build all storage variants. |
| Q4. Can compilation or solving avoid most execution? | E10 gives bounded structural services; E11 gives a costly trace encoding; E13 unfolding is not general code generation | R04: direct derivation/relation compilation or specialized execution, with preparation and answer recovery | [Exact fragment](results/R04-finite-consistency-entry.md) and [source correspondence](results/R04-source-correspondence-gate.md) support direct lowering on finite cases; solver/control costs are measured in the [lifecycle pilot](results/R04-lifecycle-pilot.md). [Inference-matched solver comparison](results/R04-solving-comparison-selection.md) is specified; [compiled explicit-search correctness](results/R03-compiled-search-semantic-gate.md) now passes. [Native arc-consistency and support/conflict CNF gates](results/R04-finite-backend-semantic-gate.md) pass. Native finite lowering wins the bounded cold/reuse cells; solver/control costs are measured in the [lifecycle pilot](results/R04-lifecycle-pilot.md). No candidate-success prerequisite. |
| Q5. Which static properties or language changes earn their cost? | E02 contains concrete accepted/excluded examples, not a language-wide benefit result | R01/R02/R04 with R05: inferred, declared and required properties; compare eliminated runtime machinery and reformulations | Cross-cutting. [Guard stability analysis](results/R02-partial-equality-serialization.md) distinguishes early positive-guard execution from stabilization or rollback for nonmonotone guards. Adoption is owner-dependent; investigation is not. |
| Q6. Recompute, retain, factor or learn? | E03–E08/E10/E12 show benefits and counterexamples for grouping, caching, delay, products and proofs | R03/R04/R05: compare the same consequential workload with a credible cheap-recomputation path and the relevant retained/solved path | Conditional candidate components. Cache hit count or a favorable mechanism probe does not select a follow-up. |
| Q7. What establishes activation, completion and failure economically? | E06 gives disconnected-failure and collapse hazards; E09/E15 give finite-service protocols; local completion analysis gives a distinct entry | R02/R03/R05: local obligations, maintained compatibility or other coherent protocol including off-output work and dynamic births | Required when a candidate changes these boundaries. Validity, work suppression and physical reclamation remain separate. |
| Q8. Where does implementation complexity arise, and what can be unnecessary? | Individual protocols are documented; no consistent cross-architecture comparison exists | R00 sketches and R05 integration: required state, invariants, analyses, coordination, lifetime and coupling | Assess in every candidate; do not invent a line-count or feature-count utility score. |
| Q9. How should answers be observed and retained? | E14 finds distinct clone/export effects and mixed graph costs; E01 shows output-dominated memory | R05/R06: choose an observation policy for a surviving architecture; examine retention/streaming if it can change that choice | Existing exact observers are usable evidence/controls. Further stand-alone sweeps are conditional. |
| Q10. Which scheduling and parallel organization pays? | E07/E09/E15 expose latency/speculation effects; E16 shows equation-transfer costs and a separate regional pilot | R00 assesses architectural implications; R03/R05/R06 select relevant finite-service/granularity comparisons | Not gated on completion of a mature sequential engine. Further regional repetitions need a consequential uncertainty. |
| Q11. Do conclusions hold across computational regimes? | Existing program examples and controlled probes have explicit limits; supplied-witness demos are not unrestricted synthesis | R01–R06 use the workload map below and independent checks; confirm on held-out regimes | Oracles can be developed independently when needed by a selected comparison. No preferred application distribution is assumed. |
| Q12. What architecture and language tradeoffs remain? | No supported overall architecture choice yet | R07 compares viable organizations with contrary evidence and uncertainty/cost audit | Requires sufficient discriminating evidence, not completion of every feasible refinement. |

## Workload coverage for selected comparisons

This is a set of contrasts to justify case selection, not a weighted benchmark suite or a requirement to run a Cartesian grid. A registration names covered regimes and explains consequential omissions.

| Characteristic | Useful contrast | What it can distinguish |
|---|---|---|
| Ordinary computation without OR | Deterministic single-head rewriting; multihead incremental work | Basic dispatch, activation, representation and overhead independent of search sharing |
| Binding and access | Known/unknown arguments, sparse/dense alias updates, selective/unselective joins | Compilation premises, indexing, wake-ups, maintained work and invalidation |
| Resource effects | Kept/consumed occurrences, repeated occurrences, propagation, late partners | Correct source effects and their coordination/history cost |
| Search shape | Shallow/deep, narrow/wide, early/late failure, low/high reuse | State storage, switching, speculation, support and recomputation |
| Dependency structure | Independent regions versus shared bindings and competing consumers | Decomposition/locality certificates and integration cost |
| Progress | Large finite operations, divergent sibling, late failure after output-shaped data | Service granularity, trustworthy publication and finite-answer progress |
| Observation | Small/large answers, ground/nonground, distinct/duplicate residual structures | Export, exact comparison, output size and retention |
| Lifetime | Cold query, preparation reused, changed queries, long-lived answer stream | Compilation amortization, cache/arena lifetime and reclamation |

Arithmetic, SK, typing and the literal lambda relation supply some of these cases. Other programs can be chosen for a missing contrast without becoming new product requirements. Bounded syntax domains and bounded evaluation are different; unresolved reduction is not rejection. Exact finite coverage, sound prefixes, fairness arguments and performance are separate claims.

## Completed experimental evidence

The following packages remain finished at the level documented by their receipts. “Finished” does not mean that every question associated with the E-number is resolved. Original results, registrations, source freezes and raw evidence remain the authority for their measurements. Their future-work suggestions are research leads subject to the current sequence.

| Package | Finished evidence / boundary | Current use |
|---|---|---|
| E00 | [Semantic registry](results/E00.md) | Independent expected observations and adverse cases, within their scope |
| E01 | [Persistent/copy storage comparison](results/E01.md) | Storage and output-retention evidence for Q3/Q9 |
| E02 | [Initial source-property inventory](results/E02-initial-inventory.md) | Analytical eligibility/reformulation examples for Q5; not a cost experiment |
| E03 | [Conditional finite-support probe](results/E03.md) | Shared physical work and bookkeeping counterpressure for Q3/Q6 |
| E04 | [Local dispatch and event caching](results/E04.md) | Separate mechanisms for Q1/Q6 |
| E05 | [Named-family equality service](results/E05.md) | Correlation/representation evidence for Q2/Q3, not a complete engine |
| E06 | [Net equality](results/E06-unification-costs.md), [native correspondence probes](results/E06-native.md) | Specific encoding costs and transferable semantic hazards for Q2/Q7 |
| E07 | [Certified delayed splitting](results/E07.md) | Work/speculation tradeoff for Q3/Q10 |
| E08 | [Permanent factors](results/E08.md) | Dependency and output-product costs for Q6 |
| E09 | [Scheduling costs](results/E09-costs.md), [maintained matching gate](results/E09-maintained-gate.md), [sizing](results/E09-maintained-sizing.md) | Scheduling and repeated-discovery evidence for Q1/Q10; sizing cutoffs remain unresolved outcomes |
| E10 | [Structural service and space probes](results/E10.md) | Bounded solving/reuse evidence for Q4/Q6 |
| E11 | [Matched bounded trace encoding](results/E11-matched.md) | Evidence about that encoding, not direct solver compilation generally |
| E12 | [Equation tables](results/E12-equations.md), [continuation tables](results/E12-continuations.md), [selective failure checks](results/E12-failure-native.md) | Representation/key/proof costs and bounded benefits for Q3/Q6 |
| E13 | [Specialization costs](results/E13-costs.md) | Bounded unfolding evidence for Q1/Q4 |
| E14 | [Exact graph gate](results/E14-graph-gate.md), [repeated observation costs](results/E14-graph-costs.md) | Credible observation options and adverse regimes for Q9 |
| E15 | [Complete service/session comparison](results/E15-costs.md) | Whole-path costs and scheduling effects within its stated organizations |
| E16 | [Equation workers](results/E16-costs.md), [regional pilot](results/E16-regions-pilot.md) | Representation/granularity evidence for Q10; pilot is not a repeated timing conclusion |
| E18 | [Integrated relation gate](results/E18-relational-gate.md), [source interleaving analysis](results/E18-source-interleaving.md) | Finite monotone feasibility and premises for Q2/Q7 |

E17 has [application/oracle preparation](../goals/chr-experiments/notes/T017-application-entry.md), not a finished broad application comparison. The [direct-choice entry](../goals/chr-experiments/notes/T021-distributed-choice-entry.md) and unfinished prototype are not a completed semantic gate. Neither is automatically the next experiment.

## Research leads without automatic execution priority

| Lead | Question it might change | Why it is not already the next task |
|---|---|---|
| Direct distributed named-choice graph | Q2/Q3/Q7 | A distinct organization worth screening, but no present comparison establishes priority over compilation, activation or another architecture. |
| Further constructor-relational integration | Q2/Q5 | Finite feasibility is known; R00 must identify which next source/representation uncertainty distinguishes the whole design. |
| Maintained-match subscriptions or join indexes | Q1 | Repeated discovery is evidenced, but compiled access, activation and cheap recomputation must be considered together. |
| More graph-observer or comparator refinements | Q9 | The bounded tradeoff is known; select when observation uncertainty can reverse a surviving architectural conclusion. |
| Regional-worker repetitions or warm pools | Q10 | Pilot/worker results identify plausible regimes; further measurement needs a relevant granularity/lifetime decision. |
| Larger SK/type/lambda or resumed-session matrices | Q11 | Useful cases and oracle work, but application labels do not justify priority without a selected architectural contrast. |
| More net-service or trace-solver tuning | Q2/Q4 | Existing results do not test boundary elimination or direct compilation; another same-boundary matrix needs a new consequential hypothesis. |
| Trailing, replay, reunion, generalized tables and certificates | Q3/Q5/Q6 | Plausible alternatives requiring screening, not mandatory implementation because their names appear in a portfolio. |

## Updating this map

For a selected experiment, add its decision brief, actual dependency, expected information, registration and priority rationale to the relevant Q-row. After execution, link evidence and record what decision changed, what remained uncertain, and the justified next task. Introduce a new question when findings expose a materially different decision; do not force it into an existing family.

At final closure, every relevant question needs a bounded conclusion with remaining uncertainty unlikely to justify further cost, a specific owner decision, or an exact unavailable external resource/result. “Unselected” is an interim priority disposition, not a final closure. R07 must examine plausible follow-ups and explain their decision value relative to cost.

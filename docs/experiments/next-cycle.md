# Experimental sequence for the unanswered architecture questions

Test each distinct unresolved mechanism, measure the complete cost of credible implementations, and challenge the resulting architecture choices. The existing baselines remain controls. Completing a pilot or this sequence does not complete the research goal.

This is the current execution order. The [governing sequence](sequence.md) defines the standards, the [57-question map](question-to-experiment-map.md) assigns every reviewed question, and the [mechanism ledger](remaining-investigations.md#distinctions-that-must-not-disappear-inside-a-stage) preserves distinctions within those questions. Those detailed obligations remain required even when grouped below.

## Where the evidence leaves us

The [design review](results/R07-design-disposition-review.md) distinguishes measured limits, incorrect transformations and directions without a direct trial. Preserve those distinctions: a counterexample rejects its stated transformation; an implementation loss bounds that implementation; an untested alternative still needs investigation.

Several useful comparisons now exist. [Integrated graph scanning](results/S02-multihead-scale.md) has favorable measured regimes. [Finite solving](results/S06-finite-lifecycle.md) has selective benefits and unselective costs. [Support ordering](results/S08-order-lifecycle.md) has opposing regimes. These findings supply stronger controls, not a complete architecture ranking.

T078 is active for host/native ownership qualification and the bounded mixed-source comparison. [Primary composition](results/S10-host-primary.md) now passes479 endpoints; [native direct heap and mapping ownership](results/S10-native-allocation.md) now passes52 processes. [Host tracing and process-residency samples](results/S10-residency.md) now preserve full endpoints and expose a corrected emitter lifetime cost. Next register the bounded complete-query mixed-source pilot; sampled residency is not a continuous peak. The [primary learning gate](results/S06-learning-primary.md) qualifies ordinary-allocator execution and cancellation/reuse without a timing conclusion. T073 remains unfinished; its substantive common-prefix and reuse cost pilot returns for selection at the T078 allocation gate or an obstruction.

## The order, and why

Resume with host/native allocation qualification in investigation 1; reconsider investigation 2 at that gate. Investigation 1 retains allocation qualification and its bounded mixed-source pilot; reconsider that work at the learning validity gate or a consequential obstruction. Language and lifetime tests accompany every candidate; they are not postponed until the last stage.

| Order | Investigation | Architectural decision it could change |
|---|---|---|
| 1 (active) | Complete lifecycle accounting and one mixed-source pilot | Whether current complete paths retain their apparent advantages once preparation, observation and disposal count |
| 2 (pending: learning costs) | Source analysis, resource derivations and independent native compilation | Whether substantial runtime machinery can disappear rather than merely run faster |
| 3 | Integrated equality, matching and consuming execution | Whether one organization can replace several services and their repair work |
| 4 | Demand-driven choices, fresh derivations and native local ownership | Whether sharing execution or distributing effects changes the viable architecture |
| 5 | Call-level reuse and reusable failures | Whether cheap recognition and reuse can substitute for shared execution |
| 6 | Restoration, adaptive splitting and repeated reunion | Whether economical explicit search can compete without retaining a shared graph |
| 7 | Retained joins and source-derived discovery plans | When maintaining knowledge beats rediscovering it, including the cost of code generation |
| 8 | Compact structural solving and richer theories | Whether solving compact descriptions avoids enumeration, and under which language contract |
| 9 | Sustained lifetime, exact observation and publication | Whether candidate gains survive long execution and realistic consumers |
| 10 | Reused workers and connected parallel work | Whether useful work surviving serial optimization repays coordination and ownership |
| 11 | Complete architectures and necessary complexity | Whether mechanisms work well together, and whether a simpler organization is preferable |
| 12 | Held-out challenges and every-question audit | Whether the recommendations withstand plausible contrary evidence |

**This is a default order with explicit dependencies, not a claim that later questions matter less.** At each package boundary compare the next proposal with the strongest ready distinct alternative. Record the decision each could change, missing prerequisites, likely effort, a plausible contrary result, and the next review point for the investigation placed later. Review breadth after four packages, including correctness and attribution packages. Reordering changes the schedule, never the evidence obligation.

## The next concrete experiments

**First test whether learning can preserve common work, then return to its total cost and the broader architecture comparison.** The current failed-region implementation provides a correctness control. Its successful depth-16 witness takes 41 solver steps versus 26 without learning because eager subtraction repeats a common prefix. That result motivates the next contrast; it does not reject learning.

| Package | Comparison and independent checks | Decision and next step |
|---|---|---|
| **Learning without upfront splitting — T073, gate recorded** | Compare eager failed-region subtraction, checking whether a refined state is wholly covered by a known failure, ordinary recomputation and exact-query caching. Retain the existing domain/alias/weight matrix and common-prefix depth sweep. Add late aliases, changed private calls, cancellation and limit failures; check complete source outcomes independently. Count region checks as well as solver work. | Can learning avoid common-prefix duplication without introducing an equally consequential checking cost? A semantic defect requires diagnosis. A sound candidate proceeds to total-cost measurement; a bounded work-count result cannot select it. |
| **Allocation qualification and paired costs — T078 with T073 evidence** | Check allocation ownership and disposal for the compared paths, including learned regions, prepared rules and retained answers. Use separate diagnostic builds. Register counter-free timing across one-shot and changing-query reuse, selective and unselective work, common-prefix depth and bounded cache capacity. Include key creation, checks, eviction and disposal. | Does avoided work repay recognition and retention, and where does it cross over? Keep requested allocation, live heap and RSS distinct. If compilation remains unmeasured, bound the claim accordingly. |
| **One mixed-source whole-path pilot — T078** | Compose the qualified primary Rust/native measurement paths with the same complete observable endpoint. Include applicable lowering and solving controls, failed siblings, meaningful common work, early/late readiness and overhead cases. Keep unsupported sources in the capability report. Freeze sizes, repetitions and interpretation before comparative runs. | Which apparent component advantages survive a coherent execution path? Diagnose only costs capable of changing that conclusion, then compare the next distinct investigation with further refinement. |
| **Return to distinct architectural mechanisms** | Reassess direct consuming-resource derivations and independent native compilation against integrated execution and demand-driven graph work. Use investigations 2–4 below to name the missing mechanism, favorable witness, strongest control and required implementation for each. | Select by the architectural decision the evidence could change and the cost of obtaining it. Record the next review point for each later investigation. Learning about finite failed regions cannot settle general conflict learning, resource derivations or compilation. |

Learning validity, allocation attribution and [primary measurement qualification](results/S06-learning-primary.md) are recorded. T078 host/native allocation qualification is next; learning comparative costs remain unfinished. Later rows depend on the candidate and measurement gates they name; they do not require unrelated mechanisms to succeed. Exact timing matrices follow correctness and exploratory sizing under the common registration rules below. Review breadth after four packages even if all four were correctness or attribution work.

## 1. Finish the current whole-path comparison

**Hypothesis:** advantages seen inside an engine may survive, disappear or reverse when the same source and complete answer are charged across their lifecycle. This tests S10 and review entries 41–47 and 55; it does not settle all graph or compilation designs.

Qualify the current Rust/native runner with existing scanned/indexed, contextual, conditional and admitted lowering/solving controls. Reuse prepared rules across changing queries. Charge source loading and decoding, host emission, dictionaries, preparation, query setup, execution, first and full observation, cancellation, consumer retention and disposal. Where phases cannot be isolated credibly, report their joint cost. Compilation remains a separate experiment until it is actually measured.

Use mixed sources with equality-enabled matching, consumed and kept occurrences, choices, failing siblings and irrelevant work. Include tiny/no-choice overhead, substantive work before and after choice discrimination, early and late readiness, and a finite answer beside ongoing work. Unsupported programs remain visible capability exclusions; the shared admitted subset must not become the definition of the language.

**Why first:** qualified source and answer gates make this a near-term comparison of coherent paths. The strongest alternative is investigation 2, which could eliminate work those paths still perform. At full runner qualification, explicitly choose between one registered mixed-source pilot and investigation 2. More runner refinement is justified only by an accounting or correctness defect that could invalidate the comparison. After that pilot and necessary consequential diagnosis, return to the distinct-mechanism schedule.

**Deliverable:** a bounded cost and capability comparison, with an owner-by-owner accounting and no complete-lifecycle superiority claim where compilation or host costs remain missing.

## 2. Test how much execution can disappear

**Hypothesis:** direct resource derivations, contextual lowering or native generation can avoid enough execution to justify their analysis, checking and compilation. S06-A/B and S07 own review entries 6, 28–34, 51, 53 and 54.

Run separate comparisons for effectful/contextual recursion, direct consuming-resource derivations, compatible-query solver learning, and independently generated native user programs. Use existing prepared specialization and finite solving as controls. A trace solver does not represent direct derivations; a prepared interpreter does not measure native compilation.

Give each mechanism a favorable source with real work eliminated, then challenge it with unknown tails, competing observers/consumers, changed assumptions, sparse constraints, short queries and large required answer sets. Vary useful eliminated work independently of query reuse. For learning, check retained facts against invalidating query changes; for compilation, measure generation, compiler execution, artifact size/loading and disposal using a frozen user program and a prepared-data execution of the same plan.

Pair inference, checked declarations and required restrictions wherever they have different consequences. Record accepted programs, eligible programs the checker misses, excluded programs and concrete reformulations. Logical set solving and consuming multisets need separate source contracts and independent observations.

**Deliverable:** measured preparation/reuse crossovers or justified bounds, plus the exact runtime responsibilities eliminated. Carry each surviving alternative into investigation 11. Failure of one finite fragment cannot close recursion, learning or general resource derivations.

## 3. Compare genuinely integrated execution

**Hypothesis:** shared equality, constructor, matching and resource state can avoid consequential repeated discovery or repair. S02 owns entries 9–11, 16 and 52.

Compare dedicated services with flat relations, contextual overlays, CHR-expressed merging and strategic incidence/port rewrites. First explain their operational differences; implement each consequential distinction unless an equivalence is established. Reuse current graph-scan and retained-join results as controls.

Use complete consuming sources with broad and narrow merges, nested constructors, shared and mostly distinct terms, selective and low-yield activation, and competing effects. Vary useful interleaving independently of repair fanout. Include output-only facts that later become relevant. Stable entailment guards and state-inspecting guards receive separate semantic studies.

**Deliverable:** show which boundary or repeated operation disappears and whether that repays identities, subscriptions, invalidation, observation and lifetime. Attribute a loss before rejecting an organization; investigate credible repair when it could reverse the conclusion.

## 4. Give distinct graph mechanisms their own trials

**Hypothesis:** demand-driven expansion, fresh derivation reuse or local resource ownership can outperform both explicit search and current conditional execution. S03 and S09-B own entries 12–16, 18–19, 23 and 50.

Separate pull-tabbing, reuse across fresh applications, caching within one application, compressed choice supports and symbolic equivalence before projection. A common representation does not establish operational equivalence. Test dynamic choice births, repeated uses of one choice, independent choices, opaque work followed by constructor demand, incompatible consumption, off-output failure and bounded publication beside continuing work.

Use substantive shared work and delayed contexts as opportunities; use immediate discrimination, little reuse, conflicting effects and large exact outputs as challenges. Charge context validity, fresh-result transport, support construction, discovery and reclamation. Compare eager partitions and demand-driven discovery with restored explicit execution and applicable source elimination.

The qualified native serial owner is a control for an actual local claim/commit design. Test disjoint claims, contention, hot identities and cancellation during claims before scaling. Broaden constructor support and identity lifetimes separately from the qualified atom/unknown fragment. A failure of one backend mapping is scoped to its premises; assess the strongest feasible alternative mapping when consequential.

**Deliverable:** one conclusion per distinct mechanism, including source capability and complete costs. Native feasibility evidence alone cannot decide whether graph execution is economical.

## 5. Reuse calls and failures beyond exact state identity

**Hypothesis:** stable keys and sound dependency projection can recognize useful repeated work more cheaply than recomputation or shared execution. S05 owns entries 26 and 35–40.

Compare stable-identity success/clash caching, checked learned failures, call-level reuse and exact/renamed/relevance-projected whole-state tables. Reuse the existing compact-key evidence; extend where caller effects or dependencies were not represented.

Use distinct callers with equivalent future work, repeated substantive failures and genuine reconvergence. Challenge with near-identical resources that have different futures, late bindings, unique/trivial requests and invalid assumptions. Vary useful work, reuse distance and cache capacity independently; compare bounded eviction, regeneration and indefinite retention. Preserve raw multiplicity and fresh returned identities outside key recognition.

**Deliverable:** a crossover including recognition, validation, transport, retained explanations and disposal. A whole-state table loss does not answer call-level reuse; successful lookup alone does not establish total savings.

## 6. Broaden explicit search organization

**Hypothesis:** restoration, selective recomputation or temporary decomposition can obtain shared-work benefits at lower total cost. S04 owns entries 17, 20–25.

Carry forward competent copying, persistence/COW, undo, replay and checkpoint controls. Extend the unresolved regimes: sparse versus broad mutation, wide frontiers, expensive replay prefixes, branch switching and long-lived retained answers. Validate bindings, occurrence identities, consumed facts, propagation history and pending work after restoration.

Separately compare fixed quotas with a simple observable-work splitting policy, including immediate child failure and continuing common work. Compare repeated temporary separation/reunion with permanent factoring and ordinary execution. Use same-predicate independent occurrences, delayed linking, shared consumables, frequent reconnection and little independent work. Measure independence-check precision and reused fact preparation.

**Deliverable:** time/memory regimes for each restoration mechanism, splitting policy and reunion strategy. Freeze adaptive policies before challenge cases; charge speculation, starvation prevention, analysis and output products.

## 7. Change how matches are discovered and maintained

**Hypothesis:** intermediate joins or source-derived plans can remove work beyond current indexing, but only when maintenance earns its cost. S01 owns entries 1–8.

Compare partial joins and subscriptions with full retention and competent recomputation on weakly keyed many-to-many updates. Cross selectivity with update density and invalidation breadth; vary arrival order, aliases, fanout and consumption. Cheap keyed requests and small stores are overhead controls.

Separately test constructor discrimination, partner ordering, selective indexes and wake policies, including newly enabled rejection competing with queued choices. Execute the same access plan as prepared data and generated code to separate plan quality from dispatch cost. Investigation 2 supplies compilation accounting. Charge immutable metadata, analysis, retained matches, repair and code growth.

**Deliverable:** identify when each policy pays and whether source information can select it reliably. Do not repeat settled corrections unless they affect a new contrast; do not treat those corrections as evidence against untested discovery organizations.

## 8. Test compact structural descriptions and changed theories

**Hypothesis:** lazy structural solving or projection can avoid enumeration beyond the measured finite relations. S06-C/D owns entries 26–27 and 34.

Compare lazy construction, state reduction, intersection and enumeration on selective, redundant and unselective constraints, correlated paths and changed queries. Charge constructing the space and producing complete outputs. Distinguish bounded syntax from bounded evaluation.

Give existential projection, names, disequality and normal/neutral distinctions separate denotations, independent small oracles and source witnesses. Pair any changed meaning with S07 language analysis; arbitrary solver equality cannot replace nonbinding source matching.

**Deliverable:** identify work avoided, required theory machinery, unfavorable regimes and expressiveness changes for each proposal. A finite path-equality result cannot discharge the richer theories.

## 9. Complete the lifetime and observation studies

**Hypothesis:** retention and publication obligations can reverse short-query advantages; some history can be reclaimed or regenerated economically. S08 and S07-B own entries 19, 23, 36, 40–47 and 52–54.

Start these checks in every earlier candidate's first lifecycle trial. Then run sustained streams and related queries with immediate consumer release, bounded windows and retained-all answers. Separate engine history, preparation/artifacts and consumer outputs. Compare reclamation, bounded caches and regeneration; establish which roots are dispensable before claiming avoidable retention.

Compare eager/tree and graph answers, mapping-sensitive exact indexing and symmetry refinement on large shared outputs, small distinct outputs and locally similar but globally different residuals. Compare bounded publication/backpressure, strict priority and ordinary scheduling under duplicate-heavy work, slow consumers and ongoing siblings. Measure first/full answers, continuing source progress, memory trajectories and cancellation.

**Deliverable:** sustainable regimes and limits for each viable organization. A different projection, guard or publication contract is an explicit language option with paired consequences, not a cheaper measurement of the original contract.

## 10. Separate useful parallelism from coordination overhead

**Hypothesis:** reused workers or local connected work can earn their ownership and coordination costs after strong serial optimization. S09 owns entries 16, 22 and 48–50.

Compare serial, one-worker and multiple-worker execution, cold and reused, using compatible representations. Include changed queries, shared immutable input, balanced/skewed work, tiny and substantive tasks, output pressure and shutdown. Sweep useful granularity on available hardware; include the strongest applicable serial lowering on both sides.

Independent regions and connected work remain separate trials. Connected scaling follows the claim/commit gate in investigation 4 and includes contested consumption, hot identities, abandoned work and cancellation. If forcing one transport protocol would erase an architecture's proposed benefit, compare complete organizations instead.

**Deliverable:** measured granularity/scaling crossovers or bounds, with startup, transfer, coordination and reclamation charged. An unavailable hardware resource blocks only the comparison requiring it.

## 11. Compare complete architectures and explain complexity

**Hypothesis:** component gains may conflict when combined; a simpler coherent design may beat a portfolio. S10 owns entry 55 and the interactions carried forward from all other stages.

Qualify at least two complete paths, including a serious replacement for the leading organization. Compare any combination with a simpler single organization. Use mixed sources spanning joins/aliases/search, equality/failure/observation, contextual effects and sustained reuse. Exercise both favorable and adverse placement of substantive work, including programs outside optimized fragments.

Charge eligibility, routing, duplicated representations, boundary conversions, compilation, changed queries and all owners. Describe complexity through semantic responsibilities, invariants, invalidation, synchronization and recovery. Identify machinery actually absent; code size or unfamiliarity is not a complexity score.

**Deliverable:** an evidence-backed tradeoff between coherent architectures. A bounded early S10 pilot informs this comparison but does not replace it.

## 12. Challenge the recommendation before closure

Freeze candidate policies before selecting held-out sources and parameter regions. Cover no-choice computation, incremental updates, broad aliases, early/late failure, low/high reuse, connected/independent work, unknown inputs and output-heavy streams. Supplied applications can contribute witnesses; they do not define the domain or supply workload weights.

Attempt the strongest surviving objection to each proposed choice. Audit all 57 review entries and every consequential mechanism within them against the tested proposition, contrary evidence, architectural consequence and remaining challenge. Explain why further feasible work could not materially change a claimed bounded conclusion.

**Deliverable:** conditional recommendations where appropriate, explicit unresolved choices where necessary, and a justified disposition for every relevant direction. A label, priority judgment, resource cutoff or completed cycle is not closure evidence. Consequential unanswered investigations keep the research goal active.

## What every experimental package must record

Before comparative runs, register exact hypotheses, source/configuration matrix, seeds, controls, oracle, endpoints, repetitions, resource bounds and interpretation. Follow [the measurement rules](sequence.md#registration-bounds-and-interpretation): independent correctness first, explicitly exploratory sizing next, prospective confirmation last. Begin pilots with at least five ordinary-allocator counter-free repetitions and two independent diagnostic repeats; freeze confirmation counts and practical thresholds from sizing. Initial limits are 60 seconds and 1 GiB per process where compatible, plus an explicit service/output bound. Justify any exception before running it.

Primary costs include preparation, changing-query setup, execution, observation, cancellation and disposal; compilation claims also include compilation and artifacts. Report requested heap traffic, live heap, peaks and RSS distinctly. Validate complete answers independently outside timed intervals, disclose inseparable phases and harness overhead, and preserve the reference interpreter's independence.

| Observed outcome | Required follow-through |
|---|---|
| Incorrect behavior | Distinguish a defective implementation from a contradiction in the proposed semantics; repair consequential defects and rerun |
| Loss without exercising the claimed benefit | Find a real favorable source or establish why the benefit cannot occur under the stated premises |
| Credible loss with an avoidable cost | Attribute and correct that cost if it could reverse the architectural conclusion |
| Gain | Challenge overhead, adverse placement, preparation reuse, sustained lifetime and source restrictions |
| Different favorable regimes | Measure the consequential crossover and whether it can be recognized economically; report the tradeoff without invented weights |
| Overlap or cutoff | Diagnose variance or unfinished work; obtain more evidence when it could change the decision |
| A bounded conclusion no longer sensitive to further precision | State the sensitivity argument and move to a distinct question while preserving broader open scope |

Each result should open with what the evidence changes in ordinary language. Link the registration, raw evidence, validation and contrary cases underneath. This document establishes a sequence; it claims no new experimental results.

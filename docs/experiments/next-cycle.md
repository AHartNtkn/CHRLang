# Experimental sequence for the unanswered architecture questions

Investigate every consequential unanswered design, then compare the complete architectures that survive those trials. Use the existing baselines as controls. The research remains open while feasible experiments could materially change the architectural choice.

This is the current execution order. The [governing sequence](sequence.md) defines the standards, the [57-question map](question-to-experiment-map.md) assigns every reviewed question, and the [mechanism ledger](remaining-investigations.md#distinctions-that-must-not-disappear-inside-a-stage) preserves distinctions within those questions. Those detailed obligations remain required even when grouped below.

## Where the evidence leaves us

The [design review](results/R07-design-disposition-review.md) distinguishes measured limits, incorrect transformations and directions without a direct trial. Preserve those distinctions: a counterexample rejects its stated transformation; an implementation loss bounds that implementation; an untested alternative still needs investigation.

Several useful comparisons now exist. [Integrated graph scanning](results/S02-multihead-scale.md) has favorable measured regimes. [Finite solving](results/S06-finite-lifecycle.md) has selective benefits and unselective costs. [Support ordering](results/S08-order-lifecycle.md) has opposing regimes. These findings supply stronger controls, not a complete architecture ranking.

The [effectful source gate](results/S05-effectful-source-gate.md) distinguishes resource recognition from valid execution boundaries. Region keys qualify 288 finite queries, but competing consumption, intermediate observation, earlier bindings and surviving history defeat unchecked contraction. No broader reuse implementation is selected.

## Start here: the next experiments

Qualify a checked source boundary and a resumable alternative. A resource-sensitive key alone cannot preserve caller interleaving or history. The source gate narrows the implementation question to those responsibilities.

| Next | Experiment | Decision and follow-through |
|---|---|---|
| 1 — T075 active | Test a checked initial priority phase with complete resource dependencies; independently compare a design that resumes at effect boundaries for interfering callers. | Can reuse extend beyond private calls while preserving current occurrences, history, intermediate effects and source order? Do not turn the four counterexamples into a claim against all reuse. |
| 2 — conditional on qualification | Qualify ordered output, interruptible cancellation, entry ownership, invalidation and changing-query reuse; register complete costs against competent recomputation. | Do saved computations repay recognition, dependency checks, transport and retained entries? Complete-answer multiset agreement is insufficient for these endpoints. |
| At the first gate or obstruction | Reconsider adaptive timing attribution, native local ownership, conditional equality/lifetime repair and incremental projection. | Effectful reuse currently has concrete semantic obligations that can change its scope. Adaptive timing uncertainty remains required work. |
| Within three further packages | Review all directions and every missing mechanism below. | The effectful source gate is package one after the adaptive review. Correctness, ownership, attribution and timing count independently. |

The [source gate report](results/S05-effectful-source-gate.md#which-investigation-comes-next-and-why) compares the next step with the strongest ready alternatives. Exact configurations, bounds and interpretation must be registered before new runs. Broader architectural and language questions remain required throughout.

## The full programme

The numbers below identify investigations and their detailed specifications later in this document. Several already have bounded results. Follow the immediate schedule above rather than restarting at investigation 1.

| Investigation | Subject | Architectural decision it could change |
|---|---|---|
| 1 (pilot measured; broader work pending) | Complete lifecycle accounting and one mixed-source pilot | Whether current complete paths retain their apparent advantages once preparation, observation and disposal count |
| 2 (bounded resource-phase costs measured) | Source analysis, resource derivations and independent native compilation | Whether substantial runtime machinery can disappear rather than merely run faster |
| 3 | Integrated equality, matching and consuming execution | Whether one organization can replace several services and their repair work |
| 4 | Demand-driven choices, fresh derivations and native local ownership | Whether sharing execution or distributing effects changes the viable architecture |
| 5 (matched finite reuse costs measured) | Call-level reuse and reusable failures | Whether cheap recognition and reuse can substitute for shared execution |
| 6 (adaptive source gate qualified) | Restoration, adaptive splitting and repeated reunion | Whether economical explicit search can compete without retaining a shared graph |
| 7 | Retained joins and source-derived discovery plans | When maintaining knowledge beats rediscovering it, including the cost of code generation |
| 8 | Compact structural solving and richer theories | Whether solving compact descriptions avoids enumeration, and under which language contract |
| 9 (qualification recorded; broader work pending) | Sustained lifetime, exact observation and publication | Whether candidate gains survive long execution and realistic consumers |
| 10 | Reused workers and connected parallel work | Whether useful work surviving serial optimization repays coordination and ownership |
| 11 | Complete architectures and necessary complexity | Whether mechanisms work well together, and whether a simpler organization is preferable |
| 12 | Held-out challenges and every-question audit | Whether the recommendations withstand plausible contrary evidence |

**The programme is complete only when its questions have justified answers.** At each package boundary compare the next proposal with the strongest ready distinct alternative. Record the decision each could change, missing prerequisites, likely effort, a plausible contrary result, and the next review point for the investigation placed later. Reordering changes the schedule, never the evidence obligation.

## How the sequence reaches every unanswered direction

**A numbered investigation is a group of experiments, not a single test that closes the group.** The table below identifies the first missing comparison in each direction and the evidence needed before its broader architectural claim can be settled. The [57-question map](question-to-experiment-map.md) supplies the individual obligations; the [mechanism ledger](remaining-investigations.md#distinctions-that-must-not-disappear-inside-a-stage) separates alternatives within a question.

| Direction | Next discriminating comparison | Dependency and required follow-through |
|---|---|---|
| **Reuse — investigation 5** | Broader continuation reconvergence and relevance projection with live effects, using the matched finite-result and learning evidence as controls. | Establish future dependencies beyond repeated private-query shapes, then measure recognition, transport and sustained retention. Preserve the qualified finite tradeoffs and their limits. |
| **Eliminating execution — investigation 2** | Extend direct resource solving into surrounding effects; compare broader prepared lowering and native generation of the same plan. | Preserve resource competition and progress before timing. Existing compilation receipts supply controls. Measure checking, compilation, changed-query reuse and boundary costs; a closed capacity fragment does not answer general resource derivation. |
| **Integrated execution — investigation 3** | Contextual equality and local consuming rewrites versus the strongest applicable dedicated and integrated controls. | Does not require reuse or compilation to win. Establish useful interleaving, then vary merge breadth and repair demand. Flat relations, CHR-expressed merging and local port rewrites each need evidence or an operational equivalence argument. |
| **Graph mechanisms — investigation 4** | Demand-driven choices, fresh-application derivation reuse and local resource claims receive separate source trials. | Begin from their actual source/effect obligations. Serial correctness precedes distributed timing. Success of current native execution does not discharge demand-driven expansion or distributed consumption. |
| **Explicit search — investigation 6** | Checkpoint/replay policy, adaptive splitting and repeated reunion against competent copying, persistence and undo. | Restoration and progress correctness precede costs. Test sparse and dense mutation, useful common work and early child failure independently; carry surviving policies into sustained execution. |
| **Discovery — investigation 7** | Partial joins on weakly keyed updates and source-derived discrimination against recomputation and current generated access. | Use the existing access results. Measure broad invalidation and changing keys, then challenge any selection policy on unseen sources. A code generator does not settle join retention. |
| **Structural solving — investigation 8** | Lazy construction, reduction and intersection versus enumeration; separately test projection and each changed theory. | Independent denotation checks precede source correspondence. Compare selective and output-heavy problems. A finite path-equality result cannot settle names, disequality or normal/neutral distinctions. |
| **Language properties — alongside 2–8** | Inference, checked declarations and mandatory restrictions for each consequential property. | Attach each comparison to a runtime responsibility it could eliminate. Include accepted programs, semantically eligible checker misses, excluded programs and concrete reformulations. Do not wait until an architecture is already selected. |
| **Lifetime and observation — investigation 9** | Reclamation/regeneration and exact tree/graph observation under immediate release, bounded retention and retained-all consumers. | Begin owner and cancellation checks with each candidate. Sustained comparisons need working candidates, not completion of every mechanism. Test ongoing siblings, backpressure and memory trajectories. |
| **Parallelism — investigation 10** | Reused workers on independent regions; separately, connected work with local ownership. | Qualify hardware and transport early. Connected scaling requires correct resource claims. Compare serial and parallel paths after applicable serial work elimination, with cold and reused preparation. |
| **Complete architectures — investigation 11** | At least two coherent paths, including a serious replacement for the leading organization and a simpler control for any proposed combination. | Start whenever credible paths exist, and repeat when a new mechanism could change the conclusion. Charge their interactions, duplicated responsibilities and language coverage. Component wins cannot substitute for this comparison. |
| **Challenge and closure — investigation 12** | Strongest surviving objections, held-out sources and every-question audit. | Freeze policies before challenges. Every consequential mechanism needs a supported bounded conclusion, applicable analytical resolution, demonstrated equivalence or exact external/owner blocker. Otherwise it remains required work. |

**Only genuine prerequisites determine readiness.** Integrated execution, graph mechanisms, restoration and structural solving can proceed independently of the selected adaptive search comparison. Their position later in the schedule is a research-priority judgment. Language studies and ownership checks accompany their beneficiaries; hardware feasibility can begin before a parallel implementation is ready.

**The breadth review must examine missing evidence, not just the most recent experiment.** After four packages, list every distinct direction still without a credible trial, identify its first feasible discriminating experiment, and compare it with further refinement of the current candidate. Record which direction receives the next package and the concrete evidence needed to reconsider the others. A correctness gate, allocation qualification and timing pilot count as three packages, even under one task ID. Further depth is justified when it could change a decision or make a required comparison credible; familiarity with the implementation is insufficient.

## The next concrete experiments

**Next qualify checked and resumable effectful reuse — T075.** Follow the source gate above and the [full breadth review](results/S04-adaptive-cost-breadth-review.md). The strongest ready alternative is targeted adaptive timing attribution: it could resolve fixture-level policies, while effectful reuse can change the admitted source boundary. Reconsider that ordering at the first gate or obstruction.

T077 still requires timing attribution, policy challenges, delayed splitting and checkpoints/replay. T076 still requires incremental projection, broader connected shapes and source/theory correspondence. T074 still requires conditional equality/lifetime repair and sustained ownership. T080 still requires native local claims, dynamic choices, failed branches and descriptor lifetime. Broader resource derivation, integrated execution, language restrictions and complete architectures retain their comparisons below.

## Concrete entry experiments for the next breadth review

**The next review must choose among executable proposals, including proposals outside the current prototype family.** These entry experiments make the missing comparisons concrete. They supplement the twelve investigations above; passing an entry experiment establishes readiness for cost measurement, not an architecture recommendation. Exact generated sources and run matrices belong in prospective registrations after sizing.

### Adaptive search: decide from work already observed

Compare an observable-work splitting policy with the existing fixed quotas, copying, checkpoint/replay and repeated-reunion controls. Construct sources with a common deterministic prefix, two choice branches, and an optional later linking constraint. Independently vary prefix length, time until the link, mutation breadth and which child fails early. Include a continuing child beside a finite answer.

The policy may use completed source steps, failed children, changes to owned state and detected links. It must not use future outcomes or the generator's case labels. First validate restoration and progress under switches and cancellation. Then charge policy decisions, speculative work, reconnection, retained checkpoints and output products. Freeze the policy before testing new combinations of these axes.

This can change whether explicit search needs expensive shared state to retain common work. A policy that saves execution but loses overall requires attribution; one whose useful regime cannot be recognized cheaply supports a conditional tradeoff. The existing repeated-reunion allocation results remain controls, not evidence that adaptation has been tested. The adaptive entry is now qualified; its ownership/cost comparison is the strongest ready alternative to selected projection.

### Local resource ownership: make competing effects the test

Start with two rule applications that consume disjoint occurrence identities, then make them compete for one identity. Add kept heads, an equality that enables a contender, and cancellation between claiming and committing. Enumerate bounded action interleavings against a small independent transition model. Compare committed effects and permitted source ordering, rather than requiring incidental thread schedules to match.

The local protocol must establish that consumption occurs at most once, abandoned claims become available, and a permitted application cannot be stranded indefinitely by a cancelled owner. A serial-owner implementation supplies the existing control. Qualify the protocol without parallel timing first; then compare serial, one-worker and multiple-worker execution with disjoint work, hot identities and output pressure.

This asks whether resource ownership itself can be distributed economically. A protocol defect requires repair or a scoped impossibility argument. A result for independent regions cannot settle connected claims, and a different permitted source order must be studied explicitly as a language option.

### Richer structural theories: separate four meanings

Give each proposal its own small mathematical denotation before implementing optimization. Use the existing finite structural solver and direct enumeration wherever their meanings coincide.

| Proposal | First witness and adverse case | What the experiment must distinguish |
|---|---|---|
| Existential projection | Many hidden internal assignments produce the same visible structure; then expose a formerly hidden alias to a later caller. | Compact representation of possibilities versus loss of raw derivation multiplicity or future binding information. |
| Names | Repeated and distinct names occur under shared structure; then transport results into a fresh caller. | Which identities may be renamed, which must remain distinct, and whether recognition saves more than transport costs. |
| Disequality | An early exclusion rules out many assignments; then leave most terms unknown until output or a later binding. | Sound delayed constraints, contradiction discovery, output obligations and their costs. |
| Normal/neutral distinctions | A known reducible form and an unknown-headed form later receive the same constructor information. | The proposed meaning of reduction and observation; whether it changes source behavior or merely avoids evaluation. |

For finite fragments, independently enumerate denotations and compare full observations. If a proposal changes meaning, compare expressiveness and concrete reformulations alongside its costs. Do not force it through an oracle for a different language. A source contract that cannot yet be stated is an unresolved semantic question, not a performance rejection.

### Sustained lifetime: vary the consumer independently

Take at least two already qualified candidates on identical changing-query streams. Run immediate answer release, a fixed-size retained window and retained-all consumers. Cross those policies with repeated versus unique work, bounded versus unbounded caches, and occasional cancellation followed by reuse of the same preparation. Include an ongoing branch beside finite results where supported.

Measure live memory over the stream, requested allocation traffic, sampled RSS, first-answer latency, throughput and final disposal. Account separately for preparation, engine history and consumer-owned answers. Increase stream length prospectively until the observed behavior distinguishes a plateau from accumulating retention, or report the remaining bound. Retained-all output growth is not evidence of an engine leak; growth after immediate release needs an owner-level explanation.

This can reverse a short-query choice without any new matching mechanism. Start with available paths rather than waiting for every graph or solver proposal. Compare reclamation against bounded retention and regeneration whenever retained work could be reproduced more cheaply.

### Continuation relevance: change callers without changing the reusable work

Construct two histories that reach the same future computation with different irrelevant facts and renamed unknowns. Then make one of those facts relevant through a delayed binding, a kept-head observer or competition for a consumable occurrence. Include returned fresh aliases and duplicate derivations.

Compare exact, renamed and sound relevance-projected keys with recomputation and the existing finite-result reuse control. Independently validate complete futures before timing. Vary useful repeated work, irrelevant caller size, reuse distance and cache capacity separately. Charge dependency discovery, key construction, invalidation, transport and retention.

This establishes whether broader reconvergence is useful beyond the qualified finite-query family. A false hit invalidates the key; a conservative missed hit establishes a precision cost. Neither result alone settles whether another sound dependency analysis is economical.

### Integrated execution and choice representations: isolate the missing operation

For integrated execution, start with an equality that enables a multihead consuming rule and another equality that arrives after a constructor becomes relevant. Compare contextual overlays, CHR-expressed equality and local incidence/port rewrites with the current dedicated-service and graph-scan controls. Vary merge breadth independently of useful newly enabled matches. Establish which repair or scheduling boundary disappears before measuring it.

For choice representations, use the same correlated choice twice, then replace it with two independent choices. Add delayed constructor demand, fresh applications, conflicting consumption and failure outside the observed output. Compare compressed supports and symbolic equivalence before projection with eager partitions and applicable demand-driven execution. Charge construction and complete extraction as well as reduced internal work.

These are separate entry experiments. Sharing a representation does not make the mechanisms equivalent. Carry a surviving result into a complete path, including its ownership and observation machinery; do not select a graph architecture from an equation-count reduction.

## From entry experiments to a defensible decision

**Every entry above follows the same three boundaries: independent correctness, qualified ownership, then prospective cost comparison.** A boundary may be combined with another only when the evidence remains independently reviewable. At each boundary, apply the strongest-alternative comparison; after four packages, review every direction, including compilation, discovery, language properties and complete architectures already specified below.

A gain requires an adverse case and a complete-path challenge. A loss requires an exercised benefit and investigation of consequential avoidable costs. An unresolved measurement requires a sensitivity argument or further work. A semantic counterexample settles only the transformation whose premises it contradicts. These rules determine follow-through without treating an unfinished direction as rejected.

The final architecture comparison must include a serious replacement for the leading design and a simpler alternative to any proposed combination. Freeze candidate policies before held-out sources. For every reviewed question, record the tested proposition, evidence, contrary case, architectural consequence and remaining scope. If a consequential experiment remains feasible, the research remains active.

## 1. Carry the whole-path pilot into broader comparisons

**Hypothesis:** advantages seen inside an engine may survive, disappear or reverse when the same source and complete answer are charged across their lifecycle. This tests S10 and review entries 41–47 and 55; it does not settle all graph or compilation designs.

Qualify the current Rust/native runner with existing scanned/indexed, contextual, conditional and admitted lowering/solving controls. Reuse prepared rules across changing queries. Charge source loading and decoding, host emission, dictionaries, preparation, query setup, execution, first and full observation, cancellation, consumer retention and disposal. Where phases cannot be isolated credibly, report their joint cost. Use the [existing compilation inventory](results/S06-compilation-inventory.md) for measured generation and amortization claims. Broader compilation remains a separate comparison; runtime-only intervals cannot establish its total cost.

Use mixed sources with equality-enabled matching, consumed and kept occurrences, choices, failing siblings and irrelevant work. Include tiny/no-choice overhead, substantive work before and after choice discrimination, early and late readiness, and a finite answer beside ongoing work. Unsupported programs remain visible capability exclusions; the shared admitted subset must not become the definition of the language.

**Current boundary:** the qualified complete-query pilot and consequential service attribution are recorded. The immediate schedule selects adaptive search in investigation 6; matched finite reuse and resource-phase composition remain controls for broader complete-path work. More runner refinement requires an accounting or correctness defect that could invalidate the comparison. Broader source, progress and lifetime obligations remain required when distinct mechanisms return to complete-path comparison.

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

# Experimental sequence: resolve the remaining architecture questions

This sequence investigates the consequential questions left open by the [design review](results/R07-design-disposition-review.md). It compares complete architectural alternatives, including designs that earlier work did not directly test. No architecture is selected in advance.

The outcome is evidence for choosing the CHR language's architecture: total efficiency, necessary complexity, expressiveness and explicit tradeoffs. A useful result may be a conditional choice between architectures. It need not be a universal winner, a new baseline, or a collection of optional runtimes.

This is the governing sequence for the renewed investigation. Existing E/R measurements retain their stated scope. The [coverage map](coverage.md) tracks all 57 reviewed decisions. The [remaining investigations](remaining-investigations.md) give distinct mechanisms their own comparisons and specify the next selection cycle. Experimental implementation and runs are already authorized; this document defines the work and its gates, not results from new runs. S-numbers distinguish this sequence from completed experimental receipts.

## Read the sequence at two levels

The [ordered investigation cycle](remaining-investigations.md#execution-order) gives the next twelve steps from the current worktree. Start there for the practical order and why each comparison matters. The stage specifications below define the required depth, controls and exit evidence; the [named-mechanism comparisons](remaining-investigations.md#distinctions-that-must-not-disappear-inside-a-stage) account for alternatives grouped within review entries.

The sequence reuses existing trustworthy controls and measurements. It requires new implementations where the missing architectural mechanism needs one; it does not require another production baseline. Exact sizes and repetitions are registered after correctness and exploratory sizing, before comparative confirmation. The common measurement rules below constrain those registrations now.

## What changes in research selection

**Decision value remains the first priority, but an untested alternative cannot be dismissed by assuming the incumbent recommendation.** A statement such as “this would only strengthen the explicit design” must be supported by the actual comparison. A candidate may replace subsystem boundaries, source scheduling where permitted, physical branch objects or an entire interpretation loop.

**Coverage is an obligation, not a workload weighting.** Every open design group below must receive a direct discriminating investigation, a valid analytical resolution, or a documented equivalence to an already tested design. Being lower priority determines order; it does not remove the obligation. Neighboring prototype losses, implementation difficulty, and absence of an owner-supplied workload are insufficient resolutions.

**Investigation depth follows the claim.** A local correctness counterexample can reject a specific transformation. A cost claim about an architecture requires a credible complete path, a favorable case where its mechanism actually operates, adverse controls and sensitivity to lifetime and preparation. Do not require production completeness before a meaningful comparison, or report fragment results as general architectural rejection.

**Complexity and language design participate from the start.** Each candidate names the responsibilities it eliminates and introduces, its accepted programs and its observation/progress contract. A faster component with more cross-system machinery is not automatically a better architecture. No feature count, source-line total or invented workload score decides the tradeoff.

## Order and dependencies

Begin with S00, then S01. After that, use the dependencies below and choose the next ready investigation by its likely effect on the architecture decision. Reordering requires a short recorded rationale; it cannot silently remove a group.

| Stage | Decision | Required preceding evidence |
|---|---|---|
| S00 | Correct comparison contracts and choose coherent candidate designs | Existing review and source audit |
| S01 | Incremental discovery versus retained matching and compiled access | S00 |
| S02 | Integrated relational/graph execution versus dedicated services | S00; relevant access findings from S01 when sensitive |
| S03 | Direct named-choice/derivation graph execution | S00; independent of S02 succeeding |
| S04 | State restoration, recomputation and temporary decomposition | S00; competent scheduling control |
| S05 | Operation reuse, failure learning and reconvergence | S00; stable identities in the compared candidates |
| S06 | Direct relational compilation and generalization of lowering | S00; independent of graph/search results |
| S07 | Language properties and contextual optimization | Starts in S00; executable candidates from S02/S03/S06 |
| S08 | Observation, reclamation and long-lived execution | At least two relevant working candidates |
| S09 | Parallel organization and reusable workers | S00 feasibility; representative work from S01–S08 |
| S10 | Compose and challenge complete architectures | Results sufficient to build at least two coherent alternatives |
| S11 | Held-out generalization and final decision audit | S10; every earlier question accounted for |

The table is not a requirement to perfect S01 before starting S02–S09. In particular, graph execution, direct compilation and language analysis do not wait for conventional executor tuning to stop. Run an early semantic/representation feasibility screen for S02, S03, S06 and S09 during the first selection cycle so their actual implementation costs inform subsequent ordering.

At each selection, compare the proposed task with the strongest ready alternative. Record the decision each could change, plausible contrary outcomes, current evidence, required work and why this order is better. Revisit an unresolved group at every cross-family checkpoint. A succession of local refinements must not displace an untested architectural contrast merely because the refinements are easier to measure.

## S00 — Establish fair contracts and coherent competitors

**Question.** Which differences are properties of the language, which are experimental policies, and which are accidental requirements of current implementations?

Audit nonbinding matching, equality, consumption, propagation, explicit choice, raw/unique observations, failure and progress against the source and tests. In particular, distinguish observable failed alternatives from diagnostic leaf records; source occurrence identity from physical allocation order; valid source schedules from one executor's queue order; and truthful answer publication from full physical state export. Keep different semantic choices explicit rather than silently weakening a gate.

Write complete query-to-answer sketches for: conventional compiled/incremental execution; integrated constructor/equality relations; direct conditional or named-choice graphs; and direct relation/derivation compilation. Include persistence/trailing/replay and sequential/parallel organization as substantive choices, not mandatory shared interfaces. Sketches must show matching, source effects, equality, failure, suspension, observation and lifetime. Identify which existing components are useful controls and which would obstruct a fair candidate.

For each open proposal in the review, record its distinctive mechanism, favorable and adverse source witnesses, and the experiment below that owns its resolution. Merge proposals only when the relevant difference is actually equivalent; explain that equivalence. A graph service is not equivalent to direct graph execution, nor a trace solver to direct relation compilation.

**Output and exit.** A contract ledger, candidate responsibility maps, coverage assignments and the first prospective registration. Resolve implementation-policy mismatches locally. If a genuine language adoption choice is required, present its consequences; otherwise investigate both contracts on an explicit fragment. No new production baseline is an output of this stage.

## S01 — Make matching an architectural comparison

**Question.** When should the runtime rediscover matches, retain partial joins, or compile access and updates directly?

Compare competent anchor-aware indexed activation and scanning with update-driven retained joins/subscriptions and generated discrimination/access plans. E09's retained-prefix implementation is diagnostic evidence, not the only retained design. Start with a small multihead source that performs real repeated updates; support kept and consumed partners, propagation and binding-driven eligibility.

Vary independently: join selectivity, fanout, update density, arrival position, alias breadth, rule count and store size. Include stable keyed requests from T057 as an adverse case for maintenance, plus weakly keyed many-to-many joins, changes affecting few versus many retained matches, and large equal-key buckets. Include no-OR and small-query controls. Measure preparation, invalidation, retained state, discovery and complete answers, not just matching calls.

**Competing outcomes.** Retained updates may avoid repeated discovery but lose on mutation or retention; generated access may remove interpretation without retaining joins; cheap recomputation may dominate some regimes. If a current candidate misses already available information, repair that defect before ranking architectures. If the best approach depends on source properties, establish whether a practical analysis can identify them and charge its cost.

**Exit.** Evidence distinguishing recomputation, maintenance and compiled access on both selective and unselective updates. T057 alone cannot discharge this stage. Carry the strongest relevant control into later comparisons without assuming one global access policy.

## S02 — Test integration that actually eliminates boundaries

**Question.** Can constructor information, equality and consuming rule execution share a representation with less total machinery or work?

Use the corrected R02 implementation as evidence and a possible control. Design a second contrast around a boundary it did not eliminate: for example direct relational joins over constructor identities and supported equality, or local incidence rewrites that execute consuming applications without reconstructing a separate solved store. Select between flat relations, contextual equality overlays, union-find-based integration and richer port rewrites by an explicit mechanism analysis. Distinct surviving mechanisms need their own gate; an isolated unifier wrapped in graph transport is insufficient.

Exercise equality enabling structural matches, repeated aliases, constructor congruence, incompatible constructors, cycles, multihead consumption, propagation, and context-local failure. Include dense updates and mostly distinct structures as adverse cases. Add an actual interleaving witness in which integration changes useful work or removes transport/coordination; merely servicing pending deductions is capability evidence.

Compare complete execution and ownership with competent dedicated terms/equality. Where possible, isolate boundary elimination within a representation; where that would make an artificial control, compare whole organizations and state the attribution limit. Test stable positive guards separately from state-inspecting guard extensions.

**Exit.** A credible integrated path and a decision-relevant cost/complexity comparison, or an analytical counterexample to a precisely stated design. A remaining loss must be traced to obligations versus repairable representation costs. Do not infer rejection of all integration from the corrected R02 cost result.

## S03 — Give direct choice and derivation graphs a real trial

**Question.** Does direct demand-driven graph execution share useful work more economically than supported execution, cached expansions or separate branches?

First distinguish direct named-choice graphs, memoized pull-tabbing/contextual expansion, and derivation-event reuse. Analyze which are equivalent on the proposed fragment and which differ in discovery, correlation, effects or lifetime. Implement the smallest coherent path for each surviving distinction. Existing unfinished prototypes are available evidence, not privileged starting points.

The initial gate must include dynamic source-choice births, repeated uses of one choice, independent coexisting choices, an unknown carried opaquely, later constructor demand, consumption or another resource-sensitive effect, off-output failure and a finite sibling beside continuing work. Pure value copying does not establish a CHR engine. Use the E06 label/failure counterexamples as adversarial tests; do not inherit the net service's copy protocol or host normalization boundary without need.

Compare direct graph execution with current Conditional and an appropriate explicit control. Cases must include high and low reuse, delayed arrival of a context, immediate discrimination, substantive common work, incompatible effects, and no-choice overhead. Charge routing, labels, support/identity maintenance, completion and answer extraction. A native HVM mapping is one substrate option, not a prerequisite; separate abstract organization from native backend feasibility and cost.

**Exit.** Direct evidence on the untested graph mechanisms. If a substrate blocks finite service or correct correlation, distinguish that substrate failure from the architecture and assess the strongest feasible alternative. Do not stop at “a protocol would be required.” Establish what it costs or why the claimed design cannot satisfy the contract.

## S04 — Compare restoring, sharing and recomputing state

**Question.** Which search organizations earn their retention and restoration machinery across different search shapes?

Compare current copy/persistent/COW controls with a trailed mutable-state path and a replay/checkpoint path under matched observable semantics. They need not expose the same internal branch API. Restoration correctness includes aliases, failed speculative bindings, consumed occurrences, propagation history, fresh identities and cancellation. Fair scheduling may require multiple trails or replay; charge that responsibility rather than excluding the design by the current frontier interface.

Vary depth, frontier width, retained state, mutation density, early/late failure and useful work between choices independently. Include the T052 read-heavy case and immediate-insertion counterpressure. Use prompt failure scheduling so a queue defect does not masquerade as a storage cost. Measure peak live memory, cumulative allocation, restoration/replay work, answer latency and disposal.

Investigate delayed splitting beyond a fixed universal quota: a simple observable-work policy, early child failure/promotion and adverse speculation. Compare permanent factoring with a bounded temporary-separation/reunion design on sources that actually reconnect. Use a conservative correct independence certificate and measure its false negatives; test finer occurrence/ownership reasoning where that limitation changes the result.

**Exit.** Measured restoration/recomputation regimes and evidence on reconnection, not a universal snapshot default. Adaptive policies require held-out confirmation; no policy may be tuned to the confirmatory suite.

## S05 — Test reuse without the old representation penalties

**Question.** When do operation caches, learned failures and reconverged continuations avoid enough work to justify validity, lookup and retention?

Implement a stable-identity shared-representation equation/failure cache and compare it directly on T059's common clash and success cases, before and after discrimination. Include unique requests, trivial equality, changing bindings, invalidated failures and cache retention across changed queries. Reuse E12 proof boundaries where applicable, but do not use its owned serialization interface as the only possible cache.

Separately compare exact whole-state tables with alpha-renamed or relevance-projected continuation keys where a sound projection can be established. Use true reconvergence after different histories, repeated equivalent calls, structurally similar but semantically distinct contexts, and sparse reuse. Preserve source effects and multiplicity independently from recognition of equal results. Compare eviction/recomputation with indefinite retention.

**Competing outcomes.** A good cache may match common-work benefits with simpler state; direct sharing may avoid recognition/replay costs that remain substantial; neither may pay when reuse is low. These outcomes change the architecture comparison in either direction.

**Exit.** A matched test of current explicit reuse against its strongest relevant sharing competitor, and a bounded conclusion on generalized tables. Do not skip a cache because it is predicted to strengthen an already favored architecture.

## S06 — Compile the relation, not only interpreter steps

**Question.** How much execution can direct compilation or solving avoid, and what preparation and language premises does that require?

Select at least two distinct mechanisms: direct relational/derivation solving that does not encode interpreter traces, and recursive/contextual lowering beyond a pure countdown. Use finite constraints with selective and overlapping relations for the former; unknown/repeated arguments, guarded recursion and meaningful equality/effects for the latter. Include independent closed-operation and term-space candidates where exact intersection or equality-constrained structure avoids enumeration. R04's finite relation and E11's trace solver are controls with different scope, not exhaustive representatives.

Investigate lazy space construction, redundant automaton states and finite path equalities where those distinguish enumeration from compact solving. Separate bounded syntax from bounded evaluation. Validate solver witnesses and source correspondence independently, including alias relationships, residuals, multiplicity and unfinished search. Logical projection must have its own argument; arbitrary solver equality cannot stand in for source nonbinding matching.

Compare generic, prepared direct and native generated execution where they are credible. Measure independent ruleset generation/compilation, code size, preparation, changing queries, observation and artifact disposal. Sweep reuse to locate a crossover or derive a measured bound, including very low reuse and a justified long-lived regime. No user-supplied production frequency is necessary to characterize that tradeoff. Do not extrapolate savings indefinitely from one query.

**Exit.** Direct compilation receives a genuine favorable opportunity and contrary controls. Larger reuse is unnecessary only when a sensitivity argument shows it cannot change the relevant decision within a stated regime, or an exact external limit prevents the measurement. General solver or native-compilation rejection cannot follow from E11 or three-use unfolding alone.

## S07 — Measure the cost of language properties

**Question.** Which properties simplify execution enough to justify inference, declarations or changes to the language?

Maintain an explicit comparison for single-head/nonoverlap, modes and groundness, finite domains, termination/progress, immutable versus writable handles, ownership/linearity, stable guards and logical versus consuming multiset regions. For each consequential property, compare inferred eligibility, checked optional declarations and mandatory restrictions where these genuinely differ. Avoid constructing meaningless combinations merely to fill a table.

For every proposal, provide accepted examples, a near-miss counterexample, a realistic reformulation or exclusion, and an accounting of runtime responsibilities removed. Distinguish semantic proof from the precision of the implemented checker. Measure checking/linking and the cost of boundaries between eligible and ordinary code. A prototype's inability to optimize a program does not make the program semantically ineligible.

Extend contextual contraction beyond the single pure carrier only where a concrete commutation/ownership argument permits it. Test multiple interacting occurrences, external observers, unknown tails, failure and progress. Evaluate explicit relational cases and alternative guard/observation contracts as language options without silently adopting them.

**Exit.** A readable language tradeoff dossier tied to executable comparisons, not a list of possible restrictions. A genuine owner choice may remain, but only after its optimization, expressiveness and complexity consequences are established. “Optional” is not automatically the least complex design.

## S08 — Make lifetime and exact observation part of architecture

**Question.** Can a surviving design publish exact answers and release irrelevant state economically during long execution?

Compare eager and graph observation with the same exact comparison semantics, including repeated/distinct terms, joint aliases, symmetric residuals, large answers and many small answers. Investigate mapping-sensitive reuse or indexing only where the exact-comparison cost is material. Fingerprints must not replace proof of equality.

For explicit, Conditional and any surviving graph design, identify retained owners and implement a credible reclamation or regeneration policy where current retention is avoidable. Test consumers that immediately release answers, retain a bounded window, or retain all answers. Separate unavoidable output retention from historical support, caches, arenas and canceled work. Include ongoing streams and changing queries over reused preparation, with time and memory trajectories rather than only final peaks.

Compare bounded publication quotas/backpressure with strict priority and ordinary scheduling. Test finite siblings, duplicate-heavy regions, ongoing alias streams, cancellation and late failure. Record partial endpoints honestly and investigate consequential cutoffs rather than classifying them as completed costs.

**Exit.** Evidence about sustainable memory and delivery under stated consumers, or a quantified limitation with its responsible owner. The availability of a low-sharing explicit control does not discharge a graph or Conditional lifetime question.

## S09 — Separate parallel capacity from cold transport costs

**Question.** Which organizations expose enough useful independent work to repay coordination and ownership?

Begin with available-core, affinity and transport feasibility measurements. Compare serial execution, one worker and multiple workers using compatible representations. Include cold starts and a reusable pool across changed queries; worker preparation, transfer, accepted versus speculative work, synchronization, cancellation and final shutdown all count.

Use balanced and skewed regions, small and large tasks, shared immutable inputs, update-heavy work, output-product pressure and finite failure beside continuing work. Sweep task granularity far enough to establish a crossover or a bound on possible benefit on the available hardware. Include a connected-work design only with an actual ownership/partition argument; do not claim coarse independent regions resolve general parallelism.

If graph or integrated execution exposes different parallel units, compare its complete organization rather than forcing owned equation messages. Do not increase worker count merely to obtain a favorable number. Resolve overlapping results when their plausible effect could change a choice; otherwise state the measured bound and decision consequence.

**Exit.** Cold and reused-worker conclusions with adverse cases and scaling limits. Hardware blockers name the exact missing resource and leave unaffected comparisons active. E16's cold owned interface does not reject these alternatives.

## S10 — Compare complete architectures and necessary complexity

**Question.** Which coherent organization offers the best supported tradeoffs once its mechanisms interact?

Build at least two credible complete paths selected from the evidence, including an alternative that can replace the favored organization rather than merely supplement it. Do not assemble every positive component into a mandatory multi-runtime portfolio. Compare a simpler single organization against any proposed combination, charging eligibility, routing, shared identities, crossing boundaries, duplicate infrastructure and lifetime ownership.

Use sources that combine two or more consequential mechanisms: joins with aliases and search; common equality with failure and observation; contextual lowering with unknowns and external effects; and long-lived execution with reuse or parallel work. Choose both favorable and adverse placement of the same substantive work. Validate complete outcomes independently; exact physical traces are required only where the stated contract needs them.

For each architecture provide a responsibility map: semantic obligations, chosen mutable structures, invariants, invalidation, coordination, recovery, compilation and observation. Identify duplicated responsibilities and mechanisms actually absent from execution. Use concrete traces and measurements to explain complexity; do not score code lines or equate an unfamiliar technique with necessary complexity. Include implementability and unresolved proof obligations candidly.

**Exit.** Whole-path comparisons supporting conditional architectural choices, including preparation and lifetime sensitivity. If composition invalidates a local advantage or adds unmeasured machinery, investigate it before recommending the combined design.

## S11 — Challenge the decision and audit closure

Freeze candidate policies before choosing held-out sources and parameter regions. Cover ordinary no-OR computation, incremental multihead updates, alias-heavy equality, cheap and substantive search, early/late failure, reconvergence, connected and independent work, unknown inputs and output-heavy streams. Supplied applications can contribute; application labels are neither domain restrictions nor weights.

For each proposed recommendation, attempt its strongest surviving counterexample. Include cases outside eligibility and report the exact behavior and cost there. Test sensitivity to compilation reuse, memory budget, observation requirements and available parallelism. Do not manufacture a global score in the absence of workload priorities.

Every open entry in the review must end with one of:

1. Direct evidence supporting a bounded choice, with contrary cases and a reason further uncertainty cannot materially change that choice within its stated scope.
2. An analytical resolution that actually applies to the proposed mechanism or semantic contract.
3. A demonstrated equivalence to a tested alternative, with the relevant differences accounted for.
4. An exact unavailable external resource or a necessary owner decision, with consequences and all independent work completed.

An implementation budget, milestone, timeout, assigned label, or anticipated reinforcement of a recommendation is not closure evidence. If a feasible consequential investigation remains, the research remains active. If the best result is a tradeoff rather than a winner, state the tradeoff and what user preference would select between its alternatives.

## Registration, bounds and interpretation

Each stage first produces a concrete design/semantic gate, then a prospective comparative registration. Stage outlines are not permission to choose workloads or interpretation thresholds after seeing timings.

Each registration fixes: hypotheses and affected decisions; exact source/configurations and seeds; independent oracle and adverse mutations; accepted observations and scheduling assumptions; primary endpoints; warmups, randomized blocks and repetitions; resource limits and cutoff treatment; toolchain/source/binary freeze; analysis and action for each plausible outcome. Include a favorable witness that actually activates the proposed mechanism and an adverse witness that charges its overhead.

Use counter-free release timing with the ordinary allocator. Work counts and requested allocation diagnostics use separate builds/runs. Report requested bytes separately from RSS and distinguish cumulative traffic, live retention and peak. Measure preparation, query setup, execution, first/full observation, cancellation and disposal; where phases cannot be credibly isolated, report their joint interval. Measure compilation and artifact lifetime for compilation claims. Validate full answers outside primary timing where possible and disclose harness interference.

Start with bounded sizing, explicitly nonconfirmatory. For native pilots, use five primary repetitions per cell as an initial floor; choose the confirmatory count and practical decision threshold prospectively from variability and the decision. Use at least two independent work/allocation repetitions to check deterministic diagnostics. Bound each process initially at 60 seconds and 1 GiB where compatible with the mechanism, and impose an algorithmic service/output bound appropriate to the semantic endpoint. Any different bound must be justified before the run. These are operational starting limits, not criteria for rejecting a design.

If a pilot reaches a bound, inspect progress and the responsible cost. Choose a justified larger bound, a paired representation correction, or an analytical limit before confirmation. If uncertainty spans a consequential crossover, obtain more evidence; overlapping ranges are not an excuse to stop. Do not repeatedly broaden a matrix when all plausible outcomes leave the same bounded decision unchanged—write the sensitivity argument instead.

All new results link raw inputs, commands, freezes, validation and unfavorable outcomes. Correct consequential defects and rerun affected comparisons. Preserve reference independence. No new architecture adopts implementation interfaces merely because they make cross-engine testing easier.

## Current execution order

S00's contract audit and first-cycle feasibility assessment are recorded. Bounded S01 discovery, S02 relational integration, S03 direct graph and S06 finite-table compilation comparisons have produced evidence. Each leaves distinct mechanisms unresolved; the [coverage map](coverage.md) links the results and their remaining scope.

The bounded S04 restoration/replay pilot and [paired matcher correction](results/S04-matcher-paired-cost.md) are complete. Avoidable matching copies explain much of mutation's cost; compatible long aliases show a smaller regression. Corrected longer replay/checkpoint policies, larger read-heavy state and temporary reunion remain unresolved.

The bounded S05 stable-identity trial now includes the [616-process operation crossover](results/S05-operation-crossover.md): sufficiently repeated large equations establish a practical cache benefit, while distinct requests lose strongly. Invalidation width, eviction/lifetime and generalized continuation tables remain unresolved.

The bounded S01 subscription investigation now includes the [1,008-process pilot](results/S01-subscription-lifecycle.md) and [861-process low-yield follow-up](results/S01-subscription-low-yield.md). Repeated costly discovery supports retention; demand gaps, inactive combinations and consumption expose opposing ownership costs. Other intermediate stages, broad active-demand populations and generated multihead access remain unresolved.

The current task is [S09 reusable parallel workers](results/S09-reusable-workers-entry.md). It tests warm pool/query ownership and coordination after cold-start costs are separated. Broader generated multihead access is the strongest ready architectural alternative, with corrected restoration also ready. The existing concurrency service provides a concrete path to a new query-lifecycle boundary; the entry records why this distinct comparison precedes another local refinement. The [worker lifecycle gate](results/S09-worker-lifecycle-gate.md) now validates changing queries, cancellation, identity and query-owner release. The [regional source gate](results/S09-regional-source-gate.md) now checks complete products, changed queries and finite publication beside continuing work with identical inline/worker source service. The [allocation diagnostic](results/S09-worker-meter-diagnostic.md) accounts for cross-thread transfers, channel-context retention and first-query bookkeeping. The [concurrent accounting gate](results/S09-concurrent-meter.md) now passes full disposal and distinguishes variable cancellation work; [native sizing](results/S09-native-sizing.md) establishes feasible initial cases. Prospective registration and the comparative lifecycle pilot remain next. No broader stage or architecture goal is closed.

The [remaining investigations](remaining-investigations.md) define the subsequent selection cycle, dependencies and evidence obligations. Order may change on evidence; the obligations remain. The sequence is complete as a plan only when every review entry has an assigned investigation. The research is complete only under S11's evidence requirements.

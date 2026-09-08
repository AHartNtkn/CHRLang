# R00: architectural alternatives and first experiment selection

> Current status: this assessment predates the [design-disposition review](R07-design-disposition-review.md). Architecture selection and research closure remain unresolved. The [renewed sequence](../sequence.md) and [current coverage map](../coverage.md) govern further investigation; recommendations and stopping judgments below are subject to that review.

The [architecture checkpoint](R07-architecture-checkpoint.md) and [closure audit](R07-closure-audit.md) record earlier assessments. The governing sequence above supplies the current investigation requirements. Existing measurements retain their stated scope; the current evidence does not support whole-goal closure.

Current parallel evidence: the [factored-contraction comparison](S09-factored-lowering.md) shows that the persistent-worker gains on these countdown sources do not survive available serial lowering plus factoring. Parallelism on other or lowered work remains open. The current investigation is [generated multihead access](S01-generated-access-entry.md), where that single-head contraction cannot settle the execution choice.

The first comparison should distinguish generic selection, incremental activation and generated rule execution on complete ordinary-CHR computations. In parallel, analyze a directly compiled finite consistency relation as a contrasting way to eliminate execution. Integrated representation and conditional execution remain serious independent candidates; existing service results do not rank them against these organizations.

This is a selection supported by analysis and existing evidence, not a performance result or a production architecture choice. It selects decision briefs before implementation. Compilation, representation, search storage and scheduling are partly independent dimensions; the candidates below are coherent starting combinations, not mutually exclusive categories.

## Generated incremental execution with explicit search state

Rules compile to occurrence entry points and partner-access plans. Query constraints become distinct live occurrences; term constructors use a shared immutable arena and logical variables use context-specific binding identities. Introductions and changed variable classes activate relevant entry points. Nonbinding matching reads established structure; unsuccessful matches retain sufficient dependencies to be reconsidered when bindings or partners arrive.

A successful application claims live distinct occurrences, records propagation where required, and posts its body with fresh locals. Explicit OR creates separately serviceable continuations, using a selected copying, persistent or trail/replay representation. Source rule competition is resolved by one permitted policy, not enumerated as search. The first compiler can use an ordinary local equality implementation; a complete-request service boundary is not required.

Quiescence requires no pending source work or missed activation. Completed bindings and residual occurrences feed a joint exact observer. Ruleset code, query state, histories, suspended work and retained answers have separate lifetimes. Generated entry points do not by themselves make long operations yield or guarantee fair service.

The necessary machinery is a compiler, occurrence/index storage, binding watches, work queues, propagation bookkeeping and search state. Its favorable regime is substantial repeated dispatch or sparse incremental changes; its adverse regime is tiny cold queries, broad invalidation and code/index growth. A compact interpreter can win when compilation and activation machinery do not repay their costs.

The HAL compiler paper supplies concrete index and continuation optimizations using modes, determinism and algebraic properties. Its textual-priority assumptions and host representation differ; no performance or semantic transfer is presumed. [Holzbaur et al., §§2–5](https://arxiv.org/pdf/cs/0408025). E04's local dispatch result and E09's repeated-discovery result identify work to eliminate. E13's bounded unfolding result does not evaluate this compilation path.

## Integrated identity/constructor and source-rule execution

Rules compile to relational accesses or local graph patterns. Query values become identity classes with constructor descriptions; source occurrences retain their own identities. Equality consistency and application activation operate over this same representation. A merge repairs affected lookup keys and dependencies. Matching can inspect established descriptors without calling an external completed unifier.

Applications claim resources and create fresh identities, equations and occurrences within the substrate. Propagation must still distinguish source occurrence tuples. The first search representation can use separate contexts or persistent overlays; conditional facts are another choice. Completion includes descriptor consistency, finite-tree acyclicity and discharged source work. Joint outputs and residuals are reified only from a consistent completed interpretation.

Required machinery includes classes/descriptors, incidence and access indexes, occurrence/resource state, activation, propagation and lifetime ownership. Value equality must not merge distinct occurrence identities. A binding/merge can affect application columns and matching, not only constructors. The favorable regime has frequent equality-enabled computation and cheap local repair. Broad aliases, unselective joins and resource conflicts are adverse.

[E18](E18-relational-gate.md) establishes finite monotone feasibility. Its explicit equality closure does not give a lower bound on integration. [Its source assessment](E18-source-assessment.md) identifies Eqlog/egglog and relational matching precedents; those do not provide full conditional consuming CHR. The consequential unknown is whether class/index repair and resource effects stay economical, not whether equality can be expressed as relations at all.

## Conditional or demand-driven execution

Compile rule bodies into suspended dependencies over conditional values and occurrence membership. A query introduces active obligations. Explicit OR creates fresh correlated identities; copies retain the identity of their birth. Structural tests demand only the information needed for eligibility. Ordinary opaque computation may proceed without inspecting alternatives. Direct dependency activation is distinct from previewing each interpretation and then searching for equal work.

A firing's effect applies where its head tuple is live and enabled. Bindings, consumption and propagation need compatible conditional state or a sound transition to separate execution. Validity/completion dependencies include active off-output obligations; inactive failures do not poison a live sibling. Local signals, maintained compatibility summaries and lazy checks are alternative protocols, not one global requirement. Finite service must cover normalization and observation as well as source work.

Output extraction enumerates or symbolically resolves compatible observations, preserving joint aliases and residual multiplicity. Exact deduplication and eventual release of supports, names, dependencies and answers remain costs. Retaining invalid graph material is distinct from allowing it to contribute an answer.

The machinery includes birth identity, suspended dependency work, conditional values/resources, compatibility, progress and publication. Favorable cases have substantial inexpensive-to-recognize common work or avoidable early discrimination. Cheap shallow choices, immediate discrimination and compatibility churn are adverse. [E03](E03.md) shows that finding compatible work can cost more than it saves; [E06](E06-native.md) supplies correlation, failure and progress hazards. Neither result tests a complete runtime organized around direct conditional activation.

## Direct relation or resource-derivation compilation

A compiler establishes the fragment's logical and resource meaning, then generates query constraints or formula templates directly. It does not encode a prescribed array of interpreter steps. Free constructors or a suitable finite domain represent values; explicit source alternatives justify solver choices. A complete model must correspond to a terminal source observation, including full residuals and unconstrained-variable relationships where allowed.

In an eligible relation fragment, dispatch and histories may be unnecessary. Otherwise the compiler must encode occurrence production/use, fresh identities, causal support and propagation uniqueness. Such resource-derivation machinery may eliminate the reason to select solving; it is not an automatic universal implementation path. Nonbinding heads cannot be satisfied by inventing a binding merely to enable a match, and solver-selected schedules cannot create implicit source disjunction.

Ruleset templates, query assertions, learned information, enumeration blocks and delivered answers have explicit lifetimes. Model values are not automatically most-general nonground answers. Finite bounds on derivations, choices, identities and terms are different; bounded UNSAT is not global refutation.

A compact globally coupled consistency problem is favorable; sparse deterministic updates and large outputs are adverse. [SMCHR](https://arxiv.org/pdf/1210.5307) demonstrates CHR/SAT integration but assumes range restriction and set semantics and retains CHR matching. [The linear-logic account of CHR with disjunction](https://arxiv.org/pdf/1009.2900) provides a resource-sensitive semantic lead, not an efficient compiler. These are reasons to test a precise eligible fragment, not reasons to transfer their contracts silently.

## Hand traces that distinguish implementation work

Consider `task(K,V), slot(K) <=> result(V)`. Give each task a unique ground key and introduce matching slots incrementally. A permitted application consumes one task and one slot. Generic re-selection can revisit unaffected pairs. Activation can start from the new slot; a key index can locate its partner directly. Generated code may then avoid interpreting the matching plan, but code generation alone does not remove partner lookup. Distinct keys are a fixture property, not a language-wide uniqueness restriction.

Next use `task(X,v), slot(k)` while X is free, then post a justified equation X=k. Before the equation, matching must not bind X to create an application. After it, the application must become discoverable. A static index without binding repair is incorrect; an integrated class store also owes that repair. This trace makes dependency maintenance part of the complete comparison.

Replace consumption with propagation to `seen(V)`, retain duplicate-valued occurrences, and introduce a fresh partner. Exactly-once behavior concerns occurrence tuples, not equal displayed facts. A set-valued representation that conflates the occurrences would pass a ground-value-only check and still implement the wrong contract.

For conditional execution, create a choice before an opaque chain, then discriminate and reject one interpretation through an active off-output obligation. Compare with immediate discrimination and a no-choice chain. Executing the chain before choice creation tests a different phenomenon. Counting only chain rewrites leaves compatibility and publication unanswered.

For direct solving, let each initial chooser explicitly select one of three atoms, with binary table constraints rejecting forbidden assigned pairs. On a finite variable set, an exhaustive assignment oracle determines all valid answers. Supplying every value initially produces a no-OR control. A direct formula can avoid procedural case exploration, but must retain the table occurrences as residuals and justify each choice and terminal result. This is a bounded eligible relation, not a proposed general set semantics.

These traces establish semantic and cost distinctions. They are not timings or proofs that one architecture wins.

## Selection and strongest competing work

**Select R01 activation/code-generation entry as the first implementation question.** Its outcomes distinguish repeated interpretation, partner access and binding/activation costs. Those costs occur in ordinary computation and can also occur inside graph, conditional or solver hybrids. The comparison needs a small compiler/runtime and independent occurrence-level observations; it does not need a full language frontend or a successful sharing engine. Native code generation must be separated from static rewriting of the AST.

**Continue the R04 finite-consistency eligibility and independent oracle design concurrently.** This is a different architectural explanation—solve a compact relation instead of executing its rules. It can be formulated without R01 succeeding. Use a credible available incremental control for an initial gate; do not claim a solver ranking against compiled execution until the relevant compiled control exists. Sparse/dense and no-OR/choice cases distinguish where global reasoning has something to earn.

**The strongest competing implementation is R02 class/index integration with source effects.** It can overturn the dedicated-representation boundary and already has a feasibility basis. Its first decisive experiment requires a competent activation/lookup design on both sides; merely extending explicit equality closure risks another representation-confounded result. This makes the R01 entry the better first implementation use of effort. It supplies a conventional comparison, not the class/index design an integrated engine must use. R02 semantic/index design proceeds independently and must not wait for this compiler or an R01 timing win.

The direct conditional graph has an important unresolved activation/compatibility premise, but its full resource and completion protocol introduces more simultaneous uncertainties. Do not automatically resume its partial prototype. First specify a complete conditional dataflow contrast that measures discovery and publication, not just node sharing. This remains an open R03 candidate.

Additional E14 repetitions, E15 service tuning and equation-worker measurements have less immediate selection value: their bounded tradeoffs are already known and they do not resolve the missing activation/compilation or representation contrasts. They become justified if a surviving architecture makes their uncertainty consequential.

## Parallelism and complexity assessment

Compiled activation offers parallel disjoint instances; integrated classes introduce merge conflicts; conditional graphs can reduce work but concentrate contention on shared nodes. None has a universal advantage. Local confluence of administrative rewrites does not settle two CHR firings consuming the same occurrence. Any local, central or partitioned ownership scheme must account for the actual conflict and propagation semantics.

The first architecture comparisons record required indexes, mutable structures, outstanding obligations, invariants and release boundaries. The relevant simplicity question is what must exist to obtain the observed benefit. No line-count score or speculative industrial deployment requirements are introduced. Parallel execution is selected early only if its protocol or cost can distinguish these candidates; the current R01/R04 entries do not depend on it.

## Next records and bounded disposition

Prepare R01's precise implementation/semantic gate and a separate comparative-cost registration after its candidate/control designs are fixed. Prepare R04's finite fragment and independent oracle without inventing a universal solver lowering. R02 and R03 retain the specific complete-path questions above. No direction is closed by R00, and no production or language choice has been adopted.

Read-only independent reviews covered integrated/conditional execution and direct solver compilation. A separate review of the selection and R01 contract found no material blocker, requiring actual emitted rule structure, independent terminal enabledness and preserved binding-update reconsideration. Root analysis covered generated incremental execution, existing implementation boundaries, cross-candidate comparisons and selection. No experimental runs support this R00 report; its evidence consists of inspected sources, completed bounded results and the explicit analyses above.


## Current evidence update after T032

The [R04 lifecycle pilot](R04-lifecycle-pilot.md) establishes a viable native finite lowering in its exact fragment: all 720 processes complete, with lower native lifecycle in each cold/reuse cell. Whole-process memory and full answer recovery are included; general CHR and native compilation superiority are not established. The large active/global source-search contrast selects a bounded T033 causal scheduling/frontier diagnosis before committing to a storage or conditional protocol. This update governs current priority; the initial selection rationale above records the entry decision. R02 and the remaining R03–R07 questions retain their stated unresolved boundaries.


## Current priority after T033

[Source diagnostics](R03-search-diagnosis.md) establish delayed failure as the branch explosion cause; global source selection supplies the early-rejection control. T034 selects the coherent conditional activation/resource/publication boundary and independent witnesses. That complete organization can change work across choices; further queue tuning cannot answer its central question. Its implementation is selected from the responsibility model, without automatic priority for an existing prototype.


## Conditional implementation entry after T034

[The reviewed protocol](R03-conditional-protocol.md) selects direct symbolic activation with supported occurrence/history ownership and causal source births. Completion and invalidation are local to support so a divergent alternative cannot continually restart a finite sibling's proof. T035 establishes executable projection, resource and progress gates before any cost claim. The initial serial owner and decision-DAG representation are implementation choices; their costs and limits remain part of the comparison.


## Current evidence update during T035

The [direct conditional runtime gate](R03-conditional-runtime.md) now checks complete source execution, resource effects, raw multiplicity and bounded publication beside divergent activity. Direct tuple/dependency activation and supported consumption are executable; their total cost is unresolved. The selected next work establishes true reusable preparation and registers a bounded lifecycle comparison against competent global source search. That comparison has more current decision value than further queue tuning because it tests whether shared physical execution pays for support, completion and retention machinery. T035 remains active; no efficiency ranking follows from its semantic gate.


## Current evidence update after T035

The [conditional lifecycle pilot](R03-conditional-lifecycle.md) validates all 256 processes. Opaque shared work pays for support machinery in these cases, while ordinary and immediately discriminating work is substantially slower; recursive output shows narrower gains. T036 selects causal maintenance diagnosis because allocation traffic grows much faster than retained requested heap, and lower-priority candidate deferral may be avoidable under monotone liveness. This distinction can change how much complexity/cost is inherent before wider architecture recommendations. No universal winner or language adoption follows.


## Current evidence update after T036

[Conditional maintenance diagnosis](R03-conditional-maintenance.md) measures the predicted quadratic deferral and validates a monotone-liveness correction in 384 paired processes. Ordinary overhead is much smaller, opaque sharing gains increase, and discrimination64 becomes inconclusive. T037 selects relation-region eligibility/correspondence: determining when general arbitration and resource machinery can be eliminated without shared-variable or contextual interference. This applies the existing finite-region cost evidence to a consequential language/architecture boundary, ahead of another conditional tuning pass.


## Current evidence update during T037

[Region boundary tests](R05-region-boundaries.md) distinguish specialization preserving source arbitration from eager relation execution. Unique predicate definitions alone fail contextual correspondence; logical finite-answer equivalence does not settle exhaustion or raw multiplicity. The next gate therefore targets actual scheduling-preserving specialization, while stronger eager-region certificates remain separate work. No general eligibility or new performance claim follows from the passing boundary examples.


## Current evidence update after T037

[Checked single-head specialization](R05-single-head-lifecycle.md) preserves source order and full observations while eliminating generic cursor/history work. Its registered pilot supports bounded lifecycle savings without mandatory language restrictions. T038 selects an R07 comparison checkpoint: reconcile viable paths, strengthened controls, responsibility costs and consequential uncertainty before another experiment. Separate-session output ratios cannot select conditional versus specialized execution, and the checkpoint cannot stand in for final goal closure.


## Current selection after T038

[R07 checkpoint](R07-architecture-checkpoint.md) compares viable organizations, necessary responsibilities and all question dispositions. T039 selects ongoing-stream lifetime/cancellation evidence on current conditional and specialized explicit engines. Consumer-retained output and finite controls will distinguish historical runtime retention from required live work. This has greater immediate architectural value than ranking one finite output cell; parallel granularity, mixed phases, compilation and eager-region certificates remain open.


## Current selection after T039

[Ongoing-stream lifecycle](R06-streaming-lifetime.md) identifies consequential conditional publication run-ahead and historical retention, with successful cancellation disposal in both paths. T040 selects support-restricted history traversal and independent correspondence before comparative rerun. Broad reclamation and optional explicit lineage remain alternatives whose priority follows the measured cost.


## Update after T040

[Restricted publication](R06-restricted-publication.md) materially reduces traversal and run-ahead, but retained history and ongoing alias cutoffs remain. T041 selects one bounded publication-flow-control feasibility gate, including the adverse latency of pausing source work behind a large finite observer. This is a scheduling tradeoff, not a bounded-memory claim. Broader mixed-phase and parallel investigations remain alternatives after that distinction.


## Update after T041

[The publication-priority gate](R06-publication-flow-gate.md) establishes exact answers with a bulk-drain sibling-delay tradeoff; no production scheduler change is adopted. T042 selects current-engine mixed-phase generalization ahead of wider scheduling ownership changes and repeated regional parallelism. The broader question dispositions remain active.


## Update after T042

[Mixed-phase comparison](R05-mixed-pipeline.md) supplies a current-engine applicability boundary: opaque pre-discrimination work can pay for conditional bookkeeping, while identical post calls retain separate occurrence work. T043 selects a bounded counter-free regional parallel entry ahead of a new static-compilation experiment, with serial-control and applicability review before repeated measurement. Broader lifetime and language questions remain explicit.


## Entry update during T043

[Regional readiness](R08-regional-readiness.md) separates diagnostic work and message payloads from admission and raw multiplicity. The existing certificate covers permanently disconnected predicate families; the next comparison must include current inferred-specialized serial execution as well as matched-quantum worker controls. Correctness and optimized-code checks pass; comparative registration and runs remain.


## Update after T043

[Regional lifecycle evidence](R08-regional-lifecycle.md) establishes a bounded capacity benefit versus one worker, with unresolved balanced ordering versus Inline and contrary controls favoring Inline. The current specialized serial control exposes large per-application overhead on one-region carry. T044 selects a deterministic causal gate for repeated Indexed structural maintenance before another timing or executor change. Static eligibility remains the strongest broader alternative; parallel expansion is not selected.

## Correction update during T044

[The structural-maintenance gate](R01-structural-maintenance.md) confirms avoidable quadratic Indexed traversal and validates immutable-subtree recognition with linear visits and preserved late-binding behavior. The paired lifecycle registration includes the added metadata cost and persistent controls; no timing improvement is inferred from work counts alone. T044 remains active pending that comparison.


## Update after T044

[The paired immutable-subtree pilot](R01-closed-subtree-lifecycle.md) validates all 288 processes and 9,664 full observations. Four carry-heavy Specialized families have separated favorable before/after ranges; all Inline changes and current balanced worker ordering remain unresolved. Metadata increases Inline requested allocation traffic and is included. T045 selects checked finite recursive-relation eligibility and correspondence, with nonground payloads, failure and contextual exclusions, ahead of more executor tuning. Native compilation lifecycle remains unmeasured; the certified R04 fragment is the alternative if recursive eligibility requires broad analysis. The goal remains active.


## Update after T045

[Finite recursive correspondence](R05-finite-recursive-gate.md) establishes a source-derived sealed certificate with unknown payloads, complete alias/failure observations and no occurrence store or rule selector. The 168-case matrix, mathematical expectations and contextual gates pass in both counter configurations. T046 selects native generation and compilation-lifecycle entry ahead of further executor tuning. The direct loop still interprets equation templates and uses finite-tree services; no native generation or speed claim follows. R04 remains the bounded alternative if artifact integration requires broad infrastructure.


## Entry update during T046

[Native recursive artifacts](R05-recursive-native-entry.md) validate 182 outcomes across seven separately compiled modules. Body operations are generated statements, with no compiled-engine runtime dependency. T046 continues with equivalent executable sessions and prospective compilation-lifecycle registration; debug correctness compilation is not timing evidence.


## Session update during T046

[Shared executable sessions](R05-recursive-session-gate.md) validate 42 complete responses across native, direct and Specialized, preserving unsupported-versus-ordinary-residual behavior. All 60 package tests pass in both configurations. The remaining T046 entry is a reviewable measurement runner and prospective compilation-lifecycle registration, with bundled phases named accurately and external execution caps. No comparative timings have run.


## Update after T046

[Recursive lifecycle evidence](R05-recursive-lifecycle.md) validates all 324 sessions and 62,316 full responses. Checked Direct captures most benefit over Specialized; Native has three separated Direct gains but does not recover its per-source compilation cost within measured reuse. T047 selects a decision checkpoint: distinguish lowering from code generation, retain unresolved current parallel benefit, and reassess conditional/explicit recommendations whose controls precede serial changes. No cross-session ratios or universal architecture winner follow.

## Update after T048

[Current mixed evidence](R05-current-mixed.md) validates all 224 processes. Conditional wins both pre64/post0 pairs; Specialized wins eleven pairs and reused balanced32/32 overlaps. T049 selects an evidence-sufficiency audit across the full goal before further experimental selection or closure.


## Current update after T055

[Carrier complete costs](R05-carrier-cost.md) and the
[known-prefix control](R05-carrier-prefix-cost.md) qualify the pure-countdown
sharing preference. Ground controls favor source lowering; long unknown timings
overlap Conditional, while short unknown endpoints and peak heap favor Conditional.
Independent review finds this bounded contrast sufficient; repeated timings would
not answer a new question. Arena ownership remains a separately bounded option.
The strongest remaining sharing question is substantive equation/constructor
pre-discrimination work, with an observable result and competent lowering control.
Assess it against a current maintained-join bottleneck screen before expanding
implementation. Goal closure remains unsupported.


## T056 source gate and T057 selection

[Substantive equation entry](R05-substantive-work-entry.md) validates current
Conditional/Specialized full aliases and clash exhaustion, plus16versus1 failed
lineages for post-choice versus naive pre-choice failure. This is not a timing
result. E12 gives actual-boundary cache/replay precedent with adverse owned-term
costs; an efficient new cache needs more machinery and a separate justification.
T057 instead selects the [current join work screen](../registrations/R01-current-join-screen.md).
Its existing Global/Active and Scan/Indexed controls can identify or defer a broad
ordinary-CHR retained-join opportunity without implementing maintenance. Current
active nonfirst-head prebinding may already avoid historical repeated-prefix work.
The equation-sharing question remains plausible and is reassessed after this
lower-cost screen; neither prototype becomes an automatic work queue.


## T057 work result and T058 audit

[Current keyed joins](R01-current-join-screen.md) validate32 processes with exact
repeated endpoints and counts. Active+Indexed uses7 candidates and10 cursor steps
per subsequent request at N16 and N128. This family supplies no maintained-state
justification. No timing or all-join claim follows. T058 now reassesses all relevant
questions, especially whether substantive shared-operation costs need a new control
before a bounded architecture comparison. Preserve both the value of competent
current primitives and the possibility of decision-changing reuse; do not demand
every imaginable optimization before measuring or assume a current prototype is
the final control. Goal closure requires the full evidence/value audit.


## Current T058 conclusion

[The all-question sufficiency assessment](R07-sufficiency-current.md) selects T059:
complete substantive common-equation costs with Conditional and ordinary/COW
Specialized controls. The dedicated arena unifier is a competent existing primitive.
E12's adverse cache interface and T056's failed-lineage hoist distinction do not
require an unimplemented cache before measurement. A favorable result would justify
reassessing that possible control; it would not prove intrinsic sharing necessity.
Other directions have bounded cost/value dispositions, subject to this final
comparative evidence. The full goal remains active.


## Renewed S03 lifecycle evidence

The [first S03 lifecycle pilot](S03-lifecycle-pilot.md) completes 210 registered processes with independent complete-answer checks. Direct graph execution favors opaque common work but loses after early discrimination; checked graphless word lowering dominates the graph on its admitted source. Profiling identifies temporary context-map work as a bounded repair candidate, selected before broad rejection or further architecture tuning. Preparation, query lifetime and disposal are included; independent native compilation, ongoing reclamation and general architecture selection remain open under the governing sequence.


## Renewed S03 context repair and next boundary

The [paired context repair](S03-context-repair.md) completes 252 processes and removes about 30% of the graph's discrimination lifecycle cost without changing semantics. The graph remains favorable on opaque work and unfavorable against compiled controls after discrimination; graphless word compilation remains distinct. T062's bounded trial is complete, while S03's broader obligations remain open. T063 selects S02 relational execution: assess a shared constructor/source join plan and choice-local equality, rather than assuming the current integrated control exports solved terms during matching.


## S02 relational head-plan gate

The [relational head-plan gate](S02-relational-head-plan.md) validates constructor-first joins and complete candidate tuples/bindings against an independent exhaustive matcher. This tests a remaining recursive-pattern boundary in the integrated control without inventing a solved-term export step. T063 remains active for actual equality/key maintenance, compatible contexts, resource commits and full source observation; snapshot correctness does not establish those runtime obligations or performance.

## S02 mutable equality/resource owner

The [mutable owner gate](S02-relational-owner.md) passes 1,296 independent ordered equation cases, checking exports and repaired joins after each update. Directed cases establish consuming interleaving and copied-context isolation. These are correctness capabilities, with full source execution, shared contexts and comparative costs still unresolved. T063 remains active.

## S02 complete source gate

The [relational source gate](S02-relational-source.md) passes 46 independently checked finite configurations, including 36 comparisons with the no-choice integrated control, and finite-sibling publication beside ongoing work. This supplies a complete path for a bounded lifecycle comparison; it does not establish a cost advantage or settle shared contextual integration. T063 remains active for prospective cost registration, with S01 and S06 reassessed after the pilot.

## S02 lifecycle pilot

The [140-process pilot](S02-lifecycle-pilot.md) demonstrates a selective-constructor advantage against the integrated control, but substantial broad-match losses and faster dedicated controls throughout the tested families. Repeated full candidate materialization is a concrete profiled cost, selected for a bounded correction overlapping S01's retained discovery. Delayed-selective relational/integrated timing remains inconclusive. No general architecture or contextual-integration disposition follows.

## S02 candidate reuse and S06 selection

The [paired candidate-reuse correction](S02-candidate-reuse.md) completes 175 processes, reducing flat/dense relational lifecycle by about two thirds and delayed-dense lifecycle by 45%, with independent invalidation counterchecks. The selective-constructor benefit survives, while broad comparisons still favor controls. T063's bounded trial is complete; broader S02 obligations remain open. [Direct relation compilation](S06-direct-relation-entry.md) is selected ahead of another local correction because it tests elimination of the execution loop, with S01 retained discovery the strongest ready alternative.

## S06 finite table-source gate

The [source-derived bag-join gate](S06-table-source-gate.md) passes 6,144 configuration/order comparisons per executor, including scalar and generic dedicated controls. Duplicate derivations, aliases, structured arguments, fresh locals, off-output failure and lazy first publication are checked. The accepted finite source eliminates occurrence/activation/history responsibilities; certificate, index, substitution and observation costs are not yet measured. T064 remains active for prospective lifecycle comparison; R04 translation and broader S06 mechanisms remain distinct.

## S06 table lifecycle and next discovery contrast

The [252-process finite-table pilot](S06-table-lifecycle.md) favors direct source compilation over both generic controls in every registered one-query and 16-query cell, including certification, query lifetime and disposal. This supports the admitted fragment without selecting language restrictions, a universal relation solver or native compilation. T064's bounded trial is complete. [S01 selective and consuming discovery](S01-next-selection.md) is selected next, with S04 restoration/replay the strongest ready independent alternative.

## S01 selective and consuming source gate

The [selective/consuming lowerings](S01-selective-consuming-gate.md) pass independent source and Global-control checks across the registered 144-configuration grid and directed binding/ownership cases. Direct execution uses composite access; retained execution correctly invalidates all partners of consumed rows. Stable selective requests exhibit fewer retained visits than direct lookups, with construction and invalidation still to price. T065 remains active for prospective lifecycle comparison; work counts do not rank architectures.

## S01 lifecycle and script ownership

The [224-process initial pilot](S01-selective-lifecycle.md) exposed a shared quadratic script-copying cost. The [336-process paired correction](S01-script-ownership-repair.md) removes most long-script overhead while preserving exact blocked-driver residuals. Both lowerings then beat generic controls throughout the registered sessions; dense consumption makes Retained about 40% slower than Direct. Other retention contrasts remain below the practical threshold or noisy, so T065 selects a focused request-frequency crossover rather than closing S01.

## S01 request crossover and S04 selection

The [288-process crossover](S01-request-crossover.md) establishes a practical retention benefit at N32/R64 stable selective requests (paired ratio 0.779), and a strong dense-consuming loss at the same size (5.619). This supports a source-dependent tradeoff without inventing workload weights or automatic routing. T065's bounded comparison is complete; broader S01 remains open. [S04 restoration/replay](S04-restoration-entry.md) is selected next, with S05 stable-identity reuse the strongest ready alternative.

## S04 source restoration gate

The [S04 restoration gate](S04-restoration-source-gate.md) establishes independent complete-answer agreement for copying, reversible paths, root replay and three periodic checkpoint intervals, with finite-sibling publication. Existing compiled Global controls pass the finite gate. Diagnostic omissions of binding, resource, history and pending restoration all cause release-mode semantic failures. These are viable candidates for the next prospective lifecycle comparison; no comparative S04 costs have run. T066 remains active, and S05 stable-identity reuse remains the strongest ready alternative to reassess after the bounded comparison.

## S04 lifecycle and attribution update

The [1,008-process S04 pilot](S04-lifecycle-pilot.md) passes full answers, exact allocation replay and disposal checks. Copying wins the retained-store runtime/traffic/peak comparison against existing indexed execution; mutation favors indexed runtime and traffic while copying uses lower peak heap. The tested trail has substantial runtime and retention costs. A [registered matcher diagnostic and correction](S04-matcher-copy-gate.md) identifies and eliminates 36,864 rejected-partner frame copies on mutation without changing source work or answers. Paired costs, including all-compatible and long-alias overhead, remain necessary before interpreting the corrected organization. T066 stays active; no architecture or general state policy is selected.

## S04 paired correction and S05 selection

The [672-process matcher comparison](S04-matcher-paired-cost.md) cuts mutation lifecycle to 11% of its previous time for Copy and 53% for Trail, with exact allocation corroboration and a repeatable 11–15% compatible-alias overhead. Peak heap is unchanged. This resolves the immediate confound without ranking all restoration policies. T066's bounded trial is complete; [T067 stable-identity reuse](S05-stable-reuse-entry.md) is selected next against corrected replay policies, with broader S04 obligations preserved.

## S05 stable-identity kernel gate

The [kernel gate](S05-stable-kernel-gate.md) checks exact-context and dependency-valid operation reuse over a common constructor owner against 3,888 independent ordered-equation cases. Unrelated binding changes distinguish useful reuse between the policies, while three faulty validity/replay variants fail semantic assertions. This establishes a bounded correctness mechanism, with full source effects, wake-ups, lifetime and direct-sharing costs still required. T067 remains active.

## S05 complete source gate

The [source gate](S05-stable-source-gate.md) validates shared-handle equation interception in the existing scalar machine across 576 registered configurations and directed cache-hit effects. Cached bindings enable consumption, common cached failure leaves a finite sibling, and owner checks reject foreign cursors/arenas. Source-level replay/failure faults are rejected. T067 remains active for prospective lifecycle costs, explicit preparation/query ownership and the direct-sharing comparison; this is correctness evidence, not a measured optimization.

## S05 lifecycle and strong-control comparison

The [588-process pilot](S05-lifecycle-pilot.md) and [336-process inferred-specialization comparison](S05-specialized-control.md) find large equation-work/traffic savings from dependency reuse but smaller runtime gains at depth64; distinct requests lose about 54–56%. Ordinary shared-arena scalar execution remains a necessary control, while the direct graph retains a favorable common-failure peak/time case. These findings motivate an operation-size/request crossover before deciding whether reuse can economically substitute for sharing. Broader invalidation, eviction, lifetime and architectural attribution remain open; T067 stays active.

## S05 operation crossover and next matching contrast

The [616-process crossover](S05-operation-crossover.md) passes complete observations, exact allocation replay in all 88 cells and full disposal. At 64 common requests, depth256–1,024 supports roughly 25–35% before-discrimination runtime savings; distinct depth1,024 requests take 2.75 times ordinary runtime. Graph common-clash peak/traffic remains lower, and the after-discrimination clash timing remains uncertain. Native compilation and broader cache validity/lifetime are not resolved. T067's bounded trial is complete; T068 selects [demand-driven subscriptions](S01-subscription-entry.md), with warm parallel workers and corrected replay policies retained as consequential alternatives. The architecture goal remains active.

## S01 demand lifetime and tuple-maintenance gate

The [subscription gate](S01-subscription-kernel-gate.md) establishes 44 source configurations against independent execution and both compiled Global controls. Indexed, eager and demand-driven tuple kernels agree with an independent product oracle over 32 update sequences each; three deliberate retention/multiplicity/invalidation faults fail release checks. Subscriptions avoid inactive retention and repeated discovery, while middle updates still examine active demand keys. This is mechanism and correctness evidence, not cost or full source-correspondence evidence. T068 remains active for source integration and prospective lifecycle comparison.

## S01 subscription source correspondence

The [integrated source gate](S01-subscription-source-gate.md) checks 49 configurations in all three join policies and two service quanta, with independent full answers, changed preparation queries, cancellation and failure cases. Four binding/resource/order faults fail release checks. The existing single-head specializer admits no region on this multihead source, so it supplies no additional specialized control; broader generated access remains open. Candidate materialization, binding scans and all query owners must be charged in the prospective lifecycle experiment. T068 remains active; no cost ranking has run.

## S01 subscription lifecycle and remaining favorable witness

The [1,008-process pilot](S01-subscription-lifecycle.md) passes exact allocation replay and full lifetime restoration in all 144 cells. Subscriptions beat eager retention practically in six cells but not indexed rediscovery; ten indexed-relative losses include consumption and reopening. Compiled execution retains lower-peak countercases despite substantially slower runtime. T068 remains active: [costly low-yield discovery](S01-subscription-low-yield-entry.md) is still missing, and its control must use both bound endpoints where advantageous. This bounded follow-up takes priority over warm workers or corrected restoration because it can change the current subscription interpretation; those alternatives remain required.

## S01 low-yield retention and warm-worker selection

The [861-process follow-up](S01-subscription-low-yield.md) passes exact allocation replay and full restoration in 123 cells. Competent two-endpoint access still repeats costly discovery: N16/R64 stable subscriptions use 30 percent of indexed lifecycle time, with eager retention similarly fast. Reopening favors eager retention; earlier inactive/consuming cases preserve contrary maintenance costs. Contemporaneous old/new indexed controls establish no practical access-plan regression on the three carried sources. T068's bounded trial is complete; [T069 reusable workers](S09-reusable-workers-entry.md) is selected against broader generated multihead access and corrected restoration. Broader S01 and the architecture goal remain open.

## S09 reusable worker ownership gate

The [worker lifecycle gate](S09-worker-lifecycle-gate.md) preserves prepared source across changing region queries, with generation/pool identity, explicit cancellation and end-query release. Three deliberate retention/identity faults fail release checks; counter-free repeated tests and the ordinary 30-query source gate pass. Worker failure remains distinct from logical refutation. The current hardware inventory permits multiworker investigation, and the existing meter is process-global, but neither establishes scaling or cross-thread phase validity. T069 remains active for complete regional/product correspondence and prospective cold/reused lifecycle costs.

## S01 source-generated continuation gate

The [native continuation gate](S01-native-continuation-gate.md) establishes that source-derived rollback and ordered range traversal can avoid generic frame snapshots, candidate vectors and key-template interpretation. The 192-case finite corpus, independent 112-case artifact, branch replay and live-demand checks preserve the tested source effects; three deliberate selection/rollback faults are detected. Compilation, generated update repair, range-lookup costs and complete lifetime costs remain open. T070 continues toward a competent indexed/retained comparison; no architecture ranking follows from eliminating those work counts.

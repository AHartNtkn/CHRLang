# R00: architectural alternatives and first experiment selection

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

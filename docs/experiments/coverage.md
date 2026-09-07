# Coverage audit for the experimental sequence

The [sequence](sequence.md) assigns the outstanding questions from all seventeen research directions to experiments or specific owner decisions. No experimental family is considered resolved by this planning document. The coverage check below concerns the research dossier's known questions; new evidence can add questions.

The numbered directions follow [T018's disposition audit](../goals/chr-sharing/notes/T018-final-direction-audit.md). E00–E17 refer to work packages in the sequence. D1–D6 are owner decisions defined afterward. A dependency on an experimental artifact is distinct from a decision to adopt the language feature being tested.

## Direction-by-direction coverage

### 1. Reference semantics and ordinary CHR behavior

**Remaining:** concrete machine correctness, preservation of explicit alternatives and source resources, guard behavior, and any additional primitive catalogue.

**Assignment:** E00 supplies independent expected observations and transition checks, including nonconfluent committed schedules and before-dedup lineage. Every candidate consumes those obligations; E15 checks composition. D1 covers a new primitive API. Current equality-entailment guards allow the experiments to proceed without that choice.

**Evidence/disposition:** [reference specification](../goals/chr-sharing/notes/T008-reference-and-candidates.md), [composed simulation](../goals/chr-sharing/notes/T016-composed-machine-argument.md), and the executable [implementation record](../implementation/reference-interpreter.md) supply the starting contract. Existing tests do not discharge conditional transitions or prove general fairness. Those questions remain experimentally active. [E00](results/E00.md) now supplies 64 executable observations and replay. Generated/coupled transition validation remains active.

### 2. Conditional multiset execution

**Remaining:** projection and lineage fidelity, conditional cycles, grouping, complete incremental matching/wake-up, support representation, histories, memory and bookkeeping cost.

**Assignment:** E03 tests finite supports before incremental discovery and BDD refinements; E00 supplies adversarial semantic cases; E01 supplies storage controls; E09 tests compact scheduling separately. E03 requires no favorable graph or net result.

**Evidence/disposition:** [kernel](../goals/chr-sharing/notes/T011-conditional-kernel.md), [matching/wake-up](../goals/chr-sharing/notes/T012-matching-and-wakeup.md), and [T013](../goals/chr-sharing/notes/T013-experiment-registration.md) are constructive inputs. [E03 v2](results/E03.md) reconciles the registered guards/runtime and demonstrates bounded common expansion with substantial support/matching costs and an adverse grouping case. General projection, incremental services and compressed supports remain active.

### 3. Contextual relation graphs and cached expansion

**Remaining:** common expansion versus per-context commit cost, fresh identities, equality and wake-up traffic, region closure and retention.

**Assignment:** E04 separates dispatch from cached expansion. E02 tests closure and reformulation costs. E05 supplies a contrasting equality representation; E15 checks relation/solver interfaces. These do not require E03's success or optimized support backend.

**Evidence/disposition:** [relation construction](../goals/chr-sharing/notes/T015-relation-graph-construction.md) and [composition](../goals/chr-sharing/notes/T018-composition-and-learning.md) establish the operation contracts. Empirical reuse frequency remains open, especially on synthesis. [E04](results/E04.md) separates measured local dispatch from asynchronous event reuse and exposes cache retention under sparse reuse. Mixed-region interfaces and wider synthesis remain active.

### 4. Named superpositions and HVM-inspired execution

**Remaining:** family-service costs; source versus administrative label correspondence; generated code; normalization/collapse yields; label lifetime; failure and quiescence beyond the output graph.

**Assignment:** E05 tests first-order native families. E06b tests encoded-machine backend execution. E06c tests native compiler correspondence, including graph/label traces. E09 tests finite service; E15 tests active effects. These are three distinct paths, not a chain requiring an encoded machine to be fast first.

**Evidence/disposition:** [family services and backend boundaries](../goals/chr-sharing/notes/T018-backend-boundaries.md) provide independent contracts. Native correspondence is an implementation/proof task in E06c; neither a backend name nor scalar ground examples close it. [E05](results/E05.md) measures frozen named equality against eager and exact-partition controls with assignment-level reference checks. Dynamic births/effects and backend correspondence remain active.

### 5. Ordinary interaction nets and richer local calculi

**Remaining:** emitted agents/rules, service and fan traffic, equality lookups, local dispatch restrictions, and whether richer interactions materially change the service cost.

**Assignment:** E06a tests the direct and net controllers. E02 compares global restrictions and regional compilation; E05 compares full and certified-safe unification. E06a also evaluates richer service proposals against concrete operation costs before choosing a protocol to implement. E16 handles parallel cost separately.

**Evidence/disposition:** [finite encoding](../goals/chr-sharing/notes/T016-finite-net-service-encoding.md), [locality/equality](../goals/chr-sharing/notes/T016-net-locality-and-equality.md), and [compiler source assessment](../goals/chr-sharing/notes/T016-compiler-source-assessment.md) support this split. Nonoverlap settles dispatch only. Source-access limitations must not be represented as a measured negative result for a calculus.

### 6. Andorra-style delayed splitting

**Remaining:** profitable split policy, resource-aware promotion, speculative work, state distribution and progress.

**Assignment:** E07 compares early, late and bounded adaptive preference with both opaque continuations and early-failing producers. E09 and E15 test scheduler interaction. E07 can use simple boxes independently of the conditional engine.

**Evidence/disposition:** [Andorra construction](../goals/chr-sharing/notes/T017-andorra-and-decomposition.md) supplies the lifting certificate and deterministic-first counterpressure. A single favorable carry example cannot choose a policy. [E07](results/E07.md) measures certified ancestor lifting across quotas, including wasted speculative work and unbounded-preference refutation starvation. Full child boxes, promotion and adaptive policy remain active.

### 7. AND decomposition and reunion

**Remaining:** certificate precision/cost, independence frequency, fair product enumeration, provisional state retention, late joins and aliases, graph maintenance and product reconciliation.

**Assignment:** E08 has separate persistent, inferred and temporary-reunion comparisons. E02 tests expressibility and certificates; E15 checks cross-region effects; E17 measures occurrence in intended applications. D3 covers user-visible factored answers; internal factoring does not need that choice.

**Evidence/disposition:** [persistent decomposition](../goals/chr-sharing/notes/T017-andorra-and-decomposition.md) and [temporary protocol](../goals/chr-sharing/notes/T017-temporary-decomposition.md) have different completion obligations. Success of persistent products cannot close reunion. [E08](results/E08.md) measures conservative permanent inference and incremental products, with negative small/connected cases. Finer ownership, stream storage and temporary reunion remain active.

### 8. Fair symbolic scheduling and encapsulation

**Remaining:** queue/support fragmentation, huge finite services, asynchronous completion, collapse overhead, grouping effects and response latency.

**Assignment:** E09 compares three schedulers on equivalent finite services; E07 varies splitting preference; E06 supplies backend yield evidence; E14 accounts for output work. E15 requires a second service organization before generalizing scheduler results. D4 covers stronger user-facing order or latency promises.

**Evidence/disposition:** [rounds](../goals/chr-sharing/notes/T015-symbolic-scheduling.md), [asynchronous jobs](../goals/chr-sharing/notes/T016-asynchronous-symbolic-scheduler.md), and [search encapsulation](../goals/chr-sharing/notes/T018-backend-boundaries.md) state finite-service premises. Timing tests do not replace that argument, and fairness does not imply a practical latency bound. [E03](results/E03.md) exposes repeated group-preview cost; [E07](results/E07.md) exposes unbounded-preference starvation; [E08](results/E08.md) checks a finite product prefix with an infinite producer. None closes resumable-service or latency questions.

### 9. Logical structural solvers and exact projection

**Remaining:** exact membership/entailment/projection cost, domain assumptions, residual replay, repeated holes, and host occurrence boundaries.

**Assignment:** E10 compares direct rules, exact operation reuse and logical interfaces with separately declared observations. E12 tests certificates and caller filtering; E15 tests composition. D2 and D3 concern adoption of closed logical regions and formula observations, respectively.

**Evidence/disposition:** [predicate certificates](../goals/chr-sharing/notes/T015-solver-certificates.md) and [exact projection](../goals/chr-sharing/notes/T016-exact-solver-projection.md) distinguish semantic formulas from CHR resources. Baseline-preserving operation reuse can proceed while adoption decisions remain open. The current implementation work is registered in [E10](registrations/E10.md). Closure and formula-observation adoption are not dependencies for baseline-preserving operation experiments.

### 10. Equality-constrained term spaces and whole-evaluator compilation

**Remaining:** repeated-hole correlation, intersection/enumeration costs, partial answers, complete transition-encoding size, solver work and reuse across bounds.

**Assignment:** E10 tests ECTAs as structural/candidate-space services. E11 independently tests full finite execution witnesses and increasing bounds. E17 compares the resulting complete candidates. D3 covers exposing arbitrary automaton residuals.

**Evidence/disposition:** [term spaces](../goals/chr-sharing/notes/T017-equality-constrained-program-spaces.md) and [bounded machine compilation](../goals/chr-sharing/notes/T018-bounded-symbolic-compilation.md) supply different claims. Neither a cyclic grammar nor a program-size bound proves evaluator termination. E11 does not wait for E10 to win.

### 11. Tabling, learned failure and reconvergence

**Remaining:** exact keys, generalization, caller pruning, complete-state equivalence, event-dependent contradiction certificates and retention cost.

**Assignment:** E12 isolates exact tables, state merge and failure learning before stronger variants. E10 supplies the exact logical interface where needed. E15 measures interactions with solving and contextual execution; E14 remains a separate final-answer operation.

**Evidence/disposition:** [state reuse](../goals/chr-sharing/notes/T016-observation-and-state-reuse.md) and [learning](../goals/chr-sharing/notes/T018-composition-and-learning.md) provide sufficient certificates. Output equality and table-hit counts alone answer none of these cost or continuation questions.

### 12. Answers, residuals and deduplication

**Remaining:** quiescence in actual compositions, exact canonicalization cost, symmetric residuals, output latency, key retention and possible logical projection.

**Assignment:** E00 specifies independent observations; E14 compares exact normalization and retention; E15 verifies global completion; E10 tests logical projection under its own interface. D3 is required to adopt a different displayed/projected answer contract.

**Evidence/disposition:** the [reference implementation](../implementation/reference-interpreter.md) tests exact structural answers; [observation analysis](../goals/chr-sharing/notes/T016-observation-and-state-reuse.md) explains stronger obligations. Full residual answers remain the executable comparison default without requiring a new owner decision. [E14](results/E14.md) supplies an independently checked exact observer. [E01](results/E01.md), [E03](results/E03.md), [E05](results/E05.md) and [E08](results/E08.md) expose materialization/retention costs. Symmetric negatives, stream storage, shared exact outputs and global composed quiescence remain active.

### 13. Language restrictions, inference and compiler regions

**Remaining:** actual source coverage, global versus local restrictions, annotation/reformulation burden, certificate inference, linking and each restriction's runtime benefit.

**Assignment:** E02 inventories and pairs programs; E04/E06 measure nonoverlap/locality; E05/E06 measure ownership/disjointness; E10 measures logical regions; E09 measures finite services; E11 measures local bounds; E13 measures static specialization; E15 validates linking and cross-region effects. D2 governs adoption.

**Evidence/disposition:** [language opportunities](../goals/chr-sharing/notes/T016-language-opportunities.md) identifies independent properties, rather than a mandatory bundle. Every benefit claim must name the service avoided and an affected real program. Experimental evaluation of a restricted language is permitted without changing the project's chosen language. [E02](results/E02-initial-inventory.md), [E04](results/E04.md), [E07](results/E07.md) and [E08](results/E08.md) now provide concrete certificate evidence and coverage limits. The relation certificate accepts 56 of 64 E00 cases; permanent factoring keeps same-predicate invocations together. Broader inference and reformulation comparisons remain active.

### 14. Partial evaluation and specialization

**Remaining:** profitable unfolding, variant size, compilation cost, all-mode preservation, active effects and interaction with sharing.

**Assignment:** E13 establishes finite specialization on a competent scalar engine; E15 repeats equivalent opportunities on a shared engine; E17 includes cold and reused compilation costs. E02 supplies eligibility and programming-cost cases.

**Evidence/disposition:** [specialization](../goals/chr-sharing/notes/T018-specialization.md) supplies a conservative construction and explicit limits. A more general theorem requires its premises to be established for the selected region; it cannot silently cover arbitrary CHR.

### 15. Storage, parallel execution and whole-system cost

**Remaining:** competent baselines, restore/replay tradeoffs, allocation/reclamation, retained memory, service overhead, composition, synchronization, bandwidth and useful parallel granularity.

**Assignment:** E01 supplies persistent/trailing/replay controls; every mechanism counts administration; E14 accounts for output; E15 measures whole-engine costs; E16 measures sequential versus parallel behavior; E17 compares complete application costs.

**Evidence/disposition:** [cost questions](../goals/chr-sharing/notes/T016-comparative-cost-questions.md) and the [portfolio](../goals/chr-sharing/notes/T018-evaluation-portfolio.md) distinguish the work units. The copying reference's speed is not an architectural baseline result. Hardware restrictions, if encountered, must name the exact remaining measurement. [E01](results/E01.md) establishes a persistent/copy control with identical execution work and measured allocation differences. Subsequent receipts separate event, service, cache, projection and output costs. Trailing/replay, reclamation, composition and physical parallelism remain active.

### 16. Arithmetic, SK, typing and lambda demonstrations

**Remaining:** actual forward/backward streams, nonground restrictions, further answers, sharing frequency and realistic synthesis cost. The user's SK typing relation adds type-directed and proof-like synthesis to the application mix.

**Assignment:** E00 registers these modes with independent witnesses and the pinned literal lambda translation. Every relevant mechanism uses application probes; E17 evaluates whole sessions and complete candidates. D5 concerns an additional conventional capture-avoiding lambda demo, not the supplied relation.

**Evidence/disposition:** [rwlog translation](../goals/chr-sharing/notes/T018-rwlog-translation.md), [intended programs](../goals/chr-sharing/notes/intended-programs.md), current `chr-programs` and its executable tests define present coverage. The unbounded duplication search remains open; ground evaluation or a finite candidate grammar does not close it. [E00](results/E00.md) validates arithmetic modes, ground/residual SK cases, a typing prefix and literal lambda cases. [E04](results/E04.md) and [E08](results/E08.md) include adverse connected/application results. Full synthesis streams and session comparisons remain active.

### 17. Query interface, streams and future features

**Remaining:** surface syntax and query-level equations/OR, extra primitive APIs, optional first-class search controls, future incremental stream/real-number use and higher-order/effectful features.

**Assignment:** D1 covers primitives, D3 observations, D4 exposed search guarantees and D6 query/future scope. E09 tests internal search encapsulation; E17 replays exploratory sessions without selecting notebook UI. No syntax decision blocks ruleset/query AST workloads.

**Evidence/disposition:** [finite stream protocol and language opportunities](../goals/chr-sharing/notes/T016-language-opportunities.md) and the original [goal](../goals/chr-sharing/goal.md) establish that finite terms suffice for request-driven future work. Building real-number libraries or adopting coinductive observations expands scope and needs owner intent. It is not a hidden prerequisite for execution comparisons.

## Owner decisions and when to ask

These questions are recorded, not presented as a questionnaire now. Ask when a concrete proposal's consequences are demonstrated, or when the owner chooses to expand scope. A pending adoption decision blocks adoption, not experiments that retain the baseline or explicitly compare the proposed language.

- **D1 — Primitive and guard catalogue.** Additional pure entailment theories or host primitives determine decidability, finite service and yielding requirements. Ask with the proposed operation and semantics when an application requires it. Equality-only cases continue.
- **D2 — Language properties as requirements or optimizations.** Global single-head/nonoverlap, ownership/modes, declared logical regions, finite domains and local scheduling guarantees have different expressibility costs. Bring E02's paired programs and the relevant measured service benefit before recommending any individual adoption. Inferred and experimental restricted variants continue independently.
- **D3 — Answer interface.** Full residual multisets, exact logical projection, formula/automaton residuals and factored products have different meanings and display costs. Show the same query's concrete outputs and the preservation argument. Until decided, report full residual answers; internal exact operations remain available.
- **D4 — Stronger search guarantees.** A specific answer order, latency promise or first-class search operation changes policy constraints. Show E09's measured consequences before asking for a stronger promise. Fair interleaving under finite-service assumptions remains the target without requiring an order choice.
- **D5 — Additional lambda demo semantics.** Faithfully translating the supplied relation proceeds in E00. Choosing a conventional capture-avoiding representation is a separate application design decision only if that additional demo is wanted.
- **D6 — Surface and future scope.** A parser, direct query equations/OR, notebook UI, incremental stream handles, real-number libraries, higher-order values or effects require a concrete usage goal. Current ruleset/query construction and session replay need none of these choices.

No question about taste needs to be answered to start E00/E02. Final priorities among measured latency, memory, compilation and expressiveness tradeoffs may require the owner; E17 must first provide concrete alternatives with evidence.

## Portfolio cross-check

The eleven comparisons in [T018's portfolio](../goals/chr-sharing/notes/T018-evaluation-portfolio.md) have the following primary owners. Their supporting experiments are specified in the sequence.

1. Conditional execution: E03.
2. Occurrence-local relation graphs: E04.
3. Named family services and backend integration: E05 and E06b/E06c.
4. Finite net services: E06a.
5. Delayed splitting: E07.
6. Decomposition: E08.
7. Schedulers: E09.
8. Structural solving and ECTAs: E10.
9. Whole-evaluator symbolic compilation: E11.
10. Tabling, reconvergence and learning: E12.
11. Specialization: E13.

E00/E01 establish observations and controls, E02 investigates language costs, E14 covers answer costs, and E15–E17 cover composition, parallelism and application decisions. These additional tasks close coverage gaps that a list of mechanism prototypes alone would leave.

## Dependency and interpretation review

The sequence has no requirement that E03 succeed before graphs, families, boxes, decomposition, solvers, whole-machine compilation, exact state reuse or specialization can begin. E06 does not require a fast encoded-machine result before native integration. E11 does not require a successful structural ECTA solver. E09 needs one finite-service implementation, not the completion of all engines. E15 needs validated mechanisms, not universally winning ones.

Each candidate has both an opportunity case and a nearby case challenging eligibility, overhead or semantics. The measurement rules distinguish prepared prefixes, natural runs, scalar steps, service work, bounded absence, output volume and nonconfluent policy differences. Source restrictions have an expressibility comparison as well as a runtime comparison.

The first review pass identified two additional gaps and assigned them explicitly: reference tests do not establish candidate subscription/race correctness, and application success on ground SK witnesses does not establish unbounded or nonground synthesis coverage. E00/E03 and E17 now own those obligations. This is a local planning review, not an independent review or validation of unimplemented algorithms.

Before each execution receipt, update the affected question's disposition with its evidence and limits. Before an overall recommendation, re-audit every direction above. Any direction still awaiting a buildable experiment remains unfinished; one engine's production readiness or one successful synthesis result cannot close it.

## Live experimental evidence

[E00 v1](results/E00.md) validates 64 registered scalar observations and records remaining transition, generation and synthesis obligations. [E02 initial inventory](results/E02-initial-inventory.md) identifies concrete compiler opportunities and the norm/var occurrence-reintroduction boundary. Neither direction is closed. [E14](results/E14.md) supplies the independent observer used by subsequent controls and candidates; its broader observation-cost questions remain open.

[E01 v2](results/E01.md) now supplies an independent persistent/copy snapshot control: 278 isolated runs passed with deterministic replay. It exposes output materialization as a major retained-memory cost in large residual answers, adding a concrete E14/E15 follow-up. Trail/replay, reclamation and broader timing remain open. E03's first finite-support results are recorded below; its broader refinements remain open.

[E03 v2](results/E03.md) completes the first finite-support probe, with 314 passing isolated runs and exact replay. Common expansion is preserved, but eager grouping is adverse under constructor discrimination; answer-resolution copying was repaired and remeasured. Support/indexing/wake-up/projection and output questions remain open. E04 occurrence-local dispatch/cache is active, independently of E03 refinements.

[E04](results/E04.md) records 447 passing graph runs and exact replay. Local dispatch and cached expansion are distinct gains; sparse reuse increases retained cache memory. Eight E00 programs require broader or mixed-region support. Regional compilation, wake-up, cache policy and synthesis remain open. E05 frozen named-family equality is active.

[E05](results/E05.md) records 240 v1 and 360 v2 passing service runs and exact replay. Named demand, eager projection and exact supported partitions distinguish unification work from construction/extraction cost. Dynamic births/effects, larger graphs and resumable observation remain open; this is not a full CHR engine. E07 delayed splitting is active.

[E07](results/E07.md) records 930 passing quota runs and exact replay. Certified ancestor lifting trades duplicated work for speculation; unbounded preference hides a finite refutation. Full child boxes, broader certificates and adaptive preference remain open. E08 permanent factor inference/product observation is active.

[E08](results/E08.md) records 206 passing factor/scalar runs and exact replay. Permanent independence avoids product recomputation; small and connected application cases expose overhead and certificate limits. Ownership refinement, temporary reunion and long-stream storage remain open. [E10](results/E10.md) now records 150 operation and 270 bounded-space configurations plus exact replay. Identity memoization and selective intersection help; unshared inputs and nondeterministic overlap expose costs. Wake-up integration, richer constraints, lazy spaces and synthesis remain open. E11 whole-evaluator compilation is active.

[E11 initial conformance](results/E11-conformance-initial.md) now covers eight symbolic equality/guard cases with independent witness replay and explicit cutoff checks. Profiling identified avoidable construction work; exact reachability pruning resolves two initial exploratory timeouts. Broader symbolic application coverage, equal-bound controls and comparative measurements remain active.

[E11 registry/bound inventory](results/E11-registry-and-bounds.md) extends symbolic coverage to 23 complete small-bound cases, with 41 explicit cutoffs. Trace-derived application probes time out during construction; a short-prefix profile attributes most cost to heap-view projection. Concrete projection/model decoding and allocation representation are feasible next investigations; application coverage remains open.

[E11 projection follow-up](results/E11-projection-followup.md) preserves small-bound registry outcomes and produces the expected type-synthesis prefix, with other paths resource-bounded. Addition remains a construction timeout. Direct term-expression encoding is now the next competing representation investigation; no E11 closure or architecture adoption follows.

[E11 term encoding](results/E11-term-encoding.md) completes forward addition with independent replay and preserves all small-bound registry outcomes. Relational decomposition still times out in construction; reachability-query cost dominates the completed addition run. Equal-bound direct controls, increasing bounds, prefix reuse and larger applications remain open.

[E11 matched execution](results/E11-matched.md) records 48 registered runs: direct execution is cheaper on all completed pairs, and symbolic decomposition times out in all three repetitions. Actual opaque continuation workloads supplement the zero-rewrite control. Compatible transition extension passes independent checks. E11 remains open for measured prefix reuse, growing bounds, timeout attribution, stronger scalar controls and larger applications; E12/E13 first probes do not depend on resolving these refinements.

[E12 operation gate](registrations/E12.md) validates a solver-independent alpha-canonical equation table against 576 finite operand pairs in two caller namespaces plus adverse cases. No performance result follows yet; operation costs, host integration, full-state reconvergence, lineage and provenance-checked failure learning remain active independent investigations.

[E12 fixed-ID continuation results](results/E12-continuations.md) contain 208 passing configurations and exact non-time replay. Duplicate workloads avoid transitions while retaining every lineage job; ordinary distinct-choice and intended application cases show no transition reduction with this conservative key. Key/edge retention can increase peak and final storage despite lower allocation traffic. Alpha-equivalence, reclamation, call tables, operation costs and failure learning remain open.

[E12 structural failure reuse](results/E12-failure-native.md) adds a 300-configuration three-mode comparison and exact replay. Selective shared-arena checks preserve certificate hits while avoiding whole-operand projection; long clash/occurs probes improve, early clashes expose overhead, and SK timing remains inconclusive despite lower work/allocation. Certificates requiring intermediate bindings, richer assumptions, indexing/lifetime and composition remain open. [Projection diagnostics](results/E12-failure-projection.md) retain the operand-order correction and its evidence.

[E12 equation tables](results/E12-equations.md) complete the first three-ablation package with 336 passing configurations and exact replay. Tables improve repeated owned-tree equations but lose to the shared-arena control on the principal repeated workload; changing inputs and identity operations expose key/interface and retention costs. Shared-representation keys, call tables, alpha-state keys, reclamation, richer failure witnesses and longer synthesis remain open. E13's first probe now proceeds independently of those refinements.

[E13 initial gate](results/E13-initial-gate.md) implements bounded entry specialization with a single-active-call certificate. Twenty of 64 cases pass its first inventory, including all arithmetic directions. A fixed-selector termination counterexample makes the scheduling boundary explicit without rejecting other permitted CHR schedules. Cost comparison, multi-call/source-boundary handling, structural dispatch, variant policy and intended synthesis applicability remain active; this certificate is not an adopted language restriction.

[E13 cost matrix](results/E13-costs.md) adds two 900-row batches with exact non-time replay. Dispatch reductions do not recover compilation allocation cost on any terminating workload within three searches. Static-argument elimination, shared/staged syntax, broader certificates and synthesis applicability remain feasible follow-ups. E06 backend service probes now proceed independently; E13 is not closed.

[E06a data gate](results/E06-data-gate.md) now supplies literal finite active-pair services for comparison, copy, erase and preserving table lookup, with 18,818 comparison/order checks and finite-yield validation. [Richer-target assessment](results/E06-directions-initial.md) identifies distinct multiport inspection and routed fusion protocols still requiring probes. The registered lookup matrix, complete unification service, source regions, encoded backend and native choice correspondence remain active independent questions.

[E06a lookup results](results/E06-lookup.md) record 135 passing configurations and exact replay. Whole-table duplication and erasure dominate first-entry service work, independently of key matching. A preserving-prefix protocol is the next concrete ablation; binary IDs, slot reclamation, complete binding services, regions, richer protocols, encoded backend and native choice correspondence remain open. No source or production decision follows from instrumented Python timings.

[E06a preserving-prefix ablation](results/E06-lookup-preserving.md) passes 180 configurations plus exact replay. It eliminates unrelated payload copying/erasure, reducing large first-entry lookup from 16,836 to 75 interactions with identical outputs. Unary keys, storage management and full unification remain open. E06b continuation-boundary execution now advances independently of further service refinements.

[E06b continuation gate](results/E06-hvm-boundary.md) builds the pinned HVM4 runtime and validates 32 positive finite data-state returns plus four adverse no-return observations, with state/outcome/counter replay. Runtime inspection explains why constructor/lambda wrappers cannot hide recursive work from normalization. General source-machine compilation, resume/storage costs, native labels and unbounded scheduling remain open; a tiny finite controller is not the encoded CHR engine.

[E06a finite-tree service](results/E06-unification.md) implements actual net-rule dereferencing, occurs traversal and private transactional unification. Thirteen tests and 864 independent reference comparisons pass with exact replay, including table preservation on failure and nonground MGU relationships. Comparative full-service costs, wake-ups, branch support, regions, richer protocols and both backend compilation paths remain open; service conformance is not engine completion.

[E06a full-service costs](results/E06-unification-costs.md) add 384 passing configurations and exact replay. Direct map/table/encoded controls distinguish environment scans and unary key costs; explicit net copying/cleanup and slot metadata remain substantial. Full output extraction is charged in every mode. Routed/shared stores, binary IDs, slot reuse, source certificates and richer matching remain feasible; no native-performance or language-adoption claim follows. General encoded-machine and native-choice paths remain independent.

[E06c native provenance probes](results/E06-native.md) retain 21 generated cases, baseline/instrumented collapse outcomes and 19 separate graph traces, with exact event/observation replay. They demonstrate copy/birth collisions, 24-bit masking, administrative SUPs sharing source labels, disconnected failure and a starvation encoding. Finite repairs pass their declared observations; allocator lifetime, full branch-local state/history effects, source correspondence and resumable scheduling remain open. The v1 wildcard/failure generator distinction is documented and corrected in v2.

[E09 whole-request yield gate](results/E09-service-gate.md) passes 288 workload/quantum configurations with exact replay and 19 tests. Construction, node-status checking and complete readback now yield alongside net reduction; small requests progress during a large sibling's build/read phases. Slot-status maintenance, three scheduler policies, full source selection/wake-ups, support ownership and observer service remain open. E06 refinements remain queued, with no dependency blocking E09.

[E09 source adapter gate](results/E09-source-gate.md) passes 192 configurations and exact replay, comparing every continued/forked state against the independent tree control and all 64 E00 answer contracts against hand expectations and Rust reference. Direct source selection, matching, binding and full residual materialization now yield. Policy/support queues, resumable exact observation, charged admission and net integration remain feasible open work; no scheduler or architecture conclusion follows yet.

[E09 observer gate](results/E09-observer-gate.md) validates a yielding exact joint-alpha/full-residual comparison service on 14,271 configurations with exact replay. Map rollback and residual multiplicity are preserved. Final dedup queues, indexing, support grouping and policy integration remain open; yielding comparisons alone do not establish output fairness or bounded memory.

[E09 integrated policies](results/E09-policy-work.md) pass 930 exhausted-case configurations plus 180 controlled configurations and exact replay. Sealed rounds delay a small sibling behind a finite large equation; exact-state support groups preserve 64 duplicate lineages with 22 rather than 382 source jobs. Different output bindings prevent sharing subsequent identical operations and expose comparator overhead. Key/traversal ablations, preparation/runtime/storage measurements, mixed scans/wake-ups, heterogeneous support subdivision, longer synthesis and second-service integration remain feasible open questions.

[E09 key ablation](results/E09-keys.md) passes 750 configurations and exact replay. Opposed mismatch positions reverse the winning traversal; immutable identity checks help shared equal structures and add work on rebuilt ones. Forward traversal removes most earlier opaque-work bookkeeping, but no mode shares operations across different states. No matched source-job counts change. Integrated preparation/runtime/storage, key construction/retention, batch sensitivity and broader support/service questions remain open.

[E09 integrated costs](results/E09-costs.md) cover 225 configurations with five untraced timed repetitions after warm-up and two separate memory runs. Duplicate grouping improves time/peak memory; application grouping avoids no source jobs and can be expensive. Identity checks mitigate SK overhead without a separated async timing win. Async first-answer latency separates from round on the large-equation probe while total timings overlap. Preparation can dominate memory peak. Broader operation sharing, mixed scans/wake-ups, batch sensitivity, longer synthesis and second-service composition remain open.

[E09 mixed scans/bindings](results/E09-mixed.md) passes 324 configurations and exact replay, with independent source-step counts and full residual observations. Batch size controls duplicate-history sharing; round and async favor different answer endpoints in a guard case. The scan service wastes work on incompatible predicates, so a yielding predicate-filtered control and matched-predicate rejection workloads are the next necessary comparison. These results establish re-enablement semantics under rescanning, not an indexed-wakeup performance result.

[E09 predicate-filtered selector](results/E09-selector.md) passes 624 configurations and exact replay while preserving independently checked source transitions. Filtering removes most incompatible-predicate delay; matching-predicate guard/join controls expose Cartesian-product and duplicate-ID overhead. Distinct-prefix matching is the next concrete ablation, with rollback/order checks and charged binding work. Maintained indexes, indexed wakeups, heterogeneous sharing and second-service composition remain open.

[E09 distinct-prefix matching](results/E09-prefix.md) passes 570 configurations and exact replay, preserving the preceding 624-row control matrix. Early head mismatch rejection removes most acyclic-join work; guard-rejected complete tuples and binding-map copies remain costly. Certified early pure-guard placement with no-pruning controls is the next ablation. Trails, maintained indexes/wakeups, measured selector costs, heterogeneous sharing and second-service composition remain open.

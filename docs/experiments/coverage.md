# Coverage audit for the experimental sequence

The [sequence](sequence.md) assigns the outstanding questions from all seventeen research directions to experiments or specific owner decisions. No experimental family is considered resolved by this planning document. The coverage check below concerns the research dossier's known questions; new evidence can add questions.

The numbered directions follow [T018's disposition audit](../goals/chr-sharing/notes/T018-final-direction-audit.md). E00–E17 refer to work packages in the sequence. D1–D6 are owner decisions defined afterward. A dependency on an experimental artifact is distinct from a decision to adopt the language feature being tested.

## Direction-by-direction coverage

### 1. Reference semantics and ordinary CHR behavior

**Remaining:** concrete machine correctness, preservation of explicit alternatives and source resources, guard behavior, and any additional primitive catalogue.

**Assignment:** E00 supplies independent expected observations and transition checks, including nonconfluent committed schedules and before-dedup lineage. Every candidate consumes those obligations; E15 checks composition. D1 covers a new primitive API. Current equality-entailment guards allow the experiments to proceed without that choice.

**Evidence/disposition:** [reference specification](../goals/chr-sharing/notes/T008-reference-and-candidates.md), [composed simulation](../goals/chr-sharing/notes/T016-composed-machine-argument.md), and the executable [implementation record](../implementation/reference-interpreter.md) supply the starting contract. Existing tests do not discharge conditional transitions or prove general fairness. Those questions remain experimentally active.

### 2. Conditional multiset execution

**Remaining:** projection and lineage fidelity, conditional cycles, grouping, complete incremental matching/wake-up, support representation, histories, memory and bookkeeping cost.

**Assignment:** E03 tests finite supports before incremental discovery and BDD refinements; E00 supplies adversarial semantic cases; E01 supplies storage controls; E09 tests compact scheduling separately. E03 requires no favorable graph or net result.

**Evidence/disposition:** [kernel](../goals/chr-sharing/notes/T011-conditional-kernel.md), [matching/wake-up](../goals/chr-sharing/notes/T012-matching-and-wakeup.md), and [T013](../goals/chr-sharing/notes/T013-experiment-registration.md) are constructive inputs. Their specific runtime and guard assumptions require registration reconciliation. No measured advantage is established.

### 3. Contextual relation graphs and cached expansion

**Remaining:** common expansion versus per-context commit cost, fresh identities, equality and wake-up traffic, region closure and retention.

**Assignment:** E04 separates dispatch from cached expansion. E02 tests closure and reformulation costs. E05 supplies a contrasting equality representation; E15 checks relation/solver interfaces. These do not require E03's success or optimized support backend.

**Evidence/disposition:** [relation construction](../goals/chr-sharing/notes/T015-relation-graph-construction.md) and [composition](../goals/chr-sharing/notes/T018-composition-and-learning.md) establish the operation contracts. Empirical reuse frequency remains open, especially on synthesis.

### 4. Named superpositions and HVM-inspired execution

**Remaining:** family-service costs; source versus administrative label correspondence; generated code; normalization/collapse yields; label lifetime; failure and quiescence beyond the output graph.

**Assignment:** E05 tests first-order native families. E06b tests encoded-machine backend execution. E06c tests native compiler correspondence, including graph/label traces. E09 tests finite service; E15 tests active effects. These are three distinct paths, not a chain requiring an encoded machine to be fast first.

**Evidence/disposition:** [family services and backend boundaries](../goals/chr-sharing/notes/T018-backend-boundaries.md) provide independent contracts. Native correspondence is an implementation/proof task in E06c; neither a backend name nor scalar ground examples close it.

### 5. Ordinary interaction nets and richer local calculi

**Remaining:** emitted agents/rules, service and fan traffic, equality lookups, local dispatch restrictions, and whether richer interactions materially change the service cost.

**Assignment:** E06a tests the direct and net controllers. E02 compares global restrictions and regional compilation; E05 compares full and certified-safe unification. E06a also evaluates richer service proposals against concrete operation costs before choosing a protocol to implement. E16 handles parallel cost separately.

**Evidence/disposition:** [finite encoding](../goals/chr-sharing/notes/T016-finite-net-service-encoding.md), [locality/equality](../goals/chr-sharing/notes/T016-net-locality-and-equality.md), and [compiler source assessment](../goals/chr-sharing/notes/T016-compiler-source-assessment.md) support this split. Nonoverlap settles dispatch only. Source-access limitations must not be represented as a measured negative result for a calculus.

### 6. Andorra-style delayed splitting

**Remaining:** profitable split policy, resource-aware promotion, speculative work, state distribution and progress.

**Assignment:** E07 compares early, late and bounded adaptive preference with both opaque continuations and early-failing producers. E09 and E15 test scheduler interaction. E07 can use simple boxes independently of the conditional engine.

**Evidence/disposition:** [Andorra construction](../goals/chr-sharing/notes/T017-andorra-and-decomposition.md) supplies the lifting certificate and deterministic-first counterpressure. A single favorable carry example cannot choose a policy.

### 7. AND decomposition and reunion

**Remaining:** certificate precision/cost, independence frequency, fair product enumeration, provisional state retention, late joins and aliases, graph maintenance and product reconciliation.

**Assignment:** E08 has separate persistent, inferred and temporary-reunion comparisons. E02 tests expressibility and certificates; E15 checks cross-region effects; E17 measures occurrence in intended applications. D3 covers user-visible factored answers; internal factoring does not need that choice.

**Evidence/disposition:** [persistent decomposition](../goals/chr-sharing/notes/T017-andorra-and-decomposition.md) and [temporary protocol](../goals/chr-sharing/notes/T017-temporary-decomposition.md) have different completion obligations. Success of persistent products cannot close reunion.

### 8. Fair symbolic scheduling and encapsulation

**Remaining:** queue/support fragmentation, huge finite services, asynchronous completion, collapse overhead, grouping effects and response latency.

**Assignment:** E09 compares three schedulers on equivalent finite services; E07 varies splitting preference; E06 supplies backend yield evidence; E14 accounts for output work. E15 requires a second service organization before generalizing scheduler results. D4 covers stronger user-facing order or latency promises.

**Evidence/disposition:** [rounds](../goals/chr-sharing/notes/T015-symbolic-scheduling.md), [asynchronous jobs](../goals/chr-sharing/notes/T016-asynchronous-symbolic-scheduler.md), and [search encapsulation](../goals/chr-sharing/notes/T018-backend-boundaries.md) state finite-service premises. Timing tests do not replace that argument, and fairness does not imply a practical latency bound.

### 9. Logical structural solvers and exact projection

**Remaining:** exact membership/entailment/projection cost, domain assumptions, residual replay, repeated holes, and host occurrence boundaries.

**Assignment:** E10 compares direct rules, exact operation reuse and logical interfaces with separately declared observations. E12 tests certificates and caller filtering; E15 tests composition. D2 and D3 concern adoption of closed logical regions and formula observations, respectively.

**Evidence/disposition:** [predicate certificates](../goals/chr-sharing/notes/T015-solver-certificates.md) and [exact projection](../goals/chr-sharing/notes/T016-exact-solver-projection.md) distinguish semantic formulas from CHR resources. Baseline-preserving operation reuse can proceed while adoption decisions remain open.

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

**Evidence/disposition:** the [reference implementation](../implementation/reference-interpreter.md) tests exact structural answers; [observation analysis](../goals/chr-sharing/notes/T016-observation-and-state-reuse.md) explains stronger obligations. Full residual answers remain the executable comparison default without requiring a new owner decision.

### 13. Language restrictions, inference and compiler regions

**Remaining:** actual source coverage, global versus local restrictions, annotation/reformulation burden, certificate inference, linking and each restriction's runtime benefit.

**Assignment:** E02 inventories and pairs programs; E04/E06 measure nonoverlap/locality; E05/E06 measure ownership/disjointness; E10 measures logical regions; E09 measures finite services; E11 measures local bounds; E13 measures static specialization; E15 validates linking and cross-region effects. D2 governs adoption.

**Evidence/disposition:** [language opportunities](../goals/chr-sharing/notes/T016-language-opportunities.md) identifies independent properties, rather than a mandatory bundle. Every benefit claim must name the service avoided and an affected real program. Experimental evaluation of a restricted language is permitted without changing the project's chosen language.

### 14. Partial evaluation and specialization

**Remaining:** profitable unfolding, variant size, compilation cost, all-mode preservation, active effects and interaction with sharing.

**Assignment:** E13 establishes finite specialization on a competent scalar engine; E15 repeats equivalent opportunities on a shared engine; E17 includes cold and reused compilation costs. E02 supplies eligibility and programming-cost cases.

**Evidence/disposition:** [specialization](../goals/chr-sharing/notes/T018-specialization.md) supplies a conservative construction and explicit limits. A more general theorem requires its premises to be established for the selected region; it cannot silently cover arbitrary CHR.

### 15. Storage, parallel execution and whole-system cost

**Remaining:** competent baselines, restore/replay tradeoffs, allocation/reclamation, retained memory, service overhead, composition, synchronization, bandwidth and useful parallel granularity.

**Assignment:** E01 supplies persistent/trailing/replay controls; every mechanism counts administration; E14 accounts for output; E15 measures whole-engine costs; E16 measures sequential versus parallel behavior; E17 compares complete application costs.

**Evidence/disposition:** [cost questions](../goals/chr-sharing/notes/T016-comparative-cost-questions.md) and the [portfolio](../goals/chr-sharing/notes/T018-evaluation-portfolio.md) distinguish the work units. The copying reference's speed is not an architectural baseline result. Hardware restrictions, if encountered, must name the exact remaining measurement.

### 16. Arithmetic, SK, typing and lambda demonstrations

**Remaining:** actual forward/backward streams, nonground restrictions, further answers, sharing frequency and realistic synthesis cost. The user's SK typing relation adds type-directed and proof-like synthesis to the application mix.

**Assignment:** E00 registers these modes with independent witnesses and the pinned literal lambda translation. Every relevant mechanism uses application probes; E17 evaluates whole sessions and complete candidates. D5 concerns an additional conventional capture-avoiding lambda demo, not the supplied relation.

**Evidence/disposition:** [rwlog translation](../goals/chr-sharing/notes/T018-rwlog-translation.md), [intended programs](../goals/chr-sharing/notes/intended-programs.md), current `chr-programs` and its executable tests define present coverage. The unbounded duplication search remains open; ground evaluation or a finite candidate grammar does not close it.

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

[E00 v1](results/E00.md) validates 64 registered scalar observations and records remaining transition, generation and synthesis obligations. [E02 initial inventory](results/E02-initial-inventory.md) identifies concrete compiler opportunities and the norm/var occurrence-reintroduction boundary. Neither direction is closed. E14's independent observer is being built before E01 so new engines can be compared without importing reference normalization; the storage-control investigation remains queued.

# Research disposition audit

The preimplementation research goal is satisfied at the level of source-backed mechanisms, concrete paper constructions, counterexamples, language tradeoffs and evaluation questions. No architecture has been selected, no semantic change adopted, and no evaluator implemented or benchmarked. The research supports a portfolio of credible designs; it does not support a performance winner.

This audit applies the owner's stopping rule to every direction carried by the investigation. “Closed” means that the stated research question has an answer with its assumptions and limits. “Implementation-dependent” names the concrete artifact or measurement needed next; it does not mean that one shared experiment blocks all directions. “Owner-dependent” identifies a language or product choice whose consequences are already described. Earlier interim notes retain their chronological open questions; the dispositions below incorporate the subsequent analyses.

## 1. Reference semantics and ordinary CHR behavior

**Question:** Can explicit search and finite-tree equality be added while preserving committed CHR outside explicit disjunction?

**Disposition: closed for the proposed reference contract; primitive catalogue is owner-dependent.** T008 and T011 specify identified multiset occurrences, nonbinding head matching, pure entailed guards, propagation history, RHS equations, fresh source choices, and failure. T016's composed argument covers initialization and every transition class. It distinguishes soundness from finite-answer coverage under a fixed permitted committed policy. No implicit exploration of conflicting CHR rules or source-order guarantee is introduced.

The owner must decide any additional primitive/guard API beyond the finite procedures assumed by this contract. Its material consequence is whether each primitive can yield and whether guard entailment is decidable for the chosen theory. This is not a blocker for equality-only comparisons. Concrete machine correctness and guard implementations require executable or formal validation.

**Evidence:** [reference/candidates](T008-reference-and-candidates.md), [conditional kernel](T011-conditional-kernel.md), [composed simulation](T016-composed-machine-argument.md).

## 2. Conditional multiset execution

**Question:** Can one store carry shared and branch-specific bindings, occurrences, consumption, histories and failure without unsound cross-branch effects?

**Disposition: closed at the paper-algorithm level; implementation-dependent costs and validation.** Supported state, birth scopes, pointwise projection, transactional unification, split pullback and relevant-version quiescence supply a construction. Conditional occurs paths use conjunction along paths and disjunction across paths, avoiding false cycles in the union graph. Shared expansion and all support bookkeeping are separately charged.

Needed results are actual support-operation counts, live/peak memory, matcher/index overhead, and conformance on the semantic counterexamples. T013 specifies one controlled experiment; it is only this direction's experiment.

**Evidence:** [kernel](T011-conditional-kernel.md), [simulation](T016-composed-machine-argument.md), [experiment](T013-experiment-registration.md).

## 3. Contextual relation graphs, memoized pull-tabbing and derivation nets

**Question:** Can common occurrence expansion after a choice remain shared while arguments are used opaquely?

**Disposition: closed constructively for the relation fragment and a defined global interface; implementation-dependent net benefit.** Event provenance, contextual logical handles and cached body expansion realize the carry-chain example. They preserve distinct identical occurrences and fresh variables. General CHR requires partner lookup and conditional effects; Angelic CHR's alternative search semantics cannot be imported unchanged.

Needed results are expansion reuse versus contextual commit work, memory retention, equality cost and interface traffic on arithmetic/SK/lambda. A graph's compact shape alone cannot settle them.

**Evidence:** [source assessment](T015-source-expansion.md), [relation construction](T015-relation-graph-construction.md), [cross-region interface](T018-composition-and-learning.md).

## 4. Named superpositions and HVM-inspired execution

**Question:** Do named choices supply the required correlated computation, and what additional services are needed?

**Disposition: closed for the abstract family-service construction and integration boundaries; concrete backend correspondence is implementation-dependent.** Finite contextual case splitting on all operands has a projection proof. Logical handles, active effects, choice birth and source quiescence are necessary. Pinned HVM source confirms that function duplication introduces administrative superpositions and that collapse calls normalization; direct SUP substitution is not a sufficient CHR embedding.

The next artifacts are a generated encoded-machine adapter or native family-service implementation. A native HVM compiler additionally needs generated-graph/label correspondence and bounded normalization validation. Those are specific implementation and formalization obligations, not something established by citations or by passing two ground examples. The first-order family service can be built and compared independently of that adapter. No claim is made that a full native HVM compiler has already been designed or proved.

**Evidence:** [label/effect counterexamples](T016-superposition-services.md), [service algorithm and backend boundaries](T018-backend-boundaries.md).

## 5. Ordinary interaction nets and richer local graph calculi

**Question:** Are overlap restrictions enough, and can finite local services encode the remaining operations?

**Disposition: closed for conservative expressibility and restriction tradeoffs; implementation-dependent locality and cost.** Single-head/nonoverlap settles only dispatch. A finite controller/data encoding provides equality worklists, occurs checking, identities and request/reply scheduling, with linear channel/control endpoints. Arbitrary aliases still need a service; source variables are not automatically net wires. Richer additive and multiport calculi provide different possibilities but do not eliminate CHR resource and fairness obligations.

Needed results are emitted agent/rule counts, service traffic, fan work, scans and queue latency. Full access to particular older fusion/distributed-net papers is not an asserted theorem dependency: the construction does not rely on their unavailable results. Their mechanisms remain comparison leads if measurements expose the corresponding bottleneck.

**Evidence:** [locality/equality](T016-net-locality-and-equality.md), [source assessment](T016-compiler-source-assessment.md), [finite encoding](T016-finite-net-service-encoding.md).

## 6. Andorra-style delayed splitting

**Question:** Can shared surrounding work run before distributing over alternatives, with fair explicit search?

**Disposition: closed with a resource-aware lifting certificate; implementation-dependent control policy.** The construction permits common work over whole live support and retains conditional effects otherwise. External-variable tests alone are insufficient because a branch can consume a common ground occurrence. BEAM's delayed/eager split tradeoff supplies evidence against an unconditional “deterministic work first” policy.

Measure late-split savings, speculative work, copying/distribution costs and finite-answer latency on both long opaque continuations and early-failing producers. This experiment is independent of the conditional-store benchmark.

**Evidence:** [Andorra construction and sources](T017-andorra-and-decomposition.md).

## 7. AND decomposition, factored products and temporary reunion

**Question:** Can independent subproblems be solved once and combined without losing future joins or starving answer products?

**Disposition: closed for persistent certificates and a conservative temporary protocol; implementation-dependent detection and memory costs.** Persistent independence includes resource interactions, future bodies and aliases, not merely disjoint variables. Regional execution products have a commutation argument and fair tuple enumeration. Temporary independence keeps live states and rejoins when dependencies appear; it cannot publish independent final answers early.

Measure inference precision, graph maintenance, false dependencies, product size and caching/recomputation. A factored *user-visible* answer would be an owner decision; internal factoring requires no surface change. AOMDD finite-graph width results do not bound unbounded CHR search.

**Evidence:** [persistent decomposition](T017-andorra-and-decomposition.md), [temporary protocol](T017-temporary-decomposition.md).

## 8. Fair symbolic scheduling and search encapsulation

**Question:** Does fairness force eager per-alternative scheduler objects, and can policy remain replaceable?

**Disposition: closed negatively, with two constructive schedulers; implementation-dependent latency and fragmentation.** Sealed rounds and asynchronous sealed support jobs give finite-answer progress under finite service premises. The asynchronous version permits completed supports to proceed without waiting for unrelated unfinished supports. Neither promises a compact worst case or a latency bound independent of finite work. Encapsulated advance/restrict/observe operations separate policy from store services; a run-to-stability call is insufficient for fairness.

Measure queue fragmentation, support bookkeeping, large-finite-service interference and progress beside infinite recursive siblings. Owner preference is needed only for additional guarantees such as a specific answer order or latency contract; neither is assumed.

**Evidence:** [rounds](T015-symbolic-scheduling.md), [asynchronous algorithm](T016-asynchronous-symbolic-scheduler.md), [encapsulation](T018-backend-boundaries.md).

## 9. Logical structural solvers and exact projection

**Question:** Which supplied pruning predicates admit exact logical treatment, and what projection is sound?

**Disposition: closed for the stated finite-tree/regular theory and supplied predicate certificates; broader interfaces are owner-dependent.** no_c, var, neq and norm have explicit interpretations and operational limits. Finite skeleton obligations can be handled exactly while retaining existential witnesses. Repeated holes cannot always be represented by an ordinary unary regular language. Solver consequences cannot silently become host CHR occurrences, and formula idempotence cannot replace multiset resource behavior outside a closed region.

Measure exact projection, membership and entailment against direct constraint processing. Adopting formula observations instead of residual occurrences changes the language contract and requires the owner's choice; retaining occurrences with exact operation memoization is available meanwhile.

**Evidence:** [predicate certificates](T015-solver-certificates.md), [exact projection](T016-exact-solver-projection.md).

## 10. Equality-constrained term spaces and whole-evaluator symbolic compilation

**Question:** Can compact program spaces retain repeated-subterm dependencies, and can this extend beyond a structural helper?

**Disposition: closed constructively for both uses; implementation-dependent compilation size and solver benefit.** ECTA path constraints represent repeated holes exactly. Its cyclic grammar denotes finite terms and does not require rational-tree source values. A separate bounded symbolic-execution witness construction represents the full reference machine, including nonground answers, and increasing bounds covers every finite successful derivation. This avoids treating a finite program grammar as a termination guarantee for its evaluator.

Needed results include factored transition-encoding size, compilation time, intersection/solver work, reuse across bounds and time to residual answers. Returning arbitrary automaton residuals as new source predicates needs an owner interface decision; internal encoding does not.

**Evidence:** [ECTA assessment](T017-equality-constrained-program-spaces.md), [whole-evaluator construction](T018-bounded-symbolic-compilation.md).

## 11. Tabling, learned failure and state reconvergence

**Question:** What can be reused when calls or states coincide, and when can failure prune more contexts?

**Disposition: closed for conservative sound certificates; implementation-dependent usefulness of stronger generalization.** Exact logical caller projection/filtering supports solver tables. Full CHR continuation reuse needs state information beyond outputs, including histories and relevant policy metadata. Structural whole-state bijection supplies a sufficient certificate. Unification clash/occurs witnesses support learned exclusions with provenance; choice assignments alone may omit committed-event assumptions.

Measure key/certificate construction, entailment cost, table hits, witness retention and avoided work. A generalizing table can lose useful caller pruning; a higher hit count is not sufficient evidence of improvement.

**Evidence:** [state reuse](T016-observation-and-state-reuse.md), [learning/interface construction](T018-composition-and-learning.md), [earlier solver/tabling assessment](T011-local-compilation-and-solvers.md).

## 12. Final answers, residuals and sound deduplication

**Question:** How can nonground answers be trusted and known duplicates identified?

**Disposition: closed for conservative full-residual observation and structural alpha equivalence; display/projection defaults are owner-dependent.** Stable source quiescence, finite-tree consistency and all active constraints determine answer eligibility. Exact canonicalization can enumerate variable renamings and compare output tuples plus residual multisets; faster canonicalization is an optimization. Different synthesized programs are not duplicates merely because evaluation agrees.

The owner must decide whether disconnected residual components should be displayed in full or projected through a proved logical interface. Hiding them without that proof is unsound. Needed implementation results are incremental canonicalization costs, residual size and practical output latency.

**Evidence:** [observation/state distinctions](T016-observation-and-state-reuse.md), [projection limits](T016-exact-solver-projection.md).

## 13. Language restrictions, inferred properties and compiler regions

**Question:** Which language changes buy distinct execution simplifications, and what do they cost the intended programs?

**Disposition: sufficiently investigated for informed choices; adoption is owner-dependent.** The dossier separately evaluates closed relational cases, global versus regional nonoverlap/single-head restrictions, immutable data versus writable handles, affine/ownership properties, occurs-check certificates, logical solver semantics, finite services, finite domains and local policy guarantees. None is treated as a bundled prerequisite.

Global restrictions can simplify the engine or compiler but may forbid multiheaded norm/var interactions, repeated aliases or unbounded synthesis. Inferred/declared regions preserve those uses at the cost of eligibility checking and service boundaries. Explicit relational case syntax can preserve all modes by elaborating to RHS OR; constructor heads alone would suspend on unknown inputs. The owner chooses restrictions only after deciding which simplicity/expressiveness tradeoff to accept. Their runtime payoff needs comparative implementations.

**Evidence:** [language opportunity assessment](T016-language-opportunities.md), [net costs](T016-net-locality-and-equality.md), [linking/regions](T018-composition-and-learning.md).

## 14. Partial evaluation and specialization

**Question:** Can compilation remove known-case work while preserving backwards use and active constraints?

**Disposition: closed for bounded closed-relation specialization; implementation-dependent profitability.** Finite unfolding with fresh variables, explicit OR and proved equation clashes gives a direct relational correspondence. Residual calls ensure finite compilation. General needed-narrowing results have stronger source premises and are not applied to arbitrary CHR. Eliminating unused active goals or merging interceptable CHR steps needs further certificates, not a purity assumption.

Measure residual code size, variant count, compilation time and runtime savings with equivalent specialization across competing engines. Mandatory input modes are unnecessary for the demonstrated construction.

**Evidence:** [specialization source and construction](T018-specialization.md).

## 15. Storage, parallel execution and whole-system cost

**Question:** Which designs actually save work and memory, and does added machinery earn its complexity?

**Disposition: implementation-dependent.** The analytical distinction among prefix sharing, storage sharing, common expansion, table reuse, pruning and independent factor reuse is established. Commuting effects and finite service protocols give opportunities for parallel scheduling; no multicore speedup follows without accounting for contention, bandwidth and task granularity. Persistent state, trails and replay are legitimate controls; they do not by themselves prove post-choice computation reuse.

Required measurements are operation counts with all bookkeeping, retained/peak memory, allocation and copying, matcher/equality work, compilation, output costs and latency. Compare equal semantics and policies or explicitly separate differences. T013 plus T016 and the additional mechanism-specific questions above define what to measure; a production implementation is not a prerequisite for small isolated comparative prototypes.

**Evidence:** [cost questions](T016-comparative-cost-questions.md), [controlled experiment](T013-experiment-registration.md), [evaluation portfolio](T018-evaluation-portfolio.md).

## 16. Arithmetic, SK and lambda demonstrations

**Question:** Are the intended programs expressible, and what exactly would their translations demonstrate?

**Disposition: closed with concrete translations; engine behavior is implementation-dependent.** The corrected recursive addition and subtraction support relational queries. The supplied SK evaluator and fold have explicit catchall-body translations; the identity and duplication equations have paper derivations. Source inspection establishes rwlog intersection on both endpoints and fresh span scopes. The lambda notebook has a literal translation and a concrete distinction from capture-avoiding beta reduction.

Needed results are actual forward/backward answer streams, residual holes, finite-answer progress and sharing profiles. Conventional capture-avoiding lambda semantics would require choosing a demo representation/freshness convention; that choice does not block faithfully translating the supplied relation. No claim of complete equivalence for every rwlog feature or arbitrary embedded nonconfluent theory is made.

**Evidence:** [intended programs](intended-programs.md), [pinned-source translations](T018-rwlog-translation.md).

## 17. Query interface, future streams and additional features

**Question:** Which unresolved surface or future-application choices must the owner make?

**Disposition: owner-dependent for adoption; no independent core mechanism remains blocked by them.** The ruleset/query split is settled. Whether queries directly contain new equations/OR, exact syntax, primitive APIs, residual display, and optional first-class search controls remain product/semantic choices. Current programs can start with an ordinary query constraint whose rule posts equations/OR.

Finite request-driven stream protocols support future incremental computation without rational trees. A full real-number/stream-calculus library, effects, higher-order source terms or coinductive answer semantics would expand the initial project scope. The supplied targets do not require those features, and no optimization result here justifies imposing them. Their adoption requires a new owner intent decision.

**Evidence:** [language tradeoffs and streams](T016-language-opportunities.md), [source authority](../goal.md).

## Coverage and stopping assessment

Coverage was expanded by mechanisms rather than by a fixed candidate count: committed CHR and concurrent/persistent variants; functional-logic sharing and narrowing; conditional/variational execution; local and richer nets; Andorra and encapsulated search; AND/OR decomposition; logical solver/tabling/learning; constrained term spaces and bounded symbolic compilation; specialization; storage/recomputation and parallel effects. New findings in T017/T018 were investigated instead of being deferred behind T013.

Some source leads have only partial access, and no systematic-review completeness claim is made. No disposition depends on an uninspected theorem. The relevant question from each such lead is either answered by an inspected source/direct construction or stated as a concrete implementation cost. Newly discovered evidence can reopen a conclusion; this audit is not a claim that future research has no possible value.

The remaining uncertainty is now about concrete implementations, comparative costs, artifact-level correctness and named owner choices. There is no retained independent analytical task in this audit that is being deferred because one experiment is ready. This is the basis for closing the present research goal. It is not architecture approval, prototype authorization, or completion of the larger language project.

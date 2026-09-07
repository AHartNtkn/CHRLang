# Experimental sequence for CHR language and execution design

Investigate the independent mechanisms before choosing an engine. Start by making comparisons trustworthy, then test each mechanism's distinctive claim, and finally compare useful combinations on the intended synthesis workloads.

This is a dependency-driven sequence with coverage obligations, not a ranking of architectures. Completing the conditional-store experiment does not complete this investigation. The companion [coverage audit](coverage.md) maps all seventeen research directions and eleven portfolio comparisons to specific work below.

## Scope and present evidence

The user authorized implementation and experimental investigation. This document is the rolling research program; the [coverage audit](coverage.md) links completed measurements and their remaining questions. It does not adopt a language restriction. The source inputs are the [research disposition audit](../goals/chr-sharing/notes/T018-final-direction-audit.md), [evaluation portfolio](../goals/chr-sharing/notes/T018-evaluation-portfolio.md), [comparative cost questions](../goals/chr-sharing/notes/T016-comparative-cost-questions.md), and [investigation protocol](../goals/chr-sharing/notes/investigation-protocol.md). Mechanism-specific constructions are linked with their experiments.

The reference interpreter at commit `0c43a41` provides executable arithmetic, SK evaluation and typing, resumable explicit search, finite-tree equality, and conservative residual answers. Its 33 passing tests establish tested behavior, not a general correctness proof. Its copying, exhaustive matching and factorial answer canonicalization deliberately favor inspectability. It is not a sufficient performance baseline. Unrestricted duplication synthesis produced no answer within 50,000 reference steps; that observation supplies neither failure nor a reason to restrict the source language.

The initial experimental language retains the reference's equality-entailment guards, RHS-only binding equations, explicit OR, finite terms, identified multiset resources, ordinary committed CHR, and full residual observations. Changes to these are separately described candidate languages. Surface restrictions may be investigated without adopting them. Performance comparisons must identify their accepted program subset and their observation contract.

## Why this order

Three questions determine dependencies. Can the result be interpreted correctly? Can a cheaper experiment distinguish the competing explanations? Does the next experiment need a particular measured result, or just an available implementation service?

The first pass establishes observations and competent controls because an answer mismatch or a weak baseline can invalidate every later performance conclusion. Language-property analysis starts alongside that work because it changes which programs each candidate can accept. It is not postponed until an engine has been built.

The next pass tests distinct mechanisms in isolation. Conditional membership, cached occurrence expansion, named families, delayed distribution, independent products and symbolic compilation save different work; none needs a positive result from another to merit its own probe. Simple service implementations suffice for early correctness and operation accounting. More elaborate condition representations, compilers and scheduling policies follow their own service-level tests.

Integration follows mechanism tests because combining changes early would make a speed difference difficult to explain. Application cases run throughout for semantic coverage and reuse-frequency observations; the final application comparison measures complete implementations with their construction, scheduling and output costs included. Parallel execution follows a correct sequential composition because parallelism can otherwise obscure both semantic defects and the source of savings.

These are qualitative information and dependency judgments. There is no evidence for numerical expected-value scores, development-time estimates or a universal speed threshold. The plan does not invent them.

## Execution order and independent paths

A dependency below means a required artifact or fact. It never means that its predecessor must be fast or beat another candidate. Every experiment can stop its affected measurements on a correctness defect while independent work continues.

1. **Establish comparisons:** E00 registers semantic cases and observations. E02 can analyze source eligibility immediately using existing programs and translations. E01 builds competent scalar storage controls; E14 checks observation cost and stronger exact canonicalization. E00's completed case subsets can be consumed incrementally; the lambda case need not delay an arithmetic mechanism probe.
2. **First distinguishing probes:** run the minimal correct variants of E03, E04, E05, E07, E08, E10, E11, E12 and E13. In a single execution slot, visit them in that listed order once their stated dependencies exist. The first three distinguish representations of correlated work; delayed splitting and decomposition then challenge the assumed unit of execution; solvers, whole-machine compilation and reuse challenge whether executing those events is necessary; specialization checks compilation as a competing explanation. This tie-break order is organizational, not a performance ranking. Do not finish all refinements of E03 before giving the other ready families their first probe.
3. **Representation and control refinements:** E06 starts as soon as the operation contracts from E00/E02 exist, independently of conditional-store results. E09 starts when finite, resumable service operations exist. Complete the registered refinements within each first-probe experiment: incremental matching, symbolic conditions, reunion, solver generalization and compilation reuse. Interleave these with E06 and E09; no whole-stage barrier applies.
4. **Composition:** E15 selects contrasting combinations from measured mechanisms and validates shared-variable effects and global completion. E16 examines parallel execution on a valid sequential composition. E17 runs complete application comparisons and produces the direction-by-direction decision audit. E17 also receives application results from earlier passes; it is not the first time real programs are run.

When several tasks are ready, first repair a defect that invalidates already-collected evidence, then give an untested family its first distinguishing probe, then run a refinement that separates competing explanations, then broaden a measured parameter range. Within a tier use the order above. Record an evidence-based departure and its reason before executing it.

After each complete pass through the ready families, list every unvisited family with its exact dependency and the task producing it. A lack of time or an unfinished prototype is unfinished work, not a finding against that family. New mechanisms or counterexamples add a question and dependency entry before the next selection; the present list is coverage of this dossier, not a claim to cover all future research.

## Experiment contracts

Each E-number below defines a bounded work package with a scientific outcome. Before its comparative run, create `docs/experiments/registrations/E<number>.md` containing exact IR cases and configurations, parameter values, hypotheses, expected observations, bounds, environment and analysis. Store the receipt at `docs/experiments/results/E<number>.md`, linking raw results and the code revision. Do not create empty directories or a generic experiment framework in anticipation of these tasks.

A registration must distinguish semantic absence, exhaustion, a tested finite prefix, timeout and an unrun case. Freeze it before comparative execution. Pilot runs may select feasible resource ranges; label them exploratory and freeze a new comparative batch before using those ranges to support conclusions. Preserve unfavorable pilot observations in the receipt.

Use the Rust source AST and reusable programs where their semantics match. Keep the reference runtime private and independent. Experimental implementations belong in isolated crates under `research/`; they may share syntax, input construction and result serialization, but may not call the reference unifier, matcher, scheduler or canonicalizer as their algorithm. A candidate must supply its own answer representation and state projection. A read-only reference trace hook may be added if required for checking transitions; it must not expose reusable execution services. Sharing services between experimental variants is permitted only when it is recorded as a controlled factor.

### E00 — Establish the semantic and measurement contract

**Dependencies:** existing reference and semantic constructions. **Decision enabled:** which comparisons are meaningful, and which observations remain unverified.

Create a concrete case registry from the 24 semantic IDs and scheduler cases in [T013](../goals/chr-sharing/notes/T013-experiment-registration.md), plus the current equality guards. Include correlated and independent dynamic choices, distinct duplicate occurrences, conditional consumption/history, suspended matching, branch-local cycles, disconnected failure, active work with ground outputs, and nonground residual aliases. Conditional subscription races and stale commits are candidate obligations, not behavior already demonstrated by scalar tests.

For each finite case write the expected answer and residual multiset independently of the reference, including alternative multiplicity before final deduplication. Where histories or intermediate source steps matter, specify the finite transition trace or projection obligation. Validate both implementations against it. A discrepancy with the reference is investigated on both sides; matching the reference is not the sole justification. For nonconfluent cases use the same committed selector or validate legal transitions with an explicitly different selector. Exploring legal CHR schedules in a test oracle must not become implicit source search.

Add application registrations for all-direction addition/subtraction, repeated-variable queries, SK identity/duplication, residual holes, SK typing and type-directed synthesis, and the [literal lambda relation](../goals/chr-sharing/notes/T018-rwlog-translation.md). Preserve the notebook's supplied rewrite semantics and residual behavior; conventional capture-avoiding substitution is a separate demo choice. Include a finite ground SK grammar and bounded evaluation separately: finite program size does not ensure terminating evaluation. Use hand-derived witnesses and small complete enumerations, not just matching prefixes of differently ordered answer streams.

Define counters at actual operation boundaries: source expansions, projected transitions, matching/guards, equality/dereferencing/occurs visits, support operations, history, cache construction, allocation/copy/reclamation, queue service and output. Document that different engine microsteps are not interchangeable units. Establish a small independently counted trace per counter family. Record first answer, subsequent answer gaps, finite all-answer cost and retained/peak memory separately.

**Follow-up:** semantic uncertainty creates a minimized case and an analysis task before affected measurement; unrelated cases remain available. Agreement permits bounded conformance claims only. A case requiring a new language feature goes into the decision queue, with baseline cases continuing.

### E01 — Establish competent storage and recomputation controls

**Dependencies:** E00's scalar cases and counters. **Decision enabled:** how much apparent improvement comes from avoiding allocation/copies rather than shared execution.

Compare the copying reference with three independently executing scalar variants: immutable constructor sharing plus persistent branch state; checkpointed trailing with restoration; and replay from recorded checkpoints. Fair FIFO or equivalent finite service must include restore/replay costs. Trailing a single depth-first branch without servicing alternatives is not an equivalent control. Use the same source selector, indexing opportunities and output observations.

Build and validate the persistent control first so that initial mechanism comparisons can distinguish storage from computation sharing. Trailing and replay remain required independent control investigations before a whole-system recommendation; they do not block first conformance probes or explicitly limited comparisons against the persistent control.

Use no-OR cases, B=1, W=0, the carry family, long aliases, large unrelated stores, early/late failures, deep recursion and large frontiers. Vary checkpoint interval in a frozen grid rather than selecting its best value separately for every published case. Measure restored/replayed operations, snapshot cost, allocation and retained memory in addition to total runtime.

**Follow-up:** retain a measured frontier of controls when latency and memory disagree; do not choose one universal baseline by aggregate rank. If indexing or representation explains the difference, align that factor before attributing a later gain to execution sharing. High restoration cost motivates a storage refinement, not abandoning fairness. These controls remain useful even if no shared mechanism wins.

### E02 — Measure language properties and reformulation costs

**Dependencies:** existing programs and research translations; E00 for executable equivalence checks. **Decision enabled:** whether a global restriction, declaration or inferred region buys enough to justify its programming cost.

For each application and synthetic join case, identify closed single-headed regions, nonoverlap, immutable data versus writable handles, uniqueness/disjointness for occurs checks, finite services, structural solver closure, local commutation, and bounds suitable for symbolic compilation. Distinguish necessary conditions from sufficient certificates and mere implementation conveniences using [language opportunities](../goals/chr-sharing/notes/T016-language-opportunities.md).

Prepare paired source forms for unrestricted CHR, global restrictions where expressible, and inferred/declared regions. Include an external head interaction that invalidates local compilation, S's repeated argument, unknown-input addition, norm/var joins, and an alias export that invalidates uniqueness. For each pair record what is accepted, rejected, inferred or explicitly annotated; source size and required reformulation; preserved relational modes, residuals and finite-answer behavior; and which runtime service becomes unnecessary. A faster restricted program has no score for a workload it cannot express.

Measure certificate/inference cost and precision when the consuming compiler is built. Global versus regional nonoverlap must have a comparison in E04/E06; ownership and cycle-check certificates in E05/E06; solver restrictions in E10; static information in E13. Do not substitute one bundle of restrictions for these individual ablations.

**Follow-up:** successful certificates support optimization without requiring language adoption. Low precision calls for a concrete missed case and analysis, not an immediate annotation mandate. A beneficial global restriction produces an owner-facing tradeoff with excluded/reformulated examples. Adoption requires the owner's decision; evaluating the restricted candidate does not.

### E03 — Conditional multiset execution

**Dependencies:** E00; E01 for performance interpretation. **Decision enabled:** whether supported bindings, occurrences and histories save net work and whether incremental discovery is worthwhile.

Use [T013's registration](../goals/chr-sharing/notes/T013-experiment-registration.md) as the detailed starting specification, with the reconciliation below. First validate extensional finite supports and complete scans. Then compare incremental discovery against complete scans on the same stable snapshot, including read/subscribe races and alias redirection. Only then compare finite sets with fixed-order BDDs and grouped versus singleton work. The first correct variant is sufficient for this family's first distinguishing probe; its refinements do not block other families.

Run the registered B/W/store-size grid, conditional cycles, joins, history, alternating support updates, early discrimination and failure. Preserve the legal setup prefix and charge it separately and in total. Include natural source executions without the prefix driver to measure how often compatible work actually becomes available together. W expansions after staged setup establish capacity, not automatic W-work execution on normal queries.

**Follow-up:** projection, lineage or wake-up defects require repairing that operation. W expansions accompanied by B×W bookkeeping trigger a cost decomposition, not a sharing-speed claim. Favorable total costs lead to E15 composition; unfavorable results lead to a named bottleneck comparison with E04/E05/E07, not closure of those alternatives.

### E04 — Occurrence-local relation graphs and cached expansion

**Dependencies:** E00 and E02's closed-region examples; E01 for performance comparison. **Decision enabled:** whether provenance-based expansion reuse and local dispatch offer a distinct advantage over general matching.

Implement the [contextual relation construction](../goals/chr-sharing/notes/T015-relation-graph-construction.md) with a simple contextual equality service. It need not wait for E03's optimized store. Compare general matching, local dispatch without cached expansion, and local dispatch with cached expansion on identical storage and contextual commit services. When comparing against E03, report service differences rather than attributing them to the graph.

Use the carry family, two identical but distinct calls, fresh body variables, binding wake-ups, SK eval/fold with no_c, and a newly linked rule that invalidates region closure. Count reusable expansions, contextual commits/lookups, freshening, partner lookup, interface traffic and retained nodes. Distinguish provenance reuse from coincident equal calls, which belong to E12.

**Follow-up:** a local-dispatch gain alone supports compilation, not cross-choice reuse. Reuse erased by contextual commit work directs comparison to E05's services. Poor region coverage is an E02 language finding. Favorable results become one E15 composition candidate without excluding general CHR engines.

### E05 — Named families and contextual equality services

**Dependencies:** E00 operation contracts; E02 for optional certificate variants. **Decision enabled:** whether named decision families carry opaque values economically and where equality forces distribution.

Build the first-order family service in [backend boundaries](../goals/chr-sharing/notes/T018-backend-boundaries.md), independently of HVM integration. Compare projected scalar equations, explicit supported partitions, and named decision families. Start with exact finite representations; compare representation refinements only after projected results agree. All operands and effect services must be restricted consistently when a choice is inspected.

Exercise duplicated references to one choice versus independent recursive choices, same-handle equality, shared constructors, alternating contextual bindings, alias depths, deep occurs failures and apparent union-graph cycles with finite projections. Include failure outside outputs and unused labels. Measure field reads that remain shared, forced partitions, dereference/occurs work, support algebra, distribution, label retention/reclamation and effect publication. Compare certified fresh/disjoint cycle-check paths with full checks on the same finite-tree cases, including a near miss that invalidates the certificate.

**Follow-up:** excess distribution motivates a representation or service comparison, not source-visible superposition terms. Invalid correlation stops affected services. Benefit supports a native-family E15 candidate and informs E06; native HVM remains a separate compiler question regardless of this result.

### E06 — Local nets and concrete backend correspondence

**Dependencies:** E00 contracts and E02 restriction examples. E05's validated service can supply a control but is not required to begin finite controller encoding. **Decision enabled:** whether local rewriting or a concrete HVM backend pays for its service and compilation costs.

Keep three subexperiments distinct. **E06a:** implement the [finite controller/data net encoding](../goals/chr-sharing/notes/T016-finite-net-service-encoding.md), compare a direct controller for the same operations, and measure active pairs, fan/eraser work, environment scans, emitted agents/rules and request latency. Include global restricted and regional source variants from E02. Compare richer multiport/additive/fusion proposals analytically against these same operation contracts at the start of E06a; this analysis does not wait for net timing results. For each proposal name the service traffic it could avoid and any extra alias, resource or endpoint obligations. A distinct mechanism with a concrete protocol receives an operation-level implementation probe; establish its correspondence before executing it. If a proposal supplies no distinct mechanism for these operations, document that specific analytical disposition and what new evidence would reopen it. Do not close a distinct but unimplemented protocol merely because the ordinary net is ready.

**E06b:** generate an encoded CHR machine on a pinned functional backend and validate finite continuation boundaries, including normalization between returns. Charge interpretation, compilation and normalization. **E06c:** specify and test native source-choice integration with generated graphs and label traces. Establish source birth versus administrative label invariants, duplicated binders, recursive fresh choices, label exhaustion/reuse, branch-local effects, off-output failure and collapse yields. Ground result agreement alone cannot establish correspondence.

E06b and E06c are separate paths; neither a slow encoded machine nor a successful first-order family service answers the native compiler question. If the native invariant cannot be maintained, provide a concrete counterexample and investigate whether a compiler repair is possible. A specific missing implementation/proof obligation is unfinished work, not an empirical rejection of the family.

**Follow-up:** valid but costly encodings identify which service to change and which proposed source restriction would avoid it. Successful net services proceed to E15. Full-access gaps for source leads are recorded; consult authoritative sources for a new concrete protocol when needed, without claiming support from an uninspected theorem.

### E07 — Delayed splitting

**Dependencies:** E00; E01 for storage comparison. **Decision enabled:** when executing common surrounding work before distribution helps and when it causes speculation or starvation.

Implement resource-aware boxes from [the Andorra construction](../goals/chr-sharing/notes/T017-andorra-and-decomposition.md) with simple finite child contexts. Compare early splitting, late splitting with finite service, and bounded adaptive preference under equivalent committed behavior or a separately proved commuting reorder. A conditional-store implementation is not a prerequisite.

Pair long opaque continuations with early-failing producers, immediate discrimination, a common ground occurrence consumed in one arm, history changes and an infinite deterministic producer beside a finite answer. Record shared work, speculation later discarded, copied/distributed state, promotion, queue growth and finite-answer latency. Sweep registered preference quotas; unlimited deterministic-first execution is an adverse policy, not the fair default.

**Follow-up:** opposite results on the paired workloads motivate a policy criterion and held-out test, not a universal preference inferred from the winning case. No productive quota closes only that tested control strategy. Useful boxes become a contrasting E15 composition to supported or graph representations.

### E08 — Independent factors and temporary reunion

**Dependencies:** E00 and E02's independence examples. **Decision enabled:** whether solving independent factors once outweighs dependency analysis, caching and later joins.

First compare no factoring with explicitly certified persistent components and fair product enumeration. Then add inferred persistent certificates. Finally compare [temporary decomposition and reunion](../goals/chr-sharing/notes/T017-temporary-decomposition.md) using a conservative interaction graph and materialized finite products at reunion; compact reunion is a separate refinement if materialization dominates.

Use independent finite products, one infinite answer stream, empty factors, shared ground parameters, ground multihead joins, late body introductions and late alias exports. Preserve fresh namespaces, occurrence histories and lineage when combining states. Provisional local completion is never a global answer. Measure factor expansions reused, inference precision, graph maintenance, false dependencies, reunion size/conflicts, cached partial answers and time to each product answer.

**Follow-up:** savings only with declared independence become an E02 programming tradeoff. A false dependency is a precision question; an unsound certificate is a correctness defect. Expensive reunion directs a bounded compact-join experiment. Persistent success never closes the temporary-reunion question.

### E09 — Search scheduling and finite service

**Dependencies:** E00's event obligations and a validated finite-service implementation from E03, E05, E06 or E07. One service is sufficient to begin; do not wait for all engines. **Decision enabled:** which schedules preserve sharing and offer useful progress and latency.

Compare per-alternative FIFO, [sealed symbolic rounds](../goals/chr-sharing/notes/T015-symbolic-scheduling.md), and [asynchronous sealed jobs](../goals/chr-sharing/notes/T016-asynchronous-symbolic-scheduler.md) on the same service operations. Include service quantum and grouping as explicit factors. Distinguish a scheduler refinement preserving a fixed source selector from one choosing a different permitted schedule.

Use a divergent sibling, huge finite equality and tuple scans, wake-up waiting on a producer, nested choice inside shared work, completed support beside unfinished support, and duplicate-heavy output. Count scheduler/support operations, fragmentation, queue retention, source progress and output latency. Backend reduction/collapse and canonicalization must be included in the yield analysis. State finite-service assumptions in the liveness argument; finite timing tests challenge it but do not prove general fairness.

**Follow-up:** latency dominated by an atomic operation creates a concrete resumable-service task; changing queue order cannot establish isolation. Good source-step counts with poor wall latency require cost attribution. Repeat promising policies on a second structurally different service in E15 before generalizing them.

### E10 — Structural solvers and equality-constrained term spaces

**Dependencies:** E00 residual semantics and E02 declared domains/closure. **Decision enabled:** whether logical solving or correlated candidate spaces remove enough search to justify their construction and interface costs.

Compare direct no_c/var/neq/norm processing, exact operation memoization that preserves occurrences, regular membership plus explicit repeated-hole equations, and [equality-constrained tree automata](../goals/chr-sharing/notes/T017-equality-constrained-program-spaces.md). Separately implement the [exact logical projection procedure](../goals/chr-sharing/notes/T016-exact-solver-projection.md) under its declared domain and formula interface. Formula equivalence cannot be checked by pretending residual multisets are interchangeable.

Use repeated and independent holes, `a(H,H)`, intersecting normal/neutral constraints, residual unknowns later bound, companion var occurrences, constructors outside a purported closed signature, and finite versus unbounded supplies of represented names. Include an external consumer invalidating solver closure. For small finite domains compare denotations exhaustively; for infinite regular domains validate the stated exact procedure and symbolic witnesses rather than treating finite enumeration as proof.

Measure construction/intersection, projection/entailment, filtering, retained residuals, partial answers and total evaluator work. Preserve explicit source provenance for generation; automaton alternatives do not introduce implicit CHR search.

**Follow-up:** exact operation reuse can proceed without a new language interface. A formula or automaton observation benefit produces an owner decision with a concrete before/after answer. Poor intersection scaling directs a representation probe; success on structural spaces does not close E11's whole-evaluator question.

### E11 — Whole-evaluator symbolic compilation

**Dependencies:** E00 transition and endpoint contracts. E10 is relevant comparison evidence, not a prerequisite. **Decision enabled:** whether factoring bounded execution witnesses can compete with incremental execution of the complete language subset.

Implement a factored finite transition encoding from [bounded symbolic compilation](../goals/chr-sharing/notes/T018-bounded-symbolic-compilation.md), including pending goals, substitutions, histories, explicit choices, policy and quiescence. Start with small inspectable instances. Compare direct execution at exactly the same transition, node, identity and choice bounds. Bounds form a recorded dominating sequence; increasing program-size alone is insufficient.

Decode nonground endpoints with residual holes and check witnesses against independent transitions. Include finite arithmetic failures, duplicated occurrences, propagation tokens and SK holes. Compare recompiling each bound with exact reuse of compatible subrelations. Measure encoding size, construction, solver work, cross-bound reuse and first/subsequent symbolic answers. Use an existing solver only after selecting and documenting the concrete encoding it supports; no backend brand is assumed by this plan.

**Follow-up:** a large encoding must be attributed to particular state fields before redesign. Bounded unsatisfiability stays bounded. Favorable results require an increasing-bound, fairly serviced application run; unfavorable structural-solver results in E10 do not reject this direction, nor vice versa.

### E12 — Tabling, reconvergence and learned failure

**Dependencies:** E00; E10's exact logical interface only for generalized solver tables. **Decision enabled:** what can be reused between independently reached calls/states, and at what certificate cost.

Run three separate ablations: no reuse versus exact operation/call tables; no merge versus whole-state structural equivalence retaining lineage; no learning versus finite constructor-clash/occurs witnesses with provenance. Add generalized solver tables and minimized assumptions only after exact versions supply measured controls. Use the contracts in [state reuse](../goals/chr-sharing/notes/T016-observation-and-state-reuse.md) and [composition and learning](../goals/chr-sharing/notes/T018-composition-and-learning.md).

Pair identical subproblems with stronger caller restrictions. Pair reconvergent states with near matches differing in one history token, occurrence multiplicity, alias or policy age. Pair a reusable contradiction with a context lacking the committed event that justified it. Measure key construction, projection/freshening/filtering, entailment, witness retention, hits that actually avoid work, lost pruning and subsequent transitions shared. Equal displayed outputs are never a continuation key.

**Follow-up:** many hits without avoided work do not support benefit. Generalization that loses pruning is compared with exact keys under the same queries. Successful failure witnesses may combine with E03/E05 without state merging; a state-merge failure does not close table or learning questions.

### E13 — Finite specialization

**Dependencies:** E00 and E02's closed relation contract. **Decision enabled:** whether compilation avoids runtime work more cheaply, and how it interacts with sharing.

Implement [bounded unfolding](../goals/chr-sharing/notes/T018-specialization.md) with fresh variables, explicit OR, proved equation clashes and residual recursive calls. Compare no specialization and fixed unfolding/variant budgets. Test known input shapes, known output shapes and unknown arguments; `add(X,Y,s(B))` must retain its zero arm. Include an unused active failing call and an external consumer of an intermediate predicate to challenge eligibility.

Measure compilation, variant/code growth, runtime matching/equality and duplicated or retained continuations. Initially run on competent scalar execution. Apply equivalent specialization to E04 or another valid shared engine in E15; a tuned compiled engine versus an uncompiled reference is not an isolated sharing comparison.

**Follow-up:** savings due only to dispatch become compilation evidence. Code growth motivates a bounded variant-policy comparison. Lost modes or effects require repair, not changing the expected program. More general functional-logic specialization requires its fragment premises to be checked before it becomes a new candidate.

### E14 — Trustworthy answer extraction and deduplication costs

**Dependencies:** E00 answer definitions. **Decision enabled:** how much runtime and retained memory are observation costs, and whether faster exact normalization helps.

Compare the reference's exhaustive alpha canonicalization with an independent exact implementation using structural partitioning and search over unresolved symmetries. Candidate fingerprints may reject inequality or select comparison buckets; they may not remove an answer on a collision. Retain full residual multisets and output aliases. Test symmetric residual graphs, disconnected components, multiple variable permutations, multiplicity, equal outputs with different residuals and different programs with equal evaluation results.

Separate raw completion events from recognized answers in experimental traces so that final dedup cannot mask lost or duplicated source alternatives. Measure normalization, key retention, first-answer delay, duplicate density and extraction cost as output count grows. If answer keys dominate an unbounded stream, compare exact retention/recomputation strategies with explicit guarantees; resource exhaustion is not permission for approximate deduplication.

**Follow-up:** observation bottlenecks can be improved in experimental output services without changing the reference algorithm. Logical projection is E10 plus an owner interface decision. Equal final answers never justify merging execution states without E12's stronger certificate.

### E15 — Compose mechanisms and test their interactions

**Dependencies:** two or more validated mechanisms with first-probe results; E02 boundaries and E00 global observations. **Decision enabled:** whether local benefits survive a complete engine and which combinations interfere.

Select combinations by measured explanations rather than taking every Cartesian product. The initial interaction obligations are local expansion × contextual equality; late splitting × fair scheduling; specialization × dynamic sharing; structural solving × tables/learning; and factoring × cross-region wake-up/reunion. For each pair run neither, each alone and both where those configurations have coherent semantics. Record when a dependency makes an ablation impossible and narrow the causal claim.

At least one composition must challenge the most-developed representation with a distinct organization of computation, such as boxes, occurrence graphs or bounded symbolic compilation. A mechanism's negative synthetic result does not exclude an application composition unless the measured limitation applies there; explain any exclusion precisely.

Use shared writable variables across eval/fold and no_c/typing regions, external linked rules, competing consumers, pending publication, failures and delayed wake-ups. Validate the [global effects and completion interface](../goals/chr-sharing/notes/T018-composition-and-learning.md). Measure total interface traffic, conflicting/stale proposals, memory retained by caches/labels, compilation and output in addition to local counts. Search policies must be recorded separately from source semantics.

**Follow-up:** loss of a local gain triggers attribution to the interface, interference or workload reuse frequency. It does not justify reporting only isolated wins. Select complete comparison candidates for E17 with the reasons and contrary evidence attached; leave untested combinations explicitly untested.

### E16 — Parallel execution and whole-system storage

**Dependencies:** a validated sequential composition and actual host capabilities recorded before running. The existing E12 owned-equation integration and E08 certified permanent factors supply independent entry points, as detailed in the [source audit](../goals/chr-experiments/notes/T016-parallel-entry.md); neither waits for the E15 broker or native nets. Establish the relevant finite-service and owner-scheduling premises before making fairness claims; the first bounded finite-work comparison does not establish a general bounded-service guarantee. **Decision enabled:** whether commuting work produces useful parallelism after synchronization and memory costs.

Compare serial execution, the parallel implementation with one worker, and its available worker-count sweep on the same queries and semantics. Include disjoint certified operations, highly contended aliases/consumers, small versus large service tasks, and allocation-heavy frontiers. Validate committed effects and quiescence under forced interleavings before interpreting timings.

Measure useful work, scheduling, contention, synchronization, bandwidth pressure where observable, allocation/reclamation and retained/peak memory. Fair service and answer validity remain requirements. No speedup follows merely from active-pair parallelism or disjoint syntax. If the available host cannot distinguish scaling, collect correctness and work evidence and state the exact machine capability needed for the timing question.

**Follow-up:** a correct parallel slowdown is a result about that granularity/storage regime. Revise the measured bottleneck or keep sequential execution as the supported choice; do not impose source ownership restrictions without the E02 tradeoff. Recheck whole-system memory after each composition, not just isolated services.

### E17 — Application evaluation and final decision audit

**Dependencies:** E00 application cases, E01 controls and E14 observation accounting support independent application workload/oracle development now. Complete E15 candidates are required only for their own comparative claims; E16 only for parallel claims. Unrestricted synthesis, bounded independent oracles and resumed-session probes do not depend on either family succeeding. See [application entry](../goals/chr-experiments/notes/T017-application-entry.md). **Decision enabled:** which language/execution choices are supported for this project's exploratory use, with what uncertainty.

Compare forward evaluation, missing-input arithmetic, fixed-result SK synthesis including duplication, nonground ignored/repeated holes, type-directed SK synthesis, and the literal lambda relation. Use the same relation for checking and synthesis. Include repeated requests for more answers and changed queries/rulesets: report cold construction and warm reuse separately so exploratory-session benefit is visible. No particular notebook UI is needed to replay those sessions.

Run controlled grids for causal explanations and application cases for relevance; neither substitutes for the other. Report finite complete result sets where available. For unbounded searches validate every returned answer and report finite progress/latency without requiring the same order or interpreting a timeout as failure. If policies differ, include aligned-policy measurements where possible and state which result is policy-sensitive. Compilation, setup, service administration, memory retention and answer extraction belong in total cost.

Present tradeoffs across first-answer latency, continued enumeration, total work, memory, compilation and programming burden. Do not average them into an invented utility score. A recommendation can identify a Pareto frontier and ask the owner to resolve a concrete taste tradeoff after showing examples and evidence.

**Follow-up:** a synthetic advantage absent in applications prompts reuse-frequency and bottleneck analysis. A promising application result without semantic coverage remains unvalidated. Before concluding, audit every E-number and every owner decision in the companion document: resolved within stated bounds, contradicted with evidence, specific external/owner dependency, or unfinished. Readiness to build one production engine is not an experiment-program completion condition.

## Measurement and interpretation rules

Correctness runs use deterministic seeds and replay of counterexamples. Use both handwritten semantic cases and the bounded generated-case method in T013; freeze generator and bounds before execution. A generated prefix is not a termination or completeness proof. Minimize discrepancies and preserve the original input and revisions.

For performance runs record compiler/runtime/backend revisions, build flags, CPU, OS, memory availability and resource ceilings. Use release builds, a fixed workload order randomized with a recorded seed, one warm-up and at least five fresh-process measurements per timed configuration initially; retain all observations and report median and range. Warm-cache session experiments are separate recorded sequences. When variation prevents distinguishing configurations, either register further repetitions or report the result as inconclusive. Hardware counters are optional diagnostics, not invented portable metrics.

Set wall-time, memory and service limits in each registration from the available host and a bounded pilot. This plan does not silently import T013's Python host assumptions or its process limits into Rust experiments. Resource ceilings constrain measurement, not source semantics. All configurations in a comparison receive the same declared ceilings; include timeouts and censored memory results. Report the cost of constructing a prepared shared state both separately and in end-to-end totals.

A smaller event count supports a mechanism claim. A faster timed run supports a bounded implementation claim. Neither establishes a universal architecture winner. An improvement must exceed the observed measurement ambiguity before being described as a timed benefit; no post hoc speed threshold selects an architecture. A null result with little reuse opportunity does not refute a reuse mechanism, but it does constrain relevance to that workload.

Stop affected comparative execution on a semantic discrepancy or invalid measurement. Repair/re-register the affected variant and repeat the affected comparison; continue independent experiments. An administrative budget expiring creates an unrun suffix, never an exclusion decision. Do not tune a candidate on its evaluation cases without labeling the tuning and reserving independently frozen cases for the resulting policy.

## Reconcile the existing conditional registration before executing it

T013 remains detailed evidence for E03; it is not an authority over every other experiment. Its Python prototype layout, always-true guard subset, explicit per-leaf scheduling, capped grouping, environment and resource limits must be explicitly reconciled with the Rust reference and this scope in the E03 registration. Use the shared AST where equivalent, state guard coverage, preserve its exact support/lineage and transition obligations, and either retain its registered grids or explain the revision before runs. Do not pool runs across materially different registrations.

The reference implementation does not discharge T013's conditional projection, incremental matching, race, BDD, local-controller or generated-prefix obligations. Its existing counters also do not measure bytes, Boolean operations or scheduler microsteps. E00 and the candidate registrations must supply those missing measurements. T013's finite ground SK generator is a bounded correctness workload; the unbounded synthesis question remains in E17.

## Completion and the immediate next task

This planning task is complete when every research direction has a concrete experiment or explicit owner decision, dependencies permit independent progress, and each experiment has a stated observation and interpretation. That is distinct from executing this plan.

The first execution task is E00's concrete semantic/application registry and operation-observation contract, together with E02's source-property inventory. It produces checkable inputs and eligibility cases, not a new engine. E01 and E14 then establish credible controls and output accounting while the ready mechanism registrations are prepared. No additional question of taste is required to begin these baseline-preserving tasks.

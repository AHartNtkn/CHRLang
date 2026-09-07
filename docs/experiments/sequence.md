# Experimental sequence for CHR language and execution design

Choose an architecture through evidence about total efficiency, necessary implementation complexity, and language-design tradeoffs. Investigate ways to avoid, compile, reorganize, or cheaply repeat work across the language. Sharing and the supplied programs are useful possibilities and examples; neither defines the optimization objective.

**Current authority:** the redesigned experimental investigation, including implementation and runs, is authorized. Work begins with R00 selection. This document governs future work selection; completed E00–E18 results remain evidence with their stated limits. The [coverage map](coverage.md) records current questions and dispositions. The [framing audit](research-framing-review.md) explains the redesign.

## Priorities and work selection

1. Resolve uncertainties that can change the organization of the implementation or a consequential language choice. Consider coherent compiled, interpreted, relational, graph and solver-based organizations without requiring existing subsystem interfaces.
2. Establish whether the expected advantage survives a complete execution path and its necessary machinery. Compare preparation, execution, observation, memory and implementation complexity. Use component ablations to explain differences where appropriate.
3. Broaden evidence where a candidate's apparent advantage may depend on a narrow computational regime. Ordinary CHR without explicit OR has first-class coverage alongside search. No invented workload weighting determines an overall winner.
4. Refine a component or repeat a measurement when doing so resolves an uncertainty that affects a surviving decision. A feasible improvement, an unfinished prototype, a new suggestion, or a completed milestone does not by itself justify priority.

Select the next task by a written qualitative comparison with the strongest ready alternative task. Identify the affected decision, the plausible outcomes, existing evidence, dependencies, anticipated implementation/measurement cost, and why this task has greater likely information value. Do not invent numerical utility scores or precise effort estimates. Repair a defect before using affected evidence; unrelated decisions can continue independently.

A new architectural direction enters on the same terms. It need not resemble a current candidate or wait for one to succeed. A difficult candidate does not block another's analysis or experiments. The sequence below has evidence dependencies, not a requirement to finish every candidate at each stage.

## Semantic and comparison boundaries

Each comparison states its language fragment, observable behavior and scheduling assumptions. Current experimental semantics include finite-tree equality on rule bodies, nonbinding heads/guards, explicit OR, multiset occurrences, propagation behavior, and selected variables with residual relationships. The implemented equality-entailment guard fragment is narrower than an unrestricted catalogue of pure guards. State that limit rather than treating a prototype as the language specification.

Investigate semantic changes where they could enable useful optimization. For each, compare inference under the existing contract, an optional declaration and a mandatory restriction where these are meaningful alternatives. Show eliminated work and a concrete exclusion or reformulation. Adoption remains the owner's decision. A restriction is neither automatically unacceptable nor justified merely because it simplifies the implementation.

The reference remains independent and favors inspectability. Candidates may share suitable syntax and fixtures, not its execution algorithms as their correctness justification. Validate observations and relevant progress through independent expectations, arguments and adverse cases. A matched-policy ablation may demand equal logical traces; an architectural comparison need not reproduce reference microsteps, module boundaries, physical branch objects or diagnostic enumeration. Different permitted CHR schedules do not become implicit source disjunction.

## R00 — Architecture sketches and selection of discriminating experiments

**First work after execution is authorized.** Produce a compact comparison of coherent candidates, drawing on completed evidence and targeted authoritative research. This is not a request to implement every candidate.

Starting organizations to assess include generated rule/occurrence execution with explicit search state; integrated relation or graph rewriting that can eliminate privileged data/control services; conditional or demand-driven execution; and direct compilation of a suitable relation or derivation into a solver. These overlap and can combine. Interpretive execution remains a candidate/control. Add a materially different organization when its mechanism warrants it; this starting set is not exhaustive.

For each candidate, describe an entire path from a query to a trustworthy answer: rule activation, matching, representation/equality, explicit choices, suspension, consumption/history, completion, observation and lifetime. Identify what is static, what runtime state remains, and the invariants needed to maintain it. Provide a favorable case and a case likely to defeat its advantage. Compare necessary mechanisms and coupling qualitatively; source-line counts are not a simplicity score.

Trace small programs analytically to separate unavoidable semantic work from costs of an implementation choice. Reuse established algorithms and counterexamples. Do not build a prototype merely to rediscover a result already supported by analysis. Conversely, do not treat a hand trace or operation-count prediction as a measured performance result.

**Output:** a candidate comparison and a selected first set of decision briefs under R01–R04 or a justified new question. Each selected brief contains competing predictions and an implementation-independent observation contract. Coverage of other candidates records why their next question is lower priority or dependent, without declaring them disproved.

**Exit:** enough specificity to select a discriminating experiment. A catalogue of architecture names is insufficient. No fixed number of prototypes or compulsory implementation for every name is required.

## R01 — Work that compilation and activation can eliminate

**Decision:** whether generic runtime selection is an appropriate core, or generated execution/activation should determine the architecture; which static information is worth obtaining.

**Initial contrast:** generic selection, changed-occurrence/variable activation with competent indexes, and generated rule/occurrence execution. Separate generation from lookup effects with a matched-index comparison where possible. Compare compiled join planning with maintained intermediate matches or cheap recomputation when matching is consequential. Do not require every contrast to preserve an interpretation loop.

**Discriminating cases:** deterministic single-head rewriting with no OR; selective and unselective multihead joins; arrivals and aliases that enable work; consumption and propagation; known versus unknown arguments. Vary rule/store size, update density and match selectivity independently. Include cases where maintaining an index or intermediate match costs more than rediscovery. Check changes of legal scheduling separately from a same-policy ablation.

**Predictions and actions:** if dispatch removal materially improves complete execution with modest machinery, carry generated execution into later comparisons. If lookup dominates after compilation, compare access/activation organizations before expanding code-generation tuning. If both are minor beside binding, storage or output, prioritize the consequential representation question. If static information pays only under a restriction, bring its source and compilation costs into R05 rather than silently imposing it.

**Evidence/dependency:** E04 separates local dispatch from caching; E09 exposes repeated discovery; E13 measures bounded unfolding, not general generated rule execution. See [coverage](coverage.md). R00 supplies candidate-specific premises; this experiment does not depend on a sharing engine or an E18 extension.

## R02 — Representation and elimination of subsystem boundaries

**Decision:** whether terms, equality, matching and source effects should share a substrate, and what representation minimizes total work and necessary machinery.

**Contrasts:** a competent dedicated representation; integrated constructor/identity relations; direct local graph organization where coherent. Compare complete paths, allowing a candidate to eliminate a unifier request, solved-store barrier or central commit. A same-representation boundary ablation is useful only if both versions are credible. An intentionally inefficient adapter is not an architectural control.

**Discriminating cases:** constructor-heavy deterministic rewriting, alias-heavy updates, equality enabling matches, clashes and finite-tree cycles, repeated and mostly distinct structures, consuming multihead rules and propagation. Add explicit alternatives to test context-local updates and failure when needed for the architectural claim. Start with the smallest fragment exercising the claimed eliminated boundary; the fragment must include application effects, not only isolated equation success.

**Predictions and actions:** if eliminated interfaces reduce total work/complexity, test how the advantage survives resource-sensitive source effects. If equality transport, fusion, indexing or validity machinery outweighs the benefit, distinguish inherent obligations from a repairable representation. Implement the repair only if the result could change the architecture comparison. Retain a dedicated representation if the integrated design has no supported advantage in the tested regimes; do not infer a universal impossibility.

**Evidence/dependency:** E18 establishes finite monotone feasibility, E06/E15 expose service overhead, and E12 demonstrates the significance of representation costs. Full integration is not a prerequisite for R01 or R03. The direct named-choice proposal is an eligible candidate here and in R03, without automatic priority.

## R03 — Search state, recomputation and scheduling

**Decision:** how to represent and service explicit alternatives economically, including when recomputing work is preferable to retaining or sharing it.

**Contrasts selected by R00:** credible copying/persistence/trailing/replay organizations; supported or named-choice execution; delayed splitting, factoring or tabling where they alter the decision. No obligation to implement every combination. A cheap independent-branch design can win. A shared design must execute directly in its own organization rather than pay an unnecessary whole-state interface.

**Discriminating cases:** shallow cheap choices, deep restoration paths, wide frontiers, low reuse, repeated work after a choice, immediate discrimination, early/late failure, reconvergence, independent and coupled computations. Include finite siblings beside divergent work, dynamic choice births and joint nonground observations. Retain no-OR overhead controls. Use both complete finite answer sets and sound finite prefixes; do not force equal answer order across different permitted policies.

**Predictions and actions:** map where saved computation exceeds support, key, restoration, switching and retention costs. If low reuse favors duplication, keep it as positive evidence. If a benefit requires a particular schedule, compare the joint design and its progress obligations. If preparation or answer extraction dominates, quantify the bound on possible search improvements before initiating another search refinement.

**Evidence/dependency:** E01/E03–E08/E09/E12/E14/E15 provide bounded mechanisms and adverse cases. R01's completion is not required: use a credible available control, then reassess any ranking sensitive to code generation or activation.

## R04 — Solving or specializing away execution

**Decision:** whether direct compilation, specialization or logical solving avoids enough runtime work to justify its preparation, code/data growth and semantic premises.

**Contrasts:** competent incremental execution; generated specialized code; direct relational/derivation solving on a stated fragment. The E11 interpreter-trace encoding is evidence about that encoding, not a compulsory solver design. Exact structural solving, tables and failure learning may be candidate components where their effects change this decision.

**Discriminating cases:** varied known input/output shapes and unknown arguments; selective and unselective constraints; early contradiction; repeated and distinct queries over one ruleset; unbounded recursion alongside explicitly bounded solver instances. Include resource-sensitive near misses for a proposed logical region. Do not equate multiset residuals with formulas without an experimental semantic change.

**Predictions and actions:** compare total cold cost and measured reuse/amortization, including compilation, validation of certificates, solving and answer recovery. If preparation dominates, identify whether a plausible reuse regime can reverse the result before extending bounds. If compact solving avoids large execution, test witness validity and the cost of obtaining further answers. If the benefit depends on lost modes or changed observations, expose that tradeoff under R05.

**Dependency:** a precise fragment and independent denotation, not the maturity of graph or conditional engines. Source analysis may resolve candidate eligibility without running a solver.

## R05 — Interactions and language-design tradeoffs

**Decision:** which coherent combinations and language properties earn their complexity in a complete architecture.

Investigate interactions as soon as they are decision-critical; do not wait for every component to be perfected. Examples include compilation with binding-aware activation, integrated equality with destructive effects, storage with finite-service scheduling, and static restrictions with simpler representations. Select combinations from evidence, not the full Cartesian product.

Use neither/each/both ablations when semantically coherent. Otherwise compare complete organizations and state what causal attribution is unavailable. Record required invariants, mutable structures, lifetime ownership, analyses, coordination and exception paths. Identify machinery made unnecessary as well as machinery introduced. Ordinary runtime representation choices remain reversible; language adoption is separate.

For each proposed language change, give the executable/analytical correspondence for accepted programs, excluded or reformulated examples, inference/declaration alternatives and measured or justified savings. Correctness, expressiveness and simplicity are separate dimensions; no invented scalar score trades them away.

**Exit:** a supported combination or a concrete unresolved tradeoff. A local improvement disappearing in composition is evidence requiring explanation, not permission to report only its local advantage.

## R06 — Generalization, lifetime and parallel cost

**Decision:** whether a surviving architectural conclusion holds beyond its discovery cases and which implementation costs can reverse it.

Use held-out programs and parameter regions chosen for contrasting computational characteristics. Test long-lived preparation and changing queries, cold starts, answer streams, allocation/reclamation, and memory retention where relevant. Observation policy comparisons use exact answers and credible eager controls; more E14 repetitions require a consequential remaining uncertainty.

Parallelism receives an independent early feasibility assessment in R00. Implement and time it when coordination or available parallel work can distinguish a candidate, including during R01–R05 if necessary. It does not universally wait for a complete sequential engine. Use serial, one-worker and multiworker controls with comparable representations; charge preparation, transfer, synchronization and shutdown. Hardware limitations identify an exact missing measurement rather than an architectural rejection.

Broader validation includes ordinary rewriting, incremental joins, binding-heavy programs, explicit search and output-heavy cases. Supplied arithmetic, SK, typing and lambda examples remain useful tests. Domain labels do not determine priority or benchmark weights. A range of regimes supports conditional recommendations, not an invented universal workload distribution.

**Exit:** documented applicability and limits of the surviving comparisons. Repetitions are justified by decision-relevant uncertainty; observation-count targets are not research outcomes.

## R07 — Architecture recommendation and research closure

Compare viable architectures on total runtime, preparation/reuse, memory/lifetime, progress, accepted language and necessary implementation complexity. Include contrary evidence and the best plausible alternative. State concrete owner decisions about language or cost tradeoffs only after explaining their consequences.

For each relevant unresolved question, identify evidence, remaining uncertainty, plausible follow-ups, cost, and whether it can materially change a project decision. A bounded conclusion can be sufficient without claiming universal optimality. A feasible investigation with material decision value must continue once execution is authorized; merely feasible tuning does not make it mandatory. Specific owner or unavailable-external dependencies must name the exact unblock, with unaffected work continuing.

Production readiness, a successful demo, a completed matrix, a timeout or an administrative limit does not establish closure. The final audit concerns questions and decisions, not completing every R-stage or E-family by name.

## Registration and measurement requirements

Before comparative runs, write `docs/experiments/registrations/Rxx-<question>.md` and record its selection in the coverage map. Include:

- Architectural decision, alternatives, discriminating predictions and action for each plausible outcome.
- Accepted semantics, eligibility/correspondence, independent oracle, correctness and adverse controls.
- Exact workloads, configurations, seeds, bounds and observation endpoints; explain why they expose the claimed difference.
- Preparation, runtime, memory, lifetime, scheduling and extraction measurements; distinguish physical work from logical effects and bytes from node counts.
- Compiler/runtime revisions, source freeze, environment, isolation, repetition and analysis protocol; distinguish exploration from confirmation.
- Known confounds, conditions invalidating the comparison, and what result would justify further implementation or measurement.

Use release/native measurements where appropriate to the claim. Cross-language or differently instrumented prototypes do not give unqualified architecture rankings. Timed runs exclude independent oracle work or disclose a boundary that prevents the proposed comparison. Keep warm-up, randomized repetition, memory instrumentation and lifecycle accounting explicit. Do not prescribe a universal repetition count before workload variability and the decision are known.

Preserve every unfavorable result and cutoff. Investigate defects, invalid comparisons and consequential inconclusive outcomes. Repair the cause and repeat affected checks; do not weaken semantics or choose favorable cases. Freeze a fresh confirmatory test when exploration changes a policy. Tests validate the claim, not only labels or scaffolding.

Keep durable results in `docs/experiments/results/`, with reproducible inputs and command/source records. Update the question disposition and actual next selection after a result. No report's follow-up paragraph independently authorizes or prioritizes another experiment.

## Immediate handoff

[R00's first selection](results/R00-architecture-selection.md) assigns bounded R01 activation/code-generation implementation and independent R04 finite-consistency analysis. Their current status and subsequent selection rationale live in the [coverage map](coverage.md). No other prototype is selected automatically by this plan. Implementation and comparative execution follow the relevant decision brief and prospective registration.

# Research framing audit and proposed sequence

The investigation does not yet support an architecture choice. Its main weakness is the connection between experimental results and architectural decisions. The sequence permits extensive refinement of particular mechanisms before establishing whether their organization of computation is worth pursuing.

Experiments are stopped. This document proposes a research redesign for review; it does not authorize implementation or runs, select an architecture, or adopt language changes.

## Findings supported by the repository

**The agenda is broader on paper than its opening objective suggests.** The [sequence](sequence.md) opens by making synthesis workloads the destination. Yet it also includes storage, specialization, solving, language restrictions and parallelism. Calling this an investigation of sharing alone would misdescribe it. The problem is that examples and particular mechanisms organize the agenda without a justified account of how their results support an overall architecture decision.

**Coverage of named mechanisms is being used as a substitute for coverage of architectural alternatives.** The execution order prioritizes an untested family, then refinements, then larger ranges. That guards against neglect within the existing list, but does not establish that its families are the right units of investigation. Several entries ask how to improve a subsystem whose existence other architectures might avoid. The [assumption register](architecture-assumptions.md) identifies these boundaries, but assigning each another experiment does not establish its relative importance.

**Some controls constrain the question to the incumbent implementation.** The [persistent engine](../../research/chr-persistent/src/state.rs) processes pending operations, calls a unifier, and then scans rules to select an application. The [Python source job](../../research/chr-scheduling/source.py) similarly exposes complete source steps and equality requests. These are legitimate experimental organizations. They cannot serve as mandatory interfaces for generated rule code, integrated relation rewriting, or a distributed graph. The register now says this explicitly; the broader comparative evidence is still limited by the implemented contrasts.

**The default separation of mechanisms from integration is too strong.** The sequence argues that early combinations make speed differences difficult to explain. That is a reason for controlled ablations, not a general reason to postpone integrated prototypes. Whether a representation avoids dispatch, copying, suspension or an entire interface can depend on compiling and executing a complete path. A component comparison can penalize an architecture precisely because it has to fit a boundary that the architecture would eliminate.

**The follow-up rule encourages indefinite refinement.** Reports usually conclude with several feasible improvements and an open direction. Feasibility establishes that work is possible; it does not establish that doing it is a good use of the investigation. A useful next experiment needs a consequential uncertainty, contrasting predictions, and a reason to prefer resolving it over another uncertainty. Conversely, an inexpensive test that can overturn a major architectural premise should not wait for a mature candidate's remaining refinements.

**Semantic scope and implementation scope need separate entries in every architectural claim.** The sequence explicitly uses equality-entailment guards for its initial experimental language. That permits bounded experiments; it does not establish coverage of every nonbinding guard. Likewise, a monotone relational closure is not full consuming CHR, and a fixed-selector comparison does not make that selector mandatory. Each candidate should identify its accepted language fragment, permitted execution policy, and observed outputs before the performance comparison is interpreted.

**Simplicity is an objective without a consistent assessment.** The records carefully count operations and allocations. They do not consistently compare the machinery each candidate needs: mutable state, coordination protocols, retained indexes, compiler analyses, invariants, or interactions among them. Line counts would not fix this. The relevant question is what complexity is necessary to obtain the measured benefit and preserve the language contract.

**The workload collection is not a justified basis for a universal ranking.** Carry, arithmetic, SK and lambda cases test real obligations, but do not establish a workload distribution. The plan contains no-OR and adverse controls; their presence does not establish broad architectural coverage. Ordinary deterministic rewriting, incremental multihead computation, binding-heavy work, shallow and deep search, low-reuse and high-reuse execution, and output-dominated work need explicit places in the comparison. These are proposed computational characteristics, not new target applications or an assertion that they occur equally often.

This audit examines the written program, result reports and selected implementation boundaries. It does not revalidate every raw measurement or estimate the fraction of effort that was useful. The documented limitations support the findings above without inventing such a fraction.

## What the existing evidence can inform

These are bounded uses of evidence. A family remaining technically open does not automatically place its next refinement in the proposed execution queue.

| Evidence | Architectural information it supplies | What it does not settle / proposed use |
|---|---|---|
| [E00](results/E00.md), [native regressions](results/E06-native.md) | Concrete requirements for aliases, occurrence effects, correlated choices, failure and progress | Preserve as independent checks where applicable. Do not require candidates to reproduce diagnostic containers or physical raw-branch enumeration. |
| [E01 storage](results/E01.md) | Snapshot representation changes allocation; retained outputs can dominate memory | Persistent versus copying is not a complete storage comparison. Trailing/replay are documented possibilities, not automatically mandatory builds. |
| [E02 inventory](results/E02-initial-inventory.md) | Concrete examples of what local dispatch, ownership and solver restrictions accept or exclude | No measured benefit establishes a global language restriction. Reassess restrictions alongside the work they eliminate. |
| [E03 conditional execution](results/E03.md), [E04 relation graphs](results/E04.md), [E05 named families](results/E05.md) | Shared work can be outweighed by grouping, contextual bookkeeping or representation costs; local dispatch is a separate benefit | Neither fewer expansions nor an equality-service result selects an engine. Retain their cost explanations for architectural screening. |
| [E06 net results](results/E06-unification-costs.md), [E15 complete costs](results/E15-costs.md) | The measured Python net services are costly relative to their direct equality control; status action savings do not imply lower runtime | This does not rank direct compiled graph execution. Further repetitions of the same service matrix have no demonstrated architecture-selection purpose. |
| [E07 delayed splitting](results/E07.md), [E08 factors](results/E08.md) | Scheduling and independence can avoid work; speculation, output products and certification impose countervailing costs | The certified fragments do not establish general policy or whole-language benefit. Use opposing workloads to screen any proposed extension. |
| [E09 maintained matching](results/E09-maintained-sizing.md) | Retention avoids rematching yet repeated discovery of cached tests remains expensive in counted actions | It motivates comparing how matches are discovered, not merely another cache refinement. Runtime and retained-memory claims require separate evidence. |
| [E10 structural services](results/E10.md), [E12 tables](results/E12-equations.md), [failure checks](results/E12-failure-native.md) | Keys, projection and proof access can cost more than direct work; selective access can change that balance | Consider solving, learning and recomputation as competing ways to avoid work. Do not equate cache hits with architectural value. |
| [E11 symbolic execution](results/E11-matched.md) | The tested bounded trace encoding is costly, including construction and observation | Directly compiling relations or derivations is a different proposition. No general solver-compilation rejection follows. |
| [E13 specialization](results/E13-costs.md) | A single-entry unfolding representation reduces dispatch but does not repay compilation allocation within three searches on terminating cases | This does not evaluate general generated rule execution. Repeating its unfolding-budget sweep is not a substitute for examining code generation and static-work elimination. |
| [E14 observation](results/E14-graph-costs.md) | Clone avoidance and export avoidance are distinct; graph observation has both favorable and adverse measured cases | Useful component evidence. It cannot determine the execution architecture; further refinement needs a demonstrated role in that decision. |
| [E16 workers](results/E16-costs.md), [regional pilot](results/E16-regions-pilot.md) | Transfer/granularity can dominate parallel gains; coarse regions have a different cost balance | A cold owned-equation result does not reject parallel execution. More regional repetitions require a decision they can change. |
| [E18 constructor relations](results/E18-relational-gate.md) | A bounded monotone fragment can integrate constructor consistency and application deductions without a unifier API | Semantic feasibility is established for that fragment. Full CHR correspondence, representation efficiency and implementation simplicity are unestablished. |

## Proposed priorities

The first priority is to identify credible organizations of execution and the costs they remove or introduce. Correctness is a condition for interpreting their results. Runtime, memory, preparation and implementation simplicity then inform architectural choice; no single operation count represents them all.

Language properties belong in that comparison from the start. For each proposed optimization, distinguish a property inferred from an ordinary program, an optional declaration, and a mandatory restriction. State which work becomes unnecessary and give both an accepted program and a meaningful exclusion or reformulation. A simpler compiler is a possible benefit; it is not sufficient justification to adopt a restriction.

No mechanism receives priority because it was suggested, implemented first, or has accumulated measurements. Deliberately recomputing cheap work can be a sound optimization. An architecture that runs ordinary CHR efficiently deserves the same scrutiny as one that compresses explicit search. Broader exploration does not authorize inventing unrelated language features.

Without a justified weighting of workloads and costs, recommendations should identify regimes and tradeoffs. Do not manufacture an average score or choose a preferred workload mix. Ask for an owner decision only when concrete surviving alternatives require a preference that the evidence cannot determine.

## The architectural questions to organize around

These axes describe choices that can interact; their Cartesian product is not a required experiment list. Candidate designs must combine them coherently and explain their complete execution path.

| Question | Substantive alternatives to examine | Discriminating evidence |
|---|---|---|
| What work survives compilation? | Generic interpretation; generated rule/occurrence code; static argument and dead-work elimination; direct relational/solver compilation | Actual runtime operations eliminated, generated size, preparation cost, behavior with known and unknown inputs |
| What triggers computation? | Repeated selection; changed-occurrence/variable activation; maintained joins; local graph interactions | Irrelevant work avoided versus index/subscription maintenance under insertions, consumption and aliasing |
| How are terms and equality represented? | Dedicated term/binding store; integrated identity/constructor relations; eligible direct representations | Consistency, updates, matching and observation in one complete path, including costs created by the representation |
| How does search retain work? | Copying, persistence, trailing or replay; supported/shared execution; delayed splitting; tables or solving | Cheap and costly branches, divergence, early failure, little reuse and substantial reuse, with switching and retention charged |
| What can be known statically? | Inferred or declared modes, determinism, nonoverlap, ownership, functional dependencies, region closure | Optimization actually enabled, inference/annotation burden, lost or reformulated programs and linking consequences |
| Where do control and lifetime costs live? | Central scheduling; local work queues; batching; eager/lazy validity; region or object reclamation | Required invariants, publication/progress correctness, coordination, allocation and retained memory |
| When does parallelism earn its machinery? | Sequential execution; parallel independent regions; finer concurrent work | Useful work after transfer/synchronization, granularity thresholds, memory pressure and service latency |

There is an established compiler direction that deserves explicit consideration alongside the graph and solver candidates. Holzbaur et al. describe CHR compilation using types, modes, determinism, functional dependencies and symmetry, including specialized indexes and continuation optimizations. Their operational assumptions include textual priority, so the results are not automatically transferable to this project's experimental scheduling contract. The paper establishes concrete alternatives to repeated generic selection; it does not establish their performance here. [Primary paper, §§2–5](https://arxiv.org/pdf/cs/0408025).

Join planning is also a compilation question, beyond retaining runtime match results. Lam and Cervesato give a compilation scheme for matching and guard ordering in CHR with comprehension patterns. Its language and cost model need separate transfer checks. It is evidence of a concrete approach to investigate, not permission to claim optimal joins for this language. [Primary report](https://www.cs.cmu.edu/~iliano/papers/cmu-cs-14-119.pdf).

These source checks address whether a missing architectural contrast is substantive. They do not justify another implementation before the proposed sequence is approved.

## Proposed experimental sequence

1. **Complete a small set of coherent architectural sketches.** Each must describe rule execution, matching, equality, explicit alternatives, suspension/wake-up, completion and observation. Explain both its expected advantage and its strongest plausible disadvantage. Include conventional compiled execution as a serious candidate, and alternatives that can eliminate its boundaries. Do not require every candidate to use the existing AST internally or become a complete compiler before it can be assessed.

2. **Screen assumptions before building large comparisons.** For each consequential difference, choose a minimal program and trace how the candidate would execute it. Identify unavoidable work, implementation-specific overhead and semantic restrictions. Use available evidence and authoritative algorithms to close questions analysis can answer. Uncertain estimates remain predictions. Select prototype work only where different outcomes could change which design or combination remains attractive.

3. **Build the smallest integrated comparisons that exercise those differences.** A proposed initial contrast is generated, indexed rule execution versus generic selection on deterministic/no-OR and incrementally enabled multihead programs. A second contrasts explicit branch storage with a coherent alternative organization on both low-reuse and high-reuse search. Integrated constructor/graph execution and direct solver compilation should receive an implementation only for the decision-critical uncertainty identified in step2; they need not fit a service API. The precise entrants and programs must be registered after the sketches, before execution. These are starting contrasts, not an exhaustive list or an adopted architecture.

4. **Explain observed differences and test interactions.** Use matched ablations where they isolate a meaningful cause. Compare complete paths when a design gains by eliminating a boundary. Include construction, runtime, storage, scheduling and observation. An adverse result may warrant a repair or another representation, but the proposed repair must state what conclusion it could change. Implementation defects that invalidate a comparison must be fixed; a correct unfavorable result must remain evidence.

5. **Validate surviving choices across varied programs and scales.** Select programs by the computational characteristics they exercise, including ordinary CHR without explicit search. The supplied examples remain valuable cases. Include finite complete queries and appropriately checked prefixes of unbounded queries. Repetition resolves measurement uncertainty only when that uncertainty affects a decision. Cross-language or differently instrumented prototypes need explicit attribution limits; do not infer architecture rankings from their raw timing ratios.

6. **Recommend an architecture or present a concrete unresolved tradeoff.** Explain which results support the recommendation, which contradict it, what semantic or implementation complexity it requires, and where it performs poorly. State why the most plausible further experiments would or would not change that recommendation. Surviving uncertainty can support a bounded conclusion without pretending the entire design space has been exhausted.

For example, the generated-rule comparison should distinguish three explanations: repeated interpretation dominates, partner lookup dominates, or both are minor beside binding/storage work. Compare generated and interpreted execution with equivalent indexing when isolating dispatch; then compare indexing opportunities explicitly. Charge compilation and reused-query execution separately. If dispatch savings are small after competent indexing, that bounds the case for a more elaborate compiler on those workloads. If representation and activation costs dominate, investigate those before expanding the compilation matrix. Neither outcome establishes a universal winner.

Before any proposed batch, its brief must answer: what decision could change, what outcomes distinguish the alternatives, why this workload exposes that distinction, and why this is the best next use of effort. If every plausible result merely produces another tuning task, reconsider the question before running it.

There is no blanket requirement to perfect a mechanism before comparing architectures. There is also no permission to treat an incorrect candidate as a cheap negative result. Validation should be proportional to the claim and reusable across candidates; independent semantic checks remain essential.

## Approval boundary and current disposition

The current sequence is suspended. Its evidence remains available under the bounds above. Neither the direct-choice prototype nor a mature component refinement has automatic priority in a revised program.

This proposal recommends resuming with architectural sketches and discriminating integrated comparisons, selected by their ability to affect the architecture decision. It does not claim that a compiled scalar engine, a graph engine, or a solver engine should win. Approval of this proposal would establish the research approach; individual batches would still need the stated decision rationale and prospective registration. Experimental execution remains stopped pending that approval.

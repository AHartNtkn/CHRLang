# Investigation protocol

The investigation must establish which shared-execution mechanisms can preserve the agreed CHR semantics and where they provide useful savings. Architecture selection follows evidence about correctness, coverage, and cost.

This document specifies future research. Seed references below are leads from preliminary conversation searches, not a completed literature review. No performance hypothesis has yet been tested.

## 1. Establish the semantic reference

Read primary definitions of CHR, CHR with disjunction, and propagation history. Describe rule application, matching versus unification, built-in entailment, failure, and residual answers independently of an implementation representation. Resolve O01 before treating a reference evaluator as an oracle.

Define how a represented collection of alternatives denotes ordinary CHR states. State which transitions are preserved and whether the claim concerns soundness, reachable answers, termination, fairness, or multiplicity. For nonconfluent programs, two permitted schedules can legitimately disagree: equality with one arbitrarily scheduled reference run is not a correctness test. Conversely, do not turn alternative CHR schedules into implicit language search.

Use bounded exploration of reference transitions as a testing aid where feasible. Establish coverage bounds and distinguish such meta-level exploration from the language's explicit disjunction. For terminating confluent examples, result equivalence can provide a stronger and simpler comparison. Preservation of successful explicit alternatives must be justified separately from answer soundness. State any bounded or conditional completeness result; never equate it with exploring alternative CHR schedules. A fairness claim must state its assumptions about finite work steps and resource limits.

## 2. Search and assess the literature

Search these intersecting areas, expanding them when references identify missing mechanisms:

- CHR operational semantics, CHR with disjunction, parallel/concurrent CHR, confluence, propagation history, and rule priorities.
- Named superpositions, interaction nets/calculi, duplication and choice correlation, and algorithms for extracting alternatives.
- Functional-logic evaluation, call-time choice, graph rewriting, bubbling/pull-tabbing, and shared nondeterminism.
- Constraint and logic-programming search with persistent stores, trailing, copying, recomputation, tabling, and dependency-directed reuse.
- Symbolic conditions, decision diagrams, conditional rewriting, and provenance where they could represent membership or dependency across alternatives.

For each area, record search engine/index, date, exact queries, screened results, inclusion/exclusion reasons, and backward/forward citation searches. Prefer original papers, formal definitions, author-maintained implementations, and official documentation. Reviews help discover sources but do not substitute for checking the original claim. Record repository revisions and software versions when implementation behavior matters.

Each source record must contain: bibliographic identity and locator; claim and exact section/page or code location; assumptions; evidence type (proof, experiment, implementation, conjecture); relevance to C01–C05; known limitations; and any mismatch with S01–S09. Record contradictions instead of silently choosing a convenient account. Separate a source's claim from our proposed transfer to CHR.

Candidate inclusion requires either a plausible mechanism for the central sharing requirement, a useful control, or an important impossibility/limitation result. Exclusion requires an articulated mismatch; familiarity, implementation convenience, or a superficial term match is insufficient. Closely related implementations may belong to one mechanism family.

Begin M1 with a bounded search tranche: one documented query pass through each listed area, screening and recording the first 20 results per query, followed by one backward-citation pass through selected primary sources. This is an initial retrieval limit, not a cap on the entire research project or a claim that lower-ranked results are irrelevant. Record the selected queries and source selection before citation expansion. Hold a coverage checkpoint after that pass; justify a specific next expansion from observed gaps rather than continuing indefinitely. Subsequent passes follow the same checkpoint discipline.

Coverage review occurs when every area has been investigated or has a documented access/relevance gap, foundational citations have been followed, and two successive citation-expansion rounds reveal no new mechanism family. This is a practical stopping rule, not proof of exhaustiveness. Record remaining uncertainty and reopen coverage when later evidence exposes a gap.

### Initial leads

These links identify a starting bibliography only. Verify versions and supporting passages during M1.

- Betz and Frühwirth, [Linear-Logic Based Analysis of Constraint Handling Rules with Disjunction](https://arxiv.org/abs/1009.2900): CHR with disjunction and its semantic treatment.
- Frühwirth, [Parallelism, concurrency and distribution in constraint handling rules: A survey](https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/article/parallelism-concurrency-and-distribution-in-constraint-handling-rules-a-survey/08565F9277BCA4509166D7B844C75251): map concurrent CHR approaches and trace primary references.
- [CHR with user-definable rule priorities](https://www.cs.kuleuven.be/publicaties/rapporten/cw/CW479.pdf): distinguish priority semantics from the selected unordered baseline.
- HigherOrderCO, [HVM4](https://github.com/HigherOrderCO/HVM4) and [HVM3 term representation](https://github.com/HigherOrderCO/HVM3/blob/main/HVM.md): identify the precise version and mechanism meant by named superpositions before transferring claims.
- HigherOrderCO, [Bend duplication and superposition documentation](https://github.com/HigherOrderCO/Bend/blob/main/docs/dups-and-sups.md): investigate restrictions and version applicability, not evidence that the mechanism already implements CHR search.

## 3. Construct comparable candidates

Separate semantic representation, storage, scheduling, and answer extraction. They are design dimensions, not mutually exclusive architectures. Named-superposition approaches must receive direct consideration; conditional membership, persistent stores, and shared execution graphs are additional leads, not an exhaustive shortlist.

For each materially distinct candidate, specify:

1. The denotation of its runtime state and its correspondence to the reference states, including preservation of explicit alternatives under stated assumptions.
2. Where choice identity and correlation live; how repeated dynamic choices receive distinct identities.
3. How terms, aliases, constraint occurrences, multiplicity, pending matches, and propagation history depend on choices.
4. How matching, finite-tree unification, insertion, consumption, propagation, and failure operate.
5. What permits an operation to execute once across alternatives and what requires separation.
6. How branch quiescence and answer extraction are detected without premature success.
7. What scheduler assumptions, normalization/collapse operations, and reclamation mechanisms are required.
8. Expected costs, existing evidence, unsupported claims, and the strongest known counterexample.

Use examples to challenge explicit invariants, not to select candidates by anecdote. Review should actively seek a representation that the classification misses and a case where each leading candidate fails. Before prototypes, record reasons for inclusion, exclusion, and any deliberate hybrid.

## 4. Define correctness evidence

A semantic argument should show that representation changes preserve denotation and that computational transitions correspond to permitted CHR behavior. If a shared step summarizes several reference steps, state their dependencies. Include failure and quiescence; successful local matching alone is insufficient. Stronger proof methods may be proposed, but no particular prover or witness is a completion requirement.

Executable validation should challenge at least these obligations:

- Correlation of repeated uses of one choice; independence of distinct choices; nested and recursively created choices.
- Aliasing and finite-tree occurs failures whose applicability differs across alternatives.
- Multiheaded matching and conditional consumption, including one occurrence retained in some alternatives and consumed in others.
- Multiset duplicates and propagation histories with overlapping conditional matches.
- Work independent of an early choice, work carrying it without inspection, and work requiring immediate discrimination.
- Failure affecting a subset of alternatives; alternatives with active work despite ground outputs; quiescence under pending conditional work.
- Nonground answers, residual conditions, existential renaming, and duplicate-answer equivalence.
- Nonconfluent schedules and divergent alternatives, with validity and liveness checked separately.

Derive cases from the semantics and candidate invariants. Later, use generated bounded programs/queries and counterexample reduction in addition to hand-worked cases. A finite test suite does not prove general correctness. Instrumentation should count actual runtime events rather than infer behavior from labels or source substrings.

## 5. Register experiments before measurement

Every experiment record must identify its research question, falsifiable hypothesis, candidate revision, controls, independent variables, observed quantities, expected discrimination between explanations, correctness oracle, resource limits, analysis method, and decision enabled. Freeze this record before collecting comparative measurements. Label subsequent changes and exploratory results explicitly.

A microexample can refute correctness or demonstrate that a specific operation is shared. It cannot establish representative speed, scalability, or universal sharing. Analytical bounds must state their model, including matching, condition manipulation, history, and extraction costs.

Proposed workload families must include both opportunities and adverse cases:

- Early choices with tunable amounts of subsequent independent computation and delayed discrimination.
- Frequent immediate discrimination with little reusable work.
- Independent versus correlated choices, nested choices, varied fan-out, and growing numbers of live alternatives.
- Multiheaded joins, variable aliasing, varied constraint multiplicity, and propagation-heavy execution.
- Early and late failure, mostly failing search, and output-heavy search with unavoidable enumeration costs.
- CHR programs with no disjunction to measure baseline overhead.
- Application-derived workloads selected from the literature and owner use cases, with an explicit rationale for relevance.

Vary store size, common-work fraction, discrimination depth, constraint selectivity, and answer count independently where possible. Do not claim a synthetic family is representative merely because a candidate performs well on it.

Controls must include a credible unshared search implementation and a storage-sharing control that does not claim to share computation. Survey evidence should determine their concrete form; whole-store copying alone may be an artificially weak baseline. Align semantics, query, indexing, compiler configuration, and hardware. Where scheduling differs, report it as a factor rather than attributing the entire difference to sharing.

Measure wall time, peak and retained memory, rule firings, matching/unification work, choice-condition operations, propagation-history overhead, allocation/reclamation, and answer-extraction/deduplication cost. Report first-answer latency and all-answer cost when both are defined. Include no-solution and timeout results. Repeated runs, warm-up policy, seeds, environment, raw observations, variability, and resource ceilings must be reproducible. Separate semantic work saved from total elapsed-time benefit.

Use ablations to isolate mechanisms when technically meaningful. Review complexity through required invariants and interacting mechanisms as well as code size. Determine numerical acceptance thresholds at M2 from the cost model and intended workloads; do not choose them after seeing results.

## 6. Review gates and record keeping

M1 review checks semantic fidelity, source quality, mechanism coverage, transfer assumptions, and unresolved contradictions. It yields a justified M2 task or an explicit research gap; it does not authorize coding by default.

M2 review checks candidate comparability, test validity, baseline strength, workload relevance, and whether each experiment can change a decision. Prototype work needs explicit scope, files, verification, resource limits, and a stopping rule. Stop a candidate experiment on a semantic counterexample until the mechanism or claim is repaired; retain the finding.

M3 review checks reproducibility, all outcomes including negative results, sensitivity to scheduling/workloads, and generalization limits. M4 presents a recommendation with contrary evidence and remaining uncertainty. Escalate product/semantic choices to the owner; resolve factual questions through research.

The PM maintains task receipts in state.yaml. Longer findings go in notes/ with stable task and claim references. Each receipt states the question answered, evidence locations, conclusion strength, limitations, and decision enabled. Decision records identify requirements, considered mechanisms, evidence, rationale, uncertainty, and what new evidence would reopen the decision.

The preparation audit must verify requirement coverage and internal consistency. It cannot certify the feasibility of the language or the performance of an architecture.

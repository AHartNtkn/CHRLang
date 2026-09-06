# Research charter: shared execution for CHR with explicit disjunction

Investigate language design and execution architecture together to develop a small, efficient CHR variant with shared computation across explicit alternatives. Discover which semantic choices enable useful execution mechanisms, and assess their costs before recommending a design.

The current deliverable is this research charter, an investigation protocol, and a prepared task record. No evaluator architecture has been selected, and no implementation or performance result is claimed.

## Purpose and authority

The owner wants reusable CHR rulesets, separate queries selecting returned variables, finite-tree unification, and explicit disjunction. The central requirement is that an early choice must not force the entire constraint store or the subsequent common computation to split. Computation that carries a choice without distinguishing its alternatives should remain shared.

The owner approved preparing a reviewable charter and investigation protocol after requesting proper research management. This authorizes the present preparation milestone. The subsequent investigation described here is prepared work, not work already performed. Starting that investigation is a separate phase boundary.

This is an existing-plan research project. Its intended beneficiary is the language's designer and eventual implementers. The principal risk is selecting an architecture through familiar examples or apparent compactness without establishing semantic correctness, candidate coverage, or net benefit.

## Current semantic baseline

The identifiers below record the owner's current decisions and provide a reference for comparisons. They govern the current language model; they do not bound the research design space. Research may challenge these decisions through explicit proposals under the responsibility defined next.

- **S01 — CHR baseline.** Preserve ordinary CHR using abstract committed-choice semantics, including ordinary heads, guards, simplification, simpagation, and propagation. Rule or body source order does not guarantee execution order. Nonconfluent programs may produce different outcomes under different schedules. Rule competition does not generate search alternatives.
- **S02 — Terms and equality.** Terms are finite first-order trees. The new unification equation propagates bindings and fails when finite-tree unification is impossible, including equations that would create cycles. It is allowed only on the RHS of a rule and is rejected in guards. Guard semantics are unchanged. Logical tree structure does not forbid an implementation from sharing acyclic substructure.
- **S03 — Explicit alternatives.** Only explicit disjunction introduces search. Each arm can contain arbitrary rule-body conjunctions, including nested disjunctions. Choice labels, superpositions, and failure are internal execution concepts, not terms that rules can inspect or match.
- **S04 — Conditional failure.** Failure excludes the affected alternative. Shared execution must preserve exactly which alternatives are affected.
- **S05 — Sharing objective.** Avoid whole-store splitting at a choice. Share subsequent computation that does not distinguish the alternatives, including computation that passes choice-dependent values onward. Sharing the prefix before a choice or sharing storage alone does not establish this requirement.
- **S06 — Search policy.** Interleaving is the default direction. Fairness and protection against starvation are target properties. The scheduler and any mechanism for exposing alternatives remain open to experimentation alongside the sharing representation.
- **S07 — Answers.** Return answers only from nonfailed alternatives at quiescence: no rule application remains enabled. Outputs may be nonground and accompanied by residual conditions. Intermediate states, if exposed for debugging, are provisional. Groundness alone does not establish success.
- **S08 — Duplicates.** Distinct successful alternatives may produce duplicate answers. Final-answer deduplication is a required capability and must remove only answers known to be equivalent, including their residual conditions. Consistent renaming of unbound variables must be respected. Deduplication during evaluation deserves investigation when sound; it is not a required storage architecture.
- **S09 — Reuse.** A program has two separable pieces: a reusable ruleset and a query. The query supplies initial constraints and names the variables to return. Concrete syntax is not selected.

Future interests include coinductive streams, stream calculus, and representations of real numbers. They do not change the finite-tree target or require corecursive evaluation in the initial implementation.

## Joint language and execution research

The research team must actively discover language-design choices that could enable better compilation, sharing, scheduling, or simpler execution. Investigate in both directions: identify execution costs imposed by language features, and identify the language properties required by promising execution mechanisms. Do not wait for the owner to suggest possibilities or for implementation work to encounter a bottleneck.

Develop and evaluate proposals that change the language where warranted by a plausible mechanism, including proposals that challenge previous decisions. A mismatch with the current baseline calls for a semantic comparison, not automatic exclusion. The project's purpose remains the basis for judging whether a tradeoff is worthwhile; neither unrestricted expressiveness nor maximum restriction is a predetermined outcome.

For each proposal, explain the precise language change, the execution opportunity it creates, the evidence for that connection, and its effects on expressiveness, ordinary programming, analyzability, and implementation complexity. Show affected programs and any required reformulation. Distinguish a language-wide restriction from a property a compiler can establish for particular programs; compare those approaches where applicable.

The team owns opportunity discovery, investigation, and reasoned recommendations. The owner decides whether to adopt a semantic change after seeing its consequences. Research authorization includes analyzing alternative semantics without first seeking approval for each hypothesis. Label candidate semantics explicitly; do not silently change the baseline or treat a proposed change as accepted.

## Open questions and their owners

These questions must not silently become assumptions. Researchers should first distinguish facts answerable from sources from choices requiring the owner.

- **O01 — Exact reference semantics (research, then owner for choices).** Identify a CHR transition system suitable for explicit disjunction, finite-tree built-ins, and propagation history. State supported guards and built-ins precisely. Separate confluence, termination, and fairness claims. Do not import a host language's effects or search semantics by accident.
- **O02 — Query operations (owner when needed).** Does the query contain only initial user constraints, or may it directly include the new equality/disjunction operations? The body-only equality decision remains controlling until clarified. Initial variable sharing can already be expressed by repeated query variables.
- **O03 — Answer representation (research, then owner).** Specify how residual conditions and existential variables are returned, including disconnected residual constraints. Define structural answer equivalence, multiset multiplicity, and the interface/default for deduplication. Do not assume that projecting displayed variables preserves all relevant conditions.
- **O04 — Conditional execution (research).** How are choice identity, variable bindings, constraint occurrence identity, consumption, pending matches, propagation history, and failure represented together?
- **O05 — Sharing and separation (research).** Which operations can execute once for several alternatives, what forces separation, and how can common work be recognized without spending more than it saves? May separated work safely be combined again?
- **O06 — Scheduling and quiescence (research).** What is the schedulable unit? How is enabled work detected under partial choice information? Which fairness guarantees are possible without forcing eager enumeration? What overhead do they introduce?
- **O07 — Cost and workloads (research with owner review).** Which workload families reflect the intended use, and what relative runtime/memory results would justify added complexity? No numerical success threshold, target platform, or implementation language is selected.

- **O08 — Language and execution opportunities (research).** Which changes to the current language model would enable materially better execution? Discover alternatives systematically, establish which language properties each mechanism actually requires, and assess the benefits and programming costs.

## Research claims and required evidence

- **C01 — Semantic fidelity.** Every reported answer corresponds to the explicitly identified semantics of the baseline or candidate language under the specified choices; failure, multiplicity, and variable correlations are respected. Require a precise semantic argument and executable checks when prototypes exist. Preservation of successful explicit alternatives is a separate obligation, subject to declared scheduling and liveness assumptions; a sound evaluator that silently omits alternatives is insufficient. This does not require exploring different committed-choice CHR schedules. State any conditional or bounded completeness limitation explicitly.
- **C02 — Compact representation.** State what is stored as alternatives increase. Require a space model and measured retained/peak memory; a small example or source-level node count is insufficient.
- **C03 — Shared computation.** Common work after a choice actually executes once where the proposal claims it can. Require operation-level accounting tied to semantics, including matching and bookkeeping. Passing through a compact choice value is not itself evidence of reduced computation.
- **C04 — Net benefit and simplicity.** Savings exceed representation, scheduling, normalization, and output overhead on justified workloads. Require controlled measurements and an account of the core mechanisms, invariants, and failure modes. Less code alone does not establish simplicity.
- **C05 — Trustworthy answers.** Quiescence and deduplication preserve the agreed meaning of answers. Require counterexample-oriented checks and a stated boundary of recognized equivalence.

- **C06 — Language-design tradeoffs.** A proposed semantic change enables a justified execution opportunity at an understood cost. Require an explicit semantic difference, supporting sources or analysis, affected-program examples, alternatives to imposing the change, and evidence limits. Evaluate correctness within each candidate language; report expressiveness differences rather than disguising them as runtime improvements.

## Milestones and decisions

1. **M0: Preparation (this deliverable).** Contract, open questions, search protocol, evidence standards, review gates, and task record exist and have been checked for consistency with the owner's decisions. Completion means the investigation is reviewable, not that the architecture is justified.
2. **M1: Semantic and literature dossier.** Primary-source evidence, a terminology map, independently discovered language/execution proposals, candidate mechanisms, exclusions, and unresolved contradictions support a coverage review. The owner resolves only semantic choices that materially block this stage.
3. **M2: Candidate and evaluation review.** Comparable language/architecture candidate descriptions, semantic tradeoffs, correctness obligations, workload rationale, controls, and decision rules justify which prototypes, if any, should be built. No fixed number of candidates is assumed.
4. **M3: Bounded prototypes and evaluation.** Reproducible implementations and experiments answer the approved questions. Negative or inconclusive results are valid outputs. A prototype must not become the production architecture merely because it exists.
5. **M4: Language and architecture decision.** A reviewed report connects each recommendation to evidence and limitations. It may recommend an architecture, a narrower claim, further research, or rejecting an approach. Production implementation requires its own scope and validation plan.

The task board currently prepares M1 and its review. M2–M4 are conditional milestones, not authorized implementation tasks with invented details.

## Management and completion

The primary agent owns sequencing, task status, evidence links, and phase reviews. Research and review can use independently scoped read-only agents. Assign write ownership explicitly and preserve one active board task unless disjoint work is justified. Task completion requires a receipt stating the result, evidence, limitations, and the next decision it enables.

The board in [state.yaml](state.yaml) is authoritative for task status; this charter is authoritative for project intent. If the two conflict, reconcile them explicitly rather than treating a status field as permission to alter semantics.

For the prepared M1 tranche, completion requires a reviewed dossier that maps every C01–C06 claim to evidence or a named gap, documents search coverage and candidate exclusions, explains discovered language-design opportunities and their consequences to the owner, and identifies the justified next step. A survey confined to implementing S01–S09 cannot satisfy M1. Counting papers, running examples, or completing board cards cannot establish that outcome. An inconclusive dossier may close M1 if it rigorously identifies the unresolved issue and the evidence needed next; it does not justify architecture selection.

The investigation protocol is [notes/investigation-protocol.md](notes/investigation-protocol.md). Preparation review is recorded in [notes/preparation-review.md](notes/preparation-review.md).

## Starting the prepared investigation

These commands start the M1 investigation, not production implementation:

```text
Codex: /goal Follow docs/goals/chr-sharing/goal.md.
Claude Code: /goalbuddy Follow docs/goals/chr-sharing/goal.md.
```

At execution start, read the GoalBuddy execution contract if available, then this charter and the board. M1's review gate controls progression: return a dossier and decision request before activating prototype work. Missing product decisions block only dependent tasks; continue independent authorized research. Do not mark the full language project complete when a research milestone finishes.

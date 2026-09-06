# Comparing semantics and execution proposals

The candidates can now be compared against explicit semantic obligations and the intended programs. Several mechanisms remain plausible, but each needs a different justification for sharing. The recommended next step is to resolve those justifications before choosing prototypes.

This document proposes a reference model and candidate contracts for review. It does not adopt a language change, prove a compiler correct, or establish performance. The evidence behind the mechanisms is in [the second research checkpoint](T006-transfer-findings.md); application requirements are in [intended programs](intended-programs.md).

## Reference model for the current language

The starting point is the theoretical token-aware disjunction semantics in [CW447, §3.1](https://www.cs.kuleuven.be/publicaties/rapporten/cw/CW447.pdf). The construction below is our proposed project adaptation. Its collection of distinct alternatives deliberately retains their identities; it does not import the source's set-based identification of equal states.

### Terms, rules and alternatives

Terms are finite first-order constructor trees with logical variables. Physical sharing of an acyclic term graph is permitted. A rule has a unique name, a nonempty collection of retained and/or consumed head occurrences, a pure guard, and a body. Body goals include user constraints, finite-tree equations, conjunction, explicit disjunction, success and failure. This notation specifies meaning, not surface syntax.

An alternative is `(a, G, S, θ, H, V, I)`: an alternative identity a; a multiset G of pending goals; a multiset S of identified constraint occurrences; finite-tree substitution θ; propagation history H; allocated logical variables V; and all occurrence identities I allocated along that derivation. Output variables are distinguished query roots. Additional built-in domains, if admitted, require their own consistency and guard-entailment interfaces; they are not silently represented as arbitrary host-language calls.

All variables in a rule application are freshened together, preserving sharing across its heads and body and extending V. Variables already present at a choice point retain their within-alternative relationships in both children, but the children's later substitutions evolve independently. A runtime may share their representation only when its interpretation preserves that independence.

Each dynamic execution of an explicit disjunction creates new child alternative identities. Reusing a static source location as the entire choice identity is invalid for recursive calls. Labels remain internal. A branch in which a nested choice never executes must not acquire extra duplicate answers merely because a representation enumerates assignments to that inactive label.

### Transitions

1. **Introduce:** move a user constraint from G into S with an occurrence identity outside I, then extend I. Consumed identities are not recycled in this abstract model. Identical terms may identify distinct occurrences.
2. **Unify:** remove a pending equation and compute a finite-tree most general unifier with θ. Compose the unifier with θ and maintain an acyclic solved substitution on success; fail the alternative if constructors conflict or an occurs check fails. Apply bindings logically throughout that alternative, including existing constraints and pending goals. Runtime traversal and wake-up are implementation choices.
3. **Apply:** choose distinct store occurrences for the rule heads. Match freshly renamed head variables against the store under θ and establish the guard without binding store variables. Matching binds rule-local variables; repeated head variables must respect equality already established in the store. The ordered rule/head occurrence token must be absent. Keep retained heads, consume removed heads, record the token and add the instantiated body goals. For pure propagation the token prevents repeated firing on the same occurrences even if their arguments later become more instantiated. Tokens for applications that consume a head can be retained harmlessly in this abstract model.
4. **Split:** replace a pending explicit disjunction by two child alternatives with the corresponding arm and the remaining goals. Each inherits the parent store, substitution and history as mathematical state. This is a semantic definition, not a requirement to copy storage or execute common work twice.
5. **Fail:** mark an inconsistent alternative failed. Its resources may remain physically shared with live alternatives; failure must not invalidate those alternatives.

Conjunction and successful empty bodies use multiset identity/flattening. No source-order guarantee is introduced. Choosing among competing enabled rule applications commits a particular allowed transition; it does not create a disjunction. Head matching can wait until a store variable is sufficiently instantiated: matching `p(s(X))` against `p(Y)` does not itself bind Y to `s(X)`.

An alternative is quiescent when it is consistent, has no pending goals, and has no enabled application with an unused token. A guard that is not currently entailed prevents that application; it does not itself fail the alternative. Nonground user constraints can remain in a quiescent store. Later instantiation may make a residual constraint reducible, so quiescence is a property of the present state, not every possible extension of its variables.

### Observation and answer equality

For semantic comparison, retain the selected output tuple together with the entire residual store and built-in information, existentially quantifying non-output variables. This is a conservative mathematical observation, not a decision to display the entire store in the UI. The presentation/projection interface remains O03. This model avoids assuming that disconnected residual constraints are irrelevant.

A sufficient initial equivalence check is equality of normalized output terms and residual multisets under a consistent renaming of unbound variables, preserving the positions of selected outputs and every alias relationship. Runtime occurrence IDs and history are not displayed terms. For equality-only built-ins, use the solved substitution rather than the syntactic sequence of equations. Extra built-in domains require a sound equality check of their own.

This criterion is intentionally incomplete: semantically equivalent constraints may not have identical structure. Failure to prove equivalence retains both answers. Two different synthesized program terms are not duplicates merely because they satisfy the same behavior specification. No claim is made that this observation supports resuming an answer as an exact saved execution state; that would require a separate continuation interface.

### Scheduling assumptions

The reference relation permits many committed schedules. A candidate must preserve a coherent permitted execution for each explicit alternative, rather than compare against one arbitrary reference run or explore all CHR schedules as language search.

For search fairness, assume finite work between yields and fair admission/service of active alternatives. A finite mathematical rule step may still require large matching or unification work; bounded latency needs a finer scheduling design. Neither a queue nor a fixed number of workers guarantees progress when a worker can run indefinitely. Fairness of alternatives and fairness among competing rule applications are distinct claims; S06 does not silently prescribe a particular intra-store rule policy.

## What counts as sound shared work

Let a runtime state denote a family of reference alternatives. Representation-only changes must preserve that family. A computational change must correspond to permitted reference steps for affected alternatives and preserve all unaffected alternatives. Internal administrative steps may correspond to no reference step; a larger operation may correspond to several.

For a shared rule application, agreement on the rule text or visible arguments is insufficient. For every participating alternative, the selected head occurrences must be live and distinct, their matching/guard conditions must hold, the history must permit firing, and the conditional update must preserve consumption, fresh variables, equations and failure. The affected alternatives can have different term values where the operation treats those values parametrically.

These obligations permit sharing without insisting that whole stores be identical. They also avoid assuming that syntactically identical subterms have the same relevant environment. A future theorem should state the exact runtime denotation and prove the chosen update rule against this criterion.

Safety does not establish liveness. Under the declared committed scheduling policy and finite-work assumptions, administrative rewriting must not prevent progress of the reference execution represented for an alternative. A stronger completeness claim across permitted CHR schedules must be stated separately. A shared node must not force unrelated alternatives to wait for a branch-specific computation. Answer extraction and deduplication must also allow progress while other alternatives remain active.

## Candidate contracts and programming costs

The following are combinations worth comparing, not mutually exclusive implementation packages. Each description states the proposed semantics separately from the properties used to optimize it.

### Conditional multiset execution

**Proposed semantics:** the full reference model above. Constraint occurrences, substitutions, tokens and pending work are interpreted under internal choice conditions. Consuming an occurrence under condition A changes its membership there while preserving membership elsewhere. A shared operation computes over the alternatives on which its prerequisites hold.

**Enabling property:** an operation's dependency and effect conditions can be established soundly. This might be discovered dynamically, inferred statically, or simplified through optional locality/mode contracts. A global prohibition on ordinary joins is not inherent in this semantic representation.

**Programming cost:** potentially none at the source level, but broad compatibility can make condition manipulation, matching indexes and history expensive. Arithmetic fits the model directly. SK/lambda synthesis requires conditional aliases and fresh term construction, with residual constraints and divergence handled throughout.

**Evidence needed:** define a compact substitution model that handles conditional occurs checks, and prove conditional consumption/history updates. Measure work saved against condition operations; Boolean representation size alone does not establish efficiency.

### Memoized relation graphs with demand information

**Proposed semantics:** either preserve the reference by tracking all relevant store dependencies, or propose a local relation fragment whose only interactions pass through declared ports. These are different candidates; an expression evaluator is not automatically a faithful CHR evaluator.

**Enabling property:** common computation identity plus sufficient information about demand, environment and ownership. Existing memoized pull-tabbing is evidence for sharing after choice in its source language. Its theorem assumptions cannot be imported as CHR semantics. Optional inferred demand information could delay inspection of choice-dependent terms.

**Programming cost:** compulsory directionality or eager grounding would obstruct backward synthesis; compiler-generated mode specializations could preserve one relational definition. A local-interface restriction affects joins and aliases across calls. Choice-free source code is not necessarily independent of surrounding alternatives.

**Evidence needed:** determine which dependencies `no_c`, `neq` and multiheaded constraints add to a relation call. Establish how lazy term demand coexists with all active constraints needed to determine failure/quiescence. Ignoring an unused term is different from ignoring an active failing constraint.

### Local net compilation: restricted source or richer target

**Proposed semantics:** an eligible fragment translated to ordinary interaction nets, or a broader language translated through a richer local calculus. The first must specify the source restriction; the second must specify how administrative competition implements committed choice while retaining explicit search alternatives.

**Enabling property:** rules/instances have the required topology and discrimination structure, and updates obey ownership invariants. Distinct rule declarations, distinct matching instances, and local active pairs are separate properties. Compiler-proved eligibility, optional annotations and a language-wide discipline must be evaluated independently.

**Programming cost:** addition has one relation head and explicit body choices, so relational use alone does not refute a nonoverlap proposal. It also does not prove net eligibility: unknown terms, duplicating references and body equations need an encoding. SK's duplicated argument makes the cost and correlation of duplication material. Multiheaded constraints may require routing or a source reformulation.

**Evidence needed:** a finite encoding and correspondence argument for at least the chosen relation/constraint fragment, including occurs failure and pending constraints. Inferred cell reuse concerns allocation; conditional lifetime must justify physical reuse separately. Richer ports preserve a route to locality but may add synchronization cost.

### Tabled solver relations

**Proposed semantics:** preserve the reference with sufficient history/environment in table entries, or introduce a logical solver fragment with explicitly stated answer and constraint semantics. Table reuse recognizes repeated subproblems; it is different from retaining an existing common computation node.

**Enabling property:** complete dependency boundaries, fresh answer reinstatement, sound call abstraction and residual projection. Ground keys can connect apparently disconnected constraints. History-free replay is not valid merely because a query looks declarative.

**Programming cost:** a set-like, isolated solver fragment can simplify reuse but excludes resource-sensitive behavior within that fragment. Nonground answers need not be prohibited; summarizing their residual conditions is part of the work. Arithmetic or synthesis may still have infinitely many distinct calls/answers, so tabling alone supplies neither termination nor fairness.

**Evidence needed:** specify whether table entries preserve observations, executable continuations, or logical solution coverage. Evaluate residual answers against that declared contract; do not equate ground-solution coverage with preservation of every represented alternative.

### Reusable learned consequences

**Proposed semantics:** a certified logical constraint fragment whose consequences remain valid across alternatives, integrated with an explicit-search host, or a separate more restrictive candidate language. This is a proposal to investigate, not a demonstrated hybrid.

**Enabling property:** rule-derived clauses remain sound under the fragment's treatment of identity, consumption, equality and fresh variables. SMCHR supplies a precedent under stronger restrictions, not the full proposed language.

**Programming cost:** global range restriction obstructs the displayed addition successor arm and synthesis of fresh structure. Applying the restriction locally could retain a relational generator outside the solver, but introduces an interface whose information flow must be specified. Set semantics changes duplicate resources; Boolean reification need not become user-visible merely because the backend uses it.

**Evidence needed:** an explicit fragment boundary and validity proof for learned clauses across alternatives. Pruning impossible alternatives and avoiding repeated propagation are valuable but do not establish that a common evaluator continuation executes once.

### Storage and recomputation controls

**Proposed semantics:** the same reference contract as the candidate being compared. Persistent storage, trailing, and copying with recomputation supply credible controls and can be components of other candidates.

**Programming cost:** none inherent beyond the selected language contract. Their purpose is to separate storage savings and recomputation tradeoffs from actual shared execution. A control with weaker indexing, different answers, or a deliberately poor copying policy cannot substantiate a speedup claim.

## Open decisions and recommended ordering

### Additional evidence from the coverage pass

Determinism typing in functional logic provides an example of analysis that preserves program admissibility while identifying a cheaper execution case. Its guarantee about choice-headed results does not establish independence from the CHR store. The relevant proposal is to investigate separate certificates for determinism and dependencies, rather than treating one as the other. [Hanus and Prott, PPDP2025, §6](https://www.michaelhanus.de/papers/PPDP25.pdf)

Sized/scheduled types and local potential inequalities can certify resource bounds for interaction nets under a stated cost model. They require annotations and proof obligations and do not supply bounds for arbitrary backward synthesis or a finite-worker search scheduler. This motivates local resource certification as a distinct option from mandatory bounded programs. [Gimenez and Moser, POPL2016, §§4–6](https://tcs-informatik.uibk.ac.at/publications/popl16.pdf)

Two followups give concrete reasons to measure adverse cases. Finer-grained conditional stack sharing can make an operation traverse the full stack; more-general tabled calls can lose pruning and alter termination behavior in a backward query. Therefore, compare condition locality and preserved constraints alongside bytes shared and cache hits. Neither observation predicts the best CHR representation. [Variational stacks, §§3–5](https://eric.walkingshaw.net/files/pubs/2017/vamos17-variational-stacks.pdf), [Tabled CLP framework, §5.1](https://cliplab.org/papers/chico-tclp-flops2012.pdf)

The [coverage record](T009-coverage.md) contains the primary source boundaries and ranked retrieval ledgers. It also distinguishes nonfailure from termination/quiescence and compiler specialization from runtime sharing.

### Decision enabled

[Worked obligations](T008-worked-obligations.md) exercise inherited and conditional histories, consumption, branch-local occurs failure, context dependencies and fresh variables. Each candidate must account for them under its declared semantics, including explicit exclusions and reformulations.

No new taste question blocks this comparison. O02 (query-level operations), O03 (answer presentation/projection) and the exact additional primitive catalogue remain explicitly open. The reference can keep the current body-only equation rule and conservative residual observation while these are investigated; implementing an API would require resolving its concrete choices.

First, prove or refute small semantic constructions for conditional substitutions/lifetimes and relation dependency boundaries. In parallel, derive a precise local-net eligibility condition against the same program requirements. Investigate learned consequences and tabling through a declared solver interface, rather than assuming that their restrictions belong on the entire language.

This ordering is based on blocked claims: without update correctness or complete dependencies, favorable timing would not support C01 or C03; without an eligibility condition, C06 programming costs cannot be assessed. It is not a prediction that one family will be faster. Candidate combinations remain revisable when these analyses expose either an impossibility or a simpler invariant.

Only after those contracts are reviewed should M2 select bounded prototypes, workloads, controls and decision criteria. No executable evaluator or benchmark was produced for this document.

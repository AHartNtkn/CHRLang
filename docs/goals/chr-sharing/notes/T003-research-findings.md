# First research checkpoint: language choices that enable sharing

The evidence supports several concrete language-design opportunities, but no architecture is ready for selection. The strongest common issue is identifying the context on which a computation depends: shared syntax or equal arguments alone do not establish reusable work in CHR.

This initial checkpoint is followed by [the second-pass findings](T006-transfer-findings.md), which resolve the token/disjunction reference gap and add scheduling, allocation and learned-consequence opportunities.

This checkpoint combines four research streams: CHR semantics/compilation, interaction nets, functional-logic sharing, and conditional execution/search storage. It records an initial primary-source and source-code pass, not benchmark results or an exhaustive literature survey. No language change has been adopted.

## What we learned

**Sharing computation after a choice is an established research objective.** Memoized pull-tabbing explicitly addresses it; variability-aware execution also delays separation and rejoins work. These are distinct mechanisms, not synonyms for persistent storage. Their source languages supply assumptions about expression structure or configuration spaces that our CHR variant does not automatically satisfy. [Memoized pull-tabbing](https://www.michaelhanus.de/papers/WFLP20.pdf), [variability-aware execution](https://www.cs.cmu.edu/~ckaestne/pdf/icse14_varex.pdf)

**CHR's surrounding store can change what a shared computation means.** ACD rewriting provides a particularly relevant precedent: its context-sensitive rewrites require special handling of shared occurrences. For this project, dependency on partner constraints, aliases, and history must be part of any reuse criterion. This is why a context/dependency contract is a promising research target. [Demand-driven ACD normalization, §§2–4](https://www.comp.nus.edu.sg/~gregory/papers/iclp09.pdf)

**Language restrictions can enable useful mechanisms, but restrictions need precise definitions.** Rule uniqueness, unique matching instances, known demand order, and local graph connectivity are different properties. A proposal must say which property the compiler needs and whether it can infer it or requires programmers to supply it.

## Opportunities worth investigating

### 1. Make dependency boundaries explicit or inferable

Candidate: identify the constraints, variable bindings, and logical facts that a computation may consult. Compare static analysis, explicit region interfaces, and a language-wide locality rule.

Potential benefit: recognize that different alternatives agree on a computation's relevant context, permitting one execution and controlled invalidation. This is a proposed CHR transfer, not an existing theorem found in the sources.

Programming cost: a hard boundary limits joins across regions and aliasing through external variables. An inferred boundary preserves source flexibility but may be imprecise or expensive. A region's completion must still compose with the entire alternative's quiescence and failure.

Next evidence: formulate a conditional-step simulation that includes consumed occurrence identities and propagation permission. Test the *statement of the criterion* against examples with identical terms but different partner availability; do not benchmark an unproved reuse rule.

### 2. Distinguish reusable facts from consumable resources

Candidate: predicate-local set semantics, persistent facts, or inferred duplicate-insensitivity. These are separate options and must not be conflated.

Potential benefit: reduce multiplicity and history work in saturation-style computations. CHR persistence research supplies a formal alternative, with range-restriction assumptions; compiler analyses also exploit duplicate-insensitivity. [Persistent CHR model](https://arxiv.org/pdf/1007.3829), [HAL compilation](https://arxiv.org/pdf/cs/0408025)

Programming cost: `coin, coin <=> pair` relies on two occurrences. Treating coin as a set changes its meaning. Restricting existential introduction also excludes rules that create fresh unknowns. A mixed system needs an explicit interaction contract; it is not a free optimization.

Next evidence: compare inferred per-predicate eligibility with declared persistence, including fresh variables and consuming rules. Determine whether the savings concern storage, repeated inference, or both.

### 3. Use stable keys, modes, and deterministic specialization

Candidate: infer or declare fixed key positions and computational regions with predictable outputs; specialize their representation and indexes.

Potential benefit: reduce partner-search and generalized choice bookkeeping. CHR functional-dependency analysis and functional-logic deterministic specialization supply concrete mechanisms. [Dependency analysis](https://www.comp.nus.edu.sg/~gregory/papers/chr05.pdf), [monadic functional-logic implementation, §7](https://arxiv.org/html/2604.27863v1)

Programming cost: requiring a key before use limits queries in other directions. Static proofs avoid some annotations but may depend on scheduling assumptions. Ground values can still differ between alternatives; a region with no OR can still inspect choice-dependent state.

Next evidence: define the difference between groundness, functional dependency, and independence from choices. Assess soundness under unordered store growth rather than importing a refined-order analysis unchanged.

### 4. Give matching a checkable demand structure

Candidate: compile eligible rule families into a discrimination structure specifying which data must be examined next, without imposing a global program execution order.

Potential benefit: leave other arguments and computations shared until needed. Definitional trees support needed evaluation in functional logic; nested interaction-net patterns have their own checkable compilation conditions. [Needed narrowing](https://web.cecs.pdx.edu/~antoy/homepage/publications/popl94/paper.pdf), [conditional nested matching](https://arxiv.org/html/2410.00540v1)

Programming cost: arbitrary relational joins may not fit this structure. Requiring particular inputs reduces relational reuse. The source papers' narrowing can generate alternatives, so it cannot simply be imported into explicit-only search; suspension is a different operation that needs its own account.

Next evidence: characterize a CHR fragment or inferred region with this structure, including guards and partner discovery. Determine when the structure can be compiled instead of imposed in the surface language.

### 5. Use ownership and connectivity to make rewriting local

Candidate: explicit or inferred ownership, affine use, and locally identifiable partners. Stronger resource disciplines are additional possibilities, not equivalent to simple linearity.

Potential benefit: simpler update/duplication accounting and direct partner access. Ordinary interaction nets obtain noncompeting local interactions from their topology; rich pattern compilation can retain useful source expressiveness. [Lafont, §§1–2](https://chorasimilarity.wordpress.com/wp-content/uploads/2024/01/ic-lafont-1.pdf), [nested-pattern compilation](https://arxiv.org/pdf/1003.4562)

Programming cost: users may have to express routing structures instead of general multiset joins. Logical aliases must retain identity; copying a value does not replace sharing an unknown. Light-logic disciplines offer stronger guarantees but can constrain recursion/typeability. [Light logics and optimal reduction](https://arxiv.org/pdf/0704.2448)

A diagnostic distinction: one rule `p(X), q(Y) <=> r(X,Y)` has two competing instances in `p(a), q(b), q(c)`. Any nonoverlap proposal needs to specify whether it excludes this case. Even unique instances do not themselves make finding a partner local.

Next evidence: compare connected/owned fragments, compiler-inferred eligibility, and richer net topologies. Quantify the source reformulation and generated routing work before assuming a net encoding is efficient.

### 6. Introduce solver-region interfaces for broader reuse

Candidate: encapsulated regions with explicit inputs, local choices, residual projection, and domain-specific entailment.

Potential benefit: reuse recurring subproblems beyond existing graph identity. TCHR gives direct evidence that tabled CHR integration needs such domain operations. [TCHR, §§6–8](https://arxiv.org/pdf/0712.3830)

Programming cost: domain obligations and boundaries on external aliases. Answer coverage, derivation multiplicity, and residual equivalence are different contracts. This is a language capability to evaluate, not implied by separate rulesets and queries.

Next evidence: define a minimal region contract and compare structural caching with semantic subsumption. Keep finite-tree terms distinct from bounded-term-size termination assumptions.

## Findings that constrain every candidate

**Logical entailment is not a general license to merge executing alternatives.** The CHR-disjunction literature contains a counterexample where logically congruent configurations have distinguishable rule applications. Deduplication must use an equivalence appropriate to its stage and observables. Final-answer deduplication and in-flight state merging need separate criteria. [CHR∨, §5.4](https://arxiv.org/pdf/1009.2900)

**Interleaving depends on the work unit, not the presence of a queue.** Inspected HVM4 commit `6defdfc7dae2a3cca5dd6e74ed0612385b5646a8` calls normalization synchronously inside collapse processing. Source inspection therefore does not establish fairness if that work diverges. This motivates a finite-yield obligation for our scheduler, not a runtime performance conclusion about HVM. [Pinned source](https://github.com/HigherOrderCO/HVM4/blob/6defdfc7dae2a3cca5dd6e74ed0612385b5646a8/src/hvm.c#L6158)

**A credible baseline must include more than naive copying.** Copying with recomputation has a substantial constraint-programming history. It is a control for total cost, even though it does not satisfy the desired shared-execution requirement by itself. [Schulte, §§7–9](https://www.ps.uni-saarland.de/Publications/documents/Schulte_99a.pdf)

## Claim status and coverage checkpoint

C01 (semantic fidelity) and C05 (answers) have relevant formal foundations but unresolved composition: token-aware CHR, explicit choices, candidate semantics, conditional histories, and quiescence. No complete project reference semantics or oracle exists yet.

C02 (space), C03 (shared work), and C04 (net benefit) have mechanism precedents and evaluation requirements. There are no project measurements or demonstrated CHR transfer results. C06 (language tradeoffs) now has the six concrete opportunities above, with programming costs and next evidence identified.

All six initial search areas have received at least screening, and selected primary texts have been inspected. However, batched retrieval did not preserve a complete ranked per-query ledger; individual passes repaired attribution for some core queries only. One citation-expansion pass has produced new relevant families, including ACD rewriting and tabled CHR. The protocol's saturation condition is not met, and no claim of exhaustive coverage is made.

The next bounded pass targets questions that block semantic fidelity, answer validity and comparison of language costs: context-dependent reuse, token/disjunction semantics, and candidate eligibility through properties versus mandatory restrictions. Follow the identified primary citation gaps and obtain evidence about scheduler granularity. Return a comparative semantic dossier before designing prototypes. No owner decision on a language restriction is needed yet; the proposals require further investigation.

## Evidence records

- [CHR semantics and compilation](T003-chr-semantics.md)
- [Interaction nets and pinned HVM4 inspection](T003-interaction-nets.md)
- [Functional logic, demand, and tabling](T003-functional-logic.md)
- [Conditional execution and state maintenance](T003-conditional-execution.md)

These records distinguish inspected proofs, source inspection, abstracts, proposed transfers, and uninspected leads. The checkpoint narrows the next research questions without selecting an architecture.

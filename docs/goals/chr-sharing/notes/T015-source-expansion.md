# Further sources for shared relational execution

The expanded search adds derivation nets and a relational machine calculus to the comparison. Neither supplies a ready CHR implementation with the project's answer contract. Both expose useful distinctions that the existing candidate descriptions missed.

## Angelic CHR: a derivation graph with a different search contract

Martinez, *Angelic CHR* (CHR 2011), §§2.1–2.3 and 3.1, describes growing derivation hypergraphs, resource markings, and sharing across executions. It explores committed scheduling alternatives; its propagation treatment uses linear/persistent constraints. Section 2.2 distinguishes unrestricted sharing of equal constraints from a construction restricted to trivial cycles. Section 3.1 explicitly limits head-decomposition equivalence to its angelic setting and observables. [Author paper](http://contraintes.inria.fr/~tmartine/papers/martinez11chr.pdf)

**Project analysis.** Explore a derivation graph in which only source OR supplies alternative markings, and application conflicts are resolved by the selected committed policy. An edge must carry occurrence identities and the token/binding assumptions that justify it. This retains an event representation as a candidate without adopting implicit search. The original reachability/completeness theorem does not prove this adaptation.

For example, splitting `a,b <=> c` into `a <=> hold` and `hold,b <=> c` can consume a when b never arrives. With `a <=> d` also present, the transformation can lose d. Even without the competing rule, residual `hold` differs from residual a. These are two separate obstacles: committed resource choice and the project's observation of residual constraints. A compiler must use reservations with rollback/revalidation or prove availability; renaming an intermediate constraint cannot justify the transformation.

Do not use the paper's general Petri reachability complexity statement as a current bound. No claim here depends on it. The relevant design conclusion is that identifying equal constraint vertices introduces a reachability obligation; keeping occurrence provenance can avoid that particular quotient at a storage cost.

## Relational Machine Calculus: finite unification with two observation levels

Barrett, Castle and Heijltjes, *The Relational Machine Calculus* (LICS 2024), §2.2, specifies machine runs with fresh variables, substitution and occurs checks; successful runs form a multiset. Definition 4.3 compares underlying sets, and Theorem 4.5 validates equations against that equivalence. Section 4.2 omits idempotence from its reduction system. Section 8.3 embeds interaction nets into RMC; §8.4 embeds Petri nets. [Paper, arXiv v1](https://arxiv.org/pdf/2405.10801v1)

**Project analysis.** RMC is a possible relational intermediate language for explicit body operations. Its net embedding runs in the opposite direction from the compiler this project needs; it is not a CHR-to-interaction-net theorem. Its set-level equational theorem cannot alone justify transformations of our pre-dedup alternative stream. Even if final observations coincide, branch accounting, finite-prefix productivity, and execution multiplicity need separate arguments.

A translation must encode matching separately from unification: a CHR head demanding `s(X)` must suspend on an unknown store argument. An RMC pop that unifies would instantiate it. A relation body containing an explicitly authorized equation can use that capability; rule selection cannot silently acquire it. Residual quiescence is also different from a machine stopped because no transition applies. These distinctions are actionable compiler obligations, not an empirical dependency.

## Direct variable support in memoized pull-tabbing

Hanus and Teegen, *Memoized Pull-Tabbing for Functional Logic Programming* (WFLP 2020), §§4–5, uses ownership, ancestor task results, and task-specific free-variable bindings. Its implementation account therefore gives a more direct starting point for logical-variable storage than an encoding of unknowns as value generators. [Author paper](https://www.michaelhanus.de/papers/WFLP20.pdf)

**Project analysis.** This removes a reason to assume ground-only use. It does not remove CHR dependency tracking: an update learned in one descendant cannot become an unconditional ancestor fact, and absence of expression demand cannot discharge a pending failing constraint. The next construction must state which graph updates are inherited, which are task-specific, and which constraints remain independently scheduled.

## Further net leads

Matsuoka's 1999 IPSJ record describes additive interaction nets carrying first-order terms with unification variables, and explicitly states that this extension lacks Church–Rosser in general. The inspected record is a one-page publication/abstract, not a detailed finite-tree service proof. [Publisher record](https://ipsj.ixsq.nii.ac.jp/records/17012)

Banach's *The Algebraic Theory of Interaction Nets*, UMCS-95-7-2, has an author-uploaded full text. It studies graph-class invariants and context-sensitive conditions for their preservation. Its relevance is validating a chosen graph discipline, not establishing CHR equality or choice services from the word “safety.” Detailed inspection remains actionable. [Author upload](https://www.researchgate.net/publication/2291786_The_Algebraic_Theory_of_Interaction_Nets)

## Retrieval record and limits

The searches on 2026-09-06 were targeted expansion, not an exhaustive bibliographic census. Queries included `CHR interaction nets compilation unification constraint handling rules interaction nets`, `memoized pull tabbing free variables constraints graph Curry unification`, `interaction nets logical variables unification occurs check implementation`, `"Angelic CHR" derivation nets`, `"interaction nets" "unification" Banach`, `"interaction nets" "static" "safety" Banach`, `Thierry Martinez "Angelic CHR" hal`, and `"Interaction Nets with" "Unification"`. Relevant primary hits and inspected sections are above; forum, aggregator, and unrelated results were not used as evidence.

The workshop proceedings failed web retrieval; author search supplied Martinez's paper. HTTPS had a certificate mismatch; its public HTTP endpoint returned the PDF, inspected with `pdftotext` in `/tmp/chr-angelic.txt`. RMC's HTML and PDF were available through the web tool. MPT's relevant implementation sections were inspected directly. No source or prototype was executed. The new leads prevent a coverage-closure claim at this stage.

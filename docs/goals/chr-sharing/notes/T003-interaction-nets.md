# Interaction-net mechanisms and language properties

Interaction-net locality relies on a structural discipline, not merely a unique rule declaration. Several source/compiler disciplines could make parts of a CHR-like language eligible; none of the inspected results establishes that translation.

Read-only research on 2026-09-06. No evaluator execution or prototypes. The source checkout is temporary and not part of the implementation.

## Primary records

- **N01 — Lafont, Interaction Combinators (1997).** [Primary paper mirror](https://chorasimilarity.wordpress.com/wp-content/uploads/2024/01/ic-lafont-1.pdf), §§1.1–1.2 Proposition 1; §2.1 Theorem 1. Fixed-arity agents, one principal port, active pairs and interface-preserving replacement give disjoint active occurrences and a one-step diamond. Universality concerns interaction systems, not an efficient arbitrary-language compiler.
- **N02 — HVM4 primer at pinned revision below.** [Primer](https://github.com/HigherOrderCO/HVM4/blob/6defdfc7dae2a3cca5dd6e74ed0612385b5646a8/docs/primer.md), variable/duplication sections. At-most-once use, explicit duplication and cloning sugar separate source convenience from resource-explicit representation. CHR aliases must not be translated as independent copies.
- **N03 — Sato, Conditional Nested Pattern Matching in Interaction Nets (2024 v1).** [Paper](https://arxiv.org/html/2410.00540v1), §2.2.1, Definitions 3.4/3.8, §§4/6. Disjoint conditions, local sequentiality, and pairwise-distinct nested patterns support conservative compilation. These conditions concern connected net patterns, not arbitrary multiset CHR heads. Rich matching need not be banned globally to investigate this route.
- **N04 — Hassan/Jiresch/Sato, An Implementation of Nested Pattern Matching in Interaction Nets (2010).** [Paper](https://arxiv.org/pdf/1003.4562), §§5–6, Proposition 5.3. Checks subnet/sequential-set conditions and translates patterns through auxiliary agents. Generated interactions and checking cost remain evaluation obligations for a CHR transfer.
- **N05 — Baillot/Coppola/Dal Lago, Light Logics and Optimal Reduction: Completeness and Complexity (2007).** [Paper](https://arxiv.org/pdf/0704.2448), §1, Theorem 2, §§7–8. EAL/LAL typeability permits abstract Lamping reduction without its oracle under stated complexity disciplines. Stronger than ordinary linearity; typeability and recursion restrictions carry costs. No CHR transfer theorem.
- **N06 — Mazza, Interaction Nets: Semantics and Concurrent Extensions (2006).** [Author thesis](https://www.lipn.fr/~mazza/papers/Thesis.pdf), §§4.1–4.2. Multiple principal ports allow competing active pairs and change strong-confluence properties. Richer topology is a candidate with its own semantics; competition is not automatically explicit exhaustive search.
- **N07 — Mackie/Sato, Parallel Evaluation of Interaction Nets: Case Studies and Experiments (GCM 2015; publisher 2016).** [Paper](https://eceasst.org/index.php/eceasst/article/download/2205/2376/2387), §§4–5. Parallelism and overhead depend on encoding. Local confluence is not a guarantee of useful parallel speed or shared alternatives.
- **N08 — Lamping, An Algorithm for Optimal Lambda Calculus Reduction (1990).** [Publisher](https://doi.org/10.1145/96709.96711), abstract/metadata only. Full proof not retrieved. Lévy-optimality is a foundational lead, not evidence of general wall-time optimality in HVM.

## Pinned source findings

HVM4 remote HEAD and inspected checkout: **6defdfc7dae2a3cca5dd6e74ed0612385b5646a8**. Local cache: `/tmp/chr-hvm4-research-20260906`. GitHub main-page indexing was stale; pinned checkout controls the findings. No build or benchmark was run.

**N09 — [Theory document](https://github.com/HigherOrderCO/HVM4/blob/6defdfc7dae2a3cca5dd6e74ed0612385b5646a8/docs/theory/interaction_calculus.md)**, Dup/Sup Labels and Four Core Interactions. Equal-label annihilation and different-label commutation supply correlation/combination behavior. Worked examples are not a general cost proof. Dynamic fresh choices and repeated uses need distinct identity obligations in a CHR encoding. Label manipulation remains internal under S03.

**N10 — [src/hvm.c](https://github.com/HigherOrderCO/HVM4/blob/6defdfc7dae2a3cca5dd6e74ed0612385b5646a8/src/hvm.c#L5776)**. `wnf_dup_sup` at 3923 compares labels; `cnf_at` at 5776 invokes WNF and recursively normalizes fields; `eval_collapse_order_key` at 6058 computes queue priority; `eval_collapse_process` at 6158 calls CNF synchronously and handles SUP/ERA/output. Inference from control flow: a queue alone supplies no finite-yield guarantee when WNF/field evaluation diverges. Term erasure also needs a separate encoding argument for failure of a disconnected CHR constraint. This is inspection, not an executed counterexample.

## Language opportunities and costs

Investigate exclusive ownership/connectivity for local partner discovery, checked discrimination structure for richer patterns, explicit or inferred duplication discipline, and stronger resource stratification. Local compiler eligibility and global source restrictions are distinct choices. Richer multiport nets are also worth comparing, with explicit conflict semantics.

An original diagnostic case distinguishes rule declaration uniqueness from occurrence competition:

```text
p(X), q(Y) <=> r(X,Y).
```

In `p(a), q(b), q(c)`, two instances compete for p(a). Any proposed nonoverlap condition needs a precise definition covering the intended kind of overlap. Even noncompetition does not itself establish local partner discovery or cheap guards. A global connectedness restriction could require programmers to replace relational joins with routing/index structures. This cost must be compared with inferred eligible regions.

## Retrieval audit

Initial queries (web search, 2026-09-06): `Lafont Interaction Nets 1990 principal ports one rule active pair pdf`; `HigherOrderCO HVM4 superposition collapse labels`; `HVM named superpositions duplication labels collapse interaction calculus`; `Conditional Nested Pattern Matching Interaction Net Sato 2024 arxiv`; `Lafont interaction combinators 1997 universality pdf`; `HVM4 collapse fairness ERA superposition breadth first`; `Lamping 1990 algorithm optimal lambda calculus reduction paper pdf`; `Mackie Sato Parallel Evaluation Interaction Nets Case Studies Experiments 2015`. Merged batches are exploratory, not attributable first-20 lists.

Individual query `interaction nets language compilation restrictions local sequentiality linearity`, domain arxiv.org: eight results, all screened. [Implementation Model](https://arxiv.org/abs/1505.07164) (follow-up); [Russian tutorial](https://arxiv.org/abs/1304.1309) (secondary); N03 (selected); [Explicit Framework](https://arxiv.org/abs/1010.1066) (follow-up); topological-order wavefunction, compilation quotient, Abelian quantum circuits, ParaLaw Nets (unrelated or no local-rewrite mechanism).

Individual query `repo HigherOrderCO HVM4 superposition collapse labels`, domain github.com: thirteen results, all screened. HVM4 repo and README (selected); pulls and AGENTS (navigation only); organization (provenance); JIT issue34 (proposal, deferred); Bend readback docs (different version); Lulzx/hvm (third-party comparison, deferred); FFI issue32 (deferred); releases (version-status locator); count_uses PR58 (robustness lead); commit-history gist (historical); Microsoft Android accessibility report (unrelated).

Backward pass: N03 references to Lafont, Hassan/Sato, Lamping, Mackie/Sato; related 2010 nested-matching paper substituted for inaccessible 2007 paper without claiming the latter was inspected. HVM theory led to N01. Light-logics oracle/complexity references, Sato thesis/macros, full Lamping proof, and further formal frameworks remain expansion leads. No saturation claim.

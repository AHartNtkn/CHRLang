# Functional-logic sharing evidence

Memoized pull-tabbing directly addresses repeated computation across alternatives. Its relevance is strong, but expression-graph results do not establish sound conditional consumption in CHR.

Read-only subagent research, 2026-09-06; no experiments or repository implementation inspection. The following are condensed source records. Proposed transfers are ours, not CHR theorems from these papers.

## Inspected primary sources

- **FL01. Hanus and Teegen, Memoized Pull-Tabbing for Functional Logic Programming (WFLP 2020; proceedings 2021).** [Author PDF](https://www.michaelhanus.de/papers/WFLP20.pdf), §§3–7. Task-specific results, choice identities, ownership, and ancestor lookup address repeated pull-tab work and share some deterministic computations across alternatives. Includes a Julia prototype/evaluation. CHR occurrence lifetimes and history are outside its result. C01–C04/C06.
- **FL02. Antoy, On the Correctness of Pull-Tabbing (2011).** [Paper](https://arxiv.org/pdf/1108.0190), §4.2 and §§5–6. Correctness concerns limited-overlapping inductively sequential, left-linear constructor systems with explicit choice overlap. Choice identifiers preserve correlation. No automatic transfer to arbitrary multiset rewrites. C01–C03/C06.
- **FL03. Antoy, Echahed, Hanus, A Needed Narrowing Strategy (POPL 1994).** [Author PDF](https://web.cecs.pdx.edu/~antoy/homepage/publications/popl94/paper.pdf), §§3–5, Theorems 2–4. Definitional trees identify needed computations in inductively sequential constructor systems. Optimality has a specified reduction model, not a machine-runtime guarantee. Narrowing may introduce alternatives through instantiation: importing that operation would change explicit-only search. C01/C03/C06.
- **FL04. Hanus, Improving Lazy Non-Deterministic Computations by Demand Analysis (ICLP 2012).** [Publisher/PDF](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ICLP.2012.130), §§1–4 and evaluation. Demand analysis supports selective earlier evaluation to avoid duplicated suspended work. Includes KiCS2 measurements. Analysis precision is a limitation, and CHR partner availability complicates argument demand. C03/C04/C06.
- **FL05. Antoy, Brown, Chiang, Lazy Context Cloning for Non-Deterministic Graph Rewriting (TERMGRAPH 2006 / ENTCS 2007).** [Author PDF](https://web.cecs.pdx.edu/~antoy/homepage/publications/entcs-176/paper.pdf), §§2,4–6. Bubbling clones a context to a dominating node and retains other graph structure; unrestricted distribution is unsafe. CHR dependencies need not form a rooted expression graph. C01–C03.
- **FL06. Antoy and Libby, Making Bubbling Practical (2018).** [Paper](https://arxiv.org/pdf/1808.07990), §§3.3–5,7. Maintains dominator information through graph updates. Correctness discussion is informal proof sketches; concluding overhead estimates are not reproduced measurements. Metadata and excess cloning need costing. C02–C04/C06.
- **FL07. Fischer, Kiselyov, Shan, Purely Functional Lazy Nondeterministic Programming (JFP 2011).** [Journal PDF locator](https://okmij.org/ftp/Haskell/FLP/lazy-nondet.pdf); [shorter version actually inspected](https://homes.luddy.indiana.edu/ccshan/rational/lazy-nondet.pdf), §§2.4,3,5. Explicit sharing supports call-time choice and pluggable search. Sharing within alternatives is not automatically sharing across alternatives. Full journal-version comparison pending. C01/C03/C04.
- **FL08. Antoy and Hanus, Set Functions for Functional Logic Programming (PPDP 2009).** [Author PDF](https://www.michaelhanus.de/papers/PPDP09.pdf), §§2–3, Theorems 1–2. Separates choices of an operation from choices of its arguments, with encapsulation/fingerprint treatment. This is not residual-store deduplication. C01/C05/C06.
- **FL09. Hanus, Prott, Teegen, A Monadic Implementation of Functional Logic Programs (2026 expanded manuscript).** [Version inspected](https://arxiv.org/html/2604.27863v1), §§4–8; [PPDP22 predecessor](https://www.michaelhanus.de/papers/PPDP22.pdf), discovered but not compared. Memoized sharing, branch identity, and static/dynamic deterministic specialization are relevant. The evidence includes mixed benchmark outcomes. Do not attribute the mechanism's origin to 2026 or infer CHR results. C01–C04/C06.

## Opportunities and costs

**Demand-directed regions:** infer or declare which arguments must be inspected. This may let opaque choices pass through other positions. Relational reuse in different modes and multiheaded joins make a global constructor discipline costly. Needed narrowing must not silently generate search where the user wrote no disjunction.

**Ownership regions:** identify unique consumers and safe updates locally. A global linear discipline would affect propagation and repeated observation; compiler-established ownership may avoid that source burden. FL01 motivates the idea, not its validity for CHR.

**Deterministic specialization:** a region can use simpler representations if its relevant behavior is choice-independent. Merely containing no OR is insufficient: bindings and partner presence can still depend on choices. State the stability conditions as the store grows.

**Explicit subproblem boundaries:** input/output and local-choice boundaries could enable reuse beyond pre-existing graph identity. Costs include restrictions on external aliases and a new account of residual conditions. Encapsulation is a language design proposal, not implied by separate queries.

**Functionalization of selected relations:** retrieved [From Logic to Functional Logic Programs](https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/article/from-logic-to-functional-logic-programs/AB29BDA26F960E605C4B6C74384ADD21) is a primary expansion lead; only abstract/result discussion inspected. Investigate whether analysis can create demand structure while preserving relational uses. No full transformation proof has been checked.

Under S07, output demand alone cannot justify ignoring an active component that could fail or prevent quiescence. Any lazy region needs a composition argument with whole-alternative success.

## Retrieval record

Engine: web search, 2026-09-06. Initial batched queries:

1. `Antoy Hanus memoized pull tabbing sharing nondeterministic computations functional logic`
2. `Antoy Echahed Hanus needed narrowing strategy inductively sequential programs`
3. `functional logic encapsulated search set functions call time choice tabling`

Selected FL01–FL04/FL09. Additional primary leads screened: [specialization based on needed narrowing](https://arxiv.org/abs/cs/0403011), [Escobar thesis](https://personales.upv.es/sanesro/PhD/escobar-thesis.pdf), [Hanus/Prehofer JFP99](https://www.michaelhanus.de/papers/JFP99.pdf), and functionalization above; deferred beyond initial scope. ResearchGate mirrors, DBLP profiles, Curry unsafe-search docs, EmergentMind, and encyclopedias were discovery/duplicate results, not supporting evidence.

Backward-citation pass through FL01 and FL02 references led to demand analysis, explicit sharing, bubbling, and set functions. Retrieval queries:

4. `Antoy bubbling nondeterministic graph rewriting 2005 2006 paper`
5. `Antoy Hanus set functions functional logic programming 2009 pdf`
6. `Fischer Kiselyov Shan purely functional lazy nondeterministic programming 2011 sharing pdf`
7. `site.michaelhanus.de papers tabled functional logic programming memoization`
8. `site.okmij.org lazy nondeterministic sharing Fischer Shan`

Selected FL05–FL08 and journal/predecessor locators. [Synthesizing Set Functions](https://arxiv.org/abs/1808.07401), [Functional Logic Program Transformations](https://arxiv.org/abs/2601.13224), and [Antoy WFLP14](https://web.cecs.pdx.edu/~antoy/homepage/publications/wflp14/post.pdf) are deferred leads. Mirrors, indexes, overviews, and discussions were not technical evidence.

Batched retrieval returns merged result sets and does not establish first-20-per-query coverage. An individual-query supplement is being collected. One citation pass is complete; search saturation is not established. No CHR transfer theorem, benchmark replication, or current implementation revision has been established in this scope.

## Individual-query and tabling supplement

Individual query `memoized pull tabbing shared nondeterminism`, 2026-09-06, returned 23 entries; first 20 screened. In displayed order: FL01 arXiv; FL01 author PDF; FL01 preprint (duplicate); [Fair Curry in Go, PPDP21](https://www-ps.informatik.uni-kiel.de/~mh/papers/PPDP21.pdf) (follow-up); FL09 PPDP22 (version lead); ResearchGate MPT (duplicate); ResearchGate monadic paper (duplicate); Teegen MPT PDF (author duplicate); Hanus ResearchGate profile (discovery); EmergentMind monadic summary (secondary); another ResearchGate monadic record (duplicate); FL02 arXiv (selected); ResearchGate Curry-in-Go (duplicate); another ResearchGate MPT record (duplicate); Wikipedia memoization (general); Moonlight monadic review (secondary); WebAssembly nondeterminism (different problem); lazily Rust crate (reactive memoization, no established choice mechanism); Chromium diff (false positive); React memo (false positive). This repairs attribution for the core family, not every earlier batch.

Individual query `Chen Warren 1996 Tabled evaluation delaying general logic programs pdf` located **FL10 — Chen/Warren, Tabled Evaluation with Delaying for General Logic Programs (JACM 1996)**, [publisher](https://doi.org/10.1145/227595.227597). Primary abstract inspected; full PDF access attempts failed. Its stated bounded-term-size termination condition is stronger than finite-tree terms. Full theorem checking remains open; do not infer unrestricted termination.

**FL11 — Schrijvers/Demoen/Warren, TCHR: a framework for tabled CLP (2007 v1).** [Paper](https://arxiv.org/pdf/0712.3830), §§2.3,4,6–8. Full primary paper inspected. Call abstraction, implication checking, and answer projection are domain operations, not automatically provided by CHR. Answer compaction preserves ground-answer coverage under its criterion, not derivation multiplicity. Includes CHR/XSB integration. This supports investigating explicit solver regions with a logical-domain interface, not replacing the entire evaluator with generic memoization. C01/C05/C06.

Proposed transfer: a region with declared inputs, residual projection, and entailment could permit broader reusable results. Cost includes domain-specific obligations and potential reformulation. Structural caching has narrower reuse but fewer semantic commitments. Finite-tree syntax alone supplies neither bounded tables nor decidable semantic answer equivalence.

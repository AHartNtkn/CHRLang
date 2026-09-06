# CHR semantics and compilation evidence

The inspected sources provide components for a semantic reference and concrete compilation opportunities. Their operational models differ; combining their results requires an explicit argument.

Read-only subagent research on 2026-09-06, web search and primary PDFs; no experiments. C01–C06 refer to the charter claims.

## Primary records

- **CHR01 — Betz/Frühwirth, Linear-Logic Based Analysis of CHR with Disjunction (2010 v1).** [PDF](https://arxiv.org/pdf/1009.2900), §§2,5, especially Example 5.22 and Lemma 5.28. Uses history-free equivalence semantics, not occurrence-token execution. Logical congruence need not preserve executable alternatives: cu(X) versus cu(0) ∨ cu(X) can be distinguished by a rule for cu(0). Analyticness/compactness and incompatible arm built-ins provide restricted equivalence results. Relevant C01/C05/C06.
- **CHR02 — Duck et al., Refined Operational Semantics of CHR (ICLP 2004).** [Author PDF](https://www.comp.nus.edu.sg/~gregory/papers/iclp2004a.pdf), §§2–3 and correctness discussion. Specifies refinement of theoretical execution. Some partner/wakeup choices remain; fairness is not guaranteed. Source-order-sensitive examples cannot supply the project's unordered contract. C01/C04/C06.
- **CHR03 — Duck/Stuckey/Sulzmann, Observable Confluence for CHR (2006).** [Author PDF](https://www.comp.nus.edu.sg/~gregory/papers/chr06.pdf), §2 Definitions 1–12, §§3–4. Token-aware states include occurrence identity and propagation history. Reachable-state confluence can differ from unrestricted confluence because some histories are unreachable. Results associated with functional dependencies include termination assumptions. C01/C05/C06.
- **CHR04 — Holzbaur et al., Optimizing Compilation of CHR in HAL (2004 preprint).** [PDF](https://arxiv.org/pdf/cs/0408025), §§2–6. Types, modes, dependencies, symmetry, and duplicate-insensitivity guide indexes and joins. Some analyses rely on refined order. Historical benchmarks are evidence for those implementations, not expected project speedups. C02/C04/C06.
- **CHR05 — Duck/Schrijvers, Accurate Functional Dependency Analysis for CHR (2005).** [Author PDF](https://www.comp.nus.edu.sg/~gregory/papers/chr05.pdf), §§1–3,5–7. Abstracts refined call-based execution; tracks lookup cardinality and supports indexing/join/storage optimizations. Aliasing can increase nonground lookup cardinality; groundness/modes improve precision. No transfer proof to unordered conditional stores. C02/C04/C06.
- **CHR06 — Lam/Sulzmann, Concurrent Goal-Based Execution of CHR (2010 v2).** [PDF](https://arxiv.org/pdf/1006.3039), §§2–4.3. Supplies matching-based concurrency and correspondence results; nonconflicting effects may share retained heads. Final-state correspondence has termination assumptions. The displayed abstract treatment does not supply complete propagation-token handling. Concurrency is not itself sharing across disjunctions. C01/C03/C04/C06.
- **CHR07 — Betz/Raiser/Frühwirth, A Complete and Terminating Execution Model for CHR (2010).** [PDF](https://arxiv.org/pdf/1007.3829), §§2–4, Definition 3.4. Separates linear and idempotent persistent constraints; correctness relates to history-free semantics. Range restriction is required for the stated correspondence. The title does not guarantee termination of every program. C01/C02/C04/C06.
- **CHR08 — De Koninck/Schrijvers/Demoen, CHR with Rule Priorities (2007).** [Institutional PDF](https://www.cs.kuleuven.be/publicaties/rapporten/cw/CW479.pdf), §§3–5. Highest applicable priority restricts theoretical execution. History/duplicate treatment affects termination. Aging and starvation need separate treatment. Global priorities introduce coordination and can affect correctness. C01/C04/C06.

## Proposed transfers

**Stable keys and modes:** fixed key positions may make indexing and conditional lookup cheaper. An unknown key must delay, be rejected, or use a general relation under a clearly chosen contract. Groundness does not imply choice independence. Inferred properties avoid a global programming restriction but require soundness under every permitted schedule.

**Set versus persistent versus resource semantics:** duplicate-insensitive predicates and persistent facts could shrink state/history. They are distinct from ordinary multiset occurrences. A global set interpretation changes programs such as `coin, coin <=> pair`; range restriction excludes rules introducing fresh existential variables. Local classifications need a proof for interacting rules.

**Reachability-aware confluence and dependency regions:** certify safe scheduling/common execution where reachable states have the needed properties. Confluence certificates do not supply termination, and concurrent-step results do not supply conditional-step sharing. Boundaries must include aliases, partners, history, and effects.

**Disjoint alternatives:** incompatible arm conditions may simplify equivalence reasoning, but overlapping alternatives are currently permitted. Requiring disjointness restricts programs and possibly outputs. A hidden tag cannot simply be assumed to preserve the property. Establish benefit before recommending the restriction.

**Phases/priorities:** normalization-before-expansion may simplify some algorithms, while constraining shared scheduling. Compare explicit semantics against scheduling chosen within proven schedule-insensitive regions.

Own diagnostic case for a source-order-dependent set analysis:

```text
p(X) \ p(X) <=> true.
p(X), p(X) <=> two(X).
```

From two p(a) occurrences, source placement does not prevent two(a) under the unordered baseline. This is a semantic counterexample, not an experiment or performance result.

## Reference candidate and unresolved composition

A token-aware theoretical CHR model plus explicit-disjunction configurations is the strongest reference candidate found. CHR03 and CHR01 supply different parts. Their composition, including conditional histories, is proposed work, not an inspected theorem.

Before an executable oracle: specify guard/built-in entailment, token ordering for symmetric heads, fresh-variable scope, pending body operations, quiescence, and answer projection. Keep rule scheduling and explicit-alternative scheduling distinct. These are formalization tasks; no new product choice should be inferred merely to make a reference implementation easy.

## Retrieval audit

Queries (web search, 2026-09-06):

1. `CHR disjunction operational semantics Betz Frühwirth linear logic`
2. `Constraint Handling Rules compilation functional dependencies set semantics Duck Schrijvers`
3. `parallel constraint handling rules concurrency monotonicity confluence propagation history`
4. `"parallel constraint handling rules" concurrency monotonicity confluence propagation history`
5. `"Constraint Handling Rules" "functional dependencies" "Duck" "Schrijvers"`
6. `Concurrent goal-based execution Constraint Handling Rules Lam Sulzmann 2011`
7. `CHR rule priorities De Koninck Schrijvers Demoen 2007 semantics`
8. `"CHR" "flexible query language" Abdennadher Schutz pdf`
9. `"Operational semantics and confluence of constraint propagation rules" pdf`

Some queries were batched; their results are merged and sometimes truncated. Query 1 was repeated individually. Its first 20 screened results were: CHR01 arXiv (selected); Betz/Frühwirth 2005 PDF (earlier foundation); 2007 disjunction PDF (earlier); CiteSeer CHR01 (duplicate); author 2012 CHR01 (alternate version); CHR07 Cambridge (selected); SciSpace CHR01 (aggregator); CHR07 arXiv (selected); FreeCHR refined semantics 2025 (deferred); ResearchGate CHR01 (duplicate); ResearchGate earlier linear logic (duplicate); JetBrains Coderules docs (indirect); FreeCHR instance 2025 (deferred); CHR compilation thesis/book page (discovery); Leuven linear-logic bibliography (discovery); Leuven 2007 bibliography (discovery); Betz bibliography (discovery); Leuven CHR documents page (discovery); ResearchGate Betz profile (aggregator); Scribd CHR survey (mirror).

Primary deferred locators: [FreeCHR semantics](https://arxiv.org/abs/2504.04962), [FreeCHR instance](https://arxiv.org/abs/2505.22155), [Logical Algorithms meets CHR](https://arxiv.org/abs/0901.1230), [proof-theoretical semantics](https://drops.dagstuhl.de/entities/document/10.4230/OASIcs.ICLP.2018.4). Author bibliographies led to CHR02/05; [TCHR](https://arxiv.org/abs/0712.3830) was handed to the functional-logic scope. ACDTR/Cadmium was handed to the PM and inspected there.

Backward pass: CHR01 references to 1997 propagation and 1998 CHR-disjunction; CHR06 to propagation/persistence; CHR05 to refined semantics and abstract interpretation. Original 1997 and full 1998 texts remain inspection gaps; later primary treatments do not count as reading those proofs. No systematic forward-citation pass or saturation established. This is a bounded initial dossier, not an exhaustive survey.

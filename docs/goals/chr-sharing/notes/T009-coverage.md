# Two-round literature coverage record

The sampled literature routes produced no additional runtime sharing family, but they sharpened the costs of specialization, conditional storage, and tabled reuse. The survey’s coverage gate remains open. These two rounds do not establish that all foundational sources and forward citations have been covered.

This record documents research on 2026-09-06 through `web.run`. No implementations or experiments were run. It extends [the investigation protocol](investigation-protocol.md) and [the previous source record](T006-source-record.md).

## Scope and decisions before retrieval

Round A followed memoized pull-tabbing, SMCHR, and static interaction-net analysis. Its question was whether forward and author followups exposed an overlooked mechanism or a useful language property. Four individual queries were issued; the resource-analysis title query followed a result from the static-analysis query. Author publication pages supplied further primary texts.

Round B resolved the partial-evaluation lead exposed by Round A, then followed conditional execution/FCC and TCHR. It was limited to three newly inspected texts. The questions were whether specialization constituted an additional family and whether conditional representations or later tabled CLP changed the comparison. Two individual queries and attributable author links supplied the texts. Publisher access to the 2019 TCLP successor failed; the 2012 predecessor supplied the bounded primary inspection.

Each ledger below preserves the displayed order of results, including academic results interleaved by the tool. All results were screened when fewer than twenty appeared; the first twenty were screened otherwise. These are search-tool results, not a citation-index export. Search-engine age estimates were not used to establish publication dates.

## Selected evidence from Round A

### F13. Optional determinism typing

Hanus and Prott, *Determinism Types for Functional Logic Programming*, PPDP2025. [Author PDF](https://www.michaelhanus.de/papers/PPDP25.pdf), §6, Theorems 2–4, PDF p.8; §8, PDF p.11. Found through Hanus’s publication page after Q1.

Typing completeness preserves the set of admissible expressions. Preservation and soundness establish that an expression typed `Det` cannot evaluate to a choice-headed expression. This is evidence for investigating optional or inferred annotations that permit cheaper execution without a global restriction. It is not a theorem about independence from CHR bindings, occurrence consumption, or propagation histories. Evidence: formal statements and accompanying proof material; no CHR transfer established. Relevant: C01/C03/C06.

### F14. Non-failure conditions are a separate certificate

Hanus, *Inferring Non-Failure Conditions for Declarative Programs*, Science of Computer Programming 2026. [Author PDF](https://www.michaelhanus.de/papers/SCICO26.pdf), §§4–5, especially §5.1 and refinement discussion, PDF pp.17–25. Found through the same author page. Targeted definitions and discussion inspected; the full 42-page paper was not audited.

Abstract input/output information and call types give sufficient non-failure conditions. Refinement terminates under finite descending chains in the abstract domain. Locally generated free variables can defeat useful call preconditions, a precision cost relevant to synthesis. This motivates a certificate distinct from determinism: non-failure neither proves termination nor CHR quiescence and cannot authorize early answers. Evidence: formal analysis framework and refinement discussion, with no CHR transfer. Relevant: C01/C05/C06.

### R3. Compositional resource certificates

Gimenez and Moser, *The Complexity of Interaction*, POPL2016. [Institutional PDF](https://tcs-informatik.uibk.ac.at/publications/popl16.pdf), §§2–6; Theorem 1 in §4; Theorem 2/Corollary 3 in §6, PDF p.10; qualifications in §§7,9. Q3 exposed the title and Q4 located the primary text. [Long-version locator](https://arxiv.org/abs/1511.01838) was screened, not fully inspected.

User-supplied sized and scheduled types, together with node potentials satisfying local rule inequalities, certify sequential and parallel time-space bounds. These are sufficient certificates, not a general inference algorithm. Local certificates could inform allocation or scheduling without requiring finite bounds for all relational programs. Their costs include annotation obligations and input/output assumptions that entirely free synthesis queries may not satisfy. The paper’s timed fully parallel reduction is not a finite-worker search scheduler. Its higher-order box example assigns unit space and time to boxes, expressly a simplification; further higher-order reduction results remain future work. No search-sharing performance result transfers. Relevant: C02/C04/C06.

Round A also revisited the known MPT successors and SMCHR’s 2013 extended abstract. The latter confirms the range-restricted, set-based setting already recorded for S4; it does not resolve fresh existential variables or multiplicity for this project. The primary author page did not expose a later SMCHR mechanism; that is a limited observation about the inspected page, not an absence claim.

## Selected evidence from Round B

### R4. Specialization and its termination obligation

Béchet, *Partial Evaluation of Interaction Nets*, WSA1992, pp.331–338. [Author-hosted recompiled draft](https://pagesperso.ls2n.fr/~bechet-d/Documents/LIENS-1992-Bec-WSA92-recompiled-2003-07-22-draft.pdf), §§4–6, especially pp.334–336. Direct followup to Q3’s selected lead.

The paper distinguishes pre-evaluation, abbreviations through definition/unfold/fold, and stronger behavioral-law transformations. Abbreviation is described as preserving complexity class; stronger laws can eliminate repeated computation and change the algorithm. Equivalence observed at selected ports does not generally preserve termination. The text asserts that conditions ensuring total correctness were found but does not define or prove those conditions here. Evidence: extended-abstract mechanism descriptions and examples, not an inspected general correctness proof. Relevant: C01/C03/C04/C06.

This lead belongs within specialization and equivalence-based optimization, rather than supplying a distinct runtime sharing family. It does expand the compiler opportunity beyond allocation reuse: specialize interpreters and intermediate representations, while proving the appropriate operational observations. The missing termination conditions prevent treating the described laws as a ready optimization recipe for CHR.

### V10. More structural sharing can increase operation costs

Meng et al., *A Choice of Variational Stacks: Exploring Variational Data Structures*, VaMoS2017. [Author PDF](https://eric.walkingshaw.net/files/pubs/2017/vamos17-variational-stacks.pdf), §§3.1–3.4, §4.1, §5. Reached through the author publication page found in Q5.

A stack containing conditional entries can share more than a choice between whole stacks, but a conditional pop can require traversing the entire stack. Buffering operations under one context avoids conditional-operation costs; another implementation uses an ordinary stack until variation occurs. Experiments use generated operation sequences and VarexJ, whose stacks are mostly short-lived with limited context variation. This supports comparing context locality and operation costs separately from storage shared. It does not predict the winning representation for long-lived CHR stores or prove conditional consumption correct. Evidence: algorithms and historical experiments. Relevant: C02/C03/C04/C06.

### F15. Generalized reuse can lose useful pruning

Chico de Guzmán et al., *A General Implementation Framework for Tabled CLP*, FLOPS2012, pp.104–119. [Institutional PDF](https://cliplab.org/papers/chico-tclp-flops2012.pdf), §§3.2–3.4, §4, §5.1. Q6 supplied this successor citing TCHR.

Constraint entailment determines reuse between calls and answers, while solver interfaces restore suspended state. More general generators may replace earlier ones, and subsumed answers may be pruned. The backward-Fibonacci comparison reports nontermination under TCHR call abstraction because abstraction loses pruning constraints. This makes constraint-sensitive reuse and backward workloads necessary comparison cases. It remains tabled CLP with model-based answer subsumption, not duplicate-preserving CHR alternative execution. Historical timings compare different solvers and engines; no speed ratio should be attributed solely to entailment. Evidence: implementation framework and experiments. Relevant: C01/C03/C04/C05/C06.

## Exact queries and ordered screening

### Q1: `"memoized pull-tabbing"`

Round A; all fifteen results screened.

1. https://arxiv.org/abs/2008.11999 — covered seed.
2. https://link.springer.com/book/10.1007/978-3-030-75333-7 — proceedings metadata.
3. https://finnteegen.de/ — author locator; search excerpt screened.
4. https://www-ps.informatik.uni-kiel.de/~mh/papers/PPDP21.pdf — covered successor.
5. https://www.researchgate.net/publication/408073745_A_Monadic_Implementation_of_Functional_Logic_Programs — 2026 successor mirror; primary preferred.
6. https://www.researchgate.net/publication/351362890_Memoized_Pull-Tabbing_for_Functional_Logic_Programming — duplicate seed.
7. https://www.michaelhanus.de/papers/ — selected author followup; opened, followed F13/F14.
8. https://michaelhanus.de/papers/PPDP22.pdf — covered successor.
9. https://www.researchgate.net/profile/Michael-Hanus — author metadata duplicate.
10. https://dblp.org/pid/211/7554.html — bibliography metadata.
11. https://www.researchgate.net/publication/404333000_A_Monadic_Implementation_of_Functional_Logic_Programs — duplicate successor.
12. https://dblp.org/pid/211/7554 — duplicate bibliography.
13. https://dblp.org/pid/h/MichaelHanus — bibliography.
14. https://dblp.org/pid/h/MichaelHanus.html — duplicate bibliography.
15. https://vufind2.lib.aegean.gr/EDS/Search?lookfor=%22Hanus%2C+Michael%22&type=AU — catalogue; excluded as evidence.

### Q2: `"SMCHR" "constraint"`

Round A; all seventeen results screened.

1. https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/article/abs/smchr-satisfiability-modulo-constraint-handling-rules/4084A73E5A7516AA4A54F11478E0B320 — original publisher.
2. https://arxiv.org/abs/1210.5307 — original version.
3. https://www.comp.nus.edu.sg/~gregory/papers/smchr.pdf — previously inspected original.
4. https://citeseerx.ist.psu.edu/document?doi=c9dd393a0ee7165b91a0a56083b9b1472c38bf28&repid=rep1&type=pdf — 2013 abstract mirror.
5. https://www.ijcai.org/Abstract/13/444 — primary 2013 abstract.
6. https://www.ijcai.org/Proceedings/13/Papers/444.pdf — primary 2013 text; indexed range/set-semantics passage inspected.
7. https://www.researchgate.net/publication/232416157_SMCHR_Satisfiability_Modulo_Constraint_Handling_Rules — original duplicate.
8. https://www.comp.nus.edu.sg/~gregory/ — selected author followup; opened.
9. https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/most-cited?pageNum=18 — index only.
10. https://www.cambridge.org/core/search?filters%5Bkeywords%5D=lazy+clause+generation — index only.
11. https://en.wikipedia.org/wiki/Satisfiability_modulo_theories — secondary; excluded.
12. https://www.researchgate.net/publication/262221870_Satisfiability_modulo_constraint_handling_rules_extended_abstract — duplicate 2013 abstract.
13. https://dblp.org/pid/74/3123.html — author metadata.
14. https://studylib.net/doc/13903026/satisfiability-modulo-constraint-handling-rules--extended... — engine displayed a truncated locator; duplicate abstract, not used.
15. https://www.researchgate.net/publication/281440568_Defeasible_Logic_Programming_in_Satisfiability_Modulo_CHR — retained uninspected application/citation lead.
16. https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/issue/261228211A479C31FC329DD3F7CCF699 — issue metadata.
17. https://openurl.ebsco.com/results?bquery=DE+%22Disjunction+%28Logic%29%22&page=1&sid=ebsco%3Aocu_results%3Acache — index only.

### Q3: `"interaction nets" "static analysis" resource reuse`

Round A; all sixteen results screened.

1. https://doi.org/10.1007/bfb0032744 — Mackie1997 topology-analysis abstract; full text not accessed.
2. https://www.researchgate.net/publication/221477516_Static_analysis_of_interaction_nets_for_distributed_implementations — duplicate and citation hints.
3. https://dblp.uni-trier.de/rec/conf/sas/Mackie97.html — metadata confirms 1997.
4. https://pagesperso.ls2n.fr/~bechet-d/Documents/LIENS-1992-Bec-WSA92-recompiled-2003-07-22-draft.pdf — selected partial-evaluation lead; inspected as R4 in Round B.
5. https://link.springer.com/book/10.1007/BFb0032729 — proceedings metadata.
6. https://citeseerx.ist.psu.edu/document?doi=214e389aae0c7c881d28db63fe6ca3c4453874be&repid=rep1&type=pdf — duplicate partial-evaluation text; termination caveat screened.
7. https://www.researchgate.net/publication/2291786_The_Algebraic_Theory_of_Interaction_Nets — retained context-sensitive static-safety lead; uninspected.
8. https://ethz.ch/content/dam/ethz/special-interest/infk/inst-pls/plf-dam/documents/StudentProjectProposals/practical-work-interaction-nets.pdf — student proposal; not established results.
9. https://dblp.org/pid/m/IanMackie — metadata.
10. https://dblp.org/pid/m/IanMackie.html — duplicate.
11. https://www.collectionscanada.gc.ca/obj/thesescanada/vol2/BVIV/TC-BVIV-6452.pdf — unrelated anthropology; excluded.
12. https://www.researchgate.net/publication/222918925_Towards_a_Programming_Language_for_Interaction_Nets — syntax/module lead; no resource result established.
13. https://www.researchgate.net/publication/301274047_The_complexity_of_interaction — selected title lead; primary fetched via Q4.
14. https://ouci.dntb.gov.ua/en/works/9GpbkYW7/ — secondary citation index; uninspected.
15. https://www.cs.unibo.it/~martini/CONCERTO/ModelloA.html — proposal; excluded as evidence.
16. https://katalog.bibliothek.kit.edu/bib/1084764 — proceedings catalogue; excluded.

### Q4: `"The complexity of interaction" pdf`

Round A; first twenty of twenty-three results screened. Items 3 and 8 locate R3; all other listed items are unrelated subject matches and were excluded.

1. https://link.springer.com/book/10.1007/978-3-031-30727-0
2. https://www.researchgate.net/publication/220286203_Complex_interaction
3. https://tcs-informatik.uibk.ac.at/publications/popl16.pdf
4. https://www.schweitzer-online.de/ebook/Haddington/Complexity-Interaction/9783031307270/A67492614/
5. https://eprints.illc.uva.nl/id/eprint/2105/
6. https://books.google.com/books/about/Complexity_of_Interaction.html?id=wgvWEAAAQBAJ
7. https://oulurepo.oulu.fi/bitstream/10024/43249/1/nbnfioulu-202311233322.pdf
8. https://arxiv.org/abs/1511.01838
9. https://citeseerx.ist.psu.edu/document?doi=b4eb9a8849a574eb118fd483122dcc7f00bf79f1&repid=rep1&type=pdf
10. https://icar.cnrs.fr/membre/hbaldauf-quilliatre/publications/
11. https://oulurepo.oulu.fi/handle/10024/47050
12. https://www.hugendubel.de/de/ebook_pdf/complexity_of_interaction-46772606-produkt-details.html
13. https://dokumen.pub/complexity-of-interaction-studies-in-multimodal-conversation-analysis-3031307267-9783031307263.html
14. https://www.scribd.com/document/1072145270/Ebook-Complexity-of-Interaction-Studies-in-Multimodal-Conversation-Analysis
15. https://scispace.com/papers/interaction-analysis-and-psychology-a-dialogical-perspective-5bn6plfbzr
16. https://dokumen.pub/complexity-of-interaction-studies-in-multimodal-conversation-analysis-1nbsped-3031307267-9783031307263.html
17. https://arxiv.org/abs/1712.05817
18. https://arxiv.org/abs/cond-mat/0510218
19. https://en.wikipedia.org/wiki/Social_network_analysis
20. https://arxiv.org/abs/2008.05905

### Q5: `"Formula Choice Calculus" variational execution`

Round B; all sixteen results screened.

1. https://eric.walkingshaw.net/projects/choice-calculus.html — existing project locator.
2. https://eric.walkingshaw.net/files/pubs/students/ataei-21-phd-dissertation.pdf — variational databases thesis; deferred application.
3. https://eric.walkingshaw.net/files/pubs/2016/fosd16-formula-choice-calculus.pdf — covered FCC.
4. https://sigplan.org/OpenTOC/fosd16.html — proceedings metadata.
5. https://www.researchgate.net/publication/309369297_Formula_choice_calculus — duplicate.
6. https://web.engr.oregonstate.edu/~walkiner/advising.html — thesis locators.
7. https://eric.walkingshaw.net/publications.html — selected author followup; opened and followed V10.
8. https://citeseerx.ist.psu.edu/document?doi=8ccb904dff79fb2280a6c7d0baf49f7acf4424c8&repid=rep1&type=pdf — projectional editing; screened as editing semantics, not CHR execution.
9. https://web.engr.oregonstate.edu/~walkiner/CV-Walkingshaw.pdf — author metadata.
10. https://conf.researchr.org/profile/ericwalkingshaw — author metadata.
11. https://dblp.dagstuhl.de/db/conf/oopsla/fosd2016.html — proceedings metadata.
12. https://dblp.org/db/conf/oopsla/fosd2016 — duplicate metadata.
13. https://2016.splashcon.org/track/fosd2016 — schedule metadata.
14. https://dblp.org/pid/21/5375 — author metadata.
15. https://2016.splashcon.org/profile/ericwalkingshaw — author metadata.
16. https://conf.researchr.org/room/SA-MDE-2016/movenpick-amsterdam-berlin — room schedule; excluded.

### Q6: `"TCHR" "tabling" constraints subsumption`

Round B; all sixteen results screened.

1. https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/article/description-implementation-and-evaluation-of-a-generic-design-for-tabled-clp/6C452C56E714D5458479091B94C58AD6 — 2019 forward lead; full fetch timed out.
2. https://people.eng.unimelb.edu.au/pstuckey/papers/tabled_clp.pdf — F15 author copy.
3. https://www.researchgate.net/publication/220643697_TCHR_a_framework_for_tabled_CLP — original mirror/citation discovery.
4. https://citeseerx.ist.psu.edu/document?doi=5e52ac87a0975b4e86f9aa4a6e0e04440a5dc7e2&repid=rep1&type=pdf — F15 duplicate.
5. https://cliplab.org/papers/chico-tclp-flops2012.pdf — selected F15 primary.
6. https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/volume/59CFB7E9C48D86957109A745F9545C2A — original volume metadata.
7. https://software.imdea.org/~mcarro/Material/Tabling/slides_tabled_clp.pdf — F15 slides; paper preferred.
8. https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/article/abs/tchr-a-framework-for-tabled-clp/9C39084E7862C575C428C48044680D94 — original abstract.
9. https://www.csauthors.net/david-scott-warren/ — secondary bibliography; not evidence.
10. https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/article/abs/description-implementation-and-evaluation-of-a-generic-design-for-tabled-clp/6C452C56E714D5458479091B94C58AD6 — item 1 abstract duplicate.
11. https://www.researchgate.net/publication/262311930_A_General_Implementation_Framework_for_Tabled_CLP — F15 duplicate.
12. https://www.researchgate.net/publication/330484102_Description_Implementation_and_Evaluation_of_a_Generic_Design_for_Tabled_CLP — item 1 mirror.
13. https://logicprogramming.org/tplp/content/tplp-volume-8-2008/ — original volume metadata.
14. https://www.researchgate.net/publication/344436914_A_Theoretical_Study_of_Full_Tabled_Constraint_Logic_Programming — deferred theory followup.
15. https://drops.dagstuhl.de/entities/document/10.4230/OASIcs.ICLP.2016.17 — deferred stream-data application; no new mechanism established.
16. https://www.cambridge.org/core/journals/theory-and-practice-of-logic-programming/article/tchr-a-framework-for-tabled-clp/9C39084E7862C575C428C48044680D94 — original full-page locator.

## Stopping-rule review and remaining gaps

The two sampled rounds found refinements within the existing families, and Round B resolved Round A’s possible specialization-family lead. This supports stability of the current candidate classification along these routes. It does not discharge the protocol’s broader requirements: title searches and author pages do not provide complete forward-citation inventories, and the earlier incomplete retrieval ledgers and foundational access gaps remain.

The practical verdict is therefore **coverage checkpoint complete; global coverage gate open**. Candidate specifications can proceed using the evidence and explicit limitations. These findings do not justify claiming literature saturation, selecting an architecture by performance, or beginning prototypes without the applicable review.

Remaining work should answer a candidate-specific question rather than accumulate citations:

- Recover the original propagation/disjunction sources already identified in T006 if their missing definitions prevent a semantic claim. Current later operational sources support composition, but do not substitute for full foundational inspection.
- Obtain Béchet’s total-correctness conditions before relying on the behavioral laws; keep specialization as a proposal meanwhile.
- Inspect the 2019 Mod TCLP successor and 2020 full-TCLP theory if a candidate relies on exact projection, entailment, or termination guarantees beyond F15.
- Follow R3’s Ghica/Smith resource-semiring types and Gimenez/Moser structure-of-interaction citations only when needed to establish a proposed certificate or compiler condition. They were located in references but not inspected.
- Investigate Banach’s static-safety lead if a candidate needs a graph invariant beyond the conditions already documented. The excerpt is not enough to claim an eligibility theorem.
- No inspected text establishes the combined preservation result for finite-tree unification, opaque explicit choices, conditional CHR consumption, and branch-specific propagation histories. This remains a semantic development obligation, not a reason to assume that another citation will supply it.

These routes also leave precision for backward SK/lambda synthesis unestablished. A certificate useful for a fixed forward call may be unavailable when its arguments are free; that is a programming and optimization cost to expose in candidate comparisons.

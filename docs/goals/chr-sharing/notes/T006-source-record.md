# Second-pass source and retrieval record

This bounded pass answered specific transfer questions raised by the first checkpoint. It is not a saturated survey. All retrieval occurred on 2026-09-06 through `web.run`; no runtime experiments were performed. The source-backed interpretation is in [the findings](T006-transfer-findings.md).

## Questions selected before retrieval

1. Does a published operational semantics already combine disjunction and propagation history? This blocks C01 reference fidelity and C05 answers.
2. At what work boundary does a real shared-search implementation yield? This blocks a meaningful fairness claim and scheduler comparison.
3. What can compiler analysis establish without mandatory source restrictions, and what do richer net targets permit? This informs C06 language costs and C02/C04 overhead.
4. Which context and equivalence assumptions underlie conditional representations? This bounds proposed C01/C03 transfers.

An additional single-text inspection of SMCHR followed its appearance in the CHR search results. Its potential to reuse learned consequences justified expanding mechanism coverage before the checkpoint.

## Primary evidence and access limits

### Token-aware disjunction

**S1. De Koninck, Schrijvers and Demoen, Flexible Search Strategies in Prolog CHR, CW447 (2006).** [Institutional PDF](https://www.cs.kuleuven.be/publicaties/rapporten/cw/CW447.pdf); [indexed primary copy](https://citeseerx.ist.psu.edu/document?doi=5675934ff9c82ad4e024d3fa69664369ff7375fe&repid=rep1&type=pdf). Inspected indexed §3.1/Table 1 and §3.3/Table 2 through targeted retrieval; full-PDF opening failed. Evidence is operational definitions, not a shared-execution theorem. The theoretical collection uses sets; concrete strategies use lists. Built-in theory is parametric. Finality precedes the success/failure distinction and requires exhaustion of transitions. Relevant: C01/C05. Search excerpts are sufficient for the narrow composition finding; full-text coverage is incomplete.

**S2. Wolf, Robin and Vitorino, Adaptive CHR Meets CHR∨ (2007/2008 version).** [DOI](https://doi.org/10.1007/978-3-540-92243-8_3); [author-uploaded indexed text](https://www.researchgate.net/publication/255409541_Adaptive_CHR_Meets_CHRv_An_Extended_Refined_Operational_Semantics_for_CHRv_Based_on_Justifications); [institutional record](https://publica.fraunhofer.de/entities/publication/45a51c1a-bc0c-4bf3-9c12-700a260e4d00). Inspected §§3–4 and conclusion. Operational definitions provide justification-based restoration; refined ordering differs from S01. Correspondence propositions are described as ongoing work in the inspected conclusion. Relevant: C01/C02; no correctness theorem for our search inferred.

**Original foundations, incomplete access.** Abdennadher, Operational Semantics and Confluence of Constraint Propagation Rules (CP1997), [DOI](https://doi.org/10.1007/BFb0017444): author archive located, PDF retrieval failed. Abdennadher and Schütz, CHR∨: A Flexible Query Language (FQAS1998), [publisher](https://link.springer.com/chapter/10.1007/BFb0055987): preview and references only. The publisher's later online date is not the original publication year. These texts are not treated as fully inspected evidence.

### Scheduling and solver reuse

**FL12. Böhm, Hanus and Teegen, From Non-determinism to Goroutines: A Fair Implementation of Curry in Go (PPDP2021).** [Author PDF](https://www.michaelhanus.de/papers/PPDP21.pdf), especially §7.2 pp.10–11, §6 p.9 and §7.4 p.12. Operational/implementation evidence and experiments. Ownership optimization is omitted from the formal machine; free-variable demand may introduce alternatives, unlike S03. No inspected theorem establishes host scheduling, lock fairness or fairness with a task cap. C01/C03/C04/C06. Direct retrieval of an existing lead; no new query. Kiel host failed, author domain succeeded. References 17–18 (Fair Scheme and Sprite) are uninspected expansion targets.

**FL11 continuation. Schrijvers, Demoen and Warren, TCHR: a framework for tabled CLP (2007 version).** [PDF](https://arxiv.org/pdf/0712.3830), §§5–8, PDF pp.18–28. Inspected suspension encoding versus history-free replay, call abstraction, constraint projection and answer compaction. Concrete design with evaluated encodings; set replay and ground-answer coverage are not duplicate-preserving execution. Fresh decoded identities and reactivation matter. Domain entailment and projection are obligations, not operations supplied automatically by arbitrary CHR rules. C01–C06. Direct follow-up to the first-pass text, not a new ranked search.

### Richer nets and inferred allocation reuse

**R1. Mazza, Multiport Interaction Nets and Concurrency (CONCUR2005 extended abstract).** [Author PDF](https://www.lipn.fr/~mazza/papers/mINSAndConcurrency-CONCUR05.pdf), §2 definition/typing; §3.1 Theorems 1–3; §3.2 Theorem 4 and conclusion. Formal encoding evidence with some proof details omitted. The stated constant-time local replacement applies to finite systems; displayed encodings use infinite symbol families. Guarded choice and match-prefix extensions are deferred. Replication relies on a prepared representation and stops at free channels. C01/C04/C06; no CHR or exhaustive-search transfer established.

**R2. Mackie and Sato, In-place Graph Rewriting with Interaction Nets (TERMGRAPH2016).** [PDF](https://arxiv.org/pdf/1609.03641), §§3–7. Rule classification and compiler annotation algorithm under a fixed-size cell representation; performance evaluation and broader integration are deferred. Local node-count eligibility is sufficient under those assumptions, not necessary for all bounded-space computations. It says nothing directly about search queues, histories or conditional ownership. C02/C04/C06. Backward leads: 2015 implementation model and Hofmann's 2000 bounded-space type system; uninspected in this pass.

### Context and conditional equivalence

**V8. Duck, Stuckey and Brand, ACD Term Rewriting (ICLP2006).** [PDF](https://www.comp.nus.edu.sg/~gregory/papers/iclp06.pdf), §§1,3–5, especially Theorems 1 and 3 and the implementation's context-change example. Formal definitions, stated soundness/correspondence and prototype algorithm. The CHR mapping restricts built-ins to true and guards to implicit equality; an underlying solver extension is proposed rather than included. Relevant: C01/C03/C06. Direct backward expansion from ICLP2008/2009 ACD texts.

**V9. Hubbard and Walkingshaw, Formula Choice Calculus (FOSD2016).** [Author PDF](https://eric.walkingshaw.net/files/pubs/2016/fosd16-formula-choice-calculus.pdf), §§2–4, especially Theorems 4.3–4.7. Denotational equivalence proofs for formula-labelled choices; configuration selection is the observation. No dynamic CHR operational semantics, multiplicity result or normalization cost bound inferred. C01/C02/C03. Followed the author project page's forward link from the original choice calculus. This is one attributable citation edge, not a systematic forward-citation pass.

### Conflict learning

**S4. Duck, SMCHR: Satisfiability Modulo Constraint Handling Rules.** [Full author PDF](https://www.comp.nus.edu.sg/~gregory/papers/smchr.pdf), §§3–8. UNSAT soundness theorem with proof sketch; UNKNOWN need not mean satisfiable. Clause reuse and historical measurements provide mechanism evidence, not project performance. Implemented solvers were manually compiled; variable indexing requires head-connectedness. Memory and post-choice common-work counts were not reported. C01/C03/C04/C06. Direct retrieval from the preceding CHR search lead; no additional query or citation expansion. The source's logical-solver assumptions are recorded in the findings and are not adopted.

## Exact queries and screening

CHR searches were individual calls, without recency filters:

- `"CHR" "disjunction" "propagation history" semantics`: selected S1 and S2; compilation thesis/surveys/book preview were discovery material; tabling and justifications were adjacent leads; topic aggregators excluded.
- `"Flexible Search Strategies in Prolog CHR" De Koninck pdf`: S1 and author/institutional/mirror locators; unrelated textbook preface excluded.
- `"Adaptive CHR Meets" pdf`: S2 author upload, mirror, institutional and publisher records; SMCHR retained for the added bounded inspection; bibliographies/catalogues metadata only.
- `"Operational Semantics and Confluence of Constraint Propagation Rules" "pdf"`: original abstract/author bibliography; later confluence, equivalence and unfolding papers were citation leads, not substitutes for original full text.
- `"CHR" "Flexible Query Language" "pdf" Abdennadher Schütz`: original publisher/author/mirror records; related language and translation papers retained as leads.
- `"Flexible Search Strategies in Prolog CHR" "Split" "Solve"` and `"Flexible Search Strategies" "Theoretical Operational" "Split"`: precise S1 definitions; latter returned one result, selected primary PDF.
- `"Adaptive CHR Meets" "Choose" "final states"`: S2 formal text and conclusion; unrelated parser issue excluded.
- `"CW447" pdf CHR`: institutional index located; unrelated identifier matches excluded.
- `"Operational Semantics and Confluence of Constraint Propagation Rules" "token" "fresh"`: compositional-semantics/unfolding leads, not original text.
- `"Flexible Search Strategies in Prolog" "answers" "equivalence"`: no useful answer-equivalence evidence.

**CHR ledger limitation:** the research receipt summarized titles by groups and did not preserve full ordered URLs for these searches. Two unsuccessful identifier/site searches were also not retained verbatim. This does not satisfy the protocol's complete first-20 ledger requirement; no supporting claim depends on the unsuccessful searches. S1/S2 locators and passage-specific retrieval remain attributable.

Net query A: `interaction nets in-place computation reuse nodes static analysis Mackie Sato`. Twenty-three results returned; first twenty screened in order:

1. [In-place rewriting, arXiv](https://arxiv.org/abs/1609.03641): selected R2.
2. [Static Analysis for Distributed Implementations](https://doi.org/10.1007/bfb0032744): deferred analysis lead.
3. [In-place rewriting, EPTCS](https://cgi.cse.unsw.edu.au/~eptcs/paper.cgi?TERMGRAPH2016.4.pdf=): duplicate primary locator.
4. [An Implementation Model](https://arxiv.org/abs/1505.07164): deferred lead.
5. [In-place rewriting, IP Paris](https://researchportal.ip-paris.fr/en/publications/in-place-graph-rewriting-with-interaction-nets/): metadata.
6. [In-place rewriting, ResearchGate](https://www.researchgate.net/publication/308009169_In-place_Graph_Rewriting_with_Interaction_Nets): duplicate.
7. [Static Analysis, ResearchGate](https://www.researchgate.net/publication/221477516_Static_analysis_of_interaction_nets_for_distributed_implementations): duplicate lead.
8. [Programming Language Design and Implementation](https://eceasst.org/index.php/eceasst/article/view/2523): deferred compiler lead.
9. [IP Paris French](https://researchportal.ip-paris.fr/fr/publications/in-place-graph-rewriting-with-interaction-nets/): duplicate.
10. [Compiling Process Networks](https://arxiv.org/abs/1609.03640): deferred topology/stream lead.
11. [Additive and Multiplicative Structures](https://academic.oup.com/logcom/article/15/2/219/1079235): deferred encoding lead.
12. [Implementation Model, EPTCS](https://cgi.cse.unsw.edu.au/~eptcs/paper.cgi?TERMGRAPH2014.5.pdf=): duplicate.
13. [Parallel Evaluation](https://eceasst.org/index.php/eceasst/article/view/2205): inspected in first pass.
14. [Language Design, ResearchGate](https://www.researchgate.net/publication/220054026_Interaction_nets_programming_language_design_and_implementation): duplicate.
15. [Iterators and Recursors](https://arxiv.org/abs/0910.3321): deferred compiler lead.
16. [Static Analysis, DBLP](https://dblp.uni-trier.de/rec/conf/sas/Mackie97.html): metadata.
17. [In-place rewriting, EmergentMind](https://www.emergentmind.com/papers/1609.03641): secondary duplicate.
18. [Compilation, ResearchGate](https://www.researchgate.net/publication/222033040_Compilation_of_Interaction_Nets): deferred lead.
19. [Interaction Nets, Wikipedia](https://en.wikipedia.org/wiki/Interaction_nets): secondary, not evidence.
20. [Spatial Architecture, Wikipedia](https://en.wikipedia.org/wiki/Spatial_architecture): indirect, excluded.

Net query B: `Mazza interaction nets semantics concurrent extensions multiport two port universality 2006`, domain `lipn.fr`. All thirteen results screened in order: [thesis 0.3](https://lipn.fr/~mazza/papers/Thesis-0.3.pdf) (earlier version); [thesis](https://www.lipn.fr/~mazza/papers/Thesis.pdf) (first-pass source); [R1](https://www.lipn.fr/~mazza/papers/mINSAndConcurrency-CONCUR05.pdf) (selected); [publications English](https://lipn.fr/pages/en/research/publications.html) (locator); [Pagani bibliography](https://lipn.fr/~pagani/bibtex.html) and [research](https://lipn.fr/~pagani/research.html) (adjacent leads); [Petrucci presentation](https://lipn.fr/~petrucci/presentation_uk.html) and [publications](https://lipn.fr/~petrucci/publications.html) (unrelated); [LOCAL theses](https://lipn.fr/pages/fr/research/teams/local/thesis.html) and [members](https://lipn.fr/pages/fr/research/teams/local/members.html) (locators); [publications French](https://www.lipn.fr/pages/fr/research/publications.html) (duplicate); [events](https://lipn.fr/pages/en/agenda/evenements.html) (adjacent, not evidence); [Le Roux](https://lipn.fr/~leroux/publications.html) (unrelated).

Conditional query: `"Formula Choice Calculus" Hubbard Walkingshaw pdf`. Fifteen visible results screened in displayed order: V9 author PDF (selected); author project page (version locator); Hubbard thesis (deferred fuller proofs); ResearchGate article (duplicate); author publications (locator); author CV (metadata); advising page (metadata); DBLP FOSD alternate host (metadata); SIGPLAN proceedings (identity); DBLP FOSD (duplicate); Eurekamag (secondary, excluded); DBLP author (metadata); ResearchGate topic list (excluded); SPLASH proceedings (identity); researchr proceedings (metadata). Search-engine publication estimates were ignored in favor of the paper's FOSD2016 imprint.

## Coverage verdict

Backward edges inspected include ACD2009/2008 to ACD2006 and later CHR∨ to earlier semantics. The original CHR papers remain access gaps. The author-maintained choice-calculus page supplies one forward edge to FCC. Systematic forward searches are still outstanding.

This pass produced a distinct conflict-learning family and additional uninspected resource/type-analysis leads. It cannot count as a no-new-family round. The protocol's stopping rule remains unmet; documentation validation cannot establish research saturation.

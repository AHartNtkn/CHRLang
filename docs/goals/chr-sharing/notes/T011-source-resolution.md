# Source access and solver-interface resolution

The full CW447 report supplies the later operational definition needed to combine propagation histories and explicit disjunction. Full TCLP theory clarifies when approximate call projection is admissible and why answer projection has stricter obligations. These findings resolve specific source gaps; they do not establish global literature exhaustion.

Research occurred on 2026-09-06 using `web.run` and, for the failed institutional PDF fetch, `curl` and `pdftotext`. No evaluator or experiment was run. This follows [the coverage record](T009-coverage.md).

## CW447: a complete later operational reference

De Koninck, Schrijvers and Demoen, *Flexible Search Strategies in Prolog CHR*, CW447, May 2006. [Institutional PDF](https://www.cs.kuleuven.be/publicaties/rapporten/cw/CW447.pdf), §3.1/Table 1 and §3.3. Full PDF recovered with curl; relevant definitions and theorem locations inspected in extracted text. The index calls the title “for Prolog CHR”; the report itself uses “in Prolog CHR”.

States contain goals, identified occurrences, built-ins, history and a fresh identifier supply. Apply uses renamed-apart rules, matching, guard entailment and absence of the occurrence/rule token. Split inherits store, built-ins, history and supply into alternatives; shared implementations therefore need branch-scoped future identities. Finality precedes success/failure classification. Theoretical alternatives form a set; refined alternatives form a list and successful Next requires an external request. This supports token/disjunction composition, not multiplicity preservation or post-choice computation sharing. Refined ordering is not the project’s unordered baseline. Sections 4–5 include correspondence arguments for that paper’s search-tree/trailing construction. Evidence: operational definitions and proofs; C01/C05.

The 1997/1998 papers remain historical access gaps. They need not block specification from this explicit later primary definition. Cite CW447 as the selected later definition; do not claim the originals were inspected or every version is identical.

## F16: exact answers and constraint-sensitive call reuse

Arias and Carro, *A Theoretical Study of (Full) Tabled Constraint Logic Programming*, 2020. [Primary PDF](https://arxiv.org/pdf/2009.14430), §§2.1–2.4; §3/Theorem 3; §4/Table 2. Full PDF accessible; pertinent definitions and arguments inspected.

The definite-program framework assumes linearized heads and a constraint domain supporting renaming, conjunction, existential quantification, satisfiability, entailment and projection. Exact projection preserves solutions under existential elimination. Exact answer projection with exact or overapproximated call projection preserves soundness/completeness under the framework’s consumer filtering: answers are conjoined with original consumer constraints. Underapproximation may lose answers; overapproximated answers may introduce invalid valuations. Call abstraction may destroy termination.

Theorem 3 concerns compactness of generated projected/renamed call and answer sets, with finite predicates and linearized definite programs. It is not a decidable general termination procedure. Finite trees alone do not suffice: unbounded Herbrand terms yield a noncompact set. Table 2 and the following discussion resolve a potentially confusing introductory sentence around its soundness table. Evidence: formal semantics/theorems and projection analysis; C01/C03/C05/C06.

For a proposed solver interface, distinguish permission to generalize a call from permission to publish its answers. Preserve consumer constraints and exact residual conditions. These set/model results do not authorize merging consumable CHR stores, omitting propagation history or discarding distinct operational alternatives. The combined project preservation result remains original semantic work.

## Retrieval and access audit

Queries were issued individually, without recency or domain restrictions. Every returned result was screened because each query returned fewer than twenty. The lists preserve displayed order, including academic results. Publication dates come from source identity, not search-engine age estimates.

### Q7: `"CW447" "Flexible" CHR`

Ten results; 1–2 selected as locators, all others unrelated identifier matches.

1. https://www.cs.kuleuven.be/publicaties/rapporten/CW/2006/
2. https://people.cs.kuleuven.be/~tom.schrijvers/portfolio.html
3. https://agasmfg.com/product/custom-street-banners/
4. https://surveydata.nl/studies/view/205
5. https://www.carousell.sg/p/professional-electric-nail-drill-kitmercedes-2000-14000-rpm-nail-drill-set-cw447-13098-1378505228/
6. https://surveydata.nl/studies/view/57
7. https://surveydata.nl/studies/view/279
8. https://www.nextag.de/shopping/products?page=9&search=elektrische-pedik%C3%BCre-und-manik%C3%BCre
9. https://www.carousell.sg/electric-nail-drill/q/
10. https://surveydata.nl/studies/view/12

### Q8: `"A Theoretical Study" "Full" "Tabled Constraint"`

Fourteen results.

1. https://arxiv.org/abs/2009.14430 — selected F16; followed PDF.
2. https://www.researchgate.net/publication/344436914_A_Theoretical_Study_of_Full_Tabled_Constraint_Logic_Programming — duplicate.
3. https://www.csauthors.net/manuel-carro/ — secondary bibliography.
4. https://software.imdea.org/research/publications/2020/techreport/ — institutional identity.
5. https://software.imdea.org/es/research/publications/2020/techreport/ — duplicate.
6. https://software.imdea.org/research/publications/2020/ — institutional identity duplicate.
7. https://www.researchgate.net/publication/220643697_TCHR_a_framework_for_tabled_CLP — predecessor mirror/citation.
8. https://www.researchgate.net/publication/330484102_Description_Implementation_and_Evaluation_of_a_Generic_Design_for_Tabled_CLP — predecessor mirror.
9. https://cliplab.org/clippubsbyyear/clippubsbyyear.pdf — institutional bibliography.
10. https://cliplab.org/clippubsbyyear/node7.html — duplicate bibliography.
11. https://dblp.org/pid/167/4968.html — author metadata.
12. https://www.deepai.org/profile/manuel-carro — secondary; not evidence.
13. https://dblp.dagstuhl.de/pid/72/789.html — author metadata.
14. https://www.researchgate.net/figure/A-Truth-Lattice-for-a-Simplified-Version-of-Courteous-Argumentation-Theory_fig2_221176607 — tangential figure/citation; excluded.

### Q9: `"Flexible search strategies" "pdf"`

Seventeen results; only 11 relevant. All other items excluded as unrelated subject matches. The query was then narrowed to the exact title.

1. https://www.sciencedirect.com/science/article/pii/S1071581902910116
2. https://www.sciencedirect.com/science/article/abs/pii/S1071581902910116
3. https://ceur-ws.org/Vol-1186/paper-02.pdf
4. https://www.cambridge.org/core/journals/symposium-international-astronomical-union/volume/DCD3ECB33AC9EF230510D64766B444E4?pageNum=4
5. https://v1.isokineticstatic.com/scientific-journal/2024-07-17/Neurocognitive%20errors%20ACL%20injuries_240705112916.pdf
6. https://dante.univ-tlse2.fr/s/fr/item/7460
7. https://www.cambridge.org/core/journals/symposium-international-astronomical-union/article/an-infrared-search-in-our-solar-system-as-part-of-a-more-flexible-search-strategy/7DC0A722787E33C13FFD65A57BA66E19
8. https://github.com/kt-chan/anyzearch
9. https://pmc.ncbi.nlm.nih.gov/articles/PMC2384229/
10. https://www.researchgate.net/publication/300484419_A_Modular_Approach_to_Promote_Creativity_and_Inspiration_in_Search
11. https://www.researchgate.net/publication/238684852_Flexible_Search_Strategies_in_Prolog_CHR — target metadata; no full text.
12. https://huggingface.co/papers/2303.00501
13. https://link.springer.com/article/10.1007/s12599-018-0569-6
14. https://www.scribd.com/document/882109576/730-Stream-of-Search-SoS-Learn-2
15. https://srhr.dspace-express.com/server/api/core/bitstreams/c2ad1118-9e3f-4834-8664-e44025aaace7/content
16. https://lib.swu.ac.th/dbma/pages/ckeditor/uploads/files/EndNote%20-%20test.pdf
17. https://aaltodoc.aalto.fi/bitstreams/9fab0ba5-7c12-4992-9aa5-e507dcbe6323/download

### Q10: `"Flexible search strategies in Prolog CHR"`

Thirteen results.

1. https://www.researchgate.net/publication/238684852_Flexible_Search_Strategies_in_Prolog_CHR — target metadata.
2. https://citeseerx.ist.psu.edu/document?doi=5675934ff9c82ad4e024d3fa69664369ff7375fe&repid=rep1&type=pdf — indexed primary mirror; full open failed.
3. https://people.cs.kuleuven.be/~tom.schrijvers/portfolio.html — author identity.
4. https://wms.cs.kuleuven.be/dtai/people/dtaiMembers/dtai-publications-2006/view?pubsonpage=10&pubtype=report&sortby=stitle — institutional report/proceedings metadata.
5. https://dtai.cs.kuleuven.be/static/projects/CHR/old/chr-papers.shtml — topic bibliography.
6. https://dtai.cs.kuleuven.be/people/dtaiMembers/dtai-publications/view?pubsonpage=50&pubtype=report&sortby=popularity — duplicate metadata.
7. https://dtai.cs.kuleuven.be/people/dtaiMembers/dtai-publications-2006/view?pubsonpage=20&pubtype=report&sortby=scdate — duplicate metadata.
8. https://wms.cs.kuleuven.be/dtai/people/dtaiMembers/dtai-publications/view?pubsonpage=20&pubtype=report&sortby=screator — duplicate metadata.
9. https://dtai.cs.kuleuven.be/people/dtaiMembers/dtai-publications-2006/view?fromnr=1&pubsonpage=50&pubtype=&sortby=screator — duplicate metadata.
10. https://wms.cs.kuleuven.be/dtai/people/dtaiMembers/dtai-publications/view?fromnr=61&pubsonpage=20&pubtype=report&sortby=scdate — duplicate metadata.
11. https://www.researchgate.net/publication/222511526_Automatic_Implication_Checking_for_CHR_Constraints — adjacent citation; not selected.
12. https://dtai1.cs.kuleuven.be/people/dtaiMembers/dtai-publications/view?pubsonpage=20&pubtype=report&sortby=screator — duplicate metadata.
13. https://dtai.cs.kuleuven.be/people/dtaiMembers/dtai-publications/view?fromnr=61&pubsonpage=20&pubtype=report&sortby=scdate — duplicate metadata.

### Direct access attempts

- Institutional-index report link: web click failed.
- CW447 URL lacking the `/cw/` path component: curl returned a 196-byte error page; not used as a PDF.
- Python urllib institutional-index fetch: DNS failure; no content obtained.
- CiteSeer full report: web cache miss.
- CHR2006 proceedings at https://arxiv.org/pdf/1406.1510: web cache miss.
- Correct institutional CW447 URL: curl succeeded (256 KB); `pdftotext -layout` succeeded. Temporary artifacts: `/tmp/chr-cw447.pdf` and `/tmp/chr-cw447.txt`.
- F16 at https://arxiv.org/pdf/2009.14430: web succeeded, 24 pages.

## Coverage disposition

The narrow combined-operational-definition access gap is resolved by the full CW447 text. F16 supplies exact projection/termination assumptions for a potential tabled solver interface. Historical source access and complete citation-inventory coverage remain open; neither prevents using these explicitly selected later definitions. Reopen source retrieval when a concrete claim depends on an unavailable result, rather than treating historical completeness as a prerequisite for every design step.

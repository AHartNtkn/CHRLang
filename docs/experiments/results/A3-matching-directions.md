# A3 matching architectures beyond delta maintenance

The first three controls isolate invalidation/reuse within a partial-match
representation. They do not exhaust matching architecture. The following primary
sources identify independent, feasible contrasts; none requires constructor
relationalization. Sources accessed2026-09-07.

[Miranker's TREAT paper, §§III–V](https://cdn.aaai.org/AAAI/1987/AAAI87-008.pdf)
contrasts retained beta memories with recomputation driven by changed-fact seeds,
while retaining alpha selections and complete matches. Deletion/storage costs
motivate the contrast, but lexical-order TREAT sometimes loses to Rete; seed
ordering matters. Therefore an unindexed rescan is not a sufficient TREAT control.
Probe repeated selective suffix arrivals versus high-fanout consumption churn,
holding predicate indexing and occurrence semantics constant. Record intermediate
retention, invalidation, seeded joins and complete-match maintenance. Open.

[Sulzmann and Lam's lazy-search CHR matching](https://edmundsllam.wordpress.com/wp-content/uploads/2017/07/sllam_chr_2007.pdf)
combines demand-driven search trees, join ordering and early guards. Probe many
mutually conflicting consuming matches, where execution needs only the next
witness, against propagation-only exhaustive enumeration and late guard failure.
Measure speculative matches invalidated before use as well as first-firing and
total costs. Current prefix selection is a relevant demand-driven control, but
retained complete-match sets and a lazy maintained search are distinct choices.
Open; no universal eager or lazy policy is selected.

[Worst-case-optimal joins](https://arxiv.org/html/1203.1952) and
[Free Join](https://arxiv.org/pdf/2301.10841) motivate multiway variable intersection
and hybrid plans. A triangle or clover can expose oversized binary intermediates;
selective acyclic heads and cold index construction are necessary contrary cases.
Free Join's combined planning and lazy trie construction caution against treating
binary versus multiway as an exhaustive binary choice. Measure updates, index
reuse and occurrence-output enumeration as well as relational value joins. Open.

[Incremental Leapfrog Triejoin, §§4.1–4.2](https://arxiv.org/pdf/1303.5313)
maintains sensitivity information about evaluation rather than beta tuples; its
analysis follows changes to execution traces, and notes little benefit for trivial
unary intersection. Probe deltas outside sensitive ranges versus broad sensitive
updates, retaining trivial heads as adverse controls. Charge sensitivity indexes
and repair alongside fresh joins and retained-prefix maintenance. Open.

[Equality-aware incremental maintenance](https://arxiv.org/pdf/1505.00212)
explicitly handles equality-induced key changes and reports cases where
rematerialization beats incremental repair. A small input delta can still touch
many indexed uses. Include high-degree equality changes, repeated-variable heads,
old aliases and consuming one of several equal-valued occurrences. Open.

For all these contrasts, value equality must not merge occurrence identities.
Finite scripted insertions, removals and alias updates can be checked by exhaustive
occurrence enumeration; actual source firings additionally require liveness,
guards, histories and a legal selection. Their costs include preparation,
rekeying, retained tuples/traces, candidate enumeration and cleanup. These are
architectural alternatives to test, not source-language changes or adopted
libraries. The first maintained-join result will update only its bounded contrast.

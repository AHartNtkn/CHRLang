# Projection avoids hidden enumeration, with explicit multiplicity and width costs

Finite-coordinate projection can eliminate hidden choices before enumerating visible answers. The implementation preserves visible correlations and, when requested, exact hidden-witness counts. Its benefit depends on both the observation contract and the size of intermediate factors; no timing or memory advantage has been measured yet.

## What was implemented and checked

The [projection implementation](../../../research/chr-structural/src/projection.rs) starts from finite source-choice domains and relation filters. Each becomes a small relation table, called a factor. To eliminate a hidden coordinate, it joins only the factors mentioning that coordinate and sums it out. Other factors remain separate. Visible answers are produced afterward, with optional restrictions on surviving coordinates.

The independent oracle enumerates complete source-choice tuples and directly tests relation membership. Across 256 generated problems and eight visible selections per problem, both elimination orders and both semantics match its exact maps: 8,192 projection comparisons per build, plus subsequent visible restrictions. Named tests cover empty domains, visible correlations, duplicated alternatives, duplicate filter rows, no visible coordinates and caller boundaries.

The existing finite grammar/search implementation independently agrees on a flat tuple/equality witness. Actual CHR source execution agrees on the visible answer set and completed-branch counts, including separate restrictions selecting each visible answer. Projection code is not used by either control. These source checks qualify the tested choice/equality construction, not arbitrary consuming or effectful hidden work.

Ten projection tests and all 16 existing finite-path tests pass in both default and metrics-off builds. Each final test binary ran under the registered 60-second CPU/wall and 1 GiB address-space bounds. The earlier development run is retained separately from those bounded receipts.

## The observation contract changes what can disappear

Logical projection returns one copy of each visible assignment. Counted projection instead records how many source-choice derivations produce it. Duplicate domain alternatives contribute to that count; repeated rows proving a membership filter do not. Treating both kinds of duplication alike would change the source result.

In the CHR witness, three hidden choices and two allowed visible assignments yield six completed branches. Set projection has two answers; counted projection records three witnesses for each. Counts are a compact representation of multiplicity, not permission to substitute two materialized answers for six raw source answers. A full-lifecycle comparison must charge any required expansion.

The counter type is checked `u128`. A case with 129 hidden Boolean coordinates explicitly reports overflow in counted observation, while its logical projection still has one empty visible tuple. Overflow and assignment limits are errors, never empty-solution results. Larger count representations or streaming multiplicity remain design options; this gate selects neither for the language.

A stronger arithmetic witness caught a defect after the initial tests: a large partial product overflowed before a later empty hidden domain was inspected. Multiplication now checks for zero factors before reporting overflow. A related test also caught assignment-size overflow before an empty domain; assignment counting now handles the empty domain first. The empty-domain witness returns no solutions, while the genuinely excessive nonzero count still reports overflow. The failing test and initial source snapshot are preserved; the full bounded gate was repeated after the repair.

## Both an opportunity and an adverse case ran

For 24 independent Boolean coordinates with one visible coordinate, elimination performs 46 local assignment visits and returns two visible tuples, each with weight 2²³. It does not enumerate the 2²⁴ full source-choice tuples. This demonstrates the distinctive operation; it does not compare wall time against every competent way to recognize independence.

The connected witness has eight Boolean coordinates, six hidden. A central coordinate is related to every leaf by the allowed pairs `00`, `01`, `10`. The two visible coordinates have counts 33 for `00` and 32 for each other pair under either elimination order.

| Elimination order | Assignment visits during elimination | Largest factor table |
|---|---:|---:|
| Central coordinate first | 504 | 128 entries |
| Hidden leaves first | 28 | 4 entries |

Elimination order changes intermediate representation and work without changing denotation. Visit counts involve different scopes and factor products, so their ratio is not a speedup. A credible cost comparison needs an economical ordering policy, its preparation cost, and cases where large intermediate factors remain unavoidable for the chosen representation.

## Privacy and caller information are not inferred

A declared caller-shared coordinate cannot be hidden. Once a coordinate has been eliminated, a later restriction mentioning it is rejected. Restrictions on surviving coordinates remain valid and are checked against fresh full enumeration.

The declaration is an input contract. The implementation does not discover whether a hidden identity escapes through CHR state, resources or future callers. Flat finite coordinate domains also do not implement arbitrary constructor-path projection, names, disequality or normal/neutral theories. Those questions remain required.

## Next investigation and evidence

Continue T076 with projection representation and lifecycle qualification. Compare a source-derived ordering policy with the fixed orders and competent enumeration; include hidden-independent, connected, dense-output and reuse cases. Separate projection preparation, later visible restrictions, weighted observation, raw-answer expansion and owner disposal. Retained input domains and intermediate factors must be accounted for before cost claims.

Adaptive separation costs remain the strongest ready alternative. Projection receives this next boundary because the favorable and adverse witnesses now identify concrete work and representation differences that could change the structural-solving choice. Reconsider adaptation, native local ownership, conditional equality and broader reuse at that qualification or a consequential obstruction. This is the first projection package after the descriptor breadth review. The architecture goal remains active.

[Entry registration](../registrations/S06-projection-entry.md), [order qualification](../registrations/S06-projection-order-gate.md), [independent tests](../../../research/chr-structural/tests/projection.rs), [bounded runner](../../../research/chr-structural/experiments/projection_gate.py), and [raw receipts](s06-projection-entry/) preserve the experiment. The [audit](s06-projection-entry/qualified/audit.json) verifies frozen binaries/sources, four successful bounded runs and exact test coverage. Scoped Clippy passes in both feature configurations and formatting passes. The reference interpreter is unchanged.

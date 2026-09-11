# Sparse joins avoid impossible assignments; dense joins add work

Joining existing relation rows avoids the large Cartesian intermediate in the equality-star witness. Dense relations expose the opposite effect: compatibility checking adds row probes without reducing emitted tuples. Both traversals now produce the same independently checked finite answers, including multiplicity and changed query restrictions.

This establishes a useful competing implementation for the next lifecycle comparison. It does not establish that sparse traversal is faster, nor that projection beats enumeration.

## The mechanism and its measured work

The new traversal uses the existing factor tables. It considers smaller tables first, joins rows that agree on shared coordinates, rejects values outside their domains, then sums the eliminated coordinate's weights. Domain duplicates count as distinct choices; duplicate filter rows remain one predicate. Cartesian traversal remains available through the same preparation code.

| Relation family | Coordinates | Cartesian assignment visits | Sparse row probes | Complete compatible joins |
|---|---:|---:|---:|---:|
| Equality star | 3 | 80 | 56 | 8 |
| Equality star | 6 | 5,456 | 164 | 20 |
| Equality star | 12 | Exceeds 100,000-assignment bound | 380 | 44 |
| All-pairs allowed star | 3 | 80 | 392 | 80 |
| All-pairs allowed star | 4 | 336 | 1,676 | 336 |

These are different units of work. A row probe checks compatibility and domain membership; a Cartesian visit looks up factor weights for one assignment. The table is evidence about avoided work and added checks, not a timing ratio.

The wide equality-star witness eliminates the central coordinate first. Its relations permit only equal values, so joining existing rows avoids enumerating impossible combinations. The twelve-coordinate case completes with four correctly weighted visible answers. The Cartesian attempt stops at its registered bound; it supplies no completed runtime measurement.

Elimination order still matters. The existing greedy order can avoid the central-first intermediate on a star. Therefore this witness alone cannot establish a gain over the current prepared solver. The connected integration retains the same greedy order for both traversals, allowing the next cost comparison to isolate row traversal.

Dense relations provide the adverse control. When every pair is allowed, joining cannot prune assignments. The four-coordinate case performs 1,676 row probes to emit the same 336 tuples Cartesian traversal visits directly. Building indexes or changing join order might reduce that overhead, but their construction and ownership costs must count.

## Correct answers before cost claims

The existing independent assignment test now checks both traversals across 2,048 generated problem/visibility combinations, in set and counted modes and both elimination orders. Additional cases cover empty domains and filters, duplicate choices, repeated filter rows, outside-domain values, incompatible overlaps, reordered outputs and changing restrictions.

The connected source test checks both prepared solvers across 128 source configurations and 2,048 observations against scalar/reference results. It includes counted outputs, aliases and changing restrictions. This checks finite answer sets and multiplicities; operational source derivation order remains a separate obligation. The existing output iterator is unchanged.

Projection and connected tests pass with counters enabled and disabled, and scoped Clippy passes. No reference-interpreter code changes. The experiment adds a sparse traversal to the existing solver and exposes it in the existing lifecycle runner; it introduces no new language restriction or replacement baseline.

## Next decision

Keep T076 active for complete lifecycle costs against Cartesian projection and enumeration. Use the same greedy elimination order, including favorable equality cliques and a genuinely dense relation control. The fixtures can express an always-true relation over two coordinates; add that control rather than treating existing fixtures as a boundary.

Charge relation construction, preparation, changed-query setup, observation, consumption and disposal. Compare ordinary timing separately from requested heap allocation. Investigate a consequential slowdown by separating repeated row scans from preparation overhead before introducing indexes. Broader caller observers and selective trace retention remain the strongest ready alternatives after that result.

[Registration](../registrations/S06-sparse-elimination.md). Reproduce the correctness and work cases with `cargo test --offline -p chr-structural --test projection --test joint_projection -- --nocapture`; repeat with `--no-default-features` for counter-free semantic checks. This is the first package since the portfolio review. The research goal remains active.

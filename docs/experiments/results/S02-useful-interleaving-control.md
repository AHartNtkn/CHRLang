# Partial equality already saves work in the relational executor

**The existing relational executor can avoid equality work by firing a consuming failure rule early.** At nested depth64 it services 2 equality deductions instead of 65 behind a settlement barrier. A source that requires the separately carried leaf to be established services all 65 in either schedule. This strengthens the control that a new contextual or local-rewrite organization must face.

The experiment also found and repaired duplicate constructor facts after equality merges. Deep joins could repeatedly traverse equivalent facts even though their final matches were deduplicated. Source occurrences retain their separate identities and multiplicity.

## The complete-source witness

A bind rule equates `f(...f(X))` with `f(...f(a))`. A consuming use rule examines an open occurrence carrying the same structure and the separate leaf X. Its body either fails or posts done. The favorable head needs only the outer constructor; the adverse head also has a stable positive guard requiring the separate leaf to equal a.

The test-local barrier drains pending equalities before each ordinary engine advance. The other path uses the existing interleaved advance. Both use the same prepared source rules, source application order and exact answer observer. The harness counts actual equality service externally; no production timing counters or alternative engine were added. The independent owned source evaluator checks every complete result. Failure cases have no answers; successful cases retain the completed leaf binding and done residual.

| Nested depth | Early failure: interleaved / barrier | Leaf-dependent failure | Successful counterparts |
|---|---:|---:|---:|
| 4 | 2 / 5 deductions | 5 / 5 | 5 / 5 |
| 16 | 2 / 17 deductions | 17 / 17 | 17 / 17 |
| 64 | 2 / 65 deductions | 65 / 65 | 65 / 65 |

Early activity therefore saves useful work when the source terminates the interpretation. It does not authorize skipping remaining equality work for successful publication. These are work counts, not runtime or complete lifecycle rankings.

## Two observations changed the investigation

**A deep ground pattern alone did not force leaf settlement.** The initial depth4 trial expected its failure rule to wait, but observed 2 deductions versus 5. Root merging exposes the other constructor descriptor, which already supports the deep pattern. A separate leaf-class guard supplies the intended adverse control. The prospective registration records that revision; it is an adjustment to a refuted workload premise, not a change to language matching.

**Duplicate facts made the deeper guarded control impractical.** The initial run completed the smaller cases and depth64 outer-only cases, then remained inside deep matching until interrupted. A focused regression found two identical constructor descriptors after alias/congruence repair. Repeated descriptor rows could generate repeated join paths before final-match deduplication.

The repair coalesces identical constructor facts during insertion and canonical-value replacement, using existing parent-column lookup. It never coalesces source rows. The regression requires one constructor descriptor and two matches for two equal-valued source occurrences. The full depth64 source experiment then completes, including successful publication and the leaf-dependent failure. The interrupted run is preserved; it supplies no completed timing result and does not establish an inherent relational cost.

This repairs an existing control, rather than establishing a new architecture. Earlier relational timings retain their source/binary scope; a new cost comparison must rebuild and measure the corrected implementation. Inactive physical rows still occupy storage, so this is not a reclamation result.

## Where genuinely different mechanisms remain

Consider the same application: `open(f(X)), ticket(X) <=> done(X)`, with an equality enabling the constructor or ticket match. Every organization must claim distinct source occurrences and prevent effects from a failed context from escaping.

| Organization | How this application becomes available | Distinct responsibility still requiring evidence |
|---|---|---|
| Flat constructor/source relations | Canonical constructor and occurrence rows join; incident columns are repaired on merges | Join discovery, duplicate-fact handling and repair costs; complete execution is already implemented |
| Contextual equality overlays | Common constructor facts are shared, while a context supplies equality consequences and resource liveness | Context-valid lookup, cross-context consequence reuse, conflicting claims and reclaiming overlays; whole-store copying does not implement this distinction |
| Union-find expressed through CHR rules | Representative/link and descriptor constraints trigger merges and source matches through rules | Administrative rule discovery, propagation history, orientation and fair progress must be charged as execution, not assumed free |
| Local incidence rewrites | A changed class visits incident constructor ports and source occurrences, enabling local resource claims | Atomic distinct-occurrence claims, incidence repair and completion detection; the existing relational store already has an incidence index, so indexing alone is not a new mechanism |
| Strategic port rewrites | Explicit graph rewrites connect equality, constructors and source applications under a selected reduction strategy | Source correspondence, duplicated identities, competing rewrites and finite service; graph representability alone establishes none of these |

These are responsibility sketches, not equivalence proofs or performance results. In particular, partial equality access and an incidence index are already present in the control. A candidate must exercise a further distinction—such as sharing contextual consequences or changing how local source applications are discovered—and charge its ownership costs.

## Next package and remaining obligations

Proceed with the contextual overlay/local-claim source gate described in [T072's entry](S02-local-rewrite-entry.md). Require a fork where a common constructor/equality consequence remains shared but consumption differs across contexts, alongside competing claims and later contradiction. Compare with cloned relational ownership and preserve complete answers and finite sibling service. This can change state ownership, whereas repeating the present early-failure contrast cannot establish that benefit.

The current package does not implement shared contextual equality, union-find through CHR or strategic port execution. Their comparison obligations remain under S02. T072 and the architecture goal remain active; T071's distinct pull-tabbing/template questions remain open.

## Evidence

[Registration](../registrations/S02-useful-interleaving-control.md), [source and regression tests](../../../research/chr-relational/src/interleaving_tests.rs), [complete structural run](s02-useful-interleaving/control.log), [interrupted run](s02-useful-interleaving/initial-incomplete.log), [failing duplicate regression](s02-useful-interleaving/duplicate-red.log), [full relational tests](s02-useful-interleaving/full-tests.log), and [Clippy](s02-useful-interleaving/clippy.log).

All 14 relational tests pass, including existing head-plan, source and store suites. Strict Clippy passes for the changed crate; the log also records existing dead-code warnings from the integrated dependency's build script. Reference implementations were unchanged.

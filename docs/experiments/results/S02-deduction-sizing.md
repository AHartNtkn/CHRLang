# Deduction reuse saves repeated work but exposes snapshot-copying costs

**The new source families exercise the intended reuse, but map retention is consequential enough to test before confirmation.** Compatible repeated equations save allocation in the sizing sample. Single, distinct and changed-binding cases pay substantial extra allocation. These are exploratory timings and deterministic allocation findings, not confirmed architecture rankings.

## The mechanism actually runs on these sources

Four families unify chains of constructors ending in an unknown with chains ending in `a`. The shared family has four alternatives using the same input identities; distinct uses four different pairs; changed first binds an unrelated variable differently in each alternative. Every alternative leaves a distinct tag, with optional local token consumption. Complete outputs include unknown leaves in unselected distinct pairs and the changed context value where applicable.

A structural test imports the actual benchmark source and counts newly computed transitions. A depth-n chain computes n+1 transitions for single and shared, 4(n+1) for distinct, and 4(n+2) for changed. The shared family therefore reuses post-fork deductions; the adverse families do not accidentally receive the same benefit. Its test-only observer is absent from cost builds.

The runner compares ordinary contextual execution, shared transitions, relational execution, compiled scan/indexing, inferred specialization and an exact-source control. The latter validates every source/query field before generating complete answers; it is not a general common-work hoister. Altered sources, missing or extra resources, missing outputs and changed input shapes are rejected.

## Sizing evidence

The [prospective registration](../registrations/S02-deduction-lifecycle-sizing.md) produced **672 exploratory timing processes, 448 allocation processes and 28 cancellation processes**. All completed successfully. All 224 allocation replay pairs agree exactly; complete answers agree with the independent scalar evaluator, and query/prepared disposal restores requested-allocation baselines. The full crate/target tests and scoped strict Clippy pass.

The table shows one timing per path at depth32/33, four changing queries, resources present and forward starting order. Primary totals include preparation, setup, execution with complete observation, engine/answer disposal and prepared disposal. Values are milliseconds.

| Family | Contextual | Shared deductions | Relational | Scan | Indexed | Specialized | Exact source |
|---|---:|---:|---:|---:|---:|---:|---:|
| Single | 0.377 | 0.859 | 0.965 | 0.128 | 0.172 | 0.160 | 0.026 |
| Compatible repeated | 1.405 | 0.881 | 4.312 | 0.336 | 0.302 | 0.389 | 0.042 |
| Distinct pairs | 1.391 | 2.964 | 4.583 | 0.512 | 0.562 | 0.692 | 0.082 |
| Changed bindings | 1.479 | 3.030 | 3.648 | 0.400 | 0.328 | 0.375 | 0.027 |

Single timings cannot establish gains, losses or ordering. The complete matrix also includes depths zero/eight, one query, absent resources and reversed starting order. [Selected values](s02-deduction-sizing/selected-sizing.json) and the adjacent raw records preserve the numerical evidence.

The exact requested-allocation figures for these same substantive configurations expose the tradeoff more directly:

| Family | Contextual, MB | Shared deductions, MB |
|---|---:|---:|
| Single | 0.508 | 1.023 |
| Compatible repeated | 1.834 | 1.102 |
| Distinct pairs | 2.011 | 4.071 |
| Changed bindings | 1.845 | 3.995 |

These are cumulative primary requested bytes, not RSS or live footprint. The favorable family saves three of four computations and requests fewer bytes, but still has lookup, state ownership and retained-map costs. Requested bytes alone do not establish total efficiency.

The subsequent [paired map attribution](S02-equality-map-attribution.md) now tests this representation question; the sizing above retains its original binary scope.

## Consequential implementation question

The current cache pins the parent and descriptor maps after each newly computed equality transition. The next mutation through `Rc::make_mut` must copy a pinned map. Repeatedly retaining growing ordered maps can therefore introduce copying proportional to the accumulated map sizes. Without retention, a uniquely owned map can mutate in place.

This is a concrete property of the current representation. It is not an unavoidable cost of retaining a derivation or of contextual integration. The allocation contrast is consistent with it, but does not separately quantify map copying versus other costs. A paired representation experiment is needed before treating the adverse total as a credible bound for the reuse mechanism.

The next package will compare structurally shared persistent maps with the current ordered-map snapshots, retaining ordinary and cached variants of both. The repository already contains a persistent map with independent update/snapshot tests. Its applicability must be checked; using it does not give that prototype architectural priority. Lookup cost, cloned values, retention and disposal must be charged, and the full source/ownership gates must pass before timing.

This attribution takes priority over immediate broad confirmation because it can change both adverse and favorable comparisons at modest implementation scope. It also takes priority over T073's reusable lowering for one package because a known representation choice currently obscures the newly tested reuse mechanism. Reconsider that decision after the paired result; neither repeated refinement nor a prototype's ease of extension is sufficient reason to continue.

## What remains open

The stronger compiled and exact-source controls remain required in confirmation. The exact control does not establish what general source analysis can eliminate, and a common-work hoisting transformation needs its own source-policy argument. Unknown-input/generalized validity, union-find expressed through CHR, strategic port rewrites and sustained cache lifetime remain distinct investigations.

Native compilation is not included. Timing is counter-free with the ordinary allocator; heap diagnostics use a separate build. This package establishes source correspondence, actual reuse and exploratory cost pressure. T072 and the architecture goal remain active.

[Freeze and raw receipts](s02-deduction-sizing/freeze.json) · [Mechanism test](s02-deduction-sizing/mechanism.log) · [Runner source gate](s02-deduction-sizing/source-gate.log) · [Full tests](s02-deduction-sizing/tests.log) · [Scoped Clippy](s02-deduction-sizing/clippy.log)

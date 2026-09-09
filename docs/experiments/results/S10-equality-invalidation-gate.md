# Avoiding redundant equality work: two complementary controls

Runtime detection and source elimination avoid different work. Detection catches equality made redundant by earlier bindings; elimination avoids constructing and processing syntactically identical terms. Both preserve the tested answers and progress obligations, but their lifecycle costs remain unmeasured.

This implements the comparison selected in the [entry](S10-equality-invalidation-entry.md), following the large equality-reset penalty in the [resumable lifecycle](S10-resumable-lifecycle.md). It does not select a complete architecture.

## What changed

The contextual runtime can now distinguish **processing an equation** from **changing its canonical equality roots**. With the experimental `precise-invalidation` feature, an equation whose endpoints already have the same root advances the queue without clearing matching candidates or resumable cursors. Distinct-root steps still invalidate, including constructor decomposition, binding changes and failures. The ordinary control retains conservative invalidation. This is a same-root check, not a general test of relevance to live rule heads.

The independent source control replaces syntactically identical body equations with `True`, recursively through conjunctions and choices. It clones rules once; guards and nonidentical equations are unchanged. It uses no fixture names or runtime knowledge. The transformation is explicitly invoked rather than enabled in ordinary preparation.

These controls remove different responsibilities. Detection retains equation creation, queueing and checking, but can preserve matching work. Elimination adds source traversal and rule cloning while avoiding the equation's runtime construction and processing. Neither establishes that broader equality dependency tracking is unnecessary.

## Measured matching work

Each source has 16 propagation occurrences. The dynamic source first aliases two different variables, then repeatedly equates them. Counts are candidate offers, including unsuccessful final searches; they are not elapsed time, allocations or source-rule applications.

Each cell lists **eager / demand / resumable** discovery. Eager retains all discovered candidates, demand restarts its search each time, and resumable retains its traversal position.

| Equation form | Source elimination | Conservative invalidation | Same-root detection |
|---|---|---|---|
| `X = X` | No | 152 / 152 / 152 | 16 / 152 / 16 |
| `X = X` | Yes | 16 / 152 / 16 | 16 / 152 / 16 |
| `box(X) = box(X)` | No | 168 / 168 / 168 | 152 / 168 / 152 |
| `box(X) = box(X)` | Yes | 16 / 152 / 16 | 16 / 152 / 16 |
| `X = Y`, after aliasing | No | 153 / 153 / 153 | 17 / 153 / 17 |
| `X = Y`, after aliasing | Yes | 153 / 153 / 153 | 17 / 153 / 17 |

**Dynamic redundancy requires more than syntactic elimination.** The source control leaves `X = Y` intact, while runtime detection reduces eager and resumable offers from 153 to 17. The extra offer beyond 16 is the initial alias rule.

**Constructor reflexivity exposes work before the same-root check can help.** Two freshly constructed outer nodes initially have distinct roots. Their merge invalidates matching and queues a child self-equation. Detection avoids invalidation for that child, but cannot avoid the outer merge. Source elimination avoids both. Pending child work also causes an additional unsuccessful demand search, accounting for its 168 offers.

**Retaining traversal is not the only way to avoid repeated discovery.** Eager and resumable counts agree in these witnesses. Their memory ownership and total costs must distinguish them; this table cannot.

## Correctness and limits

The three finite source families are checked against independently written full answers, the independent scalar evaluator, conventional Scan and all three contextual discovery modes, with original and transformed rules. Checks preserve multiplicity, residual constraints and variable aliases rather than comparing only output values.

Additional tests cover meaningful alias and constructor binding, incompatible constructors, partial decomposition, successful and failing choice branches, and fresh variables with shared and distinct identities. After merging `f(X)` with `f(Y)`, the store retains two descriptor environments even after the children become aliases. Tests preserve those alternatives and check canonical equality directly; they do not assume descriptor deduplication.

A finite answer beside a continuing reflexive loop is delivered within 10,000 service advances, followed by 1,024 further progress events without false exhaustion or extra answers. Existing late-guard, insertion, consumption, broad mixed-source and composition/progress tests also pass. These are bounded witnesses, not a proof that every source scheduling interaction is preserved.

The final runs pass 21 relational tests with detection and work counters, 19 with conservative invalidation and work counters, and 21 with detection and counters disabled. Three broad/composition tests pass with detection. Scoped strict Clippy passes for the relational controls and counter-free compiler library. Logs are in [the evidence directory](s10-equality-invalidation-gate/).

Reproduce the principal controls with:

```sh
cargo test -p chr-relational --features precise-invalidation,local-work --lib --test equality_invalidation --test demand_discovery --test resumable_discovery -- --nocapture
cargo test -p chr-relational --features local-work --lib --test equality_invalidation --test demand_discovery --test resumable_discovery -- --nocapture
cargo test -p chr-relational --features precise-invalidation --lib --test equality_invalidation --test demand_discovery --test resumable_discovery
cargo test -p chr-direct-conditional --no-default-features --features chr-relational/precise-invalidation --test broad_mixed --test composition_progress
```

## Decision and next comparison

Carry both controls into a bounded lifecycle comparison. The observed difference is large enough to change the attribution of the previous equality-reset penalty; attributing that penalty to retained cursors now would be unsupported.

Register exact configurations before comparative runs. Compare original and eliminated sources under conservative and same-root invalidation, including Scan, eager, demand and resumable execution. Include dynamic redundancy, constructor reflexivity, meaningful bindings, partial decomposition and mixed-source adverse controls. Charge analysis/cloning, preparation, changing queries, execution, observation, cancellation and all owner disposal. Keep requested-allocation diagnostics separate from ordinary-allocator counter-free timing. Native compilation is a separate unresolved cost.

Support-aware conditional joining remains the strongest ready alternative investigation. The bounded lifecycle attribution takes precedence because it tests whether a measured whole-path penalty survives these stronger controls; that is a prioritization judgment, not evidence against joining. Reconsider that choice after attribution. Broader language, sustained lifetime, coherent-architecture and held-out obligations remain required; T078 and the research goal stay active.

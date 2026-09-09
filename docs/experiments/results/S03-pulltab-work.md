# Pull-tab savings depend strongly on result validity

The local rewrite avoids some traversal, but its largest measured work reduction comes from enabling broader result reuse. A stronger ordinary-demand control is needed before timing can attribute that gain to graph rewriting. These are diagnostic counts, not runtime or memory-byte comparisons.

## What ran

The [prospective registration](../registrations/S03-pulltab-work.md) covers 72 configurations: direct, opaque or nested constructor demand; one or four consumers; shared or independent choices; producer-chain depths 0, 8 and 32; and both query insertion orders. Each source was checked against a hand-built complete answer set and the independent scalar evaluator. Each strategy completed within 100,000 ticks. The two cache-policy investigations each ran twice, producing 576 strategy rows with identical repetitions and no cutoff under the registered process limits.

The existing `pack` workload puts choices inside a constructor and does not activate this direct-argument rewrite. The new direct-demand sources do. Opaque and nested sources deliberately test the boundary: neither produces a lift. Nested demand does add unsuccessful lift-site inspection, which must count as overhead in any later timing.

The first comparison fixes StaticBirth validity: a sufficiently pure application may reuse its result at its birth context when its matched input structure is directly available. The registered attribution repeat uses CurrentContext validity in both strategies, disabling that widening. It is an attribution control, not the strongest architectural competitor.

## Where the work changes

All rows below use forward insertion order. “Independent” means four distinct choice births and 16 joint answers; the one-consumer rows produce two answers. Each cell shows ordinary demand → local rewriting.

| Source and validity policy | Source expansions | Force entries | Retained nodes | Call obligations |
|---|---:|---:|---:|---:|
| One consumer, depth 0; either policy | 3 → 3 | 45 → 59 | 11 → 14 | 2 → 4 |
| One consumer, depth 32; either policy | 35 → 35 | 2,705 → 2,619 | 107 → 110 | 34 → 36 |
| Four independent consumers, depth 32; StaticBirth | 196 → 162 | 205,569 → 117,256 | 484 → 495 | 136 → 166 |
| Four independent consumers, depth 32; CurrentContext | 196 → 196 | 205,569 → 203,009 | 484 → 529 | 136 → 166 |

The default-policy direct-demand comparison has fewer source expansions in 3 of 24 configurations and equal expansions in the rest. Force entries decrease in 11, stay equal in 9 and increase in 4. With CurrentContext validity, source expansions are equal in all 24; force entries decrease in 8, stay equal in 11 and increase in 5. Insertion order affects which calls encounter an unresolved choice, so not every direct-demand configuration actually lifts a call.

Code inspection explains the expansion difference. Substitution exposes a concrete constructor argument in each copied call. The existing purity certificate can then reuse that call's result in its birth context, before unrelated choices are fixed. The ordinary call still refers indirectly to its choice-bearing producer; its conservative certificate does not recognize equivalent dependency information. Disabling widened validity removes the expansion difference. This supports the attribution on these sources; it does not establish a universally minimal dependency key.

The remaining traversal difference is real but incurs other work. In the four-independent depth-32 CurrentContext row, the rewrite adds 15 choice nodes, 30 calls and 30 conditional obligations. It also examines 510 entries while locating the lift sites. Force entries and lift-site entries are different operations; subtracting one count from the other would not yield a time estimate.

## Measurement ownership and validation

Retained counts come from a post-execution graph scan. Each source birth creates one original choice node, so choice nodes minus births counts lifts. Each lift adds one administrative result edge; subtracting those from all result edges gives source expansions. The structural unit test checks this accounting on a known rewritten graph. Retained nodes exclude observer-owned answers and do not measure requested bytes or RSS.

Force, pattern-match and lift-site entries are counted only under the optional `work-diagnostics` feature. The ordinary build has no fields or updates for those counters. Counts include execution and answer traversal together; no phase-specific timing claim is made. Existing result-edge diagnostics now explicitly document that administrative edges are included, preventing them from being misreported as source expansions.

The package-wide tests with diagnostics enabled and Clippy validate the instrumented implementation. The structural test directly checks three force entries, one match entry and two lift-site entries for its tiny graph. Complete-answer and resource/progress tests retain their source expectations. The reference evaluator is unchanged.

Raw runs and frozen source snapshots are in [StaticBirth data](s03-pulltab-work/freeze.json) and [CurrentContext data](s03-pulltab-current-work/freeze.json). The [paired summary](s03-pulltab-work-summary.json) is reproduced by:

```sh
python3 research/chr-direct-conditional/experiments/pulltab_work_analysis.py \
  docs/experiments/results/s03-pulltab-work \
  docs/experiments/results/s03-pulltab-current-work
```

Build the diagnostic test using `cargo test -p chr-direct-conditional --no-default-features --features work-diagnostics --test pulltab_work --no-run --release`. The runner takes that emitted binary path and a new output directory; add `--current-context` for the attribution policy. Frozen snapshots retain the exact source revisions used by each run.

## Next decision: strengthen the ordinary-demand control

Investigate whether an ordinary pure call can record the choice and producer-result dependencies actually used by its match. If that permits the same reuse without copying the call, a lifecycle comparison must include it. The validity argument must account for dynamic births, aliases, fresh locals, effects and failed matches; merely dropping context entries is unsound. Keep resource-consuming calls outside a purity-based certificate unless a separate ownership argument supports them.

This is a consequential control investigation, selected ahead of both broad pull-tab timings and T073's reusable lowered-query artifacts. The observed expansion gain could otherwise be wrongly assigned to an architectural representation. After the dependency-validity gate and a bounded attribution recheck, reconsider lifecycle measurement against artifact reuse and the remaining integration mechanisms. A larger timing matrix now would measure an unresolved certificate asymmetry.

The findings do not reject local pull-tabbing, nested contextual rewriting or fresh derivation templates. Their distinct mechanisms and sustainable lifetime remain required. T071 and the architectural goal remain active.

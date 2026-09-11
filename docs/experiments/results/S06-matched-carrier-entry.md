# An existing explicit control contracts wait chains correctly

**The existing source-inferred carrier executor passes all 72 stream queries against independent answers and four existing execution paths.** No new executor is needed for this control.

The [registered gate](../registrations/S06-matched-carrier-entry.md) covers repeated and distinct calls, aliases, failure, consumption, three depths and both insertion orders. Each query checks the scalar evaluator against the source denotation, then checks Direct, sealed compiled, dependency graph, template graph and contracted compiled answers. All 360 executor searches exhaust within the registered cap. Answers are checked after producer disposal; prepared rules are reused across changing queries.

## What the implementations actually simplify

| Path | Source work removed | Work retained |
|---|---|---|
| Compiled inferred carriers | Repeated single-head decreasing `wait` steps, with known-prefix inspection | Ordinary terminal arbitration, choices and unknown tails; singleton admission prevents competing carriers from being collapsed together |
| Graph templates | Ground calls followed through residual choices; source expression edges shared within the derived plan | Producer operations and consuming calls remain live; derivation and a query-owned template table cost memory |
| Existing schema lowering | Direct construction of answers for one exact schema | Exact source/query admission checks |

These are code findings, not timing results. Carrier inference accepts `wait` and rejects the choice-bearing `stream` predicate in every tested source. Thus it is a useful partial simplification control, but has not yet matched the template's choice traversal. The schema-specific answer generator does not provide that missing source transformation.

## Scheduling is part of the next experiment

The existing scarce-token witness was rerun and passes: the scalar evaluator and ordinary demand execution give the token to the short caller; template contraction gives it to the long caller. Stopping derivation at a consuming call does not by itself preserve the timing of its arrival at the resource.

All 14 existing carrier tests pass, including unknown tails, multiple-carrier arbitration, cancellation, later bindings, rejection of observer rules and service beside divergence. Those controls give a concrete scheduling discipline to examine when extending simplification. No change to the reference interpreter or expected answers was made.

## Next action and competing priorities

T073 remains active. Next isolate choice-following from wait contraction in the existing template path and register a matched explicit comparison. Carry the scarce-token source into that gate: preserve its established winner when claiming operational correspondence, and measure any alternative scheduling semantics as a distinct language-design choice. Add sources for missing alias, unknown-input or resource boundaries rather than restricting the fixture portfolio to successful cases.

This remains more discriminating than coarser reuse-key tuning: the implementation inspection identifies source work that could be eliminated before recognition and retention. Direct solving remains a serious control, but the current exact-schema generator does not establish source-derived eligibility. Reassess these alternatives after the choice/scheduling gate. Register the matched lifecycle endpoints before comparative timing. Review package count is one; the broader goal remains active.

Reproduce:

```
cargo test -p chr-reuse --no-default-features --features carrier-contraction --test matched_carrier
cargo test -p chr-compiled --no-default-features --features carrier-contraction --test carriers
cargo test -p chr-direct-conditional --test suspended_source derivation_contraction_exposes_a_resource_scheduling_difference -- --exact
```

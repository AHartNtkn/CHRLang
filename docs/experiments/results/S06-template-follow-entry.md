# Call contraction changes resource arrival order

**Keeping templates but following zero source calls restores the scalar winner in the scarce-token witness.** The ordinary 64-call template control still gives the token to the long caller. This witness has no choices: the scheduling change is caused by contraction of calls, rather than traversal through choices.

The [registered ablation](../registrations/S06-template-follow-entry.md) now has a runnable control through `with_template_follow_limit(0)`. It keeps source-body instantiation, ground-edge sharing and the query-owned template table. The usual template entry uses 64; the independent node budget and table bound remain in force. A direct derivation test checks that limits zero, one and 64 follow exactly zero, one and two calls on a two-call source chain.

## Evidence

| Check | Result |
|---|---|
| Scarce resource, long and short callers | Zero-follow templates match the independent scalar short-caller winner; 64-follow templates retain the distinct long-caller answer |
| Stream sources, independent scalar and source denotation | All 72 queries pass across six executors, 432 completed searches; aliases, failure, consumption, depth and insertion order included |
| Suspended source suite | All 31 tests pass, with zero-follow added to the shared complete-answer helper and resource witness |
| Demand unit tests | 16 pass; three separately registered native measurement tests remain excluded from this unit invocation |
| Scoped Clippy | Demand library and matched-carrier test pass |

No timing or allocation advantage is established here. The runtime configuration representation changed from a boolean to an optional follow limit; prior frozen measurements continue to describe their frozen binaries. Fresh ownership qualification is required before cost comparisons of this representation.

## Architectural consequence

Separate three mechanisms in the next comparison: reusing an instantiated rule body, contracting deterministic calls, and following choices. The first can preserve the tested source results without contracting subsequent calls. The second changes arrival order in a nonconfluent resource program unless its scheduling is preserved or a different scheduling contract is deliberately compared. Merely leaving the eventual consuming call live does not solve that problem.

T073 next investigates preserving source service points during call contraction. Use the existing compiled carrier arbitration as a control, and test competing callers, unknown tails, failure and finite service beside divergence. Then isolate choice-following under the same explicit contract. This is an implementation experiment, not a request to restrict fixtures or select a language policy.

Coarser recognition remains a serious alternative, but the current witness exposes a concrete semantic cost that must be understood before those mechanisms can be composed. Direct solving likewise needs explicit resource-order treatment for these sources. Reassess at the service-point gate. Package count two; all broader research obligations remain active.

Reproduce:

```
cargo test -p chr-direct-choice --lib
cargo test -p chr-direct-conditional --test suspended_source
cargo test -p chr-reuse --no-default-features --features carrier-contraction --test matched_carrier
```

 The source tests, rather than a timing log, are the authoritative evidence for this correspondence experiment.

# E11 concrete projection and equation construction follow-up

Concrete projection preserves all 64 small-bound outcomes: 23 complete matches and 41 bounded-incomplete cases, with unchanged answer/model counts and cutoff predicates. Equality microstep pruning then enables the registered type-synthesis prefix to return the expected `k` program. The addition probes remain unresolved construction timeouts.

## Evidence

Concrete projection source freeze: `fb4106d`. [Registry follow-up](E11-registry-small-v2.jsonl) matches the preceding registry's semantic fields exactly. All three [application probes](E11-app-derived-v3.jsonl) still time out at 60 seconds during construction.

An interrupted 30-second cProfile diagnostic at addition T5,N27,O3,P3,U8 records 98,843,361 calls in 29.841 seconds. Two unification constructions account for 28.973 seconds, including 25.208 seconds in 23 occurs-check constructions. This incomplete profile supplies attribution, not a completed semantic result. It motivates proving equality microsteps inactive before constructing their effects, and omitting occurs checks for binding directions that simplify to false.

Microstep-pruning freeze: `3d36118`. All 27 Python tests pass. The [current registry run](E11-registry-small-v3.jsonl) preserves all 64 cases’ semantic fields, and the [type-prefix replay](E11-type-prefix-replay.jsonl) reproduces every non-time field exactly. In the [application follow-up](E11-app-derived-v4.jsonl), forward and relational addition still time out during construction. Type-synthesis-prefix returns one model and one answer, matching the expected `k`, with independent per-state replay. It reports a resource cutoff on other paths, so this is not exhaustive synthesis. Construction takes 22.64 diagnostic seconds, observation 2.61 seconds and boundary checks 6.01 seconds. The 74 construction-pruning queries prove 51 guards unreachable and consume 11.39 seconds, already included in construction.

The [four-transition addition-prefix follow-up](E11-add-prefix-projection.json) remains correctly transition-bounded, with no resource cutoff. At this prefix, no source equation executes. The profile now records 2,935,254 calls in 0.805 seconds, with allocation construction the largest contributor. This supports the projection diagnosis for that prefix; it does not establish whole-application speed or architecture superiority. Earlier profiling overlapped other experimental work, so these are diagnostic observations rather than confirmatory ratios.

## Resulting next question

The dense heap repeatedly translates handles, substitutions and constructor fields into term relations. Investigate a direct term-expression encoding as an independent E11 alternative: datatype constructors with explicit Hole(variable-ID) leaves and a substitution environment over logical variables. Retain algorithmic finite-tree unification, pure matching/guards, occurrence identities, history, fixed committed policy and independent witness replay. Do not encode source unification as arbitrary ground solver equality.

Keep transition, logical allocation, occurrence and pending bounds explicit, and document the equality/occurs service bounds before comparison. The existing dense-heap implementation remains an experimental control. This is a representation experiment, not adoption of a production architecture or different source semantics. E11 application coverage, equal-bound controls, increasing bounds and prefix reuse remain active.

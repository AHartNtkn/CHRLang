# The graphless word compiler and common-work controls pass their gates

A checked graphless compiler now reproduces the word source's complete answers, including resource observations, correlated outputs and duplicate alternatives. The ordinary-work contrast also passes across the existing executors. **The next comparison can distinguish source lowering from graph organization and test both common execution and its overhead.**

## What the direct compiler establishes

The [compiler](../../../research/chr-direct-choice/src/words.rs) accepts six exact ordered source schemas: binary, duplicate and nested choices, each with or without first-b rejection. Its query certificate accepts one or two closed-depth builds feeding one task and a distinct unknown token. It rejects additional source behavior or query obligations. This is a narrow source certificate, not a general eligibility analysis or another general CHR baseline.

The accepted source becomes a mixed-radix word iterator. Repeated task outputs reuse one constructed word; independent outputs use separate digit positions. Equal-valued arms retain separate digits. First-b rejection restricts the first letter's choices directly. Each result includes the complete `seen(H)` and `out(X,X,Y,Y,H,H)` residuals with joint identity. There is no term-choice graph or CHR matching loop in this execution path.

The lowering follows the gate's watch-before-use schedule. It does not claim that every permitted source schedule produces the same token observation. Closed-source admission is essential: an extra observer, constraint or rule could invalidate this direct interpretation and is not silently ignored.

The [prospective gate](../registrations/S03-word-lowering-gate.md) passes on all 28 original word configurations and 48 additional asymmetric/reversed-task configurations. The latter cover all six schemas with depth pairs (0,2), (1,2), (2,1), (2,0), reversing both task arguments and insertion order. Complete answers agree with independent mathematical expectations and the scalar, Global scan/index, Conditional and direct graph executors.

Cancellation followed by fresh-query reuse passes for each schema. Eighteen source mutations and 54 unsupported queries are rejected, including altered token observation, changed rule order, extra rules/constraints, unknown depths, shared token/output handles, missing tokens, duplicate tasks/tokens/builds and selected outputs. Unsupported input returns an error rather than an empty answer set.

## Ordinary work has favorable and adverse controls

The [common-work gate](../registrations/S03-common-work-gate.md) exercises the same unary work loop in three arrangements. Opaque work carries four independently chosen fields without demanding them. Early discrimination matches one of sixteen ground packs before entering the loop. No-choice work carries four unknowns. All query outputs remain jointly related to the result pack.

At depths 0, 1, 8 and 32, every engine agrees with the independently specified full answers: one nonground answer for no-choice, sixteen ground alternatives otherwise. These twelve configurations contribute 132 raw expected answers per executor. The gate provides an opportunity to measure common execution and counterpressure from early demand; it supplies no work-count or speedup result. Countdown elimination by a broader compiler remains a separate possible optimization.

## Validation and next run

Default, replay and metrics-off shared gates pass all fourteen test functions. The twelve graph kernel/equality tests and strict Clippy pass; formatting passes. Each process stayed inside 60 seconds. The initial word gate failed on the unimplemented compiler before behavior was added. [Commands and source hashes](s03-word-lowering/validation.json), [default](s03-word-lowering/default.log), [replay](s03-word-lowering/replay.log), [metrics-off](s03-word-lowering/off.log).

The [lifecycle pilot registration](../registrations/S03-lifecycle-pilot.md) fixes 30 cells, five ordinary-allocator timing repetitions and two separate allocation repetitions. It measures preparation, sixteen changing queries, execution with answer materialization, first-answer latency and disposal. It includes the graphless compiler on words, the admitted Active subset, and Global/Conditional/graph controls on ordinary work. No workload weighting or native-compilation superiority is inferred.

The runner and source/binary freeze remain to be completed before those 210 comparative processes run. No comparative timing has run in this tranche. T062 remains active; its next action is implementing and validating that registered runner, checking the new maximum word depths against the independent source oracle, then executing the bounded pilot. This advances the already selected whole-path comparison; it does not resolve the distinct S02/S06 alternatives or broader S03 mechanisms.

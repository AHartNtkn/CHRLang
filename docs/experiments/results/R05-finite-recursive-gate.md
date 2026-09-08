# Finite recursive relations: checked sealed execution

A source-derived certificate now admits finite constructor-controlled recursion with nonground payloads and results. Its direct equation loop preserves complete raw observations in the registered gate while using no CHR occurrence store, rule selector, propagation history or search frontier. T045 is complete. This establishes a meaningful native-compilation entry beyond finite-domain solving; it is not generated native code or a performance result.

## What is certified

[Registration](../registrations/R05-finite-recursive-gate.md) states the exact fragment and induction argument. [Implementation](../../../research/chr-compiled/src/recursive.rs) infers the predicate, arity, control position, base/step constructors and equation templates from two source rules. Names, variable IDs and addition-specific equations are not recognized specially. The base contains equations; the step contains equations followed by a single structurally decreasing call. Heads perform nonbinding constructor discrimination and bind distinct pattern variables only.

The public preparation owns reusable templates. Each supported query contains one call with a complete ground finite control spine. Unknown or malformed controls and surrounding source/query work return an explicit unsupported result; a supported equality conflict returns finite failure. The entire spine is checked before any equation runs, so an early contradiction cannot hide unsupported input further down the spine.

Each iteration gives body-local variables a fresh scope, uses direct head-argument bindings and executes the ordered finite-tree equations. Successful equations cannot change a ground control child. The next iteration therefore decreases the initial depth; the base terminates. A successful query has one complete raw answer with no residual constraints, jointly preserving all selected variable relationships; a failed query has none. Query outputs absent from the initial call are allocated in the query scope before application locals.

The finite-tree arena and bindings are candidate kernel services, shared with the existing compiled candidate. Correctness comparison uses the independent owned-syntax scalar evaluator, current Global/Indexed inferred specialization and separate mathematical expectations. The reference implementation is unchanged.

## Validation

Eight new test functions exercise the boundary. The independent matrix compares **168 combinations** of two recursive equation organizations, three depths, four payload shapes and seven result shapes against both scalar source execution and current Specialized. Separate mathematical checks assert the exact open result `wrap^n(X)`, zero-depth aliasing, occurs failure when result equals payload at positive depth, compatible structured results and incompatible constructors.

Focused tests cover preparation reuse, per-iteration fresh variables, output-only variables, renamed constructors/predicates/variable IDs, all three control positions, reversed rule order, different equation bodies and equations constraining the ground control child. Unsupported tests include unknown/partial/foreign controls, surrounding queries, observers, duplicate rule/output names, guards, kept heads, repeated head variables, base recursion, explicit failure, choice, work after a recursive call and nondecreasing calls. Existing R05 source-priority, external observation and divergence witnesses pass in the same package suite; they continue to explain why this certificate is sealed.

The initial API test failed before implementation as recorded in [RED](r05-finite-recursive-gate/initial-api-red.log). Root validation passes **53 package all-target tests in each feature mode**, including the new tests and existing contextual gates: [default](r05-finite-recursive-gate/default.log), [counter-free](r05-finite-recursive-gate/counter-free.log). Strict Clippy passes in [default](r05-finite-recursive-gate/clippy-default.log) and [counter-free](r05-finite-recursive-gate/clippy-counter-free.log) modes; [formatting](r05-finite-recursive-gate/fmt.log) passes. Independent read-only review found no blocking certificate, implementation or oracle defect and inspected the absence of general source execution in the direct loop.

## Responsibilities and limits

Preparation retains source equation templates and recursive argument templates. Runtime still instantiates terms, allocates fresh variables, solves finite-tree equations and exports complete answers. Each query owns its arena and bindings; preparation reuse does not retain query state. Execution is synchronous. Finite descent bounds iterations, not per-iteration term growth, unification time, export size or stack use. No timing or memory superiority is inferred from the smaller set of execution responsibilities.

This is a sufficient two-case, single-entry certificate. It does not establish arbitrary contextual inlining, multi-call composition, unknown-control synthesis, general termination analysis or a mandatory language restriction. The equation-template loop is interpreted specialized execution, so compiling it into native code still requires a distinct artifact and cost gate.

## Selection

T046 selects generated native execution and a credible compilation-lifecycle measurement entry for this certificate. It should compare the checked direct loop and corrected Specialized, validate complete answers, and make certificate construction, artifact generation, compiler work, startup/loading, repeated changed queries, observation and disposal visible. Register comparative runs only after the actual build/loading boundary is known. Shared runtime build costs and per-source compilation must be distinguished and reported without silently excluding cold prerequisites.

This has greater decision value than more serial-maintenance or worker tuning: the gate establishes a source class where internal CHR execution is unnecessary, while whether native generation earns its additional machinery and preparation remains unknown. R04's already certified finite fragment remains the lower-cost alternative if native artifact integration here requires broad infrastructure. No universal winner, language adoption or final goal closure follows.

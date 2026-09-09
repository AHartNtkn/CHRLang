# Source-derived prefix execution can reuse prepared rules

**A source-derived artifact now reuses lowered rules across changed query values, aliases and outputs.** Correctness checks pass for choices, fresh variables, consuming suffixes and finite service. Its total-cost benefit has not been measured.

The [registered gate](../registrations/S06-reusable-prefix-gate.md) follows the [selection review](S02-deduction-breadth-review.md). This tests whether repeated target preparation is necessary in the existing accepted fragment. It does not extend that fragment to recursive or resource-aware derivation compilation.

## What the artifact actually reuses

The compiler takes source rules and an ordered query signature: predicate names and arities. It constructs a formal query with one distinct variable per argument position, applies the existing source-derived prefix transformation once, and prepares the resulting target rules once. The artifact owns those prepared rules independently of the compiler.

Each new query supplies actual terms to one synthetic entry constraint. Argument positions may contain different constants, constructors or aliases. Output-only variables remain query variables. Starting execution validates the predicate signature and builds query state; it does not lower source or prepare target rules again. The implementation is in [pure_prefix.rs](../../../research/chr-compiled/src/pure_prefix.rs).

Distinct formal arguments preserve sharing supplied by the caller through actual terms. Fresh body variables remain local to source applications, and each execution gets a separate query scope. Synthetic entry names avoid both source predicates and predicates in the prepared query signature. The ordinary compiler's instantiation machinery handles these obligations; the artifact introduces no retained query answers or equality cache.

Predicate count, order or arity changes are rejected before execution. They require a separately prepared artifact. Different source rules likewise require new source analysis and preparation. This is reuse within a checked interface, not a cache keyed by an exact query value. No language-wide restriction is proposed by that experimental interface.

## Evidence and limits

The [tests](../../../research/chr-compiled/tests/pure_prefix.rs) compare complete answers against the independent scalar evaluator on original source. They exercise both scanning and indexed access, changed constructors/constants, aliased and independent arguments, repeated independent execution, duplicate resources, independent alternatives and fresh internal variables. Additional cases cover ground contradictions, changed source constants, empty queries with unconstrained outputs, synthetic-name collisions and rejected signature mismatches.

An execution trace shows a two-rule private chain becoming one synthetic-entry application. The produced answer still agrees with original-source evaluation. This proves the transformation eliminates those applications in execution; API names alone are not the evidence. The artifact executes after its source compiler is dropped. A separate witness delivers the finite answer beside a continuing recursive sibling and remains non-exhausted afterward.

All 118 compiled-package tests pass with default features and with counters disabled. Strict Clippy passes for the package's own targets. The initial missing-method failure, subsequent tests, validation hashes and commands are retained in the [receipt](s06-reusable-prefix-gate/validation.json) and adjacent logs. The reference evaluator was not changed.

These checks preserve the existing private acyclic prefix eligibility and Global source-order contract. They do not prove equivalence under arbitrary committed rule policies, service-latency equivalence, general recursion elimination or sustained artifact retention. The wrapper still allocates a query argument vector and target query state. A larger reusable entry may cost more to match or instantiate than a query-specialized entry.

## What the next cost package must distinguish

Compare ordinary execution, the existing per-query source-derived lowering and the reusable artifact on the same sources. Charge source analysis, artifact preparation, query setup, execution and full observation, answer/engine disposal and artifact disposal. Keep ordinary allocator timing separate from work counts and allocation diagnostics. Reuse prepared source across changing values; also include signature changes requiring new artifacts and one-shot cases that cannot amortize preparation.

Measure the favorable long-prefix/repeated-query regime and adverse short-prefix, large-argument and frequently changing-signature regimes. Preserve consuming suffixes and both successful and contradictory queries. The existing exact-schema control remains useful where its source contract applies; this artifact does not establish general elimination of the equality workloads from S02.

First obtain bounded exploratory lifecycle sizing, then prospectively freeze the consequential confirmatory comparison. If signature-specific preparation dominates, investigate that cause before attributing the result to compilation. T073 remains active for this cost comparison and for broader recursive/effectful lowering. This is the first package since the latest breadth review; the full architecture goal remains open.

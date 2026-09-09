# Source-derived counting eliminates the common traversal safely within a checked boundary

A source-derived transformation now eliminates the known common traversal depth while preserving complete answers across the qualified execution organizations. It also exposes a real scheduling boundary: shortening is unsafe when the entry activates a competing observer. Lifecycle costs remain unmeasured.

## What is inferred

The [compiler](../../../research/chr-compiled/src/resource_count.rs) identifies a recursive rule that keeps a nullary permit, consumes a unary traversal step and a nullary fuel occurrence, and posts the remaining traversal. It also identifies the terminal constructor and a single-entry rule whose successful body paths invoke the traversal once. Predicates, constructors and variable numbers are inferred from their relationships; the implementation does not recognize experiment names or use an evaluator.

A query must have one structurally matching entry, a ground common depth d, at least d fuel occurrences and a permit. It must contain no private traversal occurrence or other predicate with a potentially active source head. The compiler returns a query with d fuel occurrences consumed and that entry depth replaced by the inferred terminal constructor. Source rules, choices and terminal effects remain unchanged. Any branch-specific successor prefix still executes normally.

The returned query is an optional optimization artifact. Rejected queries remain ordinary source computations; neither their semantics nor the language is restricted by the compiler's current certificate. The [registration](../registrations/S10-resource-count-gate.md) records the comparison and the source-review amendment.

## Why the admitted transformation preserves answers

The removed steps bind no variables, publish no intermediate result and consume only a private fuel predicate. No other source head may read that fuel or the private traversal; no body may replenish fuel or produce another entry. The permit cannot be consumed. Thus the d certified fuel occurrences remain available until the traversal uses them, and their removal cannot deprive another computation.

Every successful entry path invokes the traversal once, with only a successor prefix over its depth parameter. Failed paths publish no answer. The depth parameter cannot escape into equations, residual posts or other head arguments. Replacing its ground common suffix by the terminal constructor and consuming the corresponding fuel preserves the remaining branch-specific depth and residual fuel. Terminal execution still creates the fresh variables and aliases.

Entry-body posts other than the traversal must be inert predicates with no source heads. This additional boundary prevents an auxiliary computation from racing a terminal that becomes enabled sooner. These are sufficient conditions for this private transformation, not a theorem that arbitrary resource-aware lowering commutes with contextual execution. Broader useful fragments require further arguments and tests.

## Independent executable evidence

The main [composition test](../../../research/chr-direct-conditional/tests/resource_count_composition.rs) covers common, independent and early-failure sources; choices 0/1/3; depths 0/1/4/16; two changed queries; and systematic predicate/constructor/variable renaming. That is 144 configurations per build. Each checks original and transformed queries against independently constructed full observations through the scalar and seven candidate configurations: 2304 finite executions per build.

Compiled Scan, Indexed, inferred-specialized Scan, contextual eager, contextual demand, shared persistent contextual and direct conditional all agree. Checks include source-choice multiplicity, residual resources, output/residual aliases and fresh terminal variables. The reference interpreter is unchanged.

A separate traced conditional check establishes the actual removed work at three choices and common depth 16:

| Source | Before counting, physical conditional steps | After counting |
|---|---:|---:|
| Common continuation | 16 | 0 |
| Independent branch-specific traversal | 140 | 12 |
| Early failure with common continuation | 16 | 0 |

The before counts come from the [post-choice mechanism gate](S10-post-choice-work.md); the new test asserts the after counts and complete answers. The independent control retains its branch-specific suffix rather than fabricating completed results.

## Rejection is supported by concrete counterexamples

The [boundary tests](../../../research/chr-compiled/tests/resource_count.rs) reject insufficient common fuel, unknown or malformed depth, absent permit, missing/repeated entry, an unestablished entry pattern, an initial private traversal, competing entry heads, private resource readers/writers, permit consumption, escaping depth and successful paths with zero or multiple traversals.

Two executable counterexamples explain why key restrictions are necessary:

- A fuel observer produces four residual markers before the original traversal. An unchecked transformed query loses those markers. Source inference rejects that observer.
- An auxiliary entry post can be consumed by either a generic observer or a terminal-shaped observer. Shortening enables the terminal-shaped observer first and changes the residual answer. Source inference now rejects active auxiliary entry effects.

Insufficient branch-specific fuel has a different outcome. When the common depth is available, counting remains valid even if a longer branch later suspends. An independent expected-answer construction checks all eight branches, including their remaining unary traversal and unbound result. Scalar, Scan, eager/demand contextual and conditional execution preserve those residuals before and after counting.

## Validation and remaining costs

The compiler interface initially fails to compile before implementation. Final compiled library/boundary/fusion checks pass 15 tests in each feature configuration; the three composition/work/suspension tests also pass in each. Scoped strict Clippy passes. [RED](s10-resource-count/red.log), [compiled feature-off](s10-resource-count/compiled-tests.log), [compiled default](s10-resource-count/compiled-default.log), [composition feature-off](s10-resource-count/composition-off.log), [composition default](s10-resource-count/composition-default.log), [compiler Clippy](s10-resource-count/clippy-compiled.log), [composition Clippy](s10-resource-count/clippy-composition.log).

No comparative allocation or timing runs were performed for this transformation. Inference retains a certificate, each query requires matching/counting, and the current implementation clones the query before editing its resources. Those costs can matter at zero or shallow depth. Eligibility rejection, artifact ownership and ordinary execution must participate in the next lifecycle comparison; eliminated source steps are not free preparation.

## Next decision

T078 next registers a paired lifecycle comparison of source counting versus ordinary execution across the same credible organizations. Include zero work, shallow/deep common work, branch-specific suffixes, changed queries and rejected certificates. Separate source inference, query certification/transformation, reusable rule preparation, answer delivery, cancellation and every owner disposal. Keep work/allocation diagnostics separate from ordinary timing and preserve the previous source controls.

This comparison has higher immediate decision value than selective conditional discovery: it tests whether the repeated work motivating that repair survives a validated source transformation. Conditional discovery, resumable contextual matching, wider lowering, compilation, sustained lifetimes and held-out architectural evaluation remain unresolved obligations. No architecture is selected, and the research goal remains active.

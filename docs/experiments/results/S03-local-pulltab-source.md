# Local pull-tabbing: a source gate and a scheduling defect

The demand executor now performs a local pull-tab rewrite on directly demanded choice arguments. The bounded source tests pass after repairing an extra scheduling step that changed resource competition. This establishes a working mechanism, not an efficiency advantage or a general graph architecture.

## What actually runs

The [prospective gate](S03-local-pulltab-entry.md#first-registered-correctness-gate) specifies the operation. When a call demands a direct argument containing an unresolved named choice, the executor installs that same choice over two copied calls. Each copy substitutes the corresponding argument and has an execution obligation conditional on its choice arm. Other arguments and the logical output identity remain shared. The operation creates no new choice label.

The original call's result edge points to the lifted graph. Its copied calls can execute, fail or remain residual independently in their supported contexts. Already computed producer results and aliases may lead to the direct argument. Choices inside constructor children and choices required by resource settlement still use the existing context-splitting operation. Those cases are not evidence for a general local rewrite implementation.

The experimental `Prepared::with_pull_tabs()` option is independent of the two cache-validity policies. Ordinary demand execution remains the control. This allows paired investigation of graph rewriting without silently attributing cache-policy differences to it.

## Evidence and the consequential repair

The structural test initially failed because the executor split the context without producing a choice over copied calls. It now checks the actual result graph, substituted arguments, retained output identity, unchanged choice-birth count and conditional obligations. It does not infer mechanism execution from a strategy name or an answer alone.

The contested-resource probe found an implementation defect. With two requests demanding the same choice and one token, an extra progress event changed which request consumed the token. The rewrite is administrative, so it now reports the same split at the same service point as the control. The graph still changes; the additional scheduling opportunity does not occur. The original paired competition test now passes without changing its expected comparison.

The complete-answer helper compares the independent scalar evaluator, compiled scanning/indexing, direct graph execution, ordinary demand execution and pull-tabbing under both cache-validity policies. The reference evaluator was not modified. Dedicated new sources cover two correlated consuming calls with two distinct tokens, a nonmatching copied call with exactly one residual occurrence, and contested consumption against the demand control. Scalar declaration priority and demand service remain distinct policies on nonconfluent programs; this gate does not make them interchangeable.

The dedicated progress witness publishes exactly one finite answer beside a continuing branch within 128 ticks. Adding an off-output failure terminates without publishing an answer. Both ordinary demand and pull-tab execution satisfy these endpoints. Existing service tests that construct their own runner remain tests of their configured strategy; the entire test-file count is not a count of new pull-tab witnesses.

## Validation

Commands run from the repository root:

```sh
cargo test -p chr-direct-choice --lib
cargo test -p chr-direct-conditional --test suspended_source
cargo test -p chr-direct-choice -p chr-direct-conditional --no-default-features
cargo clippy -p chr-direct-choice -p chr-direct-conditional --tests -- -D warnings
```

The structural test and all 26 suspended-source tests pass. The package-wide test run and Clippy pass. `--no-default-features` here describes the package test invocation, not a claim that all transitive diagnostic features are disabled. No comparative timings or allocation measurements were run for this mechanism.

## Architectural consequence and next comparison

A local rewrite can coexist with source obligations and contextual resource claims in this bounded fragment. It introduces copied call nodes, conditional obligations and a cached replacement edge. A credible cost comparison must charge these owners as well as any repeated demand traversal it avoids. Copying the call context alone does not establish useful sharing across fresh derivations.

The next package should attribute actual work before registering broad timings. Compare ordinary demand and local rewriting on repeated use of one demanded choice, independent choices, opaque arguments and early discrimination. Check copied nodes, expansions and repeated argument traversal outside primary timing; validate complete answers independently. Determine whether this operation avoids consequential work or only changes its representation, and identify what a broader contextual mechanism would add.

This bounded attribution is selected ahead of T073's reusable lowered-query artifacts. Artifact reuse could improve the measured compiler's preparation costs; attribution now can determine whether the newly implemented graph operation merits a lifecycle pilot or needs a distinct mechanism to realize its proposed benefit. Reconsider that choice after this package. T072's broader integration, T073's lowering, fresh derivation templates, nested demand, broader aliases and sustained graph reclamation remain required. No stage or goal is closed by this gate.

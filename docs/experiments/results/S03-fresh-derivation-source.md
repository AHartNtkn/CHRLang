# Fresh derivation plans avoid repeated source expansion without sharing identities

A query-local cache can reuse an input-dependent source derivation while creating fresh unknowns and choices for each application. The source gate demonstrates avoided multi-rule expansion and preserves complete answers on the equivalent witnesses. It also exposes a committed-scheduling difference on a contested resource, so this is not a transparent optimization under every fixed source policy.

No comparative timing or allocation result is claimed. The next question is whether constructing, looking up and instantiating the cached plan repays its costs.

## What is reused

Existing prepared plans encode one rule body. The new optional `with_derivation_templates()` mechanism follows known source calls and caches the resulting residual plan under a predicate and closed constructor arguments. For example, following `build(s(s(z)),R)` through recursive steps can produce a plan that creates a choice between two results with fresh unknowns. The plan contains source operations and local variable slots, not runtime unknown IDs, choice labels or resource claims.

Each distinct application instantiates the plan with a new local environment. Repeated observation of one application still uses that application's existing result. These two forms of reuse remain separate. The cache is owned by the query; prepared rules are already reusable independently of it.

The structural test initially retained 18 `build` result records for two depth-eight calls. With derivation reuse it retains two, one template and eight followed source calls in that template. The diagnostic build records a cache hit on the second application. Both independently born choices still produce the four-answer product. This proves that multi-rule source expansion was avoided after template construction; it does not count construction as free work.

## Relationship to compilation

This mechanism is bounded, memoized partial evaluation of source calls. It therefore also belongs to S06's runtime-specialization investigation. The same residual operations have direct source forms: a value becomes an output equation, a call remains a source call with its output argument, a choice becomes source OR, and a producer/effect prefix remains its source conjunction. Local slots must be freshened on each application in either representation.

That explains the semantic overlap with source-driven specialization. It does not prove equal costs across backends or account for every derivation-net proposal. It should not be counted as two independently tested architectures merely because it can be described as graph derivation reuse and runtime compilation.

## Boundaries and identity evidence

The [prospective registration](../registrations/S03-fresh-derivation.md) restricts cache keys to already closed constructor nodes. Key discovery does not force producers, choose alternatives or serialize unknown runtime identities. It is bounded at 4,096 nodes. Unknown/context-dependent inputs remain ordinary calls in this first implementation.

The compiler follows at most 64 source calls. It stops at non-ground calls, resource-matching heads and producer/effect prefixes, preserving the corresponding live source operation. It does not claim resources or create choices while compiling. Resource-headed roots also remain outside template eligibility. Cached plans alpha-rename locals from each entered clause; their instantiated graphs retain each application's identity.

Independent hand/scalar checks distinguish one call used twice from two equal-input calls. Deterministic results with fresh unknowns preserve the required joint aliases, and nondeterministic results preserve correlation versus independent products. Tests cover depths 0, 1, 8 and 65, including continuation beyond the call-following bound. Resource-boundary tests require real token consumption and retain separate residual occurrences when tokens are absent. Producer-prefix failure remains an active obligation even when its value is not observed.

A finite-sibling/off-output-failure test also exercises templates with the existing dependency and local-lifting policies. It publishes the finite answer beside continuing recursion, or terminates without an answer when an independent failure is present. This is bounded progress evidence, not an unbounded-service proof for arbitrary inputs.

## Why a construction-size bound is necessary

A source can double a ground accumulator on each recursive call. Limiting the number of followed calls does not prevent exponential materialization of a tree-shaped template. Construction therefore also has a 16,384-node budget for copied terms/plans and substitutions. When entering another clause would exceed it, the residual call survives for ordinary execution; failure to construct the initial body declines that cache entry.

The duplicating-accumulator test inspects a retained live continuation before completion, then checks the complete depth-12 result against independent source execution. A separate test fills the 64-entry query cache with distinct requests and verifies correct results for both an uncached request and a repeated cached request. These limits constrain this implementation's retention, not the source computation's meaning.

A shared or parameterized template representation could avoid some materialization. That remains a consequential alternative if these construction costs matter; the tree representation's limit is not an impossibility result for derivation reuse.

## The scheduling counterexample

Two requests, `build(s^8(z),X)` and `build(z,Y)`, eventually request one token. Ordinary demand and the scalar control give the token to the short request. Contracting the long request's derivation makes its consuming continuation available earlier, and the template mode gives it the token instead. The test verifies the different complete outputs and residual occurrence; it does not weaken them into an unordered claim that either comparison is equivalent.

The long-request outcome has a permitted source derivation: execute its eight recursive steps, replace its base call by `take(X)`, consume the token with that call, then reduce the other base call to the unmatched `take(Y)`. Each step acts on an enabled occurrence, and no resource is fabricated or consumed twice. The alternative order gives the other outcome. This is the distinction between a permitted committed schedule and a fixed reference schedule established by the [S00 contract](S00-contracts-and-candidates.md).

Consequently, causal timing comparisons must use answer-equivalent sources or match the required policy. If a language promises a specific competing-application order, this transformation needs additional scheduling constraints. The experiment has not adopted a language policy, and no timing ratio is computed for this counterexample.

## Validation and next selection

The [package tests](s03-fresh-derivation/tests.log), [counter-free structural tests](s03-fresh-derivation/counter-free.log) and [Clippy receipt](s03-fresh-derivation/clippy.log) pass. There are seven structural unit tests and 31 suspended-source tests; the complete-answer helper includes templates under all three validity policies. The test count includes existing controls and rejection cases, not 31 new template mechanisms. Reference code is unchanged. [Source hashes and validation commands](s03-fresh-derivation/freeze.json) identify this gate.

Select a bounded work/cost registration next, separating first construction, hits, fresh instantiation and disposal. Include repeated versus distinct ground inputs, short versus substantive derivations, deterministic versus independent-choice results, and the accumulator-growth boundary. Compare the prepared ordinary executor and credible compiled/specialized controls. Charge owned key construction and query-cache retention; do not infer a runtime gain from fewer retained call records.

This is currently more useful than another local-lifting timing refinement: it tests the cost of an implemented multi-rule reuse mechanism with an independent identity gate. Broader integration and reusable lowered-query artifacts remain ready alternatives and return at this package boundary. Unknown-input templates, effectful/contextual derivations, general derivation graphs and sustained lifetime remain required. T071 and the architecture goal remain active.

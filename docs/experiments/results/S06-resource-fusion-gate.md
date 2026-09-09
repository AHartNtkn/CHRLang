# Counted resources permit source-derived effectful fusion

A private producer/consumer boundary can be fused when a source check and per-query resource counts guarantee the consumer can run. A trace shows six source applications becoming three, with identical complete answers. Insufficient resources provide a concrete counterexample, so the optimization is conditional; its total cost is still unmeasured.

## What differs from pure-prefix inlining

The existing pure-prefix transformation expands private equations and choices while leaving ordinary resource selection intact. This experiment composes consuming heads across two rules: producer inputs and the consumer's additional resources are selected by one fused rule, and the consumer's source body executes directly. Preparation analyzes syntax and resource counts; it does not evaluate the original query to obtain its answer.

The compiler infers a private intermediate predicate from source. Its sole producer consumes its heads and posts that intermediate. An earlier source rule consumes the intermediate and additional resources. The intermediate carries bound variables, while the consumer body may contain equations, output constraints, fresh variables and finite alternatives. Predicate names and variable numbers are not fixed by the implementation.

The emitted rule keeps the producer's position relative to remaining rules, appends the additional consuming heads, and uses capture-free substitution for the consumer body. Source-order Global execution is the supported scheduling contract. The transformation currently fuses one eligible pair per inference call; broader resource derivations and effectful recursion remain separate work.

## Why availability is a semantic requirement

Consider a producer that consumes a seed and fuel, then posts an intermediate. Its consumer needs two permits. If no permits exist, the original program leaves the intermediate. A fused rule requiring seed, fuel and both permits cannot fire and leaves seed and fuel instead. The test demonstrates these different complete answers, rather than treating the unavailable consumer as failure or an empty answer set.

For each changed query, the certificate groups controlled occurrences by ground key and predicate/arity. The maximum producer firings are the minimum available producer-head counts divided by their multiplicities. Each additional consumer resource must cover that bound multiplied by its own head multiplicity. Spare resources are preserved, and occurrences remain distinct even when values are equal. Resources belonging to another key do not satisfy the bound.

The source checks make these counts sufficient. Controlled resource predicates have no other head owners or body producers. Producer heads share the key and otherwise have distinct variable parameters; additional consumer resources use only the key. The private intermediate is not initially present. Unknown controlled keys, external intermediate observers, competing resource owners and body replenishment are rejected by this certificate. These are sufficient implemented conditions, not necessary language restrictions.

## Why fusion preserves the scheduling boundary in this fragment

The producer's single post creates no binding and enables only its private consumer. That consumer precedes the producer, and the count certificate guarantees its additional resources. No other rule can consume or observe the controlled occurrences. Appending the consumer's resource heads therefore does not exclude an otherwise selectable producer tuple or change which available resource tuple follows it.

The consumer's effects still complete before ordinary rule selection resumes. Ground keys cannot change under consistent equality; payload aliases and final-output observers remain ordinary effects. Fresh rule variables are renamed away from producer parameters. Internal occurrence-number gaps may differ, but relative occurrence order and observable aliases are preserved in the checked source behavior. This is the reasoning behind the sufficient conditions, not a formal compiler proof.

## Evidence

| Check | Result |
|---|---|
| Count/renaming matrix | 360 query configurations: 280 admitted and 80 rejected for insufficient resources. Two predicate/variable renamings, distinct/duplicate alternatives, shared/distinct payload variables and resource multiplicities are covered. |
| Admitted answers | Independent scalar evaluation agrees before/after fusion. Generic and source-specialized Scan agree on both programs for all 280 admitted configurations. |
| Multiple keys and late aliases | Two query-order cases preserve separate resource groups; two late-binding cases preserve the filtered alternatives. A different-key resource supply is explicitly rejected. |
| Actual elimination | Three producer/consumer pairs require six traced applications originally and three after fusion. Full independent answers and three residual outputs agree. |
| Adverse boundaries | Insufficient resources change the residual state if the certificate is bypassed. Nonground keys, initial intermediates, observers, competing owners and incompatible source order are rejected. |
| Hygiene and bounds | Consumer fresh variables deliberately overlap producer variable numbers in the source. Constructor-dependent producer heads, richer resource patterns, replenishment and exhausted variable identities have explicit checks. |

The [tests](../../../research/chr-compiled/tests/resource_fusion.rs), [compiler](../../../research/chr-compiled/src/resource_fusion.rs) and [registration](../registrations/S06-resource-fusion-gate.md) retain the source contract. Default and counter-free builds pass the new tests, existing pure-prefix tests and library tests; scoped strict Clippy passes. [Default results](s06-resource-fusion-gate/default.log), [counter-free results](s06-resource-fusion-gate/counter-free.log) and [lint output](s06-resource-fusion-gate/clippy.log) provide validation receipts. Reference-interpreter code is unchanged.

## Next decision: charge inference and query certification

Select a bounded complete-lifecycle comparison before extending this fragment. Compare original and fused sources under competent generic and specialized Scan, retain Indexed as an access contrast, and reuse prepared rules across changed certified queries. Charge source inference, emitted-rule preparation, per-query count checking, input/setup, execution, owned observation and disposal. Include zero/one/many producer firings, spare resources, shared aliases and duplicate alternatives. Unsupported queries must remain explicitly outside the certified artifact's scope.

This can change whether the transformation is worth using: fewer applications may still require more head matching, resource counting or preparation than ordinary execution. No speed or allocation result follows from the trace. Compilation costs need their own credible accounting before any compilation-inclusive claim.

The strongest alternatives remain native cost measurement for broader integrated matching, adaptive reunion and coherent architecture composition. The new source certificate makes one bounded lifecycle package informative; reconsider those alternatives after it instead of automatically broadening this compiler. General resource solving, effects outside the certificate, language tradeoffs and held-out complete architectures remain unresolved. The research goal stays active.

# Retaining equality-stable candidates avoids repeated discovery

**The inferred rule property preserves the tested source behavior and removes most discovery allocation in the broad witness. It also introduces preparation and retention costs in less favorable cases.** Ordinary-allocator timing is next; diagnostic clock readings do not establish a speedup.

The [registered gate](../registrations/S02-stable-candidates.md) qualifies unguarded flat heads whose pattern variables are distinct across the entire head. Constructor patterns, repeated variables and guards retain equality invalidation. Existing arrival invalidation and occurrence-claim validation remain active. This is an inferred optimization condition, not a restriction on accepted source programs.

## Why the retained candidates can remain valid

For the qualified heads, equality does not change which occurrence tuples match. Matches are ordered by kept and removed occurrence identities before bindings, so representative changes do not change their priority. Every tuple has one binding assignment. Cached values still name their equality classes; body unification, posting, constructor construction and observation resolve their current representatives.

Consumption can invalidate a tuple, so ordinary claim validation still checks all kept/removed identities. New occurrences invalidate the corresponding predicate readers. Propagation history remains keyed by rule and occurrence tuple. These responsibilities are necessary to the result; preserving a candidate list without them would not establish the same behavior.

The source gate covers eight explicit adversarial programs in both input orders and three schedules, checked against the independent scalar evaluator. It includes a cached tuple whose representative was retired by a prior merge, guards that become true, constructor and repeated-variable matches enabled by equality, competing claims, insertion, propagation history and disjunctive forks. Existing source/store/library tests also pass in candidate and conservative builds.

## Complete ownership and work

The144-process matrix covers36 configurations with metered/profile builds, exact repetitions, independent full answers and complete disposal. Root-relative heap records and advance counts agree between diagnostic/control builds. All exclusive allocation sums are exact. The12 compiled configurations remain unchanged.

Across24 relational configurations:

| Complete lifecycle quantity | Lower | Equal | Higher |
|---|---:|---:|---:|
| Requested bytes |16|0|8|
| Peak live bytes above root |4|0|20|

The preparation analysis and retained candidates are charged. These counts are experimental configurations, not workload frequencies.

For vector incidence, borrowed cycle traversal, full settlement, width128 reversed broad sources and two changing queries:

| Quantity | Conservative invalidation | Inferred stable candidates |
|---|---:|---:|
| Discovery scopes |261|4|
| Discovery requested bytes |7,919,460|310,584|
| Complete requested bytes |9,881,101|2,272,459|
| Peak live bytes above root |354,669|353,789|

The same discovery reduction occurs under bounded settlement, but its total remains9,992,995bytes because readiness traversal is still substantial. This optimization does not solve that separate cost.

In the shared successful depth64 witness, discovery calls fall only14→10. Complete full-settlement allocation falls535,343→535,074bytes, while peak rises92,252→92,351. Admission cancellation performs no discovery: full-settlement allocation instead rises653,977→654,211bytes from preparation overhead. These are real adverse controls, not waived costs.

## Decision

Proceed to a registered, closely paired ordinary-allocator comparison of conservative and inferred invalidation, including compiled execution, broad discovery, guard-heavy equality and cancellation. Keep the same borrowed traversal and incidence representation within each contrast, charge the full lifecycle, and confirm consequential timing results before a policy choice.

This is the strongest immediate follow-up because the measured7.61MB discovery saving could alter the compiled gap, while the small/negative cases could reverse a general policy. Readiness traversal, broader constructor organization, graph memo attribution and solving remain required. Reconsider them at the timing result. T072 remains active, package count is two since the portfolio review, and the research goal remains unfinished.

[Per-cell comparison](s02-stable-candidates/comparison.json) · [Raw evidence and source/binary freeze](s02-stable-candidates/) · [Runnable auditor](../../../research/chr-relational/experiments/execution_attribution.py) · [Adversarial source checks](../../../research/chr-relational/tests/source.rs)

Source/store/library tests and candidate/control Clippy pass. The implementation adds a per-rule preparation bit and uses it at the existing equality/reconsideration invalidation sites; no new execution backend is introduced.

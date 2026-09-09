# Suspended source calls: quiescence and residual occurrence gate

Extend the suspended source evaluator so unmatched nonbinding calls remain source occurrences in a complete answer. This is a source-semantics gate; no comparative timing is authorized by this registration.

Each call has an occurrence identity and an output logical identity. Application-result records justify consumption of that occurrence only in their recorded choice context. Tail-call replacement must preserve the output identity while replacing the consumed source occurrence with the body occurrence. All created calls, including tail calls and inactive choice-arm calls, require explicit activation support. Do not memoize an unmatched call as permanently failed.

A publication attempt must discover no further source application or unresolved source choice. Reify all still-live occurrences and query outputs jointly, preserving aliases and multiplicity. An unmatched known constructor is residual, not failure; a source Fail is failure, not a residual. Existing producer and query acyclicity checks remain in force.

Before implementation test: unknown-input residuals, ground nonmatches, tail replacement, shared residual/output aliases, two distinct equal-input occurrences, one branch matching and the other residual, a later producer enabling a formerly unresolved demand, and a residual alongside an independent failure. Compare full answers with the scalar oracle and hand expectations; include the existing compiled/global and direct graph controls on these new cases. Existing finite-sibling and independent-failure gates must continue passing.

Use at most 100,000 candidate ticks and 200,000 scalar steps per finite input; at most 60 seconds per test process. Run default and package-default-metrics-disabled source gates. This work does not admit multihead consuming rules or establish general rule-competition scheduling. Those remain the next source integration obligations in T071.

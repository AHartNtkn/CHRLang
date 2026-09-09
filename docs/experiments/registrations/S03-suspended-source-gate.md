# Suspended source applications: first executable correspondence gate

Implement an experimental demand evaluator for a checked equation-producing CHR fragment. This is implementation and semantic evidence only; it does not complete T071's consuming-source obligation or establish a performance ranking.

The compiler accepts single removed heads with linear input patterns and a distinct variable output. Bodies may assign that output, tail-call a checked predicate, choose between bodies, fail, or introduce fresh call outputs followed by an output assignment. Reject guards, kept heads, multihead effects, extra output writers and unsupported bodies explicitly. Unknown inputs may be passed opaquely; a demanded unresolved constructor must report unfinished execution rather than a complete answer.

Applications retain result nodes per compatible choice context. Distinct applications allocate fresh local identities. All query calls and body producer calls remain completion obligations, including calls whose values do not reach an output. Choice labels are born during application expansion, not from printed input values. Search services tasks round-robin; one source-call expansion yields progress. The first implementation may use conservative whole-context result keys and recursive demand traversal; these costs and limitations are not architectural lower bounds.

Check hand-derived answers and the independent scalar source oracle on recursive opaque work (depths 0, 1, 8), repeated demands versus independent calls, fresh locals, nested/duplicate choices, unused failing producers, and a finite sibling next to a continuing call. Test compiler rejection of source effects and unsafe multiple writers. Cap finite runs at 100,000 ticks and process commands at 60 seconds. Require exact joint aliases and raw multiplicity. Do not collect comparative timings before later sizing and registration.

Use a memoized and reevaluating configuration only if their source identities and choices remain identical: absence of result caching must never mean allocating a new source birth for every demand. The first gate need not expose such a control until it meets that requirement.

After the gate, integrate occurrence effects and general source obligations or document a consequential obstruction. The independent scalar reference remains unchanged. The direct graph and compiled source executors remain controls; this module is not a new production baseline.

## Implementation-stage extensions

Before any comparative measurement, extend the semantic gate with discovered counterexamples: branch-local producer aliases, overlapping source-rule competition, cyclic producer dependencies, independently serviceable failure beside an ongoing output, and constructor-enabled refutation before another producer finishes. These are diagnostic/regression tests, not post-hoc performance hypotheses. The compiler now rejects overlapping input clauses and cyclic producer dependencies; those are limitations of this certificate, not proposed language restrictions. Source obligations are serviced independently of value demand. An unresolved task reports `Stuck` and remains serviceable; it cannot produce an empty residual as a substitute for its pending call.

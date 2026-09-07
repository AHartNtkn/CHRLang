# A3 maintained matching: resource sizing

Eleven of24 registered cells finish with checked full answers;13 reach the
10,000,000-action bound. No child reaches the wall/address-space bound, and no
semantic mismatch is recorded. The [recording audit](E09-maintained-sizing-v1-audit.json)
checks the full matrix and matching successful observations. Resource-limited
cells remain unresolved, not successful finite executions or rejected designs.

[Registration](../registrations/E09-maintained-sizing.md), [manifest](E09-maintained-sizing-v1-manifest.json),
[raw outcomes](E09-maintained-sizing-v1.jsonl). This is exploratory sizing;
wall diagnostics include analytic oracle work and are not timing comparisons.

| Join | N / alternatives / rounds | Prefix actions | Full invalidation | Selective actions |
|---|---|---:|---:|---:|
| Keep |4 /1 /1|50,420|92,649|23,988|
| Keep |16 /2 /4|8,204,159|work bound|1,932,485|
| Keep |64 /1 /1|work bound|work bound|8,711,356|
| Keep |64 /8 /4|work bound|work bound|work bound|
| Consume |4 /1 /1|34,217|69,530|20,473|
| Consume |16 /2 /4|work bound|work bound|4,134,517|
| Consume |64 /1 /1|work bound|work bound|6,535,009|
| Consume |64 /8 /4|work bound|work bound|work bound|

Successful matched cells agree on source steps and full output/residual multisets.
The larger gate uses analytic complete-answer expectations, supported by the
smaller independent per-transition gate; it does not claim transition replay at N64.
The source producer itself executes more work in consuming repeated-round cases.

## What the counts distinguish

For kept N16/B2/R4, prefix recomputation spends4,304,975 actions resolving terms
and1,434,945 matching. Selective maintenance instead makes7,803 physical matching
calls, but visits521,493 candidate extensions and513,690 retained tests.
For kept N64/B1/R1, these latter two counts are3,477,692 and3,443,277: together
roughly79% of its8,711,356 instrumented actions. Consuming N64 has the same pattern:
2,501,300 candidate visits and2,473,309 reuses, about76% of6,535,009 actions.
These percentages characterize this instrumentation, not percentages of runtime.

Source inspection explains the pattern. Each changed snapshot traverses retained
prefixes, loops over each appropriate predicate pool and tests whether the extension
is already cached. Failed extensions are retained too. Selective invalidation
avoids most structural rematching but still repeatedly discovers that cached tests
need no work. At N64, completed selective runs for both join variants peak at
17,171 retained tests per context. No heap-byte claim follows.

This gives a concrete next contrast: maintain update-driven extension subscriptions
or join-key indexes so an unchanged cached test need not be revisited. Compare
against this selective control, and against seed-ordered/lazy recomputation where
retention is adverse. The current prefix-only delta approach has not tested those
architectures. Broad aliases, middle-head arrivals and consumed descendants must
remain part of the correctness gate.

## Open resource questions

The13 censored cells do not yet provide complete action distributions. The next
resource diagnostic should checkpoint source progress and counters at cutoff,
then use those data to select justified larger bounds or an indexed-discovery
contrast. Raising a limit alone does not explain the repeated work. Likewise,
small selective savings do not justify omitting large or consuming cases from
future comparisons. Actual preparation, heap retention, completion cleanup,
observation, hash-seed sensitivity and runtime need separate cost measurements.

A3 remains open with specific feasible follow-ups. Independent A6 graph observation
and A1 constructor-relational source integration do not depend on resolving these
scaling costs.

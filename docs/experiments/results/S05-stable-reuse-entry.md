# Next: test operation reuse over stable shared identities

S05 will test whether explicit execution can reuse substantive equality successes and failures without serializing owned request trees. This could change the comparison with direct sharing. Earlier owned-term caches and common-failure sharing results do not settle it.

## Why this contrast comes next

The [paired S04 correction](S04-matcher-paired-cost.md) establishes that matching copies caused much of mutation's cost and identifies a smaller long-alias regression. Corrected replay policies and larger sparse-mutation stores remain the strongest ready restoration alternative. They could change storage choice, but another S04 refinement would postpone a distinct, still untested way to obtain shared-work savings. S05 therefore takes the next position in the breadth cycle; S04's remaining obligations stay in the coverage map.

This is a prioritization judgment. Neither anticipated support for an incumbent nor a requirement for a supplied application determines the selection.

## What the current code establishes

`research/chr-reuse/src/lib.rs` canonicalizes owned syntax terms into a `BTreeMap` key and replays owned term substitutions. Its costs include structural key construction; it is a useful correctness/control implementation, not the proposed stable-identity design.

`research/chr-persistent/src/terms.rs` exposes compact variable and arena-node handles, immutable constructor nodes and transactional unification. Arena cloning preserves a prefix but independently appended nodes can reuse numeric indices in different descendants. Raw numeric equality therefore cannot authorize cross-branch cache reuse. Closed identical nodes already receive cheap identity equality in the ordinary kernel; a cache must demonstrate additional avoided work rather than claim that existing shortcut as a gain.

## First implementation and correctness obligations

Compare a query-owned operation cache over stable constructor identities with ordinary unification and the relevant direct-sharing control. First choose an actual identity owner that remains valid across the compared contexts. Candidate mechanisms are a common immutable allocation owner or a verified inherited prefix with explicit branch-local identity. Record what each requires; do not add owned structural serialization solely to reuse the previous interface.

Make binding validity part of the key or proof. Contrast a conservative exact binding-context identity with a dependency check when they admit different useful reuse. A successful entry needs a sound binding replay and ordinary eligibility wake-ups; a failure entry needs valid premises and must not conflate resource consumption, source failure and unfinished computation. Cache eviction must preserve answers and make retained ownership explicit.

Gate repeated success/clash before and after discrimination, changed bindings, unrelated binding changes, sibling arenas with colliding numeric indices, fresh variables, failed speculative binding, repeated raw alternatives and off-output failure. Include an independently computed complete answer or failure for every source witness. A cache that recognizes equal equations does not thereby authorize merging branches or removing source effects.

Only after this gate, register full lifecycle costs on substantive repeated requests and contrary unique/trivial/invalidated requests, with changing queries and eviction. Preserve separate counter-free timing and allocation/work diagnostics. Include the strongest feasible direct-sharing competitor; a kernel hit count alone cannot answer the architectural question.

T067 owns this entry and the ensuing source gate and prospective comparison. Generalized continuation tables, alpha/relevance projection, broader failure learning and sustainable cache lifetime remain separate S05 obligations.

# Stable equation reuse passes its first correctness gate

A shared constructor owner supports equation reuse without copying syntax trees into cache keys. Two implemented validity policies preserve independently checked equation outcomes: exact binding-context identity and explicit binding dependencies. This is kernel correctness evidence; source execution, lifecycle costs and the comparison with direct sharing remain open.

## What is implemented

The experimental `chr-reuse::stable::Session` owns one append-only constructor arena. All sibling contexts use that owner; fresh variables receive distinct session-wide identities. Handles and contexts carry a retained owner token, and foreign owners are rejected. Equal numeric indices from two independently created arenas cannot authorize a cache hit.

The cache uses a pair of compact term handles as its lookup key. It stores one current entry per ordered pair, with bounded FIFO eviction. It does not canonicalize owned syntax, identify alpha-equivalent operations, merge branches or change source multiplicity.

| Policy | When reuse is valid | Retained evidence and success handling |
|---|---|---|
| Direct | Always execute ordinary kernel unification | No cache entry |
| Exact | The persistent binding map has the same immutable root as the recorded input | Retain input/output binding roots; restore the successful output root |
| Dependencies | Every recorded input-variable binding still has its recorded value, including absence | Retain binding premises and changed bindings; replay changes while preserving unrelated bindings |

Both successful cache policies return the original changed-variable report for ordinary source eligibility wake-ups. Failure hits leave the caller's bindings unchanged. The source executor must still perform those wake-ups and preserve resource and failure behavior; this gate does not yet demonstrate that integration.

## Why dependency reuse is sound on this kernel

On a miss, traverse the two input terms and the binding closure of every reachable variable. Record each variable's current binding, including unbound variables. Constructor nodes are immutable within the single arena, so this closure contains the data that ordinary unification and its occurs checks can inspect. Trial bindings only introduce relationships among terms and variables from that closure.

If those input bindings remain identical, unification performs the same transaction and produces the same success or failure. Its changed bindings can be replayed without overwriting unrelated bindings. If any premise changes, recompute. The proof is conservative: collecting the entire closure can include information irrelevant to an early clash, causing unnecessary misses and miss-time traversal. It avoids structural reconstruction on a hit; it does not eliminate validity costs.

Exact-context identity is also conservative. Equal immutable roots establish identical bindings, but separately constructed equal maps can miss. Both retained roots remain alive, preventing allocation-address reuse from creating a false identity match. The new `Map::same_root` method adds no mutable identity counter and does not alter ordinary map operations.

## Independent and adversarial evidence

The [tests](../../../research/chr-reuse/tests/stable.rs) compare **3,888 ordered-equation configurations** with the independent reference interpreter. The grid combines three policies, four initial binding contexts, nine left and right terms, and four subsequent context changes. It includes repeated variables, constructor disagreement, aliases, structural bindings and occurs-check failures. Successful outcomes compare all three outputs jointly, preserving alias relationships. A nonvacuity check requires more than 500 cache hits.

Directed cases establish successful binding replay, cached failure without speculative binding leakage, invalidation when a new alias creates an occurs-check cycle, foreign-handle/context rejection, and bounded eviction followed by correct recomputation. An unrelated binding change yields a dependency hit and an exact-context miss; both preserve the unrelated value.

Three deliberate implementation faults are rejected by semantic assertions: omitting dependency evidence, omitting successful replay, and accepting the wrong binding-context identity. [Fault receipts and validation](s05-stable-kernel/) preserve the failures and restored-source checks. The independent reference interpreter is unchanged.

## Necessary costs and unresolved responsibilities

Dependency entries avoid retaining entire input/output binding snapshots. They retain their premises and replay delta. Exact entries retain snapshots. Both retain changed-variable reports, hash-table entries and FIFO order. Ordinary unification remains the miss path, so the experiment can price lookup and certificate overhead against a competent direct kernel.

Bounded entry count is not bounded total memory. The session retains its constructor arena until disposal; fine-grained reclamation and cross-query ownership remain to be investigated. This organization also introduces a shared allocation owner that a full source executor must accommodate. It cannot be credited with avoiding coordination or allocation costs before integration.

The next gate must execute complete CHR sources with consumption, propagation, binding-triggered eligibility, explicit choices, fresh identities, off-output failure and finite-sibling service. Test repeated success and clash before and after discrimination, then unique, trivial and invalidated requests. Compare with the strongest applicable direct-sharing executor before making architectural claims. Kernel hits alone do not establish any lifecycle advantage or discharge S05.

## Validation and provenance

Default regression tests, counter-free release kernel tests, strict all-feature Clippy and formatting are recorded in [validation.json](s05-stable-kernel/validation.json), with [source hashes](s05-stable-kernel/sources.json). The stable path's persistent/observer diagnostics can now be disabled through `chr-reuse --no-default-features`; older owned-cache statistics are separate and have not been converted into a primary timing path.

The preceding S04 paired evidence is pinned to measured source commit `58415fa`; its audit verifies that snapshot rather than assuming this kernel extension was present in its binaries. T067 remains active. No comparative S05 timings have run.

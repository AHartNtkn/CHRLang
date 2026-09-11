# Completed graph traversal: share validation and forcing within one tick

**Sharing completed traversal removes most of the dependency engine’s repeated work on these continuing sources.** The continuing graph investigation tests whether templates' work advantage depends partly on repeated traversal of completed calls. The experimental `completed-traversal` feature shares those traversals without changing graph result identity, claim order or the source program. Its control is the existing dependency/template execution with the feature disabled.

## The responsibility boundary

Each public service tick selects one fixed choice context. Successful finite validation can share its completed subgraphs within that tick. Outermost forcing can also share an acyclic chain that ends at an exposed constructor or unknown. It cannot reuse an unfinished call's output or the result of a recursive matching probe.

Two collections own the temporary information: a set of successfully validated nodes and a map from completed path nodes to exposed values. They are cleared at tick entry. An explicit active-tick flag prevents private probes outside that boundary from using the memo. Existing cycle detection runs on paths that have not been validated, including equations whose values are not requested as outputs.

The mutation audit checks the actual production paths in `research/chr-direct-choice/src/demand.rs`:

| Mutation | Why previously collected information cannot be reused afterward |
|---|---|
| Query graph/resource construction | Happens before service and before either memo is active. |
| Ordinary or template body expansion, aliases, new calls, choices and posts | Reached after a successful rule match; expansion completes and returns `Progress` through the forcing callers, ending the tick. |
| Resource consumption and result publication | No evaluation intervenes between partner matching, claims and expansion; result publication returns `Progress`. |
| Local argument pull-tabbing | A successful rewrite returns `Split`, ending the tick. Rejected rewrites return before mutation. |
| Support reclamation | Public operation between ticks; the next tick clears both collections. |

This is an implementation-specific validity argument, not a language restriction. If a future executor continues evaluating after one of these mutations in the same tick, it must update the invalidation boundary.

## Correctness and discriminating checks

The shared-chain regression first fails with 2,211 force entries for 64 completed obligations. With the feature it requires at most 200 force and 200 validation entries while checking the exact answer. Separate tests require choice contexts to remain distinct and a later constructor cycle in an unobserved completed equation to fail. Two negative controls independently suppress the corresponding cache clear; each test then fails semantically. The source is restored exactly and the full graph suite passes again.

The independent equality oracle and existing graph tests pass. The source dependency suite passes with counters off and with work diagnostics: late posts, recursive waits, nested claims, resource-mediated aliases and constructor cycles, selected alternatives, cancellation and preparation reuse remain covered. The registered source work test also passes. Clippy passes with the feature and diagnostics enabled.

## Evidence and next decision

All 40 registered processes pass. Every repeated128 row matches exactly, and every control row reproduces the earlier graph attribution. Service calls, matching/candidate work, template hits, nodes, results and obligations are unchanged by the repair at every recorded prefix. The independently checked answers remain intact.

Counts with the passive query marker present:

| Engine / source / answers | Force entries: control → shared | Finite-validation visits: control → shared |
|---|---:|---:|
| Dependency / pure /128 | 773,957 → 39,018 | 773,699 → 47,145 |
| Dependency / resource /128 | 800,773 → 72,197 | 800,320 → 63,619 |
| Dependency / pure /512 | 45,885,521 → 589,660 | 45,884,495 → 720,475 |
| Dependency / resource /512 | 46,362,096 → 1,142,721 | 46,360,678 → 1,010,491 |
| Template / pure /512 | 706,548 → 219,962 | 576,242 → 221,611 |
| Template / resource /512 | 878,141 → 478,451 | 730,236 → 329,048 |

The512 dependency reductions are about97.5–98.7% for force entries and97.8–98.4% for validation. Templates still perform less of both kinds of work when the repair is applied to both engines. Comparing repaired dependency execution only with unrepaired templates would overstate the architectural implication.

The work gap has narrowed substantially without reusing a derivation across fresh calls. Thus repeated traversal was an avoidable cost, not evidence that templates are necessary for continuing graph execution. Templates retain fewer results and obligations, and may still repay their analysis and representation costs. Only a complete lifecycle comparison can establish that tradeoff.

The feature adds a per-tick validated-node set, a completed-path map, a temporary path vector for outer forcing and an explicit active-scope flag. Invalidation depends on the audited service protocol. Lookup, allocation, peak memory and disposal costs must be measured; the work counters do not count all of them. No runtime default is adopted.

Next, qualify ordinary and metered continuing lifecycles with the repair, including cancellation, retained answers and support reclamation. Add finite/adverse sources with little repeated completed work, and compare the same engines off/on plus credible direct controls before interpreting elapsed results. The existing lifecycle fixtures are available to extend; the128/512 work gate does not stand in for those costs.

This is package three since the [full portfolio review](S08-continuing-selection-review.md). A cost/ownership gate has high immediate value because it can reverse the apparent benefit once memo maintenance is counted. Conditional application attribution, nonempty/partial coverage costs, partner ordering and broader integration remain required alternatives. Reconsider the full portfolio at the next package boundary. T074 and the goal remain active.

Exact rows and reproduction checks are in [the audit](s08-completed-traversal/audit.json).

[Registration](../registrations/S08-completed-traversal.md), [raw evidence and frozen sources](s08-completed-traversal/), [receipt auditor](../../../research/chr-reuse/experiments/audit_completed_traversal.py).

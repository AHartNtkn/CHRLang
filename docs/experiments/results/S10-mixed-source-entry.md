# Three execution organizations pass the first mixed-source gate

Compiled, contextual and direct conditional execution agree with independently constructed complete answers on all 32 mixed-source configurations. This makes them credible candidates for further composition tests. It does not select an architecture or establish whole-program efficiency.

## What was tested

The [registered gate](../registrations/S10-mixed-source-entry.md) combines two-head consumption, a kept permit, propagation history, constructor matching, a nontrivial equality guard, fresh payload aliases, explicit choice and branch-local failure. Independent variations change equal-valued versus different-valued choices, second-arm failure, spare resource multiplicity, query occurrence order and propagation-rule position.

Each successful arm consumes exactly one matching ticket, keeps the permit, produces exactly one propagation mark, and exports a fresh unknown shared between its result and residual constraint. The first arm retains the configuration resource; the second consumes it. Equal-valued choices therefore need not be identical complete answers. A failing second arm publishes nothing and cannot consume the first arm's resources. These statements determine the expected answers directly, independently of any evaluator.

For each configuration, the existing independent scalar evaluator and six candidate configurations are checked against that constructed answer multiset: compiled Global Scan, ordinary contextual execution, shared contextual deductions, persistent contextual equality with and without sharing, and direct conditional execution. Prepared rules are reused across 16 changed queries per rule ordering. Every finite run must exhaust within 200000 service calls. No cutoff occurred.

The [test source](../../../research/chr-direct-conditional/tests/composition.rs) passes in [default](s10-mixed-source-entry/default.log) and [counter-free feature](s10-mixed-source-entry/counter-free.log) builds; [scoped strict Clippy](s10-mixed-source-entry/clippy.log) passes. This is 32 configurations × seven executions × two builds = 448 finite executions, each checked against the independent expectations. No timing or allocation measurements were performed.

## The responsibilities these candidates actually retain

This is a source inspection, not a complexity score. The candidates support the tested semantics through materially different ownership and scheduling choices.

| Responsibility | Conventional compiled | Contextual relational | Direct conditional |
|---|---|---|---|
| Source preparation | Prepared rules and matching plans; optional specialization | Shared rules and predicate-arrival map | Shared rules and lowered matching/guard plans |
| Search | FIFO frontier of complete branch continuations | FIFO frontier; source OR clones the state | Shared conditions and dynamic choice births; no evaluator per history |
| Equality and matching | Kernel terms/bindings, separate discovery and access policy | Context-local equality descriptors and relational matches; optional persistent maps and deduction reuse | Conditional equality with incremental jobs, support-sensitive matching and guards |
| Consumption/history | Occurrence identities and propagation eligibility within branch state | Context store checks consumption; ordered occurrence tuples track propagation history | Resource claims and application acknowledgement under conditions |
| Invalidation | Chosen activation/access policy maintains candidates | Posts invalidate predicate readers; equality steps conservatively invalidate candidate caches | Dependency notifications feed subsequent finite candidate rounds |
| Trusted observation | Completed branch exports its answer | Publication requires no pending effects, application or equality work | Completion conditions plus history enumeration and observation jobs |
| Principal retained obligation | Branch restoration and repeated discovery/equality work | Branch copies, candidate lists, equality ownership and cache validity | Condition algebra, births, claims, round completion and incremental publication |

Source anchors: [compiled search](../../../research/chr-compiled/src/search.rs), [contextual execution](../../../research/chr-relational/src/contextual_execute.rs), [contextual store](../../../research/chr-relational/src/contextual.rs), [conditional execution](../../../research/chr-direct-conditional/src/engine.rs), [conditional resources](../../../research/chr-direct-conditional/src/resources.rs). The relational copied-state executor also exists, but this gate does not measure or newly qualify that fourth configuration family.

The contextual candidate interleaves equality deductions with body effects and application discovery. Its optional deduction reuse changes equality storage; it does not eliminate the branch frontier or propagation history. The direct conditional candidate is a substantive replacement organization, but avoiding explicit branch evaluators introduces condition, claim and completion machinery. Neither fact determines total efficiency without measurement.

## Scheduling is a language tradeoff, not an automatic defect

The [S00 contract](S00-contracts-and-candidates.md) permits different committed schedules. Compiled Global uses its ordinary selection policy; contextual execution interleaves deductions; conditional execution fixes a finite candidate round before processing resulting invalidations. This test uses sources whose constructed outcomes survive those choices. It is not proof of general confluence.

On competing nonconfluent sources, subsequent tests must independently validate permitted executions and record changes in answers or progress. A timing ratio between different accomplished computations would not establish architectural superiority. Branch fairness and finite service of internal operations also remain separate obligations.

## What follows

Keep T078 active. Next qualify mixed finite-sibling progress and lowering/resource composition, including queries outside the fusion certificate. Existing individual progress tests are useful controls, but do not prove progress after composition. Source-derived fusion must retain its resource certificate and ordinary execution outside eligibility; preparation, routing and artifact ownership must eventually be charged.

After these gates, register complete-path lifecycle costs and identify which component questions those interactions make consequential. Sustained lifetime, native compilation, broader integrated mechanisms and held-out challenges remain required. Passing this gate resolves only whether the tested combinations are semantically compatible enough to continue.

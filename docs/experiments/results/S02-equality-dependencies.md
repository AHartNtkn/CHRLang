# Indexing awaited equalities avoids scans but adds repair obligations

Pair indexing avoids the irrelevant notification scans demonstrated by endpoint subscriptions. Event filtering avoids full matching but still performs those scans. Moving a highly connected class and advancing structural information expose the index's maintenance cost, so these results establish mechanisms and tradeoffs rather than a speed winner.

The [registered source/work comparison](../registrations/S02-equality-dependencies.md) now passes **49 package tests**, including 19 constructor/source tests. All 200 repeated/distinct-pattern configurations run in each of three modes, giving **600 independent source comparisons** in that matrix. The 36 recorded diagnostic states reproduce exactly in a second run. Scoped strict Clippy and formatting pass; no comparative timing or allocation experiment ran.

## Filtering and indexing do different work

The favorable witness registers 64 requests, each waiting for a common value to equal a distinct other value. It then aliases the common value with 64 fresh unknowns, none of which can satisfy a request. The common class survives these merges.

| Dependency organization | Full request inspections | Subscriber entries enumerated for notifications | Pair-map operations | Incident-set edits |
|---|---:|---:|---:|---:|
| Endpoint subscriptions | 4,160 | 4,096 | 0 | 0 |
| Filtered endpoint notifications | 64 | 4,096 | 0 | 0 |
| Indexed awaited pairs | 64 | 0 | 64 | 128 |

These counts include registration. The indexed control's 64 pair-map operations and 128 incident edits create its initial relationships; irrelevant merges do not enumerate those relationships. Filtering discovers the same endpoint notifications as the first control, then checks whether the blocked pair could have progressed. Its low inspection count alone would conceal the remaining quadratic discovery.

After supplying the useful identities, all 64 requests consume their own tokens and produce the independently expected complete answer. Indexed execution then totals 128 inspections and 64 notifications. Suspended forks retain the expected residual requests and tokens; the consumed branch has no remaining subscriptions or incident relations.

## The index has costs when relationships move or advance

The adverse witness starts with 64 different awaited pairs. A larger class absorbs their common endpoint, forcing relocation. The other endpoints then coalesce, and both sides acquire `f` descriptors whose children are still unequal. Finally the children alias, enabling all consumers. Every settled transition checks the bidirectional index against the live blocked requests.

| Organization, after the full adverse witness | Inspections | Notification entries | Pair-map operations | Incident-set edits | Moved relations / subscriber memberships |
|---|---:|---:|---:|---:|---:|
| Endpoint subscriptions | 2,399 | 2,399 | 0 | 0 | 0 / 0 |
| Filtered notifications | 192 | 2,399 | 0 | 0 | 0 / 0 |
| Indexed pairs | 192 | 128 | 513 | 640 | 128 / 191 |

The initial high-degree relocation alone moves 64 indexed relations and 64 memberships. The full indexed run also visits 257 incident entries and performs 319 pair-membership edit attempts. Endpoint and filtered controls instead relocate 191 endpoint subscription entries over the full witness. These are different obligations, not zero-cost alternatives to one another.

The [full work records](s02-equality-dependencies/work.json) retain registration, coalescence, descriptor supply and completion separately, at widths eight and 64. They also include recursive pattern/equality visits, subscription edit attempts and repaired handles. Tree comparisons, allocation, consistency traversal, observation and container disposal are not measured by summing these counters. No aggregate work score, timing estimate or memory ranking follows.

## What the implementation must preserve

The index maps an unordered pair of current classes to its waiting requests. Each endpoint records incident partners. Merging classes relocates only the losing class's relations; duplicate pairs coalesce their request memberships. A pair whose endpoints become identical notifies its subscribers. When an endpoint gains a descriptor, incident pairs with two known descriptors are reconsidered, allowing matching to discover an unresolved descendant pair.

The losing class is still selected by handle-set size. That controls existing handle repair, not relationship degree; a class with many incident pairs can lose and incur the relocation above. Choosing a different merge policy would have to account for both obligations. The result supplies no reason to call that policy universally inferior or to change it without a consequential comparison.

The first indexed test failed because registration alone did not deliver an identity-merge notification. The [causal failure](s02-equality-dependencies/red.log) required actual token consumption and source answers. The completed tests additionally cover a described winner absorbing an unknown watched class, pair coalescence, nested descriptors, repeated variables, permanent mismatch, and unchanged suspended forks.

Integrity checks require every indexed membership to correspond to a live blocked request, every incident edge to have its reverse and pair entry, and every blocked request to be discoverable from the selected subscription organization. After quiescence, an indexed blocked pair must still lack information on at least one side. Consumption and resolved mismatches release subscriptions. Constructor-only witnesses retain their previous inspection counts in all three modes, including 128 inspections at width 64 and 512 at width 256.

Conflicting known descriptors now fail before endpoint relocation, keeping dependency state consistent at that failure boundary. Finite-tree consistency and source failures continue to agree with independent checks. This does not change the accepted source semantics. Pending equations still settle before token consumption, and ready request order is preserved.

## Architectural consequence and next investigation

The broad-notification cost is not intrinsic to local integration. Pair indexing demonstrates a different discovery organization with complete consuming-source correspondence, while making its extra invariants and maintenance explicit. A timing comparison must include that index's preparation, ordinary operation, ownership and disposal; fewer inspections are insufficient.

**Next extend source-derived integrated execution beyond the single capture equation.** The relevant question is whether matching can produce constructor equations and further consuming applications directly in the same representation, while preserving source scheduling, fresh identities and complete observations. Register a bounded source fragment with immutable prepared rules, changed queries, chained effects, adverse competing consumers and explicit rejected boundaries. Compare against ordinary source execution and an equally specialized applicable control; do not attribute generic matcher overhead to representation alone.

| Strong alternative | What it could change | Selection at this boundary |
|---|---|---|
| Broader integrated source execution | Shows whether integration removes service boundaries across real body effects and subsequent applications | Selected. Dependency organizations and independent scalar controls now exist; the missing body/source path is a more consequential uncertainty than further counters on the same aliases. |
| Immediate lifecycle matrix of the current capture-only fragment | Measures whether the index repays its overhead in time and requested allocation | Required when this fragment can discriminate a complete candidate. It needs counter-free builds, preparation/query ownership and equally specialized execution; running only this narrow rule now could confound source capability with representation. |
| Repeated dynamic reunion and broader inference | Broadens explicit decomposition beyond one checked finite private phase | Required under T077. Reconsider at the integrated source boundary; the new source path can change whether integrated execution itself is a credible complete alternative. |
| Broader source elimination/resource solving | Avoids work in both local and conventional execution | Required under S06. Include applicable elimination when qualifying the next controls, without treating a finite countdown shortcut as general lowering. |

A further dependency refinement requires a specific defect or crossover that obstructs the next architectural comparison. If the broader source gate exposes one, investigate it; otherwise carry these three qualified controls forward. This is the second T072 package since the reunion boundary and the first package after the last four-package breadth review.

The current `Plan` still accepts a single consuming `take`/`token` rule with a captured-variable equation. Arbitrary bodies, general joins, propagation history, scheduling alternatives, full lifecycles, language tradeoffs, coherent architectures and held-out challenges remain required. No production dependency policy or language architecture is selected. Reference-interpreter and independent-evaluator implementations are unchanged, and the goal remains active.

Evidence: [registration](../registrations/S02-equality-dependencies.md), [final source/work tests](s02-equality-dependencies/tests.log), [second diagnostic run](s02-equality-dependencies/replay.log), [structured work](s02-equality-dependencies/work.json), [audit and source hashes](s02-equality-dependencies/audit.json), [Clippy](s02-equality-dependencies/clippy.log) and [formatting](s02-equality-dependencies/format.log). The scoped Clippy command succeeds; its log also records existing dependency build-script warnings.

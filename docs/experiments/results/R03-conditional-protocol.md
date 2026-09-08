# R03 direct conditional execution: selected protocol

Test direct activation over conditional state against competent explicit source search. The potential saving is performing one source operation across many alternatives without discovering it separately in each branch. The necessary cost includes conditional resource ownership, matching, equality, completion and answer recovery; shared term nodes alone do not provide this architecture.

This is an implementation contract and analytical entry, not a completed runtime or performance result. The [source-search diagnosis](R03-search-diagnosis.md) makes globally selected early rejection a necessary control on its finite regimes. [E03](E03.md) supplies the adverse precedent: shared expansion can be overwhelmed by projected discovery. [E06 native probes](E06-native.md) establish correlation, disconnected failure and progress hazards that this design must address.

## Selected organization and alternatives

Use a canonical Boolean decision DAG for supports: a support denotes the source histories in which a record exists or an operation is eligible. A support handle is not an enumerated set of completed branches. Occurrences and variable bindings activate dependent matching work directly when their supported state changes. No routine first runs one projected machine per interpretation to discover equal operations.

The initial candidate has one serial owner for source effects. It maintains supported pending body work, occurrence liveness, conditional bindings and propagation history. A rule application commits over an explicitly justified support. This permits a meaningful first implementation without making parallel coordination or speculative equality part of the initial hypothesis.

A finite list of branch tickets is a useful oracle but cannot serve as the candidate's support representation: its construction would already perform the work this experiment asks whether the runtime can avoid. A conditional term graph without resource/completion state is insufficient for consuming CHR. A fully distributed commit protocol introduces ownership conflicts before the sequential sharing question is answered. These are distinct boundaries, not interchangeable implementations of this experiment.

## Source contract and projection

The source fragment includes first-order finite trees, nonbinding heads and equality-entailment guards, kept/consumed occurrences, propagation, fresh body variables, explicit OR and selected outputs with full residual multisets. Unsupported guard forms must be rejected as outside the experimental fragment, never treated as failure or success. Infinite derivations need finite sibling progress; a finite execution budget is not exhaustion.

For any reachable source history, restriction of conditional bindings, live occurrences, propagation tokens and pending work must describe one legal source state. Candidate administrative steps preserve that state. A supported source commit performs one legal transition in every history in its support and changes no history outside it. Different physical commits may cover disjoint subsets of the same logical source step. No implicit alternative is introduced for competing CHR rules.

For nonconfluent programs, select a documented rule/occurrence order and validate it against an independent scalar executor with that order. Matching a favorable final answer set is insufficient. Architectural timing may compare other permitted schedules only where their observable relation is independently established to agree. The current global source control remains a concrete comparator, not a mandatory internal module structure.

## Records and ownership

| Record | Meaning and invariant | Owner / lifetime |
|---|---|---|
| Source birth | Fresh explicit OR identity, activation support and causal parent continuation | Query-owned; copied uses retain identity; never reused during the query |
| Variable | Fresh source identity, with guarded bindings to terms | Query-owned; binding regions are disjoint for differing definitions and acyclic after restriction |
| Occurrence | Stable identity, predicate/arguments and live support | Commit subtracts only the consumed region; identical payloads never merge source identities |
| Propagation token | Rule plus ordered occurrence tuple, with already-fired support | Eligible support subtracts fired support; repeated equal occurrences remain distinct |
| Body obligation | Supported continuation and source sequence position | Application creates it before releasing the right to select another rule on that support |
| Matching cursor | Candidate tuple, demand dependencies, tested versions and remaining search region | Restart or revalidate on relevant change; an unfinished cursor is an outstanding obligation |
| Failed support | Histories with established source failure or finite-tree inconsistency | Monotone within the query; excludes publication and eligible effects |
| Completion evidence | Support with no pending source work and no enabled application after current-version exhaustive search | Invalidated by any relevant state change before publication |
| Observation task | Supported output/residual reconstruction plus source-birth enumeration cursor | Owns required state until delivered or explicitly cancelled |

Query disposal releases all records and decision-DAG caches. First implementation uses query-scoped allocation; it must report retained nodes and disposal cost rather than claim within-query reclamation. Long-lived streams are a separate lifetime test. A completed query with retained answers may need answer-owned data or an explicitly retained query handle; ownership cannot be implicit.

## Matching and committing without projected discovery

A new occurrence or a supported binding change schedules only predicates/head positions and structural demands that can depend on it. Predicate lookup and stable occurrence identities enumerate possible tuples. A symbolic structural matcher refines the support on which the tuple matches, visiting alternatives only when the demanded constructor or equality differs. A first-use variable head carries the conditional value opaquely; it must not partition support merely to read that variable. Constructor tests and repeated-variable equality introduce demand where necessary. This is the concrete opportunity for common work after a birth. Matching never binds a query variable to make a head or guard succeed. Repeated head variables require equality already entailed under that support.

A candidate's initial eligible region intersects all head liveness, successful matching and guard regions, and subtracts failed support and the tuple's propagation history when relevant. The same occurrence cannot fill two head positions where source distinctness is required. Equality becoming known later must reactivate the affected demand even when no new occurrence is inserted. Expanding the support on which an existing equality holds also counts as a relevant change, even if term or representative identities stay the same.

To preserve the chosen committed order, a later candidate may fire only on support not reserved by an earlier enabled candidate or unfinished earlier eligibility check. Maintain support for the unresolved portion of the ordered candidate scan. This may require substantial symbolic priority bookkeeping; it is measured cost, not assumed free locality. A batch of mutually disjoint eligible regions may commit separately. Do not equate arrival order of notifications with the declared source order.

Immediately before commit, revalidate affected versions, tuple liveness, guards and history. The serial owner atomically reserves the eligible region, subtracts it from removed occurrences, extends propagation history where required, allocates fresh body variables and installs the supported body continuation. A body failure excludes that region; effects remain invisible outside it. Body processing and equality stabilization finish before another source application on that region, matching the selected control's boundary. This candidate does not require speculative application against unfinished equality.

## Conditional finite-tree equality

Body equality is a supported operation. Dereferencing follows guarded bindings and partitions the operation's support only where the referenced binding differs. Equal constructors enqueue paired fields; distinct constructors mark their supported intersection failed. Binding a variable records a supported edge only after a supported occurs check. A cycle existing only under an inconsistent conjunction must not reject a live history, while a cycle under a satisfiable history must prevent publication there. For example, `X=f(Y)` under `b` and `Y=g(X)` under `not b` form no cycle in either source history. Cycle detection intersects edge supports along the path; alias compression and derived constructor equalities carry the same contextual obligation.

The algorithm needs a versioned dependency trail for demanded variables and constructor tests. Stable negative results are not assumed across bindings. Unification, occurs checks and decision-DAG operations must be resumable jobs so a large equation does not monopolize service. Interned terms can reduce payload copying, but physical node identity is not source equality and does not identify a source choice birth.

## Births, raw multiplicity and completion

An OR obligation allocates a fresh birth identity with the obligation's activation support, then creates its two supported child continuations. Birth allocation does not duplicate unrelated records. Its activation support depends only on causal ancestors, never the birth itself or a descendant. The parent pending obligation transfers atomically to child obligations; retaining an unconditional parent obligation would incorrectly block completion of a finite sibling. If an operation is shared across mutually exclusive histories, its one physical birth can cover that support: each restricted history still sees one fresh source OR. Distinct coexisting operations must allocate distinct births. Repeated visits to the same source syntax location are distinct dynamic births.

A birth's liveness is causal, not merely its continued appearance in a term. Both `true OR true` alternatives remain two raw source results even when all output terms are equal. Conversely, an OR executed only under one outer alternative cannot introduce extra interpretations under the other. Enumeration follows active source births in causal order and assigns no extra multiplicity to inactive births. Administrative decision-DAG nodes do not create source alternatives.

Completion is a statement about all active obligations under a support, including work disconnected from selected outputs. No pending body/equality job, unresolved eligibility scan, unserved binding notification or enabled source tuple may remain there. Failed support contributes no answers. A completed support may coexist with unfinished or divergent support; it must be publishable without globally normalizing the entire query. Dormant watchers whose matching tests are currently blocked on free variables do not by themselves prevent completion; their current-version tests must be finished, and no active binding work may still change them on that support. Queued or in-progress matching checks do prevent completion.

Delivery tracks each completed active birth history exactly once, independently from equality of exported answers. Once publication is certified, work outside that support cannot mutate its logical observation. Cancellation and early answer disposal must release the associated observation ownership without silently retiring another active alternative.

Invalidation must preserve progress on disjoint support. A loop on one alternative must not repeatedly restart the finite sibling’s eligibility or completion proof merely by changing a global version. Invalidation intersects the changed support with the cursor/certificate support; unchanged regions retain their evidence and remaining work. A different implementation must establish the same noninterference property explicitly.

Finite service applies to support operations, matching, unification and output enumeration as well as source rule bodies. Round-robin resumable jobs prevent one unbounded derivation from occupying the entire service loop. The implementation must demonstrate a finite sibling answer beside divergence; naming a queue is not a proof of that property. Enumeration must preserve joint aliases, residual occurrence multiplicity and raw duplicate answers. Exact answer deduplication is a separate observation policy and is not used to repair raw birth mistakes.

## Independent witness obligations

These are expected source behaviors for the implementation gate, not claims of executed tests.

| Witness | Independent required behavior | Defect exposed |
|---|---|---|
| `X=a OR X=b`, output `pair(X,X)` | Exactly `(a,a)` and `(b,b)` | Copies losing correlation |
| Two independent binary OR operations | Four paired outputs | Distinct births sharing identity |
| `true OR true` | Two equal raw observations | Support simplification losing multiplicity |
| `true OR (true OR true)` | Three raw observations | Inactive births adding interpretations |
| A repeated recursive source OR | Fresh birth at every actual execution | Syntax location used as dynamic identity |
| `p(X), token <=> out(X)` with conditional `p` | Token consumed exactly where the chosen tuple fires | Global consumption leaking across histories |
| `p` live only under `b`, `q` only under `not b`, two-head consumer | No joint tuple | Ignoring support intersection |
| Two competing consumers of one token | Exactly the selected legal firing per history | Overlapping commits or implicit source search |
| Two equal live tokens, two-head rule | Two distinct IDs required; one token alone cannot fire | Payload equality replacing resource identity |
| Propagation followed by a binding change | Same tuple does not refire on already-fired support; newly eligible support can fire | History stored as one global bit |
| `X=f(X)` under left OR, success under right | Only right observation | Conditional cycle poisoning sibling or being ignored |
| Output-shaped data plus disconnected active failure | No observation on that support | Output demand mistaken for completion |
| Divergent left OR, finite right OR | Finite right observation with bounded service steps | Global normalization or unfair support work |
| Fresh local variable used in two body goals | Aliased uses within a firing; distinct locals across firings | Freshness or body continuation mismatch |
| Nonground output sharing a residual variable | Joint alias preserved after export | Independent output/residual reconstruction |
| Guard false while unknown, then entailed after binding | Required reconsideration and exactly one permitted application | Missing negative-demand invalidation |

The independent checker must inspect transitions, not only final answers, for consumption, propagation and competing rules. A small scalar truth-assignment projection checker is appropriate outside candidate execution. It must enumerate only reachable active births, reconstruct fresh identities and independently check enabledness/quiescence. It is not the runtime matching algorithm.

## Selected implementation and comparison scope

Implement a separate direct-activation candidate with the complete path above, sharing syntax only where useful. First gate uses the witness suite and small generated supported transitions with independent projection checks. Build ordinary no-OR cases into the same gate; the design must expose its fixed support/notification overhead as well as potential sharing. No candidate timing begins until the source-effect, completion and progress gates pass.

The initial cost contrast then uses common opaque work across choices, immediate constructor discrimination, consuming competition, early failure and ordinary computation. The exact registry must choose bounded sizes, credible compiled global/active access controls appropriate to each regime, cold/reused setup, counters-off timing, separate logical work and allocation diagnostics, complete observation and disposal. Expected favorable behavior is fewer physical applications without proportional per-history discovery. Expected adverse behavior is support fragmentation, priority maintenance and output cardinality overwhelming that saving. Neither outcome selects a universal architecture.

This implementation has greater present decision value than another active-queue refinement: T033 already identifies a competent early-rejection path. It also answers a distinct question from the finite solver, whose useful fragment has no general conditional resource execution. Compilation amortization, optional static declarations, longer lifetimes and parallel ownership remain open; they are not preconditions for this semantic entry.

## Entry review and remaining proof

Independent design and adversarial reviews agreed on the conditional-resource path and identified support-local progress, inactive-birth counting, atomic body installation and dormant-watcher completion as essential. They are incorporated above. The finite-sibling argument, projection-preserving equality/resource steps and exactly-once delivery still require executable independent validation. This document does not mark those gates as passed. The selected decision is to implement and test this organization; no architecture ranking follows from its specification.

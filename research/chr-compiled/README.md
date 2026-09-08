# Integrated activation and native rule compilation

This experimental engine executes the same no-OR CHR fragment using generic or generated rules, with global scanning or FIFO occurrence activation. Hand checks establish complete observations and legal committed applications. They establish no performance advantage or production architecture choice.

The engine supports finite first-order terms, nonbinding head matching, pure equality guards, body unification with occurs checking, conjunction, failure, simplification, simpagation and propagation. Rule bodies allocate fresh local variables on each application. OR is rejected explicitly. The reference evaluator is neither imported nor modified.

## Running and generating

Run focused validation with `cargo test -p chr-compiled -p chr-persistent`. Build-time generation is part of compiling `chr-compiled`; `build.rs` writes native Rust into Cargo's `OUT_DIR/generated.rs`. Its input is `fixtures::programs()`, containing rules only. Generated source and the native binary are derived artifacts; preserve them with their source/build hashes for a registered run.

`PreparedRuleset::bundled(program_id, execution)` prepares a bundled ruleset once. `PreparedRuleset::new(rules, code)` accepts separately supplied rules and an optional native `Compiled` descriptor. Call `prepared.start(query, policy, access)` for each independent runtime `chr_syntax::Query`. Policies are `Global` and `Active`; execution forms are `Generic` and `Generated`; access controls are `Scan` and `Indexed`. The same prepared rules and compiled selectors handle different runtime recursion depths.

Prepared rulesets share immutable lowered heads, templates and dispatch across engines. Each query gets its own term arena, predicate dictionary, variables, bindings, occurrence IDs, history, pending effects, indexes and diagnostics. Initializing the query's predicate dictionary from ordered prepared metadata remains explicit setup work. Dropping a prepared owner does not invalidate existing engines, and dropping one engine does not change another query.

`generate::emit(prefix, rules)` emits native selectors for inclusion and compilation by a caller. The emitted `{prefix}_code()` provides the corresponding `Compiled` descriptor. Native generation does not unfold queries or specialize to their values.

`step()` advances one pending effect or one part of partner traversal. `advance(budget)` performs at most that many steps and returns completion/failure status without exporting an answer; `status()` inspects that status without executing. `observe()` separately exports only a completed successful state. Repeated observations return the same final value; this is a state inspection API, not a stream of new answer events. A ground output is not published while pending effects or selection work remain.

## The controlled comparison

All four cells share the term arena, binding map, store and predicate indexes, body-effect queue, history and answer export. Generic preparation caches head sequences, slot-indexed term/guard/body templates and body predicate IDs. Generated Rust contains native constructor tests, repeated-variable slot checks, guards, body construction and a state machine for each rule's head positions. It does not call generic rule-matching, guard or body template walkers. Bound-key derivation uses the same prepared access plan in both execution forms and has separate counters.

Both forms use the same resumable depth-first partner cursor. One step examines at most one occurrence candidate, or checks one complete tuple. Binding frames and partner positions survive between steps. Pool construction, structural term matching, guards, unification, body construction, dependency traversal and answer export remain synchronous. This gate does not establish bounds on primitive work per step or universal fairness.

Global scanning considers rules in source order and partner IDs in ascending order. Activation uses a coalesced FIFO occurrence queue and predicate-to-head-position dispatch. After firing, a retained active occurrence is requeued and its traversal restarts, with history checks suppressing repeated propagation. Source effects drain before further selection. These are experimental schedules; the active policy does not imitate the global policy's minimum tuple.

Successful unification reports committed variable changes. Watchers follow original argument variables, alias chains and variables below constructors. Dependencies are refreshed after relevant bindings and discarded for consumed occurrences. A cursor cannot coexist with intervening source effects in this API: firing ends that cursor before body effects run. Thus a body update can enqueue the affected occurrence again without losing an in-progress notification.

## Bound-argument access

`Access::Scan` traverses the ordered predicate pool. `Access::Indexed` derives any entailed ground keys from the current head and prefix bindings, selects the smallest matching argument bucket, and retains ascending occurrence order. Both forms still perform complete nonbinding matching and guard checks. When no justified ground key exists, indexed access scans the predicate pool; it does not assume the source has a ground mode.

A key is a fully dereferenced, recursively normalized ground term, hash-consed in the query's arena. An arena handle containing a variable is not treated as its eventual value. Unknown arguments have no ground entry but remain stored and watched. Binding repair follows aliases and variables under constructors, updates occurrence-to-key records and ordered buckets, and activates affected occurrences when using FIFO scheduling. Global indexed execution also maintains the dependencies needed to repair keys. Consumption removes all associated entries. Equal-valued occurrences remain separate IDs in each bucket. Predicates absent from the immutable head dispatch stay in the residual store but receive no matcher pools, key entries, dependency watchers or activations. Their variables still reify through the ordinary binding map at observation. No argument-position liveness restriction is inferred.

Key derivation can allocate normalized term nodes that remain interned for the query lifetime. Derivation visits, prepared access-plan visits, normalization requests/allocations, bucket probes/materialization, repairs and retained records are reported separately. These costs are included in execution; normalization is not free preprocessing.

## Semantic fixtures

`fixtures::case(id, n)` supplies independently constructed complete answers for IDs 0–11. Only IDs 0–2 use `n`; other cases are fixed. ID 12 is checked by its legal applications and terminal state, with different cross-policy observations permitted.

| ID | Question and expected observation |
|---|---|
| 0 | Recursive construction: one list of `n` items, no residual constraints. |
| 1 | Chain closure/jobs: `n` edges, `n+1` reaches and done constraints; every job consumed, each output `seen(node(i))`. |
| 2 | The same chain with initially unknown nested edge keys, resolved by body equations; same complete result. |
| 3 | Guard wakeup after nested variable aliasing; one hit and jointly aliased outputs. |
| 4 | Two aliased occurrences fill distinct ordered propagation tuples; two hits, two retained occurrences. |
| 5 | A middle head arrives after the outer heads; one hit and complete consumption. |
| 6 | Occurs-check failure; separate runtime queries also check true and failure after grounding an output. |
| 7 | Two firings produce distinct fresh holes while preserving each output's residual alias. |
| 8 | A fresh equal-valued partner arrives after a propagation token exists; two hits for different occurrence tuples. |
| 9 | A kept active occurrence becomes ground through its own rule body; the newly enabled rule fires. |
| 10 | An unknown argument does not match a constructor by binding it; it remains residual. |
| 11 | `X=f(Y)` followed by `Y=a` wakes a constraint originally mentioning only `X`. |
| 12 | Competing consumers: FIFO activation and global scanning choose different legal committed schedules. |

Additional access fixtures are `flat_chain_case(n, delayed)` for programs 1/2, `collision_case(n)` for program 13, and `repair_case(n)` for program 11. Flat keys are depth-zero atoms with fixed-width hexadecimal names. The collision fixture retains multiple equal-valued occurrences, including a newly inserted partner after history exists. The repair fixture grounds every nested key to `f(b)` without enabling the rule for `p(f(a))`.

The checker in `tests/support` uses source terms, not candidate matching/equality/index functions. It validates application eligibility, distinct occurrence IDs, pure guards, freshness, instantiated body, consumption and history changes. At successful termination it independently searches for remaining enabled tuples. It does not enumerate rule schedules or independently check every intervening primitive equation/effect. Full-answer expectations and the atomic equality regression supply additional checks. Within each policy, generic and generated application traces must agree.

## Costs and lifetime

`stats()` separates candidate visits, template interpretation, structural tests, copied binding slots, dispatch/cursor steps, pool materialization, history checks, variable/dependency updates, activation events and arena/binding work. All counters are observational and do not select scheduling or access paths. Hot-path increments can still distort timing, particularly the generic interpretation counters. A future timing protocol must address instrumentation overhead separately. These are heterogeneous counters, not interchangeable time units or complete allocator accounting. `retention()` reports logical store, pending-stack, history, queue, dependency, arena, cursor, trace and audit records; these are not heap bytes.

Diagnostics default off. `enable_trace()` records lightweight application traces; `enable_audit()` enables traces and additionally captures source views before and after commits. Audit traversal counts are separate. State, history, interned terms and enabled diagnostics remain owned by the engine after completion or failure. Dropping the engine reclaims them. A cutoff preserves the cursor for resumption; dropping it cancels the whole experiment. There is no separate externally injected update or partial-cursor cancellation API.

Before performance comparisons, include source generation, native compilation, query preparation, execution, extraction and destruction at explicit lifecycle boundaries. Measure cold PreparedRuleset construction separately from each start/query lifetime, and retain a prepared owner when measuring reuse in either execution form. Indexed and scanned access are explicit matched controls. The original structured-key fixture grows term size with store size; the flat-key fixture separates those dimensions. Low-selectivity and repair-heavy fixtures test whether index maintenance outweighs saved access work. No generated comparative matrix or cost run is part of these hand checks.

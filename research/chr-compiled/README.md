# Integrated activation and native rule compilation

This experimental engine executes the same no-OR CHR fragment using generic or generated rules, with global scanning or FIFO occurrence activation. Hand checks establish complete observations and legal committed applications. They establish no performance advantage or production architecture choice.

The engine supports finite first-order terms, nonbinding head matching, pure equality guards, body unification with occurs checking, conjunction, failure, simplification, simpagation and propagation. Rule bodies allocate fresh local variables on each application. OR is rejected explicitly. The reference evaluator is neither imported nor modified.

## Running and generating

Run focused validation with `cargo test -p chr-compiled -p chr-persistent`. Build-time generation is part of compiling `chr-compiled`; `build.rs` writes native Rust into Cargo's `OUT_DIR/generated.rs`. Its input is `fixtures::programs()`, containing rules only. Generated source and the native binary are derived artifacts; preserve them with their source/build hashes for a registered run.

`Engine::new(program_id, query, policy, execution)` accepts a separate runtime `chr_syntax::Query`. Policies are `Global` and `Active`; execution forms are `Generic` and `Generated`. The same compiled rules handle different runtime recursion depths. `Engine::with_program(rules, query, policy, code)` also accepts separately supplied rules; `generate::emit(prefix, rules)` emits their native selectors for inclusion and compilation by a caller. The emitted `{prefix}_code()` provides the corresponding `Compiled` descriptor. Native generation does not unfold queries or specialize to their values.

`step()` advances one pending effect or one part of partner traversal. `run(budget)` advances at most that many steps and returns current completion, failure and the complete answer when successful. Repeated calls after completion re-export the same final observation; this is a state inspection API, not a stream of new answer events. A ground output is not published while pending effects or selection work remain.

## The controlled comparison

All four cells share the term arena, binding map, store and predicate indexes, body-effect queue, history and answer export. Generic preparation caches head sequences, slot-indexed term/guard/body templates and body predicate IDs. Generated Rust contains native constructor tests, repeated-variable slot checks, guards, body construction and a state machine for each rule's head positions. It does not call the generic template walkers.

Both forms use the same resumable depth-first partner cursor. One step examines at most one occurrence candidate, or checks one complete tuple. Binding frames and partner positions survive between steps. Pool construction, structural term matching, guards, unification, body construction, dependency traversal and answer export remain synchronous. This gate does not establish bounds on primitive work per step or universal fairness.

Global scanning considers rules in source order and partner IDs in ascending order. Activation uses a coalesced FIFO occurrence queue and predicate-to-head-position dispatch. After firing, a retained active occurrence is requeued and its traversal restarts, with history checks suppressing repeated propagation. Source effects drain before further selection. These are experimental schedules; the active policy does not imitate the global policy's minimum tuple.

Successful unification reports committed variable changes. Watchers follow original argument variables, alias chains and variables below constructors. Dependencies are refreshed after relevant bindings and discarded for consumed occurrences. A cursor cannot coexist with intervening source effects in this API: firing ends that cursor before body effects run. Thus a body update can enqueue the affected occurrence again without losing an in-progress notification.

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

The checker in `tests/support` uses source terms, not candidate matching/equality/index functions. It validates application eligibility, distinct occurrence IDs, pure guards, freshness, instantiated body, consumption and history changes. At successful termination it independently searches for remaining enabled tuples. It does not enumerate rule schedules or independently check every intervening primitive equation/effect. Full-answer expectations and the atomic equality regression supply additional checks. Within each policy, generic and generated application traces must agree.

## Costs and lifetime

`stats()` separates candidate visits, template interpretation, structural tests, copied binding slots, dispatch/cursor steps, pool materialization, history checks, variable/dependency updates, activation events and arena/binding work. These are heterogeneous counters, not interchangeable time units or complete allocator accounting. `retention()` reports logical store, pending-stack, history, queue, dependency, arena, cursor, trace and audit records; these are not heap bytes.

A lightweight application trace is retained in every mode. `enable_audit()` additionally captures source views before and after commits; audit traversal counts are separate. State, history, interned terms and traces remain owned by the engine after completion or failure. Dropping the engine reclaims them. A cutoff preserves the cursor for resumption; dropping it cancels the whole experiment. There is no separate externally injected update or partial-cursor cancellation API.

Before performance comparisons, include source generation, native compilation, query preparation, execution, extraction and destruction at explicit lifecycle boundaries. Current construction prepares rules afresh for each engine; reused-ruleset measurements require explicit reusable preparation for both generic and native modes. Current partner access is predicate-only: bound-key indexing remains an important matched control. Chain keys are unary and grow with `n`, so store size and term size are coupled. Flat-key workloads and binding-aware index maintenance must be considered before architecture rankings. No generated comparative matrix or cost run is part of these hand checks.

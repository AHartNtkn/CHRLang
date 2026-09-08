# Direct graph source execution passes the first complete-answer gate

The direct graph now executes source rules and matches independent complete answers on 40 finite configurations. It also publishes a finite sibling while another context continues, without reporting exhaustion. **This establishes a working candidate for comparison; it does not establish an architectural performance advantage.**

The [registration](../registrations/S03-source-executor.md) fixes the interpretation and checks. The [source executor](../../../research/chr-direct-choice/src/engine.rs) uses the graph's contextual equality and occurrence ownership directly. The [shared source gate](../../../research/chr-direct-conditional/tests/direct_graph_entry.rs) checks it alongside Conditional and the independent owned-syntax scalar executor. Neither comparator supplies the graph's implementation.

## Complete results

| Source family | Required behavior | Result |
|---|---|---|
| Recursive words, one or two calls; binary, nested or duplicate choices | Complete values, independent calls, repeated-use correlation, joint unknown aliases, raw multiplicity and rejected outputs | All 28 configurations match 290 expected raw answers per executor |
| Effectful OR with competing consumers and delayed requests | One token is observed, then consumed in the appropriate alternative; a late request retains eligibility only there | Both complete answers match, including output/residual identity |
| Repeated head variables and equality guards | Independent unknowns remain unbound; later source equality enables matching | All four pre/post-binding configurations match |
| Propagation over equal-valued occurrences | One occurrence cannot fill two distinct heads; two occurrences enable both ordered tuples exactly once | Both configurations match, including two equal-valued residual results |
| Fresh body-local unknowns | Separate applications allocate distinct unknowns; repeated uses within each application agree | Complete residuals match |
| Disconnected explicit failure and immediate/indirect cycles | Output-shaped data cannot publish before the pending contradiction | Each of three sources publishes only its good sibling |
| Mixed kept and consumed heads | Shared token survives both requests and remains jointly aliased with outputs and results | Complete answer matches |
| Continuing recursion beside a finite sibling | Publish the finite nonground answer without claiming all search finished | Exactly one answer during 20,000 steps in both graph and Conditional |

The 40 finite configurations produce 303 expected raw answers per executor per run. This count includes all residual occurrences and output relationships, not selected result roots alone. The word expectations come from a mathematical enumeration; the additional cases have explicit hand-derived answers. Every finite case is also checked against the independent scalar executor. These sources agree under the compared schedules; the engines need not agree on all programs with competing committed schedules.

Default, fresh replay and metrics-off source gates pass all nine test functions. All twelve graph kernel/equality tests also pass. Strict Clippy passes for the graph package and source test in both comparator configurations; formatting passes. No process reached its 60-second bound. [Commands, exits and source hashes](s03-source-executor/validation.json), [source results](s03-source-executor/source.log), [replay](s03-source-executor/replay.log), [metrics-off](s03-source-executor/off.log). Test durations are not comparative timing evidence.

## How source execution maps to the graph

**Matching captures nodes and demands constructors without binding the store.** Repeated pattern variables and guards use existing finite-tree equality. A successful application claims distinct consumed occurrence IDs in its matching context. Pure propagation records the rule/occurrence tuple and covered contexts. Kept heads remain available. Rule-local unknowns are fresh for each application and shared by its body goals.

**Choice has both value and effect responsibilities.** An OR tree made entirely of equations assigning the same source variable becomes a named-choice term and one graph equality. General effectful OR creates context-scoped pending goals. This distinction is explicit: the pure-assignment lowering can save interpretation work independently of graph sharing, so a graphless direct control is required when measuring that family.

**Publication depends on contextual quiescence.** Each step executes a pending goal, then selects the first enabled rule/tuple in each uncovered context. Pending bodies prevent publication in their contexts. A context with no remaining obligation or enabled application can publish its full output/residual observation even while a disjoint context remains active. Published and failed contexts cannot perform further source work.

The source scheduling policy permits rule service between pending-goal operations. It is not an enumeration of all rule schedules or a claim that every program has a unique answer across permitted schedules. The gate preserves the source's resource and equality obligations without requiring physical branch copies or diagnostic leaf identities.

## Limits relevant to the next comparison

The implementation still scans matching candidates and may materialize many candidate tuples in one step. Equality, observation and publication perform size-dependent finite work. The finite-sibling result does not prove a constant service latency or bounded-memory ongoing execution. Observation currently materializes complete answers for a finished region before returning them individually.

Consumed rows are removed from the active matching map, but occurrence slots, graph nodes, binding regions, propagation history and published regions can remain retained until query disposal. Reclamation and sustainable stream costs remain S08 questions. Preparation currently retains a shared ruleset; this is not native compilation or a measured preparation advantage.

These results cover the registered sources and earlier kernel/equality catalogues. They are not a general source-correspondence proof, nor a test of every distinct pull-tabbing, memoization or derivation-event design in S03.

## Next decision and competing work

A bounded lifecycle comparison now has more value than further graph-specific tuning: for the first time this candidate can execute complete resource-sensitive source cases against the existing controls. Before timing, validate competent compiled controls on those sources, distinguish pure-assignment lowering from graph sharing, and register both substantive shared work and immediate-discrimination/no-choice overhead. Charge preparation, changing queries, execution, first/full observation and disposal.

The strongest ready alternatives remain S01's selective/consuming lifecycle comparison and S02/S06's integration/direct-compilation work. The selected next graph comparison tests a newly available whole execution organization, whereas more implementation polishing would only elaborate its current design. Reassess after that bounded comparison; no graph-specific sequence has automatic priority. T062 and the overall goal remain active.

# Suspended source execution works on a checked fragment; scheduling remains consequential

**The first demand evaluator now executes source-derived application nodes and preserves their results across compatible demands.** Two counterexamples establish why it needs separate value-demand and source-completion paths, and why general rule competition cannot be replaced by ordinary function evaluation. This is executable semantic evidence, not an architecture ranking.

## What is implemented

The [demand module](../../../research/chr-direct-choice/src/demand.rs) compiles equation-producing CHR rules into suspended calls. It accepts single removed heads, linear nonoverlapping input patterns, a distinct output variable, output equations, checked calls, choices and failure. Bodies can produce intermediate call results. Distinct applications instantiate fresh locals; calls retain results with their choice context and may reuse a result in a compatible descendant context.

The evaluator demands constructors while matching. Unknown arguments can travel opaquely. Source choices receive dynamic labels, and tasks record selections so repeated uses stay correlated. Branch-local producer bindings have separate ownership. A round-robin task queue and an independently serviced list of active source obligations prevent output demand from being the sole driver of execution.

Every active producer must finish before publication, even when its value is not an output. However, a consumer can inspect an available constructor before all producers beneath it finish. These are separate responsibilities: knowing the outer value does not prove source completion.

This module implements context-indexed suspended-call evaluation. It does **not** yet implement the local graph transformation of classic pull-tabbing, general consuming CHR, or derivation-template reuse between distinct applications. It creates no replacement reference interpreter. Those distinctions remain required in T071.

## Two findings that affect the architecture

**Demand can change which source rule wins.** In the counterexample, `read(a,R)` has a specific rule and `read(X,R)` has a general rule. An independent `make(X)` will later establish `X=a`. The scalar source oracle fires the already-enabled general rule and returns `fallback`. Forcing `make` before selecting `read` would instead enable the specific rule. The compiler rejects this overlapping case because its current translation has no scheduling proof for it.

This does not establish nonoverlap as a language requirement. It establishes a responsibility for the broader candidate: preserve permitted source competition when servicing a demand, or prove eligibility for reordering. The existing source controls can express this program; the current demand compiler cannot admit it.

**Waiting for complete producer evaluation can prevent useful refutation.** Another source produces `box(Z)` while a producer for Z continues indefinitely. A separate rule rejects any box. The independent scalar oracle refutes the query. Initially, the demand evaluator waited for Z's producer and failed the bounded service test. With constructor availability separated from source completion, it exposes the box and refutes the query within 128 ticks. It still refuses to publish an answer with an unfinished producer.

An additional test establishes that failure independent of an ongoing output is serviced, including failure nested in a producer body. This guards against treating only output-reachable computations as source obligations.

## Validation and limits

The [registered gate](../registrations/S03-suspended-source-gate.md) and its implementation-stage extensions have nine passing tests with default features and with the test package's default metrics disabled. The tests compare ten finite inputs against hand-derived answers and the independent scalar oracle: six recursive shared/independent cases, fresh locals, unused failure, sibling producers, and duplicate nested choices. They account for 24 raw answers. The constructor-refutation witness adds an independently checked empty result; the overlapping-rule witness checks the source answer and compiler rejection.

Other tests require finite-sibling publication without exhaustion, prompt independent/nested failure, honest unfinished-demand events, and rejection of multiple writers, cyclic producer graphs and effectful sources. The branch-local alias counterexample failed before the ownership repair. The progress/refutation counterexamples failed before the obligation/value separation. Existing direct graph tests and source gates also pass; the reference implementation was unchanged.

Receipts: [default gate](s03-suspended-source/default.log), [metrics disabled](s03-suspended-source/metrics-off.log), [graph regressions](s03-suspended-source/graph-regression.log), [source regressions](s03-suspended-source/source-regression.log). The [tests](../../../research/chr-direct-conditional/tests/suspended_source.rs) retain the counterexamples. No comparative timing, allocation or compilation measurement has run for this module.

The implementation is incomplete for architecture comparison. It rejects kept/multihead rules, guards, overlapping clauses and unsupported output definitions. A demanded unresolved call reports `Stuck` and remains queued; the evaluator does not yet publish general residual-call answers. Context keys are conservative, retained results have query lifetime, and recursive demand traversal can grow with the computation. Passing bounded service witnesses does not establish a constant per-tick bound or sustainable memory.

## Next decision

Continue T071 by integrating source competition, consuming effects and residual obligations with these suspended applications. That integration must retain the identity countermodels from the [preceding experiment](S03-derivation-identity.md). It should also establish whether explicit pull-tab rewrites add a distinct mechanism beyond the current task/context evaluator, rather than assigning that name to this implementation.

The next comparison must expose the work actually avoided by demand and result retention, with immediate discrimination and low reuse as contrary cases. Use corrected graph and compiled controls, plus applicable direct lowering; test registration follows correctness and sizing. This source compiler can itself remove interpreter work, so a favorable cost must be attributed between lowering, demand and reuse.

Contextual equality/local consuming rewrites remain the strongest following alternative. Today's implementation makes the source scheduling and effect integration question concrete enough to investigate directly; another pure-value gate would not resolve it. T071, S03 and the overall research goal remain open.

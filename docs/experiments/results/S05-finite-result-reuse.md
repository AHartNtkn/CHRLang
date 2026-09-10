# Result reuse now uses the same finite execution plan as learning

A bounded result table now reuses completed private phases through the existing direct finite solver. The 432-query matrix agrees with independent complete-answer checks in both build configurations. This supplies the missing semantic control for learning; its total cost remains unmeasured.

## What changes in the comparison

**Executor choice no longer has to decide the reuse-versus-learning comparison.** Cache misses run the existing `finite_phase::Prepared` machine. They match ordinary finite execution's source-step and partition counts throughout the matrix. The existing bundled persistent call table remains a distinct complete-path competitor, with its own executor costs.

**The key recognizes renamed private queries while preserving their dependencies.** It records ordered private constraints, producer names, constants and repeated-variable relationships. One cache borrows one immutable prepared source. Outside caller constraints and output labels are excluded from recognition. Different domains and alias patterns remain different keys; equivalent domains expressed through different producer syntax may miss. This is exact private-query reuse, not logical equivalence, generalized failure-region learning or arbitrary continuation projection.

**Results belong to the private interface, not the first caller.** On a miss, the solver observes every private input variable, including variables absent from named outputs. The cache stores their normalized resulting terms and each alternative's multiplicity. Each use rebuilds the current caller's constraints and original output equations. The finite source checker forbids fresh body variables, so returned variables must lie within this interface. The existing call table's general fresh-result mechanism remains separate evidence.

This distinction matters in the adverse tests: caller-only variables remain unrelated, private variables used only by consuming caller constraints still receive their bindings, and two private variables unified to the same unbound value remain jointly aliased after renaming. Transport skips identity bindings so they do not create a self-resolving loop.

## What was tested

The [registration](../registrations/S05-finite-result-reuse.md) fixes three accepted-pair masks, two weights, two prefix depths, nine domain pairs, two alias cases and two variable-number ranges. There are 12 prepared sources and 36 changing queries per source. Each source records 18 computations followed by 18 renamed hits, including successful and failed results. The matrix uses the existing learning workload constructors, with changing outside markers and output labels.

Every result is checked against the independent scalar interpreter and compiled ordinary execution after full caller resumption. Independent enumeration of the nine ground pairs also checks weighted answer counts. The [tests](../../../research/chr-direct-conditional/tests/finite_learning_gate.rs) include these additional challenges:

- A query cancelled after progress and a query exceeding its step bound install no result. A later complete query still succeeds.
- A suspended private phase is an error and installs no result.
- A hit still obeys input admission and the solution bound. A failed bounded replay does not corrupt its retained completed result.
- Capacity zero recomputes without retaining entries. Capacity one evicts the first key when a second is installed; returning to the first requires source work again.
- Unobserved private bindings and unbound interface aliases survive changed callers.
- Diagnostic and ordinary instantiations return the same complete answers and source-work counts. Diagnostic counters stay zero in the ordinary instantiation.

All 16 learning/reuse tests pass in both feature configurations. Existing finite-phase, bridge and future-caller tests also pass in both configurations. Clippy passes in both configurations. The [receipts](s05-finite-result-reuse/) include the initial missing-module compiler failure, intermediate checks and final validation; the initial failure establishes the missing API, not an experimental semantic counterexample.

## Responsibilities and costs still to measure

The [implementation](../../../research/chr-compiled/experiments/finite_reuse.rs) adds ordered key construction, linear lookup in a bounded FIFO table, interface normalization, caller transport and result ownership. It does not add another solver. Entry capacity bounds the number of keys, not retained bytes; one successful entry can contain many weighted alternatives. FIFO hits do not refresh entry order.

Every start still uses ordinary finite-query admission and domain preparation, even on a hit. A hit avoids source execution but performs key lookup and complete transport. Misses additionally construct canonical result rows. These costs must count in the lifecycle comparison. Zero source steps on a hit is not zero time or allocation.

Input admission, key traversal, result normalization and transport use bounded term traversals; these are separate uses of the configured term-node limit, not one aggregate lifecycle budget. Source steps and partitions remain operational bounds on execution actually performed. A hit does not replay those operations, but does enforce the solution bound. A single advance may perform complete result transport, so this gate establishes cancellation between service calls, not bounded latency within transport.

No timing or allocation ranking follows. Preparation, setup, retained successful and failed results, eviction, caller execution, observation, cancellation and disposal remain to be qualified. Stable memory across repeated changing queries and consumer retention is not established by these semantic tests.

## Next experiment and architectural disposition

Select allocation/ownership qualification of recomputation, eager and covered failure learning, and this result table under the same direct finite preparation. Include repeated normalized shapes, changed domains, sparse hits, result-heavy successes, entry capacities, cancellation followed by reuse and consumer retention through preparation disposal. Keep the bundled persistent table available for the later complete-path contrast. Register primary timing only after the ownership endpoints and counter-free configuration are qualified.

Broader effectful capacity solving remains the strongest ready alternative: it could eliminate additional execution but requires a new source/resource correspondence. This result table now resolves the executor confound at the semantic level; whether its retained successful results are economical is directly consequential to the existing learning findings. The ownership comparison is therefore the next bounded package. Reconsider broader capacity phases, compilation and lifetime at that boundary or an obstruction.

This is the second experimental package since the sequence revision. General continuation relevance, richer source effects and all other distinct architectural obligations remain open. The research goal remains active.

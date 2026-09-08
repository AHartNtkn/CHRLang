# Subscription policies now preserve complete source answers

Indexed discovery, eager retention and demand-driven retention now execute the same three-relation source program and preserve its complete answers. The gate checks binding changes, demand lifetime, occurrence multiplicity and consuming order. This establishes executable competitors for lifecycle measurement, without yet ranking their costs.

## What is integrated

The [checked join kernels](S01-subscription-kernel-gate.md) now sit inside an exact-source runtime. Preparation verifies either the propagating or right-consuming ruleset. Each query owns its rows, demands, bindings, indexes, retained tuples, driver instructions, pending pulse and receipts. Reusing preparation creates a fresh query owner.

The source contract is unchanged: open a demand selecting a left key and final value, issue pulses to observe matching three-row paths, update data or bindings between pulses, and retire demands explicitly. A missing removal or close blocks the remaining script. Equality failure produces no answer. Duplicate demand and data occurrences remain distinct resources.

Bindings update data projections and demand keys without assigning new occurrence identities. Structural unknowns stay outside the join index until the required constructor becomes visible. The implementation scans rows and demands to identify changed projections, then invalidates and reinserts only those projections. The scan and binding-map copy are real costs to include in the later binding-density comparison.

## Complete-answer evidence

The matrix and directed cases cover 49 source configurations. Each runs all three lowerings with service quanta 1 and 37, yielding 294 full-answer comparisons against the independent owned-syntax evaluator. Existing compiled Global Scan and Indexed controls also agree. Hand-stated receipt expectations and selected complete residual expectations remain independent checks on the source evaluator.

Additional tests reuse preparation across changed queries, cancel at five execution prefixes before starting a fresh query, and check occurs failure and constructor clash. These establish query isolation and semantic failure behavior; allocation restoration after cancellation still belongs to the lifecycle meter.

The binding cases change both demand endpoints and shared data, enable matches through aliases and constructors, and resolve values already stored in receipts. The consuming-order case deliberately opens demands in a different order from the left-row order and gives those rows different receipt payloads. A demand-first implementation would choose a different observable receipt, so this test detects an actual scheduling error.

Four deliberate source-effect faults fail release assertions: skipping right consumption, leaving demand keys stale after binding, leaving row projections stale after binding, and processing consuming candidates in demand-first order. The original runtime is restored and rerun successfully. Metrics-on and counter-free release tests, plus counter-free Clippy, pass.

## The existing specializer does not add a distinct control here

The current inferred specializer requires every use of an admitted predicate to have one removed head and no kept heads. Every active predicate in this source participates in a multihead rule: the data/demand/pulse join, pulse acknowledgement, or driver/resource removal. Its eligibility check therefore admits no region in either source variant.

Forced admission is rejected, and inferred execution performs zero specialized candidate visits while agreeing with the source. This is a limitation of the present single-head mechanism, not evidence that generated multihead access is impossible or uneconomical. The next cost comparison should include competent indexed discovery and the existing compiled Global/Indexed path; it must not label the empty inferred selection as an additional specialized competitor. Broader generated access remains an S01 obligation.

## Responsibilities and remaining measurement risks

The runtime implements source scheduling, equality, row/demand identity, blocked-driver residuals and observation. The join policies change which matching results are maintained. Eager and subscribed policies use reverse incidence to invalidate retained triples; subscriptions additionally own demand registrations and endpoint indexes. Indexed discovery avoids those retained tuples. The source-specific runtime replaces generic rule selection for this exact admitted program; that specialization and its preparation cost must be explicit in any comparison with a generic executor.

A pulse currently materializes its candidate tuples, orders them according to source priority, and services them incrementally. Distinct demand occurrences each receive their own candidates. Right consumption invalidates future matches, and pending candidates whose right occurrence has been consumed are skipped. Because updates cannot interleave within a pulse under this source schedule, traversing its ordered candidates once represents propagation history without a separate history table. This reasoning does not extend automatically to arbitrary interleaved CHR programs.

Candidate materialization, sorting and pending storage may become consequential at high fanout. Measure their contribution and investigate it before attributing a loss to retention or rediscovery. Binding scans, source-script ownership, answer retention and preparation also remain visible cost obligations. No performance comparison has run for these integrated candidates.

## Next experiment

T068 remains active for prospective lifecycle measurement. Vary requests per live demand independently from inactive data volume, update density, final selectivity and intermediate fanout. Include demand retirement/reopening, unique short-lived demands, broadly affected active demands, binding-enabled matches and right consumption. The favorable subscription witness needs both repeated useful work and unused join combinations; its adverse witnesses need maintenance without useful repetition.

Before comparative timings, fix exact source generation, configurations, repetitions, endpoints and bounds. Charge preparation, query setup, execution/observation, cancellation and disposal with ordinary-allocator counter-free release timing; obtain work and requested-heap diagnostics separately. Compare consequential corrections and the strongest ready alternative before expanding this matrix.

## Evidence

- [Runtime](../../../research/chr-compiled/experiments/subscription_runtime.rs), [source gate](../../../research/chr-compiled/tests/subscription_source_gate.rs), and [validation runner](../../../research/chr-compiled/experiments/subscription_source_validation.py).
- [Release checks](s01-subscription-source/release-metrics.json), [counter-free checks](s01-subscription-source/release-counter-free.json), [Clippy](s01-subscription-source/clippy.json), [restored runtime](s01-subscription-source/release-restored.json), and [source hashes](s01-subscription-source/source-hashes.json).
- Exact faulty substitutions and their failing test receipts accompany those files. [Specialization admission](s01-subscription-source/specialization-admission.json) records the checker reasons and zero specialized work.
- The earlier kernel evidence remains tied to its [recorded commit](s01-subscription-kernel/provenance.json); it is separate from the current integration evidence. The reference interpreter and independent evaluator are unchanged.

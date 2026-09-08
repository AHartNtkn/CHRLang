# R04 finite backends: source and inference gate

Reusable native arc-consistency search and incremental support/conflict CNF solving now preserve the registered finite source observations. Support-CNF unit propagation matches native arc consistency on the exhaustive binary relation/domain gate; conflict-only inference does not match it on the unsupported-value witness. This establishes credible controls for a performance comparison, not a solver or architecture winner.

[Registration](../registrations/R04-finite-backend-semantic-gate.md), [source receipt](R04-finite-backend-gate/source.json), [tests](R04-finite-backend-gate/final-tests.log), [environment and dependencies](R04-finite-backend-gate/environment.json). Implementation is under `research/chr-finite`; it depends on source syntax and the installed Z3 library, not the reference evaluator.

## Exact relation and query ownership

A prepared problem retains the original forbidden-occurrence multiset and compiles its inference representation once. Reverse endpoint order transposes the table. Repeated restrictions combine for inference while remaining repeated in observations. Equal endpoints and constant endpoints produce unary restrictions or explicit inconsistency. Query domains carry an independent inconsistency flag, so a zero-variable contradiction is distinct from the single valid empty assignment and from invalid input.

Prepared structures are immutable to callers. The common relation does not construct native indexes for Boolean consumers. Native preparation owns directed support tables and affected-arc incidence, which survive changing queries. Native enumeration owns domain masks, a reversible trail, decisions and a coalesced arc queue; it streams complete assignments. It does not materialize the Cartesian product or clone a full domain vector per decision. This is a selected native control, not a proof of optimal native search organization.

The Boolean control uses one-hot exactly-one values. Support encoding includes implications in both directions over the combined table; conflict encoding includes forbidden pairs. Prepared clauses/atoms survive changing query domains. Each query pushes a scope, adds domain exclusions, enumerates full assignments and blocks each complete assignment, including hidden variables. Query drop pops those exclusions after either early abandonment or complete exhaustion. UNKNOWN propagates as an error rather than UNSAT.

The installed library reports Z3 4.8.12.0. The wrapper uses a reference-counted context and explicitly owns solver, AST and model references, including model-evaluation results. Query scope and context references follow the [official C API contract](https://z3prover.github.io/api/html/group__capi.html). Header/library hashes are retained. This is an incremental Z3 Boolean control; its heuristics and deductions can differ from the independently checked plain unit-propagation mechanism.

## Independent evidence

All 344 source queries pass on native, support-CNF and conflict-CNF backends: 1,032 query/backend checks. The registry includes the existing 338 finite cases plus unsupported values, a propagated equality chain with changed givens, asymmetric tables and an arc-consistent contradiction. Every delivered raw observation is compared with projections of the independent oracle's satisfying assignments. The comparison retains multiplicity per observation, all residual copies, hidden choices and exhaustion. Source eligibility checks reject uncovered variables and invalid output declarations before finite-domain construction.

Every one of the 512 two-variable binary tables is tested with seven supplied-value configurations against independent exhaustive assignments. Both Boolean encodings pass all 3,584 queries each, and native results agree on the same cases. The prepared Boolean solver is reused across the changed givens. Separate tests abandon enumeration early and then exhaust a fresh query, including empty assignments and contradictory ground givens.

For each table, all 49 pairs of nonempty domain masks are tested: 25,088 inference cases. Native AC and plain support-CNF unit propagation both equal the independent projection of satisfying original-table pairs, or both report contradiction. The unsupported-value witness distinguishes support from conflict encoding: support/AC excludes a value lacking any partner before a neighboring assignment, whereas conflict-only unit propagation initially leaves it possible. Therefore conflict-only timing cannot stand in for an inference-matched Boolean control.

Seven module tests and three exhaustive integration tests pass. Workspace all-target tests and Clippy pass; final package tests/Clippy and formatting pass after strengthening the inference oracle. Read-only review checked normalization, restoration, ownership and source boundaries. The reference evaluator remains unchanged and independent.

## Interpretation and next comparison

The gate makes R04's complete-path comparison feasible. It does not measure preparation, runtime, retained heap, model enumeration or observation cost. The source adapter collects results for validation only; the underlying native and Boolean APIs stream them. A performance runner must time preparation, query setup, execution to each result, actual reconstruction, query disposal and prepared disposal separately, with full correctness checks outside measured intervals.

Initial native propagation currently occurs during query start, while Boolean initial solving occurs on the first result request. Compare complete first-answer/lifecycle costs and disclose this phase placement; execution-only labels would not identify matched work.

Reuse requires particular care. Native prepared topology and Boolean clauses can survive changed givens. The compiled control currently prepares source rules; a credible fixed-topology reuse comparison must also permit its corresponding reusable query state, or restrict and label the comparison as cold topology rather than attributing repeated setup to source execution. Charge all preparation and retain a cold-query result; do not invent an application reuse frequency or aggregate winner.

Rust requested-allocation diagnostics cannot account for Z3's external allocations. Use process-wide memory evidence for cross-backend comparison and keep requested-Rust allocations as a separate diagnostic. Native queues/trails, Boolean clauses/learning/blocking, and compiled occurrences/indexes/frontiers have different lifetime costs. Query pop correctness does not establish bounded physical memory retention across long runs. These are measurement obligations, not reasons to infer a result from semantic success.

Select that prospective lifecycle pilot next, including a competent compiled fixed-topology control and matched cold/reuse boundaries. The strongest alternative remains direct conditional activation with resource-sensitive sharing. The ready finite semantic gates now offer a lower-cost way to test whether inference or logical compilation eliminates meaningful execution, while the conditional protocol still has more simultaneous semantic uncertainties. Native compilation amortization, conditional execution, broad language restrictions and parallelism remain open; this finite fragment does not dictate language adoption.

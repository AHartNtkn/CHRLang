# Source-derived capacity solving avoids a choice product in the tested fragment

The new solver infers weighted consuming-capacity relations from source and reproduces the independent complete-answer matrix. A domain-capacity bottleneck rejects 24 independent binary choices before any assignment branch. This establishes a different execution mechanism; preparation, allocation and total efficiency remain unmeasured.

## What the candidate actually does

The [implementation](../../../research/chr-compiled/experiments/resource_capacity.rs) depends only on source syntax and standard collections. Preparation recognizes a closed sequence of finite producers, a two-head consuming rule and its lower-priority unmatched-need failure sink. It derives producer domains, token/need/output predicates and duplicate-choice weights from rule syntax. Predicate and atom names are data, not benchmark identifiers.

Query setup groups producer occurrences by initial variable identity, intersects their domains and multiplies their choice weights. An aliased group consumes one token per occurrence, all at the same chosen value. Ground producer arguments form fixed-value groups. The solver retains unused tokens and substitutes chosen atoms into ordinary residual terms and output variables.

Before branching, the solver excludes values that cannot satisfy an entire alias group's demand. For each distinct remaining domain set, and their union, it compares available capacity with demand from groups whose domains are contained in that set. If demand exceeds supply, that state cannot succeed. These are sound necessary inequalities, not a claim to implement every possible consistency check. Feasible states proceed to bounded assignment extraction; complete successful answers retain raw derivation multiplicity.

This replaces source occurrence matching and consuming-rule execution with source admission, grouped domains, capacity checks and backtracking. It still needs source/query validation, weighted answer extraction and output ownership. General source execution and arbitrary equality are not hidden behind an engine call; they are outside the admitted fragment.

## The discriminating witness

Twelve independent choices have domain {alpha, beta}; another twelve have {gamma, delta}. Supplies are 5 alpha, 6 beta, 6 gamma and 7 delta tokens. Total supply is 24 for 24 needs, but the first twelve needs can use only eleven tokens.

The solver reports one visited state, zero assignment branches and one capacity rejection. The syntax has a potential complete choice product of 2^24. That figure is a mathematical product, not a measured source-engine work count or speed ratio. A smaller corresponding source with two choices in each domain is independently executed and also has no answers. The large rejection is additionally justified by the explicit twelve-demand/eleven-token inequality.

An unselective four-choice control with sufficient supply must return all 16 raw answers. It does so, agreeing with independent scalar and compiled source execution. The implementation therefore does not obtain the rejection result by suppressing required answer enumeration generally.

## Correspondence and adverse checks

All 1,536 cases from the [semantic entry](S06-resource-capacity-entry.md) now also compare the inferred solver with the independent mathematical relation. The existing source checks still compare full answers with scalar source semantics and ordinary compiled Global Scan. The matrix covers two name/variable renamings, choice weights one/two, independent and fully aliased variables, three domain profiles, zero to three requests and independently varied token supplies.

Additional tests cover partial alias groups, ground producer arguments, substitution into structured inert residuals, arbitrary atom names, reversed consumer head order and reuse of preparation across changed queries. A failed solve caused by a state/output limit does not poison a later successful query. Limits return an explicit error rather than a partial answer vector advertised as complete.

The source checker rejects absent sinks, competing token consumers, token replenishment in producer bodies and multiple producer rules for the same predicate. Query admission rejects initial needs, nonground token keys, duplicate output names and too many alias groups. The preceding source counterexamples still demonstrate why missing sinks and competing owners cannot be treated as ordinary capacity failure. The alias witness still distinguishes two independent successful allocations from an impossible shared-variable allocation.

Five tests pass in default and metrics-off builds; scoped Clippy passes. The test first fails with the candidate module unavailable and passes after implementation. [RED](s06-resource-capacity-solver/red.log), [initial GREEN](s06-resource-capacity-solver/green.log), [final default](s06-resource-capacity-solver/default.log), [final metrics-off](s06-resource-capacity-solver/off.log), [Clippy](s06-resource-capacity-solver/clippy.log), [frozen evidence](s06-resource-capacity-solver/freeze.json). No reference or existing execution-engine implementation changes.

## The exact language and measurement limits

This implementation accepts an entire closed source consisting of unary finite producers, one unary-keyed two-head consumer and one failure sink. Producers have priority over consumption. It does not yet infer a private phase inside arbitrary surrounding rules, support kept observers or guards, model replenishment, or accept constructor-valued or unknown token keys. Inert residual constraints are preserved because the accepted source has no rules that operate on them. These are sufficient experimental admission conditions, not proposed mandatory language restrictions.

Preparation allows at most 4,096 rules, bounded syntax traversal and at most 64 values per producer. Query traversal is bounded, with at most 10,000 constraints/outputs and 64 identity groups. Default search limits are 100,000 states and 4,096 raw answers; multiplicity arithmetic is checked. A state may perform many set/collection operations. These limits do not supply constant-time service or a byte-accurate memory bound.

The current interface is synchronous and owns a complete answer vector. It has no incremental cancellation or streaming claim. States enforce an operational limit; branch and capacity-prune counts establish mechanism evidence and are not yet qualified as counter-free timing instrumentation. Source inference, group construction, repeated domain sets, answer expansion and disposal could outweigh avoided execution on small or unselective inputs.

## Next selection: qualify complete costs before broader admission

Continue T073 with bounded lifecycle and measurement qualification for this candidate against competent ordinary execution and any applicable existing lowering control. Charge source construction/inference, reusable preparation, changed-query grouping, capacity checking, successful extraction and all owners' disposal. Separate operational limits from diagnostic counters for primary timing. Include cold, reused, infeasible and output-heavy sources; preserve explicit unsupported and resource-cutoff outcomes. Do not time only the root capacity test.

Renaming-aware call reuse remains the strongest distinct alternative. It could change the earlier learning comparison, but it cannot provide an initial failure fact without solving or deriving it. The capacity candidate now directly demonstrates cold-query elimination and has an independent oracle, so a bounded accounting gate can change whether that mechanism is worth carrying into an architecture. Select that gate first; compare any subsequent cost pilot with call reuse and broader compilation at qualification or an obstruction, before increasing the admitted language or running a broad timing sweep.

A measured gain would still require lifetime, richer source boundaries and coherent architecture challenges. A loss would require separating inference/checking overhead from consequential correctable choices. This gate selects no universal solver, cache or architecture; the research goal remains active.

The [registration](../registrations/S06-resource-capacity-solver.md) defines the mechanism and controls. Reproduce with `cargo test --offline -p chr-direct-conditional --test resource_capacity_entry -- --nocapture`, then repeat with `--no-default-features`. Test commands used a 180-second wall bound and independent source executions retain the 200,000-step bound.

# Consuming choices admit a capacity relation, but aliases and failure semantics matter

A finite assignment relation reproduces complete outcomes for the tested closed consuming-choice sources. It must preserve grouped demands from aliases, leftover resources and raw derivation multiplicity. This establishes a semantic entry for a direct resource solver; no automatic source checker or solver cost result is delivered here.

## What is new relative to existing resource evidence

| Existing mechanism | Established scope | Distinct question here |
|---|---|---|
| [Resource fusion](S06-resource-fusion-gate.md) and its [cost pilot](S06-resource-fusion-lifecycle.md) | Count certification lets a producer and its guaranteed private consumer fuse; certification costs can outweigh execution savings. | Can resource infeasibility eliminate choice combinations rather than merely certify and fuse guaranteed execution? |
| [Source resource counting](S10-resource-count-lifecycle.md) | Removing a certified traversal depth saves deeper work but adds shallow preparation overhead. | Can jointly constrained demands and supplies replace a consuming search phase? |
| [Finite phase solving](S06-finite-phase-gate.md) | Source-derived finite domains and deterministic single-head consumers, followed by caller execution. The current checker rejects private consumers with two removed heads. | Can direct capacity reasoning account for actual shared token consumption, including aliases and residual resources? |
| [Compatible-query learning](S06-learning-costs.md) | Retaining whole-query failures pays in some measured regimes. | Can direct constraints avoid executing the initial infeasible choice combinations at all? |

These are differences in the work performed, not a claim that the new relation will be faster. Existing competent execution and source transformations remain controls wherever they admit the source.

## The source and independent relation

Three producer rules choose a, b or either value, then post a need. Producers have priority over the consumer `need(X), token(X) -> done(X)`. A lower-priority failure rule rejects any need that cannot be consumed. No other rule owns these private predicates. Queries provide ground tokens plus unrelated noise; answers expose chosen values, done occurrences, spare tokens and an unused output variable.

The mathematical oracle enumerates finite assignments without executing source rules. It checks each producer's domain, enforces equality for aliased query variables and counts the needs assigned to each value. An assignment succeeds exactly when its demand does not exceed that value's supply in this closed source. Duplicate choice arms multiply derivation count; equal token occurrences do not create nondeterministic matching alternatives. Each accepted assignment retains its exact leftover resources and output aliases.

This enumeration is an independent oracle, not the intended efficient solver. The proposed solver must infer the relation from source and use capacity propagation or a compressed relation to avoid a complete assignment product in a discriminating witness. Merely moving this exhaustive oracle into the runtime would not establish that architectural mechanism.

## Experimental evidence

The [prospective registration](../registrations/S06-resource-capacity-entry.md) fixes 1,536 configurations: zero to three requests; three domain profiles; zero to three a and b tokens independently; independent or fully aliased variables; choice weights one/two; and two predicate/variable renamings. Every mathematical answer multiset agrees with independent scalar source execution and ordinary compiled Global Scan, including finite failure, output aliases, residual token/done multiplicity and unused variables.

Three tests pass in default and metrics-off builds; scoped Clippy passes. Each source execution has a 200,000-step bound and each test command a 180-second wall bound. No cutoff occurs. This is finite correctness evidence, not timing, allocation or ongoing-progress evidence. [Default log](s06-resource-capacity-entry/default.log), [metrics-off log](s06-resource-capacity-entry/off.log), [Clippy](s06-resource-capacity-entry/clippy.log), [source and evidence hashes](s06-resource-capacity-entry/freeze.json).

## Counterexamples define the required checker

**Aliases make resource demands indivisible by value.** With two independent a-or-b variables and one token of each value, two assignments succeed. With two uses of the same variable, no assignment succeeds: both needs must choose a or both must choose b. Total demand and total supply are both two in either query. A matching algorithm that independently splits the aliased uses across a and b would be unsound.

**Insufficient resources need not mean failure in CHR.** With one a-demand and no token, the registered source fails because its unmatched-need sink runs. The corresponding source without that sink has one suspended answer containing the need. A direct solver cannot silently discard such residual states. It must either represent them or prove the source's failure behavior.

**Competing resource owners invalidate a simple supply certificate.** A higher-priority rule consuming tokens causes a query to fail even though the isolated capacity relation predicts one success. A source checker must account for competition, replenishment, private observers and priority; seeing enough input tokens is insufficient.

The entry's closure conditions are sufficient conditions for a future experiment, not mandatory language restrictions. More general resource derivations may account for those effects rather than rejecting the source. Such a claim needs its own source correspondence and language tradeoff evidence.

## Next implementation and why it is worth testing

Continue T073 with a bounded source-derived capacity solver. Infer finite producer domains, the shared consuming relation and the failure boundary from syntax, without predicate-name recognition or query-specific expected answers. Group initial variable aliases, multiply weights from repeated producer occurrences, and charge demand multiplicity against resource supply. Preserve complete assignments, residual resources and raw answer counts. Make unsupported sources, unresolved partial inputs, bounded progress and resource limits explicit.

Its first mechanism witness must detect a jointly impossible capacity demand before enumerating the full source choice product. Its adverse witness must force many successful outputs, where extracting them is unavoidable. Challenge competing owners, absent failure sinks, aliases, unknown keys, multiple values, spare resources and source renamings. Measure work structurally before any performance claim, then register full preparation/query/observation/disposal costs against the strongest applicable execution control.

The strongest ready alternative remains renaming-aware call reuse. That could change the interpretation of the measured learning gains but needs a sound answer-remapping and lifetime implementation. The capacity entry now supplies a precise independent relation and concrete falsifiers for a mechanism that could avoid initial execution entirely. Select its bounded inference/solver gate first; review call reuse and broader compilation at that gate or any consequential obstruction, before a cost campaign or broader source expansion. No solver or architectural winner follows from this entry.

Run `cargo test --offline -p chr-direct-conditional --test resource_capacity_entry -- --nocapture` and repeat with `--no-default-features`. The [test source](../../../research/chr-direct-conditional/tests/resource_capacity_entry.rs) contains the independent oracle and all counterexamples. The reference interpreter and experimental execution engines are unchanged. The architecture goal remains active.

# Resource solving can resume a caller only with the right occurrence order

The capacity solver now supplies an ordered initial-phase continuation that agrees with ordinary execution across 2,880 caller configurations. Two counterexamples show why its closed multiset answers alone were insufficient: an ordinary consumer can distinguish which equivalent-result occurrence appears first. The corrected phase preserves that order and private bindings without replaying the private rules through another engine.

## The boundary has a real semantic obligation

**Generated fact order can change the caller's result.** One query has a `pick2` occurrence with the smaller variable identity and a `pick1` occurrence with the larger identity. Source priority executes `pick1` first, so ordinary consumption produces `done(a)` before `done(b)`. The closed solver groups by variable identity and exports the opposite order. Resuming a caller that consumes the first `done` chooses `b` from that export, whereas the complete original source chooses `a`.

**Unused token order also matters.** With no private requests and initial tokens `[b,a]`, the closed answer exports tokens in value order. A resumed caller chooses `a`; the original source chooses `b`. The closed answer remains a correct multiset. The error is interpreting that value-level endpoint as an execution continuation under the tested source-priority contract.

**Consumer head order determines which order to preserve.** With the token head first and tokens `[b,a]`, the source produces `done(b)` first even when producer priority posts the `a` need first. A producer-order-only reconstruction chooses the wrong fact. The [token-first failure receipt](s06-capacity-phase-entry/token-first-red.log) records that mismatch. The corrected construction handles both consumer head orders; the expanded matrix also reverses token order independently.

The counterexamples run through independent scalar and compiled controls. Their corrected continuations now match the original full source. A language with a different rule-choice or observation contract would need a separate comparison; this gate preserves the current contract.

## The corrected phase derives the ordered residual

The [phase preparation](../../../research/chr-compiled/experiments/capacity_phase.rs) accepts an actual initial source prefix that the existing capacity checker recognizes: ordered finite producers, the consuming need/token rule, and the final need-failure rule. It uses the existing relation solver and an ordered result construction in [resource_capacity.rs](../../../research/chr-compiled/experiments/resource_capacity.rs). There is no engine or oracle dependency in this derivation.

Every producer outranks consumption and is unconditionally applicable on the admitted variable/atom inputs. Thus producers post needs in source-rule priority and original occurrence order. Each value consumes its earliest matching tokens, while the first consumer head determines which value is consumed next. The phase reconstructs the endpoint accordingly:

- Keep inert facts and surviving tokens in their original input order, consuming the earliest required tokens of each value.
- Append generated `done` facts in need-creation order for a need-first consumer, or filtered original token order for a token-first consumer, including duplicate occurrences.
- Transport every producer-variable binding and every original output binding. Caller constraints are substituted even when those variables were absent from named outputs.
- Preserve every weighted source derivation as a separate continuation. Resource infeasibility produces no continuations; a service/output limit is an error.

Initial `done` occurrences remain ahead of generated ones. The private prefix has no propagation rules or fresh body variables; its occurrence history and private fresh identities do not need to be transferred. The resumed ordinary caller may itself propagate, create fresh variables or create more private work. The test driver resumes the full original rules after applying the returned bindings; later private work runs ordinarily.

The closed multiset endpoint remains available for terminal observation. Its existing cost receipts apply to their frozen implementation. Ordered continuation construction has not yet been measured, and the closed endpoint's performance cannot be assigned to it.

## Independent qualification

The [registration](../registrations/S06-capacity-phase-entry.md) fixes three domain profiles, three supplies, shared/distinct variables, ascending/descending variable identities, weights1/2, original/reversed producer query order and five caller kinds: consume a done fact, consume a spare token, observe shared variables, write a shared variable, and create further private work. The prospectively registered extension crosses both consumer head orders and both token orders. All **2,880 configurations** agree on complete outputs and residuals with the independent scalar evaluator and compiled ordinary execution in each feature configuration.

The [executable gate](../../../research/chr-direct-conditional/tests/capacity_phase_gate.rs) adds named-output transport, duplicated-output rejection, limit/error recovery and a before/after priority counterexample. A token consumer before the proposed phase changes successful execution into failure; the prefix checker rejects that source boundary. The same consumer after the phase agrees and receives the spare token.

Four supplemental propagation/freshness cases preserve six ordered propagation applications among three `done` occurrences, fresh alias relationships and selection of an initial fact before generated facts. The six existing capacity regressions also pass, including the 1,536-case independent closed relation matrix, alias demand, capacity pruning and operational limits.

All twelve tests and Clippy pass in default and metrics-off configurations. [Receipts](s06-capacity-phase-entry/) include the counterexamples, the initial missing-phase API compiler failure and final validation. The [manifest](s06-capacity-phase-entry/manifest.json) records final source/log hashes after validation; no comparative timing or pre-run binary freeze is claimed.

## What remains to decide

This is a finite, complete initial-phase gate. It does not establish a resumable live-cursor transformation, arbitrary private source support, bounded interruption during solving or fair incremental publication across ongoing caller branches. The phase returns owned continuations; a production caller must resume the intended source and own scheduling, transport and disposal. Those responsibilities are not justified merely by the existence of a complete answer vector.

At this semantic boundary, select **composition ownership and complete costs** as T073's next bounded work. Compare phase solving plus ordinary caller execution with generic and applicable specialized whole-source controls. Charge phase/caller preparation, ordered residual construction, temporary observed bindings, continuation retention, caller execution, owned outputs and disposal. Include small and output-heavy sources, shared aliases, failure and later private work. Establish ownership and cancellation endpoints before prospective primary timing.

Adaptive reunion remains the strongest distinct alternative: it could change search-state organization, but requires policy and progress qualification. Local resource claims, richer theories and sustained consumers remain required and return at the composition ownership boundary or an obstruction. The present gate adds a concrete ordering obligation that could erase some closed-solver benefit; the existing source and caller controls make testing that cost more immediately discriminating than another unrelated implementation entry.

This is the first package after the matched-reuse breadth review. The broader source/language and architecture questions remain open; the goal remains active.

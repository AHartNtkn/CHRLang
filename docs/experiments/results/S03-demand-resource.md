# Suspended applications can claim shared resources under an explicit policy

**The demand evaluator now executes consuming multihead applications on a checked resource fragment.** Distinct partners are matched without binding, removed partners are claimed atomically within the current choice context, and kept partners remain live. The tests establish complete outcomes, not an efficiency advantage.

## Implemented ownership

The first removed head is an output-producing call. Additional removed heads and kept heads refer to passive resource occurrences. Each occurrence has a birth context and consumed contexts. Repeated demands reuse the application's recorded result; they do not claim its partners again. An unsuccessful tuple search changes no occurrence.

Resource-dependent applications settle active previously born choices before claiming partners. A claim before a rule-body choice therefore applies to both descendants; a claim inside one sibling does not affect the other. This is a conservative implementation boundary. Its context expansion and retained histories must be charged, and finer conditional claims remain possible.

Ground resource posts in rule bodies can enable waiting requests. Unknown resource handles preserve their identity and can appear in returned values. Repeated head variables test established equality; they never bind an unknown key to manufacture a partner.

The compiler currently keeps passive resources separate from call-output writers. Passive query resources cannot refer to call output variables, and passive body posts must be ground. This avoids an unimplemented resource-alias repair protocol. These are explicit certificate limits, not selected restrictions for the language. Pure propagation, overlapping call clauses, arbitrary writable heads and broader effect bodies remain outside this implementation.

## Evidence and a policy difference

The [registration](../registrations/S03-demand-resource-gate.md) fixes the ownership and committed-policy contract. Eleven new finite resource inputs cover one/two tokens, two competing requests, retained heads, atomic two-partner claims, late and conditional posts, consumption before/inside choices, nonbinding keys, and equal/distinct unknown handles. Hand-derived complete answers agree with the independent scalar oracle, compiled global scanned/indexed controls, direct graph and demand evaluator on those cases.

A separate nonconfluent probe has one `a` request, one `b` request and one token. Both applications are initially enabled. Declaration-priority scalar execution selects a. The demand queue services b first and selects b, leaving a residual. Both are valid single committed firings, but their answers differ. Reversing the scalar rule-priority configuration confirms the b outcome. This is a bounded hand-trace and alternate-policy check, not a general allowed-execution validator or equivalence to declaration-priority execution.

This distinction follows the [S00 contract](S00-contracts-and-candidates.md): committed scheduling must be explicit for nonconfluent sources. Future matched timings must use agreeing observations or deliberately compare the two language/policy contracts. The probe cannot be pooled as if it had the same answer on both engines.

The full suspended-source suite has **22 passing tests** with default features and with this test package's default metrics disabled. Earlier residual, alias, disconnected-failure, constructor-refutation and finite-sibling witnesses still pass. Clippy passes with warnings denied. No reference evaluator was changed.

Receipts: [default](s03-demand-resource/default.log), [metrics disabled](s03-demand-resource/metrics-off.log), [Clippy](s03-demand-resource/clippy.log), [source tests](../../../research/chr-direct-conditional/tests/suspended_source.rs). The processes completed within the registered 60-second bound. These are semantic executions; no comparative timing or allocation matrix has run for this candidate.

## Decision and remaining scope

The candidate now connects source preparation, suspended applications, nonbinding matching, context-specific resource ownership, residual observation and finite-service witnesses. It is ready for mechanism attribution and bounded lifecycle sizing on that fragment. A gain could arise from source lowering, demand placement or result retention; the next comparison must distinguish them instead of attributing every difference to sharing.

The [four-package breadth review](S03-first-breadth-review.md) selects that bounded diagnostic next and retains contextual/local consuming rewrites as the strongest following alternative. It does not authorize another long series of syntax gates before checking whether the mechanism does useful work.

Local pull-tab rewrites or an established equivalence, derivation-template reuse across distinct applications, general source competition and resource aliasing, sustained retention and full architectural comparisons remain open. This result neither completes T071 nor selects a demand-driven architecture.

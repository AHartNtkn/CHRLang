# Source-derived finite phase: implementation gate

Implement T073's finite relation experiment. This package measures correctness and
mechanism work, not timings or architectural lifecycle superiority.

Source contract to enforce: a selected initial rule-priority prefix, single
consumed heads, no guards, no fresh body variables, and no outside heads touching
private predicates. Initial unary finite atom choices precede deterministic
private rules. Their alternatives are extracted from equality/Or syntax with
multiplicity. Private bodies may use equality, conjunction and private calls, but
may not create more choice producers. Caller rules resume only after every
surviving private execution completes; suspension, unknown matching, overflow and
resource limits are explicit errors, never empty answers or silent fallback.

Algorithm under test: build weighted atom domains from actual initial choice
occurrences; symbolically partition only when source-priority matching demands a
value; execute deterministic private rules under those restrictions; enumerate
remaining domains at phase completion and transport bindings to the ordinary
caller. Keep duplicate derivations as checked integer weights. A failing private
branch contributes zero. A branch that cannot finish invalidates admission for the
whole call. No output is published before complete admission.

Compare every small completed case against independent owned-syntax scalar
semantics and compiled Global Scan, including resumed caller outputs, aliases,
residuals and raw multiplicity. Reuse prepared rules across query changes.
Cover the existing closed arrival matrix (224 configurations), names/values
changed independently, aliased/duplicate choices, conflicting domains, nonlinear
query aliases, rule competition and output binding. Reject linked private
consumers, source-order violations, dynamic choices, unknown tails, unsupported
heads/guards and ongoing private work. Include explicit term, execution, result
and multiplicity limits. Test a 64-choice selective source with a service bound far
below its 2^64 explicit combinations; do not run the scalar product there.

Initial implementation limits: bounded whole-phase execution, cumulative symbolic
partitions and returned solution rows, checked u128 multiplicities and bounded
term traversal/depth. Exact API limits and measured work are recorded in the
report. Run focused default/counter-free tests, relevant crate tests and scoped
strict Clippy. Each test command has 180 seconds initially; inspect any failure or
cutoff before extending it. No repetitions for deterministic semantics beyond the
two configurations. First observe a failing missing-implementation test.

Selection: directly eliminating alternatives could reverse the measured
conditional/explicit choice-check comparison. This is the immediate next package
selected by the semantic obligations gate. Native graph/connected feasibility is
the strongest distinct alternative; broader reuse and effect precision remain
required. Reassess after a genuine mechanism result, before tuning implementation
or registering costs. No whole-architecture rejection follows from ineligibility.

## Service extension within this mechanism gate

Before any cost experiment, extend the owned work state to advance one symbolic
state at a time. Retain the complete-phase publication rule. Test cancellation
after 0, 1, 16 and 100 advances, completion transfer and terminal error cleanup;
record the missing-API failure before implementation. A service call may scan
several source rules and traverse terms, so no equal-work or constant-latency
claim follows. The synchronous solve operation must drive this same machine.

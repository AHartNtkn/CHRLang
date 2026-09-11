# Caller observers work at reusable phase boundaries

The live trace caller now accepts rules that observe or consume suspended private
facts. All 504 tested sources preserve 31,176 one-step delivery and exhaustion
checkpoints, including caller bindings that re-enable private work. Reuse remains
active; these are not executions routed around the trace mechanism.

**A separate higher-priority private phase can support caller observers.** The
existing suspension protocol already returns its bindings and residual facts to
the original live cursor. This expands the usable boundary without adding a new
per-step observation mechanism.

## Why this boundary works

Private rules precede caller rules in these programs. While a private rule is
enabled, or its body still has pending work, the caller would not select an
observer. When the phase suspends, the trace restores bindings and residual facts
before the live cursor selects the next rule. Caller rules can then propagate an
observation, consume facts, supply a binding or post a new private call.

Admission still requires exactly one private occurrence and an applicable private
rule. That first simplification consumes the admitted occurrence. The caller's
other occurrences and propagation history remain in its original cursor; resumed
private facts are introduced as the phase's surviving results. Cases involving
multiple returned occurrences test the order and identity consequences directly.
This is an experimental execution organization, not a language restriction.

## What was tested

| Variation | Consequence checked |
|---|---|
| Private recursion at depths 0/2/5 | Repeated work, suspended progress and reuse |
| Ordinary, fresh-alias, duplicate and failing alternatives | Output aliases, multiplicity, branch order and failure |
| Two independent or aliased suspended facts | Multi-head matching, ordered propagation-history tuples and conflicting consuming bindings |
| Propagating versus consuming observer | Actual observations and scarce-resource effects |
| Observer before or after caller supply | Whether observation happens before the binding re-enables private work |
| Zero/one/two tokens | Absence, competition and residual multiplicity |
| Two input variable namespaces | Fresh transport and shared binding |
| Caller posts a fresh private call | Re-entry beside the original suspended fact |

Every case is cancelled at 0/1/5 steps and restarted to completion with the same
table. Direct supplies ordered one-step comparisons. An independent scalar
interpreter checks complete raw answer bags after both running query handles are
dropped. The metrics build requires trace hits and replay for every prepared
program. Extra assertions require observers to fire in applicable sources and
require failure when consuming aliased facts supplies conflicting bindings.

The first 288 cases covered one suspended fact and matched 17,388 checkpoints.
The two-fact challenge expanded this to 432 cases and 26,532 checkpoints. Caller-
posted private work brings the final matrix to 504 cases and 31,176 checkpoints.
All 37 caller, trace, result-reuse and effectful-call tests pass in default and
counter-free builds. The final observer assertions and scoped Clippy pass. No
reference interpreter implementation changed. No comparative timing was run.

## Architectural consequence and next experiment

An observer predicate is not by itself a reason to prohibit call reuse. The source
priority and the point where the phase returns to its caller matter. These results
support phase-boundary reuse with observable suspended facts; they do not establish
that a whole private phase may hide work from a higher-priority observer.

Next, investigate reuse of a selected rule body while rule selection and consuming
head ownership remain in the live caller. That would allow observers between
private rules without assuming a whole higher-priority private phase. Compare
arbitrary observer priorities, shared bindings, rule histories and raw delivery
against Direct before timing the resulting mechanism. Body reuse may save less
work and require more lookups; those are architectural tradeoffs to measure.

This has greater immediate decision value than tuning retained traces or timing
the same phase boundary again: it could change the granularity at which reuse is
valid for ordinary rule programs. Selective retention and complete costs for
observed sources remain required. T075 stays active, package count one since the
portfolio review; the research goal remains active.

[Registration](../registrations/S05-live-observers.md) ·
[Executable experiment](../../../research/chr-reuse/tests/call_trace_caller.rs) ·
[Completed-caller comparison](S06-caller-reuse.md)

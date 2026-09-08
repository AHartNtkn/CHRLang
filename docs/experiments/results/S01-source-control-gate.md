# S01 source controls: acknowledgement changes the comparison

**Current Active execution performs different work from the registered propagation-before-acknowledgement policy.** A two-by-two join returns one receipt under Active, versus four under Global. Both Active access modes exhibit the difference. Global Scan and Indexed satisfy the full registered row/update observations; they are the matched controls for the next retained-versus-direct discovery experiment.

This is semantic and checker evidence. It does not rank access performance or reject activation generally. The [registration](../registrations/S01-update-discovery-gate.md) explicitly permits screening out policies that do not implement its contract. A future activation policy could preserve that contract with additional scheduling machinery, whose cost would need measurement.

## Executable source and independent checks

The [source fixture and oracle](../../../research/chr-compiled/tests/update_join_support/mod.rs) encode serial request, replacement and binding instructions as ordinary CHR. The propagation rule emits every compatible `(request,left,right)` receipt. A lower-priority consuming acknowledgement advances the driver. Final answers include the live rows, every receipt and done. A separate unique-partner test consumes both the request and right occurrence while retaining the left.

The oracle uses plain row lists, explicit row identities and a fired-tuple set. It enumerates Cartesian products under a small independently interpreted instruction list, without the candidate matcher, index or unifier. Replacement allocates a fresh oracle identity. Binding is deliberately restricted to one fresh variable becoming a ground atom; broader unification remains outside this fixture's oracle domain. Complete answers are compared under joint variable renaming with the independent exact observer.

Five test functions make 209 engine executions per feature configuration: 204 Global grid/adverse executions, one repeated single-step resume, two Active policy screens and two unique-partner consumption checks. Repeated G=1/G=N entries coincide at N=1; this is not 209 distinct workloads. Each execution starts with a zero-budget check, respects the service bound and replays all fired source applications using the independent owned-syntax interpreter. Terminal replay checks the absence of enabled rules.

The cases cover N=1,2,4,8; zero and repeated requests; dense and selective keys; no matching key; driver arrival first/last; equal/changed replacement payloads; late shared-key binding; duplicate equal-valued rows; and independent versus shared unknown payloads. Negative checks reject missing/duplicate residuals, missing receipt rounds and merged unknowns. After a replacement, injecting its retired right identity into a later join is rejected by independent replay. Enabled/default and counter-free runs agree on semantics; disabled engine counters remain zero.

[Tests](../../../research/chr-compiled/tests/update_join_gate.rs), [default execution](s01-update-gate/default-final.log), [counter-free execution](s01-update-gate/off.log), [commands and source hashes](s01-update-gate/validation.json).

## Why Active is not a matched control here

Acknowledgement is enabled while more propagation tuples still exist. The current activation organization reaches that consuming application after one receipt. Consuming the request makes the remaining tuples unavailable. This is legal under a committed source schedule, which independent replay validates, but it violates this experiment's explicit all-pairs-before-acknowledgement policy.

The screen asserts both the observed single receipt and inequality with the complete four-receipt expectation. It does not silently accept fewer answers as a faster implementation. Nor does the lower-priority source declaration impose a language-wide order: the S00 distinction between source semantics and reference policy remains in force.

## Checker cost uncovered and repaired

The initial debug invocation exceeded the intended wall bound and was explicitly terminated after its live process was identified. It is an unfinished diagnostic, not a failed architecture result. A bounded release diagnostic then completed all four initial tests. Instrumentation localized the delay to independent terminal replay, which enumerated tuples of all residual occurrences before testing their predicate names. Large receipt sets unnecessarily entered a three-head Cartesian enumeration.

The checker now filters each head by predicate name and arity before recursing over remaining heads. This is a necessary condition already checked by its final matcher; no potentially enabled tuple is excluded. It retains distinct occurrence checking, substitutions, guards and history validation. No candidate or reference execution algorithm changed. The corrected debug gate completes within the registered bound. The diagnostic durations are not controlled performance measurements and do not support an engine speed claim.

[Bounded release diagnostic](s01-update-gate/release-diagnostic.log), [checker implementation](../../../research/chr-compiled/tests/search_support/mod.rs). The final validation launcher gives every command a 60-second timeout and records its exit code. Focused strict Clippy passes with default and disabled metrics. The complete compiled-package test suite also passes, exercising other users of the modified replay checker. [Package tests](s01-update-gate/package.log), [Clippy](s01-update-gate/clippy.log), [counter-free Clippy](s01-update-gate/clippy-off.log).

## Decision and next work

Use Global Scan/Indexed as the policy-matched controls. Implement the registered direct prepared pair enumerator and update-driven retained pairs against this source/oracle. They must execute source updates and completion correctly, not merely compute a static join result. The pair cache must maintain incidence on replacement and binding and preserve fresh occurrence identity. Measure necessary receipt visits separately from avoidable pair construction or candidate discovery.

This completes the source/control portion only. Retained/direct candidates, their mechanism diagnostics and comparative lifecycle registration remain unfinished; S01 is not complete. S00's first-cycle integration, graph, direct-compilation and reusable-worker feasibility assessments also remain active obligations. The prior turn's committed contract work was progress; this turn adds a new control incompatibility result and executable update witnesses rather than merely restating that plan.

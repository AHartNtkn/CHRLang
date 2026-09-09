# Source-derived recursive updates preserve suspension and terminal effects

**The compiler now derives a recursive loop that updates carried terms while skipping intermediate occurrence handling.** It preserves unknown-tail suspension, later binding and ordinary terminal arbitration. Correctness and reduced candidate discovery are established; total-cost benefits are not yet measured.

The [registration](../registrations/S06-recursive-accumulator-gate.md) extends the existing [known-prefix carrier mechanism](R05-carrier-prefix-gate.md). That earlier mechanism already handled unknown tails and singleton admission, but required unchanged carried arguments. The new distinction is construction or rearrangement of those arguments during recursion—for example, `fold(s(n), a, r)` becomes `fold(n, f(a), r)` until the terminal source rule becomes eligible.

## What comes from the source

The certificate identifies exactly two single-removed-head rules for a predicate. The recursive rule has one unary control pattern, distinct variables in its other positions, no guards and exactly one recursive insertion as its body. The recursive control must become that constructor's child. Other arguments may be constructor expressions over existing head variables; fresh variables and additional effects are rejected. The other rule has a nullary control pattern and retains ordinary matching, guards and body execution.

The derived plan records how head variables supply the next recursive arguments. Primary execution inspects one control node and evaluates the corresponding constructor update per service tick. It does not reselect the recursive rule or maintain an intermediate source occurrence for every admitted step. On reaching an unknown, malformed or terminal tail, it publishes the corresponding occurrence with the source-equivalent identity and returns to ordinary arbitration.

This is a compact source-derived recurrence, not finite query unrolling or an exact-schema answer generator. Constructor expressions still use the existing prepared body instantiator. This implementation is not native code generation and does not eliminate the required constructor work.

## Why the earlier counterexamples still constrain it

Only one live occurrence of the recursive predicate may enter the contraction. Multiple occurrences retain ordinary source rotation until singleton admission becomes valid. That preserves the competing-completion order demonstrated in the [boundary probes](S06-recursive-boundary-probes.md).

No binding or terminal effect occurs during the contracted steps. Head-variable distinctness prevents hidden equality tests, and the source head-use check excludes intermediate observers. Because the control continues to be unary throughout the admitted steps, the nullary terminal rule cannot become eligible before the actual tail. Carried constructor creation alone does not enable another source rule. These conditions explain the contraction; they are not a general theorem for effectful recursion.

An unknown tail retains its original handle. The operation neither guesses its constructor nor advances a later binding. Ordinary source execution can bind it afterward and reactivate recursion. Terminal alternatives and resource claims remain outside the contracted loop, preserving the source scheduling boundary rather than requiring their effects to be pure.

## Evidence

Seven new tests compare the derived path with the independent scalar evaluator and ordinary specialized execution. Forty-eight renamed-source/depth/tail configurations exercise both scanning and indexing, with primary and diagnostic execution. They preserve full outputs, residual aliases and source occurrence identities. Diagnostic runs also compare exact traces and commit views. Metrics-enabled primary runs require nonzero skipped steps and fewer specialized candidates where contraction applies.

A further 36 source argument-update combinations cover rearrangement, constants, nested constructors and control-tail references. Separate cases exercise terminal alternatives, competing calls, duplicate tokens, constructor contradictions, unknown-tail aliases, rejected fresh variables and extra effects. A finite result remains serviceable beside a continuing recursive sibling, including after cancelled searches from the same preparation.

A diagnostic-transition test found a defect during development: enabling tracing halfway through inspection attempted replay from partially updated arguments. The job now keeps its original occurrence separate from pending updates. Replay starts from the original; primary completion publishes the accumulated update. The failing witness and passing repair are retained in the [validation receipt](s06-recursive-accumulator-gate/validation.json).

All 143 compiled-package tests pass with contraction enabled under default and counter-free features. The 21 existing/new carrier tests also pass with COW and fork diagnostics. Strict package Clippy passes for enabled default and counter-free builds. Final targeted checks include the reduced-candidate assertion. The independent reference evaluator is unchanged. Implementation and tests are in [carriers.rs](../../../research/chr-compiled/src/carriers.rs) and [recursive_accumulator.rs](../../../research/chr-compiled/tests/recursive_accumulator.rs).

## Costs and limits still to test

The update plan adds source checking, a head-variable frame and constructor instantiation. The job retains both the original occurrence arguments and pending updates so diagnostics can begin during inspection. That costs memory and may outweigh avoided selection and index maintenance on short recurrences. Required output construction and observation remain. Unchanged passthrough steps avoid the update frame, but shared runtime layout/configuration costs still need competent controls.

The per-tick work depends on the statically supplied constructor expression size; the finite-sibling witness is not a universal latency bound for arbitrary source bodies. Cancellation tests establish behavior and subsequent-query independence, not allocation restoration or sustained lifetime. A separate meter gate remains necessary before comparative timing.

Fresh recursive locals, equations or choices inside the step, multiple interacting recursive occurrences and richer recurrences remain outside this certificate. Their rejection does not reject the corresponding language designs. T073's broader effectful/resource-derivation and structural-solving questions remain open.

The [four-package breadth review](S06-recursive-breadth-review.md) selects one bounded complete-cost comparison of this new operation, with allocation/cancellation gates and strong ordinary controls. No architectural winner or overall goal completion follows from this source gate.

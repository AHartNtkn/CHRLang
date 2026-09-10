# Direct solving must preserve the execution boundary and answer counts

Direct solving remains worth implementing: pruning doomed choices preserves all 224 closed choice/check cases tested here. The same reasoning becomes unsound when outside rules can rescue a check or receive a binding early. A solver must also preserve repeated answers from distinct successful derivations.

This is source-semantic evidence. It establishes obligations for the next implementation, not a working general solver or a cost advantage.

## What the experiments distinguish

| Transformation or challenge | Observed result | Consequence for a solver |
|---|---|---|
| Keep only `t` at the original coin rule in the closed arrival source | Full answers agree in all 224 configurations, including both arrival orders, final failure, token consumption and output aliases. | A useful elimination witness exists. Automatic analysis must establish why `f` cannot survive. |
| Let a linked consumer take the check when the choice is `f` | Ordinary execution has two answers; pruning leaves one. The extra answer contains `rescued`. | Seeing a failing check rule is insufficient. Admission must account for all rules that can consume or observe that private work. |
| Publish `t` at entry instead of when the coin rule executes | Both versions have one answer, but different bindings and residual resources. Pruning at the original location still agrees. | Correct value prediction does not justify moving its publication across competing effects. |
| Give each aliased coin two `t` alternatives and one `f` alternative | One, two and three requests produce 2, 4 and 8 identical successful answers. A distinct-value control produces one. | Source derivation counts must survive propagation and extraction. Set-valued solving changes the result contract. |

The early-binding example has a concrete writer. `watch(t,Y), token` binds `Y=f`; `coin(X), token` chooses `X=t` or `X=f`. The writer has higher priority but initially cannot match unknown `X`. Ordinary execution lets the coin consume the token, leaving `Y` unbound. Publishing `X=t` earlier lets the writer consume the token instead, leaving a residual coin and binding `Y=f`. The writer never mentions the coin or check predicates; their names alone cannot establish privacy when the token is shared.

The closed matrix varies depth 0 through 6, two arrival orders, work 0/2, resource presence, final success/failure and both query tags. Every original and transformed execution is checked by the independent scalar semantics and existing compiled Global Scan. Analytical complete answers additionally check the closed source. Comparisons include joint aliases and residual multiplicity, not merely result cardinality.

## What remains unproved

The pruning code deliberately replaces the known coin body in a test. It does not inspect arbitrary source rules to infer a relation. These results cannot be advertised as automatic compilation, a solver eligibility check, or evidence of speed.

The 224 successful checks are bounded evidence for that source family. The counterexamples disprove the specific broader shortcuts they exercise. They do not show that direct solving requires changing language semantics, nor that a more precise effect analysis is impossible.

Finite-answer equivalence also does not establish progress equivalence on ongoing programs. The existing [private-phase entry analysis](S05-call-entry-gate.md) supplies the separate first-rule and ongoing-work counterexamples; those remain prerequisites for admitting a complete phase.

## Next implementation and why it is selected

Implement source-derived finite relations behind a checked complete-phase boundary. Start with closed single-head choice/check rules whose constructor cases and finite choices can be extracted from syntax. Derive allowed values from those rules and the actual query; do not recognize the benchmark name or emit its analytical expected answer. Preserve each choice occurrence's identity and successful derivation count. Reject unsupported or unfinished analyses explicitly.

For the first implementation, require the private rules to form a priority prefix, reject outside heads touching private predicates, and establish that every admitted private computation terminates without suspended private residuals. These are proposed sufficient conditions to prove and test, not a checker delivered by this package. Shared caller variables may be valid if their bindings are published only when the complete private phase finishes. Consuming multihead boundary work remains in ordinary execution until its own correspondence is established. Do not move token claims as in the counterexample.

The first discriminating mechanism test must show constraints eliminating alternatives without enumerating the complete choice product. It must also change predicate/constructor names and query values, preserve aliases and duplicate derivations, and reject linked-consumer, early-writer, unknown-input and ongoing-work near misses. If these conditions cannot cover the intended favorable source, investigate that limitation before claiming the mechanism has been tested. Broader resource-aware derivations remain required.

This implementation is selected over native graph/connected feasibility, broader reuse and finer effect precision for the next package. That is a prioritization judgment: the [qualified compiler controls](S10-arrival-controls.md) still enumerate the alternatives, while this gate identifies both a promising elimination case and concrete boundaries to check. It can therefore test a different amount of necessary execution on a currently consequential comparison. Native graphs remain the strongest distinct alternative; the next mechanism package must compare its remaining effort and decision value against that alternative again. This selection supplies no adverse evidence about graphs or reuse.

After source correspondence, register preparation, changed-query setup, solving, first/full observation, consumer retention and disposal against the qualified controls. Include unselective choices, tiny cold queries and reused artifacts. Compilation and solver construction costs must be explicit. This package supplies none of those cost results.

## Evidence and reproduction

The [prospective registration](../registrations/S06-direct-solving-obligations.md) fixes the hypotheses, matrix, bounds and interpretation. The [test source](../../../research/chr-direct-conditional/tests/direct_solving_obligations.rs) contains four executable experiments. All four pass in [default](s06-direct-solving-obligations/default.log) and [counter-free](s06-direct-solving-obligations/counter-free.log) builds. [Strict scoped Clippy](s06-direct-solving-obligations/clippy.log) passes. No engine or reference behavior changes.

Run `cargo test -p chr-direct-conditional --test direct_solving_obligations -- --nocapture`, then repeat with `--no-default-features`. Each command has a 180-second process limit in the recorded run; each individual execution also has the registered deterministic service bound. [Source hashes](s06-direct-solving-obligations/freeze.sha256) and [environment](s06-direct-solving-obligations/environment.txt) identify the tested state.

T073 and the research goal remain active. No architecture or language restriction is selected by this gate.

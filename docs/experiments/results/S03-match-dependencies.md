# Equal dependency handling removes the expansion advantage of pull-tabbing

Ordinary demand can obtain the measured reuse benefit without copying calls. After correcting validity handling on both sides, ordinary demand and local pull-tabbing perform equal source expansions in all 72 registered configurations. Traversal, dependency analysis and retained ownership still differ; elapsed-time and allocation comparisons remain necessary.

## The control and its correctness argument

The [registration](../registrations/S03-match-dependencies.md) adds an explicit MatchDependencies policy. A successful pure match starts with its call's birth conditions. It adds the validity conditions of cached producer results, inspected choice selections and those choices' birth conditions. Constructor fields are inspected only where the head pattern requires them. Variable patterns keep opaque graph references.

This is a sufficient validity certificate for the existing checked fragment. Heads are linear and nonoverlapping; bodies of qualifying predicates are transitively pure and choice-free. A successful head therefore identifies its clause without depending on an earlier competing effect. Under the recorded conditions, its inspected structure and captured references remain the same. Uninspected choice-bearing references can still resolve differently when observed in different contexts, as they already do under the opaque StaticBirth policy.

Consuming calls, resource posts and definitions that create fresh choices retain the full current context for their source result. A producer's cached result can be read by a pure consumer, but its own validity conditions must be retained. This does not reuse resource consumption or fresh choice creation across contexts. Broader writable aliases, overlapping heads and other source contracts remain outside this certificate.

A structural test failed before implementation because the result retained an unrelated choice. It now verifies both omission of that choice and retention of the inspected choice, producer support and call birth. Reusing the result across a changed unrelated choice does not create another expansion; changing the inspected choice does not reuse the result. A separate test verifies independent resource claims and full-context validity for consuming calls.

## A second asymmetry appeared in the lifted control

The first dependency-policy measurement gave ordinary demand 140 expansions versus 162 for lifting in the four-independent-choice, depth-32 case. The copied calls still inherited all conditions present when the rewrite happened. That needlessly prevented reuse across unrelated choices.

The registered repair gives the administrative rewrite its own dependency support: original call birth, traversed producer-result supports, selected choices on the path and the unresolved target choice's birth. Each copied call adds its arm selection. Source effects still execute under their normal contextual ownership. A structural test failed on the unnecessary condition before this repair; it now checks the producer, call and choice-birth conditions as well as exclusion of the unrelated condition.

The older policies remain explicit attribution controls. MatchDependencies is the strengthened experimental comparison; it does not select a production language architecture or replace the existing measurement records.

## Corrected work comparison

These rows use forward insertion order and MatchDependencies on both sides. Every cell shows ordinary demand → local pull-tabbing. The four-independent-choice cases produce 16 complete answers.

| Source | Source expansions | Force entries | Dependency-inspection entries | Lift-site entries | Retained nodes |
|---|---:|---:|---:|---:|---:|
| One consumer, depth 0 | 3 → 3 | 45 → 59 | 6 → 2 | 0 → 2 | 11 → 14 |
| One consumer, depth 32 | 35 → 35 | 2,705 → 2,619 | 70 → 2 | 0 → 34 | 107 → 110 |
| Four independent consumers, depth 0 | 12 → 12 | 733 → 1,122 | 24 → 8 | 0 → 8 | 44 → 56 |
| Four independent consumers, depth 32 | 140 → 140 | 61,637 → 61,290 | 280 → 8 | 0 → 136 | 428 → 440 |

Expansion counts are equal across all 72 configurations, including both insertion orders. Opaque and nested demand produce no lifts. The dependency certificate also improves nested demand without requiring a local rewrite inside the constructor. The corrected four-choice direct case creates four lifts and eight additional call obligations, rather than the earlier 15 lifts and 30 additional obligations.

These counts answer an attribution question: graph rewriting is not required for the expansion savings on these sources. They do not decide which strategy is faster. Inspecting and merging dependency support has costs; lifting can shorten subsequent traversals but introduces retained calls, result edges and obligations. The operations have different costs and must not be added into an invented efficiency score.

## Evidence and validation

The [first dependency run](s03-match-dependencies-work/freeze.json), [lift-support repair run](s03-match-dependencies-lift/freeze.json) and [final implementation recheck](s03-match-dependencies-final/freeze.json) each contain two exact 144-row repetitions with complete hand/scalar answer checks. The final recheck also verifies that avoiding an unnecessary birth-context copy in conservative control paths leaves the diagnostic rows unchanged. All registered processes complete within the 60-second, 1-GiB and algorithmic bounds.

Four structural unit tests and the 26-test suspended-source suite pass. Its complete-answer helper exercises MatchDependencies with and without lifting, including branch-local producers, unknown handles, correlated choices, residual multiplicity and resource effects. The explicit finite-sibling/off-output-failure witness also runs the new policy. Package-wide tests with diagnostics enabled and Clippy pass; an ordinary build checks the same structural invariants without counters. Reference code is unchanged.

The raw directories freeze their source files and binary hashes. The runner now selects an explicit `--policy CurrentContext|StaticBirth|MatchDependencies`; the frozen snapshots preserve the earlier invocation where applicable. This remains a diagnostic experiment with no comparative timing or allocation-byte result.

## Consequence for the next investigation

The [breadth review](S03-dependency-breadth-review.md) selects one bounded lifecycle pilot using the corrected controls. Measure preparation, changed-query setup, execution with complete observation, first answer, cancellation and disposal; use ordinary-allocator counter-free timings and separate requested-allocation diagnostics. Include applicable compiled and source-derived lowering controls. First establish their answer correspondence on this source family and register exact sizes and repetitions.

That pilot must charge the dependency certificate and lifted ownership equally. It may establish a narrow crossover, a loss or unresolved overlap. It cannot reject broader pull-tabbing, fresh derivation templates or direct graph architectures. Those questions, along with sustained lifetime and the other architecture stages, remain open.

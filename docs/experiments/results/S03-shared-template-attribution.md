# Sharing compiler terms removes substantial derivation overhead

**The adverse substantive sizing was largely an implementation cost.** Sharing immutable compiler terms produces practical lifecycle gains in all 32 substantive configurations of the paired experiment. Shallow cases still expose overhead, so this repair does not establish a general preference for derivation reuse.

## Result at a glance

The [registered corrected comparison](../registrations/S03-shared-template-attribution.md) completed 952 timing processes, 272 allocation processes and eight cancellation processes. Each of the 68 configurations has seven paired timings and two allocation runs per build. All allocation replays and disposal checks pass; full source answers agree with the independent scalar evaluator.

| Paired comparison | Practical gains | Practical losses | Unresolved |
|---|---:|---:|---:|
| Shared versus owned template terms, 64 configurations | 32 | 2 | 30 |
| Ordinary-demand control across the two builds, four configurations | 0 | 0 | 4 |

A gain requires the pointwise 95% bootstrap interval for the paired cost ratio to lie below 0.90; a loss requires it above 1.10. Counts describe these configurations, not a population or simultaneous confidence claim. All 32 substantive configurations gain. The other 32 use depth zero, including queries alternating depth zero/one.

## Where the difference comes from

The compiler previously copied ground subtrees when binding or substituting an argument. It now retains immutable references, caches groundness and shares ground runtime nodes within an instantiation. This avoids repeatedly copying countdown suffixes and duplicating accumulator trees. Fresh unknowns remain application-local; source choices, calls and resource effects still execute through the common plan evaluator.

The table reports median primary milliseconds and requested allocation over eight changing queries, resources present, forward starting order. Depth alternates 32/33, or 12/13 for the growing accumulator. Preparation, setup, complete execution/observation and disposal all count.

| Source | Owned terms → shared terms, ms | Requested allocation, MB | Peak requested growth, kB |
|---|---:|---:|---:|
| One call | 0.981 → 0.212 | 1.993 → 0.278 | 248 → 27 |
| Four equal-input calls | 1.115 → 0.406 | 2.478 → 0.767 | 267 → 45 |
| Four distinct-input calls | 3.354 → 0.657 | 7.907 → 1.044 | 289 → 68 |
| Doubling accumulator | 25.862 → 12.418 | 45.218 → 12.003 | 2,826 → 1,653 |

Most elapsed savings occur in execution with construction and observation: single-call execution falls from 0.871 to 0.138 ms, distinct-call execution from 3.040 to 0.443 ms. Including source/input construction preserves the substantive reductions. The growing family still materializes complete owned answers and retains considerable observation/disposal cost; compact internal storage cannot eliminate that output obligation.

Setup and disposal also move in the measurements, including higher setup cost for the growing case. Those intervals do not directly measure template construction, and may reflect allocator history and cache effects across changed queries. The experiment attributes the combined representation change, not every phase movement to a separate mechanism. Construction-only and instantiation-only timings were not isolated.

The four ordinary-demand controls request exactly the same allocation bytes across builds. Their timing intervals do not establish a practical shift under the registered criterion. This limits the evidence for an unrelated evaluator-wide speed change.

## Contrary cases and implementation obligations

Two shallow reverse-order cases meet the practical-loss criterion: resource-consuming repeated calls at depth zero/one query have a ratio of 1.228, and nonconsuming growing calls at depth zero/one query have a ratio of 1.364. Their intervals are respectively 1.101–1.389 and 1.159–1.612. Allocation rises from 27,494 to 28,037 bytes in the former and falls slightly from 9,836 to 9,803 in the latter. Allocation alone therefore does not explain both timing losses.

Shared terms introduce reference-count ownership, groundness metadata and a per-instantiation ground-node lookup. These are real obligations on short work, even when there is little copying to save. The shallow findings remain adverse evidence; no universal representation policy is inferred. Further tuning of these small cases is less informative now than comparing the repaired mechanism against the strongest compiled and lowering controls.

The first paired run exposed an additional construction-bound issue during review: repeated references were not charged. A wide-source regression demonstrated that 17,000 references could bypass the budget. The corrected implementation charges every substituted edge. Its complete rerun supplies the result above; the [initial source freeze and receipts](s03-shared-template-attribution/freeze.json) remain distinct from the [corrected freeze and receipts](s03-shared-template-bounded/freeze.json).

## Correctness and complexity evidence

The compact-growth regression first failed on the owned implementation: copying exhausted the budget after 10 of 12 recursive steps. It now derives all 12 steps and retains fewer than 100 runtime nodes before observation. A separate wide-reference test exceeds the construction budget, uses ordinary execution and produces the complete expected answer. Another test stops specialization at 64 followed calls, retains a live continuation and obtains the final answer.

The complete source/identity/effect/progress suite passes, including 31 suspended-source tests, fresh application unknowns and choices, resource claims, off-output failure, residual occurrence identity and the explicit contested-scheduling difference. Full package tests with diagnostics, Clippy and the counter-free runner gate pass. The reference interpreter is unchanged.

There is one plan traversal for source and template terms. Sharing stores immutable expression nodes; runtime identities are introduced during instantiation. Only ground values enter the instantiation lookup, so local producer bindings and choice-arm environments cannot accidentally reuse a context-dependent runtime node. The construction, key, cache-entry and followed-call limits remain enforced. This organization needs no additional source scheduling policy, but contraction retains the already documented scheduling boundary.

## Architectural consequence and next selection

The current evidence supports keeping this representation repair. The earlier substantive overhead cannot be treated as inherent to fresh-derivation reuse. It also demonstrates that preserving sharing inside analysis and instantiation matters independently of caching a complete derivation.

The next bounded package should confirm the repaired template path against ordinary demand, scanned/indexed execution, inferred specialization and the exact-source control, including independent-choice sources. The repair is large enough to change that comparison; the original cross-engine sizing no longer gives an adequate estimate. Preparation, changed queries, adverse shallow cases, cancellation and complete answers must remain in the registered comparison.

This confirmation takes priority over T072's broader integration because the source gate, controls and counter-free runner are ready, while the repaired ordering is now consequential and unknown. T073's reusable source-derived artifacts are another strong alternative: exact-schema controls cannot establish what a general compiler can derive. After one comparative package, review breadth before further template refinements and select between these distinct mechanisms. Neither integration nor broader lowering is resolved by the present result.

Unknown-input derivations, effectful recursive analysis, cross-query artifacts, sustained lifetime and complete architectural composition remain open. The four-family attribution does not settle independent-choice cost or native compilation. Requested allocation is not RSS, and no whole-architecture winner or language restriction is selected.

[Analysis and every configuration](s03-shared-template-bounded/summary.json) · [Reproducible analysis script](../../../research/chr-direct-conditional/experiments/shared_template_analysis.py)

# Carrier complete-lifecycle comparison (T054)

Source-derived contraction changes the ground countdown recommendation: Carrier beats Conditional with separated observed ranges in all 14 ground cells, and beats uncontracted Specialized in all 12 nonzero ground cells. Conditional wins both unknown-tail cells. These are bounded source-shape results, not workload weights or a universal architecture ranking.

The [prospective registration](../registrations/R05-carrier-cost.md) and [raw audit](r05-carrier-cost/audit.json) cover all 640 processes, 25,600 independently validated complete answers and 80 configuration cells. No cutoff, missing run, allocation restoration failure or freeze mismatch occurred. Root and independent reader checked the manifest, exact work predictions, full endpoints and frozen sources/binaries at commit `4a169b1` before subsequent work.

## Measured complete cost

Median milliseconds per query, including preparation amortized over the registered query count, query construction, setup, execution/full observation, and engine/output/prepared disposal. Primary builds use ordinary allocation with engine, kernel and observer counters disabled. Allocation and work counts are separate runs. Five primary repetitions per cell establish observed ranges, not confidence intervals.

| [pre, post, queries, tail] | Conditional | Specialized | Specialized COW | Carrier | Carrier COW |
|---|---:|---:|---:|---:|---:|
| [0, 0, 1, 'ground'] | 1.258 | 0.569 | 0.565 | 0.588 | 0.613 |
| [0, 0, 4, 'ground'] | 1.365 | 0.525 | 0.518 | 0.539 | 0.536 |
| [0, 16, 1, 'ground'] | 3.895 | 2.104 | 1.979 | 0.796 | 0.774 |
| [0, 16, 4, 'ground'] | 4.003 | 2.053 | 2.024 | 0.639 | 0.629 |
| [8, 8, 1, 'ground'] | 2.644 | 1.514 | 1.460 | 0.784 | 0.757 |
| [8, 8, 4, 'ground'] | 2.667 | 1.423 | 1.496 | 0.679 | 0.715 |
| [16, 0, 1, 'ground'] | 1.499 | 1.017 | 1.018 | 0.729 | 0.731 |
| [16, 0, 4, 'ground'] | 1.448 | 1.513 | 0.928 | 0.612 | 0.644 |
| [0, 64, 1, 'ground'] | 11.774 | 6.683 | 6.553 | 0.885 | 0.954 |
| [0, 64, 4, 'ground'] | 12.201 | 6.813 | 6.804 | 0.769 | 0.759 |
| [32, 32, 1, 'ground'] | 6.791 | 4.376 | 4.222 | 0.932 | 0.923 |
| [32, 32, 4, 'ground'] | 7.090 | 4.345 | 5.126 | 0.728 | 0.727 |
| [64, 0, 1, 'ground'] | 1.916 | 2.215 | 2.291 | 0.867 | 0.826 |
| [64, 0, 4, 'ground'] | 1.990 | 2.131 | 2.139 | 0.766 | 0.728 |
| [64, 0, 1, 'unknown'] | 0.861 | 4.636 | 4.515 | 4.809 | 4.618 |
| [64, 0, 4, 'unknown'] | 0.734 | 4.491 | 4.386 | 4.670 | 4.551 |

Carrier versus Carrier COW ranges overlap in all 16 cells. This provides no basis for combining the options by default. Both zero-work carrier comparisons with Specialized overlap. The [summary](r05-carrier-cost/summary.json) retains ranges, preparation, first observation, phases, requested heap traffic, peak live requested bytes and exact work. Requested allocation is not RSS. Compilation cost is not credibly isolated; no complete architectural lifecycle superiority follows.

## Consequential boundary and next decision

The ground result shows that sharing repeated pure countdown execution is not the preferred measured complete path when competent source lowering is admitted. Groundness is currently an implementation admission restriction. Independent review finds that the same singleton/sealed-head proof permits contracting just the known unary prefix and returning to ordinary arbitration at the actual tail, without binding it or inventing a base constructor. The corresponding occurrence ID must be preserved. Later equality may independently enable more work.

T055 selects that bounded semantic simplification before interpreting the unknown-tail result as durable sharing support. Require malformed/unknown tails, shared aliases, later binding, zero remaining prefix, primary/trace IDs and cancellation. Only after these pass should a small prospective cost comparison resolve the unknown-tail contrast. This is a competent-control question exposed by a consequential result, not a repeat seeking a preferred winner.

The strongest broader alternative is substantive noncontractible equation/constructor pre-discrimination work with observable correlations and early-discrimination/postwork controls. Pure countdowns do not answer that question. Its wider relevance is real, but the narrow prefix gate first resolves a concrete confound at lower entry cost. Maintained joins require a current matched bottleneck screen; no automatic prototype priority follows. The full goal remains active.

## Build contract follow-up

Allocation builds emitted an unexpected-cfg warning because the direct-conditional manifest did not declare the shared meter's `fork-diagnostics` feature. None of the measured configurations enabled that feature. The feature declaration and a feature-off fallible-preparation Clippy issue were corrected after the frozen audit. Strict allocation-build Clippy passes with carrier off and with carrier/COW on; runner tests pass. These follow-up edits are not the measured source freeze. Reproduce T054 from `4a169b1`; subsequent comparisons need a fresh freeze.

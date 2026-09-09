# Contextual ownership trades allocation traffic for a smaller live footprint

**Contextual execution has a supported advantage over the relational control, but not a general runtime advantage over scanning.** Its lower peak requested memory comes with greater allocation traffic than scanning on the branching cases. An exact-schema lowering beats contextual execution in every tested configuration, so this source family does not justify selecting the contextual architecture.

The broader contextual mechanism remains viable and unresolved. The direct control accepts this schema only; it does not establish a general replacement for CHR execution.

## What was confirmed

The [registered comparison](../registrations/S02-contextual-lifecycle-confirmation.md) crosses five paths, four source families, depths 0/8/32, one/four changed queries and consumption absent/present: 240 configurations. Payload size, zero/three choices, zero/eight local bindings and zero/eight keyed claims vary independently. Preparation is reused across changed depth and insertion order.

Every configuration has seven measured ordinary-allocator runs following a warmup. The experiment completed 1,680 measured processes, 240 warmups, 480 allocation processes and ten cancellation/reuse processes. Full answers agree with the independent scalar evaluator. Every pair of allocation replays agrees exactly phase by phase, and query/prepared disposal restores its requested-allocation baseline. No process hit a cutoff.

Primary timing sums preparation, setup, execution with complete observation and engine/answer/prepared disposal. Source/input construction, first-answer latency and process wall time are separately reported. Counter-free ordinary timing and allocation diagnostics use separate binaries. Fixtures and validation are outside measured intervals. Compilation is not isolated, and these shared harness builds do not support compilation-inclusive superiority.

## Supported comparisons and uncertainty

For each of 48 source configurations, the registered rule uses seven same-block ratios. A gain or loss requires a median difference of at least 10% and consistent direction in all seven pairs. These are descriptive practical criteria, not confidence intervals.

| Comparison | Practical gains | Practical losses | Unresolved by the rule |
|---|---:|---:|---:|
| Contextual / relational | 44 | 0 | 4 |
| Contextual / scan | 5 | 9 | 34 |
| Contextual / indexed | 13 | 0 | 35 |
| Lowered / contextual | 48 | 0 | 0 |

These counts describe coverage, not workload frequencies or an aggregate architecture score. Many short comparisons vary enough to cross one. The result does not establish parity or a loss in those cells.

At depth32/four queries with consumption, contextual/scan has paired median ratio 1.225 for no-choice local updates and 1.218 for three-choice local updates. Their paired ranges are 1.078–1.527 and 1.142–1.666, respectively: practical losses. The unchanged-local cases remain unresolved against scanning. All four cases show a practical contextual gain over relational execution.

## The memory tradeoff remains

The repeated requested-byte measurements reproduce sizing exactly. For three choices, eight local bindings, depth32 and four queries with consumption:

| Path | Cumulative requested bytes | Peak requested growth above host baseline |
|---|---:|---:|
| Contextual | 2,956,087 | 54,741 |
| Relational | 3,803,438 | 512,253 |
| Scan | 1,277,305 | 125,788 |
| Indexed | 1,705,321 | 202,925 |
| Exact-schema lowering | 206,254 | 24,197 |

Contextual ownership's small live footprint does not imply less total allocation work. The lowering demonstrates that neither observed executor footprint is a lower bound for this source. These are requested heap bytes, not RSS, and exclude the held oracle-fixture baseline. Short disposal checks do not establish sustainable memory under ongoing consumers or large failed frontiers.

## What the lowering actually proves

The lowering checks exact source-rule equality and the complete query layout. It parses the finite payload, enumerates the zero/three choice bits, carries the payload into each answer, and constructs eight hold residuals. In local-update cases the holds are bound to the selected first choice; otherwise they remain distinct unknowns. The source's exact keyed eat/token pairs consume without leaving residual resources. Changed queries reuse the checked source preparation.

Checking, retained payload, complete output construction and disposal are measured. Tests compare all five paths with independent scalar answers and reject altered rules, extra tokens, missing outputs, unknown payload structure and missing patch constraints. The checker is deliberately exact: rejecting a renaming or other equivalent source is a checker limitation, not evidence that the language must forbid it.

At depth32/four queries with consumption, lowered/contextual paired median ratios range from 0.105 to 0.432 across the four families. All 48 configurations meet the practical gain criterion, including one-query cases. This is a direct comparison with contextual execution; no unregistered pairwise statistical claim against every other control is inferred from it.

## Bounded disposition and next selection

Retain contextual ownership as a candidate with measured allocation/lifetime tradeoffs and several favorable regimes. Do not select it from these regular sources, or turn a local-update loss into rejection of all contextual organizations. Post-fork consequence reuse, stronger discovery, source-general integration and sustainable arena retention remain unanswered.

Further precision on the close contextual/scanning comparisons cannot change the present implementation choice between contextual execution and its verified direct control within this schema. It could refine a component tradeoff, but is lower value than testing whether lowering can be derived from broader source properties. The direct control's strong result does not justify compilation-inclusive claims or automatic multi-runtime routing.

The next selected investigation is [source-driven lowering beyond exact schemas](S06-source-driven-lowering-entry.md). Actual local pull-tab operations and remaining integrated mechanisms are strong alternatives; the entry records why broader eligibility comes first now. All remain open in the coverage map. This completes a bounded cost comparison, not S02 or the research goal.

## Evidence

[Analysis with every paired ratio](s02-contextual-confirmation/analysis.json), [source/binary freeze](s02-contextual-confirmation/freeze.json), adjacent raw processes, [correctness and rejection gate](s02-contextual-confirmation/source-gate.log), [meter self-check](s02-contextual-confirmation/meter-check.log), and [Clippy](s02-contextual-confirmation/clippy.log). The two runner tests pass and strict Clippy passes for the example; existing integrated build-script warnings are recorded.

[Lowering implementation](../../../research/chr-relational/examples/support/contextual_source.rs), [runner](../../../research/chr-relational/examples/s02_contextual.rs), [registered launcher](../../../research/chr-relational/experiments/contextual_confirm.py), and [analysis](../../../research/chr-relational/experiments/contextual_confirm_analysis.py). The [sizing evidence](S02-contextual-sizing.md) remains linked to its original binaries.

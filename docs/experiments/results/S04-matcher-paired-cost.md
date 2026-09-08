# Matching copies explain much of the mutation cost, but not the restoration tradeoff

Rejecting incompatible roots before copying cuts mutation time sharply. Long compatible alias chains incur a smaller, repeatable overhead. Keep the correction as the current experimental implementation, with that adverse case explicit; it is not a universal optimization claim.

The [registered comparison](../registrations/S04-matcher-paired-cost.md) completed all **672 processes**. Both versions use identical harnesses, manifests and dependency locks. All 96 cells pass exact repeated allocation checks and complete disposal. The original restoration pilot remains independently frozen.

## What changed in mutation

These are eight-changing-query lifecycle medians. Preparation, setup, complete execution/observation and disposal are included. Ratios are medians of five adjacent after/before pairs; they need not equal the ratio of the displayed medians.

| Executor | Before, ms | After, ms | Paired ratio and range | Requested heap traffic before → after, MiB |
|---|---:|---:|---|---:|
| Copy | 432.117 | 48.026 | 0.110 [0.109, 0.111] | 796.015 → 56.401 |
| Trail | 818.185 | 423.426 | 0.526 [0.511, 0.534] | 969.922 → 230.308 |
| Checkpoint 1 | 488.179 | 103.747 | 0.210 [0.208, 0.295] | 898.263 → 158.649 |
| Indexed, unchanged control | 20.267 | 19.703 | 0.972 [0.616, 1.021] | 25.518 → 25.518 |

**The copies were a substantial confound.** The separately validated diagnostic predicts and observes 36,864 unnecessary environment copies per mutation query. The paired allocation reduction is about 740 MiB per eight-query batch in each affected mode. Source applications and complete answers remain unchanged. One-query mutation ratios show the same large improvements: 0.109 for Copy, 0.521 for Trail and 0.207 for Checkpoint 1.

**The correction does not remove retained-state costs.** Incremental peak requested heap stays exactly unchanged in every before/after cell. On mutation it is about 202 KiB for Copy, 208 KiB for Checkpoint 1, 4,125 KiB for Trail and 1,207 KiB for Indexed. Avoided temporary copies reduce traffic while different objects determine peak retention.

**The corrected source still shows opposing architectural costs.** Mutation Indexed remains faster and allocates less cumulatively than corrected Copy, while Copy has lower peak. On the retained-store family, corrected Copy takes 3.684 ms and requests 3.118 MiB versus Indexed's 5.846 ms and 11.098 MiB, with lower peak as well. These are bounded regimes, not a weighted preference or a universal architecture ranking. Mode-to-mode figures are descriptive medians from the same matrix; the registered paired attribution analysis compares matcher versions.

## The contrary cases matter

**Long aliases make the precheck more expensive.** With 64 aliases and 32 entirely compatible partners, every Copy and Trail timing pair is slower. Eight-query median after/before ratios are 1.115 [1.062, 1.211] and 1.113 [1.036, 1.150]; one-query ratios are 1.144 [1.093, 1.226] and 1.149 [1.106, 1.176]. The precheck follows the alias chain before full matching follows it again. It saves no copies on this source. Allocation is identical.

This is a measured smaller regression, despite falling below the prospectively registered 20% median threshold for a practical change. That threshold does not turn the overhead into zero. A language or implementation dominated by this pattern would require a different decision. No such workload weight is assumed here.

**Large equal structures do not show a corresponding practical regression.** On the 64-deep constructor key, eight-query Copy and Trail ratios are 0.991 and 1.003, with ranges crossing one. The precheck inspects constructor roots, not their children. Atomic compatible keys, retained state and the work-between-choices controls also show no registered practical regression.

**Some timing cells remain noisy.** Alias Checkpoint 1 has a broad eight-query range, and unchanged Indexed controls include isolated large variations. No unchanged-control cell meets the registered practical-change rule. The strongest mutation reductions are nevertheless present in every pair and have exact allocation corroboration. Do not infer equivalence or a small speedup from overlapping controls.

## What this resolves, and what comes next

This resolves the immediate question of whether a shared matching defect substantially distorted the mutation pilot: it did. It supports keeping the correction for further experiments while carrying forward its explicit alias-traversal cost. It does not justify general rejection of trails or replay.

Longer checkpoint intervals and root replay have not received this corrected timing matrix. Their original times remain measurements of the original source. Larger read-heavy payloads, working-state reuse, better switching policies, retained-owner attribution, adaptive splitting and temporary separation/reunion remain assigned to S04. The current trial cannot stand in for them.

Select [S05 stable-identity operation/failure reuse](S05-stable-reuse-entry.md) next. It tests whether economical operation recognition changes the choice between direct sharing and explicit execution, a contrast still lacking a direct implementation. Further matcher tuning would currently target an 11–15% adverse case without resolving a new architectural mechanism. Corrected replay policies are a stronger alternative, but their open questions are now explicit and the breadth cycle calls for a distinct contrast. This is an ordering judgment, not negative evidence about replay or alias optimizations.

## Evidence and limits

The [audit](s04-matcher-paired/audit.json), [all ratios and endpoints](s04-matcher-paired/summary.json), [phase table](s04-matcher-paired/phases.csv), [source/binary freeze](s04-matcher-paired/freeze.json), [identical build packages](s04-matcher-paired/packages.json), and [execution order](s04-matcher-paired/order.json) preserve reproduction details. Build and independent source-gate receipts are in the same directory. Release semantic tests, strict all-feature Clippy and formatting pass.

Timing uses the ordinary allocator with engine/kernel counters disabled. Requested allocation diagnostics use separate builds and are not RSS. Full answers are independently validated outside timing; first observation and a separate first-answer/cancellation scenario are also measured. Five timing pairs support bounded pilot findings, not population confidence intervals. Source construction, oracle work, process startup and native compilation are excluded; this is not complete architectural lifecycle superiority.

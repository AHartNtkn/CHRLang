# Attribute equality snapshot costs with persistent maps

The [deduction sizing](../results/S02-deduction-sizing.md) identifies whole-map copying caused by cached equality snapshots. This package compares structural persistence with ordered-map copy-on-write in both cached and uncached execution. It is selected before broad confirmation or another T073 package because that representation choice can change the credibility of the new reuse comparison.

## Candidate and validity

Reuse the existing tested persistent ordered-map kernel, with its metrics disabled in timing and allocation builds. Equality parents and descriptors change storage; live occurrences retain their existing local ownership. Ordered-map and persistent-map variants use the same source evaluator, transition keys, cache cap, constructor nodes, matching and output contract. Persistent snapshots share update paths rather than requiring whole-map copies, but add path allocation, indirection and value-copying costs.

The generated equality oracle, local consumption/failure, cached transition identity, retention cap, complete-source and finite-service tests must pass under both representations. The kernel's independent mixed-update/snapshot tests remain applicable. The map choice is experimental, not a language restriction or a selected storage default. Sharing this low-level kernel does not share the independent reference evaluator or its oracle.

## Exact configurations and accounting

Use the same four sources: single, compatible shared work, distinct inputs and intervening changed bindings. Cross depths zero/32, one/four changing queries, resources absent/present and both starting orders: **64 configurations**. Compare `contextual`, `shared`, `persistent`, `persistent-shared`. Prepared rules are reused; caches and equality storage are query-owned.

Run seven ordinary-allocator repetitions, shuffling source configurations and mode order inside each block with seed 7221: **1,792 timing processes**. Run every cell twice with the separate allocation binary: **512 processes**, requiring exact replay. Cancellation uses compatible shared work at depth8/two queries/resources/forward, first-query cancellation after zero/one ticks, all four modes and both allocators: **16 processes**.

Calibrate the ordered modes against the frozen sizing binaries at `/tmp/chr-deduction-lifecycle-e51a0be6/`: four families, depth32/four queries/resources/forward, both contextual/shared modes, seven paired ordinary repetitions with shuffled before/after order using the same RNG after the main matrix: **112 processes**. Repeat their allocation records twice per build: **32 processes**. Total **2,464 processes**, plus both meter self-checks. Preserve the before/after calibration separately from the representation comparison.

New binaries live in `/tmp/chr-equality-map-d648f15a/`. Freeze source, kernel, manifest/lockfile, driver/analysis, toolchain and binaries before runs. Verify old binaries against the sizing freeze. All runs use the first available CPU, 60 seconds wall/CPU, 1 GiB address space and the existing two-million-step source/scalar limit. Diagnose cutoffs and failures before drawing a cost conclusion.

Primary sum includes preparation, setup, execution with complete observation and engine/answer/prepared disposal. Record source/input-inclusive total, first-answer latency, phase costs, requested allocation and peak requested growth. Complete scalar answers are checked outside primary intervals. Ordinary timing is counter-free; allocation diagnostics use a separate meter and are not RSS. Native compilation is excluded.

## Prospective interpretation

For each source configuration compare persistent-shared/shared, persistent/contextual, shared/contextual and persistent-shared/persistent. Use seven same-block primary ratios, their geometric mean, and a pointwise 95% percentile bootstrap interval from 10,000 resamples. Seed `7222 + 4 * configuration_index + contrast_index`. Upper bound below 0.90 is a practical gain for the numerator; lower bound above 1.10 is a practical loss; otherwise unresolved. Calibration uses the same rule for after/before with seed `7322 + calibration_index`. These are pointwise descriptive claims, not simultaneous population-wide conclusions.

Report all adverse configurations and allocation differences. A calibration shift requires attribution before relating the new result to older controls. A persistent-map improvement supports a representation correction within its regime, not a universal persistence policy. An uncached regression may expose the price of making snapshots cheap. Examine whether the surviving advantage could alter the compiled/direct-control comparison before selecting confirmation. Revisit T073 general lowering and remaining T072 rewrite mechanisms after this package rather than automatically tuning map constants.

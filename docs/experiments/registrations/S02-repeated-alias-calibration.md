# Contemporaneous calibration of the one-update control

The repeated-alias matrix completed 784 processes with exact within-build allocation/work replay. Its M=1 source is identical to the preceding low-yield source, but its sampled times are substantially higher. Both runs use CPU 0 and the same toolchain; that does not establish the cause of the shift.

Interleave the frozen ordinary-allocator binaries from s02-local-validation-attribution and s02-repeated-alias-lifecycle. Use all seven controls at N=128, reuse=4, M=1: old family `lowyield`, new family `aliases-1`. Ten repetitions give 140 processes. Randomize mode order per repetition and old/new order per pair with seed 7206. Use the same 60-second, 1-GiB, CPU-0 limits and existing complete scalar answer checks. Do not rebuild either binary.

Compare within-pair total-time ratios and their range, not historical absolute times alone. A consistent binary-specific effect requires attribution before using the new timing to infer a source effect. If both binaries move together or ratios overlap materially, scope the historical shift as unresolved run conditions rather than assert a hardware cause. Treat this as exploratory calibration, not a significance test. Independent allocation/work equality remains evidence even if timing cannot select a dependency policy.

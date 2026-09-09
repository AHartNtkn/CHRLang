# Attribute temporary child-vector copying during normalization

The preceding forcing-copy experiment confirms avoidable traffic but leaves a second copy in `normal`. This comparison changes only normalization: clone the constructor name required by the owned answer, read each child ID directly from the graph, and construct the final owned child vector without a temporary ID vector.

Constructor nodes are stable across recursive forcing. The only node replacement is `bind`, called only by `producer`, which checks that its old node is Unknown. Constructors are appended and never replaced; vector growth preserves integer IDs. Reading a child ID ends its borrow before recursive forcing. The traversal order, short-circuit signals and owned output terms remain the same. No output contract changes. The final owned vector reserves the known arity directly; allocation differences include that capacity choice as well as the temporary-vector elimination, so this comparison does not attribute each byte solely to the temporary copy.

## Hypothesis and controls

Temporary-vector elimination should lower execution/observation requested allocation, especially in growing outputs. Timing may remain uncertain or worsen due to traversal bookkeeping; retain adverse cases. This is attribution, not a cross-engine ranking. Calls, resources, preparation and forcing remain unchanged.

Use all 80 source configurations and both modes from the preceding forcing-copy registration: single/repeat/distinct/choice/grow, depth zero or 32 (grow 12), one/eight changed queries, resource absence/presence and both starting orders. Freeze the preceding repaired binaries as `before` after verifying all runtime Rust/manifests/lockfile hashes against their source freeze. Build `after` with identical features. Independent full answers, aliases, residual multiplicity, failure, finite service and cancellation remain required.

## Registration

Build ordinary binaries with no default features and `experiment`; meter binaries with no default features and `alloc-meter`. Freeze source snapshots, source and binary hashes, compiler version, commands and this registration before comparative runs. No engine/kernel/traversal counters in either timing build. Allocation metering measures requested heap bytes, not RSS.

Run both versions and both modes twice under the meter, requiring exact replay for all 320 cells: 640 processes. Then run seven paired timing blocks, shuffling 80 source configurations and four version/mode cells within each block with seed 7751: 2,240 processes. Cancellation uses choice/depth8/two queries/resources/forward and cancellation at zero/one service ticks for each version/mode and allocator: 16 processes. Total 2,896 processes plus two meter self-checks. Each process is pinned to the first available CPU with 60-second wall/CPU and 1 GiB address-space limits; scalar/candidate bounds remain two million steps. Preserve every cutoff or failure and investigate before interpretation.

The primary endpoint includes preparation, setup, execution/complete observation, engine and answer disposal, and prepared disposal. Source/input-inclusive time, first observation, phase timings, requested traffic and peak requested growth are secondary. Native compilation is excluded. Do not use meter elapsed times as speed evidence.

For each of the 80 configurations and each mode, compare after/before primary time with seven paired log ratios and 10,000 bootstrap resamples, seed `7752 + 2 * configuration_index + mode_index`. Pointwise percentile bounds use sorted indices 249 and 9749. Upper bound below 0.90 is a practical gain; lower above 1.10 is a loss; otherwise unresolved. No exclusions, workload weights, population claims or equivalence claims from overlap.

## Follow-through

Require exact allocation replay and phase attribution. Investigate any correctness discrepancy, cutoff or increase before interpretation. If ownership and allocation evidence support the change, keep it and advance to a fresh comparison with scanning, specialization and exact-source controls; do not automatically refine additional small clones. Broader output/lifetime contracts and whole architectures remain required. Review breadth after this attribution and the ensuing full-control confirmation.

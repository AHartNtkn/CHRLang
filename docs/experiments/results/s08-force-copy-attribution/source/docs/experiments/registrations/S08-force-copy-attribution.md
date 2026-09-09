# Attribute constructor copying during graph forcing

This experiment tests one concrete implementation cost behind the growing-output graph loss. Calling `force` on an already exposed constructor currently clones its owned name and child vector before returning the same graph ID. A borrowed constructor/unknown check can return that ID without copying or changing graph state. Normalization's separate child-vector copy remains unchanged, so this experiment isolates the forcing cost.

## Hypothesis and controls

The change should reduce requested allocations during execution/observation, especially with growing complete outputs. It may improve total lifecycle time; small sources may show no practical difference. This is implementation attribution, not an architectural ranking. The strongest compiled and exact-source comparisons remain required after consequential copying costs are addressed.

Compare frozen `before` and `after` binaries built from the current workspace, differing only in the early constructor/unknown return in `demand.rs`. Use existing `chr-derivation-cost`, modes `dependencies` and `templates`, and all 80 source configurations from S03 confirmation: five families single/repeat/distinct/choice/grow; depth zero or 32 (grow 12); one/eight changing queries; resource absent/present; starting order forward/reverse. Source and complete independent scalar checks remain unchanged. Retain aliases, full outputs, raw residual multiplicity, failure and cancellation contract tests.

## Registration

Build ordinary binaries with no default features and `experiment`; meter binaries with no default features and `alloc-meter`. Freeze source snapshots, source and binary hashes, compiler version, commands and this registration before comparative runs. No engine/kernel/traversal counters in either timing build. Allocation metering measures requested heap bytes, not RSS.

Run both versions and both modes twice under the meter, requiring exact replay for all 320 cells: 640 processes. Then run seven paired timing blocks, shuffling 80 source configurations and four version/mode cells within each block with seed 7741: 2,240 processes. Cancellation uses choice/depth8/two queries/resources/forward and cancellation at zero/one service ticks for each version/mode and allocator: 16 processes. Total 2,896 processes plus two meter self-checks. Each process is pinned to the first available CPU with 60-second wall/CPU and 1 GiB address-space limits; scalar/candidate bounds remain two million steps. Preserve every cutoff or failure and investigate before interpretation.

The primary endpoint includes preparation, setup, execution/complete observation, engine and answer disposal, and prepared disposal. Source/input-inclusive time, first observation, phase timings, requested traffic and peak requested growth are secondary. Native compilation is excluded. Do not use meter elapsed times as speed evidence.

For each of the 80 configurations and each mode, compare after/before primary time with seven paired log ratios and 10,000 bootstrap resamples, seed `7742 + 2 * configuration_index + mode_index`. Pointwise percentile bounds use sorted indices 249 and 9749. Upper bound below 0.90 is a practical gain; lower above 1.10 is a loss; otherwise unresolved. No exclusions, workload weights, population claims or equivalence claims from overlap.

## Follow-through

Check that allocation differences occur in execution/observation and that preparation and query setup remain unchanged. Investigate any correctness discrepancy or allocation increase. Retain the simpler allocation-free forcing path if semantics hold and allocation evidence confirms the mechanism; timing uncertainty alone does not require an allocation that serves no ownership purpose. Evaluate normalization copies separately before repeating complete competing-engine comparisons. Broader output contracts, sustained consumers and lifetime remain required under T074/S08.

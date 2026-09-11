# Borrow candidate state until a binding changes

This pilot asks whether avoiding copies on unsuccessful resource matches changes demand's complete cost and its comparison with strong explicit execution. Source tests and two exact work replays already qualify the change; no comparative lifecycle run has yet occurred.

## Mechanism and predictions

Resource argument IDs are read by index instead of cloning their vector. Existing resource vectors are immutable; forcing can append resources but cannot change those IDs. Candidate environments borrow the incoming binding map. The first new binding creates an owned copy; a fully matched borrowed environment becomes owned before recursive partner search. Failed candidates and signals cannot change the incoming map. Successful selection-vector handling is unchanged.

Predict zero resource-argument-copy traffic and fewer environment clones on read-only misses, with the same source work and answers. This does not avoid candidate visits, graph forcing, normalization or all environment allocation. New-binding/successful cases may still copy and may pay an extra ownership check. Keep all six post families and adverse arrival orders; do not select only misses.

## Existing controls and qualification

Reuse the exact frozen ordinary/meter binaries from S03-post-control-cost, with hashes verified. Its source archive supplies the before implementation; compare its relevant dependency/runners' source hashes with the pre-change commit. The changed implementation and relevant runner/dependency source are archived before running. Require identical toolchain text. These are existing baselines, not a new architecture.

The borrowing unit test checks read-only rejection, first extension, later rejection and a split signal without leaking bindings. Existing nonground-post and post-choice semantic suites pass against independent expectations and explicit execution; the finite-sibling test also passes. Two diagnostic post-work replays give 288 exact rows and match the parent, including ticks, matches, resource visits and retained graph counts. Repeat reconstruction from the raw logs at audit. Primary binaries have no work counters, allocation meter or candidate profiler.

## Exact lifecycle matrix

Six existing post families; sizes 8/32; original/reverse arrival; immediate/window/all consumers. Each session prepares once and runs four changing a/b queries. There are 72 scenarios.

Primary modes: three frozen original demand policies (birth, birth-miss, birth-miss-template), their three changed versions, and all nine existing explicit controls (Scan/Indexed, inferred, Active Scan/Indexed, generated Scan/Indexed and Active generated Scan/Indexed). Use the frozen ordinary binary for original demand and explicit controls, and the new ordinary binary for changed demand. One excluded warmup per cell (1,080 processes); five primary repetitions per cell (5,400 processes). Shuffle scenarios and mode order within each paired repetition with seed 710733. Five samples are exploratory, not confirmatory inference.

Allocation: two new-meter repeats for all 216 changed demand cells (432 processes), plus one new-meter run for each of 648 explicit-control cells to verify exact correspondence with the frozen parent allocation receipts. Allocation/profile clocks are not primary evidence.

Copy attribution: two new-profile repeats for all 72 retained-all changed demand cells (144 processes). Compare with the corresponding frozen candidate-copy receipts. Attribute only the existing three nonnested copy scopes. Check whether changes in total requested bytes equal the changes in attributed copy bytes, rather than assuming no other effect.

Cancellation: one new ordinary and one new metered replay for all 216 changed-demand scenarios with alternating cancellation (432 processes), validating retained outputs and final ownership. These are ownership/source checks, not matched cancellation latency against explicit controls.

Total scheduled lifecycle processes: 8,136. Additionally run three old and three new ordinary clock calibrations (the existing runner uses 100,000 intervals), and old/new meter plus new-profile allocator self-checks. Diagnostics/cancellation precede excluded warmups and primary blocks. Run one benchmark process at a time.

## Accounting, bounds and interpretation

Keep DEPENDENCY_FIRST_CLOCK=off: execution and owned observation are combined. Source/preparation/input/setup, consumers and all engine/prepared/input/output disposal count in complete totals. Independent full-answer validation and preallocated recording storage are outside intervals. Preserve exact endpoint metadata and source multiplicity; outputs must survive producer disposal and metered final ownership must restore. Reuse the existing runner's exact phase structure and heap continuity checks. Requested heap is not RSS. Compilation/artifacts, startup, validation and sustained service remain excluded.

Pin children to available CPU 0, with 60 seconds wall/CPU, 1 GiB address space and the existing 2,000,000 service limit. Campaign cap 30 minutes excluding builds. Retain all receipts on a failure and stop for diagnosis; do not silently discard or restart failures. Source/binary hashes and job order are frozen before comparisons.

Audit all outcomes and exact diagnostic repeats. Compare new demand against its paired old policy and each explicit control, reporting every scenario without weights. Flag a timing comparison if either cell has any total below 100 × the largest calibrated empty-interval median × its number of phases. Do not classify reliable gains from signal-limited cells. Report median paired ratios and phase/allocation changes; register adequate confirmation if a consequential near-threshold conclusion is needed. A large local improvement need not close the complete architecture gap.

At the result, compare confirmation, further discovery/normalization changes, broader writable-head/dynamic-choice capability, broader call recognition and actual composed paths. This is package three after the full call-key allocation review; a full portfolio review is required within one further package. No result alone closes demand, architecture selection or the research goal.

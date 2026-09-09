# S04: reusable reunion preparation and requested-heap ownership

Establish whether saved private execution reduces requested allocation through complete queries, and whether its owners release correctly. This gate measures allocation only. The [source gate](../results/S04-reunion-source-gate.md) establishes the mechanism and its restricted correspondence; it does not establish total efficiency.

## Hypotheses and controls

H1: retaining a checked ruleset amortizes source checking without retaining completed query state. H2: private caches and state transport can offset saved private execution. H3: increased private work, component count, output payload and joining selectivity produce different allocation regimes. H4: first-answer cancellation can retain a different mix of private/product/resumed state from complete execution.

Compare reunion, ordinary Copy, Indexed, Indexed-COW and the existing permanent-factoring executor. All execute the same source. Permanent factoring yields one component on this source and therefore tests its coupled path, including its existing preparation and publication costs. That executor publishes unique answers: require the independent scalar evaluator's complete raw multiset to equal its actual full output on every measured query. Do not generalize this comparison to duplicate-answer sources.

Reunion uses reusable checked preparation with query-specific variable ownership validation. Copy and Indexed use their existing prepared rules. The permanent executor's current API redoes source preparation at query setup; charge that work there and disclose it as a control limitation rather than an intrinsic cost of permanent factoring. Do not create an extra baseline.

## Exact sources and matrix

Freeze `examples/reunion_cost.rs`. Four families share keyed two-choice recursive private jobs and a joining rule: plain joins all values; equal requires all selected values equal; late binds private unknowns at reunion and reactivates suspended local rules; payload adds eight distinct residual payload occurrences per owner, each with a depth64 term. All names and occurrence ownership are ordinary source data.

Use two/four owners, base private depths0/12/48, and one/four queries per prepared owner. Query seed i ranges from zero through reuse minus one, changes variable identities and payload tags, and uses base depth plus i modulo2. Expected complete raw answer count is two for equal, otherwise 2^owners; the independent evaluator validates actual terms, aliases and residuals, not just these counts.

There are 4 families × 2 owner counts × 3 depths × 2 reuse counts × 5 modes = 240 configurations, each run twice in separate processes: 480 allocation processes. Shuffle each complete repetition with Python Random seed20260910. Run serially, pinned to the first available CPU, under a 1 GiB address-space and 60-second per-process wall bound. The harness bounds each execution at 200,000 service calls. A failure/cutoff stops the driver with its receipt retained; diagnose before changing a bound or resuming. No timing comparisons occur.

## Measurement and validation

Build release allocation-meter and allocation-meter+COW binaries with replay, engine and kernel work diagnostics disabled. COW cells execute only Indexed. Freeze source/toolchain/binary hashes before samples. The meter measures requested heap traffic and live/peak requested bytes, not RSS. It records no timings here.

Each process runs the allocator self-check and complete oracle comparisons as a warm/preflight execution for every changed query. Then measure a fresh owner: preparation; per-query input construction, setup, complete execution/owned observation, engine release, answer release and input release; separately repeat input/setup/first answer/release for cancellation; finally prepared-owner release. Source-rule generation, oracle work and validation are outside measured phases. Outputs are retained through each query's end.

Require exact allocation readings across both repetitions; live-byte continuity between phases; return to the prepared-owner baseline after each complete query and cancellation; and return to the initial baseline after prepared disposal. Fixed bookkeeping buffers are allocated before the measured baseline. Complete answers are independently checked outside allocation phases, including actual measured executions.

The harness cannot use a first-answer cutoff to claim full computation completed. Report cancellation separately from complete-query traffic. Preparation and query inputs are included, but compilation and process startup are not. Counter-free ordinary tests and separate diagnostic-feature tests must pass; diagnostics disappear through conditional compilation, not by ignoring their readings.

## Interpretation and next decision

Report traffic, peak growth and prepared/query/consumer retention by phase. Do not infer speed from allocation or sum peaks. Attribute large costs to source work, copying, output ownership or retained caches before any architectural conclusion. A credible allocation gain requires ordinary timing and adverse regimes; a loss requires investigating a plausible consequential representation defect.

This is the fourth bounded T077 package. At its boundary compare further reunion attribution/timing with T072 integrated dependency repair, and review broader restoration, adaptive splitting, repeated dynamic reunion and lifetime obligations. Priority does not resolve any of them. Register ordinary timing separately only when its result could change the decision and controls are adequate.

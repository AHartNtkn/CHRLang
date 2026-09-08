# S04 lifecycle pilot: restoring or replaying state has not earned a general advantage

All 1,008 registered processes complete with correct full observations; all 144 cells replay their allocation diagnostics exactly. Copying performs well on small and retained-state cases, while indexed execution is much faster and allocates less traffic on mutation. The tested trail and replay policies do not establish a general time or memory advantage.

These results describe the implemented organizations and source sizes. They do not reject other trails, checkpoint policies or service quanta. A concrete matcher-copy cost requires investigation before interpreting the large mutation loss as an architectural obligation.

## The comparison

The [registration](../registrations/S04-lifecycle-pilot.md) compares six restoration configurations with existing Global Indexed execution, ordinary and COW. Nine source families run one or eight changing queries per prepared ruleset. Five counter-free ordinary-allocator timing processes and two requested-allocation processes per cell give 720 timing and 288 allocation processes.

Runtime lifecycle includes preparation, setup, joint execution/observation, engine and answer disposal, and final prepared-rule disposal. First observation and a separate first-answer-then-cancel scenario are also measured. Native compilation, startup, source generation and validation are excluded. Consumers retain all answers until each query finishes. The [measurement review](S04-measurement-review.md) explains phase-boundary and scheduling limits.

The initial allocation gate cutoff was isolated to root replay on mutation. Its registered bound was extended before matrix runs, without reducing the workload. The [sizing investigation](S04-replay-sizing.md) records the cutoff and checked work projection. All final matrix processes complete within their registered limits.

## Opposing regimes in total runtime and memory

Eight-query batch medians are shown below. Requested MiB is cumulative heap traffic, while peak KiB is incremental live requested heap above the pre-preparation baseline. Neither is RSS.

| Case and organization | Runtime ms | Requested MiB | Peak KiB |
|---|---:|---:|---:|
| Retained — Copy | 4.011 | 3.118 | 184.700 |
| Retained — Trail | 6.699 | 3.579 | 201.004 |
| Retained — Indexed | 6.680 | 11.098 | 1,070.188 |
| Retained — Indexed COW | 6.224 | 10.347 | 975.134 |
| Mutation — Copy | 483.422 | 796.015 | 201.813 |
| Mutation — Trail | 877.781 | 969.922 | 4,125.339 |
| Mutation — Checkpoint 1 | 544.733 | 898.263 | 208.290 |
| Mutation — Indexed | 21.183 | 25.518 | 1,206.670 |
| Mutation — Indexed COW | 20.560 | 24.649 | 1,096.490 |

**Copying wins the retained-store comparison on all three displayed costs.** Its paired runtime ratio against Indexed is 0.612, with observed range 0.561–0.679. This favorable case is real, but it uses 128 cells and a particular consumer; it does not establish a universal snapshot default.

**Mutation presents a time/traffic versus peak tradeoff.** Copy/Indexed has median paired runtime ratio 23.034, ranging from 21.954 to 24.651. Indexed also requests about 31 times less cumulative allocation, while Copy's peak is about one sixth as large. Those facts cannot be collapsed into a preference without a relevant resource requirement.

**The trail policy's memory result is adverse, not merely its runtime.** On mutation it peaks near 4 MiB versus Copy's 202 KiB; on the work-heavy case it peaks at 1,487.752 KiB versus 31.545 KiB. The implementation retains reversible paths containing prior source occurrences and pending effects while siblings need them. The exact contribution of those owners remains to be diagnosed before making a claim about necessary trail memory.

## Replay and checkpointing

Root replay takes a median 112.948 seconds for the eight-query mutation batch and requests 184,213 MiB cumulatively, while peaking at only 209.245 KiB. Large allocation traffic is repeatedly allocated and freed; it is not simultaneous resident storage. Checkpoint 1 reduces this to 0.545 seconds and 898.263 MiB, but still does not beat Copy's costs on that case.

Replay can reduce peak modestly elsewhere. On the work-heavy case it peaks at 27.345 KiB versus Copy's 31.545 KiB, while taking 2,677.574 ms versus 18.087 ms and requesting 2,960.973 MiB versus 20.899 MiB. This quantifies a tradeoff; it is not evidence for an unconditional replay policy.

The observed source-work counts and checked projection explain repeated prefixes. They do not establish that every replay implementation must repeat them: the current root policy reconstructs even when the same lone interpretation will be serviced again. Working-state reuse, checkpoint placement and service quantum remain consequential alternatives.

## Latency, reuse and remaining uncertainty

First observation and cancellation preserve the main mutation contrast. For one-query batches, Copy's cold first observation is 61.017 ms versus Indexed's 2.397 ms; their separate first-answer/cancel scenarios take 60.782 and 2.506 ms. Full phase and latency results are in the [summary](s04-lifecycle/summary.json) and [phase table](s04-lifecycle/phases.csv). Cancellation is not included a second time in the complete-query total.

For eight-query batches, Trail is consistently more than 20% slower than Copy on small, retained, mutation, work, deep, spine and late rejection. Its early-rejection comparison remains uncertain: paired Trail/Copy ranges from 0.782 to 1.359. Copy/Indexed on the deep case ranges from 0.837 to 1.041. These ranges establish neither equivalence nor a practical winner.

Peak requested heap is unchanged between one and eight sequential queries in each configuration, consistent with release between queries and immutable prepared rules. That is bounded ownership evidence for this batch model, not a long-stream reclamation result.

## Decision and next investigation

The pilot establishes opposing source regimes and substantial costs for these trail/replay policies. It does not select a whole architecture or close S04. In particular, the common restoration matcher clones slot environments before rejecting partners whose keys disagree with previous heads; the mutation comparison is sensitive to whether that work is avoidable.

The [registered matcher-copy diagnostic](../registrations/S04-matcher-copy-diagnostic.md) predicts 36,864 rejected-partner environment copies and 1,572,864 cloned source-term nodes per mutation query. Validate that prediction before changing the matcher, then test a conservative precheck with an all-compatible adverse case if the cost is material. Preserve this pilot's binaries and provenance for any paired comparison.

That attribution check is selected before S05 stable-identity operation/failure reuse because the measured mutation gap can change the interpretation of the current state-organization comparison, and the proposed check has a small semantic surface. This is a prioritization judgment. S05 remains required; further matcher work must not continue without consequential evidence.

Larger immutable payloads, better working-state/service policies, adaptive splitting, temporary separation/reunion and sustainable lifetime remain distinct S04 obligations. Whole architectures and held-out challenges remain open.

## Validation

The [full audit](s04-lifecycle/audit.json) verifies 1,008 successful process receipts, all 144 cells, exact two-run allocation replay, allocation restoration and source/binary freezes. Each process independently validates complete answers outside timing. Final source/build gates, strict Clippy, release regression tests and formatting are linked from [the receipt directory](s04-lifecycle/). The reference interpreter is unchanged.

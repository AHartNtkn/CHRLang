# Repeated separation saves traffic only when there is useful private work

Repeated reunion reduces requested allocation on longer independent work, but adds costs when reconnection is frequent or late links prevent useful separation. All allocation repetitions and disposal checks pass. The next investigation moves to broader integrated matching and consumption; adaptive splitting remains required.

## What was compared

The [registered gate](../registrations/S04-repeated-reunion-ownership.md) compares ordinary Copy, initial-phase reunion and repeated reunion on identical source programs. Three families vary independent choices, 16 inherited propagation stamps per owner, and late shared unknowns. Additional rounds are 0/1/3, private depths 0/12, and prepared-rule reuse 1/4 changed queries. These are diagnostic families, not workload weights.

There are 216 measured processes across 108 cells, following 54 preflight processes. Every cell has two exactly matching allocation readings. A separate ordinary-allocator diagnostic run validates 54 work configurations against scalar. After instrumentation, 108 counter-disabled allocation replays match every original reading. All 378 allocation/preflight/replay processes complete without a service or resource cutoff.

Each measured process validates complete raw answers independently outside measured intervals, including changed queries. Query release and first-answer cancellation restore the prepared owner's live-byte baseline; prepared release restores the initial baseline. The audit independently reconstructs phase order, cell coverage, command arguments, exact pairs and memory continuity from receipts.

## Where repeated reunion helps and hurts

The table reports **requested MB per prepared owner serving four complete queries**, including preparation, inputs and disposal. It excludes the separately measured cancellation query. One MB is 1,000,000 bytes. All examples have three additional rounds.

| Source family | Private depth | Copy | Initial-phase reunion | Repeated reunion |
|---|---:|---:|---:|---:|
| Plain independent choices | 0 | 2.506 | 2.518 | 2.794 |
| Plain independent choices | 12 | 14.153 | 13.921 | 10.719 |
| Inherited propagation history | 0 | 37.869 | 31.005 | 42.618 |
| Inherited propagation history | 12 | 114.881 | 103.424 | 50.671 |
| Late shared unknowns | 0 | 2.011 | 2.023 | 2.961 |
| Late shared unknowns | 12 | 13.556 | 13.324 | 17.896 |

Repeated reunion's lower traffic can come with higher peak requested memory. In the history/depth12 example, peak growth is 687,563 bytes for Copy, 739,961 for initial-phase reunion and 1,107,544 for repeated reunion. For plain/depth12 those values are 70,683, 74,216 and 88,907 bytes. Peaks are measured over the lifecycle, not summed across phases.

Even no additional rounds can expose a cost. For plain/depth0 with four queries, traffic is 175,445 bytes for Copy, 186,213 for initial-phase reunion and 190,715 for repeated reunion. This prevents attributing every repeated-engine cost to useful additional decomposition.

## Why the late-link loss matters

The diagnostic counts show no avoided source work in the late/depth12/three-round case. Copy and repeated reunion both execute 2,886 source-step calls; initial-phase reunion executes 2,830. Repeated reunion additionally checks 423 boundaries, creates 29 successful separation epochs and copies 384 inherited binding entries into components. This is a measured work liability, not merely an unfavorable allocator result.

Independent private work behaves differently. For plain/depth12/three rounds, Copy executes 2,984 source steps, initial-phase reunion 2,928 and repeated reunion 2,144. Repeated reunion performs 31 boundary checks and copies 136 inherited binding entries. At depth0 it still reduces source steps from 824 to 704, but its allocation traffic is higher. Counting fewer source steps alone therefore does not establish efficiency.

Retained history adds another explicit responsibility. The history/depth12/three-round case copies 1,920 inherited history entries into components across the query, in addition to 424 binding entries. Its source steps fall from Copy's 3,176 to 2,304, and its traffic falls substantially, while peak demand rises. These counts describe the current implementation; they do not prove every correct representation must copy these entries.

## Interpretation and next investigation

Keep repeated reunion as a qualified candidate with a favorable regime and a measured adverse regime. Do not select eager separation as the general policy. A useful adaptive comparison must distinguish eligibility from benefit, account for boundary checks and retained contexts, and include late links where independent execution avoids no work. That comparison remains unfinished.

Select T072 broader integrated heads and consuming history next. Its next source gate should combine equality updates with multi-head matching, occurrence-sensitive propagation history and competing consumers, comparing the local rewrite organization with conventional execution and the independent scalar evaluator. This can expose responsibilities or eliminate work that the current single-request integrated comparison does not cover.

The strongest nearby alternatives are adaptive reunion and resource-aware source lowering. Adaptive reunion now has concrete signals to investigate, but another local refinement would not answer the broader integrated-language question. Resource-aware lowering remains required as a distinct way to eliminate execution. This selection is a prioritization judgment after the bounded reunion source and ownership packages; it is not a resolution of either alternative.

## Measurement limits and reproducibility

These are requested-heap allocation measurements, not RSS or timing. The engine API constructs owned answers during execution, so observation is included in that interval; there is no separate execution-versus-observation attribution. First-answer cancellation is recorded separately. Compilation, process startup and source/oracle generation are outside the measured lifecycle. No speed, compilation-inclusive or complete-architecture ranking follows.

The [allocation audit](s04-repeated-reunion-ownership/audit.json), [instrumentation replay](s04-repeated-reunion-ownership/instrumentation-audit.json), [work records](s04-repeated-reunion-ownership/work.jsonl) and [binary/source freeze](s04-repeated-reunion-ownership/freeze.json) retain the evidence. The measured reunion source and original registration are archived alongside the records; current source adds diagnostic-only counters. The drivers stop on failures and preserve receipts.

Restoration tests pass in ordinary and diagnostic configurations. Clippy checks both new runners with all warnings denied except constant assertions: their runtime feature guards deliberately reject invalid measurement configurations. Formatting and local evidence links are checked. The research goal and the broader 57-question sequence remain active.

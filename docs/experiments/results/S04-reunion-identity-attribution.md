# Identity transport avoids unnecessary copies, but payload overhead remains

Sharing immutable values when variable relocation is exactly the identity reduces requested traffic in all 48 reunion configurations. The repair explains a large part of the payload overhead, but short payload queries still request more bytes than Copy. No timing claim follows.

The [registered attribution](../registrations/S04-reunion-identity-attribution.md) repeats all 480 allocation processes. Every one of the 240 allocation pairs reproduces exactly; all 384 unchanged-control process pairs match the baseline. Source and binary hashes, baseline source snapshots, query/prepared restoration and phase continuity all validate.

## The changed responsibility is narrow

During product construction, the candidate computes a fresh-variable offset for each component. When the offset is zero, both original and private variable identities are unchanged. Binding terms and constraints can therefore retain their existing immutable Arc values. The product still owns its own maps, and occurrence identifiers and propagation-history references still receive their required offsets.

A failing ownership test directly exposed copied values on this identity path. The passing test now requires sharing while checking occurrence/history relocation. Nonzero variable relocation continues to construct renamed values; the existing fresh-alias, consuming-source, propagation-history and finite-sibling tests remain active. The repair neither deduplicates source alternatives nor changes their scheduling or answers.

## The measured effect

Four-owner, four-query figures below include preparation, query input, execution through owned answers and disposal. Traffic is requested MiB and peak growth is requested KiB.

| Source | Traffic before → after | Peak before → after |
|---|---:|---:|
| Plain, depth0 | 0.666 → 0.597 | 45.2 → 37.3 |
| Plain, depth48 | 10.005 → 9.935 | 54.4 → 50.8 |
| Late binding, depth0 | 1.146 → 1.051 | 57.6 → 53.3 |
| Payload, depth0 | 15.598 → 8.958 | 2,053.6 → 1,919.1 |
| Payload, depth48 | 24.936 → 18.296 | 2,062.8 → 1,928.3 |

Requested traffic changes only in complete execution/observation and first-answer cancellation, across all 48 reunion configurations. Preparation and query setup traffic are unchanged. The observed end-of-phase live bytes in the inspected payload example are also unchanged: temporary transport copies affect traffic and peak, while retained answers still dominate the final observation boundary.

The short payload result remains adverse to Copy: 8.958 versus 8.262 MiB in the displayed four-owner/four-query case, and 476,546 versus 365,936 bytes in the two-owner/one-query case. A consequential confound has been corrected; the underlying tradeoff has not disappeared.

## Architectural disposition

Keep the identity repair as the measured implementation. It avoids a responsibility that has no semantic purpose when relocation is identity, while retaining the general renaming path. Do not extrapolate the same saving to components requiring nonzero relocation or claim that allocation determines speed.

The [complete ownership report](S04-reunion-ownership-gate.md) records the current controls and next selection. Before ordinary timing, qualify inferred specialization/Scan and the possibility of eliminating ground private countdown work. The carrier checker's current restrictions do not settle that opportunity. Integrated execution, broader dynamic reunion and sustained lifetime remain independent required investigations.

[Audit](s04-reunion-identity/audit.json), [all corrected readings](s04-reunion-identity/summary.json), [freeze](s04-reunion-identity/freeze.json), [failing ownership test](s04-reunion-identity/red.log), [ordinary tests](s04-reunion-identity/tests.log), [diagnostic tests](s04-reunion-identity/tests-diagnostic.log) and [Clippy](s04-reunion-identity/clippy.log) preserve the evidence. The driver `research/chr-restoration/experiments/reunion_identity_pair.py` verifies baseline snapshots and exact unchanged-control readings as part of its audit.

# Ground evaluation helps projection but does not beat enumeration

**The repair qualifies faster than the original projection in 55 of 64 unequal-name
cases. It qualifies faster than enumeration in none of 192 cases.** Requested
allocation falls on unequal-name stars, while peak ownership stays unchanged.
Equality sources provide an unchanged-allocation control.

The [registered comparison](../registrations/S06-connected-repair-costs.md) ran
1,152 separate allocation processes followed by 5,760 ordinary processes. Original
binaries match the previous pilot's hashes. Repaired builds disable diagnostics;
the allocator meter runs separately. All three configurations retain identical
source, query, weighted-output, retention, cancellation and disposal endpoints.
Each process checks full independent answers before measurement and measured
records outside clocks. No builds ran alongside timing.

| Ratio, 64 scenarios per row | Qualified gains | Qualified losses | Unresolved | Median time ratio range |
|---|---:|---:|---:|---:|
| Repaired/original, equality star | 0 | 0 | 64 | 0.79–1.24 |
| Repaired/original, equality clique | 0 | 0 | 64 | 0.79–1.37 |
| Repaired/original, unequal-name star | 55 | 0 | 9 | 0.40–0.66 |
| Repaired/enumeration, equality star | 0 | 63 | 1 | 2.81–5.77 |
| Repaired/enumeration, equality clique | 0 | 62 | 2 | 3.47–12.01 |
| Repaired/enumeration, unequal-name star | 0 | 58 | 6 | 1.34–5.07 |

Every comparison clears the registered clock floor. Qualification also requires
all ten paired ratios on the same side of one and a median difference of at least
10%. Contrary observations remain included; favorable medians alone do not change
an unresolved verdict. The unchanged equality paths do not qualify in either
direction, despite their spread of medians.

## Allocation explains what improved

On unequal-name stars, repaired/original requested-byte ratios are 0.365–0.414.
The peak ratio is exactly one in every case. Direct atom checks remove temporary
theory construction, but do not remove the larger retained state elsewhere in the
session. Equality-source requested bytes and peaks match the original exactly.

Against enumeration, repaired projection still requests 4.05–6.99 times as many
bytes on unequal-name stars and peaks 1.20–2.10 times higher. Equality-star ratios
remain 5.13–7.87 requested and 1.77–2.34 peak; clique ratios are 6.53–11.24 and
2.16–2.23. Every repeated allocation record is exact, consumer-owned bytes match
all three configurations, and every session restores its original heap ownership.
These are requested heap measurements, not RSS.

## Decision

Keep the simpler direct ground evaluator. This is an experimentally supported
local improvement, not support for choosing projection over early-filtered
enumeration in these regimes. Larger domains, sparse factor traversal, broader
query reuse, symbolic terms and ordered source transport remain separate
investigations; no fixture becomes a language boundary.

The [full portfolio review](S06-connected-repair-review.md) selects T075's
scheduling-preserving whole-call recognition next. Projection has now received
source repair, full costs, attribution, a targeted implementation repair and fresh
costs. A different way to remove recognition and retained intermediate states is
more discriminating now than extending this component campaign immediately.
T076 remains unfinished. The goal remains active.

[All 384 paired comparisons](s06-connected-repair-costs/analysis.json) ·
[Executable audit](../../../research/chr-structural/experiments/connected_repair_costs.py).
The audit verifies all 6,912 terminal processes, command identities, exact ownership,
complete phase sums and frozen source/binary inputs.

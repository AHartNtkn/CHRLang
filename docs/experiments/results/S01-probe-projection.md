# Duplicate projections: repair the spike and strengthen the control

**Two avoidable costs are repaired.** Duplicate projections now retrieve each current bucket once, and all execution variants stop searching a rule when a required predicate pool is empty. The second repair benefits the existing control as well as the probe, changing the comparison materially.

## The adverse source exposed both costs

At width 256, 255 later occurrences share one projection onto 256 current occurrences. The first probe retrieves **65,535 indexed entries in one engine step**. After one receipt consumes the only token, source-order search continues enumerating pairs before reaching the empty token head; complete exhaustion takes **131,595 steps**.

The fixture is valid: duplicate later occurrences remain distinct retained facts, and only one token is consumed. The independent expected answer includes every fact and the receipt selected by the original occurrence order. Excluding those duplicates would miss the cost we need to understand.

## Repairs and attribution

The probe now records selected projection keys within its construction and unions each corresponding bucket once. This preserves the candidate set, including the unkeyed-pool case. It still charges matching each later occurrence, key selection and the temporary deduplication set.

At rule-search admission, the shared engine checks required predicate pools. An empty pool makes that search impossible at that moment. The check is repeated for later admissions: it does not permanently disable a rule. New source-produced tokens and distinct nullary heads pass explicit tests.

| Duplicate source width | Original steps to exhaustion | Repaired steps | Original largest indexed retrieval | Repaired largest indexed retrieval |
|---|---:|---:|---:|---:|
| 16 | 555 | 41 | 255 | 31 |
| 64 | 8,331 | 137 | 4,095 | 127 |
| 256 | 131,595 | 521 | 65,535 | 511 |

These are operation counts, not elapsed times. Deduplication removes repeated bucket retrieval; empty-head admission removes impossible post-consumption searches. Both are needed to explain the table.

## The stronger control changes the conclusion

Empty-head admission does not depend on selective probing, so it is now shared with the existing control. On the duplicate width-256 source, **both finish in 521 steps**. The control's largest indexed retrieval is 255 entries; the probe's is 511, with 255 additional later-occurrence inspections in that step. The control also materializes an unkeyed current pool, which is recorded separately; indexed-entry counts alone are not total memory traffic.

The earlier selective source still offers a benefit, but its policy regimes separate further. At size128, right-bound, left-first, successful last-key query and indexed access:

| Policy | Shared control candidate visits | Probe candidate visits | Control / probe index lookups |
|---|---:|---:|---:|
| Global | 258 | 4 | 384 / 7 |
| Active | 640 | 640 | 638 / 766 |

The Active witness now saves no candidate work and pays additional eligibility lookups. The Global witness retains a substantial opportunity. Crediting the probe with the shared repair would exaggerate its value.

## What service evidence now establishes

The duplicate case's quadratic repeated retrieval is gone. For a fixed current-head arity, each selected index bucket is retrieved once; distinct keys for the same predicate and argument have disjoint occurrence sets. This analytically bounds that retrieval component by the number of indexed arguments times the current pool size, plus a possible unkeyed pool. It does not bound key normalization or matching independently of term structure.

One step still visits up to255 later occurrences in the measured duplicate case. Neither this work reduction nor equal step counts establishes constant-time service or a latency guarantee. The next experiment must charge those operations, allocations and cancellation directly; if the required service contract needs smaller units, probe construction must become resumable.

## Validation and next decision

Three frozen attribution processes and two separate24-process confirmations pass: **51 processes** with complete per-step traces, identical diagnostic repeats and metrics-off source agreement. Both partner-order families, ambiguous-consuming sentinels, the independent small ground tuple oracle, nonground differential traces and broad-pool controls are carried through both confirmations. The projection fixture reuses preparation across two changed queries. Tests check complete residual multiplicity, original tuple IDs, fresh head insertion and distinct nullary consumption.

Access, generated-access, join-screen and nullary-region regressions pass. Scoped Clippy passes. Feature-specific compilation of the control pool wrapper was validated afterward; its exact source delta is recorded beside the final freeze. Receipts are losslessly compressed with raw and compressed byte hashes. The reference interpreter is unchanged.

T082 next measures prepared and temporary allocations, retained ownership, cancellation and ordinary service/lifecycle costs against the strengthened control. This has more immediate decision value than another source-level search tweak: the mechanism now has both a credible favorable source and a clear adverse one, and its full price could reverse the choice. Reassess the broader portfolio at that result; this is package three since the full review, with review required by four. The goal remains active.

Registrations: [attribution](../registrations/S01-probe-projection.md), [repairs](../registrations/S01-probe-projection-repair.md), [shared control](../registrations/S01-probe-strong-control.md). Analyses: [causal comparison](s01-probe-projection-repair/analysis.json), [strong-control comparison](s01-probe-strong-control/analysis.json). Reproduce with `audit_probe_projection.py` and `audit_probe_strong_control.py` under `research/chr-compiled/experiments/`.

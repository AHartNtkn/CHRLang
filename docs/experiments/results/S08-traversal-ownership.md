# Traversal reuse saves dependency allocation, but costs memory elsewhere

**The allocation benefit depends on the engine and source.** Sharing completed traversal cuts dependency execution's requested allocation traffic on the continuing source, but increases template traffic there and increases traffic for both engines on flat sources. Maximum requested live heap rises in every matched comparison. The existing direct control requests fewer bytes than either graph variant throughout this gate.

All 336 processes pass:112 ordinary qualifications and 224 allocation runs covering112 cells with exact allocation repeats. Every producer, prepared artifact and retained consumer releases its requested heap. These are ownership results; the ordinary qualification readings do not constitute a comparative timing campaign.

## A fixture that exposes overhead

The continuing fixture preserves two changed queries, prepared-rule reuse, capacity-four delivery, blocked-producer checks, cancellation, immediate/all consumers and optional support reclamation. It uses32/128 delivered answers per query.

The new flat fixture contains one call with 1/32 direct alternatives returning a fresh aliased pair. The resource variant adds the same consuming finish/token rule. It has no recursively growing chain of completed calls. Two marked queries reuse preparation and run to full exhaustion; immediate/all consumers exercise output ownership. An independent scalar interpreter checks the complete source against the closed-form answer multiset before measurement. Every delivered answer and retained answer is checked outside the recorded intervals, including after producer disposal.

Both fixtures charge preparation, query setup, production/observation, consumer storage, cancellation or exhaustion, producer/prepared disposal and consumer disposal. Reporting buffers and oracle work are excluded. The flat runner's final Clippy annotation preserves a runtime counter guard; measured sources are archived exactly.

## What the matched allocation comparisons show

There are 32 continuing cache-on/off comparisons and 16 flat comparisons, spanning the registered engines, resource use, sizes, consumers and reclamation settings.

| Comparison | Requested traffic | Maximum requested live heap |
|---|---|---|
| Continuing dependency, including reclamation | Lower in all 16 cases | Higher in all 16 |
| Continuing template, including reclamation | Higher in all 16 cases | Higher in all 16 |
| Flat dependency/template | Higher in all 16 cases | Higher in all 16 |

For dependency execution without reclamation, continuing traffic ratios range from 0.127 to 0.510: roughly49–87% less requested allocation. Template ratios range from 1.099 to 1.175: roughly10–18% more. Flat ratios range from 1.004 to 1.079. Retained consumer ownership is identical between each graph's cache variants; the difference belongs to execution and memo ownership, not a different answer contract.

Representative continuing cases at 128 answers per query, immediate release, no reclamation (bytes):

| Engine / source | Control traffic → shared | Control peak → shared |
|---|---:|---:|
| Dependency / pure | 76,294,067 → 9,689,899 | 678,058 → 682,842 |
| Dependency / resource | 79,719,090 → 14,067,558 | 1,062,740 → 1,068,036 |
| Template / pure | 5,673,647 → 6,464,623 | 979,184 → 979,336 |
| Template / resource | 7,248,678 → 8,519,230 | 1,352,700 → 1,357,268 |

The direct controls request 766,053 bytes for pure and 1,308,354 for resource execution in those scenarios. Their peaks are 50,788 and 53,079 respectively. Direct has the lowest traffic in all 48 graph comparison scenarios, but this does not establish its elapsed advantage or a universal architecture choice. These source families do not exercise every proposed sharing or solving benefit.

For the flat resource source with 32 alternatives, immediate release, dependency traffic rises from 554,085 to 598,057 bytes and peak from 74,853 to 76,062. That is an actual adverse fixture, not an inferred penalty. Even the one-alternative cases incur additional requested bytes.

Reclamation remains operational. With shared traversal at the continuing resource128 endpoint, it lowers dependency peak from 1,068,036 to 700,452 bytes while adding1,792 requested bytes. It lowers template peak from 1,357,268 to 994,964 with the same extra traffic. Pure-source reclamation has no incompatible resource history to remove and reproduces its unreclaimed traffic.

## Architectural consequence

The [work result](S08-completed-traversal.md) and this allocation result jointly show why component counters cannot choose the architecture. Both graph engines traverse less, yet one allocates more. Templates retain fewer completed results and obligations, but use more peak heap than dependency execution in these witnesses. Direct execution remains a credible competitor.

The memo's current sets, maps and per-probe path vectors are implementation choices. Dense-ID storage or reused path capacity could change its allocation cost and must be compared if that cost affects adoption. They are not semantic necessities. Complete elapsed/RSS costs and broader lifetime regimes also remain required; no default cache policy is adopted.

The [full portfolio review](S08-traversal-portfolio-review.md) selects useful partial equality and consuming execution under T072 next. It can answer whether a different organization avoids source work through early information, a promise that these lifetime refinements cannot test. T074 retains memo-storage alternatives, complete timing, RSS, wider retention and regeneration as explicit work. The review count resets and the research goal remains active.

[Registration](../registrations/S08-traversal-ownership.md), [frozen sources and receipts](s08-traversal-ownership/), [exact audited rows](s08-traversal-ownership/audit.json), [auditor](../../../research/chr-reuse/experiments/audit_traversal_ownership.py). Reproduce with `PYTHONDONTWRITEBYTECODE=1 python3 research/chr-reuse/experiments/audit_traversal_ownership.py`.

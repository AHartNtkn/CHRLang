# Structural solving has a favorable equality regime and substantial contrary cases

Lazy finite-path solving is cheaper than enumeration in the inspected equality case, while reused enumeration is cheaper on unselective and overlapping descriptions. The exact-family elimination control is cheaper in these examples. Keep structural solving as a candidate with unresolved scope, and investigate restoration/reunion next rather than refining a ranking on this small family.

These are exploratory ordinary-allocator timings. The [registration](../registrations/S06-finite-timing-pilot.md) specifies five samples per cell, descriptive medians/ranges and no formal gain/loss classifications. It does not supply a universal architecture ranking.

## The registered matrix passes

All **1,800 processes across 360 cells** completed with the required five repetitions. The audit validates exact cell/repetition coverage, phase order, complete/cancelled answer counts, absent first observations for empty results and timing bounds for observed first answers. Every process independently validates four complete query results before fresh measured preparation. Source and binary hashes match the freeze.

The engine and sources are unchanged from the validated overlap-attribution gate. Primary timings use release code, ordinary allocation and disabled finite-solver counters. The [separate allocation gate](S06-overlap-factor-attribution.md) supplies exact traffic and disposal evidence; its elapsed times are not used here. Native compilation, process startup and parsing are outside the endpoint. The preflight warms code and allocator state.

The primary total includes preparation, request construction, setup, execution with owned observation, query disposal, consumer release and final prepared disposal. Four changing queries share one prepared owner. Immediate answer disposal occurs inside execution/observation. This is not an isolated one-query-lifetime measurement or a sustained-cache study.

## Equality and enumeration expose different costs

These are four-query totals in microseconds for six-bit sources with immediate answer release and complete enumeration. Each range contains the five observed values.

| Path | Equal positions: median [range] | All values: median [range] |
|---|---:|---:|
| Fresh enumeration | 690.87 [425.82–737.41] | 918.32 [764.83–983.34] |
| Reused enumeration | 224.68 [168.48–230.00] | 466.62 [318.55–471.12] |
| Lazy solving | 110.94 [105.44–112.66] | 2,262.39 [1,518.45–2,417.53] |
| Reduced lazy solving | 111.93 [72.98–140.62] | 1,723.99 [1,529.03–2,317.59] |
| Exact-family elimination | 16.56 [15.43–22.80] | 127.31 [89.79–156.58] |

The equality source has only two accepted values among 64 possible lists. Lazy propagation avoids materializing that candidate language. On the all-values source, every value is required and the solver's search representation and observation history add work. Several ranges are wide; these observations do not justify fine distinctions between reduced and unreduced execution.

Allocation adds a separate tradeoff. In the equality case, lazy solving requests 200,261 bytes with 13,748 bytes peak growth; reused enumeration requests 140,284 with 91,376 peak growth. Thus lower traffic, lower peak and lower time do not necessarily select the same path. Those allocation values are from the separately validated control data; they are not estimated from elapsed time.

## Factoring helps overlap without making the solver the strongest control

For six-bit overlap, unreduced lazy execution has a median of **25,317.07 µs** [24,916.71–29,411.37]. Factored reduction has **1,722.73 µs** [1,460.33–2,265.71]. The ordinary matrix compares the two current modes; the earlier paired allocation attribution isolates the reducer change itself.

Reused enumeration remains cheaper at **414.73 µs** [310.51–534.72], and exact-family elimination at **102.49 µs** [84.31–130.21]. The source-derived control is deliberately strong for this exact family; it is hand-derived and does not establish a general lowering compiler.

Other adverse cases remain visible. For selective membership, reduced solving has a 396.79 µs median versus 51.52 for reused enumeration and 13.69 for exact-family elimination. For identical redundant alternatives, the corresponding medians are 346.05, 34.18 and 10.32 µs. Reduction preparation contributes 21.83 and 23.72 µs respectively. Empty membership also favors the enumeration controls in this inspected size; early emptiness is not automatically cheapest through the current lazy machinery.

These are bounded results for the actual implementations, not intrinsic costs of structural solving. Generalized reduction, better state ownership or a source where the exact control cannot eliminate the work could change the comparison. Those mechanisms require their own evidence.

## First observation differs from complete-query cost

In the six-bit equality case, the setup-to-first-answer median for lazy solving is **24.63 µs on query 1** and **20.86 on query 4**. Reused enumeration takes **128.04 µs on query 1** but **1.89 on query 4**, after candidate construction has been paid. Preparation is separate. A faster later first answer does not imply cheaper complete execution: the reused candidate domain still has to be filtered.

Cancellation is measured independently. In the six-bit all-values case, four first-answer/cancellation queries cost a median **136.59 µs** for lazy solving and **104.01 µs** for reused enumeration, compared with their much larger full-enumeration totals. Their cancellation ranges overlap, so this pilot does not establish a close ordering. The separate allocation gate checks query and prepared disposal after cancellation; unfinished search is not reported as complete enumeration.

## What is resolved and what stays required

The pilot establishes that the finite-path mechanism has a favorable measured regime and contrary regimes under complete four-query accounting. It also demonstrates why preparation, reuse, first observation, full output and memory must be compared separately. It does not resolve broader structural solving, language theories, consuming-source correspondence or architecture selection.

Additional precision on these six-bit mode rankings cannot answer the larger question: the strongest exact-family control already changes the comparison, and all outcomes leave broader source applicability open. Larger descriptions, longer reuse, general correlated filters and source-derived eligibility may change architectural conclusions. They remain required under T076; they are not dismissed as low-value research.

Select [restoration/reunion](S06-finite-next-investigation.md) next. It tests a distinct state-ownership mechanism, whereas another confirmation of this pilot would mainly refine the current fragment. Current repository code already implements root replay and configurable checkpoint intervals; reuse those controls and investigate the unanswered policies and actual reconnection.

## Evidence and reproduction

Run `PYTHONDONTWRITEBYTECODE=1 python3 research/chr-structural/experiments/finite_timing.py --audit`. The runner without `--audit` launches the registered experiment and refuses to overwrite existing records. The ordinary binary is identified by its frozen temporary path and hash.

[Raw runs](s06-finite-timing-pilot/runs.jsonl), [all cell/phase summaries](s06-finite-timing-pilot/summary.json), [audit](s06-finite-timing-pilot/audit.json), [source/binary freeze](s06-finite-timing-pilot/freeze.sha256), [toolchain](s06-finite-timing-pilot/toolchain.txt) and [host](s06-finite-timing-pilot/host.txt) preserve the experiment. All five samples remain available, including unfavorable and noisy cells.

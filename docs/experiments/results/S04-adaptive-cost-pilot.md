# Adaptive checking has no selected timing policy

The lifecycle pilot does not establish a generally preferable backoff policy. It finds a few separated comparisons, but most overlap and stronger controls remain competitive. Broader adaptive search remains required work.

All 768 qualification processes and 2,688 pilot processes passed the recorded endpoint and ownership audit. The [registration](../registrations/S04-adaptive-cost-pilot.md), [manifest](s04-adaptive-cost-pilot/manifest.json), [schedule](s04-adaptive-cost-pilot/schedule.json) and [complete audit](s04-adaptive-cost-pilot/audit.json) preserve the scope and observations. This is exploratory warmed service, not confirmation or an architecture ranking.

## What the comparison changes

**Saving failed checks is insufficient to choose backoff.** In late-link, depth-zero, four-query first-answer cancellation, backoff cap 1 takes 1.591–1.993 ms against eager's 2.254–3.846 ms. That passes the registered separation rule. Copy takes 1.221–1.588 ms, however, and initial reunion takes 1.256–1.576 ms. The improvement against eager does not establish an advantage over ordinary execution.

**A gain against Copy need not favor adaptation over its other competitors.** In history, depth-four, single-query cancellation, cap 1 takes 6.692–7.929 ms against Copy's 9.693–13.994 ms. Fixed skip 1 takes 4.677–6.219 ms. The history fixture has no failed-check allocation benefit for backoff; this result cannot attribute the improvement over Copy to backoff itself.

**Smaller allocation does not settle the remaining timing comparisons.** The table shows full exhaustion of four changed queries at depth four, with immediate answer release. Times are medians, followed by all-sample ranges. Heap values are requested bytes across charged phases, not RSS.

| Source | Control | Lifecycle ms: median [range] | Requested bytes |
|---|---|---:|---:|
| Late links | Copy | 3.569 [3.297–4.089] | 4,735,118 |
| Late links | Eager | 4.969 [4.723–6.919] | 6,576,704 |
| Late links | Backoff 1 | 4.136 [4.069–4.719] | 5,924,162 |
| Late links | Backoff 8 | 4.255 [3.610–4.923] | 5,640,668 |
| History | Copy | 40.789 [39.077–57.520] | 62,418,394 |
| History | Eager | 40.915 [36.513–45.536] | 44,557,436 |
| History | Backoff 8 | 38.354 [35.017–49.581] | 44,557,436 |
| Plain | Copy | 3.475 [3.226–4.048] | 5,263,214 |
| Plain | Backoff 8 | 3.211 [2.761–3.401] | 4,611,576 |

## How much remains uncertain?

**Variation is large enough to obscure consequential differences.** Across 384 cells, the median ratio of slowest to fastest sample is 1.416; 139 cells exceed 1.5, and the largest ratio is 2.280. These are within-cell spreads, not uncertainty intervals. The conservative registered rule requires the candidate's slowest sample to beat 90% of the control's fastest sample for a gain; losses use the converse 110% threshold.

| Candidate versus control | Gains | Losses | Unresolved |
|---|---:|---:|---:|
| Scheduled eager / stateless eager | 0 | 0 | 48 |
| Backoff 1 / Copy | 1 | 5 | 42 |
| Backoff 1 / initial reunion | 0 | 7 | 41 |
| Backoff 1 / eager | 1 | 0 | 47 |
| Backoff 1 / fixed 1 | 0 | 2 | 46 |
| Backoff 1 / fixed 8 | 0 | 5 | 43 |
| Backoff 8 / Copy | 0 | 9 | 39 |
| Backoff 8 / initial reunion | 0 | 9 | 39 |
| Backoff 8 / eager | 0 | 0 | 48 |
| Backoff 8 / fixed 1 | 0 | 5 | 43 |
| Backoff 8 / fixed 8 | 1 | 5 | 42 |

**Preparation is not the main measured cost in these fixtures.** First-answer and remaining service together constitute at least 91.1% of the sum of phase medians in every cell, with a median share of 96.6%. For the depth-four examples above, preparation medians are roughly 4–12 microseconds. This points future attribution toward service, allocation and timing variability, rather than tuning preparation to explain millisecond differences. A sum of phase medians is descriptive and is not an individual lifecycle sample.

**The pilot does not identify the cause of timing variation.** Exact paired allocation traces rule out differing requested-allocation sequences in those diagnostic repeats; they do not explain primary timing variation. The receipts do not separate scheduler delay, CPU frequency, allocator state or cache effects. No claim about a noisy host or negligible scheduling overhead follows. A targeted follow-up must prospectively register randomized paired blocks and elapsed/CPU attribution, retain contrary controls, and preserve these samples unchanged. It must not repeatedly resample until a preferred policy wins.

## What was actually validated and charged?

Eight execution modes cover plain, history and late-link sources; depths zero/four; one/four changing queries; exhaustion/first-answer cancellation; and immediate/all output retention. Five ordinary-allocator, diagnostic-counter-free runs and two allocation runs cover every cell. The operational service bounds remain enabled. Static engine types preserve the stateless eager control's smaller representation.

The primary endpoint charges source construction, preparation, source disposal, query input and setup, first owned observation, remaining service, engine/input disposal, prepared disposal and consumer disposal. Rules are reused across changing queries through prepared ownership. Output retention remains valid after producer disposal; all metered owners return to their initial baseline. The source's creation is included here, so byte totals differ from the preceding owner gate.

Complete same-source preflights use the independent scalar evaluator outside timing. Measured outputs pass through black_box, and counts, exhaustion and cancellation are checked. Retained measured answers are independently validated after producer disposal. Immediate-release answers cannot be reread: their semantic evidence is the identical-source preflight and preceding owned-output qualification, not a count-only claim of general correctness. Oracle work and preallocated bookkeeping are outside charged phases. Preflight warms the process. Compilation, artifacts, process startup and RSS are outside this experiment's claim.

Strict primary and allocation Clippy checks passed after runner repair; the first Clippy diagnostic is retained. The frozen-source audit verifies all 3,456 process receipts, exact matrix coverage, paired allocation equality and ownership endpoints. Reproduce the audit with `python3 research/chr-restoration/experiments/audit_adaptive_cost.py pilot` while the recorded binaries remain available.

## Next architectural question

The [full breadth review](S04-adaptive-cost-breadth-review.md) selects effectful caller reuse next. This pilot completes the four-package cycle: stateless-control qualification, owner qualification, timing-runner qualification and timing. Adaptive precision, delayed splitting, successful-but-unprofitable separation, checkpoints/replay, wider sources and sustained ownership remain required. No adaptive policy is adopted or rejected by this report.

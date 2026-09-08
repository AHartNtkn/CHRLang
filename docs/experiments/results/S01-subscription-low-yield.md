# Retaining low-yield joins repays repeated discovery

Subscriptions use about 30% of indexed rediscovery's complete runtime on the largest stable low-yield case; eager retention is similarly fast. Reopening demands favors eager retention because it keeps the useful tuple between requests. Together with the earlier maintenance losses, this establishes opposing retention regimes rather than a universal matching policy.

## The missing mechanism is now measured

The [initial pilot](S01-subscription-lifecycle.md) covered cheap selective discovery and dense output-producing joins. This follow-up gives every query N selected left endpoints, N selected right endpoints and 2N²+1 middle rows, but only one useful data triple. Repeated pulses therefore repeat unsuccessful search without multiplying output fanout.

The indexed control considers left-first, right-first and exact middle-index probes from both endpoint sets. It estimates current index degrees and charges the planning scans. On this source it selects N² endpoint-pair probes per pulse. Subscriptions use that same discovery when a demand first becomes live, then maintain the useful triple. The comparison does not exploit an unavailable two-endpoint access path.

The [registration](../registrations/S01-subscription-low-yield.md) fixes 123 cells and 861 processes, including contemporaneous frozen-indexed controls on three earlier sources. All runs completed within the registered resource bounds. The audit verifies exact allocation replay in all 123 cells and live-heap restoration after queries, bounded-prefix disposal and prepared disposal. Complete independent source observations pass; work diagnostics replay and match the registered plans, probes, construction and invalidation counts.

## Stable-demand runtime crossover

Each entry is the subscribed/indexed paired median runtime ratio and its five-pair range. Below 1 favors subscriptions. Bold entries meet the registered practical criterion: at least 20% median benefit, with every pair below 1. Ranges are pilot observations, not confidence intervals.

| Endpoint count N | 2 pulses | 16 pulses | 64 pulses |
|---|---|---|---|
| 4 | 0.985 [0.860, 1.300] | 0.870 [0.781, 1.011] | **0.715 [0.654, 0.717]** |
| 8 | 1.013 [0.954, 1.043] | 0.714 [0.678, 1.340] | **0.444 [0.438, 0.482]** |
| 16 | 0.953 [0.509, 0.982] | **0.606 [0.599, 0.625]** | **0.301 [0.291, 0.391]** |

The N8/16-pulse case has a favorable median but a contrary pair, so its practical benefit remains uncertain. N8/64 and N16/16–64 establish clear benefits. Refining the exact boundary would not change the present bounded conclusion that sufficiently repeated costly discovery can repay retention.

At N16/64, indexed execution performs 16,384 middle probes and 2,048 planning-row visits per query. Subscriptions perform 256 probes and 32 planning visits. The execution-phase medians across two queries are 4.345 ms for indexed and 0.159 ms for subscriptions. Preparation, setup, full observation and disposal reduce that much larger execution saving to a roughly 70% complete-runtime saving.

## Maintenance and lifetime change the choice

At N16/64, replacing the one real bridge before every pulse still favors subscriptions: paired ratio 0.469 [0.461, 0.482]. Its maintenance affects one useful tuple. This does not contradict the earlier dense-consuming loss, where many retained tuples had to be invalidated per consumed occurrence.

Retiring and reopening the demand changes the result. At N16/64, subscriptions repeat discovery and have a ratio of 1.048 [1.004, 1.100] against indexed execution: no practical difference. Eager retention keeps the bridge between demands and uses about one third of indexed runtime. At N4/64, subscription reopening produces a practical 26% loss against indexed execution. Demand lifetime is therefore a substantive ownership tradeoff, not merely an implementation label.

The table gives complete two-query runtime medians and requested heap diagnostics for N16/64. Requested traffic and peak live increments are not RSS; paired ratios need not equal ratios of these separate medians.

| Source | Executor | Runtime ms | Requested KiB | Peak KiB |
|---|---|---:|---:|---:|
| low-stable | indexed | 5.923 | 1908.8 | 685.1 |
| low-stable | eager | 1.742 | 1763.9 | 688.6 |
| low-stable | subscribed | 1.761 | 1776.7 | 690.4 |
| low-stable | compiled | 14.700 | 15959.7 | 525.6 |
| low-reopen | indexed | 6.073 | 2106.5 | 729.1 |
| low-reopen | eager | 2.011 | 1961.6 | 732.6 |
| low-reopen | subscribed | 6.376 | 2566.5 | 734.4 |
| low-reopen | compiled | 27.171 | 29329.5 | 543.3 |
| low-churn | indexed | 7.598 | 2624.5 | 744.1 |
| low-churn | eager | 3.585 | 2645.2 | 747.6 |
| low-churn | subscribed | 3.581 | 2671.9 | 749.4 |
| low-churn | compiled | 27.545 | 30024.9 | 543.6 |

The compiled control remains slower and allocates more cumulatively, yet has a smaller peak than the dedicated lowerings in these largest cells. Eager retention is at least as strong a runtime contender as subscriptions here because only one useful tuple exists; the earlier eight-group cases supply its unused-retention counterpressure. There is no workload weighting or universal winner.

## The access-plan change has a contemporaneous control

Frozen earlier indexed binaries ran alongside the revised implementation on identical N4/eight-pulse sources. Revised/earlier paired medians are 0.998 for selective, 1.012 for dense and 1.049 for consuming. Every range crosses 1, and none establishes a practical regression or improvement. This bounds the observed overhead on those controls; it does not prove globally optimal access planning.

The new control preserves the independent Cartesian-product update checks and complete source tests. The added low-yield gate checks all unmatched data rows and exact bridge receipts. The existing selective-final-endpoint test still selects the selective traversal, while the new two-endpoint test validates 64 probes and 16 planning visits at N8. Four deliberate source binding/resource/order faults still fail release checks, followed by a successful restored run. Counter-free release checks and Clippy pass.

## Architectural disposition and remaining scope

Retaining useful partial matches is economically viable on repeated costly discovery. Maintaining every join can waste work and memory on inactive combinations; maintaining only live demands can lose reuse across demand gaps. Cheap indexed discovery and consuming updates retain strong contrary cases from the first pilot. Choosing ownership therefore requires source/lifetime evidence, not a blanket preference for retention or recomputation.

The candidate implementations still own exact-source scheduling, bindings, data, ordered pending tuples and complete observation. This experiment does not resolve arbitrary join-stage selection, large active-demand populations, binding-dependency indexing, general generated multihead execution or coherent whole-language architectures. The current source is deterministic and finite; first observation is its complete quiescent answer. Native compilation, startup and external input/oracle generation are excluded. Bounded-prefix disposal uses different executor service units and is not an equal-work cancellation ranking.

T068's bounded subscription investigation is complete. The next investigation is [reusable parallel workers](S09-reusable-workers-entry.md), which can change the ownership and coordination decision without another local crossover refinement. Broader generated access and corrected restoration remain strong alternatives; the entry records their dependencies and the selection rationale. Broader S01 and the research goal remain open.

## Reproducible evidence

- [Source/binary freeze](s01-subscription-low-yield/freeze.json), [randomized order](s01-subscription-low-yield/order.json), [audit](s01-subscription-low-yield/audit.json), [summaries and paired ratios](s01-subscription-low-yield/summary.json), [phase data](s01-subscription-low-yield/phases.csv).
- [Runner](../../../research/chr-compiled/experiments/subscription_low_yield_pilot.py), [auditor](../../../research/chr-compiled/experiments/summarize_subscription_low_yield.py), [low-yield source](../../../research/chr-compiled/experiments/subscription_low_yield.rs).
- [Restored source gate](s01-low-yield-gate/release-restored.json), [counter-free checks](s01-low-yield-gate/release-counter-free.json), source hashes and deliberate-fault receipts in the same directory. Largest-case semantic sizing receipts are in `s01-low-yield-sizing`.
- The original lifecycle freeze is pinned to its source commit and remains auditable after the access change. The reference interpreter and independent source evaluator are unchanged.

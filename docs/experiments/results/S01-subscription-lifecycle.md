# Subscriptions reduce eager maintenance, but indexed rediscovery remains stronger here

Demand-driven subscriptions beat eager retention in six registered cells, but establish no practical runtime gain over indexed rediscovery. Consuming updates and repeatedly reopened demands expose clear subscription losses. These results leave expensive low-yield discovery unresolved; they do not justify rejecting subscriptions generally.

## What ran and what passed

The [prospective registration](../registrations/S01-subscription-lifecycle.md) fixes nine source families, two occurrence counts, two request counts and four executors: 144 cells. Each cell reuses prepared rules for two changed queries. All 1,008 comparative processes completed within 60 seconds and 1 GiB per process. Independent complete-answer gates passed for both timing and allocation builds.

Separate work runs replayed exactly and matched every registered construction, invalidation and retention prediction. The [receipt audit](s01-subscription-lifecycle/audit.json) verifies all source/binary hashes, exact allocation replay in all 144 cells, and live-allocation restoration after each query, the bounded cancellation run and prepared disposal. No cutoff occurred.

The primary endpoint includes preparation, both queries' setup, execution, observation, engine/answer disposal and prepared disposal. Query cloning is charged inside setup. Timings use counters disabled and the ordinary allocator; allocation and work diagnostics run separately. Native compilation, startup, fixture generation and independent validation are excluded.

## Runtime regimes

The table shows subscribed/indexed paired runtime ratios at N=4 and eight pulses. Below 1 favors subscriptions. Ranges contain all five observed pairs; they are not confidence intervals. The registered practical criterion requires at least 20% median change and every pair on the same side of 1.

| Source | Median ratio | Five-pair range | Interpretation |
|---|---:|---|---|
| binding | 1.037 | 0.994–1.081 | No practical difference established |
| churn-broad | 1.100 | 1.087–1.125 | No practical difference established |
| churn-sparse | 1.141 | 1.068–1.154 | No practical difference established |
| consuming | 2.232 | 2.053–2.434 | Practical subscription loss |
| dense | 1.145 | 1.085–1.211 | No practical difference established |
| inactive | 1.043 | 1.024–1.192 | No practical difference established |
| reopen | 1.258 | 1.209–1.271 | Practical subscription loss |
| selective | 1.103 | 1.054–1.199 | No practical difference established |
| unique | 1.278 | 1.257–1.338 | Practical subscription loss |

Across all 36 subscription configurations, none meets the practical gain criterion over indexed rediscovery, and ten meet the practical loss criterion. Subscription peak requested heap is never smaller than indexed peak in this matrix. This is a count of bounded observations, not a workload-weighted score.

Against eager retention, subscriptions establish practical gains in six N=4/two-pulse cells: binding, sparse churn, broad churn, inactive groups, reopening and unique demands. Those sources all contain eight data groups. Maintaining only the demanded group avoids otherwise unused tuples. At eight pulses, source execution and output costs reduce that advantage below the practical threshold in these comparisons.

All three dedicated lowerings avoid generic source selection. Subscriptions beat the existing compiled Global/Indexed control practically in every registered cell, but this does not isolate a retention benefit. Indexed rediscovery itself is a much stronger runtime control on this exact source, and the compiled organization retains contrary memory evidence.

## Traffic and peak remain distinct tradeoffs

Here are complete two-query lifecycle medians and requested heap diagnostics at N=4/eight pulses. Requested traffic is cumulative allocation; peak is the measured live increment. Neither is RSS.

| Source | Executor | Runtime ms | Requested KiB | Peak KiB |
|---|---|---:|---:|---:|
| inactive | indexed | 1.317 | 1666.4 | 591.7 |
| inactive | eager | 1.689 | 1935.2 | 706.9 |
| inactive | subscribed | 1.403 | 1695.2 | 597.0 |
| inactive | compiled | 22.099 | 31849.4 | 454.4 |
| consuming | indexed | 0.287 | 367.8 | 69.4 |
| consuming | eager | 0.605 | 647.1 | 71.8 |
| consuming | subscribed | 0.647 | 680.2 | 74.7 |
| consuming | compiled | 0.992 | 1460.7 | 62.9 |
| unique | indexed | 1.416 | 1689.1 | 596.7 |
| unique | eager | 1.761 | 1957.9 | 712.0 |
| unique | subscribed | 1.822 | 2024.1 | 602.0 |
| unique | compiled | 69.071 | 90243.4 | 462.0 |

For inactive groups, subscriptions reduce eager traffic and peak, but indexed rediscovery is smaller and faster. The compiled executor is much slower and allocates much more cumulatively, yet has lower peak heap than the dedicated lowerings. Total efficiency therefore cannot be reduced to either elapsed time or allocation traffic alone.

## What explains the bounded result—and what is still missing

Work diagnostics confirm that the intended mechanism operates. At N=4, eager inactive-group execution constructs and retains 512 data triples per query. Subscriptions construct 64, release them at last-demand retirement, and do no rediscovery on subsequent pulses. Indexed execution retains none and rediscovers matches. These are useful differences even though subscriptions do not win the complete runtime comparison.

The selective source has only one short indexed path; its discovery is cheap. Dense sources produce N³ actual receipts per pulse, so enumeration, candidate storage, source effects and output remain substantial even when discovery is reused. In the inactive N=4/eight-pulse subscription cell, execution has a 0.346 ms median, observation 0.402 ms, setup 0.271 ms and disposal 0.200 ms. Separate phase medians need not sum to the median total. The phase evidence supports inspecting complete-path costs, not attributing everything to matching probes.

Consumption makes maintenance unavoidable in these implementations. At N=4/eight pulses, retained policies construct and invalidate 512 triples per query while only 32 right occurrences produce receipts. Subscriptions take about 2.23 times indexed runtime. Reopening and unique demands repeatedly rebuild subscription tuples; their roughly 26–28% losses at this size meet the practical criterion.

The matrix does not contain an expensive unsuccessful search with very few emitted matches. Therefore it cannot establish whether the same retention mechanism pays when useful discovery is large relative to output and ownership costs. The [next low-yield contrast](S01-subscription-low-yield-entry.md) defines that missing witness and requires a competent two-endpoint access plan before timing it.

## Complexity, observation and limits

Subscription retention adds demand registrations, endpoint indexes, tuple ownership and reverse invalidation links. Eager retention has the tuple/incidence responsibilities without demand filtering. Indexed discovery avoids retained tuples. All three dedicated paths still own source scheduling, bindings, rows, pending candidates and observation; their exact-program certificate eliminates generic selection, not all runtime machinery.

Pulses currently materialize ordered candidate vectors. Binding updates scan rows/demands to identify changed projections. Neither implementation choice is a language requirement; both are charged here and remain possible sources of consequential cost. Larger active-demand populations and alternative intermediate stages are not resolved by this fixed-stage pilot.

First observation here is the complete deterministic answer at source quiescence. A separate run advances at most 32 executor service units before disposal; some small sources may already finish. Units differ across executors, so these measurements establish cleanup accounting and are not equal-work cancellation rankings. No OR progress, partial-publication or complete deployment-lifecycle superiority is claimed.

## Disposition and reproducibility

This pilot is complete; T068 remains active for the [low-yield discovery comparison](S01-subscription-low-yield-entry.md). That comparison can change whether retention is economically useful beyond avoiding eager maintenance. Reusable workers and corrected restoration remain the strongest ready alternatives for the next selection review. Broader S01 and the architecture goal remain open.

The [freeze](s01-subscription-lifecycle/freeze.json), [randomized order](s01-subscription-lifecycle/order.json), [all summaries and paired ratios](s01-subscription-lifecycle/summary.json), [phase data](s01-subscription-lifecycle/phases.csv) and per-process receipts preserve all favorable and adverse cells. The [runner](../../../research/chr-compiled/experiments/subscription_lifecycle.py) builds/gates before running, and the [auditor](../../../research/chr-compiled/experiments/summarize_subscription.py) checks full lifecycle restoration and exact diagnostic replay. The reference interpreter is unchanged.

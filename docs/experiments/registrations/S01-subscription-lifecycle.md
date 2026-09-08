# S01 subscription lifecycle pilot — prospective registration

## Decision and hypotheses

Compare active-demand retention with eager data-join retention and competent indexed rediscovery. Subscription retention may avoid inactive combinations while repaying discovery through repeated pulses. Demand retirement, broad updates, consumption or short-lived unique demands may erase that advantage. The current generic compiled Global/Indexed executor is an additional complete-source control. Its single-head specializer admits no region here; broader generated multihead access remains unresolved.

The source and three runtime policies pass the [independent source gate](../results/S01-subscription-source-gate.md). This pilot tests their complete runtime cost, not a universal matching policy or language restriction. Reusable workers and corrected restoration remain ready alternative investigations; this bounded matrix prices the now-integrated subscription mechanism before the next selection checkpoint.

## Exact configurations

Use [subscription_cases.rs](../../../research/chr-compiled/experiments/subscription_cases.rs), the exact three-relation source and [s01_subscription_lifecycle.rs](../../../research/chr-compiled/examples/s01_subscription_lifecycle.rs). Nine families independently combine occurrence fanout N in {2,4} and request rounds R in {2,8}. Four modes: `indexed`, `eager`, `subscribed`, `compiled`. There are 144 cells, each with two changed queries under one prepared ruleset (inert `done(query0)` and `done(query1)` source constraints distinguish them).

| Family | Source construction and driver |
|---|---|
| selective | One group; N distinct connected paths with distinct final values; one demand selects path0 for R pulses |
| dense | One group; N duplicate rows per relation, yielding N³ matching occurrence triples per pulse |
| inactive | Eight disjoint dense groups; only group0 has a live demand for R pulses |
| churn-sparse | Same eight groups; remove/reinsert one middle occurrence in group0 before each pulse |
| churn-broad | Same, but N middle remove/reinsert operations before each pulse |
| reopen | Same eight groups; open, pulse and retire group0's demand each round |
| unique | Same eight groups; open, pulse and retire a different group's demand each round |
| binding | Same eight groups; group0 left endpoints share an unknown; issue an empty pulse, bind it to the matching constructor child, then issue R pulses |
| consuming | One dense group; consume right rows; replenish N right rows before subsequent pulses |

Non-short-lived demands open once and retire after the last pulse. Churn recreates equal-valued rows with fresh occurrence identities. Complete receipt counts per query are R for selective, RN for consuming and RN³ otherwise. Count checks supplement, not replace, independent full-answer validation. Input construction and oracle execution happen outside measured intervals. Query cloning into each executor is inside setup.

## Controls and work predictions

Both data endpoints are indexed; rediscovery starts from the smaller selected endpoint bucket and probes the other by its complete key. Eager retention maintains three-data-row tuples without demand filtering. Subscribed retention maintains those tuples only for active endpoint pairs. It must preserve duplicate demand identity, consuming order, binding updates and blocked residuals.

Separate metrics builds must repeat work diagnostics exactly. At final disposal preparation stage, all source demands are retired. Indexed retains/constructs zero triples. Eager retains N triples for selective, N³ for dense, zero for consuming and 8N³ for the other families. Subscribed retains zero.

Subscribed constructs one tuple for selective; N³ for dense/inactive/binding; RN³ for reopen/unique/consuming; N³ + R·U·N² for churn, U=1 or N. Eager constructs N for selective, N³ for dense, RN³ for consuming, 8N³ for inactive/reopen/unique/binding and 8N³ + R·U·N² for churn. Invalidations equal constructed minus final retained for this monotone insertion/removal accounting. Every changed query repeats these counts. Counter-free diagnostics remain disabled in timing and allocation builds.

## Endpoints and repetitions

Primary endpoint: preparation + both queries' setup, execution, observation, engine disposal and answer disposal + prepared disposal. Preserve all phases and the first query's cold latency through observation. This deterministic source yields its complete answer only at quiescence; the harness does not claim earlier publication of partial residuals.

Run two warmup queries before measured preparation. After the two measured queries, separately measure setup, 32 executor service units and cancellation disposal. The service units differ across engines; cancellation totals are not equal-work performance comparisons and do not enter the primary sum. No OR progress claim is made.

Run five ordinary-allocator counter-free release timing repetitions and two separate requested-allocation repetitions per cell: **1,008 comparative processes**. Randomize all 144 cells within each repetition using seed20260915. Execute serially on the lowest available CPU. Record source hashes, executable hashes, compiler version, platform, CPU affinity, commands and exit status. Do not rebuild frozen binaries during the matrix.

Report paired ratios, medians and all five observed pairs. A practical runtime difference requires at least 20% median difference from 1, with every pair on the same side. These are bounded pilot observations, not confidence intervals. No weighted aggregate or universal winner.

## Bounds, validity and interpretation

Each source run has a two-million-service-step bound. Each gate, work or comparative process has a 60-second wall bound and 1 GiB address-space bound. Build commands have a 120-second bound. Gates are exploratory resource sizing and semantic validation, not comparative timing evidence. A cutoff stops for attribution before registration amendments or comparative continuation; it cannot reject a design by itself.

All complete answers must agree with the independent owned-syntax source evaluator outside measured intervals. Require exact two-run allocation replay, live heap restoration after each query, cancellation and prepared disposal. Requested allocation traffic and peak live heap are not RSS. Charge pending candidate materialization, sorting, binding scans and all retained owners. Source generation, oracle work, native compilation and process startup are excluded; do not claim complete architectural deployment-lifecycle superiority.

Investigate consequential phase dominance, cutoffs and uncertainty. In particular, pulse materialization or whole-script handling may obscure the matching mechanism. Repair a demonstrated defect with a paired registration when it could change the decision; do not silently tune the measured candidate. A favorable subscription result preserves adverse maintenance and lifetime cases. A loss preserves alternative intermediate stages and broader generated access as unresolved. Reassess this investigation against reusable workers and corrected replay before expanding it.

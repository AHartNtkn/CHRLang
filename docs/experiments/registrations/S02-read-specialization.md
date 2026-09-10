# Recorded-read pilot: existing specialization control

Test whether inferred specialized dispatch changes the favorable recorded-read comparison. The existing runner supports this control; the main pilot did not include it. This is a prospectively registered supplementary pilot, not confirmation of an architecture choice.

The source gate checks analytical complete answers and the independent scalar executor, retained answers after producer disposal, changing depth/arrival, and exact specialized applications across split, failed and completed segments. It covers n=8/32, unique/repeated/mixed leaves, depths 1/8 (second query 2/9), and resource absent/present. Primary builds have counters disabled. No executor changes are proposed.

Use the frozen ordinary and allocation binaries from `s02-read-cost/freeze.json`; verify their hashes and the parent source freeze. Retain all answers and use resources. Complete cases, in fixed ID order:

1. near-repeated-8, depth8, one query (small favorable control).
2. near-repeated-32, depth8, one query (main favorable witness).
3. near-repeated-32, depth8, four changing queries (reuse and observed timing variation).
4. near-unique-32, depth8, four changing queries (no useful reuse).
5. near-mixed-32, depth8, four changing queries (failed and successful alternatives).

Modes: sealed, indexed, validated, contextual. Twenty complete configurations each receive two separate metered processes, one ordinary warmup and five ordinary measured repeats. Each repeat block shuffles scenarios and modes using seed86402; retain every sample. Ten additional cancellation configurations use all five families at four queries with sealed/indexed, cancelling alternate queries after one service call; each receives two metered processes and one ordinary semantic replay. Total:190 workload processes, five empty-clock controls and one meter self-check. No cancellation timing ranking.

Pin CPU0. Bound each workload to60 seconds wall/CPU,1GiB address space and the runner's2million service steps; bound the campaign to10minutes. Preserve failed processes and cutoffs. Run semantic and scoped strict Clippy checks before costs. Freeze the driver, gate, registration, parent freeze, binaries, scenario list and order before execution.

Hypotheses: specialized dispatch may remove enough explicit matching work to overturn the apparent ordered-validation advantage; irrelevant/failed contexts may still favor explicit execution; query reuse may amortize specialized preparation differently. Comparisons: sealed/indexed isolates dispatch selection, validated/sealed tests the missing competitor, validated/contextual retains the recomputation contrast. Report each scenario separately.

Charge source construction, preparation, input creation, query setup, execution with owned observation, holding/releasing answers, and engine/prepared/consumer disposal. Validate complete answers outside timing as in the parent runner. Sum intervals; do not call that gross process time or isolated first-observation time. Allocation diagnostics measure requested heap bytes and live requested peaks, not RSS. No compilation or sustained-consumer claim.

Use the parent pilot's clock qualification: every total must exceed100 times the largest empty median times all phase intervals plus one first-answer interval per query. Report medians and full paired ratio ranges. Gain requires all five ratios below0.90; loss all above1.10; otherwise unresolved; unqualified signal remains insufficient. These are one-core exploratory contrasts, not confidence intervals. Do not pool with the earlier cohorts or discard excursions. A consequential unresolved contrast requires attribution or registered confirmation only if it could change the next architectural decision.

After this comparison, integrate the full recorded-read report and select between consequential missing evidence and round A1 demand capability. An adverse cache result does not reject direct integration or other graph organizations.

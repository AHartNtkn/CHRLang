# Recorded-read reuse: a useful regime, substantial miss costs, no general selection

Validating recorded reads saves work when substantive equality repeats. Incompatible contexts still make this cache expensive. The stronger specialization control leaves the favorable runtime comparison unresolved, so these results support a conditional candidate, not a selected architecture.

**The next distinct investigation is demand execution with nonground passive body posts.** Further precision on the favorable cache case remains required if it could select a complete execution path. Direct integration, broader reuse and sustained lifetime are also unanswered; this pilot does not resolve them.

## What was compared

Recorded-read reuse checks the dependencies of a prior equality deduction before replaying it. Its causal control rebuilds an owned relevant-state key before recognizing reuse. Other controls recompute equality, cache exact states, or execute the source through compiled scanning/indexing. Both ordered and persistent cache representations are included. Existing source lowering runs only on the previously qualified single/changed families.

The near-miss sources compare the same nested inputs in many alternatives. Repeated leaves permit reuse; distinct leaves force unsuccessful recognition; mixed leaves add failing alternatives. Branch tags and token consumption remain observable. Rules are prepared once, queries change depth and arrival order, and answers remain alive through engine and prepared-rule disposal. Caches are query-local: this is not cross-query deduction reuse.

The [main registration](../registrations/S02-read-cost.md) fixes 28 complete scenarios and seven cancellation scenarios, with 320 configurations. The [supplement](../registrations/S02-read-specialization.md) tests the existing inferred-specialization control on five consequential scenarios, including the favorable witness and adverse contexts.

## What changes the decision

**Avoiding key construction produces a real allocation saving on repeated work.** At 32 repeated alternatives, depth 8 and four changing queries, ordered validation requests 1,482,714 bytes versus 3,818,130 for rebuilding relevant keys. Key construction falls from 1,472 scopes and 2,565,120 bytes to 294 scopes and 229,704 bytes. Both replay 1,178 deductions with 156,240 requested replay bytes. This isolates the saving in recognition rather than attributing all reduced traffic to less replay.

**That saving does not make caching economical on incompatible contexts.** The corresponding unique-leaf case has no replay. Ordered validation still requests 5,343,358 bytes versus 2,378,878 for recomputation. Its peak live excess is 629,821 versus 191,008 requested bytes. Checking records without allocating does not make the checks free in time.

**Persistent lookup adds a consequential implementation cost, but that cost alone cannot reverse the adverse allocation result.** In the unique-leaf case, validation lookup requests 961,248 bytes in 37,696 allocations; ordered lookup requests none. Persistent validation requests 8,147,374 bytes overall. Even subtracting all of that lookup traffic leaves 7,186,126 bytes, above recomputation's 2,378,878. This is a bound on requested traffic only, not an achievable speedup or an argument against other cache representations.

**The strongest-control check matters to the favorable inference.** The main pilot omitted `sealed`, the existing source-inferred specialized dispatch. Inspection and a new source gate establish that it applies to `start` and, when present, `take`. Exact work counts include split, failed and completed segments: one specialized start application plus one take per successful alternative. An initial terminal-only diagnostic check missed the pre-split application; the corrected gate follows the engine's segment ownership contract. No engine change was needed.

The following are complete measured-interval medians from the supplementary pilot. All sources use depth 8, with later queries alternating to 9. “Validation” means ordered recorded-read reuse. All five paired ratios must be below 0.90 to register a pilot gain, or above 1.10 for a loss; otherwise the comparison is unresolved.

| Source | Validation | Specialized explicit | Recompute equality | Validation versus specialization |
|---|---:|---:|---:|---|
| 8 repeated alternatives, one query | 0.144 ms | 0.122 ms | 0.150 ms | Unresolved; ratios 0.847–1.249 |
| 32 repeated alternatives, one query | 0.300 ms | 0.396 ms | 0.482 ms | Unresolved; ratios 0.668–0.970 |
| 32 repeated alternatives, four queries | 1.019 ms | 1.071 ms | 1.537 ms | Unresolved; ratios 0.798–0.971 |
| 32 unique alternatives, four queries | 3.664 ms | 1.357 ms | 1.535 ms | Loss; ratios 2.520–3.066 |
| 32 mixed alternatives, four queries | 3.931 ms | 1.361 ms | 1.552 ms | Loss; ratios 2.655–3.058 |

Validation has registered gains against recomputation in both large repeated scenarios and losses in both adverse scenarios. The specialized-versus-Indexed timing contrast is unresolved in all five scenarios. Thus specialization is an eligible, exercised competitor; this supplement does not claim that its preparation and dispatch always pay.

**Memory presents a separate tradeoff even in the favorable case.** At 32 repeated alternatives and one query, validation requests 426,312 bytes with 163,391 peak excess, versus specialization's 558,373/391,632. Recomputation requests 628,454 bytes but has the lowest peak of these three, 154,418. Compared with rebuilding owned keys, validation leaves peaks unchanged in 24 of 28 complete ordered-cache scenarios and all 28 persistent-cache scenarios; four ordered cases have lower peaks. Less allocation traffic is not the same as a smaller live cache.

## How much confidence these runs support

The final main cohort has 2,730 workload processes: 640 allocation diagnostics, 490 attribution diagnostics, 256 warmups and 1,344 ordinary runs, including cancellation replays. All 320 allocation pairs and 245 profile pairs repeat exactly; all 245 profile/meter comparisons agree on normalized phase ownership. Of 256 complete configurations, 253 pass the registered clock-signal threshold. The 288 exploratory contrasts contain 35 gains, 122 losses, 125 unresolved results and six insufficient-signal results. Counts describe this matrix; they are not workload weights.

Two earlier complete cohorts are preserved. Clippy findings prompted small fixture return-expression and clock-guard corrections; the final cohort ran after qualification passed. The audit verifies their frozen sources/binaries and identical normalized allocations, endpoints and profiles across all three cohorts. Their timing dispositions differ. They are neither pooled nor presented as independent statistical confirmation.

Timing excursions remain in the evidence. In the initial repeated32/four-query validation run, one total reached 3.834 ms versus1.173–1.345 ms in its other repeats. The extra time lies mainly in two execution/observation intervals. No CPU-time or operating-system telemetry in this pilot identifies its cause. The [preserved interval breakdown](s02-read-cost-initial/repeated-excursion.json) supports attribution to the interval, not an explanation of why it slowed.

The supplement adds 190 workload processes, 30 exact allocation pairs and five clock controls. All 15 matched existing-control allocation results equal the parent pilot. Its complete configurations pass clock qualification. Independent scalar and analytical answers, retained ownership, cancellation/reuse, exact specialized work and scoped Clippy checks pass. No outlier is excluded.

The favorable supplemental comparison also varies within execution/observation: validation takes 239,746–322,216 ns and specialization 293,817–343,828 ns in that interval. The [phase breakdown](s02-read-specialization/favorable-phase-breakdown.json) shows that preparation alone does not account for the unresolved ratio. A future confirmation should distinguish execution-time variation from elapsed scheduling effects rather than simply repeat until a threshold is crossed.

These are one-core, five-repeat exploratory timing comparisons. Primary runs use ordinary allocation with engine/kernel counters disabled; requested allocations and attribution use separate builds. Totals include source creation, preparation, changing inputs, setup, execution with owned observation, answer handling and disposal. They are sums of measured intervals, not gross process times. Complete-answer validation and harness storage are outside those intervals. Execution and owned observation remain combined; first-answer latency is not an isolated observation cost.

Compilation, RSS, immediate/window consumers, sustained histories and whole-architecture superiority are not established. Requested heap bytes and live requested peaks must not be relabelled RSS. A favorable finite case does not establish a generally sound cheap eligibility policy.

## Dispositions and the next experiment

| Question | Evidence-backed consequence | Remaining investigation |
|---|---|---|
| Can recorded reads avoid expensive key rebuilding? | Yes, on qualified substantive repetition, with exact allocation attribution. | Broader dependencies, cache validity and lifetime. |
| Does the cache win on incompatible contexts? | These implementations lose the supplementary total-cost comparison and incur larger ownership costs. | Other recognition/representation mechanisms require their own trial; this is no general rejection. |
| Does it outperform the strongest eligible explicit path? | The new specialized comparison is unresolved in favorable cases and adverse in unique/mixed cases. | Prospectively registered confirmation/attribution when choosing a complete path; retain both candidates meanwhile. |
| Should persistent lookup be tuned next? | Its extra allocation is real, but eliminating it alone cannot reverse the tested adverse traffic contrast. | Runtime or broader-regime value must justify a repair package; the traffic bound does not settle those questions. |
| Does this settle integrated execution? | No. Reuse still retains deduction recognition, dependency validation and replay machinery. | Direct consuming integration and local rewrites must show which responsibilities actually disappear. |

**Demand capability is the next package because it can change which complete sources a competing executor supports.** Its qualified dependency machinery and independent controls make nonground passive body posts a concrete entry. A further cache timing repeat could narrow the favorable conditional tradeoff, but would not establish broader source capability or eliminate a service boundary. Keep that timing question open and reconsider it at the demand capability/cost boundary, or earlier if an actual integration choice requires it. Do not infer equality from overlap or adopt a cache policy from the medians.

T072 remains unfinished for distinct integration, relevant lifetime and consequential confirmation. T071 becomes the sole active task under the [sequence](../next-cycle.md). The full research goal remains active.

## Reproduction evidence

- Main [registration](../registrations/S02-read-cost.md), [driver](../../../research/chr-relational/experiments/read_cost.py), [audit implementation](../../../research/chr-relational/experiments/audit_read_cost.py) and [reconstructed results](s02-read-cost/review-audit.json).
- Preserved [initial audit](s02-read-cost-initial/review-audit.json) and [second audit](s02-read-cost-second/review-audit.json), with exact source snapshots in their respective directories.
- Supplementary [registration](../registrations/S02-read-specialization.md), [source gate](../../../research/chr-relational/tests/read_specialization.rs), [driver](../../../research/chr-relational/experiments/read_specialization.py), [audit implementation](../../../research/chr-relational/experiments/audit_read_specialization.py) and [results](s02-read-specialization/audit.json).

Each run directory retains its raw process receipts, scenario/order manifest, binary/source freeze and validation logs. Frozen binary paths are local artifacts; their hashes bind the measurements to the recorded builds.

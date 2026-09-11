# Compact union buys a modest overlap benefit at extra preparation cost

Union is faster on repeated overlapping full output under this run series, but its advantage over direct single traversal lies near 10%. The direct traversal keeps its allocation advantage, performs similarly on single-alternative output and is faster on redundant relations. This supports a conditional tradeoff, not a universal compact or explicit architecture.

The [registered confirmation](../registrations/S06-unique-confirmation.md) completes 8,192 primary processes and 128 excluded warmups using the unchanged qualified binary. Every process checks complete outputs against its independent finite oracle, early stopping and prepared reuse, and retained outputs after prepared disposal. The [audit](s06-unique-confirmation/audit.json) reconstructs every configuration and phase.

## A stronger direct competitor changes the scale of the benefit

Union prepares a decision graph; the `unique` control traverses coordinate assignments once and prunes a prefix when no source alternative remains viable. Both produce the same finite logical set without duplicate discovery/output. Deduplicated branchwise traversal and reduced disjunction remain separate controls.

On repeated overlap, union strongly outperforms those branchwise controls, but only modestly outperforms unique traversal. With set collection, union/control median-ratio intervals are 0.503–0.538 against deduplicated traversal and 0.509–0.544 against reduced disjunction. With ordered collection they are 0.272–0.295 and 0.270–0.285. Those large gains cannot stand in for the much smaller advantage over unique traversal.

The table uses width eight, 64 changing queries, complete output and retained-all consumers. Ratios divide union by unique time. Median times and paired ratios are distinct summaries.

| Relation / collector | Union / unique median time (ms) | Interval for paired median ratio | Interpretation |
|---|---:|---:|---|
| Overlap / set | 6.178 / 6.809 | 0.866–0.924 | Union faster; 10% magnitude unresolved |
| Overlap / ordered | 3.106 / 3.530 | 0.846–0.932 | Union faster; 10% magnitude unresolved |
| Disjoint / set | 0.997 / 0.922 | 1.036–1.112 | Union slower; 10% magnitude unresolved |
| Disjoint / ordered | 0.578 / 0.508 | 1.102–1.175 | Union over 10% slower |
| Redundant / set | 1.494 / 0.908 | 1.579–1.668 | Union over 10% slower |
| Redundant / ordered | 1.104 / 0.549 | 1.943–2.065 | Union over 10% slower |
| Single / set | 0.905 / 0.906 | 0.975–1.038 | Within ±10% |
| Single / ordered | 0.552 / 0.549 | 0.962–1.047 | Within ±10% |

**The overlap result establishes a direction while leaving the registered practical threshold unresolved.** Both intervals lie below one but cross 0.90. It would be incorrect to turn “10% not established” into “no benefit.” Whether this magnitude earns a prepared logical region depends on its preparation, ownership and language boundary in an actual composed architecture.

**The memory cost does not disappear with the speed gain.** The unchanged [parent allocation matrix](S06-unique-output.md) gives union and unique identical execution allocation and retained output in all 96 matched scenarios, but higher union preparation traffic and peak. For overlapping 64-query set output, union requests 3,801,864 bytes versus 3,277,712; peaks are 3,296,120 versus 3,271,952. Ordered output reduces retained memory but raises requested traffic: 5,080,856 versus 4,556,704 bytes. These are requested heap measures, not RSS. More same-source reuse cannot amortize away this implementation's fixed traffic difference without a mechanism change.

## Small timing signals and sampling uncertainty remain explicit

All four families, one/64 queries, membership/full output and both collectors remain in the matrix. There are 32 scenarios, four modes and 96 predeclared union/control contrasts. The registered classification finds four practical gains, 32 losses, seven intervals within ±10%, five unresolved magnitudes and 48 instrumentation-sensitive contrasts. Against unique specifically: zero established 10% gains, 11 losses, two within ±10%, three unresolved magnitudes and 16 instrumentation-sensitive contrasts. These are unweighted coverage counts.

All 48 withheld contrasts are membership comparisons. Fifty-two individual cells have at least one total below the registered signal floor. Clock medians are 12 ns in all five calibrations; the floor is 100 times that cost times 4q+5 phase intervals. Every sample is retained and the threshold is unchanged. Membership timing therefore still needs a suitably qualified measurement design if it can change a concrete architectural choice; this confirmation does not answer it indirectly through full output.

The fixed 64 repetitions use randomized matched blocks. Simultaneous median intervals use sorted ratios 18 and 47 across all 96 contrasts, with a family error bound below 5% under independent repetitions from a stable distribution. Individual ratios vary substantially: ordered overlap ranges from 0.497 to 1.360, and ordered redundancy from 0.983 to 8.816. None is excluded. Host interference or temporal dependence can invalidate the sampling assumptions; the intervals are conditional machine/run-series evidence, not hardware or program generalization.

## What was qualified and charged

The structural crate and measured executable have metrics, allocation metering and extra clock features disabled. Cargo also built an unused `chr-observe` development dependency with metrics. Archived manifests/modules and [binary symbols](s06-unique-confirmation/symbols.txt) establish that it is outside this measured path; the [qualification receipt](s06-unique-confirmation/qualification.json) records both the initial broad feature-check failure and its resolution. The parent allocation audit reproduces as the same JSON structure; map-printing order accounts for the byte difference.

Each primary process performs the unchanged independent preflight before fresh charged preparation. Source construction, preparation, source disposal, request creation, combined execution/owned observation, request disposal, consumer retention and all final disposal count. First-delivery latency, oracle/preflight work, startup, fixed recording containers, native compilation and artifacts do not. Timing covers retained-all consumers; the parent allocation results for releasing/window consumers are not timing evidence for those consumers.

The [freeze](s06-unique-confirmation/freeze.json), [primary receipts](s06-unique-confirmation/runs.jsonl.gz), [warmups](s06-unique-confirmation/warmup.jsonl.gz), [campaign](s06-unique-confirmation/campaign.json) and [auditor](../../../research/chr-structural/experiments/audit_unique_confirmation.py) preserve source/binary identities, exact orders and every status. The campaign completes in 72.1 seconds without a cutoff. No executor or reference implementation changes.

The [next-selection review](S06-unique-confirmation-review.md) selects call-reuse extraction and runtime costs under T075. Compact membership signal, near-threshold magnitude, broader relations, connected projection, richer theories and complete-path integration remain unanswered where consequential. The research goal remains active.

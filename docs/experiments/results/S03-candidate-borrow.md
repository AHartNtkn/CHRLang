# Borrowing candidate bindings removes a real demand-execution cost

Delaying candidate copies makes the large unsuccessful-search case about 26–32% faster in the targeted confirmation. It removes 9.93 MB of requested allocation in one four-query session. The repaired executor still trails generated indexed execution on that case, so this is a useful implementation repair, not an architectural winner.

## What changed, and why it is sound

The demand executor now reads existing resource arguments directly and borrows the current variable bindings while testing a candidate. It copies the bindings only when a match introduces a variable or a successful candidate needs its own recursive environment. A failed candidate cannot change the bindings used by subsequent candidates. Candidate visitation, normalization, consumption order and selected-resource identity are unchanged.

The semantic tests exercise a mismatch after introducing a binding, a failure at a later partner, a later successful candidate, choice suspension, late aliases and retained answers. Independent scalar and compiled observations agree with the demand variants. All 288 diagnostic work rows repeat exactly and match the preceding implementation. The reference interpreter is unchanged. [Work comparison](s03-candidate-borrow/work-bridge.json), [implementation](../../../research/chr-direct-choice/src/demand.rs), [source tests](../../../research/chr-direct-conditional/tests/nonground_posts.rs).

## The large miss saving is attributable to the repaired copies

For size 32, original arrival order, four changing queries and all answers retained, the miss-reuse policy changes as follows:

| Complete-session measure | Before | After |
|---|---:|---:|
| Pilot median elapsed time | 6.28 ms | 4.55 ms |
| Requested allocation | 12,259,743 bytes | 2,328,991 bytes |
| Argument-copy allocation | 763,904 bytes | 0 |
| Candidate-environment-copy allocation | 9,166,848 bytes | 0 |

The two copy scopes account for the entire 9,930,752-byte reduction. There were 47,744 calls in each scope before the repair. This source consists of read-only unsuccessful candidates, so neither scope needs an owned copy afterward. Across all 72 profiled cells, the complete allocation difference equals the difference in the three measured copy scopes. This does not attribute all runtime to allocation or remove other normalization and forcing copies.

Generated Indexed takes a pilot median 1.08 ms on the same source; activation with generated scanning takes 1.82 ms. Avoiding copies therefore narrows a substantial gap without resolving it. On other sources demand remains credible: the repaired miss-reuse policy takes 0.94 ms on size-32 output posts, against generated Indexed's 1.34 ms. These five-sample comparisons with explicit controls are exploratory, not confirmed architecture rankings.

## Confirmation includes the apparent regressions

The pilot contains 216 before/after contrasts. Thirty-seven median ratios are below 0.90, two above 1.10, and 177 between those thresholds. These are unweighted counts of exploratory medians. The two apparent regressions occur on duplicate resources; their sample ratios vary substantially.

A separately registered confirmation tests both apparent regressions, the large miss gain, a small forwarding case and an output case. Each is tested with all three demand policies: 15 contrasts with 64 paired repetitions each. The table gives the envelope of the three individual median intervals, not a pooled interval.

| Scenario | New/old median-interval envelope | Registered outcome for the three policies |
|---|---:|---|
| Duplicate, size 8, original arrival, window retention | 0.918–1.006 | All within ±10% |
| Duplicate, size 32, reversed arrival, immediate release | 0.954–1.051 | All within ±10% |
| Miss, size 32, original arrival, retain all | 0.678–0.741 | All practical gains |
| Forward, size 8, original arrival, retain all | 0.914–1.006 | All within ±10% |
| Output, size 32, reversed arrival, retain all | 0.942–1.060 | All within ±10% |

The apparent regressions do not reproduce as practical regressions under this confirmation. Their original samples remain in the evidence; no machine-level cause is established. Some intervals within the practical band are directionally below one. The result does not establish equivalence outside the selected cases.

Intervals use the 20th and 45th ordered paired ratios out of 64. Under independent samples from a stable distribution, the simultaneous error bound for all 15 is at most 0.02345. Independence and stability are assumptions, not measured guarantees. First/last-half medians and every ratio are retained in the [confirmation audit](s03-candidate-borrow/confirmation/audit.json). These selected confirmation cases are not held-out generalization evidence.

## Measurement and validation

The [pilot registration](../registrations/S03-candidate-borrow.md) fixes 8,136 lifecycle processes: 5,400 primary timings, 1,080 excluded warmups, 1,080 allocation runs, 144 profiles and 432 cancellation checks. The [confirmation registration](../registrations/S03-candidate-borrow-confirmation.md) adds 1,920 primary processes and 30 excluded warmups. All complete successfully. Six clock calibrations establish a 36 µs signal floor; no before/after comparison is signal-limited.

The pilot crosses six post families, sizes 8/32, two arrival orders and immediate/window/all consumers. It compares the three demand policies before and after with nine existing generic/generated, activation and scanned/indexed controls. Ordinary timing disables engine/kernel/work counters and uses the ordinary allocator. Allocation and copy profiles use separate builds. All 216 new allocation pairs and 72 profile pairs repeat exactly; all 648 explicit-control allocation cells match the earlier frozen control. Cancellation and final owner restoration pass.

Each session reuses prepared rules across four changed queries. The measured sum charges source construction, preparation, query input/setup, execution with owned observation, consumer operations and engine/prepared/answer disposal in 24 disjoint phases. Complete-answer validation is outside the intervals. Requested heap traffic and peak are not RSS. Compilation, process startup, validation and sustained service remain outside this result; execution and observation do not supply a separate first-answer latency.

Frozen old and new binaries, source snapshots, feature checks, raw runs and reconstructed results are retained in the [pilot evidence](s03-candidate-borrow/freeze.json) and [confirmation evidence](s03-candidate-borrow/confirmation/freeze.json). [Pilot audit](s03-candidate-borrow/audit.json), [confirmation audit log](s03-candidate-borrow/confirmation/audit.log). Unit tests, independent source tests, repeated diagnostic work checks and strict Clippy pass; test and build logs accompany the evidence.

## Consequence for architecture selection

Keep the repair and carry demand into complete-path comparisons. Its successful regimes survive a concrete cost correction, while unsuccessful discovery remains a large liability against strong controls. Candidate visits, normalization, writable-head dependencies, dynamic choices and sustained ownership remain separate unanswered questions.

The [full portfolio review](S03-candidate-borrow-review.md) selects mixed-source composition next. This package and its confirmation complete the four-package review interval. They do not complete the research goal.

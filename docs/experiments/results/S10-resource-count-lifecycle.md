# Source counting repays its costs on deeper work, but adds shallow-query overhead

Counting substantially reduces complete measured allocation on eligible depth-16 sources, including the cost of inference and query transformation. It brings the common-work organizations much closer together. At zero or shallow depth, preparation can outweigh the removed execution, so eligibility alone does not justify always applying it.

The [registered pilot](../registrations/S10-resource-count-lifecycle.md) completes 7056 isolated processes: 2016 allocation runs with 1008 exact pairs, followed by 5040 ordinary-allocator timing runs. All complete observations, admission counts, phase order and query/certificate/prepared ownership checks pass. [Independent audit](s10-resource-count-lifecycle/audit.log), [all cells](s10-resource-count-lifecycle/summary.csv), [frozen sources and binaries](s10-resource-count-lifecycle/freeze.json), [toolchain](s10-resource-count-lifecycle/toolchain.txt).

## Full costs with eligible common work

The table shows three choices, depth 16, four changed queries per prepared source and one cancellation probe. Figures are total requested allocation bytes, including inference, transformation, preparation, inputs, delivery and disposal. Peak means requested live memory above the baseline, not RSS.

| Execution | Original traffic | Counted traffic | Original peak | Counted peak |
|---|---:|---:|---:|---:|
| Scan | 1240946 | 403610 | 93976 | 54017 |
| Indexed | 1708174 | 486326 | 124877 | 68174 |
| Specialized Scan | 1228906 | 403346 | 91590 | 54575 |
| Contextual eager | 4456874 | 425610 | 72476 | 32734 |
| Contextual demand | 2474098 | 424786 | 36076 | 32662 |
| Persistent shared contextual | 4505978 | 469002 | 72638 | 38792 |
| Conditional | 2546543 | 465235 | 54964 | 24385 |

This is a stronger control than saving repeated interpreter applications. The common traversal disappears, leaving preparation, choices and complete observation. Conditional retains less peak memory in this cell while Scan allocates less traffic; a universal ordering does not follow.

For Scan, inference allocates 2908 bytes; the four query transformations allocate 15504 and cancellation transformation 3876. Main query setup drops from 59832 to 18072 bytes, and first-delivery work from 1121136 to 313712. Source rules are unchanged, so prepared-rule allocation stays 9437 bytes. The transformation's query clone and certificate remain charged rather than hidden outside the lifecycle.

Branch-specific work remains consequential. On the independent family at the same dimensions, counted traffic is 518179 bytes for Scan, 649262 for eager contextual, 644982 for demand and 2505803 for conditional. Counting removes the common depth but leaves the distinct suffixes and their resource/equality/search obligations. These results retain a reason to investigate conditional discovery rather than treating it as resolved.

## Shallow and rejected queries provide contrary evidence

With no choices, zero depth and one query, Scan traffic rises from 24283 to 28315 bytes after counting. Conditional rises from 55207 to 59239. With depth one, Scan still rises from 27291 to 29313, while conditional falls from 68170 to 60237. The crossover depends on the executor's avoided work, not simply whether the certificate succeeds.

Mixed schedules alternate eligible queries with queries lacking a permit. The latter must be rejected by the optimizer and execute the original rules. Independently constructed answers include suspended traversal terms, untouched fuel, unbound results and the absence of permit-history markers. Rejected queries reuse the same prepared source; the runner charges attempted certification. Cancellation in the mixed schedule also uses a rejected query.

At three choices/depth 16/reuse four, mixed Scan traffic changes from 880914 to 459251 bytes, and conditional from 1644013 to 610616. These are mixed-batch totals, not an isolated saving on rejected queries. The complete table includes reuse one and four; admission counts are independently audited for every process.

## Timing and validation limits

Five ordinary samples per cell are exploratory. For eligible common work at the substantive dimensions, Scan median phase sum changes from 1198295 ns (1032268–1776200) to 267556 (252825–319625); conditional from 1913218 (1781244–2302955) to 369157 (353759–606677). Counted eager contextual has median 191442 (190345–235278). These are sampled results, not formal practical-win classifications or a weighted architectural ranking.

Each process validates warm and measured observations against independently constructed expectations, and checks the scalar against those expectations outside intervals. The [source composition tests](s10-resource-count-validation/source-tests.log) pass again. Strict Clippy passes for [allocation](s10-resource-count-validation/clippy-meter.log) and [ordinary timing](s10-resource-count-validation/clippy-time.log). [Runner](../../../research/chr-direct-conditional/examples/resource_count_cost.rs), [driver](../../../research/chr-direct-conditional/experiments/resource_count_lifecycle.py), [auditor](../../../research/chr-direct-conditional/experiments/audit_resource_count_lifecycle.py).

Execution and observation remain coupled in delivery. Native compilation, startup, original source AST construction, expected/scalar checks and preallocated bookkeeping are excluded. All source definitions satisfy inference; mixed rejection tests query conditions, not invalid source declarations. Answers are released per completed query; sustained retention is not measured. The [source certificate's boundaries](S10-resource-count-gate.md) remain in force.

## Breadth review and next investigation

Recent S10 packages have qualified complete mixed sources, measured their lifecycle, repaired discovery/export costs, demonstrated real post-choice sharing, and tested a source-derived elimination control. This supplies bounded architectural evidence, not closure of S10 or earlier stages. Another small timing refinement would not resolve the broader language and responsibility questions.

Select T079 under S07: an executable comparison of inferred resource privacy/groundness, checked optional declarations and mandatory restrictions. The current evidence makes the tradeoff concrete: source properties permit a large execution reduction, but observer/scheduling counterexamples and rejected queries constrain applicability. Determine what each language option can express, what checking and boundary work remains, and whether any runtime responsibility actually disappears. No restriction or default is selected now.

The strongest ready alternative is selective conditional discovery, followed by resumable contextual matching. Those can change costs on branch-specific and ineligible sources and remain required. The S07 comparison goes first because it can affect accepted programs, compiler/runtime boundaries and the necessity of generic execution across organizations, rather than refine one implementation's cost. Bound the first comparison to privacy and ground entry modes, then reconsider these alternatives after its first executable gate.

Other required directions retain their owners: broader restoration/reunion, direct graph mechanisms, integrated matching/equality, reuse/learning, richer structural theories, native compilation, sustained observation and connected parallelism. The final comparison still requires stronger contrary sources, necessary-complexity accounting and held-out challenges. Lower priority changes order, not resolution. The goal remains active.

# Transfer preparation data to the owner that needs it

The query initializer does not need transformed rules after engine preparation. Transferring those rules to their next owner lowers retained preparation heap and total peak in every one of144 tested comparisons. This is avoidable implementation cost, not a necessary property of static initialization or any executor.

The initializer now separates the transformed rules from the posting plan. Explicit and conditional preparation take the rules directly; demand lowering reads them and releases them when lowering finishes. The posting plan remains reusable across queries. The existing retained-owner runner is the experimental control; the detached runner is the path for subsequent lifetime work.

## What changed in measured costs

All figures are requested heap bytes, not RSS. Ranges cover the registered source, consumer and cancellation cases; they are not workload weights.

| Path | Reduction in total allocation traffic | Reduction in preparation retained heap | Reduction in total peak |
|---|---:|---:|---:|
| Demand, miss reuse | 0 | 4,202–10,103 | 1,422–10,103 |
| Conditional | 3,818–9,719 | 4,202–10,103 | 4,202–10,103 |
| Generated GlobalScan | 3,818–9,719 | 4,202–10,103 | 3,818–10,103 |

Demand still builds its lowered rules, so total traffic is unchanged. Its unnecessary source owner dies earlier. The other paths also avoid cloning the transformed rules. No engine algorithm, answer contract or reference implementation changes.

Across all144 comparisons, preparation retention and peak are lower; traffic is lower in96 and equal in48. No measured allocation metric increases. The result corrects how the matched-initialization memory evidence should guide architecture selection: part of that measured overhead is avoidable. It does not establish the cost of initialization versus no initialization after this repair; carry both opportunities into the sustained comparison.

## Evidence and controls

The [prospective registration](../registrations/S08-preparation-ownership.md) fixes four shapes, three executors, history on/off, immediate/window/all consumers, cancellation on/off and two repeats of each implementation. The campaign completes576 isolated processes with288 exact allocation pairs. Each runner checks complete answers against independent source observations outside its measured phases, checks cancelled prefixes, reuses preparation after cancellation and checks retained answers after producer disposal. Every process returns task-owned heap to its starting level.

A separate source gate covers216 configurations and5,400 complete executions, including counted/original queries, generic/generated access policies and conditional execution. Static multiplicity and interfering-source rejection tests also pass. The missing ownership-transfer operation produces the recorded RED failure; the implemented operation passes the gate. Clippy passes after resolving a duplicated dead-code annotation shared with build generation. That annotation-only build-script edit follows the measurement freeze and changes no generated algorithm. The archive preserves the measured sources.

[Raw receipts and audit](s08-preparation-ownership/audit.json), [source/binary freeze](s08-preparation-ownership/freeze.json), [campaign](s08-preparation-ownership/campaign.json) and [source archive](s08-preparation-ownership/sources.zip) are durable. The audit verifies576 records,36 phases per run, complete noncancelled endpoints, ownership restoration and exact repeats. Metered elapsed samples are retained but support no runtime claim.

## Next decision and why it comes next

Keep T074 active for the registered preparation-reuse and sustained-session comparison. This first package after the portfolio review establishes that source retention was repairable. The next package should extend actual changing queries and ownership trajectories, comparing repeated preparation with reuse and measuring ordinary timing and RSS separately. Four alternating queries here qualify the repair; they do not substitute for that experiment. Continuing production with surviving active state remains a distinct required contrast.

Broader integration could change which boundaries exist at all; coarser recognition could change whether reuse repays lookup and transport. Both remain stronger alternatives than further local allocation tuning. One sustained-session cost package comes first because these now-corrected complete paths are ready and their preparation/memory ordering can change directly with lifetime. Reconsider those alternatives at that result; full portfolio review is due within three further packages. No architecture or language restriction is selected.

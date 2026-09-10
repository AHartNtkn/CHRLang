# Later failure checks preserve common execution

Checking refined states avoids the common-work duplication caused by eager failed-region subtraction. It adds repeated region checks, so this result supports measuring both policies' total costs; it does not establish a speed or memory advantage.

## What changed in the comparison

The learner still retains only completed failed input regions for one immutable prepared source. The new policy lets ordinary source execution refine a query, then discards a pending state only when its remaining input possibilities lie entirely inside a known failed region. It creates no additional partitions. Eager subtraction remains the competing policy, alongside ordinary recomputation and an exact-query result cache.

The controlled successful query accepts pairs containing c. A failed seed covers a/b on both arguments; the later query widens both to a/b/c. Varying the deterministic prefix isolates common work from the unchanged five distinct answers.

| Prefix depth | Recompute: source steps | Eager subtraction: source steps | Later checks: source steps | Later region probes |
|---|---:|---:|---:|---:|
| 0 | 10 | 9 | 9 | 10 |
| 1 | 11 | 11 | 10 | 11 |
| 4 | 14 | 17 | 13 | 14 |
| 16 | 26 | 41 | 25 | 26 |

Both duplicate-choice weights reproduce these counts, with raw answer multiplicities of five and twenty. Later checks preserve the prefix once and discard one doomed state. They save one source step against recomputation while checking each pending state; that recognition work can still outweigh the saving.

## Broader validity and work evidence

Each feature build covers 96 seeds and 1,920 changed-query sessions across accepted relations, domain supports, alias topology, renaming, duplicate weights and cache capacity. Later checks reduce source steps in 298 sessions, leave 1,622 unchanged and increase none. Of the reductions, 32 are successful queries. The follow-ups perform 2,780 region probes and discard 298 states. These are separate work counts, not interchangeable units of cost.

Complete raw answers agree with the independent scalar execution and compiled Scan through the full consuming caller. Analytical pair enumeration independently checks answer counts and weights. Eager-policy matrix and common-prefix results exactly match the previous frozen evidence. Default and metrics-off builds have identical recorded work signatures. Each build passes 32 relevant tests; strict scoped Clippy and formatting pass.

Additional witnesses cover late variable equality, constructor-shaped private calls, changed private goals, cancellation after a covered state is discarded, and a traversal-budget error after successful preparation. Cancellation uses a distinct narrower region so an erroneous early installation would be observable. A traversal error discards unpublished work and cannot install a failure; the same learner subsequently answers a valid wider query correctly.

## Why the pruning is valid within this fragment

The key remains the normalized ordered initial private goals, including alias topology, under the same immutable source owner. It never uses the changing continuation as a substitute key. Each initial private variable is resolved through current bindings to either an atom or a remaining finite domain. If every support is contained in the corresponding known failed support, every completion of the state belonged to a failed initial assignment.

Late aliases can correlate those supports. Testing containment of their Cartesian overapproximation remains sound: it may miss a pruning opportunity but cannot admit a new assignment into the failed region. New query weights remain with ordinary solving. The existing closed-private-phase contract prevents caller changes from affecting the learned failure before phase completion. Errors and cancellation do not prove failure. Binding traversal consumes the existing term budget, and one discarded state produces one progress event.

This mechanism does not extract minimal conflicts from failed branches inside successful queries. It does not establish broader consuming-resource derivations, general learning, independent native compilation or sustained memory behavior. Source steps omit key construction, region lookup and other administration. Capacity bounds entry count rather than retained bytes.

## Decision and next experiment

Proceed to allocation qualification and paired total costs, retaining both pruning policies and recomputation. The strongest ready alternative is T078's coherent Rust/native path comparison. Learning has now passed the paired mechanism gate; further source-step tuning alone is less informative than measuring whether recognition and retention repay avoided work. T078 accounting supports that measurement and remains required for the mixed-source architecture pilot.

Register timing only after allocation ownership and exploratory sizing are checked. Vary one-shot versus changing-query reuse, common-prefix depth, cache capacity, selective failure and unselective successful work. Charge preparation, keys, checks, retention, eviction, observation and disposal. Report compilation as unmeasured until independently isolated. Return to direct resource derivations, native compilation and the distinct mechanisms in the [sequence](../next-cycle.md) at the stated package boundary.

## Evidence

- [Prospective semantic/work registration](../registrations/S06-covered-learning.md)
- [Metrics-off raw gate](s06-covered-learning/gate-off.log), [default raw gate](s06-covered-learning/gate-default.log), [Clippy](s06-covered-learning/clippy.log)
- [Audit results and source hashes](s06-covered-learning/audit.json), [audit program](../../../research/chr-hvm/learned_regions/covered_audit.py)
- [Previous eager-region evidence](S06-learned-regions.md)

This package is architectural evidence about a bounded learning mechanism. The research goal remains active.

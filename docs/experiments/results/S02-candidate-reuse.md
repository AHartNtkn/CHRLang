# Candidate reuse removes most broad-join overhead, without settling integration

Retaining ordered candidate lists materially improves the relational executor on flat and dense joins. The correction preserves independently checked source behavior and has no practically consequential regression in the five registered families. It remains slower than the integrated control on broad and delayed matching, while retaining its immediate selective-constructor advantage.

All 175 paired processes completed with correct answers: 125 primary timing runs and 50 separate allocation runs. The corrected controls and original relational binary were measured in fresh randomized blocks. The runner and fixtures are byte-identical to the first pilot. All 25 allocation cells replay exactly after excluding time and comparing requested traffic, deallocations, net live changes and peak increments.

## Paired correction results

Ratios below divide corrected relational lifecycle by the original relational lifecycle within the same repetition. Lower values favor reuse. Both versions include preparation, 16 changing queries, observation and disposal.

| Family | Median ratio | Five-repetition range | Requested allocation, original → corrected |
|---|---:|---:|---:|
| Flat joins | 0.342 | 0.326–0.395 | 6.371 → 1.525 MiB |
| Immediate dense constructors | 0.316 | 0.313–0.365 | 8.490 → 2.130 MiB |
| Delayed dense constructors | 0.548 | 0.494–0.564 | 14.315 → 7.952 MiB |
| Immediate selective constructors | 1.012 | 0.868–1.127 | 2.035 → 2.031 MiB |
| Delayed selective constructors | 1.043 | 1.001–1.070 | 7.852 → 7.848 MiB |

Flat and dense improvements clear the registered 20% practical threshold in every repetition. The selective families do not show a consequential change at that threshold. Requested allocations measure heap requests, not RSS; retained candidate lists and copied state still have lifetime costs.

## What the correction owns

Each interpretation retains one ordered candidate list per rule. Consumption cannot create a new positive match, so it can discard tuples whose resources are no longer live. A relevant predicate/arity arrival invalidates its readers. Every processed equality conservatively invalidates all lists, including previously guard-failed candidates. Rule order, tuple order, guards and propagation history remain checked at application time. Forks copy their lists with their resource owner.

The change removes repeated full-list construction when successive applications only consume partners and post unrelated results. It does not implement subscriptions to partial joins, incremental equality maintenance of candidate tuples, or a bound on the size of an individual list. Those remain distinct S01/S02 choices. Dense delayed sources still pay repeated invalidation during binding updates.

The independent source gate adds six adverse cases covering priority-sensitive fresh arrivals, failed guards later enabled by equality, shared consumed partners, divergent forks and constructor congruence. Deliberately omitting either equality invalidation or arrival invalidation causes this gate to fail. The corrected code passes tests, replay, strict Clippy and formatting, including the earlier 46 finite source configurations, 1,664 head-plan configurations, 1,296 equation-pair cases and finite-sibling progress test.

## What remains true against other engines

In fresh paired runs, corrected relational/integrated lifecycle ratios are 1.487 on flat joins, 1.585 on immediate dense joins, 2.770 on delayed dense joins and 1.489 on delayed selective joins. Every repetition favors the integrated control in these cells. The delayed-selective direction is now consistent in this fresh comparison, unlike the noisy first pilot; this remains a bounded pilot result.

Immediate selective constructors continue to favor relational execution: the paired relational/integrated ratio is 0.609, range 0.596–0.685. Dedicated generic controls remain favorable in the other registered comparisons, except that the dense/scanned comparison has a wide range crossing one (0.992–2.684) and does not satisfy the same-direction rule. Do not manufacture precision from its median.

The evidence supports keeping candidate reuse and retaining direct constructor planning as a conditional opportunity. It does not establish relational integration as the preferred complete architecture. Generated execution, shared contextual equality, richer incidence designs, sustainable reclamation and whole-architecture composition remain unresolved.

## Evidence and next investigation

The [registration](../registrations/S02-candidate-reuse.md), [freeze](s02-candidate-reuse/freeze.json), [raw processes](s02-candidate-reuse/raw.jsonl), [correction ratios](s02-candidate-reuse/correction.json), [full cell summaries](s02-candidate-reuse/summary.csv) and [mutation outcomes](s02-candidate-reuse/mutations.json) support these conclusions. The [runner](../scripts/s02_candidate_reuse.py) verifies original binary hashes and an unchanged lifecycle harness before freezing the comparison.

T063's bounded relational implementation and comparison is complete; the S02 stage remains open for its other mechanisms and regimes. The next task is [S06 direct relation compilation](S06-direct-relation-entry.md). A further local relational repair could refine these ratios, but direct solving can eliminate the rule-selection path altogether and remains a more distinct unanswered architectural contrast. S01 retained joins remains required; this complete-list cache does not discharge it.

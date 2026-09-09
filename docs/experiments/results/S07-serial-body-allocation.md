# Serial execution saves bookkeeping without restricting overlap

The existing serial scheduler can avoid three busy-body Boolean jobs without restricting source programs. The allocation pilot saves completed-query traffic in every nonempty comparison and changes none of the empty cases. This strengthens the unrestricted control; it supplies no evidence for requiring non-overlapping rules.

## What the experiment establishes

The engine finishes and acknowledges one committed body before preparing another application. Before preparation, no body is pending and the busy condition is false. During that body, its support is the busy condition; acknowledgement restores false. These identities make the general busy subtraction, union and acknowledgement difference unnecessary in this scheduler.

The experimental `serial-body-accounting` feature uses these identities at the existing preparation and acknowledgement boundaries. It preserves the actual busy condition. Generic resource clients still use the general operations and exclude busy scopes correctly. Assertions check the serial premises, and ordinary owner/version validation still protects tokens. The feature is off by default.

**Serial bodies do not imply conflict-free application instances.** Overlapping rules, multiple matches of one rule, consuming competition, propagation history, late equality and source priority still require their existing handling. None is certified away here.

## Allocation result

The [prospective registration](../registrations/S07-serial-body-allocation.md) crosses general/serial builds with ordinary conditional execution and inferred unary dispatch. It covers propagation history, independent unary choices and mixed unary/multi-head rules sharing a predicate, at sizes 0 and 3 and prepared-rule reuse 1 and 4. Consumers release each query's answers. Two processes per cell give 96 runs and 48 exact allocation repeats.

Completed-query requested heap bytes at size 3 and four changing queries:

| Source | General, ordinary dispatch | Serial, ordinary dispatch | General, inferred dispatch | Serial, inferred dispatch |
|---|---:|---:|---:|---:|
| Propagation history | 9,574,818 | 9,310,626 | 9,466,075 | 9,201,883 |
| Unary choices | 417,165 | 404,781 | 408,906 | 396,522 |
| Mixed shared predicate | 529,860 | 505,092 | 522,575 | 497,807 |

All 12 nonempty build comparisons save traffic; all 12 empty comparisons are unchanged. In completed queries, allocation differences occur only during first delivery or exhaustion. Preparation, input construction, setup and disposal traffic are identical between builds. The extra specialization state does not cause an observed allocation penalty in these cells. This is a bounded result, not a general layout guarantee.

**Cancellation is accounted for separately.** Each process also interrupts fresh queries after 1, 64 and 256 service calls. Equal budgets can reach different logical progress after specialization, so these costs are not combined with completed queries to invent an application workload. Every query, cancellation and prepared owner returns to its recorded baseline.

The [raw receipts](s07-serial-body-allocation/), [summary](s07-serial-body-allocation/summary.csv) and [independent audit](s07-serial-body-allocation/audit.json) preserve the records. The audit verifies all cells, exact randomized order, phase order, repeatability, individual owner restoration and frozen source/toolchain/binary hashes.

## Correctness and useful-work checks

The serial feature passes 137 crate tests, including full source answers and application traces on overlapping and multi-head programs. Another 46 selected tests pass with counters disabled. The new resource test commits a serial body, proves an ordinary competing client cannot enter its scope, then checks acknowledgement and release. Strict scoped Clippy and package formatting pass. A workspace-wide formatting check reports differences outside this package; this result makes no workspace-wide formatting claim. The [validation logs](s07-serial-body-allocation/validation/) record the checks.

A separate diagnostic comparison enables prefix joins as well as unary dispatch and equality invalidation. Candidates stay unchanged. Dense propagation falls from 8,712 to 8,622 service calls; the independent-choice example falls from 37,049 to 36,748. The allocation builds use unary dispatch and equality invalidation, without prefix joins. These configurations answer different attribution questions and must not be treated as one timing comparison.

No counter-free ordinary-allocator timing matrix ran. Allocation-run clocks are not speed evidence. Requested heap traffic is not RSS; source construction, process startup, validation and native compilation are outside this pilot. Sustained answer retention remains unmeasured here.

## Consequence and next investigation

Use the serial specialization as a stronger unrestricted control in the next applicable comparison. Its premise comes from execution organization, so obtaining this saving does not require users to prove non-overlap or change their programs.

The [non-overlap investigation](S07-overlap-entry.md) remains required. Its next gate must identify an additional operation needed by an overlapping witness and removable under a sound property of application instances. Head-pattern discrimination alone cannot establish resource-conflict freedom. Compare inferred eligibility, checked declarations and required admission through that actual beneficiary, with linked and late-binding counterexamples.

This package counts toward the sequence's breadth review. Further serial-bookkeeping refinement is not selected now: the demonstrated saving already removes this confound from the language comparison. The strongest ready distinct alternative remains sustained observation and retained-answer layout; reconsider it at the non-overlap source/beneficiary gate. Neither this pilot nor completion of a bounded language study completes the architecture research.

# Key construction dominates deep-call allocation, but is not the only cost

Separated memoization saves source execution while spending substantially more allocation on recognizing reusable states. Key construction dominates its deep inert-call traffic, yet even assigning key construction zero allocation leaves it above Direct in all 96 matched query scenarios. The remaining runtime comparison must charge recognition, transport and complete lifecycle costs; an allocation-only key repair cannot settle it.

The [registered attribution](../registrations/S05-key-allocation.md) completes 717 of 720 isolated processes. Three repeats preserve the known owned whole-state-key allocation failure; no new failing source appears. The 478 completed query profiles repeat exactly, and 478 plain query rows agree with the preceding source/work gate. Stage invocation counts match the independent key-request, executed-transition and logical-transition counts.

## Where the requests occur

Values are requested bytes for one depth-128 query with distinct inert callers. The diagnostic interval starts at query engine setup and ends after complete answer collection. It excludes source/prepared construction, query construction, preflight, validation and disposal; these are not full lifecycle totals.

| Execution | Total requested | Key export/normalization | Source-machine steps | Other requests |
|---|---:|---:|---:|---:|
| Direct | 432,768 | 0 | 431,010 | 1,758 |
| Whole-state AlphaLive | 9,096,439 | 8,232,411 | 431,010 | 433,018 |
| Whole-state CompactLive | 6,641,007 | 5,778,499 | 431,010 | 431,498 |
| Separated uncached | 430,240 | 0 | 393,396 | 36,844 |
| Separated memo | 4,420,488 | 3,948,701 | 227,819 | 243,968 |

**The saved execution is real, but recognizing it is expensive.** Separated memo's source-step allocation drops to 227,819 bytes while key construction requests 3,948,701. Its other requests include 17,390 for detaching/exporting inert residuals, 556 for cached-edge copies, 1,152 for frontier/output assembly and 224,870 outside those scopes. The remainder includes table/container work and initial machine setup. It is not silently assigned to key construction.

**Zero-benefit callers retain both computation and recognition overhead.** At the same depth, readable callers request 9,300,283 bytes in separated memo versus 436,944 in Direct. Late-bound variable callers request 9,540,611 versus 485,952. The [complete audit](s05-key-allocation/audit.json) retains both caller-identity settings, all four depths, early failure and duplicate observations.

## Compact representation explains part of the capacity problem

On depth-32 duplicate residuals, whole-state AlphaLive requests 16,191,411 bytes, including 15,510,539 in key construction. CompactLive requests 4,505,787, including 3,822,163 in keys, while preserving the source answers and the same normalization policy. This directly measures a representation-dependent reduction before the larger owned-key case hits its resource limit.

At depth 128, distinct-caller AlphaLive again fails the unchanged 1 GiB address-space bound in both profiling repeats and the plain replay. CompactLive completes but still requests 67,551,163 bytes, including 62,227,987 in keys. Compact ground identities avoid much owned tree export; they do not eliminate constructing and normalizing a whole state key at every transition. These traffic figures are not retained memory, peak heap or RSS.

Separated memo on that large duplicate source requests 13,013,148 bytes: 9,550,173 in keys, 930,462 in inert export, 319,703 in source steps, 1,670,444 in cached-edge copying, 122,976 in frontier/output assembly and a 419,390 remainder. Source answer copies and per-derivation observations remain consequential after active-state separation. This measures the present representation, not a lower bound on exact observation.

## A key-only allocation repair is insufficient in this scope

The [optimistic bound](s05-key-allocation/key-bound.json) subtracts every byte measured in key export/normalization, charging no replacement machinery. All 96 separated-memo query scenarios still request more than their complete Direct setup/execution/collection interval. On deep inert callers, the remainder is 471,787 versus 432,768; on deep duplicate residuals it is 3,462,975 versus 2,848,476.

That bound does not reject compact separated keys, different recognition algorithms, coarser call reuse or cheaper transport. It establishes only that eliminating this allocation scope alone cannot reverse the current interval's traffic ordering. It says nothing about elapsed time, preparation/disposal or a changed algorithm that removes other work too.

## Stronger runtime controls now qualify

Global and Active Scan/Indexed execution, plus Global inferred specialization, pass 480 complete independent scalar/FIFO comparisons over the same deep sources. The checks include cancellation followed by prepared reuse and retained answers after producer disposal. Inferred specialization requires Global policy; all 96 Active requests are explicitly rejected by the existing API. An initial unsupported-policy attempt and its diagnosis remain in the receipts.

Those qualified controls can now challenge reuse in primary lifecycle timing. This is prepared/inferred execution evidence, not a fresh generated-native implementation or a timing result. The reference evaluator remains unchanged.

## Validation and next investigation

Allocation attribution is enabled only by `stage-alloc`. Five non-overlapping scopes record requested traffic and calls; plain builds omit the profile fields and meter dependency from the measured path. Plain feature closure and all successful metadata/counters match the earlier frozen gate. Nested meter starts make enclosing peak readings unsuitable, so no peak is reported. The complete diagnostic sum is bounded by the total interval and repeated exactly.

The [freeze](s05-key-allocation/freeze.json), [raw receipts](s05-key-allocation/runs/), [audit](s05-key-allocation/audit.json) and [bound script](../../../research/chr-reuse/experiments/key_allocation_bound.py) retain the evidence, including all three resource failures. Accounting tests, deep-source profiling tests, plain semantic regressions, stronger-control tests and strict Clippy validate the instrumentation and source responsibilities.

The [full portfolio review](S05-key-allocation-review.md) selects one bounded ordinary lifecycle comparison next under T075. It has substantial useful work, adverse cases and stronger qualified competitors; allocation attribution cannot answer that remaining time question. Broader recognition, sustained ownership, integration and other architectures remain required. The research goal remains active.

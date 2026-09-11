# Proper intermediates reduce retained-join costs but simpler controls remain strong

Proper intermediates substantially lower peak heap and sampled time relative to retaining terminal extensions on the tested product sources. They still request more bytes and reach higher peaks than local scanning in every matched scenario. Conventional indexing is a strong competitor on the keyed sources. This supports a conditional retention tradeoff, not an architecture selection.

The [registered sizing study](../registrations/S01-intermediate-lifecycle.md) completes **2,688 processes**:768 allocation runs and1,920 ordinary counter-free timing runs over384 cells. Every allocation pair agrees exactly and all final owners restore. Timing has five repetitions per cell and remains exploratory.

## The complete comparison

Six families cover reusable many-to-many prefixes, broad prefix consumption, late terminal information, distinct diagonal keys, cold first heads and dense unresolved terminal constructors. Widths4/8, one/four changing queries and immediate/all consumers are compared under local scanning, full tuples, partial joins, proper intermediates, conventional Scan/Indexed and specialized Scan/Indexed.

Every process validates complete scalar answers and warms its complete query paths before measurement. Preparation is then reused for the measured changing queries. Source/input creation, setup, execution, owned observation, producer and source disposal, consumer handling and final retained-answer release are charged. Two cancellation trials are measured separately. Retained answers are checked after all producer/source owners are disposed.

The qualification also cancels at setup or after one adapter call, resumes new queries using the same preparation, and checks held answers after disposal. All80 smoke configurations and48 initial size8 ownership qualifications pass. Adapter calls differ in granularity, so cancellation is an ownership result rather than a source-progress comparison.

## Lower retained state comes with rediscovery traffic

The table shows width8, four changing queries and retained-all consumers. Heap peaks refer to complete phases above the session baseline; cancellation peaks are reported separately in the audit. Requested bytes are traffic, not RSS.

| Source | Partial-join bytes / peak | Proper-intermediate bytes / peak | Local-scan bytes / peak | Indexed bytes / peak |
|---|---:|---:|---:|---:|
| Reusable product | 2,251,220 / 356,108 | 2,627,924 / 67,756 | 2,384,820 / 28,460 | 365,393 / 33,568 |
| Broad prefix invalidation | 2,284,520 / 364,692 | 345,352 / 70,164 | 139,464 / 32,120 | 167,743 / 36,076 |
| Late terminal information | 3,754,836 / 490,384 | 3,093,780 / 68,904 | 2,836,084 / 28,976 | 412,541 / 34,768 |
| Distinct diagonal keys | 2,251,220 / 356,084 | 1,224,788 / 67,756 | 1,008,564 / 28,460 | 296,401 / 36,008 |
| Cold first heads | 116,456 / 27,182 | 116,456 / 27,182 | 75,080 / 18,646 | 93,593 / 24,620 |
| Dense unknown terminal heads | 324,192 / 50,870 | 163,552 / 22,166 | 133,536 / 15,862 | 104,388 / 24,455 |

Across48 matched source/width/reuse/consumer scenarios, proper intermediates lower peak heap relative to partial joins in40 and tie in8 cold cases. Requested traffic is lower in36, higher in4 and equal in8. Against local scanning, both traffic and peak are higher in all48. Against Indexed, traffic is higher throughout, while peak is lower in8 dense cases and higher in40. These unweighted counts describe coverage, not workload importance.

The reusable-product case explains why entry counts alone were insufficient. Partial joins request2,033,312 bytes during setup and185,120 during execution. Proper intermediates reduce setup to277,280 but raise execution to2,317,856. Observation requests9,184 bytes in both. The smaller retained structure therefore has substantially more repeated execution allocation.

Compared with local scanning in that case, proper intermediates add185,472 setup bytes and57,632 execution bytes. The current retention policy saves earlier head checks, but the saved checks do not automatically offset saved-environment traversal, copying and final-partner rediscovery. These phase differences do not attribute all bytes to one primitive.

## Timing suggests a large improvement over partial joins, not over every control

These values are medians of five complete-session phase sums, in microseconds, for the same width8/four-query/retained-all cases. Cancellation phases are excluded from the complete total and retained separately. Medians of individual phases must not be added to reproduce these total medians.

| Source | Partial joins | Proper intermediates | Local scan | Indexed |
|---|---:|---:|---:|---:|
| Reusable product | 4,294 | 905 | 687 | 446 |
| Broad prefix invalidation | 3,653 | 407 | 77 | 139 |
| Late terminal information | 6,435 | 1,019 | 749 | 525 |
| Distinct diagonal keys | 4,281 | 546 | 334 | 320 |
| Cold first heads | 63 | 63 | 42 | 78 |
| Dense unknown terminal heads | 304 | 60 | 43 | 84 |

Controls run in randomized paired blocks on CPU0. For the reusable product, the intermediate/partial paired ratios range0.197–0.233, while intermediate/local-scan ranges1.204–1.391 and intermediate/Indexed ranges1.782–2.073. These are sampled contrasts, not confirmed statistical classifications. Other cells have outliers and overlap: the keyed intermediate/Indexed range spans0.914–1.901, for example.

The phase data also caution against attributing total time solely to matching. Identical source-input construction has different sampled times after different backends' preceding queries, despite matching input-allocation counts. Allocator/cache history and independent validation can influence later phases. The total charges the observed sequence; this study does not identify the cause of every timing difference.

Five repetitions are sufficient to identify contrasts worth further work, but not to turn overlap into equality or select a universal policy. Counter-free timing, allocation diagnostics and source/work evidence remain distinct.

## What the result changes next

The lifecycle comparison confirms that avoiding terminal retention can materially reduce peak heap and the sampled cost of this retained strategy. It also prevents a weak conclusion based only on comparing two caches: local scanning and competent conventional indexing remain stronger in important tested dimensions.

The current reusable prefixes contain cheap variable captures and atomic keys. The next discriminating source should make a genuinely reusable proper-prefix match expensive while keeping terminal changes sparse. First check whether existing canonical structure reuse, source specialization or a selective index already avoids that work. A favorable witness must demonstrate actual remaining repeated work in those strong controls; it must not disable indexing to manufacture a benefit.

T081 next qualifies that missing cost contrast and prospectively sizes it. Retain the present cheap keyed, cold and broad-invalidation sources as adverse controls. Investigate expensive structural/repeated-variable prefixes, prepared reuse and source-order-preserving selection only where they change the work comparison. This is a new source regime for the qualified mechanisms, not a reason to add another implementation before measuring.

This is package three after the full portfolio review. At the next result or obstruction, review the entire portfolio against compact solving's still-unexecuted operations and unfinished integration/demand costs. Further confirmation of the present large partial-join contrast has lower immediate value than determining whether the missing expensive-prefix benefit survives a strong alternative. That is a priority judgment; timing uncertainty, broader join decompositions, guarded/contextual ownership and sustained consumers remain open.

## Validation and limits

The [audit](s01-intermediate-lifecycle/audit.json) reconstructs384 exact allocation pairs,240 randomized timing blocks and all paired ratios. It checks phase order/continuity, immediate-query and cancellation restoration, final restoration after retained outputs, metadata and state-layout consistency. The [freeze](s01-intermediate-lifecycle/freeze.json) preserves exact sources, binaries, configurations and run orders. [Raw receipts](s01-intermediate-lifecycle/) include both smoke paths, all process outcomes, clock probes and strict allocation/ordinary Clippy results. No cutoff occurs.

Complete totals exclude process startup, validation, reporting and compiler/artifact costs. Cancellation is reported separately; finite retained-answer ownership does not establish sustained memory behavior. State-layout size is separate from measured requested heap. The reference interpreter is unchanged, and the research goal remains active.

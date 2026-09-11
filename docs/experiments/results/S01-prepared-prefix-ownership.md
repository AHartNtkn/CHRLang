# Prepared queries save allocation on completion, but retain more memory

**Reusing tables lowers total requested allocation in every normal-completion comparison, while raising peak live allocation in every comparison.** Early cancellation can reverse the saving. This is a measured ownership tradeoff; the metered timings do not establish execution speed.

The [registered matrix](../registrations/S01-prepared-prefix-ownership.md) completed 1,024 isolated processes, producing 512 exactly repeated allocation records and 256 matched fresh/reuse comparisons. Every process restored its initial live-byte count after producer, template and retained-answer disposal. The independent source gate passes in both counter-free builds.

## What the comparison says

| Scenario | Reuse requests fewer bytes | Reuse requests more bytes | Reuse has higher peak |
|---|---:|---:|---:|
| Both queries complete |128|0|128|
| First query cancelled, second completes |60|68|128|
| Total |188|68|256|

The matrix includes selective and neutral joins with two unchanged tables, and duplicate/broad joins whose right table changes on each query. Both scheduling policies, both access methods, widths16/128, ordinary/probe execution and immediate/retained answers are included. No workload weights are assigned to these counts.

For ordinary execution, width128, Global scheduling, Indexed access, two complete queries and immediately released answers:

| Family | Fresh requested bytes | Reuse requested bytes | Fresh peak above root | Reuse peak above root |
|---|---:|---:|---:|---:|
| Selective |1,025,080|996,706|321,014|543,010|
| Neutral |1,019,613|991,421|319,444|541,350|
| Duplicate |749,638|699,620|224,669|310,719|
| Broad |946,746|906,758|297,868|421,156|

These are requested heap bytes, not RSS. Peak includes the retained template alongside each live query, and retained answers where selected.

## Why setup alone would give the wrong conclusion

In the selective example, reuse spends403,118bytes preparing the prefix. Across the two queries it then requests470,880bytes in setup and38,240 in execution. Fresh execution requests488,132 in setup and452,480 in execution. Observation requests79,876 in both. The apparent execution saving mostly pays for earlier preparation and copied ownership; the net saving is28,374bytes.

The implementation confirms the responsibility: `PreparedQuery::start` clones the template engine, including store, pools, indexes, queues and arena, then inserts the suffix. Fresh insertion occurs during engine execution. Thus phase names alone do not identify equal work. The sum across the lifecycle is the useful comparison.

With the selective example's first query cancelled, reuse instead requests954,404bytes versus777,770 fresh. Fresh cancellation avoids much initial admission; reuse has already paid for a prefix and its clone. This is an actual upfront-cost exposure, not an assumption about how often users cancel.

## The experiment repair

The first attempt found that a prepared duplicate query could complete within the planned n/2 cancellation ticks. The current fixture cancels both paths after exactly one tick, verifies that neither has completed, and then runs the second query to completion. The full matrix was repeated with new frozen sources and binaries. The first attempt and its failing cell remain in [the attempt evidence](s01-prepared-prefix-ownership-cancel-attempt/failure.json).

The successful archive contains the exact execution driver. After execution, its audit was strengthened to require the full phase sequence and exactly one sample per cell in each block; all records pass those checks too. The runner's current optional modes extend the earlier lifecycle experiment; its historical binary/source freezes remain the authority for those earlier results.

## Which decision comes next

The results justify testing whether the peak premium comes from avoidable cloning before using prepared queries as an architectural advantage. The existing arena sharing option and fork ownership diagnostics offer a bounded intervention: qualify the same source contract, attribute copies, and compare complete allocation costs including any copy forced by changing suffixes. Sharing the arena cannot be assumed to fix cloned stores or indexes.

That intervention currently has greater decision value per implementation cost than another timing-precision campaign: every matched case shows a concrete ownership premium, and the candidate mechanism already exists. Integrated admission and graph attribution require broader representation work; direct solving addresses a distinct responsibility. Reconsider all three at this next result. Neither prepared queries nor sharing receive a production disposition yet. Package count is two since the full portfolio review; T082 and the goal remain active.

[Raw results and source/binary freeze](s01-prepared-prefix-ownership/) · [Per-cell analysis](s01-prepared-prefix-ownership/analysis.json) · [Runnable driver and auditor](../../../research/chr-compiled/experiments/prepared_prefix_ownership.py)

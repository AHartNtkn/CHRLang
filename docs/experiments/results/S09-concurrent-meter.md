# Concurrent queries dispose completely; cancellation traffic varies

All eighty registered diagnostic processes pass complete-answer, cancellation/reuse and requested-heap restoration checks. Completed-query phase traffic is stable in these samples. Cancellation traffic varies because outstanding services can perform different amounts of work before observing cancellation; it must not be required to replay as deterministic execution.

## Evidence

The [prospective registration](../registrations/S09-concurrent-meter.md) fixes inline and 1/2/4 workers, source quanta1/16, phased and whole-lifecycle windows, and five fresh processes per configuration. Every process performs four runtime lifetimes, each containing balanced work, cancellation, skewed work, a tiny query and a smaller changed query. This gives **320 runtime lifetimes, 1,280 completed queries and 320 cancelled queries**. All complete products and raw counts agree with the hand-derived independent products. All runtimes restore the post-channel-initialization baseline after disposal.

The [raw receipts](s09-concurrent-meter/) include executable commands, outputs, source/binary hashes and the [traffic summary](s09-concurrent-meter/traffic-values.json). The [runner](../../../research/chr-factors/experiments/validate_concurrent_meter.py) enforces 60 seconds and 1 GiB address space per process. No process reaches a bound. Counter-free release compilation and Clippy pass.

Every configuration requests up to four regional services before admitting the first response. Closing the cancelled query must handle the other outstanding requests whether workers are still servicing them or their replies are buffered. Source-service cancellation is checked between atomic source steps. The earlier worker latch gate independently establishes cancellation with workers physically in service; this diagnostic prices allocation consequences without those test hooks.

## Interpretation for measurement

**The variation is localized to the cancelled query's service/close intervals in the phased samples.** Completed-query phase traffic repeats exactly by configuration and query position. Total traffic also varies in worker modes because the total includes that cancelled query. Inline execution performs the prefetched work serially before admission and therefore has a different cancellation-work pattern; this is an explicit coordinator policy, not a claim about intrinsic parallel overhead.

**The two meter windows serve different purposes.** Whole-lifecycle runs preserve one peak window through runtime disposal. Phased runs reset the peak at each phase and support phase-local readings; their final outer peak is not a whole-lifecycle maximum. Allocation counters remain cumulative, so total traffic is interpretable in both modes. Concurrent peaks and cancellation work need ranges rather than forced equality across repetitions.

**No extra synchronization protocol is justified by this witness.** The tested completed paths have stable phase traffic and full disposal. This does not prove all workloads have quiescent acknowledgement boundaries. Carry the same accounting checks into the cost pilot and investigate new discrepancies; report a joint interval if individual phases cannot be credibly separated. Do not insert barriers merely to manufacture identical concurrent peaks or cancellation counts.

## Next decision

The [native sizing](S09-native-sizing.md) now establishes safe initial sizes for a counter-free timing comparison. Register the cold/reuse and granularity matrix prospectively, include process CPU as a separate resource endpoint, and retain skewed/tiny cases. Completed workloads and cancellation are different endpoints and need separate interpretation. T069 remains active; connected-work parallelism and the broader sequence remain unresolved.

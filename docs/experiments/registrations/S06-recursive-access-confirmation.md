# Confirm recursive costs with Scan and Indexed controls

The prior Indexed pilot omitted a consequential existing control. Global Scan avoids dependency/index maintenance that Indexed execution repeats over growing open accumulators. Compare both access modes under identical sources before attributing a total-cost benefit to contraction.

Use feature-off original/sealed and feature-on original/sealed/contracted, each with Indexed and Scan access: ten configurations. Names without a suffix retain Indexed; "-scan" names select Scan. Cross all ten recursive source families, depths 0/64, query counts 1/4 and resources absent/present: 80 source configurations, 800 cells. Queries alternate depth and occurrence order and reuse preparation. Sources and runtime are unchanged; the runners expose access choice and additional work counters.

Gate all cells with two separate counter-free allocation runs (1,600 processes), requiring exact per-phase replay and query/prepared restoration. Run 100 metered cancellation cases at depth64/two queries/resources/32 ticks, all configurations/families. Then run seven ordinary-allocator timing blocks (5,600 processes), shuffling source configurations and then the ten configurations within each block with seed 7611. Run 100 ordinary cancellation cases and 800 independent metrics-enabled work processes. Total 8,200 plus two meter self-checks. No warmups or outlier exclusion.

Freeze sources, driver/analysis, registration, compiler and six release binaries. Each process uses the first available CPU, 60 seconds wall/CPU, 1 GiB address space and two-million scalar/source steps. Full scalar answers are validated outside primary intervals. Ordinary timing disables counters and metering. Neither allocation nor work elapsed times are speed evidence; requested heap bytes are not RSS.

Primary totals include preparation, setup, complete execution/observation and engine/answer/prepared disposal. Report source/input-inclusive time, first observation, phases, requested traffic and peak growth separately. Native compilation is excluded. Work records separate dependency visits, refreshes and index repairs; sum retired split/terminal segments once and require matching source applications/forks/failures/answers across configurations.

For each source configuration compare in this order:
1. on/contracted-scan versus on/sealed-scan
2. on/contracted versus on/sealed
3. off/sealed-scan versus off/sealed
4. on/contracted-scan versus on/contracted
5. on/contracted-scan versus off/sealed-scan
6. on/sealed versus off/sealed
7. on/sealed-scan versus off/sealed-scan
8. off/original-scan versus off/original
9. on/contracted-scan versus off/original-scan

Use the geometric mean of seven paired primary ratios and a pointwise 95% percentile bootstrap interval with 10,000 resamples, seed 7612 + 9*source_index + contrast_index, sorted bounds 249/9749. Upper bound below 0.90 is a practical numerator gain; lower bound above 1.10 is a loss; otherwise unresolved. Counts describe this suite without simultaneous/population claims or workload weights.

H1: Scan removes unnecessary maintenance and can materially weaken the earlier contraction advantage. H2: contraction may retain gains through reduced discovery/reposting even under Scan. H3: short and competing-call cases can retain overhead. Use work counters to distinguish maintenance from admission; investigate consequential contradictions or uncertainty before a broader recommendation. Exact/direct code, successful ground-seed controls, broader recurrences and sustained lifetimes remain unresolved. This comparison cannot select a universal architecture.

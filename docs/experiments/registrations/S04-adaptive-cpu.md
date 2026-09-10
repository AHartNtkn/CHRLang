# Targeted CPU/elapsed attribution for adaptive restoration

T077 investigates the consequential uncertainty in S04-adaptive-cost-pilot. Service supplied at least 91.1% of measured cost there, so this package targets service rather than preparation refinement. The strongest distinct alternative is T072 sound cheaper relevant-read validation; it still requires implementation and source-validity work. The existing restoration sources and qualified static controls make attribution a bounded way to determine whether elapsed variation conceals a policy consequence. Checkpoint/replay costs remain independently required.

## Mechanism and qualification

Add compile-time optional Linux 64-bit thread-CPU clocks around existing phase intervals. Preserve original Instant-only timing as a separately built control. CPU brackets surround the elapsed interval, so small negative elapsed-minus-CPU values can reflect clock overhead. Verify installed CLOCK_THREAD_CPUTIME_ID=3 and the native timespec ABI. No fallback clock is allowed. Preserve static engine types, source policies, operational bounds and independent preflights. Record existing operational service counts without adding engine diagnostics.

Five diagnostic processes each sample 100,000 empty CPU/elapsed pairs. Each also checks a 20-ms sleep (CPU less than 25% of elapsed) and a 20-ms busy loop (positive CPU no greater than 110% of elapsed). These are clock-identity checks, not frequency calibration. A diagnostic service aggregate qualifies only if both CPU and elapsed exceed 100 times its number of nonempty service intervals times the largest corresponding empty-pair median. Tiny preparation/disposal phases are not independently ranked.

## Exact targeted comparison

Eight unchanged modes: Copy, initial reunion, stateless eager, scheduled EveryBoundary, fixed skips 1/8 and failed-check backoff caps 1/8. Six source scenarios: late/depth0/four queries/first-answer cancellation; late/depth4/four queries/exhaustion; history/depth4/one query/first-answer cancellation; history/depth4/four queries/exhaustion; plain/depth4/four queries/exhaustion; plain/depth0/one query/exhaustion. Retain all measured answers so independent validation remains possible after producer disposal. Source construction and first/full observation accounting remain those of the existing pilot. No immediate-consumer inference follows.

Pin serial child processes to CPUs 0 and 2 after verifying availability and distinct physical cores. Use seed 771004, one warmup per mode/scenario/CPU/build, then 16 paired blocks per CPU. Rotate eight-mode permutations by block modulo eight, twice per position. Randomize which build runs first in each pair and shuffle scenario/CPU order per block. Two builds (wall-only and CPU-instrumented) give 3,072 primary/diagnostic runs plus 192 warmups, and five clock controls. Freeze exact inputs, binaries and complete schedule before execution.

Per process: 60-second wall/CPU, 1-GiB address space and existing 200,000 service calls per query. Campaign bound: 30 minutes. All searches must exhaust or meet the registered first-answer cancellation endpoint. Validate complete retained answers independently and check repeated counts/service counts. No source policy changes or adaptive tuning during the run. CPU instrumentation is diagnostic, not a primary timing optimization.

## Interpretation

For each cell report all elapsed/CPU samples and their spreads. Material elapsed excess means wall-minus-thread-CPU exceeds both 5% of elapsed and ten times the service-interval count times the largest empty CPU median. This is evidence of time not accounted by the executing thread, not proof of a particular scheduling cause. Mark wall excursions above 1.5 times the cell median whose CPU stays within 1.1 times its median; retain every sample, including mixed cases. Compare wall-only and instrumented elapsed distributions to expose possible perturbation; do not infer its absence merely from matching answers.

Optionally report the prospectively fixed wall-only policy contrasts: backoff 1/8 versus Copy, reunion, eager, fixed1 and fixed8, plus scheduled versus eager (66 contrasts). A gain requires every paired lifecycle ratio below 0.90 on both CPUs; a loss requires every ratio above 1.10 on both; otherwise unresolved. This is observed-repeat separation, not a population confidence interval or workload weight. Do not select a policy from CPU diagnostics alone.

Use the outcome to choose targeted restoration work or a complete checkpoint/replay cost comparison, comparing that next action with relevant-read repair. Frequency, allocation, cache behavior, successful-but-unprofitable separation, broader sources, sustained lifetime and complete architectures remain required wherever consequential. This package does not complete T077 or the research goal.

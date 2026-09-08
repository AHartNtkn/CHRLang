# Native worker sizing is feasible; the ordering needs confirmation

All twelve sizing cells completed within the registered bounds with exact complete products. The observations suggest that useful work size and worker count can change wall-time behavior while CPU use increases. One observation per cell does not establish a speedup or ordering.

The [sizing registration](../registrations/S09-native-sizing.md) fixes four balanced regions, four changing queries, quantum16, window4 and depths16/64/256. The [ordinary-allocator runner](../../../research/chr-factors/examples/worker_cost.rs) prepares source once and records setup, joint execution/observation, first observation, cancellation/drain-capable close, disposal and shutdown. Every query's answers and raw count are checked outside primary wall intervals.

| Depth | Inline lifecycle | One worker | Two workers | Four workers |
|---|---:|---:|---:|---:|
| 16 | 0.495 ms | 1.431 ms | 1.170 ms | 1.096 ms |
| 64 | 2.242 ms | 2.831 ms | 2.430 ms | 1.725 ms |
| 256 | 4.603 ms | 9.739 ms | 5.954 ms | 4.445 ms |

These are exploratory observations, not comparative conclusions. At depth256, recorded process CPU was approximately 6.00 ms for inline and 15.29 ms for four workers. Process CPU includes startup, validation and reporting, whereas the wall lifecycle sums the runner's measured phases. They answer different resource questions and must not be treated as identical intervals.

The [receipts](s09-native-sizing/) contain commands, complete outputs, process CPU, bounds and [source/binary hashes](s09-native-sizing/hashes.json). The [sizing driver](../../../research/chr-factors/experiments/size_workers.py) runs cells sequentially, once each, with a 60-second timeout and 1 GiB address-space limit. No cutoff occurred.

The next pilot should prospectively fix repetitions, randomized order, practical thresholds, cold versus repeated query counts, source-service quantum, and balanced/skewed/tiny controls. Quantum16 requires many service round trips at larger depths; include a coarser quantum to test coordination amortization. Keep CPU, wall time, memory and first-answer delay separate. No user-program compilation cost has been isolated, and this experimental runtime comparison cannot establish complete architectural lifecycle superiority.

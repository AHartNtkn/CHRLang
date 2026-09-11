# Periodic clearing reduces retained state, but can increase peak memory

Clearing traces between queries is correct in the tested callers, but automatic periodic clearing is not a clear efficiency improvement. Frequent clearing sacrifices reuse; infrequent clearing preserves some speed gains without reducing peak memory. A useful next retention design would need to distinguish valuable entries from expensive abandoned work.

This experiment supports an explicit way to release trace state when a caller is finished with it. It does not select a general cache policy or impose a language restriction.

## What was compared

The existing caller now supports `clear_traces`: release cached keys, completed traces and abandoned computations while keeping prepared rules. Live query handles prevent clearing because they may still refer to that work. Queries, retained answers and subsequent regenerated calls preserve their existing meaning.

The pilot compares unbounded trace retention with clearing after 1, 4 or 16 queries, plus Direct execution and prepared data plans with inferred specialization. Four source families cover ordinary recursion, a failed alternative, fresh aliased outputs and binding before remaining private work. Each serves 32 queries, with repeated input, a four-input cycle or sixteen distinct inputs followed by revisits. Complete and first-answer cancellation runs both use immediate and retained-all consumers.

The [registration](../registrations/S05-call-reclamation.md) records the controls and measurement rules. Allocation runs use the requested-heap meter separately from ordinary, counter-free timing. Source construction, preparation, query setup, service/observation, consumption, maintenance and disposal all count. Correct answers are checked outside those measured phases.

## Less retained state is not necessarily a lower peak

| Clearing policy | Scenarios with a lower peak than unbounded traces | Scenarios allocating more | Peak ratio range |
|---|---:|---:|---:|
| Every query | 27 / 48 | 48 / 48 | 0.225–1.209 |
| Every four queries | 16 / 48 | 48 / 48 | 0.444–1.244 |
| Every sixteen queries | 0 / 48 | 48 / 48 | 1.000–1.151 |

The strongest favorable memory case involves distinct inputs and cancellation. For ordinary recursion, sixteen distinct inputs followed by revisits, and immediate answer disposal, unbounded traces peak at 216,524 bytes and retain 190,149 bytes after the last query. Clearing every query reduces the peak to 49,154 bytes and leaves 4,413 bytes, but increases requested allocation from 2,298,572 to 3,544,468 bytes. Direct peaks at 24,098 bytes; data plans plus inference peak at 37,542 bytes.

The adverse cases expose a different lifetime interaction. With a failed alternative and all answers retained, clearing every query raises peak ownership from 26,197 to 31,670 bytes. Clearing every four queries on cycling input raises it from 36,331 to 45,192 bytes. Both peaks occur during the last query's service, while earlier consumer-owned answers remain live.

The cause is regeneration's temporary working state. In the every-query example, live ownership before the peak is lower with clearing: 20,003 versus 23,641 bytes. Yet service temporarily adds 11,667 bytes instead of 2,556. Clearing a completed trace trades a compact replay record for a fresh computation alongside the accumulated answers. The [six targeted measurements](S05-call-reclamation-peaks.json) reproduce all three policies' largest relative peak regressions.

## Frequent clearing loses the execution advantage

A timing gain requires at least a 10% median paired improvement and every one of five repetitions faster. Other comparisons remain uncertain unless the symmetric loss rule holds. All comparisons exceed the measured 2,300 ns clock floor.

| Trace policy | Against Direct: gains / losses / uncertain | Against data plans + inference: gains / losses / uncertain |
|---|---:|---:|
| Unbounded | 16 / 0 / 32 | 22 / 1 / 25 |
| Clear every query | 0 / 27 / 21 | 0 / 17 / 31 |
| Clear every four | 1 / 18 / 29 | 8 / 13 / 27 |
| Clear every sixteen | 11 / 10 / 27 | 19 / 8 / 21 |

Against unbounded traces, clearing every query loses 42 of 48 comparisons; every four loses 26; every sixteen loses 12. The others remain uncertain. No bounded policy qualifies faster than unbounded traces.

Every-sixteen clearing retains gains against both recomputation controls in seven scenarios. None has a lower peak than unbounded traces: six peaks are equal and one is about 1% higher. Releasing state after a window can still reduce idle ownership, but these measurements do not establish a combined speed-and-peak improvement.

A separate work-count test explains the cost of revisits. Across sixteen distinct inputs then revisits, ordinary calls execute 1,632 private service steps and replay 1,632 when retained. Every tested clearing window executes 3,264 steps and replays none. The other three families show the same doubling of private execution. Each policy clears entries before their next use; a query-count window is insensitive to reuse distance.

## What was checked, and what comes next

The new semantic test compares 512 complete queries and 512 cancellation prefixes against Direct, including 60,544 complete-run service checkpoints. Independent scalar answers also agree. Existing caller and isolated-trace tests pass with counters enabled or disabled as applicable. The allocation pilot completes 576 runs across 288 cells with equal repeated costs, equivalent consumer ownership and full final release. Timing completes 1,440 ordinary processes and 528 comparisons. Retained answers remain valid after clearing and producer disposal. Scoped Clippy and the extended fixture-oracle test pass.

The implementation adds an explicit clearing operation to the existing table and caller; it introduces no replacement executor. Remaining retention alternatives include selective eviction, retaining useful completed results while releasing abandoned work, and policies informed by reuse distance. Their extra bookkeeping needs a benefit that beats the simple controls measured here.

The [portfolio decision](S05-call-reclamation-review.md) selects sparse connected projection next. It addresses a measured source of work in an independent architectural direction. Broader caller observers and selective retention remain required investigations; the private-call fixtures do not constrain the eventual language.

Detailed results are in [requested ownership](S05-call-reclamation.json) and [runtime samples and comparisons](S05-call-reclamation-time.json). Reproduce the measurements with `call_reclamation.py` (allocation), `call_reclamation.py time` and `call_reclamation.py peaks` after building the runner into the two target directories named in that script. The research goal remains active.

# Installed plans and retained structures have reproducible allocation accounting

The artifact runner now measures requested heap allocation separately from ordinary timing. All 544 independently checked queries pass; 34 metered configurations reproduce identical phase readings. Every query and prepared lifetime returns to its starting live-byte count.

This closes the installed-plan allocation gap under the [prospective gate](../registrations/S01-artifact-allocation-gate.md). It does not establish runtime superiority or cover cancellation and artifact-file lifetime.

## What is measured

The runner reuses the existing experimental allocator. Each phase records requested bytes, allocation/deallocation calls, starting and ending live bytes, and absolute peak live requested bytes. Source construction, preparation, setup, execution, observation and each disposal phase have separate readings. JSON formatting happens after capturing them.

The ordinary build contains no meter symbols and emits no allocation fields. The diagnostic build identifies its allocator as `requested-meter`; its timing values are not primary timing evidence. Both disable engine, kernel and observer counters. The allocator's isolated zeroed-allocation, growth, shrink and release checks run in each diagnostic process.

Input and oracle construction, exact answer validation and JSON output are outside the phase traffic. Query ownership checks span input/oracle creation through their release, along with the engine and answer. Prepared ownership checks span source construction through prepared-state disposal. Process output buffering is initialized before those checks. Thus excluded harness allocations cannot be mistaken for leaked runtime state merely because they occur between measured phases.

Peak readings include all live requested allocations in the process at that phase, including the oracle and process-owned state. They are not RSS, incremental phase ownership, or an isolated engine peak. Adding phase peaks is invalid. Summing phase traffic describes the included runtime intervals, whose input construction was excluded; it is not complete process allocation traffic.

## Validation

The gate uses payload's seven generic/native modes and subscription's ten modes, sizes zero and sixteen, four changing queries, two repetitions, and separate ordinary/metered builds. All 136 processes exhaust and produce complete answers equivalent to independent scalar execution. All query and lifecycle timing sums reconcile.

The 34 metered cells reproduce their full allocation readings exactly, including phase live endpoints and peaks. Both query and prepared-state restoration checks pass in every diagnostic process. These runs exercise installed generated update tables, prepared data plans and retained join structures, rather than measuring an empty update-plan field.

Both emitter/library configurations pass strict Clippy. Executable hashes and allocator-symbol separation are independently checked. See the [validation receipt](s01-artifact-allocation-gate/validation.json), [artifact summary](s01-artifact-allocation-gate/summary.json) and [source freeze](s01-artifact-allocation-gate/source-freeze.json).

## Concrete diagnostic observations

For the first size-sixteen query, the included setup-through-answer-disposal phases request the following bytes. These are exact replayed observations for these sources; they do not rank total efficiency.

| Source | Generic execution | Prepared update plan | Native generated repair | Native with prepared repair |
|---|---:|---:|---:|---:|
| Private payload | 102,970 | 37,773 | 30,797 | 30,797 |
| Subscription | 572,041 | 567,305 | 309,793 | 309,793 |

The same subscription query requests 117,210 bytes with the dedicated indexed source executor, 137,310 with eager retention and 142,228 with demand subscriptions. These stronger controls remain essential: the native/generic difference alone cannot settle the matching organization.

Preparation has its own cost. Payload preparation requests 3,513 bytes for generic execution, 5,620 for a prepared plan, 5,017 for native generated repair and 7,124 for native plus prepared repair. Query savings therefore cannot stand in for lifecycle allocation, and none of these counts includes native compilation. The native-plus-prepared constructor currently performs additional linking/preparation work; the pilot must preserve that attribution rather than call it an unavoidable native cost.

## Next evidence

Complete cancellation and artifact-lifetime accounting, then register the pilot over favorable and adverse source placements and changing-query reuse. Carry retained controls into subscription comparisons and use separate ordinary timing and allocation runs. The current readings supply validated diagnostic endpoints, not a reason to restrict the study to these two small sources or to recommend a compiler mode.

T070 remains active. Direct pull-tabbing and derivation reuse remain the strongest distinct alternative at the selection boundary after the first complete cost contrast. The broader architecture goal remains unresolved.

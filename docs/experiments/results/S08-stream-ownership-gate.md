# Releasing answers does not release all engine history

The first stream probe separates consumer-owned answers from engine retention. Graph and continuation-table owners retain substantial memory after consumers release answers; every engine releases its query allocations when disposed. The next question is which retained state future work still needs, and what sound reclamation would cost.

**This is allocation and correctness evidence only.** No ordinary timing comparison ran. Two finite stream lengths do not establish asymptotic growth, sustainable limits or a winning architecture. The [registration](../registrations/S08-stream-ownership-gate.md) and [raw audit](s08-stream-ownership-gate/audit.json) define the bounded scope.

## A stream inside one live query

**The source progressively offers an answer or continues to a later answer.** A separate countdown supplies work before each observation. Output payload size, repeated versus distinct values, and unknown aliases are independent source parameters. Optional completion consumes a token after an output constructor becomes available; the final continuation can explicitly fail. These are complete CHR answers from alternative branches, not partial snapshots presented as answers.

**Seven paths implement the same bounded source contract.** They are direct continuation execution, compact live-history reuse, ordinary and specialized Scan, dependency-aware graphs, graph templates, and a checked lazy exact-source generator. The generator checks the full rules and query shape, then constructs one answer per request. It never preconstructs the complete answer set. It is a hand-derived control, not evidence that a general compiler can derive that generator.

**Independent correctness covers 384 source configurations and 2,688 engine runs.** The scalar source evaluator agrees with an analytical answer formula across three families, both resource and terminal-failure settings, depths 0/1/4/16, work 0/3, payload 0/4 and both input orders. Every engine agrees on complete raw answers, including duplicate multiplicity and joint aliases. Another 126 consumer runs validate immediate release, a four-answer window and retain-all, with complete delivery or cancellation after four answers. The two tests pass with default and counter-free features; scoped strict Clippy passes for the tests and allocation runner.

**The graph source certificate accepts explicit failure but does not accept arbitrary closed contradictory equations in a body.** The stream expresses its failing terminal branch using the language's explicit failure goal. General equality-enabled graph execution remains a separate required investigation. This probe makes no broader eligibility claim.

## What the allocation probe measures

**All 546 registered processes succeeded, with 273 exact replays.** The complete matrix uses depths 16/64, work four, payload eight, successful terminals, all three families, both resource settings, three consumers and seven engines. Each process reuses preparation for depth n and n+1, reversing the second query's input order. The cancellation matrix interrupts the first alias query after four answers and completes the second.

**The harness releases validation data before the measured run.** An unmeasured replay checks complete answers independently. During the allocation probe, only the selected consumer retains answers. Fixed consumer slots and checkpoint storage are preallocated outside the owner baseline, equally across consumer policies at each source size. Answer payload allocations are included. Snapshots record setup, common answer counts, exhaustion/cancellation, consumer release, engine disposal and prepared disposal.

**All 168 matched engine snapshots are independent of consumer policy after consumer release.** For each engine/source/query combination, immediate release, window-four and retain-all converge to the same engine-owned requested bytes. Every query and prepared disposal returns to its prior live baseline. The [audit](s08-stream-ownership-gate/audit.json) independently checks all commands, snapshot sequences, counts, baselines, summaries, source/binary hashes and exact replays.

## The owners have different trajectories

**Shared execution state can remain large even with immediate answer release.** The table shows requested-live query growth in bytes on repeated values with token consumption. Preparation and fixed harness slots are excluded; query input and engine allocations are included. “After release” means all answers are released and the exhausted engine is still alive. “At answer 64” occurs while delivery has not yet finished for the depth-64 query.

| Engine | Depth 16: after release of 17 answers | Depth 64: at answer 64 | Depth 64: after release of 65 answers |
|---|---:|---:|---:|
| Direct | 6,010 | 23,797 | 17,818 |
| Compact live-history reuse | 118,865 | 572,289 | 572,289 |
| Scan | 22,080 | 46,337 | 22,080 |
| Specialized Scan | 44,096 | 64,177 | 44,096 |
| Graph dependencies | 102,813 | 846,577 | 851,405 |
| Graph templates | 58,256 | 356,098 | 354,882 |
| Checked lazy generator | 0 | 0 | 0 |

**Setup size and scheduling also matter.** Direct setup grows from 5,881 bytes at depth 16 to 17,689 at depth 64; much of its final retained memory therefore originates with query representation. Scan at depth 64 holds 271,478 bytes after its first answer, then releases branch state as the frontier empties. A final-only observation would miss that larger live frontier. The [complete snapshots](s08-stream-ownership-gate/summary.json) retain these trajectories rather than reducing them to an engine ranking.

**Distinct outputs amplify some retained owners.** After 65 distinct answers have been released, compact-live still owns 3,597,457 bytes, direct owns 40,602, graph dependencies 852,173, and graph templates 477,130. A zero query-heap result for the exact-source generator excludes its fixed stack state and prepared schema. It demonstrates an available source-specific ownership control, not zero computation or a general constant-space implementation.

**Consumer ownership is separately visible.** With 65 alias answers retained, direct holds 101,595 bytes; releasing them leaves 17,810. The dependency graph moves from 944,546 to 851,401. The released answer allocations differ across engines despite semantic equivalence, so exact observation does not imply identical physical output representation. Any later output-storage attribution must separate those costs from engine history.

## What can and cannot yet be concluded

**Current code identifies plausible retaining owners, not their necessity.** The continuation machine owns an append-only term arena, and reuse additionally owns key/transition tables including cached answers. The graph owns nodes, contextual results, births, obligations, resources and templates. The compiled search retains its frontier allocation even after the frontier empties. Raw live-byte measurements do not apportion every byte among those structures or prove which could be reclaimed before exhaustion.

**A next ownership analysis is more valuable than timing these representations immediately.** Establish the future roots at successive delivery boundaries, distinguish reusable cached results from state reachable only through completed histories, and validate any proposed release against aliases, resource consumption, failure and later work. Reclamation, eviction and regeneration must be compared with retaining useful state. Do not infer that all past graph nodes are dead, or clear a cache merely to obtain a lower memory number.

**The required follow-up is a sound retention/reclamation gate, followed by bounded lifetime costs.** This can change both sustainable efficiency and necessary implementation complexity. It is ready because the source, incremental engines, independent observations and owner baselines now exist. Call-level reuse and distinct integrated execution remain stronger independent alternatives than further small output-copy tuning; reconsider them at the first lifetime-cost boundary, as required by the [selection review](S05-opportunity-next.md).

**T074 remains active and the goal remains open.** Longer regimes, ordinary timing, failing-stream costs, compact outputs, publication/backpressure, complete architectures and held-out challenges remain required. This probe establishes the ownership question; it does not settle it.

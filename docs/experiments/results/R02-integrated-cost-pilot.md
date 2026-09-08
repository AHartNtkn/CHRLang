# R02 first cost pilot: fanout benefit, higher retained heap, incomplete repair coverage

Integrated execution is faster on the moderate shared-fanout cases, but slower than the best measured dedicated control on independent bindings, build and zero-yield repair. Its requested live heap is higher throughout those comparisons. The result supports a conditional representation tradeoff, not an overall architecture choice.

A consequential coverage limit emerged from the work counts: every cost cell performs zero descriptor repairs and zero source applications with pending equality. The semantic gate exercises those operations, but this pilot does not measure them. Constructor decomposition and nested alias repair are selected next before drawing a broader R02 conclusion.

## Evidence and measurement boundary

All 490 prospectively registered processes complete and validate 1,960 full answers. The [registry](../registrations/R02-integrated-cost-pilot.md), [source/binary freeze](R02-integrated-pilot/freeze.json), [environment/features](R02-integrated-pilot/environment.json), [order](R02-integrated-pilot/order.json), [raw results](R02-integrated-pilot/raw.jsonl) and [70-cell summary](R02-integrated-pilot/summary.tsv) preserve the experiment. `research/chr-integrated/experiments/analyze.py` regenerates the summary and verifies completion and bounded allocation no-growth.

Each process prepares once and runs n,n+1,n,n+1. Lifecycle includes preparation, query lowering/setup, source execution, first full observation, engine/answer disposal and prepared disposal. Fixture/oracle work is timed separately. Native compilation and process launch are excluded, so the generated controls do not establish cold compilation break-even. Primary binaries use ordinary allocation with integrated, compiled and kernel counters disabled; separate work and requested-allocation processes explain physical costs.

The [semantic gate](R02-integrated-semantic-gate.md) supplies independent full-source replay, terminal checks and adverse partial-equality witnesses. Small workload cases additionally check the dedicated generated/generic paths against complete analytic answers. Default/counter-free tests, Clippy, formatting and independent review are recorded. The reference implementation is unchanged.

## Complete measured lifecycle

Five-process medians in milliseconds at base size 64:

| Workload / depth | Integrated | Dedicated generic indexed | Dedicated generated indexed | Generated active scan | Generated global scan |
|---|---:|---:|---:|---:|---:|
| Independent / 1 | 2.803 | 2.667 | 2.555 | 2.892 | 1.781 |
| Independent / 8 | 3.273 | 3.767 | 3.665 | 3.392 | 2.397 |
| Shared fanout / 1 | 1.287 | 1.856 | 2.101 | 2.261 | 1.476 |
| Shared fanout / 8 | 1.566 | 2.504 | 2.560 | 2.691 | 1.792 |
| Zero-yield repair / 1 | 0.765 | 1.180 | 1.137 | 1.759 | 0.359 |
| Zero-yield repair / 8 | 0.942 | 1.525 | 1.531 | 1.999 | 0.550 |
| Recursive build | 0.901 | 1.381 | 1.383 | 0.743 | 0.483 |

Shared fanout's integrated lifecycle is approximately 13% lower than generated global scan at both depths, with separated five-process ranges. At size 8, depth 1, that direction reverses: integrated fanout costs 0.218 ms against 0.176 ms for global scan. No universal crossover or workload weighting follows from two sizes.

The independent-binding, zero-yield and build size-64 losses against global scan have separated timing ranges. A comparison only against indexed active controls would miss this contrary evidence. Source scheduling is part of the measured organizations: global rule selection processes the bind rule before joins, whereas integrated activation services queued occurrences and consistency work. These are not matched-trace representation ablations.

For size-64 depth-1 fanout, integrated execution averages 176.5 microseconds per query versus 296.0 for generated global scan. Setup rises from 35.5 to 99.7 microseconds, diluting the execution benefit. For independent bindings, integrated execution itself is slower (500.4 versus 354.7 microseconds), in addition to more expensive setup. Source applications agree across controls; implementation-specific candidate counters have different units and must not be compared as equal operations.

## Allocation and necessary machinery

At size-64 depth-1 fanout, integrated execution requests 4,560 allocations versus 9,054 for generated global scan, but peak live requested heap across the four queries is 217,302 versus 92,721 bytes. Fewer allocation requests do not imply lower retention. Build peaks are 176,660 versus 55,230 bytes; independent bindings 259,924 versus 101,579; repair 160,550 versus 83,033. These peaks include their prepared/report/source baselines and the n+1 queries. They are allocator requests, not RSS or memory scalability results.

Every allocation process returns to equal requested live bytes after each query's answer is disposed; prepared disposal is separately recorded. This is no-growth across four queries, not proof of zero one-time retention. Integrated state retains class nodes, descriptors, occurrence slots, canonical constructor keys, argument columns, child-parent incidence and source activation state. Dedicated global scan avoids argument indexes and binding watchers in these controls; its term arena and binding map have different ownership and retention.

Source inspection also identifies argument-column and incidence maintenance for output-only integrated residuals. Its share of the heap/time gap is not isolated, and it is not required by the source semantics. A future ablation must measure that contribution rather than attribute the entire difference to integration.

## Next decision and bounded disposition

Select a constructor-decomposing binding workload with nested parent descriptors, plus the output-only maintenance ablation. The current independent/fanout fixtures bind a variable directly to an already known constructor; increasing depth does not create descriptor repairs or pending child equations. Consequently this pilot cannot decide whether interleaving equality and source effects earns its machinery under the mechanism R02 was designed to investigate.

That follow-up has higher immediate information value than starting the strongest separate alternative, R04's inference-matched finite solver comparison: it exercises missing decision-critical work in an existing independently checked candidate and isolates a concrete retention confound with bounded implementation effort. R04 remains independently eligible. R03 search organizations and R05 guard choices are still open; none is rejected by this ordinary no-OR pilot.

Retain both integrated and dedicated candidates. Integrated fanout has a measured advantage with an explicit memory cost; cheap scanning remains a positive control and is stronger in the other displayed regimes. No production representation, native-compilation policy, workload weights or language restrictions are adopted. The overall goal remains active.

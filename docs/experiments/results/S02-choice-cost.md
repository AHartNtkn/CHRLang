# Local graph search remains competitive; contextual search changes the comparison

The local graph is faster than compiled scanning across much of this bounded pilot, including preparation and disposal. It does not dominate contextual search. The larger late-failure cases expose a consequential service and allocation cost that deterministic graph-scan measurements did not answer.

## The comparison and its scope

Four existing paths execute the same guarded consuming sources: local copied graphs, compiled Global Scan/Indexed, and contextual search. A countdown performs zero or 16 consuming steps, independently of choice depth zero/three and zero/eight inert facts. Early readiness permits consumption before the countdown finishes; late readiness waits for it. Failed alternatives fail before or after posting branch work, before further rule execution.

The matrix also crosses one/eight queries per preparation and immediate/all-answer retention. Changing queries vary namespaces, payload values and caller/result aliases. Each of the 128 scenarios runs on CPUs 0 and 8 in five randomized confirming blocks. One-batch exploratory sizing fixes 3–1,184 batches per engine/scenario before confirmation. Batching targets 10 ms of measured phases; validation is outside those intervals.

All 5,120 primary processes pass complete-answer checks against the independent scalar evaluator. Another 1,024 allocation runs have identical paired phase readings and restore final ownership. The 512 sizing processes and 40 cancellation processes are preserved separately. No process reaches its registered resource or service limit. Build records show compiled and persistent metrics disabled; primary uses the ordinary allocator.

## What the timing pilot says

The table uses the registered descriptive range test: a lower signal requires the candidate's slowest observed sample to be below 90% of the control's fastest. A higher signal requires its fastest sample to exceed 110% of the control's slowest. The five-block ranges are not confidence intervals; placement, frequency and shared-host effects can remain consequential.

| Local graph relative to | Lower range | Overlapping ranges | Higher range |
|---|---:|---:|---:|
| Compiled Scan | 195 | 61 | 0 |
| Compiled Indexed | 242 | 14 | 0 |
| Contextual search | 22 | 228 | 6 |

There are 256 scenario/CPU comparisons per row. These counts carry no workload weights. Overlap does not establish equivalence, and the pilot does not justify an adaptive policy or universal architecture choice.

Two CPU-0 examples show the practical distinction:

| Source | Local graph | Scan | Indexed | Contextual |
|---|---:|---:|---:|---:|
| Early readiness, no alternatives/payload/countdown, one query, immediate release | 4.92 μs | 11.22 μs | 14.98 μs | 8.24 μs |
| Late failed sibling, depth three, eight inert facts, no countdown, one query, immediate release | 125.21 μs | 166.51 μs | 201.70 μs | 102.24 μs |

These are medians of full measured session totals. The first case favors the local graph against all controls under the range criterion. The second favors it against compiled paths but favors contextual search against the local graph. The latter direction appears on both CPUs with both retention policies for the one-query case.

## Investigating the adverse case

Service with answer construction explains most of the second example's local/contextual gap. On CPU 0 its medians are 101.00 μs locally and 78.20 μs contextually; setup is 13.63 and 14.76 μs. Independent phase medians need not sum to the median total. Preparation alone therefore cannot explain this loss.

Allocation diagnostics support a representation-cost hypothesis. The same local service requests 245,199 bytes, compared with 134,734 contextually. Its peak live bytes above the service entry are 141,114 versus 35,599. These are requested-heap measurements, not RSS. Adding 16 countdown steps changes service requests to 507,439 versus 515,662, while the CPU-0 timing comparison overlaps. Allocation traffic alone therefore cannot predict the runtime winner.

Source inspection identifies a real implementation distinction. Local search clones branch execution, including graph nodes, targets, occurrences, history and pending goals. Contextual search clones its branch while sharing its constructor arena and using copy-on-write state maps. Both retain source obligations, but they pay different copying and lifetime costs. The measurements locate the adverse cost in service; they do not isolate how much comes from copying versus matching, equality or extraction. A shared immutable constructor representation or demand-driven expansion is a credible further contrast, not an established repair or an intrinsic lower bound against local rewriting.

The local organization remains viable for broader comparisons. It must carry this adverse regime and the contextual competitor forward. Further replication could refine their crossover, but cannot now justify selecting either complete architecture: their sustained ownership and broader source/mechanism obligations remain open. The breadth review selects a distinct missing comparison while retaining causal attribution as required follow-through.

## Lifecycle accounting and limitations

The primary total includes preparation, changing-query setup, service with answer construction, search disposal, consumer transfer/release, preparation disposal and final retained-answer release. First-service and first-query answer times are recorded separately. Source/query/oracle fixtures, preallocated harness capacity, parsing, process startup and compilation are outside these totals. This is a prepared-engine lifecycle comparison, not complete architectural lifecycle superiority.

The cancellation witnesses deliver a finite answer beside continuing work, advance another 32 nonterminal turns, then measure disposal. All pass. These separate cancellation measurements are cold-path, unlike the warmed finite-query matrix; this is a limitation relative to the registration's general warm-up wording. Their schedulers also reach unequal internal work at the specified boundary. They establish executed cancellation and raw disposal observations, not comparable cancellation-performance rankings. No architectural selection depends on those timings.

The allocation build rejects the cancellation timing command and multi-batch timing use. Final live-byte restoration holds for the extended finite sources. Neither that endpoint nor a short CPU measurement settles indefinite service, retained-all streams, arena reclamation or consumer-independent RSS.

## Next investigation

This is the fourth package after the normal/neutral breadth review: choice admission, positive guards, ownership, and costs. The [full breadth review](S02-choice-cost-breadth-review.md) selects joint structural theory/source integration under T076. Local graph copying attribution, contextual deductions, CHR-expressed merging, sustained ownership and broader complete-path comparisons remain required under their existing tasks. The research goal remains active.

## Evidence

[Prospective registration](../registrations/S02-choice-cost.md), [harness](../../../research/chr-relational/tests/choice_cost.rs), [runner](../../../research/chr-relational/experiments/choice_cost.py), [independent reconstruction](../../../research/chr-relational/experiments/analyze_choice_cost.py), [audit](s02-choice-cost/audit.json), [all contrasts and phase medians](s02-choice-cost/contrasts.json), [frozen artifacts](s02-choice-cost/freeze.json), [frozen sizes](s02-choice-cost/sizes.json), [schedule](s02-choice-cost/schedule.json) and [raw receipts](s02-choice-cost/). Scoped Clippy passes in both builds. No engine or reference implementation changes were needed.

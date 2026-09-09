# Native compilation repays its cost with sufficient measured reuse

Native execution recovers compilation on both tested sources when preparation is reused long enough. At 16,384 chain queries and 8,192 payload queries, compilation-inclusive costs meet the registered practical-gain criterion. Both sources still lose at 1,024 queries.

This directly resolves the earlier pilot's unmeasured ordinary-link amortization question within these regimes. It does not select a universal compilation policy or establish a minimal production compiler cost.

## Observed total costs

Ratios below compare native with prepared generic execution. The compilation-inclusive ratio charges native source emission, compilation and artifact filesystem disposal once; the generic executable is reusable across rulesets. Each ratio is the median of seven paired blocks using the median of five native artifact-cost observations.

| Source | Queries | Runtime ratio | Compilation-inclusive ratio | Registered result |
|---|---:|---:|---:|---|
| Chain, 32 edges | 1,024 | 0.887 | 1.140 | Practical loss |
| Chain, 32 edges | 8,192 | 0.878 | 0.910 | Compilation recovered; gain below practical threshold |
| Chain, 32 edges | 16,384 | 0.881 | 0.897 | Practical gain |
| Payload, 64 variables | 1,024 | 0.695 | 2.007 | Practical loss |
| Payload, 64 variables | 8,192 | 0.710 | 0.872 | Practical gain |

A practical gain requires a median ratio at most 0.90 and all seven ratios below one. At 16,384 chain queries the compilation-inclusive ratios range from 0.894 to 0.913; at 8,192 payload queries they range from 0.862 to 0.888. These are descriptive repeated-run ranges, not confidence intervals.

For all three larger-reuse cases, even the highest observed native runtime plus the highest artifact cost is below the lowest observed prepared runtime. Thus recovery is observed across the recorded ranges, including chain's 8,192-query case where the benefit does not meet the 10% practical threshold.

Median native artifact costs are 0.177 seconds for chain and 0.162 seconds for payload. At the largest query counts, median measured runtime sums are 11.149 versus 9.817 seconds for prepared/native chain, and 0.979 versus 0.697 seconds for prepared/native payload. The [analysis](s01-native-amortization-cached-oracle/analysis.json) retains every repetition and allocation reading.

## The process cutoff belonged to the harness

The initial 16,384-query chain warmup reached the 60-second process limit after 11,100 recorded validated queries. Their runtime intervals summed to 6.961 seconds. An isolated 1,024-query diagnostic attributed 4.474 seconds to independent oracle construction, 0.0066 seconds to answer comparison, and 0.692 seconds to setup/execution/observation. This identifies repeated oracle construction as the dominant excluded cost; it is not evidence against native execution.

Chain and payload alternate between two exact source-query shapes. The corrected harness constructs those independent expected answers once, requires full structural Query equality before reusing either, and still compares every produced answer. Unexpected inputs are rejected. Other source families retain their own oracle construction. This is harness fixture reuse, not language/runtime memoization.

The [prospective amendment](../registrations/S01-native-amortization.md) preserves all source work, query counts, bounds and interpretation criteria. The entire matrix restarted in a fresh result/target directory; no measured block from the initial attempt existed to pool. The [initial receipt and source snapshot](s01-native-amortization/source-freeze.json) and [attribution run](s01-native-amortization/diagnosis-run.json) preserve the cutoff evidence.

Reusing oracle fixtures changes cache and allocator context. Conclusions therefore use only matched corrected runs, not ratios across the two harness versions or a claim of unchanged runtime distributions.

## Sustained ownership checks

All 80 ordinary and 20 diagnostic processes complete under the original process/resource limits, performing 696,320 independent complete-answer checks. All ten diagnostic configurations replay exactly and restore query/prepared live-byte baselines.

Cumulative requested traffic grows with work while recorded peaks remain effectively constant across reuse counts. For 16,384 chain queries, included traffic is 9,321,805,779 bytes for prepared execution and 5,824,673,733 for native; absolute requested-live peaks are 162,510 and 162,328 bytes. For 8,192 payload queries, traffic is 1,271,970,104 versus 945,043,165 bytes, with peaks 80,236 and 80,054 bytes.

Those peaks include the retained independent oracle fixtures and process state. They are not engine-owned memory or RSS. The result concerns sequential queries that release answers; it does not establish arbitrary cross-query cache, retained-output or ongoing-stream behavior. Runtime phase sums exclude oracle/input construction, validation, stdout and process startup/exit. Artifact disposal measures filesystem syscall cost, not storage synchronization.

The [validation receipt](s01-native-amortization-cached-oracle/validation.json), [source freeze](s01-native-amortization-cached-oracle/source-freeze.json) and [run summary](s01-native-amortization-cached-oracle/summary.json) preserve the corrected evidence. Ordinary and diagnostic emitter/library Clippy checks pass. Source execution and the independent oracle implementation are unchanged.

## Architectural consequence and next investigation

The bounded T070 comparison now has both a measured low-reuse compilation loss and measured high-reuse recovery. It is no longer defensible to reject source-native execution because compilation failed to amortize in the initial small-query pilot. Nor does long reuse make native universally preferable: dispatch had adverse runtime cases, and dedicated subscription controls remained stronger within their supported source contract.

Prepared data plans, native execution and source-specialized execution retain different responsibilities and applicability. These measured regimes support carrying them into later coherent-architecture comparisons. ThinLTO at its larger modeled crossover, broader access plans, held-out source behavior and other lifetime policies remain open under S01/S06/S08/S11.

T070's bounded generated/prepared/retained cost investigation is complete. The next active investigation is [direct demand-driven execution and derivation reuse](S03-demand-execution-entry.md), T071. It can change search organization rather than tune another access policy, and already has independent source oracles and corrected direct-graph controls. The broader architecture goal remains active.

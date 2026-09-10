# Broader deduction reuse lowers some retained peaks but increases allocation work

Relevant-key deduction reuse passes finite ownership and cancellation checks. It lowers peak requested heap against exact-state ordered caching in 38 of 64 scenarios, but requests more bytes than uncached contextual execution in every scenario. The result is a measured space/traffic tradeoff, not a runtime or architecture ranking.

## What the comparison includes

The [registration](../registrations/S02-relevant-ownership.md) covers seven paths: uncached contextual execution, ordered exact-state reuse, ordered relevant-key reuse, persistent exact-state reuse, persistent relevant-key reuse, compiled Scan and exact-schema elimination. Four source families distinguish one use, shared inputs, distinct inputs and shared inputs after changed caller bindings.

Depths 0/16, one/four changed queries, token absent/present and immediate/retained-all answers give 64 scenarios per mode. The four-query sessions alternate depth and query order while reusing preparation. Every complete answer is checked independently. The exact-schema control verifies eligibility; it is not a general compiler.

All **1,008 allocation processes and 448 ordinary-allocator semantic replays pass**. This includes two allocation repetitions for each of 448 complete cells, plus two repetitions for each of 56 cancellation cells. Every allocation tuple repeats exactly. Ordinary runs reproduce the semantic metadata and independently check their answers; their clocks are not comparative timing evidence.

## What the owners do

Retained batches remain valid after their engine and preparation are disposed. Final consumer disposal restores requested live heap to the initial baseline in every metered run. Immediate-release sessions also restore the per-query baseline. Cancellation interrupts even-numbered queries after one service turn; subsequent odd-numbered queries complete using the same preparation.

The runner separately accounts for source construction, preparation, input construction, query setup, execution with owned observation, engine disposal, answer holding/release, preparation disposal and final consumer disposal. First observation is timed inside execution; it is not an additional phase to sum. Cancellation occurs at unequal engine work, so its costs are not ranked as equivalent completed computation.

Fixture construction, independent validation and fixed reporting/held-batch slot reservation precede the measurement baseline. Allocation totals below sum the nonoverlapping measured phases. Peak excess is the maximum measured-phase live heap above that baseline. Validation clones are excluded, and neither requested bytes nor these peaks are RSS. Parsing arbitrary source and native compilation are not measured.

## Favorable and adverse allocation regimes

Each row compares the same 64 scenarios. “Lower” is an exact requested-allocation comparison, not statistical evidence about runtime.

| Relevant-key candidate / control | Traffic lower / equal / higher | Peak lower / equal / higher |
|---|---:|---:|
| Ordered relevant / uncached contextual | 0 / 0 / 64 | 0 / 0 / 64 |
| Ordered relevant / ordered exact-state | 12 / 0 / 52 | 38 / 0 / 26 |
| Persistent relevant / persistent exact-state | 0 / 0 / 64 | 16 / 0 / 48 |
| Ordered relevant / compiled Scan | 24 / 0 / 40 | 32 / 0 / 32 |
| Ordered relevant / exact-schema elimination | 0 / 0 / 64 | 0 / 0 / 64 |

All 12 traffic reductions against ordered exact-state caching occur in the changed-binding family: depth 16 at either query count, and depth 0 with four changed queries. This is the regime the broader validity key was intended to address. Single-use and distinct-input overhead remain important contrary cases.

The following witness uses changed bindings, depth 16, four queries, a token and retained answers. Bytes are exact measured requested values.

| Path | Requested bytes | Peak excess bytes |
|---|---:|---:|
| Uncached contextual | 669,196 | 38,976 |
| Ordered exact-state reuse | 1,311,396 | 208,960 |
| Ordered relevant-key reuse | 1,292,590 | 72,959 |
| Persistent exact-state reuse | 1,091,764 | 115,952 |
| Persistent relevant-key reuse | 1,667,574 | 72,308 |
| Compiled Scan | 265,971 | 52,934 |
| Exact-schema elimination | 43,562 | 9,944 |

The new key avoids retaining whole successor equality maps, which is consistent with its lower peak than either exact-state cache in this witness. It nevertheless allocates more than recomputation. Persistence further illustrates the tradeoff: its relevant-key peak is slightly smaller here, but its traffic is substantially larger. This attribution is a representation-based explanation, not an isolated measurement of each cause.

## The consequential cost is in execution, but its components remain unresolved

In the witness, contextual, ordered exact-state and ordered relevant-key paths each request 3,889 preparation bytes and 30,900 setup bytes across the four queries. Their execution/observation traffic is respectively 619,332, 1,261,532 and 1,242,726 bytes. Preparation does not explain this contrast.

Relevant-key execution constructs transitive keys, retains read descriptions and applies local parent/descriptor updates on hits. Exact-state reuse instead retains and adopts map snapshots. Observation and source discovery also occur within the measured execution phase. The current evidence cannot assign the traffic difference among those operations merely by inspecting their names or counting hits.

The owned outputs are semantically equivalent, but their spare vector capacities need not be identical. The exact-schema path retains a different byte count of consumer storage in the witness. Those implementation costs are included; no physical identity or equal-capacity requirement is imposed on the controls.

## Next decision

Select a bounded diagnostic attribution of key construction, cache retention and local replay before selecting a repair or claiming an unfavorable architectural balance. The measured execution traffic is large enough to change a fair comparison, while the lower retained peak leaves a plausible benefit. Then qualify clock resolution and prospectively register ordinary-allocator lifecycle timing; allocation alone cannot decide speed.

The strongest ready alternative is dependency/miss-reuse lifecycle measurement under T071. Symbolic compiled-formula reuse and union/inclusion remain distinct required investigations under T076. Reconsider them at the attribution gate or an obstruction. This is package one after the relevant-deduction breadth review; another repair counts as another package and full breadth review remains due within four.

T072 remains active. Wider source coverage, long-running retention, cap saturation, compilation costs, coherent architectures and language tradeoffs remain unresolved. No production cache policy or architecture is selected.

## Validation and receipts

Both harness tests pass across all eleven supported runner modes, including source rejection and interruption followed by preparation reuse. The measured seven-mode matrix additionally qualifies persistent relevant-key source execution and retained outputs. Scoped Clippy passes with warnings denied. Cost executables reject engine/kernel, deduction and local work counters. Reference and independent scalar implementations are unchanged.

[Registration](../registrations/S02-relevant-ownership.md) · [runner](../../../research/chr-relational/examples/s02_deduction.rs) · [experiment driver](../../../research/chr-relational/experiments/relevant_ownership.py) · [freeze](s02-relevant-ownership/freeze.json) · [all results and phases](s02-relevant-ownership/results.json) · [audit](s02-relevant-ownership/audit.json) · [meter check](s02-relevant-ownership/meter-check.log) · [harness tests](s02-relevant-ownership/harness-tests.log) · [Clippy](s02-relevant-ownership/clippy.log).

The runs have 60-second wall/CPU, 1-GiB address-space and 2,000,000 source-turn bounds. There are no cutoffs. The driver refuses to overwrite its receipt directory. Earlier runner/executor bytes are preserved in `before/`, and new builds use separate target directories.

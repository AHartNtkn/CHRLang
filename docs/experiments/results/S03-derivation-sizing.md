# Fresh derivation reuse: construction cost needs a direct test

Fresh derivation reuse has promising repeated-call timings, but construction overhead can dominate single and distinct requests. The next experiment must isolate that overhead before confirming a ranking. These are exploratory observations, not an architecture selection.

## What was measured

The [prospective registration](../registrations/S03-derivation-lifecycle.md) defines five source families and six controls. All **720 exploratory timing processes, 480 allocation processes and 24 cancellation processes** completed successfully. The 240 paired allocation configurations replay exactly; query and prepared disposal restored their initial live requested allocation. Frozen source and binary hashes were checked against the [raw receipts](s03-derivation-sizing/freeze.json).

The runner's two tests pass: complete source/control agreement across 360 configurations, including reuse after cancellation, and rejection of changed source/query inputs by the exact control. Counter-free runner Clippy passes. Complete answers are checked against the independent scalar evaluator outside primary intervals, including joint fresh-variable identities and raw choice multiplicity.

## What the sizing suggests

The table shows one exploratory timing per configuration, in milliseconds. Each process prepares once and runs eight changing queries, with resources present and forward starting order. Depth alternates 32/33, or 12/13 for the growing accumulator. Primary totals include preparation, setup, execution with observation, and engine, answer and prepared disposal.

| Source | Ordinary demand | Fresh templates | Compiled scan | Inferred specialization | Exact source control |
|---|---:|---:|---:|---:|---:|
| One call | 0.480 | 1.012 | 0.409 | 0.302 | 0.061 |
| Four equal-input calls | 1.945 | 1.107 | 1.508 | 1.144 | 0.177 |
| Four distinct-input calls | 1.789 | 3.752 | 1.375 | 1.264 | 0.146 |
| Four calls with independent choices | 14.688 | 2.833 | 4.044 | 2.775 | 0.241 |
| Doubling accumulator | 11.121 | 25.363 | 6.887 | 7.051 | 5.984 |

The [machine-readable summary](S03-derivation-sizing-summary.json) includes indexed execution too. These selected rows expose the question; the full matrix includes short work, resource absence and reversed order. Single timings cannot establish practical gains or losses.

**Repeated applications may repay specialization, while new requests pay its construction cost.** A query owns its templates; eight changing queries do not reuse one template cache. The repeated-call families exercise reuse within each query. The exact source control illustrates work that can be eliminated on this schema; it is not a general recursive compiler.

**The compiler currently copies ground terms recursively when binding and substituting arguments.** This is visible in `Budget::copy`, `matches` and `term` in the frozen template source. Repeated countdown suffixes and duplicated accumulator subtrees therefore incur construction work that a shared internal representation could avoid. This is a concrete implementation hypothesis, not yet a measured attribution of the timing differences. Instantiation and complete output construction may remain material afterward.

The subsequent [paired attribution](S03-shared-template-attribution.md) now tests this copying hypothesis. The sizing observations above retain their original implementation scope.

## Next experiment and its limits

First compare the current owned-tree compiler with a shared immutable representation of its internal terms. Establish that repeated ground subterms are shared, while each template application still creates independent unknowns and dynamic choices. Preserve actual resource claims, off-output obligations, bounded continuation and the recorded scheduling distinction. Sharing immutable terms must not share a source effect or a fresh logical identity.

Then register a paired attribution run using the frozen current binaries and the repaired implementation, with single, repeated, distinct and growing sources, short and substantive work, resources absent/present, and both orders. Measure ordinary-allocator lifecycle separately from exact allocation replays. Inspect construction, instantiation and output obligations before assigning the remaining cost to derivation reuse. Exact repetitions and practical thresholds must be frozen before those runs; none are inferred from this sizing table.

This repair is selected ahead of T072's broader integration and T073's reusable lowering artifacts because the suspected copying can distort the immediate comparison of fresh derivations. It is a bounded attribution package, not permission for indefinite graph refinement. After it, compare the value of confirmation with those two distinct alternatives and record the next selection. Unknown-input derivations, richer effectful recursion and sustained template lifetime remain required investigations.

Native compilation is outside these intervals. Requested allocation is not RSS. The output-heavy growing family charges full owned answers; a compact output contract is a separate language/observation comparison. No timing ratio is used for the contested-resource source whose committed outcome changes under contraction. The architecture goal remains open.

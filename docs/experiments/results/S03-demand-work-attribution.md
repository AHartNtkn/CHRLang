# Opaque work was repeated because the result key included unrelated choices

**The suspended evaluator now retains one opaque box chain across choices, matching the direct graph on the registered sources.** Before the correction, three choices and depth 32 produced 249 retained boxes and 257 work-result records; afterward they produce 32 boxes and 33 records. This fixes an avoidable implementation cost before timing the candidate. It does not establish a lifecycle advantage.

## What the comparison measured

The [registration](../registrations/S03-demand-work-attribution.md) fixed 80 cases: no choice, opaque work or prior discrimination; zero, one or three choices as applicable; depths 0, 1, 8 and 32; both query insertion orders; and a consuming endpoint present or absent. Every complete answer is checked against hand expectations and the independent scalar source oracle. The direct graph and both demand policies agree on the registered answers.

Diagnostics inspect retained state after execution. They count internal `box` nodes and the suspended evaluator's recorded `work` expansions. They add no hot-path counters. They exclude the owned terms created for complete answer observation and are not interchangeable counts of all engine operations.

| Source at depth 32, three choices | Direct graph boxes | Original demand boxes / work results | Corrected demand boxes / work results |
|---|---:|---:|---:|
| Choice tuple carried opaquely | 32 | 249 / 257 | 32 / 33 |
| Full tuple discrimination before work | 256 | 256 / 264 | 256 / 264 |

The result is unchanged by the two insertion orders or the consuming endpoint in this matrix. No-choice depth 32 retains 32 boxes and 33 demand work results under either policy. Across all 80 cases, 24 have fewer demand boxes or work-result records after correction; the remaining 56 have unchanged counts. Corrected box counts match the direct graph in every case.

Both original executions agree exactly. Both paired executions agree exactly, and the explicit current-context control reproduces the original 80 rows. See the [original runs](s03-demand-work/original-1.log), [replay](s03-demand-work/original-2.log), [paired runs](s03-demand-work/paired-1.log), [paired replay](s03-demand-work/paired-2.log), and [analysis](s03-demand-work/analysis.json). The [original source snapshot](s03-demand-work/original-source/demand.rs) preserves the implementation used before correction.

## Why the correction is valid within its certificate

The original cache attached every application result to the entire current choice context. If a call was first expanded after unrelated choices split, sibling tasks repeated that expansion even when its matched inputs did not inspect any of those choices.

The compiler now computes a sufficient certificate for deterministic, transitively effect-free call definitions. A qualifying match must read only ordinary immutable constructor nodes; variable patterns may capture opaque graph pointers. It cannot obtain its matched constructor through a call, alias or choice. The resulting expansion may then be recorded in that **same call's birth context**. Its children retain that activation scope.

This is not a cache keyed by equal input values. Each distinct source call still instantiates its own fresh locals. Captured pointers still resolve in the observation's choice context. Calls created inside a selected branch cannot acquire a wider birth context. Definitions containing choices, resource partners, resource posts or calls to such definitions are excluded transitively. Other applications retain current-context validity.

These premises justify reuse of the pure application expansion; they do not prove identical scheduler traces for arbitrary nonconfluent source programs. Changes in available work can affect which consumer a committed scheduler services first. The paired cases therefore require complete-answer agreement, and the separate nonconfluent policy probe remains outside equivalent timing controls.

The previous source/resource suite passes 22 tests with default features and with package-default metrics disabled. Its finite complete-answer cases exercise both reuse policies. The 17 existing source/identity tests also pass. The opaque-work expectation failed before the correction; [that failure](s03-demand-work/red.log) is retained. [Clippy](s03-demand-work/clippy.log) passes with warnings denied. Reference semantics are unchanged.

## What this means for the architectural comparison

A timing of the original conservative policy would have charged the candidate for avoidable repeated construction. The corrected policy is now the default experimental candidate; the original policy remains an explicit attribution control. The compiler's purity analysis, validity check and retained metadata are costs to measure, not free improvements.

Matching box counts does not mean equal execution costs. Source discovery, context lookup, resource claims, complete observation and disposal remain different. All raw answers still require their full output shape, and this source admits a direct lowering that constructs that shape without interpreting each recursive rule. That lowering must be a control in lifecycle sizing.

Proceed to the selected bounded lifecycle sizing with ordinary-allocator, counter-free execution, changed queries and separate allocation diagnostics. No timing matrix has run for this candidate. At that boundary, compare confirmation value with contextual/local consuming rewrites again. Local pull-tab transformations, general dependency projection, cross-application derivation templates, broader resource aliases and sustained retention remain unresolved. T071 and the overall goal remain active.

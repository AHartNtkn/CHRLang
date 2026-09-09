# Work attribution before demand lifecycle timings

Test whether whole-context application-result keys cause opaque work to be repeated after unrelated choices. This is a structural diagnostic, not a timing comparison. Read retained nodes and application-result records only after execution; add no per-operation counters to the primary path.

Freeze a source family with binary independent `choose` calls, a `work` recursion that constructs one `box` per depth while carrying its argument opaquely, and an optional consuming `take` with one token. An adverse `gate` variant inspects the complete choice tuple before starting work. The no-choice case uses an empty tuple. Outputs repeat the result twice; complete observations retain required output construction costs.

Matrix: plain with 0 choices; opaque and discriminating with 1 or 3 independent choices; depths 0, 1, 8, 32; work before/after chooser constraints; with/without the consuming endpoint. This is 80 cases. Run twice. Compare hand-derived complete answers, scalar semantics and the direct graph with the suspended evaluator. Bound each engine at 100,000 ticks, scalar at 200,000 steps, each test process at 60 seconds.

Record retained `box` constructor nodes in both representations and demand application-result entries for `work`. These count retained internal construction, not complete query cost or logical answer volume. The representations currently retain all such nodes for the query. Do not interpret smaller counts as a timing win, and do not compare counts of unrelated node kinds as equal-cost operations.

Hypotheses: the direct graph can retain one opaque box chain across source choices; complete discrimination should prevent that sharing. Conservative demand keys may repeat the same source-call expansion in sibling contexts even though its matched constructor inputs did not depend on those choices. Query order may change how much work precedes the first split. No-choice cases control the arithmetic and constructor definition.

If the result identifies that avoidable duplication, permit one bounded correction: reuse a deterministic, transitively effect-free call's expansion in its birth context only when every matched constructor is already an ordinary immutable constructor node and the other patterns merely capture variables. Preserve fresh identity per call. Dynamic constructor demand and resource applications keep their current-context validity. Compare the original key policy and corrected policy on this same prospective matrix, and rerun all source/resource gates before claiming correctness. This is a deliberately sufficient certificate, not a general dependency projection.

The box-building source has a direct lowering that constructs the required shape; it cannot select sharing architecture by itself. Lifecycle sizing and an applicable direct-lowering control remain required after attribution. Record the selection boundary with contextual/local rewrites before further extensions.

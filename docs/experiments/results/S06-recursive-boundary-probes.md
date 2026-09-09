# Recursive lowering must preserve when effects become available

**Collapsing recursive work can change the winning consumer even when final bindings agree.** Ground inputs alone do not make contraction safe when completion posts resource claims. Initially unknown inputs also require an explicit suspension and reactivation boundary.

These [registered semantic probes](../registrations/S06-recursive-boundary-probes.md) constrain the next source-derived compiler. They test particular eager transformations, not the impossibility of recursive lowering. No performance measurements or new compiler implementation are claimed.

## Two counterexamples that change the design

| Source situation | Ordinary source result | Eager transformation result | What the compiler must preserve |
|---|---|---|---|
| Two ground recursive calls, depths two and one; each base case posts a claim for one shared token | The second call finishes first; **b wins**, leaving a's claim | Replacing both inputs by their base constructor makes the first call finish first; **a wins**, leaving b's claim | Observable completion order when base cases emit work that competes for resources |
| A pure recursive call has an unknown input tail; another call finishes while a later rule establishes that input | **b wins** before the later binding makes a eligible | Making the eventual constructor available at entry lets **a win** | The point at which input information becomes available, including opportunities for intervening consumers |

Both counterexamples use the current Global source-order policy. The independent scalar evaluator and compiled execution with scanning and indexed access agree on each side. These are different permitted source executions after a specific transformation, not measurement noise.

The late-binding example produces the same final output bindings in both executions: x=a and y=b. Full residuals distinguish them: one contains won(b) and ready(a), the other won(a) and ready(b). An output-only validation would miss the semantic change.

The ground example is a boundary for effectful completion, not merely for effectful recursive steps. Its recursive step only removes one constructor; the base case posts the claim. Groundness and structurally decreasing recursion therefore do not establish that completion may be reordered.

## Favorable and suspension controls

When the recursive base case only establishes an equation, the bounded ground comparisons agree. Fifty combinations of depths zero through four and initial occurrence order preserve complete outputs and residuals after collapsing known recursive inputs. Higher-priority private recursion completes before the consuming suffix in these sources. This supports investigating a pure fragment, but is not a general soundness theorem for arbitrary equations, aliases, choices or rule ordering.

An unknown recursive tail remains suspended without inventing its constructor. Across depths zero through four, the final observation retains an unknown input, a distinct unknown output, a residual recursive occurrence, ready(output), and the unused token. Quiescent residual work is not failed execution.

When a later binding rule is present, the same suspended computation resumes, binds the output and completes the consuming application. The input becomes z, the output a, and the sole residual is won(). A compiler accepting unknown inputs must preserve this behavior; rejecting all unknown inputs is instead an explicit eligible-fragment restriction and cannot answer the broader language question.

## Evidence and interpretation

All four probe tests pass with default features and counters disabled. Scoped strict Clippy passes. Each finite source execution is bounded at 10,000 service ticks and must exhaust. The tests compare full raw answer collections against the independent evaluator; the reference implementation is unchanged. See [test source](../../../research/chr-compiled/tests/recursive_boundary.rs) and [validation receipt](s06-recursive-boundary-probes/validation.json).

The eager transformations are constructed explicitly in the tests. They are semantic witnesses, not source-general compilers or performance controls. The observed differences rule out those transformations under the tested source contract. They do not reject a compiler that preserves relevant scheduling or represents the dependency directly.

## Consequence for the next implementation

Investigate a source-derived recursive plan with a resumable boundary at unavailable constructor information. Derive its recurrence and applicability from source, separate pure local work from observable completion effects, and preserve occurrence ownership and source competition. A ground-input specialization can be a competing eligible control, but must not replace the unknown-input investigation.

Before timing, require actual source-derived execution, independent full-answer agreement, fresh-variable and failure checks, and finite service beside continuing work. The current probes provide adversarial cases for that gate; they do not complete it. Broader effectful/resource derivations and sustained artifact lifetime remain unresolved.

The recursive mechanism package is still in progress. This work does not reset the breadth counter: completing that package will make four packages since the latest review, at which point integration, observation and lifetime must be reconsidered against further compiler refinement. T073 and the overall research goal remain active.

# S04 matcher correction: known mismatches no longer copy their environments

The registered copy prediction is confirmed exactly. A constructor/variable-root precheck eliminates all 36,864 rejected-partner environment copies on the mutation query while preserving complete answers and source-step counts. This is a semantically gated correction; its paired lifecycle benefit and adverse overhead remain unmeasured.

## Observed work reduction

The [prospective diagnostic](../registrations/S04-matcher-copy-diagnostic.md) derived its prediction from the source and occurrence order before counting. A separate diagnostic build observes the following for both Copy and Trail on one mutation query:

| Matching-frame work | Before | After |
|---|---:|---:|
| Environment copies | 37,312 | 448 |
| Source-term nodes cloned by those copies | 1,591,872 | 19,008 |
| Environment copies for rejected partners | 36,864 | 0 |
| Term nodes cloned for rejected partners | 1,572,864 | 0 |
| Rejections performed before copying | Not instrumented | 36,864 |

The remaining 448 copies correspond to accepted partners. These are counts of environment copies and logical term nodes, not allocated bytes or a timing estimate. Other mismatch shapes can still pass the precheck and later fail full matching.

The corrected root-replay implementation also directly executes 799,623 source steps on mutation, matching the earlier projection. This confirms the count on the corrected implementation; it does not substitute a new timing for the original frozen pilot.

## Why the precheck preserves matching

Before cloning a nonempty entry environment, inspect the argument roots required by earlier bindings or by a constructor pattern. An unbound pattern variable imposes no precheck condition. Follow query-variable bindings to find an actual root, then compare free-variable identity or constructor name and arity. Do not recursively compare constructor children in the precheck; the full matcher remains responsible for them.

Every full match satisfies these root conditions. Equal fully resolved values have equal roots; successful constructor matching requires equal names and arities. Variables not bound at head entry are deliberately left unconstrained by the precheck. The state does not mutate during tuple selection, and new bindings created within a candidate cannot invalidate this one-sided argument.

Pattern-variable names are interpreted through the matching environment, not as query-variable identities. A directed check covers that distinction when a query binding happens to use the same numeric name as an unbound pattern slot.

## Validation and limits

The local necessary-condition test checks 623,295 pattern/value/environment/binding configurations, including aliases, constructors and repeated variables: every accepted full match passes the precheck. Existing independent restoration/source tests pass, as do the nine-family lifecycle gates against the independent evaluator and existing controls in ordinary and COW builds.

Two additional whole-source cases use entirely compatible partners with shallow and 64-deep keys, preserving repeated unknowns and joint output aliases. These establish correctness on an adverse shape, not its performance. Debug and release tests, strict all-feature Clippy and formatting pass; [validation receipts and source hashes](s04-matcher-copy/validation.json) record the commands.

The precheck does not reduce candidate visits and is not an index. It follows variable-binding chains, so all-compatible inputs with long aliases can add traversal without avoiding any frame copy. The paired cost comparison must include that case as well as small and large compatible values before retaining an unconditional performance claim.

The [before diagnostic](s04-matcher-copy/before-work.json) and [after diagnostic](s04-matcher-copy/after-work.json) preserve complete-answer-validated counts. The archived diagnostic sources match their recorded hashes. The original 1,008-process pilot remains tied to its measured source commit and frozen binaries; its [full report](S04-lifecycle-pilot.md) describes the pre-correction implementation.

T066 remains active for a prospectively registered paired lifecycle comparison. S05 stable-identity reuse remains required; this correction does not settle other restoration policies or the architecture choice.

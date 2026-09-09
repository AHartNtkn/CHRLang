# Source-derived nested matching needs deeper dependencies

The local rewrite prototype now derives a bounded consuming plan from source patterns and correctly wakes suspended nested matches. Independent checks include repeated variables and late aliases. Conservative dependency repair adds substantial work on broad alias merges, so the first selective activation result does not generalize without qualification.

## What the source plan supports

The [plan compiler and executor](../../../research/chr-relational/tests/support/local_ports.rs) accept a source rule of the form `take(Pattern, Result), token() <=> Result = Captured`, where `Captured` occurs in the pattern and `Result` does not. Patterns may contain arbitrary constructor names/arities and repeated variables. Body effects outside this capture equation, kept heads, guards, other resource shapes and fresh uncaptured results are rejected explicitly.

The consuming operation now uses the checked plan instead of assuming an outer `f` and returning its child. Each request keeps its source-derived pattern and capture variable. This remains a bounded source fragment, not a general CHR compiler: general multihead matching, propagation history, arbitrary bodies and rule competition still need implementation and correspondence evidence.

Matching traverses known descriptors without binding unknown inputs. Repeated variables require established structural equality, including existing aliases; two independently unknown values cannot be equated merely to make a match succeed. The capture remains a repairable handle, and the consuming body posts an ordinary local equation.

A suspended request subscribes to the nodes actually inspected, including descendants and both sides of an equality check. Subscriptions use stable handles so node merging cannot strand them on retired nodes. Reinspection replaces those subscriptions, and consumption unregisters them. Source occurrence registration order still controls the ready set in this fragment.

## Discriminating evidence

| Test | Result and meaning |
|---|---|
| `f(g(X))` with only the outer `f` known | No token is consumed. Supplying the inner `g(a)` wakes the request and produces the independently expected complete answer. Watching only the outer node would miss this transition. |
| Repeated versus distinct pattern variables | 200 configurations combine two pattern shapes, 25 ordered term pairs and four late-information actions. Full results agree with independent scalar source evaluation, including unknowns, aliases, remaining requests and tokens. |
| Invalid source plans | Unsupported bodies, kept heads, resource arguments, uncaptured results and output variables occurring in the pattern are rejected. No success-shaped fallback executes them. |
| Existing constructor and effect checks | All 2,160 independent equation configurations and the existing local/CHR source, fork, consuming and scheduling checks continue to pass. |
| Broad alias control | At width 8, total request inspections are 51; at width 64, 2,207. The narrower source gate recorded 16 and 128 respectively. This is diagnostic operation evidence, not a timing comparison. |

The 200 pattern configurations use inputs drawn from `a`, `b`, an unknown, `f` of that unknown and `f` of a separate unknown. Late actions either do nothing, join the unknowns, bind the first to `a`, or bind the second to `b`. This checks both late successful equality and persistent mismatch. Expected answers come from ordinary source rules using the independent recursive-substitution evaluator.

## Why the broad control became more expensive

The general dependency mechanism conservatively reinspects watchers whenever their node participates in a merge. This is necessary for some repeated-variable matches: two unknowns becoming identical can enable a match without adding any constructor. It is unnecessary for an outer constructor test still waiting on two descriptor-free nodes.

In the broad witness, the growing alias class is revisited at every merge. The exact total is registration plus successive alias rechecks plus final activation: `2n + n(n+1)/2 - 1`. All values and individual token consumptions still validate. The independent 64-request known-input registration check remains 64 inspections, so this is distinct from the earlier registration defect.

The result identifies an implementation opportunity, not an intrinsic lower bound. A more selective subscription could distinguish waiting for a descriptor from waiting for an equality, and avoid rechecking unaffected conditions. That distinction must preserve nested and repeated-variable wakeups. A future cost comparison needs this attribution and a competent control before concluding that local integration loses.

## Validation and next selection

All 43 relational-package tests pass, including 13 combined constructor/source tests. Scoped strict Clippy passes. [Receipts](s02-local-pattern-gate/) contain the initial unimplemented-plan failure, final package tests and lint result. Commands: `cargo test -p chr-relational` and `cargo clippy -p chr-relational --test chr_constructors --no-deps -- -D warnings`. Reference-interpreter code is unchanged.

This diagnostic prototype has counters enabled, retains query-owned structures and does not measure source preparation or full lifecycles. No timing matrix or architecture ranking follows. The [four-package breadth review](S02-local-rewrite-breadth-review.md) selects call-level reuse next and keeps integrated matching, precise dependencies, complete source support and lifecycle costs required under T072.

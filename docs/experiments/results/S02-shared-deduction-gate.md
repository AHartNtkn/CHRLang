# Newly computed equality can be shared without sharing consumption

**The contextual executor can reuse equality deductions computed after a source fork while preserving separate resource effects.** The new gate demonstrates this with complete answers and unchanged service events. It establishes a mechanism and its correctness boundary; its total cost is not measured yet.

## What is different from the contextual control

The existing store shares immutable constructors but computes each context's equality updates separately. The experimental mode additionally records a completed equality transition: its successor parent/descriptor maps, generated child equations and failure result. A sibling with the same equality state and equation can adopt that result instead of recomputing the merge, reachability and constructor decomposition.

Each equality state has an arena-local, monotonically fresh identity. The key contains that identity and the ordered canonical equation inputs. It does not contain the resource store or the caller's remaining queue. A hit appends the recorded child equations to the caller's queue and leaves its occurrences, propagation history and pending source effects alone. Every genuinely new transition receives a new equality-state identity, including misses after the retention cap.

This is exact-state memoization of an equality transition, projected away from resource/history state that cannot affect that operation. It is an operational connection to S05 reuse, not evidence for arbitrary relevance projection or whole-source continuation equivalence. It also does not implement union-find through CHR or strategic port rewriting. The [registered organization comparison](../registrations/S02-shared-deduction-gate.md) keeps those distinctions explicit.

## Discriminating evidence

| Obligation | Evidence |
|---|---|
| Share new deductions, not only input nodes | Two stores fork before solving a constructor equation. After independent service, they share the resulting equality maps, while the parent retains its original state. The regression failed on the ordinary implementation and passes with transition reuse. |
| Preserve different pending work | Siblings reuse the same decomposition but retain different queued bindings. One exports `(a,a)`, the other `(a,b)`. Their subsequent equality states differ. |
| Keep consumption local | The derived outer constructor enables matching. Two duplicate ticket occurrences remain distinct; consuming an open/ticket pair in one sibling neither consumes the other's pair nor changes the parent. |
| Reject incompatible reuse | A context already binding the input differently reaches a clash rather than adopting the compatible result. Cyclic equations fail, including repeated cached failure, without poisoning siblings. |
| Preserve complete source behavior | The source suite compares enabled and ordinary contextual execution, relational execution and independent scalar answers. New cases include duplicate resources, fresh aliases, failed alternatives and raw choice multiplicity. Existing late-failure and finite-sibling cases run in both modes. |
| Demonstrate reuse during source execution | A source fork posts different `left`/`right` resources, then performs the same constructor equation and consumes its own token. Both derive `a` and publish their own tag plus `done`. The test observes shared post-fork equality maps and verifies the entire progress/answer/exhaustion event sequence matches the ordinary control. |
| Bound retention and release ownership | The cache retains 4,096 transitions. A 4,097th distinct request still completes, and a retained earlier request still hits. A weak-reference test proves cached maps do not keep the arena alive after its final store owner is released. |

The independent owned-substitution check covers 1,296 distinct ordered equation-pair configurations in each mode, repeating second requests to exercise cached results: **5,184 checked branch evaluations**. It compares joint normalized exports, success/failure and complete repeated-variable occurrence matches. Its oracle has no contextual arena, equality-state identities or memo table.

All **26 crate tests pass**, including the complete-source and ownership gates. Strict Clippy passes for this crate's library and targets with dependency linting excluded; the existing integrated build-script dead-code warnings are retained in the receipt. No reference implementation changed.

## Why the reuse key is sound within this implementation

A shared state identity denotes the same parent/descriptor maps. The arena only appends immutable nodes, so constructing later values cannot change an earlier descriptor. The ordinary equality step reads only those maps, immutable nodes and its canonical input pair. Its generated child equations and failure decision are therefore the same for an identical key.

Queue contents and live occurrences are not read by that equality calculation. They remain owned by the caller. Cached failure clears only that caller's equation queue, just as ordinary failure would. The executor still invalidates candidate matches after the serviced equality step and performs one ordinary source service step. Reuse does not contract the source schedule.

The argument depends on this boundary. Context-dependent guards, source resource claims or mutable constructor metadata cannot be added to a transition without revisiting its key and ownership proof. Different equality states miss even when their differences are irrelevant; the gate does not establish a generalized key.

## Costs now worth measuring

The table retains intermediate equality maps. A later update can therefore force a copy that the uncached store could have performed in place. Lookup, state identity allocation, retained child equations and final cache disposal also cost work. The 4,096-entry cap bounds the entry count, not a universal byte limit or an optimal retention policy.

A favorable case must repeat substantive equality work from compatible states. Adverse controls need mostly distinct requests, intervening unrelated bindings, cheap equalities and large state with little reuse. Compare against the ordinary contextual and corrected relational executors, relevant compiled controls, and a credible common-work elimination control where source semantics permit it. Merely sharing pointers cannot justify a runtime recommendation.

This result supports a bounded lifecycle pilot next, after prospective registration. Its decision value is whether reusing newly derived state repays the copying and retention it introduces. A source-level control that moves provably common pure work before a fork is a strong alternative; any such transformation must retain raw alternatives, effects and permitted scheduling. T073's reusable general lowering remains the strongest independent next direction and must be reconsidered after this pilot.

New equality-state reuse now has direct evidence, but wider contextual validity, union-find expressed through CHR, strategic port rewrites, distributed claims and sustained lifetime remain required investigations. T072 and the architecture goal remain active.

[Registration](../registrations/S02-shared-deduction-gate.md) · [Source freeze](s02-shared-deduction-gate/freeze.json) · [Full tests](s02-shared-deduction-gate/tests.log) · [Scoped Clippy](s02-shared-deduction-gate/clippy.log) · [Store implementation](../../../research/chr-relational/src/contextual.rs)

# R02: when partial equality may enable consuming rules

Finite successful runs in the current positive-equality fragment can be serialized to ordinary CHR with atomic equations. Failed speculative traces need not have that correspondence. Pure guards alone do not justify the optimization: a read-only guard can change from true to false as an equation progresses.

This analysis resolves a prerequisite of the [consuming integration entry](R02-consuming-integration-entry.md). It supports an integrated semantic prototype under explicit obligations, not a performance claim or a change to the language's guard contract.

## Bounded successful-run claim

Consider one interpretation using nonbinding constructor matching and positive equality-entailment guards. Assume deductions follow soundly from already-posted equations, source applications claim live distinct occurrences with valid propagation history, and a finite run fully accounts for every fired body, ends with consistent finite-tree equations and complete quiescence, and extracts their most-general joint relationships. Its source application order can be reproduced with atomic equations, modulo fresh-variable renaming.

Introduce the complete query and replay the recorded source application order. After each firing, install all its conjunctive body occurrences and equations, including those still behind unexpanded administrative nodes in the candidate. Solve those equations atomically before the next application. These equations belong to the completed run's consistent accumulated obligations. Their most-general solution satisfies the partial deductions that justified the next application.

Nonbinding constructor matches and positive equality entailment survive a consistent substitution extension. The chosen application therefore remains enabled. Solving equations consumes no source occurrence and creates no propagation token; keeping preceding applications in order preserves the live tuple and its history eligibility. Earlier sibling occurrences can enable more applications, but do not prevent choosing the same recorded tuple. Allocate body-local variables earlier if necessary using an injective renaming. Faithful most-general extraction then preserves outputs, aliases and residual multiplicity; merely returning some satisfying grounding would not establish this claim.

Additional equations can enable competing rules, but no mandatory source priority requires selecting them. This proves that one permitted execution exists; it does not assume confluence or equate all schedules. Solved substitutions in the proof are a reasoning device, not an implementation interface requirement.

This is a soundness argument for completed runs. It does not prove that every permitted source answer is found, that an engine terminates, or that it services pending work fairly.

## Failure speculation is a different claim

```text
query: start(X), p(X)

start(X) <=> pair(X,a) =:= pair(b,c)
p(b)     <=> hit
```

An integrated reducer can establish `X=b`, consume `p(X)` and produce `hit`, then discover the clash between `a` and `c`. Atomic source equality fails before enabling `p(b)`. That speculative application has no corresponding atomic source trace.

Failing the whole interpretation preserves its lack of answers. Publishing `hit`, counting it as a completed source answer, or letting it affect an incompatible interpretation would be incorrect. Consuming the producer must not erase its outstanding consistency obligation. The experiment must count such speculation as work, even though it contributes no answer.

Fair service of consistency work is also necessary. A speculative recursive rule could keep generating work while a pending clash waits. The successful-run proof does not excuse that loss of progress; a concrete scheduler must service the clash.

## Purity and substitution stability differ

Consider a broader read-only guard that tests whether its argument is currently a variable:

```text
query: start(X,Y), p(X,Y)

start(X,Y) <=> pair(X,Y) =:= pair(a,b)
p(a,Y)     <=> var(Y) | hit
```

Partial equality can expose `X=a` while `Y` is still free, making the guard succeed. Atomic equality binds both variables before `p(a,Y)` can match, so it does not permit that firing. Here even the final equation set is consistent. Read-only behavior does not make this guard stable under substitution.

`var` is outside the current implemented `Equal` guard fragment. The example identifies an architectural obligation if such guards are admitted; it does not assert a new catalog of supported guards. The distinction between logical and nonlogical built-ins also appears in [Christiansen and Kirkeby's CHR semantics](https://arxiv.org/pdf/1611.03628).

Preserving a broader guard contract has several possible implementation paths. Infer stability for known guards and permit early application only there; stabilize already-posted equations before evaluating other guards; or retain enough speculative state to validate and undo a guard-dependent application. The last option adds consumption/history rollback obligations. Optional declarations or a mandatory stable-guard fragment are language-design alternatives whose adoption would require an owner decision. None is required to investigate the current positive fragment.

## Obligations for the integrated gate

- Enabling facts have causal producers. Preallocated future bodies or inactive alternatives cannot justify a match.
- Every fired body is fully expanded or equivalently accounted for. Its occurrences, equations and consistency checks survive consumption of their source producers, even when administrative expansion is pending.
- Value fusion preserves source occurrence IDs, legal claims and propagation tuples.
- Finite-tree consistency rejects symbol/arity clashes and positive constructor cycles modulo equality; equality-only cycles remain aliases.
- Publication waits for consistency, source and index-repair obligations across complete fired bodies, and extracts most-general joint relationships. An empty current work queue alone is insufficient.
- Scheduling services consistency work as well as applications. Speculation remains local to its interpretation and has no external effects.

The gate should check successful serialization and the discarded-failure witness separately. General guard support remains a cross-cutting R05 question with concrete implementation alternatives. No successful-run proof can substitute for coverage, progress or lifetime measurements.

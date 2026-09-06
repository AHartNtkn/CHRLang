# Sound answer deduplication and stronger state reuse

Finite structural answer equivalence is decidable and sufficient for conservative deduplication. Reusing an execution state requires more information than an answer: live occurrence identity relationships, propagation tokens, pending work and any scheduling state that the implementation promises to preserve.

The constructions below are original consequences of the reference semantics. They preserve the baseline's full residual multiset observation. A solver-specific projection interface is a separate proposal and does not silently change these observations.

## An exact conservative answer key

Start from the selected output tuple in its fixed positional order and the entire finite residual multiset. Interpret finite-tree bindings throughout. Preserve rigid constructor names, all repeated-variable relationships, and duplicate residual occurrences. Non-output free variables are existential and may be consistently renamed. Unused existential variables can be omitted in the equality-only, nonempty finite-term domain.

An elementary decision procedure enumerates bijections from the n remaining free variables to canonical names 0 through n−1. For each bijection, serialize the output tuple and sort the serialized residual multiset, keeping duplicates. The lexicographically least serialization is a canonical key. Two observations have equal keys exactly when they are structurally equal up to a consistent variable renaming. This follows because a common key supplies a witnessing pair of renamings; conversely, an isomorphism induces the same set of serialized candidates.

The n! upper bound makes this an existence/control algorithm, not the proposed fast implementation. Output-root positions and constructor structure can refine variable partitions before permutation; exact structural comparison must still resolve hash collisions and any ambiguous partitions. Shared term DAGs may avoid repeated traversal, but physical subterm sharing is not part of the source observation. Two differently shared graphs denoting the same finite terms must compare equal.

Examples separate the requirements. `(X,X)` differs from `(X,Y)` with independent X,Y. Residual `{p(X),p(X)}` differs from `{p(X)}`. `{p(X),q(X)}` differs from `{p(X),q(Y)}`. Renaming every occurrence of X to U preserves equality. Different synthesized program terms remain different even when they pass the same examples.

This closes the existence question for trustworthy structural deduplication. Efficient canonical labeling on realistic residual stores, memory retention during infinite enumeration, and the API's deduplication toggle/default still need separate evaluation or owner choice.

## Why answer equality is not continuation equality

Take a propagation rule `p(X) ==> q(X)` and another rule that can consume q. A quiescent state containing only p may have a token showing the propagation already fired and its q was consumed. Another state with the same visible p but no token can still fire. Comparing visible residual constraints alone cannot justify identical future behavior. A suspended or currently unavailable match may also become enabled after a future binding.

Therefore an output cache cannot automatically become a saved-state cache. A continuation interface must declare whether external constraints or bindings can later be added, whether fresh query roots can be introduced, and whether prior occurrence identities are observable. The current answer contract declares none of those extra operations.

## A sufficient whole-state equivalence

For internal merging under the existing evaluator, compare complete projected states at source-step boundaries: pending goals; live identified occurrences; normalized bindings; output roots; and relevant propagation tokens. Require a bijection of logical variables and live occurrence identities that preserves rule IDs, ordered token head positions and every alias relationship. If the implementation promises a fixed committed policy, also preserve the policy's relevant order, ages and continuation state. Otherwise the merged state may choose a common permitted CHR schedule, but must not claim to reproduce a particular policy's trace.

Tokens referencing an occurrence that is permanently consumed can be omitted from that projection. The same tuple can never become live again because identities are not recycled. For conditional states this removal is valid only on contexts where that occurrence is absent permanently. Tokens on fully live tuples remain necessary even if their arguments currently do not match a rule: later unification may change applicability.

The historical set of all allocated identities need not be part of the comparison if fresh allocation is implemented so that it can always choose names disjoint from the continuing state, and the semantic proof consistently renames the future fresh supply. This is alpha-equivalence of an abstract machine, not permission to recycle an identity still referenced by a live token, job or snapshot.

## Bisimulation argument

Let two complete states be related by the bijections above. Each enabled rule tuple in one maps to a live distinct tuple in the other, with the same guard and matching result because bindings and term structure correspond. Token absence is preserved. Applying the same rule adds corresponding bodies with a fresh extension of the bijection. Introduce and Unify correspond by fresh naming and finite-tree unification; failures agree. A source OR supplies two corresponding children. Thus the states simulate each other's source transitions, including quiescence and observations.

This establishes a sufficient state-merging criterion without deciding arbitrary semantic equivalence of CHR programs. It is stronger than necessary but testable on finite data. It can be applied to a shared continuation while retaining separate alternative lineages or a multiplicity representation; merging stored state does not authorize losing the two explicit derivations. New choices of that common continuation must create two children for each retained lineage.

For the fixed-policy variant, age/order is part of the relation rather than an afterthought. If merging different ages changes which enabled nonconfluent rule fires, answer preservation relative to that policy can fail even though the unordered source semantics still permits the new schedule.

## Projection needs an explicit observation contract

Dropping a disconnected residual is not generally safe. A disconnected active constraint can still have an enabled failure rule, so output-variable reachability alone does not establish quiescence. At quiescence, an uninterpreted residual may still express a future restriction; pretending it is true changes the answer formula.

A certified logical solver can prove that an existential component is satisfiable and independent, then remove it from a projected formula. For instance, `exists X. no_c(X)` over the declared SK domain has witness k. This preserves that projected logical formula, but the baseline's full residual multiset still distinguishes its presence. Adopting projected formulas as answer observations is an owner semantic decision after these consequences are explained.

A presentation-only UI can keep the complete internal answer while showing a summary and offering the residual detail. That is compatible with either observation contract; hiding text must not be confused with proving a residual irrelevant.

## Remaining empirical and owner questions

The exact criteria above justify investigating both final deduplication and internal whole-state merging. Their practical value depends on the frequency of identical residual graphs or complete states, and on canonicalization cost. The SK examples provide repeated constrained families, but do not establish that those families contain identical complete states.

The owner still needs to choose a concrete answer interface, deduplication defaults and any future continuation API. Those choices do not block the conservative internal equivalence or further architecture research. A claim that arbitrary answers with the same possible ground instances are interchangeable remains outside this structural criterion.

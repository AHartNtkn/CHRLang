# What named superpositions must represent for this language

Named superpositions can encode correlated values, but relational execution also needs contextual variable bindings and active constraint effects. A direct use of a functional runtime's value-collapse operation does not establish those services. The useful comparison is between a pure encoded machine and an extension with native logical-variable and occurrence services.

The runtime inspected is HVM4 commit `6defdfc7dae2a3cca5dd6e74ed0612385b5646a8`, locally at `/tmp/chr-hvm4-research-20260906`. The relevant source is [src/hvm.c](https://github.com/HigherOrderCO/HVM4/blob/6defdfc7dae2a3cca5dd6e74ed0612385b5646a8/src/hvm.c): `wnf_dup_sup` at line 3923 and `eval_collapse_process` at line 6158. The arguments below are a proposed encoding analysis, not claims of an existing CHR implementation.

## Label selection is a useful semantic model

Let an internal node Sup(d,u,v) select u or v according to the same decision d everywhere it is encountered in one alternative. Different dynamic choices have distinct identities. Define projection recursively by following those decisions and leaving ordinary constructors intact. Two occurrences of Sup with the same label are correlated; independent labels permit all compatible combinations. Nodes outside the birth scope of d must not create an extra answer for d.

For a pure operation F and a supported family, distribution is valid only with correlated projection of all operands. Splitting an operand with label d requires restricting other operands to that same decision in each arm. Otherwise `eq(Sup(d,a,b), Sup(d,a,b))` could manufacture the cross-pairs (a,b) and (b,a). This projection model gives a simple correctness criterion for a proposed graph rewrite: each compatible decision assignment must retain its result.

The local HVM source implements same-label DUP/SUP annihilation and different-label commutation. That makes label allocation semantically significant. It is not permission to identify every duplication label with the nearby source choice.

Consider copying X, where X is Sup(d,a,b), to construct Pair(X,X). Using a duplication with label d directly annihilates the superposition and produces Pair(a,b). The intended alternatives are Pair(a,a) and Pair(b,b). A source compiler must distinguish copying a value from projecting a choice. A fresh duplication label different from d allows the different-label commute rule to retain d's correlation. Complete label hygiene must also cover administrative superpositions introduced by function duplication; they must not become unrequested source alternatives at observation.

## A finite-tree variable is not a functional binder

Represent a source logical variable by an opaque handle h and a contextual binding service. Copying h copies a reference to the same source variable; it does not allocate two unknowns. A backend lambda binder and its substitution mechanism cannot alone implement symmetric first-order unification of arbitrarily aliased source variables.

For an equation X =:= t, first dereference under a consistent context. Equal representatives succeed. Distinct constructor roots fail there. Equal roots generate child equations. Binding an unbound representative requires a contextual occurs check, then composition with the existing environment. All aliases consult the resulting binding. Intermediate updates remain private until the whole source equation succeeds.

`X =:= Sup(d,s(X),z)` is notation for a runtime family, never source syntax. The d-left projection fails its occurs check and the d-right projection binds X to z. Rejecting the union graph as cyclic loses the successful branch. Omitting the check accepts a forbidden infinite term. A native equality service must therefore propagate both the binding and its validity condition. Alternatively, a pure encoded machine can run the finite-tree algorithm separately under each decision assignment; that is correct but may repeat its work.

An already identical opaque handle need not be forced into separate values to unify it with itself. This identifies a useful fast path independent of architecture. Constructor clashes, repeated head variables and pure guards may force more information; merely passing a handle through a body does not.

## Failure and consumption are active effects

A failing active constraint cannot disappear because the selected output never reads its value. The runtime needs a condition for which alternatives remain live, and it must include all active constraints in quiescence certification. Representing failure only as an erased value at one argument can leave an unrelated output incorrectly publishable.

Consumption has the same issue. If occurrence p is consumed only under d-left, it remains available under d-right. A single destructive update of p cannot implement both. The implementation needs contextual membership, contextual occurrence versions, or a graph of alternative effects that projects to those memberships. Named values do not remove that information requirement.

For full CHR, propagation history is contextual too: firing once on a tuple before a source split suppresses it in both descendants; firing only in one descendant must not suppress the other. A projected term-value cache does not determine the token's membership. This is why a value-only superposition evaluator is insufficient even when every output term is represented correctly.

## Pure encoded machine

One baseline translation makes logical-variable handles, substitutions, occurrence identities, histories and pending work ordinary backend data. A resumable pure function advances a finite machine action, returning a new state, a source fork, failure, or a certified answer. Only source forks introduce semantic alternatives; administrative lambdas and duplication nodes are not observable source terms.

This can preserve the source by interpreting each projected backend state as one reference alternative and proving every machine action. It avoids extending the backend's mutation rules. It also risks distributing the entire encoded state whenever a demanded field is at a root superposition. Persistent data alone saves storage, not necessarily post-choice computation.

To retain common work, factor the state and operation so that an expansion receives opaque argument handles and only the fields it actually inspects. Shared event provenance can then preserve the carry-chain example described in T015-relation-graph-construction.md. A transformation that repeatedly rebuilds or pattern-matches a whole contextual state needs its cost charged, even if its source code is short.

## Native services

A second candidate adds contextual bindings, occurrence membership, wake-up and history to the backend. Named superpositions then implement part of the value representation while services perform source effects. This may avoid repeated traversal of an encoded environment, but it is a larger runtime with conditional-update invariants. The conditional kernel already supplies useful local proof obligations; a new backend still has to show how its concrete rewrites satisfy them.

These candidates differ in engineering responsibility. The pure encoding places the source semantics in an interpreter/compiler and uses existing backend operations. Native services put more of the semantics into the runtime. Both can keep choices opaque to source rules, both can support nonground queries, and neither has an established performance advantage from this analysis alone.

## Collapse and fair answers

At the pinned revision, `eval_collapse_process` calls `cnf(before)` before returning to its queue; the inspected `cnf_at` descends into terms. The queue's presence is therefore not a proof of finite service for every source program. This is a source-level dependency observation, not a measured starvation result.

A CHR embedding can expose bounded source-machine continuations and use its own fair scheduler, or the runtime can provide resumable normalization/collapse. In either case, an answer is released only after projected source quiescence and consistency are certified. Normal form of an output term is not that certificate. Asking for the next ten answers also requires incremental output and sound deduplication while other alternatives remain active.

## Remaining work

A complete pure embedding needs machine-action correspondence, fresh-label hygiene, and a proof that no administrative alternative leaks into the source answer stream. A native embedding needs concrete service rewrite rules and their invariants. These are still analytical tasks. Backend node count, distribution work, normalization overhead and actual savings on synthesis are implementation questions once those encodings are fixed.

The practical recommendation at this stage is to keep both encodings in the comparison and make their service costs explicit. Do not adopt the runtime's structural equality as source unification or its collapse policy as the language's fairness semantics without those proofs.
